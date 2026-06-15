//! A32 simd add_sub tests.
//!
//! Auto-generated from ARM ASL specifications.
//! DO NOT EDIT MANUALLY.

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::generated::test_helpers_32::*;

// ============================================================================
// aarch32_VADDL_A Tests
// ============================================================================

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `field U 24 +: 1`
/// Requirement: FieldBoundary { field: "U", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_vaddl_t1a1_a_field_u_0_min_100_f2800000() {
    // Encoding: 0xF2800000
    // Test aarch32_VADDL_T1A1_A field U = 0 (Min)
    // ISET: A32
    // Fields: D=0, op=0, U=0, M=0, size=0, Vn=0, N=0, Vm=0, Vd=0
    let encoding: u32 = 0xF2800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `field U 24 +: 1`
/// Requirement: FieldBoundary { field: "U", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_vaddl_t1a1_a_field_u_1_max_100_f3800000() {
    // Encoding: 0xF3800000
    // Test aarch32_VADDL_T1A1_A field U = 1 (Max)
    // ISET: A32
    // Fields: D=0, Vm=0, op=0, N=0, Vn=0, size=0, M=0, U=1, Vd=0
    let encoding: u32 = 0xF3800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `field D 22 +: 1`
/// Requirement: FieldBoundary { field: "D", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_vaddl_t1a1_a_field_d_0_min_100_f2800000() {
    // Encoding: 0xF2800000
    // Test aarch32_VADDL_T1A1_A field D = 0 (Min)
    // ISET: A32
    // Fields: N=0, M=0, Vd=0, D=0, Vm=0, U=0, size=0, Vn=0, op=0
    let encoding: u32 = 0xF2800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `field D 22 +: 1`
/// Requirement: FieldBoundary { field: "D", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_vaddl_t1a1_a_field_d_1_max_100_f2c00000() {
    // Encoding: 0xF2C00000
    // Test aarch32_VADDL_T1A1_A field D = 1 (Max)
    // ISET: A32
    // Fields: op=0, U=0, M=0, Vm=0, Vn=0, Vd=0, size=0, D=1, N=0
    let encoding: u32 = 0xF2C00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `field size 20 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch32_vaddl_t1a1_a_field_size_0_min_100_f2800000() {
    // Encoding: 0xF2800000
    // Test aarch32_VADDL_T1A1_A field size = 0 (Min)
    // ISET: A32
    // Fields: N=0, U=0, Vd=0, op=0, size=0, Vn=0, D=0, Vm=0, M=0
    let encoding: u32 = 0xF2800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `field size 20 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 1, boundary: PowerOfTwo }
/// 16-bit / halfword size
#[test]
fn test_aarch32_vaddl_t1a1_a_field_size_1_poweroftwo_100_f2900000() {
    // Encoding: 0xF2900000
    // Test aarch32_VADDL_T1A1_A field size = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: D=0, M=0, Vn=0, Vm=0, N=0, size=1, op=0, U=0, Vd=0
    let encoding: u32 = 0xF2900000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `field size 20 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 2, boundary: PowerOfTwo }
/// 32-bit / word size
#[test]
fn test_aarch32_vaddl_t1a1_a_field_size_2_poweroftwo_100_f2a00000() {
    // Encoding: 0xF2A00000
    // Test aarch32_VADDL_T1A1_A field size = 2 (PowerOfTwo)
    // ISET: A32
    // Fields: size=2, Vm=0, D=0, op=0, Vn=0, M=0, U=0, Vd=0, N=0
    let encoding: u32 = 0xF2A00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `field size 20 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 3, boundary: Max }
/// 64-bit / doubleword size
#[test]
fn test_aarch32_vaddl_t1a1_a_field_size_3_max_100_f2b00000() {
    // Encoding: 0xF2B00000
    // Test aarch32_VADDL_T1A1_A field size = 3 (Max)
    // ISET: A32
    // Fields: op=0, Vd=0, M=0, N=0, U=0, Vm=0, size=3, Vn=0, D=0
    let encoding: u32 = 0xF2B00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `field Vn 16 +: 4`
/// Requirement: FieldBoundary { field: "Vn", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_aarch32_vaddl_t1a1_a_field_vn_0_min_100_f2800000() {
    // Encoding: 0xF2800000
    // Test aarch32_VADDL_T1A1_A field Vn = 0 (Min)
    // ISET: A32
    // Fields: Vd=0, size=0, U=0, N=0, Vn=0, M=0, D=0, Vm=0, op=0
    let encoding: u32 = 0xF2800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `field Vn 16 +: 4`
/// Requirement: FieldBoundary { field: "Vn", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_aarch32_vaddl_t1a1_a_field_vn_1_poweroftwo_100_f2810000() {
    // Encoding: 0xF2810000
    // Test aarch32_VADDL_T1A1_A field Vn = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: size=0, M=0, op=0, N=0, Vm=0, D=0, Vd=0, U=0, Vn=1
    let encoding: u32 = 0xF2810000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `field Vd 12 +: 4`
/// Requirement: FieldBoundary { field: "Vd", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_aarch32_vaddl_t1a1_a_field_vd_0_min_100_f2800000() {
    // Encoding: 0xF2800000
    // Test aarch32_VADDL_T1A1_A field Vd = 0 (Min)
    // ISET: A32
    // Fields: size=0, U=0, D=0, Vm=0, N=0, Vd=0, Vn=0, M=0, op=0
    let encoding: u32 = 0xF2800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `field Vd 12 +: 4`
/// Requirement: FieldBoundary { field: "Vd", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_aarch32_vaddl_t1a1_a_field_vd_1_poweroftwo_100_f2801000() {
    // Encoding: 0xF2801000
    // Test aarch32_VADDL_T1A1_A field Vd = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: M=0, N=0, U=0, D=0, Vd=1, size=0, Vn=0, Vm=0, op=0
    let encoding: u32 = 0xF2801000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `field op 8 +: 1`
/// Requirement: FieldBoundary { field: "op", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_vaddl_t1a1_a_field_op_0_min_100_f2800000() {
    // Encoding: 0xF2800000
    // Test aarch32_VADDL_T1A1_A field op = 0 (Min)
    // ISET: A32
    // Fields: D=0, Vn=0, Vd=0, N=0, Vm=0, size=0, op=0, M=0, U=0
    let encoding: u32 = 0xF2800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `field op 8 +: 1`
/// Requirement: FieldBoundary { field: "op", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_vaddl_t1a1_a_field_op_1_max_100_f2800100() {
    // Encoding: 0xF2800100
    // Test aarch32_VADDL_T1A1_A field op = 1 (Max)
    // ISET: A32
    // Fields: U=0, Vm=0, N=0, size=0, D=0, Vd=0, op=1, M=0, Vn=0
    let encoding: u32 = 0xF2800100;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `field N 7 +: 1`
/// Requirement: FieldBoundary { field: "N", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_vaddl_t1a1_a_field_n_0_min_100_f2800000() {
    // Encoding: 0xF2800000
    // Test aarch32_VADDL_T1A1_A field N = 0 (Min)
    // ISET: A32
    // Fields: U=0, Vm=0, Vd=0, D=0, op=0, M=0, size=0, Vn=0, N=0
    let encoding: u32 = 0xF2800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `field N 7 +: 1`
/// Requirement: FieldBoundary { field: "N", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_vaddl_t1a1_a_field_n_1_max_100_f2800080() {
    // Encoding: 0xF2800080
    // Test aarch32_VADDL_T1A1_A field N = 1 (Max)
    // ISET: A32
    // Fields: M=0, D=0, op=0, size=0, Vd=0, Vn=0, Vm=0, N=1, U=0
    let encoding: u32 = 0xF2800080;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `field M 5 +: 1`
/// Requirement: FieldBoundary { field: "M", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_vaddl_t1a1_a_field_m_0_min_100_f2800000() {
    // Encoding: 0xF2800000
    // Test aarch32_VADDL_T1A1_A field M = 0 (Min)
    // ISET: A32
    // Fields: N=0, U=0, size=0, D=0, M=0, Vd=0, Vn=0, Vm=0, op=0
    let encoding: u32 = 0xF2800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `field M 5 +: 1`
/// Requirement: FieldBoundary { field: "M", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_vaddl_t1a1_a_field_m_1_max_100_f2800020() {
    // Encoding: 0xF2800020
    // Test aarch32_VADDL_T1A1_A field M = 1 (Max)
    // ISET: A32
    // Fields: op=0, N=0, M=1, size=0, Vm=0, U=0, D=0, Vd=0, Vn=0
    let encoding: u32 = 0xF2800020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `field Vm 0 +: 4`
/// Requirement: FieldBoundary { field: "Vm", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_aarch32_vaddl_t1a1_a_field_vm_0_min_100_f2800000() {
    // Encoding: 0xF2800000
    // Test aarch32_VADDL_T1A1_A field Vm = 0 (Min)
    // ISET: A32
    // Fields: size=0, N=0, Vn=0, M=0, D=0, Vm=0, Vd=0, U=0, op=0
    let encoding: u32 = 0xF2800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `field Vm 0 +: 4`
/// Requirement: FieldBoundary { field: "Vm", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_aarch32_vaddl_t1a1_a_field_vm_1_poweroftwo_100_f2800001() {
    // Encoding: 0xF2800001
    // Test aarch32_VADDL_T1A1_A field Vm = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: D=0, size=0, Vn=0, Vm=1, op=0, N=0, Vd=0, U=0, M=0
    let encoding: u32 = 0xF2800001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// U=0 (minimum value)
#[test]
fn test_aarch32_vaddl_t1a1_a_combo_0_100_f2800000() {
    // Encoding: 0xF2800000
    // Test aarch32_VADDL_T1A1_A field combination: U=0, D=0, size=0, Vn=0, Vd=0, op=0, N=0, M=0, Vm=0
    // ISET: A32
    // Fields: op=0, U=0, Vd=0, Vm=0, size=0, D=0, N=0, M=0, Vn=0
    let encoding: u32 = 0xF2800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `field size = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "size", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch32_vaddl_t1a1_a_special_size_0_size_variant_0_256_f2800000() {
    // Encoding: 0xF2800000
    // Test aarch32_VADDL_T1A1_A special value size = 0 (Size variant 0)
    // ISET: A32
    // Fields: size=0, U=0, Vd=0, D=0, op=0, N=0, Vn=0, M=0, Vm=0
    let encoding: u32 = 0xF2800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `field size = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "size", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch32_vaddl_t1a1_a_special_size_1_size_variant_1_256_f2900000() {
    // Encoding: 0xF2900000
    // Test aarch32_VADDL_T1A1_A special value size = 1 (Size variant 1)
    // ISET: A32
    // Fields: M=0, N=0, op=0, Vn=0, U=0, Vm=0, D=0, size=1, Vd=0
    let encoding: u32 = 0xF2900000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `field size = 2 (Size variant 2)`
/// Requirement: FieldSpecial { field: "size", value: 2, meaning: "Size variant 2" }
/// Size variant 2
#[test]
fn test_aarch32_vaddl_t1a1_a_special_size_2_size_variant_2_256_f2a00000() {
    // Encoding: 0xF2A00000
    // Test aarch32_VADDL_T1A1_A special value size = 2 (Size variant 2)
    // ISET: A32
    // Fields: Vm=0, op=0, N=0, size=2, D=0, M=0, Vn=0, U=0, Vd=0
    let encoding: u32 = 0xF2A00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `field size = 3 (Size variant 3)`
/// Requirement: FieldSpecial { field: "size", value: 3, meaning: "Size variant 3" }
/// Size variant 3
#[test]
fn test_aarch32_vaddl_t1a1_a_special_size_3_size_variant_3_256_f2b00000() {
    // Encoding: 0xF2B00000
    // Test aarch32_VADDL_T1A1_A special value size = 3 (Size variant 3)
    // ISET: A32
    // Fields: M=0, U=0, Vm=0, size=3, N=0, D=0, Vd=0, Vn=0, op=0
    let encoding: u32 = 0xF2B00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `Binary { op: Eq, lhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: "Vd" }), indices: [Single(LitInt(0))] }, rhs: Binary { op: Or, lhs: LitBits([true]), rhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "op" }), rhs: Binary { op: And, lhs: LitBits([true]), rhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: "Vn" }), indices: [Single(LitInt(0))] } } }, rhs: LitBits([true]) } } }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: \"Vd\" }), indices: [Single(LitInt(0))] }, rhs: Binary { op: Or, lhs: LitBits([true]), rhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"op\" }), rhs: Binary { op: And, lhs: LitBits([true]), rhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: \"Vn\" }), indices: [Single(LitInt(0))] } } }, rhs: LitBits([true]) } } }" }
/// triggers Undefined
#[test]
fn test_aarch32_vaddl_t1a1_a_invalid_0_100_f2800000() {
    // Encoding: 0xF2800000
    // Test aarch32_VADDL_T1A1_A invalid encoding: Binary { op: Eq, lhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: "Vd" }), indices: [Single(LitInt(0))] }, rhs: Binary { op: Or, lhs: LitBits([true]), rhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "op" }), rhs: Binary { op: And, lhs: LitBits([true]), rhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: "Vn" }), indices: [Single(LitInt(0))] } } }, rhs: LitBits([true]) } } }
    // ISET: A32
    // Fields: D=0, Vd=0, op=0, N=0, size=0, Vn=0, M=0, U=0, Vm=0
    let encoding: u32 = 0xF2800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || !matches!(exit.unwrap(), CpuExit::Continue),
        "expected UNDEFINED for A32 encoding 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `Unconditional UNDEFINED`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNDEFINED" }
/// triggers Undefined
#[test]
fn test_aarch32_vaddl_t1a1_a_invalid_1_100_f2800000() {
    // Encoding: 0xF2800000
    // Test aarch32_VADDL_T1A1_A invalid encoding: Unconditional UNDEFINED
    // ISET: A32
    // Fields: N=0, Vn=0, D=0, op=0, size=0, Vm=0, M=0, U=0, Vd=0
    let encoding: u32 = 0xF2800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || !matches!(exit.unwrap(), CpuExit::Continue),
        "expected UNDEFINED for A32 encoding 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `field U 28 +: 1`
/// Requirement: FieldBoundary { field: "U", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_vaddl_t1a1_a_field_u_0_min_100_ef800000() {
    // Thumb encoding (32): 0xEF800000
    // Test aarch32_VADDL_T1A1_A field U = 0 (Min)
    // ISET: T32
    // Fields: size=0, U=0, op=0, N=0, Vd=0, D=0, Vn=0, Vm=0, M=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF800000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `field U 28 +: 1`
/// Requirement: FieldBoundary { field: "U", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_vaddl_t1a1_a_field_u_1_max_100_ff800000() {
    // Thumb encoding (32): 0xFF800000
    // Test aarch32_VADDL_T1A1_A field U = 1 (Max)
    // ISET: T32
    // Fields: size=0, M=0, U=1, Vn=0, op=0, D=0, Vd=0, Vm=0, N=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFF800000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `field D 22 +: 1`
/// Requirement: FieldBoundary { field: "D", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_vaddl_t1a1_a_field_d_0_min_100_ef800000() {
    // Thumb encoding (32): 0xEF800000
    // Test aarch32_VADDL_T1A1_A field D = 0 (Min)
    // ISET: T32
    // Fields: U=0, D=0, Vd=0, Vn=0, op=0, M=0, N=0, Vm=0, size=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF800000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `field D 22 +: 1`
/// Requirement: FieldBoundary { field: "D", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_vaddl_t1a1_a_field_d_1_max_100_efc00000() {
    // Thumb encoding (32): 0xEFC00000
    // Test aarch32_VADDL_T1A1_A field D = 1 (Max)
    // ISET: T32
    // Fields: Vn=0, Vm=0, size=0, N=0, M=0, D=1, U=0, Vd=0, op=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEFC00000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `field size 20 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch32_vaddl_t1a1_a_field_size_0_min_100_ef800000() {
    // Thumb encoding (32): 0xEF800000
    // Test aarch32_VADDL_T1A1_A field size = 0 (Min)
    // ISET: T32
    // Fields: Vm=0, Vn=0, Vd=0, op=0, size=0, D=0, U=0, N=0, M=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF800000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `field size 20 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 1, boundary: PowerOfTwo }
/// 16-bit / halfword size
#[test]
fn test_aarch32_vaddl_t1a1_a_field_size_1_poweroftwo_100_ef900000() {
    // Thumb encoding (32): 0xEF900000
    // Test aarch32_VADDL_T1A1_A field size = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Vd=0, D=0, Vn=0, N=0, U=0, size=1, Vm=0, M=0, op=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF900000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `field size 20 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 2, boundary: PowerOfTwo }
/// 32-bit / word size
#[test]
fn test_aarch32_vaddl_t1a1_a_field_size_2_poweroftwo_100_efa00000() {
    // Thumb encoding (32): 0xEFA00000
    // Test aarch32_VADDL_T1A1_A field size = 2 (PowerOfTwo)
    // ISET: T32
    // Fields: op=0, Vn=0, M=0, N=0, Vd=0, Vm=0, size=2, D=0, U=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEFA00000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `field size 20 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 3, boundary: Max }
/// 64-bit / doubleword size
#[test]
fn test_aarch32_vaddl_t1a1_a_field_size_3_max_100_efb00000() {
    // Thumb encoding (32): 0xEFB00000
    // Test aarch32_VADDL_T1A1_A field size = 3 (Max)
    // ISET: T32
    // Fields: size=3, N=0, op=0, M=0, U=0, Vn=0, Vm=0, Vd=0, D=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEFB00000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `field Vn 16 +: 4`
/// Requirement: FieldBoundary { field: "Vn", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_aarch32_vaddl_t1a1_a_field_vn_0_min_100_ef800000() {
    // Thumb encoding (32): 0xEF800000
    // Test aarch32_VADDL_T1A1_A field Vn = 0 (Min)
    // ISET: T32
    // Fields: U=0, Vd=0, D=0, Vn=0, M=0, op=0, size=0, N=0, Vm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF800000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `field Vn 16 +: 4`
/// Requirement: FieldBoundary { field: "Vn", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_aarch32_vaddl_t1a1_a_field_vn_1_poweroftwo_100_ef810000() {
    // Thumb encoding (32): 0xEF810000
    // Test aarch32_VADDL_T1A1_A field Vn = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: size=0, Vm=0, U=0, N=0, M=0, Vd=0, op=0, Vn=1, D=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF810000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `field Vd 12 +: 4`
/// Requirement: FieldBoundary { field: "Vd", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_aarch32_vaddl_t1a1_a_field_vd_0_min_100_ef800000() {
    // Thumb encoding (32): 0xEF800000
    // Test aarch32_VADDL_T1A1_A field Vd = 0 (Min)
    // ISET: T32
    // Fields: Vn=0, Vd=0, op=0, Vm=0, D=0, size=0, M=0, U=0, N=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF800000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `field Vd 12 +: 4`
/// Requirement: FieldBoundary { field: "Vd", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_aarch32_vaddl_t1a1_a_field_vd_1_poweroftwo_100_ef801000() {
    // Thumb encoding (32): 0xEF801000
    // Test aarch32_VADDL_T1A1_A field Vd = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: N=0, op=0, Vm=0, D=0, Vn=0, size=0, U=0, Vd=1, M=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF801000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `field op 8 +: 1`
/// Requirement: FieldBoundary { field: "op", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_vaddl_t1a1_a_field_op_0_min_100_ef800000() {
    // Thumb encoding (32): 0xEF800000
    // Test aarch32_VADDL_T1A1_A field op = 0 (Min)
    // ISET: T32
    // Fields: M=0, N=0, U=0, size=0, D=0, Vd=0, Vm=0, Vn=0, op=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF800000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `field op 8 +: 1`
/// Requirement: FieldBoundary { field: "op", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_vaddl_t1a1_a_field_op_1_max_100_ef800100() {
    // Thumb encoding (32): 0xEF800100
    // Test aarch32_VADDL_T1A1_A field op = 1 (Max)
    // ISET: T32
    // Fields: N=0, D=0, op=1, Vn=0, size=0, U=0, Vm=0, Vd=0, M=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF800100;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `field N 7 +: 1`
/// Requirement: FieldBoundary { field: "N", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_vaddl_t1a1_a_field_n_0_min_100_ef800000() {
    // Thumb encoding (32): 0xEF800000
    // Test aarch32_VADDL_T1A1_A field N = 0 (Min)
    // ISET: T32
    // Fields: Vd=0, size=0, N=0, D=0, op=0, M=0, Vm=0, U=0, Vn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF800000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `field N 7 +: 1`
/// Requirement: FieldBoundary { field: "N", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_vaddl_t1a1_a_field_n_1_max_100_ef800080() {
    // Thumb encoding (32): 0xEF800080
    // Test aarch32_VADDL_T1A1_A field N = 1 (Max)
    // ISET: T32
    // Fields: D=0, M=0, U=0, Vd=0, op=0, Vm=0, size=0, Vn=0, N=1
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF800080;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `field M 5 +: 1`
/// Requirement: FieldBoundary { field: "M", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_vaddl_t1a1_a_field_m_0_min_100_ef800000() {
    // Thumb encoding (32): 0xEF800000
    // Test aarch32_VADDL_T1A1_A field M = 0 (Min)
    // ISET: T32
    // Fields: D=0, op=0, Vd=0, Vn=0, U=0, M=0, Vm=0, size=0, N=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF800000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `field M 5 +: 1`
/// Requirement: FieldBoundary { field: "M", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_vaddl_t1a1_a_field_m_1_max_100_ef800020() {
    // Thumb encoding (32): 0xEF800020
    // Test aarch32_VADDL_T1A1_A field M = 1 (Max)
    // ISET: T32
    // Fields: Vn=0, size=0, M=1, Vm=0, U=0, D=0, Vd=0, op=0, N=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF800020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `field Vm 0 +: 4`
/// Requirement: FieldBoundary { field: "Vm", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_aarch32_vaddl_t1a1_a_field_vm_0_min_100_ef800000() {
    // Thumb encoding (32): 0xEF800000
    // Test aarch32_VADDL_T1A1_A field Vm = 0 (Min)
    // ISET: T32
    // Fields: M=0, Vm=0, Vn=0, U=0, D=0, N=0, size=0, Vd=0, op=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF800000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `field Vm 0 +: 4`
/// Requirement: FieldBoundary { field: "Vm", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_aarch32_vaddl_t1a1_a_field_vm_1_poweroftwo_100_ef800001() {
    // Thumb encoding (32): 0xEF800001
    // Test aarch32_VADDL_T1A1_A field Vm = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Vd=0, U=0, Vn=0, D=0, N=0, size=0, M=0, op=0, Vm=1
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF800001;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// U=0 (minimum value)
#[test]
fn test_aarch32_vaddl_t1a1_a_combo_0_100_ef800000() {
    // Thumb encoding (32): 0xEF800000
    // Test aarch32_VADDL_T1A1_A field combination: U=0, D=0, size=0, Vn=0, Vd=0, op=0, N=0, M=0, Vm=0
    // ISET: T32
    // Fields: Vm=0, U=0, size=0, D=0, Vd=0, Vn=0, N=0, op=0, M=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF800000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `field size = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "size", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch32_vaddl_t1a1_a_special_size_0_size_variant_0_256_ef800000() {
    // Thumb encoding (32): 0xEF800000
    // Test aarch32_VADDL_T1A1_A special value size = 0 (Size variant 0)
    // ISET: T32
    // Fields: size=0, D=0, Vd=0, op=0, M=0, Vm=0, Vn=0, N=0, U=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF800000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `field size = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "size", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch32_vaddl_t1a1_a_special_size_1_size_variant_1_256_ef900000() {
    // Thumb encoding (32): 0xEF900000
    // Test aarch32_VADDL_T1A1_A special value size = 1 (Size variant 1)
    // ISET: T32
    // Fields: Vn=0, size=1, M=0, Vm=0, Vd=0, D=0, N=0, op=0, U=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF900000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `field size = 2 (Size variant 2)`
/// Requirement: FieldSpecial { field: "size", value: 2, meaning: "Size variant 2" }
/// Size variant 2
#[test]
fn test_aarch32_vaddl_t1a1_a_special_size_2_size_variant_2_256_efa00000() {
    // Thumb encoding (32): 0xEFA00000
    // Test aarch32_VADDL_T1A1_A special value size = 2 (Size variant 2)
    // ISET: T32
    // Fields: size=2, D=0, N=0, U=0, Vn=0, op=0, M=0, Vd=0, Vm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEFA00000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `field size = 3 (Size variant 3)`
/// Requirement: FieldSpecial { field: "size", value: 3, meaning: "Size variant 3" }
/// Size variant 3
#[test]
fn test_aarch32_vaddl_t1a1_a_special_size_3_size_variant_3_256_efb00000() {
    // Thumb encoding (32): 0xEFB00000
    // Test aarch32_VADDL_T1A1_A special value size = 3 (Size variant 3)
    // ISET: T32
    // Fields: N=0, Vn=0, U=0, op=0, D=0, M=0, size=3, Vm=0, Vd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEFB00000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `Binary { op: Eq, lhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: "Vd" }), indices: [Single(LitInt(0))] }, rhs: Binary { op: Or, lhs: LitBits([true]), rhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "op" }), rhs: Binary { op: And, lhs: LitBits([true]), rhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: "Vn" }), indices: [Single(LitInt(0))] } } }, rhs: LitBits([true]) } } }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: \"Vd\" }), indices: [Single(LitInt(0))] }, rhs: Binary { op: Or, lhs: LitBits([true]), rhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"op\" }), rhs: Binary { op: And, lhs: LitBits([true]), rhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: \"Vn\" }), indices: [Single(LitInt(0))] } } }, rhs: LitBits([true]) } } }" }
/// triggers Undefined
#[test]
fn test_aarch32_vaddl_t1a1_a_invalid_0_100_ef800000() {
    // Thumb encoding (32): 0xEF800000
    // Test aarch32_VADDL_T1A1_A invalid encoding: Binary { op: Eq, lhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: "Vd" }), indices: [Single(LitInt(0))] }, rhs: Binary { op: Or, lhs: LitBits([true]), rhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "op" }), rhs: Binary { op: And, lhs: LitBits([true]), rhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: "Vn" }), indices: [Single(LitInt(0))] } } }, rhs: LitBits([true]) } } }
    // ISET: T32
    // Fields: size=0, U=0, Vd=0, N=0, Vm=0, op=0, D=0, Vn=0, M=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF800000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || !matches!(exit.unwrap(), CpuExit::Continue),
        "expected UNDEFINED for T32 encoding"
    );
}

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `Unconditional UNDEFINED`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNDEFINED" }
/// triggers Undefined
#[test]
fn test_aarch32_vaddl_t1a1_a_invalid_1_100_ef800000() {
    // Thumb encoding (32): 0xEF800000
    // Test aarch32_VADDL_T1A1_A invalid encoding: Unconditional UNDEFINED
    // ISET: T32
    // Fields: size=0, Vn=0, M=0, Vm=0, D=0, U=0, N=0, Vd=0, op=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF800000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || !matches!(exit.unwrap(), CpuExit::Continue),
        "expected UNDEFINED for T32 encoding"
    );
}

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `ADD X0, X1, #10`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// simple addition/subtraction (32)
#[test]
fn test_aarch32_vaddl_t1a1_a_add_oracle_32_0_f2802920() {
    // Test ADD 32-bit: simple addition/subtraction (with oracle verification)
    // Encoding: 0xF2802920
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x64);
    let encoding: u32 = 0xF2802920;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFF6,
        "X0 should be 0xFFFFFFFFFFFFFFF6"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `ADD X0, X1, #10`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// simple addition/subtraction (64)
#[test]
fn test_aarch32_vaddl_t1a1_a_add_oracle_64_0_f2802920() {
    // Test ADD 64-bit: simple addition/subtraction (with oracle verification)
    // Encoding: 0xF2802920
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x64);
    let encoding: u32 = 0xF2802920;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFF6,
        "X0 should be 0xFFFFFFFFFFFFFFF6"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `ADD X0, X1, #0`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// zero operands (32)
#[test]
fn test_aarch32_vaddl_t1a1_a_add_oracle_32_1_f2800120() {
    // Test ADD 32-bit: zero operands (with oracle verification)
    // Encoding: 0xF2800120
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2800120;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x0, "X0 should be 0x0000000000000000");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z flag should be true");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `ADD X0, X1, #0`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// zero operands (64)
#[test]
fn test_aarch32_vaddl_t1a1_a_add_oracle_64_1_f2800120() {
    // Test ADD 64-bit: zero operands (with oracle verification)
    // Encoding: 0xF2800120
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2800120;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x0, "X0 should be 0x0000000000000000");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z flag should be true");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `ADD X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// small values (32)
#[test]
fn test_aarch32_vaddl_t1a1_a_add_oracle_32_2_f2800520() {
    // Test ADD 32-bit: small values (with oracle verification)
    // Encoding: 0xF2800520
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x1);
    let encoding: u32 = 0xF2800520;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `ADD X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// small values (64)
#[test]
fn test_aarch32_vaddl_t1a1_a_add_oracle_64_2_f2800520() {
    // Test ADD 64-bit: small values (with oracle verification)
    // Encoding: 0xF2800520
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x1);
    let encoding: u32 = 0xF2800520;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `ADD X0, X1, #4095`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// max imm12 unshifted (32)
#[test]
fn test_aarch32_vaddl_t1a1_a_add_oracle_32_3_f2bffd20() {
    // Test ADD 32-bit: max imm12 unshifted (with oracle verification)
    // Encoding: 0xF2BFFD20
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2BFFD20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFF001,
        "X0 should be 0xFFFFFFFFFFFFF001"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `ADD X0, X1, #4095`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// max imm12 unshifted (64)
#[test]
fn test_aarch32_vaddl_t1a1_a_add_oracle_64_3_f2bffd20() {
    // Test ADD 64-bit: max imm12 unshifted (with oracle verification)
    // Encoding: 0xF2BFFD20
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2BFFD20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFF001,
        "X0 should be 0xFFFFFFFFFFFFF001"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `ADD X0, X1, #4095, LSL #12`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// max imm12 shifted (32)
#[test]
fn test_aarch32_vaddl_t1a1_a_add_oracle_32_4_f2fffd20() {
    // Test ADD 32-bit: max imm12 shifted (with oracle verification)
    // Encoding: 0xF2FFFD20
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2FFFD20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFF001000,
        "X0 should be 0xFFFFFFFFFF001000"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `ADD X0, X1, #4095, LSL #12`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// max imm12 shifted (64)
#[test]
fn test_aarch32_vaddl_t1a1_a_add_oracle_64_4_f2fffd20() {
    // Test ADD 64-bit: max imm12 shifted (with oracle verification)
    // Encoding: 0xF2FFFD20
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2FFFD20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFF001000,
        "X0 should be 0xFFFFFFFFFF001000"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `ADD X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// max u64 operand (32)
#[test]
fn test_aarch32_vaddl_t1a1_a_add_oracle_32_5_f2800520() {
    // Test ADD 32-bit: max u64 operand (with oracle verification)
    // Encoding: 0xF2800520
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800520;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `ADD X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// max u64 operand (64)
#[test]
fn test_aarch32_vaddl_t1a1_a_add_oracle_64_5_f2800520() {
    // Test ADD 64-bit: max u64 operand (with oracle verification)
    // Encoding: 0xF2800520
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800520;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `ADD X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// zero result (for sub 1-1) (32)
#[test]
fn test_aarch32_vaddl_t1a1_a_add_oracle_32_6_f2800520() {
    // Test ADD 32-bit: zero result (for sub 1-1) (with oracle verification)
    // Encoding: 0xF2800520
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2800520;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `ADD X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// zero result (for sub 1-1) (64)
#[test]
fn test_aarch32_vaddl_t1a1_a_add_oracle_64_6_f2800520() {
    // Test ADD 64-bit: zero result (for sub 1-1) (with oracle verification)
    // Encoding: 0xF2800520
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2800520;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `ADD X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// signed overflow boundary 64-bit (32)
#[test]
fn test_aarch32_vaddl_t1a1_a_add_oracle_32_7_f2800520() {
    // Test ADD 32-bit: signed overflow boundary 64-bit (with oracle verification)
    // Encoding: 0xF2800520
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800520;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `ADD X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// signed overflow boundary 64-bit (64)
#[test]
fn test_aarch32_vaddl_t1a1_a_add_oracle_64_7_f2800520() {
    // Test ADD 64-bit: signed overflow boundary 64-bit (with oracle verification)
    // Encoding: 0xF2800520
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800520;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `ADD X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// signed overflow boundary 32-bit (32)
#[test]
fn test_aarch32_vaddl_t1a1_a_add_oracle_32_8_f2800520() {
    // Test ADD 32-bit: signed overflow boundary 32-bit (with oracle verification)
    // Encoding: 0xF2800520
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x7FFFFFFF);
    let encoding: u32 = 0xF2800520;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `ADD X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// signed overflow boundary 32-bit (64)
#[test]
fn test_aarch32_vaddl_t1a1_a_add_oracle_64_8_f2800520() {
    // Test ADD 64-bit: signed overflow boundary 32-bit (with oracle verification)
    // Encoding: 0xF2800520
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x7FFFFFFF);
    let encoding: u32 = 0xF2800520;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `ADD X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// unsigned overflow 64-bit (32)
#[test]
fn test_aarch32_vaddl_t1a1_a_add_oracle_32_9_f2800520() {
    // Test ADD 32-bit: unsigned overflow 64-bit (with oracle verification)
    // Encoding: 0xF2800520
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800520;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `ADD X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// unsigned overflow 64-bit (64)
#[test]
fn test_aarch32_vaddl_t1a1_a_add_oracle_64_9_f2800520() {
    // Test ADD 64-bit: unsigned overflow 64-bit (with oracle verification)
    // Encoding: 0xF2800520
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800520;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `ADD X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// unsigned overflow 32-bit (32)
#[test]
fn test_aarch32_vaddl_t1a1_a_add_oracle_32_10_f2800520() {
    // Test ADD 32-bit: unsigned overflow 32-bit (with oracle verification)
    // Encoding: 0xF2800520
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800520;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `ADD X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// unsigned overflow 32-bit (64)
#[test]
fn test_aarch32_vaddl_t1a1_a_add_oracle_64_10_f2800520() {
    // Test ADD 64-bit: unsigned overflow 32-bit (with oracle verification)
    // Encoding: 0xF2800520
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800520;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `ADD SP, X1, #10`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "writes to stack pointer" }
/// SP destination (32)
#[test]
fn test_aarch32_vaddl_t1a1_a_add_oracle_32_rd31_sp_f280293f() {
    // Test ADD 32-bit with Rd=31 (SP)
    // Encoding: 0xF280293F
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x64);
    let encoding: u32 = 0xF280293F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `ADD SP, X1, #10`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "writes to stack pointer" }
/// SP destination (64)
#[test]
fn test_aarch32_vaddl_t1a1_a_add_oracle_64_rd31_sp_f280293f() {
    // Test ADD 64-bit with Rd=31 (SP)
    // Encoding: 0xF280293F
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x64);
    let encoding: u32 = 0xF280293F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `ADDS X0, X1, #10`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// simple addition/subtraction (32)
#[test]
fn test_aarch32_vaddl_t1a1_a_adds_oracle_32_0_f2802920() {
    // Test ADDS 32-bit: simple addition/subtraction (with oracle verification)
    // Encoding: 0xF2802920
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x64);
    let encoding: u32 = 0xF2802920;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFF6,
        "X0 should be 0xFFFFFFFFFFFFFFF6"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `ADDS X0, X1, #10`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// simple addition/subtraction (64)
#[test]
fn test_aarch32_vaddl_t1a1_a_adds_oracle_64_0_f2802920() {
    // Test ADDS 64-bit: simple addition/subtraction (with oracle verification)
    // Encoding: 0xF2802920
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x64);
    let encoding: u32 = 0xF2802920;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFF6,
        "X0 should be 0xFFFFFFFFFFFFFFF6"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `ADDS X0, X1, #0`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// zero operands (32)
#[test]
fn test_aarch32_vaddl_t1a1_a_adds_oracle_32_1_f2800120() {
    // Test ADDS 32-bit: zero operands (with oracle verification)
    // Encoding: 0xF2800120
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2800120;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x0, "X0 should be 0x0000000000000000");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z flag should be true");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `ADDS X0, X1, #0`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// zero operands (64)
#[test]
fn test_aarch32_vaddl_t1a1_a_adds_oracle_64_1_f2800120() {
    // Test ADDS 64-bit: zero operands (with oracle verification)
    // Encoding: 0xF2800120
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2800120;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x0, "X0 should be 0x0000000000000000");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z flag should be true");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `ADDS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// small values (32)
#[test]
fn test_aarch32_vaddl_t1a1_a_adds_oracle_32_2_f2800520() {
    // Test ADDS 32-bit: small values (with oracle verification)
    // Encoding: 0xF2800520
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x1);
    let encoding: u32 = 0xF2800520;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `ADDS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// small values (64)
#[test]
fn test_aarch32_vaddl_t1a1_a_adds_oracle_64_2_f2800520() {
    // Test ADDS 64-bit: small values (with oracle verification)
    // Encoding: 0xF2800520
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x1);
    let encoding: u32 = 0xF2800520;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `ADDS X0, X1, #4095`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// max imm12 unshifted (32)
#[test]
fn test_aarch32_vaddl_t1a1_a_adds_oracle_32_3_f2bffd20() {
    // Test ADDS 32-bit: max imm12 unshifted (with oracle verification)
    // Encoding: 0xF2BFFD20
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2BFFD20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFF001,
        "X0 should be 0xFFFFFFFFFFFFF001"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `ADDS X0, X1, #4095`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// max imm12 unshifted (64)
#[test]
fn test_aarch32_vaddl_t1a1_a_adds_oracle_64_3_f2bffd20() {
    // Test ADDS 64-bit: max imm12 unshifted (with oracle verification)
    // Encoding: 0xF2BFFD20
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2BFFD20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFF001,
        "X0 should be 0xFFFFFFFFFFFFF001"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `ADDS X0, X1, #4095, LSL #12`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// max imm12 shifted (32)
#[test]
fn test_aarch32_vaddl_t1a1_a_adds_oracle_32_4_f2fffd20() {
    // Test ADDS 32-bit: max imm12 shifted (with oracle verification)
    // Encoding: 0xF2FFFD20
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2FFFD20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFF001000,
        "X0 should be 0xFFFFFFFFFF001000"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `ADDS X0, X1, #4095, LSL #12`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// max imm12 shifted (64)
#[test]
fn test_aarch32_vaddl_t1a1_a_adds_oracle_64_4_f2fffd20() {
    // Test ADDS 64-bit: max imm12 shifted (with oracle verification)
    // Encoding: 0xF2FFFD20
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2FFFD20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFF001000,
        "X0 should be 0xFFFFFFFFFF001000"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `ADDS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// max u64 operand (32)
#[test]
fn test_aarch32_vaddl_t1a1_a_adds_oracle_32_5_f2800520() {
    // Test ADDS 32-bit: max u64 operand (with oracle verification)
    // Encoding: 0xF2800520
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800520;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `ADDS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// max u64 operand (64)
#[test]
fn test_aarch32_vaddl_t1a1_a_adds_oracle_64_5_f2800520() {
    // Test ADDS 64-bit: max u64 operand (with oracle verification)
    // Encoding: 0xF2800520
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800520;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `ADDS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// zero result (for sub 1-1) (32)
#[test]
fn test_aarch32_vaddl_t1a1_a_adds_oracle_32_6_f2800520() {
    // Test ADDS 32-bit: zero result (for sub 1-1) (with oracle verification)
    // Encoding: 0xF2800520
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2800520;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `ADDS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// zero result (for sub 1-1) (64)
#[test]
fn test_aarch32_vaddl_t1a1_a_adds_oracle_64_6_f2800520() {
    // Test ADDS 64-bit: zero result (for sub 1-1) (with oracle verification)
    // Encoding: 0xF2800520
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2800520;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `ADDS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// signed overflow boundary 64-bit (32)
#[test]
fn test_aarch32_vaddl_t1a1_a_adds_oracle_32_7_f2800520() {
    // Test ADDS 32-bit: signed overflow boundary 64-bit (with oracle verification)
    // Encoding: 0xF2800520
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800520;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `ADDS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// signed overflow boundary 64-bit (64)
#[test]
fn test_aarch32_vaddl_t1a1_a_adds_oracle_64_7_f2800520() {
    // Test ADDS 64-bit: signed overflow boundary 64-bit (with oracle verification)
    // Encoding: 0xF2800520
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800520;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `ADDS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// signed overflow boundary 32-bit (32)
#[test]
fn test_aarch32_vaddl_t1a1_a_adds_oracle_32_8_f2800520() {
    // Test ADDS 32-bit: signed overflow boundary 32-bit (with oracle verification)
    // Encoding: 0xF2800520
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x7FFFFFFF);
    let encoding: u32 = 0xF2800520;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `ADDS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// signed overflow boundary 32-bit (64)
#[test]
fn test_aarch32_vaddl_t1a1_a_adds_oracle_64_8_f2800520() {
    // Test ADDS 64-bit: signed overflow boundary 32-bit (with oracle verification)
    // Encoding: 0xF2800520
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x7FFFFFFF);
    let encoding: u32 = 0xF2800520;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `ADDS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// unsigned overflow 64-bit (32)
#[test]
fn test_aarch32_vaddl_t1a1_a_adds_oracle_32_9_f2800520() {
    // Test ADDS 32-bit: unsigned overflow 64-bit (with oracle verification)
    // Encoding: 0xF2800520
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800520;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `ADDS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// unsigned overflow 64-bit (64)
#[test]
fn test_aarch32_vaddl_t1a1_a_adds_oracle_64_9_f2800520() {
    // Test ADDS 64-bit: unsigned overflow 64-bit (with oracle verification)
    // Encoding: 0xF2800520
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800520;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `ADDS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// unsigned overflow 32-bit (32)
#[test]
fn test_aarch32_vaddl_t1a1_a_adds_oracle_32_10_f2800520() {
    // Test ADDS 32-bit: unsigned overflow 32-bit (with oracle verification)
    // Encoding: 0xF2800520
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800520;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `ADDS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// unsigned overflow 32-bit (64)
#[test]
fn test_aarch32_vaddl_t1a1_a_adds_oracle_64_10_f2800520() {
    // Test ADDS 64-bit: unsigned overflow 32-bit (with oracle verification)
    // Encoding: 0xF2800520
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800520;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `ADDS ZR, X1, #10`
/// Requirement: RegisterSpecial { reg: Zr, behavior: "result discarded, flags set" }
/// ZR destination (32)
#[test]
fn test_aarch32_vaddl_t1a1_a_adds_oracle_32_rd31_zr_f280293f() {
    // Test ADDS 32-bit with Rd=31 (ZR)
    // Encoding: 0xF280293F
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x64);
    let encoding: u32 = 0xF280293F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `ADDS ZR, X1, #10`
/// Requirement: RegisterSpecial { reg: Zr, behavior: "result discarded, flags set" }
/// ZR destination (64)
#[test]
fn test_aarch32_vaddl_t1a1_a_adds_oracle_64_rd31_zr_f280293f() {
    // Test ADDS 64-bit with Rd=31 (ZR)
    // Encoding: 0xF280293F
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x64);
    let encoding: u32 = 0xF280293F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `SUB X0, X1, #10`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// simple addition/subtraction (32)
#[test]
fn test_aarch32_vaddl_t1a1_a_sub_oracle_32_0_f2802920() {
    // Test SUB 32-bit: simple addition/subtraction (with oracle verification)
    // Encoding: 0xF2802920
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x64);
    let encoding: u32 = 0xF2802920;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFF6,
        "X0 should be 0xFFFFFFFFFFFFFFF6"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `SUB X0, X1, #10`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// simple addition/subtraction (64)
#[test]
fn test_aarch32_vaddl_t1a1_a_sub_oracle_64_0_f2802920() {
    // Test SUB 64-bit: simple addition/subtraction (with oracle verification)
    // Encoding: 0xF2802920
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x64);
    let encoding: u32 = 0xF2802920;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFF6,
        "X0 should be 0xFFFFFFFFFFFFFFF6"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `SUB X0, X1, #0`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// zero operands (32)
#[test]
fn test_aarch32_vaddl_t1a1_a_sub_oracle_32_1_f2800120() {
    // Test SUB 32-bit: zero operands (with oracle verification)
    // Encoding: 0xF2800120
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2800120;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x0, "X0 should be 0x0000000000000000");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z flag should be true");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `SUB X0, X1, #0`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// zero operands (64)
#[test]
fn test_aarch32_vaddl_t1a1_a_sub_oracle_64_1_f2800120() {
    // Test SUB 64-bit: zero operands (with oracle verification)
    // Encoding: 0xF2800120
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2800120;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x0, "X0 should be 0x0000000000000000");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z flag should be true");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `SUB X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// small values (32)
#[test]
fn test_aarch32_vaddl_t1a1_a_sub_oracle_32_2_f2800520() {
    // Test SUB 32-bit: small values (with oracle verification)
    // Encoding: 0xF2800520
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x1);
    let encoding: u32 = 0xF2800520;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `SUB X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// small values (64)
#[test]
fn test_aarch32_vaddl_t1a1_a_sub_oracle_64_2_f2800520() {
    // Test SUB 64-bit: small values (with oracle verification)
    // Encoding: 0xF2800520
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x1);
    let encoding: u32 = 0xF2800520;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `SUB X0, X1, #4095`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// max imm12 unshifted (32)
#[test]
fn test_aarch32_vaddl_t1a1_a_sub_oracle_32_3_f2bffd20() {
    // Test SUB 32-bit: max imm12 unshifted (with oracle verification)
    // Encoding: 0xF2BFFD20
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2BFFD20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFF001,
        "X0 should be 0xFFFFFFFFFFFFF001"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `SUB X0, X1, #4095`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// max imm12 unshifted (64)
#[test]
fn test_aarch32_vaddl_t1a1_a_sub_oracle_64_3_f2bffd20() {
    // Test SUB 64-bit: max imm12 unshifted (with oracle verification)
    // Encoding: 0xF2BFFD20
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2BFFD20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFF001,
        "X0 should be 0xFFFFFFFFFFFFF001"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `SUB X0, X1, #4095, LSL #12`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// max imm12 shifted (32)
#[test]
fn test_aarch32_vaddl_t1a1_a_sub_oracle_32_4_f2fffd20() {
    // Test SUB 32-bit: max imm12 shifted (with oracle verification)
    // Encoding: 0xF2FFFD20
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2FFFD20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFF001000,
        "X0 should be 0xFFFFFFFFFF001000"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `SUB X0, X1, #4095, LSL #12`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// max imm12 shifted (64)
#[test]
fn test_aarch32_vaddl_t1a1_a_sub_oracle_64_4_f2fffd20() {
    // Test SUB 64-bit: max imm12 shifted (with oracle verification)
    // Encoding: 0xF2FFFD20
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2FFFD20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFF001000,
        "X0 should be 0xFFFFFFFFFF001000"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `SUB X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// max u64 operand (32)
#[test]
fn test_aarch32_vaddl_t1a1_a_sub_oracle_32_5_f2800520() {
    // Test SUB 32-bit: max u64 operand (with oracle verification)
    // Encoding: 0xF2800520
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800520;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `SUB X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// max u64 operand (64)
#[test]
fn test_aarch32_vaddl_t1a1_a_sub_oracle_64_5_f2800520() {
    // Test SUB 64-bit: max u64 operand (with oracle verification)
    // Encoding: 0xF2800520
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800520;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `SUB X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// zero result (for sub 1-1) (32)
#[test]
fn test_aarch32_vaddl_t1a1_a_sub_oracle_32_6_f2800520() {
    // Test SUB 32-bit: zero result (for sub 1-1) (with oracle verification)
    // Encoding: 0xF2800520
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2800520;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `SUB X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// zero result (for sub 1-1) (64)
#[test]
fn test_aarch32_vaddl_t1a1_a_sub_oracle_64_6_f2800520() {
    // Test SUB 64-bit: zero result (for sub 1-1) (with oracle verification)
    // Encoding: 0xF2800520
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2800520;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `SUB X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// signed overflow boundary 64-bit (32)
#[test]
fn test_aarch32_vaddl_t1a1_a_sub_oracle_32_7_f2800520() {
    // Test SUB 32-bit: signed overflow boundary 64-bit (with oracle verification)
    // Encoding: 0xF2800520
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800520;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `SUB X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// signed overflow boundary 64-bit (64)
#[test]
fn test_aarch32_vaddl_t1a1_a_sub_oracle_64_7_f2800520() {
    // Test SUB 64-bit: signed overflow boundary 64-bit (with oracle verification)
    // Encoding: 0xF2800520
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800520;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `SUB X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// signed overflow boundary 32-bit (32)
#[test]
fn test_aarch32_vaddl_t1a1_a_sub_oracle_32_8_f2800520() {
    // Test SUB 32-bit: signed overflow boundary 32-bit (with oracle verification)
    // Encoding: 0xF2800520
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x7FFFFFFF);
    let encoding: u32 = 0xF2800520;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `SUB X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// signed overflow boundary 32-bit (64)
#[test]
fn test_aarch32_vaddl_t1a1_a_sub_oracle_64_8_f2800520() {
    // Test SUB 64-bit: signed overflow boundary 32-bit (with oracle verification)
    // Encoding: 0xF2800520
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x7FFFFFFF);
    let encoding: u32 = 0xF2800520;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `SUB X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// unsigned overflow 64-bit (32)
#[test]
fn test_aarch32_vaddl_t1a1_a_sub_oracle_32_9_f2800520() {
    // Test SUB 32-bit: unsigned overflow 64-bit (with oracle verification)
    // Encoding: 0xF2800520
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800520;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `SUB X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// unsigned overflow 64-bit (64)
#[test]
fn test_aarch32_vaddl_t1a1_a_sub_oracle_64_9_f2800520() {
    // Test SUB 64-bit: unsigned overflow 64-bit (with oracle verification)
    // Encoding: 0xF2800520
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800520;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `SUB X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// unsigned overflow 32-bit (32)
#[test]
fn test_aarch32_vaddl_t1a1_a_sub_oracle_32_10_f2800520() {
    // Test SUB 32-bit: unsigned overflow 32-bit (with oracle verification)
    // Encoding: 0xF2800520
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800520;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `SUB X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// unsigned overflow 32-bit (64)
#[test]
fn test_aarch32_vaddl_t1a1_a_sub_oracle_64_10_f2800520() {
    // Test SUB 64-bit: unsigned overflow 32-bit (with oracle verification)
    // Encoding: 0xF2800520
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800520;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `SUB SP, X1, #10`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "writes to stack pointer" }
/// SP destination (32)
#[test]
fn test_aarch32_vaddl_t1a1_a_sub_oracle_32_rd31_sp_f280293f() {
    // Test SUB 32-bit with Rd=31 (SP)
    // Encoding: 0xF280293F
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x64);
    let encoding: u32 = 0xF280293F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `SUB SP, X1, #10`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "writes to stack pointer" }
/// SP destination (64)
#[test]
fn test_aarch32_vaddl_t1a1_a_sub_oracle_64_rd31_sp_f280293f() {
    // Test SUB 64-bit with Rd=31 (SP)
    // Encoding: 0xF280293F
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x64);
    let encoding: u32 = 0xF280293F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `SUBS X0, X1, #10`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// simple addition/subtraction (32)
#[test]
fn test_aarch32_vaddl_t1a1_a_subs_oracle_32_0_f2802920() {
    // Test SUBS 32-bit: simple addition/subtraction (with oracle verification)
    // Encoding: 0xF2802920
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x64);
    let encoding: u32 = 0xF2802920;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFF6,
        "X0 should be 0xFFFFFFFFFFFFFFF6"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `SUBS X0, X1, #10`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// simple addition/subtraction (64)
#[test]
fn test_aarch32_vaddl_t1a1_a_subs_oracle_64_0_f2802920() {
    // Test SUBS 64-bit: simple addition/subtraction (with oracle verification)
    // Encoding: 0xF2802920
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x64);
    let encoding: u32 = 0xF2802920;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFF6,
        "X0 should be 0xFFFFFFFFFFFFFFF6"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `SUBS X0, X1, #0`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// zero operands (32)
#[test]
fn test_aarch32_vaddl_t1a1_a_subs_oracle_32_1_f2800120() {
    // Test SUBS 32-bit: zero operands (with oracle verification)
    // Encoding: 0xF2800120
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2800120;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x0, "X0 should be 0x0000000000000000");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z flag should be true");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `SUBS X0, X1, #0`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// zero operands (64)
#[test]
fn test_aarch32_vaddl_t1a1_a_subs_oracle_64_1_f2800120() {
    // Test SUBS 64-bit: zero operands (with oracle verification)
    // Encoding: 0xF2800120
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2800120;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x0, "X0 should be 0x0000000000000000");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z flag should be true");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `SUBS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// small values (32)
#[test]
fn test_aarch32_vaddl_t1a1_a_subs_oracle_32_2_f2800520() {
    // Test SUBS 32-bit: small values (with oracle verification)
    // Encoding: 0xF2800520
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x1);
    let encoding: u32 = 0xF2800520;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `SUBS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// small values (64)
#[test]
fn test_aarch32_vaddl_t1a1_a_subs_oracle_64_2_f2800520() {
    // Test SUBS 64-bit: small values (with oracle verification)
    // Encoding: 0xF2800520
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x1);
    let encoding: u32 = 0xF2800520;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `SUBS X0, X1, #4095`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// max imm12 unshifted (32)
#[test]
fn test_aarch32_vaddl_t1a1_a_subs_oracle_32_3_f2bffd20() {
    // Test SUBS 32-bit: max imm12 unshifted (with oracle verification)
    // Encoding: 0xF2BFFD20
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2BFFD20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFF001,
        "X0 should be 0xFFFFFFFFFFFFF001"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `SUBS X0, X1, #4095`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// max imm12 unshifted (64)
#[test]
fn test_aarch32_vaddl_t1a1_a_subs_oracle_64_3_f2bffd20() {
    // Test SUBS 64-bit: max imm12 unshifted (with oracle verification)
    // Encoding: 0xF2BFFD20
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2BFFD20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFF001,
        "X0 should be 0xFFFFFFFFFFFFF001"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `SUBS X0, X1, #4095, LSL #12`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// max imm12 shifted (32)
#[test]
fn test_aarch32_vaddl_t1a1_a_subs_oracle_32_4_f2fffd20() {
    // Test SUBS 32-bit: max imm12 shifted (with oracle verification)
    // Encoding: 0xF2FFFD20
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2FFFD20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFF001000,
        "X0 should be 0xFFFFFFFFFF001000"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `SUBS X0, X1, #4095, LSL #12`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// max imm12 shifted (64)
#[test]
fn test_aarch32_vaddl_t1a1_a_subs_oracle_64_4_f2fffd20() {
    // Test SUBS 64-bit: max imm12 shifted (with oracle verification)
    // Encoding: 0xF2FFFD20
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2FFFD20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFF001000,
        "X0 should be 0xFFFFFFFFFF001000"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `SUBS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// max u64 operand (32)
#[test]
fn test_aarch32_vaddl_t1a1_a_subs_oracle_32_5_f2800520() {
    // Test SUBS 32-bit: max u64 operand (with oracle verification)
    // Encoding: 0xF2800520
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800520;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `SUBS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// max u64 operand (64)
#[test]
fn test_aarch32_vaddl_t1a1_a_subs_oracle_64_5_f2800520() {
    // Test SUBS 64-bit: max u64 operand (with oracle verification)
    // Encoding: 0xF2800520
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800520;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `SUBS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// zero result (for sub 1-1) (32)
#[test]
fn test_aarch32_vaddl_t1a1_a_subs_oracle_32_6_f2800520() {
    // Test SUBS 32-bit: zero result (for sub 1-1) (with oracle verification)
    // Encoding: 0xF2800520
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2800520;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `SUBS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// zero result (for sub 1-1) (64)
#[test]
fn test_aarch32_vaddl_t1a1_a_subs_oracle_64_6_f2800520() {
    // Test SUBS 64-bit: zero result (for sub 1-1) (with oracle verification)
    // Encoding: 0xF2800520
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2800520;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `SUBS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// signed overflow boundary 64-bit (32)
#[test]
fn test_aarch32_vaddl_t1a1_a_subs_oracle_32_7_f2800520() {
    // Test SUBS 32-bit: signed overflow boundary 64-bit (with oracle verification)
    // Encoding: 0xF2800520
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800520;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `SUBS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// signed overflow boundary 64-bit (64)
#[test]
fn test_aarch32_vaddl_t1a1_a_subs_oracle_64_7_f2800520() {
    // Test SUBS 64-bit: signed overflow boundary 64-bit (with oracle verification)
    // Encoding: 0xF2800520
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800520;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `SUBS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// signed overflow boundary 32-bit (32)
#[test]
fn test_aarch32_vaddl_t1a1_a_subs_oracle_32_8_f2800520() {
    // Test SUBS 32-bit: signed overflow boundary 32-bit (with oracle verification)
    // Encoding: 0xF2800520
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x7FFFFFFF);
    let encoding: u32 = 0xF2800520;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `SUBS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// signed overflow boundary 32-bit (64)
#[test]
fn test_aarch32_vaddl_t1a1_a_subs_oracle_64_8_f2800520() {
    // Test SUBS 64-bit: signed overflow boundary 32-bit (with oracle verification)
    // Encoding: 0xF2800520
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x7FFFFFFF);
    let encoding: u32 = 0xF2800520;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `SUBS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// unsigned overflow 64-bit (32)
#[test]
fn test_aarch32_vaddl_t1a1_a_subs_oracle_32_9_f2800520() {
    // Test SUBS 32-bit: unsigned overflow 64-bit (with oracle verification)
    // Encoding: 0xF2800520
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800520;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `SUBS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// unsigned overflow 64-bit (64)
#[test]
fn test_aarch32_vaddl_t1a1_a_subs_oracle_64_9_f2800520() {
    // Test SUBS 64-bit: unsigned overflow 64-bit (with oracle verification)
    // Encoding: 0xF2800520
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800520;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `SUBS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// unsigned overflow 32-bit (32)
#[test]
fn test_aarch32_vaddl_t1a1_a_subs_oracle_32_10_f2800520() {
    // Test SUBS 32-bit: unsigned overflow 32-bit (with oracle verification)
    // Encoding: 0xF2800520
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800520;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `SUBS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// unsigned overflow 32-bit (64)
#[test]
fn test_aarch32_vaddl_t1a1_a_subs_oracle_64_10_f2800520() {
    // Test SUBS 64-bit: unsigned overflow 32-bit (with oracle verification)
    // Encoding: 0xF2800520
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800520;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `SUBS ZR, X1, #10`
/// Requirement: RegisterSpecial { reg: Zr, behavior: "result discarded, flags set" }
/// ZR destination (32)
#[test]
fn test_aarch32_vaddl_t1a1_a_subs_oracle_32_rd31_zr_f280293f() {
    // Test SUBS 32-bit with Rd=31 (ZR)
    // Encoding: 0xF280293F
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x64);
    let encoding: u32 = 0xF280293F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `SUBS ZR, X1, #10`
/// Requirement: RegisterSpecial { reg: Zr, behavior: "result discarded, flags set" }
/// ZR destination (64)
#[test]
fn test_aarch32_vaddl_t1a1_a_subs_oracle_64_rd31_zr_f280293f() {
    // Test SUBS 64-bit with Rd=31 (ZR)
    // Encoding: 0xF280293F
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x64);
    let encoding: u32 = 0xF280293F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `ADD R0, R1, #10`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// small immediate (Rn=0x00000000)
#[test]
fn test_aarch32_vaddl_t1a1_a_a32_add_sub_imm_0_0_f281000a() {
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

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `ADD R0, R1, #10`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// small immediate (Rn=0x00000064)
#[test]
fn test_aarch32_vaddl_t1a1_a_a32_add_sub_imm_0_64_f281000a() {
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

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `ADD R0, R1, #10`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// small immediate (Rn=0x7FFFFFFF)
#[test]
fn test_aarch32_vaddl_t1a1_a_a32_add_sub_imm_0_7fffffff_f281000a() {
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

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `ADD R0, R1, #10`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// small immediate (Rn=0x80000000)
#[test]
fn test_aarch32_vaddl_t1a1_a_a32_add_sub_imm_0_80000000_f281000a() {
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

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `ADD R0, R1, #10`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// small immediate (Rn=0xFFFFFFFF)
#[test]
fn test_aarch32_vaddl_t1a1_a_a32_add_sub_imm_0_ffffffff_f281000a() {
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

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `ADD R0, R1, #255`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// max imm8 (Rn=0x00000000)
#[test]
fn test_aarch32_vaddl_t1a1_a_a32_add_sub_imm_1_0_f28100ff() {
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

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `ADD R0, R1, #255`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// max imm8 (Rn=0x00000064)
#[test]
fn test_aarch32_vaddl_t1a1_a_a32_add_sub_imm_1_64_f28100ff() {
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

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `ADD R0, R1, #255`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// max imm8 (Rn=0x7FFFFFFF)
#[test]
fn test_aarch32_vaddl_t1a1_a_a32_add_sub_imm_1_7fffffff_f28100ff() {
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

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `ADD R0, R1, #255`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// max imm8 (Rn=0x80000000)
#[test]
fn test_aarch32_vaddl_t1a1_a_a32_add_sub_imm_1_80000000_f28100ff() {
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

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `ADD R0, R1, #255`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// max imm8 (Rn=0xFFFFFFFF)
#[test]
fn test_aarch32_vaddl_t1a1_a_a32_add_sub_imm_1_ffffffff_f28100ff() {
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

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `ADD R0, R1, #32`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// rotated by 2 (Rn=0x00000000)
#[test]
fn test_aarch32_vaddl_t1a1_a_a32_add_sub_imm_2_0_f2810180() {
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

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `ADD R0, R1, #32`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// rotated by 2 (Rn=0x00000064)
#[test]
fn test_aarch32_vaddl_t1a1_a_a32_add_sub_imm_2_64_f2810180() {
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

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `ADD R0, R1, #32`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// rotated by 2 (Rn=0x7FFFFFFF)
#[test]
fn test_aarch32_vaddl_t1a1_a_a32_add_sub_imm_2_7fffffff_f2810180() {
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

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `ADD R0, R1, #32`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// rotated by 2 (Rn=0x80000000)
#[test]
fn test_aarch32_vaddl_t1a1_a_a32_add_sub_imm_2_80000000_f2810180() {
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

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `ADD R0, R1, #32`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// rotated by 2 (Rn=0xFFFFFFFF)
#[test]
fn test_aarch32_vaddl_t1a1_a_a32_add_sub_imm_2_ffffffff_f2810180() {
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

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `ADD R0, R1, #251658240`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// rotated by 8 (Rn=0x00000000)
#[test]
fn test_aarch32_vaddl_t1a1_a_a32_add_sub_imm_3_0_f281040f() {
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

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `ADD R0, R1, #251658240`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// rotated by 8 (Rn=0x00000064)
#[test]
fn test_aarch32_vaddl_t1a1_a_a32_add_sub_imm_3_64_f281040f() {
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

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `ADD R0, R1, #251658240`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// rotated by 8 (Rn=0x7FFFFFFF)
#[test]
fn test_aarch32_vaddl_t1a1_a_a32_add_sub_imm_3_7fffffff_f281040f() {
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

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `ADD R0, R1, #251658240`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// rotated by 8 (Rn=0x80000000)
#[test]
fn test_aarch32_vaddl_t1a1_a_a32_add_sub_imm_3_80000000_f281040f() {
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

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `ADD R0, R1, #251658240`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// rotated by 8 (Rn=0xFFFFFFFF)
#[test]
fn test_aarch32_vaddl_t1a1_a_a32_add_sub_imm_3_ffffffff_f281040f() {
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

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `ADD R0, R1, #0`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// zero immediate (Rn=0x00000000)
#[test]
fn test_aarch32_vaddl_t1a1_a_a32_add_sub_imm_4_0_f2810000() {
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

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `ADD R0, R1, #0`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// zero immediate (Rn=0x00000064)
#[test]
fn test_aarch32_vaddl_t1a1_a_a32_add_sub_imm_4_64_f2810000() {
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

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `ADD R0, R1, #0`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// zero immediate (Rn=0x7FFFFFFF)
#[test]
fn test_aarch32_vaddl_t1a1_a_a32_add_sub_imm_4_7fffffff_f2810000() {
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

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `ADD R0, R1, #0`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// zero immediate (Rn=0x80000000)
#[test]
fn test_aarch32_vaddl_t1a1_a_a32_add_sub_imm_4_80000000_f2810000() {
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

/// Provenance: aarch32_VADDL_T1A1_A
/// ASL: `ADD R0, R1, #0`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// zero immediate (Rn=0xFFFFFFFF)
#[test]
fn test_aarch32_vaddl_t1a1_a_a32_add_sub_imm_4_ffffffff_f2810000() {
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
// aarch32_VSUB_i_A Tests
// ============================================================================

/// Provenance: aarch32_VSUB_i_T1A1_A
/// ASL: `field D 22 +: 1`
/// Requirement: FieldBoundary { field: "D", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_vsub_i_t1a1_a_field_d_0_min_800_f3000800() {
    // Encoding: 0xF3000800
    // Test aarch32_VSUB_i_T1A1_A field D = 0 (Min)
    // ISET: A32
    // Fields: D=0, Vn=0, M=0, N=0, Q=0, Vd=0, size=0, Vm=0
    let encoding: u32 = 0xF3000800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VSUB_i_T1A1_A
/// ASL: `field D 22 +: 1`
/// Requirement: FieldBoundary { field: "D", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_vsub_i_t1a1_a_field_d_1_max_800_f3400800() {
    // Encoding: 0xF3400800
    // Test aarch32_VSUB_i_T1A1_A field D = 1 (Max)
    // ISET: A32
    // Fields: Vd=0, Q=0, Vm=0, M=0, size=0, D=1, Vn=0, N=0
    let encoding: u32 = 0xF3400800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VSUB_i_T1A1_A
/// ASL: `field size 20 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch32_vsub_i_t1a1_a_field_size_0_min_800_f3000800() {
    // Encoding: 0xF3000800
    // Test aarch32_VSUB_i_T1A1_A field size = 0 (Min)
    // ISET: A32
    // Fields: D=0, M=0, Vd=0, N=0, Q=0, Vm=0, size=0, Vn=0
    let encoding: u32 = 0xF3000800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VSUB_i_T1A1_A
/// ASL: `field size 20 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 1, boundary: PowerOfTwo }
/// 16-bit / halfword size
#[test]
fn test_aarch32_vsub_i_t1a1_a_field_size_1_poweroftwo_800_f3100800() {
    // Encoding: 0xF3100800
    // Test aarch32_VSUB_i_T1A1_A field size = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Vd=0, N=0, Vm=0, M=0, Vn=0, Q=0, size=1, D=0
    let encoding: u32 = 0xF3100800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VSUB_i_T1A1_A
/// ASL: `field size 20 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 2, boundary: PowerOfTwo }
/// 32-bit / word size
#[test]
fn test_aarch32_vsub_i_t1a1_a_field_size_2_poweroftwo_800_f3200800() {
    // Encoding: 0xF3200800
    // Test aarch32_VSUB_i_T1A1_A field size = 2 (PowerOfTwo)
    // ISET: A32
    // Fields: N=0, Q=0, Vm=0, Vn=0, M=0, D=0, Vd=0, size=2
    let encoding: u32 = 0xF3200800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VSUB_i_T1A1_A
/// ASL: `field size 20 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 3, boundary: Max }
/// 64-bit / doubleword size
#[test]
fn test_aarch32_vsub_i_t1a1_a_field_size_3_max_800_f3300800() {
    // Encoding: 0xF3300800
    // Test aarch32_VSUB_i_T1A1_A field size = 3 (Max)
    // ISET: A32
    // Fields: size=3, Vm=0, Vd=0, Vn=0, M=0, D=0, N=0, Q=0
    let encoding: u32 = 0xF3300800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VSUB_i_T1A1_A
/// ASL: `field Vn 16 +: 4`
/// Requirement: FieldBoundary { field: "Vn", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_aarch32_vsub_i_t1a1_a_field_vn_0_min_800_f3000800() {
    // Encoding: 0xF3000800
    // Test aarch32_VSUB_i_T1A1_A field Vn = 0 (Min)
    // ISET: A32
    // Fields: Vd=0, Vm=0, size=0, N=0, Q=0, M=0, D=0, Vn=0
    let encoding: u32 = 0xF3000800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VSUB_i_T1A1_A
/// ASL: `field Vn 16 +: 4`
/// Requirement: FieldBoundary { field: "Vn", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_aarch32_vsub_i_t1a1_a_field_vn_1_poweroftwo_800_f3010800() {
    // Encoding: 0xF3010800
    // Test aarch32_VSUB_i_T1A1_A field Vn = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Vd=0, Vn=1, Q=0, M=0, D=0, N=0, size=0, Vm=0
    let encoding: u32 = 0xF3010800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VSUB_i_T1A1_A
/// ASL: `field Vd 12 +: 4`
/// Requirement: FieldBoundary { field: "Vd", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_aarch32_vsub_i_t1a1_a_field_vd_0_min_800_f3000800() {
    // Encoding: 0xF3000800
    // Test aarch32_VSUB_i_T1A1_A field Vd = 0 (Min)
    // ISET: A32
    // Fields: M=0, Vd=0, Vm=0, D=0, Vn=0, Q=0, size=0, N=0
    let encoding: u32 = 0xF3000800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VSUB_i_T1A1_A
/// ASL: `field Vd 12 +: 4`
/// Requirement: FieldBoundary { field: "Vd", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_aarch32_vsub_i_t1a1_a_field_vd_1_poweroftwo_800_f3001800() {
    // Encoding: 0xF3001800
    // Test aarch32_VSUB_i_T1A1_A field Vd = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: M=0, N=0, Vd=1, Q=0, Vm=0, size=0, Vn=0, D=0
    let encoding: u32 = 0xF3001800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VSUB_i_T1A1_A
/// ASL: `field N 7 +: 1`
/// Requirement: FieldBoundary { field: "N", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_vsub_i_t1a1_a_field_n_0_min_800_f3000800() {
    // Encoding: 0xF3000800
    // Test aarch32_VSUB_i_T1A1_A field N = 0 (Min)
    // ISET: A32
    // Fields: N=0, Q=0, Vm=0, Vn=0, D=0, M=0, size=0, Vd=0
    let encoding: u32 = 0xF3000800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VSUB_i_T1A1_A
/// ASL: `field N 7 +: 1`
/// Requirement: FieldBoundary { field: "N", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_vsub_i_t1a1_a_field_n_1_max_800_f3000880() {
    // Encoding: 0xF3000880
    // Test aarch32_VSUB_i_T1A1_A field N = 1 (Max)
    // ISET: A32
    // Fields: Vn=0, size=0, Q=0, D=0, M=0, Vd=0, Vm=0, N=1
    let encoding: u32 = 0xF3000880;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VSUB_i_T1A1_A
/// ASL: `field Q 6 +: 1`
/// Requirement: FieldBoundary { field: "Q", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch32_vsub_i_t1a1_a_field_q_0_min_800_f3000800() {
    // Encoding: 0xF3000800
    // Test aarch32_VSUB_i_T1A1_A field Q = 0 (Min)
    // ISET: A32
    // Fields: Vd=0, D=0, N=0, Vm=0, Vn=0, size=0, Q=0, M=0
    let encoding: u32 = 0xF3000800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VSUB_i_T1A1_A
/// ASL: `field Q 6 +: 1`
/// Requirement: FieldBoundary { field: "Q", value: 1, boundary: Max }
/// 16-bit / halfword size
#[test]
fn test_aarch32_vsub_i_t1a1_a_field_q_1_max_800_f3000840() {
    // Encoding: 0xF3000840
    // Test aarch32_VSUB_i_T1A1_A field Q = 1 (Max)
    // ISET: A32
    // Fields: Vd=0, Vm=0, M=0, Q=1, N=0, D=0, size=0, Vn=0
    let encoding: u32 = 0xF3000840;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VSUB_i_T1A1_A
/// ASL: `field M 5 +: 1`
/// Requirement: FieldBoundary { field: "M", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_vsub_i_t1a1_a_field_m_0_min_800_f3000800() {
    // Encoding: 0xF3000800
    // Test aarch32_VSUB_i_T1A1_A field M = 0 (Min)
    // ISET: A32
    // Fields: size=0, Vd=0, Q=0, Vm=0, D=0, M=0, Vn=0, N=0
    let encoding: u32 = 0xF3000800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VSUB_i_T1A1_A
/// ASL: `field M 5 +: 1`
/// Requirement: FieldBoundary { field: "M", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_vsub_i_t1a1_a_field_m_1_max_800_f3000820() {
    // Encoding: 0xF3000820
    // Test aarch32_VSUB_i_T1A1_A field M = 1 (Max)
    // ISET: A32
    // Fields: D=0, size=0, Vd=0, Q=0, Vn=0, N=0, M=1, Vm=0
    let encoding: u32 = 0xF3000820;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VSUB_i_T1A1_A
/// ASL: `field Vm 0 +: 4`
/// Requirement: FieldBoundary { field: "Vm", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_aarch32_vsub_i_t1a1_a_field_vm_0_min_800_f3000800() {
    // Encoding: 0xF3000800
    // Test aarch32_VSUB_i_T1A1_A field Vm = 0 (Min)
    // ISET: A32
    // Fields: M=0, D=0, Vd=0, Vm=0, size=0, Vn=0, Q=0, N=0
    let encoding: u32 = 0xF3000800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VSUB_i_T1A1_A
/// ASL: `field Vm 0 +: 4`
/// Requirement: FieldBoundary { field: "Vm", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_aarch32_vsub_i_t1a1_a_field_vm_1_poweroftwo_800_f3000801() {
    // Encoding: 0xF3000801
    // Test aarch32_VSUB_i_T1A1_A field Vm = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Q=0, Vn=0, Vd=0, D=0, N=0, Vm=1, size=0, M=0
    let encoding: u32 = 0xF3000801;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VSUB_i_T1A1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// D=0 (minimum value)
#[test]
fn test_aarch32_vsub_i_t1a1_a_combo_0_800_f3000800() {
    // Encoding: 0xF3000800
    // Test aarch32_VSUB_i_T1A1_A field combination: D=0, size=0, Vn=0, Vd=0, N=0, Q=0, M=0, Vm=0
    // ISET: A32
    // Fields: Vm=0, Vn=0, N=0, Q=0, Vd=0, M=0, D=0, size=0
    let encoding: u32 = 0xF3000800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VSUB_i_T1A1_A
/// ASL: `field size = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "size", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch32_vsub_i_t1a1_a_special_size_0_size_variant_0_2048_f3000800() {
    // Encoding: 0xF3000800
    // Test aarch32_VSUB_i_T1A1_A special value size = 0 (Size variant 0)
    // ISET: A32
    // Fields: M=0, Vd=0, Vn=0, size=0, D=0, Vm=0, Q=0, N=0
    let encoding: u32 = 0xF3000800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VSUB_i_T1A1_A
/// ASL: `field size = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "size", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch32_vsub_i_t1a1_a_special_size_1_size_variant_1_2048_f3100800() {
    // Encoding: 0xF3100800
    // Test aarch32_VSUB_i_T1A1_A special value size = 1 (Size variant 1)
    // ISET: A32
    // Fields: Vd=0, Vn=0, M=0, N=0, Q=0, Vm=0, size=1, D=0
    let encoding: u32 = 0xF3100800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VSUB_i_T1A1_A
/// ASL: `field size = 2 (Size variant 2)`
/// Requirement: FieldSpecial { field: "size", value: 2, meaning: "Size variant 2" }
/// Size variant 2
#[test]
fn test_aarch32_vsub_i_t1a1_a_special_size_2_size_variant_2_2048_f3200800() {
    // Encoding: 0xF3200800
    // Test aarch32_VSUB_i_T1A1_A special value size = 2 (Size variant 2)
    // ISET: A32
    // Fields: Q=0, M=0, size=2, Vd=0, Vm=0, Vn=0, D=0, N=0
    let encoding: u32 = 0xF3200800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VSUB_i_T1A1_A
/// ASL: `field size = 3 (Size variant 3)`
/// Requirement: FieldSpecial { field: "size", value: 3, meaning: "Size variant 3" }
/// Size variant 3
#[test]
fn test_aarch32_vsub_i_t1a1_a_special_size_3_size_variant_3_2048_f3300800() {
    // Encoding: 0xF3300800
    // Test aarch32_VSUB_i_T1A1_A special value size = 3 (Size variant 3)
    // ISET: A32
    // Fields: Vn=0, D=0, M=0, size=3, Vd=0, N=0, Vm=0, Q=0
    let encoding: u32 = 0xF3300800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VSUB_i_T1A1_A
/// ASL: `field Q = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "Q", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch32_vsub_i_t1a1_a_special_q_0_size_variant_0_2048_f3000800() {
    // Encoding: 0xF3000800
    // Test aarch32_VSUB_i_T1A1_A special value Q = 0 (Size variant 0)
    // ISET: A32
    // Fields: Q=0, M=0, Vn=0, N=0, Vm=0, D=0, size=0, Vd=0
    let encoding: u32 = 0xF3000800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VSUB_i_T1A1_A
/// ASL: `field Q = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "Q", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch32_vsub_i_t1a1_a_special_q_1_size_variant_1_2048_f3000840() {
    // Encoding: 0xF3000840
    // Test aarch32_VSUB_i_T1A1_A special value Q = 1 (Size variant 1)
    // ISET: A32
    // Fields: size=0, Q=1, Vm=0, M=0, Vd=0, N=0, Vn=0, D=0
    let encoding: u32 = 0xF3000840;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VSUB_i_T1A1_A
/// ASL: `Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "Q" }), rhs: Binary { op: And, lhs: LitBits([true]), rhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: "Vd" }), indices: [Single(LitInt(0))] }, rhs: Binary { op: Or, lhs: LitBits([true]), rhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: "Vn" }), indices: [Single(LitInt(0))] } } }, rhs: Binary { op: Or, lhs: LitBits([true]), rhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: "Vm" }), indices: [Single(LitInt(0))] } } }, rhs: LitBits([true]) } } }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"Q\" }), rhs: Binary { op: And, lhs: LitBits([true]), rhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: \"Vd\" }), indices: [Single(LitInt(0))] }, rhs: Binary { op: Or, lhs: LitBits([true]), rhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: \"Vn\" }), indices: [Single(LitInt(0))] } } }, rhs: Binary { op: Or, lhs: LitBits([true]), rhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: \"Vm\" }), indices: [Single(LitInt(0))] } } }, rhs: LitBits([true]) } } }" }
/// triggers Undefined
#[test]
fn test_aarch32_vsub_i_t1a1_a_invalid_0_800_f3000800() {
    // Encoding: 0xF3000800
    // Test aarch32_VSUB_i_T1A1_A invalid encoding: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "Q" }), rhs: Binary { op: And, lhs: LitBits([true]), rhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: "Vd" }), indices: [Single(LitInt(0))] }, rhs: Binary { op: Or, lhs: LitBits([true]), rhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: "Vn" }), indices: [Single(LitInt(0))] } } }, rhs: Binary { op: Or, lhs: LitBits([true]), rhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: "Vm" }), indices: [Single(LitInt(0))] } } }, rhs: LitBits([true]) } } }
    // ISET: A32
    // Fields: Q=0, Vn=0, Vm=0, size=0, N=0, D=0, M=0, Vd=0
    let encoding: u32 = 0xF3000800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || !matches!(exit.unwrap(), CpuExit::Continue),
        "expected UNDEFINED for A32 encoding 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch32_VSUB_i_T1A1_A
/// ASL: `Unconditional UNDEFINED`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNDEFINED" }
/// triggers Undefined
#[test]
fn test_aarch32_vsub_i_t1a1_a_invalid_1_800_f3000800() {
    // Encoding: 0xF3000800
    // Test aarch32_VSUB_i_T1A1_A invalid encoding: Unconditional UNDEFINED
    // ISET: A32
    // Fields: Vd=0, Q=0, N=0, size=0, D=0, Vn=0, M=0, Vm=0
    let encoding: u32 = 0xF3000800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || !matches!(exit.unwrap(), CpuExit::Continue),
        "expected UNDEFINED for A32 encoding 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch32_VSUB_i_T1A1_A
/// ASL: `field D 22 +: 1`
/// Requirement: FieldBoundary { field: "D", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_vsub_i_t1a1_a_field_d_0_min_800_ff000800() {
    // Thumb encoding (32): 0xFF000800
    // Test aarch32_VSUB_i_T1A1_A field D = 0 (Min)
    // ISET: T32
    // Fields: M=0, N=0, Vd=0, size=0, Vn=0, D=0, Q=0, Vm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFF000800;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VSUB_i_T1A1_A
/// ASL: `field D 22 +: 1`
/// Requirement: FieldBoundary { field: "D", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_vsub_i_t1a1_a_field_d_1_max_800_ff400800() {
    // Thumb encoding (32): 0xFF400800
    // Test aarch32_VSUB_i_T1A1_A field D = 1 (Max)
    // ISET: T32
    // Fields: M=0, Vd=0, Vm=0, Vn=0, N=0, D=1, size=0, Q=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFF400800;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VSUB_i_T1A1_A
/// ASL: `field size 20 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch32_vsub_i_t1a1_a_field_size_0_min_800_ff000800() {
    // Thumb encoding (32): 0xFF000800
    // Test aarch32_VSUB_i_T1A1_A field size = 0 (Min)
    // ISET: T32
    // Fields: Q=0, M=0, Vm=0, Vn=0, Vd=0, N=0, size=0, D=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFF000800;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VSUB_i_T1A1_A
/// ASL: `field size 20 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 1, boundary: PowerOfTwo }
/// 16-bit / halfword size
#[test]
fn test_aarch32_vsub_i_t1a1_a_field_size_1_poweroftwo_800_ff100800() {
    // Thumb encoding (32): 0xFF100800
    // Test aarch32_VSUB_i_T1A1_A field size = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Q=0, Vd=0, size=1, M=0, Vm=0, Vn=0, N=0, D=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFF100800;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VSUB_i_T1A1_A
/// ASL: `field size 20 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 2, boundary: PowerOfTwo }
/// 32-bit / word size
#[test]
fn test_aarch32_vsub_i_t1a1_a_field_size_2_poweroftwo_800_ff200800() {
    // Thumb encoding (32): 0xFF200800
    // Test aarch32_VSUB_i_T1A1_A field size = 2 (PowerOfTwo)
    // ISET: T32
    // Fields: Vn=0, Q=0, Vd=0, D=0, M=0, size=2, Vm=0, N=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFF200800;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VSUB_i_T1A1_A
/// ASL: `field size 20 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 3, boundary: Max }
/// 64-bit / doubleword size
#[test]
fn test_aarch32_vsub_i_t1a1_a_field_size_3_max_800_ff300800() {
    // Thumb encoding (32): 0xFF300800
    // Test aarch32_VSUB_i_T1A1_A field size = 3 (Max)
    // ISET: T32
    // Fields: Vn=0, Q=0, M=0, Vm=0, D=0, N=0, Vd=0, size=3
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFF300800;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VSUB_i_T1A1_A
/// ASL: `field Vn 16 +: 4`
/// Requirement: FieldBoundary { field: "Vn", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_aarch32_vsub_i_t1a1_a_field_vn_0_min_800_ff000800() {
    // Thumb encoding (32): 0xFF000800
    // Test aarch32_VSUB_i_T1A1_A field Vn = 0 (Min)
    // ISET: T32
    // Fields: D=0, N=0, Vd=0, Q=0, Vm=0, M=0, size=0, Vn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFF000800;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VSUB_i_T1A1_A
/// ASL: `field Vn 16 +: 4`
/// Requirement: FieldBoundary { field: "Vn", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_aarch32_vsub_i_t1a1_a_field_vn_1_poweroftwo_800_ff010800() {
    // Thumb encoding (32): 0xFF010800
    // Test aarch32_VSUB_i_T1A1_A field Vn = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: size=0, M=0, Vd=0, N=0, D=0, Vm=0, Q=0, Vn=1
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFF010800;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VSUB_i_T1A1_A
/// ASL: `field Vd 12 +: 4`
/// Requirement: FieldBoundary { field: "Vd", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_aarch32_vsub_i_t1a1_a_field_vd_0_min_800_ff000800() {
    // Thumb encoding (32): 0xFF000800
    // Test aarch32_VSUB_i_T1A1_A field Vd = 0 (Min)
    // ISET: T32
    // Fields: D=0, Vm=0, M=0, Q=0, Vn=0, size=0, N=0, Vd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFF000800;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VSUB_i_T1A1_A
/// ASL: `field Vd 12 +: 4`
/// Requirement: FieldBoundary { field: "Vd", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_aarch32_vsub_i_t1a1_a_field_vd_1_poweroftwo_800_ff001800() {
    // Thumb encoding (32): 0xFF001800
    // Test aarch32_VSUB_i_T1A1_A field Vd = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: D=0, Vd=1, Vn=0, Q=0, M=0, Vm=0, N=0, size=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFF001800;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VSUB_i_T1A1_A
/// ASL: `field N 7 +: 1`
/// Requirement: FieldBoundary { field: "N", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_vsub_i_t1a1_a_field_n_0_min_800_ff000800() {
    // Thumb encoding (32): 0xFF000800
    // Test aarch32_VSUB_i_T1A1_A field N = 0 (Min)
    // ISET: T32
    // Fields: Vn=0, M=0, D=0, N=0, Q=0, Vm=0, size=0, Vd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFF000800;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VSUB_i_T1A1_A
/// ASL: `field N 7 +: 1`
/// Requirement: FieldBoundary { field: "N", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_vsub_i_t1a1_a_field_n_1_max_800_ff000880() {
    // Thumb encoding (32): 0xFF000880
    // Test aarch32_VSUB_i_T1A1_A field N = 1 (Max)
    // ISET: T32
    // Fields: Vd=0, N=1, Vn=0, M=0, size=0, Q=0, Vm=0, D=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFF000880;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VSUB_i_T1A1_A
/// ASL: `field Q 6 +: 1`
/// Requirement: FieldBoundary { field: "Q", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch32_vsub_i_t1a1_a_field_q_0_min_800_ff000800() {
    // Thumb encoding (32): 0xFF000800
    // Test aarch32_VSUB_i_T1A1_A field Q = 0 (Min)
    // ISET: T32
    // Fields: M=0, Q=0, size=0, Vd=0, N=0, D=0, Vn=0, Vm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFF000800;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VSUB_i_T1A1_A
/// ASL: `field Q 6 +: 1`
/// Requirement: FieldBoundary { field: "Q", value: 1, boundary: Max }
/// 16-bit / halfword size
#[test]
fn test_aarch32_vsub_i_t1a1_a_field_q_1_max_800_ff000840() {
    // Thumb encoding (32): 0xFF000840
    // Test aarch32_VSUB_i_T1A1_A field Q = 1 (Max)
    // ISET: T32
    // Fields: Vn=0, size=0, N=0, Vd=0, M=0, Vm=0, Q=1, D=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFF000840;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VSUB_i_T1A1_A
/// ASL: `field M 5 +: 1`
/// Requirement: FieldBoundary { field: "M", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_vsub_i_t1a1_a_field_m_0_min_800_ff000800() {
    // Thumb encoding (32): 0xFF000800
    // Test aarch32_VSUB_i_T1A1_A field M = 0 (Min)
    // ISET: T32
    // Fields: M=0, Vd=0, Vm=0, Q=0, size=0, Vn=0, D=0, N=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFF000800;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VSUB_i_T1A1_A
/// ASL: `field M 5 +: 1`
/// Requirement: FieldBoundary { field: "M", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_vsub_i_t1a1_a_field_m_1_max_800_ff000820() {
    // Thumb encoding (32): 0xFF000820
    // Test aarch32_VSUB_i_T1A1_A field M = 1 (Max)
    // ISET: T32
    // Fields: Vm=0, Q=0, D=0, Vn=0, M=1, N=0, size=0, Vd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFF000820;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VSUB_i_T1A1_A
/// ASL: `field Vm 0 +: 4`
/// Requirement: FieldBoundary { field: "Vm", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_aarch32_vsub_i_t1a1_a_field_vm_0_min_800_ff000800() {
    // Thumb encoding (32): 0xFF000800
    // Test aarch32_VSUB_i_T1A1_A field Vm = 0 (Min)
    // ISET: T32
    // Fields: D=0, Vd=0, M=0, Q=0, Vn=0, size=0, N=0, Vm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFF000800;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VSUB_i_T1A1_A
/// ASL: `field Vm 0 +: 4`
/// Requirement: FieldBoundary { field: "Vm", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_aarch32_vsub_i_t1a1_a_field_vm_1_poweroftwo_800_ff000801() {
    // Thumb encoding (32): 0xFF000801
    // Test aarch32_VSUB_i_T1A1_A field Vm = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: D=0, Vn=0, size=0, M=0, Q=0, Vd=0, N=0, Vm=1
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFF000801;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VSUB_i_T1A1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// D=0 (minimum value)
#[test]
fn test_aarch32_vsub_i_t1a1_a_combo_0_800_ff000800() {
    // Thumb encoding (32): 0xFF000800
    // Test aarch32_VSUB_i_T1A1_A field combination: D=0, size=0, Vn=0, Vd=0, N=0, Q=0, M=0, Vm=0
    // ISET: T32
    // Fields: Vm=0, size=0, D=0, N=0, Vd=0, Q=0, M=0, Vn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFF000800;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VSUB_i_T1A1_A
/// ASL: `field size = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "size", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch32_vsub_i_t1a1_a_special_size_0_size_variant_0_2048_ff000800() {
    // Thumb encoding (32): 0xFF000800
    // Test aarch32_VSUB_i_T1A1_A special value size = 0 (Size variant 0)
    // ISET: T32
    // Fields: M=0, size=0, Q=0, D=0, Vd=0, N=0, Vm=0, Vn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFF000800;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VSUB_i_T1A1_A
/// ASL: `field size = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "size", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch32_vsub_i_t1a1_a_special_size_1_size_variant_1_2048_ff100800() {
    // Thumb encoding (32): 0xFF100800
    // Test aarch32_VSUB_i_T1A1_A special value size = 1 (Size variant 1)
    // ISET: T32
    // Fields: D=0, M=0, size=1, N=0, Vd=0, Vm=0, Vn=0, Q=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFF100800;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VSUB_i_T1A1_A
/// ASL: `field size = 2 (Size variant 2)`
/// Requirement: FieldSpecial { field: "size", value: 2, meaning: "Size variant 2" }
/// Size variant 2
#[test]
fn test_aarch32_vsub_i_t1a1_a_special_size_2_size_variant_2_2048_ff200800() {
    // Thumb encoding (32): 0xFF200800
    // Test aarch32_VSUB_i_T1A1_A special value size = 2 (Size variant 2)
    // ISET: T32
    // Fields: Vn=0, Vd=0, N=0, Vm=0, M=0, size=2, D=0, Q=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFF200800;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VSUB_i_T1A1_A
/// ASL: `field size = 3 (Size variant 3)`
/// Requirement: FieldSpecial { field: "size", value: 3, meaning: "Size variant 3" }
/// Size variant 3
#[test]
fn test_aarch32_vsub_i_t1a1_a_special_size_3_size_variant_3_2048_ff300800() {
    // Thumb encoding (32): 0xFF300800
    // Test aarch32_VSUB_i_T1A1_A special value size = 3 (Size variant 3)
    // ISET: T32
    // Fields: Q=0, D=0, size=3, M=0, N=0, Vn=0, Vm=0, Vd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFF300800;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VSUB_i_T1A1_A
/// ASL: `field Q = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "Q", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch32_vsub_i_t1a1_a_special_q_0_size_variant_0_2048_ff000800() {
    // Thumb encoding (32): 0xFF000800
    // Test aarch32_VSUB_i_T1A1_A special value Q = 0 (Size variant 0)
    // ISET: T32
    // Fields: Q=0, Vn=0, N=0, Vd=0, M=0, D=0, Vm=0, size=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFF000800;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VSUB_i_T1A1_A
/// ASL: `field Q = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "Q", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch32_vsub_i_t1a1_a_special_q_1_size_variant_1_2048_ff000840() {
    // Thumb encoding (32): 0xFF000840
    // Test aarch32_VSUB_i_T1A1_A special value Q = 1 (Size variant 1)
    // ISET: T32
    // Fields: Vd=0, Q=1, M=0, size=0, Vn=0, D=0, N=0, Vm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFF000840;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VSUB_i_T1A1_A
/// ASL: `Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "Q" }), rhs: Binary { op: And, lhs: LitBits([true]), rhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: "Vd" }), indices: [Single(LitInt(0))] }, rhs: Binary { op: Or, lhs: LitBits([true]), rhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: "Vn" }), indices: [Single(LitInt(0))] } } }, rhs: Binary { op: Or, lhs: LitBits([true]), rhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: "Vm" }), indices: [Single(LitInt(0))] } } }, rhs: LitBits([true]) } } }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"Q\" }), rhs: Binary { op: And, lhs: LitBits([true]), rhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: \"Vd\" }), indices: [Single(LitInt(0))] }, rhs: Binary { op: Or, lhs: LitBits([true]), rhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: \"Vn\" }), indices: [Single(LitInt(0))] } } }, rhs: Binary { op: Or, lhs: LitBits([true]), rhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: \"Vm\" }), indices: [Single(LitInt(0))] } } }, rhs: LitBits([true]) } } }" }
/// triggers Undefined
#[test]
fn test_aarch32_vsub_i_t1a1_a_invalid_0_800_ff000800() {
    // Thumb encoding (32): 0xFF000800
    // Test aarch32_VSUB_i_T1A1_A invalid encoding: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "Q" }), rhs: Binary { op: And, lhs: LitBits([true]), rhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: "Vd" }), indices: [Single(LitInt(0))] }, rhs: Binary { op: Or, lhs: LitBits([true]), rhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: "Vn" }), indices: [Single(LitInt(0))] } } }, rhs: Binary { op: Or, lhs: LitBits([true]), rhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: "Vm" }), indices: [Single(LitInt(0))] } } }, rhs: LitBits([true]) } } }
    // ISET: T32
    // Fields: N=0, M=0, Vm=0, Vd=0, D=0, size=0, Vn=0, Q=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFF000800;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || !matches!(exit.unwrap(), CpuExit::Continue),
        "expected UNDEFINED for T32 encoding"
    );
}

/// Provenance: aarch32_VSUB_i_T1A1_A
/// ASL: `Unconditional UNDEFINED`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNDEFINED" }
/// triggers Undefined
#[test]
fn test_aarch32_vsub_i_t1a1_a_invalid_1_800_ff000800() {
    // Thumb encoding (32): 0xFF000800
    // Test aarch32_VSUB_i_T1A1_A invalid encoding: Unconditional UNDEFINED
    // ISET: T32
    // Fields: Q=0, size=0, Vd=0, M=0, Vm=0, N=0, D=0, Vn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFF000800;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || !matches!(exit.unwrap(), CpuExit::Continue),
        "expected UNDEFINED for T32 encoding"
    );
}

/// Provenance: aarch32_VSUB_i_T1A1_A
/// ASL: `TST X1, X2`
/// Requirement: FlagComputation { flag: Z, scenario: ZeroResult }
/// zero AND zero (32-bit)
#[test]
fn test_aarch32_vsub_i_t1a1_a_oracle_32_0_6a02003f() {
    // Test TST 32-bit: zero AND zero (oracle)
    // Encoding: 0x6A02003F
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x0);
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0x6A02003F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z flag should be true");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be 0");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be 0");
}

/// Provenance: aarch32_VSUB_i_T1A1_A
/// ASL: `TST X1, X2`
/// Requirement: FlagComputation { flag: Z, scenario: ZeroResult }
/// zero AND zero (64-bit)
#[test]
fn test_aarch32_vsub_i_t1a1_a_oracle_64_0_ea02003f() {
    // Test TST 64-bit: zero AND zero (oracle)
    // Encoding: 0xEA02003F
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x0);
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xEA02003F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z flag should be true");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be 0");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be 0");
}

/// Provenance: aarch32_VSUB_i_T1A1_A
/// ASL: `TST X1, X2`
/// Requirement: FlagComputation { flag: Z, scenario: ZeroResult }
/// partial overlap (32-bit)
#[test]
fn test_aarch32_vsub_i_t1a1_a_oracle_32_1_6a02003f() {
    // Test TST 32-bit: partial overlap (oracle)
    // Encoding: 0x6A02003F
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFF);
    set_w(&mut cpu, 2, 0xF);
    let encoding: u32 = 0x6A02003F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be 0");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be 0");
}

/// Provenance: aarch32_VSUB_i_T1A1_A
/// ASL: `TST X1, X2`
/// Requirement: FlagComputation { flag: Z, scenario: ZeroResult }
/// partial overlap (64-bit)
#[test]
fn test_aarch32_vsub_i_t1a1_a_oracle_64_1_ea02003f() {
    // Test TST 64-bit: partial overlap (oracle)
    // Encoding: 0xEA02003F
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0xF);
    set_w(&mut cpu, 1, 0xFF);
    let encoding: u32 = 0xEA02003F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be 0");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be 0");
}

/// Provenance: aarch32_VSUB_i_T1A1_A
/// ASL: `TST X1, X2`
/// Requirement: FlagComputation { flag: Z, scenario: ZeroResult }
/// no overlap (32-bit)
#[test]
fn test_aarch32_vsub_i_t1a1_a_oracle_32_2_6a02003f() {
    // Test TST 32-bit: no overlap (oracle)
    // Encoding: 0x6A02003F
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFF);
    set_w(&mut cpu, 2, 0x0);
    let encoding: u32 = 0x6A02003F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z flag should be true");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be 0");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be 0");
}

/// Provenance: aarch32_VSUB_i_T1A1_A
/// ASL: `TST X1, X2`
/// Requirement: FlagComputation { flag: Z, scenario: ZeroResult }
/// no overlap (64-bit)
#[test]
fn test_aarch32_vsub_i_t1a1_a_oracle_64_2_ea02003f() {
    // Test TST 64-bit: no overlap (oracle)
    // Encoding: 0xEA02003F
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x0);
    set_w(&mut cpu, 1, 0xFF);
    let encoding: u32 = 0xEA02003F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z flag should be true");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be 0");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be 0");
}

/// Provenance: aarch32_VSUB_i_T1A1_A
/// ASL: `TST X1, X2`
/// Requirement: FlagComputation { flag: Z, scenario: ZeroResult }
/// MSB set (32-bit)
#[test]
fn test_aarch32_vsub_i_t1a1_a_oracle_32_3_6a02003f() {
    // Test TST 32-bit: MSB set (oracle)
    // Encoding: 0x6A02003F
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x0);
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0x6A02003F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z flag should be true");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be 0");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be 0");
}

/// Provenance: aarch32_VSUB_i_T1A1_A
/// ASL: `TST X1, X2`
/// Requirement: FlagComputation { flag: Z, scenario: ZeroResult }
/// MSB set (64-bit)
#[test]
fn test_aarch32_vsub_i_t1a1_a_oracle_64_3_ea02003f() {
    // Test TST 64-bit: MSB set (oracle)
    // Encoding: 0xEA02003F
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    set_w(&mut cpu, 2, 0x0);
    let encoding: u32 = 0xEA02003F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be 0");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be 0");
}

/// Provenance: aarch32_VSUB_i_T1A1_A
/// ASL: `TST X1, X2`
/// Requirement: FlagComputation { flag: Z, scenario: ZeroResult }
/// all ones (32-bit)
#[test]
fn test_aarch32_vsub_i_t1a1_a_oracle_32_4_6a02003f() {
    // Test TST 32-bit: all ones (oracle)
    // Encoding: 0x6A02003F
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    set_w(&mut cpu, 2, 0xFFFFFFFF);
    let encoding: u32 = 0x6A02003F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be 0");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be 0");
}

/// Provenance: aarch32_VSUB_i_T1A1_A
/// ASL: `TST X1, X2`
/// Requirement: FlagComputation { flag: Z, scenario: ZeroResult }
/// all ones (64-bit)
#[test]
fn test_aarch32_vsub_i_t1a1_a_oracle_64_4_ea02003f() {
    // Test TST 64-bit: all ones (oracle)
    // Encoding: 0xEA02003F
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0xFFFFFFFF);
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xEA02003F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be 0");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be 0");
}

/// Provenance: aarch32_VSUB_i_T1A1_A
/// ASL: `TST X1, X2`
/// Requirement: FlagComputation { flag: Z, scenario: ZeroResult }
/// alternating (no match) (32-bit)
#[test]
fn test_aarch32_vsub_i_t1a1_a_oracle_32_5_6a02003f() {
    // Test TST 32-bit: alternating (no match) (oracle)
    // Encoding: 0x6A02003F
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x55555555);
    set_w(&mut cpu, 1, 0xAAAAAAAA);
    let encoding: u32 = 0x6A02003F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z flag should be true");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be 0");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be 0");
}

/// Provenance: aarch32_VSUB_i_T1A1_A
/// ASL: `TST X1, X2`
/// Requirement: FlagComputation { flag: Z, scenario: ZeroResult }
/// alternating (no match) (64-bit)
#[test]
fn test_aarch32_vsub_i_t1a1_a_oracle_64_5_ea02003f() {
    // Test TST 64-bit: alternating (no match) (oracle)
    // Encoding: 0xEA02003F
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x55555555);
    set_w(&mut cpu, 1, 0xAAAAAAAA);
    let encoding: u32 = 0xEA02003F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z flag should be true");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be 0");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be 0");
}

/// Provenance: aarch32_VSUB_i_T1A1_A
/// ASL: `MUL X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// simple multiply (32)
#[test]
fn test_aarch32_vsub_i_t1a1_a_mul_oracle_32_0_1b027c20() {
    // Test MUL 32-bit: simple multiply (oracle)
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x3);
    set_w(&mut cpu, 1, 0x2);
    let encoding: u32 = 0x1B027C20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x6, "W0 should be 0x00000006");
}

/// Provenance: aarch32_VSUB_i_T1A1_A
/// ASL: `MUL X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// simple multiply (64)
#[test]
fn test_aarch32_vsub_i_t1a1_a_mul_oracle_64_0_9b027c20() {
    // Test MUL 64-bit: simple multiply (oracle)
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x2);
    set_w(&mut cpu, 2, 0x3);
    let encoding: u32 = 0x9B027C20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x6, "X0 should be 0x0000000000000006");
}

/// Provenance: aarch32_VSUB_i_T1A1_A
/// ASL: `MUL X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// multiply by zero (32)
#[test]
fn test_aarch32_vsub_i_t1a1_a_mul_oracle_32_1_1b027c20() {
    // Test MUL 32-bit: multiply by zero (oracle)
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    set_w(&mut cpu, 2, 0x64);
    let encoding: u32 = 0x1B027C20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x0, "W0 should be 0x00000000");
}

/// Provenance: aarch32_VSUB_i_T1A1_A
/// ASL: `MUL X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// multiply by zero (64)
#[test]
fn test_aarch32_vsub_i_t1a1_a_mul_oracle_64_1_9b027c20() {
    // Test MUL 64-bit: multiply by zero (oracle)
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    set_w(&mut cpu, 2, 0x64);
    let encoding: u32 = 0x9B027C20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x0, "X0 should be 0x0000000000000000");
}

/// Provenance: aarch32_VSUB_i_T1A1_A
/// ASL: `MUL X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// multiply by one (32)
#[test]
fn test_aarch32_vsub_i_t1a1_a_mul_oracle_32_2_1b027c20() {
    // Test MUL 32-bit: multiply by one (oracle)
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x1);
    set_w(&mut cpu, 2, 0x1);
    let encoding: u32 = 0x1B027C20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x1, "W0 should be 0x00000001");
}

/// Provenance: aarch32_VSUB_i_T1A1_A
/// ASL: `MUL X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// multiply by one (64)
#[test]
fn test_aarch32_vsub_i_t1a1_a_mul_oracle_64_2_9b027c20() {
    // Test MUL 64-bit: multiply by one (oracle)
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x1);
    set_w(&mut cpu, 1, 0x1);
    let encoding: u32 = 0x9B027C20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x1, "X0 should be 0x0000000000000001");
}

/// Provenance: aarch32_VSUB_i_T1A1_A
/// ASL: `MUL X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// 16-bit max * 16-bit max (32)
#[test]
fn test_aarch32_vsub_i_t1a1_a_mul_oracle_32_3_1b027c20() {
    // Test MUL 32-bit: 16-bit max * 16-bit max (oracle)
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFF);
    set_w(&mut cpu, 2, 0xFFFF);
    let encoding: u32 = 0x1B027C20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0xFFFE0001, "W0 should be 0xFFFE0001");
}

/// Provenance: aarch32_VSUB_i_T1A1_A
/// ASL: `MUL X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// 16-bit max * 16-bit max (64)
#[test]
fn test_aarch32_vsub_i_t1a1_a_mul_oracle_64_3_9b027c20() {
    // Test MUL 64-bit: 16-bit max * 16-bit max (oracle)
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0xFFFF);
    set_w(&mut cpu, 1, 0xFFFF);
    let encoding: u32 = 0x9B027C20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFE0001,
        "X0 should be 0x00000000FFFE0001"
    );
}

/// Provenance: aarch32_VSUB_i_T1A1_A
/// ASL: `MUL X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// shift-like multiply (32)
#[test]
fn test_aarch32_vsub_i_t1a1_a_mul_oracle_32_4_1b027c20() {
    // Test MUL 32-bit: shift-like multiply (oracle)
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x12345678);
    set_w(&mut cpu, 2, 0x2);
    let encoding: u32 = 0x1B027C20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x2468ACF0, "W0 should be 0x2468ACF0");
}

/// Provenance: aarch32_VSUB_i_T1A1_A
/// ASL: `MUL X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// shift-like multiply (64)
#[test]
fn test_aarch32_vsub_i_t1a1_a_mul_oracle_64_4_9b027c20() {
    // Test MUL 64-bit: shift-like multiply (oracle)
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x2);
    set_w(&mut cpu, 1, 0x12345678);
    let encoding: u32 = 0x9B027C20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0x2468ACF0,
        "X0 should be 0x000000002468ACF0"
    );
}

/// Provenance: aarch32_VSUB_i_T1A1_A
/// ASL: `MUL X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// larger values (32)
#[test]
fn test_aarch32_vsub_i_t1a1_a_mul_oracle_32_5_1b027c20() {
    // Test MUL 32-bit: larger values (oracle)
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0xC8);
    set_w(&mut cpu, 1, 0x64);
    let encoding: u32 = 0x1B027C20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x4E20, "W0 should be 0x00004E20");
}

/// Provenance: aarch32_VSUB_i_T1A1_A
/// ASL: `MUL X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// larger values (64)
#[test]
fn test_aarch32_vsub_i_t1a1_a_mul_oracle_64_5_9b027c20() {
    // Test MUL 64-bit: larger values (oracle)
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x64);
    set_w(&mut cpu, 2, 0xC8);
    let encoding: u32 = 0x9B027C20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x4E20, "X0 should be 0x0000000000004E20");
}

/// Provenance: aarch32_VSUB_i_T1A1_A
/// ASL: `MUL X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// 32-bit overflow (32)
#[test]
fn test_aarch32_vsub_i_t1a1_a_mul_oracle_32_6_1b027c20() {
    // Test MUL 32-bit: 32-bit overflow (oracle)
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0xFFFFFFFF);
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0x1B027C20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x1, "W0 should be 0x00000001");
}

/// Provenance: aarch32_VSUB_i_T1A1_A
/// ASL: `MUL X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// 32-bit overflow (64)
#[test]
fn test_aarch32_vsub_i_t1a1_a_mul_oracle_64_6_9b027c20() {
    // Test MUL 64-bit: 32-bit overflow (oracle)
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    set_w(&mut cpu, 2, 0xFFFFFFFF);
    let encoding: u32 = 0x9B027C20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x1, "X0 should be 0xFFFFFFFE00000001");
}

/// Provenance: aarch32_VSUB_i_T1A1_A
/// ASL: `MUL X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// prime numbers (32)
#[test]
fn test_aarch32_vsub_i_t1a1_a_mul_oracle_32_7_1b027c20() {
    // Test MUL 32-bit: prime numbers (oracle)
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0xB);
    set_w(&mut cpu, 1, 0x7);
    let encoding: u32 = 0x1B027C20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x4D, "W0 should be 0x0000004D");
}

/// Provenance: aarch32_VSUB_i_T1A1_A
/// ASL: `MUL X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// prime numbers (64)
#[test]
fn test_aarch32_vsub_i_t1a1_a_mul_oracle_64_7_9b027c20() {
    // Test MUL 64-bit: prime numbers (oracle)
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x7);
    set_w(&mut cpu, 2, 0xB);
    let encoding: u32 = 0x9B027C20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x4D, "X0 should be 0x000000000000004D");
}

/// Provenance: aarch32_VSUB_i_T1A1_A
/// ASL: `MUL R0, R1, R2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// simple values
#[test]
fn test_aarch32_vsub_i_t1a1_a_t32_oracle_0_ff000800() {
    // Test T32 MUL: simple values (oracle)
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x64);
    set_w(&mut cpu, 2, 0x32);
    let encoding: u32 = 0xFF000800;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x1388, "R0 should be 0x00001388");
}

/// Provenance: aarch32_VSUB_i_T1A1_A
/// ASL: `MUL R0, R1, R2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// zero values
#[test]
fn test_aarch32_vsub_i_t1a1_a_t32_oracle_1_ff000800() {
    // Test T32 MUL: zero values (oracle)
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    set_w(&mut cpu, 2, 0x0);
    let encoding: u32 = 0xFF000800;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x0, "R0 should be 0x00000000");
}

/// Provenance: aarch32_VSUB_i_T1A1_A
/// ASL: `MUL R0, R1, R2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// max value
#[test]
fn test_aarch32_vsub_i_t1a1_a_t32_oracle_2_ff000800() {
    // Test T32 MUL: max value (oracle)
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x1);
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xFF000800;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0xFFFFFFFF, "R0 should be 0xFFFFFFFF");
}

/// Provenance: aarch32_VSUB_i_T1A1_A
/// ASL: `MUL R0, R1, R2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// mixed pattern
#[test]
fn test_aarch32_vsub_i_t1a1_a_t32_oracle_3_ff000800() {
    // Test T32 MUL: mixed pattern (oracle)
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x12345678);
    set_w(&mut cpu, 2, 0xABCDEF01);
    let encoding: u32 = 0xFF000800;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x55065E78, "R0 should be 0x55065E78");
}

// ============================================================================
// aarch32_VSUBHN_A Tests
// ============================================================================

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `field D 22 +: 1`
/// Requirement: FieldBoundary { field: "D", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_vsubhn_t1a1_a_field_d_0_min_600_f2800600() {
    // Encoding: 0xF2800600
    // Test aarch32_VSUBHN_T1A1_A field D = 0 (Min)
    // ISET: A32
    // Fields: Vn=0, N=0, Vm=0, M=0, D=0, size=0, Vd=0
    let encoding: u32 = 0xF2800600;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `field D 22 +: 1`
/// Requirement: FieldBoundary { field: "D", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_vsubhn_t1a1_a_field_d_1_max_600_f2c00600() {
    // Encoding: 0xF2C00600
    // Test aarch32_VSUBHN_T1A1_A field D = 1 (Max)
    // ISET: A32
    // Fields: Vd=0, size=0, Vn=0, Vm=0, D=1, M=0, N=0
    let encoding: u32 = 0xF2C00600;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `field size 20 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch32_vsubhn_t1a1_a_field_size_0_min_600_f2800600() {
    // Encoding: 0xF2800600
    // Test aarch32_VSUBHN_T1A1_A field size = 0 (Min)
    // ISET: A32
    // Fields: Vd=0, N=0, Vn=0, M=0, Vm=0, size=0, D=0
    let encoding: u32 = 0xF2800600;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `field size 20 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 1, boundary: PowerOfTwo }
/// 16-bit / halfword size
#[test]
fn test_aarch32_vsubhn_t1a1_a_field_size_1_poweroftwo_600_f2900600() {
    // Encoding: 0xF2900600
    // Test aarch32_VSUBHN_T1A1_A field size = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Vn=0, Vd=0, N=0, M=0, D=0, Vm=0, size=1
    let encoding: u32 = 0xF2900600;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `field size 20 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 2, boundary: PowerOfTwo }
/// 32-bit / word size
#[test]
fn test_aarch32_vsubhn_t1a1_a_field_size_2_poweroftwo_600_f2a00600() {
    // Encoding: 0xF2A00600
    // Test aarch32_VSUBHN_T1A1_A field size = 2 (PowerOfTwo)
    // ISET: A32
    // Fields: Vn=0, Vd=0, Vm=0, size=2, D=0, N=0, M=0
    let encoding: u32 = 0xF2A00600;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `field size 20 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 3, boundary: Max }
/// 64-bit / doubleword size
#[test]
fn test_aarch32_vsubhn_t1a1_a_field_size_3_max_600_f2b00600() {
    // Encoding: 0xF2B00600
    // Test aarch32_VSUBHN_T1A1_A field size = 3 (Max)
    // ISET: A32
    // Fields: M=0, Vm=0, D=0, size=3, Vn=0, Vd=0, N=0
    let encoding: u32 = 0xF2B00600;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `field Vn 16 +: 4`
/// Requirement: FieldBoundary { field: "Vn", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_aarch32_vsubhn_t1a1_a_field_vn_0_min_600_f2800600() {
    // Encoding: 0xF2800600
    // Test aarch32_VSUBHN_T1A1_A field Vn = 0 (Min)
    // ISET: A32
    // Fields: size=0, D=0, N=0, Vn=0, Vm=0, M=0, Vd=0
    let encoding: u32 = 0xF2800600;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `field Vn 16 +: 4`
/// Requirement: FieldBoundary { field: "Vn", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_aarch32_vsubhn_t1a1_a_field_vn_1_poweroftwo_600_f2810600() {
    // Encoding: 0xF2810600
    // Test aarch32_VSUBHN_T1A1_A field Vn = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: N=0, M=0, Vm=0, D=0, Vn=1, Vd=0, size=0
    let encoding: u32 = 0xF2810600;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `field Vd 12 +: 4`
/// Requirement: FieldBoundary { field: "Vd", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_aarch32_vsubhn_t1a1_a_field_vd_0_min_600_f2800600() {
    // Encoding: 0xF2800600
    // Test aarch32_VSUBHN_T1A1_A field Vd = 0 (Min)
    // ISET: A32
    // Fields: N=0, D=0, Vd=0, M=0, Vm=0, size=0, Vn=0
    let encoding: u32 = 0xF2800600;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `field Vd 12 +: 4`
/// Requirement: FieldBoundary { field: "Vd", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_aarch32_vsubhn_t1a1_a_field_vd_1_poweroftwo_600_f2801600() {
    // Encoding: 0xF2801600
    // Test aarch32_VSUBHN_T1A1_A field Vd = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: D=0, size=0, N=0, M=0, Vn=0, Vd=1, Vm=0
    let encoding: u32 = 0xF2801600;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `field N 7 +: 1`
/// Requirement: FieldBoundary { field: "N", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_vsubhn_t1a1_a_field_n_0_min_600_f2800600() {
    // Encoding: 0xF2800600
    // Test aarch32_VSUBHN_T1A1_A field N = 0 (Min)
    // ISET: A32
    // Fields: M=0, N=0, Vn=0, D=0, size=0, Vd=0, Vm=0
    let encoding: u32 = 0xF2800600;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `field N 7 +: 1`
/// Requirement: FieldBoundary { field: "N", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_vsubhn_t1a1_a_field_n_1_max_600_f2800680() {
    // Encoding: 0xF2800680
    // Test aarch32_VSUBHN_T1A1_A field N = 1 (Max)
    // ISET: A32
    // Fields: Vm=0, D=0, size=0, M=0, N=1, Vn=0, Vd=0
    let encoding: u32 = 0xF2800680;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `field M 5 +: 1`
/// Requirement: FieldBoundary { field: "M", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_vsubhn_t1a1_a_field_m_0_min_600_f2800600() {
    // Encoding: 0xF2800600
    // Test aarch32_VSUBHN_T1A1_A field M = 0 (Min)
    // ISET: A32
    // Fields: size=0, Vd=0, Vn=0, D=0, N=0, M=0, Vm=0
    let encoding: u32 = 0xF2800600;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `field M 5 +: 1`
/// Requirement: FieldBoundary { field: "M", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_vsubhn_t1a1_a_field_m_1_max_600_f2800620() {
    // Encoding: 0xF2800620
    // Test aarch32_VSUBHN_T1A1_A field M = 1 (Max)
    // ISET: A32
    // Fields: Vn=0, size=0, N=0, M=1, Vm=0, Vd=0, D=0
    let encoding: u32 = 0xF2800620;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `field Vm 0 +: 4`
/// Requirement: FieldBoundary { field: "Vm", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_aarch32_vsubhn_t1a1_a_field_vm_0_min_600_f2800600() {
    // Encoding: 0xF2800600
    // Test aarch32_VSUBHN_T1A1_A field Vm = 0 (Min)
    // ISET: A32
    // Fields: Vm=0, D=0, M=0, size=0, Vd=0, N=0, Vn=0
    let encoding: u32 = 0xF2800600;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `field Vm 0 +: 4`
/// Requirement: FieldBoundary { field: "Vm", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_aarch32_vsubhn_t1a1_a_field_vm_1_poweroftwo_600_f2800601() {
    // Encoding: 0xF2800601
    // Test aarch32_VSUBHN_T1A1_A field Vm = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: M=0, Vm=1, N=0, Vd=0, D=0, size=0, Vn=0
    let encoding: u32 = 0xF2800601;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// D=0 (minimum value)
#[test]
fn test_aarch32_vsubhn_t1a1_a_combo_0_600_f2800600() {
    // Encoding: 0xF2800600
    // Test aarch32_VSUBHN_T1A1_A field combination: D=0, size=0, Vn=0, Vd=0, N=0, M=0, Vm=0
    // ISET: A32
    // Fields: D=0, Vd=0, Vm=0, size=0, Vn=0, N=0, M=0
    let encoding: u32 = 0xF2800600;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `field size = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "size", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch32_vsubhn_t1a1_a_special_size_0_size_variant_0_1536_f2800600() {
    // Encoding: 0xF2800600
    // Test aarch32_VSUBHN_T1A1_A special value size = 0 (Size variant 0)
    // ISET: A32
    // Fields: Vd=0, Vm=0, Vn=0, M=0, N=0, size=0, D=0
    let encoding: u32 = 0xF2800600;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `field size = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "size", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch32_vsubhn_t1a1_a_special_size_1_size_variant_1_1536_f2900600() {
    // Encoding: 0xF2900600
    // Test aarch32_VSUBHN_T1A1_A special value size = 1 (Size variant 1)
    // ISET: A32
    // Fields: M=0, Vm=0, Vn=0, D=0, N=0, size=1, Vd=0
    let encoding: u32 = 0xF2900600;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `field size = 2 (Size variant 2)`
/// Requirement: FieldSpecial { field: "size", value: 2, meaning: "Size variant 2" }
/// Size variant 2
#[test]
fn test_aarch32_vsubhn_t1a1_a_special_size_2_size_variant_2_1536_f2a00600() {
    // Encoding: 0xF2A00600
    // Test aarch32_VSUBHN_T1A1_A special value size = 2 (Size variant 2)
    // ISET: A32
    // Fields: size=2, M=0, N=0, Vn=0, Vd=0, D=0, Vm=0
    let encoding: u32 = 0xF2A00600;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `field size = 3 (Size variant 3)`
/// Requirement: FieldSpecial { field: "size", value: 3, meaning: "Size variant 3" }
/// Size variant 3
#[test]
fn test_aarch32_vsubhn_t1a1_a_special_size_3_size_variant_3_1536_f2b00600() {
    // Encoding: 0xF2B00600
    // Test aarch32_VSUBHN_T1A1_A special value size = 3 (Size variant 3)
    // ISET: A32
    // Fields: Vd=0, N=0, D=0, Vm=0, M=0, Vn=0, size=3
    let encoding: u32 = 0xF2B00600;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `Binary { op: Eq, lhs: Binary { op: Eq, lhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: "Vn" }), indices: [Single(LitInt(0))] }, rhs: Binary { op: Or, lhs: LitBits([true]), rhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: "Vm" }), indices: [Single(LitInt(0))] } } }, rhs: LitBits([true]) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Binary { op: Eq, lhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: \"Vn\" }), indices: [Single(LitInt(0))] }, rhs: Binary { op: Or, lhs: LitBits([true]), rhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: \"Vm\" }), indices: [Single(LitInt(0))] } } }, rhs: LitBits([true]) }" }
/// triggers Undefined
#[test]
fn test_aarch32_vsubhn_t1a1_a_invalid_0_600_f2800600() {
    // Encoding: 0xF2800600
    // Test aarch32_VSUBHN_T1A1_A invalid encoding: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: "Vn" }), indices: [Single(LitInt(0))] }, rhs: Binary { op: Or, lhs: LitBits([true]), rhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: "Vm" }), indices: [Single(LitInt(0))] } } }, rhs: LitBits([true]) }
    // ISET: A32
    // Fields: M=0, Vm=0, Vd=0, D=0, size=0, Vn=0, N=0
    let encoding: u32 = 0xF2800600;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || !matches!(exit.unwrap(), CpuExit::Continue),
        "expected UNDEFINED for A32 encoding 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `Unconditional UNDEFINED`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNDEFINED" }
/// triggers Undefined
#[test]
fn test_aarch32_vsubhn_t1a1_a_invalid_1_600_f2800600() {
    // Encoding: 0xF2800600
    // Test aarch32_VSUBHN_T1A1_A invalid encoding: Unconditional UNDEFINED
    // ISET: A32
    // Fields: N=0, M=0, Vm=0, Vd=0, size=0, D=0, Vn=0
    let encoding: u32 = 0xF2800600;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || !matches!(exit.unwrap(), CpuExit::Continue),
        "expected UNDEFINED for A32 encoding 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `field D 22 +: 1`
/// Requirement: FieldBoundary { field: "D", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_vsubhn_t1a1_a_field_d_0_min_600_ef800600() {
    // Thumb encoding (32): 0xEF800600
    // Test aarch32_VSUBHN_T1A1_A field D = 0 (Min)
    // ISET: T32
    // Fields: M=0, Vd=0, Vm=0, D=0, Vn=0, N=0, size=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF800600;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `field D 22 +: 1`
/// Requirement: FieldBoundary { field: "D", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_vsubhn_t1a1_a_field_d_1_max_600_efc00600() {
    // Thumb encoding (32): 0xEFC00600
    // Test aarch32_VSUBHN_T1A1_A field D = 1 (Max)
    // ISET: T32
    // Fields: Vd=0, M=0, N=0, Vm=0, size=0, D=1, Vn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEFC00600;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `field size 20 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch32_vsubhn_t1a1_a_field_size_0_min_600_ef800600() {
    // Thumb encoding (32): 0xEF800600
    // Test aarch32_VSUBHN_T1A1_A field size = 0 (Min)
    // ISET: T32
    // Fields: N=0, Vm=0, Vn=0, D=0, size=0, M=0, Vd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF800600;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `field size 20 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 1, boundary: PowerOfTwo }
/// 16-bit / halfword size
#[test]
fn test_aarch32_vsubhn_t1a1_a_field_size_1_poweroftwo_600_ef900600() {
    // Thumb encoding (32): 0xEF900600
    // Test aarch32_VSUBHN_T1A1_A field size = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Vn=0, Vm=0, size=1, M=0, D=0, Vd=0, N=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF900600;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `field size 20 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 2, boundary: PowerOfTwo }
/// 32-bit / word size
#[test]
fn test_aarch32_vsubhn_t1a1_a_field_size_2_poweroftwo_600_efa00600() {
    // Thumb encoding (32): 0xEFA00600
    // Test aarch32_VSUBHN_T1A1_A field size = 2 (PowerOfTwo)
    // ISET: T32
    // Fields: Vd=0, D=0, Vm=0, size=2, Vn=0, N=0, M=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEFA00600;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `field size 20 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 3, boundary: Max }
/// 64-bit / doubleword size
#[test]
fn test_aarch32_vsubhn_t1a1_a_field_size_3_max_600_efb00600() {
    // Thumb encoding (32): 0xEFB00600
    // Test aarch32_VSUBHN_T1A1_A field size = 3 (Max)
    // ISET: T32
    // Fields: D=0, N=0, Vn=0, size=3, Vd=0, M=0, Vm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEFB00600;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `field Vn 16 +: 4`
/// Requirement: FieldBoundary { field: "Vn", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_aarch32_vsubhn_t1a1_a_field_vn_0_min_600_ef800600() {
    // Thumb encoding (32): 0xEF800600
    // Test aarch32_VSUBHN_T1A1_A field Vn = 0 (Min)
    // ISET: T32
    // Fields: size=0, N=0, M=0, Vm=0, Vd=0, D=0, Vn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF800600;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `field Vn 16 +: 4`
/// Requirement: FieldBoundary { field: "Vn", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_aarch32_vsubhn_t1a1_a_field_vn_1_poweroftwo_600_ef810600() {
    // Thumb encoding (32): 0xEF810600
    // Test aarch32_VSUBHN_T1A1_A field Vn = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Vm=0, N=0, D=0, Vd=0, M=0, size=0, Vn=1
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF810600;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `field Vd 12 +: 4`
/// Requirement: FieldBoundary { field: "Vd", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_aarch32_vsubhn_t1a1_a_field_vd_0_min_600_ef800600() {
    // Thumb encoding (32): 0xEF800600
    // Test aarch32_VSUBHN_T1A1_A field Vd = 0 (Min)
    // ISET: T32
    // Fields: Vd=0, Vm=0, N=0, M=0, size=0, D=0, Vn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF800600;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `field Vd 12 +: 4`
/// Requirement: FieldBoundary { field: "Vd", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_aarch32_vsubhn_t1a1_a_field_vd_1_poweroftwo_600_ef801600() {
    // Thumb encoding (32): 0xEF801600
    // Test aarch32_VSUBHN_T1A1_A field Vd = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: N=0, M=0, size=0, Vd=1, Vm=0, Vn=0, D=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF801600;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `field N 7 +: 1`
/// Requirement: FieldBoundary { field: "N", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_vsubhn_t1a1_a_field_n_0_min_600_ef800600() {
    // Thumb encoding (32): 0xEF800600
    // Test aarch32_VSUBHN_T1A1_A field N = 0 (Min)
    // ISET: T32
    // Fields: N=0, Vn=0, M=0, Vm=0, Vd=0, D=0, size=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF800600;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `field N 7 +: 1`
/// Requirement: FieldBoundary { field: "N", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_vsubhn_t1a1_a_field_n_1_max_600_ef800680() {
    // Thumb encoding (32): 0xEF800680
    // Test aarch32_VSUBHN_T1A1_A field N = 1 (Max)
    // ISET: T32
    // Fields: N=1, D=0, Vd=0, size=0, Vn=0, M=0, Vm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF800680;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `field M 5 +: 1`
/// Requirement: FieldBoundary { field: "M", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_vsubhn_t1a1_a_field_m_0_min_600_ef800600() {
    // Thumb encoding (32): 0xEF800600
    // Test aarch32_VSUBHN_T1A1_A field M = 0 (Min)
    // ISET: T32
    // Fields: Vd=0, N=0, M=0, Vm=0, D=0, Vn=0, size=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF800600;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `field M 5 +: 1`
/// Requirement: FieldBoundary { field: "M", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_vsubhn_t1a1_a_field_m_1_max_600_ef800620() {
    // Thumb encoding (32): 0xEF800620
    // Test aarch32_VSUBHN_T1A1_A field M = 1 (Max)
    // ISET: T32
    // Fields: D=0, Vn=0, M=1, Vd=0, N=0, Vm=0, size=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF800620;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `field Vm 0 +: 4`
/// Requirement: FieldBoundary { field: "Vm", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_aarch32_vsubhn_t1a1_a_field_vm_0_min_600_ef800600() {
    // Thumb encoding (32): 0xEF800600
    // Test aarch32_VSUBHN_T1A1_A field Vm = 0 (Min)
    // ISET: T32
    // Fields: Vm=0, N=0, M=0, D=0, size=0, Vn=0, Vd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF800600;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `field Vm 0 +: 4`
/// Requirement: FieldBoundary { field: "Vm", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_aarch32_vsubhn_t1a1_a_field_vm_1_poweroftwo_600_ef800601() {
    // Thumb encoding (32): 0xEF800601
    // Test aarch32_VSUBHN_T1A1_A field Vm = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: M=0, Vm=1, Vn=0, D=0, N=0, size=0, Vd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF800601;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// D=0 (minimum value)
#[test]
fn test_aarch32_vsubhn_t1a1_a_combo_0_600_ef800600() {
    // Thumb encoding (32): 0xEF800600
    // Test aarch32_VSUBHN_T1A1_A field combination: D=0, size=0, Vn=0, Vd=0, N=0, M=0, Vm=0
    // ISET: T32
    // Fields: Vn=0, size=0, D=0, Vd=0, N=0, M=0, Vm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF800600;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `field size = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "size", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch32_vsubhn_t1a1_a_special_size_0_size_variant_0_1536_ef800600() {
    // Thumb encoding (32): 0xEF800600
    // Test aarch32_VSUBHN_T1A1_A special value size = 0 (Size variant 0)
    // ISET: T32
    // Fields: M=0, Vn=0, N=0, Vm=0, D=0, size=0, Vd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF800600;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `field size = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "size", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch32_vsubhn_t1a1_a_special_size_1_size_variant_1_1536_ef900600() {
    // Thumb encoding (32): 0xEF900600
    // Test aarch32_VSUBHN_T1A1_A special value size = 1 (Size variant 1)
    // ISET: T32
    // Fields: D=0, size=1, Vd=0, N=0, M=0, Vm=0, Vn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF900600;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `field size = 2 (Size variant 2)`
/// Requirement: FieldSpecial { field: "size", value: 2, meaning: "Size variant 2" }
/// Size variant 2
#[test]
fn test_aarch32_vsubhn_t1a1_a_special_size_2_size_variant_2_1536_efa00600() {
    // Thumb encoding (32): 0xEFA00600
    // Test aarch32_VSUBHN_T1A1_A special value size = 2 (Size variant 2)
    // ISET: T32
    // Fields: size=2, N=0, D=0, Vd=0, Vn=0, M=0, Vm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEFA00600;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `field size = 3 (Size variant 3)`
/// Requirement: FieldSpecial { field: "size", value: 3, meaning: "Size variant 3" }
/// Size variant 3
#[test]
fn test_aarch32_vsubhn_t1a1_a_special_size_3_size_variant_3_1536_efb00600() {
    // Thumb encoding (32): 0xEFB00600
    // Test aarch32_VSUBHN_T1A1_A special value size = 3 (Size variant 3)
    // ISET: T32
    // Fields: M=0, N=0, Vm=0, Vd=0, size=3, D=0, Vn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEFB00600;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `Binary { op: Eq, lhs: Binary { op: Eq, lhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: "Vn" }), indices: [Single(LitInt(0))] }, rhs: Binary { op: Or, lhs: LitBits([true]), rhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: "Vm" }), indices: [Single(LitInt(0))] } } }, rhs: LitBits([true]) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Binary { op: Eq, lhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: \"Vn\" }), indices: [Single(LitInt(0))] }, rhs: Binary { op: Or, lhs: LitBits([true]), rhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: \"Vm\" }), indices: [Single(LitInt(0))] } } }, rhs: LitBits([true]) }" }
/// triggers Undefined
#[test]
fn test_aarch32_vsubhn_t1a1_a_invalid_0_600_ef800600() {
    // Thumb encoding (32): 0xEF800600
    // Test aarch32_VSUBHN_T1A1_A invalid encoding: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: "Vn" }), indices: [Single(LitInt(0))] }, rhs: Binary { op: Or, lhs: LitBits([true]), rhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: "Vm" }), indices: [Single(LitInt(0))] } } }, rhs: LitBits([true]) }
    // ISET: T32
    // Fields: Vn=0, N=0, Vm=0, D=0, Vd=0, size=0, M=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF800600;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || !matches!(exit.unwrap(), CpuExit::Continue),
        "expected UNDEFINED for T32 encoding"
    );
}

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `Unconditional UNDEFINED`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNDEFINED" }
/// triggers Undefined
#[test]
fn test_aarch32_vsubhn_t1a1_a_invalid_1_600_ef800600() {
    // Thumb encoding (32): 0xEF800600
    // Test aarch32_VSUBHN_T1A1_A invalid encoding: Unconditional UNDEFINED
    // ISET: T32
    // Fields: Vn=0, D=0, N=0, M=0, size=0, Vd=0, Vm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF800600;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || !matches!(exit.unwrap(), CpuExit::Continue),
        "expected UNDEFINED for T32 encoding"
    );
}

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `ADD X0, X1, #10`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// simple addition/subtraction (32)
#[test]
fn test_aarch32_vsubhn_t1a1_a_add_oracle_32_0_f2802e20() {
    // Test ADD 32-bit: simple addition/subtraction (with oracle verification)
    // Encoding: 0xF2802E20
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x64);
    let encoding: u32 = 0xF2802E20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFF5,
        "X0 should be 0xFFFFFFFFFFFFFFF5"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `ADD X0, X1, #10`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// simple addition/subtraction (64)
#[test]
fn test_aarch32_vsubhn_t1a1_a_add_oracle_64_0_f2802e20() {
    // Test ADD 64-bit: simple addition/subtraction (with oracle verification)
    // Encoding: 0xF2802E20
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x64);
    let encoding: u32 = 0xF2802E20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFF5,
        "X0 should be 0xFFFFFFFFFFFFFFF5"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `ADD X0, X1, #0`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// zero operands (32)
#[test]
fn test_aarch32_vsubhn_t1a1_a_add_oracle_32_1_f2800620() {
    // Test ADD 32-bit: zero operands (with oracle verification)
    // Encoding: 0xF2800620
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2800620;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `ADD X0, X1, #0`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// zero operands (64)
#[test]
fn test_aarch32_vsubhn_t1a1_a_add_oracle_64_1_f2800620() {
    // Test ADD 64-bit: zero operands (with oracle verification)
    // Encoding: 0xF2800620
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2800620;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `ADD X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// small values (32)
#[test]
fn test_aarch32_vsubhn_t1a1_a_add_oracle_32_2_f2800620() {
    // Test ADD 32-bit: small values (with oracle verification)
    // Encoding: 0xF2800620
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x1);
    let encoding: u32 = 0xF2800620;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `ADD X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// small values (64)
#[test]
fn test_aarch32_vsubhn_t1a1_a_add_oracle_64_2_f2800620() {
    // Test ADD 64-bit: small values (with oracle verification)
    // Encoding: 0xF2800620
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x1);
    let encoding: u32 = 0xF2800620;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `ADD X0, X1, #4095`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// max imm12 unshifted (32)
#[test]
fn test_aarch32_vsubhn_t1a1_a_add_oracle_32_3_f2bffe20() {
    // Test ADD 32-bit: max imm12 unshifted (with oracle verification)
    // Encoding: 0xF2BFFE20
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2BFFE20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFF001,
        "X0 should be 0xFFFFFFFFFFFFF001"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `ADD X0, X1, #4095`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// max imm12 unshifted (64)
#[test]
fn test_aarch32_vsubhn_t1a1_a_add_oracle_64_3_f2bffe20() {
    // Test ADD 64-bit: max imm12 unshifted (with oracle verification)
    // Encoding: 0xF2BFFE20
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2BFFE20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFF001,
        "X0 should be 0xFFFFFFFFFFFFF001"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `ADD X0, X1, #4095, LSL #12`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// max imm12 shifted (32)
#[test]
fn test_aarch32_vsubhn_t1a1_a_add_oracle_32_4_f2fffe20() {
    // Test ADD 32-bit: max imm12 shifted (with oracle verification)
    // Encoding: 0xF2FFFE20
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2FFFE20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFF001000,
        "X0 should be 0xFFFFFFFFFF001000"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `ADD X0, X1, #4095, LSL #12`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// max imm12 shifted (64)
#[test]
fn test_aarch32_vsubhn_t1a1_a_add_oracle_64_4_f2fffe20() {
    // Test ADD 64-bit: max imm12 shifted (with oracle verification)
    // Encoding: 0xF2FFFE20
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2FFFE20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFF001000,
        "X0 should be 0xFFFFFFFFFF001000"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `ADD X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// max u64 operand (32)
#[test]
fn test_aarch32_vsubhn_t1a1_a_add_oracle_32_5_f2800620() {
    // Test ADD 32-bit: max u64 operand (with oracle verification)
    // Encoding: 0xF2800620
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800620;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `ADD X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// max u64 operand (64)
#[test]
fn test_aarch32_vsubhn_t1a1_a_add_oracle_64_5_f2800620() {
    // Test ADD 64-bit: max u64 operand (with oracle verification)
    // Encoding: 0xF2800620
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800620;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `ADD X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// zero result (for sub 1-1) (32)
#[test]
fn test_aarch32_vsubhn_t1a1_a_add_oracle_32_6_f2800620() {
    // Test ADD 32-bit: zero result (for sub 1-1) (with oracle verification)
    // Encoding: 0xF2800620
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2800620;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `ADD X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// zero result (for sub 1-1) (64)
#[test]
fn test_aarch32_vsubhn_t1a1_a_add_oracle_64_6_f2800620() {
    // Test ADD 64-bit: zero result (for sub 1-1) (with oracle verification)
    // Encoding: 0xF2800620
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2800620;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `ADD X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// signed overflow boundary 64-bit (32)
#[test]
fn test_aarch32_vsubhn_t1a1_a_add_oracle_32_7_f2800620() {
    // Test ADD 32-bit: signed overflow boundary 64-bit (with oracle verification)
    // Encoding: 0xF2800620
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800620;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `ADD X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// signed overflow boundary 64-bit (64)
#[test]
fn test_aarch32_vsubhn_t1a1_a_add_oracle_64_7_f2800620() {
    // Test ADD 64-bit: signed overflow boundary 64-bit (with oracle verification)
    // Encoding: 0xF2800620
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800620;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `ADD X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// signed overflow boundary 32-bit (32)
#[test]
fn test_aarch32_vsubhn_t1a1_a_add_oracle_32_8_f2800620() {
    // Test ADD 32-bit: signed overflow boundary 32-bit (with oracle verification)
    // Encoding: 0xF2800620
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x7FFFFFFF);
    let encoding: u32 = 0xF2800620;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `ADD X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// signed overflow boundary 32-bit (64)
#[test]
fn test_aarch32_vsubhn_t1a1_a_add_oracle_64_8_f2800620() {
    // Test ADD 64-bit: signed overflow boundary 32-bit (with oracle verification)
    // Encoding: 0xF2800620
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x7FFFFFFF);
    let encoding: u32 = 0xF2800620;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `ADD X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// unsigned overflow 64-bit (32)
#[test]
fn test_aarch32_vsubhn_t1a1_a_add_oracle_32_9_f2800620() {
    // Test ADD 32-bit: unsigned overflow 64-bit (with oracle verification)
    // Encoding: 0xF2800620
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800620;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `ADD X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// unsigned overflow 64-bit (64)
#[test]
fn test_aarch32_vsubhn_t1a1_a_add_oracle_64_9_f2800620() {
    // Test ADD 64-bit: unsigned overflow 64-bit (with oracle verification)
    // Encoding: 0xF2800620
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800620;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `ADD X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// unsigned overflow 32-bit (32)
#[test]
fn test_aarch32_vsubhn_t1a1_a_add_oracle_32_10_f2800620() {
    // Test ADD 32-bit: unsigned overflow 32-bit (with oracle verification)
    // Encoding: 0xF2800620
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800620;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `ADD X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// unsigned overflow 32-bit (64)
#[test]
fn test_aarch32_vsubhn_t1a1_a_add_oracle_64_10_f2800620() {
    // Test ADD 64-bit: unsigned overflow 32-bit (with oracle verification)
    // Encoding: 0xF2800620
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800620;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `ADD SP, X1, #10`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "writes to stack pointer" }
/// SP destination (32)
#[test]
fn test_aarch32_vsubhn_t1a1_a_add_oracle_32_rd31_sp_f2802e3f() {
    // Test ADD 32-bit with Rd=31 (SP)
    // Encoding: 0xF2802E3F
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x64);
    let encoding: u32 = 0xF2802E3F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `ADD SP, X1, #10`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "writes to stack pointer" }
/// SP destination (64)
#[test]
fn test_aarch32_vsubhn_t1a1_a_add_oracle_64_rd31_sp_f2802e3f() {
    // Test ADD 64-bit with Rd=31 (SP)
    // Encoding: 0xF2802E3F
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x64);
    let encoding: u32 = 0xF2802E3F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `ADDS X0, X1, #10`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// simple addition/subtraction (32)
#[test]
fn test_aarch32_vsubhn_t1a1_a_adds_oracle_32_0_f2802e20() {
    // Test ADDS 32-bit: simple addition/subtraction (with oracle verification)
    // Encoding: 0xF2802E20
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x64);
    let encoding: u32 = 0xF2802E20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFF5,
        "X0 should be 0xFFFFFFFFFFFFFFF5"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `ADDS X0, X1, #10`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// simple addition/subtraction (64)
#[test]
fn test_aarch32_vsubhn_t1a1_a_adds_oracle_64_0_f2802e20() {
    // Test ADDS 64-bit: simple addition/subtraction (with oracle verification)
    // Encoding: 0xF2802E20
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x64);
    let encoding: u32 = 0xF2802E20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFF5,
        "X0 should be 0xFFFFFFFFFFFFFFF5"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `ADDS X0, X1, #0`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// zero operands (32)
#[test]
fn test_aarch32_vsubhn_t1a1_a_adds_oracle_32_1_f2800620() {
    // Test ADDS 32-bit: zero operands (with oracle verification)
    // Encoding: 0xF2800620
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2800620;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `ADDS X0, X1, #0`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// zero operands (64)
#[test]
fn test_aarch32_vsubhn_t1a1_a_adds_oracle_64_1_f2800620() {
    // Test ADDS 64-bit: zero operands (with oracle verification)
    // Encoding: 0xF2800620
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2800620;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `ADDS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// small values (32)
#[test]
fn test_aarch32_vsubhn_t1a1_a_adds_oracle_32_2_f2800620() {
    // Test ADDS 32-bit: small values (with oracle verification)
    // Encoding: 0xF2800620
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x1);
    let encoding: u32 = 0xF2800620;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `ADDS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// small values (64)
#[test]
fn test_aarch32_vsubhn_t1a1_a_adds_oracle_64_2_f2800620() {
    // Test ADDS 64-bit: small values (with oracle verification)
    // Encoding: 0xF2800620
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x1);
    let encoding: u32 = 0xF2800620;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `ADDS X0, X1, #4095`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// max imm12 unshifted (32)
#[test]
fn test_aarch32_vsubhn_t1a1_a_adds_oracle_32_3_f2bffe20() {
    // Test ADDS 32-bit: max imm12 unshifted (with oracle verification)
    // Encoding: 0xF2BFFE20
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2BFFE20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFF001,
        "X0 should be 0xFFFFFFFFFFFFF001"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `ADDS X0, X1, #4095`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// max imm12 unshifted (64)
#[test]
fn test_aarch32_vsubhn_t1a1_a_adds_oracle_64_3_f2bffe20() {
    // Test ADDS 64-bit: max imm12 unshifted (with oracle verification)
    // Encoding: 0xF2BFFE20
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2BFFE20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFF001,
        "X0 should be 0xFFFFFFFFFFFFF001"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `ADDS X0, X1, #4095, LSL #12`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// max imm12 shifted (32)
#[test]
fn test_aarch32_vsubhn_t1a1_a_adds_oracle_32_4_f2fffe20() {
    // Test ADDS 32-bit: max imm12 shifted (with oracle verification)
    // Encoding: 0xF2FFFE20
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2FFFE20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFF001000,
        "X0 should be 0xFFFFFFFFFF001000"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `ADDS X0, X1, #4095, LSL #12`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// max imm12 shifted (64)
#[test]
fn test_aarch32_vsubhn_t1a1_a_adds_oracle_64_4_f2fffe20() {
    // Test ADDS 64-bit: max imm12 shifted (with oracle verification)
    // Encoding: 0xF2FFFE20
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2FFFE20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFF001000,
        "X0 should be 0xFFFFFFFFFF001000"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `ADDS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// max u64 operand (32)
#[test]
fn test_aarch32_vsubhn_t1a1_a_adds_oracle_32_5_f2800620() {
    // Test ADDS 32-bit: max u64 operand (with oracle verification)
    // Encoding: 0xF2800620
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800620;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `ADDS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// max u64 operand (64)
#[test]
fn test_aarch32_vsubhn_t1a1_a_adds_oracle_64_5_f2800620() {
    // Test ADDS 64-bit: max u64 operand (with oracle verification)
    // Encoding: 0xF2800620
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800620;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `ADDS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// zero result (for sub 1-1) (32)
#[test]
fn test_aarch32_vsubhn_t1a1_a_adds_oracle_32_6_f2800620() {
    // Test ADDS 32-bit: zero result (for sub 1-1) (with oracle verification)
    // Encoding: 0xF2800620
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2800620;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `ADDS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// zero result (for sub 1-1) (64)
#[test]
fn test_aarch32_vsubhn_t1a1_a_adds_oracle_64_6_f2800620() {
    // Test ADDS 64-bit: zero result (for sub 1-1) (with oracle verification)
    // Encoding: 0xF2800620
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2800620;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `ADDS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// signed overflow boundary 64-bit (32)
#[test]
fn test_aarch32_vsubhn_t1a1_a_adds_oracle_32_7_f2800620() {
    // Test ADDS 32-bit: signed overflow boundary 64-bit (with oracle verification)
    // Encoding: 0xF2800620
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800620;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `ADDS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// signed overflow boundary 64-bit (64)
#[test]
fn test_aarch32_vsubhn_t1a1_a_adds_oracle_64_7_f2800620() {
    // Test ADDS 64-bit: signed overflow boundary 64-bit (with oracle verification)
    // Encoding: 0xF2800620
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800620;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `ADDS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// signed overflow boundary 32-bit (32)
#[test]
fn test_aarch32_vsubhn_t1a1_a_adds_oracle_32_8_f2800620() {
    // Test ADDS 32-bit: signed overflow boundary 32-bit (with oracle verification)
    // Encoding: 0xF2800620
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x7FFFFFFF);
    let encoding: u32 = 0xF2800620;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `ADDS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// signed overflow boundary 32-bit (64)
#[test]
fn test_aarch32_vsubhn_t1a1_a_adds_oracle_64_8_f2800620() {
    // Test ADDS 64-bit: signed overflow boundary 32-bit (with oracle verification)
    // Encoding: 0xF2800620
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x7FFFFFFF);
    let encoding: u32 = 0xF2800620;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `ADDS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// unsigned overflow 64-bit (32)
#[test]
fn test_aarch32_vsubhn_t1a1_a_adds_oracle_32_9_f2800620() {
    // Test ADDS 32-bit: unsigned overflow 64-bit (with oracle verification)
    // Encoding: 0xF2800620
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800620;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `ADDS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// unsigned overflow 64-bit (64)
#[test]
fn test_aarch32_vsubhn_t1a1_a_adds_oracle_64_9_f2800620() {
    // Test ADDS 64-bit: unsigned overflow 64-bit (with oracle verification)
    // Encoding: 0xF2800620
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800620;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `ADDS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// unsigned overflow 32-bit (32)
#[test]
fn test_aarch32_vsubhn_t1a1_a_adds_oracle_32_10_f2800620() {
    // Test ADDS 32-bit: unsigned overflow 32-bit (with oracle verification)
    // Encoding: 0xF2800620
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800620;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `ADDS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// unsigned overflow 32-bit (64)
#[test]
fn test_aarch32_vsubhn_t1a1_a_adds_oracle_64_10_f2800620() {
    // Test ADDS 64-bit: unsigned overflow 32-bit (with oracle verification)
    // Encoding: 0xF2800620
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800620;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `ADDS ZR, X1, #10`
/// Requirement: RegisterSpecial { reg: Zr, behavior: "result discarded, flags set" }
/// ZR destination (32)
#[test]
fn test_aarch32_vsubhn_t1a1_a_adds_oracle_32_rd31_zr_f2802e3f() {
    // Test ADDS 32-bit with Rd=31 (ZR)
    // Encoding: 0xF2802E3F
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x64);
    let encoding: u32 = 0xF2802E3F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `ADDS ZR, X1, #10`
/// Requirement: RegisterSpecial { reg: Zr, behavior: "result discarded, flags set" }
/// ZR destination (64)
#[test]
fn test_aarch32_vsubhn_t1a1_a_adds_oracle_64_rd31_zr_f2802e3f() {
    // Test ADDS 64-bit with Rd=31 (ZR)
    // Encoding: 0xF2802E3F
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x64);
    let encoding: u32 = 0xF2802E3F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `SUB X0, X1, #10`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// simple addition/subtraction (32)
#[test]
fn test_aarch32_vsubhn_t1a1_a_sub_oracle_32_0_f2802e20() {
    // Test SUB 32-bit: simple addition/subtraction (with oracle verification)
    // Encoding: 0xF2802E20
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x64);
    let encoding: u32 = 0xF2802E20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFF5,
        "X0 should be 0xFFFFFFFFFFFFFFF5"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `SUB X0, X1, #10`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// simple addition/subtraction (64)
#[test]
fn test_aarch32_vsubhn_t1a1_a_sub_oracle_64_0_f2802e20() {
    // Test SUB 64-bit: simple addition/subtraction (with oracle verification)
    // Encoding: 0xF2802E20
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x64);
    let encoding: u32 = 0xF2802E20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFF5,
        "X0 should be 0xFFFFFFFFFFFFFFF5"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `SUB X0, X1, #0`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// zero operands (32)
#[test]
fn test_aarch32_vsubhn_t1a1_a_sub_oracle_32_1_f2800620() {
    // Test SUB 32-bit: zero operands (with oracle verification)
    // Encoding: 0xF2800620
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2800620;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `SUB X0, X1, #0`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// zero operands (64)
#[test]
fn test_aarch32_vsubhn_t1a1_a_sub_oracle_64_1_f2800620() {
    // Test SUB 64-bit: zero operands (with oracle verification)
    // Encoding: 0xF2800620
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2800620;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `SUB X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// small values (32)
#[test]
fn test_aarch32_vsubhn_t1a1_a_sub_oracle_32_2_f2800620() {
    // Test SUB 32-bit: small values (with oracle verification)
    // Encoding: 0xF2800620
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x1);
    let encoding: u32 = 0xF2800620;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `SUB X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// small values (64)
#[test]
fn test_aarch32_vsubhn_t1a1_a_sub_oracle_64_2_f2800620() {
    // Test SUB 64-bit: small values (with oracle verification)
    // Encoding: 0xF2800620
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x1);
    let encoding: u32 = 0xF2800620;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `SUB X0, X1, #4095`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// max imm12 unshifted (32)
#[test]
fn test_aarch32_vsubhn_t1a1_a_sub_oracle_32_3_f2bffe20() {
    // Test SUB 32-bit: max imm12 unshifted (with oracle verification)
    // Encoding: 0xF2BFFE20
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2BFFE20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFF001,
        "X0 should be 0xFFFFFFFFFFFFF001"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `SUB X0, X1, #4095`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// max imm12 unshifted (64)
#[test]
fn test_aarch32_vsubhn_t1a1_a_sub_oracle_64_3_f2bffe20() {
    // Test SUB 64-bit: max imm12 unshifted (with oracle verification)
    // Encoding: 0xF2BFFE20
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2BFFE20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFF001,
        "X0 should be 0xFFFFFFFFFFFFF001"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `SUB X0, X1, #4095, LSL #12`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// max imm12 shifted (32)
#[test]
fn test_aarch32_vsubhn_t1a1_a_sub_oracle_32_4_f2fffe20() {
    // Test SUB 32-bit: max imm12 shifted (with oracle verification)
    // Encoding: 0xF2FFFE20
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2FFFE20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFF001000,
        "X0 should be 0xFFFFFFFFFF001000"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `SUB X0, X1, #4095, LSL #12`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// max imm12 shifted (64)
#[test]
fn test_aarch32_vsubhn_t1a1_a_sub_oracle_64_4_f2fffe20() {
    // Test SUB 64-bit: max imm12 shifted (with oracle verification)
    // Encoding: 0xF2FFFE20
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2FFFE20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFF001000,
        "X0 should be 0xFFFFFFFFFF001000"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `SUB X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// max u64 operand (32)
#[test]
fn test_aarch32_vsubhn_t1a1_a_sub_oracle_32_5_f2800620() {
    // Test SUB 32-bit: max u64 operand (with oracle verification)
    // Encoding: 0xF2800620
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800620;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `SUB X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// max u64 operand (64)
#[test]
fn test_aarch32_vsubhn_t1a1_a_sub_oracle_64_5_f2800620() {
    // Test SUB 64-bit: max u64 operand (with oracle verification)
    // Encoding: 0xF2800620
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800620;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `SUB X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// zero result (for sub 1-1) (32)
#[test]
fn test_aarch32_vsubhn_t1a1_a_sub_oracle_32_6_f2800620() {
    // Test SUB 32-bit: zero result (for sub 1-1) (with oracle verification)
    // Encoding: 0xF2800620
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2800620;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `SUB X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// zero result (for sub 1-1) (64)
#[test]
fn test_aarch32_vsubhn_t1a1_a_sub_oracle_64_6_f2800620() {
    // Test SUB 64-bit: zero result (for sub 1-1) (with oracle verification)
    // Encoding: 0xF2800620
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2800620;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `SUB X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// signed overflow boundary 64-bit (32)
#[test]
fn test_aarch32_vsubhn_t1a1_a_sub_oracle_32_7_f2800620() {
    // Test SUB 32-bit: signed overflow boundary 64-bit (with oracle verification)
    // Encoding: 0xF2800620
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800620;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `SUB X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// signed overflow boundary 64-bit (64)
#[test]
fn test_aarch32_vsubhn_t1a1_a_sub_oracle_64_7_f2800620() {
    // Test SUB 64-bit: signed overflow boundary 64-bit (with oracle verification)
    // Encoding: 0xF2800620
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800620;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `SUB X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// signed overflow boundary 32-bit (32)
#[test]
fn test_aarch32_vsubhn_t1a1_a_sub_oracle_32_8_f2800620() {
    // Test SUB 32-bit: signed overflow boundary 32-bit (with oracle verification)
    // Encoding: 0xF2800620
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x7FFFFFFF);
    let encoding: u32 = 0xF2800620;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `SUB X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// signed overflow boundary 32-bit (64)
#[test]
fn test_aarch32_vsubhn_t1a1_a_sub_oracle_64_8_f2800620() {
    // Test SUB 64-bit: signed overflow boundary 32-bit (with oracle verification)
    // Encoding: 0xF2800620
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x7FFFFFFF);
    let encoding: u32 = 0xF2800620;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `SUB X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// unsigned overflow 64-bit (32)
#[test]
fn test_aarch32_vsubhn_t1a1_a_sub_oracle_32_9_f2800620() {
    // Test SUB 32-bit: unsigned overflow 64-bit (with oracle verification)
    // Encoding: 0xF2800620
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800620;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `SUB X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// unsigned overflow 64-bit (64)
#[test]
fn test_aarch32_vsubhn_t1a1_a_sub_oracle_64_9_f2800620() {
    // Test SUB 64-bit: unsigned overflow 64-bit (with oracle verification)
    // Encoding: 0xF2800620
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800620;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `SUB X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// unsigned overflow 32-bit (32)
#[test]
fn test_aarch32_vsubhn_t1a1_a_sub_oracle_32_10_f2800620() {
    // Test SUB 32-bit: unsigned overflow 32-bit (with oracle verification)
    // Encoding: 0xF2800620
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800620;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `SUB X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// unsigned overflow 32-bit (64)
#[test]
fn test_aarch32_vsubhn_t1a1_a_sub_oracle_64_10_f2800620() {
    // Test SUB 64-bit: unsigned overflow 32-bit (with oracle verification)
    // Encoding: 0xF2800620
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800620;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `SUB SP, X1, #10`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "writes to stack pointer" }
/// SP destination (32)
#[test]
fn test_aarch32_vsubhn_t1a1_a_sub_oracle_32_rd31_sp_f2802e3f() {
    // Test SUB 32-bit with Rd=31 (SP)
    // Encoding: 0xF2802E3F
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x64);
    let encoding: u32 = 0xF2802E3F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `SUB SP, X1, #10`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "writes to stack pointer" }
/// SP destination (64)
#[test]
fn test_aarch32_vsubhn_t1a1_a_sub_oracle_64_rd31_sp_f2802e3f() {
    // Test SUB 64-bit with Rd=31 (SP)
    // Encoding: 0xF2802E3F
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x64);
    let encoding: u32 = 0xF2802E3F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `SUBS X0, X1, #10`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// simple addition/subtraction (32)
#[test]
fn test_aarch32_vsubhn_t1a1_a_subs_oracle_32_0_f2802e20() {
    // Test SUBS 32-bit: simple addition/subtraction (with oracle verification)
    // Encoding: 0xF2802E20
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x64);
    let encoding: u32 = 0xF2802E20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFF5,
        "X0 should be 0xFFFFFFFFFFFFFFF5"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `SUBS X0, X1, #10`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// simple addition/subtraction (64)
#[test]
fn test_aarch32_vsubhn_t1a1_a_subs_oracle_64_0_f2802e20() {
    // Test SUBS 64-bit: simple addition/subtraction (with oracle verification)
    // Encoding: 0xF2802E20
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x64);
    let encoding: u32 = 0xF2802E20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFF5,
        "X0 should be 0xFFFFFFFFFFFFFFF5"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `SUBS X0, X1, #0`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// zero operands (32)
#[test]
fn test_aarch32_vsubhn_t1a1_a_subs_oracle_32_1_f2800620() {
    // Test SUBS 32-bit: zero operands (with oracle verification)
    // Encoding: 0xF2800620
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2800620;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `SUBS X0, X1, #0`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// zero operands (64)
#[test]
fn test_aarch32_vsubhn_t1a1_a_subs_oracle_64_1_f2800620() {
    // Test SUBS 64-bit: zero operands (with oracle verification)
    // Encoding: 0xF2800620
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2800620;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `SUBS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// small values (32)
#[test]
fn test_aarch32_vsubhn_t1a1_a_subs_oracle_32_2_f2800620() {
    // Test SUBS 32-bit: small values (with oracle verification)
    // Encoding: 0xF2800620
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x1);
    let encoding: u32 = 0xF2800620;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `SUBS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// small values (64)
#[test]
fn test_aarch32_vsubhn_t1a1_a_subs_oracle_64_2_f2800620() {
    // Test SUBS 64-bit: small values (with oracle verification)
    // Encoding: 0xF2800620
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x1);
    let encoding: u32 = 0xF2800620;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `SUBS X0, X1, #4095`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// max imm12 unshifted (32)
#[test]
fn test_aarch32_vsubhn_t1a1_a_subs_oracle_32_3_f2bffe20() {
    // Test SUBS 32-bit: max imm12 unshifted (with oracle verification)
    // Encoding: 0xF2BFFE20
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2BFFE20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFF001,
        "X0 should be 0xFFFFFFFFFFFFF001"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `SUBS X0, X1, #4095`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// max imm12 unshifted (64)
#[test]
fn test_aarch32_vsubhn_t1a1_a_subs_oracle_64_3_f2bffe20() {
    // Test SUBS 64-bit: max imm12 unshifted (with oracle verification)
    // Encoding: 0xF2BFFE20
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2BFFE20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFF001,
        "X0 should be 0xFFFFFFFFFFFFF001"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `SUBS X0, X1, #4095, LSL #12`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// max imm12 shifted (32)
#[test]
fn test_aarch32_vsubhn_t1a1_a_subs_oracle_32_4_f2fffe20() {
    // Test SUBS 32-bit: max imm12 shifted (with oracle verification)
    // Encoding: 0xF2FFFE20
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2FFFE20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFF001000,
        "X0 should be 0xFFFFFFFFFF001000"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `SUBS X0, X1, #4095, LSL #12`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// max imm12 shifted (64)
#[test]
fn test_aarch32_vsubhn_t1a1_a_subs_oracle_64_4_f2fffe20() {
    // Test SUBS 64-bit: max imm12 shifted (with oracle verification)
    // Encoding: 0xF2FFFE20
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2FFFE20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFF001000,
        "X0 should be 0xFFFFFFFFFF001000"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `SUBS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// max u64 operand (32)
#[test]
fn test_aarch32_vsubhn_t1a1_a_subs_oracle_32_5_f2800620() {
    // Test SUBS 32-bit: max u64 operand (with oracle verification)
    // Encoding: 0xF2800620
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800620;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `SUBS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// max u64 operand (64)
#[test]
fn test_aarch32_vsubhn_t1a1_a_subs_oracle_64_5_f2800620() {
    // Test SUBS 64-bit: max u64 operand (with oracle verification)
    // Encoding: 0xF2800620
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800620;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `SUBS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// zero result (for sub 1-1) (32)
#[test]
fn test_aarch32_vsubhn_t1a1_a_subs_oracle_32_6_f2800620() {
    // Test SUBS 32-bit: zero result (for sub 1-1) (with oracle verification)
    // Encoding: 0xF2800620
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2800620;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `SUBS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// zero result (for sub 1-1) (64)
#[test]
fn test_aarch32_vsubhn_t1a1_a_subs_oracle_64_6_f2800620() {
    // Test SUBS 64-bit: zero result (for sub 1-1) (with oracle verification)
    // Encoding: 0xF2800620
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2800620;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `SUBS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// signed overflow boundary 64-bit (32)
#[test]
fn test_aarch32_vsubhn_t1a1_a_subs_oracle_32_7_f2800620() {
    // Test SUBS 32-bit: signed overflow boundary 64-bit (with oracle verification)
    // Encoding: 0xF2800620
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800620;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `SUBS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// signed overflow boundary 64-bit (64)
#[test]
fn test_aarch32_vsubhn_t1a1_a_subs_oracle_64_7_f2800620() {
    // Test SUBS 64-bit: signed overflow boundary 64-bit (with oracle verification)
    // Encoding: 0xF2800620
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800620;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `SUBS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// signed overflow boundary 32-bit (32)
#[test]
fn test_aarch32_vsubhn_t1a1_a_subs_oracle_32_8_f2800620() {
    // Test SUBS 32-bit: signed overflow boundary 32-bit (with oracle verification)
    // Encoding: 0xF2800620
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x7FFFFFFF);
    let encoding: u32 = 0xF2800620;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `SUBS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// signed overflow boundary 32-bit (64)
#[test]
fn test_aarch32_vsubhn_t1a1_a_subs_oracle_64_8_f2800620() {
    // Test SUBS 64-bit: signed overflow boundary 32-bit (with oracle verification)
    // Encoding: 0xF2800620
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x7FFFFFFF);
    let encoding: u32 = 0xF2800620;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `SUBS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// unsigned overflow 64-bit (32)
#[test]
fn test_aarch32_vsubhn_t1a1_a_subs_oracle_32_9_f2800620() {
    // Test SUBS 32-bit: unsigned overflow 64-bit (with oracle verification)
    // Encoding: 0xF2800620
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800620;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `SUBS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// unsigned overflow 64-bit (64)
#[test]
fn test_aarch32_vsubhn_t1a1_a_subs_oracle_64_9_f2800620() {
    // Test SUBS 64-bit: unsigned overflow 64-bit (with oracle verification)
    // Encoding: 0xF2800620
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800620;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `SUBS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// unsigned overflow 32-bit (32)
#[test]
fn test_aarch32_vsubhn_t1a1_a_subs_oracle_32_10_f2800620() {
    // Test SUBS 32-bit: unsigned overflow 32-bit (with oracle verification)
    // Encoding: 0xF2800620
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800620;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `SUBS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// unsigned overflow 32-bit (64)
#[test]
fn test_aarch32_vsubhn_t1a1_a_subs_oracle_64_10_f2800620() {
    // Test SUBS 64-bit: unsigned overflow 32-bit (with oracle verification)
    // Encoding: 0xF2800620
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800620;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `SUBS ZR, X1, #10`
/// Requirement: RegisterSpecial { reg: Zr, behavior: "result discarded, flags set" }
/// ZR destination (32)
#[test]
fn test_aarch32_vsubhn_t1a1_a_subs_oracle_32_rd31_zr_f2802e3f() {
    // Test SUBS 32-bit with Rd=31 (ZR)
    // Encoding: 0xF2802E3F
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x64);
    let encoding: u32 = 0xF2802E3F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `SUBS ZR, X1, #10`
/// Requirement: RegisterSpecial { reg: Zr, behavior: "result discarded, flags set" }
/// ZR destination (64)
#[test]
fn test_aarch32_vsubhn_t1a1_a_subs_oracle_64_rd31_zr_f2802e3f() {
    // Test SUBS 64-bit with Rd=31 (ZR)
    // Encoding: 0xF2802E3F
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x64);
    let encoding: u32 = 0xF2802E3F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `ADD R0, R1, #10`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// small immediate (Rn=0x00000000)
#[test]
fn test_aarch32_vsubhn_t1a1_a_a32_add_sub_imm_0_0_f281000a() {
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

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `ADD R0, R1, #10`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// small immediate (Rn=0x00000064)
#[test]
fn test_aarch32_vsubhn_t1a1_a_a32_add_sub_imm_0_64_f281000a() {
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

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `ADD R0, R1, #10`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// small immediate (Rn=0x7FFFFFFF)
#[test]
fn test_aarch32_vsubhn_t1a1_a_a32_add_sub_imm_0_7fffffff_f281000a() {
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

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `ADD R0, R1, #10`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// small immediate (Rn=0x80000000)
#[test]
fn test_aarch32_vsubhn_t1a1_a_a32_add_sub_imm_0_80000000_f281000a() {
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

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `ADD R0, R1, #10`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// small immediate (Rn=0xFFFFFFFF)
#[test]
fn test_aarch32_vsubhn_t1a1_a_a32_add_sub_imm_0_ffffffff_f281000a() {
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

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `ADD R0, R1, #255`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// max imm8 (Rn=0x00000000)
#[test]
fn test_aarch32_vsubhn_t1a1_a_a32_add_sub_imm_1_0_f28100ff() {
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

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `ADD R0, R1, #255`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// max imm8 (Rn=0x00000064)
#[test]
fn test_aarch32_vsubhn_t1a1_a_a32_add_sub_imm_1_64_f28100ff() {
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

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `ADD R0, R1, #255`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// max imm8 (Rn=0x7FFFFFFF)
#[test]
fn test_aarch32_vsubhn_t1a1_a_a32_add_sub_imm_1_7fffffff_f28100ff() {
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

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `ADD R0, R1, #255`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// max imm8 (Rn=0x80000000)
#[test]
fn test_aarch32_vsubhn_t1a1_a_a32_add_sub_imm_1_80000000_f28100ff() {
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

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `ADD R0, R1, #255`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// max imm8 (Rn=0xFFFFFFFF)
#[test]
fn test_aarch32_vsubhn_t1a1_a_a32_add_sub_imm_1_ffffffff_f28100ff() {
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

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `ADD R0, R1, #32`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// rotated by 2 (Rn=0x00000000)
#[test]
fn test_aarch32_vsubhn_t1a1_a_a32_add_sub_imm_2_0_f2810180() {
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

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `ADD R0, R1, #32`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// rotated by 2 (Rn=0x00000064)
#[test]
fn test_aarch32_vsubhn_t1a1_a_a32_add_sub_imm_2_64_f2810180() {
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

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `ADD R0, R1, #32`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// rotated by 2 (Rn=0x7FFFFFFF)
#[test]
fn test_aarch32_vsubhn_t1a1_a_a32_add_sub_imm_2_7fffffff_f2810180() {
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

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `ADD R0, R1, #32`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// rotated by 2 (Rn=0x80000000)
#[test]
fn test_aarch32_vsubhn_t1a1_a_a32_add_sub_imm_2_80000000_f2810180() {
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

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `ADD R0, R1, #32`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// rotated by 2 (Rn=0xFFFFFFFF)
#[test]
fn test_aarch32_vsubhn_t1a1_a_a32_add_sub_imm_2_ffffffff_f2810180() {
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

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `ADD R0, R1, #251658240`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// rotated by 8 (Rn=0x00000000)
#[test]
fn test_aarch32_vsubhn_t1a1_a_a32_add_sub_imm_3_0_f281040f() {
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

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `ADD R0, R1, #251658240`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// rotated by 8 (Rn=0x00000064)
#[test]
fn test_aarch32_vsubhn_t1a1_a_a32_add_sub_imm_3_64_f281040f() {
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

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `ADD R0, R1, #251658240`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// rotated by 8 (Rn=0x7FFFFFFF)
#[test]
fn test_aarch32_vsubhn_t1a1_a_a32_add_sub_imm_3_7fffffff_f281040f() {
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

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `ADD R0, R1, #251658240`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// rotated by 8 (Rn=0x80000000)
#[test]
fn test_aarch32_vsubhn_t1a1_a_a32_add_sub_imm_3_80000000_f281040f() {
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

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `ADD R0, R1, #251658240`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// rotated by 8 (Rn=0xFFFFFFFF)
#[test]
fn test_aarch32_vsubhn_t1a1_a_a32_add_sub_imm_3_ffffffff_f281040f() {
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

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `ADD R0, R1, #0`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// zero immediate (Rn=0x00000000)
#[test]
fn test_aarch32_vsubhn_t1a1_a_a32_add_sub_imm_4_0_f2810000() {
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

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `ADD R0, R1, #0`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// zero immediate (Rn=0x00000064)
#[test]
fn test_aarch32_vsubhn_t1a1_a_a32_add_sub_imm_4_64_f2810000() {
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

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `ADD R0, R1, #0`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// zero immediate (Rn=0x7FFFFFFF)
#[test]
fn test_aarch32_vsubhn_t1a1_a_a32_add_sub_imm_4_7fffffff_f2810000() {
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

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `ADD R0, R1, #0`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// zero immediate (Rn=0x80000000)
#[test]
fn test_aarch32_vsubhn_t1a1_a_a32_add_sub_imm_4_80000000_f2810000() {
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

/// Provenance: aarch32_VSUBHN_T1A1_A
/// ASL: `ADD R0, R1, #0`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// zero immediate (Rn=0xFFFFFFFF)
#[test]
fn test_aarch32_vsubhn_t1a1_a_a32_add_sub_imm_4_ffffffff_f2810000() {
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
// aarch32_VSUBL_A Tests
// ============================================================================

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `field U 24 +: 1`
/// Requirement: FieldBoundary { field: "U", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_vsubl_t1a1_a_field_u_0_min_200_f2800200() {
    // Encoding: 0xF2800200
    // Test aarch32_VSUBL_T1A1_A field U = 0 (Min)
    // ISET: A32
    // Fields: N=0, M=0, U=0, Vd=0, D=0, Vn=0, size=0, op=0, Vm=0
    let encoding: u32 = 0xF2800200;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `field U 24 +: 1`
/// Requirement: FieldBoundary { field: "U", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_vsubl_t1a1_a_field_u_1_max_200_f3800200() {
    // Encoding: 0xF3800200
    // Test aarch32_VSUBL_T1A1_A field U = 1 (Max)
    // ISET: A32
    // Fields: Vn=0, D=0, Vd=0, op=0, M=0, size=0, N=0, U=1, Vm=0
    let encoding: u32 = 0xF3800200;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `field D 22 +: 1`
/// Requirement: FieldBoundary { field: "D", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_vsubl_t1a1_a_field_d_0_min_200_f2800200() {
    // Encoding: 0xF2800200
    // Test aarch32_VSUBL_T1A1_A field D = 0 (Min)
    // ISET: A32
    // Fields: size=0, M=0, Vm=0, N=0, Vn=0, Vd=0, op=0, D=0, U=0
    let encoding: u32 = 0xF2800200;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `field D 22 +: 1`
/// Requirement: FieldBoundary { field: "D", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_vsubl_t1a1_a_field_d_1_max_200_f2c00200() {
    // Encoding: 0xF2C00200
    // Test aarch32_VSUBL_T1A1_A field D = 1 (Max)
    // ISET: A32
    // Fields: Vm=0, Vn=0, U=0, size=0, Vd=0, op=0, M=0, D=1, N=0
    let encoding: u32 = 0xF2C00200;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `field size 20 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch32_vsubl_t1a1_a_field_size_0_min_200_f2800200() {
    // Encoding: 0xF2800200
    // Test aarch32_VSUBL_T1A1_A field size = 0 (Min)
    // ISET: A32
    // Fields: Vd=0, U=0, N=0, Vm=0, size=0, Vn=0, M=0, D=0, op=0
    let encoding: u32 = 0xF2800200;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `field size 20 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 1, boundary: PowerOfTwo }
/// 16-bit / halfword size
#[test]
fn test_aarch32_vsubl_t1a1_a_field_size_1_poweroftwo_200_f2900200() {
    // Encoding: 0xF2900200
    // Test aarch32_VSUBL_T1A1_A field size = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: N=0, op=0, Vd=0, size=1, D=0, M=0, Vn=0, Vm=0, U=0
    let encoding: u32 = 0xF2900200;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `field size 20 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 2, boundary: PowerOfTwo }
/// 32-bit / word size
#[test]
fn test_aarch32_vsubl_t1a1_a_field_size_2_poweroftwo_200_f2a00200() {
    // Encoding: 0xF2A00200
    // Test aarch32_VSUBL_T1A1_A field size = 2 (PowerOfTwo)
    // ISET: A32
    // Fields: Vm=0, size=2, op=0, U=0, Vn=0, Vd=0, D=0, N=0, M=0
    let encoding: u32 = 0xF2A00200;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `field size 20 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 3, boundary: Max }
/// 64-bit / doubleword size
#[test]
fn test_aarch32_vsubl_t1a1_a_field_size_3_max_200_f2b00200() {
    // Encoding: 0xF2B00200
    // Test aarch32_VSUBL_T1A1_A field size = 3 (Max)
    // ISET: A32
    // Fields: size=3, Vd=0, N=0, M=0, D=0, Vm=0, Vn=0, U=0, op=0
    let encoding: u32 = 0xF2B00200;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `field Vn 16 +: 4`
/// Requirement: FieldBoundary { field: "Vn", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_aarch32_vsubl_t1a1_a_field_vn_0_min_200_f2800200() {
    // Encoding: 0xF2800200
    // Test aarch32_VSUBL_T1A1_A field Vn = 0 (Min)
    // ISET: A32
    // Fields: Vn=0, Vd=0, Vm=0, size=0, N=0, M=0, U=0, op=0, D=0
    let encoding: u32 = 0xF2800200;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `field Vn 16 +: 4`
/// Requirement: FieldBoundary { field: "Vn", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_aarch32_vsubl_t1a1_a_field_vn_1_poweroftwo_200_f2810200() {
    // Encoding: 0xF2810200
    // Test aarch32_VSUBL_T1A1_A field Vn = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: D=0, M=0, op=0, Vd=0, Vm=0, N=0, Vn=1, size=0, U=0
    let encoding: u32 = 0xF2810200;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `field Vd 12 +: 4`
/// Requirement: FieldBoundary { field: "Vd", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_aarch32_vsubl_t1a1_a_field_vd_0_min_200_f2800200() {
    // Encoding: 0xF2800200
    // Test aarch32_VSUBL_T1A1_A field Vd = 0 (Min)
    // ISET: A32
    // Fields: Vn=0, Vm=0, U=0, D=0, size=0, Vd=0, M=0, op=0, N=0
    let encoding: u32 = 0xF2800200;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `field Vd 12 +: 4`
/// Requirement: FieldBoundary { field: "Vd", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_aarch32_vsubl_t1a1_a_field_vd_1_poweroftwo_200_f2801200() {
    // Encoding: 0xF2801200
    // Test aarch32_VSUBL_T1A1_A field Vd = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: size=0, Vd=1, U=0, op=0, Vm=0, Vn=0, M=0, N=0, D=0
    let encoding: u32 = 0xF2801200;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `field op 8 +: 1`
/// Requirement: FieldBoundary { field: "op", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_vsubl_t1a1_a_field_op_0_min_200_f2800200() {
    // Encoding: 0xF2800200
    // Test aarch32_VSUBL_T1A1_A field op = 0 (Min)
    // ISET: A32
    // Fields: D=0, op=0, U=0, size=0, M=0, Vm=0, N=0, Vn=0, Vd=0
    let encoding: u32 = 0xF2800200;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `field op 8 +: 1`
/// Requirement: FieldBoundary { field: "op", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_vsubl_t1a1_a_field_op_1_max_200_f2800300() {
    // Encoding: 0xF2800300
    // Test aarch32_VSUBL_T1A1_A field op = 1 (Max)
    // ISET: A32
    // Fields: N=0, op=1, U=0, Vn=0, size=0, M=0, Vm=0, Vd=0, D=0
    let encoding: u32 = 0xF2800300;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `field N 7 +: 1`
/// Requirement: FieldBoundary { field: "N", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_vsubl_t1a1_a_field_n_0_min_200_f2800200() {
    // Encoding: 0xF2800200
    // Test aarch32_VSUBL_T1A1_A field N = 0 (Min)
    // ISET: A32
    // Fields: Vn=0, D=0, N=0, Vm=0, size=0, Vd=0, U=0, M=0, op=0
    let encoding: u32 = 0xF2800200;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `field N 7 +: 1`
/// Requirement: FieldBoundary { field: "N", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_vsubl_t1a1_a_field_n_1_max_200_f2800280() {
    // Encoding: 0xF2800280
    // Test aarch32_VSUBL_T1A1_A field N = 1 (Max)
    // ISET: A32
    // Fields: M=0, Vm=0, op=0, Vd=0, N=1, size=0, Vn=0, U=0, D=0
    let encoding: u32 = 0xF2800280;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `field M 5 +: 1`
/// Requirement: FieldBoundary { field: "M", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_vsubl_t1a1_a_field_m_0_min_200_f2800200() {
    // Encoding: 0xF2800200
    // Test aarch32_VSUBL_T1A1_A field M = 0 (Min)
    // ISET: A32
    // Fields: size=0, D=0, Vn=0, M=0, Vd=0, op=0, N=0, U=0, Vm=0
    let encoding: u32 = 0xF2800200;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `field M 5 +: 1`
/// Requirement: FieldBoundary { field: "M", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_vsubl_t1a1_a_field_m_1_max_200_f2800220() {
    // Encoding: 0xF2800220
    // Test aarch32_VSUBL_T1A1_A field M = 1 (Max)
    // ISET: A32
    // Fields: N=0, U=0, D=0, Vd=0, size=0, op=0, M=1, Vm=0, Vn=0
    let encoding: u32 = 0xF2800220;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `field Vm 0 +: 4`
/// Requirement: FieldBoundary { field: "Vm", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_aarch32_vsubl_t1a1_a_field_vm_0_min_200_f2800200() {
    // Encoding: 0xF2800200
    // Test aarch32_VSUBL_T1A1_A field Vm = 0 (Min)
    // ISET: A32
    // Fields: D=0, Vd=0, size=0, M=0, N=0, op=0, U=0, Vm=0, Vn=0
    let encoding: u32 = 0xF2800200;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `field Vm 0 +: 4`
/// Requirement: FieldBoundary { field: "Vm", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_aarch32_vsubl_t1a1_a_field_vm_1_poweroftwo_200_f2800201() {
    // Encoding: 0xF2800201
    // Test aarch32_VSUBL_T1A1_A field Vm = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: size=0, op=0, N=0, Vn=0, Vd=0, U=0, M=0, D=0, Vm=1
    let encoding: u32 = 0xF2800201;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// U=0 (minimum value)
#[test]
fn test_aarch32_vsubl_t1a1_a_combo_0_200_f2800200() {
    // Encoding: 0xF2800200
    // Test aarch32_VSUBL_T1A1_A field combination: U=0, D=0, size=0, Vn=0, Vd=0, op=0, N=0, M=0, Vm=0
    // ISET: A32
    // Fields: size=0, Vd=0, Vn=0, op=0, M=0, U=0, D=0, Vm=0, N=0
    let encoding: u32 = 0xF2800200;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `field size = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "size", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch32_vsubl_t1a1_a_special_size_0_size_variant_0_512_f2800200() {
    // Encoding: 0xF2800200
    // Test aarch32_VSUBL_T1A1_A special value size = 0 (Size variant 0)
    // ISET: A32
    // Fields: U=0, N=0, D=0, op=0, M=0, size=0, Vm=0, Vd=0, Vn=0
    let encoding: u32 = 0xF2800200;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `field size = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "size", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch32_vsubl_t1a1_a_special_size_1_size_variant_1_512_f2900200() {
    // Encoding: 0xF2900200
    // Test aarch32_VSUBL_T1A1_A special value size = 1 (Size variant 1)
    // ISET: A32
    // Fields: M=0, U=0, Vd=0, size=1, N=0, Vm=0, op=0, D=0, Vn=0
    let encoding: u32 = 0xF2900200;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `field size = 2 (Size variant 2)`
/// Requirement: FieldSpecial { field: "size", value: 2, meaning: "Size variant 2" }
/// Size variant 2
#[test]
fn test_aarch32_vsubl_t1a1_a_special_size_2_size_variant_2_512_f2a00200() {
    // Encoding: 0xF2A00200
    // Test aarch32_VSUBL_T1A1_A special value size = 2 (Size variant 2)
    // ISET: A32
    // Fields: D=0, M=0, size=2, Vd=0, Vn=0, U=0, Vm=0, op=0, N=0
    let encoding: u32 = 0xF2A00200;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `field size = 3 (Size variant 3)`
/// Requirement: FieldSpecial { field: "size", value: 3, meaning: "Size variant 3" }
/// Size variant 3
#[test]
fn test_aarch32_vsubl_t1a1_a_special_size_3_size_variant_3_512_f2b00200() {
    // Encoding: 0xF2B00200
    // Test aarch32_VSUBL_T1A1_A special value size = 3 (Size variant 3)
    // ISET: A32
    // Fields: M=0, U=0, D=0, size=3, N=0, Vn=0, Vm=0, op=0, Vd=0
    let encoding: u32 = 0xF2B00200;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `Binary { op: Eq, lhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: "Vd" }), indices: [Single(LitInt(0))] }, rhs: Binary { op: Or, lhs: LitBits([true]), rhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "op" }), rhs: Binary { op: And, lhs: LitBits([true]), rhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: "Vn" }), indices: [Single(LitInt(0))] } } }, rhs: LitBits([true]) } } }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: \"Vd\" }), indices: [Single(LitInt(0))] }, rhs: Binary { op: Or, lhs: LitBits([true]), rhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"op\" }), rhs: Binary { op: And, lhs: LitBits([true]), rhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: \"Vn\" }), indices: [Single(LitInt(0))] } } }, rhs: LitBits([true]) } } }" }
/// triggers Undefined
#[test]
fn test_aarch32_vsubl_t1a1_a_invalid_0_200_f2800200() {
    // Encoding: 0xF2800200
    // Test aarch32_VSUBL_T1A1_A invalid encoding: Binary { op: Eq, lhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: "Vd" }), indices: [Single(LitInt(0))] }, rhs: Binary { op: Or, lhs: LitBits([true]), rhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "op" }), rhs: Binary { op: And, lhs: LitBits([true]), rhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: "Vn" }), indices: [Single(LitInt(0))] } } }, rhs: LitBits([true]) } } }
    // ISET: A32
    // Fields: size=0, Vn=0, Vd=0, op=0, M=0, Vm=0, U=0, D=0, N=0
    let encoding: u32 = 0xF2800200;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || !matches!(exit.unwrap(), CpuExit::Continue),
        "expected UNDEFINED for A32 encoding 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `Unconditional UNDEFINED`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNDEFINED" }
/// triggers Undefined
#[test]
fn test_aarch32_vsubl_t1a1_a_invalid_1_200_f2800200() {
    // Encoding: 0xF2800200
    // Test aarch32_VSUBL_T1A1_A invalid encoding: Unconditional UNDEFINED
    // ISET: A32
    // Fields: D=0, N=0, Vn=0, Vd=0, U=0, op=0, Vm=0, size=0, M=0
    let encoding: u32 = 0xF2800200;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || !matches!(exit.unwrap(), CpuExit::Continue),
        "expected UNDEFINED for A32 encoding 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `field U 28 +: 1`
/// Requirement: FieldBoundary { field: "U", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_vsubl_t1a1_a_field_u_0_min_200_ef800200() {
    // Thumb encoding (32): 0xEF800200
    // Test aarch32_VSUBL_T1A1_A field U = 0 (Min)
    // ISET: T32
    // Fields: Vn=0, Vm=0, U=0, D=0, size=0, N=0, Vd=0, op=0, M=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF800200;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `field U 28 +: 1`
/// Requirement: FieldBoundary { field: "U", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_vsubl_t1a1_a_field_u_1_max_200_ff800200() {
    // Thumb encoding (32): 0xFF800200
    // Test aarch32_VSUBL_T1A1_A field U = 1 (Max)
    // ISET: T32
    // Fields: U=1, M=0, Vm=0, op=0, Vd=0, D=0, Vn=0, N=0, size=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFF800200;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `field D 22 +: 1`
/// Requirement: FieldBoundary { field: "D", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_vsubl_t1a1_a_field_d_0_min_200_ef800200() {
    // Thumb encoding (32): 0xEF800200
    // Test aarch32_VSUBL_T1A1_A field D = 0 (Min)
    // ISET: T32
    // Fields: Vn=0, op=0, N=0, M=0, size=0, U=0, D=0, Vm=0, Vd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF800200;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `field D 22 +: 1`
/// Requirement: FieldBoundary { field: "D", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_vsubl_t1a1_a_field_d_1_max_200_efc00200() {
    // Thumb encoding (32): 0xEFC00200
    // Test aarch32_VSUBL_T1A1_A field D = 1 (Max)
    // ISET: T32
    // Fields: Vm=0, Vn=0, op=0, N=0, size=0, U=0, D=1, Vd=0, M=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEFC00200;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `field size 20 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch32_vsubl_t1a1_a_field_size_0_min_200_ef800200() {
    // Thumb encoding (32): 0xEF800200
    // Test aarch32_VSUBL_T1A1_A field size = 0 (Min)
    // ISET: T32
    // Fields: Vd=0, Vm=0, size=0, D=0, Vn=0, op=0, N=0, U=0, M=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF800200;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `field size 20 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 1, boundary: PowerOfTwo }
/// 16-bit / halfword size
#[test]
fn test_aarch32_vsubl_t1a1_a_field_size_1_poweroftwo_200_ef900200() {
    // Thumb encoding (32): 0xEF900200
    // Test aarch32_VSUBL_T1A1_A field size = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: size=1, M=0, D=0, Vm=0, Vn=0, U=0, op=0, Vd=0, N=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF900200;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `field size 20 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 2, boundary: PowerOfTwo }
/// 32-bit / word size
#[test]
fn test_aarch32_vsubl_t1a1_a_field_size_2_poweroftwo_200_efa00200() {
    // Thumb encoding (32): 0xEFA00200
    // Test aarch32_VSUBL_T1A1_A field size = 2 (PowerOfTwo)
    // ISET: T32
    // Fields: size=2, M=0, op=0, D=0, Vm=0, Vd=0, Vn=0, U=0, N=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEFA00200;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `field size 20 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 3, boundary: Max }
/// 64-bit / doubleword size
#[test]
fn test_aarch32_vsubl_t1a1_a_field_size_3_max_200_efb00200() {
    // Thumb encoding (32): 0xEFB00200
    // Test aarch32_VSUBL_T1A1_A field size = 3 (Max)
    // ISET: T32
    // Fields: Vm=0, Vn=0, Vd=0, M=0, N=0, size=3, op=0, U=0, D=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEFB00200;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `field Vn 16 +: 4`
/// Requirement: FieldBoundary { field: "Vn", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_aarch32_vsubl_t1a1_a_field_vn_0_min_200_ef800200() {
    // Thumb encoding (32): 0xEF800200
    // Test aarch32_VSUBL_T1A1_A field Vn = 0 (Min)
    // ISET: T32
    // Fields: D=0, N=0, M=0, op=0, Vm=0, U=0, size=0, Vn=0, Vd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF800200;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `field Vn 16 +: 4`
/// Requirement: FieldBoundary { field: "Vn", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_aarch32_vsubl_t1a1_a_field_vn_1_poweroftwo_200_ef810200() {
    // Thumb encoding (32): 0xEF810200
    // Test aarch32_VSUBL_T1A1_A field Vn = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Vm=0, U=0, M=0, Vd=0, Vn=1, D=0, N=0, op=0, size=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF810200;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `field Vd 12 +: 4`
/// Requirement: FieldBoundary { field: "Vd", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_aarch32_vsubl_t1a1_a_field_vd_0_min_200_ef800200() {
    // Thumb encoding (32): 0xEF800200
    // Test aarch32_VSUBL_T1A1_A field Vd = 0 (Min)
    // ISET: T32
    // Fields: D=0, Vd=0, op=0, Vm=0, M=0, size=0, Vn=0, N=0, U=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF800200;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `field Vd 12 +: 4`
/// Requirement: FieldBoundary { field: "Vd", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_aarch32_vsubl_t1a1_a_field_vd_1_poweroftwo_200_ef801200() {
    // Thumb encoding (32): 0xEF801200
    // Test aarch32_VSUBL_T1A1_A field Vd = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: U=0, N=0, op=0, D=0, Vd=1, size=0, Vm=0, M=0, Vn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF801200;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `field op 8 +: 1`
/// Requirement: FieldBoundary { field: "op", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_vsubl_t1a1_a_field_op_0_min_200_ef800200() {
    // Thumb encoding (32): 0xEF800200
    // Test aarch32_VSUBL_T1A1_A field op = 0 (Min)
    // ISET: T32
    // Fields: op=0, D=0, Vn=0, size=0, N=0, Vm=0, Vd=0, M=0, U=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF800200;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `field op 8 +: 1`
/// Requirement: FieldBoundary { field: "op", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_vsubl_t1a1_a_field_op_1_max_200_ef800300() {
    // Thumb encoding (32): 0xEF800300
    // Test aarch32_VSUBL_T1A1_A field op = 1 (Max)
    // ISET: T32
    // Fields: N=0, U=0, Vn=0, D=0, Vm=0, M=0, size=0, Vd=0, op=1
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF800300;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `field N 7 +: 1`
/// Requirement: FieldBoundary { field: "N", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_vsubl_t1a1_a_field_n_0_min_200_ef800200() {
    // Thumb encoding (32): 0xEF800200
    // Test aarch32_VSUBL_T1A1_A field N = 0 (Min)
    // ISET: T32
    // Fields: op=0, D=0, N=0, U=0, Vd=0, size=0, Vm=0, M=0, Vn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF800200;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `field N 7 +: 1`
/// Requirement: FieldBoundary { field: "N", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_vsubl_t1a1_a_field_n_1_max_200_ef800280() {
    // Thumb encoding (32): 0xEF800280
    // Test aarch32_VSUBL_T1A1_A field N = 1 (Max)
    // ISET: T32
    // Fields: size=0, U=0, D=0, N=1, Vm=0, M=0, op=0, Vd=0, Vn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF800280;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `field M 5 +: 1`
/// Requirement: FieldBoundary { field: "M", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_vsubl_t1a1_a_field_m_0_min_200_ef800200() {
    // Thumb encoding (32): 0xEF800200
    // Test aarch32_VSUBL_T1A1_A field M = 0 (Min)
    // ISET: T32
    // Fields: Vn=0, D=0, U=0, op=0, M=0, size=0, Vm=0, N=0, Vd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF800200;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `field M 5 +: 1`
/// Requirement: FieldBoundary { field: "M", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_vsubl_t1a1_a_field_m_1_max_200_ef800220() {
    // Thumb encoding (32): 0xEF800220
    // Test aarch32_VSUBL_T1A1_A field M = 1 (Max)
    // ISET: T32
    // Fields: D=0, U=0, Vd=0, op=0, Vn=0, size=0, N=0, Vm=0, M=1
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF800220;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `field Vm 0 +: 4`
/// Requirement: FieldBoundary { field: "Vm", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_aarch32_vsubl_t1a1_a_field_vm_0_min_200_ef800200() {
    // Thumb encoding (32): 0xEF800200
    // Test aarch32_VSUBL_T1A1_A field Vm = 0 (Min)
    // ISET: T32
    // Fields: D=0, M=0, size=0, Vm=0, op=0, U=0, Vd=0, N=0, Vn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF800200;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `field Vm 0 +: 4`
/// Requirement: FieldBoundary { field: "Vm", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_aarch32_vsubl_t1a1_a_field_vm_1_poweroftwo_200_ef800201() {
    // Thumb encoding (32): 0xEF800201
    // Test aarch32_VSUBL_T1A1_A field Vm = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: size=0, U=0, Vn=0, op=0, M=0, Vd=0, Vm=1, N=0, D=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF800201;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// U=0 (minimum value)
#[test]
fn test_aarch32_vsubl_t1a1_a_combo_0_200_ef800200() {
    // Thumb encoding (32): 0xEF800200
    // Test aarch32_VSUBL_T1A1_A field combination: U=0, D=0, size=0, Vn=0, Vd=0, op=0, N=0, M=0, Vm=0
    // ISET: T32
    // Fields: M=0, Vm=0, Vn=0, size=0, D=0, U=0, Vd=0, N=0, op=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF800200;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `field size = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "size", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch32_vsubl_t1a1_a_special_size_0_size_variant_0_512_ef800200() {
    // Thumb encoding (32): 0xEF800200
    // Test aarch32_VSUBL_T1A1_A special value size = 0 (Size variant 0)
    // ISET: T32
    // Fields: N=0, D=0, Vd=0, op=0, Vm=0, Vn=0, size=0, M=0, U=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF800200;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `field size = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "size", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch32_vsubl_t1a1_a_special_size_1_size_variant_1_512_ef900200() {
    // Thumb encoding (32): 0xEF900200
    // Test aarch32_VSUBL_T1A1_A special value size = 1 (Size variant 1)
    // ISET: T32
    // Fields: D=0, size=1, N=0, Vd=0, Vm=0, op=0, M=0, U=0, Vn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF900200;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `field size = 2 (Size variant 2)`
/// Requirement: FieldSpecial { field: "size", value: 2, meaning: "Size variant 2" }
/// Size variant 2
#[test]
fn test_aarch32_vsubl_t1a1_a_special_size_2_size_variant_2_512_efa00200() {
    // Thumb encoding (32): 0xEFA00200
    // Test aarch32_VSUBL_T1A1_A special value size = 2 (Size variant 2)
    // ISET: T32
    // Fields: M=0, op=0, D=0, size=2, N=0, Vd=0, Vm=0, U=0, Vn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEFA00200;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `field size = 3 (Size variant 3)`
/// Requirement: FieldSpecial { field: "size", value: 3, meaning: "Size variant 3" }
/// Size variant 3
#[test]
fn test_aarch32_vsubl_t1a1_a_special_size_3_size_variant_3_512_efb00200() {
    // Thumb encoding (32): 0xEFB00200
    // Test aarch32_VSUBL_T1A1_A special value size = 3 (Size variant 3)
    // ISET: T32
    // Fields: M=0, D=0, U=0, Vd=0, op=0, size=3, Vn=0, Vm=0, N=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEFB00200;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `Binary { op: Eq, lhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: "Vd" }), indices: [Single(LitInt(0))] }, rhs: Binary { op: Or, lhs: LitBits([true]), rhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "op" }), rhs: Binary { op: And, lhs: LitBits([true]), rhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: "Vn" }), indices: [Single(LitInt(0))] } } }, rhs: LitBits([true]) } } }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: \"Vd\" }), indices: [Single(LitInt(0))] }, rhs: Binary { op: Or, lhs: LitBits([true]), rhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"op\" }), rhs: Binary { op: And, lhs: LitBits([true]), rhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: \"Vn\" }), indices: [Single(LitInt(0))] } } }, rhs: LitBits([true]) } } }" }
/// triggers Undefined
#[test]
fn test_aarch32_vsubl_t1a1_a_invalid_0_200_ef800200() {
    // Thumb encoding (32): 0xEF800200
    // Test aarch32_VSUBL_T1A1_A invalid encoding: Binary { op: Eq, lhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: "Vd" }), indices: [Single(LitInt(0))] }, rhs: Binary { op: Or, lhs: LitBits([true]), rhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "op" }), rhs: Binary { op: And, lhs: LitBits([true]), rhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: "Vn" }), indices: [Single(LitInt(0))] } } }, rhs: LitBits([true]) } } }
    // ISET: T32
    // Fields: Vn=0, Vm=0, op=0, N=0, D=0, U=0, Vd=0, M=0, size=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF800200;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || !matches!(exit.unwrap(), CpuExit::Continue),
        "expected UNDEFINED for T32 encoding"
    );
}

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `Unconditional UNDEFINED`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNDEFINED" }
/// triggers Undefined
#[test]
fn test_aarch32_vsubl_t1a1_a_invalid_1_200_ef800200() {
    // Thumb encoding (32): 0xEF800200
    // Test aarch32_VSUBL_T1A1_A invalid encoding: Unconditional UNDEFINED
    // ISET: T32
    // Fields: U=0, size=0, Vn=0, N=0, Vm=0, Vd=0, op=0, M=0, D=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF800200;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || !matches!(exit.unwrap(), CpuExit::Continue),
        "expected UNDEFINED for T32 encoding"
    );
}

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `ADD X0, X1, #10`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// simple addition/subtraction (32)
#[test]
fn test_aarch32_vsubl_t1a1_a_add_oracle_32_0_f2802a20() {
    // Test ADD 32-bit: simple addition/subtraction (with oracle verification)
    // Encoding: 0xF2802A20
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x64);
    let encoding: u32 = 0xF2802A20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFF6,
        "X0 should be 0xFFFFFFFFFFFFFFF6"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `ADD X0, X1, #10`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// simple addition/subtraction (64)
#[test]
fn test_aarch32_vsubl_t1a1_a_add_oracle_64_0_f2802a20() {
    // Test ADD 64-bit: simple addition/subtraction (with oracle verification)
    // Encoding: 0xF2802A20
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x64);
    let encoding: u32 = 0xF2802A20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFF6,
        "X0 should be 0xFFFFFFFFFFFFFFF6"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `ADD X0, X1, #0`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// zero operands (32)
#[test]
fn test_aarch32_vsubl_t1a1_a_add_oracle_32_1_f2800220() {
    // Test ADD 32-bit: zero operands (with oracle verification)
    // Encoding: 0xF2800220
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2800220;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x0, "X0 should be 0x0000000000000000");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z flag should be true");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `ADD X0, X1, #0`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// zero operands (64)
#[test]
fn test_aarch32_vsubl_t1a1_a_add_oracle_64_1_f2800220() {
    // Test ADD 64-bit: zero operands (with oracle verification)
    // Encoding: 0xF2800220
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2800220;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x0, "X0 should be 0x0000000000000000");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z flag should be true");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `ADD X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// small values (32)
#[test]
fn test_aarch32_vsubl_t1a1_a_add_oracle_32_2_f2800620() {
    // Test ADD 32-bit: small values (with oracle verification)
    // Encoding: 0xF2800620
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x1);
    let encoding: u32 = 0xF2800620;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `ADD X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// small values (64)
#[test]
fn test_aarch32_vsubl_t1a1_a_add_oracle_64_2_f2800620() {
    // Test ADD 64-bit: small values (with oracle verification)
    // Encoding: 0xF2800620
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x1);
    let encoding: u32 = 0xF2800620;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `ADD X0, X1, #4095`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// max imm12 unshifted (32)
#[test]
fn test_aarch32_vsubl_t1a1_a_add_oracle_32_3_f2bffe20() {
    // Test ADD 32-bit: max imm12 unshifted (with oracle verification)
    // Encoding: 0xF2BFFE20
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2BFFE20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFF001,
        "X0 should be 0xFFFFFFFFFFFFF001"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `ADD X0, X1, #4095`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// max imm12 unshifted (64)
#[test]
fn test_aarch32_vsubl_t1a1_a_add_oracle_64_3_f2bffe20() {
    // Test ADD 64-bit: max imm12 unshifted (with oracle verification)
    // Encoding: 0xF2BFFE20
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2BFFE20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFF001,
        "X0 should be 0xFFFFFFFFFFFFF001"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `ADD X0, X1, #4095, LSL #12`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// max imm12 shifted (32)
#[test]
fn test_aarch32_vsubl_t1a1_a_add_oracle_32_4_f2fffe20() {
    // Test ADD 32-bit: max imm12 shifted (with oracle verification)
    // Encoding: 0xF2FFFE20
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2FFFE20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFF001000,
        "X0 should be 0xFFFFFFFFFF001000"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `ADD X0, X1, #4095, LSL #12`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// max imm12 shifted (64)
#[test]
fn test_aarch32_vsubl_t1a1_a_add_oracle_64_4_f2fffe20() {
    // Test ADD 64-bit: max imm12 shifted (with oracle verification)
    // Encoding: 0xF2FFFE20
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2FFFE20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFF001000,
        "X0 should be 0xFFFFFFFFFF001000"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `ADD X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// max u64 operand (32)
#[test]
fn test_aarch32_vsubl_t1a1_a_add_oracle_32_5_f2800620() {
    // Test ADD 32-bit: max u64 operand (with oracle verification)
    // Encoding: 0xF2800620
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800620;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `ADD X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// max u64 operand (64)
#[test]
fn test_aarch32_vsubl_t1a1_a_add_oracle_64_5_f2800620() {
    // Test ADD 64-bit: max u64 operand (with oracle verification)
    // Encoding: 0xF2800620
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800620;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `ADD X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// zero result (for sub 1-1) (32)
#[test]
fn test_aarch32_vsubl_t1a1_a_add_oracle_32_6_f2800620() {
    // Test ADD 32-bit: zero result (for sub 1-1) (with oracle verification)
    // Encoding: 0xF2800620
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2800620;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `ADD X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// zero result (for sub 1-1) (64)
#[test]
fn test_aarch32_vsubl_t1a1_a_add_oracle_64_6_f2800620() {
    // Test ADD 64-bit: zero result (for sub 1-1) (with oracle verification)
    // Encoding: 0xF2800620
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2800620;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `ADD X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// signed overflow boundary 64-bit (32)
#[test]
fn test_aarch32_vsubl_t1a1_a_add_oracle_32_7_f2800620() {
    // Test ADD 32-bit: signed overflow boundary 64-bit (with oracle verification)
    // Encoding: 0xF2800620
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800620;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `ADD X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// signed overflow boundary 64-bit (64)
#[test]
fn test_aarch32_vsubl_t1a1_a_add_oracle_64_7_f2800620() {
    // Test ADD 64-bit: signed overflow boundary 64-bit (with oracle verification)
    // Encoding: 0xF2800620
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800620;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `ADD X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// signed overflow boundary 32-bit (32)
#[test]
fn test_aarch32_vsubl_t1a1_a_add_oracle_32_8_f2800620() {
    // Test ADD 32-bit: signed overflow boundary 32-bit (with oracle verification)
    // Encoding: 0xF2800620
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x7FFFFFFF);
    let encoding: u32 = 0xF2800620;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `ADD X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// signed overflow boundary 32-bit (64)
#[test]
fn test_aarch32_vsubl_t1a1_a_add_oracle_64_8_f2800620() {
    // Test ADD 64-bit: signed overflow boundary 32-bit (with oracle verification)
    // Encoding: 0xF2800620
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x7FFFFFFF);
    let encoding: u32 = 0xF2800620;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `ADD X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// unsigned overflow 64-bit (32)
#[test]
fn test_aarch32_vsubl_t1a1_a_add_oracle_32_9_f2800620() {
    // Test ADD 32-bit: unsigned overflow 64-bit (with oracle verification)
    // Encoding: 0xF2800620
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800620;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `ADD X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// unsigned overflow 64-bit (64)
#[test]
fn test_aarch32_vsubl_t1a1_a_add_oracle_64_9_f2800620() {
    // Test ADD 64-bit: unsigned overflow 64-bit (with oracle verification)
    // Encoding: 0xF2800620
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800620;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `ADD X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// unsigned overflow 32-bit (32)
#[test]
fn test_aarch32_vsubl_t1a1_a_add_oracle_32_10_f2800620() {
    // Test ADD 32-bit: unsigned overflow 32-bit (with oracle verification)
    // Encoding: 0xF2800620
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800620;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `ADD X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// unsigned overflow 32-bit (64)
#[test]
fn test_aarch32_vsubl_t1a1_a_add_oracle_64_10_f2800620() {
    // Test ADD 64-bit: unsigned overflow 32-bit (with oracle verification)
    // Encoding: 0xF2800620
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800620;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `ADD SP, X1, #10`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "writes to stack pointer" }
/// SP destination (32)
#[test]
fn test_aarch32_vsubl_t1a1_a_add_oracle_32_rd31_sp_f2802a3f() {
    // Test ADD 32-bit with Rd=31 (SP)
    // Encoding: 0xF2802A3F
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x64);
    let encoding: u32 = 0xF2802A3F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `ADD SP, X1, #10`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "writes to stack pointer" }
/// SP destination (64)
#[test]
fn test_aarch32_vsubl_t1a1_a_add_oracle_64_rd31_sp_f2802a3f() {
    // Test ADD 64-bit with Rd=31 (SP)
    // Encoding: 0xF2802A3F
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x64);
    let encoding: u32 = 0xF2802A3F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `ADDS X0, X1, #10`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// simple addition/subtraction (32)
#[test]
fn test_aarch32_vsubl_t1a1_a_adds_oracle_32_0_f2802a20() {
    // Test ADDS 32-bit: simple addition/subtraction (with oracle verification)
    // Encoding: 0xF2802A20
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x64);
    let encoding: u32 = 0xF2802A20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFF6,
        "X0 should be 0xFFFFFFFFFFFFFFF6"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `ADDS X0, X1, #10`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// simple addition/subtraction (64)
#[test]
fn test_aarch32_vsubl_t1a1_a_adds_oracle_64_0_f2802a20() {
    // Test ADDS 64-bit: simple addition/subtraction (with oracle verification)
    // Encoding: 0xF2802A20
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x64);
    let encoding: u32 = 0xF2802A20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFF6,
        "X0 should be 0xFFFFFFFFFFFFFFF6"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `ADDS X0, X1, #0`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// zero operands (32)
#[test]
fn test_aarch32_vsubl_t1a1_a_adds_oracle_32_1_f2800220() {
    // Test ADDS 32-bit: zero operands (with oracle verification)
    // Encoding: 0xF2800220
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2800220;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x0, "X0 should be 0x0000000000000000");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z flag should be true");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `ADDS X0, X1, #0`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// zero operands (64)
#[test]
fn test_aarch32_vsubl_t1a1_a_adds_oracle_64_1_f2800220() {
    // Test ADDS 64-bit: zero operands (with oracle verification)
    // Encoding: 0xF2800220
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2800220;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x0, "X0 should be 0x0000000000000000");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z flag should be true");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `ADDS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// small values (32)
#[test]
fn test_aarch32_vsubl_t1a1_a_adds_oracle_32_2_f2800620() {
    // Test ADDS 32-bit: small values (with oracle verification)
    // Encoding: 0xF2800620
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x1);
    let encoding: u32 = 0xF2800620;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `ADDS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// small values (64)
#[test]
fn test_aarch32_vsubl_t1a1_a_adds_oracle_64_2_f2800620() {
    // Test ADDS 64-bit: small values (with oracle verification)
    // Encoding: 0xF2800620
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x1);
    let encoding: u32 = 0xF2800620;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `ADDS X0, X1, #4095`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// max imm12 unshifted (32)
#[test]
fn test_aarch32_vsubl_t1a1_a_adds_oracle_32_3_f2bffe20() {
    // Test ADDS 32-bit: max imm12 unshifted (with oracle verification)
    // Encoding: 0xF2BFFE20
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2BFFE20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFF001,
        "X0 should be 0xFFFFFFFFFFFFF001"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `ADDS X0, X1, #4095`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// max imm12 unshifted (64)
#[test]
fn test_aarch32_vsubl_t1a1_a_adds_oracle_64_3_f2bffe20() {
    // Test ADDS 64-bit: max imm12 unshifted (with oracle verification)
    // Encoding: 0xF2BFFE20
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2BFFE20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFF001,
        "X0 should be 0xFFFFFFFFFFFFF001"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `ADDS X0, X1, #4095, LSL #12`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// max imm12 shifted (32)
#[test]
fn test_aarch32_vsubl_t1a1_a_adds_oracle_32_4_f2fffe20() {
    // Test ADDS 32-bit: max imm12 shifted (with oracle verification)
    // Encoding: 0xF2FFFE20
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2FFFE20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFF001000,
        "X0 should be 0xFFFFFFFFFF001000"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `ADDS X0, X1, #4095, LSL #12`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// max imm12 shifted (64)
#[test]
fn test_aarch32_vsubl_t1a1_a_adds_oracle_64_4_f2fffe20() {
    // Test ADDS 64-bit: max imm12 shifted (with oracle verification)
    // Encoding: 0xF2FFFE20
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2FFFE20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFF001000,
        "X0 should be 0xFFFFFFFFFF001000"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `ADDS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// max u64 operand (32)
#[test]
fn test_aarch32_vsubl_t1a1_a_adds_oracle_32_5_f2800620() {
    // Test ADDS 32-bit: max u64 operand (with oracle verification)
    // Encoding: 0xF2800620
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800620;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `ADDS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// max u64 operand (64)
#[test]
fn test_aarch32_vsubl_t1a1_a_adds_oracle_64_5_f2800620() {
    // Test ADDS 64-bit: max u64 operand (with oracle verification)
    // Encoding: 0xF2800620
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800620;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `ADDS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// zero result (for sub 1-1) (32)
#[test]
fn test_aarch32_vsubl_t1a1_a_adds_oracle_32_6_f2800620() {
    // Test ADDS 32-bit: zero result (for sub 1-1) (with oracle verification)
    // Encoding: 0xF2800620
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2800620;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `ADDS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// zero result (for sub 1-1) (64)
#[test]
fn test_aarch32_vsubl_t1a1_a_adds_oracle_64_6_f2800620() {
    // Test ADDS 64-bit: zero result (for sub 1-1) (with oracle verification)
    // Encoding: 0xF2800620
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2800620;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `ADDS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// signed overflow boundary 64-bit (32)
#[test]
fn test_aarch32_vsubl_t1a1_a_adds_oracle_32_7_f2800620() {
    // Test ADDS 32-bit: signed overflow boundary 64-bit (with oracle verification)
    // Encoding: 0xF2800620
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800620;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `ADDS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// signed overflow boundary 64-bit (64)
#[test]
fn test_aarch32_vsubl_t1a1_a_adds_oracle_64_7_f2800620() {
    // Test ADDS 64-bit: signed overflow boundary 64-bit (with oracle verification)
    // Encoding: 0xF2800620
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800620;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `ADDS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// signed overflow boundary 32-bit (32)
#[test]
fn test_aarch32_vsubl_t1a1_a_adds_oracle_32_8_f2800620() {
    // Test ADDS 32-bit: signed overflow boundary 32-bit (with oracle verification)
    // Encoding: 0xF2800620
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x7FFFFFFF);
    let encoding: u32 = 0xF2800620;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `ADDS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// signed overflow boundary 32-bit (64)
#[test]
fn test_aarch32_vsubl_t1a1_a_adds_oracle_64_8_f2800620() {
    // Test ADDS 64-bit: signed overflow boundary 32-bit (with oracle verification)
    // Encoding: 0xF2800620
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x7FFFFFFF);
    let encoding: u32 = 0xF2800620;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `ADDS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// unsigned overflow 64-bit (32)
#[test]
fn test_aarch32_vsubl_t1a1_a_adds_oracle_32_9_f2800620() {
    // Test ADDS 32-bit: unsigned overflow 64-bit (with oracle verification)
    // Encoding: 0xF2800620
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800620;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `ADDS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// unsigned overflow 64-bit (64)
#[test]
fn test_aarch32_vsubl_t1a1_a_adds_oracle_64_9_f2800620() {
    // Test ADDS 64-bit: unsigned overflow 64-bit (with oracle verification)
    // Encoding: 0xF2800620
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800620;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `ADDS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// unsigned overflow 32-bit (32)
#[test]
fn test_aarch32_vsubl_t1a1_a_adds_oracle_32_10_f2800620() {
    // Test ADDS 32-bit: unsigned overflow 32-bit (with oracle verification)
    // Encoding: 0xF2800620
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800620;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `ADDS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// unsigned overflow 32-bit (64)
#[test]
fn test_aarch32_vsubl_t1a1_a_adds_oracle_64_10_f2800620() {
    // Test ADDS 64-bit: unsigned overflow 32-bit (with oracle verification)
    // Encoding: 0xF2800620
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800620;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `ADDS ZR, X1, #10`
/// Requirement: RegisterSpecial { reg: Zr, behavior: "result discarded, flags set" }
/// ZR destination (32)
#[test]
fn test_aarch32_vsubl_t1a1_a_adds_oracle_32_rd31_zr_f2802a3f() {
    // Test ADDS 32-bit with Rd=31 (ZR)
    // Encoding: 0xF2802A3F
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x64);
    let encoding: u32 = 0xF2802A3F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `ADDS ZR, X1, #10`
/// Requirement: RegisterSpecial { reg: Zr, behavior: "result discarded, flags set" }
/// ZR destination (64)
#[test]
fn test_aarch32_vsubl_t1a1_a_adds_oracle_64_rd31_zr_f2802a3f() {
    // Test ADDS 64-bit with Rd=31 (ZR)
    // Encoding: 0xF2802A3F
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x64);
    let encoding: u32 = 0xF2802A3F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `SUB X0, X1, #10`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// simple addition/subtraction (32)
#[test]
fn test_aarch32_vsubl_t1a1_a_sub_oracle_32_0_f2802a20() {
    // Test SUB 32-bit: simple addition/subtraction (with oracle verification)
    // Encoding: 0xF2802A20
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x64);
    let encoding: u32 = 0xF2802A20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFF6,
        "X0 should be 0xFFFFFFFFFFFFFFF6"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `SUB X0, X1, #10`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// simple addition/subtraction (64)
#[test]
fn test_aarch32_vsubl_t1a1_a_sub_oracle_64_0_f2802a20() {
    // Test SUB 64-bit: simple addition/subtraction (with oracle verification)
    // Encoding: 0xF2802A20
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x64);
    let encoding: u32 = 0xF2802A20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFF6,
        "X0 should be 0xFFFFFFFFFFFFFFF6"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `SUB X0, X1, #0`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// zero operands (32)
#[test]
fn test_aarch32_vsubl_t1a1_a_sub_oracle_32_1_f2800220() {
    // Test SUB 32-bit: zero operands (with oracle verification)
    // Encoding: 0xF2800220
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2800220;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x0, "X0 should be 0x0000000000000000");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z flag should be true");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `SUB X0, X1, #0`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// zero operands (64)
#[test]
fn test_aarch32_vsubl_t1a1_a_sub_oracle_64_1_f2800220() {
    // Test SUB 64-bit: zero operands (with oracle verification)
    // Encoding: 0xF2800220
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2800220;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x0, "X0 should be 0x0000000000000000");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z flag should be true");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `SUB X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// small values (32)
#[test]
fn test_aarch32_vsubl_t1a1_a_sub_oracle_32_2_f2800620() {
    // Test SUB 32-bit: small values (with oracle verification)
    // Encoding: 0xF2800620
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x1);
    let encoding: u32 = 0xF2800620;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `SUB X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// small values (64)
#[test]
fn test_aarch32_vsubl_t1a1_a_sub_oracle_64_2_f2800620() {
    // Test SUB 64-bit: small values (with oracle verification)
    // Encoding: 0xF2800620
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x1);
    let encoding: u32 = 0xF2800620;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `SUB X0, X1, #4095`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// max imm12 unshifted (32)
#[test]
fn test_aarch32_vsubl_t1a1_a_sub_oracle_32_3_f2bffe20() {
    // Test SUB 32-bit: max imm12 unshifted (with oracle verification)
    // Encoding: 0xF2BFFE20
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2BFFE20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFF001,
        "X0 should be 0xFFFFFFFFFFFFF001"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `SUB X0, X1, #4095`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// max imm12 unshifted (64)
#[test]
fn test_aarch32_vsubl_t1a1_a_sub_oracle_64_3_f2bffe20() {
    // Test SUB 64-bit: max imm12 unshifted (with oracle verification)
    // Encoding: 0xF2BFFE20
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2BFFE20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFF001,
        "X0 should be 0xFFFFFFFFFFFFF001"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `SUB X0, X1, #4095, LSL #12`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// max imm12 shifted (32)
#[test]
fn test_aarch32_vsubl_t1a1_a_sub_oracle_32_4_f2fffe20() {
    // Test SUB 32-bit: max imm12 shifted (with oracle verification)
    // Encoding: 0xF2FFFE20
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2FFFE20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFF001000,
        "X0 should be 0xFFFFFFFFFF001000"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `SUB X0, X1, #4095, LSL #12`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// max imm12 shifted (64)
#[test]
fn test_aarch32_vsubl_t1a1_a_sub_oracle_64_4_f2fffe20() {
    // Test SUB 64-bit: max imm12 shifted (with oracle verification)
    // Encoding: 0xF2FFFE20
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2FFFE20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFF001000,
        "X0 should be 0xFFFFFFFFFF001000"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `SUB X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// max u64 operand (32)
#[test]
fn test_aarch32_vsubl_t1a1_a_sub_oracle_32_5_f2800620() {
    // Test SUB 32-bit: max u64 operand (with oracle verification)
    // Encoding: 0xF2800620
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800620;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `SUB X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// max u64 operand (64)
#[test]
fn test_aarch32_vsubl_t1a1_a_sub_oracle_64_5_f2800620() {
    // Test SUB 64-bit: max u64 operand (with oracle verification)
    // Encoding: 0xF2800620
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800620;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `SUB X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// zero result (for sub 1-1) (32)
#[test]
fn test_aarch32_vsubl_t1a1_a_sub_oracle_32_6_f2800620() {
    // Test SUB 32-bit: zero result (for sub 1-1) (with oracle verification)
    // Encoding: 0xF2800620
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2800620;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `SUB X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// zero result (for sub 1-1) (64)
#[test]
fn test_aarch32_vsubl_t1a1_a_sub_oracle_64_6_f2800620() {
    // Test SUB 64-bit: zero result (for sub 1-1) (with oracle verification)
    // Encoding: 0xF2800620
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2800620;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `SUB X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// signed overflow boundary 64-bit (32)
#[test]
fn test_aarch32_vsubl_t1a1_a_sub_oracle_32_7_f2800620() {
    // Test SUB 32-bit: signed overflow boundary 64-bit (with oracle verification)
    // Encoding: 0xF2800620
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800620;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `SUB X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// signed overflow boundary 64-bit (64)
#[test]
fn test_aarch32_vsubl_t1a1_a_sub_oracle_64_7_f2800620() {
    // Test SUB 64-bit: signed overflow boundary 64-bit (with oracle verification)
    // Encoding: 0xF2800620
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800620;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `SUB X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// signed overflow boundary 32-bit (32)
#[test]
fn test_aarch32_vsubl_t1a1_a_sub_oracle_32_8_f2800620() {
    // Test SUB 32-bit: signed overflow boundary 32-bit (with oracle verification)
    // Encoding: 0xF2800620
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x7FFFFFFF);
    let encoding: u32 = 0xF2800620;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `SUB X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// signed overflow boundary 32-bit (64)
#[test]
fn test_aarch32_vsubl_t1a1_a_sub_oracle_64_8_f2800620() {
    // Test SUB 64-bit: signed overflow boundary 32-bit (with oracle verification)
    // Encoding: 0xF2800620
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x7FFFFFFF);
    let encoding: u32 = 0xF2800620;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `SUB X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// unsigned overflow 64-bit (32)
#[test]
fn test_aarch32_vsubl_t1a1_a_sub_oracle_32_9_f2800620() {
    // Test SUB 32-bit: unsigned overflow 64-bit (with oracle verification)
    // Encoding: 0xF2800620
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800620;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `SUB X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// unsigned overflow 64-bit (64)
#[test]
fn test_aarch32_vsubl_t1a1_a_sub_oracle_64_9_f2800620() {
    // Test SUB 64-bit: unsigned overflow 64-bit (with oracle verification)
    // Encoding: 0xF2800620
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800620;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `SUB X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// unsigned overflow 32-bit (32)
#[test]
fn test_aarch32_vsubl_t1a1_a_sub_oracle_32_10_f2800620() {
    // Test SUB 32-bit: unsigned overflow 32-bit (with oracle verification)
    // Encoding: 0xF2800620
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800620;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `SUB X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// unsigned overflow 32-bit (64)
#[test]
fn test_aarch32_vsubl_t1a1_a_sub_oracle_64_10_f2800620() {
    // Test SUB 64-bit: unsigned overflow 32-bit (with oracle verification)
    // Encoding: 0xF2800620
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800620;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `SUB SP, X1, #10`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "writes to stack pointer" }
/// SP destination (32)
#[test]
fn test_aarch32_vsubl_t1a1_a_sub_oracle_32_rd31_sp_f2802a3f() {
    // Test SUB 32-bit with Rd=31 (SP)
    // Encoding: 0xF2802A3F
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x64);
    let encoding: u32 = 0xF2802A3F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `SUB SP, X1, #10`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "writes to stack pointer" }
/// SP destination (64)
#[test]
fn test_aarch32_vsubl_t1a1_a_sub_oracle_64_rd31_sp_f2802a3f() {
    // Test SUB 64-bit with Rd=31 (SP)
    // Encoding: 0xF2802A3F
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x64);
    let encoding: u32 = 0xF2802A3F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `SUBS X0, X1, #10`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// simple addition/subtraction (32)
#[test]
fn test_aarch32_vsubl_t1a1_a_subs_oracle_32_0_f2802a20() {
    // Test SUBS 32-bit: simple addition/subtraction (with oracle verification)
    // Encoding: 0xF2802A20
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x64);
    let encoding: u32 = 0xF2802A20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFF6,
        "X0 should be 0xFFFFFFFFFFFFFFF6"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `SUBS X0, X1, #10`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// simple addition/subtraction (64)
#[test]
fn test_aarch32_vsubl_t1a1_a_subs_oracle_64_0_f2802a20() {
    // Test SUBS 64-bit: simple addition/subtraction (with oracle verification)
    // Encoding: 0xF2802A20
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x64);
    let encoding: u32 = 0xF2802A20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFF6,
        "X0 should be 0xFFFFFFFFFFFFFFF6"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `SUBS X0, X1, #0`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// zero operands (32)
#[test]
fn test_aarch32_vsubl_t1a1_a_subs_oracle_32_1_f2800220() {
    // Test SUBS 32-bit: zero operands (with oracle verification)
    // Encoding: 0xF2800220
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2800220;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x0, "X0 should be 0x0000000000000000");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z flag should be true");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `SUBS X0, X1, #0`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// zero operands (64)
#[test]
fn test_aarch32_vsubl_t1a1_a_subs_oracle_64_1_f2800220() {
    // Test SUBS 64-bit: zero operands (with oracle verification)
    // Encoding: 0xF2800220
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2800220;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x0, "X0 should be 0x0000000000000000");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z flag should be true");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `SUBS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// small values (32)
#[test]
fn test_aarch32_vsubl_t1a1_a_subs_oracle_32_2_f2800620() {
    // Test SUBS 32-bit: small values (with oracle verification)
    // Encoding: 0xF2800620
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x1);
    let encoding: u32 = 0xF2800620;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `SUBS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// small values (64)
#[test]
fn test_aarch32_vsubl_t1a1_a_subs_oracle_64_2_f2800620() {
    // Test SUBS 64-bit: small values (with oracle verification)
    // Encoding: 0xF2800620
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x1);
    let encoding: u32 = 0xF2800620;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `SUBS X0, X1, #4095`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// max imm12 unshifted (32)
#[test]
fn test_aarch32_vsubl_t1a1_a_subs_oracle_32_3_f2bffe20() {
    // Test SUBS 32-bit: max imm12 unshifted (with oracle verification)
    // Encoding: 0xF2BFFE20
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2BFFE20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFF001,
        "X0 should be 0xFFFFFFFFFFFFF001"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `SUBS X0, X1, #4095`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// max imm12 unshifted (64)
#[test]
fn test_aarch32_vsubl_t1a1_a_subs_oracle_64_3_f2bffe20() {
    // Test SUBS 64-bit: max imm12 unshifted (with oracle verification)
    // Encoding: 0xF2BFFE20
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2BFFE20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFF001,
        "X0 should be 0xFFFFFFFFFFFFF001"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `SUBS X0, X1, #4095, LSL #12`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// max imm12 shifted (32)
#[test]
fn test_aarch32_vsubl_t1a1_a_subs_oracle_32_4_f2fffe20() {
    // Test SUBS 32-bit: max imm12 shifted (with oracle verification)
    // Encoding: 0xF2FFFE20
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2FFFE20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFF001000,
        "X0 should be 0xFFFFFFFFFF001000"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `SUBS X0, X1, #4095, LSL #12`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// max imm12 shifted (64)
#[test]
fn test_aarch32_vsubl_t1a1_a_subs_oracle_64_4_f2fffe20() {
    // Test SUBS 64-bit: max imm12 shifted (with oracle verification)
    // Encoding: 0xF2FFFE20
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2FFFE20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFF001000,
        "X0 should be 0xFFFFFFFFFF001000"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `SUBS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// max u64 operand (32)
#[test]
fn test_aarch32_vsubl_t1a1_a_subs_oracle_32_5_f2800620() {
    // Test SUBS 32-bit: max u64 operand (with oracle verification)
    // Encoding: 0xF2800620
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800620;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `SUBS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// max u64 operand (64)
#[test]
fn test_aarch32_vsubl_t1a1_a_subs_oracle_64_5_f2800620() {
    // Test SUBS 64-bit: max u64 operand (with oracle verification)
    // Encoding: 0xF2800620
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800620;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `SUBS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// zero result (for sub 1-1) (32)
#[test]
fn test_aarch32_vsubl_t1a1_a_subs_oracle_32_6_f2800620() {
    // Test SUBS 32-bit: zero result (for sub 1-1) (with oracle verification)
    // Encoding: 0xF2800620
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2800620;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `SUBS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// zero result (for sub 1-1) (64)
#[test]
fn test_aarch32_vsubl_t1a1_a_subs_oracle_64_6_f2800620() {
    // Test SUBS 64-bit: zero result (for sub 1-1) (with oracle verification)
    // Encoding: 0xF2800620
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2800620;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `SUBS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// signed overflow boundary 64-bit (32)
#[test]
fn test_aarch32_vsubl_t1a1_a_subs_oracle_32_7_f2800620() {
    // Test SUBS 32-bit: signed overflow boundary 64-bit (with oracle verification)
    // Encoding: 0xF2800620
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800620;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `SUBS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// signed overflow boundary 64-bit (64)
#[test]
fn test_aarch32_vsubl_t1a1_a_subs_oracle_64_7_f2800620() {
    // Test SUBS 64-bit: signed overflow boundary 64-bit (with oracle verification)
    // Encoding: 0xF2800620
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800620;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `SUBS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// signed overflow boundary 32-bit (32)
#[test]
fn test_aarch32_vsubl_t1a1_a_subs_oracle_32_8_f2800620() {
    // Test SUBS 32-bit: signed overflow boundary 32-bit (with oracle verification)
    // Encoding: 0xF2800620
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x7FFFFFFF);
    let encoding: u32 = 0xF2800620;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `SUBS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// signed overflow boundary 32-bit (64)
#[test]
fn test_aarch32_vsubl_t1a1_a_subs_oracle_64_8_f2800620() {
    // Test SUBS 64-bit: signed overflow boundary 32-bit (with oracle verification)
    // Encoding: 0xF2800620
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x7FFFFFFF);
    let encoding: u32 = 0xF2800620;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `SUBS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// unsigned overflow 64-bit (32)
#[test]
fn test_aarch32_vsubl_t1a1_a_subs_oracle_32_9_f2800620() {
    // Test SUBS 32-bit: unsigned overflow 64-bit (with oracle verification)
    // Encoding: 0xF2800620
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800620;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `SUBS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// unsigned overflow 64-bit (64)
#[test]
fn test_aarch32_vsubl_t1a1_a_subs_oracle_64_9_f2800620() {
    // Test SUBS 64-bit: unsigned overflow 64-bit (with oracle verification)
    // Encoding: 0xF2800620
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800620;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `SUBS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// unsigned overflow 32-bit (32)
#[test]
fn test_aarch32_vsubl_t1a1_a_subs_oracle_32_10_f2800620() {
    // Test SUBS 32-bit: unsigned overflow 32-bit (with oracle verification)
    // Encoding: 0xF2800620
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800620;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `SUBS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// unsigned overflow 32-bit (64)
#[test]
fn test_aarch32_vsubl_t1a1_a_subs_oracle_64_10_f2800620() {
    // Test SUBS 64-bit: unsigned overflow 32-bit (with oracle verification)
    // Encoding: 0xF2800620
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800620;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `SUBS ZR, X1, #10`
/// Requirement: RegisterSpecial { reg: Zr, behavior: "result discarded, flags set" }
/// ZR destination (32)
#[test]
fn test_aarch32_vsubl_t1a1_a_subs_oracle_32_rd31_zr_f2802a3f() {
    // Test SUBS 32-bit with Rd=31 (ZR)
    // Encoding: 0xF2802A3F
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x64);
    let encoding: u32 = 0xF2802A3F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `SUBS ZR, X1, #10`
/// Requirement: RegisterSpecial { reg: Zr, behavior: "result discarded, flags set" }
/// ZR destination (64)
#[test]
fn test_aarch32_vsubl_t1a1_a_subs_oracle_64_rd31_zr_f2802a3f() {
    // Test SUBS 64-bit with Rd=31 (ZR)
    // Encoding: 0xF2802A3F
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x64);
    let encoding: u32 = 0xF2802A3F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `ADD R0, R1, #10`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// small immediate (Rn=0x00000000)
#[test]
fn test_aarch32_vsubl_t1a1_a_a32_add_sub_imm_0_0_f281000a() {
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

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `ADD R0, R1, #10`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// small immediate (Rn=0x00000064)
#[test]
fn test_aarch32_vsubl_t1a1_a_a32_add_sub_imm_0_64_f281000a() {
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

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `ADD R0, R1, #10`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// small immediate (Rn=0x7FFFFFFF)
#[test]
fn test_aarch32_vsubl_t1a1_a_a32_add_sub_imm_0_7fffffff_f281000a() {
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

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `ADD R0, R1, #10`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// small immediate (Rn=0x80000000)
#[test]
fn test_aarch32_vsubl_t1a1_a_a32_add_sub_imm_0_80000000_f281000a() {
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

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `ADD R0, R1, #10`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// small immediate (Rn=0xFFFFFFFF)
#[test]
fn test_aarch32_vsubl_t1a1_a_a32_add_sub_imm_0_ffffffff_f281000a() {
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

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `ADD R0, R1, #255`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// max imm8 (Rn=0x00000000)
#[test]
fn test_aarch32_vsubl_t1a1_a_a32_add_sub_imm_1_0_f28100ff() {
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

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `ADD R0, R1, #255`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// max imm8 (Rn=0x00000064)
#[test]
fn test_aarch32_vsubl_t1a1_a_a32_add_sub_imm_1_64_f28100ff() {
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

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `ADD R0, R1, #255`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// max imm8 (Rn=0x7FFFFFFF)
#[test]
fn test_aarch32_vsubl_t1a1_a_a32_add_sub_imm_1_7fffffff_f28100ff() {
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

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `ADD R0, R1, #255`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// max imm8 (Rn=0x80000000)
#[test]
fn test_aarch32_vsubl_t1a1_a_a32_add_sub_imm_1_80000000_f28100ff() {
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

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `ADD R0, R1, #255`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// max imm8 (Rn=0xFFFFFFFF)
#[test]
fn test_aarch32_vsubl_t1a1_a_a32_add_sub_imm_1_ffffffff_f28100ff() {
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

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `ADD R0, R1, #32`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// rotated by 2 (Rn=0x00000000)
#[test]
fn test_aarch32_vsubl_t1a1_a_a32_add_sub_imm_2_0_f2810180() {
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

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `ADD R0, R1, #32`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// rotated by 2 (Rn=0x00000064)
#[test]
fn test_aarch32_vsubl_t1a1_a_a32_add_sub_imm_2_64_f2810180() {
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

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `ADD R0, R1, #32`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// rotated by 2 (Rn=0x7FFFFFFF)
#[test]
fn test_aarch32_vsubl_t1a1_a_a32_add_sub_imm_2_7fffffff_f2810180() {
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

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `ADD R0, R1, #32`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// rotated by 2 (Rn=0x80000000)
#[test]
fn test_aarch32_vsubl_t1a1_a_a32_add_sub_imm_2_80000000_f2810180() {
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

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `ADD R0, R1, #32`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// rotated by 2 (Rn=0xFFFFFFFF)
#[test]
fn test_aarch32_vsubl_t1a1_a_a32_add_sub_imm_2_ffffffff_f2810180() {
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

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `ADD R0, R1, #251658240`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// rotated by 8 (Rn=0x00000000)
#[test]
fn test_aarch32_vsubl_t1a1_a_a32_add_sub_imm_3_0_f281040f() {
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

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `ADD R0, R1, #251658240`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// rotated by 8 (Rn=0x00000064)
#[test]
fn test_aarch32_vsubl_t1a1_a_a32_add_sub_imm_3_64_f281040f() {
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

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `ADD R0, R1, #251658240`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// rotated by 8 (Rn=0x7FFFFFFF)
#[test]
fn test_aarch32_vsubl_t1a1_a_a32_add_sub_imm_3_7fffffff_f281040f() {
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

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `ADD R0, R1, #251658240`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// rotated by 8 (Rn=0x80000000)
#[test]
fn test_aarch32_vsubl_t1a1_a_a32_add_sub_imm_3_80000000_f281040f() {
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

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `ADD R0, R1, #251658240`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// rotated by 8 (Rn=0xFFFFFFFF)
#[test]
fn test_aarch32_vsubl_t1a1_a_a32_add_sub_imm_3_ffffffff_f281040f() {
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

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `ADD R0, R1, #0`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// zero immediate (Rn=0x00000000)
#[test]
fn test_aarch32_vsubl_t1a1_a_a32_add_sub_imm_4_0_f2810000() {
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

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `ADD R0, R1, #0`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// zero immediate (Rn=0x00000064)
#[test]
fn test_aarch32_vsubl_t1a1_a_a32_add_sub_imm_4_64_f2810000() {
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

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `ADD R0, R1, #0`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// zero immediate (Rn=0x7FFFFFFF)
#[test]
fn test_aarch32_vsubl_t1a1_a_a32_add_sub_imm_4_7fffffff_f2810000() {
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

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `ADD R0, R1, #0`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// zero immediate (Rn=0x80000000)
#[test]
fn test_aarch32_vsubl_t1a1_a_a32_add_sub_imm_4_80000000_f2810000() {
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

/// Provenance: aarch32_VSUBL_T1A1_A
/// ASL: `ADD R0, R1, #0`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// zero immediate (Rn=0xFFFFFFFF)
#[test]
fn test_aarch32_vsubl_t1a1_a_a32_add_sub_imm_4_ffffffff_f2810000() {
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
// aarch32_VADD_i_A Tests
// ============================================================================

/// Provenance: aarch32_VADD_i_T1A1_A
/// ASL: `field D 22 +: 1`
/// Requirement: FieldBoundary { field: "D", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_vadd_i_t1a1_a_field_d_0_min_800_f2000800() {
    // Encoding: 0xF2000800
    // Test aarch32_VADD_i_T1A1_A field D = 0 (Min)
    // ISET: A32
    // Fields: size=0, N=0, Vn=0, M=0, D=0, Q=0, Vm=0, Vd=0
    let encoding: u32 = 0xF2000800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VADD_i_T1A1_A
/// ASL: `field D 22 +: 1`
/// Requirement: FieldBoundary { field: "D", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_vadd_i_t1a1_a_field_d_1_max_800_f2400800() {
    // Encoding: 0xF2400800
    // Test aarch32_VADD_i_T1A1_A field D = 1 (Max)
    // ISET: A32
    // Fields: size=0, Q=0, D=1, Vd=0, M=0, N=0, Vm=0, Vn=0
    let encoding: u32 = 0xF2400800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VADD_i_T1A1_A
/// ASL: `field size 20 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch32_vadd_i_t1a1_a_field_size_0_min_800_f2000800() {
    // Encoding: 0xF2000800
    // Test aarch32_VADD_i_T1A1_A field size = 0 (Min)
    // ISET: A32
    // Fields: Vm=0, Vn=0, Q=0, M=0, Vd=0, N=0, size=0, D=0
    let encoding: u32 = 0xF2000800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VADD_i_T1A1_A
/// ASL: `field size 20 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 1, boundary: PowerOfTwo }
/// 16-bit / halfword size
#[test]
fn test_aarch32_vadd_i_t1a1_a_field_size_1_poweroftwo_800_f2100800() {
    // Encoding: 0xF2100800
    // Test aarch32_VADD_i_T1A1_A field size = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Vd=0, M=0, D=0, Vn=0, size=1, Q=0, Vm=0, N=0
    let encoding: u32 = 0xF2100800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VADD_i_T1A1_A
/// ASL: `field size 20 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 2, boundary: PowerOfTwo }
/// 32-bit / word size
#[test]
fn test_aarch32_vadd_i_t1a1_a_field_size_2_poweroftwo_800_f2200800() {
    // Encoding: 0xF2200800
    // Test aarch32_VADD_i_T1A1_A field size = 2 (PowerOfTwo)
    // ISET: A32
    // Fields: Vn=0, M=0, N=0, Vm=0, Vd=0, D=0, size=2, Q=0
    let encoding: u32 = 0xF2200800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VADD_i_T1A1_A
/// ASL: `field size 20 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 3, boundary: Max }
/// 64-bit / doubleword size
#[test]
fn test_aarch32_vadd_i_t1a1_a_field_size_3_max_800_f2300800() {
    // Encoding: 0xF2300800
    // Test aarch32_VADD_i_T1A1_A field size = 3 (Max)
    // ISET: A32
    // Fields: size=3, Q=0, M=0, Vn=0, D=0, Vd=0, N=0, Vm=0
    let encoding: u32 = 0xF2300800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VADD_i_T1A1_A
/// ASL: `field Vn 16 +: 4`
/// Requirement: FieldBoundary { field: "Vn", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_aarch32_vadd_i_t1a1_a_field_vn_0_min_800_f2000800() {
    // Encoding: 0xF2000800
    // Test aarch32_VADD_i_T1A1_A field Vn = 0 (Min)
    // ISET: A32
    // Fields: D=0, Vd=0, M=0, Vm=0, N=0, Q=0, Vn=0, size=0
    let encoding: u32 = 0xF2000800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VADD_i_T1A1_A
/// ASL: `field Vn 16 +: 4`
/// Requirement: FieldBoundary { field: "Vn", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_aarch32_vadd_i_t1a1_a_field_vn_1_poweroftwo_800_f2010800() {
    // Encoding: 0xF2010800
    // Test aarch32_VADD_i_T1A1_A field Vn = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: size=0, N=0, Vm=0, Q=0, Vd=0, M=0, Vn=1, D=0
    let encoding: u32 = 0xF2010800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VADD_i_T1A1_A
/// ASL: `field Vd 12 +: 4`
/// Requirement: FieldBoundary { field: "Vd", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_aarch32_vadd_i_t1a1_a_field_vd_0_min_800_f2000800() {
    // Encoding: 0xF2000800
    // Test aarch32_VADD_i_T1A1_A field Vd = 0 (Min)
    // ISET: A32
    // Fields: M=0, N=0, Vd=0, size=0, Vm=0, D=0, Vn=0, Q=0
    let encoding: u32 = 0xF2000800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VADD_i_T1A1_A
/// ASL: `field Vd 12 +: 4`
/// Requirement: FieldBoundary { field: "Vd", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_aarch32_vadd_i_t1a1_a_field_vd_1_poweroftwo_800_f2001800() {
    // Encoding: 0xF2001800
    // Test aarch32_VADD_i_T1A1_A field Vd = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Q=0, M=0, Vm=0, size=0, Vd=1, Vn=0, D=0, N=0
    let encoding: u32 = 0xF2001800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VADD_i_T1A1_A
/// ASL: `field N 7 +: 1`
/// Requirement: FieldBoundary { field: "N", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_vadd_i_t1a1_a_field_n_0_min_800_f2000800() {
    // Encoding: 0xF2000800
    // Test aarch32_VADD_i_T1A1_A field N = 0 (Min)
    // ISET: A32
    // Fields: N=0, size=0, Vm=0, Vn=0, Q=0, M=0, D=0, Vd=0
    let encoding: u32 = 0xF2000800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VADD_i_T1A1_A
/// ASL: `field N 7 +: 1`
/// Requirement: FieldBoundary { field: "N", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_vadd_i_t1a1_a_field_n_1_max_800_f2000880() {
    // Encoding: 0xF2000880
    // Test aarch32_VADD_i_T1A1_A field N = 1 (Max)
    // ISET: A32
    // Fields: M=0, size=0, D=0, Vm=0, N=1, Vn=0, Q=0, Vd=0
    let encoding: u32 = 0xF2000880;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VADD_i_T1A1_A
/// ASL: `field Q 6 +: 1`
/// Requirement: FieldBoundary { field: "Q", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch32_vadd_i_t1a1_a_field_q_0_min_800_f2000800() {
    // Encoding: 0xF2000800
    // Test aarch32_VADD_i_T1A1_A field Q = 0 (Min)
    // ISET: A32
    // Fields: Vm=0, Vn=0, D=0, size=0, N=0, Q=0, M=0, Vd=0
    let encoding: u32 = 0xF2000800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VADD_i_T1A1_A
/// ASL: `field Q 6 +: 1`
/// Requirement: FieldBoundary { field: "Q", value: 1, boundary: Max }
/// 16-bit / halfword size
#[test]
fn test_aarch32_vadd_i_t1a1_a_field_q_1_max_800_f2000840() {
    // Encoding: 0xF2000840
    // Test aarch32_VADD_i_T1A1_A field Q = 1 (Max)
    // ISET: A32
    // Fields: Vn=0, M=0, Vm=0, size=0, Q=1, D=0, Vd=0, N=0
    let encoding: u32 = 0xF2000840;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VADD_i_T1A1_A
/// ASL: `field M 5 +: 1`
/// Requirement: FieldBoundary { field: "M", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_vadd_i_t1a1_a_field_m_0_min_800_f2000800() {
    // Encoding: 0xF2000800
    // Test aarch32_VADD_i_T1A1_A field M = 0 (Min)
    // ISET: A32
    // Fields: Vn=0, Q=0, size=0, Vm=0, D=0, M=0, Vd=0, N=0
    let encoding: u32 = 0xF2000800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VADD_i_T1A1_A
/// ASL: `field M 5 +: 1`
/// Requirement: FieldBoundary { field: "M", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_vadd_i_t1a1_a_field_m_1_max_800_f2000820() {
    // Encoding: 0xF2000820
    // Test aarch32_VADD_i_T1A1_A field M = 1 (Max)
    // ISET: A32
    // Fields: Q=0, size=0, Vd=0, D=0, Vm=0, M=1, Vn=0, N=0
    let encoding: u32 = 0xF2000820;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VADD_i_T1A1_A
/// ASL: `field Vm 0 +: 4`
/// Requirement: FieldBoundary { field: "Vm", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_aarch32_vadd_i_t1a1_a_field_vm_0_min_800_f2000800() {
    // Encoding: 0xF2000800
    // Test aarch32_VADD_i_T1A1_A field Vm = 0 (Min)
    // ISET: A32
    // Fields: D=0, N=0, Q=0, Vd=0, size=0, M=0, Vm=0, Vn=0
    let encoding: u32 = 0xF2000800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VADD_i_T1A1_A
/// ASL: `field Vm 0 +: 4`
/// Requirement: FieldBoundary { field: "Vm", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_aarch32_vadd_i_t1a1_a_field_vm_1_poweroftwo_800_f2000801() {
    // Encoding: 0xF2000801
    // Test aarch32_VADD_i_T1A1_A field Vm = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Vm=1, Vn=0, Vd=0, size=0, N=0, D=0, Q=0, M=0
    let encoding: u32 = 0xF2000801;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VADD_i_T1A1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// D=0 (minimum value)
#[test]
fn test_aarch32_vadd_i_t1a1_a_combo_0_800_f2000800() {
    // Encoding: 0xF2000800
    // Test aarch32_VADD_i_T1A1_A field combination: D=0, size=0, Vn=0, Vd=0, N=0, Q=0, M=0, Vm=0
    // ISET: A32
    // Fields: Vm=0, M=0, N=0, Vd=0, size=0, Vn=0, D=0, Q=0
    let encoding: u32 = 0xF2000800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VADD_i_T1A1_A
/// ASL: `field size = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "size", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch32_vadd_i_t1a1_a_special_size_0_size_variant_0_2048_f2000800() {
    // Encoding: 0xF2000800
    // Test aarch32_VADD_i_T1A1_A special value size = 0 (Size variant 0)
    // ISET: A32
    // Fields: size=0, Vm=0, M=0, Vn=0, Q=0, N=0, D=0, Vd=0
    let encoding: u32 = 0xF2000800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VADD_i_T1A1_A
/// ASL: `field size = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "size", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch32_vadd_i_t1a1_a_special_size_1_size_variant_1_2048_f2100800() {
    // Encoding: 0xF2100800
    // Test aarch32_VADD_i_T1A1_A special value size = 1 (Size variant 1)
    // ISET: A32
    // Fields: N=0, M=0, Q=0, Vd=0, D=0, Vn=0, size=1, Vm=0
    let encoding: u32 = 0xF2100800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VADD_i_T1A1_A
/// ASL: `field size = 2 (Size variant 2)`
/// Requirement: FieldSpecial { field: "size", value: 2, meaning: "Size variant 2" }
/// Size variant 2
#[test]
fn test_aarch32_vadd_i_t1a1_a_special_size_2_size_variant_2_2048_f2200800() {
    // Encoding: 0xF2200800
    // Test aarch32_VADD_i_T1A1_A special value size = 2 (Size variant 2)
    // ISET: A32
    // Fields: Vn=0, Q=0, N=0, size=2, Vd=0, M=0, Vm=0, D=0
    let encoding: u32 = 0xF2200800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VADD_i_T1A1_A
/// ASL: `field size = 3 (Size variant 3)`
/// Requirement: FieldSpecial { field: "size", value: 3, meaning: "Size variant 3" }
/// Size variant 3
#[test]
fn test_aarch32_vadd_i_t1a1_a_special_size_3_size_variant_3_2048_f2300800() {
    // Encoding: 0xF2300800
    // Test aarch32_VADD_i_T1A1_A special value size = 3 (Size variant 3)
    // ISET: A32
    // Fields: Vn=0, size=3, M=0, Vm=0, N=0, Q=0, Vd=0, D=0
    let encoding: u32 = 0xF2300800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VADD_i_T1A1_A
/// ASL: `field Q = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "Q", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch32_vadd_i_t1a1_a_special_q_0_size_variant_0_2048_f2000800() {
    // Encoding: 0xF2000800
    // Test aarch32_VADD_i_T1A1_A special value Q = 0 (Size variant 0)
    // ISET: A32
    // Fields: Vn=0, D=0, Q=0, size=0, N=0, Vd=0, Vm=0, M=0
    let encoding: u32 = 0xF2000800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VADD_i_T1A1_A
/// ASL: `field Q = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "Q", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch32_vadd_i_t1a1_a_special_q_1_size_variant_1_2048_f2000840() {
    // Encoding: 0xF2000840
    // Test aarch32_VADD_i_T1A1_A special value Q = 1 (Size variant 1)
    // ISET: A32
    // Fields: N=0, size=0, Vn=0, Vd=0, Vm=0, M=0, Q=1, D=0
    let encoding: u32 = 0xF2000840;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VADD_i_T1A1_A
/// ASL: `Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "Q" }), rhs: Binary { op: And, lhs: LitBits([true]), rhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: "Vd" }), indices: [Single(LitInt(0))] }, rhs: Binary { op: Or, lhs: LitBits([true]), rhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: "Vn" }), indices: [Single(LitInt(0))] } } }, rhs: Binary { op: Or, lhs: LitBits([true]), rhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: "Vm" }), indices: [Single(LitInt(0))] } } }, rhs: LitBits([true]) } } }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"Q\" }), rhs: Binary { op: And, lhs: LitBits([true]), rhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: \"Vd\" }), indices: [Single(LitInt(0))] }, rhs: Binary { op: Or, lhs: LitBits([true]), rhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: \"Vn\" }), indices: [Single(LitInt(0))] } } }, rhs: Binary { op: Or, lhs: LitBits([true]), rhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: \"Vm\" }), indices: [Single(LitInt(0))] } } }, rhs: LitBits([true]) } } }" }
/// triggers Undefined
#[test]
fn test_aarch32_vadd_i_t1a1_a_invalid_0_800_f2000800() {
    // Encoding: 0xF2000800
    // Test aarch32_VADD_i_T1A1_A invalid encoding: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "Q" }), rhs: Binary { op: And, lhs: LitBits([true]), rhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: "Vd" }), indices: [Single(LitInt(0))] }, rhs: Binary { op: Or, lhs: LitBits([true]), rhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: "Vn" }), indices: [Single(LitInt(0))] } } }, rhs: Binary { op: Or, lhs: LitBits([true]), rhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: "Vm" }), indices: [Single(LitInt(0))] } } }, rhs: LitBits([true]) } } }
    // ISET: A32
    // Fields: Q=0, size=0, Vm=0, N=0, Vn=0, M=0, D=0, Vd=0
    let encoding: u32 = 0xF2000800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || !matches!(exit.unwrap(), CpuExit::Continue),
        "expected UNDEFINED for A32 encoding 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch32_VADD_i_T1A1_A
/// ASL: `Unconditional UNDEFINED`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNDEFINED" }
/// triggers Undefined
#[test]
fn test_aarch32_vadd_i_t1a1_a_invalid_1_800_f2000800() {
    // Encoding: 0xF2000800
    // Test aarch32_VADD_i_T1A1_A invalid encoding: Unconditional UNDEFINED
    // ISET: A32
    // Fields: Vm=0, M=0, size=0, Q=0, Vd=0, N=0, Vn=0, D=0
    let encoding: u32 = 0xF2000800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || !matches!(exit.unwrap(), CpuExit::Continue),
        "expected UNDEFINED for A32 encoding 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch32_VADD_i_T1A1_A
/// ASL: `field D 22 +: 1`
/// Requirement: FieldBoundary { field: "D", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_vadd_i_t1a1_a_field_d_0_min_800_ef000800() {
    // Thumb encoding (32): 0xEF000800
    // Test aarch32_VADD_i_T1A1_A field D = 0 (Min)
    // ISET: T32
    // Fields: Vd=0, Vn=0, Q=0, M=0, D=0, size=0, Vm=0, N=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF000800;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VADD_i_T1A1_A
/// ASL: `field D 22 +: 1`
/// Requirement: FieldBoundary { field: "D", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_vadd_i_t1a1_a_field_d_1_max_800_ef400800() {
    // Thumb encoding (32): 0xEF400800
    // Test aarch32_VADD_i_T1A1_A field D = 1 (Max)
    // ISET: T32
    // Fields: M=0, Vn=0, Vm=0, N=0, D=1, Q=0, Vd=0, size=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF400800;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VADD_i_T1A1_A
/// ASL: `field size 20 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch32_vadd_i_t1a1_a_field_size_0_min_800_ef000800() {
    // Thumb encoding (32): 0xEF000800
    // Test aarch32_VADD_i_T1A1_A field size = 0 (Min)
    // ISET: T32
    // Fields: Vd=0, D=0, M=0, Q=0, size=0, Vm=0, N=0, Vn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF000800;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VADD_i_T1A1_A
/// ASL: `field size 20 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 1, boundary: PowerOfTwo }
/// 16-bit / halfword size
#[test]
fn test_aarch32_vadd_i_t1a1_a_field_size_1_poweroftwo_800_ef100800() {
    // Thumb encoding (32): 0xEF100800
    // Test aarch32_VADD_i_T1A1_A field size = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: N=0, Vn=0, size=1, Q=0, Vm=0, Vd=0, D=0, M=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF100800;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VADD_i_T1A1_A
/// ASL: `field size 20 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 2, boundary: PowerOfTwo }
/// 32-bit / word size
#[test]
fn test_aarch32_vadd_i_t1a1_a_field_size_2_poweroftwo_800_ef200800() {
    // Thumb encoding (32): 0xEF200800
    // Test aarch32_VADD_i_T1A1_A field size = 2 (PowerOfTwo)
    // ISET: T32
    // Fields: Vn=0, Vd=0, N=0, Vm=0, D=0, M=0, size=2, Q=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF200800;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VADD_i_T1A1_A
/// ASL: `field size 20 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 3, boundary: Max }
/// 64-bit / doubleword size
#[test]
fn test_aarch32_vadd_i_t1a1_a_field_size_3_max_800_ef300800() {
    // Thumb encoding (32): 0xEF300800
    // Test aarch32_VADD_i_T1A1_A field size = 3 (Max)
    // ISET: T32
    // Fields: Vd=0, size=3, Vn=0, Vm=0, D=0, M=0, Q=0, N=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF300800;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VADD_i_T1A1_A
/// ASL: `field Vn 16 +: 4`
/// Requirement: FieldBoundary { field: "Vn", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_aarch32_vadd_i_t1a1_a_field_vn_0_min_800_ef000800() {
    // Thumb encoding (32): 0xEF000800
    // Test aarch32_VADD_i_T1A1_A field Vn = 0 (Min)
    // ISET: T32
    // Fields: Vd=0, Q=0, M=0, Vn=0, N=0, Vm=0, size=0, D=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF000800;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VADD_i_T1A1_A
/// ASL: `field Vn 16 +: 4`
/// Requirement: FieldBoundary { field: "Vn", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_aarch32_vadd_i_t1a1_a_field_vn_1_poweroftwo_800_ef010800() {
    // Thumb encoding (32): 0xEF010800
    // Test aarch32_VADD_i_T1A1_A field Vn = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Vn=1, Q=0, N=0, Vm=0, D=0, M=0, size=0, Vd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF010800;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VADD_i_T1A1_A
/// ASL: `field Vd 12 +: 4`
/// Requirement: FieldBoundary { field: "Vd", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_aarch32_vadd_i_t1a1_a_field_vd_0_min_800_ef000800() {
    // Thumb encoding (32): 0xEF000800
    // Test aarch32_VADD_i_T1A1_A field Vd = 0 (Min)
    // ISET: T32
    // Fields: M=0, N=0, Q=0, Vn=0, D=0, Vd=0, Vm=0, size=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF000800;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VADD_i_T1A1_A
/// ASL: `field Vd 12 +: 4`
/// Requirement: FieldBoundary { field: "Vd", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_aarch32_vadd_i_t1a1_a_field_vd_1_poweroftwo_800_ef001800() {
    // Thumb encoding (32): 0xEF001800
    // Test aarch32_VADD_i_T1A1_A field Vd = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Vm=0, size=0, N=0, Vd=1, D=0, Q=0, M=0, Vn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF001800;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VADD_i_T1A1_A
/// ASL: `field N 7 +: 1`
/// Requirement: FieldBoundary { field: "N", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_vadd_i_t1a1_a_field_n_0_min_800_ef000800() {
    // Thumb encoding (32): 0xEF000800
    // Test aarch32_VADD_i_T1A1_A field N = 0 (Min)
    // ISET: T32
    // Fields: N=0, Vn=0, D=0, Vm=0, M=0, size=0, Q=0, Vd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF000800;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VADD_i_T1A1_A
/// ASL: `field N 7 +: 1`
/// Requirement: FieldBoundary { field: "N", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_vadd_i_t1a1_a_field_n_1_max_800_ef000880() {
    // Thumb encoding (32): 0xEF000880
    // Test aarch32_VADD_i_T1A1_A field N = 1 (Max)
    // ISET: T32
    // Fields: Vd=0, M=0, D=0, N=1, Vm=0, size=0, Q=0, Vn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF000880;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VADD_i_T1A1_A
/// ASL: `field Q 6 +: 1`
/// Requirement: FieldBoundary { field: "Q", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch32_vadd_i_t1a1_a_field_q_0_min_800_ef000800() {
    // Thumb encoding (32): 0xEF000800
    // Test aarch32_VADD_i_T1A1_A field Q = 0 (Min)
    // ISET: T32
    // Fields: size=0, N=0, M=0, Vm=0, D=0, Vd=0, Q=0, Vn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF000800;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VADD_i_T1A1_A
/// ASL: `field Q 6 +: 1`
/// Requirement: FieldBoundary { field: "Q", value: 1, boundary: Max }
/// 16-bit / halfword size
#[test]
fn test_aarch32_vadd_i_t1a1_a_field_q_1_max_800_ef000840() {
    // Thumb encoding (32): 0xEF000840
    // Test aarch32_VADD_i_T1A1_A field Q = 1 (Max)
    // ISET: T32
    // Fields: Vn=0, size=0, Vd=0, M=0, D=0, Vm=0, N=0, Q=1
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF000840;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VADD_i_T1A1_A
/// ASL: `field M 5 +: 1`
/// Requirement: FieldBoundary { field: "M", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_vadd_i_t1a1_a_field_m_0_min_800_ef000800() {
    // Thumb encoding (32): 0xEF000800
    // Test aarch32_VADD_i_T1A1_A field M = 0 (Min)
    // ISET: T32
    // Fields: size=0, Vn=0, N=0, Q=0, D=0, Vm=0, Vd=0, M=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF000800;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VADD_i_T1A1_A
/// ASL: `field M 5 +: 1`
/// Requirement: FieldBoundary { field: "M", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_vadd_i_t1a1_a_field_m_1_max_800_ef000820() {
    // Thumb encoding (32): 0xEF000820
    // Test aarch32_VADD_i_T1A1_A field M = 1 (Max)
    // ISET: T32
    // Fields: Vn=0, Vd=0, D=0, size=0, Q=0, Vm=0, M=1, N=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF000820;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VADD_i_T1A1_A
/// ASL: `field Vm 0 +: 4`
/// Requirement: FieldBoundary { field: "Vm", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_aarch32_vadd_i_t1a1_a_field_vm_0_min_800_ef000800() {
    // Thumb encoding (32): 0xEF000800
    // Test aarch32_VADD_i_T1A1_A field Vm = 0 (Min)
    // ISET: T32
    // Fields: Vd=0, D=0, N=0, Q=0, size=0, M=0, Vn=0, Vm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF000800;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VADD_i_T1A1_A
/// ASL: `field Vm 0 +: 4`
/// Requirement: FieldBoundary { field: "Vm", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_aarch32_vadd_i_t1a1_a_field_vm_1_poweroftwo_800_ef000801() {
    // Thumb encoding (32): 0xEF000801
    // Test aarch32_VADD_i_T1A1_A field Vm = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: D=0, M=0, size=0, N=0, Vm=1, Vn=0, Q=0, Vd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF000801;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VADD_i_T1A1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// D=0 (minimum value)
#[test]
fn test_aarch32_vadd_i_t1a1_a_combo_0_800_ef000800() {
    // Thumb encoding (32): 0xEF000800
    // Test aarch32_VADD_i_T1A1_A field combination: D=0, size=0, Vn=0, Vd=0, N=0, Q=0, M=0, Vm=0
    // ISET: T32
    // Fields: size=0, Q=0, Vd=0, M=0, Vn=0, D=0, N=0, Vm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF000800;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VADD_i_T1A1_A
/// ASL: `field size = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "size", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch32_vadd_i_t1a1_a_special_size_0_size_variant_0_2048_ef000800() {
    // Thumb encoding (32): 0xEF000800
    // Test aarch32_VADD_i_T1A1_A special value size = 0 (Size variant 0)
    // ISET: T32
    // Fields: size=0, Vd=0, N=0, D=0, Vn=0, M=0, Q=0, Vm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF000800;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VADD_i_T1A1_A
/// ASL: `field size = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "size", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch32_vadd_i_t1a1_a_special_size_1_size_variant_1_2048_ef100800() {
    // Thumb encoding (32): 0xEF100800
    // Test aarch32_VADD_i_T1A1_A special value size = 1 (Size variant 1)
    // ISET: T32
    // Fields: M=0, N=0, size=1, Vd=0, Q=0, Vn=0, D=0, Vm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF100800;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VADD_i_T1A1_A
/// ASL: `field size = 2 (Size variant 2)`
/// Requirement: FieldSpecial { field: "size", value: 2, meaning: "Size variant 2" }
/// Size variant 2
#[test]
fn test_aarch32_vadd_i_t1a1_a_special_size_2_size_variant_2_2048_ef200800() {
    // Thumb encoding (32): 0xEF200800
    // Test aarch32_VADD_i_T1A1_A special value size = 2 (Size variant 2)
    // ISET: T32
    // Fields: D=0, size=2, N=0, M=0, Vn=0, Q=0, Vm=0, Vd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF200800;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VADD_i_T1A1_A
/// ASL: `field size = 3 (Size variant 3)`
/// Requirement: FieldSpecial { field: "size", value: 3, meaning: "Size variant 3" }
/// Size variant 3
#[test]
fn test_aarch32_vadd_i_t1a1_a_special_size_3_size_variant_3_2048_ef300800() {
    // Thumb encoding (32): 0xEF300800
    // Test aarch32_VADD_i_T1A1_A special value size = 3 (Size variant 3)
    // ISET: T32
    // Fields: D=0, N=0, Vd=0, M=0, Q=0, Vm=0, size=3, Vn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF300800;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VADD_i_T1A1_A
/// ASL: `field Q = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "Q", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch32_vadd_i_t1a1_a_special_q_0_size_variant_0_2048_ef000800() {
    // Thumb encoding (32): 0xEF000800
    // Test aarch32_VADD_i_T1A1_A special value Q = 0 (Size variant 0)
    // ISET: T32
    // Fields: size=0, M=0, Vm=0, Vd=0, Q=0, D=0, N=0, Vn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF000800;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VADD_i_T1A1_A
/// ASL: `field Q = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "Q", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch32_vadd_i_t1a1_a_special_q_1_size_variant_1_2048_ef000840() {
    // Thumb encoding (32): 0xEF000840
    // Test aarch32_VADD_i_T1A1_A special value Q = 1 (Size variant 1)
    // ISET: T32
    // Fields: Q=1, Vm=0, Vn=0, size=0, Vd=0, M=0, D=0, N=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF000840;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VADD_i_T1A1_A
/// ASL: `Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "Q" }), rhs: Binary { op: And, lhs: LitBits([true]), rhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: "Vd" }), indices: [Single(LitInt(0))] }, rhs: Binary { op: Or, lhs: LitBits([true]), rhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: "Vn" }), indices: [Single(LitInt(0))] } } }, rhs: Binary { op: Or, lhs: LitBits([true]), rhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: "Vm" }), indices: [Single(LitInt(0))] } } }, rhs: LitBits([true]) } } }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"Q\" }), rhs: Binary { op: And, lhs: LitBits([true]), rhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: \"Vd\" }), indices: [Single(LitInt(0))] }, rhs: Binary { op: Or, lhs: LitBits([true]), rhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: \"Vn\" }), indices: [Single(LitInt(0))] } } }, rhs: Binary { op: Or, lhs: LitBits([true]), rhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: \"Vm\" }), indices: [Single(LitInt(0))] } } }, rhs: LitBits([true]) } } }" }
/// triggers Undefined
#[test]
fn test_aarch32_vadd_i_t1a1_a_invalid_0_800_ef000800() {
    // Thumb encoding (32): 0xEF000800
    // Test aarch32_VADD_i_T1A1_A invalid encoding: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "Q" }), rhs: Binary { op: And, lhs: LitBits([true]), rhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: "Vd" }), indices: [Single(LitInt(0))] }, rhs: Binary { op: Or, lhs: LitBits([true]), rhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: "Vn" }), indices: [Single(LitInt(0))] } } }, rhs: Binary { op: Or, lhs: LitBits([true]), rhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: "Vm" }), indices: [Single(LitInt(0))] } } }, rhs: LitBits([true]) } } }
    // ISET: T32
    // Fields: D=0, Vn=0, size=0, N=0, M=0, Vd=0, Vm=0, Q=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF000800;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || !matches!(exit.unwrap(), CpuExit::Continue),
        "expected UNDEFINED for T32 encoding"
    );
}

/// Provenance: aarch32_VADD_i_T1A1_A
/// ASL: `Unconditional UNDEFINED`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNDEFINED" }
/// triggers Undefined
#[test]
fn test_aarch32_vadd_i_t1a1_a_invalid_1_800_ef000800() {
    // Thumb encoding (32): 0xEF000800
    // Test aarch32_VADD_i_T1A1_A invalid encoding: Unconditional UNDEFINED
    // ISET: T32
    // Fields: Vn=0, N=0, D=0, size=0, Vm=0, M=0, Q=0, Vd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF000800;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || !matches!(exit.unwrap(), CpuExit::Continue),
        "expected UNDEFINED for T32 encoding"
    );
}

/// Provenance: aarch32_VADD_i_T1A1_A
/// ASL: `AND X0, X1, #0xFF`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// mask lower 8 bits (64)
#[test]
fn test_aarch32_vadd_i_t1a1_a_and_oracle_64_0_92401c20() {
    // Test AND 64-bit: mask lower 8 bits (oracle)
    // Encoding: 0x92401C20
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0x92401C20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0xFF, "X0 should be 0x00000000000000FF");
}

/// Provenance: aarch32_VADD_i_T1A1_A
/// ASL: `AND X0, X1, #0xFFFF`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// mask lower 16 bits (64)
#[test]
fn test_aarch32_vadd_i_t1a1_a_and_oracle_64_1_92403c20() {
    // Test AND 64-bit: mask lower 16 bits (oracle)
    // Encoding: 0x92403C20
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0x92403C20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0xFFFF, "X0 should be 0x000000000000FFFF");
}

/// Provenance: aarch32_VADD_i_T1A1_A
/// ASL: `AND X0, X1, #0xFFFFFFFF`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// mask lower 32 bits (64)
#[test]
fn test_aarch32_vadd_i_t1a1_a_and_oracle_64_2_92407c20() {
    // Test AND 64-bit: mask lower 32 bits (oracle)
    // Encoding: 0x92407C20
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0x92407C20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0x00000000FFFFFFFF"
    );
}

/// Provenance: aarch32_VADD_i_T1A1_A
/// ASL: `AND X0, X1, #0x1`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// single bit mask (64)
#[test]
fn test_aarch32_vadd_i_t1a1_a_and_oracle_64_3_92400020() {
    // Test AND 64-bit: single bit mask (oracle)
    // Encoding: 0x92400020
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xCAFEBABE);
    let encoding: u32 = 0x92400020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x0, "X0 should be 0x0000000000000000");
}

/// Provenance: aarch32_VADD_i_T1A1_A
/// ASL: `AND X0, X1, #0x7FFFFFFFFFFFFFFF`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// all but MSB (64)
#[test]
fn test_aarch32_vadd_i_t1a1_a_and_oracle_64_4_9240f820() {
    // Test AND 64-bit: all but MSB (oracle)
    // Encoding: 0x9240F820
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xAAAAAAAA);
    let encoding: u32 = 0x9240F820;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xAAAAAAAA,
        "X0 should be 0x2AAAAAAAAAAAAAAA"
    );
}

/// Provenance: aarch32_VADD_i_T1A1_A
/// ASL: `AND W0, W1, #0xFF`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// mask lower 8 bits (32)
#[test]
fn test_aarch32_vadd_i_t1a1_a_and_oracle_32_0_12001c20() {
    // Test AND 32-bit: mask lower 8 bits (oracle)
    // Encoding: 0x12001C20
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0x12001C20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0xFF, "W0 should be 0x000000FF");
}

/// Provenance: aarch32_VADD_i_T1A1_A
/// ASL: `AND W0, W1, #0xFFFF`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// mask lower 16 bits (32)
#[test]
fn test_aarch32_vadd_i_t1a1_a_and_oracle_32_1_12003c20() {
    // Test AND 32-bit: mask lower 16 bits (oracle)
    // Encoding: 0x12003C20
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0x12003C20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0xFFFF, "W0 should be 0x0000FFFF");
}

/// Provenance: aarch32_VADD_i_T1A1_A
/// ASL: `AND W0, W1, #0x1`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// single bit mask (32)
#[test]
fn test_aarch32_vadd_i_t1a1_a_and_oracle_32_2_12000020() {
    // Test AND 32-bit: single bit mask (oracle)
    // Encoding: 0x12000020
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xDEADBEEF);
    let encoding: u32 = 0x12000020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x1, "W0 should be 0x00000001");
}

/// Provenance: aarch32_VADD_i_T1A1_A
/// ASL: `AND R0, R1, #10`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// small immediate (Rn=0x00000000)
#[test]
fn test_aarch32_vadd_i_t1a1_a_a32_logical_imm_0_0_f201000a() {
    // Test A32 AND: small immediate (oracle)
    // Encoding: 0xF201000A
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF201000A;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x0, "R0 should be 0x00000000");
}

/// Provenance: aarch32_VADD_i_T1A1_A
/// ASL: `AND R0, R1, #10`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// small immediate (Rn=0x000000FF)
#[test]
fn test_aarch32_vadd_i_t1a1_a_a32_logical_imm_0_ff_f201000a() {
    // Test A32 AND: small immediate (oracle)
    // Encoding: 0xF201000A
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFF);
    let encoding: u32 = 0xF201000A;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0xA, "R0 should be 0x0000000A");
}

/// Provenance: aarch32_VADD_i_T1A1_A
/// ASL: `AND R0, R1, #10`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// small immediate (Rn=0xAAAAAAAA)
#[test]
fn test_aarch32_vadd_i_t1a1_a_a32_logical_imm_0_aaaaaaaa_f201000a() {
    // Test A32 AND: small immediate (oracle)
    // Encoding: 0xF201000A
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xAAAAAAAA);
    let encoding: u32 = 0xF201000A;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0xA, "R0 should be 0x0000000A");
}

/// Provenance: aarch32_VADD_i_T1A1_A
/// ASL: `AND R0, R1, #10`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// small immediate (Rn=0x55555555)
#[test]
fn test_aarch32_vadd_i_t1a1_a_a32_logical_imm_0_55555555_f201000a() {
    // Test A32 AND: small immediate (oracle)
    // Encoding: 0xF201000A
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x55555555);
    let encoding: u32 = 0xF201000A;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x0, "R0 should be 0x00000000");
}

/// Provenance: aarch32_VADD_i_T1A1_A
/// ASL: `AND R0, R1, #10`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// small immediate (Rn=0xFFFFFFFF)
#[test]
fn test_aarch32_vadd_i_t1a1_a_a32_logical_imm_0_ffffffff_f201000a() {
    // Test A32 AND: small immediate (oracle)
    // Encoding: 0xF201000A
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF201000A;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0xA, "R0 should be 0x0000000A");
}

/// Provenance: aarch32_VADD_i_T1A1_A
/// ASL: `AND R0, R1, #255`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// max imm8 (Rn=0x00000000)
#[test]
fn test_aarch32_vadd_i_t1a1_a_a32_logical_imm_1_0_f20100ff() {
    // Test A32 AND: max imm8 (oracle)
    // Encoding: 0xF20100FF
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF20100FF;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x0, "R0 should be 0x00000000");
}

/// Provenance: aarch32_VADD_i_T1A1_A
/// ASL: `AND R0, R1, #255`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// max imm8 (Rn=0x000000FF)
#[test]
fn test_aarch32_vadd_i_t1a1_a_a32_logical_imm_1_ff_f20100ff() {
    // Test A32 AND: max imm8 (oracle)
    // Encoding: 0xF20100FF
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFF);
    let encoding: u32 = 0xF20100FF;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0xFF, "R0 should be 0x000000FF");
}

/// Provenance: aarch32_VADD_i_T1A1_A
/// ASL: `AND R0, R1, #255`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// max imm8 (Rn=0xAAAAAAAA)
#[test]
fn test_aarch32_vadd_i_t1a1_a_a32_logical_imm_1_aaaaaaaa_f20100ff() {
    // Test A32 AND: max imm8 (oracle)
    // Encoding: 0xF20100FF
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xAAAAAAAA);
    let encoding: u32 = 0xF20100FF;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0xAA, "R0 should be 0x000000AA");
}

/// Provenance: aarch32_VADD_i_T1A1_A
/// ASL: `AND R0, R1, #255`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// max imm8 (Rn=0x55555555)
#[test]
fn test_aarch32_vadd_i_t1a1_a_a32_logical_imm_1_55555555_f20100ff() {
    // Test A32 AND: max imm8 (oracle)
    // Encoding: 0xF20100FF
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x55555555);
    let encoding: u32 = 0xF20100FF;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x55, "R0 should be 0x00000055");
}

/// Provenance: aarch32_VADD_i_T1A1_A
/// ASL: `AND R0, R1, #255`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// max imm8 (Rn=0xFFFFFFFF)
#[test]
fn test_aarch32_vadd_i_t1a1_a_a32_logical_imm_1_ffffffff_f20100ff() {
    // Test A32 AND: max imm8 (oracle)
    // Encoding: 0xF20100FF
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF20100FF;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0xFF, "R0 should be 0x000000FF");
}

/// Provenance: aarch32_VADD_i_T1A1_A
/// ASL: `AND R0, R1, #32`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// rotated by 2 (Rn=0x00000000)
#[test]
fn test_aarch32_vadd_i_t1a1_a_a32_logical_imm_2_0_f2010180() {
    // Test A32 AND: rotated by 2 (oracle)
    // Encoding: 0xF2010180
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2010180;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x0, "R0 should be 0x00000000");
}

/// Provenance: aarch32_VADD_i_T1A1_A
/// ASL: `AND R0, R1, #32`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// rotated by 2 (Rn=0x000000FF)
#[test]
fn test_aarch32_vadd_i_t1a1_a_a32_logical_imm_2_ff_f2010180() {
    // Test A32 AND: rotated by 2 (oracle)
    // Encoding: 0xF2010180
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFF);
    let encoding: u32 = 0xF2010180;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x20, "R0 should be 0x00000020");
}

/// Provenance: aarch32_VADD_i_T1A1_A
/// ASL: `AND R0, R1, #32`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// rotated by 2 (Rn=0xAAAAAAAA)
#[test]
fn test_aarch32_vadd_i_t1a1_a_a32_logical_imm_2_aaaaaaaa_f2010180() {
    // Test A32 AND: rotated by 2 (oracle)
    // Encoding: 0xF2010180
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xAAAAAAAA);
    let encoding: u32 = 0xF2010180;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x20, "R0 should be 0x00000020");
}

/// Provenance: aarch32_VADD_i_T1A1_A
/// ASL: `AND R0, R1, #32`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// rotated by 2 (Rn=0x55555555)
#[test]
fn test_aarch32_vadd_i_t1a1_a_a32_logical_imm_2_55555555_f2010180() {
    // Test A32 AND: rotated by 2 (oracle)
    // Encoding: 0xF2010180
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x55555555);
    let encoding: u32 = 0xF2010180;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x0, "R0 should be 0x00000000");
}

/// Provenance: aarch32_VADD_i_T1A1_A
/// ASL: `AND R0, R1, #32`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// rotated by 2 (Rn=0xFFFFFFFF)
#[test]
fn test_aarch32_vadd_i_t1a1_a_a32_logical_imm_2_ffffffff_f2010180() {
    // Test A32 AND: rotated by 2 (oracle)
    // Encoding: 0xF2010180
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2010180;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x20, "R0 should be 0x00000020");
}

/// Provenance: aarch32_VADD_i_T1A1_A
/// ASL: `AND R0, R1, #251658240`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// rotated by 8 (Rn=0x00000000)
#[test]
fn test_aarch32_vadd_i_t1a1_a_a32_logical_imm_3_0_f201040f() {
    // Test A32 AND: rotated by 8 (oracle)
    // Encoding: 0xF201040F
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF201040F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x0, "R0 should be 0x00000000");
}

/// Provenance: aarch32_VADD_i_T1A1_A
/// ASL: `AND R0, R1, #251658240`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// rotated by 8 (Rn=0x000000FF)
#[test]
fn test_aarch32_vadd_i_t1a1_a_a32_logical_imm_3_ff_f201040f() {
    // Test A32 AND: rotated by 8 (oracle)
    // Encoding: 0xF201040F
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFF);
    let encoding: u32 = 0xF201040F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x0, "R0 should be 0x00000000");
}

/// Provenance: aarch32_VADD_i_T1A1_A
/// ASL: `AND R0, R1, #251658240`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// rotated by 8 (Rn=0xAAAAAAAA)
#[test]
fn test_aarch32_vadd_i_t1a1_a_a32_logical_imm_3_aaaaaaaa_f201040f() {
    // Test A32 AND: rotated by 8 (oracle)
    // Encoding: 0xF201040F
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xAAAAAAAA);
    let encoding: u32 = 0xF201040F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0xA000000, "R0 should be 0x0A000000");
}

/// Provenance: aarch32_VADD_i_T1A1_A
/// ASL: `AND R0, R1, #251658240`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// rotated by 8 (Rn=0x55555555)
#[test]
fn test_aarch32_vadd_i_t1a1_a_a32_logical_imm_3_55555555_f201040f() {
    // Test A32 AND: rotated by 8 (oracle)
    // Encoding: 0xF201040F
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x55555555);
    let encoding: u32 = 0xF201040F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x5000000, "R0 should be 0x05000000");
}

/// Provenance: aarch32_VADD_i_T1A1_A
/// ASL: `AND R0, R1, #251658240`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// rotated by 8 (Rn=0xFFFFFFFF)
#[test]
fn test_aarch32_vadd_i_t1a1_a_a32_logical_imm_3_ffffffff_f201040f() {
    // Test A32 AND: rotated by 8 (oracle)
    // Encoding: 0xF201040F
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF201040F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0xF000000, "R0 should be 0x0F000000");
}

/// Provenance: aarch32_VADD_i_T1A1_A
/// ASL: `AND R0, R1, #0`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// zero immediate (Rn=0x00000000)
#[test]
fn test_aarch32_vadd_i_t1a1_a_a32_logical_imm_4_0_f2010000() {
    // Test A32 AND: zero immediate (oracle)
    // Encoding: 0xF2010000
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2010000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x0, "R0 should be 0x00000000");
}

/// Provenance: aarch32_VADD_i_T1A1_A
/// ASL: `AND R0, R1, #0`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// zero immediate (Rn=0x000000FF)
#[test]
fn test_aarch32_vadd_i_t1a1_a_a32_logical_imm_4_ff_f2010000() {
    // Test A32 AND: zero immediate (oracle)
    // Encoding: 0xF2010000
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFF);
    let encoding: u32 = 0xF2010000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x0, "R0 should be 0x00000000");
}

/// Provenance: aarch32_VADD_i_T1A1_A
/// ASL: `AND R0, R1, #0`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// zero immediate (Rn=0xAAAAAAAA)
#[test]
fn test_aarch32_vadd_i_t1a1_a_a32_logical_imm_4_aaaaaaaa_f2010000() {
    // Test A32 AND: zero immediate (oracle)
    // Encoding: 0xF2010000
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xAAAAAAAA);
    let encoding: u32 = 0xF2010000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x0, "R0 should be 0x00000000");
}

/// Provenance: aarch32_VADD_i_T1A1_A
/// ASL: `AND R0, R1, #0`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// zero immediate (Rn=0x55555555)
#[test]
fn test_aarch32_vadd_i_t1a1_a_a32_logical_imm_4_55555555_f2010000() {
    // Test A32 AND: zero immediate (oracle)
    // Encoding: 0xF2010000
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x55555555);
    let encoding: u32 = 0xF2010000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x0, "R0 should be 0x00000000");
}

/// Provenance: aarch32_VADD_i_T1A1_A
/// ASL: `AND R0, R1, #0`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// zero immediate (Rn=0xFFFFFFFF)
#[test]
fn test_aarch32_vadd_i_t1a1_a_a32_logical_imm_4_ffffffff_f2010000() {
    // Test A32 AND: zero immediate (oracle)
    // Encoding: 0xF2010000
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2010000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x0, "R0 should be 0x00000000");
}

// ============================================================================
// aarch32_VADDHN_A Tests
// ============================================================================

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `field D 22 +: 1`
/// Requirement: FieldBoundary { field: "D", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_vaddhn_t1a1_a_field_d_0_min_400_f2800400() {
    // Encoding: 0xF2800400
    // Test aarch32_VADDHN_T1A1_A field D = 0 (Min)
    // ISET: A32
    // Fields: size=0, Vm=0, Vn=0, D=0, Vd=0, N=0, M=0
    let encoding: u32 = 0xF2800400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `field D 22 +: 1`
/// Requirement: FieldBoundary { field: "D", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_vaddhn_t1a1_a_field_d_1_max_400_f2c00400() {
    // Encoding: 0xF2C00400
    // Test aarch32_VADDHN_T1A1_A field D = 1 (Max)
    // ISET: A32
    // Fields: Vn=0, D=1, N=0, Vm=0, size=0, Vd=0, M=0
    let encoding: u32 = 0xF2C00400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `field size 20 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch32_vaddhn_t1a1_a_field_size_0_min_400_f2800400() {
    // Encoding: 0xF2800400
    // Test aarch32_VADDHN_T1A1_A field size = 0 (Min)
    // ISET: A32
    // Fields: Vd=0, N=0, size=0, M=0, Vm=0, Vn=0, D=0
    let encoding: u32 = 0xF2800400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `field size 20 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 1, boundary: PowerOfTwo }
/// 16-bit / halfword size
#[test]
fn test_aarch32_vaddhn_t1a1_a_field_size_1_poweroftwo_400_f2900400() {
    // Encoding: 0xF2900400
    // Test aarch32_VADDHN_T1A1_A field size = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Vn=0, size=1, Vd=0, M=0, Vm=0, N=0, D=0
    let encoding: u32 = 0xF2900400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `field size 20 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 2, boundary: PowerOfTwo }
/// 32-bit / word size
#[test]
fn test_aarch32_vaddhn_t1a1_a_field_size_2_poweroftwo_400_f2a00400() {
    // Encoding: 0xF2A00400
    // Test aarch32_VADDHN_T1A1_A field size = 2 (PowerOfTwo)
    // ISET: A32
    // Fields: M=0, N=0, Vn=0, size=2, D=0, Vd=0, Vm=0
    let encoding: u32 = 0xF2A00400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `field size 20 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 3, boundary: Max }
/// 64-bit / doubleword size
#[test]
fn test_aarch32_vaddhn_t1a1_a_field_size_3_max_400_f2b00400() {
    // Encoding: 0xF2B00400
    // Test aarch32_VADDHN_T1A1_A field size = 3 (Max)
    // ISET: A32
    // Fields: D=0, Vm=0, size=3, M=0, Vn=0, N=0, Vd=0
    let encoding: u32 = 0xF2B00400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `field Vn 16 +: 4`
/// Requirement: FieldBoundary { field: "Vn", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_aarch32_vaddhn_t1a1_a_field_vn_0_min_400_f2800400() {
    // Encoding: 0xF2800400
    // Test aarch32_VADDHN_T1A1_A field Vn = 0 (Min)
    // ISET: A32
    // Fields: size=0, Vn=0, Vd=0, M=0, D=0, N=0, Vm=0
    let encoding: u32 = 0xF2800400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `field Vn 16 +: 4`
/// Requirement: FieldBoundary { field: "Vn", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_aarch32_vaddhn_t1a1_a_field_vn_1_poweroftwo_400_f2810400() {
    // Encoding: 0xF2810400
    // Test aarch32_VADDHN_T1A1_A field Vn = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: size=0, Vn=1, N=0, D=0, Vm=0, M=0, Vd=0
    let encoding: u32 = 0xF2810400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `field Vd 12 +: 4`
/// Requirement: FieldBoundary { field: "Vd", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_aarch32_vaddhn_t1a1_a_field_vd_0_min_400_f2800400() {
    // Encoding: 0xF2800400
    // Test aarch32_VADDHN_T1A1_A field Vd = 0 (Min)
    // ISET: A32
    // Fields: D=0, M=0, N=0, Vm=0, Vn=0, Vd=0, size=0
    let encoding: u32 = 0xF2800400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `field Vd 12 +: 4`
/// Requirement: FieldBoundary { field: "Vd", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_aarch32_vaddhn_t1a1_a_field_vd_1_poweroftwo_400_f2801400() {
    // Encoding: 0xF2801400
    // Test aarch32_VADDHN_T1A1_A field Vd = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Vd=1, Vn=0, M=0, Vm=0, size=0, N=0, D=0
    let encoding: u32 = 0xF2801400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `field N 7 +: 1`
/// Requirement: FieldBoundary { field: "N", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_vaddhn_t1a1_a_field_n_0_min_400_f2800400() {
    // Encoding: 0xF2800400
    // Test aarch32_VADDHN_T1A1_A field N = 0 (Min)
    // ISET: A32
    // Fields: D=0, Vn=0, M=0, size=0, N=0, Vm=0, Vd=0
    let encoding: u32 = 0xF2800400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `field N 7 +: 1`
/// Requirement: FieldBoundary { field: "N", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_vaddhn_t1a1_a_field_n_1_max_400_f2800480() {
    // Encoding: 0xF2800480
    // Test aarch32_VADDHN_T1A1_A field N = 1 (Max)
    // ISET: A32
    // Fields: Vd=0, size=0, M=0, D=0, Vn=0, N=1, Vm=0
    let encoding: u32 = 0xF2800480;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `field M 5 +: 1`
/// Requirement: FieldBoundary { field: "M", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_vaddhn_t1a1_a_field_m_0_min_400_f2800400() {
    // Encoding: 0xF2800400
    // Test aarch32_VADDHN_T1A1_A field M = 0 (Min)
    // ISET: A32
    // Fields: M=0, size=0, N=0, Vm=0, D=0, Vd=0, Vn=0
    let encoding: u32 = 0xF2800400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `field M 5 +: 1`
/// Requirement: FieldBoundary { field: "M", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_vaddhn_t1a1_a_field_m_1_max_400_f2800420() {
    // Encoding: 0xF2800420
    // Test aarch32_VADDHN_T1A1_A field M = 1 (Max)
    // ISET: A32
    // Fields: Vd=0, M=1, Vm=0, Vn=0, N=0, size=0, D=0
    let encoding: u32 = 0xF2800420;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `field Vm 0 +: 4`
/// Requirement: FieldBoundary { field: "Vm", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_aarch32_vaddhn_t1a1_a_field_vm_0_min_400_f2800400() {
    // Encoding: 0xF2800400
    // Test aarch32_VADDHN_T1A1_A field Vm = 0 (Min)
    // ISET: A32
    // Fields: M=0, Vm=0, D=0, size=0, Vd=0, N=0, Vn=0
    let encoding: u32 = 0xF2800400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `field Vm 0 +: 4`
/// Requirement: FieldBoundary { field: "Vm", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_aarch32_vaddhn_t1a1_a_field_vm_1_poweroftwo_400_f2800401() {
    // Encoding: 0xF2800401
    // Test aarch32_VADDHN_T1A1_A field Vm = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: size=0, D=0, Vd=0, N=0, M=0, Vm=1, Vn=0
    let encoding: u32 = 0xF2800401;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// D=0 (minimum value)
#[test]
fn test_aarch32_vaddhn_t1a1_a_combo_0_400_f2800400() {
    // Encoding: 0xF2800400
    // Test aarch32_VADDHN_T1A1_A field combination: D=0, size=0, Vn=0, Vd=0, N=0, M=0, Vm=0
    // ISET: A32
    // Fields: Vn=0, D=0, size=0, N=0, M=0, Vm=0, Vd=0
    let encoding: u32 = 0xF2800400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `field size = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "size", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch32_vaddhn_t1a1_a_special_size_0_size_variant_0_1024_f2800400() {
    // Encoding: 0xF2800400
    // Test aarch32_VADDHN_T1A1_A special value size = 0 (Size variant 0)
    // ISET: A32
    // Fields: N=0, size=0, Vd=0, D=0, Vn=0, M=0, Vm=0
    let encoding: u32 = 0xF2800400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `field size = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "size", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch32_vaddhn_t1a1_a_special_size_1_size_variant_1_1024_f2900400() {
    // Encoding: 0xF2900400
    // Test aarch32_VADDHN_T1A1_A special value size = 1 (Size variant 1)
    // ISET: A32
    // Fields: M=0, Vm=0, D=0, Vn=0, size=1, Vd=0, N=0
    let encoding: u32 = 0xF2900400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `field size = 2 (Size variant 2)`
/// Requirement: FieldSpecial { field: "size", value: 2, meaning: "Size variant 2" }
/// Size variant 2
#[test]
fn test_aarch32_vaddhn_t1a1_a_special_size_2_size_variant_2_1024_f2a00400() {
    // Encoding: 0xF2A00400
    // Test aarch32_VADDHN_T1A1_A special value size = 2 (Size variant 2)
    // ISET: A32
    // Fields: D=0, M=0, Vm=0, size=2, Vn=0, Vd=0, N=0
    let encoding: u32 = 0xF2A00400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `field size = 3 (Size variant 3)`
/// Requirement: FieldSpecial { field: "size", value: 3, meaning: "Size variant 3" }
/// Size variant 3
#[test]
fn test_aarch32_vaddhn_t1a1_a_special_size_3_size_variant_3_1024_f2b00400() {
    // Encoding: 0xF2B00400
    // Test aarch32_VADDHN_T1A1_A special value size = 3 (Size variant 3)
    // ISET: A32
    // Fields: Vd=0, D=0, Vn=0, Vm=0, M=0, N=0, size=3
    let encoding: u32 = 0xF2B00400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `Binary { op: Eq, lhs: Binary { op: Eq, lhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: "Vn" }), indices: [Single(LitInt(0))] }, rhs: Binary { op: Or, lhs: LitBits([true]), rhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: "Vm" }), indices: [Single(LitInt(0))] } } }, rhs: LitBits([true]) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Binary { op: Eq, lhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: \"Vn\" }), indices: [Single(LitInt(0))] }, rhs: Binary { op: Or, lhs: LitBits([true]), rhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: \"Vm\" }), indices: [Single(LitInt(0))] } } }, rhs: LitBits([true]) }" }
/// triggers Undefined
#[test]
fn test_aarch32_vaddhn_t1a1_a_invalid_0_400_f2800400() {
    // Encoding: 0xF2800400
    // Test aarch32_VADDHN_T1A1_A invalid encoding: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: "Vn" }), indices: [Single(LitInt(0))] }, rhs: Binary { op: Or, lhs: LitBits([true]), rhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: "Vm" }), indices: [Single(LitInt(0))] } } }, rhs: LitBits([true]) }
    // ISET: A32
    // Fields: Vn=0, D=0, size=0, N=0, Vm=0, M=0, Vd=0
    let encoding: u32 = 0xF2800400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || !matches!(exit.unwrap(), CpuExit::Continue),
        "expected UNDEFINED for A32 encoding 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `Unconditional UNDEFINED`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNDEFINED" }
/// triggers Undefined
#[test]
fn test_aarch32_vaddhn_t1a1_a_invalid_1_400_f2800400() {
    // Encoding: 0xF2800400
    // Test aarch32_VADDHN_T1A1_A invalid encoding: Unconditional UNDEFINED
    // ISET: A32
    // Fields: Vm=0, D=0, Vd=0, N=0, size=0, M=0, Vn=0
    let encoding: u32 = 0xF2800400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || !matches!(exit.unwrap(), CpuExit::Continue),
        "expected UNDEFINED for A32 encoding 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `field D 22 +: 1`
/// Requirement: FieldBoundary { field: "D", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_vaddhn_t1a1_a_field_d_0_min_400_ef800400() {
    // Thumb encoding (32): 0xEF800400
    // Test aarch32_VADDHN_T1A1_A field D = 0 (Min)
    // ISET: T32
    // Fields: Vd=0, N=0, size=0, Vn=0, M=0, D=0, Vm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF800400;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `field D 22 +: 1`
/// Requirement: FieldBoundary { field: "D", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_vaddhn_t1a1_a_field_d_1_max_400_efc00400() {
    // Thumb encoding (32): 0xEFC00400
    // Test aarch32_VADDHN_T1A1_A field D = 1 (Max)
    // ISET: T32
    // Fields: M=0, Vd=0, Vn=0, Vm=0, size=0, N=0, D=1
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEFC00400;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `field size 20 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch32_vaddhn_t1a1_a_field_size_0_min_400_ef800400() {
    // Thumb encoding (32): 0xEF800400
    // Test aarch32_VADDHN_T1A1_A field size = 0 (Min)
    // ISET: T32
    // Fields: Vn=0, M=0, Vm=0, D=0, Vd=0, size=0, N=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF800400;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `field size 20 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 1, boundary: PowerOfTwo }
/// 16-bit / halfword size
#[test]
fn test_aarch32_vaddhn_t1a1_a_field_size_1_poweroftwo_400_ef900400() {
    // Thumb encoding (32): 0xEF900400
    // Test aarch32_VADDHN_T1A1_A field size = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: size=1, Vd=0, N=0, D=0, M=0, Vm=0, Vn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF900400;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `field size 20 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 2, boundary: PowerOfTwo }
/// 32-bit / word size
#[test]
fn test_aarch32_vaddhn_t1a1_a_field_size_2_poweroftwo_400_efa00400() {
    // Thumb encoding (32): 0xEFA00400
    // Test aarch32_VADDHN_T1A1_A field size = 2 (PowerOfTwo)
    // ISET: T32
    // Fields: Vn=0, M=0, Vd=0, size=2, N=0, D=0, Vm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEFA00400;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `field size 20 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 3, boundary: Max }
/// 64-bit / doubleword size
#[test]
fn test_aarch32_vaddhn_t1a1_a_field_size_3_max_400_efb00400() {
    // Thumb encoding (32): 0xEFB00400
    // Test aarch32_VADDHN_T1A1_A field size = 3 (Max)
    // ISET: T32
    // Fields: M=0, Vn=0, D=0, size=3, N=0, Vd=0, Vm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEFB00400;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `field Vn 16 +: 4`
/// Requirement: FieldBoundary { field: "Vn", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_aarch32_vaddhn_t1a1_a_field_vn_0_min_400_ef800400() {
    // Thumb encoding (32): 0xEF800400
    // Test aarch32_VADDHN_T1A1_A field Vn = 0 (Min)
    // ISET: T32
    // Fields: Vn=0, D=0, size=0, N=0, Vd=0, M=0, Vm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF800400;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `field Vn 16 +: 4`
/// Requirement: FieldBoundary { field: "Vn", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_aarch32_vaddhn_t1a1_a_field_vn_1_poweroftwo_400_ef810400() {
    // Thumb encoding (32): 0xEF810400
    // Test aarch32_VADDHN_T1A1_A field Vn = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: D=0, M=0, size=0, Vm=0, Vn=1, Vd=0, N=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF810400;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `field Vd 12 +: 4`
/// Requirement: FieldBoundary { field: "Vd", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_aarch32_vaddhn_t1a1_a_field_vd_0_min_400_ef800400() {
    // Thumb encoding (32): 0xEF800400
    // Test aarch32_VADDHN_T1A1_A field Vd = 0 (Min)
    // ISET: T32
    // Fields: size=0, Vn=0, Vd=0, N=0, M=0, Vm=0, D=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF800400;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `field Vd 12 +: 4`
/// Requirement: FieldBoundary { field: "Vd", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_aarch32_vaddhn_t1a1_a_field_vd_1_poweroftwo_400_ef801400() {
    // Thumb encoding (32): 0xEF801400
    // Test aarch32_VADDHN_T1A1_A field Vd = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: D=0, Vn=0, M=0, Vd=1, size=0, Vm=0, N=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF801400;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `field N 7 +: 1`
/// Requirement: FieldBoundary { field: "N", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_vaddhn_t1a1_a_field_n_0_min_400_ef800400() {
    // Thumb encoding (32): 0xEF800400
    // Test aarch32_VADDHN_T1A1_A field N = 0 (Min)
    // ISET: T32
    // Fields: Vd=0, M=0, Vm=0, size=0, D=0, N=0, Vn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF800400;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `field N 7 +: 1`
/// Requirement: FieldBoundary { field: "N", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_vaddhn_t1a1_a_field_n_1_max_400_ef800480() {
    // Thumb encoding (32): 0xEF800480
    // Test aarch32_VADDHN_T1A1_A field N = 1 (Max)
    // ISET: T32
    // Fields: Vm=0, Vn=0, N=1, Vd=0, M=0, D=0, size=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF800480;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `field M 5 +: 1`
/// Requirement: FieldBoundary { field: "M", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_vaddhn_t1a1_a_field_m_0_min_400_ef800400() {
    // Thumb encoding (32): 0xEF800400
    // Test aarch32_VADDHN_T1A1_A field M = 0 (Min)
    // ISET: T32
    // Fields: D=0, size=0, Vn=0, Vd=0, Vm=0, N=0, M=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF800400;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `field M 5 +: 1`
/// Requirement: FieldBoundary { field: "M", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_vaddhn_t1a1_a_field_m_1_max_400_ef800420() {
    // Thumb encoding (32): 0xEF800420
    // Test aarch32_VADDHN_T1A1_A field M = 1 (Max)
    // ISET: T32
    // Fields: size=0, D=0, N=0, Vm=0, Vd=0, M=1, Vn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF800420;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `field Vm 0 +: 4`
/// Requirement: FieldBoundary { field: "Vm", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_aarch32_vaddhn_t1a1_a_field_vm_0_min_400_ef800400() {
    // Thumb encoding (32): 0xEF800400
    // Test aarch32_VADDHN_T1A1_A field Vm = 0 (Min)
    // ISET: T32
    // Fields: Vn=0, size=0, N=0, D=0, Vd=0, Vm=0, M=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF800400;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `field Vm 0 +: 4`
/// Requirement: FieldBoundary { field: "Vm", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_aarch32_vaddhn_t1a1_a_field_vm_1_poweroftwo_400_ef800401() {
    // Thumb encoding (32): 0xEF800401
    // Test aarch32_VADDHN_T1A1_A field Vm = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Vn=0, N=0, Vm=1, M=0, Vd=0, D=0, size=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF800401;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// D=0 (minimum value)
#[test]
fn test_aarch32_vaddhn_t1a1_a_combo_0_400_ef800400() {
    // Thumb encoding (32): 0xEF800400
    // Test aarch32_VADDHN_T1A1_A field combination: D=0, size=0, Vn=0, Vd=0, N=0, M=0, Vm=0
    // ISET: T32
    // Fields: N=0, Vn=0, size=0, M=0, Vm=0, D=0, Vd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF800400;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `field size = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "size", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch32_vaddhn_t1a1_a_special_size_0_size_variant_0_1024_ef800400() {
    // Thumb encoding (32): 0xEF800400
    // Test aarch32_VADDHN_T1A1_A special value size = 0 (Size variant 0)
    // ISET: T32
    // Fields: size=0, Vn=0, M=0, Vm=0, Vd=0, N=0, D=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF800400;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `field size = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "size", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch32_vaddhn_t1a1_a_special_size_1_size_variant_1_1024_ef900400() {
    // Thumb encoding (32): 0xEF900400
    // Test aarch32_VADDHN_T1A1_A special value size = 1 (Size variant 1)
    // ISET: T32
    // Fields: D=0, Vd=0, M=0, Vm=0, N=0, size=1, Vn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF900400;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `field size = 2 (Size variant 2)`
/// Requirement: FieldSpecial { field: "size", value: 2, meaning: "Size variant 2" }
/// Size variant 2
#[test]
fn test_aarch32_vaddhn_t1a1_a_special_size_2_size_variant_2_1024_efa00400() {
    // Thumb encoding (32): 0xEFA00400
    // Test aarch32_VADDHN_T1A1_A special value size = 2 (Size variant 2)
    // ISET: T32
    // Fields: N=0, Vm=0, Vn=0, M=0, D=0, size=2, Vd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEFA00400;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `field size = 3 (Size variant 3)`
/// Requirement: FieldSpecial { field: "size", value: 3, meaning: "Size variant 3" }
/// Size variant 3
#[test]
fn test_aarch32_vaddhn_t1a1_a_special_size_3_size_variant_3_1024_efb00400() {
    // Thumb encoding (32): 0xEFB00400
    // Test aarch32_VADDHN_T1A1_A special value size = 3 (Size variant 3)
    // ISET: T32
    // Fields: size=3, Vm=0, M=0, Vd=0, Vn=0, N=0, D=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEFB00400;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `Binary { op: Eq, lhs: Binary { op: Eq, lhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: "Vn" }), indices: [Single(LitInt(0))] }, rhs: Binary { op: Or, lhs: LitBits([true]), rhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: "Vm" }), indices: [Single(LitInt(0))] } } }, rhs: LitBits([true]) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Binary { op: Eq, lhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: \"Vn\" }), indices: [Single(LitInt(0))] }, rhs: Binary { op: Or, lhs: LitBits([true]), rhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: \"Vm\" }), indices: [Single(LitInt(0))] } } }, rhs: LitBits([true]) }" }
/// triggers Undefined
#[test]
fn test_aarch32_vaddhn_t1a1_a_invalid_0_400_ef800400() {
    // Thumb encoding (32): 0xEF800400
    // Test aarch32_VADDHN_T1A1_A invalid encoding: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: "Vn" }), indices: [Single(LitInt(0))] }, rhs: Binary { op: Or, lhs: LitBits([true]), rhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: "Vm" }), indices: [Single(LitInt(0))] } } }, rhs: LitBits([true]) }
    // ISET: T32
    // Fields: size=0, D=0, N=0, Vn=0, M=0, Vd=0, Vm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF800400;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || !matches!(exit.unwrap(), CpuExit::Continue),
        "expected UNDEFINED for T32 encoding"
    );
}

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `Unconditional UNDEFINED`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNDEFINED" }
/// triggers Undefined
#[test]
fn test_aarch32_vaddhn_t1a1_a_invalid_1_400_ef800400() {
    // Thumb encoding (32): 0xEF800400
    // Test aarch32_VADDHN_T1A1_A invalid encoding: Unconditional UNDEFINED
    // ISET: T32
    // Fields: N=0, D=0, Vn=0, Vd=0, M=0, Vm=0, size=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF800400;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || !matches!(exit.unwrap(), CpuExit::Continue),
        "expected UNDEFINED for T32 encoding"
    );
}

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `ADD X0, X1, #10`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// simple addition/subtraction (32)
#[test]
fn test_aarch32_vaddhn_t1a1_a_add_oracle_32_0_f2802c20() {
    // Test ADD 32-bit: simple addition/subtraction (with oracle verification)
    // Encoding: 0xF2802C20
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x64);
    let encoding: u32 = 0xF2802C20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x59, "X0 should be 0x0000000000000059");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `ADD X0, X1, #10`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// simple addition/subtraction (64)
#[test]
fn test_aarch32_vaddhn_t1a1_a_add_oracle_64_0_f2802c20() {
    // Test ADD 64-bit: simple addition/subtraction (with oracle verification)
    // Encoding: 0xF2802C20
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x64);
    let encoding: u32 = 0xF2802C20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x59, "X0 should be 0x0000000000000059");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `ADD X0, X1, #0`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// zero operands (32)
#[test]
fn test_aarch32_vaddhn_t1a1_a_add_oracle_32_1_f2800420() {
    // Test ADD 32-bit: zero operands (with oracle verification)
    // Encoding: 0xF2800420
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2800420;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `ADD X0, X1, #0`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// zero operands (64)
#[test]
fn test_aarch32_vaddhn_t1a1_a_add_oracle_64_1_f2800420() {
    // Test ADD 64-bit: zero operands (with oracle verification)
    // Encoding: 0xF2800420
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2800420;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `ADD X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// small values (32)
#[test]
fn test_aarch32_vaddhn_t1a1_a_add_oracle_32_2_f2800420() {
    // Test ADD 32-bit: small values (with oracle verification)
    // Encoding: 0xF2800420
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x1);
    let encoding: u32 = 0xF2800420;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x0, "X0 should be 0x0000000000000000");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z flag should be true");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `ADD X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// small values (64)
#[test]
fn test_aarch32_vaddhn_t1a1_a_add_oracle_64_2_f2800420() {
    // Test ADD 64-bit: small values (with oracle verification)
    // Encoding: 0xF2800420
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x1);
    let encoding: u32 = 0xF2800420;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x0, "X0 should be 0x0000000000000000");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z flag should be true");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `ADD X0, X1, #4095`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// max imm12 unshifted (32)
#[test]
fn test_aarch32_vaddhn_t1a1_a_add_oracle_32_3_f2bffc20() {
    // Test ADD 32-bit: max imm12 unshifted (with oracle verification)
    // Encoding: 0xF2BFFC20
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2BFFC20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFF001,
        "X0 should be 0xFFFFFFFFFFFFF001"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `ADD X0, X1, #4095`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// max imm12 unshifted (64)
#[test]
fn test_aarch32_vaddhn_t1a1_a_add_oracle_64_3_f2bffc20() {
    // Test ADD 64-bit: max imm12 unshifted (with oracle verification)
    // Encoding: 0xF2BFFC20
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2BFFC20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFF001,
        "X0 should be 0xFFFFFFFFFFFFF001"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `ADD X0, X1, #4095, LSL #12`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// max imm12 shifted (32)
#[test]
fn test_aarch32_vaddhn_t1a1_a_add_oracle_32_4_f2fffc20() {
    // Test ADD 32-bit: max imm12 shifted (with oracle verification)
    // Encoding: 0xF2FFFC20
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2FFFC20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFF001000,
        "X0 should be 0xFFFFFFFFFF001000"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `ADD X0, X1, #4095, LSL #12`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// max imm12 shifted (64)
#[test]
fn test_aarch32_vaddhn_t1a1_a_add_oracle_64_4_f2fffc20() {
    // Test ADD 64-bit: max imm12 shifted (with oracle verification)
    // Encoding: 0xF2FFFC20
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2FFFC20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFF001000,
        "X0 should be 0xFFFFFFFFFF001000"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `ADD X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// max u64 operand (32)
#[test]
fn test_aarch32_vaddhn_t1a1_a_add_oracle_32_5_f2800420() {
    // Test ADD 32-bit: max u64 operand (with oracle verification)
    // Encoding: 0xF2800420
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800420;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFE,
        "X0 should be 0x00000000FFFFFFFE"
    );
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `ADD X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// max u64 operand (64)
#[test]
fn test_aarch32_vaddhn_t1a1_a_add_oracle_64_5_f2800420() {
    // Test ADD 64-bit: max u64 operand (with oracle verification)
    // Encoding: 0xF2800420
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800420;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFE,
        "X0 should be 0xFFFFFFFFFFFFFFFE"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `ADD X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// zero result (for sub 1-1) (32)
#[test]
fn test_aarch32_vaddhn_t1a1_a_add_oracle_32_6_f2800420() {
    // Test ADD 32-bit: zero result (for sub 1-1) (with oracle verification)
    // Encoding: 0xF2800420
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2800420;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `ADD X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// zero result (for sub 1-1) (64)
#[test]
fn test_aarch32_vaddhn_t1a1_a_add_oracle_64_6_f2800420() {
    // Test ADD 64-bit: zero result (for sub 1-1) (with oracle verification)
    // Encoding: 0xF2800420
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2800420;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `ADD X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// signed overflow boundary 64-bit (32)
#[test]
fn test_aarch32_vaddhn_t1a1_a_add_oracle_32_7_f2800420() {
    // Test ADD 32-bit: signed overflow boundary 64-bit (with oracle verification)
    // Encoding: 0xF2800420
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800420;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFE,
        "X0 should be 0x00000000FFFFFFFE"
    );
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `ADD X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// signed overflow boundary 64-bit (64)
#[test]
fn test_aarch32_vaddhn_t1a1_a_add_oracle_64_7_f2800420() {
    // Test ADD 64-bit: signed overflow boundary 64-bit (with oracle verification)
    // Encoding: 0xF2800420
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800420;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFE,
        "X0 should be 0x7FFFFFFFFFFFFFFE"
    );
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `ADD X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// signed overflow boundary 32-bit (32)
#[test]
fn test_aarch32_vaddhn_t1a1_a_add_oracle_32_8_f2800420() {
    // Test ADD 32-bit: signed overflow boundary 32-bit (with oracle verification)
    // Encoding: 0xF2800420
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x7FFFFFFF);
    let encoding: u32 = 0xF2800420;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0x7FFFFFFE,
        "X0 should be 0x000000007FFFFFFE"
    );
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `ADD X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// signed overflow boundary 32-bit (64)
#[test]
fn test_aarch32_vaddhn_t1a1_a_add_oracle_64_8_f2800420() {
    // Test ADD 64-bit: signed overflow boundary 32-bit (with oracle verification)
    // Encoding: 0xF2800420
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x7FFFFFFF);
    let encoding: u32 = 0xF2800420;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0x7FFFFFFE,
        "X0 should be 0x000000007FFFFFFE"
    );
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `ADD X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// unsigned overflow 64-bit (32)
#[test]
fn test_aarch32_vaddhn_t1a1_a_add_oracle_32_9_f2800420() {
    // Test ADD 32-bit: unsigned overflow 64-bit (with oracle verification)
    // Encoding: 0xF2800420
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800420;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFE,
        "X0 should be 0x00000000FFFFFFFE"
    );
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `ADD X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// unsigned overflow 64-bit (64)
#[test]
fn test_aarch32_vaddhn_t1a1_a_add_oracle_64_9_f2800420() {
    // Test ADD 64-bit: unsigned overflow 64-bit (with oracle verification)
    // Encoding: 0xF2800420
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800420;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFE,
        "X0 should be 0xFFFFFFFFFFFFFFFE"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `ADD X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// unsigned overflow 32-bit (32)
#[test]
fn test_aarch32_vaddhn_t1a1_a_add_oracle_32_10_f2800420() {
    // Test ADD 32-bit: unsigned overflow 32-bit (with oracle verification)
    // Encoding: 0xF2800420
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800420;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFE,
        "X0 should be 0x00000000FFFFFFFE"
    );
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `ADD X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// unsigned overflow 32-bit (64)
#[test]
fn test_aarch32_vaddhn_t1a1_a_add_oracle_64_10_f2800420() {
    // Test ADD 64-bit: unsigned overflow 32-bit (with oracle verification)
    // Encoding: 0xF2800420
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800420;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFE,
        "X0 should be 0x00000000FFFFFFFE"
    );
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `ADD SP, X1, #10`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "writes to stack pointer" }
/// SP destination (32)
#[test]
fn test_aarch32_vaddhn_t1a1_a_add_oracle_32_rd31_sp_f2802c3f() {
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

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `ADD SP, X1, #10`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "writes to stack pointer" }
/// SP destination (64)
#[test]
fn test_aarch32_vaddhn_t1a1_a_add_oracle_64_rd31_sp_f2802c3f() {
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

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `ADDS X0, X1, #10`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// simple addition/subtraction (32)
#[test]
fn test_aarch32_vaddhn_t1a1_a_adds_oracle_32_0_f2802c20() {
    // Test ADDS 32-bit: simple addition/subtraction (with oracle verification)
    // Encoding: 0xF2802C20
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x64);
    let encoding: u32 = 0xF2802C20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x59, "X0 should be 0x0000000000000059");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `ADDS X0, X1, #10`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// simple addition/subtraction (64)
#[test]
fn test_aarch32_vaddhn_t1a1_a_adds_oracle_64_0_f2802c20() {
    // Test ADDS 64-bit: simple addition/subtraction (with oracle verification)
    // Encoding: 0xF2802C20
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x64);
    let encoding: u32 = 0xF2802C20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x59, "X0 should be 0x0000000000000059");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `ADDS X0, X1, #0`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// zero operands (32)
#[test]
fn test_aarch32_vaddhn_t1a1_a_adds_oracle_32_1_f2800420() {
    // Test ADDS 32-bit: zero operands (with oracle verification)
    // Encoding: 0xF2800420
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2800420;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `ADDS X0, X1, #0`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// zero operands (64)
#[test]
fn test_aarch32_vaddhn_t1a1_a_adds_oracle_64_1_f2800420() {
    // Test ADDS 64-bit: zero operands (with oracle verification)
    // Encoding: 0xF2800420
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2800420;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `ADDS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// small values (32)
#[test]
fn test_aarch32_vaddhn_t1a1_a_adds_oracle_32_2_f2800420() {
    // Test ADDS 32-bit: small values (with oracle verification)
    // Encoding: 0xF2800420
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x1);
    let encoding: u32 = 0xF2800420;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x0, "X0 should be 0x0000000000000000");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z flag should be true");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `ADDS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// small values (64)
#[test]
fn test_aarch32_vaddhn_t1a1_a_adds_oracle_64_2_f2800420() {
    // Test ADDS 64-bit: small values (with oracle verification)
    // Encoding: 0xF2800420
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x1);
    let encoding: u32 = 0xF2800420;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x0, "X0 should be 0x0000000000000000");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z flag should be true");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `ADDS X0, X1, #4095`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// max imm12 unshifted (32)
#[test]
fn test_aarch32_vaddhn_t1a1_a_adds_oracle_32_3_f2bffc20() {
    // Test ADDS 32-bit: max imm12 unshifted (with oracle verification)
    // Encoding: 0xF2BFFC20
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2BFFC20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFF001,
        "X0 should be 0xFFFFFFFFFFFFF001"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `ADDS X0, X1, #4095`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// max imm12 unshifted (64)
#[test]
fn test_aarch32_vaddhn_t1a1_a_adds_oracle_64_3_f2bffc20() {
    // Test ADDS 64-bit: max imm12 unshifted (with oracle verification)
    // Encoding: 0xF2BFFC20
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2BFFC20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFF001,
        "X0 should be 0xFFFFFFFFFFFFF001"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `ADDS X0, X1, #4095, LSL #12`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// max imm12 shifted (32)
#[test]
fn test_aarch32_vaddhn_t1a1_a_adds_oracle_32_4_f2fffc20() {
    // Test ADDS 32-bit: max imm12 shifted (with oracle verification)
    // Encoding: 0xF2FFFC20
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2FFFC20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFF001000,
        "X0 should be 0xFFFFFFFFFF001000"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `ADDS X0, X1, #4095, LSL #12`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// max imm12 shifted (64)
#[test]
fn test_aarch32_vaddhn_t1a1_a_adds_oracle_64_4_f2fffc20() {
    // Test ADDS 64-bit: max imm12 shifted (with oracle verification)
    // Encoding: 0xF2FFFC20
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2FFFC20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFF001000,
        "X0 should be 0xFFFFFFFFFF001000"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `ADDS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// max u64 operand (32)
#[test]
fn test_aarch32_vaddhn_t1a1_a_adds_oracle_32_5_f2800420() {
    // Test ADDS 32-bit: max u64 operand (with oracle verification)
    // Encoding: 0xF2800420
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800420;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFE,
        "X0 should be 0x00000000FFFFFFFE"
    );
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `ADDS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// max u64 operand (64)
#[test]
fn test_aarch32_vaddhn_t1a1_a_adds_oracle_64_5_f2800420() {
    // Test ADDS 64-bit: max u64 operand (with oracle verification)
    // Encoding: 0xF2800420
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800420;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFE,
        "X0 should be 0xFFFFFFFFFFFFFFFE"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `ADDS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// zero result (for sub 1-1) (32)
#[test]
fn test_aarch32_vaddhn_t1a1_a_adds_oracle_32_6_f2800420() {
    // Test ADDS 32-bit: zero result (for sub 1-1) (with oracle verification)
    // Encoding: 0xF2800420
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2800420;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `ADDS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// zero result (for sub 1-1) (64)
#[test]
fn test_aarch32_vaddhn_t1a1_a_adds_oracle_64_6_f2800420() {
    // Test ADDS 64-bit: zero result (for sub 1-1) (with oracle verification)
    // Encoding: 0xF2800420
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2800420;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `ADDS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// signed overflow boundary 64-bit (32)
#[test]
fn test_aarch32_vaddhn_t1a1_a_adds_oracle_32_7_f2800420() {
    // Test ADDS 32-bit: signed overflow boundary 64-bit (with oracle verification)
    // Encoding: 0xF2800420
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800420;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFE,
        "X0 should be 0x00000000FFFFFFFE"
    );
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `ADDS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// signed overflow boundary 64-bit (64)
#[test]
fn test_aarch32_vaddhn_t1a1_a_adds_oracle_64_7_f2800420() {
    // Test ADDS 64-bit: signed overflow boundary 64-bit (with oracle verification)
    // Encoding: 0xF2800420
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800420;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFE,
        "X0 should be 0x7FFFFFFFFFFFFFFE"
    );
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `ADDS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// signed overflow boundary 32-bit (32)
#[test]
fn test_aarch32_vaddhn_t1a1_a_adds_oracle_32_8_f2800420() {
    // Test ADDS 32-bit: signed overflow boundary 32-bit (with oracle verification)
    // Encoding: 0xF2800420
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x7FFFFFFF);
    let encoding: u32 = 0xF2800420;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0x7FFFFFFE,
        "X0 should be 0x000000007FFFFFFE"
    );
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `ADDS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// signed overflow boundary 32-bit (64)
#[test]
fn test_aarch32_vaddhn_t1a1_a_adds_oracle_64_8_f2800420() {
    // Test ADDS 64-bit: signed overflow boundary 32-bit (with oracle verification)
    // Encoding: 0xF2800420
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x7FFFFFFF);
    let encoding: u32 = 0xF2800420;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0x7FFFFFFE,
        "X0 should be 0x000000007FFFFFFE"
    );
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `ADDS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// unsigned overflow 64-bit (32)
#[test]
fn test_aarch32_vaddhn_t1a1_a_adds_oracle_32_9_f2800420() {
    // Test ADDS 32-bit: unsigned overflow 64-bit (with oracle verification)
    // Encoding: 0xF2800420
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800420;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFE,
        "X0 should be 0x00000000FFFFFFFE"
    );
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `ADDS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// unsigned overflow 64-bit (64)
#[test]
fn test_aarch32_vaddhn_t1a1_a_adds_oracle_64_9_f2800420() {
    // Test ADDS 64-bit: unsigned overflow 64-bit (with oracle verification)
    // Encoding: 0xF2800420
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800420;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFE,
        "X0 should be 0xFFFFFFFFFFFFFFFE"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `ADDS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// unsigned overflow 32-bit (32)
#[test]
fn test_aarch32_vaddhn_t1a1_a_adds_oracle_32_10_f2800420() {
    // Test ADDS 32-bit: unsigned overflow 32-bit (with oracle verification)
    // Encoding: 0xF2800420
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800420;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFE,
        "X0 should be 0x00000000FFFFFFFE"
    );
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `ADDS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// unsigned overflow 32-bit (64)
#[test]
fn test_aarch32_vaddhn_t1a1_a_adds_oracle_64_10_f2800420() {
    // Test ADDS 64-bit: unsigned overflow 32-bit (with oracle verification)
    // Encoding: 0xF2800420
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800420;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFE,
        "X0 should be 0x00000000FFFFFFFE"
    );
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `ADDS ZR, X1, #10`
/// Requirement: RegisterSpecial { reg: Zr, behavior: "result discarded, flags set" }
/// ZR destination (32)
#[test]
fn test_aarch32_vaddhn_t1a1_a_adds_oracle_32_rd31_zr_f2802c3f() {
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

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `ADDS ZR, X1, #10`
/// Requirement: RegisterSpecial { reg: Zr, behavior: "result discarded, flags set" }
/// ZR destination (64)
#[test]
fn test_aarch32_vaddhn_t1a1_a_adds_oracle_64_rd31_zr_f2802c3f() {
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

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `SUB X0, X1, #10`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// simple addition/subtraction (32)
#[test]
fn test_aarch32_vaddhn_t1a1_a_sub_oracle_32_0_f2802c20() {
    // Test SUB 32-bit: simple addition/subtraction (with oracle verification)
    // Encoding: 0xF2802C20
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x64);
    let encoding: u32 = 0xF2802C20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x59, "X0 should be 0x0000000000000059");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `SUB X0, X1, #10`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// simple addition/subtraction (64)
#[test]
fn test_aarch32_vaddhn_t1a1_a_sub_oracle_64_0_f2802c20() {
    // Test SUB 64-bit: simple addition/subtraction (with oracle verification)
    // Encoding: 0xF2802C20
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x64);
    let encoding: u32 = 0xF2802C20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x59, "X0 should be 0x0000000000000059");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `SUB X0, X1, #0`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// zero operands (32)
#[test]
fn test_aarch32_vaddhn_t1a1_a_sub_oracle_32_1_f2800420() {
    // Test SUB 32-bit: zero operands (with oracle verification)
    // Encoding: 0xF2800420
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2800420;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `SUB X0, X1, #0`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// zero operands (64)
#[test]
fn test_aarch32_vaddhn_t1a1_a_sub_oracle_64_1_f2800420() {
    // Test SUB 64-bit: zero operands (with oracle verification)
    // Encoding: 0xF2800420
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2800420;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `SUB X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// small values (32)
#[test]
fn test_aarch32_vaddhn_t1a1_a_sub_oracle_32_2_f2800420() {
    // Test SUB 32-bit: small values (with oracle verification)
    // Encoding: 0xF2800420
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x1);
    let encoding: u32 = 0xF2800420;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x0, "X0 should be 0x0000000000000000");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z flag should be true");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `SUB X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// small values (64)
#[test]
fn test_aarch32_vaddhn_t1a1_a_sub_oracle_64_2_f2800420() {
    // Test SUB 64-bit: small values (with oracle verification)
    // Encoding: 0xF2800420
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x1);
    let encoding: u32 = 0xF2800420;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x0, "X0 should be 0x0000000000000000");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z flag should be true");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `SUB X0, X1, #4095`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// max imm12 unshifted (32)
#[test]
fn test_aarch32_vaddhn_t1a1_a_sub_oracle_32_3_f2bffc20() {
    // Test SUB 32-bit: max imm12 unshifted (with oracle verification)
    // Encoding: 0xF2BFFC20
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2BFFC20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFF001,
        "X0 should be 0xFFFFFFFFFFFFF001"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `SUB X0, X1, #4095`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// max imm12 unshifted (64)
#[test]
fn test_aarch32_vaddhn_t1a1_a_sub_oracle_64_3_f2bffc20() {
    // Test SUB 64-bit: max imm12 unshifted (with oracle verification)
    // Encoding: 0xF2BFFC20
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2BFFC20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFF001,
        "X0 should be 0xFFFFFFFFFFFFF001"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `SUB X0, X1, #4095, LSL #12`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// max imm12 shifted (32)
#[test]
fn test_aarch32_vaddhn_t1a1_a_sub_oracle_32_4_f2fffc20() {
    // Test SUB 32-bit: max imm12 shifted (with oracle verification)
    // Encoding: 0xF2FFFC20
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2FFFC20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFF001000,
        "X0 should be 0xFFFFFFFFFF001000"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `SUB X0, X1, #4095, LSL #12`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// max imm12 shifted (64)
#[test]
fn test_aarch32_vaddhn_t1a1_a_sub_oracle_64_4_f2fffc20() {
    // Test SUB 64-bit: max imm12 shifted (with oracle verification)
    // Encoding: 0xF2FFFC20
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2FFFC20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFF001000,
        "X0 should be 0xFFFFFFFFFF001000"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `SUB X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// max u64 operand (32)
#[test]
fn test_aarch32_vaddhn_t1a1_a_sub_oracle_32_5_f2800420() {
    // Test SUB 32-bit: max u64 operand (with oracle verification)
    // Encoding: 0xF2800420
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800420;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFE,
        "X0 should be 0x00000000FFFFFFFE"
    );
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `SUB X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// max u64 operand (64)
#[test]
fn test_aarch32_vaddhn_t1a1_a_sub_oracle_64_5_f2800420() {
    // Test SUB 64-bit: max u64 operand (with oracle verification)
    // Encoding: 0xF2800420
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800420;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFE,
        "X0 should be 0xFFFFFFFFFFFFFFFE"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `SUB X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// zero result (for sub 1-1) (32)
#[test]
fn test_aarch32_vaddhn_t1a1_a_sub_oracle_32_6_f2800420() {
    // Test SUB 32-bit: zero result (for sub 1-1) (with oracle verification)
    // Encoding: 0xF2800420
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2800420;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `SUB X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// zero result (for sub 1-1) (64)
#[test]
fn test_aarch32_vaddhn_t1a1_a_sub_oracle_64_6_f2800420() {
    // Test SUB 64-bit: zero result (for sub 1-1) (with oracle verification)
    // Encoding: 0xF2800420
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2800420;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `SUB X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// signed overflow boundary 64-bit (32)
#[test]
fn test_aarch32_vaddhn_t1a1_a_sub_oracle_32_7_f2800420() {
    // Test SUB 32-bit: signed overflow boundary 64-bit (with oracle verification)
    // Encoding: 0xF2800420
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800420;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFE,
        "X0 should be 0x00000000FFFFFFFE"
    );
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `SUB X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// signed overflow boundary 64-bit (64)
#[test]
fn test_aarch32_vaddhn_t1a1_a_sub_oracle_64_7_f2800420() {
    // Test SUB 64-bit: signed overflow boundary 64-bit (with oracle verification)
    // Encoding: 0xF2800420
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800420;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFE,
        "X0 should be 0x7FFFFFFFFFFFFFFE"
    );
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `SUB X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// signed overflow boundary 32-bit (32)
#[test]
fn test_aarch32_vaddhn_t1a1_a_sub_oracle_32_8_f2800420() {
    // Test SUB 32-bit: signed overflow boundary 32-bit (with oracle verification)
    // Encoding: 0xF2800420
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x7FFFFFFF);
    let encoding: u32 = 0xF2800420;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0x7FFFFFFE,
        "X0 should be 0x000000007FFFFFFE"
    );
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `SUB X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// signed overflow boundary 32-bit (64)
#[test]
fn test_aarch32_vaddhn_t1a1_a_sub_oracle_64_8_f2800420() {
    // Test SUB 64-bit: signed overflow boundary 32-bit (with oracle verification)
    // Encoding: 0xF2800420
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x7FFFFFFF);
    let encoding: u32 = 0xF2800420;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0x7FFFFFFE,
        "X0 should be 0x000000007FFFFFFE"
    );
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `SUB X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// unsigned overflow 64-bit (32)
#[test]
fn test_aarch32_vaddhn_t1a1_a_sub_oracle_32_9_f2800420() {
    // Test SUB 32-bit: unsigned overflow 64-bit (with oracle verification)
    // Encoding: 0xF2800420
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800420;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFE,
        "X0 should be 0x00000000FFFFFFFE"
    );
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `SUB X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// unsigned overflow 64-bit (64)
#[test]
fn test_aarch32_vaddhn_t1a1_a_sub_oracle_64_9_f2800420() {
    // Test SUB 64-bit: unsigned overflow 64-bit (with oracle verification)
    // Encoding: 0xF2800420
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800420;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFE,
        "X0 should be 0xFFFFFFFFFFFFFFFE"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `SUB X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// unsigned overflow 32-bit (32)
#[test]
fn test_aarch32_vaddhn_t1a1_a_sub_oracle_32_10_f2800420() {
    // Test SUB 32-bit: unsigned overflow 32-bit (with oracle verification)
    // Encoding: 0xF2800420
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800420;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFE,
        "X0 should be 0x00000000FFFFFFFE"
    );
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `SUB X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// unsigned overflow 32-bit (64)
#[test]
fn test_aarch32_vaddhn_t1a1_a_sub_oracle_64_10_f2800420() {
    // Test SUB 64-bit: unsigned overflow 32-bit (with oracle verification)
    // Encoding: 0xF2800420
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800420;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFE,
        "X0 should be 0x00000000FFFFFFFE"
    );
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `SUB SP, X1, #10`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "writes to stack pointer" }
/// SP destination (32)
#[test]
fn test_aarch32_vaddhn_t1a1_a_sub_oracle_32_rd31_sp_f2802c3f() {
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

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `SUB SP, X1, #10`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "writes to stack pointer" }
/// SP destination (64)
#[test]
fn test_aarch32_vaddhn_t1a1_a_sub_oracle_64_rd31_sp_f2802c3f() {
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

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `SUBS X0, X1, #10`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// simple addition/subtraction (32)
#[test]
fn test_aarch32_vaddhn_t1a1_a_subs_oracle_32_0_f2802c20() {
    // Test SUBS 32-bit: simple addition/subtraction (with oracle verification)
    // Encoding: 0xF2802C20
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x64);
    let encoding: u32 = 0xF2802C20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x59, "X0 should be 0x0000000000000059");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `SUBS X0, X1, #10`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// simple addition/subtraction (64)
#[test]
fn test_aarch32_vaddhn_t1a1_a_subs_oracle_64_0_f2802c20() {
    // Test SUBS 64-bit: simple addition/subtraction (with oracle verification)
    // Encoding: 0xF2802C20
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x64);
    let encoding: u32 = 0xF2802C20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x59, "X0 should be 0x0000000000000059");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `SUBS X0, X1, #0`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// zero operands (32)
#[test]
fn test_aarch32_vaddhn_t1a1_a_subs_oracle_32_1_f2800420() {
    // Test SUBS 32-bit: zero operands (with oracle verification)
    // Encoding: 0xF2800420
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2800420;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `SUBS X0, X1, #0`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// zero operands (64)
#[test]
fn test_aarch32_vaddhn_t1a1_a_subs_oracle_64_1_f2800420() {
    // Test SUBS 64-bit: zero operands (with oracle verification)
    // Encoding: 0xF2800420
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2800420;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `SUBS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// small values (32)
#[test]
fn test_aarch32_vaddhn_t1a1_a_subs_oracle_32_2_f2800420() {
    // Test SUBS 32-bit: small values (with oracle verification)
    // Encoding: 0xF2800420
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x1);
    let encoding: u32 = 0xF2800420;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x0, "X0 should be 0x0000000000000000");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z flag should be true");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `SUBS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// small values (64)
#[test]
fn test_aarch32_vaddhn_t1a1_a_subs_oracle_64_2_f2800420() {
    // Test SUBS 64-bit: small values (with oracle verification)
    // Encoding: 0xF2800420
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x1);
    let encoding: u32 = 0xF2800420;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x0, "X0 should be 0x0000000000000000");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z flag should be true");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `SUBS X0, X1, #4095`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// max imm12 unshifted (32)
#[test]
fn test_aarch32_vaddhn_t1a1_a_subs_oracle_32_3_f2bffc20() {
    // Test SUBS 32-bit: max imm12 unshifted (with oracle verification)
    // Encoding: 0xF2BFFC20
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2BFFC20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFF001,
        "X0 should be 0xFFFFFFFFFFFFF001"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `SUBS X0, X1, #4095`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// max imm12 unshifted (64)
#[test]
fn test_aarch32_vaddhn_t1a1_a_subs_oracle_64_3_f2bffc20() {
    // Test SUBS 64-bit: max imm12 unshifted (with oracle verification)
    // Encoding: 0xF2BFFC20
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2BFFC20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFF001,
        "X0 should be 0xFFFFFFFFFFFFF001"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `SUBS X0, X1, #4095, LSL #12`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// max imm12 shifted (32)
#[test]
fn test_aarch32_vaddhn_t1a1_a_subs_oracle_32_4_f2fffc20() {
    // Test SUBS 32-bit: max imm12 shifted (with oracle verification)
    // Encoding: 0xF2FFFC20
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2FFFC20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFF001000,
        "X0 should be 0xFFFFFFFFFF001000"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `SUBS X0, X1, #4095, LSL #12`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// max imm12 shifted (64)
#[test]
fn test_aarch32_vaddhn_t1a1_a_subs_oracle_64_4_f2fffc20() {
    // Test SUBS 64-bit: max imm12 shifted (with oracle verification)
    // Encoding: 0xF2FFFC20
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2FFFC20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFF001000,
        "X0 should be 0xFFFFFFFFFF001000"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `SUBS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// max u64 operand (32)
#[test]
fn test_aarch32_vaddhn_t1a1_a_subs_oracle_32_5_f2800420() {
    // Test SUBS 32-bit: max u64 operand (with oracle verification)
    // Encoding: 0xF2800420
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800420;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFE,
        "X0 should be 0x00000000FFFFFFFE"
    );
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `SUBS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// max u64 operand (64)
#[test]
fn test_aarch32_vaddhn_t1a1_a_subs_oracle_64_5_f2800420() {
    // Test SUBS 64-bit: max u64 operand (with oracle verification)
    // Encoding: 0xF2800420
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800420;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFE,
        "X0 should be 0xFFFFFFFFFFFFFFFE"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `SUBS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// zero result (for sub 1-1) (32)
#[test]
fn test_aarch32_vaddhn_t1a1_a_subs_oracle_32_6_f2800420() {
    // Test SUBS 32-bit: zero result (for sub 1-1) (with oracle verification)
    // Encoding: 0xF2800420
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2800420;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `SUBS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// zero result (for sub 1-1) (64)
#[test]
fn test_aarch32_vaddhn_t1a1_a_subs_oracle_64_6_f2800420() {
    // Test SUBS 64-bit: zero result (for sub 1-1) (with oracle verification)
    // Encoding: 0xF2800420
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2800420;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `SUBS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// signed overflow boundary 64-bit (32)
#[test]
fn test_aarch32_vaddhn_t1a1_a_subs_oracle_32_7_f2800420() {
    // Test SUBS 32-bit: signed overflow boundary 64-bit (with oracle verification)
    // Encoding: 0xF2800420
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800420;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFE,
        "X0 should be 0x00000000FFFFFFFE"
    );
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `SUBS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// signed overflow boundary 64-bit (64)
#[test]
fn test_aarch32_vaddhn_t1a1_a_subs_oracle_64_7_f2800420() {
    // Test SUBS 64-bit: signed overflow boundary 64-bit (with oracle verification)
    // Encoding: 0xF2800420
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800420;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFE,
        "X0 should be 0x7FFFFFFFFFFFFFFE"
    );
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `SUBS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// signed overflow boundary 32-bit (32)
#[test]
fn test_aarch32_vaddhn_t1a1_a_subs_oracle_32_8_f2800420() {
    // Test SUBS 32-bit: signed overflow boundary 32-bit (with oracle verification)
    // Encoding: 0xF2800420
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x7FFFFFFF);
    let encoding: u32 = 0xF2800420;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0x7FFFFFFE,
        "X0 should be 0x000000007FFFFFFE"
    );
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `SUBS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// signed overflow boundary 32-bit (64)
#[test]
fn test_aarch32_vaddhn_t1a1_a_subs_oracle_64_8_f2800420() {
    // Test SUBS 64-bit: signed overflow boundary 32-bit (with oracle verification)
    // Encoding: 0xF2800420
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x7FFFFFFF);
    let encoding: u32 = 0xF2800420;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0x7FFFFFFE,
        "X0 should be 0x000000007FFFFFFE"
    );
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `SUBS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// unsigned overflow 64-bit (32)
#[test]
fn test_aarch32_vaddhn_t1a1_a_subs_oracle_32_9_f2800420() {
    // Test SUBS 32-bit: unsigned overflow 64-bit (with oracle verification)
    // Encoding: 0xF2800420
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800420;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFE,
        "X0 should be 0x00000000FFFFFFFE"
    );
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `SUBS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// unsigned overflow 64-bit (64)
#[test]
fn test_aarch32_vaddhn_t1a1_a_subs_oracle_64_9_f2800420() {
    // Test SUBS 64-bit: unsigned overflow 64-bit (with oracle verification)
    // Encoding: 0xF2800420
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800420;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFE,
        "X0 should be 0xFFFFFFFFFFFFFFFE"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `SUBS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// unsigned overflow 32-bit (32)
#[test]
fn test_aarch32_vaddhn_t1a1_a_subs_oracle_32_10_f2800420() {
    // Test SUBS 32-bit: unsigned overflow 32-bit (with oracle verification)
    // Encoding: 0xF2800420
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800420;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFE,
        "X0 should be 0x00000000FFFFFFFE"
    );
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `SUBS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// unsigned overflow 32-bit (64)
#[test]
fn test_aarch32_vaddhn_t1a1_a_subs_oracle_64_10_f2800420() {
    // Test SUBS 64-bit: unsigned overflow 32-bit (with oracle verification)
    // Encoding: 0xF2800420
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800420;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFE,
        "X0 should be 0x00000000FFFFFFFE"
    );
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `SUBS ZR, X1, #10`
/// Requirement: RegisterSpecial { reg: Zr, behavior: "result discarded, flags set" }
/// ZR destination (32)
#[test]
fn test_aarch32_vaddhn_t1a1_a_subs_oracle_32_rd31_zr_f2802c3f() {
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

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `SUBS ZR, X1, #10`
/// Requirement: RegisterSpecial { reg: Zr, behavior: "result discarded, flags set" }
/// ZR destination (64)
#[test]
fn test_aarch32_vaddhn_t1a1_a_subs_oracle_64_rd31_zr_f2802c3f() {
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

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `ADD R0, R1, #10`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// small immediate (Rn=0x00000000)
#[test]
fn test_aarch32_vaddhn_t1a1_a_a32_add_sub_imm_0_0_f281000a() {
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

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `ADD R0, R1, #10`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// small immediate (Rn=0x00000064)
#[test]
fn test_aarch32_vaddhn_t1a1_a_a32_add_sub_imm_0_64_f281000a() {
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

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `ADD R0, R1, #10`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// small immediate (Rn=0x7FFFFFFF)
#[test]
fn test_aarch32_vaddhn_t1a1_a_a32_add_sub_imm_0_7fffffff_f281000a() {
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

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `ADD R0, R1, #10`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// small immediate (Rn=0x80000000)
#[test]
fn test_aarch32_vaddhn_t1a1_a_a32_add_sub_imm_0_80000000_f281000a() {
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

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `ADD R0, R1, #10`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// small immediate (Rn=0xFFFFFFFF)
#[test]
fn test_aarch32_vaddhn_t1a1_a_a32_add_sub_imm_0_ffffffff_f281000a() {
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

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `ADD R0, R1, #255`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// max imm8 (Rn=0x00000000)
#[test]
fn test_aarch32_vaddhn_t1a1_a_a32_add_sub_imm_1_0_f28100ff() {
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

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `ADD R0, R1, #255`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// max imm8 (Rn=0x00000064)
#[test]
fn test_aarch32_vaddhn_t1a1_a_a32_add_sub_imm_1_64_f28100ff() {
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

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `ADD R0, R1, #255`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// max imm8 (Rn=0x7FFFFFFF)
#[test]
fn test_aarch32_vaddhn_t1a1_a_a32_add_sub_imm_1_7fffffff_f28100ff() {
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

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `ADD R0, R1, #255`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// max imm8 (Rn=0x80000000)
#[test]
fn test_aarch32_vaddhn_t1a1_a_a32_add_sub_imm_1_80000000_f28100ff() {
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

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `ADD R0, R1, #255`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// max imm8 (Rn=0xFFFFFFFF)
#[test]
fn test_aarch32_vaddhn_t1a1_a_a32_add_sub_imm_1_ffffffff_f28100ff() {
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

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `ADD R0, R1, #32`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// rotated by 2 (Rn=0x00000000)
#[test]
fn test_aarch32_vaddhn_t1a1_a_a32_add_sub_imm_2_0_f2810180() {
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

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `ADD R0, R1, #32`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// rotated by 2 (Rn=0x00000064)
#[test]
fn test_aarch32_vaddhn_t1a1_a_a32_add_sub_imm_2_64_f2810180() {
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

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `ADD R0, R1, #32`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// rotated by 2 (Rn=0x7FFFFFFF)
#[test]
fn test_aarch32_vaddhn_t1a1_a_a32_add_sub_imm_2_7fffffff_f2810180() {
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

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `ADD R0, R1, #32`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// rotated by 2 (Rn=0x80000000)
#[test]
fn test_aarch32_vaddhn_t1a1_a_a32_add_sub_imm_2_80000000_f2810180() {
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

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `ADD R0, R1, #32`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// rotated by 2 (Rn=0xFFFFFFFF)
#[test]
fn test_aarch32_vaddhn_t1a1_a_a32_add_sub_imm_2_ffffffff_f2810180() {
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

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `ADD R0, R1, #251658240`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// rotated by 8 (Rn=0x00000000)
#[test]
fn test_aarch32_vaddhn_t1a1_a_a32_add_sub_imm_3_0_f281040f() {
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

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `ADD R0, R1, #251658240`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// rotated by 8 (Rn=0x00000064)
#[test]
fn test_aarch32_vaddhn_t1a1_a_a32_add_sub_imm_3_64_f281040f() {
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

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `ADD R0, R1, #251658240`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// rotated by 8 (Rn=0x7FFFFFFF)
#[test]
fn test_aarch32_vaddhn_t1a1_a_a32_add_sub_imm_3_7fffffff_f281040f() {
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

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `ADD R0, R1, #251658240`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// rotated by 8 (Rn=0x80000000)
#[test]
fn test_aarch32_vaddhn_t1a1_a_a32_add_sub_imm_3_80000000_f281040f() {
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

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `ADD R0, R1, #251658240`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// rotated by 8 (Rn=0xFFFFFFFF)
#[test]
fn test_aarch32_vaddhn_t1a1_a_a32_add_sub_imm_3_ffffffff_f281040f() {
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

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `ADD R0, R1, #0`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// zero immediate (Rn=0x00000000)
#[test]
fn test_aarch32_vaddhn_t1a1_a_a32_add_sub_imm_4_0_f2810000() {
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

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `ADD R0, R1, #0`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// zero immediate (Rn=0x00000064)
#[test]
fn test_aarch32_vaddhn_t1a1_a_a32_add_sub_imm_4_64_f2810000() {
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

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `ADD R0, R1, #0`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// zero immediate (Rn=0x7FFFFFFF)
#[test]
fn test_aarch32_vaddhn_t1a1_a_a32_add_sub_imm_4_7fffffff_f2810000() {
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

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `ADD R0, R1, #0`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// zero immediate (Rn=0x80000000)
#[test]
fn test_aarch32_vaddhn_t1a1_a_a32_add_sub_imm_4_80000000_f2810000() {
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

/// Provenance: aarch32_VADDHN_T1A1_A
/// ASL: `ADD R0, R1, #0`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// zero immediate (Rn=0xFFFFFFFF)
#[test]
fn test_aarch32_vaddhn_t1a1_a_a32_add_sub_imm_4_ffffffff_f2810000() {
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
// aarch32_VADD_f_A Tests
// ============================================================================

/// Provenance: aarch32_VADD_f_A1_A
/// ASL: `field D 22 +: 1`
/// Requirement: FieldBoundary { field: "D", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_vadd_f_a1_a_field_d_0_min_d00_f2000d00() {
    // Encoding: 0xF2000D00
    // Test aarch32_VADD_f_A1_A field D = 0 (Min)
    // ISET: A32
    // Fields: Vn=0, M=0, Vm=0, Q=0, Vd=0, D=0, N=0, sz=0
    let encoding: u32 = 0xF2000D00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VADD_f_A1_A
/// ASL: `field D 22 +: 1`
/// Requirement: FieldBoundary { field: "D", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_vadd_f_a1_a_field_d_1_max_d00_f2400d00() {
    // Encoding: 0xF2400D00
    // Test aarch32_VADD_f_A1_A field D = 1 (Max)
    // ISET: A32
    // Fields: sz=0, M=0, D=1, Vm=0, N=0, Vd=0, Q=0, Vn=0
    let encoding: u32 = 0xF2400D00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VADD_f_A1_A
/// ASL: `field sz 20 +: 1`
/// Requirement: FieldBoundary { field: "sz", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch32_vadd_f_a1_a_field_sz_0_min_d00_f2000d00() {
    // Encoding: 0xF2000D00
    // Test aarch32_VADD_f_A1_A field sz = 0 (Min)
    // ISET: A32
    // Fields: Vn=0, D=0, Vd=0, sz=0, M=0, Vm=0, N=0, Q=0
    let encoding: u32 = 0xF2000D00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VADD_f_A1_A
/// ASL: `field sz 20 +: 1`
/// Requirement: FieldBoundary { field: "sz", value: 1, boundary: Max }
/// 16-bit / halfword size
#[test]
fn test_aarch32_vadd_f_a1_a_field_sz_1_max_d00_f2100d00() {
    // Encoding: 0xF2100D00
    // Test aarch32_VADD_f_A1_A field sz = 1 (Max)
    // ISET: A32
    // Fields: sz=1, Vd=0, Vm=0, Q=0, M=0, D=0, Vn=0, N=0
    let encoding: u32 = 0xF2100D00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VADD_f_A1_A
/// ASL: `field Vn 16 +: 4`
/// Requirement: FieldBoundary { field: "Vn", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_aarch32_vadd_f_a1_a_field_vn_0_min_d00_f2000d00() {
    // Encoding: 0xF2000D00
    // Test aarch32_VADD_f_A1_A field Vn = 0 (Min)
    // ISET: A32
    // Fields: D=0, Vn=0, Vd=0, sz=0, N=0, M=0, Q=0, Vm=0
    let encoding: u32 = 0xF2000D00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VADD_f_A1_A
/// ASL: `field Vn 16 +: 4`
/// Requirement: FieldBoundary { field: "Vn", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_aarch32_vadd_f_a1_a_field_vn_1_poweroftwo_d00_f2010d00() {
    // Encoding: 0xF2010D00
    // Test aarch32_VADD_f_A1_A field Vn = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Vn=1, D=0, Vm=0, N=0, Vd=0, sz=0, Q=0, M=0
    let encoding: u32 = 0xF2010D00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VADD_f_A1_A
/// ASL: `field Vd 12 +: 4`
/// Requirement: FieldBoundary { field: "Vd", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_aarch32_vadd_f_a1_a_field_vd_0_min_d00_f2000d00() {
    // Encoding: 0xF2000D00
    // Test aarch32_VADD_f_A1_A field Vd = 0 (Min)
    // ISET: A32
    // Fields: Vn=0, N=0, Vd=0, Q=0, M=0, Vm=0, D=0, sz=0
    let encoding: u32 = 0xF2000D00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VADD_f_A1_A
/// ASL: `field Vd 12 +: 4`
/// Requirement: FieldBoundary { field: "Vd", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_aarch32_vadd_f_a1_a_field_vd_1_poweroftwo_d00_f2001d00() {
    // Encoding: 0xF2001D00
    // Test aarch32_VADD_f_A1_A field Vd = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Vn=0, sz=0, Vd=1, N=0, D=0, M=0, Vm=0, Q=0
    let encoding: u32 = 0xF2001D00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VADD_f_A1_A
/// ASL: `field N 7 +: 1`
/// Requirement: FieldBoundary { field: "N", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_vadd_f_a1_a_field_n_0_min_d00_f2000d00() {
    // Encoding: 0xF2000D00
    // Test aarch32_VADD_f_A1_A field N = 0 (Min)
    // ISET: A32
    // Fields: Vd=0, Vm=0, N=0, M=0, Vn=0, sz=0, D=0, Q=0
    let encoding: u32 = 0xF2000D00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VADD_f_A1_A
/// ASL: `field N 7 +: 1`
/// Requirement: FieldBoundary { field: "N", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_vadd_f_a1_a_field_n_1_max_d00_f2000d80() {
    // Encoding: 0xF2000D80
    // Test aarch32_VADD_f_A1_A field N = 1 (Max)
    // ISET: A32
    // Fields: Q=0, N=1, D=0, sz=0, Vm=0, Vn=0, M=0, Vd=0
    let encoding: u32 = 0xF2000D80;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VADD_f_A1_A
/// ASL: `field Q 6 +: 1`
/// Requirement: FieldBoundary { field: "Q", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch32_vadd_f_a1_a_field_q_0_min_d00_f2000d00() {
    // Encoding: 0xF2000D00
    // Test aarch32_VADD_f_A1_A field Q = 0 (Min)
    // ISET: A32
    // Fields: D=0, M=0, N=0, Vm=0, sz=0, Q=0, Vn=0, Vd=0
    let encoding: u32 = 0xF2000D00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VADD_f_A1_A
/// ASL: `field Q 6 +: 1`
/// Requirement: FieldBoundary { field: "Q", value: 1, boundary: Max }
/// 16-bit / halfword size
#[test]
fn test_aarch32_vadd_f_a1_a_field_q_1_max_d00_f2000d40() {
    // Encoding: 0xF2000D40
    // Test aarch32_VADD_f_A1_A field Q = 1 (Max)
    // ISET: A32
    // Fields: M=0, sz=0, N=0, Vm=0, Q=1, Vd=0, Vn=0, D=0
    let encoding: u32 = 0xF2000D40;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VADD_f_A1_A
/// ASL: `field M 5 +: 1`
/// Requirement: FieldBoundary { field: "M", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_vadd_f_a1_a_field_m_0_min_d00_f2000d00() {
    // Encoding: 0xF2000D00
    // Test aarch32_VADD_f_A1_A field M = 0 (Min)
    // ISET: A32
    // Fields: Vm=0, Q=0, D=0, sz=0, Vn=0, Vd=0, M=0, N=0
    let encoding: u32 = 0xF2000D00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VADD_f_A1_A
/// ASL: `field M 5 +: 1`
/// Requirement: FieldBoundary { field: "M", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_vadd_f_a1_a_field_m_1_max_d00_f2000d20() {
    // Encoding: 0xF2000D20
    // Test aarch32_VADD_f_A1_A field M = 1 (Max)
    // ISET: A32
    // Fields: sz=0, M=1, N=0, D=0, Vm=0, Q=0, Vd=0, Vn=0
    let encoding: u32 = 0xF2000D20;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VADD_f_A1_A
/// ASL: `field Vm 0 +: 4`
/// Requirement: FieldBoundary { field: "Vm", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_aarch32_vadd_f_a1_a_field_vm_0_min_d00_f2000d00() {
    // Encoding: 0xF2000D00
    // Test aarch32_VADD_f_A1_A field Vm = 0 (Min)
    // ISET: A32
    // Fields: Q=0, sz=0, Vd=0, D=0, Vm=0, Vn=0, N=0, M=0
    let encoding: u32 = 0xF2000D00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VADD_f_A1_A
/// ASL: `field Vm 0 +: 4`
/// Requirement: FieldBoundary { field: "Vm", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_aarch32_vadd_f_a1_a_field_vm_1_poweroftwo_d00_f2000d01() {
    // Encoding: 0xF2000D01
    // Test aarch32_VADD_f_A1_A field Vm = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Vn=0, N=0, Q=0, Vd=0, M=0, Vm=1, D=0, sz=0
    let encoding: u32 = 0xF2000D01;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VADD_f_A1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// D=0 (minimum value)
#[test]
fn test_aarch32_vadd_f_a1_a_combo_0_d00_f2000d00() {
    // Encoding: 0xF2000D00
    // Test aarch32_VADD_f_A1_A field combination: D=0, sz=0, Vn=0, Vd=0, N=0, Q=0, M=0, Vm=0
    // ISET: A32
    // Fields: Q=0, sz=0, Vn=0, N=0, M=0, D=0, Vd=0, Vm=0
    let encoding: u32 = 0xF2000D00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VADD_f_A1_A
/// ASL: `field sz = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "sz", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch32_vadd_f_a1_a_special_sz_0_size_variant_0_3328_f2000d00() {
    // Encoding: 0xF2000D00
    // Test aarch32_VADD_f_A1_A special value sz = 0 (Size variant 0)
    // ISET: A32
    // Fields: Vd=0, M=0, D=0, Vn=0, Q=0, N=0, sz=0, Vm=0
    let encoding: u32 = 0xF2000D00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VADD_f_A1_A
/// ASL: `field sz = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "sz", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch32_vadd_f_a1_a_special_sz_1_size_variant_1_3328_f2100d00() {
    // Encoding: 0xF2100D00
    // Test aarch32_VADD_f_A1_A special value sz = 1 (Size variant 1)
    // ISET: A32
    // Fields: D=0, Vd=0, Vn=0, M=0, Vm=0, sz=1, Q=0, N=0
    let encoding: u32 = 0xF2100D00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VADD_f_A1_A
/// ASL: `field Q = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "Q", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch32_vadd_f_a1_a_special_q_0_size_variant_0_3328_f2000d00() {
    // Encoding: 0xF2000D00
    // Test aarch32_VADD_f_A1_A special value Q = 0 (Size variant 0)
    // ISET: A32
    // Fields: Q=0, sz=0, N=0, D=0, Vn=0, Vd=0, M=0, Vm=0
    let encoding: u32 = 0xF2000D00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VADD_f_A1_A
/// ASL: `field Q = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "Q", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch32_vadd_f_a1_a_special_q_1_size_variant_1_3328_f2000d40() {
    // Encoding: 0xF2000D40
    // Test aarch32_VADD_f_A1_A special value Q = 1 (Size variant 1)
    // ISET: A32
    // Fields: Vm=0, M=0, D=0, Vn=0, Q=1, Vd=0, sz=0, N=0
    let encoding: u32 = 0xF2000D40;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VADD_f_A1_A
/// ASL: `Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "Q" }), rhs: Binary { op: And, lhs: LitBits([true]), rhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: "Vd" }), indices: [Single(LitInt(0))] }, rhs: Binary { op: Or, lhs: LitBits([true]), rhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: "Vn" }), indices: [Single(LitInt(0))] } } }, rhs: Binary { op: Or, lhs: LitBits([true]), rhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: "Vm" }), indices: [Single(LitInt(0))] } } }, rhs: LitBits([true]) } } }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"Q\" }), rhs: Binary { op: And, lhs: LitBits([true]), rhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: \"Vd\" }), indices: [Single(LitInt(0))] }, rhs: Binary { op: Or, lhs: LitBits([true]), rhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: \"Vn\" }), indices: [Single(LitInt(0))] } } }, rhs: Binary { op: Or, lhs: LitBits([true]), rhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: \"Vm\" }), indices: [Single(LitInt(0))] } } }, rhs: LitBits([true]) } } }" }
/// triggers Undefined
#[test]
fn test_aarch32_vadd_f_a1_a_invalid_0_d00_f2000d00() {
    // Encoding: 0xF2000D00
    // Test aarch32_VADD_f_A1_A invalid encoding: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "Q" }), rhs: Binary { op: And, lhs: LitBits([true]), rhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: "Vd" }), indices: [Single(LitInt(0))] }, rhs: Binary { op: Or, lhs: LitBits([true]), rhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: "Vn" }), indices: [Single(LitInt(0))] } } }, rhs: Binary { op: Or, lhs: LitBits([true]), rhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: "Vm" }), indices: [Single(LitInt(0))] } } }, rhs: LitBits([true]) } } }
    // ISET: A32
    // Fields: M=0, D=0, sz=0, Vn=0, Vd=0, Vm=0, Q=0, N=0
    let encoding: u32 = 0xF2000D00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || !matches!(exit.unwrap(), CpuExit::Continue),
        "expected UNDEFINED for A32 encoding 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch32_VADD_f_A1_A
/// ASL: `Unconditional UNDEFINED`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNDEFINED" }
/// triggers Undefined
#[test]
fn test_aarch32_vadd_f_a1_a_invalid_1_d00_f2000d00() {
    // Encoding: 0xF2000D00
    // Test aarch32_VADD_f_A1_A invalid encoding: Unconditional UNDEFINED
    // ISET: A32
    // Fields: N=0, sz=0, D=0, M=0, Vn=0, Vd=0, Q=0, Vm=0
    let encoding: u32 = 0xF2000D00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || !matches!(exit.unwrap(), CpuExit::Continue),
        "expected UNDEFINED for A32 encoding 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch32_VADD_f_A1_A
/// ASL: `Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "sz" }), rhs: Binary { op: And, lhs: LitBits([true]), rhs: Unary { op: Not, operand: Call { name: QualifiedIdentifier { qualifier: Any, name: "HaveFP16Ext" }, args: [] } } } }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"sz\" }), rhs: Binary { op: And, lhs: LitBits([true]), rhs: Unary { op: Not, operand: Call { name: QualifiedIdentifier { qualifier: Any, name: \"HaveFP16Ext\" }, args: [] } } } }" }
/// triggers Undefined
#[test]
fn test_aarch32_vadd_f_a1_a_invalid_2_d00_f2000d00() {
    // Encoding: 0xF2000D00
    // Test aarch32_VADD_f_A1_A invalid encoding: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "sz" }), rhs: Binary { op: And, lhs: LitBits([true]), rhs: Unary { op: Not, operand: Call { name: QualifiedIdentifier { qualifier: Any, name: "HaveFP16Ext" }, args: [] } } } }
    // ISET: A32
    // Fields: sz=0, Vn=0, M=0, Vm=0, D=0, N=0, Q=0, Vd=0
    let encoding: u32 = 0xF2000D00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || !matches!(exit.unwrap(), CpuExit::Continue),
        "expected UNDEFINED for A32 encoding 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch32_VADD_f_A1_A
/// ASL: `Unconditional UNDEFINED`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNDEFINED" }
/// triggers Undefined
#[test]
fn test_aarch32_vadd_f_a1_a_invalid_3_d00_f2000d00() {
    // Encoding: 0xF2000D00
    // Test aarch32_VADD_f_A1_A invalid encoding: Unconditional UNDEFINED
    // ISET: A32
    // Fields: M=0, Vn=0, N=0, Q=0, Vd=0, Vm=0, sz=0, D=0
    let encoding: u32 = 0xF2000D00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || !matches!(exit.unwrap(), CpuExit::Continue),
        "expected UNDEFINED for A32 encoding 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch32_VADD_f_A2_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 0, boundary: Min }
/// condition EQ (equal)
#[test]
fn test_aarch32_vadd_f_a2_a_field_cond_0_min_800_0e300800() {
    // Encoding: 0x0E300800
    // Test aarch32_VADD_f_A2_A field cond = 0 (Min)
    // ISET: A32
    // Fields: Vd=0, Vm=0, Vn=0, size=0, cond=0, D=0, N=0, M=0
    let encoding: u32 = 0x0E300800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VADD_f_A2_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 1, boundary: PowerOfTwo }
/// condition NE (not equal)
#[test]
fn test_aarch32_vadd_f_a2_a_field_cond_1_poweroftwo_800_1e300800() {
    // Encoding: 0x1E300800
    // Test aarch32_VADD_f_A2_A field cond = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Vd=0, N=0, size=0, cond=1, Vm=0, Vn=0, M=0, D=0
    let encoding: u32 = 0x1E300800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VADD_f_A2_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 2, boundary: PowerOfTwo }
/// condition CS/HS (carry set)
#[test]
fn test_aarch32_vadd_f_a2_a_field_cond_2_poweroftwo_800_2e300800() {
    // Encoding: 0x2E300800
    // Test aarch32_VADD_f_A2_A field cond = 2 (PowerOfTwo)
    // ISET: A32
    // Fields: D=0, Vd=0, cond=2, Vn=0, M=0, Vm=0, size=0, N=0
    let encoding: u32 = 0x2E300800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VADD_f_A2_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 3, boundary: PowerOfTwo }
/// condition CC/LO (carry clear)
#[test]
fn test_aarch32_vadd_f_a2_a_field_cond_3_poweroftwo_800_3e300800() {
    // Encoding: 0x3E300800
    // Test aarch32_VADD_f_A2_A field cond = 3 (PowerOfTwo)
    // ISET: A32
    // Fields: D=0, Vm=0, M=0, size=0, Vn=0, cond=3, N=0, Vd=0
    let encoding: u32 = 0x3E300800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VADD_f_A2_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 4, boundary: PowerOfTwo }
/// condition MI (minus/negative)
#[test]
fn test_aarch32_vadd_f_a2_a_field_cond_4_poweroftwo_800_4e300800() {
    // Encoding: 0x4E300800
    // Test aarch32_VADD_f_A2_A field cond = 4 (PowerOfTwo)
    // ISET: A32
    // Fields: Vm=0, D=0, Vd=0, size=0, N=0, cond=4, Vn=0, M=0
    let encoding: u32 = 0x4E300800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VADD_f_A2_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 5, boundary: PowerOfTwo }
/// condition PL (plus/positive)
#[test]
fn test_aarch32_vadd_f_a2_a_field_cond_5_poweroftwo_800_5e300800() {
    // Encoding: 0x5E300800
    // Test aarch32_VADD_f_A2_A field cond = 5 (PowerOfTwo)
    // ISET: A32
    // Fields: N=0, Vn=0, D=0, Vd=0, Vm=0, size=0, M=0, cond=5
    let encoding: u32 = 0x5E300800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VADD_f_A2_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 6, boundary: PowerOfTwo }
/// condition VS (overflow set)
#[test]
fn test_aarch32_vadd_f_a2_a_field_cond_6_poweroftwo_800_6e300800() {
    // Encoding: 0x6E300800
    // Test aarch32_VADD_f_A2_A field cond = 6 (PowerOfTwo)
    // ISET: A32
    // Fields: D=0, Vm=0, size=0, Vn=0, N=0, M=0, Vd=0, cond=6
    let encoding: u32 = 0x6E300800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VADD_f_A2_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 7, boundary: PowerOfTwo }
/// condition VC (overflow clear)
#[test]
fn test_aarch32_vadd_f_a2_a_field_cond_7_poweroftwo_800_7e300800() {
    // Encoding: 0x7E300800
    // Test aarch32_VADD_f_A2_A field cond = 7 (PowerOfTwo)
    // ISET: A32
    // Fields: Vd=0, size=0, Vn=0, N=0, D=0, cond=7, M=0, Vm=0
    let encoding: u32 = 0x7E300800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VADD_f_A2_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 8, boundary: PowerOfTwo }
/// condition HI (unsigned higher)
#[test]
fn test_aarch32_vadd_f_a2_a_field_cond_8_poweroftwo_800_8e300800() {
    // Encoding: 0x8E300800
    // Test aarch32_VADD_f_A2_A field cond = 8 (PowerOfTwo)
    // ISET: A32
    // Fields: N=0, Vm=0, size=0, Vn=0, Vd=0, D=0, M=0, cond=8
    let encoding: u32 = 0x8E300800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VADD_f_A2_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 9, boundary: PowerOfTwo }
/// condition LS (unsigned lower or same)
#[test]
fn test_aarch32_vadd_f_a2_a_field_cond_9_poweroftwo_800_9e300800() {
    // Encoding: 0x9E300800
    // Test aarch32_VADD_f_A2_A field cond = 9 (PowerOfTwo)
    // ISET: A32
    // Fields: Vn=0, N=0, M=0, size=0, Vd=0, D=0, Vm=0, cond=9
    let encoding: u32 = 0x9E300800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VADD_f_A2_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 10, boundary: PowerOfTwo }
/// condition GE (signed >=)
#[test]
fn test_aarch32_vadd_f_a2_a_field_cond_10_poweroftwo_800_ae300800() {
    // Encoding: 0xAE300800
    // Test aarch32_VADD_f_A2_A field cond = 10 (PowerOfTwo)
    // ISET: A32
    // Fields: Vm=0, cond=10, D=0, Vn=0, Vd=0, M=0, N=0, size=0
    let encoding: u32 = 0xAE300800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VADD_f_A2_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 11, boundary: PowerOfTwo }
/// condition LT (signed <)
#[test]
fn test_aarch32_vadd_f_a2_a_field_cond_11_poweroftwo_800_be300800() {
    // Encoding: 0xBE300800
    // Test aarch32_VADD_f_A2_A field cond = 11 (PowerOfTwo)
    // ISET: A32
    // Fields: Vn=0, D=0, N=0, cond=11, Vd=0, M=0, size=0, Vm=0
    let encoding: u32 = 0xBE300800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VADD_f_A2_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 12, boundary: PowerOfTwo }
/// condition GT (signed >)
#[test]
fn test_aarch32_vadd_f_a2_a_field_cond_12_poweroftwo_800_ce300800() {
    // Encoding: 0xCE300800
    // Test aarch32_VADD_f_A2_A field cond = 12 (PowerOfTwo)
    // ISET: A32
    // Fields: M=0, D=0, size=0, Vd=0, N=0, Vm=0, cond=12, Vn=0
    let encoding: u32 = 0xCE300800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VADD_f_A2_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 13, boundary: PowerOfTwo }
/// condition LE (signed <=)
#[test]
fn test_aarch32_vadd_f_a2_a_field_cond_13_poweroftwo_800_de300800() {
    // Encoding: 0xDE300800
    // Test aarch32_VADD_f_A2_A field cond = 13 (PowerOfTwo)
    // ISET: A32
    // Fields: Vd=0, Vn=0, N=0, D=0, cond=13, M=0, Vm=0, size=0
    let encoding: u32 = 0xDE300800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VADD_f_A2_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 14, boundary: PowerOfTwo }
/// condition AL (always)
#[test]
fn test_aarch32_vadd_f_a2_a_field_cond_14_poweroftwo_800_ee300800() {
    // Encoding: 0xEE300800
    // Test aarch32_VADD_f_A2_A field cond = 14 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=14, N=0, D=0, M=0, Vd=0, size=0, Vm=0, Vn=0
    let encoding: u32 = 0xEE300800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VADD_f_A2_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 15, boundary: Max }
/// condition NV (never, reserved)
#[test]
fn test_aarch32_vadd_f_a2_a_field_cond_15_max_800_fe300800() {
    // Encoding: 0xFE300800
    // Test aarch32_VADD_f_A2_A field cond = 15 (Max)
    // ISET: A32
    // Fields: size=0, N=0, D=0, Vd=0, Vn=0, cond=15, M=0, Vm=0
    let encoding: u32 = 0xFE300800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VADD_f_A2_A
/// ASL: `field D 22 +: 1`
/// Requirement: FieldBoundary { field: "D", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_vadd_f_a2_a_field_d_0_min_800_0e300800() {
    // Encoding: 0x0E300800
    // Test aarch32_VADD_f_A2_A field D = 0 (Min)
    // ISET: A32
    // Fields: Vn=0, Vd=0, D=0, size=0, Vm=0, cond=0, N=0, M=0
    let encoding: u32 = 0x0E300800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VADD_f_A2_A
/// ASL: `field D 22 +: 1`
/// Requirement: FieldBoundary { field: "D", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_vadd_f_a2_a_field_d_1_max_800_0e700800() {
    // Encoding: 0x0E700800
    // Test aarch32_VADD_f_A2_A field D = 1 (Max)
    // ISET: A32
    // Fields: cond=0, Vd=0, M=0, Vm=0, Vn=0, D=1, size=0, N=0
    let encoding: u32 = 0x0E700800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VADD_f_A2_A
/// ASL: `field Vn 16 +: 4`
/// Requirement: FieldBoundary { field: "Vn", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_aarch32_vadd_f_a2_a_field_vn_0_min_800_0e300800() {
    // Encoding: 0x0E300800
    // Test aarch32_VADD_f_A2_A field Vn = 0 (Min)
    // ISET: A32
    // Fields: size=0, Vn=0, N=0, Vd=0, D=0, Vm=0, cond=0, M=0
    let encoding: u32 = 0x0E300800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VADD_f_A2_A
/// ASL: `field Vn 16 +: 4`
/// Requirement: FieldBoundary { field: "Vn", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_aarch32_vadd_f_a2_a_field_vn_1_poweroftwo_800_0e310800() {
    // Encoding: 0x0E310800
    // Test aarch32_VADD_f_A2_A field Vn = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=0, D=0, size=0, Vn=1, Vd=0, Vm=0, M=0, N=0
    let encoding: u32 = 0x0E310800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VADD_f_A2_A
/// ASL: `field Vd 12 +: 4`
/// Requirement: FieldBoundary { field: "Vd", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_aarch32_vadd_f_a2_a_field_vd_0_min_800_0e300800() {
    // Encoding: 0x0E300800
    // Test aarch32_VADD_f_A2_A field Vd = 0 (Min)
    // ISET: A32
    // Fields: Vd=0, size=0, cond=0, Vn=0, D=0, N=0, M=0, Vm=0
    let encoding: u32 = 0x0E300800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VADD_f_A2_A
/// ASL: `field Vd 12 +: 4`
/// Requirement: FieldBoundary { field: "Vd", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_aarch32_vadd_f_a2_a_field_vd_1_poweroftwo_800_0e301800() {
    // Encoding: 0x0E301800
    // Test aarch32_VADD_f_A2_A field Vd = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: D=0, Vn=0, cond=0, N=0, Vd=1, M=0, Vm=0, size=0
    let encoding: u32 = 0x0E301800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VADD_f_A2_A
/// ASL: `field size 8 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch32_vadd_f_a2_a_field_size_0_min_800_0e300800() {
    // Encoding: 0x0E300800
    // Test aarch32_VADD_f_A2_A field size = 0 (Min)
    // ISET: A32
    // Fields: Vn=0, M=0, Vm=0, size=0, Vd=0, cond=0, N=0, D=0
    let encoding: u32 = 0x0E300800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VADD_f_A2_A
/// ASL: `field size 8 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 1, boundary: PowerOfTwo }
/// 16-bit / halfword size
#[test]
fn test_aarch32_vadd_f_a2_a_field_size_1_poweroftwo_800_0e300900() {
    // Encoding: 0x0E300900
    // Test aarch32_VADD_f_A2_A field size = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: size=1, D=0, M=0, Vm=0, Vd=0, Vn=0, N=0, cond=0
    let encoding: u32 = 0x0E300900;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VADD_f_A2_A
/// ASL: `field size 8 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 2, boundary: PowerOfTwo }
/// 32-bit / word size
#[test]
fn test_aarch32_vadd_f_a2_a_field_size_2_poweroftwo_800_0e300a00() {
    // Encoding: 0x0E300A00
    // Test aarch32_VADD_f_A2_A field size = 2 (PowerOfTwo)
    // ISET: A32
    // Fields: M=0, Vd=0, Vm=0, cond=0, D=0, size=2, N=0, Vn=0
    let encoding: u32 = 0x0E300A00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VADD_f_A2_A
/// ASL: `field size 8 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 3, boundary: Max }
/// 64-bit / doubleword size
#[test]
fn test_aarch32_vadd_f_a2_a_field_size_3_max_800_0e300b00() {
    // Encoding: 0x0E300B00
    // Test aarch32_VADD_f_A2_A field size = 3 (Max)
    // ISET: A32
    // Fields: cond=0, Vn=0, D=0, Vd=0, size=3, N=0, M=0, Vm=0
    let encoding: u32 = 0x0E300B00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VADD_f_A2_A
/// ASL: `field N 7 +: 1`
/// Requirement: FieldBoundary { field: "N", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_vadd_f_a2_a_field_n_0_min_800_0e300800() {
    // Encoding: 0x0E300800
    // Test aarch32_VADD_f_A2_A field N = 0 (Min)
    // ISET: A32
    // Fields: Vn=0, D=0, Vm=0, Vd=0, N=0, cond=0, size=0, M=0
    let encoding: u32 = 0x0E300800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VADD_f_A2_A
/// ASL: `field N 7 +: 1`
/// Requirement: FieldBoundary { field: "N", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_vadd_f_a2_a_field_n_1_max_800_0e300880() {
    // Encoding: 0x0E300880
    // Test aarch32_VADD_f_A2_A field N = 1 (Max)
    // ISET: A32
    // Fields: cond=0, Vn=0, Vd=0, N=1, M=0, D=0, Vm=0, size=0
    let encoding: u32 = 0x0E300880;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VADD_f_A2_A
/// ASL: `field M 5 +: 1`
/// Requirement: FieldBoundary { field: "M", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_vadd_f_a2_a_field_m_0_min_800_0e300800() {
    // Encoding: 0x0E300800
    // Test aarch32_VADD_f_A2_A field M = 0 (Min)
    // ISET: A32
    // Fields: Vd=0, N=0, size=0, cond=0, M=0, Vm=0, D=0, Vn=0
    let encoding: u32 = 0x0E300800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VADD_f_A2_A
/// ASL: `field M 5 +: 1`
/// Requirement: FieldBoundary { field: "M", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_vadd_f_a2_a_field_m_1_max_800_0e300820() {
    // Encoding: 0x0E300820
    // Test aarch32_VADD_f_A2_A field M = 1 (Max)
    // ISET: A32
    // Fields: N=0, Vn=0, Vd=0, D=0, cond=0, Vm=0, M=1, size=0
    let encoding: u32 = 0x0E300820;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VADD_f_A2_A
/// ASL: `field Vm 0 +: 4`
/// Requirement: FieldBoundary { field: "Vm", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_aarch32_vadd_f_a2_a_field_vm_0_min_800_0e300800() {
    // Encoding: 0x0E300800
    // Test aarch32_VADD_f_A2_A field Vm = 0 (Min)
    // ISET: A32
    // Fields: Vn=0, Vm=0, Vd=0, M=0, size=0, N=0, cond=0, D=0
    let encoding: u32 = 0x0E300800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VADD_f_A2_A
/// ASL: `field Vm 0 +: 4`
/// Requirement: FieldBoundary { field: "Vm", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_aarch32_vadd_f_a2_a_field_vm_1_poweroftwo_800_0e300801() {
    // Encoding: 0x0E300801
    // Test aarch32_VADD_f_A2_A field Vm = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Vd=0, size=0, Vn=0, N=0, cond=0, M=0, Vm=1, D=0
    let encoding: u32 = 0x0E300801;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VADD_f_A2_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=0 (condition EQ (equal))
#[test]
fn test_aarch32_vadd_f_a2_a_combo_0_800_0e300800() {
    // Encoding: 0x0E300800
    // Test aarch32_VADD_f_A2_A field combination: cond=0, D=0, Vn=0, Vd=0, size=0, N=0, M=0, Vm=0
    // ISET: A32
    // Fields: D=0, N=0, M=0, Vm=0, cond=0, size=0, Vn=0, Vd=0
    let encoding: u32 = 0x0E300800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VADD_f_A2_A
/// ASL: `field cond = 0 (Condition EQ)`
/// Requirement: FieldSpecial { field: "cond", value: 0, meaning: "Condition EQ" }
/// Condition EQ
#[test]
fn test_aarch32_vadd_f_a2_a_special_cond_0_condition_eq_2048_0e300800() {
    // Encoding: 0x0E300800
    // Test aarch32_VADD_f_A2_A special value cond = 0 (Condition EQ)
    // ISET: A32
    // Fields: D=0, cond=0, M=0, N=0, Vn=0, size=0, Vd=0, Vm=0
    let encoding: u32 = 0x0E300800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VADD_f_A2_A
/// ASL: `field cond = 1 (Condition NE)`
/// Requirement: FieldSpecial { field: "cond", value: 1, meaning: "Condition NE" }
/// Condition NE
#[test]
fn test_aarch32_vadd_f_a2_a_special_cond_1_condition_ne_2048_1e300800() {
    // Encoding: 0x1E300800
    // Test aarch32_VADD_f_A2_A special value cond = 1 (Condition NE)
    // ISET: A32
    // Fields: D=0, Vm=0, size=0, M=0, N=0, cond=1, Vn=0, Vd=0
    let encoding: u32 = 0x1E300800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VADD_f_A2_A
/// ASL: `field cond = 2 (Condition CS/HS)`
/// Requirement: FieldSpecial { field: "cond", value: 2, meaning: "Condition CS/HS" }
/// Condition CS/HS
#[test]
fn test_aarch32_vadd_f_a2_a_special_cond_2_condition_cs_hs_2048_2e300800() {
    // Encoding: 0x2E300800
    // Test aarch32_VADD_f_A2_A special value cond = 2 (Condition CS/HS)
    // ISET: A32
    // Fields: cond=2, Vn=0, N=0, Vd=0, Vm=0, size=0, D=0, M=0
    let encoding: u32 = 0x2E300800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VADD_f_A2_A
/// ASL: `field cond = 3 (Condition CC/LO)`
/// Requirement: FieldSpecial { field: "cond", value: 3, meaning: "Condition CC/LO" }
/// Condition CC/LO
#[test]
fn test_aarch32_vadd_f_a2_a_special_cond_3_condition_cc_lo_2048_3e300800() {
    // Encoding: 0x3E300800
    // Test aarch32_VADD_f_A2_A special value cond = 3 (Condition CC/LO)
    // ISET: A32
    // Fields: Vn=0, Vd=0, D=0, M=0, Vm=0, cond=3, size=0, N=0
    let encoding: u32 = 0x3E300800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VADD_f_A2_A
/// ASL: `field cond = 4 (Condition MI)`
/// Requirement: FieldSpecial { field: "cond", value: 4, meaning: "Condition MI" }
/// Condition MI
#[test]
fn test_aarch32_vadd_f_a2_a_special_cond_4_condition_mi_2048_4e300800() {
    // Encoding: 0x4E300800
    // Test aarch32_VADD_f_A2_A special value cond = 4 (Condition MI)
    // ISET: A32
    // Fields: M=0, Vd=0, size=0, cond=4, Vm=0, N=0, Vn=0, D=0
    let encoding: u32 = 0x4E300800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VADD_f_A2_A
/// ASL: `field cond = 5 (Condition PL)`
/// Requirement: FieldSpecial { field: "cond", value: 5, meaning: "Condition PL" }
/// Condition PL
#[test]
fn test_aarch32_vadd_f_a2_a_special_cond_5_condition_pl_2048_5e300800() {
    // Encoding: 0x5E300800
    // Test aarch32_VADD_f_A2_A special value cond = 5 (Condition PL)
    // ISET: A32
    // Fields: cond=5, Vn=0, size=0, Vd=0, D=0, N=0, Vm=0, M=0
    let encoding: u32 = 0x5E300800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VADD_f_A2_A
/// ASL: `field cond = 6 (Condition VS)`
/// Requirement: FieldSpecial { field: "cond", value: 6, meaning: "Condition VS" }
/// Condition VS
#[test]
fn test_aarch32_vadd_f_a2_a_special_cond_6_condition_vs_2048_6e300800() {
    // Encoding: 0x6E300800
    // Test aarch32_VADD_f_A2_A special value cond = 6 (Condition VS)
    // ISET: A32
    // Fields: N=0, size=0, Vm=0, Vd=0, D=0, M=0, Vn=0, cond=6
    let encoding: u32 = 0x6E300800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VADD_f_A2_A
/// ASL: `field cond = 7 (Condition VC)`
/// Requirement: FieldSpecial { field: "cond", value: 7, meaning: "Condition VC" }
/// Condition VC
#[test]
fn test_aarch32_vadd_f_a2_a_special_cond_7_condition_vc_2048_7e300800() {
    // Encoding: 0x7E300800
    // Test aarch32_VADD_f_A2_A special value cond = 7 (Condition VC)
    // ISET: A32
    // Fields: Vd=0, D=0, cond=7, N=0, Vn=0, size=0, M=0, Vm=0
    let encoding: u32 = 0x7E300800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VADD_f_A2_A
/// ASL: `field cond = 8 (Condition HI)`
/// Requirement: FieldSpecial { field: "cond", value: 8, meaning: "Condition HI" }
/// Condition HI
#[test]
fn test_aarch32_vadd_f_a2_a_special_cond_8_condition_hi_2048_8e300800() {
    // Encoding: 0x8E300800
    // Test aarch32_VADD_f_A2_A special value cond = 8 (Condition HI)
    // ISET: A32
    // Fields: Vn=0, cond=8, size=0, Vd=0, D=0, Vm=0, N=0, M=0
    let encoding: u32 = 0x8E300800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VADD_f_A2_A
/// ASL: `field cond = 9 (Condition LS)`
/// Requirement: FieldSpecial { field: "cond", value: 9, meaning: "Condition LS" }
/// Condition LS
#[test]
fn test_aarch32_vadd_f_a2_a_special_cond_9_condition_ls_2048_9e300800() {
    // Encoding: 0x9E300800
    // Test aarch32_VADD_f_A2_A special value cond = 9 (Condition LS)
    // ISET: A32
    // Fields: cond=9, Vm=0, D=0, size=0, Vd=0, Vn=0, N=0, M=0
    let encoding: u32 = 0x9E300800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VADD_f_A2_A
/// ASL: `field cond = 10 (Condition GE)`
/// Requirement: FieldSpecial { field: "cond", value: 10, meaning: "Condition GE" }
/// Condition GE
#[test]
fn test_aarch32_vadd_f_a2_a_special_cond_10_condition_ge_2048_ae300800() {
    // Encoding: 0xAE300800
    // Test aarch32_VADD_f_A2_A special value cond = 10 (Condition GE)
    // ISET: A32
    // Fields: D=0, Vd=0, size=0, Vn=0, M=0, cond=10, N=0, Vm=0
    let encoding: u32 = 0xAE300800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VADD_f_A2_A
/// ASL: `field cond = 11 (Condition LT)`
/// Requirement: FieldSpecial { field: "cond", value: 11, meaning: "Condition LT" }
/// Condition LT
#[test]
fn test_aarch32_vadd_f_a2_a_special_cond_11_condition_lt_2048_be300800() {
    // Encoding: 0xBE300800
    // Test aarch32_VADD_f_A2_A special value cond = 11 (Condition LT)
    // ISET: A32
    // Fields: cond=11, Vn=0, Vd=0, size=0, Vm=0, M=0, N=0, D=0
    let encoding: u32 = 0xBE300800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VADD_f_A2_A
/// ASL: `field cond = 12 (Condition GT)`
/// Requirement: FieldSpecial { field: "cond", value: 12, meaning: "Condition GT" }
/// Condition GT
#[test]
fn test_aarch32_vadd_f_a2_a_special_cond_12_condition_gt_2048_ce300800() {
    // Encoding: 0xCE300800
    // Test aarch32_VADD_f_A2_A special value cond = 12 (Condition GT)
    // ISET: A32
    // Fields: Vn=0, Vm=0, cond=12, size=0, Vd=0, D=0, N=0, M=0
    let encoding: u32 = 0xCE300800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VADD_f_A2_A
/// ASL: `field cond = 13 (Condition LE)`
/// Requirement: FieldSpecial { field: "cond", value: 13, meaning: "Condition LE" }
/// Condition LE
#[test]
fn test_aarch32_vadd_f_a2_a_special_cond_13_condition_le_2048_de300800() {
    // Encoding: 0xDE300800
    // Test aarch32_VADD_f_A2_A special value cond = 13 (Condition LE)
    // ISET: A32
    // Fields: cond=13, N=0, M=0, D=0, Vn=0, Vd=0, size=0, Vm=0
    let encoding: u32 = 0xDE300800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VADD_f_A2_A
/// ASL: `field cond = 14 (Condition AL)`
/// Requirement: FieldSpecial { field: "cond", value: 14, meaning: "Condition AL" }
/// Condition AL
#[test]
fn test_aarch32_vadd_f_a2_a_special_cond_14_condition_al_2048_ee300800() {
    // Encoding: 0xEE300800
    // Test aarch32_VADD_f_A2_A special value cond = 14 (Condition AL)
    // ISET: A32
    // Fields: size=0, D=0, Vm=0, Vd=0, cond=14, M=0, N=0, Vn=0
    let encoding: u32 = 0xEE300800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VADD_f_A2_A
/// ASL: `field cond = 15 (Condition NV)`
/// Requirement: FieldSpecial { field: "cond", value: 15, meaning: "Condition NV" }
/// Condition NV
#[test]
fn test_aarch32_vadd_f_a2_a_special_cond_15_condition_nv_2048_fe300800() {
    // Encoding: 0xFE300800
    // Test aarch32_VADD_f_A2_A special value cond = 15 (Condition NV)
    // ISET: A32
    // Fields: Vm=0, Vn=0, N=0, D=0, cond=15, M=0, size=0, Vd=0
    let encoding: u32 = 0xFE300800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VADD_f_A2_A
/// ASL: `field size = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "size", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch32_vadd_f_a2_a_special_size_0_size_variant_0_2048_0e300800() {
    // Encoding: 0x0E300800
    // Test aarch32_VADD_f_A2_A special value size = 0 (Size variant 0)
    // ISET: A32
    // Fields: Vm=0, D=0, Vd=0, N=0, cond=0, Vn=0, size=0, M=0
    let encoding: u32 = 0x0E300800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VADD_f_A2_A
/// ASL: `field size = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "size", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch32_vadd_f_a2_a_special_size_1_size_variant_1_2048_0e300900() {
    // Encoding: 0x0E300900
    // Test aarch32_VADD_f_A2_A special value size = 1 (Size variant 1)
    // ISET: A32
    // Fields: Vm=0, Vd=0, size=1, M=0, N=0, cond=0, Vn=0, D=0
    let encoding: u32 = 0x0E300900;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VADD_f_A2_A
/// ASL: `field size = 2 (Size variant 2)`
/// Requirement: FieldSpecial { field: "size", value: 2, meaning: "Size variant 2" }
/// Size variant 2
#[test]
fn test_aarch32_vadd_f_a2_a_special_size_2_size_variant_2_2048_0e300a00() {
    // Encoding: 0x0E300A00
    // Test aarch32_VADD_f_A2_A special value size = 2 (Size variant 2)
    // ISET: A32
    // Fields: Vn=0, Vm=0, cond=0, M=0, N=0, size=2, Vd=0, D=0
    let encoding: u32 = 0x0E300A00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VADD_f_A2_A
/// ASL: `field size = 3 (Size variant 3)`
/// Requirement: FieldSpecial { field: "size", value: 3, meaning: "Size variant 3" }
/// Size variant 3
#[test]
fn test_aarch32_vadd_f_a2_a_special_size_3_size_variant_3_2048_0e300b00() {
    // Encoding: 0x0E300B00
    // Test aarch32_VADD_f_A2_A special value size = 3 (Size variant 3)
    // ISET: A32
    // Fields: size=3, cond=0, Vd=0, N=0, Vm=0, D=0, Vn=0, M=0
    let encoding: u32 = 0x0E300B00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VADD_f_A2_A
/// ASL: `Binary { op: Ne, lhs: Binary { op: Ne, lhs: Member { base: Var(QualifiedIdentifier { qualifier: Any, name: "FPSCR" }), field: "Len" }, rhs: Binary { op: Or, lhs: LitBits([false, false, false]), rhs: Member { base: Var(QualifiedIdentifier { qualifier: Any, name: "FPSCR" }), field: "Stride" } } }, rhs: LitBits([false, false]) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Ne, lhs: Binary { op: Ne, lhs: Member { base: Var(QualifiedIdentifier { qualifier: Any, name: \"FPSCR\" }), field: \"Len\" }, rhs: Binary { op: Or, lhs: LitBits([false, false, false]), rhs: Member { base: Var(QualifiedIdentifier { qualifier: Any, name: \"FPSCR\" }), field: \"Stride\" } } }, rhs: LitBits([false, false]) }" }
/// triggers Undefined
#[test]
fn test_aarch32_vadd_f_a2_a_invalid_0_800_0e300800() {
    // Encoding: 0x0E300800
    // Test aarch32_VADD_f_A2_A invalid encoding: Binary { op: Ne, lhs: Binary { op: Ne, lhs: Member { base: Var(QualifiedIdentifier { qualifier: Any, name: "FPSCR" }), field: "Len" }, rhs: Binary { op: Or, lhs: LitBits([false, false, false]), rhs: Member { base: Var(QualifiedIdentifier { qualifier: Any, name: "FPSCR" }), field: "Stride" } } }, rhs: LitBits([false, false]) }
    // ISET: A32
    // Fields: Vn=0, M=0, Vd=0, size=0, D=0, N=0, cond=0, Vm=0
    let encoding: u32 = 0x0E300800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || !matches!(exit.unwrap(), CpuExit::Continue),
        "expected UNDEFINED for A32 encoding 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch32_VADD_f_A2_A
/// ASL: `Unconditional UNDEFINED`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNDEFINED" }
/// triggers Undefined
#[test]
fn test_aarch32_vadd_f_a2_a_invalid_1_800_0e300800() {
    // Encoding: 0x0E300800
    // Test aarch32_VADD_f_A2_A invalid encoding: Unconditional UNDEFINED
    // ISET: A32
    // Fields: M=0, size=0, N=0, Vd=0, Vn=0, D=0, Vm=0, cond=0
    let encoding: u32 = 0x0E300800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || !matches!(exit.unwrap(), CpuExit::Continue),
        "expected UNDEFINED for A32 encoding 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch32_VADD_f_A2_A
/// ASL: `Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "size" }), rhs: Binary { op: Or, lhs: LitBits([false, false]), rhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "size" }), rhs: Binary { op: And, lhs: LitBits([false, true]), rhs: Unary { op: Not, operand: Call { name: QualifiedIdentifier { qualifier: Any, name: "HaveFP16Ext" }, args: [] } } } } } }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"size\" }), rhs: Binary { op: Or, lhs: LitBits([false, false]), rhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"size\" }), rhs: Binary { op: And, lhs: LitBits([false, true]), rhs: Unary { op: Not, operand: Call { name: QualifiedIdentifier { qualifier: Any, name: \"HaveFP16Ext\" }, args: [] } } } } } }" }
/// triggers Undefined
#[test]
fn test_aarch32_vadd_f_a2_a_invalid_2_800_0e300800() {
    // Encoding: 0x0E300800
    // Test aarch32_VADD_f_A2_A invalid encoding: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "size" }), rhs: Binary { op: Or, lhs: LitBits([false, false]), rhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "size" }), rhs: Binary { op: And, lhs: LitBits([false, true]), rhs: Unary { op: Not, operand: Call { name: QualifiedIdentifier { qualifier: Any, name: "HaveFP16Ext" }, args: [] } } } } } }
    // ISET: A32
    // Fields: D=0, Vm=0, Vd=0, Vn=0, N=0, size=0, cond=0, M=0
    let encoding: u32 = 0x0E300800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || !matches!(exit.unwrap(), CpuExit::Continue),
        "expected UNDEFINED for A32 encoding 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch32_VADD_f_A2_A
/// ASL: `Unconditional UNDEFINED`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNDEFINED" }
/// triggers Undefined
#[test]
fn test_aarch32_vadd_f_a2_a_invalid_3_800_0e300800() {
    // Encoding: 0x0E300800
    // Test aarch32_VADD_f_A2_A invalid encoding: Unconditional UNDEFINED
    // ISET: A32
    // Fields: Vm=0, Vn=0, D=0, M=0, Vd=0, size=0, N=0, cond=0
    let encoding: u32 = 0x0E300800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || !matches!(exit.unwrap(), CpuExit::Continue),
        "expected UNDEFINED for A32 encoding 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch32_VADD_f_A2_A
/// ASL: `Binary { op: Ne, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "size" }), rhs: Binary { op: And, lhs: LitBits([false, true]), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "cond" }) } }, rhs: LitBits([true, true, true, false]) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Ne, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"size\" }), rhs: Binary { op: And, lhs: LitBits([false, true]), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"cond\" }) } }, rhs: LitBits([true, true, true, false]) }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_vadd_f_a2_a_invalid_4_800_0e300800() {
    // Encoding: 0x0E300800
    // Test aarch32_VADD_f_A2_A invalid encoding: Binary { op: Ne, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "size" }), rhs: Binary { op: And, lhs: LitBits([false, true]), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "cond" }) } }, rhs: LitBits([true, true, true, false]) }
    // ISET: A32
    // Fields: Vn=0, size=0, N=0, D=0, Vm=0, Vd=0, cond=0, M=0
    let encoding: u32 = 0x0E300800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // UNPREDICTABLE - behavior is implementation-defined
    let _ = cpu.step();
}

/// Provenance: aarch32_VADD_f_A2_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_vadd_f_a2_a_invalid_5_800_0e300800() {
    // Encoding: 0x0E300800
    // Test aarch32_VADD_f_A2_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: A32
    // Fields: D=0, cond=0, Vn=0, Vm=0, size=0, M=0, Vd=0, N=0
    let encoding: u32 = 0x0E300800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // UNPREDICTABLE - behavior is implementation-defined
    let _ = cpu.step();
}

/// Provenance: aarch32_VADD_f_T1_A
/// ASL: `field D 22 +: 1`
/// Requirement: FieldBoundary { field: "D", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_vadd_f_t1_a_field_d_0_min_d00_ef000d00() {
    // Thumb encoding (32): 0xEF000D00
    // Test aarch32_VADD_f_T1_A field D = 0 (Min)
    // ISET: T32
    // Fields: D=0, M=0, sz=0, Vd=0, Vn=0, Vm=0, N=0, Q=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF000D00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VADD_f_T1_A
/// ASL: `field D 22 +: 1`
/// Requirement: FieldBoundary { field: "D", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_vadd_f_t1_a_field_d_1_max_d00_ef400d00() {
    // Thumb encoding (32): 0xEF400D00
    // Test aarch32_VADD_f_T1_A field D = 1 (Max)
    // ISET: T32
    // Fields: Vn=0, N=0, Vm=0, Vd=0, Q=0, M=0, D=1, sz=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF400D00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VADD_f_T1_A
/// ASL: `field sz 20 +: 1`
/// Requirement: FieldBoundary { field: "sz", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch32_vadd_f_t1_a_field_sz_0_min_d00_ef000d00() {
    // Thumb encoding (32): 0xEF000D00
    // Test aarch32_VADD_f_T1_A field sz = 0 (Min)
    // ISET: T32
    // Fields: Vd=0, N=0, Q=0, sz=0, M=0, Vn=0, D=0, Vm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF000D00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VADD_f_T1_A
/// ASL: `field sz 20 +: 1`
/// Requirement: FieldBoundary { field: "sz", value: 1, boundary: Max }
/// 16-bit / halfword size
#[test]
fn test_aarch32_vadd_f_t1_a_field_sz_1_max_d00_ef100d00() {
    // Thumb encoding (32): 0xEF100D00
    // Test aarch32_VADD_f_T1_A field sz = 1 (Max)
    // ISET: T32
    // Fields: N=0, M=0, Vd=0, Vm=0, D=0, sz=1, Vn=0, Q=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF100D00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VADD_f_T1_A
/// ASL: `field Vn 16 +: 4`
/// Requirement: FieldBoundary { field: "Vn", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_aarch32_vadd_f_t1_a_field_vn_0_min_d00_ef000d00() {
    // Thumb encoding (32): 0xEF000D00
    // Test aarch32_VADD_f_T1_A field Vn = 0 (Min)
    // ISET: T32
    // Fields: Vn=0, M=0, Vm=0, D=0, sz=0, Q=0, Vd=0, N=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF000D00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VADD_f_T1_A
/// ASL: `field Vn 16 +: 4`
/// Requirement: FieldBoundary { field: "Vn", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_aarch32_vadd_f_t1_a_field_vn_1_poweroftwo_d00_ef010d00() {
    // Thumb encoding (32): 0xEF010D00
    // Test aarch32_VADD_f_T1_A field Vn = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: M=0, Vm=0, D=0, N=0, Q=0, sz=0, Vd=0, Vn=1
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF010D00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VADD_f_T1_A
/// ASL: `field Vd 12 +: 4`
/// Requirement: FieldBoundary { field: "Vd", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_aarch32_vadd_f_t1_a_field_vd_0_min_d00_ef000d00() {
    // Thumb encoding (32): 0xEF000D00
    // Test aarch32_VADD_f_T1_A field Vd = 0 (Min)
    // ISET: T32
    // Fields: D=0, N=0, Vm=0, Q=0, M=0, sz=0, Vd=0, Vn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF000D00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VADD_f_T1_A
/// ASL: `field Vd 12 +: 4`
/// Requirement: FieldBoundary { field: "Vd", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_aarch32_vadd_f_t1_a_field_vd_1_poweroftwo_d00_ef001d00() {
    // Thumb encoding (32): 0xEF001D00
    // Test aarch32_VADD_f_T1_A field Vd = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Q=0, M=0, N=0, sz=0, Vn=0, Vm=0, Vd=1, D=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF001D00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VADD_f_T1_A
/// ASL: `field N 7 +: 1`
/// Requirement: FieldBoundary { field: "N", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_vadd_f_t1_a_field_n_0_min_d00_ef000d00() {
    // Thumb encoding (32): 0xEF000D00
    // Test aarch32_VADD_f_T1_A field N = 0 (Min)
    // ISET: T32
    // Fields: sz=0, Vn=0, D=0, N=0, M=0, Q=0, Vm=0, Vd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF000D00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VADD_f_T1_A
/// ASL: `field N 7 +: 1`
/// Requirement: FieldBoundary { field: "N", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_vadd_f_t1_a_field_n_1_max_d00_ef000d80() {
    // Thumb encoding (32): 0xEF000D80
    // Test aarch32_VADD_f_T1_A field N = 1 (Max)
    // ISET: T32
    // Fields: Vd=0, M=0, Vm=0, Vn=0, D=0, sz=0, Q=0, N=1
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF000D80;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VADD_f_T1_A
/// ASL: `field Q 6 +: 1`
/// Requirement: FieldBoundary { field: "Q", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch32_vadd_f_t1_a_field_q_0_min_d00_ef000d00() {
    // Thumb encoding (32): 0xEF000D00
    // Test aarch32_VADD_f_T1_A field Q = 0 (Min)
    // ISET: T32
    // Fields: D=0, Vd=0, N=0, Q=0, Vn=0, sz=0, M=0, Vm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF000D00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VADD_f_T1_A
/// ASL: `field Q 6 +: 1`
/// Requirement: FieldBoundary { field: "Q", value: 1, boundary: Max }
/// 16-bit / halfword size
#[test]
fn test_aarch32_vadd_f_t1_a_field_q_1_max_d00_ef000d40() {
    // Thumb encoding (32): 0xEF000D40
    // Test aarch32_VADD_f_T1_A field Q = 1 (Max)
    // ISET: T32
    // Fields: M=0, Q=1, Vn=0, N=0, Vd=0, Vm=0, sz=0, D=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF000D40;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VADD_f_T1_A
/// ASL: `field M 5 +: 1`
/// Requirement: FieldBoundary { field: "M", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_vadd_f_t1_a_field_m_0_min_d00_ef000d00() {
    // Thumb encoding (32): 0xEF000D00
    // Test aarch32_VADD_f_T1_A field M = 0 (Min)
    // ISET: T32
    // Fields: M=0, Vd=0, D=0, N=0, Q=0, sz=0, Vn=0, Vm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF000D00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VADD_f_T1_A
/// ASL: `field M 5 +: 1`
/// Requirement: FieldBoundary { field: "M", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_vadd_f_t1_a_field_m_1_max_d00_ef000d20() {
    // Thumb encoding (32): 0xEF000D20
    // Test aarch32_VADD_f_T1_A field M = 1 (Max)
    // ISET: T32
    // Fields: Vd=0, Vn=0, D=0, Q=0, sz=0, N=0, Vm=0, M=1
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF000D20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VADD_f_T1_A
/// ASL: `field Vm 0 +: 4`
/// Requirement: FieldBoundary { field: "Vm", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_aarch32_vadd_f_t1_a_field_vm_0_min_d00_ef000d00() {
    // Thumb encoding (32): 0xEF000D00
    // Test aarch32_VADD_f_T1_A field Vm = 0 (Min)
    // ISET: T32
    // Fields: Q=0, M=0, N=0, sz=0, Vd=0, Vn=0, Vm=0, D=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF000D00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VADD_f_T1_A
/// ASL: `field Vm 0 +: 4`
/// Requirement: FieldBoundary { field: "Vm", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_aarch32_vadd_f_t1_a_field_vm_1_poweroftwo_d00_ef000d01() {
    // Thumb encoding (32): 0xEF000D01
    // Test aarch32_VADD_f_T1_A field Vm = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: D=0, Vn=0, Vm=1, Q=0, Vd=0, M=0, sz=0, N=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF000D01;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VADD_f_T1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// D=0 (minimum value)
#[test]
fn test_aarch32_vadd_f_t1_a_combo_0_d00_ef000d00() {
    // Thumb encoding (32): 0xEF000D00
    // Test aarch32_VADD_f_T1_A field combination: D=0, sz=0, Vn=0, Vd=0, N=0, Q=0, M=0, Vm=0
    // ISET: T32
    // Fields: Vm=0, D=0, Vn=0, Q=0, M=0, sz=0, Vd=0, N=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF000D00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VADD_f_T1_A
/// ASL: `field sz = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "sz", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch32_vadd_f_t1_a_special_sz_0_size_variant_0_3328_ef000d00() {
    // Thumb encoding (32): 0xEF000D00
    // Test aarch32_VADD_f_T1_A special value sz = 0 (Size variant 0)
    // ISET: T32
    // Fields: N=0, Vn=0, D=0, M=0, sz=0, Vd=0, Q=0, Vm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF000D00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VADD_f_T1_A
/// ASL: `field sz = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "sz", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch32_vadd_f_t1_a_special_sz_1_size_variant_1_3328_ef100d00() {
    // Thumb encoding (32): 0xEF100D00
    // Test aarch32_VADD_f_T1_A special value sz = 1 (Size variant 1)
    // ISET: T32
    // Fields: Vd=0, Q=0, D=0, sz=1, M=0, Vn=0, Vm=0, N=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF100D00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VADD_f_T1_A
/// ASL: `field Q = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "Q", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch32_vadd_f_t1_a_special_q_0_size_variant_0_3328_ef000d00() {
    // Thumb encoding (32): 0xEF000D00
    // Test aarch32_VADD_f_T1_A special value Q = 0 (Size variant 0)
    // ISET: T32
    // Fields: Vd=0, N=0, Q=0, Vn=0, D=0, sz=0, Vm=0, M=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF000D00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VADD_f_T1_A
/// ASL: `field Q = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "Q", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch32_vadd_f_t1_a_special_q_1_size_variant_1_3328_ef000d40() {
    // Thumb encoding (32): 0xEF000D40
    // Test aarch32_VADD_f_T1_A special value Q = 1 (Size variant 1)
    // ISET: T32
    // Fields: M=0, N=0, Vm=0, D=0, Q=1, sz=0, Vd=0, Vn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF000D40;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VADD_f_T1_A
/// ASL: `Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "Q" }), rhs: Binary { op: And, lhs: LitBits([true]), rhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: "Vd" }), indices: [Single(LitInt(0))] }, rhs: Binary { op: Or, lhs: LitBits([true]), rhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: "Vn" }), indices: [Single(LitInt(0))] } } }, rhs: Binary { op: Or, lhs: LitBits([true]), rhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: "Vm" }), indices: [Single(LitInt(0))] } } }, rhs: LitBits([true]) } } }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"Q\" }), rhs: Binary { op: And, lhs: LitBits([true]), rhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: \"Vd\" }), indices: [Single(LitInt(0))] }, rhs: Binary { op: Or, lhs: LitBits([true]), rhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: \"Vn\" }), indices: [Single(LitInt(0))] } } }, rhs: Binary { op: Or, lhs: LitBits([true]), rhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: \"Vm\" }), indices: [Single(LitInt(0))] } } }, rhs: LitBits([true]) } } }" }
/// triggers Undefined
#[test]
fn test_aarch32_vadd_f_t1_a_invalid_0_d00_ef000d00() {
    // Thumb encoding (32): 0xEF000D00
    // Test aarch32_VADD_f_T1_A invalid encoding: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "Q" }), rhs: Binary { op: And, lhs: LitBits([true]), rhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: "Vd" }), indices: [Single(LitInt(0))] }, rhs: Binary { op: Or, lhs: LitBits([true]), rhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: "Vn" }), indices: [Single(LitInt(0))] } } }, rhs: Binary { op: Or, lhs: LitBits([true]), rhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: "Vm" }), indices: [Single(LitInt(0))] } } }, rhs: LitBits([true]) } } }
    // ISET: T32
    // Fields: M=0, Vn=0, Vd=0, N=0, D=0, Q=0, sz=0, Vm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF000D00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || !matches!(exit.unwrap(), CpuExit::Continue),
        "expected UNDEFINED for T32 encoding"
    );
}

/// Provenance: aarch32_VADD_f_T1_A
/// ASL: `Unconditional UNDEFINED`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNDEFINED" }
/// triggers Undefined
#[test]
fn test_aarch32_vadd_f_t1_a_invalid_1_d00_ef000d00() {
    // Thumb encoding (32): 0xEF000D00
    // Test aarch32_VADD_f_T1_A invalid encoding: Unconditional UNDEFINED
    // ISET: T32
    // Fields: Vm=0, sz=0, D=0, M=0, Vd=0, Q=0, N=0, Vn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF000D00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || !matches!(exit.unwrap(), CpuExit::Continue),
        "expected UNDEFINED for T32 encoding"
    );
}

/// Provenance: aarch32_VADD_f_T1_A
/// ASL: `Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "sz" }), rhs: Binary { op: And, lhs: LitBits([true]), rhs: Unary { op: Not, operand: Call { name: QualifiedIdentifier { qualifier: Any, name: "HaveFP16Ext" }, args: [] } } } }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"sz\" }), rhs: Binary { op: And, lhs: LitBits([true]), rhs: Unary { op: Not, operand: Call { name: QualifiedIdentifier { qualifier: Any, name: \"HaveFP16Ext\" }, args: [] } } } }" }
/// triggers Undefined
#[test]
fn test_aarch32_vadd_f_t1_a_invalid_2_d00_ef000d00() {
    // Thumb encoding (32): 0xEF000D00
    // Test aarch32_VADD_f_T1_A invalid encoding: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "sz" }), rhs: Binary { op: And, lhs: LitBits([true]), rhs: Unary { op: Not, operand: Call { name: QualifiedIdentifier { qualifier: Any, name: "HaveFP16Ext" }, args: [] } } } }
    // ISET: T32
    // Fields: D=0, Vn=0, N=0, sz=0, Q=0, M=0, Vd=0, Vm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF000D00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || !matches!(exit.unwrap(), CpuExit::Continue),
        "expected UNDEFINED for T32 encoding"
    );
}

/// Provenance: aarch32_VADD_f_T1_A
/// ASL: `Unconditional UNDEFINED`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNDEFINED" }
/// triggers Undefined
#[test]
fn test_aarch32_vadd_f_t1_a_invalid_3_d00_ef000d00() {
    // Thumb encoding (32): 0xEF000D00
    // Test aarch32_VADD_f_T1_A invalid encoding: Unconditional UNDEFINED
    // ISET: T32
    // Fields: Vd=0, D=0, M=0, Vm=0, sz=0, Vn=0, N=0, Q=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF000D00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || !matches!(exit.unwrap(), CpuExit::Continue),
        "expected UNDEFINED for T32 encoding"
    );
}

/// Provenance: aarch32_VADD_f_T1_A
/// ASL: `Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "sz" }), rhs: Binary { op: And, lhs: LitBits([true]), rhs: Call { name: QualifiedIdentifier { qualifier: Any, name: "InITBlock" }, args: [] } } }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"sz\" }), rhs: Binary { op: And, lhs: LitBits([true]), rhs: Call { name: QualifiedIdentifier { qualifier: Any, name: \"InITBlock\" }, args: [] } } }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_vadd_f_t1_a_invalid_4_d00_ef000d00() {
    // Thumb encoding (32): 0xEF000D00
    // Test aarch32_VADD_f_T1_A invalid encoding: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "sz" }), rhs: Binary { op: And, lhs: LitBits([true]), rhs: Call { name: QualifiedIdentifier { qualifier: Any, name: "InITBlock" }, args: [] } } }
    // ISET: T32
    // Fields: Vd=0, Vn=0, Q=0, Vm=0, sz=0, M=0, N=0, D=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF000D00;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

/// Provenance: aarch32_VADD_f_T1_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_vadd_f_t1_a_invalid_5_d00_ef000d00() {
    // Thumb encoding (32): 0xEF000D00
    // Test aarch32_VADD_f_T1_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: T32
    // Fields: D=0, Vn=0, sz=0, Vd=0, Vm=0, Q=0, M=0, N=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF000D00;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

/// Provenance: aarch32_VADD_f_T2_A
/// ASL: `field D 22 +: 1`
/// Requirement: FieldBoundary { field: "D", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_vadd_f_t2_a_field_d_0_min_800_ee300800() {
    // Thumb encoding (32): 0xEE300800
    // Test aarch32_VADD_f_T2_A field D = 0 (Min)
    // ISET: T32
    // Fields: Vd=0, Vm=0, Vn=0, N=0, size=0, M=0, D=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEE300800;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VADD_f_T2_A
/// ASL: `field D 22 +: 1`
/// Requirement: FieldBoundary { field: "D", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_vadd_f_t2_a_field_d_1_max_800_ee700800() {
    // Thumb encoding (32): 0xEE700800
    // Test aarch32_VADD_f_T2_A field D = 1 (Max)
    // ISET: T32
    // Fields: D=1, size=0, Vm=0, Vd=0, N=0, Vn=0, M=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEE700800;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VADD_f_T2_A
/// ASL: `field Vn 16 +: 4`
/// Requirement: FieldBoundary { field: "Vn", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_aarch32_vadd_f_t2_a_field_vn_0_min_800_ee300800() {
    // Thumb encoding (32): 0xEE300800
    // Test aarch32_VADD_f_T2_A field Vn = 0 (Min)
    // ISET: T32
    // Fields: D=0, size=0, Vm=0, Vn=0, N=0, M=0, Vd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEE300800;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VADD_f_T2_A
/// ASL: `field Vn 16 +: 4`
/// Requirement: FieldBoundary { field: "Vn", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_aarch32_vadd_f_t2_a_field_vn_1_poweroftwo_800_ee310800() {
    // Thumb encoding (32): 0xEE310800
    // Test aarch32_VADD_f_T2_A field Vn = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Vm=0, size=0, N=0, Vn=1, D=0, M=0, Vd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEE310800;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VADD_f_T2_A
/// ASL: `field Vd 12 +: 4`
/// Requirement: FieldBoundary { field: "Vd", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_aarch32_vadd_f_t2_a_field_vd_0_min_800_ee300800() {
    // Thumb encoding (32): 0xEE300800
    // Test aarch32_VADD_f_T2_A field Vd = 0 (Min)
    // ISET: T32
    // Fields: D=0, N=0, Vm=0, M=0, size=0, Vn=0, Vd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEE300800;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VADD_f_T2_A
/// ASL: `field Vd 12 +: 4`
/// Requirement: FieldBoundary { field: "Vd", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_aarch32_vadd_f_t2_a_field_vd_1_poweroftwo_800_ee301800() {
    // Thumb encoding (32): 0xEE301800
    // Test aarch32_VADD_f_T2_A field Vd = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Vn=0, Vd=1, size=0, D=0, M=0, Vm=0, N=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEE301800;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VADD_f_T2_A
/// ASL: `field size 8 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch32_vadd_f_t2_a_field_size_0_min_800_ee300800() {
    // Thumb encoding (32): 0xEE300800
    // Test aarch32_VADD_f_T2_A field size = 0 (Min)
    // ISET: T32
    // Fields: D=0, size=0, M=0, Vn=0, Vd=0, N=0, Vm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEE300800;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VADD_f_T2_A
/// ASL: `field size 8 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 1, boundary: PowerOfTwo }
/// 16-bit / halfword size
#[test]
fn test_aarch32_vadd_f_t2_a_field_size_1_poweroftwo_800_ee300900() {
    // Thumb encoding (32): 0xEE300900
    // Test aarch32_VADD_f_T2_A field size = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: N=0, Vn=0, D=0, M=0, Vm=0, Vd=0, size=1
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEE300900;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VADD_f_T2_A
/// ASL: `field size 8 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 2, boundary: PowerOfTwo }
/// 32-bit / word size
#[test]
fn test_aarch32_vadd_f_t2_a_field_size_2_poweroftwo_800_ee300a00() {
    // Thumb encoding (32): 0xEE300A00
    // Test aarch32_VADD_f_T2_A field size = 2 (PowerOfTwo)
    // ISET: T32
    // Fields: D=0, M=0, Vm=0, Vn=0, N=0, Vd=0, size=2
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEE300A00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VADD_f_T2_A
/// ASL: `field size 8 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 3, boundary: Max }
/// 64-bit / doubleword size
#[test]
fn test_aarch32_vadd_f_t2_a_field_size_3_max_800_ee300b00() {
    // Thumb encoding (32): 0xEE300B00
    // Test aarch32_VADD_f_T2_A field size = 3 (Max)
    // ISET: T32
    // Fields: size=3, N=0, Vm=0, M=0, Vd=0, Vn=0, D=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEE300B00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VADD_f_T2_A
/// ASL: `field N 7 +: 1`
/// Requirement: FieldBoundary { field: "N", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_vadd_f_t2_a_field_n_0_min_800_ee300800() {
    // Thumb encoding (32): 0xEE300800
    // Test aarch32_VADD_f_T2_A field N = 0 (Min)
    // ISET: T32
    // Fields: Vn=0, size=0, M=0, N=0, Vm=0, D=0, Vd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEE300800;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VADD_f_T2_A
/// ASL: `field N 7 +: 1`
/// Requirement: FieldBoundary { field: "N", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_vadd_f_t2_a_field_n_1_max_800_ee300880() {
    // Thumb encoding (32): 0xEE300880
    // Test aarch32_VADD_f_T2_A field N = 1 (Max)
    // ISET: T32
    // Fields: M=0, D=0, Vm=0, Vd=0, size=0, Vn=0, N=1
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEE300880;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VADD_f_T2_A
/// ASL: `field M 5 +: 1`
/// Requirement: FieldBoundary { field: "M", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_vadd_f_t2_a_field_m_0_min_800_ee300800() {
    // Thumb encoding (32): 0xEE300800
    // Test aarch32_VADD_f_T2_A field M = 0 (Min)
    // ISET: T32
    // Fields: D=0, Vm=0, Vd=0, size=0, N=0, Vn=0, M=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEE300800;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VADD_f_T2_A
/// ASL: `field M 5 +: 1`
/// Requirement: FieldBoundary { field: "M", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_vadd_f_t2_a_field_m_1_max_800_ee300820() {
    // Thumb encoding (32): 0xEE300820
    // Test aarch32_VADD_f_T2_A field M = 1 (Max)
    // ISET: T32
    // Fields: N=0, Vm=0, Vd=0, D=0, size=0, Vn=0, M=1
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEE300820;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VADD_f_T2_A
/// ASL: `field Vm 0 +: 4`
/// Requirement: FieldBoundary { field: "Vm", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_aarch32_vadd_f_t2_a_field_vm_0_min_800_ee300800() {
    // Thumb encoding (32): 0xEE300800
    // Test aarch32_VADD_f_T2_A field Vm = 0 (Min)
    // ISET: T32
    // Fields: M=0, D=0, size=0, Vm=0, Vd=0, Vn=0, N=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEE300800;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VADD_f_T2_A
/// ASL: `field Vm 0 +: 4`
/// Requirement: FieldBoundary { field: "Vm", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_aarch32_vadd_f_t2_a_field_vm_1_poweroftwo_800_ee300801() {
    // Thumb encoding (32): 0xEE300801
    // Test aarch32_VADD_f_T2_A field Vm = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: N=0, Vm=1, D=0, Vd=0, M=0, size=0, Vn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEE300801;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VADD_f_T2_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// D=0 (minimum value)
#[test]
fn test_aarch32_vadd_f_t2_a_combo_0_800_ee300800() {
    // Thumb encoding (32): 0xEE300800
    // Test aarch32_VADD_f_T2_A field combination: D=0, Vn=0, Vd=0, size=0, N=0, M=0, Vm=0
    // ISET: T32
    // Fields: D=0, Vd=0, size=0, Vn=0, N=0, M=0, Vm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEE300800;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VADD_f_T2_A
/// ASL: `field size = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "size", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch32_vadd_f_t2_a_special_size_0_size_variant_0_2048_ee300800() {
    // Thumb encoding (32): 0xEE300800
    // Test aarch32_VADD_f_T2_A special value size = 0 (Size variant 0)
    // ISET: T32
    // Fields: N=0, Vm=0, D=0, size=0, Vd=0, M=0, Vn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEE300800;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VADD_f_T2_A
/// ASL: `field size = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "size", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch32_vadd_f_t2_a_special_size_1_size_variant_1_2048_ee300900() {
    // Thumb encoding (32): 0xEE300900
    // Test aarch32_VADD_f_T2_A special value size = 1 (Size variant 1)
    // ISET: T32
    // Fields: Vn=0, N=0, M=0, Vm=0, Vd=0, D=0, size=1
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEE300900;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VADD_f_T2_A
/// ASL: `field size = 2 (Size variant 2)`
/// Requirement: FieldSpecial { field: "size", value: 2, meaning: "Size variant 2" }
/// Size variant 2
#[test]
fn test_aarch32_vadd_f_t2_a_special_size_2_size_variant_2_2048_ee300a00() {
    // Thumb encoding (32): 0xEE300A00
    // Test aarch32_VADD_f_T2_A special value size = 2 (Size variant 2)
    // ISET: T32
    // Fields: Vd=0, N=0, M=0, Vm=0, D=0, Vn=0, size=2
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEE300A00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VADD_f_T2_A
/// ASL: `field size = 3 (Size variant 3)`
/// Requirement: FieldSpecial { field: "size", value: 3, meaning: "Size variant 3" }
/// Size variant 3
#[test]
fn test_aarch32_vadd_f_t2_a_special_size_3_size_variant_3_2048_ee300b00() {
    // Thumb encoding (32): 0xEE300B00
    // Test aarch32_VADD_f_T2_A special value size = 3 (Size variant 3)
    // ISET: T32
    // Fields: D=0, M=0, size=3, Vn=0, Vm=0, Vd=0, N=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEE300B00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VADD_f_T2_A
/// ASL: `Binary { op: Ne, lhs: Binary { op: Ne, lhs: Member { base: Var(QualifiedIdentifier { qualifier: Any, name: "FPSCR" }), field: "Len" }, rhs: Binary { op: Or, lhs: LitBits([false, false, false]), rhs: Member { base: Var(QualifiedIdentifier { qualifier: Any, name: "FPSCR" }), field: "Stride" } } }, rhs: LitBits([false, false]) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Ne, lhs: Binary { op: Ne, lhs: Member { base: Var(QualifiedIdentifier { qualifier: Any, name: \"FPSCR\" }), field: \"Len\" }, rhs: Binary { op: Or, lhs: LitBits([false, false, false]), rhs: Member { base: Var(QualifiedIdentifier { qualifier: Any, name: \"FPSCR\" }), field: \"Stride\" } } }, rhs: LitBits([false, false]) }" }
/// triggers Undefined
#[test]
fn test_aarch32_vadd_f_t2_a_invalid_0_800_ee300800() {
    // Thumb encoding (32): 0xEE300800
    // Test aarch32_VADD_f_T2_A invalid encoding: Binary { op: Ne, lhs: Binary { op: Ne, lhs: Member { base: Var(QualifiedIdentifier { qualifier: Any, name: "FPSCR" }), field: "Len" }, rhs: Binary { op: Or, lhs: LitBits([false, false, false]), rhs: Member { base: Var(QualifiedIdentifier { qualifier: Any, name: "FPSCR" }), field: "Stride" } } }, rhs: LitBits([false, false]) }
    // ISET: T32
    // Fields: D=0, Vn=0, N=0, M=0, Vm=0, size=0, Vd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEE300800;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || !matches!(exit.unwrap(), CpuExit::Continue),
        "expected UNDEFINED for T32 encoding"
    );
}

/// Provenance: aarch32_VADD_f_T2_A
/// ASL: `Unconditional UNDEFINED`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNDEFINED" }
/// triggers Undefined
#[test]
fn test_aarch32_vadd_f_t2_a_invalid_1_800_ee300800() {
    // Thumb encoding (32): 0xEE300800
    // Test aarch32_VADD_f_T2_A invalid encoding: Unconditional UNDEFINED
    // ISET: T32
    // Fields: D=0, size=0, M=0, N=0, Vm=0, Vn=0, Vd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEE300800;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || !matches!(exit.unwrap(), CpuExit::Continue),
        "expected UNDEFINED for T32 encoding"
    );
}

/// Provenance: aarch32_VADD_f_T2_A
/// ASL: `Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "size" }), rhs: Binary { op: Or, lhs: LitBits([false, false]), rhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "size" }), rhs: Binary { op: And, lhs: LitBits([false, true]), rhs: Unary { op: Not, operand: Call { name: QualifiedIdentifier { qualifier: Any, name: "HaveFP16Ext" }, args: [] } } } } } }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"size\" }), rhs: Binary { op: Or, lhs: LitBits([false, false]), rhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"size\" }), rhs: Binary { op: And, lhs: LitBits([false, true]), rhs: Unary { op: Not, operand: Call { name: QualifiedIdentifier { qualifier: Any, name: \"HaveFP16Ext\" }, args: [] } } } } } }" }
/// triggers Undefined
#[test]
fn test_aarch32_vadd_f_t2_a_invalid_2_800_ee300800() {
    // Thumb encoding (32): 0xEE300800
    // Test aarch32_VADD_f_T2_A invalid encoding: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "size" }), rhs: Binary { op: Or, lhs: LitBits([false, false]), rhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "size" }), rhs: Binary { op: And, lhs: LitBits([false, true]), rhs: Unary { op: Not, operand: Call { name: QualifiedIdentifier { qualifier: Any, name: "HaveFP16Ext" }, args: [] } } } } } }
    // ISET: T32
    // Fields: M=0, size=0, D=0, N=0, Vm=0, Vd=0, Vn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEE300800;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || !matches!(exit.unwrap(), CpuExit::Continue),
        "expected UNDEFINED for T32 encoding"
    );
}

/// Provenance: aarch32_VADD_f_T2_A
/// ASL: `Unconditional UNDEFINED`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNDEFINED" }
/// triggers Undefined
#[test]
fn test_aarch32_vadd_f_t2_a_invalid_3_800_ee300800() {
    // Thumb encoding (32): 0xEE300800
    // Test aarch32_VADD_f_T2_A invalid encoding: Unconditional UNDEFINED
    // ISET: T32
    // Fields: Vm=0, Vn=0, D=0, size=0, N=0, Vd=0, M=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEE300800;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || !matches!(exit.unwrap(), CpuExit::Continue),
        "expected UNDEFINED for T32 encoding"
    );
}

/// Provenance: aarch32_VADD_f_T2_A
/// ASL: `Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "size" }), rhs: Binary { op: And, lhs: LitBits([false, true]), rhs: Call { name: QualifiedIdentifier { qualifier: Any, name: "InITBlock" }, args: [] } } }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"size\" }), rhs: Binary { op: And, lhs: LitBits([false, true]), rhs: Call { name: QualifiedIdentifier { qualifier: Any, name: \"InITBlock\" }, args: [] } } }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_vadd_f_t2_a_invalid_4_800_ee300800() {
    // Thumb encoding (32): 0xEE300800
    // Test aarch32_VADD_f_T2_A invalid encoding: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "size" }), rhs: Binary { op: And, lhs: LitBits([false, true]), rhs: Call { name: QualifiedIdentifier { qualifier: Any, name: "InITBlock" }, args: [] } } }
    // ISET: T32
    // Fields: size=0, Vn=0, N=0, Vm=0, Vd=0, D=0, M=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEE300800;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

/// Provenance: aarch32_VADD_f_T2_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_vadd_f_t2_a_invalid_5_800_ee300800() {
    // Thumb encoding (32): 0xEE300800
    // Test aarch32_VADD_f_T2_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: T32
    // Fields: size=0, Vm=0, Vd=0, Vn=0, N=0, D=0, M=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEE300800;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

/// Provenance: aarch32_VADD_f_A1_A
/// ASL: `AND X0, X1, #0xFF`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// mask lower 8 bits (64)
#[test]
fn test_aarch32_vadd_f_a1_a_and_oracle_64_0_92401c20() {
    // Test AND 64-bit: mask lower 8 bits (oracle)
    // Encoding: 0x92401C20
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0x92401C20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0xFF, "X0 should be 0x00000000000000FF");
}

/// Provenance: aarch32_VADD_f_A1_A
/// ASL: `AND X0, X1, #0xFFFF`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// mask lower 16 bits (64)
#[test]
fn test_aarch32_vadd_f_a1_a_and_oracle_64_1_92403c20() {
    // Test AND 64-bit: mask lower 16 bits (oracle)
    // Encoding: 0x92403C20
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0x92403C20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0xFFFF, "X0 should be 0x000000000000FFFF");
}

/// Provenance: aarch32_VADD_f_A1_A
/// ASL: `AND X0, X1, #0xFFFFFFFF`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// mask lower 32 bits (64)
#[test]
fn test_aarch32_vadd_f_a1_a_and_oracle_64_2_92407c20() {
    // Test AND 64-bit: mask lower 32 bits (oracle)
    // Encoding: 0x92407C20
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0x92407C20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0x00000000FFFFFFFF"
    );
}

/// Provenance: aarch32_VADD_f_A1_A
/// ASL: `AND X0, X1, #0x1`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// single bit mask (64)
#[test]
fn test_aarch32_vadd_f_a1_a_and_oracle_64_3_92400020() {
    // Test AND 64-bit: single bit mask (oracle)
    // Encoding: 0x92400020
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xCAFEBABE);
    let encoding: u32 = 0x92400020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x0, "X0 should be 0x0000000000000000");
}

/// Provenance: aarch32_VADD_f_A1_A
/// ASL: `AND X0, X1, #0x7FFFFFFFFFFFFFFF`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// all but MSB (64)
#[test]
fn test_aarch32_vadd_f_a1_a_and_oracle_64_4_9240f820() {
    // Test AND 64-bit: all but MSB (oracle)
    // Encoding: 0x9240F820
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xAAAAAAAA);
    let encoding: u32 = 0x9240F820;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xAAAAAAAA,
        "X0 should be 0x2AAAAAAAAAAAAAAA"
    );
}

/// Provenance: aarch32_VADD_f_A1_A
/// ASL: `AND W0, W1, #0xFF`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// mask lower 8 bits (32)
#[test]
fn test_aarch32_vadd_f_a1_a_and_oracle_32_0_12001c20() {
    // Test AND 32-bit: mask lower 8 bits (oracle)
    // Encoding: 0x12001C20
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0x12001C20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0xFF, "W0 should be 0x000000FF");
}

/// Provenance: aarch32_VADD_f_A1_A
/// ASL: `AND W0, W1, #0xFFFF`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// mask lower 16 bits (32)
#[test]
fn test_aarch32_vadd_f_a1_a_and_oracle_32_1_12003c20() {
    // Test AND 32-bit: mask lower 16 bits (oracle)
    // Encoding: 0x12003C20
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0x12003C20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0xFFFF, "W0 should be 0x0000FFFF");
}

/// Provenance: aarch32_VADD_f_A1_A
/// ASL: `AND W0, W1, #0x1`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// single bit mask (32)
#[test]
fn test_aarch32_vadd_f_a1_a_and_oracle_32_2_12000020() {
    // Test AND 32-bit: single bit mask (oracle)
    // Encoding: 0x12000020
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xDEADBEEF);
    let encoding: u32 = 0x12000020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x1, "W0 should be 0x00000001");
}

/// Provenance: aarch32_VADD_f_A1_A
/// ASL: `AND R0, R1, #10`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// small immediate (Rn=0x00000000)
#[test]
fn test_aarch32_vadd_f_a1_a_a32_logical_imm_0_0_f201000a() {
    // Test A32 AND: small immediate (oracle)
    // Encoding: 0xF201000A
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF201000A;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x0, "R0 should be 0x00000000");
}

/// Provenance: aarch32_VADD_f_A1_A
/// ASL: `AND R0, R1, #10`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// small immediate (Rn=0x000000FF)
#[test]
fn test_aarch32_vadd_f_a1_a_a32_logical_imm_0_ff_f201000a() {
    // Test A32 AND: small immediate (oracle)
    // Encoding: 0xF201000A
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFF);
    let encoding: u32 = 0xF201000A;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0xA, "R0 should be 0x0000000A");
}

/// Provenance: aarch32_VADD_f_A1_A
/// ASL: `AND R0, R1, #10`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// small immediate (Rn=0xAAAAAAAA)
#[test]
fn test_aarch32_vadd_f_a1_a_a32_logical_imm_0_aaaaaaaa_f201000a() {
    // Test A32 AND: small immediate (oracle)
    // Encoding: 0xF201000A
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xAAAAAAAA);
    let encoding: u32 = 0xF201000A;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0xA, "R0 should be 0x0000000A");
}

/// Provenance: aarch32_VADD_f_A1_A
/// ASL: `AND R0, R1, #10`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// small immediate (Rn=0x55555555)
#[test]
fn test_aarch32_vadd_f_a1_a_a32_logical_imm_0_55555555_f201000a() {
    // Test A32 AND: small immediate (oracle)
    // Encoding: 0xF201000A
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x55555555);
    let encoding: u32 = 0xF201000A;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x0, "R0 should be 0x00000000");
}

/// Provenance: aarch32_VADD_f_A1_A
/// ASL: `AND R0, R1, #10`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// small immediate (Rn=0xFFFFFFFF)
#[test]
fn test_aarch32_vadd_f_a1_a_a32_logical_imm_0_ffffffff_f201000a() {
    // Test A32 AND: small immediate (oracle)
    // Encoding: 0xF201000A
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF201000A;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0xA, "R0 should be 0x0000000A");
}

/// Provenance: aarch32_VADD_f_A1_A
/// ASL: `AND R0, R1, #255`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// max imm8 (Rn=0x00000000)
#[test]
fn test_aarch32_vadd_f_a1_a_a32_logical_imm_1_0_f20100ff() {
    // Test A32 AND: max imm8 (oracle)
    // Encoding: 0xF20100FF
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF20100FF;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x0, "R0 should be 0x00000000");
}

/// Provenance: aarch32_VADD_f_A1_A
/// ASL: `AND R0, R1, #255`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// max imm8 (Rn=0x000000FF)
#[test]
fn test_aarch32_vadd_f_a1_a_a32_logical_imm_1_ff_f20100ff() {
    // Test A32 AND: max imm8 (oracle)
    // Encoding: 0xF20100FF
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFF);
    let encoding: u32 = 0xF20100FF;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0xFF, "R0 should be 0x000000FF");
}

/// Provenance: aarch32_VADD_f_A1_A
/// ASL: `AND R0, R1, #255`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// max imm8 (Rn=0xAAAAAAAA)
#[test]
fn test_aarch32_vadd_f_a1_a_a32_logical_imm_1_aaaaaaaa_f20100ff() {
    // Test A32 AND: max imm8 (oracle)
    // Encoding: 0xF20100FF
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xAAAAAAAA);
    let encoding: u32 = 0xF20100FF;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0xAA, "R0 should be 0x000000AA");
}

/// Provenance: aarch32_VADD_f_A1_A
/// ASL: `AND R0, R1, #255`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// max imm8 (Rn=0x55555555)
#[test]
fn test_aarch32_vadd_f_a1_a_a32_logical_imm_1_55555555_f20100ff() {
    // Test A32 AND: max imm8 (oracle)
    // Encoding: 0xF20100FF
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x55555555);
    let encoding: u32 = 0xF20100FF;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x55, "R0 should be 0x00000055");
}

/// Provenance: aarch32_VADD_f_A1_A
/// ASL: `AND R0, R1, #255`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// max imm8 (Rn=0xFFFFFFFF)
#[test]
fn test_aarch32_vadd_f_a1_a_a32_logical_imm_1_ffffffff_f20100ff() {
    // Test A32 AND: max imm8 (oracle)
    // Encoding: 0xF20100FF
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF20100FF;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0xFF, "R0 should be 0x000000FF");
}

/// Provenance: aarch32_VADD_f_A1_A
/// ASL: `AND R0, R1, #32`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// rotated by 2 (Rn=0x00000000)
#[test]
fn test_aarch32_vadd_f_a1_a_a32_logical_imm_2_0_f2010180() {
    // Test A32 AND: rotated by 2 (oracle)
    // Encoding: 0xF2010180
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2010180;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x0, "R0 should be 0x00000000");
}

/// Provenance: aarch32_VADD_f_A1_A
/// ASL: `AND R0, R1, #32`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// rotated by 2 (Rn=0x000000FF)
#[test]
fn test_aarch32_vadd_f_a1_a_a32_logical_imm_2_ff_f2010180() {
    // Test A32 AND: rotated by 2 (oracle)
    // Encoding: 0xF2010180
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFF);
    let encoding: u32 = 0xF2010180;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x20, "R0 should be 0x00000020");
}

/// Provenance: aarch32_VADD_f_A1_A
/// ASL: `AND R0, R1, #32`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// rotated by 2 (Rn=0xAAAAAAAA)
#[test]
fn test_aarch32_vadd_f_a1_a_a32_logical_imm_2_aaaaaaaa_f2010180() {
    // Test A32 AND: rotated by 2 (oracle)
    // Encoding: 0xF2010180
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xAAAAAAAA);
    let encoding: u32 = 0xF2010180;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x20, "R0 should be 0x00000020");
}

/// Provenance: aarch32_VADD_f_A1_A
/// ASL: `AND R0, R1, #32`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// rotated by 2 (Rn=0x55555555)
#[test]
fn test_aarch32_vadd_f_a1_a_a32_logical_imm_2_55555555_f2010180() {
    // Test A32 AND: rotated by 2 (oracle)
    // Encoding: 0xF2010180
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x55555555);
    let encoding: u32 = 0xF2010180;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x0, "R0 should be 0x00000000");
}

/// Provenance: aarch32_VADD_f_A1_A
/// ASL: `AND R0, R1, #32`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// rotated by 2 (Rn=0xFFFFFFFF)
#[test]
fn test_aarch32_vadd_f_a1_a_a32_logical_imm_2_ffffffff_f2010180() {
    // Test A32 AND: rotated by 2 (oracle)
    // Encoding: 0xF2010180
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2010180;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x20, "R0 should be 0x00000020");
}

/// Provenance: aarch32_VADD_f_A1_A
/// ASL: `AND R0, R1, #251658240`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// rotated by 8 (Rn=0x00000000)
#[test]
fn test_aarch32_vadd_f_a1_a_a32_logical_imm_3_0_f201040f() {
    // Test A32 AND: rotated by 8 (oracle)
    // Encoding: 0xF201040F
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF201040F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x0, "R0 should be 0x00000000");
}

/// Provenance: aarch32_VADD_f_A1_A
/// ASL: `AND R0, R1, #251658240`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// rotated by 8 (Rn=0x000000FF)
#[test]
fn test_aarch32_vadd_f_a1_a_a32_logical_imm_3_ff_f201040f() {
    // Test A32 AND: rotated by 8 (oracle)
    // Encoding: 0xF201040F
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFF);
    let encoding: u32 = 0xF201040F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x0, "R0 should be 0x00000000");
}

/// Provenance: aarch32_VADD_f_A1_A
/// ASL: `AND R0, R1, #251658240`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// rotated by 8 (Rn=0xAAAAAAAA)
#[test]
fn test_aarch32_vadd_f_a1_a_a32_logical_imm_3_aaaaaaaa_f201040f() {
    // Test A32 AND: rotated by 8 (oracle)
    // Encoding: 0xF201040F
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xAAAAAAAA);
    let encoding: u32 = 0xF201040F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0xA000000, "R0 should be 0x0A000000");
}

/// Provenance: aarch32_VADD_f_A1_A
/// ASL: `AND R0, R1, #251658240`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// rotated by 8 (Rn=0x55555555)
#[test]
fn test_aarch32_vadd_f_a1_a_a32_logical_imm_3_55555555_f201040f() {
    // Test A32 AND: rotated by 8 (oracle)
    // Encoding: 0xF201040F
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x55555555);
    let encoding: u32 = 0xF201040F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x5000000, "R0 should be 0x05000000");
}

/// Provenance: aarch32_VADD_f_A1_A
/// ASL: `AND R0, R1, #251658240`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// rotated by 8 (Rn=0xFFFFFFFF)
#[test]
fn test_aarch32_vadd_f_a1_a_a32_logical_imm_3_ffffffff_f201040f() {
    // Test A32 AND: rotated by 8 (oracle)
    // Encoding: 0xF201040F
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF201040F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0xF000000, "R0 should be 0x0F000000");
}

/// Provenance: aarch32_VADD_f_A1_A
/// ASL: `AND R0, R1, #0`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// zero immediate (Rn=0x00000000)
#[test]
fn test_aarch32_vadd_f_a1_a_a32_logical_imm_4_0_f2010000() {
    // Test A32 AND: zero immediate (oracle)
    // Encoding: 0xF2010000
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2010000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x0, "R0 should be 0x00000000");
}

/// Provenance: aarch32_VADD_f_A1_A
/// ASL: `AND R0, R1, #0`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// zero immediate (Rn=0x000000FF)
#[test]
fn test_aarch32_vadd_f_a1_a_a32_logical_imm_4_ff_f2010000() {
    // Test A32 AND: zero immediate (oracle)
    // Encoding: 0xF2010000
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFF);
    let encoding: u32 = 0xF2010000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x0, "R0 should be 0x00000000");
}

/// Provenance: aarch32_VADD_f_A1_A
/// ASL: `AND R0, R1, #0`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// zero immediate (Rn=0xAAAAAAAA)
#[test]
fn test_aarch32_vadd_f_a1_a_a32_logical_imm_4_aaaaaaaa_f2010000() {
    // Test A32 AND: zero immediate (oracle)
    // Encoding: 0xF2010000
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xAAAAAAAA);
    let encoding: u32 = 0xF2010000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x0, "R0 should be 0x00000000");
}

/// Provenance: aarch32_VADD_f_A1_A
/// ASL: `AND R0, R1, #0`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// zero immediate (Rn=0x55555555)
#[test]
fn test_aarch32_vadd_f_a1_a_a32_logical_imm_4_55555555_f2010000() {
    // Test A32 AND: zero immediate (oracle)
    // Encoding: 0xF2010000
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x55555555);
    let encoding: u32 = 0xF2010000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x0, "R0 should be 0x00000000");
}

/// Provenance: aarch32_VADD_f_A1_A
/// ASL: `AND R0, R1, #0`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// zero immediate (Rn=0xFFFFFFFF)
#[test]
fn test_aarch32_vadd_f_a1_a_a32_logical_imm_4_ffffffff_f2010000() {
    // Test A32 AND: zero immediate (oracle)
    // Encoding: 0xF2010000
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2010000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x0, "R0 should be 0x00000000");
}

// ============================================================================
// aarch32_VSUB_f_A Tests
// ============================================================================

/// Provenance: aarch32_VSUB_f_A1_A
/// ASL: `field D 22 +: 1`
/// Requirement: FieldBoundary { field: "D", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_vsub_f_a1_a_field_d_0_min_d00_f2200d00() {
    // Encoding: 0xF2200D00
    // Test aarch32_VSUB_f_A1_A field D = 0 (Min)
    // ISET: A32
    // Fields: D=0, Q=0, N=0, M=0, sz=0, Vm=0, Vn=0, Vd=0
    let encoding: u32 = 0xF2200D00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VSUB_f_A1_A
/// ASL: `field D 22 +: 1`
/// Requirement: FieldBoundary { field: "D", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_vsub_f_a1_a_field_d_1_max_d00_f2600d00() {
    // Encoding: 0xF2600D00
    // Test aarch32_VSUB_f_A1_A field D = 1 (Max)
    // ISET: A32
    // Fields: N=0, Vm=0, M=0, Vd=0, Q=0, Vn=0, sz=0, D=1
    let encoding: u32 = 0xF2600D00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VSUB_f_A1_A
/// ASL: `field sz 20 +: 1`
/// Requirement: FieldBoundary { field: "sz", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch32_vsub_f_a1_a_field_sz_0_min_d00_f2200d00() {
    // Encoding: 0xF2200D00
    // Test aarch32_VSUB_f_A1_A field sz = 0 (Min)
    // ISET: A32
    // Fields: Vn=0, sz=0, N=0, D=0, Q=0, Vm=0, M=0, Vd=0
    let encoding: u32 = 0xF2200D00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VSUB_f_A1_A
/// ASL: `field sz 20 +: 1`
/// Requirement: FieldBoundary { field: "sz", value: 1, boundary: Max }
/// 16-bit / halfword size
#[test]
fn test_aarch32_vsub_f_a1_a_field_sz_1_max_d00_f2300d00() {
    // Encoding: 0xF2300D00
    // Test aarch32_VSUB_f_A1_A field sz = 1 (Max)
    // ISET: A32
    // Fields: Vm=0, Q=0, sz=1, Vd=0, D=0, Vn=0, N=0, M=0
    let encoding: u32 = 0xF2300D00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VSUB_f_A1_A
/// ASL: `field Vn 16 +: 4`
/// Requirement: FieldBoundary { field: "Vn", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_aarch32_vsub_f_a1_a_field_vn_0_min_d00_f2200d00() {
    // Encoding: 0xF2200D00
    // Test aarch32_VSUB_f_A1_A field Vn = 0 (Min)
    // ISET: A32
    // Fields: N=0, D=0, Vd=0, sz=0, Q=0, M=0, Vm=0, Vn=0
    let encoding: u32 = 0xF2200D00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VSUB_f_A1_A
/// ASL: `field Vn 16 +: 4`
/// Requirement: FieldBoundary { field: "Vn", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_aarch32_vsub_f_a1_a_field_vn_1_poweroftwo_d00_f2210d00() {
    // Encoding: 0xF2210D00
    // Test aarch32_VSUB_f_A1_A field Vn = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: sz=0, Vm=0, N=0, D=0, Vd=0, Vn=1, Q=0, M=0
    let encoding: u32 = 0xF2210D00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VSUB_f_A1_A
/// ASL: `field Vd 12 +: 4`
/// Requirement: FieldBoundary { field: "Vd", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_aarch32_vsub_f_a1_a_field_vd_0_min_d00_f2200d00() {
    // Encoding: 0xF2200D00
    // Test aarch32_VSUB_f_A1_A field Vd = 0 (Min)
    // ISET: A32
    // Fields: Vn=0, Q=0, D=0, Vm=0, sz=0, Vd=0, M=0, N=0
    let encoding: u32 = 0xF2200D00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VSUB_f_A1_A
/// ASL: `field Vd 12 +: 4`
/// Requirement: FieldBoundary { field: "Vd", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_aarch32_vsub_f_a1_a_field_vd_1_poweroftwo_d00_f2201d00() {
    // Encoding: 0xF2201D00
    // Test aarch32_VSUB_f_A1_A field Vd = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: M=0, Q=0, N=0, sz=0, Vd=1, D=0, Vm=0, Vn=0
    let encoding: u32 = 0xF2201D00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VSUB_f_A1_A
/// ASL: `field N 7 +: 1`
/// Requirement: FieldBoundary { field: "N", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_vsub_f_a1_a_field_n_0_min_d00_f2200d00() {
    // Encoding: 0xF2200D00
    // Test aarch32_VSUB_f_A1_A field N = 0 (Min)
    // ISET: A32
    // Fields: Vm=0, Q=0, M=0, D=0, Vd=0, Vn=0, sz=0, N=0
    let encoding: u32 = 0xF2200D00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VSUB_f_A1_A
/// ASL: `field N 7 +: 1`
/// Requirement: FieldBoundary { field: "N", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_vsub_f_a1_a_field_n_1_max_d00_f2200d80() {
    // Encoding: 0xF2200D80
    // Test aarch32_VSUB_f_A1_A field N = 1 (Max)
    // ISET: A32
    // Fields: Vn=0, M=0, D=0, sz=0, Q=0, Vd=0, N=1, Vm=0
    let encoding: u32 = 0xF2200D80;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VSUB_f_A1_A
/// ASL: `field Q 6 +: 1`
/// Requirement: FieldBoundary { field: "Q", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch32_vsub_f_a1_a_field_q_0_min_d00_f2200d00() {
    // Encoding: 0xF2200D00
    // Test aarch32_VSUB_f_A1_A field Q = 0 (Min)
    // ISET: A32
    // Fields: D=0, N=0, M=0, Q=0, Vm=0, Vd=0, sz=0, Vn=0
    let encoding: u32 = 0xF2200D00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VSUB_f_A1_A
/// ASL: `field Q 6 +: 1`
/// Requirement: FieldBoundary { field: "Q", value: 1, boundary: Max }
/// 16-bit / halfword size
#[test]
fn test_aarch32_vsub_f_a1_a_field_q_1_max_d00_f2200d40() {
    // Encoding: 0xF2200D40
    // Test aarch32_VSUB_f_A1_A field Q = 1 (Max)
    // ISET: A32
    // Fields: sz=0, Vm=0, Vn=0, Q=1, M=0, Vd=0, N=0, D=0
    let encoding: u32 = 0xF2200D40;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VSUB_f_A1_A
/// ASL: `field M 5 +: 1`
/// Requirement: FieldBoundary { field: "M", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_vsub_f_a1_a_field_m_0_min_d00_f2200d00() {
    // Encoding: 0xF2200D00
    // Test aarch32_VSUB_f_A1_A field M = 0 (Min)
    // ISET: A32
    // Fields: M=0, Vn=0, sz=0, N=0, Vd=0, Vm=0, D=0, Q=0
    let encoding: u32 = 0xF2200D00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VSUB_f_A1_A
/// ASL: `field M 5 +: 1`
/// Requirement: FieldBoundary { field: "M", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_vsub_f_a1_a_field_m_1_max_d00_f2200d20() {
    // Encoding: 0xF2200D20
    // Test aarch32_VSUB_f_A1_A field M = 1 (Max)
    // ISET: A32
    // Fields: D=0, Vd=0, Q=0, Vn=0, N=0, Vm=0, M=1, sz=0
    let encoding: u32 = 0xF2200D20;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VSUB_f_A1_A
/// ASL: `field Vm 0 +: 4`
/// Requirement: FieldBoundary { field: "Vm", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_aarch32_vsub_f_a1_a_field_vm_0_min_d00_f2200d00() {
    // Encoding: 0xF2200D00
    // Test aarch32_VSUB_f_A1_A field Vm = 0 (Min)
    // ISET: A32
    // Fields: Vm=0, Q=0, Vd=0, sz=0, Vn=0, N=0, M=0, D=0
    let encoding: u32 = 0xF2200D00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VSUB_f_A1_A
/// ASL: `field Vm 0 +: 4`
/// Requirement: FieldBoundary { field: "Vm", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_aarch32_vsub_f_a1_a_field_vm_1_poweroftwo_d00_f2200d01() {
    // Encoding: 0xF2200D01
    // Test aarch32_VSUB_f_A1_A field Vm = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: M=0, D=0, N=0, Q=0, sz=0, Vd=0, Vm=1, Vn=0
    let encoding: u32 = 0xF2200D01;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VSUB_f_A1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// D=0 (minimum value)
#[test]
fn test_aarch32_vsub_f_a1_a_combo_0_d00_f2200d00() {
    // Encoding: 0xF2200D00
    // Test aarch32_VSUB_f_A1_A field combination: D=0, sz=0, Vn=0, Vd=0, N=0, Q=0, M=0, Vm=0
    // ISET: A32
    // Fields: N=0, D=0, sz=0, Vn=0, M=0, Vd=0, Q=0, Vm=0
    let encoding: u32 = 0xF2200D00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VSUB_f_A1_A
/// ASL: `field sz = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "sz", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch32_vsub_f_a1_a_special_sz_0_size_variant_0_3328_f2200d00() {
    // Encoding: 0xF2200D00
    // Test aarch32_VSUB_f_A1_A special value sz = 0 (Size variant 0)
    // ISET: A32
    // Fields: Vm=0, M=0, sz=0, Q=0, Vd=0, D=0, Vn=0, N=0
    let encoding: u32 = 0xF2200D00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VSUB_f_A1_A
/// ASL: `field sz = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "sz", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch32_vsub_f_a1_a_special_sz_1_size_variant_1_3328_f2300d00() {
    // Encoding: 0xF2300D00
    // Test aarch32_VSUB_f_A1_A special value sz = 1 (Size variant 1)
    // ISET: A32
    // Fields: Q=0, Vd=0, N=0, sz=1, Vn=0, M=0, D=0, Vm=0
    let encoding: u32 = 0xF2300D00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VSUB_f_A1_A
/// ASL: `field Q = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "Q", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch32_vsub_f_a1_a_special_q_0_size_variant_0_3328_f2200d00() {
    // Encoding: 0xF2200D00
    // Test aarch32_VSUB_f_A1_A special value Q = 0 (Size variant 0)
    // ISET: A32
    // Fields: N=0, Vn=0, M=0, D=0, sz=0, Vd=0, Q=0, Vm=0
    let encoding: u32 = 0xF2200D00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VSUB_f_A1_A
/// ASL: `field Q = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "Q", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch32_vsub_f_a1_a_special_q_1_size_variant_1_3328_f2200d40() {
    // Encoding: 0xF2200D40
    // Test aarch32_VSUB_f_A1_A special value Q = 1 (Size variant 1)
    // ISET: A32
    // Fields: D=0, Vd=0, N=0, sz=0, Q=1, M=0, Vm=0, Vn=0
    let encoding: u32 = 0xF2200D40;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VSUB_f_A1_A
/// ASL: `Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "Q" }), rhs: Binary { op: And, lhs: LitBits([true]), rhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: "Vd" }), indices: [Single(LitInt(0))] }, rhs: Binary { op: Or, lhs: LitBits([true]), rhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: "Vn" }), indices: [Single(LitInt(0))] } } }, rhs: Binary { op: Or, lhs: LitBits([true]), rhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: "Vm" }), indices: [Single(LitInt(0))] } } }, rhs: LitBits([true]) } } }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"Q\" }), rhs: Binary { op: And, lhs: LitBits([true]), rhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: \"Vd\" }), indices: [Single(LitInt(0))] }, rhs: Binary { op: Or, lhs: LitBits([true]), rhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: \"Vn\" }), indices: [Single(LitInt(0))] } } }, rhs: Binary { op: Or, lhs: LitBits([true]), rhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: \"Vm\" }), indices: [Single(LitInt(0))] } } }, rhs: LitBits([true]) } } }" }
/// triggers Undefined
#[test]
fn test_aarch32_vsub_f_a1_a_invalid_0_d00_f2200d00() {
    // Encoding: 0xF2200D00
    // Test aarch32_VSUB_f_A1_A invalid encoding: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "Q" }), rhs: Binary { op: And, lhs: LitBits([true]), rhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: "Vd" }), indices: [Single(LitInt(0))] }, rhs: Binary { op: Or, lhs: LitBits([true]), rhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: "Vn" }), indices: [Single(LitInt(0))] } } }, rhs: Binary { op: Or, lhs: LitBits([true]), rhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: "Vm" }), indices: [Single(LitInt(0))] } } }, rhs: LitBits([true]) } } }
    // ISET: A32
    // Fields: Vd=0, Vm=0, sz=0, M=0, Q=0, N=0, Vn=0, D=0
    let encoding: u32 = 0xF2200D00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || !matches!(exit.unwrap(), CpuExit::Continue),
        "expected UNDEFINED for A32 encoding 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch32_VSUB_f_A1_A
/// ASL: `Unconditional UNDEFINED`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNDEFINED" }
/// triggers Undefined
#[test]
fn test_aarch32_vsub_f_a1_a_invalid_1_d00_f2200d00() {
    // Encoding: 0xF2200D00
    // Test aarch32_VSUB_f_A1_A invalid encoding: Unconditional UNDEFINED
    // ISET: A32
    // Fields: D=0, Vm=0, Q=0, M=0, Vn=0, N=0, Vd=0, sz=0
    let encoding: u32 = 0xF2200D00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || !matches!(exit.unwrap(), CpuExit::Continue),
        "expected UNDEFINED for A32 encoding 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch32_VSUB_f_A1_A
/// ASL: `Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "sz" }), rhs: Binary { op: And, lhs: LitBits([true]), rhs: Unary { op: Not, operand: Call { name: QualifiedIdentifier { qualifier: Any, name: "HaveFP16Ext" }, args: [] } } } }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"sz\" }), rhs: Binary { op: And, lhs: LitBits([true]), rhs: Unary { op: Not, operand: Call { name: QualifiedIdentifier { qualifier: Any, name: \"HaveFP16Ext\" }, args: [] } } } }" }
/// triggers Undefined
#[test]
fn test_aarch32_vsub_f_a1_a_invalid_2_d00_f2200d00() {
    // Encoding: 0xF2200D00
    // Test aarch32_VSUB_f_A1_A invalid encoding: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "sz" }), rhs: Binary { op: And, lhs: LitBits([true]), rhs: Unary { op: Not, operand: Call { name: QualifiedIdentifier { qualifier: Any, name: "HaveFP16Ext" }, args: [] } } } }
    // ISET: A32
    // Fields: N=0, M=0, sz=0, Vn=0, Vm=0, Vd=0, D=0, Q=0
    let encoding: u32 = 0xF2200D00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || !matches!(exit.unwrap(), CpuExit::Continue),
        "expected UNDEFINED for A32 encoding 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch32_VSUB_f_A1_A
/// ASL: `Unconditional UNDEFINED`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNDEFINED" }
/// triggers Undefined
#[test]
fn test_aarch32_vsub_f_a1_a_invalid_3_d00_f2200d00() {
    // Encoding: 0xF2200D00
    // Test aarch32_VSUB_f_A1_A invalid encoding: Unconditional UNDEFINED
    // ISET: A32
    // Fields: D=0, sz=0, Vn=0, N=0, M=0, Vd=0, Vm=0, Q=0
    let encoding: u32 = 0xF2200D00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || !matches!(exit.unwrap(), CpuExit::Continue),
        "expected UNDEFINED for A32 encoding 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch32_VSUB_f_A2_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 0, boundary: Min }
/// condition EQ (equal)
#[test]
fn test_aarch32_vsub_f_a2_a_field_cond_0_min_840_0e300840() {
    // Encoding: 0x0E300840
    // Test aarch32_VSUB_f_A2_A field cond = 0 (Min)
    // ISET: A32
    // Fields: size=0, N=0, Vm=0, Vd=0, Vn=0, M=0, cond=0, D=0
    let encoding: u32 = 0x0E300840;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VSUB_f_A2_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 1, boundary: PowerOfTwo }
/// condition NE (not equal)
#[test]
fn test_aarch32_vsub_f_a2_a_field_cond_1_poweroftwo_840_1e300840() {
    // Encoding: 0x1E300840
    // Test aarch32_VSUB_f_A2_A field cond = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Vn=0, Vd=0, D=0, Vm=0, cond=1, size=0, N=0, M=0
    let encoding: u32 = 0x1E300840;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VSUB_f_A2_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 2, boundary: PowerOfTwo }
/// condition CS/HS (carry set)
#[test]
fn test_aarch32_vsub_f_a2_a_field_cond_2_poweroftwo_840_2e300840() {
    // Encoding: 0x2E300840
    // Test aarch32_VSUB_f_A2_A field cond = 2 (PowerOfTwo)
    // ISET: A32
    // Fields: M=0, cond=2, Vn=0, size=0, Vm=0, Vd=0, N=0, D=0
    let encoding: u32 = 0x2E300840;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VSUB_f_A2_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 3, boundary: PowerOfTwo }
/// condition CC/LO (carry clear)
#[test]
fn test_aarch32_vsub_f_a2_a_field_cond_3_poweroftwo_840_3e300840() {
    // Encoding: 0x3E300840
    // Test aarch32_VSUB_f_A2_A field cond = 3 (PowerOfTwo)
    // ISET: A32
    // Fields: D=0, Vd=0, size=0, N=0, Vn=0, cond=3, Vm=0, M=0
    let encoding: u32 = 0x3E300840;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VSUB_f_A2_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 4, boundary: PowerOfTwo }
/// condition MI (minus/negative)
#[test]
fn test_aarch32_vsub_f_a2_a_field_cond_4_poweroftwo_840_4e300840() {
    // Encoding: 0x4E300840
    // Test aarch32_VSUB_f_A2_A field cond = 4 (PowerOfTwo)
    // ISET: A32
    // Fields: Vn=0, cond=4, N=0, size=0, Vm=0, Vd=0, M=0, D=0
    let encoding: u32 = 0x4E300840;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VSUB_f_A2_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 5, boundary: PowerOfTwo }
/// condition PL (plus/positive)
#[test]
fn test_aarch32_vsub_f_a2_a_field_cond_5_poweroftwo_840_5e300840() {
    // Encoding: 0x5E300840
    // Test aarch32_VSUB_f_A2_A field cond = 5 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=5, N=0, size=0, Vn=0, Vm=0, D=0, Vd=0, M=0
    let encoding: u32 = 0x5E300840;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VSUB_f_A2_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 6, boundary: PowerOfTwo }
/// condition VS (overflow set)
#[test]
fn test_aarch32_vsub_f_a2_a_field_cond_6_poweroftwo_840_6e300840() {
    // Encoding: 0x6E300840
    // Test aarch32_VSUB_f_A2_A field cond = 6 (PowerOfTwo)
    // ISET: A32
    // Fields: Vd=0, size=0, cond=6, N=0, Vm=0, D=0, M=0, Vn=0
    let encoding: u32 = 0x6E300840;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VSUB_f_A2_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 7, boundary: PowerOfTwo }
/// condition VC (overflow clear)
#[test]
fn test_aarch32_vsub_f_a2_a_field_cond_7_poweroftwo_840_7e300840() {
    // Encoding: 0x7E300840
    // Test aarch32_VSUB_f_A2_A field cond = 7 (PowerOfTwo)
    // ISET: A32
    // Fields: size=0, cond=7, N=0, Vd=0, D=0, Vn=0, Vm=0, M=0
    let encoding: u32 = 0x7E300840;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VSUB_f_A2_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 8, boundary: PowerOfTwo }
/// condition HI (unsigned higher)
#[test]
fn test_aarch32_vsub_f_a2_a_field_cond_8_poweroftwo_840_8e300840() {
    // Encoding: 0x8E300840
    // Test aarch32_VSUB_f_A2_A field cond = 8 (PowerOfTwo)
    // ISET: A32
    // Fields: N=0, cond=8, D=0, M=0, Vm=0, size=0, Vn=0, Vd=0
    let encoding: u32 = 0x8E300840;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VSUB_f_A2_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 9, boundary: PowerOfTwo }
/// condition LS (unsigned lower or same)
#[test]
fn test_aarch32_vsub_f_a2_a_field_cond_9_poweroftwo_840_9e300840() {
    // Encoding: 0x9E300840
    // Test aarch32_VSUB_f_A2_A field cond = 9 (PowerOfTwo)
    // ISET: A32
    // Fields: M=0, Vd=0, size=0, N=0, cond=9, D=0, Vn=0, Vm=0
    let encoding: u32 = 0x9E300840;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VSUB_f_A2_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 10, boundary: PowerOfTwo }
/// condition GE (signed >=)
#[test]
fn test_aarch32_vsub_f_a2_a_field_cond_10_poweroftwo_840_ae300840() {
    // Encoding: 0xAE300840
    // Test aarch32_VSUB_f_A2_A field cond = 10 (PowerOfTwo)
    // ISET: A32
    // Fields: Vd=0, N=0, Vn=0, size=0, M=0, Vm=0, cond=10, D=0
    let encoding: u32 = 0xAE300840;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VSUB_f_A2_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 11, boundary: PowerOfTwo }
/// condition LT (signed <)
#[test]
fn test_aarch32_vsub_f_a2_a_field_cond_11_poweroftwo_840_be300840() {
    // Encoding: 0xBE300840
    // Test aarch32_VSUB_f_A2_A field cond = 11 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=11, D=0, Vd=0, Vn=0, Vm=0, N=0, M=0, size=0
    let encoding: u32 = 0xBE300840;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VSUB_f_A2_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 12, boundary: PowerOfTwo }
/// condition GT (signed >)
#[test]
fn test_aarch32_vsub_f_a2_a_field_cond_12_poweroftwo_840_ce300840() {
    // Encoding: 0xCE300840
    // Test aarch32_VSUB_f_A2_A field cond = 12 (PowerOfTwo)
    // ISET: A32
    // Fields: Vd=0, D=0, N=0, cond=12, Vm=0, size=0, M=0, Vn=0
    let encoding: u32 = 0xCE300840;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VSUB_f_A2_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 13, boundary: PowerOfTwo }
/// condition LE (signed <=)
#[test]
fn test_aarch32_vsub_f_a2_a_field_cond_13_poweroftwo_840_de300840() {
    // Encoding: 0xDE300840
    // Test aarch32_VSUB_f_A2_A field cond = 13 (PowerOfTwo)
    // ISET: A32
    // Fields: Vm=0, size=0, Vn=0, Vd=0, D=0, N=0, cond=13, M=0
    let encoding: u32 = 0xDE300840;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VSUB_f_A2_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 14, boundary: PowerOfTwo }
/// condition AL (always)
#[test]
fn test_aarch32_vsub_f_a2_a_field_cond_14_poweroftwo_840_ee300840() {
    // Encoding: 0xEE300840
    // Test aarch32_VSUB_f_A2_A field cond = 14 (PowerOfTwo)
    // ISET: A32
    // Fields: Vm=0, D=0, Vd=0, M=0, Vn=0, cond=14, N=0, size=0
    let encoding: u32 = 0xEE300840;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VSUB_f_A2_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 15, boundary: Max }
/// condition NV (never, reserved)
#[test]
fn test_aarch32_vsub_f_a2_a_field_cond_15_max_840_fe300840() {
    // Encoding: 0xFE300840
    // Test aarch32_VSUB_f_A2_A field cond = 15 (Max)
    // ISET: A32
    // Fields: size=0, Vd=0, Vn=0, Vm=0, M=0, cond=15, D=0, N=0
    let encoding: u32 = 0xFE300840;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VSUB_f_A2_A
/// ASL: `field D 22 +: 1`
/// Requirement: FieldBoundary { field: "D", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_vsub_f_a2_a_field_d_0_min_840_0e300840() {
    // Encoding: 0x0E300840
    // Test aarch32_VSUB_f_A2_A field D = 0 (Min)
    // ISET: A32
    // Fields: size=0, Vn=0, D=0, N=0, M=0, Vd=0, cond=0, Vm=0
    let encoding: u32 = 0x0E300840;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VSUB_f_A2_A
/// ASL: `field D 22 +: 1`
/// Requirement: FieldBoundary { field: "D", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_vsub_f_a2_a_field_d_1_max_840_0e700840() {
    // Encoding: 0x0E700840
    // Test aarch32_VSUB_f_A2_A field D = 1 (Max)
    // ISET: A32
    // Fields: Vm=0, size=0, cond=0, N=0, Vd=0, Vn=0, D=1, M=0
    let encoding: u32 = 0x0E700840;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VSUB_f_A2_A
/// ASL: `field Vn 16 +: 4`
/// Requirement: FieldBoundary { field: "Vn", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_aarch32_vsub_f_a2_a_field_vn_0_min_840_0e300840() {
    // Encoding: 0x0E300840
    // Test aarch32_VSUB_f_A2_A field Vn = 0 (Min)
    // ISET: A32
    // Fields: N=0, Vn=0, Vd=0, Vm=0, M=0, D=0, cond=0, size=0
    let encoding: u32 = 0x0E300840;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VSUB_f_A2_A
/// ASL: `field Vn 16 +: 4`
/// Requirement: FieldBoundary { field: "Vn", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_aarch32_vsub_f_a2_a_field_vn_1_poweroftwo_840_0e310840() {
    // Encoding: 0x0E310840
    // Test aarch32_VSUB_f_A2_A field Vn = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: size=0, Vm=0, Vd=0, D=0, M=0, N=0, Vn=1, cond=0
    let encoding: u32 = 0x0E310840;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VSUB_f_A2_A
/// ASL: `field Vd 12 +: 4`
/// Requirement: FieldBoundary { field: "Vd", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_aarch32_vsub_f_a2_a_field_vd_0_min_840_0e300840() {
    // Encoding: 0x0E300840
    // Test aarch32_VSUB_f_A2_A field Vd = 0 (Min)
    // ISET: A32
    // Fields: cond=0, Vm=0, D=0, size=0, Vd=0, M=0, Vn=0, N=0
    let encoding: u32 = 0x0E300840;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VSUB_f_A2_A
/// ASL: `field Vd 12 +: 4`
/// Requirement: FieldBoundary { field: "Vd", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_aarch32_vsub_f_a2_a_field_vd_1_poweroftwo_840_0e301840() {
    // Encoding: 0x0E301840
    // Test aarch32_VSUB_f_A2_A field Vd = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=0, N=0, Vn=0, Vd=1, M=0, size=0, Vm=0, D=0
    let encoding: u32 = 0x0E301840;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VSUB_f_A2_A
/// ASL: `field size 8 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch32_vsub_f_a2_a_field_size_0_min_840_0e300840() {
    // Encoding: 0x0E300840
    // Test aarch32_VSUB_f_A2_A field size = 0 (Min)
    // ISET: A32
    // Fields: Vm=0, size=0, M=0, Vd=0, N=0, D=0, cond=0, Vn=0
    let encoding: u32 = 0x0E300840;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VSUB_f_A2_A
/// ASL: `field size 8 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 1, boundary: PowerOfTwo }
/// 16-bit / halfword size
#[test]
fn test_aarch32_vsub_f_a2_a_field_size_1_poweroftwo_840_0e300940() {
    // Encoding: 0x0E300940
    // Test aarch32_VSUB_f_A2_A field size = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: N=0, cond=0, D=0, Vn=0, M=0, Vd=0, size=1, Vm=0
    let encoding: u32 = 0x0E300940;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VSUB_f_A2_A
/// ASL: `field size 8 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 2, boundary: PowerOfTwo }
/// 32-bit / word size
#[test]
fn test_aarch32_vsub_f_a2_a_field_size_2_poweroftwo_840_0e300a40() {
    // Encoding: 0x0E300A40
    // Test aarch32_VSUB_f_A2_A field size = 2 (PowerOfTwo)
    // ISET: A32
    // Fields: D=0, Vn=0, M=0, Vd=0, cond=0, size=2, Vm=0, N=0
    let encoding: u32 = 0x0E300A40;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VSUB_f_A2_A
/// ASL: `field size 8 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 3, boundary: Max }
/// 64-bit / doubleword size
#[test]
fn test_aarch32_vsub_f_a2_a_field_size_3_max_840_0e300b40() {
    // Encoding: 0x0E300B40
    // Test aarch32_VSUB_f_A2_A field size = 3 (Max)
    // ISET: A32
    // Fields: D=0, size=3, Vn=0, cond=0, N=0, Vd=0, Vm=0, M=0
    let encoding: u32 = 0x0E300B40;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VSUB_f_A2_A
/// ASL: `field N 7 +: 1`
/// Requirement: FieldBoundary { field: "N", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_vsub_f_a2_a_field_n_0_min_840_0e300840() {
    // Encoding: 0x0E300840
    // Test aarch32_VSUB_f_A2_A field N = 0 (Min)
    // ISET: A32
    // Fields: D=0, size=0, cond=0, M=0, Vn=0, Vd=0, N=0, Vm=0
    let encoding: u32 = 0x0E300840;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VSUB_f_A2_A
/// ASL: `field N 7 +: 1`
/// Requirement: FieldBoundary { field: "N", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_vsub_f_a2_a_field_n_1_max_840_0e3008c0() {
    // Encoding: 0x0E3008C0
    // Test aarch32_VSUB_f_A2_A field N = 1 (Max)
    // ISET: A32
    // Fields: cond=0, Vn=0, size=0, Vm=0, M=0, D=0, N=1, Vd=0
    let encoding: u32 = 0x0E3008C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VSUB_f_A2_A
/// ASL: `field M 5 +: 1`
/// Requirement: FieldBoundary { field: "M", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_vsub_f_a2_a_field_m_0_min_840_0e300840() {
    // Encoding: 0x0E300840
    // Test aarch32_VSUB_f_A2_A field M = 0 (Min)
    // ISET: A32
    // Fields: size=0, D=0, cond=0, M=0, Vn=0, Vd=0, N=0, Vm=0
    let encoding: u32 = 0x0E300840;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VSUB_f_A2_A
/// ASL: `field M 5 +: 1`
/// Requirement: FieldBoundary { field: "M", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_vsub_f_a2_a_field_m_1_max_840_0e300860() {
    // Encoding: 0x0E300860
    // Test aarch32_VSUB_f_A2_A field M = 1 (Max)
    // ISET: A32
    // Fields: cond=0, D=0, M=1, size=0, Vm=0, N=0, Vn=0, Vd=0
    let encoding: u32 = 0x0E300860;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VSUB_f_A2_A
/// ASL: `field Vm 0 +: 4`
/// Requirement: FieldBoundary { field: "Vm", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_aarch32_vsub_f_a2_a_field_vm_0_min_840_0e300840() {
    // Encoding: 0x0E300840
    // Test aarch32_VSUB_f_A2_A field Vm = 0 (Min)
    // ISET: A32
    // Fields: Vm=0, D=0, M=0, N=0, size=0, Vn=0, cond=0, Vd=0
    let encoding: u32 = 0x0E300840;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VSUB_f_A2_A
/// ASL: `field Vm 0 +: 4`
/// Requirement: FieldBoundary { field: "Vm", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_aarch32_vsub_f_a2_a_field_vm_1_poweroftwo_840_0e300841() {
    // Encoding: 0x0E300841
    // Test aarch32_VSUB_f_A2_A field Vm = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: size=0, Vn=0, D=0, cond=0, M=0, Vd=0, Vm=1, N=0
    let encoding: u32 = 0x0E300841;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VSUB_f_A2_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=0 (condition EQ (equal))
#[test]
fn test_aarch32_vsub_f_a2_a_combo_0_840_0e300840() {
    // Encoding: 0x0E300840
    // Test aarch32_VSUB_f_A2_A field combination: cond=0, D=0, Vn=0, Vd=0, size=0, N=0, M=0, Vm=0
    // ISET: A32
    // Fields: Vn=0, size=0, Vm=0, D=0, cond=0, N=0, M=0, Vd=0
    let encoding: u32 = 0x0E300840;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VSUB_f_A2_A
/// ASL: `field cond = 0 (Condition EQ)`
/// Requirement: FieldSpecial { field: "cond", value: 0, meaning: "Condition EQ" }
/// Condition EQ
#[test]
fn test_aarch32_vsub_f_a2_a_special_cond_0_condition_eq_2112_0e300840() {
    // Encoding: 0x0E300840
    // Test aarch32_VSUB_f_A2_A special value cond = 0 (Condition EQ)
    // ISET: A32
    // Fields: Vd=0, N=0, Vn=0, D=0, cond=0, size=0, M=0, Vm=0
    let encoding: u32 = 0x0E300840;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VSUB_f_A2_A
/// ASL: `field cond = 1 (Condition NE)`
/// Requirement: FieldSpecial { field: "cond", value: 1, meaning: "Condition NE" }
/// Condition NE
#[test]
fn test_aarch32_vsub_f_a2_a_special_cond_1_condition_ne_2112_1e300840() {
    // Encoding: 0x1E300840
    // Test aarch32_VSUB_f_A2_A special value cond = 1 (Condition NE)
    // ISET: A32
    // Fields: Vd=0, D=0, size=0, N=0, Vn=0, M=0, Vm=0, cond=1
    let encoding: u32 = 0x1E300840;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VSUB_f_A2_A
/// ASL: `field cond = 2 (Condition CS/HS)`
/// Requirement: FieldSpecial { field: "cond", value: 2, meaning: "Condition CS/HS" }
/// Condition CS/HS
#[test]
fn test_aarch32_vsub_f_a2_a_special_cond_2_condition_cs_hs_2112_2e300840() {
    // Encoding: 0x2E300840
    // Test aarch32_VSUB_f_A2_A special value cond = 2 (Condition CS/HS)
    // ISET: A32
    // Fields: Vn=0, size=0, M=0, Vd=0, N=0, cond=2, D=0, Vm=0
    let encoding: u32 = 0x2E300840;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VSUB_f_A2_A
/// ASL: `field cond = 3 (Condition CC/LO)`
/// Requirement: FieldSpecial { field: "cond", value: 3, meaning: "Condition CC/LO" }
/// Condition CC/LO
#[test]
fn test_aarch32_vsub_f_a2_a_special_cond_3_condition_cc_lo_2112_3e300840() {
    // Encoding: 0x3E300840
    // Test aarch32_VSUB_f_A2_A special value cond = 3 (Condition CC/LO)
    // ISET: A32
    // Fields: Vm=0, size=0, cond=3, D=0, Vd=0, M=0, Vn=0, N=0
    let encoding: u32 = 0x3E300840;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VSUB_f_A2_A
/// ASL: `field cond = 4 (Condition MI)`
/// Requirement: FieldSpecial { field: "cond", value: 4, meaning: "Condition MI" }
/// Condition MI
#[test]
fn test_aarch32_vsub_f_a2_a_special_cond_4_condition_mi_2112_4e300840() {
    // Encoding: 0x4E300840
    // Test aarch32_VSUB_f_A2_A special value cond = 4 (Condition MI)
    // ISET: A32
    // Fields: D=0, M=0, Vn=0, Vd=0, size=0, N=0, Vm=0, cond=4
    let encoding: u32 = 0x4E300840;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VSUB_f_A2_A
/// ASL: `field cond = 5 (Condition PL)`
/// Requirement: FieldSpecial { field: "cond", value: 5, meaning: "Condition PL" }
/// Condition PL
#[test]
fn test_aarch32_vsub_f_a2_a_special_cond_5_condition_pl_2112_5e300840() {
    // Encoding: 0x5E300840
    // Test aarch32_VSUB_f_A2_A special value cond = 5 (Condition PL)
    // ISET: A32
    // Fields: cond=5, D=0, Vd=0, Vm=0, Vn=0, size=0, N=0, M=0
    let encoding: u32 = 0x5E300840;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VSUB_f_A2_A
/// ASL: `field cond = 6 (Condition VS)`
/// Requirement: FieldSpecial { field: "cond", value: 6, meaning: "Condition VS" }
/// Condition VS
#[test]
fn test_aarch32_vsub_f_a2_a_special_cond_6_condition_vs_2112_6e300840() {
    // Encoding: 0x6E300840
    // Test aarch32_VSUB_f_A2_A special value cond = 6 (Condition VS)
    // ISET: A32
    // Fields: cond=6, Vd=0, Vm=0, size=0, D=0, M=0, Vn=0, N=0
    let encoding: u32 = 0x6E300840;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VSUB_f_A2_A
/// ASL: `field cond = 7 (Condition VC)`
/// Requirement: FieldSpecial { field: "cond", value: 7, meaning: "Condition VC" }
/// Condition VC
#[test]
fn test_aarch32_vsub_f_a2_a_special_cond_7_condition_vc_2112_7e300840() {
    // Encoding: 0x7E300840
    // Test aarch32_VSUB_f_A2_A special value cond = 7 (Condition VC)
    // ISET: A32
    // Fields: D=0, Vm=0, Vd=0, N=0, Vn=0, size=0, cond=7, M=0
    let encoding: u32 = 0x7E300840;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VSUB_f_A2_A
/// ASL: `field cond = 8 (Condition HI)`
/// Requirement: FieldSpecial { field: "cond", value: 8, meaning: "Condition HI" }
/// Condition HI
#[test]
fn test_aarch32_vsub_f_a2_a_special_cond_8_condition_hi_2112_8e300840() {
    // Encoding: 0x8E300840
    // Test aarch32_VSUB_f_A2_A special value cond = 8 (Condition HI)
    // ISET: A32
    // Fields: Vd=0, N=0, M=0, size=0, cond=8, D=0, Vm=0, Vn=0
    let encoding: u32 = 0x8E300840;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VSUB_f_A2_A
/// ASL: `field cond = 9 (Condition LS)`
/// Requirement: FieldSpecial { field: "cond", value: 9, meaning: "Condition LS" }
/// Condition LS
#[test]
fn test_aarch32_vsub_f_a2_a_special_cond_9_condition_ls_2112_9e300840() {
    // Encoding: 0x9E300840
    // Test aarch32_VSUB_f_A2_A special value cond = 9 (Condition LS)
    // ISET: A32
    // Fields: N=0, cond=9, Vd=0, D=0, M=0, Vm=0, size=0, Vn=0
    let encoding: u32 = 0x9E300840;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VSUB_f_A2_A
/// ASL: `field cond = 10 (Condition GE)`
/// Requirement: FieldSpecial { field: "cond", value: 10, meaning: "Condition GE" }
/// Condition GE
#[test]
fn test_aarch32_vsub_f_a2_a_special_cond_10_condition_ge_2112_ae300840() {
    // Encoding: 0xAE300840
    // Test aarch32_VSUB_f_A2_A special value cond = 10 (Condition GE)
    // ISET: A32
    // Fields: M=0, cond=10, N=0, Vm=0, Vd=0, D=0, size=0, Vn=0
    let encoding: u32 = 0xAE300840;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VSUB_f_A2_A
/// ASL: `field cond = 11 (Condition LT)`
/// Requirement: FieldSpecial { field: "cond", value: 11, meaning: "Condition LT" }
/// Condition LT
#[test]
fn test_aarch32_vsub_f_a2_a_special_cond_11_condition_lt_2112_be300840() {
    // Encoding: 0xBE300840
    // Test aarch32_VSUB_f_A2_A special value cond = 11 (Condition LT)
    // ISET: A32
    // Fields: N=0, size=0, cond=11, Vd=0, D=0, Vn=0, M=0, Vm=0
    let encoding: u32 = 0xBE300840;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VSUB_f_A2_A
/// ASL: `field cond = 12 (Condition GT)`
/// Requirement: FieldSpecial { field: "cond", value: 12, meaning: "Condition GT" }
/// Condition GT
#[test]
fn test_aarch32_vsub_f_a2_a_special_cond_12_condition_gt_2112_ce300840() {
    // Encoding: 0xCE300840
    // Test aarch32_VSUB_f_A2_A special value cond = 12 (Condition GT)
    // ISET: A32
    // Fields: size=0, Vn=0, M=0, N=0, cond=12, Vd=0, D=0, Vm=0
    let encoding: u32 = 0xCE300840;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VSUB_f_A2_A
/// ASL: `field cond = 13 (Condition LE)`
/// Requirement: FieldSpecial { field: "cond", value: 13, meaning: "Condition LE" }
/// Condition LE
#[test]
fn test_aarch32_vsub_f_a2_a_special_cond_13_condition_le_2112_de300840() {
    // Encoding: 0xDE300840
    // Test aarch32_VSUB_f_A2_A special value cond = 13 (Condition LE)
    // ISET: A32
    // Fields: cond=13, Vd=0, D=0, Vn=0, Vm=0, M=0, N=0, size=0
    let encoding: u32 = 0xDE300840;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VSUB_f_A2_A
/// ASL: `field cond = 14 (Condition AL)`
/// Requirement: FieldSpecial { field: "cond", value: 14, meaning: "Condition AL" }
/// Condition AL
#[test]
fn test_aarch32_vsub_f_a2_a_special_cond_14_condition_al_2112_ee300840() {
    // Encoding: 0xEE300840
    // Test aarch32_VSUB_f_A2_A special value cond = 14 (Condition AL)
    // ISET: A32
    // Fields: Vm=0, Vn=0, size=0, cond=14, M=0, D=0, N=0, Vd=0
    let encoding: u32 = 0xEE300840;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VSUB_f_A2_A
/// ASL: `field cond = 15 (Condition NV)`
/// Requirement: FieldSpecial { field: "cond", value: 15, meaning: "Condition NV" }
/// Condition NV
#[test]
fn test_aarch32_vsub_f_a2_a_special_cond_15_condition_nv_2112_fe300840() {
    // Encoding: 0xFE300840
    // Test aarch32_VSUB_f_A2_A special value cond = 15 (Condition NV)
    // ISET: A32
    // Fields: size=0, N=0, D=0, cond=15, Vm=0, Vn=0, M=0, Vd=0
    let encoding: u32 = 0xFE300840;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VSUB_f_A2_A
/// ASL: `field size = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "size", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch32_vsub_f_a2_a_special_size_0_size_variant_0_2112_0e300840() {
    // Encoding: 0x0E300840
    // Test aarch32_VSUB_f_A2_A special value size = 0 (Size variant 0)
    // ISET: A32
    // Fields: cond=0, size=0, N=0, Vn=0, Vm=0, Vd=0, D=0, M=0
    let encoding: u32 = 0x0E300840;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VSUB_f_A2_A
/// ASL: `field size = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "size", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch32_vsub_f_a2_a_special_size_1_size_variant_1_2112_0e300940() {
    // Encoding: 0x0E300940
    // Test aarch32_VSUB_f_A2_A special value size = 1 (Size variant 1)
    // ISET: A32
    // Fields: D=0, cond=0, Vd=0, Vn=0, N=0, Vm=0, M=0, size=1
    let encoding: u32 = 0x0E300940;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VSUB_f_A2_A
/// ASL: `field size = 2 (Size variant 2)`
/// Requirement: FieldSpecial { field: "size", value: 2, meaning: "Size variant 2" }
/// Size variant 2
#[test]
fn test_aarch32_vsub_f_a2_a_special_size_2_size_variant_2_2112_0e300a40() {
    // Encoding: 0x0E300A40
    // Test aarch32_VSUB_f_A2_A special value size = 2 (Size variant 2)
    // ISET: A32
    // Fields: D=0, Vn=0, Vd=0, N=0, size=2, Vm=0, M=0, cond=0
    let encoding: u32 = 0x0E300A40;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VSUB_f_A2_A
/// ASL: `field size = 3 (Size variant 3)`
/// Requirement: FieldSpecial { field: "size", value: 3, meaning: "Size variant 3" }
/// Size variant 3
#[test]
fn test_aarch32_vsub_f_a2_a_special_size_3_size_variant_3_2112_0e300b40() {
    // Encoding: 0x0E300B40
    // Test aarch32_VSUB_f_A2_A special value size = 3 (Size variant 3)
    // ISET: A32
    // Fields: cond=0, Vn=0, M=0, D=0, N=0, Vm=0, size=3, Vd=0
    let encoding: u32 = 0x0E300B40;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VSUB_f_A2_A
/// ASL: `Binary { op: Ne, lhs: Binary { op: Ne, lhs: Member { base: Var(QualifiedIdentifier { qualifier: Any, name: "FPSCR" }), field: "Len" }, rhs: Binary { op: Or, lhs: LitBits([false, false, false]), rhs: Member { base: Var(QualifiedIdentifier { qualifier: Any, name: "FPSCR" }), field: "Stride" } } }, rhs: LitBits([false, false]) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Ne, lhs: Binary { op: Ne, lhs: Member { base: Var(QualifiedIdentifier { qualifier: Any, name: \"FPSCR\" }), field: \"Len\" }, rhs: Binary { op: Or, lhs: LitBits([false, false, false]), rhs: Member { base: Var(QualifiedIdentifier { qualifier: Any, name: \"FPSCR\" }), field: \"Stride\" } } }, rhs: LitBits([false, false]) }" }
/// triggers Undefined
#[test]
fn test_aarch32_vsub_f_a2_a_invalid_0_840_0e300840() {
    // Encoding: 0x0E300840
    // Test aarch32_VSUB_f_A2_A invalid encoding: Binary { op: Ne, lhs: Binary { op: Ne, lhs: Member { base: Var(QualifiedIdentifier { qualifier: Any, name: "FPSCR" }), field: "Len" }, rhs: Binary { op: Or, lhs: LitBits([false, false, false]), rhs: Member { base: Var(QualifiedIdentifier { qualifier: Any, name: "FPSCR" }), field: "Stride" } } }, rhs: LitBits([false, false]) }
    // ISET: A32
    // Fields: Vd=0, Vn=0, size=0, N=0, D=0, M=0, cond=0, Vm=0
    let encoding: u32 = 0x0E300840;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || !matches!(exit.unwrap(), CpuExit::Continue),
        "expected UNDEFINED for A32 encoding 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch32_VSUB_f_A2_A
/// ASL: `Unconditional UNDEFINED`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNDEFINED" }
/// triggers Undefined
#[test]
fn test_aarch32_vsub_f_a2_a_invalid_1_840_0e300840() {
    // Encoding: 0x0E300840
    // Test aarch32_VSUB_f_A2_A invalid encoding: Unconditional UNDEFINED
    // ISET: A32
    // Fields: M=0, Vm=0, Vd=0, Vn=0, N=0, cond=0, size=0, D=0
    let encoding: u32 = 0x0E300840;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || !matches!(exit.unwrap(), CpuExit::Continue),
        "expected UNDEFINED for A32 encoding 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch32_VSUB_f_A2_A
/// ASL: `Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "size" }), rhs: Binary { op: Or, lhs: LitBits([false, false]), rhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "size" }), rhs: Binary { op: And, lhs: LitBits([false, true]), rhs: Unary { op: Not, operand: Call { name: QualifiedIdentifier { qualifier: Any, name: "HaveFP16Ext" }, args: [] } } } } } }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"size\" }), rhs: Binary { op: Or, lhs: LitBits([false, false]), rhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"size\" }), rhs: Binary { op: And, lhs: LitBits([false, true]), rhs: Unary { op: Not, operand: Call { name: QualifiedIdentifier { qualifier: Any, name: \"HaveFP16Ext\" }, args: [] } } } } } }" }
/// triggers Undefined
#[test]
fn test_aarch32_vsub_f_a2_a_invalid_2_840_0e300840() {
    // Encoding: 0x0E300840
    // Test aarch32_VSUB_f_A2_A invalid encoding: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "size" }), rhs: Binary { op: Or, lhs: LitBits([false, false]), rhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "size" }), rhs: Binary { op: And, lhs: LitBits([false, true]), rhs: Unary { op: Not, operand: Call { name: QualifiedIdentifier { qualifier: Any, name: "HaveFP16Ext" }, args: [] } } } } } }
    // ISET: A32
    // Fields: Vd=0, M=0, Vn=0, Vm=0, cond=0, N=0, D=0, size=0
    let encoding: u32 = 0x0E300840;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || !matches!(exit.unwrap(), CpuExit::Continue),
        "expected UNDEFINED for A32 encoding 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch32_VSUB_f_A2_A
/// ASL: `Unconditional UNDEFINED`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNDEFINED" }
/// triggers Undefined
#[test]
fn test_aarch32_vsub_f_a2_a_invalid_3_840_0e300840() {
    // Encoding: 0x0E300840
    // Test aarch32_VSUB_f_A2_A invalid encoding: Unconditional UNDEFINED
    // ISET: A32
    // Fields: D=0, size=0, N=0, cond=0, Vn=0, Vd=0, M=0, Vm=0
    let encoding: u32 = 0x0E300840;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || !matches!(exit.unwrap(), CpuExit::Continue),
        "expected UNDEFINED for A32 encoding 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch32_VSUB_f_A2_A
/// ASL: `Binary { op: Ne, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "size" }), rhs: Binary { op: And, lhs: LitBits([false, true]), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "cond" }) } }, rhs: LitBits([true, true, true, false]) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Ne, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"size\" }), rhs: Binary { op: And, lhs: LitBits([false, true]), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"cond\" }) } }, rhs: LitBits([true, true, true, false]) }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_vsub_f_a2_a_invalid_4_840_0e300840() {
    // Encoding: 0x0E300840
    // Test aarch32_VSUB_f_A2_A invalid encoding: Binary { op: Ne, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "size" }), rhs: Binary { op: And, lhs: LitBits([false, true]), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "cond" }) } }, rhs: LitBits([true, true, true, false]) }
    // ISET: A32
    // Fields: cond=0, Vm=0, M=0, D=0, N=0, Vd=0, size=0, Vn=0
    let encoding: u32 = 0x0E300840;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // UNPREDICTABLE - behavior is implementation-defined
    let _ = cpu.step();
}

/// Provenance: aarch32_VSUB_f_A2_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_vsub_f_a2_a_invalid_5_840_0e300840() {
    // Encoding: 0x0E300840
    // Test aarch32_VSUB_f_A2_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: A32
    // Fields: Vd=0, Vn=0, Vm=0, N=0, cond=0, size=0, D=0, M=0
    let encoding: u32 = 0x0E300840;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // UNPREDICTABLE - behavior is implementation-defined
    let _ = cpu.step();
}

/// Provenance: aarch32_VSUB_f_T1_A
/// ASL: `field D 22 +: 1`
/// Requirement: FieldBoundary { field: "D", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_vsub_f_t1_a_field_d_0_min_d00_ef200d00() {
    // Thumb encoding (32): 0xEF200D00
    // Test aarch32_VSUB_f_T1_A field D = 0 (Min)
    // ISET: T32
    // Fields: Vm=0, Q=0, D=0, Vd=0, sz=0, M=0, Vn=0, N=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF200D00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VSUB_f_T1_A
/// ASL: `field D 22 +: 1`
/// Requirement: FieldBoundary { field: "D", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_vsub_f_t1_a_field_d_1_max_d00_ef600d00() {
    // Thumb encoding (32): 0xEF600D00
    // Test aarch32_VSUB_f_T1_A field D = 1 (Max)
    // ISET: T32
    // Fields: Vn=0, Vd=0, Vm=0, sz=0, N=0, Q=0, M=0, D=1
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF600D00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VSUB_f_T1_A
/// ASL: `field sz 20 +: 1`
/// Requirement: FieldBoundary { field: "sz", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch32_vsub_f_t1_a_field_sz_0_min_d00_ef200d00() {
    // Thumb encoding (32): 0xEF200D00
    // Test aarch32_VSUB_f_T1_A field sz = 0 (Min)
    // ISET: T32
    // Fields: Vm=0, Q=0, D=0, Vn=0, N=0, sz=0, Vd=0, M=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF200D00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VSUB_f_T1_A
/// ASL: `field sz 20 +: 1`
/// Requirement: FieldBoundary { field: "sz", value: 1, boundary: Max }
/// 16-bit / halfword size
#[test]
fn test_aarch32_vsub_f_t1_a_field_sz_1_max_d00_ef300d00() {
    // Thumb encoding (32): 0xEF300D00
    // Test aarch32_VSUB_f_T1_A field sz = 1 (Max)
    // ISET: T32
    // Fields: M=0, Vn=0, Q=0, D=0, N=0, Vd=0, sz=1, Vm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF300D00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VSUB_f_T1_A
/// ASL: `field Vn 16 +: 4`
/// Requirement: FieldBoundary { field: "Vn", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_aarch32_vsub_f_t1_a_field_vn_0_min_d00_ef200d00() {
    // Thumb encoding (32): 0xEF200D00
    // Test aarch32_VSUB_f_T1_A field Vn = 0 (Min)
    // ISET: T32
    // Fields: Vn=0, Q=0, M=0, Vm=0, N=0, D=0, Vd=0, sz=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF200D00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VSUB_f_T1_A
/// ASL: `field Vn 16 +: 4`
/// Requirement: FieldBoundary { field: "Vn", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_aarch32_vsub_f_t1_a_field_vn_1_poweroftwo_d00_ef210d00() {
    // Thumb encoding (32): 0xEF210D00
    // Test aarch32_VSUB_f_T1_A field Vn = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Vm=0, M=0, Q=0, D=0, N=0, sz=0, Vd=0, Vn=1
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF210D00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VSUB_f_T1_A
/// ASL: `field Vd 12 +: 4`
/// Requirement: FieldBoundary { field: "Vd", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_aarch32_vsub_f_t1_a_field_vd_0_min_d00_ef200d00() {
    // Thumb encoding (32): 0xEF200D00
    // Test aarch32_VSUB_f_T1_A field Vd = 0 (Min)
    // ISET: T32
    // Fields: D=0, Vd=0, N=0, Q=0, sz=0, M=0, Vn=0, Vm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF200D00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VSUB_f_T1_A
/// ASL: `field Vd 12 +: 4`
/// Requirement: FieldBoundary { field: "Vd", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_aarch32_vsub_f_t1_a_field_vd_1_poweroftwo_d00_ef201d00() {
    // Thumb encoding (32): 0xEF201D00
    // Test aarch32_VSUB_f_T1_A field Vd = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Vn=0, M=0, Q=0, Vm=0, D=0, Vd=1, N=0, sz=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF201D00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VSUB_f_T1_A
/// ASL: `field N 7 +: 1`
/// Requirement: FieldBoundary { field: "N", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_vsub_f_t1_a_field_n_0_min_d00_ef200d00() {
    // Thumb encoding (32): 0xEF200D00
    // Test aarch32_VSUB_f_T1_A field N = 0 (Min)
    // ISET: T32
    // Fields: Vn=0, D=0, Q=0, Vm=0, M=0, sz=0, N=0, Vd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF200D00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VSUB_f_T1_A
/// ASL: `field N 7 +: 1`
/// Requirement: FieldBoundary { field: "N", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_vsub_f_t1_a_field_n_1_max_d00_ef200d80() {
    // Thumb encoding (32): 0xEF200D80
    // Test aarch32_VSUB_f_T1_A field N = 1 (Max)
    // ISET: T32
    // Fields: D=0, M=0, N=1, sz=0, Vn=0, Vm=0, Q=0, Vd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF200D80;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VSUB_f_T1_A
/// ASL: `field Q 6 +: 1`
/// Requirement: FieldBoundary { field: "Q", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch32_vsub_f_t1_a_field_q_0_min_d00_ef200d00() {
    // Thumb encoding (32): 0xEF200D00
    // Test aarch32_VSUB_f_T1_A field Q = 0 (Min)
    // ISET: T32
    // Fields: Vm=0, Q=0, Vd=0, sz=0, Vn=0, M=0, N=0, D=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF200D00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VSUB_f_T1_A
/// ASL: `field Q 6 +: 1`
/// Requirement: FieldBoundary { field: "Q", value: 1, boundary: Max }
/// 16-bit / halfword size
#[test]
fn test_aarch32_vsub_f_t1_a_field_q_1_max_d00_ef200d40() {
    // Thumb encoding (32): 0xEF200D40
    // Test aarch32_VSUB_f_T1_A field Q = 1 (Max)
    // ISET: T32
    // Fields: D=0, N=0, Q=1, sz=0, Vd=0, M=0, Vn=0, Vm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF200D40;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VSUB_f_T1_A
/// ASL: `field M 5 +: 1`
/// Requirement: FieldBoundary { field: "M", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_vsub_f_t1_a_field_m_0_min_d00_ef200d00() {
    // Thumb encoding (32): 0xEF200D00
    // Test aarch32_VSUB_f_T1_A field M = 0 (Min)
    // ISET: T32
    // Fields: M=0, N=0, Vm=0, Q=0, sz=0, Vn=0, D=0, Vd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF200D00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VSUB_f_T1_A
/// ASL: `field M 5 +: 1`
/// Requirement: FieldBoundary { field: "M", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_vsub_f_t1_a_field_m_1_max_d00_ef200d20() {
    // Thumb encoding (32): 0xEF200D20
    // Test aarch32_VSUB_f_T1_A field M = 1 (Max)
    // ISET: T32
    // Fields: Vn=0, N=0, Vd=0, Q=0, D=0, sz=0, M=1, Vm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF200D20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VSUB_f_T1_A
/// ASL: `field Vm 0 +: 4`
/// Requirement: FieldBoundary { field: "Vm", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_aarch32_vsub_f_t1_a_field_vm_0_min_d00_ef200d00() {
    // Thumb encoding (32): 0xEF200D00
    // Test aarch32_VSUB_f_T1_A field Vm = 0 (Min)
    // ISET: T32
    // Fields: Vn=0, Vd=0, D=0, M=0, N=0, Vm=0, Q=0, sz=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF200D00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VSUB_f_T1_A
/// ASL: `field Vm 0 +: 4`
/// Requirement: FieldBoundary { field: "Vm", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_aarch32_vsub_f_t1_a_field_vm_1_poweroftwo_d00_ef200d01() {
    // Thumb encoding (32): 0xEF200D01
    // Test aarch32_VSUB_f_T1_A field Vm = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Vm=1, D=0, N=0, Vn=0, sz=0, M=0, Vd=0, Q=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF200D01;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VSUB_f_T1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// D=0 (minimum value)
#[test]
fn test_aarch32_vsub_f_t1_a_combo_0_d00_ef200d00() {
    // Thumb encoding (32): 0xEF200D00
    // Test aarch32_VSUB_f_T1_A field combination: D=0, sz=0, Vn=0, Vd=0, N=0, Q=0, M=0, Vm=0
    // ISET: T32
    // Fields: N=0, D=0, Vd=0, M=0, sz=0, Vm=0, Q=0, Vn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF200D00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VSUB_f_T1_A
/// ASL: `field sz = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "sz", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch32_vsub_f_t1_a_special_sz_0_size_variant_0_3328_ef200d00() {
    // Thumb encoding (32): 0xEF200D00
    // Test aarch32_VSUB_f_T1_A special value sz = 0 (Size variant 0)
    // ISET: T32
    // Fields: Vd=0, N=0, Q=0, M=0, Vm=0, D=0, sz=0, Vn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF200D00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VSUB_f_T1_A
/// ASL: `field sz = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "sz", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch32_vsub_f_t1_a_special_sz_1_size_variant_1_3328_ef300d00() {
    // Thumb encoding (32): 0xEF300D00
    // Test aarch32_VSUB_f_T1_A special value sz = 1 (Size variant 1)
    // ISET: T32
    // Fields: Vm=0, sz=1, M=0, Vn=0, D=0, Vd=0, N=0, Q=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF300D00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VSUB_f_T1_A
/// ASL: `field Q = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "Q", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch32_vsub_f_t1_a_special_q_0_size_variant_0_3328_ef200d00() {
    // Thumb encoding (32): 0xEF200D00
    // Test aarch32_VSUB_f_T1_A special value Q = 0 (Size variant 0)
    // ISET: T32
    // Fields: Q=0, M=0, D=0, N=0, Vd=0, sz=0, Vm=0, Vn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF200D00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VSUB_f_T1_A
/// ASL: `field Q = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "Q", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch32_vsub_f_t1_a_special_q_1_size_variant_1_3328_ef200d40() {
    // Thumb encoding (32): 0xEF200D40
    // Test aarch32_VSUB_f_T1_A special value Q = 1 (Size variant 1)
    // ISET: T32
    // Fields: N=0, M=0, Vm=0, D=0, Vd=0, sz=0, Vn=0, Q=1
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF200D40;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VSUB_f_T1_A
/// ASL: `Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "Q" }), rhs: Binary { op: And, lhs: LitBits([true]), rhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: "Vd" }), indices: [Single(LitInt(0))] }, rhs: Binary { op: Or, lhs: LitBits([true]), rhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: "Vn" }), indices: [Single(LitInt(0))] } } }, rhs: Binary { op: Or, lhs: LitBits([true]), rhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: "Vm" }), indices: [Single(LitInt(0))] } } }, rhs: LitBits([true]) } } }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"Q\" }), rhs: Binary { op: And, lhs: LitBits([true]), rhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: \"Vd\" }), indices: [Single(LitInt(0))] }, rhs: Binary { op: Or, lhs: LitBits([true]), rhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: \"Vn\" }), indices: [Single(LitInt(0))] } } }, rhs: Binary { op: Or, lhs: LitBits([true]), rhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: \"Vm\" }), indices: [Single(LitInt(0))] } } }, rhs: LitBits([true]) } } }" }
/// triggers Undefined
#[test]
fn test_aarch32_vsub_f_t1_a_invalid_0_d00_ef200d00() {
    // Thumb encoding (32): 0xEF200D00
    // Test aarch32_VSUB_f_T1_A invalid encoding: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "Q" }), rhs: Binary { op: And, lhs: LitBits([true]), rhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: "Vd" }), indices: [Single(LitInt(0))] }, rhs: Binary { op: Or, lhs: LitBits([true]), rhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: "Vn" }), indices: [Single(LitInt(0))] } } }, rhs: Binary { op: Or, lhs: LitBits([true]), rhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: "Vm" }), indices: [Single(LitInt(0))] } } }, rhs: LitBits([true]) } } }
    // ISET: T32
    // Fields: sz=0, Q=0, Vn=0, M=0, N=0, Vm=0, Vd=0, D=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF200D00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || !matches!(exit.unwrap(), CpuExit::Continue),
        "expected UNDEFINED for T32 encoding"
    );
}

/// Provenance: aarch32_VSUB_f_T1_A
/// ASL: `Unconditional UNDEFINED`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNDEFINED" }
/// triggers Undefined
#[test]
fn test_aarch32_vsub_f_t1_a_invalid_1_d00_ef200d00() {
    // Thumb encoding (32): 0xEF200D00
    // Test aarch32_VSUB_f_T1_A invalid encoding: Unconditional UNDEFINED
    // ISET: T32
    // Fields: M=0, D=0, N=0, Vd=0, sz=0, Q=0, Vm=0, Vn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF200D00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || !matches!(exit.unwrap(), CpuExit::Continue),
        "expected UNDEFINED for T32 encoding"
    );
}

/// Provenance: aarch32_VSUB_f_T1_A
/// ASL: `Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "sz" }), rhs: Binary { op: And, lhs: LitBits([true]), rhs: Unary { op: Not, operand: Call { name: QualifiedIdentifier { qualifier: Any, name: "HaveFP16Ext" }, args: [] } } } }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"sz\" }), rhs: Binary { op: And, lhs: LitBits([true]), rhs: Unary { op: Not, operand: Call { name: QualifiedIdentifier { qualifier: Any, name: \"HaveFP16Ext\" }, args: [] } } } }" }
/// triggers Undefined
#[test]
fn test_aarch32_vsub_f_t1_a_invalid_2_d00_ef200d00() {
    // Thumb encoding (32): 0xEF200D00
    // Test aarch32_VSUB_f_T1_A invalid encoding: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "sz" }), rhs: Binary { op: And, lhs: LitBits([true]), rhs: Unary { op: Not, operand: Call { name: QualifiedIdentifier { qualifier: Any, name: "HaveFP16Ext" }, args: [] } } } }
    // ISET: T32
    // Fields: D=0, Vd=0, M=0, sz=0, Vn=0, N=0, Q=0, Vm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF200D00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || !matches!(exit.unwrap(), CpuExit::Continue),
        "expected UNDEFINED for T32 encoding"
    );
}

/// Provenance: aarch32_VSUB_f_T1_A
/// ASL: `Unconditional UNDEFINED`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNDEFINED" }
/// triggers Undefined
#[test]
fn test_aarch32_vsub_f_t1_a_invalid_3_d00_ef200d00() {
    // Thumb encoding (32): 0xEF200D00
    // Test aarch32_VSUB_f_T1_A invalid encoding: Unconditional UNDEFINED
    // ISET: T32
    // Fields: M=0, D=0, Vd=0, N=0, sz=0, Vm=0, Vn=0, Q=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF200D00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || !matches!(exit.unwrap(), CpuExit::Continue),
        "expected UNDEFINED for T32 encoding"
    );
}

/// Provenance: aarch32_VSUB_f_T1_A
/// ASL: `Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "sz" }), rhs: Binary { op: And, lhs: LitBits([true]), rhs: Call { name: QualifiedIdentifier { qualifier: Any, name: "InITBlock" }, args: [] } } }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"sz\" }), rhs: Binary { op: And, lhs: LitBits([true]), rhs: Call { name: QualifiedIdentifier { qualifier: Any, name: \"InITBlock\" }, args: [] } } }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_vsub_f_t1_a_invalid_4_d00_ef200d00() {
    // Thumb encoding (32): 0xEF200D00
    // Test aarch32_VSUB_f_T1_A invalid encoding: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "sz" }), rhs: Binary { op: And, lhs: LitBits([true]), rhs: Call { name: QualifiedIdentifier { qualifier: Any, name: "InITBlock" }, args: [] } } }
    // ISET: T32
    // Fields: Q=0, N=0, M=0, Vd=0, sz=0, Vn=0, Vm=0, D=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF200D00;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

/// Provenance: aarch32_VSUB_f_T1_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_vsub_f_t1_a_invalid_5_d00_ef200d00() {
    // Thumb encoding (32): 0xEF200D00
    // Test aarch32_VSUB_f_T1_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: T32
    // Fields: Q=0, Vm=0, Vn=0, Vd=0, sz=0, N=0, D=0, M=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF200D00;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

/// Provenance: aarch32_VSUB_f_T2_A
/// ASL: `field D 22 +: 1`
/// Requirement: FieldBoundary { field: "D", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_vsub_f_t2_a_field_d_0_min_840_ee300840() {
    // Thumb encoding (32): 0xEE300840
    // Test aarch32_VSUB_f_T2_A field D = 0 (Min)
    // ISET: T32
    // Fields: M=0, Vd=0, Vm=0, Vn=0, D=0, N=0, size=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEE300840;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VSUB_f_T2_A
/// ASL: `field D 22 +: 1`
/// Requirement: FieldBoundary { field: "D", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_vsub_f_t2_a_field_d_1_max_840_ee700840() {
    // Thumb encoding (32): 0xEE700840
    // Test aarch32_VSUB_f_T2_A field D = 1 (Max)
    // ISET: T32
    // Fields: M=0, Vm=0, Vn=0, Vd=0, D=1, size=0, N=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEE700840;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VSUB_f_T2_A
/// ASL: `field Vn 16 +: 4`
/// Requirement: FieldBoundary { field: "Vn", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_aarch32_vsub_f_t2_a_field_vn_0_min_840_ee300840() {
    // Thumb encoding (32): 0xEE300840
    // Test aarch32_VSUB_f_T2_A field Vn = 0 (Min)
    // ISET: T32
    // Fields: size=0, Vn=0, N=0, Vd=0, M=0, D=0, Vm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEE300840;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VSUB_f_T2_A
/// ASL: `field Vn 16 +: 4`
/// Requirement: FieldBoundary { field: "Vn", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_aarch32_vsub_f_t2_a_field_vn_1_poweroftwo_840_ee310840() {
    // Thumb encoding (32): 0xEE310840
    // Test aarch32_VSUB_f_T2_A field Vn = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: D=0, Vn=1, Vd=0, M=0, size=0, Vm=0, N=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEE310840;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VSUB_f_T2_A
/// ASL: `field Vd 12 +: 4`
/// Requirement: FieldBoundary { field: "Vd", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_aarch32_vsub_f_t2_a_field_vd_0_min_840_ee300840() {
    // Thumb encoding (32): 0xEE300840
    // Test aarch32_VSUB_f_T2_A field Vd = 0 (Min)
    // ISET: T32
    // Fields: Vd=0, M=0, size=0, Vm=0, N=0, D=0, Vn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEE300840;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VSUB_f_T2_A
/// ASL: `field Vd 12 +: 4`
/// Requirement: FieldBoundary { field: "Vd", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_aarch32_vsub_f_t2_a_field_vd_1_poweroftwo_840_ee301840() {
    // Thumb encoding (32): 0xEE301840
    // Test aarch32_VSUB_f_T2_A field Vd = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: D=0, size=0, M=0, Vd=1, N=0, Vm=0, Vn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEE301840;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VSUB_f_T2_A
/// ASL: `field size 8 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch32_vsub_f_t2_a_field_size_0_min_840_ee300840() {
    // Thumb encoding (32): 0xEE300840
    // Test aarch32_VSUB_f_T2_A field size = 0 (Min)
    // ISET: T32
    // Fields: M=0, Vd=0, Vn=0, size=0, N=0, Vm=0, D=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEE300840;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VSUB_f_T2_A
/// ASL: `field size 8 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 1, boundary: PowerOfTwo }
/// 16-bit / halfword size
#[test]
fn test_aarch32_vsub_f_t2_a_field_size_1_poweroftwo_840_ee300940() {
    // Thumb encoding (32): 0xEE300940
    // Test aarch32_VSUB_f_T2_A field size = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Vn=0, Vd=0, size=1, N=0, M=0, D=0, Vm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEE300940;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VSUB_f_T2_A
/// ASL: `field size 8 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 2, boundary: PowerOfTwo }
/// 32-bit / word size
#[test]
fn test_aarch32_vsub_f_t2_a_field_size_2_poweroftwo_840_ee300a40() {
    // Thumb encoding (32): 0xEE300A40
    // Test aarch32_VSUB_f_T2_A field size = 2 (PowerOfTwo)
    // ISET: T32
    // Fields: Vn=0, M=0, Vm=0, Vd=0, N=0, D=0, size=2
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEE300A40;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VSUB_f_T2_A
/// ASL: `field size 8 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 3, boundary: Max }
/// 64-bit / doubleword size
#[test]
fn test_aarch32_vsub_f_t2_a_field_size_3_max_840_ee300b40() {
    // Thumb encoding (32): 0xEE300B40
    // Test aarch32_VSUB_f_T2_A field size = 3 (Max)
    // ISET: T32
    // Fields: D=0, size=3, M=0, Vm=0, Vd=0, N=0, Vn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEE300B40;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VSUB_f_T2_A
/// ASL: `field N 7 +: 1`
/// Requirement: FieldBoundary { field: "N", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_vsub_f_t2_a_field_n_0_min_840_ee300840() {
    // Thumb encoding (32): 0xEE300840
    // Test aarch32_VSUB_f_T2_A field N = 0 (Min)
    // ISET: T32
    // Fields: D=0, Vd=0, N=0, Vm=0, M=0, size=0, Vn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEE300840;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VSUB_f_T2_A
/// ASL: `field N 7 +: 1`
/// Requirement: FieldBoundary { field: "N", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_vsub_f_t2_a_field_n_1_max_840_ee3008c0() {
    // Thumb encoding (32): 0xEE3008C0
    // Test aarch32_VSUB_f_T2_A field N = 1 (Max)
    // ISET: T32
    // Fields: Vd=0, Vm=0, N=1, Vn=0, D=0, size=0, M=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEE3008C0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VSUB_f_T2_A
/// ASL: `field M 5 +: 1`
/// Requirement: FieldBoundary { field: "M", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_vsub_f_t2_a_field_m_0_min_840_ee300840() {
    // Thumb encoding (32): 0xEE300840
    // Test aarch32_VSUB_f_T2_A field M = 0 (Min)
    // ISET: T32
    // Fields: size=0, Vd=0, M=0, N=0, D=0, Vm=0, Vn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEE300840;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VSUB_f_T2_A
/// ASL: `field M 5 +: 1`
/// Requirement: FieldBoundary { field: "M", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_vsub_f_t2_a_field_m_1_max_840_ee300860() {
    // Thumb encoding (32): 0xEE300860
    // Test aarch32_VSUB_f_T2_A field M = 1 (Max)
    // ISET: T32
    // Fields: D=0, N=0, size=0, Vd=0, Vn=0, M=1, Vm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEE300860;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VSUB_f_T2_A
/// ASL: `field Vm 0 +: 4`
/// Requirement: FieldBoundary { field: "Vm", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_aarch32_vsub_f_t2_a_field_vm_0_min_840_ee300840() {
    // Thumb encoding (32): 0xEE300840
    // Test aarch32_VSUB_f_T2_A field Vm = 0 (Min)
    // ISET: T32
    // Fields: M=0, Vn=0, D=0, size=0, N=0, Vm=0, Vd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEE300840;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VSUB_f_T2_A
/// ASL: `field Vm 0 +: 4`
/// Requirement: FieldBoundary { field: "Vm", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_aarch32_vsub_f_t2_a_field_vm_1_poweroftwo_840_ee300841() {
    // Thumb encoding (32): 0xEE300841
    // Test aarch32_VSUB_f_T2_A field Vm = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: D=0, Vn=0, N=0, Vd=0, size=0, Vm=1, M=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEE300841;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VSUB_f_T2_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// D=0 (minimum value)
#[test]
fn test_aarch32_vsub_f_t2_a_combo_0_840_ee300840() {
    // Thumb encoding (32): 0xEE300840
    // Test aarch32_VSUB_f_T2_A field combination: D=0, Vn=0, Vd=0, size=0, N=0, M=0, Vm=0
    // ISET: T32
    // Fields: D=0, Vn=0, Vd=0, N=0, Vm=0, size=0, M=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEE300840;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VSUB_f_T2_A
/// ASL: `field size = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "size", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch32_vsub_f_t2_a_special_size_0_size_variant_0_2112_ee300840() {
    // Thumb encoding (32): 0xEE300840
    // Test aarch32_VSUB_f_T2_A special value size = 0 (Size variant 0)
    // ISET: T32
    // Fields: Vm=0, N=0, D=0, size=0, M=0, Vd=0, Vn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEE300840;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VSUB_f_T2_A
/// ASL: `field size = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "size", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch32_vsub_f_t2_a_special_size_1_size_variant_1_2112_ee300940() {
    // Thumb encoding (32): 0xEE300940
    // Test aarch32_VSUB_f_T2_A special value size = 1 (Size variant 1)
    // ISET: T32
    // Fields: Vn=0, Vd=0, N=0, D=0, M=0, size=1, Vm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEE300940;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VSUB_f_T2_A
/// ASL: `field size = 2 (Size variant 2)`
/// Requirement: FieldSpecial { field: "size", value: 2, meaning: "Size variant 2" }
/// Size variant 2
#[test]
fn test_aarch32_vsub_f_t2_a_special_size_2_size_variant_2_2112_ee300a40() {
    // Thumb encoding (32): 0xEE300A40
    // Test aarch32_VSUB_f_T2_A special value size = 2 (Size variant 2)
    // ISET: T32
    // Fields: size=2, Vd=0, N=0, D=0, M=0, Vn=0, Vm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEE300A40;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VSUB_f_T2_A
/// ASL: `field size = 3 (Size variant 3)`
/// Requirement: FieldSpecial { field: "size", value: 3, meaning: "Size variant 3" }
/// Size variant 3
#[test]
fn test_aarch32_vsub_f_t2_a_special_size_3_size_variant_3_2112_ee300b40() {
    // Thumb encoding (32): 0xEE300B40
    // Test aarch32_VSUB_f_T2_A special value size = 3 (Size variant 3)
    // ISET: T32
    // Fields: Vm=0, Vd=0, D=0, size=3, Vn=0, N=0, M=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEE300B40;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VSUB_f_T2_A
/// ASL: `Binary { op: Ne, lhs: Binary { op: Ne, lhs: Member { base: Var(QualifiedIdentifier { qualifier: Any, name: "FPSCR" }), field: "Len" }, rhs: Binary { op: Or, lhs: LitBits([false, false, false]), rhs: Member { base: Var(QualifiedIdentifier { qualifier: Any, name: "FPSCR" }), field: "Stride" } } }, rhs: LitBits([false, false]) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Ne, lhs: Binary { op: Ne, lhs: Member { base: Var(QualifiedIdentifier { qualifier: Any, name: \"FPSCR\" }), field: \"Len\" }, rhs: Binary { op: Or, lhs: LitBits([false, false, false]), rhs: Member { base: Var(QualifiedIdentifier { qualifier: Any, name: \"FPSCR\" }), field: \"Stride\" } } }, rhs: LitBits([false, false]) }" }
/// triggers Undefined
#[test]
fn test_aarch32_vsub_f_t2_a_invalid_0_840_ee300840() {
    // Thumb encoding (32): 0xEE300840
    // Test aarch32_VSUB_f_T2_A invalid encoding: Binary { op: Ne, lhs: Binary { op: Ne, lhs: Member { base: Var(QualifiedIdentifier { qualifier: Any, name: "FPSCR" }), field: "Len" }, rhs: Binary { op: Or, lhs: LitBits([false, false, false]), rhs: Member { base: Var(QualifiedIdentifier { qualifier: Any, name: "FPSCR" }), field: "Stride" } } }, rhs: LitBits([false, false]) }
    // ISET: T32
    // Fields: D=0, Vn=0, size=0, N=0, M=0, Vm=0, Vd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEE300840;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || !matches!(exit.unwrap(), CpuExit::Continue),
        "expected UNDEFINED for T32 encoding"
    );
}

/// Provenance: aarch32_VSUB_f_T2_A
/// ASL: `Unconditional UNDEFINED`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNDEFINED" }
/// triggers Undefined
#[test]
fn test_aarch32_vsub_f_t2_a_invalid_1_840_ee300840() {
    // Thumb encoding (32): 0xEE300840
    // Test aarch32_VSUB_f_T2_A invalid encoding: Unconditional UNDEFINED
    // ISET: T32
    // Fields: size=0, Vm=0, N=0, Vn=0, M=0, Vd=0, D=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEE300840;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || !matches!(exit.unwrap(), CpuExit::Continue),
        "expected UNDEFINED for T32 encoding"
    );
}

/// Provenance: aarch32_VSUB_f_T2_A
/// ASL: `Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "size" }), rhs: Binary { op: Or, lhs: LitBits([false, false]), rhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "size" }), rhs: Binary { op: And, lhs: LitBits([false, true]), rhs: Unary { op: Not, operand: Call { name: QualifiedIdentifier { qualifier: Any, name: "HaveFP16Ext" }, args: [] } } } } } }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"size\" }), rhs: Binary { op: Or, lhs: LitBits([false, false]), rhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"size\" }), rhs: Binary { op: And, lhs: LitBits([false, true]), rhs: Unary { op: Not, operand: Call { name: QualifiedIdentifier { qualifier: Any, name: \"HaveFP16Ext\" }, args: [] } } } } } }" }
/// triggers Undefined
#[test]
fn test_aarch32_vsub_f_t2_a_invalid_2_840_ee300840() {
    // Thumb encoding (32): 0xEE300840
    // Test aarch32_VSUB_f_T2_A invalid encoding: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "size" }), rhs: Binary { op: Or, lhs: LitBits([false, false]), rhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "size" }), rhs: Binary { op: And, lhs: LitBits([false, true]), rhs: Unary { op: Not, operand: Call { name: QualifiedIdentifier { qualifier: Any, name: "HaveFP16Ext" }, args: [] } } } } } }
    // ISET: T32
    // Fields: Vn=0, size=0, D=0, N=0, M=0, Vm=0, Vd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEE300840;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || !matches!(exit.unwrap(), CpuExit::Continue),
        "expected UNDEFINED for T32 encoding"
    );
}

/// Provenance: aarch32_VSUB_f_T2_A
/// ASL: `Unconditional UNDEFINED`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNDEFINED" }
/// triggers Undefined
#[test]
fn test_aarch32_vsub_f_t2_a_invalid_3_840_ee300840() {
    // Thumb encoding (32): 0xEE300840
    // Test aarch32_VSUB_f_T2_A invalid encoding: Unconditional UNDEFINED
    // ISET: T32
    // Fields: Vd=0, D=0, N=0, M=0, Vm=0, size=0, Vn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEE300840;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || !matches!(exit.unwrap(), CpuExit::Continue),
        "expected UNDEFINED for T32 encoding"
    );
}

/// Provenance: aarch32_VSUB_f_T2_A
/// ASL: `Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "size" }), rhs: Binary { op: And, lhs: LitBits([false, true]), rhs: Call { name: QualifiedIdentifier { qualifier: Any, name: "InITBlock" }, args: [] } } }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"size\" }), rhs: Binary { op: And, lhs: LitBits([false, true]), rhs: Call { name: QualifiedIdentifier { qualifier: Any, name: \"InITBlock\" }, args: [] } } }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_vsub_f_t2_a_invalid_4_840_ee300840() {
    // Thumb encoding (32): 0xEE300840
    // Test aarch32_VSUB_f_T2_A invalid encoding: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "size" }), rhs: Binary { op: And, lhs: LitBits([false, true]), rhs: Call { name: QualifiedIdentifier { qualifier: Any, name: "InITBlock" }, args: [] } } }
    // ISET: T32
    // Fields: D=0, size=0, M=0, Vm=0, Vn=0, N=0, Vd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEE300840;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

/// Provenance: aarch32_VSUB_f_T2_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_vsub_f_t2_a_invalid_5_840_ee300840() {
    // Thumb encoding (32): 0xEE300840
    // Test aarch32_VSUB_f_T2_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: T32
    // Fields: size=0, Vd=0, D=0, N=0, M=0, Vn=0, Vm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEE300840;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

/// Provenance: aarch32_VSUB_f_A1_A
/// ASL: `EOR X0, X1, #0xFF`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// mask lower 8 bits (64)
#[test]
fn test_aarch32_vsub_f_a1_a_eor_oracle_64_0_d2401c20() {
    // Test EOR 64-bit: mask lower 8 bits (oracle)
    // Encoding: 0xD2401C20
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xD2401C20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFF00,
        "X0 should be 0xFFFFFFFFFFFFFF00"
    );
}

/// Provenance: aarch32_VSUB_f_A1_A
/// ASL: `EOR X0, X1, #0xFFFF`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// mask lower 16 bits (64)
#[test]
fn test_aarch32_vsub_f_a1_a_eor_oracle_64_1_d2403c20() {
    // Test EOR 64-bit: mask lower 16 bits (oracle)
    // Encoding: 0xD2403C20
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xD2403C20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFF0000,
        "X0 should be 0xFFFFFFFFFFFF0000"
    );
}

/// Provenance: aarch32_VSUB_f_A1_A
/// ASL: `EOR X0, X1, #0xFFFFFFFF`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// mask lower 32 bits (64)
#[test]
fn test_aarch32_vsub_f_a1_a_eor_oracle_64_2_d2407c20() {
    // Test EOR 64-bit: mask lower 32 bits (oracle)
    // Encoding: 0xD2407C20
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xD2407C20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x0, "X0 should be 0xFFFFFFFF00000000");
}

/// Provenance: aarch32_VSUB_f_A1_A
/// ASL: `EOR X0, X1, #0x1`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// single bit mask (64)
#[test]
fn test_aarch32_vsub_f_a1_a_eor_oracle_64_3_d2400020() {
    // Test EOR 64-bit: single bit mask (oracle)
    // Encoding: 0xD2400020
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xCAFEBABE);
    let encoding: u32 = 0xD2400020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xCAFEBABF,
        "X0 should be 0xDEADBEEFCAFEBABF"
    );
}

/// Provenance: aarch32_VSUB_f_A1_A
/// ASL: `EOR X0, X1, #0x7FFFFFFFFFFFFFFF`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// all but MSB (64)
#[test]
fn test_aarch32_vsub_f_a1_a_eor_oracle_64_4_d240f820() {
    // Test EOR 64-bit: all but MSB (oracle)
    // Encoding: 0xD240F820
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xAAAAAAAA);
    let encoding: u32 = 0xD240F820;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0x55555555,
        "X0 should be 0xD555555555555555"
    );
}

/// Provenance: aarch32_VSUB_f_A1_A
/// ASL: `EOR W0, W1, #0xFF`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// mask lower 8 bits (32)
#[test]
fn test_aarch32_vsub_f_a1_a_eor_oracle_32_0_52001c20() {
    // Test EOR 32-bit: mask lower 8 bits (oracle)
    // Encoding: 0x52001C20
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0x52001C20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0xFFFFFF00, "W0 should be 0xFFFFFF00");
}

/// Provenance: aarch32_VSUB_f_A1_A
/// ASL: `EOR W0, W1, #0xFFFF`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// mask lower 16 bits (32)
#[test]
fn test_aarch32_vsub_f_a1_a_eor_oracle_32_1_52003c20() {
    // Test EOR 32-bit: mask lower 16 bits (oracle)
    // Encoding: 0x52003C20
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0x52003C20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0xFFFF0000, "W0 should be 0xFFFF0000");
}

/// Provenance: aarch32_VSUB_f_A1_A
/// ASL: `EOR W0, W1, #0x1`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// single bit mask (32)
#[test]
fn test_aarch32_vsub_f_a1_a_eor_oracle_32_2_52000020() {
    // Test EOR 32-bit: single bit mask (oracle)
    // Encoding: 0x52000020
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xDEADBEEF);
    let encoding: u32 = 0x52000020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0xDEADBEEE, "W0 should be 0xDEADBEEE");
}

/// Provenance: aarch32_VSUB_f_A1_A
/// ASL: `EOR R0, R1, #10`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// small immediate (Rn=0x00000000)
#[test]
fn test_aarch32_vsub_f_a1_a_a32_logical_imm_0_0_f221000a() {
    // Test A32 EOR: small immediate (oracle)
    // Encoding: 0xF221000A
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF221000A;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0xA, "R0 should be 0x0000000A");
}

/// Provenance: aarch32_VSUB_f_A1_A
/// ASL: `EOR R0, R1, #10`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// small immediate (Rn=0x000000FF)
#[test]
fn test_aarch32_vsub_f_a1_a_a32_logical_imm_0_ff_f221000a() {
    // Test A32 EOR: small immediate (oracle)
    // Encoding: 0xF221000A
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFF);
    let encoding: u32 = 0xF221000A;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0xF5, "R0 should be 0x000000F5");
}

/// Provenance: aarch32_VSUB_f_A1_A
/// ASL: `EOR R0, R1, #10`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// small immediate (Rn=0xAAAAAAAA)
#[test]
fn test_aarch32_vsub_f_a1_a_a32_logical_imm_0_aaaaaaaa_f221000a() {
    // Test A32 EOR: small immediate (oracle)
    // Encoding: 0xF221000A
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xAAAAAAAA);
    let encoding: u32 = 0xF221000A;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0xAAAAAAA0, "R0 should be 0xAAAAAAA0");
}

/// Provenance: aarch32_VSUB_f_A1_A
/// ASL: `EOR R0, R1, #10`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// small immediate (Rn=0x55555555)
#[test]
fn test_aarch32_vsub_f_a1_a_a32_logical_imm_0_55555555_f221000a() {
    // Test A32 EOR: small immediate (oracle)
    // Encoding: 0xF221000A
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x55555555);
    let encoding: u32 = 0xF221000A;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x5555555F, "R0 should be 0x5555555F");
}

/// Provenance: aarch32_VSUB_f_A1_A
/// ASL: `EOR R0, R1, #10`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// small immediate (Rn=0xFFFFFFFF)
#[test]
fn test_aarch32_vsub_f_a1_a_a32_logical_imm_0_ffffffff_f221000a() {
    // Test A32 EOR: small immediate (oracle)
    // Encoding: 0xF221000A
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF221000A;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0xFFFFFFF5, "R0 should be 0xFFFFFFF5");
}

/// Provenance: aarch32_VSUB_f_A1_A
/// ASL: `EOR R0, R1, #255`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// max imm8 (Rn=0x00000000)
#[test]
fn test_aarch32_vsub_f_a1_a_a32_logical_imm_1_0_f22100ff() {
    // Test A32 EOR: max imm8 (oracle)
    // Encoding: 0xF22100FF
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF22100FF;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0xFF, "R0 should be 0x000000FF");
}

/// Provenance: aarch32_VSUB_f_A1_A
/// ASL: `EOR R0, R1, #255`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// max imm8 (Rn=0x000000FF)
#[test]
fn test_aarch32_vsub_f_a1_a_a32_logical_imm_1_ff_f22100ff() {
    // Test A32 EOR: max imm8 (oracle)
    // Encoding: 0xF22100FF
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFF);
    let encoding: u32 = 0xF22100FF;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x0, "R0 should be 0x00000000");
}

/// Provenance: aarch32_VSUB_f_A1_A
/// ASL: `EOR R0, R1, #255`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// max imm8 (Rn=0xAAAAAAAA)
#[test]
fn test_aarch32_vsub_f_a1_a_a32_logical_imm_1_aaaaaaaa_f22100ff() {
    // Test A32 EOR: max imm8 (oracle)
    // Encoding: 0xF22100FF
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xAAAAAAAA);
    let encoding: u32 = 0xF22100FF;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0xAAAAAA55, "R0 should be 0xAAAAAA55");
}

/// Provenance: aarch32_VSUB_f_A1_A
/// ASL: `EOR R0, R1, #255`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// max imm8 (Rn=0x55555555)
#[test]
fn test_aarch32_vsub_f_a1_a_a32_logical_imm_1_55555555_f22100ff() {
    // Test A32 EOR: max imm8 (oracle)
    // Encoding: 0xF22100FF
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x55555555);
    let encoding: u32 = 0xF22100FF;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x555555AA, "R0 should be 0x555555AA");
}

/// Provenance: aarch32_VSUB_f_A1_A
/// ASL: `EOR R0, R1, #255`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// max imm8 (Rn=0xFFFFFFFF)
#[test]
fn test_aarch32_vsub_f_a1_a_a32_logical_imm_1_ffffffff_f22100ff() {
    // Test A32 EOR: max imm8 (oracle)
    // Encoding: 0xF22100FF
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF22100FF;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0xFFFFFF00, "R0 should be 0xFFFFFF00");
}

/// Provenance: aarch32_VSUB_f_A1_A
/// ASL: `EOR R0, R1, #32`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// rotated by 2 (Rn=0x00000000)
#[test]
fn test_aarch32_vsub_f_a1_a_a32_logical_imm_2_0_f2210180() {
    // Test A32 EOR: rotated by 2 (oracle)
    // Encoding: 0xF2210180
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2210180;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x20, "R0 should be 0x00000020");
}

/// Provenance: aarch32_VSUB_f_A1_A
/// ASL: `EOR R0, R1, #32`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// rotated by 2 (Rn=0x000000FF)
#[test]
fn test_aarch32_vsub_f_a1_a_a32_logical_imm_2_ff_f2210180() {
    // Test A32 EOR: rotated by 2 (oracle)
    // Encoding: 0xF2210180
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFF);
    let encoding: u32 = 0xF2210180;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0xDF, "R0 should be 0x000000DF");
}

/// Provenance: aarch32_VSUB_f_A1_A
/// ASL: `EOR R0, R1, #32`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// rotated by 2 (Rn=0xAAAAAAAA)
#[test]
fn test_aarch32_vsub_f_a1_a_a32_logical_imm_2_aaaaaaaa_f2210180() {
    // Test A32 EOR: rotated by 2 (oracle)
    // Encoding: 0xF2210180
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xAAAAAAAA);
    let encoding: u32 = 0xF2210180;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0xAAAAAA8A, "R0 should be 0xAAAAAA8A");
}

/// Provenance: aarch32_VSUB_f_A1_A
/// ASL: `EOR R0, R1, #32`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// rotated by 2 (Rn=0x55555555)
#[test]
fn test_aarch32_vsub_f_a1_a_a32_logical_imm_2_55555555_f2210180() {
    // Test A32 EOR: rotated by 2 (oracle)
    // Encoding: 0xF2210180
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x55555555);
    let encoding: u32 = 0xF2210180;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x55555575, "R0 should be 0x55555575");
}

/// Provenance: aarch32_VSUB_f_A1_A
/// ASL: `EOR R0, R1, #32`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// rotated by 2 (Rn=0xFFFFFFFF)
#[test]
fn test_aarch32_vsub_f_a1_a_a32_logical_imm_2_ffffffff_f2210180() {
    // Test A32 EOR: rotated by 2 (oracle)
    // Encoding: 0xF2210180
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2210180;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0xFFFFFFDF, "R0 should be 0xFFFFFFDF");
}

/// Provenance: aarch32_VSUB_f_A1_A
/// ASL: `EOR R0, R1, #251658240`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// rotated by 8 (Rn=0x00000000)
#[test]
fn test_aarch32_vsub_f_a1_a_a32_logical_imm_3_0_f221040f() {
    // Test A32 EOR: rotated by 8 (oracle)
    // Encoding: 0xF221040F
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF221040F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0xF000000, "R0 should be 0x0F000000");
}

/// Provenance: aarch32_VSUB_f_A1_A
/// ASL: `EOR R0, R1, #251658240`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// rotated by 8 (Rn=0x000000FF)
#[test]
fn test_aarch32_vsub_f_a1_a_a32_logical_imm_3_ff_f221040f() {
    // Test A32 EOR: rotated by 8 (oracle)
    // Encoding: 0xF221040F
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFF);
    let encoding: u32 = 0xF221040F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0xF0000FF, "R0 should be 0x0F0000FF");
}

/// Provenance: aarch32_VSUB_f_A1_A
/// ASL: `EOR R0, R1, #251658240`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// rotated by 8 (Rn=0xAAAAAAAA)
#[test]
fn test_aarch32_vsub_f_a1_a_a32_logical_imm_3_aaaaaaaa_f221040f() {
    // Test A32 EOR: rotated by 8 (oracle)
    // Encoding: 0xF221040F
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xAAAAAAAA);
    let encoding: u32 = 0xF221040F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0xA5AAAAAA, "R0 should be 0xA5AAAAAA");
}

/// Provenance: aarch32_VSUB_f_A1_A
/// ASL: `EOR R0, R1, #251658240`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// rotated by 8 (Rn=0x55555555)
#[test]
fn test_aarch32_vsub_f_a1_a_a32_logical_imm_3_55555555_f221040f() {
    // Test A32 EOR: rotated by 8 (oracle)
    // Encoding: 0xF221040F
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x55555555);
    let encoding: u32 = 0xF221040F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x5A555555, "R0 should be 0x5A555555");
}

/// Provenance: aarch32_VSUB_f_A1_A
/// ASL: `EOR R0, R1, #251658240`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// rotated by 8 (Rn=0xFFFFFFFF)
#[test]
fn test_aarch32_vsub_f_a1_a_a32_logical_imm_3_ffffffff_f221040f() {
    // Test A32 EOR: rotated by 8 (oracle)
    // Encoding: 0xF221040F
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF221040F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0xF0FFFFFF, "R0 should be 0xF0FFFFFF");
}

/// Provenance: aarch32_VSUB_f_A1_A
/// ASL: `EOR R0, R1, #0`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// zero immediate (Rn=0x00000000)
#[test]
fn test_aarch32_vsub_f_a1_a_a32_logical_imm_4_0_f2210000() {
    // Test A32 EOR: zero immediate (oracle)
    // Encoding: 0xF2210000
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2210000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x0, "R0 should be 0x00000000");
}

/// Provenance: aarch32_VSUB_f_A1_A
/// ASL: `EOR R0, R1, #0`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// zero immediate (Rn=0x000000FF)
#[test]
fn test_aarch32_vsub_f_a1_a_a32_logical_imm_4_ff_f2210000() {
    // Test A32 EOR: zero immediate (oracle)
    // Encoding: 0xF2210000
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFF);
    let encoding: u32 = 0xF2210000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0xFF, "R0 should be 0x000000FF");
}

/// Provenance: aarch32_VSUB_f_A1_A
/// ASL: `EOR R0, R1, #0`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// zero immediate (Rn=0xAAAAAAAA)
#[test]
fn test_aarch32_vsub_f_a1_a_a32_logical_imm_4_aaaaaaaa_f2210000() {
    // Test A32 EOR: zero immediate (oracle)
    // Encoding: 0xF2210000
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xAAAAAAAA);
    let encoding: u32 = 0xF2210000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0xAAAAAAAA, "R0 should be 0xAAAAAAAA");
}

/// Provenance: aarch32_VSUB_f_A1_A
/// ASL: `EOR R0, R1, #0`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// zero immediate (Rn=0x55555555)
#[test]
fn test_aarch32_vsub_f_a1_a_a32_logical_imm_4_55555555_f2210000() {
    // Test A32 EOR: zero immediate (oracle)
    // Encoding: 0xF2210000
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x55555555);
    let encoding: u32 = 0xF2210000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x55555555, "R0 should be 0x55555555");
}

/// Provenance: aarch32_VSUB_f_A1_A
/// ASL: `EOR R0, R1, #0`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// zero immediate (Rn=0xFFFFFFFF)
#[test]
fn test_aarch32_vsub_f_a1_a_a32_logical_imm_4_ffffffff_f2210000() {
    // Test A32 EOR: zero immediate (oracle)
    // Encoding: 0xF2210000
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2210000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0xFFFFFFFF, "R0 should be 0xFFFFFFFF");
}
