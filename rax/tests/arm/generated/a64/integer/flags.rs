//! A64 integer flags tests.
//!
//! Auto-generated from ARM ASL specifications.
//! DO NOT EDIT MANUALLY.

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::generated::test_helpers::*;

// ============================================================================
// aarch64_integer_flags_setf Tests
// ============================================================================

/// Provenance: aarch64_integer_flags_setf
/// ASL: `field sf 31 +: 1`
/// Requirement: FieldBoundary { field: "sf", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_integer_flags_setf_field_sf_0_min_80d_3a00080d() {
    // Encoding: 0x3A00080D
    // Test aarch64_integer_flags_setf field sf = 0 (Min)
    // Fields: sf=0, sz=0, Rn=0
    let encoding: u32 = 0x3A00080D;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_flags_setf
/// ASL: `field sf 31 +: 1`
/// Requirement: FieldBoundary { field: "sf", value: 1, boundary: Max }
/// 16-bit / halfword size
#[test]
fn test_aarch64_integer_flags_setf_field_sf_1_max_80d_ba00080d() {
    // Encoding: 0xBA00080D
    // Test aarch64_integer_flags_setf field sf = 1 (Max)
    // Fields: Rn=0, sf=1, sz=0
    let encoding: u32 = 0xBA00080D;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_flags_setf
/// ASL: `field sz 14 +: 1`
/// Requirement: FieldBoundary { field: "sz", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_integer_flags_setf_field_sz_0_min_80d_3a00080d() {
    // Encoding: 0x3A00080D
    // Test aarch64_integer_flags_setf field sz = 0 (Min)
    // Fields: Rn=0, sf=0, sz=0
    let encoding: u32 = 0x3A00080D;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_flags_setf
/// ASL: `field sz 14 +: 1`
/// Requirement: FieldBoundary { field: "sz", value: 1, boundary: Max }
/// 16-bit / halfword size
#[test]
fn test_aarch64_integer_flags_setf_field_sz_1_max_80d_3a00480d() {
    // Encoding: 0x3A00480D
    // Test aarch64_integer_flags_setf field sz = 1 (Max)
    // Fields: Rn=0, sf=0, sz=1
    let encoding: u32 = 0x3A00480D;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_flags_setf
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_integer_flags_setf_field_rn_0_min_80d_3a00080d() {
    // Encoding: 0x3A00080D
    // Test aarch64_integer_flags_setf field Rn = 0 (Min)
    // Fields: sz=0, Rn=0, sf=0
    let encoding: u32 = 0x3A00080D;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_flags_setf
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_integer_flags_setf_field_rn_1_poweroftwo_80d_3a00082d() {
    // Encoding: 0x3A00082D
    // Test aarch64_integer_flags_setf field Rn = 1 (PowerOfTwo)
    // Fields: sz=0, sf=0, Rn=1
    let encoding: u32 = 0x3A00082D;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_flags_setf
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_integer_flags_setf_field_rn_30_poweroftwominusone_80d_3a000bcd() {
    // Encoding: 0x3A000BCD
    // Test aarch64_integer_flags_setf field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: sz=0, Rn=30, sf=0
    let encoding: u32 = 0x3A000BCD;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_flags_setf
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_integer_flags_setf_field_rn_31_max_80d_3a000bed() {
    // Encoding: 0x3A000BED
    // Test aarch64_integer_flags_setf field Rn = 31 (Max)
    // Fields: sf=0, sz=0, Rn=31
    let encoding: u32 = 0x3A000BED;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_flags_setf
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// sf=0 (8-bit / byte size)
#[test]
fn test_aarch64_integer_flags_setf_combo_0_80d_3a00080d() {
    // Encoding: 0x3A00080D
    // Test aarch64_integer_flags_setf field combination: sf=0, sz=0, Rn=0
    // Fields: sf=0, Rn=0, sz=0
    let encoding: u32 = 0x3A00080D;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_flags_setf
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// sf=1 (16-bit / halfword size)
#[test]
fn test_aarch64_integer_flags_setf_combo_1_80d_ba00080d() {
    // Encoding: 0xBA00080D
    // Test aarch64_integer_flags_setf field combination: sf=1, sz=0, Rn=0
    // Fields: Rn=0, sf=1, sz=0
    let encoding: u32 = 0xBA00080D;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_flags_setf
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// sz=0 (8-bit / byte size)
#[test]
fn test_aarch64_integer_flags_setf_combo_2_80d_3a00080d() {
    // Encoding: 0x3A00080D
    // Test aarch64_integer_flags_setf field combination: sf=0, sz=0, Rn=0
    // Fields: Rn=0, sf=0, sz=0
    let encoding: u32 = 0x3A00080D;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_flags_setf
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// sz=1 (16-bit / halfword size)
#[test]
fn test_aarch64_integer_flags_setf_combo_3_80d_3a00480d() {
    // Encoding: 0x3A00480D
    // Test aarch64_integer_flags_setf field combination: sf=0, sz=1, Rn=0
    // Fields: Rn=0, sf=0, sz=1
    let encoding: u32 = 0x3A00480D;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_flags_setf
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch64_integer_flags_setf_combo_4_80d_3a00080d() {
    // Encoding: 0x3A00080D
    // Test aarch64_integer_flags_setf field combination: sf=0, sz=0, Rn=0
    // Fields: sf=0, sz=0, Rn=0
    let encoding: u32 = 0x3A00080D;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_flags_setf
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_aarch64_integer_flags_setf_combo_5_80d_3a00082d() {
    // Encoding: 0x3A00082D
    // Test aarch64_integer_flags_setf field combination: sf=0, sz=0, Rn=1
    // Fields: sf=0, sz=0, Rn=1
    let encoding: u32 = 0x3A00082D;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_flags_setf
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_integer_flags_setf_combo_6_80d_3a000bcd() {
    // Encoding: 0x3A000BCD
    // Test aarch64_integer_flags_setf field combination: sf=0, sz=0, Rn=30
    // Fields: sf=0, sz=0, Rn=30
    let encoding: u32 = 0x3A000BCD;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_flags_setf
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_integer_flags_setf_combo_7_80d_3a000bed() {
    // Encoding: 0x3A000BED
    // Test aarch64_integer_flags_setf field combination: sf=0, sz=0, Rn=31
    // Fields: Rn=31, sz=0, sf=0
    let encoding: u32 = 0x3A000BED;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_flags_setf
/// ASL: `field sf = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "sf", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_integer_flags_setf_special_sf_0_size_variant_0_2061_3a00480d() {
    // Encoding: 0x3A00480D
    // Test aarch64_integer_flags_setf special value sf = 0 (Size variant 0)
    // Fields: Rn=0, sz=1, sf=0
    let encoding: u32 = 0x3A00480D;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_flags_setf
/// ASL: `field sf = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "sf", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_integer_flags_setf_special_sf_1_size_variant_1_2061_ba00480d() {
    // Encoding: 0xBA00480D
    // Test aarch64_integer_flags_setf special value sf = 1 (Size variant 1)
    // Fields: sz=1, sf=1, Rn=0
    let encoding: u32 = 0xBA00480D;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_flags_setf
/// ASL: `field sz = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "sz", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_integer_flags_setf_special_sz_0_size_variant_0_2061_3a00080d() {
    // Encoding: 0x3A00080D
    // Test aarch64_integer_flags_setf special value sz = 0 (Size variant 0)
    // Fields: sz=0, sf=0, Rn=0
    let encoding: u32 = 0x3A00080D;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_flags_setf
/// ASL: `field sz = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "sz", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_integer_flags_setf_special_sz_1_size_variant_1_2061_3a00480d() {
    // Encoding: 0x3A00480D
    // Test aarch64_integer_flags_setf special value sz = 1 (Size variant 1)
    // Fields: sz=1, sf=0, Rn=0
    let encoding: u32 = 0x3A00480D;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_flags_setf
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_integer_flags_setf_special_rn_31_stack_pointer_sp_may_require_alignment_2061_3a004bed()
 {
    // Encoding: 0x3A004BED
    // Test aarch64_integer_flags_setf special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: Rn=31, sf=0, sz=1
    let encoding: u32 = 0x3A004BED;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_flags_setf
/// ASL: `Rn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Rn = 31)
#[test]
fn test_aarch64_integer_flags_setf_sp_rn_3a000bed() {
    // Test aarch64_integer_flags_setf with Rn = SP (31)
    // Encoding: 0x3A000BED
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x3A000BED;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_integer_flags_setf
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 0 + 0 = 0 (Z=1)
#[test]
fn test_aarch64_integer_flags_setf_flags_zeroresult_0_ba00082d() {
    // Test aarch64_integer_flags_setf flag computation: ZeroResult
    // Encoding: 0xBA00082D
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0x0);
    set_x(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xBA00082D;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_flags_setf
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 1 + (-1) = 0 (Z=1, C=1)
#[test]
fn test_aarch64_integer_flags_setf_flags_zeroresult_1_ba00082d() {
    // Test aarch64_integer_flags_setf flag computation: ZeroResult
    // Encoding: 0xBA00082D
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x1);
    set_x(&mut cpu, 2, 0xFFFFFFFFFFFFFFFF);
    let encoding: u32 = 0xBA00082D;
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

/// Provenance: aarch64_integer_flags_setf
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: NegativeResult }
/// negative value (N=1)
#[test]
fn test_aarch64_integer_flags_setf_flags_negativeresult_2_ba00082d() {
    // Test aarch64_integer_flags_setf flag computation: NegativeResult
    // Encoding: 0xBA00082D
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0x0);
    set_x(&mut cpu, 1, 0x8000000000000000);
    let encoding: u32 = 0xBA00082D;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_flags_setf
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 1 = 0 (C=1, Z=1)
#[test]
fn test_aarch64_integer_flags_setf_flags_unsignedoverflow_3_ba00082d() {
    // Test aarch64_integer_flags_setf flag computation: UnsignedOverflow
    // Encoding: 0xBA00082D
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0xFFFFFFFFFFFFFFFF);
    set_x(&mut cpu, 2, 0x1);
    let encoding: u32 = 0xBA00082D;
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

/// Provenance: aarch64_integer_flags_setf
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 2 = 1 (C=1)
#[test]
fn test_aarch64_integer_flags_setf_flags_unsignedoverflow_4_ba00082d() {
    // Test aarch64_integer_flags_setf flag computation: UnsignedOverflow
    // Encoding: 0xBA00082D
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0x2);
    set_x(&mut cpu, 1, 0xFFFFFFFFFFFFFFFF);
    let encoding: u32 = 0xBA00082D;
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

/// Provenance: aarch64_integer_flags_setf
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// max_signed + 1 = min_signed (V=1, N=1)
#[test]
fn test_aarch64_integer_flags_setf_flags_signedoverflow_5_ba00082d() {
    // Test aarch64_integer_flags_setf flag computation: SignedOverflow
    // Encoding: 0xBA00082D
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0x1);
    set_x(&mut cpu, 1, 0x7FFFFFFFFFFFFFFF);
    let encoding: u32 = 0xBA00082D;
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

/// Provenance: aarch64_integer_flags_setf
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// min_signed + (-1) = max_signed (V=1)
#[test]
fn test_aarch64_integer_flags_setf_flags_signedoverflow_6_ba00082d() {
    // Test aarch64_integer_flags_setf flag computation: SignedOverflow
    // Encoding: 0xBA00082D
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0xFFFFFFFFFFFFFFFF);
    set_x(&mut cpu, 1, 0x8000000000000000);
    let encoding: u32 = 0xBA00082D;
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

/// Provenance: aarch64_integer_flags_setf
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: PositiveResult }
/// 100 + 50 = 150 (no flags)
#[test]
fn test_aarch64_integer_flags_setf_flags_positiveresult_7_ba00082d() {
    // Test aarch64_integer_flags_setf flag computation: PositiveResult
    // Encoding: 0xBA00082D
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0x32);
    set_x(&mut cpu, 1, 0x64);
    let encoding: u32 = 0xBA00082D;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

// ============================================================================
// aarch64_integer_flags_xaflag Tests
// ============================================================================

/// Provenance: aarch64_integer_flags_xaflag
/// ASL: `field CRm 8 +: 4`
/// Requirement: FieldBoundary { field: "CRm", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_integer_flags_xaflag_field_crm_0_min_403f_d500403f() {
    // Encoding: 0xD500403F
    // Test aarch64_integer_flags_xaflag field CRm = 0 (Min)
    // Fields: CRm=0
    let encoding: u32 = 0xD500403F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_flags_xaflag
/// ASL: `field CRm 8 +: 4`
/// Requirement: FieldBoundary { field: "CRm", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_aarch64_integer_flags_xaflag_field_crm_1_poweroftwo_403f_d500413f() {
    // Encoding: 0xD500413F
    // Test aarch64_integer_flags_xaflag field CRm = 1 (PowerOfTwo)
    // Fields: CRm=1
    let encoding: u32 = 0xD500413F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_flags_xaflag
/// ASL: `field CRm 8 +: 4`
/// Requirement: FieldBoundary { field: "CRm", value: 7, boundary: PowerOfTwoMinusOne }
/// midpoint (7)
#[test]
fn test_aarch64_integer_flags_xaflag_field_crm_7_poweroftwominusone_403f_d500473f() {
    // Encoding: 0xD500473F
    // Test aarch64_integer_flags_xaflag field CRm = 7 (PowerOfTwoMinusOne)
    // Fields: CRm=7
    let encoding: u32 = 0xD500473F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_flags_xaflag
/// ASL: `field CRm 8 +: 4`
/// Requirement: FieldBoundary { field: "CRm", value: 15, boundary: Max }
/// maximum value (15)
#[test]
fn test_aarch64_integer_flags_xaflag_field_crm_15_max_403f_d5004f3f() {
    // Encoding: 0xD5004F3F
    // Test aarch64_integer_flags_xaflag field CRm = 15 (Max)
    // Fields: CRm=15
    let encoding: u32 = 0xD5004F3F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_flags_xaflag
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// CRm=0 (minimum value)
#[test]
fn test_aarch64_integer_flags_xaflag_combo_0_403f_d500403f() {
    // Encoding: 0xD500403F
    // Test aarch64_integer_flags_xaflag field combination: CRm=0
    // Fields: CRm=0
    let encoding: u32 = 0xD500403F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_flags_xaflag
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// CRm=1 (value 1)
#[test]
fn test_aarch64_integer_flags_xaflag_combo_1_403f_d500413f() {
    // Encoding: 0xD500413F
    // Test aarch64_integer_flags_xaflag field combination: CRm=1
    // Fields: CRm=1
    let encoding: u32 = 0xD500413F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_flags_xaflag
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// CRm=7 (midpoint (7))
#[test]
fn test_aarch64_integer_flags_xaflag_combo_2_403f_d500473f() {
    // Encoding: 0xD500473F
    // Test aarch64_integer_flags_xaflag field combination: CRm=7
    // Fields: CRm=7
    let encoding: u32 = 0xD500473F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_flags_xaflag
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// CRm=15 (maximum value (15))
#[test]
fn test_aarch64_integer_flags_xaflag_combo_3_403f_d5004f3f() {
    // Encoding: 0xD5004F3F
    // Test aarch64_integer_flags_xaflag field combination: CRm=15
    // Fields: CRm=15
    let encoding: u32 = 0xD5004F3F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_flags_xaflag
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 0 + 0 = 0 (Z=1)
#[test]
fn test_aarch64_integer_flags_xaflag_flags_zeroresult_0_d500403f() {
    // Test aarch64_integer_flags_xaflag flag computation: ZeroResult
    // Encoding: 0xD500403F
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0x0);
    set_x(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xD500403F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_flags_xaflag
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 1 + (-1) = 0 (Z=1, C=1)
#[test]
fn test_aarch64_integer_flags_xaflag_flags_zeroresult_1_d500403f() {
    // Test aarch64_integer_flags_xaflag flag computation: ZeroResult
    // Encoding: 0xD500403F
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0xFFFFFFFFFFFFFFFF);
    set_x(&mut cpu, 1, 0x1);
    let encoding: u32 = 0xD500403F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_flags_xaflag
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: NegativeResult }
/// negative value (N=1)
#[test]
fn test_aarch64_integer_flags_xaflag_flags_negativeresult_2_d500403f() {
    // Test aarch64_integer_flags_xaflag flag computation: NegativeResult
    // Encoding: 0xD500403F
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0x0);
    set_x(&mut cpu, 1, 0x8000000000000000);
    let encoding: u32 = 0xD500403F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_flags_xaflag
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 1 = 0 (C=1, Z=1)
#[test]
fn test_aarch64_integer_flags_xaflag_flags_unsignedoverflow_3_d500403f() {
    // Test aarch64_integer_flags_xaflag flag computation: UnsignedOverflow
    // Encoding: 0xD500403F
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0x1);
    set_x(&mut cpu, 1, 0xFFFFFFFFFFFFFFFF);
    let encoding: u32 = 0xD500403F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_flags_xaflag
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 2 = 1 (C=1)
#[test]
fn test_aarch64_integer_flags_xaflag_flags_unsignedoverflow_4_d500403f() {
    // Test aarch64_integer_flags_xaflag flag computation: UnsignedOverflow
    // Encoding: 0xD500403F
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0x2);
    set_x(&mut cpu, 1, 0xFFFFFFFFFFFFFFFF);
    let encoding: u32 = 0xD500403F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_flags_xaflag
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// max_signed + 1 = min_signed (V=1, N=1)
#[test]
fn test_aarch64_integer_flags_xaflag_flags_signedoverflow_5_d500403f() {
    // Test aarch64_integer_flags_xaflag flag computation: SignedOverflow
    // Encoding: 0xD500403F
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0x1);
    set_x(&mut cpu, 1, 0x7FFFFFFFFFFFFFFF);
    let encoding: u32 = 0xD500403F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_flags_xaflag
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// min_signed + (-1) = max_signed (V=1)
#[test]
fn test_aarch64_integer_flags_xaflag_flags_signedoverflow_6_d500403f() {
    // Test aarch64_integer_flags_xaflag flag computation: SignedOverflow
    // Encoding: 0xD500403F
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x8000000000000000);
    set_x(&mut cpu, 2, 0xFFFFFFFFFFFFFFFF);
    let encoding: u32 = 0xD500403F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_flags_xaflag
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: PositiveResult }
/// 100 + 50 = 150 (no flags)
#[test]
fn test_aarch64_integer_flags_xaflag_flags_positiveresult_7_d500403f() {
    // Test aarch64_integer_flags_xaflag flag computation: PositiveResult
    // Encoding: 0xD500403F
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x64);
    set_x(&mut cpu, 2, 0x32);
    let encoding: u32 = 0xD500403F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
    assert_eq!(cpu.get_pstate().v, false, "V should be false");
}

// ============================================================================
// aarch64_integer_flags_cfinv Tests
// ============================================================================

/// Provenance: aarch64_integer_flags_cfinv
/// ASL: `field CRm 8 +: 4`
/// Requirement: FieldBoundary { field: "CRm", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_integer_flags_cfinv_field_crm_0_min_401f_d500401f() {
    // Encoding: 0xD500401F
    // Test aarch64_integer_flags_cfinv field CRm = 0 (Min)
    // Fields: CRm=0
    let encoding: u32 = 0xD500401F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_flags_cfinv
/// ASL: `field CRm 8 +: 4`
/// Requirement: FieldBoundary { field: "CRm", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_aarch64_integer_flags_cfinv_field_crm_1_poweroftwo_401f_d500411f() {
    // Encoding: 0xD500411F
    // Test aarch64_integer_flags_cfinv field CRm = 1 (PowerOfTwo)
    // Fields: CRm=1
    let encoding: u32 = 0xD500411F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_flags_cfinv
/// ASL: `field CRm 8 +: 4`
/// Requirement: FieldBoundary { field: "CRm", value: 7, boundary: PowerOfTwoMinusOne }
/// midpoint (7)
#[test]
fn test_aarch64_integer_flags_cfinv_field_crm_7_poweroftwominusone_401f_d500471f() {
    // Encoding: 0xD500471F
    // Test aarch64_integer_flags_cfinv field CRm = 7 (PowerOfTwoMinusOne)
    // Fields: CRm=7
    let encoding: u32 = 0xD500471F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_flags_cfinv
/// ASL: `field CRm 8 +: 4`
/// Requirement: FieldBoundary { field: "CRm", value: 15, boundary: Max }
/// maximum value (15)
#[test]
fn test_aarch64_integer_flags_cfinv_field_crm_15_max_401f_d5004f1f() {
    // Encoding: 0xD5004F1F
    // Test aarch64_integer_flags_cfinv field CRm = 15 (Max)
    // Fields: CRm=15
    let encoding: u32 = 0xD5004F1F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_flags_cfinv
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// CRm=0 (minimum value)
#[test]
fn test_aarch64_integer_flags_cfinv_combo_0_401f_d500401f() {
    // Encoding: 0xD500401F
    // Test aarch64_integer_flags_cfinv field combination: CRm=0
    // Fields: CRm=0
    let encoding: u32 = 0xD500401F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_flags_cfinv
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// CRm=1 (value 1)
#[test]
fn test_aarch64_integer_flags_cfinv_combo_1_401f_d500411f() {
    // Encoding: 0xD500411F
    // Test aarch64_integer_flags_cfinv field combination: CRm=1
    // Fields: CRm=1
    let encoding: u32 = 0xD500411F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_flags_cfinv
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// CRm=7 (midpoint (7))
#[test]
fn test_aarch64_integer_flags_cfinv_combo_2_401f_d500471f() {
    // Encoding: 0xD500471F
    // Test aarch64_integer_flags_cfinv field combination: CRm=7
    // Fields: CRm=7
    let encoding: u32 = 0xD500471F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_flags_cfinv
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// CRm=15 (maximum value (15))
#[test]
fn test_aarch64_integer_flags_cfinv_combo_3_401f_d5004f1f() {
    // Encoding: 0xD5004F1F
    // Test aarch64_integer_flags_cfinv field combination: CRm=15
    // Fields: CRm=15
    let encoding: u32 = 0xD5004F1F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_flags_cfinv
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 0 + 0 = 0 (Z=1)
#[test]
fn test_aarch64_integer_flags_cfinv_flags_zeroresult_0_d500401f() {
    // Test aarch64_integer_flags_cfinv flag computation: ZeroResult
    // Encoding: 0xD500401F
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x0);
    set_x(&mut cpu, 2, 0x0);
    let encoding: u32 = 0xD500401F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_flags_cfinv
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 1 + (-1) = 0 (Z=1, C=1)
#[test]
fn test_aarch64_integer_flags_cfinv_flags_zeroresult_1_d500401f() {
    // Test aarch64_integer_flags_cfinv flag computation: ZeroResult
    // Encoding: 0xD500401F
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x1);
    set_x(&mut cpu, 2, 0xFFFFFFFFFFFFFFFF);
    let encoding: u32 = 0xD500401F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
}

/// Provenance: aarch64_integer_flags_cfinv
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: NegativeResult }
/// negative value (N=1)
#[test]
fn test_aarch64_integer_flags_cfinv_flags_negativeresult_2_d500401f() {
    // Test aarch64_integer_flags_cfinv flag computation: NegativeResult
    // Encoding: 0xD500401F
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x8000000000000000);
    set_x(&mut cpu, 2, 0x0);
    let encoding: u32 = 0xD500401F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_flags_cfinv
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 1 = 0 (C=1, Z=1)
#[test]
fn test_aarch64_integer_flags_cfinv_flags_unsignedoverflow_3_d500401f() {
    // Test aarch64_integer_flags_cfinv flag computation: UnsignedOverflow
    // Encoding: 0xD500401F
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0xFFFFFFFFFFFFFFFF);
    set_x(&mut cpu, 2, 0x1);
    let encoding: u32 = 0xD500401F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
}

/// Provenance: aarch64_integer_flags_cfinv
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 2 = 1 (C=1)
#[test]
fn test_aarch64_integer_flags_cfinv_flags_unsignedoverflow_4_d500401f() {
    // Test aarch64_integer_flags_cfinv flag computation: UnsignedOverflow
    // Encoding: 0xD500401F
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0x2);
    set_x(&mut cpu, 1, 0xFFFFFFFFFFFFFFFF);
    let encoding: u32 = 0xD500401F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
}

/// Provenance: aarch64_integer_flags_cfinv
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// max_signed + 1 = min_signed (V=1, N=1)
#[test]
fn test_aarch64_integer_flags_cfinv_flags_signedoverflow_5_d500401f() {
    // Test aarch64_integer_flags_cfinv flag computation: SignedOverflow
    // Encoding: 0xD500401F
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0x1);
    set_x(&mut cpu, 1, 0x7FFFFFFFFFFFFFFF);
    let encoding: u32 = 0xD500401F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_flags_cfinv
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// min_signed + (-1) = max_signed (V=1)
#[test]
fn test_aarch64_integer_flags_cfinv_flags_signedoverflow_6_d500401f() {
    // Test aarch64_integer_flags_cfinv flag computation: SignedOverflow
    // Encoding: 0xD500401F
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x8000000000000000);
    set_x(&mut cpu, 2, 0xFFFFFFFFFFFFFFFF);
    let encoding: u32 = 0xD500401F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
}

/// Provenance: aarch64_integer_flags_cfinv
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: PositiveResult }
/// 100 + 50 = 150 (no flags)
#[test]
fn test_aarch64_integer_flags_cfinv_flags_positiveresult_7_d500401f() {
    // Test aarch64_integer_flags_cfinv flag computation: PositiveResult
    // Encoding: 0xD500401F
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x64);
    set_x(&mut cpu, 2, 0x32);
    let encoding: u32 = 0xD500401F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

// ============================================================================
// aarch64_integer_flags_axflag Tests
// ============================================================================

/// Provenance: aarch64_integer_flags_axflag
/// ASL: `field CRm 8 +: 4`
/// Requirement: FieldBoundary { field: "CRm", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_integer_flags_axflag_field_crm_0_min_405f_d500405f() {
    // Encoding: 0xD500405F
    // Test aarch64_integer_flags_axflag field CRm = 0 (Min)
    // Fields: CRm=0
    let encoding: u32 = 0xD500405F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_flags_axflag
/// ASL: `field CRm 8 +: 4`
/// Requirement: FieldBoundary { field: "CRm", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_aarch64_integer_flags_axflag_field_crm_1_poweroftwo_405f_d500415f() {
    // Encoding: 0xD500415F
    // Test aarch64_integer_flags_axflag field CRm = 1 (PowerOfTwo)
    // Fields: CRm=1
    let encoding: u32 = 0xD500415F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_flags_axflag
/// ASL: `field CRm 8 +: 4`
/// Requirement: FieldBoundary { field: "CRm", value: 7, boundary: PowerOfTwoMinusOne }
/// midpoint (7)
#[test]
fn test_aarch64_integer_flags_axflag_field_crm_7_poweroftwominusone_405f_d500475f() {
    // Encoding: 0xD500475F
    // Test aarch64_integer_flags_axflag field CRm = 7 (PowerOfTwoMinusOne)
    // Fields: CRm=7
    let encoding: u32 = 0xD500475F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_flags_axflag
/// ASL: `field CRm 8 +: 4`
/// Requirement: FieldBoundary { field: "CRm", value: 15, boundary: Max }
/// maximum value (15)
#[test]
fn test_aarch64_integer_flags_axflag_field_crm_15_max_405f_d5004f5f() {
    // Encoding: 0xD5004F5F
    // Test aarch64_integer_flags_axflag field CRm = 15 (Max)
    // Fields: CRm=15
    let encoding: u32 = 0xD5004F5F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_flags_axflag
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// CRm=0 (minimum value)
#[test]
fn test_aarch64_integer_flags_axflag_combo_0_405f_d500405f() {
    // Encoding: 0xD500405F
    // Test aarch64_integer_flags_axflag field combination: CRm=0
    // Fields: CRm=0
    let encoding: u32 = 0xD500405F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_flags_axflag
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// CRm=1 (value 1)
#[test]
fn test_aarch64_integer_flags_axflag_combo_1_405f_d500415f() {
    // Encoding: 0xD500415F
    // Test aarch64_integer_flags_axflag field combination: CRm=1
    // Fields: CRm=1
    let encoding: u32 = 0xD500415F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_flags_axflag
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// CRm=7 (midpoint (7))
#[test]
fn test_aarch64_integer_flags_axflag_combo_2_405f_d500475f() {
    // Encoding: 0xD500475F
    // Test aarch64_integer_flags_axflag field combination: CRm=7
    // Fields: CRm=7
    let encoding: u32 = 0xD500475F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_flags_axflag
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// CRm=15 (maximum value (15))
#[test]
fn test_aarch64_integer_flags_axflag_combo_3_405f_d5004f5f() {
    // Encoding: 0xD5004F5F
    // Test aarch64_integer_flags_axflag field combination: CRm=15
    // Fields: CRm=15
    let encoding: u32 = 0xD5004F5F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_flags_axflag
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 0 + 0 = 0 (Z=1)
#[test]
fn test_aarch64_integer_flags_axflag_flags_zeroresult_0_d500405f() {
    // Test aarch64_integer_flags_axflag flag computation: ZeroResult
    // Encoding: 0xD500405F
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x0);
    set_x(&mut cpu, 2, 0x0);
    let encoding: u32 = 0xD500405F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_flags_axflag
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 1 + (-1) = 0 (Z=1, C=1)
#[test]
fn test_aarch64_integer_flags_axflag_flags_zeroresult_1_d500405f() {
    // Test aarch64_integer_flags_axflag flag computation: ZeroResult
    // Encoding: 0xD500405F
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x1);
    set_x(&mut cpu, 2, 0xFFFFFFFFFFFFFFFF);
    let encoding: u32 = 0xD500405F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_flags_axflag
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: NegativeResult }
/// negative value (N=1)
#[test]
fn test_aarch64_integer_flags_axflag_flags_negativeresult_2_d500405f() {
    // Test aarch64_integer_flags_axflag flag computation: NegativeResult
    // Encoding: 0xD500405F
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0x0);
    set_x(&mut cpu, 1, 0x8000000000000000);
    let encoding: u32 = 0xD500405F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_flags_axflag
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 1 = 0 (C=1, Z=1)
#[test]
fn test_aarch64_integer_flags_axflag_flags_unsignedoverflow_3_d500405f() {
    // Test aarch64_integer_flags_axflag flag computation: UnsignedOverflow
    // Encoding: 0xD500405F
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0xFFFFFFFFFFFFFFFF);
    set_x(&mut cpu, 2, 0x1);
    let encoding: u32 = 0xD500405F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_flags_axflag
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 2 = 1 (C=1)
#[test]
fn test_aarch64_integer_flags_axflag_flags_unsignedoverflow_4_d500405f() {
    // Test aarch64_integer_flags_axflag flag computation: UnsignedOverflow
    // Encoding: 0xD500405F
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0xFFFFFFFFFFFFFFFF);
    set_x(&mut cpu, 2, 0x2);
    let encoding: u32 = 0xD500405F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_flags_axflag
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// max_signed + 1 = min_signed (V=1, N=1)
#[test]
fn test_aarch64_integer_flags_axflag_flags_signedoverflow_5_d500405f() {
    // Test aarch64_integer_flags_axflag flag computation: SignedOverflow
    // Encoding: 0xD500405F
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0x1);
    set_x(&mut cpu, 1, 0x7FFFFFFFFFFFFFFF);
    let encoding: u32 = 0xD500405F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_flags_axflag
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// min_signed + (-1) = max_signed (V=1)
#[test]
fn test_aarch64_integer_flags_axflag_flags_signedoverflow_6_d500405f() {
    // Test aarch64_integer_flags_axflag flag computation: SignedOverflow
    // Encoding: 0xD500405F
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x8000000000000000);
    set_x(&mut cpu, 2, 0xFFFFFFFFFFFFFFFF);
    let encoding: u32 = 0xD500405F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_flags_axflag
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: PositiveResult }
/// 100 + 50 = 150 (no flags)
#[test]
fn test_aarch64_integer_flags_axflag_flags_positiveresult_7_d500405f() {
    // Test aarch64_integer_flags_axflag flag computation: PositiveResult
    // Encoding: 0xD500405F
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0x32);
    set_x(&mut cpu, 1, 0x64);
    let encoding: u32 = 0xD500405F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
    assert_eq!(cpu.get_pstate().v, false, "V should be false");
}

// ============================================================================
// aarch64_integer_flags_rmif Tests
// ============================================================================

/// Provenance: aarch64_integer_flags_rmif
/// ASL: `field sf 31 +: 1`
/// Requirement: FieldBoundary { field: "sf", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_integer_flags_rmif_field_sf_0_min_400_3a000400() {
    // Encoding: 0x3A000400
    // Test aarch64_integer_flags_rmif field sf = 0 (Min)
    // Fields: imm6=0, Rn=0, sf=0, mask=0
    let encoding: u32 = 0x3A000400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_flags_rmif
/// ASL: `field sf 31 +: 1`
/// Requirement: FieldBoundary { field: "sf", value: 1, boundary: Max }
/// 16-bit / halfword size
#[test]
fn test_aarch64_integer_flags_rmif_field_sf_1_max_400_ba000400() {
    // Encoding: 0xBA000400
    // Test aarch64_integer_flags_rmif field sf = 1 (Max)
    // Fields: imm6=0, Rn=0, sf=1, mask=0
    let encoding: u32 = 0xBA000400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_flags_rmif
/// ASL: `field imm6 15 +: 6`
/// Requirement: FieldBoundary { field: "imm6", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch64_integer_flags_rmif_field_imm6_0_zero_400_3a000400() {
    // Encoding: 0x3A000400
    // Test aarch64_integer_flags_rmif field imm6 = 0 (Zero)
    // Fields: mask=0, imm6=0, Rn=0, sf=0
    let encoding: u32 = 0x3A000400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_flags_rmif
/// ASL: `field imm6 15 +: 6`
/// Requirement: FieldBoundary { field: "imm6", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch64_integer_flags_rmif_field_imm6_1_poweroftwo_400_3a008400() {
    // Encoding: 0x3A008400
    // Test aarch64_integer_flags_rmif field imm6 = 1 (PowerOfTwo)
    // Fields: mask=0, sf=0, imm6=1, Rn=0
    let encoding: u32 = 0x3A008400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_flags_rmif
/// ASL: `field imm6 15 +: 6`
/// Requirement: FieldBoundary { field: "imm6", value: 3, boundary: PowerOfTwoMinusOne }
/// 2^2 - 1 = 3
#[test]
fn test_aarch64_integer_flags_rmif_field_imm6_3_poweroftwominusone_400_3a018400() {
    // Encoding: 0x3A018400
    // Test aarch64_integer_flags_rmif field imm6 = 3 (PowerOfTwoMinusOne)
    // Fields: sf=0, mask=0, Rn=0, imm6=3
    let encoding: u32 = 0x3A018400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_flags_rmif
/// ASL: `field imm6 15 +: 6`
/// Requirement: FieldBoundary { field: "imm6", value: 4, boundary: PowerOfTwo }
/// power of 2 (2^2 = 4)
#[test]
fn test_aarch64_integer_flags_rmif_field_imm6_4_poweroftwo_400_3a020400() {
    // Encoding: 0x3A020400
    // Test aarch64_integer_flags_rmif field imm6 = 4 (PowerOfTwo)
    // Fields: imm6=4, mask=0, Rn=0, sf=0
    let encoding: u32 = 0x3A020400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_flags_rmif
/// ASL: `field imm6 15 +: 6`
/// Requirement: FieldBoundary { field: "imm6", value: 7, boundary: PowerOfTwoMinusOne }
/// 2^3 - 1 = 7
#[test]
fn test_aarch64_integer_flags_rmif_field_imm6_7_poweroftwominusone_400_3a038400() {
    // Encoding: 0x3A038400
    // Test aarch64_integer_flags_rmif field imm6 = 7 (PowerOfTwoMinusOne)
    // Fields: imm6=7, Rn=0, mask=0, sf=0
    let encoding: u32 = 0x3A038400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_flags_rmif
/// ASL: `field imm6 15 +: 6`
/// Requirement: FieldBoundary { field: "imm6", value: 8, boundary: PowerOfTwo }
/// power of 2 (2^3 = 8)
#[test]
fn test_aarch64_integer_flags_rmif_field_imm6_8_poweroftwo_400_3a040400() {
    // Encoding: 0x3A040400
    // Test aarch64_integer_flags_rmif field imm6 = 8 (PowerOfTwo)
    // Fields: sf=0, imm6=8, mask=0, Rn=0
    let encoding: u32 = 0x3A040400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_flags_rmif
/// ASL: `field imm6 15 +: 6`
/// Requirement: FieldBoundary { field: "imm6", value: 15, boundary: PowerOfTwoMinusOne }
/// 2^4 - 1 = 15
#[test]
fn test_aarch64_integer_flags_rmif_field_imm6_15_poweroftwominusone_400_3a078400() {
    // Encoding: 0x3A078400
    // Test aarch64_integer_flags_rmif field imm6 = 15 (PowerOfTwoMinusOne)
    // Fields: Rn=0, sf=0, mask=0, imm6=15
    let encoding: u32 = 0x3A078400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_flags_rmif
/// ASL: `field imm6 15 +: 6`
/// Requirement: FieldBoundary { field: "imm6", value: 16, boundary: PowerOfTwo }
/// power of 2 (2^4 = 16)
#[test]
fn test_aarch64_integer_flags_rmif_field_imm6_16_poweroftwo_400_3a080400() {
    // Encoding: 0x3A080400
    // Test aarch64_integer_flags_rmif field imm6 = 16 (PowerOfTwo)
    // Fields: imm6=16, sf=0, Rn=0, mask=0
    let encoding: u32 = 0x3A080400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_flags_rmif
/// ASL: `field imm6 15 +: 6`
/// Requirement: FieldBoundary { field: "imm6", value: 31, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (31)
#[test]
fn test_aarch64_integer_flags_rmif_field_imm6_31_poweroftwominusone_400_3a0f8400() {
    // Encoding: 0x3A0F8400
    // Test aarch64_integer_flags_rmif field imm6 = 31 (PowerOfTwoMinusOne)
    // Fields: mask=0, imm6=31, sf=0, Rn=0
    let encoding: u32 = 0x3A0F8400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_flags_rmif
/// ASL: `field imm6 15 +: 6`
/// Requirement: FieldBoundary { field: "imm6", value: 32, boundary: PowerOfTwo }
/// power of 2 (2^5 = 32)
#[test]
fn test_aarch64_integer_flags_rmif_field_imm6_32_poweroftwo_400_3a100400() {
    // Encoding: 0x3A100400
    // Test aarch64_integer_flags_rmif field imm6 = 32 (PowerOfTwo)
    // Fields: sf=0, Rn=0, imm6=32, mask=0
    let encoding: u32 = 0x3A100400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_flags_rmif
/// ASL: `field imm6 15 +: 6`
/// Requirement: FieldBoundary { field: "imm6", value: 63, boundary: Max }
/// maximum immediate (63)
#[test]
fn test_aarch64_integer_flags_rmif_field_imm6_63_max_400_3a1f8400() {
    // Encoding: 0x3A1F8400
    // Test aarch64_integer_flags_rmif field imm6 = 63 (Max)
    // Fields: sf=0, Rn=0, mask=0, imm6=63
    let encoding: u32 = 0x3A1F8400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_flags_rmif
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_integer_flags_rmif_field_rn_0_min_400_3a000400() {
    // Encoding: 0x3A000400
    // Test aarch64_integer_flags_rmif field Rn = 0 (Min)
    // Fields: imm6=0, sf=0, Rn=0, mask=0
    let encoding: u32 = 0x3A000400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_flags_rmif
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_integer_flags_rmif_field_rn_1_poweroftwo_400_3a000420() {
    // Encoding: 0x3A000420
    // Test aarch64_integer_flags_rmif field Rn = 1 (PowerOfTwo)
    // Fields: imm6=0, Rn=1, mask=0, sf=0
    let encoding: u32 = 0x3A000420;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_flags_rmif
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_integer_flags_rmif_field_rn_30_poweroftwominusone_400_3a0007c0() {
    // Encoding: 0x3A0007C0
    // Test aarch64_integer_flags_rmif field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: sf=0, mask=0, Rn=30, imm6=0
    let encoding: u32 = 0x3A0007C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_flags_rmif
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_integer_flags_rmif_field_rn_31_max_400_3a0007e0() {
    // Encoding: 0x3A0007E0
    // Test aarch64_integer_flags_rmif field Rn = 31 (Max)
    // Fields: sf=0, imm6=0, mask=0, Rn=31
    let encoding: u32 = 0x3A0007E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_flags_rmif
/// ASL: `field mask 0 +: 4`
/// Requirement: FieldBoundary { field: "mask", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_integer_flags_rmif_field_mask_0_min_400_3a000400() {
    // Encoding: 0x3A000400
    // Test aarch64_integer_flags_rmif field mask = 0 (Min)
    // Fields: sf=0, mask=0, imm6=0, Rn=0
    let encoding: u32 = 0x3A000400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_flags_rmif
/// ASL: `field mask 0 +: 4`
/// Requirement: FieldBoundary { field: "mask", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_aarch64_integer_flags_rmif_field_mask_1_poweroftwo_400_3a000401() {
    // Encoding: 0x3A000401
    // Test aarch64_integer_flags_rmif field mask = 1 (PowerOfTwo)
    // Fields: sf=0, imm6=0, Rn=0, mask=1
    let encoding: u32 = 0x3A000401;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_flags_rmif
/// ASL: `field mask 0 +: 4`
/// Requirement: FieldBoundary { field: "mask", value: 7, boundary: PowerOfTwoMinusOne }
/// midpoint (7)
#[test]
fn test_aarch64_integer_flags_rmif_field_mask_7_poweroftwominusone_400_3a000407() {
    // Encoding: 0x3A000407
    // Test aarch64_integer_flags_rmif field mask = 7 (PowerOfTwoMinusOne)
    // Fields: sf=0, imm6=0, mask=7, Rn=0
    let encoding: u32 = 0x3A000407;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_flags_rmif
/// ASL: `field mask 0 +: 4`
/// Requirement: FieldBoundary { field: "mask", value: 15, boundary: Max }
/// maximum value (15)
#[test]
fn test_aarch64_integer_flags_rmif_field_mask_15_max_400_3a00040f() {
    // Encoding: 0x3A00040F
    // Test aarch64_integer_flags_rmif field mask = 15 (Max)
    // Fields: sf=0, mask=15, Rn=0, imm6=0
    let encoding: u32 = 0x3A00040F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_flags_rmif
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// sf=0 (8-bit / byte size)
#[test]
fn test_aarch64_integer_flags_rmif_combo_0_400_3a000400() {
    // Encoding: 0x3A000400
    // Test aarch64_integer_flags_rmif field combination: sf=0, imm6=0, Rn=0, mask=0
    // Fields: mask=0, sf=0, imm6=0, Rn=0
    let encoding: u32 = 0x3A000400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_flags_rmif
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// sf=1 (16-bit / halfword size)
#[test]
fn test_aarch64_integer_flags_rmif_combo_1_400_ba000400() {
    // Encoding: 0xBA000400
    // Test aarch64_integer_flags_rmif field combination: sf=1, imm6=0, Rn=0, mask=0
    // Fields: Rn=0, mask=0, sf=1, imm6=0
    let encoding: u32 = 0xBA000400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_flags_rmif
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm6=0 (immediate value 0)
#[test]
fn test_aarch64_integer_flags_rmif_combo_2_400_3a000400() {
    // Encoding: 0x3A000400
    // Test aarch64_integer_flags_rmif field combination: sf=0, imm6=0, Rn=0, mask=0
    // Fields: imm6=0, sf=0, mask=0, Rn=0
    let encoding: u32 = 0x3A000400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_flags_rmif
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm6=1 (immediate value 1)
#[test]
fn test_aarch64_integer_flags_rmif_combo_3_400_3a008400() {
    // Encoding: 0x3A008400
    // Test aarch64_integer_flags_rmif field combination: sf=0, imm6=1, Rn=0, mask=0
    // Fields: sf=0, Rn=0, mask=0, imm6=1
    let encoding: u32 = 0x3A008400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_flags_rmif
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm6=3 (2^2 - 1 = 3)
#[test]
fn test_aarch64_integer_flags_rmif_combo_4_400_3a018400() {
    // Encoding: 0x3A018400
    // Test aarch64_integer_flags_rmif field combination: sf=0, imm6=3, Rn=0, mask=0
    // Fields: imm6=3, Rn=0, sf=0, mask=0
    let encoding: u32 = 0x3A018400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_flags_rmif
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm6=4 (power of 2 (2^2 = 4))
#[test]
fn test_aarch64_integer_flags_rmif_combo_5_400_3a020400() {
    // Encoding: 0x3A020400
    // Test aarch64_integer_flags_rmif field combination: sf=0, imm6=4, Rn=0, mask=0
    // Fields: Rn=0, mask=0, sf=0, imm6=4
    let encoding: u32 = 0x3A020400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_flags_rmif
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm6=7 (2^3 - 1 = 7)
#[test]
fn test_aarch64_integer_flags_rmif_combo_6_400_3a038400() {
    // Encoding: 0x3A038400
    // Test aarch64_integer_flags_rmif field combination: sf=0, imm6=7, Rn=0, mask=0
    // Fields: mask=0, sf=0, imm6=7, Rn=0
    let encoding: u32 = 0x3A038400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_flags_rmif
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm6=8 (power of 2 (2^3 = 8))
#[test]
fn test_aarch64_integer_flags_rmif_combo_7_400_3a040400() {
    // Encoding: 0x3A040400
    // Test aarch64_integer_flags_rmif field combination: sf=0, imm6=8, Rn=0, mask=0
    // Fields: mask=0, sf=0, Rn=0, imm6=8
    let encoding: u32 = 0x3A040400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_flags_rmif
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm6=15 (2^4 - 1 = 15)
#[test]
fn test_aarch64_integer_flags_rmif_combo_8_400_3a078400() {
    // Encoding: 0x3A078400
    // Test aarch64_integer_flags_rmif field combination: sf=0, imm6=15, Rn=0, mask=0
    // Fields: Rn=0, sf=0, imm6=15, mask=0
    let encoding: u32 = 0x3A078400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_flags_rmif
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm6=16 (power of 2 (2^4 = 16))
#[test]
fn test_aarch64_integer_flags_rmif_combo_9_400_3a080400() {
    // Encoding: 0x3A080400
    // Test aarch64_integer_flags_rmif field combination: sf=0, imm6=16, Rn=0, mask=0
    // Fields: Rn=0, sf=0, imm6=16, mask=0
    let encoding: u32 = 0x3A080400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_flags_rmif
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm6=31 (immediate midpoint (31))
#[test]
fn test_aarch64_integer_flags_rmif_combo_10_400_3a0f8400() {
    // Encoding: 0x3A0F8400
    // Test aarch64_integer_flags_rmif field combination: sf=0, imm6=31, Rn=0, mask=0
    // Fields: mask=0, imm6=31, sf=0, Rn=0
    let encoding: u32 = 0x3A0F8400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_flags_rmif
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm6=32 (power of 2 (2^5 = 32))
#[test]
fn test_aarch64_integer_flags_rmif_combo_11_400_3a100400() {
    // Encoding: 0x3A100400
    // Test aarch64_integer_flags_rmif field combination: sf=0, imm6=32, Rn=0, mask=0
    // Fields: imm6=32, sf=0, mask=0, Rn=0
    let encoding: u32 = 0x3A100400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_flags_rmif
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm6=63 (maximum immediate (63))
#[test]
fn test_aarch64_integer_flags_rmif_combo_12_400_3a1f8400() {
    // Encoding: 0x3A1F8400
    // Test aarch64_integer_flags_rmif field combination: sf=0, imm6=63, Rn=0, mask=0
    // Fields: sf=0, imm6=63, Rn=0, mask=0
    let encoding: u32 = 0x3A1F8400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_flags_rmif
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch64_integer_flags_rmif_combo_13_400_3a000400() {
    // Encoding: 0x3A000400
    // Test aarch64_integer_flags_rmif field combination: sf=0, imm6=0, Rn=0, mask=0
    // Fields: sf=0, Rn=0, imm6=0, mask=0
    let encoding: u32 = 0x3A000400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_flags_rmif
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_aarch64_integer_flags_rmif_combo_14_400_3a000420() {
    // Encoding: 0x3A000420
    // Test aarch64_integer_flags_rmif field combination: sf=0, imm6=0, Rn=1, mask=0
    // Fields: Rn=1, imm6=0, sf=0, mask=0
    let encoding: u32 = 0x3A000420;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_flags_rmif
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_integer_flags_rmif_combo_15_400_3a0007c0() {
    // Encoding: 0x3A0007C0
    // Test aarch64_integer_flags_rmif field combination: sf=0, imm6=0, Rn=30, mask=0
    // Fields: imm6=0, Rn=30, mask=0, sf=0
    let encoding: u32 = 0x3A0007C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_flags_rmif
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_integer_flags_rmif_combo_16_400_3a0007e0() {
    // Encoding: 0x3A0007E0
    // Test aarch64_integer_flags_rmif field combination: sf=0, imm6=0, Rn=31, mask=0
    // Fields: imm6=0, Rn=31, sf=0, mask=0
    let encoding: u32 = 0x3A0007E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_flags_rmif
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// mask=0 (minimum value)
#[test]
fn test_aarch64_integer_flags_rmif_combo_17_400_3a000400() {
    // Encoding: 0x3A000400
    // Test aarch64_integer_flags_rmif field combination: sf=0, imm6=0, Rn=0, mask=0
    // Fields: Rn=0, imm6=0, sf=0, mask=0
    let encoding: u32 = 0x3A000400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_flags_rmif
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// mask=1 (value 1)
#[test]
fn test_aarch64_integer_flags_rmif_combo_18_400_3a000401() {
    // Encoding: 0x3A000401
    // Test aarch64_integer_flags_rmif field combination: sf=0, imm6=0, Rn=0, mask=1
    // Fields: sf=0, mask=1, Rn=0, imm6=0
    let encoding: u32 = 0x3A000401;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_flags_rmif
/// ASL: `field combination 19`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// mask=7 (midpoint (7))
#[test]
fn test_aarch64_integer_flags_rmif_combo_19_400_3a000407() {
    // Encoding: 0x3A000407
    // Test aarch64_integer_flags_rmif field combination: sf=0, imm6=0, Rn=0, mask=7
    // Fields: imm6=0, sf=0, Rn=0, mask=7
    let encoding: u32 = 0x3A000407;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_flags_rmif
/// ASL: `field combination 20`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// mask=15 (maximum value (15))
#[test]
fn test_aarch64_integer_flags_rmif_combo_20_400_3a00040f() {
    // Encoding: 0x3A00040F
    // Test aarch64_integer_flags_rmif field combination: sf=0, imm6=0, Rn=0, mask=15
    // Fields: mask=15, Rn=0, sf=0, imm6=0
    let encoding: u32 = 0x3A00040F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_flags_rmif
/// ASL: `field sf = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "sf", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_integer_flags_rmif_special_sf_0_size_variant_0_1024_3a008400() {
    // Encoding: 0x3A008400
    // Test aarch64_integer_flags_rmif special value sf = 0 (Size variant 0)
    // Fields: imm6=1, Rn=0, sf=0, mask=0
    let encoding: u32 = 0x3A008400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_flags_rmif
/// ASL: `field sf = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "sf", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_integer_flags_rmif_special_sf_1_size_variant_1_1024_ba008400() {
    // Encoding: 0xBA008400
    // Test aarch64_integer_flags_rmif special value sf = 1 (Size variant 1)
    // Fields: Rn=0, mask=0, sf=1, imm6=1
    let encoding: u32 = 0xBA008400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_flags_rmif
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_integer_flags_rmif_special_rn_31_stack_pointer_sp_may_require_alignment_1024_3a0087e0()
 {
    // Encoding: 0x3A0087E0
    // Test aarch64_integer_flags_rmif special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: imm6=1, sf=0, mask=0, Rn=31
    let encoding: u32 = 0x3A0087E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_flags_rmif
/// ASL: `Rn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Rn = 31)
#[test]
fn test_aarch64_integer_flags_rmif_sp_rn_3a0007e0() {
    // Test aarch64_integer_flags_rmif with Rn = SP (31)
    // Encoding: 0x3A0007E0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x3A0007E0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_flags_rmif
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 0 + 0 = 0 (Z=1)
#[test]
fn test_aarch64_integer_flags_rmif_flags_zeroresult_0_ba000420() {
    // Test aarch64_integer_flags_rmif flag computation: ZeroResult
    // Encoding: 0xBA000420
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x0);
    set_x(&mut cpu, 2, 0x0);
    let encoding: u32 = 0xBA000420;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_integer_flags_rmif
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 1 + (-1) = 0 (Z=1, C=1)
#[test]
fn test_aarch64_integer_flags_rmif_flags_zeroresult_1_ba000420() {
    // Test aarch64_integer_flags_rmif flag computation: ZeroResult
    // Encoding: 0xBA000420
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0xFFFFFFFFFFFFFFFF);
    set_x(&mut cpu, 1, 0x1);
    let encoding: u32 = 0xBA000420;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_flags_rmif
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: NegativeResult }
/// negative value (N=1)
#[test]
fn test_aarch64_integer_flags_rmif_flags_negativeresult_2_ba000420() {
    // Test aarch64_integer_flags_rmif flag computation: NegativeResult
    // Encoding: 0xBA000420
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0x0);
    set_x(&mut cpu, 1, 0x8000000000000000);
    let encoding: u32 = 0xBA000420;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_integer_flags_rmif
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 1 = 0 (C=1, Z=1)
#[test]
fn test_aarch64_integer_flags_rmif_flags_unsignedoverflow_3_ba000420() {
    // Test aarch64_integer_flags_rmif flag computation: UnsignedOverflow
    // Encoding: 0xBA000420
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0x1);
    set_x(&mut cpu, 1, 0xFFFFFFFFFFFFFFFF);
    let encoding: u32 = 0xBA000420;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_flags_rmif
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 2 = 1 (C=1)
#[test]
fn test_aarch64_integer_flags_rmif_flags_unsignedoverflow_4_ba000420() {
    // Test aarch64_integer_flags_rmif flag computation: UnsignedOverflow
    // Encoding: 0xBA000420
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0xFFFFFFFFFFFFFFFF);
    set_x(&mut cpu, 2, 0x2);
    let encoding: u32 = 0xBA000420;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_flags_rmif
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// max_signed + 1 = min_signed (V=1, N=1)
#[test]
fn test_aarch64_integer_flags_rmif_flags_signedoverflow_5_ba000420() {
    // Test aarch64_integer_flags_rmif flag computation: SignedOverflow
    // Encoding: 0xBA000420
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x7FFFFFFFFFFFFFFF);
    set_x(&mut cpu, 2, 0x1);
    let encoding: u32 = 0xBA000420;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_flags_rmif
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// min_signed + (-1) = max_signed (V=1)
#[test]
fn test_aarch64_integer_flags_rmif_flags_signedoverflow_6_ba000420() {
    // Test aarch64_integer_flags_rmif flag computation: SignedOverflow
    // Encoding: 0xBA000420
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0xFFFFFFFFFFFFFFFF);
    set_x(&mut cpu, 1, 0x8000000000000000);
    let encoding: u32 = 0xBA000420;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_flags_rmif
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: PositiveResult }
/// 100 + 50 = 150 (no flags)
#[test]
fn test_aarch64_integer_flags_rmif_flags_positiveresult_7_ba000420() {
    // Test aarch64_integer_flags_rmif flag computation: PositiveResult
    // Encoding: 0xBA000420
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x64);
    set_x(&mut cpu, 2, 0x32);
    let encoding: u32 = 0xBA000420;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
    assert_eq!(cpu.get_pstate().v, false, "V should be false");
}
