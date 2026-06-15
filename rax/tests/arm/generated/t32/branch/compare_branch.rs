//! T32 branch compare_branch tests.
//!
//! Auto-generated from ARM ASL specifications.
//! DO NOT EDIT MANUALLY.

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::generated::test_helpers_32::*;

// ============================================================================
// aarch32_TBB_A Tests
// ============================================================================

/// Provenance: aarch32_TBB_T1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_tbb_t1_a_field_rn_0_min_0_e8d00000() {
    // Thumb encoding (32): 0xE8D00000
    // Test aarch32_TBB_T1_A field Rn = 0 (Min)
    // ISET: T32
    // Fields: Rm=0, Rn=0, H=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xE8D00000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_TBB_T1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_tbb_t1_a_field_rn_1_poweroftwo_0_e8d10000() {
    // Thumb encoding (32): 0xE8D10000
    // Test aarch32_TBB_T1_A field Rn = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rn=1, H=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xE8D10000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_TBB_T1_A
/// ASL: `field H 4 +: 1`
/// Requirement: FieldBoundary { field: "H", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_tbb_t1_a_field_h_0_min_0_e8d00000() {
    // Thumb encoding (32): 0xE8D00000
    // Test aarch32_TBB_T1_A field H = 0 (Min)
    // ISET: T32
    // Fields: H=0, Rn=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xE8D00000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_TBB_T1_A
/// ASL: `field H 4 +: 1`
/// Requirement: FieldBoundary { field: "H", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_tbb_t1_a_field_h_1_max_0_e8d00010() {
    // Thumb encoding (32): 0xE8D00010
    // Test aarch32_TBB_T1_A field H = 1 (Max)
    // ISET: T32
    // Fields: Rn=0, H=1, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xE8D00010;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_TBB_T1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_tbb_t1_a_field_rm_0_min_0_e8d00000() {
    // Thumb encoding (32): 0xE8D00000
    // Test aarch32_TBB_T1_A field Rm = 0 (Min)
    // ISET: T32
    // Fields: Rm=0, Rn=0, H=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xE8D00000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_TBB_T1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_tbb_t1_a_field_rm_1_poweroftwo_0_e8d00001() {
    // Thumb encoding (32): 0xE8D00001
    // Test aarch32_TBB_T1_A field Rm = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rm=1, Rn=0, H=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xE8D00001;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_TBB_T1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch32_tbb_t1_a_combo_0_0_e8d00000() {
    // Thumb encoding (32): 0xE8D00000
    // Test aarch32_TBB_T1_A field combination: Rn=0, H=0, Rm=0
    // ISET: T32
    // Fields: H=0, Rn=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xE8D00000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_TBB_T1_A
/// ASL: `Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }), rhs: LitInt(15) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"m\" }), rhs: LitInt(15) }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_tbb_t1_a_invalid_0_0_e8d00000() {
    // Thumb encoding (32): 0xE8D00000
    // Test aarch32_TBB_T1_A invalid encoding: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }), rhs: LitInt(15) }
    // ISET: T32
    // Fields: Rn=0, Rm=0, H=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xE8D00000;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

/// Provenance: aarch32_TBB_T1_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_tbb_t1_a_invalid_1_0_e8d00000() {
    // Thumb encoding (32): 0xE8D00000
    // Test aarch32_TBB_T1_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: T32
    // Fields: Rm=0, Rn=0, H=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xE8D00000;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

/// Provenance: aarch32_TBB_T1_A
/// ASL: `Binary { op: And, lhs: Call { name: QualifiedIdentifier { qualifier: Any, name: "InITBlock" }, args: [] }, rhs: Unary { op: Not, operand: Call { name: QualifiedIdentifier { qualifier: Any, name: "LastInITBlock" }, args: [] } } }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: And, lhs: Call { name: QualifiedIdentifier { qualifier: Any, name: \"InITBlock\" }, args: [] }, rhs: Unary { op: Not, operand: Call { name: QualifiedIdentifier { qualifier: Any, name: \"LastInITBlock\" }, args: [] } } }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_tbb_t1_a_invalid_2_0_e8d00000() {
    // Thumb encoding (32): 0xE8D00000
    // Test aarch32_TBB_T1_A invalid encoding: Binary { op: And, lhs: Call { name: QualifiedIdentifier { qualifier: Any, name: "InITBlock" }, args: [] }, rhs: Unary { op: Not, operand: Call { name: QualifiedIdentifier { qualifier: Any, name: "LastInITBlock" }, args: [] } } }
    // ISET: T32
    // Fields: H=0, Rm=0, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xE8D00000;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

/// Provenance: aarch32_TBB_T1_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_tbb_t1_a_invalid_3_0_e8d00000() {
    // Thumb encoding (32): 0xE8D00000
    // Test aarch32_TBB_T1_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: T32
    // Fields: Rn=0, Rm=0, H=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xE8D00000;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}
