//! T32 misc other tests.
//!
//! Auto-generated from ARM ASL specifications.
//! DO NOT EDIT MANUALLY.

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::generated::test_helpers_32::*;

// ============================================================================
// aarch32_DCPS_A Tests
// ============================================================================

/// Provenance: aarch32_DCPS_T1_A
/// ASL: `field opt 0 +: 2`
/// Requirement: FieldBoundary { field: "opt", value: 0, boundary: Min }
/// option 0
#[test]
fn test_aarch32_dcps_t1_a_field_opt_0_min_8000_f78f8000() {
    // Thumb encoding (32): 0xF78F8000
    // Test aarch32_DCPS_T1_A field opt = 0 (Min)
    // ISET: T32
    // Fields: opt=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF78F8000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_DCPS_T1_A
/// ASL: `field opt 0 +: 2`
/// Requirement: FieldBoundary { field: "opt", value: 1, boundary: PowerOfTwo }
/// option 1
#[test]
fn test_aarch32_dcps_t1_a_field_opt_1_poweroftwo_8000_f78f8001() {
    // Thumb encoding (32): 0xF78F8001
    // Test aarch32_DCPS_T1_A field opt = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: opt=1
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF78F8001;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_DCPS_T1_A
/// ASL: `field opt 0 +: 2`
/// Requirement: FieldBoundary { field: "opt", value: 2, boundary: PowerOfTwo }
/// option 2
#[test]
fn test_aarch32_dcps_t1_a_field_opt_2_poweroftwo_8000_f78f8002() {
    // Thumb encoding (32): 0xF78F8002
    // Test aarch32_DCPS_T1_A field opt = 2 (PowerOfTwo)
    // ISET: T32
    // Fields: opt=2
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF78F8002;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_DCPS_T1_A
/// ASL: `field opt 0 +: 2`
/// Requirement: FieldBoundary { field: "opt", value: 3, boundary: Max }
/// option 3
#[test]
fn test_aarch32_dcps_t1_a_field_opt_3_max_8000_f78f8003() {
    // Thumb encoding (32): 0xF78F8003
    // Test aarch32_DCPS_T1_A field opt = 3 (Max)
    // ISET: T32
    // Fields: opt=3
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF78F8003;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_DCPS_T1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// opt=0 (option 0)
#[test]
fn test_aarch32_dcps_t1_a_combo_0_8000_f78f8000() {
    // Thumb encoding (32): 0xF78F8000
    // Test aarch32_DCPS_T1_A field combination: opt=0
    // ISET: T32
    // Fields: opt=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF78F8000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_DCPS_T1_A
/// ASL: `Binary { op: Eq, lhs: Binary { op: Or, lhs: Unary { op: Not, operand: Call { name: QualifiedIdentifier { qualifier: Any, name: "Halted" }, args: [] } }, rhs: Var(QualifiedIdentifier { qualifier: Any, name: "opt" }) }, rhs: LitBits([false, false]) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Binary { op: Or, lhs: Unary { op: Not, operand: Call { name: QualifiedIdentifier { qualifier: Any, name: \"Halted\" }, args: [] } }, rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"opt\" }) }, rhs: LitBits([false, false]) }" }
/// triggers Undefined
#[test]
fn test_aarch32_dcps_t1_a_invalid_0_8000_f78f8000() {
    // Thumb encoding (32): 0xF78F8000
    // Test aarch32_DCPS_T1_A invalid encoding: Binary { op: Eq, lhs: Binary { op: Or, lhs: Unary { op: Not, operand: Call { name: QualifiedIdentifier { qualifier: Any, name: "Halted" }, args: [] } }, rhs: Var(QualifiedIdentifier { qualifier: Any, name: "opt" }) }, rhs: LitBits([false, false]) }
    // ISET: T32
    // Fields: opt=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF78F8000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || !matches!(exit.unwrap(), CpuExit::Continue),
        "expected UNDEFINED for T32 encoding"
    );
}

/// Provenance: aarch32_DCPS_T1_A
/// ASL: `Unconditional UNDEFINED`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNDEFINED" }
/// triggers Undefined
#[test]
fn test_aarch32_dcps_t1_a_invalid_1_8000_f78f8000() {
    // Thumb encoding (32): 0xF78F8000
    // Test aarch32_DCPS_T1_A invalid encoding: Unconditional UNDEFINED
    // ISET: T32
    // Fields: opt=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF78F8000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || !matches!(exit.unwrap(), CpuExit::Continue),
        "expected UNDEFINED for T32 encoding"
    );
}
