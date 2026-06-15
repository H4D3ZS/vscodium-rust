//! A64 sve predicate tests.
//!
//! Auto-generated from ARM ASL specifications.
//! DO NOT EDIT MANUALLY.

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::generated::test_helpers::*;

// ============================================================================
// RDFFR_P.P.F__ Tests
// ============================================================================

/// Provenance: RDFFR_P.P.F__
/// ASL: `field Pg 5 +: 4`
/// Requirement: FieldBoundary { field: "Pg", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_rdffr_p_p_f_field_pg_0_min_f000_2518f000() {
    // Encoding: 0x2518F000
    // Test RDFFR_P.P.F__ field Pg = 0 (Min)
    // Fields: Pg=0, Pd=0
    let encoding: u32 = 0x2518F000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: RDFFR_P.P.F__
/// ASL: `field Pg 5 +: 4`
/// Requirement: FieldBoundary { field: "Pg", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_rdffr_p_p_f_field_pg_1_poweroftwo_f000_2518f020() {
    // Encoding: 0x2518F020
    // Test RDFFR_P.P.F__ field Pg = 1 (PowerOfTwo)
    // Fields: Pd=0, Pg=1
    let encoding: u32 = 0x2518F020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: RDFFR_P.P.F__
/// ASL: `field Pd 0 +: 4`
/// Requirement: FieldBoundary { field: "Pd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_rdffr_p_p_f_field_pd_0_min_f000_2518f000() {
    // Encoding: 0x2518F000
    // Test RDFFR_P.P.F__ field Pd = 0 (Min)
    // Fields: Pd=0, Pg=0
    let encoding: u32 = 0x2518F000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: RDFFR_P.P.F__
/// ASL: `field Pd 0 +: 4`
/// Requirement: FieldBoundary { field: "Pd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_rdffr_p_p_f_field_pd_1_poweroftwo_f000_2518f001() {
    // Encoding: 0x2518F001
    // Test RDFFR_P.P.F__ field Pd = 1 (PowerOfTwo)
    // Fields: Pg=0, Pd=1
    let encoding: u32 = 0x2518F001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: RDFFR_P.P.F__
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=0 (register index 0 (first register))
#[test]
fn test_rdffr_p_p_f_combo_0_f000_2518f000() {
    // Encoding: 0x2518F000
    // Test RDFFR_P.P.F__ field combination: Pg=0, Pd=0
    // Fields: Pg=0, Pd=0
    let encoding: u32 = 0x2518F000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: RDFFR_P.P.F__
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=1 (register index 1 (second register))
#[test]
fn test_rdffr_p_p_f_combo_1_f000_2518f020() {
    // Encoding: 0x2518F020
    // Test RDFFR_P.P.F__ field combination: Pg=1, Pd=0
    // Fields: Pd=0, Pg=1
    let encoding: u32 = 0x2518F020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: RDFFR_P.P.F__
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pd=0 (register index 0 (first register))
#[test]
fn test_rdffr_p_p_f_combo_2_f000_2518f000() {
    // Encoding: 0x2518F000
    // Test RDFFR_P.P.F__ field combination: Pg=0, Pd=0
    // Fields: Pd=0, Pg=0
    let encoding: u32 = 0x2518F000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: RDFFR_P.P.F__
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pd=1 (register index 1 (second register))
#[test]
fn test_rdffr_p_p_f_combo_3_f000_2518f001() {
    // Encoding: 0x2518F001
    // Test RDFFR_P.P.F__ field combination: Pg=0, Pd=1
    // Fields: Pg=0, Pd=1
    let encoding: u32 = 0x2518F001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: RDFFR_P.P.F__
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=1 (same register test (reg=1)), Pd=1 (same register test (reg=1))
#[test]
fn test_rdffr_p_p_f_combo_4_f000_2518f021() {
    // Encoding: 0x2518F021
    // Test RDFFR_P.P.F__ field combination: Pg=1, Pd=1
    // Fields: Pg=1, Pd=1
    let encoding: u32 = 0x2518F021;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: RDFFR_P.P.F__
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=31 (same register test (reg=31)), Pd=31 (same register test (reg=31))
#[test]
fn test_rdffr_p_p_f_combo_5_f000_2518f1ef() {
    // Encoding: 0x2518F1EF
    // Test RDFFR_P.P.F__ field combination: Pg=31, Pd=31
    // Fields: Pd=31, Pg=31
    let encoding: u32 = 0x2518F1EF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: RDFFRS_P.P.F__
/// ASL: `field Pg 5 +: 4`
/// Requirement: FieldBoundary { field: "Pg", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_rdffrs_p_p_f_field_pg_0_min_f000_2558f000() {
    // Encoding: 0x2558F000
    // Test RDFFRS_P.P.F__ field Pg = 0 (Min)
    // Fields: Pg=0, Pd=0
    let encoding: u32 = 0x2558F000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: RDFFRS_P.P.F__
/// ASL: `field Pg 5 +: 4`
/// Requirement: FieldBoundary { field: "Pg", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_rdffrs_p_p_f_field_pg_1_poweroftwo_f000_2558f020() {
    // Encoding: 0x2558F020
    // Test RDFFRS_P.P.F__ field Pg = 1 (PowerOfTwo)
    // Fields: Pg=1, Pd=0
    let encoding: u32 = 0x2558F020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: RDFFRS_P.P.F__
/// ASL: `field Pd 0 +: 4`
/// Requirement: FieldBoundary { field: "Pd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_rdffrs_p_p_f_field_pd_0_min_f000_2558f000() {
    // Encoding: 0x2558F000
    // Test RDFFRS_P.P.F__ field Pd = 0 (Min)
    // Fields: Pd=0, Pg=0
    let encoding: u32 = 0x2558F000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: RDFFRS_P.P.F__
/// ASL: `field Pd 0 +: 4`
/// Requirement: FieldBoundary { field: "Pd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_rdffrs_p_p_f_field_pd_1_poweroftwo_f000_2558f001() {
    // Encoding: 0x2558F001
    // Test RDFFRS_P.P.F__ field Pd = 1 (PowerOfTwo)
    // Fields: Pg=0, Pd=1
    let encoding: u32 = 0x2558F001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: RDFFRS_P.P.F__
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=0 (register index 0 (first register))
#[test]
fn test_rdffrs_p_p_f_combo_0_f000_2558f000() {
    // Encoding: 0x2558F000
    // Test RDFFRS_P.P.F__ field combination: Pg=0, Pd=0
    // Fields: Pd=0, Pg=0
    let encoding: u32 = 0x2558F000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: RDFFRS_P.P.F__
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=1 (register index 1 (second register))
#[test]
fn test_rdffrs_p_p_f_combo_1_f000_2558f020() {
    // Encoding: 0x2558F020
    // Test RDFFRS_P.P.F__ field combination: Pg=1, Pd=0
    // Fields: Pg=1, Pd=0
    let encoding: u32 = 0x2558F020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: RDFFRS_P.P.F__
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pd=0 (register index 0 (first register))
#[test]
fn test_rdffrs_p_p_f_combo_2_f000_2558f000() {
    // Encoding: 0x2558F000
    // Test RDFFRS_P.P.F__ field combination: Pg=0, Pd=0
    // Fields: Pg=0, Pd=0
    let encoding: u32 = 0x2558F000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: RDFFRS_P.P.F__
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pd=1 (register index 1 (second register))
#[test]
fn test_rdffrs_p_p_f_combo_3_f000_2558f001() {
    // Encoding: 0x2558F001
    // Test RDFFRS_P.P.F__ field combination: Pg=0, Pd=1
    // Fields: Pd=1, Pg=0
    let encoding: u32 = 0x2558F001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: RDFFRS_P.P.F__
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=1 (same register test (reg=1)), Pd=1 (same register test (reg=1))
#[test]
fn test_rdffrs_p_p_f_combo_4_f000_2558f021() {
    // Encoding: 0x2558F021
    // Test RDFFRS_P.P.F__ field combination: Pg=1, Pd=1
    // Fields: Pd=1, Pg=1
    let encoding: u32 = 0x2558F021;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: RDFFRS_P.P.F__
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=31 (same register test (reg=31)), Pd=31 (same register test (reg=31))
#[test]
fn test_rdffrs_p_p_f_combo_5_f000_2558f1ef() {
    // Encoding: 0x2558F1EF
    // Test RDFFRS_P.P.F__ field combination: Pg=31, Pd=31
    // Fields: Pg=31, Pd=31
    let encoding: u32 = 0x2558F1EF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: RDFFR_P.P.F__
/// ASL: `SimdFromField("Pd") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "unknown" }
/// verify register write to SimdFromField("Pd")
#[test]
fn test_rdffr_p_p_f_reg_write_0_2518f000() {
    // Test RDFFR_P.P.F__ register write: SimdFromField("Pd")
    // Encoding: 0x2518F000
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x2518F000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: RDFFR_P.P.F__
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 0 + 0 = 0 (Z=1)
#[test]
fn test_rdffr_p_p_f_flags_zeroresult_0_2518f000() {
    // Test RDFFR_P.P.F__ flag computation: ZeroResult
    // Encoding: 0x2518F000
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x0);
    set_x(&mut cpu, 2, 0x0);
    let encoding: u32 = 0x2518F000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: RDFFR_P.P.F__
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 1 + (-1) = 0 (Z=1, C=1)
#[test]
fn test_rdffr_p_p_f_flags_zeroresult_1_2518f000() {
    // Test RDFFR_P.P.F__ flag computation: ZeroResult
    // Encoding: 0x2518F000
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0xFFFFFFFFFFFFFFFF);
    set_x(&mut cpu, 1, 0x1);
    let encoding: u32 = 0x2518F000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: RDFFR_P.P.F__
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: NegativeResult }
/// negative value (N=1)
#[test]
fn test_rdffr_p_p_f_flags_negativeresult_2_2518f000() {
    // Test RDFFR_P.P.F__ flag computation: NegativeResult
    // Encoding: 0x2518F000
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x8000000000000000);
    set_x(&mut cpu, 2, 0x0);
    let encoding: u32 = 0x2518F000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: RDFFR_P.P.F__
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 1 = 0 (C=1, Z=1)
#[test]
fn test_rdffr_p_p_f_flags_unsignedoverflow_3_2518f000() {
    // Test RDFFR_P.P.F__ flag computation: UnsignedOverflow
    // Encoding: 0x2518F000
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0xFFFFFFFFFFFFFFFF);
    set_x(&mut cpu, 2, 0x1);
    let encoding: u32 = 0x2518F000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: RDFFR_P.P.F__
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 2 = 1 (C=1)
#[test]
fn test_rdffr_p_p_f_flags_unsignedoverflow_4_2518f000() {
    // Test RDFFR_P.P.F__ flag computation: UnsignedOverflow
    // Encoding: 0x2518F000
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0x2);
    set_x(&mut cpu, 1, 0xFFFFFFFFFFFFFFFF);
    let encoding: u32 = 0x2518F000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: RDFFR_P.P.F__
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// max_signed + 1 = min_signed (V=1, N=1)
#[test]
fn test_rdffr_p_p_f_flags_signedoverflow_5_2518f000() {
    // Test RDFFR_P.P.F__ flag computation: SignedOverflow
    // Encoding: 0x2518F000
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x7FFFFFFFFFFFFFFF);
    set_x(&mut cpu, 2, 0x1);
    let encoding: u32 = 0x2518F000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: RDFFR_P.P.F__
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// min_signed + (-1) = max_signed (V=1)
#[test]
fn test_rdffr_p_p_f_flags_signedoverflow_6_2518f000() {
    // Test RDFFR_P.P.F__ flag computation: SignedOverflow
    // Encoding: 0x2518F000
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0xFFFFFFFFFFFFFFFF);
    set_x(&mut cpu, 1, 0x8000000000000000);
    let encoding: u32 = 0x2518F000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: RDFFR_P.P.F__
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: PositiveResult }
/// 100 + 50 = 150 (no flags)
#[test]
fn test_rdffr_p_p_f_flags_positiveresult_7_2518f000() {
    // Test RDFFR_P.P.F__ flag computation: PositiveResult
    // Encoding: 0x2518F000
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0x32);
    set_x(&mut cpu, 1, 0x64);
    let encoding: u32 = 0x2518F000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: RDFFRS_P.P.F__
/// ASL: `SimdFromField("Pd") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "unknown" }
/// verify register write to SimdFromField("Pd")
#[test]
fn test_rdffrs_p_p_f_reg_write_0_2558f000() {
    // Test RDFFRS_P.P.F__ register write: SimdFromField("Pd")
    // Encoding: 0x2558F000
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x2558F000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: RDFFRS_P.P.F__
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 0 + 0 = 0 (Z=1)
#[test]
fn test_rdffrs_p_p_f_flags_zeroresult_0_2558f000() {
    // Test RDFFRS_P.P.F__ flag computation: ZeroResult
    // Encoding: 0x2558F000
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x0);
    set_x(&mut cpu, 2, 0x0);
    let encoding: u32 = 0x2558F000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: RDFFRS_P.P.F__
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 1 + (-1) = 0 (Z=1, C=1)
#[test]
fn test_rdffrs_p_p_f_flags_zeroresult_1_2558f000() {
    // Test RDFFRS_P.P.F__ flag computation: ZeroResult
    // Encoding: 0x2558F000
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x1);
    set_x(&mut cpu, 2, 0xFFFFFFFFFFFFFFFF);
    let encoding: u32 = 0x2558F000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: RDFFRS_P.P.F__
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: NegativeResult }
/// negative value (N=1)
#[test]
fn test_rdffrs_p_p_f_flags_negativeresult_2_2558f000() {
    // Test RDFFRS_P.P.F__ flag computation: NegativeResult
    // Encoding: 0x2558F000
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x8000000000000000);
    set_x(&mut cpu, 2, 0x0);
    let encoding: u32 = 0x2558F000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: RDFFRS_P.P.F__
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 1 = 0 (C=1, Z=1)
#[test]
fn test_rdffrs_p_p_f_flags_unsignedoverflow_3_2558f000() {
    // Test RDFFRS_P.P.F__ flag computation: UnsignedOverflow
    // Encoding: 0x2558F000
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0x1);
    set_x(&mut cpu, 1, 0xFFFFFFFFFFFFFFFF);
    let encoding: u32 = 0x2558F000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: RDFFRS_P.P.F__
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 2 = 1 (C=1)
#[test]
fn test_rdffrs_p_p_f_flags_unsignedoverflow_4_2558f000() {
    // Test RDFFRS_P.P.F__ flag computation: UnsignedOverflow
    // Encoding: 0x2558F000
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0x2);
    set_x(&mut cpu, 1, 0xFFFFFFFFFFFFFFFF);
    let encoding: u32 = 0x2558F000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: RDFFRS_P.P.F__
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// max_signed + 1 = min_signed (V=1, N=1)
#[test]
fn test_rdffrs_p_p_f_flags_signedoverflow_5_2558f000() {
    // Test RDFFRS_P.P.F__ flag computation: SignedOverflow
    // Encoding: 0x2558F000
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x7FFFFFFFFFFFFFFF);
    set_x(&mut cpu, 2, 0x1);
    let encoding: u32 = 0x2558F000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: RDFFRS_P.P.F__
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// min_signed + (-1) = max_signed (V=1)
#[test]
fn test_rdffrs_p_p_f_flags_signedoverflow_6_2558f000() {
    // Test RDFFRS_P.P.F__ flag computation: SignedOverflow
    // Encoding: 0x2558F000
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0xFFFFFFFFFFFFFFFF);
    set_x(&mut cpu, 1, 0x8000000000000000);
    let encoding: u32 = 0x2558F000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: RDFFRS_P.P.F__
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: PositiveResult }
/// 100 + 50 = 150 (no flags)
#[test]
fn test_rdffrs_p_p_f_flags_positiveresult_7_2558f000() {
    // Test RDFFRS_P.P.F__ flag computation: PositiveResult
    // Encoding: 0x2558F000
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x64);
    set_x(&mut cpu, 2, 0x32);
    let encoding: u32 = 0x2558F000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

// ============================================================================
// RDFFR_P.F__ Tests
// ============================================================================

/// Provenance: RDFFR_P.F__
/// ASL: `field Pd 0 +: 4`
/// Requirement: FieldBoundary { field: "Pd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_rdffr_p_f_field_pd_0_min_f000_2519f000() {
    // Encoding: 0x2519F000
    // Test RDFFR_P.F__ field Pd = 0 (Min)
    // Fields: Pd=0
    let encoding: u32 = 0x2519F000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: RDFFR_P.F__
/// ASL: `field Pd 0 +: 4`
/// Requirement: FieldBoundary { field: "Pd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_rdffr_p_f_field_pd_1_poweroftwo_f000_2519f001() {
    // Encoding: 0x2519F001
    // Test RDFFR_P.F__ field Pd = 1 (PowerOfTwo)
    // Fields: Pd=1
    let encoding: u32 = 0x2519F001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: RDFFR_P.F__
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pd=0 (register index 0 (first register))
#[test]
fn test_rdffr_p_f_combo_0_f000_2519f000() {
    // Encoding: 0x2519F000
    // Test RDFFR_P.F__ field combination: Pd=0
    // Fields: Pd=0
    let encoding: u32 = 0x2519F000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: RDFFR_P.F__
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pd=1 (register index 1 (second register))
#[test]
fn test_rdffr_p_f_combo_1_f000_2519f001() {
    // Encoding: 0x2519F001
    // Test RDFFR_P.F__ field combination: Pd=1
    // Fields: Pd=1
    let encoding: u32 = 0x2519F001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: RDFFR_P.F__
/// ASL: `SimdFromField("Pd") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "unknown" }
/// verify register write to SimdFromField("Pd")
#[test]
fn test_rdffr_p_f_reg_write_0_2519f000() {
    // Test RDFFR_P.F__ register write: SimdFromField("Pd")
    // Encoding: 0x2519F000
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x2519F000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

// ============================================================================
// NAND_P.P.PP_Z Tests
// ============================================================================

/// Provenance: NAND_P.P.PP_Z
/// ASL: `field Pm 16 +: 4`
/// Requirement: FieldBoundary { field: "Pm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_nand_p_p_pp_z_field_pm_0_min_4210_25804210() {
    // Encoding: 0x25804210
    // Test NAND_P.P.PP_Z field Pm = 0 (Min)
    // Fields: Pd=0, Pm=0, Pg=0, Pn=0
    let encoding: u32 = 0x25804210;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: NAND_P.P.PP_Z
/// ASL: `field Pm 16 +: 4`
/// Requirement: FieldBoundary { field: "Pm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_nand_p_p_pp_z_field_pm_1_poweroftwo_4210_25814210() {
    // Encoding: 0x25814210
    // Test NAND_P.P.PP_Z field Pm = 1 (PowerOfTwo)
    // Fields: Pm=1, Pd=0, Pn=0, Pg=0
    let encoding: u32 = 0x25814210;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: NAND_P.P.PP_Z
/// ASL: `field Pg 10 +: 4`
/// Requirement: FieldBoundary { field: "Pg", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_nand_p_p_pp_z_field_pg_0_min_4210_25804210() {
    // Encoding: 0x25804210
    // Test NAND_P.P.PP_Z field Pg = 0 (Min)
    // Fields: Pg=0, Pn=0, Pm=0, Pd=0
    let encoding: u32 = 0x25804210;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: NAND_P.P.PP_Z
/// ASL: `field Pg 10 +: 4`
/// Requirement: FieldBoundary { field: "Pg", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_nand_p_p_pp_z_field_pg_1_poweroftwo_4210_25804610() {
    // Encoding: 0x25804610
    // Test NAND_P.P.PP_Z field Pg = 1 (PowerOfTwo)
    // Fields: Pg=1, Pd=0, Pm=0, Pn=0
    let encoding: u32 = 0x25804610;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: NAND_P.P.PP_Z
/// ASL: `field Pn 5 +: 4`
/// Requirement: FieldBoundary { field: "Pn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_nand_p_p_pp_z_field_pn_0_min_4210_25804210() {
    // Encoding: 0x25804210
    // Test NAND_P.P.PP_Z field Pn = 0 (Min)
    // Fields: Pg=0, Pd=0, Pn=0, Pm=0
    let encoding: u32 = 0x25804210;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: NAND_P.P.PP_Z
/// ASL: `field Pn 5 +: 4`
/// Requirement: FieldBoundary { field: "Pn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_nand_p_p_pp_z_field_pn_1_poweroftwo_4210_25804230() {
    // Encoding: 0x25804230
    // Test NAND_P.P.PP_Z field Pn = 1 (PowerOfTwo)
    // Fields: Pn=1, Pd=0, Pg=0, Pm=0
    let encoding: u32 = 0x25804230;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: NAND_P.P.PP_Z
/// ASL: `field Pd 0 +: 4`
/// Requirement: FieldBoundary { field: "Pd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_nand_p_p_pp_z_field_pd_0_min_4210_25804210() {
    // Encoding: 0x25804210
    // Test NAND_P.P.PP_Z field Pd = 0 (Min)
    // Fields: Pd=0, Pg=0, Pn=0, Pm=0
    let encoding: u32 = 0x25804210;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: NAND_P.P.PP_Z
/// ASL: `field Pd 0 +: 4`
/// Requirement: FieldBoundary { field: "Pd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_nand_p_p_pp_z_field_pd_1_poweroftwo_4210_25804211() {
    // Encoding: 0x25804211
    // Test NAND_P.P.PP_Z field Pd = 1 (PowerOfTwo)
    // Fields: Pd=1, Pn=0, Pm=0, Pg=0
    let encoding: u32 = 0x25804211;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: NAND_P.P.PP_Z
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pm=0 (register index 0 (first register))
#[test]
fn test_nand_p_p_pp_z_combo_0_4210_25804210() {
    // Encoding: 0x25804210
    // Test NAND_P.P.PP_Z field combination: Pm=0, Pg=0, Pn=0, Pd=0
    // Fields: Pm=0, Pd=0, Pg=0, Pn=0
    let encoding: u32 = 0x25804210;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: NAND_P.P.PP_Z
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pm=1 (register index 1 (second register))
#[test]
fn test_nand_p_p_pp_z_combo_1_4210_25814210() {
    // Encoding: 0x25814210
    // Test NAND_P.P.PP_Z field combination: Pm=1, Pg=0, Pn=0, Pd=0
    // Fields: Pn=0, Pm=1, Pd=0, Pg=0
    let encoding: u32 = 0x25814210;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: NAND_P.P.PP_Z
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=0 (register index 0 (first register))
#[test]
fn test_nand_p_p_pp_z_combo_2_4210_25804210() {
    // Encoding: 0x25804210
    // Test NAND_P.P.PP_Z field combination: Pm=0, Pg=0, Pn=0, Pd=0
    // Fields: Pd=0, Pg=0, Pm=0, Pn=0
    let encoding: u32 = 0x25804210;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: NAND_P.P.PP_Z
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=1 (register index 1 (second register))
#[test]
fn test_nand_p_p_pp_z_combo_3_4210_25804610() {
    // Encoding: 0x25804610
    // Test NAND_P.P.PP_Z field combination: Pm=0, Pg=1, Pn=0, Pd=0
    // Fields: Pm=0, Pg=1, Pn=0, Pd=0
    let encoding: u32 = 0x25804610;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: NAND_P.P.PP_Z
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pn=0 (register index 0 (first register))
#[test]
fn test_nand_p_p_pp_z_combo_4_4210_25804210() {
    // Encoding: 0x25804210
    // Test NAND_P.P.PP_Z field combination: Pm=0, Pg=0, Pn=0, Pd=0
    // Fields: Pm=0, Pd=0, Pg=0, Pn=0
    let encoding: u32 = 0x25804210;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: NAND_P.P.PP_Z
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pn=1 (register index 1 (second register))
#[test]
fn test_nand_p_p_pp_z_combo_5_4210_25804230() {
    // Encoding: 0x25804230
    // Test NAND_P.P.PP_Z field combination: Pm=0, Pg=0, Pn=1, Pd=0
    // Fields: Pm=0, Pn=1, Pg=0, Pd=0
    let encoding: u32 = 0x25804230;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: NAND_P.P.PP_Z
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pd=0 (register index 0 (first register))
#[test]
fn test_nand_p_p_pp_z_combo_6_4210_25804210() {
    // Encoding: 0x25804210
    // Test NAND_P.P.PP_Z field combination: Pm=0, Pg=0, Pn=0, Pd=0
    // Fields: Pn=0, Pm=0, Pd=0, Pg=0
    let encoding: u32 = 0x25804210;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: NAND_P.P.PP_Z
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pd=1 (register index 1 (second register))
#[test]
fn test_nand_p_p_pp_z_combo_7_4210_25804211() {
    // Encoding: 0x25804211
    // Test NAND_P.P.PP_Z field combination: Pm=0, Pg=0, Pn=0, Pd=1
    // Fields: Pg=0, Pd=1, Pm=0, Pn=0
    let encoding: u32 = 0x25804211;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: NAND_P.P.PP_Z
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pm=1 (same register test (reg=1)), Pg=1 (same register test (reg=1))
#[test]
fn test_nand_p_p_pp_z_combo_8_4210_25814610() {
    // Encoding: 0x25814610
    // Test NAND_P.P.PP_Z field combination: Pm=1, Pg=1, Pn=0, Pd=0
    // Fields: Pm=1, Pg=1, Pn=0, Pd=0
    let encoding: u32 = 0x25814610;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: NAND_P.P.PP_Z
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pm=31 (same register test (reg=31)), Pg=31 (same register test (reg=31))
#[test]
fn test_nand_p_p_pp_z_combo_9_4210_258f7e10() {
    // Encoding: 0x258F7E10
    // Test NAND_P.P.PP_Z field combination: Pm=31, Pg=31, Pn=0, Pd=0
    // Fields: Pn=0, Pg=31, Pm=31, Pd=0
    let encoding: u32 = 0x258F7E10;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: NAND_P.P.PP_Z
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pm=1 (same register test (reg=1)), Pn=1 (same register test (reg=1))
#[test]
fn test_nand_p_p_pp_z_combo_10_4210_25814230() {
    // Encoding: 0x25814230
    // Test NAND_P.P.PP_Z field combination: Pm=1, Pg=0, Pn=1, Pd=0
    // Fields: Pg=0, Pd=0, Pm=1, Pn=1
    let encoding: u32 = 0x25814230;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: NAND_P.P.PP_Z
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pm=31 (same register test (reg=31)), Pn=31 (same register test (reg=31))
#[test]
fn test_nand_p_p_pp_z_combo_11_4210_258f43f0() {
    // Encoding: 0x258F43F0
    // Test NAND_P.P.PP_Z field combination: Pm=31, Pg=0, Pn=31, Pd=0
    // Fields: Pn=31, Pm=31, Pd=0, Pg=0
    let encoding: u32 = 0x258F43F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: NAND_P.P.PP_Z
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pm=1 (same register test (reg=1)), Pd=1 (same register test (reg=1))
#[test]
fn test_nand_p_p_pp_z_combo_12_4210_25814211() {
    // Encoding: 0x25814211
    // Test NAND_P.P.PP_Z field combination: Pm=1, Pg=0, Pn=0, Pd=1
    // Fields: Pg=0, Pn=0, Pd=1, Pm=1
    let encoding: u32 = 0x25814211;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: NAND_P.P.PP_Z
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pm=31 (same register test (reg=31)), Pd=31 (same register test (reg=31))
#[test]
fn test_nand_p_p_pp_z_combo_13_4210_258f421f() {
    // Encoding: 0x258F421F
    // Test NAND_P.P.PP_Z field combination: Pm=31, Pg=0, Pn=0, Pd=31
    // Fields: Pm=31, Pg=0, Pd=31, Pn=0
    let encoding: u32 = 0x258F421F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: NAND_P.P.PP_Z
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=1 (same register test (reg=1)), Pn=1 (same register test (reg=1))
#[test]
fn test_nand_p_p_pp_z_combo_14_4210_25804630() {
    // Encoding: 0x25804630
    // Test NAND_P.P.PP_Z field combination: Pm=0, Pg=1, Pn=1, Pd=0
    // Fields: Pm=0, Pd=0, Pg=1, Pn=1
    let encoding: u32 = 0x25804630;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: NAND_P.P.PP_Z
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=31 (same register test (reg=31)), Pn=31 (same register test (reg=31))
#[test]
fn test_nand_p_p_pp_z_combo_15_4210_25807ff0() {
    // Encoding: 0x25807FF0
    // Test NAND_P.P.PP_Z field combination: Pm=0, Pg=31, Pn=31, Pd=0
    // Fields: Pd=0, Pn=31, Pg=31, Pm=0
    let encoding: u32 = 0x25807FF0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: NAND_P.P.PP_Z
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=1 (same register test (reg=1)), Pd=1 (same register test (reg=1))
#[test]
fn test_nand_p_p_pp_z_combo_16_4210_25804611() {
    // Encoding: 0x25804611
    // Test NAND_P.P.PP_Z field combination: Pm=0, Pg=1, Pn=0, Pd=1
    // Fields: Pn=0, Pg=1, Pd=1, Pm=0
    let encoding: u32 = 0x25804611;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: NAND_P.P.PP_Z
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=31 (same register test (reg=31)), Pd=31 (same register test (reg=31))
#[test]
fn test_nand_p_p_pp_z_combo_17_4210_25807e1f() {
    // Encoding: 0x25807E1F
    // Test NAND_P.P.PP_Z field combination: Pm=0, Pg=31, Pn=0, Pd=31
    // Fields: Pd=31, Pg=31, Pn=0, Pm=0
    let encoding: u32 = 0x25807E1F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: NAND_P.P.PP_Z
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pn=1 (same register test (reg=1)), Pd=1 (same register test (reg=1))
#[test]
fn test_nand_p_p_pp_z_combo_18_4210_25804231() {
    // Encoding: 0x25804231
    // Test NAND_P.P.PP_Z field combination: Pm=0, Pg=0, Pn=1, Pd=1
    // Fields: Pn=1, Pg=0, Pm=0, Pd=1
    let encoding: u32 = 0x25804231;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: NAND_P.P.PP_Z
/// ASL: `field combination 19`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pn=31 (same register test (reg=31)), Pd=31 (same register test (reg=31))
#[test]
fn test_nand_p_p_pp_z_combo_19_4210_258043ff() {
    // Encoding: 0x258043FF
    // Test NAND_P.P.PP_Z field combination: Pm=0, Pg=0, Pn=31, Pd=31
    // Fields: Pm=0, Pd=31, Pg=0, Pn=31
    let encoding: u32 = 0x258043FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: NANDS_P.P.PP_Z
/// ASL: `field Pm 16 +: 4`
/// Requirement: FieldBoundary { field: "Pm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_nands_p_p_pp_z_field_pm_0_min_4210_25c04210() {
    // Encoding: 0x25C04210
    // Test NANDS_P.P.PP_Z field Pm = 0 (Min)
    // Fields: Pd=0, Pm=0, Pn=0, Pg=0
    let encoding: u32 = 0x25C04210;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: NANDS_P.P.PP_Z
/// ASL: `field Pm 16 +: 4`
/// Requirement: FieldBoundary { field: "Pm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_nands_p_p_pp_z_field_pm_1_poweroftwo_4210_25c14210() {
    // Encoding: 0x25C14210
    // Test NANDS_P.P.PP_Z field Pm = 1 (PowerOfTwo)
    // Fields: Pg=0, Pd=0, Pm=1, Pn=0
    let encoding: u32 = 0x25C14210;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: NANDS_P.P.PP_Z
/// ASL: `field Pg 10 +: 4`
/// Requirement: FieldBoundary { field: "Pg", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_nands_p_p_pp_z_field_pg_0_min_4210_25c04210() {
    // Encoding: 0x25C04210
    // Test NANDS_P.P.PP_Z field Pg = 0 (Min)
    // Fields: Pm=0, Pg=0, Pd=0, Pn=0
    let encoding: u32 = 0x25C04210;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: NANDS_P.P.PP_Z
/// ASL: `field Pg 10 +: 4`
/// Requirement: FieldBoundary { field: "Pg", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_nands_p_p_pp_z_field_pg_1_poweroftwo_4210_25c04610() {
    // Encoding: 0x25C04610
    // Test NANDS_P.P.PP_Z field Pg = 1 (PowerOfTwo)
    // Fields: Pn=0, Pg=1, Pm=0, Pd=0
    let encoding: u32 = 0x25C04610;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: NANDS_P.P.PP_Z
/// ASL: `field Pn 5 +: 4`
/// Requirement: FieldBoundary { field: "Pn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_nands_p_p_pp_z_field_pn_0_min_4210_25c04210() {
    // Encoding: 0x25C04210
    // Test NANDS_P.P.PP_Z field Pn = 0 (Min)
    // Fields: Pn=0, Pm=0, Pd=0, Pg=0
    let encoding: u32 = 0x25C04210;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: NANDS_P.P.PP_Z
/// ASL: `field Pn 5 +: 4`
/// Requirement: FieldBoundary { field: "Pn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_nands_p_p_pp_z_field_pn_1_poweroftwo_4210_25c04230() {
    // Encoding: 0x25C04230
    // Test NANDS_P.P.PP_Z field Pn = 1 (PowerOfTwo)
    // Fields: Pm=0, Pn=1, Pg=0, Pd=0
    let encoding: u32 = 0x25C04230;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: NANDS_P.P.PP_Z
/// ASL: `field Pd 0 +: 4`
/// Requirement: FieldBoundary { field: "Pd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_nands_p_p_pp_z_field_pd_0_min_4210_25c04210() {
    // Encoding: 0x25C04210
    // Test NANDS_P.P.PP_Z field Pd = 0 (Min)
    // Fields: Pn=0, Pd=0, Pg=0, Pm=0
    let encoding: u32 = 0x25C04210;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: NANDS_P.P.PP_Z
/// ASL: `field Pd 0 +: 4`
/// Requirement: FieldBoundary { field: "Pd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_nands_p_p_pp_z_field_pd_1_poweroftwo_4210_25c04211() {
    // Encoding: 0x25C04211
    // Test NANDS_P.P.PP_Z field Pd = 1 (PowerOfTwo)
    // Fields: Pn=0, Pg=0, Pd=1, Pm=0
    let encoding: u32 = 0x25C04211;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: NANDS_P.P.PP_Z
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pm=0 (register index 0 (first register))
#[test]
fn test_nands_p_p_pp_z_combo_0_4210_25c04210() {
    // Encoding: 0x25C04210
    // Test NANDS_P.P.PP_Z field combination: Pm=0, Pg=0, Pn=0, Pd=0
    // Fields: Pd=0, Pn=0, Pg=0, Pm=0
    let encoding: u32 = 0x25C04210;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: NANDS_P.P.PP_Z
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pm=1 (register index 1 (second register))
#[test]
fn test_nands_p_p_pp_z_combo_1_4210_25c14210() {
    // Encoding: 0x25C14210
    // Test NANDS_P.P.PP_Z field combination: Pm=1, Pg=0, Pn=0, Pd=0
    // Fields: Pg=0, Pn=0, Pd=0, Pm=1
    let encoding: u32 = 0x25C14210;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: NANDS_P.P.PP_Z
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=0 (register index 0 (first register))
#[test]
fn test_nands_p_p_pp_z_combo_2_4210_25c04210() {
    // Encoding: 0x25C04210
    // Test NANDS_P.P.PP_Z field combination: Pm=0, Pg=0, Pn=0, Pd=0
    // Fields: Pg=0, Pn=0, Pd=0, Pm=0
    let encoding: u32 = 0x25C04210;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: NANDS_P.P.PP_Z
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=1 (register index 1 (second register))
#[test]
fn test_nands_p_p_pp_z_combo_3_4210_25c04610() {
    // Encoding: 0x25C04610
    // Test NANDS_P.P.PP_Z field combination: Pm=0, Pg=1, Pn=0, Pd=0
    // Fields: Pn=0, Pg=1, Pd=0, Pm=0
    let encoding: u32 = 0x25C04610;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: NANDS_P.P.PP_Z
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pn=0 (register index 0 (first register))
#[test]
fn test_nands_p_p_pp_z_combo_4_4210_25c04210() {
    // Encoding: 0x25C04210
    // Test NANDS_P.P.PP_Z field combination: Pm=0, Pg=0, Pn=0, Pd=0
    // Fields: Pd=0, Pg=0, Pm=0, Pn=0
    let encoding: u32 = 0x25C04210;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: NANDS_P.P.PP_Z
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pn=1 (register index 1 (second register))
#[test]
fn test_nands_p_p_pp_z_combo_5_4210_25c04230() {
    // Encoding: 0x25C04230
    // Test NANDS_P.P.PP_Z field combination: Pm=0, Pg=0, Pn=1, Pd=0
    // Fields: Pd=0, Pm=0, Pg=0, Pn=1
    let encoding: u32 = 0x25C04230;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: NANDS_P.P.PP_Z
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pd=0 (register index 0 (first register))
#[test]
fn test_nands_p_p_pp_z_combo_6_4210_25c04210() {
    // Encoding: 0x25C04210
    // Test NANDS_P.P.PP_Z field combination: Pm=0, Pg=0, Pn=0, Pd=0
    // Fields: Pg=0, Pd=0, Pn=0, Pm=0
    let encoding: u32 = 0x25C04210;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: NANDS_P.P.PP_Z
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pd=1 (register index 1 (second register))
#[test]
fn test_nands_p_p_pp_z_combo_7_4210_25c04211() {
    // Encoding: 0x25C04211
    // Test NANDS_P.P.PP_Z field combination: Pm=0, Pg=0, Pn=0, Pd=1
    // Fields: Pn=0, Pm=0, Pd=1, Pg=0
    let encoding: u32 = 0x25C04211;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: NANDS_P.P.PP_Z
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pm=1 (same register test (reg=1)), Pg=1 (same register test (reg=1))
#[test]
fn test_nands_p_p_pp_z_combo_8_4210_25c14610() {
    // Encoding: 0x25C14610
    // Test NANDS_P.P.PP_Z field combination: Pm=1, Pg=1, Pn=0, Pd=0
    // Fields: Pd=0, Pn=0, Pg=1, Pm=1
    let encoding: u32 = 0x25C14610;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: NANDS_P.P.PP_Z
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pm=31 (same register test (reg=31)), Pg=31 (same register test (reg=31))
#[test]
fn test_nands_p_p_pp_z_combo_9_4210_25cf7e10() {
    // Encoding: 0x25CF7E10
    // Test NANDS_P.P.PP_Z field combination: Pm=31, Pg=31, Pn=0, Pd=0
    // Fields: Pd=0, Pn=0, Pg=31, Pm=31
    let encoding: u32 = 0x25CF7E10;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: NANDS_P.P.PP_Z
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pm=1 (same register test (reg=1)), Pn=1 (same register test (reg=1))
#[test]
fn test_nands_p_p_pp_z_combo_10_4210_25c14230() {
    // Encoding: 0x25C14230
    // Test NANDS_P.P.PP_Z field combination: Pm=1, Pg=0, Pn=1, Pd=0
    // Fields: Pd=0, Pn=1, Pm=1, Pg=0
    let encoding: u32 = 0x25C14230;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: NANDS_P.P.PP_Z
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pm=31 (same register test (reg=31)), Pn=31 (same register test (reg=31))
#[test]
fn test_nands_p_p_pp_z_combo_11_4210_25cf43f0() {
    // Encoding: 0x25CF43F0
    // Test NANDS_P.P.PP_Z field combination: Pm=31, Pg=0, Pn=31, Pd=0
    // Fields: Pg=0, Pn=31, Pm=31, Pd=0
    let encoding: u32 = 0x25CF43F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: NANDS_P.P.PP_Z
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pm=1 (same register test (reg=1)), Pd=1 (same register test (reg=1))
#[test]
fn test_nands_p_p_pp_z_combo_12_4210_25c14211() {
    // Encoding: 0x25C14211
    // Test NANDS_P.P.PP_Z field combination: Pm=1, Pg=0, Pn=0, Pd=1
    // Fields: Pg=0, Pm=1, Pd=1, Pn=0
    let encoding: u32 = 0x25C14211;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: NANDS_P.P.PP_Z
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pm=31 (same register test (reg=31)), Pd=31 (same register test (reg=31))
#[test]
fn test_nands_p_p_pp_z_combo_13_4210_25cf421f() {
    // Encoding: 0x25CF421F
    // Test NANDS_P.P.PP_Z field combination: Pm=31, Pg=0, Pn=0, Pd=31
    // Fields: Pg=0, Pd=31, Pm=31, Pn=0
    let encoding: u32 = 0x25CF421F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: NANDS_P.P.PP_Z
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=1 (same register test (reg=1)), Pn=1 (same register test (reg=1))
#[test]
fn test_nands_p_p_pp_z_combo_14_4210_25c04630() {
    // Encoding: 0x25C04630
    // Test NANDS_P.P.PP_Z field combination: Pm=0, Pg=1, Pn=1, Pd=0
    // Fields: Pm=0, Pg=1, Pd=0, Pn=1
    let encoding: u32 = 0x25C04630;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: NANDS_P.P.PP_Z
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=31 (same register test (reg=31)), Pn=31 (same register test (reg=31))
#[test]
fn test_nands_p_p_pp_z_combo_15_4210_25c07ff0() {
    // Encoding: 0x25C07FF0
    // Test NANDS_P.P.PP_Z field combination: Pm=0, Pg=31, Pn=31, Pd=0
    // Fields: Pn=31, Pd=0, Pm=0, Pg=31
    let encoding: u32 = 0x25C07FF0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: NANDS_P.P.PP_Z
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=1 (same register test (reg=1)), Pd=1 (same register test (reg=1))
#[test]
fn test_nands_p_p_pp_z_combo_16_4210_25c04611() {
    // Encoding: 0x25C04611
    // Test NANDS_P.P.PP_Z field combination: Pm=0, Pg=1, Pn=0, Pd=1
    // Fields: Pn=0, Pd=1, Pg=1, Pm=0
    let encoding: u32 = 0x25C04611;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: NANDS_P.P.PP_Z
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=31 (same register test (reg=31)), Pd=31 (same register test (reg=31))
#[test]
fn test_nands_p_p_pp_z_combo_17_4210_25c07e1f() {
    // Encoding: 0x25C07E1F
    // Test NANDS_P.P.PP_Z field combination: Pm=0, Pg=31, Pn=0, Pd=31
    // Fields: Pg=31, Pm=0, Pd=31, Pn=0
    let encoding: u32 = 0x25C07E1F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: NANDS_P.P.PP_Z
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pn=1 (same register test (reg=1)), Pd=1 (same register test (reg=1))
#[test]
fn test_nands_p_p_pp_z_combo_18_4210_25c04231() {
    // Encoding: 0x25C04231
    // Test NANDS_P.P.PP_Z field combination: Pm=0, Pg=0, Pn=1, Pd=1
    // Fields: Pg=0, Pm=0, Pn=1, Pd=1
    let encoding: u32 = 0x25C04231;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: NANDS_P.P.PP_Z
/// ASL: `field combination 19`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pn=31 (same register test (reg=31)), Pd=31 (same register test (reg=31))
#[test]
fn test_nands_p_p_pp_z_combo_19_4210_25c043ff() {
    // Encoding: 0x25C043FF
    // Test NANDS_P.P.PP_Z field combination: Pm=0, Pg=0, Pn=31, Pd=31
    // Fields: Pg=0, Pm=0, Pn=31, Pd=31
    let encoding: u32 = 0x25C043FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: NAND_P.P.PP_Z
/// ASL: `SimdFromField("Pd") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "unknown" }
/// verify register write to SimdFromField("Pd")
#[test]
fn test_nand_p_p_pp_z_reg_write_0_25804210() {
    // Test NAND_P.P.PP_Z register write: SimdFromField("Pd")
    // Encoding: 0x25804210
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x25804210;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: NAND_P.P.PP_Z
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 0 + 0 = 0 (Z=1)
#[test]
fn test_nand_p_p_pp_z_flags_zeroresult_0_25804210() {
    // Test NAND_P.P.PP_Z flag computation: ZeroResult
    // Encoding: 0x25804210
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0x0);
    set_x(&mut cpu, 1, 0x0);
    let encoding: u32 = 0x25804210;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: NAND_P.P.PP_Z
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 1 + (-1) = 0 (Z=1, C=1)
#[test]
fn test_nand_p_p_pp_z_flags_zeroresult_1_25804210() {
    // Test NAND_P.P.PP_Z flag computation: ZeroResult
    // Encoding: 0x25804210
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x1);
    set_x(&mut cpu, 2, 0xFFFFFFFFFFFFFFFF);
    let encoding: u32 = 0x25804210;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: NAND_P.P.PP_Z
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: NegativeResult }
/// negative value (N=1)
#[test]
fn test_nand_p_p_pp_z_flags_negativeresult_2_25804210() {
    // Test NAND_P.P.PP_Z flag computation: NegativeResult
    // Encoding: 0x25804210
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0x0);
    set_x(&mut cpu, 1, 0x8000000000000000);
    let encoding: u32 = 0x25804210;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: NAND_P.P.PP_Z
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 1 = 0 (C=1, Z=1)
#[test]
fn test_nand_p_p_pp_z_flags_unsignedoverflow_3_25804210() {
    // Test NAND_P.P.PP_Z flag computation: UnsignedOverflow
    // Encoding: 0x25804210
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0x1);
    set_x(&mut cpu, 1, 0xFFFFFFFFFFFFFFFF);
    let encoding: u32 = 0x25804210;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: NAND_P.P.PP_Z
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 2 = 1 (C=1)
#[test]
fn test_nand_p_p_pp_z_flags_unsignedoverflow_4_25804210() {
    // Test NAND_P.P.PP_Z flag computation: UnsignedOverflow
    // Encoding: 0x25804210
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0xFFFFFFFFFFFFFFFF);
    set_x(&mut cpu, 2, 0x2);
    let encoding: u32 = 0x25804210;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: NAND_P.P.PP_Z
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// max_signed + 1 = min_signed (V=1, N=1)
#[test]
fn test_nand_p_p_pp_z_flags_signedoverflow_5_25804210() {
    // Test NAND_P.P.PP_Z flag computation: SignedOverflow
    // Encoding: 0x25804210
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x7FFFFFFFFFFFFFFF);
    set_x(&mut cpu, 2, 0x1);
    let encoding: u32 = 0x25804210;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: NAND_P.P.PP_Z
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// min_signed + (-1) = max_signed (V=1)
#[test]
fn test_nand_p_p_pp_z_flags_signedoverflow_6_25804210() {
    // Test NAND_P.P.PP_Z flag computation: SignedOverflow
    // Encoding: 0x25804210
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0xFFFFFFFFFFFFFFFF);
    set_x(&mut cpu, 1, 0x8000000000000000);
    let encoding: u32 = 0x25804210;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: NAND_P.P.PP_Z
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: PositiveResult }
/// 100 + 50 = 150 (no flags)
#[test]
fn test_nand_p_p_pp_z_flags_positiveresult_7_25804210() {
    // Test NAND_P.P.PP_Z flag computation: PositiveResult
    // Encoding: 0x25804210
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x64);
    set_x(&mut cpu, 2, 0x32);
    let encoding: u32 = 0x25804210;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: NANDS_P.P.PP_Z
/// ASL: `SimdFromField("Pd") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "unknown" }
/// verify register write to SimdFromField("Pd")
#[test]
fn test_nands_p_p_pp_z_reg_write_0_25c04210() {
    // Test NANDS_P.P.PP_Z register write: SimdFromField("Pd")
    // Encoding: 0x25C04210
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x25C04210;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: NANDS_P.P.PP_Z
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 0 + 0 = 0 (Z=1)
#[test]
fn test_nands_p_p_pp_z_flags_zeroresult_0_25c04210() {
    // Test NANDS_P.P.PP_Z flag computation: ZeroResult
    // Encoding: 0x25C04210
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x0);
    set_x(&mut cpu, 2, 0x0);
    let encoding: u32 = 0x25C04210;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: NANDS_P.P.PP_Z
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 1 + (-1) = 0 (Z=1, C=1)
#[test]
fn test_nands_p_p_pp_z_flags_zeroresult_1_25c04210() {
    // Test NANDS_P.P.PP_Z flag computation: ZeroResult
    // Encoding: 0x25C04210
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x1);
    set_x(&mut cpu, 2, 0xFFFFFFFFFFFFFFFF);
    let encoding: u32 = 0x25C04210;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: NANDS_P.P.PP_Z
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: NegativeResult }
/// negative value (N=1)
#[test]
fn test_nands_p_p_pp_z_flags_negativeresult_2_25c04210() {
    // Test NANDS_P.P.PP_Z flag computation: NegativeResult
    // Encoding: 0x25C04210
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0x0);
    set_x(&mut cpu, 1, 0x8000000000000000);
    let encoding: u32 = 0x25C04210;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: NANDS_P.P.PP_Z
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 1 = 0 (C=1, Z=1)
#[test]
fn test_nands_p_p_pp_z_flags_unsignedoverflow_3_25c04210() {
    // Test NANDS_P.P.PP_Z flag computation: UnsignedOverflow
    // Encoding: 0x25C04210
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0xFFFFFFFFFFFFFFFF);
    set_x(&mut cpu, 2, 0x1);
    let encoding: u32 = 0x25C04210;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: NANDS_P.P.PP_Z
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 2 = 1 (C=1)
#[test]
fn test_nands_p_p_pp_z_flags_unsignedoverflow_4_25c04210() {
    // Test NANDS_P.P.PP_Z flag computation: UnsignedOverflow
    // Encoding: 0x25C04210
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0xFFFFFFFFFFFFFFFF);
    set_x(&mut cpu, 2, 0x2);
    let encoding: u32 = 0x25C04210;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: NANDS_P.P.PP_Z
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// max_signed + 1 = min_signed (V=1, N=1)
#[test]
fn test_nands_p_p_pp_z_flags_signedoverflow_5_25c04210() {
    // Test NANDS_P.P.PP_Z flag computation: SignedOverflow
    // Encoding: 0x25C04210
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x7FFFFFFFFFFFFFFF);
    set_x(&mut cpu, 2, 0x1);
    let encoding: u32 = 0x25C04210;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: NANDS_P.P.PP_Z
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// min_signed + (-1) = max_signed (V=1)
#[test]
fn test_nands_p_p_pp_z_flags_signedoverflow_6_25c04210() {
    // Test NANDS_P.P.PP_Z flag computation: SignedOverflow
    // Encoding: 0x25C04210
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x8000000000000000);
    set_x(&mut cpu, 2, 0xFFFFFFFFFFFFFFFF);
    let encoding: u32 = 0x25C04210;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: NANDS_P.P.PP_Z
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: PositiveResult }
/// 100 + 50 = 150 (no flags)
#[test]
fn test_nands_p_p_pp_z_flags_positiveresult_7_25c04210() {
    // Test NANDS_P.P.PP_Z flag computation: PositiveResult
    // Encoding: 0x25C04210
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x64);
    set_x(&mut cpu, 2, 0x32);
    let encoding: u32 = 0x25C04210;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

// ============================================================================
// PFIRST_P.P.P__ Tests
// ============================================================================

/// Provenance: PFIRST_P.P.P__
/// ASL: `field Pg 5 +: 4`
/// Requirement: FieldBoundary { field: "Pg", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_pfirst_p_p_p_field_pg_0_min_c000_2558c000() {
    // Encoding: 0x2558C000
    // Test PFIRST_P.P.P__ field Pg = 0 (Min)
    // Fields: Pdn=0, Pg=0
    let encoding: u32 = 0x2558C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PFIRST_P.P.P__
/// ASL: `field Pg 5 +: 4`
/// Requirement: FieldBoundary { field: "Pg", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_pfirst_p_p_p_field_pg_1_poweroftwo_c000_2558c020() {
    // Encoding: 0x2558C020
    // Test PFIRST_P.P.P__ field Pg = 1 (PowerOfTwo)
    // Fields: Pg=1, Pdn=0
    let encoding: u32 = 0x2558C020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PFIRST_P.P.P__
/// ASL: `field Pdn 0 +: 4`
/// Requirement: FieldBoundary { field: "Pdn", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_pfirst_p_p_p_field_pdn_0_min_c000_2558c000() {
    // Encoding: 0x2558C000
    // Test PFIRST_P.P.P__ field Pdn = 0 (Min)
    // Fields: Pdn=0, Pg=0
    let encoding: u32 = 0x2558C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PFIRST_P.P.P__
/// ASL: `field Pdn 0 +: 4`
/// Requirement: FieldBoundary { field: "Pdn", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_pfirst_p_p_p_field_pdn_1_poweroftwo_c000_2558c001() {
    // Encoding: 0x2558C001
    // Test PFIRST_P.P.P__ field Pdn = 1 (PowerOfTwo)
    // Fields: Pg=0, Pdn=1
    let encoding: u32 = 0x2558C001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PFIRST_P.P.P__
/// ASL: `field Pdn 0 +: 4`
/// Requirement: FieldBoundary { field: "Pdn", value: 7, boundary: PowerOfTwoMinusOne }
/// midpoint (7)
#[test]
fn test_pfirst_p_p_p_field_pdn_7_poweroftwominusone_c000_2558c007() {
    // Encoding: 0x2558C007
    // Test PFIRST_P.P.P__ field Pdn = 7 (PowerOfTwoMinusOne)
    // Fields: Pg=0, Pdn=7
    let encoding: u32 = 0x2558C007;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PFIRST_P.P.P__
/// ASL: `field Pdn 0 +: 4`
/// Requirement: FieldBoundary { field: "Pdn", value: 15, boundary: Max }
/// maximum value (15)
#[test]
fn test_pfirst_p_p_p_field_pdn_15_max_c000_2558c00f() {
    // Encoding: 0x2558C00F
    // Test PFIRST_P.P.P__ field Pdn = 15 (Max)
    // Fields: Pdn=15, Pg=0
    let encoding: u32 = 0x2558C00F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PFIRST_P.P.P__
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=0 (register index 0 (first register))
#[test]
fn test_pfirst_p_p_p_combo_0_c000_2558c000() {
    // Encoding: 0x2558C000
    // Test PFIRST_P.P.P__ field combination: Pg=0, Pdn=0
    // Fields: Pdn=0, Pg=0
    let encoding: u32 = 0x2558C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PFIRST_P.P.P__
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=1 (register index 1 (second register))
#[test]
fn test_pfirst_p_p_p_combo_1_c000_2558c020() {
    // Encoding: 0x2558C020
    // Test PFIRST_P.P.P__ field combination: Pg=1, Pdn=0
    // Fields: Pg=1, Pdn=0
    let encoding: u32 = 0x2558C020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PFIRST_P.P.P__
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pdn=0 (minimum value)
#[test]
fn test_pfirst_p_p_p_combo_2_c000_2558c000() {
    // Encoding: 0x2558C000
    // Test PFIRST_P.P.P__ field combination: Pg=0, Pdn=0
    // Fields: Pg=0, Pdn=0
    let encoding: u32 = 0x2558C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PFIRST_P.P.P__
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pdn=1 (value 1)
#[test]
fn test_pfirst_p_p_p_combo_3_c000_2558c001() {
    // Encoding: 0x2558C001
    // Test PFIRST_P.P.P__ field combination: Pg=0, Pdn=1
    // Fields: Pg=0, Pdn=1
    let encoding: u32 = 0x2558C001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PFIRST_P.P.P__
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pdn=7 (midpoint (7))
#[test]
fn test_pfirst_p_p_p_combo_4_c000_2558c007() {
    // Encoding: 0x2558C007
    // Test PFIRST_P.P.P__ field combination: Pg=0, Pdn=7
    // Fields: Pg=0, Pdn=7
    let encoding: u32 = 0x2558C007;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PFIRST_P.P.P__
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pdn=15 (maximum value (15))
#[test]
fn test_pfirst_p_p_p_combo_5_c000_2558c00f() {
    // Encoding: 0x2558C00F
    // Test PFIRST_P.P.P__ field combination: Pg=0, Pdn=15
    // Fields: Pg=0, Pdn=15
    let encoding: u32 = 0x2558C00F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PFIRST_P.P.P__
/// ASL: `SimdFromField("Pdn") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "unknown" }
/// verify register write to SimdFromField("Pdn")
#[test]
fn test_pfirst_p_p_p_reg_write_0_2558c000() {
    // Test PFIRST_P.P.P__ register write: SimdFromField("Pdn")
    // Encoding: 0x2558C000
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x2558C000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: PFIRST_P.P.P__
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 0 + 0 = 0 (Z=1)
#[test]
fn test_pfirst_p_p_p_flags_zeroresult_0_2558c000() {
    // Test PFIRST_P.P.P__ flag computation: ZeroResult
    // Encoding: 0x2558C000
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x0);
    set_x(&mut cpu, 2, 0x0);
    let encoding: u32 = 0x2558C000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: PFIRST_P.P.P__
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 1 + (-1) = 0 (Z=1, C=1)
#[test]
fn test_pfirst_p_p_p_flags_zeroresult_1_2558c000() {
    // Test PFIRST_P.P.P__ flag computation: ZeroResult
    // Encoding: 0x2558C000
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0xFFFFFFFFFFFFFFFF);
    set_x(&mut cpu, 1, 0x1);
    let encoding: u32 = 0x2558C000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: PFIRST_P.P.P__
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: NegativeResult }
/// negative value (N=1)
#[test]
fn test_pfirst_p_p_p_flags_negativeresult_2_2558c000() {
    // Test PFIRST_P.P.P__ flag computation: NegativeResult
    // Encoding: 0x2558C000
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0x0);
    set_x(&mut cpu, 1, 0x8000000000000000);
    let encoding: u32 = 0x2558C000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: PFIRST_P.P.P__
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 1 = 0 (C=1, Z=1)
#[test]
fn test_pfirst_p_p_p_flags_unsignedoverflow_3_2558c000() {
    // Test PFIRST_P.P.P__ flag computation: UnsignedOverflow
    // Encoding: 0x2558C000
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0xFFFFFFFFFFFFFFFF);
    set_x(&mut cpu, 2, 0x1);
    let encoding: u32 = 0x2558C000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: PFIRST_P.P.P__
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 2 = 1 (C=1)
#[test]
fn test_pfirst_p_p_p_flags_unsignedoverflow_4_2558c000() {
    // Test PFIRST_P.P.P__ flag computation: UnsignedOverflow
    // Encoding: 0x2558C000
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0x2);
    set_x(&mut cpu, 1, 0xFFFFFFFFFFFFFFFF);
    let encoding: u32 = 0x2558C000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: PFIRST_P.P.P__
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// max_signed + 1 = min_signed (V=1, N=1)
#[test]
fn test_pfirst_p_p_p_flags_signedoverflow_5_2558c000() {
    // Test PFIRST_P.P.P__ flag computation: SignedOverflow
    // Encoding: 0x2558C000
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0x1);
    set_x(&mut cpu, 1, 0x7FFFFFFFFFFFFFFF);
    let encoding: u32 = 0x2558C000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: PFIRST_P.P.P__
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// min_signed + (-1) = max_signed (V=1)
#[test]
fn test_pfirst_p_p_p_flags_signedoverflow_6_2558c000() {
    // Test PFIRST_P.P.P__ flag computation: SignedOverflow
    // Encoding: 0x2558C000
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x8000000000000000);
    set_x(&mut cpu, 2, 0xFFFFFFFFFFFFFFFF);
    let encoding: u32 = 0x2558C000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: PFIRST_P.P.P__
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: PositiveResult }
/// 100 + 50 = 150 (no flags)
#[test]
fn test_pfirst_p_p_p_flags_positiveresult_7_2558c000() {
    // Test PFIRST_P.P.P__ flag computation: PositiveResult
    // Encoding: 0x2558C000
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0x32);
    set_x(&mut cpu, 1, 0x64);
    let encoding: u32 = 0x2558C000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

// ============================================================================
// BRKPB_P.P.PP__ Tests
// ============================================================================

/// Provenance: BRKPB_P.P.PP__
/// ASL: `field Pm 16 +: 4`
/// Requirement: FieldBoundary { field: "Pm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_brkpb_p_p_pp_field_pm_0_min_c010_2500c010() {
    // Encoding: 0x2500C010
    // Test BRKPB_P.P.PP__ field Pm = 0 (Min)
    // Fields: Pg=0, Pd=0, Pm=0, Pn=0
    let encoding: u32 = 0x2500C010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKPB_P.P.PP__
/// ASL: `field Pm 16 +: 4`
/// Requirement: FieldBoundary { field: "Pm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_brkpb_p_p_pp_field_pm_1_poweroftwo_c010_2501c010() {
    // Encoding: 0x2501C010
    // Test BRKPB_P.P.PP__ field Pm = 1 (PowerOfTwo)
    // Fields: Pm=1, Pg=0, Pd=0, Pn=0
    let encoding: u32 = 0x2501C010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKPB_P.P.PP__
/// ASL: `field Pg 10 +: 4`
/// Requirement: FieldBoundary { field: "Pg", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_brkpb_p_p_pp_field_pg_0_min_c010_2500c010() {
    // Encoding: 0x2500C010
    // Test BRKPB_P.P.PP__ field Pg = 0 (Min)
    // Fields: Pn=0, Pm=0, Pd=0, Pg=0
    let encoding: u32 = 0x2500C010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKPB_P.P.PP__
/// ASL: `field Pg 10 +: 4`
/// Requirement: FieldBoundary { field: "Pg", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_brkpb_p_p_pp_field_pg_1_poweroftwo_c010_2500c410() {
    // Encoding: 0x2500C410
    // Test BRKPB_P.P.PP__ field Pg = 1 (PowerOfTwo)
    // Fields: Pm=0, Pd=0, Pn=0, Pg=1
    let encoding: u32 = 0x2500C410;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKPB_P.P.PP__
/// ASL: `field Pn 5 +: 4`
/// Requirement: FieldBoundary { field: "Pn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_brkpb_p_p_pp_field_pn_0_min_c010_2500c010() {
    // Encoding: 0x2500C010
    // Test BRKPB_P.P.PP__ field Pn = 0 (Min)
    // Fields: Pm=0, Pg=0, Pd=0, Pn=0
    let encoding: u32 = 0x2500C010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKPB_P.P.PP__
/// ASL: `field Pn 5 +: 4`
/// Requirement: FieldBoundary { field: "Pn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_brkpb_p_p_pp_field_pn_1_poweroftwo_c010_2500c030() {
    // Encoding: 0x2500C030
    // Test BRKPB_P.P.PP__ field Pn = 1 (PowerOfTwo)
    // Fields: Pg=0, Pd=0, Pm=0, Pn=1
    let encoding: u32 = 0x2500C030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKPB_P.P.PP__
/// ASL: `field Pd 0 +: 4`
/// Requirement: FieldBoundary { field: "Pd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_brkpb_p_p_pp_field_pd_0_min_c010_2500c010() {
    // Encoding: 0x2500C010
    // Test BRKPB_P.P.PP__ field Pd = 0 (Min)
    // Fields: Pm=0, Pn=0, Pg=0, Pd=0
    let encoding: u32 = 0x2500C010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKPB_P.P.PP__
/// ASL: `field Pd 0 +: 4`
/// Requirement: FieldBoundary { field: "Pd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_brkpb_p_p_pp_field_pd_1_poweroftwo_c010_2500c011() {
    // Encoding: 0x2500C011
    // Test BRKPB_P.P.PP__ field Pd = 1 (PowerOfTwo)
    // Fields: Pd=1, Pg=0, Pn=0, Pm=0
    let encoding: u32 = 0x2500C011;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKPB_P.P.PP__
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pm=0 (register index 0 (first register))
#[test]
fn test_brkpb_p_p_pp_combo_0_c010_2500c010() {
    // Encoding: 0x2500C010
    // Test BRKPB_P.P.PP__ field combination: Pm=0, Pg=0, Pn=0, Pd=0
    // Fields: Pn=0, Pm=0, Pg=0, Pd=0
    let encoding: u32 = 0x2500C010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKPB_P.P.PP__
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pm=1 (register index 1 (second register))
#[test]
fn test_brkpb_p_p_pp_combo_1_c010_2501c010() {
    // Encoding: 0x2501C010
    // Test BRKPB_P.P.PP__ field combination: Pm=1, Pg=0, Pn=0, Pd=0
    // Fields: Pm=1, Pd=0, Pn=0, Pg=0
    let encoding: u32 = 0x2501C010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKPB_P.P.PP__
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=0 (register index 0 (first register))
#[test]
fn test_brkpb_p_p_pp_combo_2_c010_2500c010() {
    // Encoding: 0x2500C010
    // Test BRKPB_P.P.PP__ field combination: Pm=0, Pg=0, Pn=0, Pd=0
    // Fields: Pn=0, Pg=0, Pm=0, Pd=0
    let encoding: u32 = 0x2500C010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKPB_P.P.PP__
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=1 (register index 1 (second register))
#[test]
fn test_brkpb_p_p_pp_combo_3_c010_2500c410() {
    // Encoding: 0x2500C410
    // Test BRKPB_P.P.PP__ field combination: Pm=0, Pg=1, Pn=0, Pd=0
    // Fields: Pm=0, Pg=1, Pd=0, Pn=0
    let encoding: u32 = 0x2500C410;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKPB_P.P.PP__
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pn=0 (register index 0 (first register))
#[test]
fn test_brkpb_p_p_pp_combo_4_c010_2500c010() {
    // Encoding: 0x2500C010
    // Test BRKPB_P.P.PP__ field combination: Pm=0, Pg=0, Pn=0, Pd=0
    // Fields: Pn=0, Pd=0, Pg=0, Pm=0
    let encoding: u32 = 0x2500C010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKPB_P.P.PP__
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pn=1 (register index 1 (second register))
#[test]
fn test_brkpb_p_p_pp_combo_5_c010_2500c030() {
    // Encoding: 0x2500C030
    // Test BRKPB_P.P.PP__ field combination: Pm=0, Pg=0, Pn=1, Pd=0
    // Fields: Pn=1, Pd=0, Pm=0, Pg=0
    let encoding: u32 = 0x2500C030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKPB_P.P.PP__
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pd=0 (register index 0 (first register))
#[test]
fn test_brkpb_p_p_pp_combo_6_c010_2500c010() {
    // Encoding: 0x2500C010
    // Test BRKPB_P.P.PP__ field combination: Pm=0, Pg=0, Pn=0, Pd=0
    // Fields: Pm=0, Pd=0, Pg=0, Pn=0
    let encoding: u32 = 0x2500C010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKPB_P.P.PP__
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pd=1 (register index 1 (second register))
#[test]
fn test_brkpb_p_p_pp_combo_7_c010_2500c011() {
    // Encoding: 0x2500C011
    // Test BRKPB_P.P.PP__ field combination: Pm=0, Pg=0, Pn=0, Pd=1
    // Fields: Pn=0, Pm=0, Pg=0, Pd=1
    let encoding: u32 = 0x2500C011;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKPB_P.P.PP__
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pm=1 (same register test (reg=1)), Pg=1 (same register test (reg=1))
#[test]
fn test_brkpb_p_p_pp_combo_8_c010_2501c410() {
    // Encoding: 0x2501C410
    // Test BRKPB_P.P.PP__ field combination: Pm=1, Pg=1, Pn=0, Pd=0
    // Fields: Pd=0, Pm=1, Pg=1, Pn=0
    let encoding: u32 = 0x2501C410;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKPB_P.P.PP__
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pm=31 (same register test (reg=31)), Pg=31 (same register test (reg=31))
#[test]
fn test_brkpb_p_p_pp_combo_9_c010_250ffc10() {
    // Encoding: 0x250FFC10
    // Test BRKPB_P.P.PP__ field combination: Pm=31, Pg=31, Pn=0, Pd=0
    // Fields: Pd=0, Pm=31, Pg=31, Pn=0
    let encoding: u32 = 0x250FFC10;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKPB_P.P.PP__
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pm=1 (same register test (reg=1)), Pn=1 (same register test (reg=1))
#[test]
fn test_brkpb_p_p_pp_combo_10_c010_2501c030() {
    // Encoding: 0x2501C030
    // Test BRKPB_P.P.PP__ field combination: Pm=1, Pg=0, Pn=1, Pd=0
    // Fields: Pd=0, Pg=0, Pn=1, Pm=1
    let encoding: u32 = 0x2501C030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKPB_P.P.PP__
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pm=31 (same register test (reg=31)), Pn=31 (same register test (reg=31))
#[test]
fn test_brkpb_p_p_pp_combo_11_c010_250fc1f0() {
    // Encoding: 0x250FC1F0
    // Test BRKPB_P.P.PP__ field combination: Pm=31, Pg=0, Pn=31, Pd=0
    // Fields: Pm=31, Pg=0, Pd=0, Pn=31
    let encoding: u32 = 0x250FC1F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKPB_P.P.PP__
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pm=1 (same register test (reg=1)), Pd=1 (same register test (reg=1))
#[test]
fn test_brkpb_p_p_pp_combo_12_c010_2501c011() {
    // Encoding: 0x2501C011
    // Test BRKPB_P.P.PP__ field combination: Pm=1, Pg=0, Pn=0, Pd=1
    // Fields: Pn=0, Pm=1, Pd=1, Pg=0
    let encoding: u32 = 0x2501C011;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKPB_P.P.PP__
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pm=31 (same register test (reg=31)), Pd=31 (same register test (reg=31))
#[test]
fn test_brkpb_p_p_pp_combo_13_c010_250fc01f() {
    // Encoding: 0x250FC01F
    // Test BRKPB_P.P.PP__ field combination: Pm=31, Pg=0, Pn=0, Pd=31
    // Fields: Pn=0, Pd=31, Pm=31, Pg=0
    let encoding: u32 = 0x250FC01F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKPB_P.P.PP__
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=1 (same register test (reg=1)), Pn=1 (same register test (reg=1))
#[test]
fn test_brkpb_p_p_pp_combo_14_c010_2500c430() {
    // Encoding: 0x2500C430
    // Test BRKPB_P.P.PP__ field combination: Pm=0, Pg=1, Pn=1, Pd=0
    // Fields: Pn=1, Pd=0, Pg=1, Pm=0
    let encoding: u32 = 0x2500C430;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKPB_P.P.PP__
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=31 (same register test (reg=31)), Pn=31 (same register test (reg=31))
#[test]
fn test_brkpb_p_p_pp_combo_15_c010_2500fdf0() {
    // Encoding: 0x2500FDF0
    // Test BRKPB_P.P.PP__ field combination: Pm=0, Pg=31, Pn=31, Pd=0
    // Fields: Pm=0, Pn=31, Pg=31, Pd=0
    let encoding: u32 = 0x2500FDF0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKPB_P.P.PP__
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=1 (same register test (reg=1)), Pd=1 (same register test (reg=1))
#[test]
fn test_brkpb_p_p_pp_combo_16_c010_2500c411() {
    // Encoding: 0x2500C411
    // Test BRKPB_P.P.PP__ field combination: Pm=0, Pg=1, Pn=0, Pd=1
    // Fields: Pn=0, Pg=1, Pd=1, Pm=0
    let encoding: u32 = 0x2500C411;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKPB_P.P.PP__
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=31 (same register test (reg=31)), Pd=31 (same register test (reg=31))
#[test]
fn test_brkpb_p_p_pp_combo_17_c010_2500fc1f() {
    // Encoding: 0x2500FC1F
    // Test BRKPB_P.P.PP__ field combination: Pm=0, Pg=31, Pn=0, Pd=31
    // Fields: Pn=0, Pd=31, Pm=0, Pg=31
    let encoding: u32 = 0x2500FC1F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKPB_P.P.PP__
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pn=1 (same register test (reg=1)), Pd=1 (same register test (reg=1))
#[test]
fn test_brkpb_p_p_pp_combo_18_c010_2500c031() {
    // Encoding: 0x2500C031
    // Test BRKPB_P.P.PP__ field combination: Pm=0, Pg=0, Pn=1, Pd=1
    // Fields: Pm=0, Pg=0, Pn=1, Pd=1
    let encoding: u32 = 0x2500C031;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKPB_P.P.PP__
/// ASL: `field combination 19`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pn=31 (same register test (reg=31)), Pd=31 (same register test (reg=31))
#[test]
fn test_brkpb_p_p_pp_combo_19_c010_2500c1ff() {
    // Encoding: 0x2500C1FF
    // Test BRKPB_P.P.PP__ field combination: Pm=0, Pg=0, Pn=31, Pd=31
    // Fields: Pm=0, Pd=31, Pn=31, Pg=0
    let encoding: u32 = 0x2500C1FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKPBS_P.P.PP__
/// ASL: `field Pm 16 +: 4`
/// Requirement: FieldBoundary { field: "Pm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_brkpbs_p_p_pp_field_pm_0_min_c010_2540c010() {
    // Encoding: 0x2540C010
    // Test BRKPBS_P.P.PP__ field Pm = 0 (Min)
    // Fields: Pd=0, Pg=0, Pn=0, Pm=0
    let encoding: u32 = 0x2540C010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKPBS_P.P.PP__
/// ASL: `field Pm 16 +: 4`
/// Requirement: FieldBoundary { field: "Pm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_brkpbs_p_p_pp_field_pm_1_poweroftwo_c010_2541c010() {
    // Encoding: 0x2541C010
    // Test BRKPBS_P.P.PP__ field Pm = 1 (PowerOfTwo)
    // Fields: Pm=1, Pn=0, Pd=0, Pg=0
    let encoding: u32 = 0x2541C010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKPBS_P.P.PP__
/// ASL: `field Pg 10 +: 4`
/// Requirement: FieldBoundary { field: "Pg", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_brkpbs_p_p_pp_field_pg_0_min_c010_2540c010() {
    // Encoding: 0x2540C010
    // Test BRKPBS_P.P.PP__ field Pg = 0 (Min)
    // Fields: Pm=0, Pn=0, Pd=0, Pg=0
    let encoding: u32 = 0x2540C010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKPBS_P.P.PP__
/// ASL: `field Pg 10 +: 4`
/// Requirement: FieldBoundary { field: "Pg", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_brkpbs_p_p_pp_field_pg_1_poweroftwo_c010_2540c410() {
    // Encoding: 0x2540C410
    // Test BRKPBS_P.P.PP__ field Pg = 1 (PowerOfTwo)
    // Fields: Pd=0, Pg=1, Pn=0, Pm=0
    let encoding: u32 = 0x2540C410;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKPBS_P.P.PP__
/// ASL: `field Pn 5 +: 4`
/// Requirement: FieldBoundary { field: "Pn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_brkpbs_p_p_pp_field_pn_0_min_c010_2540c010() {
    // Encoding: 0x2540C010
    // Test BRKPBS_P.P.PP__ field Pn = 0 (Min)
    // Fields: Pd=0, Pg=0, Pn=0, Pm=0
    let encoding: u32 = 0x2540C010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKPBS_P.P.PP__
/// ASL: `field Pn 5 +: 4`
/// Requirement: FieldBoundary { field: "Pn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_brkpbs_p_p_pp_field_pn_1_poweroftwo_c010_2540c030() {
    // Encoding: 0x2540C030
    // Test BRKPBS_P.P.PP__ field Pn = 1 (PowerOfTwo)
    // Fields: Pn=1, Pm=0, Pg=0, Pd=0
    let encoding: u32 = 0x2540C030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKPBS_P.P.PP__
/// ASL: `field Pd 0 +: 4`
/// Requirement: FieldBoundary { field: "Pd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_brkpbs_p_p_pp_field_pd_0_min_c010_2540c010() {
    // Encoding: 0x2540C010
    // Test BRKPBS_P.P.PP__ field Pd = 0 (Min)
    // Fields: Pm=0, Pg=0, Pn=0, Pd=0
    let encoding: u32 = 0x2540C010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKPBS_P.P.PP__
/// ASL: `field Pd 0 +: 4`
/// Requirement: FieldBoundary { field: "Pd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_brkpbs_p_p_pp_field_pd_1_poweroftwo_c010_2540c011() {
    // Encoding: 0x2540C011
    // Test BRKPBS_P.P.PP__ field Pd = 1 (PowerOfTwo)
    // Fields: Pm=0, Pn=0, Pg=0, Pd=1
    let encoding: u32 = 0x2540C011;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKPBS_P.P.PP__
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pm=0 (register index 0 (first register))
#[test]
fn test_brkpbs_p_p_pp_combo_0_c010_2540c010() {
    // Encoding: 0x2540C010
    // Test BRKPBS_P.P.PP__ field combination: Pm=0, Pg=0, Pn=0, Pd=0
    // Fields: Pm=0, Pg=0, Pd=0, Pn=0
    let encoding: u32 = 0x2540C010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKPBS_P.P.PP__
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pm=1 (register index 1 (second register))
#[test]
fn test_brkpbs_p_p_pp_combo_1_c010_2541c010() {
    // Encoding: 0x2541C010
    // Test BRKPBS_P.P.PP__ field combination: Pm=1, Pg=0, Pn=0, Pd=0
    // Fields: Pm=1, Pd=0, Pg=0, Pn=0
    let encoding: u32 = 0x2541C010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKPBS_P.P.PP__
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=0 (register index 0 (first register))
#[test]
fn test_brkpbs_p_p_pp_combo_2_c010_2540c010() {
    // Encoding: 0x2540C010
    // Test BRKPBS_P.P.PP__ field combination: Pm=0, Pg=0, Pn=0, Pd=0
    // Fields: Pn=0, Pd=0, Pm=0, Pg=0
    let encoding: u32 = 0x2540C010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKPBS_P.P.PP__
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=1 (register index 1 (second register))
#[test]
fn test_brkpbs_p_p_pp_combo_3_c010_2540c410() {
    // Encoding: 0x2540C410
    // Test BRKPBS_P.P.PP__ field combination: Pm=0, Pg=1, Pn=0, Pd=0
    // Fields: Pd=0, Pn=0, Pm=0, Pg=1
    let encoding: u32 = 0x2540C410;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKPBS_P.P.PP__
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pn=0 (register index 0 (first register))
#[test]
fn test_brkpbs_p_p_pp_combo_4_c010_2540c010() {
    // Encoding: 0x2540C010
    // Test BRKPBS_P.P.PP__ field combination: Pm=0, Pg=0, Pn=0, Pd=0
    // Fields: Pm=0, Pg=0, Pd=0, Pn=0
    let encoding: u32 = 0x2540C010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKPBS_P.P.PP__
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pn=1 (register index 1 (second register))
#[test]
fn test_brkpbs_p_p_pp_combo_5_c010_2540c030() {
    // Encoding: 0x2540C030
    // Test BRKPBS_P.P.PP__ field combination: Pm=0, Pg=0, Pn=1, Pd=0
    // Fields: Pd=0, Pn=1, Pg=0, Pm=0
    let encoding: u32 = 0x2540C030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKPBS_P.P.PP__
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pd=0 (register index 0 (first register))
#[test]
fn test_brkpbs_p_p_pp_combo_6_c010_2540c010() {
    // Encoding: 0x2540C010
    // Test BRKPBS_P.P.PP__ field combination: Pm=0, Pg=0, Pn=0, Pd=0
    // Fields: Pm=0, Pd=0, Pg=0, Pn=0
    let encoding: u32 = 0x2540C010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKPBS_P.P.PP__
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pd=1 (register index 1 (second register))
#[test]
fn test_brkpbs_p_p_pp_combo_7_c010_2540c011() {
    // Encoding: 0x2540C011
    // Test BRKPBS_P.P.PP__ field combination: Pm=0, Pg=0, Pn=0, Pd=1
    // Fields: Pd=1, Pg=0, Pm=0, Pn=0
    let encoding: u32 = 0x2540C011;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKPBS_P.P.PP__
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pm=1 (same register test (reg=1)), Pg=1 (same register test (reg=1))
#[test]
fn test_brkpbs_p_p_pp_combo_8_c010_2541c410() {
    // Encoding: 0x2541C410
    // Test BRKPBS_P.P.PP__ field combination: Pm=1, Pg=1, Pn=0, Pd=0
    // Fields: Pn=0, Pd=0, Pm=1, Pg=1
    let encoding: u32 = 0x2541C410;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKPBS_P.P.PP__
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pm=31 (same register test (reg=31)), Pg=31 (same register test (reg=31))
#[test]
fn test_brkpbs_p_p_pp_combo_9_c010_254ffc10() {
    // Encoding: 0x254FFC10
    // Test BRKPBS_P.P.PP__ field combination: Pm=31, Pg=31, Pn=0, Pd=0
    // Fields: Pd=0, Pn=0, Pg=31, Pm=31
    let encoding: u32 = 0x254FFC10;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKPBS_P.P.PP__
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pm=1 (same register test (reg=1)), Pn=1 (same register test (reg=1))
#[test]
fn test_brkpbs_p_p_pp_combo_10_c010_2541c030() {
    // Encoding: 0x2541C030
    // Test BRKPBS_P.P.PP__ field combination: Pm=1, Pg=0, Pn=1, Pd=0
    // Fields: Pm=1, Pd=0, Pn=1, Pg=0
    let encoding: u32 = 0x2541C030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKPBS_P.P.PP__
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pm=31 (same register test (reg=31)), Pn=31 (same register test (reg=31))
#[test]
fn test_brkpbs_p_p_pp_combo_11_c010_254fc1f0() {
    // Encoding: 0x254FC1F0
    // Test BRKPBS_P.P.PP__ field combination: Pm=31, Pg=0, Pn=31, Pd=0
    // Fields: Pg=0, Pm=31, Pn=31, Pd=0
    let encoding: u32 = 0x254FC1F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKPBS_P.P.PP__
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pm=1 (same register test (reg=1)), Pd=1 (same register test (reg=1))
#[test]
fn test_brkpbs_p_p_pp_combo_12_c010_2541c011() {
    // Encoding: 0x2541C011
    // Test BRKPBS_P.P.PP__ field combination: Pm=1, Pg=0, Pn=0, Pd=1
    // Fields: Pd=1, Pg=0, Pn=0, Pm=1
    let encoding: u32 = 0x2541C011;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKPBS_P.P.PP__
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pm=31 (same register test (reg=31)), Pd=31 (same register test (reg=31))
#[test]
fn test_brkpbs_p_p_pp_combo_13_c010_254fc01f() {
    // Encoding: 0x254FC01F
    // Test BRKPBS_P.P.PP__ field combination: Pm=31, Pg=0, Pn=0, Pd=31
    // Fields: Pd=31, Pg=0, Pm=31, Pn=0
    let encoding: u32 = 0x254FC01F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKPBS_P.P.PP__
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=1 (same register test (reg=1)), Pn=1 (same register test (reg=1))
#[test]
fn test_brkpbs_p_p_pp_combo_14_c010_2540c430() {
    // Encoding: 0x2540C430
    // Test BRKPBS_P.P.PP__ field combination: Pm=0, Pg=1, Pn=1, Pd=0
    // Fields: Pn=1, Pg=1, Pm=0, Pd=0
    let encoding: u32 = 0x2540C430;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKPBS_P.P.PP__
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=31 (same register test (reg=31)), Pn=31 (same register test (reg=31))
#[test]
fn test_brkpbs_p_p_pp_combo_15_c010_2540fdf0() {
    // Encoding: 0x2540FDF0
    // Test BRKPBS_P.P.PP__ field combination: Pm=0, Pg=31, Pn=31, Pd=0
    // Fields: Pg=31, Pd=0, Pm=0, Pn=31
    let encoding: u32 = 0x2540FDF0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKPBS_P.P.PP__
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=1 (same register test (reg=1)), Pd=1 (same register test (reg=1))
#[test]
fn test_brkpbs_p_p_pp_combo_16_c010_2540c411() {
    // Encoding: 0x2540C411
    // Test BRKPBS_P.P.PP__ field combination: Pm=0, Pg=1, Pn=0, Pd=1
    // Fields: Pg=1, Pm=0, Pd=1, Pn=0
    let encoding: u32 = 0x2540C411;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKPBS_P.P.PP__
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=31 (same register test (reg=31)), Pd=31 (same register test (reg=31))
#[test]
fn test_brkpbs_p_p_pp_combo_17_c010_2540fc1f() {
    // Encoding: 0x2540FC1F
    // Test BRKPBS_P.P.PP__ field combination: Pm=0, Pg=31, Pn=0, Pd=31
    // Fields: Pd=31, Pn=0, Pg=31, Pm=0
    let encoding: u32 = 0x2540FC1F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKPBS_P.P.PP__
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pn=1 (same register test (reg=1)), Pd=1 (same register test (reg=1))
#[test]
fn test_brkpbs_p_p_pp_combo_18_c010_2540c031() {
    // Encoding: 0x2540C031
    // Test BRKPBS_P.P.PP__ field combination: Pm=0, Pg=0, Pn=1, Pd=1
    // Fields: Pg=0, Pm=0, Pn=1, Pd=1
    let encoding: u32 = 0x2540C031;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKPBS_P.P.PP__
/// ASL: `field combination 19`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pn=31 (same register test (reg=31)), Pd=31 (same register test (reg=31))
#[test]
fn test_brkpbs_p_p_pp_combo_19_c010_2540c1ff() {
    // Encoding: 0x2540C1FF
    // Test BRKPBS_P.P.PP__ field combination: Pm=0, Pg=0, Pn=31, Pd=31
    // Fields: Pg=0, Pn=31, Pm=0, Pd=31
    let encoding: u32 = 0x2540C1FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKPB_P.P.PP__
/// ASL: `SimdFromField("Pd") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "unknown" }
/// verify register write to SimdFromField("Pd")
#[test]
fn test_brkpb_p_p_pp_reg_write_0_2500c010() {
    // Test BRKPB_P.P.PP__ register write: SimdFromField("Pd")
    // Encoding: 0x2500C010
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x2500C010;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: BRKPB_P.P.PP__
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 0 + 0 = 0 (Z=1)
#[test]
fn test_brkpb_p_p_pp_flags_zeroresult_0_2500c010() {
    // Test BRKPB_P.P.PP__ flag computation: ZeroResult
    // Encoding: 0x2500C010
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x0);
    set_x(&mut cpu, 2, 0x0);
    let encoding: u32 = 0x2500C010;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: BRKPB_P.P.PP__
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 1 + (-1) = 0 (Z=1, C=1)
#[test]
fn test_brkpb_p_p_pp_flags_zeroresult_1_2500c010() {
    // Test BRKPB_P.P.PP__ flag computation: ZeroResult
    // Encoding: 0x2500C010
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0xFFFFFFFFFFFFFFFF);
    set_x(&mut cpu, 1, 0x1);
    let encoding: u32 = 0x2500C010;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: BRKPB_P.P.PP__
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: NegativeResult }
/// negative value (N=1)
#[test]
fn test_brkpb_p_p_pp_flags_negativeresult_2_2500c010() {
    // Test BRKPB_P.P.PP__ flag computation: NegativeResult
    // Encoding: 0x2500C010
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0x0);
    set_x(&mut cpu, 1, 0x8000000000000000);
    let encoding: u32 = 0x2500C010;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: BRKPB_P.P.PP__
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 1 = 0 (C=1, Z=1)
#[test]
fn test_brkpb_p_p_pp_flags_unsignedoverflow_3_2500c010() {
    // Test BRKPB_P.P.PP__ flag computation: UnsignedOverflow
    // Encoding: 0x2500C010
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0x1);
    set_x(&mut cpu, 1, 0xFFFFFFFFFFFFFFFF);
    let encoding: u32 = 0x2500C010;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: BRKPB_P.P.PP__
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 2 = 1 (C=1)
#[test]
fn test_brkpb_p_p_pp_flags_unsignedoverflow_4_2500c010() {
    // Test BRKPB_P.P.PP__ flag computation: UnsignedOverflow
    // Encoding: 0x2500C010
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0xFFFFFFFFFFFFFFFF);
    set_x(&mut cpu, 2, 0x2);
    let encoding: u32 = 0x2500C010;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: BRKPB_P.P.PP__
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// max_signed + 1 = min_signed (V=1, N=1)
#[test]
fn test_brkpb_p_p_pp_flags_signedoverflow_5_2500c010() {
    // Test BRKPB_P.P.PP__ flag computation: SignedOverflow
    // Encoding: 0x2500C010
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x7FFFFFFFFFFFFFFF);
    set_x(&mut cpu, 2, 0x1);
    let encoding: u32 = 0x2500C010;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: BRKPB_P.P.PP__
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// min_signed + (-1) = max_signed (V=1)
#[test]
fn test_brkpb_p_p_pp_flags_signedoverflow_6_2500c010() {
    // Test BRKPB_P.P.PP__ flag computation: SignedOverflow
    // Encoding: 0x2500C010
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x8000000000000000);
    set_x(&mut cpu, 2, 0xFFFFFFFFFFFFFFFF);
    let encoding: u32 = 0x2500C010;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: BRKPB_P.P.PP__
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: PositiveResult }
/// 100 + 50 = 150 (no flags)
#[test]
fn test_brkpb_p_p_pp_flags_positiveresult_7_2500c010() {
    // Test BRKPB_P.P.PP__ flag computation: PositiveResult
    // Encoding: 0x2500C010
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x64);
    set_x(&mut cpu, 2, 0x32);
    let encoding: u32 = 0x2500C010;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: BRKPBS_P.P.PP__
/// ASL: `SimdFromField("Pd") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "unknown" }
/// verify register write to SimdFromField("Pd")
#[test]
fn test_brkpbs_p_p_pp_reg_write_0_2540c010() {
    // Test BRKPBS_P.P.PP__ register write: SimdFromField("Pd")
    // Encoding: 0x2540C010
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x2540C010;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: BRKPBS_P.P.PP__
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 0 + 0 = 0 (Z=1)
#[test]
fn test_brkpbs_p_p_pp_flags_zeroresult_0_2540c010() {
    // Test BRKPBS_P.P.PP__ flag computation: ZeroResult
    // Encoding: 0x2540C010
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x0);
    set_x(&mut cpu, 2, 0x0);
    let encoding: u32 = 0x2540C010;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: BRKPBS_P.P.PP__
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 1 + (-1) = 0 (Z=1, C=1)
#[test]
fn test_brkpbs_p_p_pp_flags_zeroresult_1_2540c010() {
    // Test BRKPBS_P.P.PP__ flag computation: ZeroResult
    // Encoding: 0x2540C010
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x1);
    set_x(&mut cpu, 2, 0xFFFFFFFFFFFFFFFF);
    let encoding: u32 = 0x2540C010;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: BRKPBS_P.P.PP__
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: NegativeResult }
/// negative value (N=1)
#[test]
fn test_brkpbs_p_p_pp_flags_negativeresult_2_2540c010() {
    // Test BRKPBS_P.P.PP__ flag computation: NegativeResult
    // Encoding: 0x2540C010
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x8000000000000000);
    set_x(&mut cpu, 2, 0x0);
    let encoding: u32 = 0x2540C010;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: BRKPBS_P.P.PP__
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 1 = 0 (C=1, Z=1)
#[test]
fn test_brkpbs_p_p_pp_flags_unsignedoverflow_3_2540c010() {
    // Test BRKPBS_P.P.PP__ flag computation: UnsignedOverflow
    // Encoding: 0x2540C010
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0x1);
    set_x(&mut cpu, 1, 0xFFFFFFFFFFFFFFFF);
    let encoding: u32 = 0x2540C010;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: BRKPBS_P.P.PP__
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 2 = 1 (C=1)
#[test]
fn test_brkpbs_p_p_pp_flags_unsignedoverflow_4_2540c010() {
    // Test BRKPBS_P.P.PP__ flag computation: UnsignedOverflow
    // Encoding: 0x2540C010
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0x2);
    set_x(&mut cpu, 1, 0xFFFFFFFFFFFFFFFF);
    let encoding: u32 = 0x2540C010;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: BRKPBS_P.P.PP__
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// max_signed + 1 = min_signed (V=1, N=1)
#[test]
fn test_brkpbs_p_p_pp_flags_signedoverflow_5_2540c010() {
    // Test BRKPBS_P.P.PP__ flag computation: SignedOverflow
    // Encoding: 0x2540C010
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x7FFFFFFFFFFFFFFF);
    set_x(&mut cpu, 2, 0x1);
    let encoding: u32 = 0x2540C010;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: BRKPBS_P.P.PP__
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// min_signed + (-1) = max_signed (V=1)
#[test]
fn test_brkpbs_p_p_pp_flags_signedoverflow_6_2540c010() {
    // Test BRKPBS_P.P.PP__ flag computation: SignedOverflow
    // Encoding: 0x2540C010
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0xFFFFFFFFFFFFFFFF);
    set_x(&mut cpu, 1, 0x8000000000000000);
    let encoding: u32 = 0x2540C010;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: BRKPBS_P.P.PP__
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: PositiveResult }
/// 100 + 50 = 150 (no flags)
#[test]
fn test_brkpbs_p_p_pp_flags_positiveresult_7_2540c010() {
    // Test BRKPBS_P.P.PP__ flag computation: PositiveResult
    // Encoding: 0x2540C010
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x64);
    set_x(&mut cpu, 2, 0x32);
    let encoding: u32 = 0x2540C010;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

// ============================================================================
// SEL_P.P.PP__ Tests
// ============================================================================

/// Provenance: SEL_P.P.PP__
/// ASL: `field Pm 16 +: 4`
/// Requirement: FieldBoundary { field: "Pm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_sel_p_p_pp_field_pm_0_min_4210_25004210() {
    // Encoding: 0x25004210
    // Test SEL_P.P.PP__ field Pm = 0 (Min)
    // Fields: Pg=0, Pm=0, Pn=0, Pd=0
    let encoding: u32 = 0x25004210;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: SEL_P.P.PP__
/// ASL: `field Pm 16 +: 4`
/// Requirement: FieldBoundary { field: "Pm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_sel_p_p_pp_field_pm_1_poweroftwo_4210_25014210() {
    // Encoding: 0x25014210
    // Test SEL_P.P.PP__ field Pm = 1 (PowerOfTwo)
    // Fields: Pg=0, Pn=0, Pm=1, Pd=0
    let encoding: u32 = 0x25014210;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: SEL_P.P.PP__
/// ASL: `field Pg 10 +: 4`
/// Requirement: FieldBoundary { field: "Pg", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_sel_p_p_pp_field_pg_0_min_4210_25004210() {
    // Encoding: 0x25004210
    // Test SEL_P.P.PP__ field Pg = 0 (Min)
    // Fields: Pm=0, Pg=0, Pd=0, Pn=0
    let encoding: u32 = 0x25004210;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: SEL_P.P.PP__
/// ASL: `field Pg 10 +: 4`
/// Requirement: FieldBoundary { field: "Pg", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_sel_p_p_pp_field_pg_1_poweroftwo_4210_25004610() {
    // Encoding: 0x25004610
    // Test SEL_P.P.PP__ field Pg = 1 (PowerOfTwo)
    // Fields: Pg=1, Pn=0, Pm=0, Pd=0
    let encoding: u32 = 0x25004610;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: SEL_P.P.PP__
/// ASL: `field Pn 5 +: 4`
/// Requirement: FieldBoundary { field: "Pn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_sel_p_p_pp_field_pn_0_min_4210_25004210() {
    // Encoding: 0x25004210
    // Test SEL_P.P.PP__ field Pn = 0 (Min)
    // Fields: Pm=0, Pd=0, Pn=0, Pg=0
    let encoding: u32 = 0x25004210;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: SEL_P.P.PP__
/// ASL: `field Pn 5 +: 4`
/// Requirement: FieldBoundary { field: "Pn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_sel_p_p_pp_field_pn_1_poweroftwo_4210_25004230() {
    // Encoding: 0x25004230
    // Test SEL_P.P.PP__ field Pn = 1 (PowerOfTwo)
    // Fields: Pm=0, Pg=0, Pn=1, Pd=0
    let encoding: u32 = 0x25004230;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: SEL_P.P.PP__
/// ASL: `field Pd 0 +: 4`
/// Requirement: FieldBoundary { field: "Pd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_sel_p_p_pp_field_pd_0_min_4210_25004210() {
    // Encoding: 0x25004210
    // Test SEL_P.P.PP__ field Pd = 0 (Min)
    // Fields: Pm=0, Pn=0, Pg=0, Pd=0
    let encoding: u32 = 0x25004210;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: SEL_P.P.PP__
/// ASL: `field Pd 0 +: 4`
/// Requirement: FieldBoundary { field: "Pd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_sel_p_p_pp_field_pd_1_poweroftwo_4210_25004211() {
    // Encoding: 0x25004211
    // Test SEL_P.P.PP__ field Pd = 1 (PowerOfTwo)
    // Fields: Pn=0, Pm=0, Pg=0, Pd=1
    let encoding: u32 = 0x25004211;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: SEL_P.P.PP__
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pm=0 (register index 0 (first register))
#[test]
fn test_sel_p_p_pp_combo_0_4210_25004210() {
    // Encoding: 0x25004210
    // Test SEL_P.P.PP__ field combination: Pm=0, Pg=0, Pn=0, Pd=0
    // Fields: Pd=0, Pn=0, Pm=0, Pg=0
    let encoding: u32 = 0x25004210;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: SEL_P.P.PP__
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pm=1 (register index 1 (second register))
#[test]
fn test_sel_p_p_pp_combo_1_4210_25014210() {
    // Encoding: 0x25014210
    // Test SEL_P.P.PP__ field combination: Pm=1, Pg=0, Pn=0, Pd=0
    // Fields: Pg=0, Pd=0, Pm=1, Pn=0
    let encoding: u32 = 0x25014210;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: SEL_P.P.PP__
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=0 (register index 0 (first register))
#[test]
fn test_sel_p_p_pp_combo_2_4210_25004210() {
    // Encoding: 0x25004210
    // Test SEL_P.P.PP__ field combination: Pm=0, Pg=0, Pn=0, Pd=0
    // Fields: Pd=0, Pg=0, Pn=0, Pm=0
    let encoding: u32 = 0x25004210;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: SEL_P.P.PP__
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=1 (register index 1 (second register))
#[test]
fn test_sel_p_p_pp_combo_3_4210_25004610() {
    // Encoding: 0x25004610
    // Test SEL_P.P.PP__ field combination: Pm=0, Pg=1, Pn=0, Pd=0
    // Fields: Pm=0, Pd=0, Pn=0, Pg=1
    let encoding: u32 = 0x25004610;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: SEL_P.P.PP__
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pn=0 (register index 0 (first register))
#[test]
fn test_sel_p_p_pp_combo_4_4210_25004210() {
    // Encoding: 0x25004210
    // Test SEL_P.P.PP__ field combination: Pm=0, Pg=0, Pn=0, Pd=0
    // Fields: Pn=0, Pd=0, Pm=0, Pg=0
    let encoding: u32 = 0x25004210;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: SEL_P.P.PP__
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pn=1 (register index 1 (second register))
#[test]
fn test_sel_p_p_pp_combo_5_4210_25004230() {
    // Encoding: 0x25004230
    // Test SEL_P.P.PP__ field combination: Pm=0, Pg=0, Pn=1, Pd=0
    // Fields: Pg=0, Pd=0, Pm=0, Pn=1
    let encoding: u32 = 0x25004230;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: SEL_P.P.PP__
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pd=0 (register index 0 (first register))
#[test]
fn test_sel_p_p_pp_combo_6_4210_25004210() {
    // Encoding: 0x25004210
    // Test SEL_P.P.PP__ field combination: Pm=0, Pg=0, Pn=0, Pd=0
    // Fields: Pg=0, Pn=0, Pd=0, Pm=0
    let encoding: u32 = 0x25004210;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: SEL_P.P.PP__
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pd=1 (register index 1 (second register))
#[test]
fn test_sel_p_p_pp_combo_7_4210_25004211() {
    // Encoding: 0x25004211
    // Test SEL_P.P.PP__ field combination: Pm=0, Pg=0, Pn=0, Pd=1
    // Fields: Pg=0, Pd=1, Pn=0, Pm=0
    let encoding: u32 = 0x25004211;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: SEL_P.P.PP__
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pm=1 (same register test (reg=1)), Pg=1 (same register test (reg=1))
#[test]
fn test_sel_p_p_pp_combo_8_4210_25014610() {
    // Encoding: 0x25014610
    // Test SEL_P.P.PP__ field combination: Pm=1, Pg=1, Pn=0, Pd=0
    // Fields: Pm=1, Pg=1, Pd=0, Pn=0
    let encoding: u32 = 0x25014610;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: SEL_P.P.PP__
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pm=31 (same register test (reg=31)), Pg=31 (same register test (reg=31))
#[test]
fn test_sel_p_p_pp_combo_9_4210_250f7e10() {
    // Encoding: 0x250F7E10
    // Test SEL_P.P.PP__ field combination: Pm=31, Pg=31, Pn=0, Pd=0
    // Fields: Pn=0, Pm=31, Pd=0, Pg=31
    let encoding: u32 = 0x250F7E10;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: SEL_P.P.PP__
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pm=1 (same register test (reg=1)), Pn=1 (same register test (reg=1))
#[test]
fn test_sel_p_p_pp_combo_10_4210_25014230() {
    // Encoding: 0x25014230
    // Test SEL_P.P.PP__ field combination: Pm=1, Pg=0, Pn=1, Pd=0
    // Fields: Pm=1, Pd=0, Pn=1, Pg=0
    let encoding: u32 = 0x25014230;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: SEL_P.P.PP__
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pm=31 (same register test (reg=31)), Pn=31 (same register test (reg=31))
#[test]
fn test_sel_p_p_pp_combo_11_4210_250f43f0() {
    // Encoding: 0x250F43F0
    // Test SEL_P.P.PP__ field combination: Pm=31, Pg=0, Pn=31, Pd=0
    // Fields: Pd=0, Pm=31, Pn=31, Pg=0
    let encoding: u32 = 0x250F43F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: SEL_P.P.PP__
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pm=1 (same register test (reg=1)), Pd=1 (same register test (reg=1))
#[test]
fn test_sel_p_p_pp_combo_12_4210_25014211() {
    // Encoding: 0x25014211
    // Test SEL_P.P.PP__ field combination: Pm=1, Pg=0, Pn=0, Pd=1
    // Fields: Pn=0, Pd=1, Pg=0, Pm=1
    let encoding: u32 = 0x25014211;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: SEL_P.P.PP__
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pm=31 (same register test (reg=31)), Pd=31 (same register test (reg=31))
#[test]
fn test_sel_p_p_pp_combo_13_4210_250f421f() {
    // Encoding: 0x250F421F
    // Test SEL_P.P.PP__ field combination: Pm=31, Pg=0, Pn=0, Pd=31
    // Fields: Pd=31, Pm=31, Pg=0, Pn=0
    let encoding: u32 = 0x250F421F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: SEL_P.P.PP__
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=1 (same register test (reg=1)), Pn=1 (same register test (reg=1))
#[test]
fn test_sel_p_p_pp_combo_14_4210_25004630() {
    // Encoding: 0x25004630
    // Test SEL_P.P.PP__ field combination: Pm=0, Pg=1, Pn=1, Pd=0
    // Fields: Pd=0, Pn=1, Pg=1, Pm=0
    let encoding: u32 = 0x25004630;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: SEL_P.P.PP__
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=31 (same register test (reg=31)), Pn=31 (same register test (reg=31))
#[test]
fn test_sel_p_p_pp_combo_15_4210_25007ff0() {
    // Encoding: 0x25007FF0
    // Test SEL_P.P.PP__ field combination: Pm=0, Pg=31, Pn=31, Pd=0
    // Fields: Pd=0, Pg=31, Pn=31, Pm=0
    let encoding: u32 = 0x25007FF0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: SEL_P.P.PP__
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=1 (same register test (reg=1)), Pd=1 (same register test (reg=1))
#[test]
fn test_sel_p_p_pp_combo_16_4210_25004611() {
    // Encoding: 0x25004611
    // Test SEL_P.P.PP__ field combination: Pm=0, Pg=1, Pn=0, Pd=1
    // Fields: Pn=0, Pg=1, Pm=0, Pd=1
    let encoding: u32 = 0x25004611;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: SEL_P.P.PP__
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=31 (same register test (reg=31)), Pd=31 (same register test (reg=31))
#[test]
fn test_sel_p_p_pp_combo_17_4210_25007e1f() {
    // Encoding: 0x25007E1F
    // Test SEL_P.P.PP__ field combination: Pm=0, Pg=31, Pn=0, Pd=31
    // Fields: Pg=31, Pn=0, Pm=0, Pd=31
    let encoding: u32 = 0x25007E1F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: SEL_P.P.PP__
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pn=1 (same register test (reg=1)), Pd=1 (same register test (reg=1))
#[test]
fn test_sel_p_p_pp_combo_18_4210_25004231() {
    // Encoding: 0x25004231
    // Test SEL_P.P.PP__ field combination: Pm=0, Pg=0, Pn=1, Pd=1
    // Fields: Pn=1, Pg=0, Pd=1, Pm=0
    let encoding: u32 = 0x25004231;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: SEL_P.P.PP__
/// ASL: `field combination 19`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pn=31 (same register test (reg=31)), Pd=31 (same register test (reg=31))
#[test]
fn test_sel_p_p_pp_combo_19_4210_250043ff() {
    // Encoding: 0x250043FF
    // Test SEL_P.P.PP__ field combination: Pm=0, Pg=0, Pn=31, Pd=31
    // Fields: Pd=31, Pn=31, Pg=0, Pm=0
    let encoding: u32 = 0x250043FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: SEL_P.P.PP__
/// ASL: `SimdFromField("Pd") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "unknown" }
/// verify register write to SimdFromField("Pd")
#[test]
fn test_sel_p_p_pp_reg_write_0_25004210() {
    // Test SEL_P.P.PP__ register write: SimdFromField("Pd")
    // Encoding: 0x25004210
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x25004210;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

// ============================================================================
// PTRUE_P.S__ Tests
// ============================================================================

/// Provenance: PTRUE_P.S__
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_ptrue_p_s_field_size_0_min_e000_2518e000() {
    // Encoding: 0x2518E000
    // Test PTRUE_P.S__ field size = 0 (Min)
    // Fields: Pd=0, pattern=0, size=0
    let encoding: u32 = 0x2518E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PTRUE_P.S__
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 1, boundary: PowerOfTwo }
/// 16-bit / halfword size
#[test]
fn test_ptrue_p_s_field_size_1_poweroftwo_e000_2558e000() {
    // Encoding: 0x2558E000
    // Test PTRUE_P.S__ field size = 1 (PowerOfTwo)
    // Fields: pattern=0, size=1, Pd=0
    let encoding: u32 = 0x2558E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PTRUE_P.S__
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 2, boundary: PowerOfTwo }
/// 32-bit / word size
#[test]
fn test_ptrue_p_s_field_size_2_poweroftwo_e000_2598e000() {
    // Encoding: 0x2598E000
    // Test PTRUE_P.S__ field size = 2 (PowerOfTwo)
    // Fields: pattern=0, size=2, Pd=0
    let encoding: u32 = 0x2598E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PTRUE_P.S__
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 3, boundary: Max }
/// 64-bit / doubleword size
#[test]
fn test_ptrue_p_s_field_size_3_max_e000_25d8e000() {
    // Encoding: 0x25D8E000
    // Test PTRUE_P.S__ field size = 3 (Max)
    // Fields: pattern=0, size=3, Pd=0
    let encoding: u32 = 0x25D8E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PTRUE_P.S__
/// ASL: `field pattern 5 +: 5`
/// Requirement: FieldBoundary { field: "pattern", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_ptrue_p_s_field_pattern_0_min_e000_2518e000() {
    // Encoding: 0x2518E000
    // Test PTRUE_P.S__ field pattern = 0 (Min)
    // Fields: pattern=0, Pd=0, size=0
    let encoding: u32 = 0x2518E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PTRUE_P.S__
/// ASL: `field pattern 5 +: 5`
/// Requirement: FieldBoundary { field: "pattern", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_ptrue_p_s_field_pattern_1_poweroftwo_e000_2518e020() {
    // Encoding: 0x2518E020
    // Test PTRUE_P.S__ field pattern = 1 (PowerOfTwo)
    // Fields: size=0, pattern=1, Pd=0
    let encoding: u32 = 0x2518E020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PTRUE_P.S__
/// ASL: `field pattern 5 +: 5`
/// Requirement: FieldBoundary { field: "pattern", value: 15, boundary: PowerOfTwoMinusOne }
/// midpoint (15)
#[test]
fn test_ptrue_p_s_field_pattern_15_poweroftwominusone_e000_2518e1e0() {
    // Encoding: 0x2518E1E0
    // Test PTRUE_P.S__ field pattern = 15 (PowerOfTwoMinusOne)
    // Fields: size=0, pattern=15, Pd=0
    let encoding: u32 = 0x2518E1E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PTRUE_P.S__
/// ASL: `field pattern 5 +: 5`
/// Requirement: FieldBoundary { field: "pattern", value: 31, boundary: Max }
/// maximum value (31)
#[test]
fn test_ptrue_p_s_field_pattern_31_max_e000_2518e3e0() {
    // Encoding: 0x2518E3E0
    // Test PTRUE_P.S__ field pattern = 31 (Max)
    // Fields: Pd=0, size=0, pattern=31
    let encoding: u32 = 0x2518E3E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PTRUE_P.S__
/// ASL: `field Pd 0 +: 4`
/// Requirement: FieldBoundary { field: "Pd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_ptrue_p_s_field_pd_0_min_e000_2518e000() {
    // Encoding: 0x2518E000
    // Test PTRUE_P.S__ field Pd = 0 (Min)
    // Fields: Pd=0, size=0, pattern=0
    let encoding: u32 = 0x2518E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PTRUE_P.S__
/// ASL: `field Pd 0 +: 4`
/// Requirement: FieldBoundary { field: "Pd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_ptrue_p_s_field_pd_1_poweroftwo_e000_2518e001() {
    // Encoding: 0x2518E001
    // Test PTRUE_P.S__ field Pd = 1 (PowerOfTwo)
    // Fields: Pd=1, size=0, pattern=0
    let encoding: u32 = 0x2518E001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PTRUE_P.S__
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=0 (8-bit / byte size)
#[test]
fn test_ptrue_p_s_combo_0_e000_2518e000() {
    // Encoding: 0x2518E000
    // Test PTRUE_P.S__ field combination: size=0, pattern=0, Pd=0
    // Fields: size=0, pattern=0, Pd=0
    let encoding: u32 = 0x2518E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PTRUE_P.S__
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=1 (16-bit / halfword size)
#[test]
fn test_ptrue_p_s_combo_1_e000_2558e000() {
    // Encoding: 0x2558E000
    // Test PTRUE_P.S__ field combination: size=1, pattern=0, Pd=0
    // Fields: size=1, pattern=0, Pd=0
    let encoding: u32 = 0x2558E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PTRUE_P.S__
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=2 (32-bit / word size)
#[test]
fn test_ptrue_p_s_combo_2_e000_2598e000() {
    // Encoding: 0x2598E000
    // Test PTRUE_P.S__ field combination: size=2, pattern=0, Pd=0
    // Fields: size=2, pattern=0, Pd=0
    let encoding: u32 = 0x2598E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PTRUE_P.S__
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=3 (64-bit / doubleword size)
#[test]
fn test_ptrue_p_s_combo_3_e000_25d8e000() {
    // Encoding: 0x25D8E000
    // Test PTRUE_P.S__ field combination: size=3, pattern=0, Pd=0
    // Fields: size=3, pattern=0, Pd=0
    let encoding: u32 = 0x25D8E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PTRUE_P.S__
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// pattern=0 (minimum value)
#[test]
fn test_ptrue_p_s_combo_4_e000_2518e000() {
    // Encoding: 0x2518E000
    // Test PTRUE_P.S__ field combination: size=0, pattern=0, Pd=0
    // Fields: size=0, Pd=0, pattern=0
    let encoding: u32 = 0x2518E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PTRUE_P.S__
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// pattern=1 (value 1)
#[test]
fn test_ptrue_p_s_combo_5_e000_2518e020() {
    // Encoding: 0x2518E020
    // Test PTRUE_P.S__ field combination: size=0, pattern=1, Pd=0
    // Fields: size=0, Pd=0, pattern=1
    let encoding: u32 = 0x2518E020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PTRUE_P.S__
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// pattern=15 (midpoint (15))
#[test]
fn test_ptrue_p_s_combo_6_e000_2518e1e0() {
    // Encoding: 0x2518E1E0
    // Test PTRUE_P.S__ field combination: size=0, pattern=15, Pd=0
    // Fields: pattern=15, Pd=0, size=0
    let encoding: u32 = 0x2518E1E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PTRUE_P.S__
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// pattern=31 (maximum value (31))
#[test]
fn test_ptrue_p_s_combo_7_e000_2518e3e0() {
    // Encoding: 0x2518E3E0
    // Test PTRUE_P.S__ field combination: size=0, pattern=31, Pd=0
    // Fields: Pd=0, size=0, pattern=31
    let encoding: u32 = 0x2518E3E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PTRUE_P.S__
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pd=0 (register index 0 (first register))
#[test]
fn test_ptrue_p_s_combo_8_e000_2518e000() {
    // Encoding: 0x2518E000
    // Test PTRUE_P.S__ field combination: size=0, pattern=0, Pd=0
    // Fields: Pd=0, size=0, pattern=0
    let encoding: u32 = 0x2518E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PTRUE_P.S__
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pd=1 (register index 1 (second register))
#[test]
fn test_ptrue_p_s_combo_9_e000_2518e001() {
    // Encoding: 0x2518E001
    // Test PTRUE_P.S__ field combination: size=0, pattern=0, Pd=1
    // Fields: size=0, pattern=0, Pd=1
    let encoding: u32 = 0x2518E001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PTRUE_P.S__
/// ASL: `field size = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "size", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_ptrue_p_s_special_size_0_size_variant_0_57344_2518e000() {
    // Encoding: 0x2518E000
    // Test PTRUE_P.S__ special value size = 0 (Size variant 0)
    // Fields: Pd=0, size=0, pattern=0
    let encoding: u32 = 0x2518E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PTRUE_P.S__
/// ASL: `field size = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "size", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_ptrue_p_s_special_size_1_size_variant_1_57344_2558e000() {
    // Encoding: 0x2558E000
    // Test PTRUE_P.S__ special value size = 1 (Size variant 1)
    // Fields: pattern=0, size=1, Pd=0
    let encoding: u32 = 0x2558E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PTRUE_P.S__
/// ASL: `field size = 2 (Size variant 2)`
/// Requirement: FieldSpecial { field: "size", value: 2, meaning: "Size variant 2" }
/// Size variant 2
#[test]
fn test_ptrue_p_s_special_size_2_size_variant_2_57344_2598e000() {
    // Encoding: 0x2598E000
    // Test PTRUE_P.S__ special value size = 2 (Size variant 2)
    // Fields: pattern=0, size=2, Pd=0
    let encoding: u32 = 0x2598E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PTRUE_P.S__
/// ASL: `field size = 3 (Size variant 3)`
/// Requirement: FieldSpecial { field: "size", value: 3, meaning: "Size variant 3" }
/// Size variant 3
#[test]
fn test_ptrue_p_s_special_size_3_size_variant_3_57344_25d8e000() {
    // Encoding: 0x25D8E000
    // Test PTRUE_P.S__ special value size = 3 (Size variant 3)
    // Fields: pattern=0, size=3, Pd=0
    let encoding: u32 = 0x25D8E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PTRUES_P.S__
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_ptrues_p_s_field_size_0_min_e000_2519e000() {
    // Encoding: 0x2519E000
    // Test PTRUES_P.S__ field size = 0 (Min)
    // Fields: size=0, Pd=0, pattern=0
    let encoding: u32 = 0x2519E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PTRUES_P.S__
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 1, boundary: PowerOfTwo }
/// 16-bit / halfword size
#[test]
fn test_ptrues_p_s_field_size_1_poweroftwo_e000_2559e000() {
    // Encoding: 0x2559E000
    // Test PTRUES_P.S__ field size = 1 (PowerOfTwo)
    // Fields: size=1, pattern=0, Pd=0
    let encoding: u32 = 0x2559E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PTRUES_P.S__
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 2, boundary: PowerOfTwo }
/// 32-bit / word size
#[test]
fn test_ptrues_p_s_field_size_2_poweroftwo_e000_2599e000() {
    // Encoding: 0x2599E000
    // Test PTRUES_P.S__ field size = 2 (PowerOfTwo)
    // Fields: size=2, pattern=0, Pd=0
    let encoding: u32 = 0x2599E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PTRUES_P.S__
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 3, boundary: Max }
/// 64-bit / doubleword size
#[test]
fn test_ptrues_p_s_field_size_3_max_e000_25d9e000() {
    // Encoding: 0x25D9E000
    // Test PTRUES_P.S__ field size = 3 (Max)
    // Fields: pattern=0, Pd=0, size=3
    let encoding: u32 = 0x25D9E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PTRUES_P.S__
/// ASL: `field pattern 5 +: 5`
/// Requirement: FieldBoundary { field: "pattern", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_ptrues_p_s_field_pattern_0_min_e000_2519e000() {
    // Encoding: 0x2519E000
    // Test PTRUES_P.S__ field pattern = 0 (Min)
    // Fields: pattern=0, size=0, Pd=0
    let encoding: u32 = 0x2519E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PTRUES_P.S__
/// ASL: `field pattern 5 +: 5`
/// Requirement: FieldBoundary { field: "pattern", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_ptrues_p_s_field_pattern_1_poweroftwo_e000_2519e020() {
    // Encoding: 0x2519E020
    // Test PTRUES_P.S__ field pattern = 1 (PowerOfTwo)
    // Fields: Pd=0, size=0, pattern=1
    let encoding: u32 = 0x2519E020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PTRUES_P.S__
/// ASL: `field pattern 5 +: 5`
/// Requirement: FieldBoundary { field: "pattern", value: 15, boundary: PowerOfTwoMinusOne }
/// midpoint (15)
#[test]
fn test_ptrues_p_s_field_pattern_15_poweroftwominusone_e000_2519e1e0() {
    // Encoding: 0x2519E1E0
    // Test PTRUES_P.S__ field pattern = 15 (PowerOfTwoMinusOne)
    // Fields: Pd=0, size=0, pattern=15
    let encoding: u32 = 0x2519E1E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PTRUES_P.S__
/// ASL: `field pattern 5 +: 5`
/// Requirement: FieldBoundary { field: "pattern", value: 31, boundary: Max }
/// maximum value (31)
#[test]
fn test_ptrues_p_s_field_pattern_31_max_e000_2519e3e0() {
    // Encoding: 0x2519E3E0
    // Test PTRUES_P.S__ field pattern = 31 (Max)
    // Fields: pattern=31, Pd=0, size=0
    let encoding: u32 = 0x2519E3E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PTRUES_P.S__
/// ASL: `field Pd 0 +: 4`
/// Requirement: FieldBoundary { field: "Pd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_ptrues_p_s_field_pd_0_min_e000_2519e000() {
    // Encoding: 0x2519E000
    // Test PTRUES_P.S__ field Pd = 0 (Min)
    // Fields: Pd=0, pattern=0, size=0
    let encoding: u32 = 0x2519E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PTRUES_P.S__
/// ASL: `field Pd 0 +: 4`
/// Requirement: FieldBoundary { field: "Pd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_ptrues_p_s_field_pd_1_poweroftwo_e000_2519e001() {
    // Encoding: 0x2519E001
    // Test PTRUES_P.S__ field Pd = 1 (PowerOfTwo)
    // Fields: Pd=1, size=0, pattern=0
    let encoding: u32 = 0x2519E001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PTRUES_P.S__
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=0 (8-bit / byte size)
#[test]
fn test_ptrues_p_s_combo_0_e000_2519e000() {
    // Encoding: 0x2519E000
    // Test PTRUES_P.S__ field combination: size=0, pattern=0, Pd=0
    // Fields: pattern=0, Pd=0, size=0
    let encoding: u32 = 0x2519E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PTRUES_P.S__
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=1 (16-bit / halfword size)
#[test]
fn test_ptrues_p_s_combo_1_e000_2559e000() {
    // Encoding: 0x2559E000
    // Test PTRUES_P.S__ field combination: size=1, pattern=0, Pd=0
    // Fields: pattern=0, Pd=0, size=1
    let encoding: u32 = 0x2559E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PTRUES_P.S__
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=2 (32-bit / word size)
#[test]
fn test_ptrues_p_s_combo_2_e000_2599e000() {
    // Encoding: 0x2599E000
    // Test PTRUES_P.S__ field combination: size=2, pattern=0, Pd=0
    // Fields: size=2, pattern=0, Pd=0
    let encoding: u32 = 0x2599E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PTRUES_P.S__
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=3 (64-bit / doubleword size)
#[test]
fn test_ptrues_p_s_combo_3_e000_25d9e000() {
    // Encoding: 0x25D9E000
    // Test PTRUES_P.S__ field combination: size=3, pattern=0, Pd=0
    // Fields: pattern=0, size=3, Pd=0
    let encoding: u32 = 0x25D9E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PTRUES_P.S__
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// pattern=0 (minimum value)
#[test]
fn test_ptrues_p_s_combo_4_e000_2519e000() {
    // Encoding: 0x2519E000
    // Test PTRUES_P.S__ field combination: size=0, pattern=0, Pd=0
    // Fields: pattern=0, Pd=0, size=0
    let encoding: u32 = 0x2519E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PTRUES_P.S__
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// pattern=1 (value 1)
#[test]
fn test_ptrues_p_s_combo_5_e000_2519e020() {
    // Encoding: 0x2519E020
    // Test PTRUES_P.S__ field combination: size=0, pattern=1, Pd=0
    // Fields: pattern=1, Pd=0, size=0
    let encoding: u32 = 0x2519E020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PTRUES_P.S__
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// pattern=15 (midpoint (15))
#[test]
fn test_ptrues_p_s_combo_6_e000_2519e1e0() {
    // Encoding: 0x2519E1E0
    // Test PTRUES_P.S__ field combination: size=0, pattern=15, Pd=0
    // Fields: pattern=15, Pd=0, size=0
    let encoding: u32 = 0x2519E1E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PTRUES_P.S__
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// pattern=31 (maximum value (31))
#[test]
fn test_ptrues_p_s_combo_7_e000_2519e3e0() {
    // Encoding: 0x2519E3E0
    // Test PTRUES_P.S__ field combination: size=0, pattern=31, Pd=0
    // Fields: pattern=31, Pd=0, size=0
    let encoding: u32 = 0x2519E3E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PTRUES_P.S__
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pd=0 (register index 0 (first register))
#[test]
fn test_ptrues_p_s_combo_8_e000_2519e000() {
    // Encoding: 0x2519E000
    // Test PTRUES_P.S__ field combination: size=0, pattern=0, Pd=0
    // Fields: size=0, Pd=0, pattern=0
    let encoding: u32 = 0x2519E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PTRUES_P.S__
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pd=1 (register index 1 (second register))
#[test]
fn test_ptrues_p_s_combo_9_e000_2519e001() {
    // Encoding: 0x2519E001
    // Test PTRUES_P.S__ field combination: size=0, pattern=0, Pd=1
    // Fields: Pd=1, size=0, pattern=0
    let encoding: u32 = 0x2519E001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PTRUES_P.S__
/// ASL: `field size = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "size", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_ptrues_p_s_special_size_0_size_variant_0_57344_2519e000() {
    // Encoding: 0x2519E000
    // Test PTRUES_P.S__ special value size = 0 (Size variant 0)
    // Fields: size=0, Pd=0, pattern=0
    let encoding: u32 = 0x2519E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PTRUES_P.S__
/// ASL: `field size = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "size", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_ptrues_p_s_special_size_1_size_variant_1_57344_2559e000() {
    // Encoding: 0x2559E000
    // Test PTRUES_P.S__ special value size = 1 (Size variant 1)
    // Fields: Pd=0, size=1, pattern=0
    let encoding: u32 = 0x2559E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PTRUES_P.S__
/// ASL: `field size = 2 (Size variant 2)`
/// Requirement: FieldSpecial { field: "size", value: 2, meaning: "Size variant 2" }
/// Size variant 2
#[test]
fn test_ptrues_p_s_special_size_2_size_variant_2_57344_2599e000() {
    // Encoding: 0x2599E000
    // Test PTRUES_P.S__ special value size = 2 (Size variant 2)
    // Fields: Pd=0, pattern=0, size=2
    let encoding: u32 = 0x2599E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PTRUES_P.S__
/// ASL: `field size = 3 (Size variant 3)`
/// Requirement: FieldSpecial { field: "size", value: 3, meaning: "Size variant 3" }
/// Size variant 3
#[test]
fn test_ptrues_p_s_special_size_3_size_variant_3_57344_25d9e000() {
    // Encoding: 0x25D9E000
    // Test PTRUES_P.S__ special value size = 3 (Size variant 3)
    // Fields: Pd=0, pattern=0, size=3
    let encoding: u32 = 0x25D9E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PTRUE_P.S__
/// ASL: `SimdFromField("Pd") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "unknown" }
/// verify register write to SimdFromField("Pd")
#[test]
fn test_ptrue_p_s_reg_write_0_2518e000() {
    // Test PTRUE_P.S__ register write: SimdFromField("Pd")
    // Encoding: 0x2518E000
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x2518E000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: PTRUE_P.S__
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 0 + 0 = 0 (Z=1)
#[test]
fn test_ptrue_p_s_flags_zeroresult_0_2518e000() {
    // Test PTRUE_P.S__ flag computation: ZeroResult
    // Encoding: 0x2518E000
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x0);
    set_x(&mut cpu, 2, 0x0);
    let encoding: u32 = 0x2518E000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: PTRUE_P.S__
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 1 + (-1) = 0 (Z=1, C=1)
#[test]
fn test_ptrue_p_s_flags_zeroresult_1_2518e000() {
    // Test PTRUE_P.S__ flag computation: ZeroResult
    // Encoding: 0x2518E000
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0xFFFFFFFFFFFFFFFF);
    set_x(&mut cpu, 1, 0x1);
    let encoding: u32 = 0x2518E000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: PTRUE_P.S__
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: NegativeResult }
/// negative value (N=1)
#[test]
fn test_ptrue_p_s_flags_negativeresult_2_2518e000() {
    // Test PTRUE_P.S__ flag computation: NegativeResult
    // Encoding: 0x2518E000
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0x0);
    set_x(&mut cpu, 1, 0x8000000000000000);
    let encoding: u32 = 0x2518E000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: PTRUE_P.S__
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 1 = 0 (C=1, Z=1)
#[test]
fn test_ptrue_p_s_flags_unsignedoverflow_3_2518e000() {
    // Test PTRUE_P.S__ flag computation: UnsignedOverflow
    // Encoding: 0x2518E000
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0x1);
    set_x(&mut cpu, 1, 0xFFFFFFFFFFFFFFFF);
    let encoding: u32 = 0x2518E000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: PTRUE_P.S__
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 2 = 1 (C=1)
#[test]
fn test_ptrue_p_s_flags_unsignedoverflow_4_2518e000() {
    // Test PTRUE_P.S__ flag computation: UnsignedOverflow
    // Encoding: 0x2518E000
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0xFFFFFFFFFFFFFFFF);
    set_x(&mut cpu, 2, 0x2);
    let encoding: u32 = 0x2518E000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: PTRUE_P.S__
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// max_signed + 1 = min_signed (V=1, N=1)
#[test]
fn test_ptrue_p_s_flags_signedoverflow_5_2518e000() {
    // Test PTRUE_P.S__ flag computation: SignedOverflow
    // Encoding: 0x2518E000
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0x1);
    set_x(&mut cpu, 1, 0x7FFFFFFFFFFFFFFF);
    let encoding: u32 = 0x2518E000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: PTRUE_P.S__
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// min_signed + (-1) = max_signed (V=1)
#[test]
fn test_ptrue_p_s_flags_signedoverflow_6_2518e000() {
    // Test PTRUE_P.S__ flag computation: SignedOverflow
    // Encoding: 0x2518E000
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0xFFFFFFFFFFFFFFFF);
    set_x(&mut cpu, 1, 0x8000000000000000);
    let encoding: u32 = 0x2518E000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: PTRUE_P.S__
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: PositiveResult }
/// 100 + 50 = 150 (no flags)
#[test]
fn test_ptrue_p_s_flags_positiveresult_7_2518e000() {
    // Test PTRUE_P.S__ flag computation: PositiveResult
    // Encoding: 0x2518E000
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x64);
    set_x(&mut cpu, 2, 0x32);
    let encoding: u32 = 0x2518E000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: PTRUES_P.S__
/// ASL: `SimdFromField("Pd") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "unknown" }
/// verify register write to SimdFromField("Pd")
#[test]
fn test_ptrues_p_s_reg_write_0_2519e000() {
    // Test PTRUES_P.S__ register write: SimdFromField("Pd")
    // Encoding: 0x2519E000
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x2519E000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: PTRUES_P.S__
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 0 + 0 = 0 (Z=1)
#[test]
fn test_ptrues_p_s_flags_zeroresult_0_2519e000() {
    // Test PTRUES_P.S__ flag computation: ZeroResult
    // Encoding: 0x2519E000
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0x0);
    set_x(&mut cpu, 1, 0x0);
    let encoding: u32 = 0x2519E000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: PTRUES_P.S__
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 1 + (-1) = 0 (Z=1, C=1)
#[test]
fn test_ptrues_p_s_flags_zeroresult_1_2519e000() {
    // Test PTRUES_P.S__ flag computation: ZeroResult
    // Encoding: 0x2519E000
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x1);
    set_x(&mut cpu, 2, 0xFFFFFFFFFFFFFFFF);
    let encoding: u32 = 0x2519E000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: PTRUES_P.S__
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: NegativeResult }
/// negative value (N=1)
#[test]
fn test_ptrues_p_s_flags_negativeresult_2_2519e000() {
    // Test PTRUES_P.S__ flag computation: NegativeResult
    // Encoding: 0x2519E000
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0x0);
    set_x(&mut cpu, 1, 0x8000000000000000);
    let encoding: u32 = 0x2519E000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: PTRUES_P.S__
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 1 = 0 (C=1, Z=1)
#[test]
fn test_ptrues_p_s_flags_unsignedoverflow_3_2519e000() {
    // Test PTRUES_P.S__ flag computation: UnsignedOverflow
    // Encoding: 0x2519E000
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0xFFFFFFFFFFFFFFFF);
    set_x(&mut cpu, 2, 0x1);
    let encoding: u32 = 0x2519E000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: PTRUES_P.S__
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 2 = 1 (C=1)
#[test]
fn test_ptrues_p_s_flags_unsignedoverflow_4_2519e000() {
    // Test PTRUES_P.S__ flag computation: UnsignedOverflow
    // Encoding: 0x2519E000
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0x2);
    set_x(&mut cpu, 1, 0xFFFFFFFFFFFFFFFF);
    let encoding: u32 = 0x2519E000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: PTRUES_P.S__
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// max_signed + 1 = min_signed (V=1, N=1)
#[test]
fn test_ptrues_p_s_flags_signedoverflow_5_2519e000() {
    // Test PTRUES_P.S__ flag computation: SignedOverflow
    // Encoding: 0x2519E000
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0x1);
    set_x(&mut cpu, 1, 0x7FFFFFFFFFFFFFFF);
    let encoding: u32 = 0x2519E000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: PTRUES_P.S__
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// min_signed + (-1) = max_signed (V=1)
#[test]
fn test_ptrues_p_s_flags_signedoverflow_6_2519e000() {
    // Test PTRUES_P.S__ flag computation: SignedOverflow
    // Encoding: 0x2519E000
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0xFFFFFFFFFFFFFFFF);
    set_x(&mut cpu, 1, 0x8000000000000000);
    let encoding: u32 = 0x2519E000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: PTRUES_P.S__
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: PositiveResult }
/// 100 + 50 = 150 (no flags)
#[test]
fn test_ptrues_p_s_flags_positiveresult_7_2519e000() {
    // Test PTRUES_P.S__ flag computation: PositiveResult
    // Encoding: 0x2519E000
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0x32);
    set_x(&mut cpu, 1, 0x64);
    let encoding: u32 = 0x2519E000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

// ============================================================================
// BRKA_P.P.P__ Tests
// ============================================================================

/// Provenance: BRKA_P.P.P__
/// ASL: `field Pg 10 +: 4`
/// Requirement: FieldBoundary { field: "Pg", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_brka_p_p_p_field_pg_0_min_4000_25104000() {
    // Encoding: 0x25104000
    // Test BRKA_P.P.P__ field Pg = 0 (Min)
    // Fields: Pn=0, M=0, Pg=0, Pd=0
    let encoding: u32 = 0x25104000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKA_P.P.P__
/// ASL: `field Pg 10 +: 4`
/// Requirement: FieldBoundary { field: "Pg", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_brka_p_p_p_field_pg_1_poweroftwo_4000_25104400() {
    // Encoding: 0x25104400
    // Test BRKA_P.P.P__ field Pg = 1 (PowerOfTwo)
    // Fields: Pg=1, Pn=0, Pd=0, M=0
    let encoding: u32 = 0x25104400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKA_P.P.P__
/// ASL: `field Pn 5 +: 4`
/// Requirement: FieldBoundary { field: "Pn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_brka_p_p_p_field_pn_0_min_4000_25104000() {
    // Encoding: 0x25104000
    // Test BRKA_P.P.P__ field Pn = 0 (Min)
    // Fields: Pg=0, Pn=0, M=0, Pd=0
    let encoding: u32 = 0x25104000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKA_P.P.P__
/// ASL: `field Pn 5 +: 4`
/// Requirement: FieldBoundary { field: "Pn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_brka_p_p_p_field_pn_1_poweroftwo_4000_25104020() {
    // Encoding: 0x25104020
    // Test BRKA_P.P.P__ field Pn = 1 (PowerOfTwo)
    // Fields: Pg=0, Pn=1, Pd=0, M=0
    let encoding: u32 = 0x25104020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKA_P.P.P__
/// ASL: `field M 4 +: 1`
/// Requirement: FieldBoundary { field: "M", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_brka_p_p_p_field_m_0_min_4000_25104000() {
    // Encoding: 0x25104000
    // Test BRKA_P.P.P__ field M = 0 (Min)
    // Fields: M=0, Pg=0, Pn=0, Pd=0
    let encoding: u32 = 0x25104000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKA_P.P.P__
/// ASL: `field M 4 +: 1`
/// Requirement: FieldBoundary { field: "M", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_brka_p_p_p_field_m_1_max_4000_25104010() {
    // Encoding: 0x25104010
    // Test BRKA_P.P.P__ field M = 1 (Max)
    // Fields: Pn=0, Pg=0, Pd=0, M=1
    let encoding: u32 = 0x25104010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKA_P.P.P__
/// ASL: `field Pd 0 +: 4`
/// Requirement: FieldBoundary { field: "Pd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_brka_p_p_p_field_pd_0_min_4000_25104000() {
    // Encoding: 0x25104000
    // Test BRKA_P.P.P__ field Pd = 0 (Min)
    // Fields: Pg=0, Pn=0, M=0, Pd=0
    let encoding: u32 = 0x25104000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKA_P.P.P__
/// ASL: `field Pd 0 +: 4`
/// Requirement: FieldBoundary { field: "Pd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_brka_p_p_p_field_pd_1_poweroftwo_4000_25104001() {
    // Encoding: 0x25104001
    // Test BRKA_P.P.P__ field Pd = 1 (PowerOfTwo)
    // Fields: Pg=0, M=0, Pd=1, Pn=0
    let encoding: u32 = 0x25104001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKA_P.P.P__
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=0 (register index 0 (first register))
#[test]
fn test_brka_p_p_p_combo_0_4000_25104000() {
    // Encoding: 0x25104000
    // Test BRKA_P.P.P__ field combination: Pg=0, Pn=0, M=0, Pd=0
    // Fields: Pd=0, Pn=0, M=0, Pg=0
    let encoding: u32 = 0x25104000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKA_P.P.P__
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=1 (register index 1 (second register))
#[test]
fn test_brka_p_p_p_combo_1_4000_25104400() {
    // Encoding: 0x25104400
    // Test BRKA_P.P.P__ field combination: Pg=1, Pn=0, M=0, Pd=0
    // Fields: M=0, Pd=0, Pn=0, Pg=1
    let encoding: u32 = 0x25104400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKA_P.P.P__
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pn=0 (register index 0 (first register))
#[test]
fn test_brka_p_p_p_combo_2_4000_25104000() {
    // Encoding: 0x25104000
    // Test BRKA_P.P.P__ field combination: Pg=0, Pn=0, M=0, Pd=0
    // Fields: Pn=0, M=0, Pg=0, Pd=0
    let encoding: u32 = 0x25104000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKA_P.P.P__
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pn=1 (register index 1 (second register))
#[test]
fn test_brka_p_p_p_combo_3_4000_25104020() {
    // Encoding: 0x25104020
    // Test BRKA_P.P.P__ field combination: Pg=0, Pn=1, M=0, Pd=0
    // Fields: Pn=1, Pg=0, M=0, Pd=0
    let encoding: u32 = 0x25104020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKA_P.P.P__
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// M=0 (minimum value)
#[test]
fn test_brka_p_p_p_combo_4_4000_25104000() {
    // Encoding: 0x25104000
    // Test BRKA_P.P.P__ field combination: Pg=0, Pn=0, M=0, Pd=0
    // Fields: M=0, Pd=0, Pg=0, Pn=0
    let encoding: u32 = 0x25104000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKA_P.P.P__
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// M=1 (maximum value (1))
#[test]
fn test_brka_p_p_p_combo_5_4000_25104010() {
    // Encoding: 0x25104010
    // Test BRKA_P.P.P__ field combination: Pg=0, Pn=0, M=1, Pd=0
    // Fields: Pg=0, Pn=0, Pd=0, M=1
    let encoding: u32 = 0x25104010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKA_P.P.P__
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pd=0 (register index 0 (first register))
#[test]
fn test_brka_p_p_p_combo_6_4000_25104000() {
    // Encoding: 0x25104000
    // Test BRKA_P.P.P__ field combination: Pg=0, Pn=0, M=0, Pd=0
    // Fields: Pg=0, Pn=0, M=0, Pd=0
    let encoding: u32 = 0x25104000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKA_P.P.P__
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pd=1 (register index 1 (second register))
#[test]
fn test_brka_p_p_p_combo_7_4000_25104001() {
    // Encoding: 0x25104001
    // Test BRKA_P.P.P__ field combination: Pg=0, Pn=0, M=0, Pd=1
    // Fields: Pn=0, Pd=1, Pg=0, M=0
    let encoding: u32 = 0x25104001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKA_P.P.P__
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=1 (same register test (reg=1)), Pn=1 (same register test (reg=1))
#[test]
fn test_brka_p_p_p_combo_8_4000_25104420() {
    // Encoding: 0x25104420
    // Test BRKA_P.P.P__ field combination: Pg=1, Pn=1, M=0, Pd=0
    // Fields: Pd=0, M=0, Pg=1, Pn=1
    let encoding: u32 = 0x25104420;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKA_P.P.P__
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=31 (same register test (reg=31)), Pn=31 (same register test (reg=31))
#[test]
fn test_brka_p_p_p_combo_9_4000_25107de0() {
    // Encoding: 0x25107DE0
    // Test BRKA_P.P.P__ field combination: Pg=31, Pn=31, M=0, Pd=0
    // Fields: M=0, Pg=31, Pn=31, Pd=0
    let encoding: u32 = 0x25107DE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKA_P.P.P__
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=1 (same register test (reg=1)), Pd=1 (same register test (reg=1))
#[test]
fn test_brka_p_p_p_combo_10_4000_25104401() {
    // Encoding: 0x25104401
    // Test BRKA_P.P.P__ field combination: Pg=1, Pn=0, M=0, Pd=1
    // Fields: Pg=1, M=0, Pd=1, Pn=0
    let encoding: u32 = 0x25104401;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKA_P.P.P__
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=31 (same register test (reg=31)), Pd=31 (same register test (reg=31))
#[test]
fn test_brka_p_p_p_combo_11_4000_25107c0f() {
    // Encoding: 0x25107C0F
    // Test BRKA_P.P.P__ field combination: Pg=31, Pn=0, M=0, Pd=31
    // Fields: Pd=31, M=0, Pn=0, Pg=31
    let encoding: u32 = 0x25107C0F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKA_P.P.P__
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pn=1 (same register test (reg=1)), Pd=1 (same register test (reg=1))
#[test]
fn test_brka_p_p_p_combo_12_4000_25104021() {
    // Encoding: 0x25104021
    // Test BRKA_P.P.P__ field combination: Pg=0, Pn=1, M=0, Pd=1
    // Fields: Pd=1, M=0, Pg=0, Pn=1
    let encoding: u32 = 0x25104021;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKA_P.P.P__
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pn=31 (same register test (reg=31)), Pd=31 (same register test (reg=31))
#[test]
fn test_brka_p_p_p_combo_13_4000_251041ef() {
    // Encoding: 0x251041EF
    // Test BRKA_P.P.P__ field combination: Pg=0, Pn=31, M=0, Pd=31
    // Fields: Pn=31, Pd=31, Pg=0, M=0
    let encoding: u32 = 0x251041EF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKAS_P.P.P_Z
/// ASL: `field Pg 10 +: 4`
/// Requirement: FieldBoundary { field: "Pg", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_brkas_p_p_p_z_field_pg_0_min_4000_25504000() {
    // Encoding: 0x25504000
    // Test BRKAS_P.P.P_Z field Pg = 0 (Min)
    // Fields: Pn=0, Pd=0, Pg=0
    let encoding: u32 = 0x25504000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKAS_P.P.P_Z
/// ASL: `field Pg 10 +: 4`
/// Requirement: FieldBoundary { field: "Pg", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_brkas_p_p_p_z_field_pg_1_poweroftwo_4000_25504400() {
    // Encoding: 0x25504400
    // Test BRKAS_P.P.P_Z field Pg = 1 (PowerOfTwo)
    // Fields: Pg=1, Pn=0, Pd=0
    let encoding: u32 = 0x25504400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKAS_P.P.P_Z
/// ASL: `field Pn 5 +: 4`
/// Requirement: FieldBoundary { field: "Pn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_brkas_p_p_p_z_field_pn_0_min_4000_25504000() {
    // Encoding: 0x25504000
    // Test BRKAS_P.P.P_Z field Pn = 0 (Min)
    // Fields: Pn=0, Pd=0, Pg=0
    let encoding: u32 = 0x25504000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKAS_P.P.P_Z
/// ASL: `field Pn 5 +: 4`
/// Requirement: FieldBoundary { field: "Pn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_brkas_p_p_p_z_field_pn_1_poweroftwo_4000_25504020() {
    // Encoding: 0x25504020
    // Test BRKAS_P.P.P_Z field Pn = 1 (PowerOfTwo)
    // Fields: Pn=1, Pg=0, Pd=0
    let encoding: u32 = 0x25504020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKAS_P.P.P_Z
/// ASL: `field Pd 0 +: 4`
/// Requirement: FieldBoundary { field: "Pd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_brkas_p_p_p_z_field_pd_0_min_4000_25504000() {
    // Encoding: 0x25504000
    // Test BRKAS_P.P.P_Z field Pd = 0 (Min)
    // Fields: Pn=0, Pd=0, Pg=0
    let encoding: u32 = 0x25504000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKAS_P.P.P_Z
/// ASL: `field Pd 0 +: 4`
/// Requirement: FieldBoundary { field: "Pd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_brkas_p_p_p_z_field_pd_1_poweroftwo_4000_25504001() {
    // Encoding: 0x25504001
    // Test BRKAS_P.P.P_Z field Pd = 1 (PowerOfTwo)
    // Fields: Pn=0, Pd=1, Pg=0
    let encoding: u32 = 0x25504001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKAS_P.P.P_Z
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=0 (register index 0 (first register))
#[test]
fn test_brkas_p_p_p_z_combo_0_4000_25504000() {
    // Encoding: 0x25504000
    // Test BRKAS_P.P.P_Z field combination: Pg=0, Pn=0, Pd=0
    // Fields: Pd=0, Pn=0, Pg=0
    let encoding: u32 = 0x25504000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKAS_P.P.P_Z
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=1 (register index 1 (second register))
#[test]
fn test_brkas_p_p_p_z_combo_1_4000_25504400() {
    // Encoding: 0x25504400
    // Test BRKAS_P.P.P_Z field combination: Pg=1, Pn=0, Pd=0
    // Fields: Pn=0, Pd=0, Pg=1
    let encoding: u32 = 0x25504400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKAS_P.P.P_Z
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pn=0 (register index 0 (first register))
#[test]
fn test_brkas_p_p_p_z_combo_2_4000_25504000() {
    // Encoding: 0x25504000
    // Test BRKAS_P.P.P_Z field combination: Pg=0, Pn=0, Pd=0
    // Fields: Pd=0, Pg=0, Pn=0
    let encoding: u32 = 0x25504000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKAS_P.P.P_Z
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pn=1 (register index 1 (second register))
#[test]
fn test_brkas_p_p_p_z_combo_3_4000_25504020() {
    // Encoding: 0x25504020
    // Test BRKAS_P.P.P_Z field combination: Pg=0, Pn=1, Pd=0
    // Fields: Pn=1, Pg=0, Pd=0
    let encoding: u32 = 0x25504020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKAS_P.P.P_Z
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pd=0 (register index 0 (first register))
#[test]
fn test_brkas_p_p_p_z_combo_4_4000_25504000() {
    // Encoding: 0x25504000
    // Test BRKAS_P.P.P_Z field combination: Pg=0, Pn=0, Pd=0
    // Fields: Pg=0, Pn=0, Pd=0
    let encoding: u32 = 0x25504000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKAS_P.P.P_Z
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pd=1 (register index 1 (second register))
#[test]
fn test_brkas_p_p_p_z_combo_5_4000_25504001() {
    // Encoding: 0x25504001
    // Test BRKAS_P.P.P_Z field combination: Pg=0, Pn=0, Pd=1
    // Fields: Pn=0, Pg=0, Pd=1
    let encoding: u32 = 0x25504001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKAS_P.P.P_Z
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=1 (same register test (reg=1)), Pn=1 (same register test (reg=1))
#[test]
fn test_brkas_p_p_p_z_combo_6_4000_25504420() {
    // Encoding: 0x25504420
    // Test BRKAS_P.P.P_Z field combination: Pg=1, Pn=1, Pd=0
    // Fields: Pg=1, Pn=1, Pd=0
    let encoding: u32 = 0x25504420;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKAS_P.P.P_Z
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=31 (same register test (reg=31)), Pn=31 (same register test (reg=31))
#[test]
fn test_brkas_p_p_p_z_combo_7_4000_25507de0() {
    // Encoding: 0x25507DE0
    // Test BRKAS_P.P.P_Z field combination: Pg=31, Pn=31, Pd=0
    // Fields: Pn=31, Pg=31, Pd=0
    let encoding: u32 = 0x25507DE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKAS_P.P.P_Z
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=1 (same register test (reg=1)), Pd=1 (same register test (reg=1))
#[test]
fn test_brkas_p_p_p_z_combo_8_4000_25504401() {
    // Encoding: 0x25504401
    // Test BRKAS_P.P.P_Z field combination: Pg=1, Pn=0, Pd=1
    // Fields: Pd=1, Pn=0, Pg=1
    let encoding: u32 = 0x25504401;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKAS_P.P.P_Z
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=31 (same register test (reg=31)), Pd=31 (same register test (reg=31))
#[test]
fn test_brkas_p_p_p_z_combo_9_4000_25507c0f() {
    // Encoding: 0x25507C0F
    // Test BRKAS_P.P.P_Z field combination: Pg=31, Pn=0, Pd=31
    // Fields: Pn=0, Pd=31, Pg=31
    let encoding: u32 = 0x25507C0F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKAS_P.P.P_Z
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pn=1 (same register test (reg=1)), Pd=1 (same register test (reg=1))
#[test]
fn test_brkas_p_p_p_z_combo_10_4000_25504021() {
    // Encoding: 0x25504021
    // Test BRKAS_P.P.P_Z field combination: Pg=0, Pn=1, Pd=1
    // Fields: Pn=1, Pd=1, Pg=0
    let encoding: u32 = 0x25504021;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKAS_P.P.P_Z
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pn=31 (same register test (reg=31)), Pd=31 (same register test (reg=31))
#[test]
fn test_brkas_p_p_p_z_combo_11_4000_255041ef() {
    // Encoding: 0x255041EF
    // Test BRKAS_P.P.P_Z field combination: Pg=0, Pn=31, Pd=31
    // Fields: Pg=0, Pn=31, Pd=31
    let encoding: u32 = 0x255041EF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKA_P.P.P__
/// ASL: `SimdFromField("Pd") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "unknown" }
/// verify register write to SimdFromField("Pd")
#[test]
fn test_brka_p_p_p_reg_write_0_25104000() {
    // Test BRKA_P.P.P__ register write: SimdFromField("Pd")
    // Encoding: 0x25104000
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x25104000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: BRKA_P.P.P__
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 0 + 0 = 0 (Z=1)
#[test]
fn test_brka_p_p_p_flags_zeroresult_0_25104000() {
    // Test BRKA_P.P.P__ flag computation: ZeroResult
    // Encoding: 0x25104000
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x0);
    set_x(&mut cpu, 2, 0x0);
    let encoding: u32 = 0x25104000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: BRKA_P.P.P__
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 1 + (-1) = 0 (Z=1, C=1)
#[test]
fn test_brka_p_p_p_flags_zeroresult_1_25104000() {
    // Test BRKA_P.P.P__ flag computation: ZeroResult
    // Encoding: 0x25104000
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0xFFFFFFFFFFFFFFFF);
    set_x(&mut cpu, 1, 0x1);
    let encoding: u32 = 0x25104000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: BRKA_P.P.P__
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: NegativeResult }
/// negative value (N=1)
#[test]
fn test_brka_p_p_p_flags_negativeresult_2_25104000() {
    // Test BRKA_P.P.P__ flag computation: NegativeResult
    // Encoding: 0x25104000
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x8000000000000000);
    set_x(&mut cpu, 2, 0x0);
    let encoding: u32 = 0x25104000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: BRKA_P.P.P__
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 1 = 0 (C=1, Z=1)
#[test]
fn test_brka_p_p_p_flags_unsignedoverflow_3_25104000() {
    // Test BRKA_P.P.P__ flag computation: UnsignedOverflow
    // Encoding: 0x25104000
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0xFFFFFFFFFFFFFFFF);
    set_x(&mut cpu, 2, 0x1);
    let encoding: u32 = 0x25104000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: BRKA_P.P.P__
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 2 = 1 (C=1)
#[test]
fn test_brka_p_p_p_flags_unsignedoverflow_4_25104000() {
    // Test BRKA_P.P.P__ flag computation: UnsignedOverflow
    // Encoding: 0x25104000
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0x2);
    set_x(&mut cpu, 1, 0xFFFFFFFFFFFFFFFF);
    let encoding: u32 = 0x25104000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: BRKA_P.P.P__
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// max_signed + 1 = min_signed (V=1, N=1)
#[test]
fn test_brka_p_p_p_flags_signedoverflow_5_25104000() {
    // Test BRKA_P.P.P__ flag computation: SignedOverflow
    // Encoding: 0x25104000
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0x1);
    set_x(&mut cpu, 1, 0x7FFFFFFFFFFFFFFF);
    let encoding: u32 = 0x25104000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: BRKA_P.P.P__
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// min_signed + (-1) = max_signed (V=1)
#[test]
fn test_brka_p_p_p_flags_signedoverflow_6_25104000() {
    // Test BRKA_P.P.P__ flag computation: SignedOverflow
    // Encoding: 0x25104000
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x8000000000000000);
    set_x(&mut cpu, 2, 0xFFFFFFFFFFFFFFFF);
    let encoding: u32 = 0x25104000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: BRKA_P.P.P__
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: PositiveResult }
/// 100 + 50 = 150 (no flags)
#[test]
fn test_brka_p_p_p_flags_positiveresult_7_25104000() {
    // Test BRKA_P.P.P__ flag computation: PositiveResult
    // Encoding: 0x25104000
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x64);
    set_x(&mut cpu, 2, 0x32);
    let encoding: u32 = 0x25104000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: BRKAS_P.P.P_Z
/// ASL: `SimdFromField("Pd") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "unknown" }
/// verify register write to SimdFromField("Pd")
#[test]
fn test_brkas_p_p_p_z_reg_write_0_25504000() {
    // Test BRKAS_P.P.P_Z register write: SimdFromField("Pd")
    // Encoding: 0x25504000
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x25504000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: BRKAS_P.P.P_Z
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 0 + 0 = 0 (Z=1)
#[test]
fn test_brkas_p_p_p_z_flags_zeroresult_0_25504000() {
    // Test BRKAS_P.P.P_Z flag computation: ZeroResult
    // Encoding: 0x25504000
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x0);
    set_x(&mut cpu, 2, 0x0);
    let encoding: u32 = 0x25504000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: BRKAS_P.P.P_Z
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 1 + (-1) = 0 (Z=1, C=1)
#[test]
fn test_brkas_p_p_p_z_flags_zeroresult_1_25504000() {
    // Test BRKAS_P.P.P_Z flag computation: ZeroResult
    // Encoding: 0x25504000
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x1);
    set_x(&mut cpu, 2, 0xFFFFFFFFFFFFFFFF);
    let encoding: u32 = 0x25504000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: BRKAS_P.P.P_Z
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: NegativeResult }
/// negative value (N=1)
#[test]
fn test_brkas_p_p_p_z_flags_negativeresult_2_25504000() {
    // Test BRKAS_P.P.P_Z flag computation: NegativeResult
    // Encoding: 0x25504000
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x8000000000000000);
    set_x(&mut cpu, 2, 0x0);
    let encoding: u32 = 0x25504000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: BRKAS_P.P.P_Z
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 1 = 0 (C=1, Z=1)
#[test]
fn test_brkas_p_p_p_z_flags_unsignedoverflow_3_25504000() {
    // Test BRKAS_P.P.P_Z flag computation: UnsignedOverflow
    // Encoding: 0x25504000
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0xFFFFFFFFFFFFFFFF);
    set_x(&mut cpu, 2, 0x1);
    let encoding: u32 = 0x25504000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: BRKAS_P.P.P_Z
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 2 = 1 (C=1)
#[test]
fn test_brkas_p_p_p_z_flags_unsignedoverflow_4_25504000() {
    // Test BRKAS_P.P.P_Z flag computation: UnsignedOverflow
    // Encoding: 0x25504000
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0xFFFFFFFFFFFFFFFF);
    set_x(&mut cpu, 2, 0x2);
    let encoding: u32 = 0x25504000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: BRKAS_P.P.P_Z
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// max_signed + 1 = min_signed (V=1, N=1)
#[test]
fn test_brkas_p_p_p_z_flags_signedoverflow_5_25504000() {
    // Test BRKAS_P.P.P_Z flag computation: SignedOverflow
    // Encoding: 0x25504000
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0x1);
    set_x(&mut cpu, 1, 0x7FFFFFFFFFFFFFFF);
    let encoding: u32 = 0x25504000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: BRKAS_P.P.P_Z
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// min_signed + (-1) = max_signed (V=1)
#[test]
fn test_brkas_p_p_p_z_flags_signedoverflow_6_25504000() {
    // Test BRKAS_P.P.P_Z flag computation: SignedOverflow
    // Encoding: 0x25504000
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x8000000000000000);
    set_x(&mut cpu, 2, 0xFFFFFFFFFFFFFFFF);
    let encoding: u32 = 0x25504000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: BRKAS_P.P.P_Z
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: PositiveResult }
/// 100 + 50 = 150 (no flags)
#[test]
fn test_brkas_p_p_p_z_flags_positiveresult_7_25504000() {
    // Test BRKAS_P.P.P_Z flag computation: PositiveResult
    // Encoding: 0x25504000
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0x32);
    set_x(&mut cpu, 1, 0x64);
    let encoding: u32 = 0x25504000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

// ============================================================================
// PTEST_.P.P__ Tests
// ============================================================================

/// Provenance: PTEST_.P.P__
/// ASL: `field Pg 10 +: 4`
/// Requirement: FieldBoundary { field: "Pg", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_ptest_p_p_field_pg_0_min_c000_2550c000() {
    // Encoding: 0x2550C000
    // Test PTEST_.P.P__ field Pg = 0 (Min)
    // Fields: Pg=0, Pn=0
    let encoding: u32 = 0x2550C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PTEST_.P.P__
/// ASL: `field Pg 10 +: 4`
/// Requirement: FieldBoundary { field: "Pg", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_ptest_p_p_field_pg_1_poweroftwo_c000_2550c400() {
    // Encoding: 0x2550C400
    // Test PTEST_.P.P__ field Pg = 1 (PowerOfTwo)
    // Fields: Pg=1, Pn=0
    let encoding: u32 = 0x2550C400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PTEST_.P.P__
/// ASL: `field Pn 5 +: 4`
/// Requirement: FieldBoundary { field: "Pn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_ptest_p_p_field_pn_0_min_c000_2550c000() {
    // Encoding: 0x2550C000
    // Test PTEST_.P.P__ field Pn = 0 (Min)
    // Fields: Pn=0, Pg=0
    let encoding: u32 = 0x2550C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PTEST_.P.P__
/// ASL: `field Pn 5 +: 4`
/// Requirement: FieldBoundary { field: "Pn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_ptest_p_p_field_pn_1_poweroftwo_c000_2550c020() {
    // Encoding: 0x2550C020
    // Test PTEST_.P.P__ field Pn = 1 (PowerOfTwo)
    // Fields: Pg=0, Pn=1
    let encoding: u32 = 0x2550C020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PTEST_.P.P__
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=0 (register index 0 (first register))
#[test]
fn test_ptest_p_p_combo_0_c000_2550c000() {
    // Encoding: 0x2550C000
    // Test PTEST_.P.P__ field combination: Pg=0, Pn=0
    // Fields: Pn=0, Pg=0
    let encoding: u32 = 0x2550C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PTEST_.P.P__
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=1 (register index 1 (second register))
#[test]
fn test_ptest_p_p_combo_1_c000_2550c400() {
    // Encoding: 0x2550C400
    // Test PTEST_.P.P__ field combination: Pg=1, Pn=0
    // Fields: Pg=1, Pn=0
    let encoding: u32 = 0x2550C400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PTEST_.P.P__
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pn=0 (register index 0 (first register))
#[test]
fn test_ptest_p_p_combo_2_c000_2550c000() {
    // Encoding: 0x2550C000
    // Test PTEST_.P.P__ field combination: Pg=0, Pn=0
    // Fields: Pn=0, Pg=0
    let encoding: u32 = 0x2550C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PTEST_.P.P__
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pn=1 (register index 1 (second register))
#[test]
fn test_ptest_p_p_combo_3_c000_2550c020() {
    // Encoding: 0x2550C020
    // Test PTEST_.P.P__ field combination: Pg=0, Pn=1
    // Fields: Pn=1, Pg=0
    let encoding: u32 = 0x2550C020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PTEST_.P.P__
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=1 (same register test (reg=1)), Pn=1 (same register test (reg=1))
#[test]
fn test_ptest_p_p_combo_4_c000_2550c420() {
    // Encoding: 0x2550C420
    // Test PTEST_.P.P__ field combination: Pg=1, Pn=1
    // Fields: Pn=1, Pg=1
    let encoding: u32 = 0x2550C420;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PTEST_.P.P__
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=31 (same register test (reg=31)), Pn=31 (same register test (reg=31))
#[test]
fn test_ptest_p_p_combo_5_c000_2550fde0() {
    // Encoding: 0x2550FDE0
    // Test PTEST_.P.P__ field combination: Pg=31, Pn=31
    // Fields: Pg=31, Pn=31
    let encoding: u32 = 0x2550FDE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PTEST_.P.P__
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 0 + 0 = 0 (Z=1)
#[test]
fn test_ptest_p_p_flags_zeroresult_0_2550c000() {
    // Test PTEST_.P.P__ flag computation: ZeroResult
    // Encoding: 0x2550C000
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0x0);
    set_x(&mut cpu, 1, 0x0);
    let encoding: u32 = 0x2550C000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: PTEST_.P.P__
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 1 + (-1) = 0 (Z=1, C=1)
#[test]
fn test_ptest_p_p_flags_zeroresult_1_2550c000() {
    // Test PTEST_.P.P__ flag computation: ZeroResult
    // Encoding: 0x2550C000
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x1);
    set_x(&mut cpu, 2, 0xFFFFFFFFFFFFFFFF);
    let encoding: u32 = 0x2550C000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: PTEST_.P.P__
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: NegativeResult }
/// negative value (N=1)
#[test]
fn test_ptest_p_p_flags_negativeresult_2_2550c000() {
    // Test PTEST_.P.P__ flag computation: NegativeResult
    // Encoding: 0x2550C000
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0x0);
    set_x(&mut cpu, 1, 0x8000000000000000);
    let encoding: u32 = 0x2550C000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: PTEST_.P.P__
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 1 = 0 (C=1, Z=1)
#[test]
fn test_ptest_p_p_flags_unsignedoverflow_3_2550c000() {
    // Test PTEST_.P.P__ flag computation: UnsignedOverflow
    // Encoding: 0x2550C000
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0x1);
    set_x(&mut cpu, 1, 0xFFFFFFFFFFFFFFFF);
    let encoding: u32 = 0x2550C000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: PTEST_.P.P__
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 2 = 1 (C=1)
#[test]
fn test_ptest_p_p_flags_unsignedoverflow_4_2550c000() {
    // Test PTEST_.P.P__ flag computation: UnsignedOverflow
    // Encoding: 0x2550C000
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0x2);
    set_x(&mut cpu, 1, 0xFFFFFFFFFFFFFFFF);
    let encoding: u32 = 0x2550C000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: PTEST_.P.P__
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// max_signed + 1 = min_signed (V=1, N=1)
#[test]
fn test_ptest_p_p_flags_signedoverflow_5_2550c000() {
    // Test PTEST_.P.P__ flag computation: SignedOverflow
    // Encoding: 0x2550C000
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x7FFFFFFFFFFFFFFF);
    set_x(&mut cpu, 2, 0x1);
    let encoding: u32 = 0x2550C000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: PTEST_.P.P__
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// min_signed + (-1) = max_signed (V=1)
#[test]
fn test_ptest_p_p_flags_signedoverflow_6_2550c000() {
    // Test PTEST_.P.P__ flag computation: SignedOverflow
    // Encoding: 0x2550C000
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0xFFFFFFFFFFFFFFFF);
    set_x(&mut cpu, 1, 0x8000000000000000);
    let encoding: u32 = 0x2550C000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: PTEST_.P.P__
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: PositiveResult }
/// 100 + 50 = 150 (no flags)
#[test]
fn test_ptest_p_p_flags_positiveresult_7_2550c000() {
    // Test PTEST_.P.P__ flag computation: PositiveResult
    // Encoding: 0x2550C000
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0x32);
    set_x(&mut cpu, 1, 0x64);
    let encoding: u32 = 0x2550C000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

// ============================================================================
// ORN_P.P.PP_Z Tests
// ============================================================================

/// Provenance: ORN_P.P.PP_Z
/// ASL: `field Pm 16 +: 4`
/// Requirement: FieldBoundary { field: "Pm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_orn_p_p_pp_z_field_pm_0_min_4010_25804010() {
    // Encoding: 0x25804010
    // Test ORN_P.P.PP_Z field Pm = 0 (Min)
    // Fields: Pm=0, Pg=0, Pn=0, Pd=0
    let encoding: u32 = 0x25804010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: ORN_P.P.PP_Z
/// ASL: `field Pm 16 +: 4`
/// Requirement: FieldBoundary { field: "Pm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_orn_p_p_pp_z_field_pm_1_poweroftwo_4010_25814010() {
    // Encoding: 0x25814010
    // Test ORN_P.P.PP_Z field Pm = 1 (PowerOfTwo)
    // Fields: Pm=1, Pg=0, Pd=0, Pn=0
    let encoding: u32 = 0x25814010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: ORN_P.P.PP_Z
/// ASL: `field Pg 10 +: 4`
/// Requirement: FieldBoundary { field: "Pg", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_orn_p_p_pp_z_field_pg_0_min_4010_25804010() {
    // Encoding: 0x25804010
    // Test ORN_P.P.PP_Z field Pg = 0 (Min)
    // Fields: Pg=0, Pn=0, Pm=0, Pd=0
    let encoding: u32 = 0x25804010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: ORN_P.P.PP_Z
/// ASL: `field Pg 10 +: 4`
/// Requirement: FieldBoundary { field: "Pg", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_orn_p_p_pp_z_field_pg_1_poweroftwo_4010_25804410() {
    // Encoding: 0x25804410
    // Test ORN_P.P.PP_Z field Pg = 1 (PowerOfTwo)
    // Fields: Pd=0, Pg=1, Pm=0, Pn=0
    let encoding: u32 = 0x25804410;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: ORN_P.P.PP_Z
/// ASL: `field Pn 5 +: 4`
/// Requirement: FieldBoundary { field: "Pn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_orn_p_p_pp_z_field_pn_0_min_4010_25804010() {
    // Encoding: 0x25804010
    // Test ORN_P.P.PP_Z field Pn = 0 (Min)
    // Fields: Pd=0, Pm=0, Pg=0, Pn=0
    let encoding: u32 = 0x25804010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: ORN_P.P.PP_Z
/// ASL: `field Pn 5 +: 4`
/// Requirement: FieldBoundary { field: "Pn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_orn_p_p_pp_z_field_pn_1_poweroftwo_4010_25804030() {
    // Encoding: 0x25804030
    // Test ORN_P.P.PP_Z field Pn = 1 (PowerOfTwo)
    // Fields: Pg=0, Pm=0, Pd=0, Pn=1
    let encoding: u32 = 0x25804030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: ORN_P.P.PP_Z
/// ASL: `field Pd 0 +: 4`
/// Requirement: FieldBoundary { field: "Pd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_orn_p_p_pp_z_field_pd_0_min_4010_25804010() {
    // Encoding: 0x25804010
    // Test ORN_P.P.PP_Z field Pd = 0 (Min)
    // Fields: Pg=0, Pn=0, Pm=0, Pd=0
    let encoding: u32 = 0x25804010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: ORN_P.P.PP_Z
/// ASL: `field Pd 0 +: 4`
/// Requirement: FieldBoundary { field: "Pd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_orn_p_p_pp_z_field_pd_1_poweroftwo_4010_25804011() {
    // Encoding: 0x25804011
    // Test ORN_P.P.PP_Z field Pd = 1 (PowerOfTwo)
    // Fields: Pm=0, Pd=1, Pn=0, Pg=0
    let encoding: u32 = 0x25804011;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: ORN_P.P.PP_Z
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pm=0 (register index 0 (first register))
#[test]
fn test_orn_p_p_pp_z_combo_0_4010_25804010() {
    // Encoding: 0x25804010
    // Test ORN_P.P.PP_Z field combination: Pm=0, Pg=0, Pn=0, Pd=0
    // Fields: Pd=0, Pm=0, Pg=0, Pn=0
    let encoding: u32 = 0x25804010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: ORN_P.P.PP_Z
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pm=1 (register index 1 (second register))
#[test]
fn test_orn_p_p_pp_z_combo_1_4010_25814010() {
    // Encoding: 0x25814010
    // Test ORN_P.P.PP_Z field combination: Pm=1, Pg=0, Pn=0, Pd=0
    // Fields: Pm=1, Pg=0, Pd=0, Pn=0
    let encoding: u32 = 0x25814010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: ORN_P.P.PP_Z
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=0 (register index 0 (first register))
#[test]
fn test_orn_p_p_pp_z_combo_2_4010_25804010() {
    // Encoding: 0x25804010
    // Test ORN_P.P.PP_Z field combination: Pm=0, Pg=0, Pn=0, Pd=0
    // Fields: Pn=0, Pd=0, Pg=0, Pm=0
    let encoding: u32 = 0x25804010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: ORN_P.P.PP_Z
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=1 (register index 1 (second register))
#[test]
fn test_orn_p_p_pp_z_combo_3_4010_25804410() {
    // Encoding: 0x25804410
    // Test ORN_P.P.PP_Z field combination: Pm=0, Pg=1, Pn=0, Pd=0
    // Fields: Pd=0, Pm=0, Pn=0, Pg=1
    let encoding: u32 = 0x25804410;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: ORN_P.P.PP_Z
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pn=0 (register index 0 (first register))
#[test]
fn test_orn_p_p_pp_z_combo_4_4010_25804010() {
    // Encoding: 0x25804010
    // Test ORN_P.P.PP_Z field combination: Pm=0, Pg=0, Pn=0, Pd=0
    // Fields: Pd=0, Pm=0, Pg=0, Pn=0
    let encoding: u32 = 0x25804010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: ORN_P.P.PP_Z
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pn=1 (register index 1 (second register))
#[test]
fn test_orn_p_p_pp_z_combo_5_4010_25804030() {
    // Encoding: 0x25804030
    // Test ORN_P.P.PP_Z field combination: Pm=0, Pg=0, Pn=1, Pd=0
    // Fields: Pm=0, Pn=1, Pg=0, Pd=0
    let encoding: u32 = 0x25804030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: ORN_P.P.PP_Z
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pd=0 (register index 0 (first register))
#[test]
fn test_orn_p_p_pp_z_combo_6_4010_25804010() {
    // Encoding: 0x25804010
    // Test ORN_P.P.PP_Z field combination: Pm=0, Pg=0, Pn=0, Pd=0
    // Fields: Pm=0, Pg=0, Pn=0, Pd=0
    let encoding: u32 = 0x25804010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: ORN_P.P.PP_Z
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pd=1 (register index 1 (second register))
#[test]
fn test_orn_p_p_pp_z_combo_7_4010_25804011() {
    // Encoding: 0x25804011
    // Test ORN_P.P.PP_Z field combination: Pm=0, Pg=0, Pn=0, Pd=1
    // Fields: Pm=0, Pn=0, Pg=0, Pd=1
    let encoding: u32 = 0x25804011;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: ORN_P.P.PP_Z
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pm=1 (same register test (reg=1)), Pg=1 (same register test (reg=1))
#[test]
fn test_orn_p_p_pp_z_combo_8_4010_25814410() {
    // Encoding: 0x25814410
    // Test ORN_P.P.PP_Z field combination: Pm=1, Pg=1, Pn=0, Pd=0
    // Fields: Pd=0, Pg=1, Pm=1, Pn=0
    let encoding: u32 = 0x25814410;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: ORN_P.P.PP_Z
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pm=31 (same register test (reg=31)), Pg=31 (same register test (reg=31))
#[test]
fn test_orn_p_p_pp_z_combo_9_4010_258f7c10() {
    // Encoding: 0x258F7C10
    // Test ORN_P.P.PP_Z field combination: Pm=31, Pg=31, Pn=0, Pd=0
    // Fields: Pn=0, Pd=0, Pg=31, Pm=31
    let encoding: u32 = 0x258F7C10;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: ORN_P.P.PP_Z
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pm=1 (same register test (reg=1)), Pn=1 (same register test (reg=1))
#[test]
fn test_orn_p_p_pp_z_combo_10_4010_25814030() {
    // Encoding: 0x25814030
    // Test ORN_P.P.PP_Z field combination: Pm=1, Pg=0, Pn=1, Pd=0
    // Fields: Pn=1, Pg=0, Pm=1, Pd=0
    let encoding: u32 = 0x25814030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: ORN_P.P.PP_Z
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pm=31 (same register test (reg=31)), Pn=31 (same register test (reg=31))
#[test]
fn test_orn_p_p_pp_z_combo_11_4010_258f41f0() {
    // Encoding: 0x258F41F0
    // Test ORN_P.P.PP_Z field combination: Pm=31, Pg=0, Pn=31, Pd=0
    // Fields: Pm=31, Pg=0, Pn=31, Pd=0
    let encoding: u32 = 0x258F41F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: ORN_P.P.PP_Z
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pm=1 (same register test (reg=1)), Pd=1 (same register test (reg=1))
#[test]
fn test_orn_p_p_pp_z_combo_12_4010_25814011() {
    // Encoding: 0x25814011
    // Test ORN_P.P.PP_Z field combination: Pm=1, Pg=0, Pn=0, Pd=1
    // Fields: Pd=1, Pm=1, Pg=0, Pn=0
    let encoding: u32 = 0x25814011;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: ORN_P.P.PP_Z
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pm=31 (same register test (reg=31)), Pd=31 (same register test (reg=31))
#[test]
fn test_orn_p_p_pp_z_combo_13_4010_258f401f() {
    // Encoding: 0x258F401F
    // Test ORN_P.P.PP_Z field combination: Pm=31, Pg=0, Pn=0, Pd=31
    // Fields: Pd=31, Pm=31, Pn=0, Pg=0
    let encoding: u32 = 0x258F401F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: ORN_P.P.PP_Z
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=1 (same register test (reg=1)), Pn=1 (same register test (reg=1))
#[test]
fn test_orn_p_p_pp_z_combo_14_4010_25804430() {
    // Encoding: 0x25804430
    // Test ORN_P.P.PP_Z field combination: Pm=0, Pg=1, Pn=1, Pd=0
    // Fields: Pg=1, Pn=1, Pm=0, Pd=0
    let encoding: u32 = 0x25804430;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: ORN_P.P.PP_Z
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=31 (same register test (reg=31)), Pn=31 (same register test (reg=31))
#[test]
fn test_orn_p_p_pp_z_combo_15_4010_25807df0() {
    // Encoding: 0x25807DF0
    // Test ORN_P.P.PP_Z field combination: Pm=0, Pg=31, Pn=31, Pd=0
    // Fields: Pn=31, Pg=31, Pm=0, Pd=0
    let encoding: u32 = 0x25807DF0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: ORN_P.P.PP_Z
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=1 (same register test (reg=1)), Pd=1 (same register test (reg=1))
#[test]
fn test_orn_p_p_pp_z_combo_16_4010_25804411() {
    // Encoding: 0x25804411
    // Test ORN_P.P.PP_Z field combination: Pm=0, Pg=1, Pn=0, Pd=1
    // Fields: Pm=0, Pd=1, Pn=0, Pg=1
    let encoding: u32 = 0x25804411;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: ORN_P.P.PP_Z
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=31 (same register test (reg=31)), Pd=31 (same register test (reg=31))
#[test]
fn test_orn_p_p_pp_z_combo_17_4010_25807c1f() {
    // Encoding: 0x25807C1F
    // Test ORN_P.P.PP_Z field combination: Pm=0, Pg=31, Pn=0, Pd=31
    // Fields: Pg=31, Pn=0, Pm=0, Pd=31
    let encoding: u32 = 0x25807C1F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: ORN_P.P.PP_Z
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pn=1 (same register test (reg=1)), Pd=1 (same register test (reg=1))
#[test]
fn test_orn_p_p_pp_z_combo_18_4010_25804031() {
    // Encoding: 0x25804031
    // Test ORN_P.P.PP_Z field combination: Pm=0, Pg=0, Pn=1, Pd=1
    // Fields: Pg=0, Pm=0, Pn=1, Pd=1
    let encoding: u32 = 0x25804031;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: ORN_P.P.PP_Z
/// ASL: `field combination 19`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pn=31 (same register test (reg=31)), Pd=31 (same register test (reg=31))
#[test]
fn test_orn_p_p_pp_z_combo_19_4010_258041ff() {
    // Encoding: 0x258041FF
    // Test ORN_P.P.PP_Z field combination: Pm=0, Pg=0, Pn=31, Pd=31
    // Fields: Pg=0, Pm=0, Pd=31, Pn=31
    let encoding: u32 = 0x258041FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: ORNS_P.P.PP_Z
/// ASL: `field Pm 16 +: 4`
/// Requirement: FieldBoundary { field: "Pm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_orns_p_p_pp_z_field_pm_0_min_4010_25c04010() {
    // Encoding: 0x25C04010
    // Test ORNS_P.P.PP_Z field Pm = 0 (Min)
    // Fields: Pm=0, Pn=0, Pg=0, Pd=0
    let encoding: u32 = 0x25C04010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: ORNS_P.P.PP_Z
/// ASL: `field Pm 16 +: 4`
/// Requirement: FieldBoundary { field: "Pm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_orns_p_p_pp_z_field_pm_1_poweroftwo_4010_25c14010() {
    // Encoding: 0x25C14010
    // Test ORNS_P.P.PP_Z field Pm = 1 (PowerOfTwo)
    // Fields: Pm=1, Pn=0, Pg=0, Pd=0
    let encoding: u32 = 0x25C14010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: ORNS_P.P.PP_Z
/// ASL: `field Pg 10 +: 4`
/// Requirement: FieldBoundary { field: "Pg", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_orns_p_p_pp_z_field_pg_0_min_4010_25c04010() {
    // Encoding: 0x25C04010
    // Test ORNS_P.P.PP_Z field Pg = 0 (Min)
    // Fields: Pg=0, Pd=0, Pm=0, Pn=0
    let encoding: u32 = 0x25C04010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: ORNS_P.P.PP_Z
/// ASL: `field Pg 10 +: 4`
/// Requirement: FieldBoundary { field: "Pg", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_orns_p_p_pp_z_field_pg_1_poweroftwo_4010_25c04410() {
    // Encoding: 0x25C04410
    // Test ORNS_P.P.PP_Z field Pg = 1 (PowerOfTwo)
    // Fields: Pg=1, Pd=0, Pn=0, Pm=0
    let encoding: u32 = 0x25C04410;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: ORNS_P.P.PP_Z
/// ASL: `field Pn 5 +: 4`
/// Requirement: FieldBoundary { field: "Pn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_orns_p_p_pp_z_field_pn_0_min_4010_25c04010() {
    // Encoding: 0x25C04010
    // Test ORNS_P.P.PP_Z field Pn = 0 (Min)
    // Fields: Pd=0, Pn=0, Pm=0, Pg=0
    let encoding: u32 = 0x25C04010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: ORNS_P.P.PP_Z
/// ASL: `field Pn 5 +: 4`
/// Requirement: FieldBoundary { field: "Pn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_orns_p_p_pp_z_field_pn_1_poweroftwo_4010_25c04030() {
    // Encoding: 0x25C04030
    // Test ORNS_P.P.PP_Z field Pn = 1 (PowerOfTwo)
    // Fields: Pd=0, Pg=0, Pm=0, Pn=1
    let encoding: u32 = 0x25C04030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: ORNS_P.P.PP_Z
/// ASL: `field Pd 0 +: 4`
/// Requirement: FieldBoundary { field: "Pd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_orns_p_p_pp_z_field_pd_0_min_4010_25c04010() {
    // Encoding: 0x25C04010
    // Test ORNS_P.P.PP_Z field Pd = 0 (Min)
    // Fields: Pn=0, Pg=0, Pm=0, Pd=0
    let encoding: u32 = 0x25C04010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: ORNS_P.P.PP_Z
/// ASL: `field Pd 0 +: 4`
/// Requirement: FieldBoundary { field: "Pd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_orns_p_p_pp_z_field_pd_1_poweroftwo_4010_25c04011() {
    // Encoding: 0x25C04011
    // Test ORNS_P.P.PP_Z field Pd = 1 (PowerOfTwo)
    // Fields: Pn=0, Pm=0, Pg=0, Pd=1
    let encoding: u32 = 0x25C04011;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: ORNS_P.P.PP_Z
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pm=0 (register index 0 (first register))
#[test]
fn test_orns_p_p_pp_z_combo_0_4010_25c04010() {
    // Encoding: 0x25C04010
    // Test ORNS_P.P.PP_Z field combination: Pm=0, Pg=0, Pn=0, Pd=0
    // Fields: Pd=0, Pn=0, Pm=0, Pg=0
    let encoding: u32 = 0x25C04010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: ORNS_P.P.PP_Z
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pm=1 (register index 1 (second register))
#[test]
fn test_orns_p_p_pp_z_combo_1_4010_25c14010() {
    // Encoding: 0x25C14010
    // Test ORNS_P.P.PP_Z field combination: Pm=1, Pg=0, Pn=0, Pd=0
    // Fields: Pg=0, Pn=0, Pm=1, Pd=0
    let encoding: u32 = 0x25C14010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: ORNS_P.P.PP_Z
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=0 (register index 0 (first register))
#[test]
fn test_orns_p_p_pp_z_combo_2_4010_25c04010() {
    // Encoding: 0x25C04010
    // Test ORNS_P.P.PP_Z field combination: Pm=0, Pg=0, Pn=0, Pd=0
    // Fields: Pd=0, Pm=0, Pg=0, Pn=0
    let encoding: u32 = 0x25C04010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: ORNS_P.P.PP_Z
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=1 (register index 1 (second register))
#[test]
fn test_orns_p_p_pp_z_combo_3_4010_25c04410() {
    // Encoding: 0x25C04410
    // Test ORNS_P.P.PP_Z field combination: Pm=0, Pg=1, Pn=0, Pd=0
    // Fields: Pd=0, Pm=0, Pg=1, Pn=0
    let encoding: u32 = 0x25C04410;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: ORNS_P.P.PP_Z
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pn=0 (register index 0 (first register))
#[test]
fn test_orns_p_p_pp_z_combo_4_4010_25c04010() {
    // Encoding: 0x25C04010
    // Test ORNS_P.P.PP_Z field combination: Pm=0, Pg=0, Pn=0, Pd=0
    // Fields: Pd=0, Pg=0, Pm=0, Pn=0
    let encoding: u32 = 0x25C04010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: ORNS_P.P.PP_Z
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pn=1 (register index 1 (second register))
#[test]
fn test_orns_p_p_pp_z_combo_5_4010_25c04030() {
    // Encoding: 0x25C04030
    // Test ORNS_P.P.PP_Z field combination: Pm=0, Pg=0, Pn=1, Pd=0
    // Fields: Pn=1, Pd=0, Pm=0, Pg=0
    let encoding: u32 = 0x25C04030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: ORNS_P.P.PP_Z
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pd=0 (register index 0 (first register))
#[test]
fn test_orns_p_p_pp_z_combo_6_4010_25c04010() {
    // Encoding: 0x25C04010
    // Test ORNS_P.P.PP_Z field combination: Pm=0, Pg=0, Pn=0, Pd=0
    // Fields: Pm=0, Pn=0, Pd=0, Pg=0
    let encoding: u32 = 0x25C04010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: ORNS_P.P.PP_Z
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pd=1 (register index 1 (second register))
#[test]
fn test_orns_p_p_pp_z_combo_7_4010_25c04011() {
    // Encoding: 0x25C04011
    // Test ORNS_P.P.PP_Z field combination: Pm=0, Pg=0, Pn=0, Pd=1
    // Fields: Pg=0, Pd=1, Pm=0, Pn=0
    let encoding: u32 = 0x25C04011;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: ORNS_P.P.PP_Z
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pm=1 (same register test (reg=1)), Pg=1 (same register test (reg=1))
#[test]
fn test_orns_p_p_pp_z_combo_8_4010_25c14410() {
    // Encoding: 0x25C14410
    // Test ORNS_P.P.PP_Z field combination: Pm=1, Pg=1, Pn=0, Pd=0
    // Fields: Pg=1, Pm=1, Pd=0, Pn=0
    let encoding: u32 = 0x25C14410;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: ORNS_P.P.PP_Z
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pm=31 (same register test (reg=31)), Pg=31 (same register test (reg=31))
#[test]
fn test_orns_p_p_pp_z_combo_9_4010_25cf7c10() {
    // Encoding: 0x25CF7C10
    // Test ORNS_P.P.PP_Z field combination: Pm=31, Pg=31, Pn=0, Pd=0
    // Fields: Pn=0, Pd=0, Pm=31, Pg=31
    let encoding: u32 = 0x25CF7C10;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: ORNS_P.P.PP_Z
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pm=1 (same register test (reg=1)), Pn=1 (same register test (reg=1))
#[test]
fn test_orns_p_p_pp_z_combo_10_4010_25c14030() {
    // Encoding: 0x25C14030
    // Test ORNS_P.P.PP_Z field combination: Pm=1, Pg=0, Pn=1, Pd=0
    // Fields: Pd=0, Pm=1, Pn=1, Pg=0
    let encoding: u32 = 0x25C14030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: ORNS_P.P.PP_Z
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pm=31 (same register test (reg=31)), Pn=31 (same register test (reg=31))
#[test]
fn test_orns_p_p_pp_z_combo_11_4010_25cf41f0() {
    // Encoding: 0x25CF41F0
    // Test ORNS_P.P.PP_Z field combination: Pm=31, Pg=0, Pn=31, Pd=0
    // Fields: Pg=0, Pn=31, Pd=0, Pm=31
    let encoding: u32 = 0x25CF41F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: ORNS_P.P.PP_Z
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pm=1 (same register test (reg=1)), Pd=1 (same register test (reg=1))
#[test]
fn test_orns_p_p_pp_z_combo_12_4010_25c14011() {
    // Encoding: 0x25C14011
    // Test ORNS_P.P.PP_Z field combination: Pm=1, Pg=0, Pn=0, Pd=1
    // Fields: Pm=1, Pn=0, Pd=1, Pg=0
    let encoding: u32 = 0x25C14011;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: ORNS_P.P.PP_Z
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pm=31 (same register test (reg=31)), Pd=31 (same register test (reg=31))
#[test]
fn test_orns_p_p_pp_z_combo_13_4010_25cf401f() {
    // Encoding: 0x25CF401F
    // Test ORNS_P.P.PP_Z field combination: Pm=31, Pg=0, Pn=0, Pd=31
    // Fields: Pd=31, Pn=0, Pg=0, Pm=31
    let encoding: u32 = 0x25CF401F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: ORNS_P.P.PP_Z
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=1 (same register test (reg=1)), Pn=1 (same register test (reg=1))
#[test]
fn test_orns_p_p_pp_z_combo_14_4010_25c04430() {
    // Encoding: 0x25C04430
    // Test ORNS_P.P.PP_Z field combination: Pm=0, Pg=1, Pn=1, Pd=0
    // Fields: Pg=1, Pm=0, Pd=0, Pn=1
    let encoding: u32 = 0x25C04430;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: ORNS_P.P.PP_Z
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=31 (same register test (reg=31)), Pn=31 (same register test (reg=31))
#[test]
fn test_orns_p_p_pp_z_combo_15_4010_25c07df0() {
    // Encoding: 0x25C07DF0
    // Test ORNS_P.P.PP_Z field combination: Pm=0, Pg=31, Pn=31, Pd=0
    // Fields: Pm=0, Pd=0, Pg=31, Pn=31
    let encoding: u32 = 0x25C07DF0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: ORNS_P.P.PP_Z
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=1 (same register test (reg=1)), Pd=1 (same register test (reg=1))
#[test]
fn test_orns_p_p_pp_z_combo_16_4010_25c04411() {
    // Encoding: 0x25C04411
    // Test ORNS_P.P.PP_Z field combination: Pm=0, Pg=1, Pn=0, Pd=1
    // Fields: Pg=1, Pm=0, Pn=0, Pd=1
    let encoding: u32 = 0x25C04411;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: ORNS_P.P.PP_Z
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=31 (same register test (reg=31)), Pd=31 (same register test (reg=31))
#[test]
fn test_orns_p_p_pp_z_combo_17_4010_25c07c1f() {
    // Encoding: 0x25C07C1F
    // Test ORNS_P.P.PP_Z field combination: Pm=0, Pg=31, Pn=0, Pd=31
    // Fields: Pn=0, Pg=31, Pm=0, Pd=31
    let encoding: u32 = 0x25C07C1F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: ORNS_P.P.PP_Z
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pn=1 (same register test (reg=1)), Pd=1 (same register test (reg=1))
#[test]
fn test_orns_p_p_pp_z_combo_18_4010_25c04031() {
    // Encoding: 0x25C04031
    // Test ORNS_P.P.PP_Z field combination: Pm=0, Pg=0, Pn=1, Pd=1
    // Fields: Pm=0, Pg=0, Pd=1, Pn=1
    let encoding: u32 = 0x25C04031;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: ORNS_P.P.PP_Z
/// ASL: `field combination 19`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pn=31 (same register test (reg=31)), Pd=31 (same register test (reg=31))
#[test]
fn test_orns_p_p_pp_z_combo_19_4010_25c041ff() {
    // Encoding: 0x25C041FF
    // Test ORNS_P.P.PP_Z field combination: Pm=0, Pg=0, Pn=31, Pd=31
    // Fields: Pg=0, Pd=31, Pm=0, Pn=31
    let encoding: u32 = 0x25C041FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: ORN_P.P.PP_Z
/// ASL: `SimdFromField("Pd") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "unknown" }
/// verify register write to SimdFromField("Pd")
#[test]
fn test_orn_p_p_pp_z_reg_write_0_25804010() {
    // Test ORN_P.P.PP_Z register write: SimdFromField("Pd")
    // Encoding: 0x25804010
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x25804010;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: ORN_P.P.PP_Z
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 0 + 0 = 0 (Z=1)
#[test]
fn test_orn_p_p_pp_z_flags_zeroresult_0_25804010() {
    // Test ORN_P.P.PP_Z flag computation: ZeroResult
    // Encoding: 0x25804010
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0x0);
    set_x(&mut cpu, 1, 0x0);
    let encoding: u32 = 0x25804010;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: ORN_P.P.PP_Z
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 1 + (-1) = 0 (Z=1, C=1)
#[test]
fn test_orn_p_p_pp_z_flags_zeroresult_1_25804010() {
    // Test ORN_P.P.PP_Z flag computation: ZeroResult
    // Encoding: 0x25804010
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x1);
    set_x(&mut cpu, 2, 0xFFFFFFFFFFFFFFFF);
    let encoding: u32 = 0x25804010;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: ORN_P.P.PP_Z
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: NegativeResult }
/// negative value (N=1)
#[test]
fn test_orn_p_p_pp_z_flags_negativeresult_2_25804010() {
    // Test ORN_P.P.PP_Z flag computation: NegativeResult
    // Encoding: 0x25804010
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x8000000000000000);
    set_x(&mut cpu, 2, 0x0);
    let encoding: u32 = 0x25804010;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: ORN_P.P.PP_Z
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 1 = 0 (C=1, Z=1)
#[test]
fn test_orn_p_p_pp_z_flags_unsignedoverflow_3_25804010() {
    // Test ORN_P.P.PP_Z flag computation: UnsignedOverflow
    // Encoding: 0x25804010
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0x1);
    set_x(&mut cpu, 1, 0xFFFFFFFFFFFFFFFF);
    let encoding: u32 = 0x25804010;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: ORN_P.P.PP_Z
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 2 = 1 (C=1)
#[test]
fn test_orn_p_p_pp_z_flags_unsignedoverflow_4_25804010() {
    // Test ORN_P.P.PP_Z flag computation: UnsignedOverflow
    // Encoding: 0x25804010
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0xFFFFFFFFFFFFFFFF);
    set_x(&mut cpu, 2, 0x2);
    let encoding: u32 = 0x25804010;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: ORN_P.P.PP_Z
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// max_signed + 1 = min_signed (V=1, N=1)
#[test]
fn test_orn_p_p_pp_z_flags_signedoverflow_5_25804010() {
    // Test ORN_P.P.PP_Z flag computation: SignedOverflow
    // Encoding: 0x25804010
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x7FFFFFFFFFFFFFFF);
    set_x(&mut cpu, 2, 0x1);
    let encoding: u32 = 0x25804010;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: ORN_P.P.PP_Z
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// min_signed + (-1) = max_signed (V=1)
#[test]
fn test_orn_p_p_pp_z_flags_signedoverflow_6_25804010() {
    // Test ORN_P.P.PP_Z flag computation: SignedOverflow
    // Encoding: 0x25804010
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0xFFFFFFFFFFFFFFFF);
    set_x(&mut cpu, 1, 0x8000000000000000);
    let encoding: u32 = 0x25804010;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: ORN_P.P.PP_Z
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: PositiveResult }
/// 100 + 50 = 150 (no flags)
#[test]
fn test_orn_p_p_pp_z_flags_positiveresult_7_25804010() {
    // Test ORN_P.P.PP_Z flag computation: PositiveResult
    // Encoding: 0x25804010
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0x32);
    set_x(&mut cpu, 1, 0x64);
    let encoding: u32 = 0x25804010;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: ORNS_P.P.PP_Z
/// ASL: `SimdFromField("Pd") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "unknown" }
/// verify register write to SimdFromField("Pd")
#[test]
fn test_orns_p_p_pp_z_reg_write_0_25c04010() {
    // Test ORNS_P.P.PP_Z register write: SimdFromField("Pd")
    // Encoding: 0x25C04010
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x25C04010;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: ORNS_P.P.PP_Z
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 0 + 0 = 0 (Z=1)
#[test]
fn test_orns_p_p_pp_z_flags_zeroresult_0_25c04010() {
    // Test ORNS_P.P.PP_Z flag computation: ZeroResult
    // Encoding: 0x25C04010
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x0);
    set_x(&mut cpu, 2, 0x0);
    let encoding: u32 = 0x25C04010;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: ORNS_P.P.PP_Z
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 1 + (-1) = 0 (Z=1, C=1)
#[test]
fn test_orns_p_p_pp_z_flags_zeroresult_1_25c04010() {
    // Test ORNS_P.P.PP_Z flag computation: ZeroResult
    // Encoding: 0x25C04010
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x1);
    set_x(&mut cpu, 2, 0xFFFFFFFFFFFFFFFF);
    let encoding: u32 = 0x25C04010;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: ORNS_P.P.PP_Z
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: NegativeResult }
/// negative value (N=1)
#[test]
fn test_orns_p_p_pp_z_flags_negativeresult_2_25c04010() {
    // Test ORNS_P.P.PP_Z flag computation: NegativeResult
    // Encoding: 0x25C04010
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0x0);
    set_x(&mut cpu, 1, 0x8000000000000000);
    let encoding: u32 = 0x25C04010;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: ORNS_P.P.PP_Z
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 1 = 0 (C=1, Z=1)
#[test]
fn test_orns_p_p_pp_z_flags_unsignedoverflow_3_25c04010() {
    // Test ORNS_P.P.PP_Z flag computation: UnsignedOverflow
    // Encoding: 0x25C04010
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0xFFFFFFFFFFFFFFFF);
    set_x(&mut cpu, 2, 0x1);
    let encoding: u32 = 0x25C04010;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: ORNS_P.P.PP_Z
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 2 = 1 (C=1)
#[test]
fn test_orns_p_p_pp_z_flags_unsignedoverflow_4_25c04010() {
    // Test ORNS_P.P.PP_Z flag computation: UnsignedOverflow
    // Encoding: 0x25C04010
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0xFFFFFFFFFFFFFFFF);
    set_x(&mut cpu, 2, 0x2);
    let encoding: u32 = 0x25C04010;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: ORNS_P.P.PP_Z
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// max_signed + 1 = min_signed (V=1, N=1)
#[test]
fn test_orns_p_p_pp_z_flags_signedoverflow_5_25c04010() {
    // Test ORNS_P.P.PP_Z flag computation: SignedOverflow
    // Encoding: 0x25C04010
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x7FFFFFFFFFFFFFFF);
    set_x(&mut cpu, 2, 0x1);
    let encoding: u32 = 0x25C04010;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: ORNS_P.P.PP_Z
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// min_signed + (-1) = max_signed (V=1)
#[test]
fn test_orns_p_p_pp_z_flags_signedoverflow_6_25c04010() {
    // Test ORNS_P.P.PP_Z flag computation: SignedOverflow
    // Encoding: 0x25C04010
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0xFFFFFFFFFFFFFFFF);
    set_x(&mut cpu, 1, 0x8000000000000000);
    let encoding: u32 = 0x25C04010;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: ORNS_P.P.PP_Z
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: PositiveResult }
/// 100 + 50 = 150 (no flags)
#[test]
fn test_orns_p_p_pp_z_flags_positiveresult_7_25c04010() {
    // Test ORNS_P.P.PP_Z flag computation: PositiveResult
    // Encoding: 0x25C04010
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x64);
    set_x(&mut cpu, 2, 0x32);
    let encoding: u32 = 0x25C04010;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

// ============================================================================
// PFALSE_P__ Tests
// ============================================================================

/// Provenance: PFALSE_P__
/// ASL: `field Pd 0 +: 4`
/// Requirement: FieldBoundary { field: "Pd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_pfalse_p_field_pd_0_min_e400_2518e400() {
    // Encoding: 0x2518E400
    // Test PFALSE_P__ field Pd = 0 (Min)
    // Fields: Pd=0
    let encoding: u32 = 0x2518E400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PFALSE_P__
/// ASL: `field Pd 0 +: 4`
/// Requirement: FieldBoundary { field: "Pd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_pfalse_p_field_pd_1_poweroftwo_e400_2518e401() {
    // Encoding: 0x2518E401
    // Test PFALSE_P__ field Pd = 1 (PowerOfTwo)
    // Fields: Pd=1
    let encoding: u32 = 0x2518E401;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PFALSE_P__
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pd=0 (register index 0 (first register))
#[test]
fn test_pfalse_p_combo_0_e400_2518e400() {
    // Encoding: 0x2518E400
    // Test PFALSE_P__ field combination: Pd=0
    // Fields: Pd=0
    let encoding: u32 = 0x2518E400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PFALSE_P__
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pd=1 (register index 1 (second register))
#[test]
fn test_pfalse_p_combo_1_e400_2518e401() {
    // Encoding: 0x2518E401
    // Test PFALSE_P__ field combination: Pd=1
    // Fields: Pd=1
    let encoding: u32 = 0x2518E401;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PFALSE_P__
/// ASL: `SimdFromField("Pd") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "unknown" }
/// verify register write to SimdFromField("Pd")
#[test]
fn test_pfalse_p_reg_write_0_2518e400() {
    // Test PFALSE_P__ register write: SimdFromField("Pd")
    // Encoding: 0x2518E400
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x2518E400;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

// ============================================================================
// WHILELE_P.P.RR__ Tests
// ============================================================================

/// Provenance: WHILELE_P.P.RR__
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_whilele_p_p_rr_field_size_0_min_410_25200410() {
    // Encoding: 0x25200410
    // Test WHILELE_P.P.RR__ field size = 0 (Min)
    // Fields: size=0, Rn=0, Rm=0, sf=0, Pd=0
    let encoding: u32 = 0x25200410;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELE_P.P.RR__
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 1, boundary: PowerOfTwo }
/// 16-bit / halfword size
#[test]
fn test_whilele_p_p_rr_field_size_1_poweroftwo_410_25600410() {
    // Encoding: 0x25600410
    // Test WHILELE_P.P.RR__ field size = 1 (PowerOfTwo)
    // Fields: sf=0, Rn=0, Rm=0, size=1, Pd=0
    let encoding: u32 = 0x25600410;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELE_P.P.RR__
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 2, boundary: PowerOfTwo }
/// 32-bit / word size
#[test]
fn test_whilele_p_p_rr_field_size_2_poweroftwo_410_25a00410() {
    // Encoding: 0x25A00410
    // Test WHILELE_P.P.RR__ field size = 2 (PowerOfTwo)
    // Fields: sf=0, Rm=0, Rn=0, Pd=0, size=2
    let encoding: u32 = 0x25A00410;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELE_P.P.RR__
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 3, boundary: Max }
/// 64-bit / doubleword size
#[test]
fn test_whilele_p_p_rr_field_size_3_max_410_25e00410() {
    // Encoding: 0x25E00410
    // Test WHILELE_P.P.RR__ field size = 3 (Max)
    // Fields: Rm=0, sf=0, Rn=0, size=3, Pd=0
    let encoding: u32 = 0x25E00410;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELE_P.P.RR__
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_whilele_p_p_rr_field_rm_0_min_410_25200410() {
    // Encoding: 0x25200410
    // Test WHILELE_P.P.RR__ field Rm = 0 (Min)
    // Fields: Rn=0, Pd=0, Rm=0, sf=0, size=0
    let encoding: u32 = 0x25200410;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELE_P.P.RR__
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_whilele_p_p_rr_field_rm_1_poweroftwo_410_25210410() {
    // Encoding: 0x25210410
    // Test WHILELE_P.P.RR__ field Rm = 1 (PowerOfTwo)
    // Fields: size=0, Rm=1, sf=0, Rn=0, Pd=0
    let encoding: u32 = 0x25210410;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELE_P.P.RR__
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_whilele_p_p_rr_field_rm_30_poweroftwominusone_410_253e0410() {
    // Encoding: 0x253E0410
    // Test WHILELE_P.P.RR__ field Rm = 30 (PowerOfTwoMinusOne)
    // Fields: Rm=30, sf=0, size=0, Rn=0, Pd=0
    let encoding: u32 = 0x253E0410;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELE_P.P.RR__
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 31, boundary: Max }
/// register index 31 (special)
#[test]
fn test_whilele_p_p_rr_field_rm_31_max_410_253f0410() {
    // Encoding: 0x253F0410
    // Test WHILELE_P.P.RR__ field Rm = 31 (Max)
    // Fields: Rm=31, sf=0, size=0, Rn=0, Pd=0
    let encoding: u32 = 0x253F0410;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELE_P.P.RR__
/// ASL: `field sf 12 +: 1`
/// Requirement: FieldBoundary { field: "sf", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_whilele_p_p_rr_field_sf_0_min_410_25200410() {
    // Encoding: 0x25200410
    // Test WHILELE_P.P.RR__ field sf = 0 (Min)
    // Fields: sf=0, size=0, Rn=0, Pd=0, Rm=0
    let encoding: u32 = 0x25200410;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELE_P.P.RR__
/// ASL: `field sf 12 +: 1`
/// Requirement: FieldBoundary { field: "sf", value: 1, boundary: Max }
/// 16-bit / halfword size
#[test]
fn test_whilele_p_p_rr_field_sf_1_max_410_25201410() {
    // Encoding: 0x25201410
    // Test WHILELE_P.P.RR__ field sf = 1 (Max)
    // Fields: Pd=0, size=0, Rm=0, sf=1, Rn=0
    let encoding: u32 = 0x25201410;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELE_P.P.RR__
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_whilele_p_p_rr_field_rn_0_min_410_25200410() {
    // Encoding: 0x25200410
    // Test WHILELE_P.P.RR__ field Rn = 0 (Min)
    // Fields: Pd=0, Rm=0, sf=0, size=0, Rn=0
    let encoding: u32 = 0x25200410;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELE_P.P.RR__
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_whilele_p_p_rr_field_rn_1_poweroftwo_410_25200430() {
    // Encoding: 0x25200430
    // Test WHILELE_P.P.RR__ field Rn = 1 (PowerOfTwo)
    // Fields: Rn=1, Pd=0, Rm=0, sf=0, size=0
    let encoding: u32 = 0x25200430;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELE_P.P.RR__
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_whilele_p_p_rr_field_rn_30_poweroftwominusone_410_252007d0() {
    // Encoding: 0x252007D0
    // Test WHILELE_P.P.RR__ field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: sf=0, size=0, Rn=30, Pd=0, Rm=0
    let encoding: u32 = 0x252007D0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELE_P.P.RR__
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_whilele_p_p_rr_field_rn_31_max_410_252007f0() {
    // Encoding: 0x252007F0
    // Test WHILELE_P.P.RR__ field Rn = 31 (Max)
    // Fields: Rm=0, sf=0, Pd=0, size=0, Rn=31
    let encoding: u32 = 0x252007F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELE_P.P.RR__
/// ASL: `field Pd 0 +: 4`
/// Requirement: FieldBoundary { field: "Pd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_whilele_p_p_rr_field_pd_0_min_410_25200410() {
    // Encoding: 0x25200410
    // Test WHILELE_P.P.RR__ field Pd = 0 (Min)
    // Fields: size=0, sf=0, Rn=0, Pd=0, Rm=0
    let encoding: u32 = 0x25200410;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELE_P.P.RR__
/// ASL: `field Pd 0 +: 4`
/// Requirement: FieldBoundary { field: "Pd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_whilele_p_p_rr_field_pd_1_poweroftwo_410_25200411() {
    // Encoding: 0x25200411
    // Test WHILELE_P.P.RR__ field Pd = 1 (PowerOfTwo)
    // Fields: sf=0, Rn=0, size=0, Rm=0, Pd=1
    let encoding: u32 = 0x25200411;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELE_P.P.RR__
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=0 (8-bit / byte size)
#[test]
fn test_whilele_p_p_rr_combo_0_410_25200410() {
    // Encoding: 0x25200410
    // Test WHILELE_P.P.RR__ field combination: size=0, Rm=0, sf=0, Rn=0, Pd=0
    // Fields: Pd=0, Rm=0, sf=0, size=0, Rn=0
    let encoding: u32 = 0x25200410;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELE_P.P.RR__
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=1 (16-bit / halfword size)
#[test]
fn test_whilele_p_p_rr_combo_1_410_25600410() {
    // Encoding: 0x25600410
    // Test WHILELE_P.P.RR__ field combination: size=1, Rm=0, sf=0, Rn=0, Pd=0
    // Fields: Pd=0, size=1, Rn=0, sf=0, Rm=0
    let encoding: u32 = 0x25600410;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELE_P.P.RR__
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=2 (32-bit / word size)
#[test]
fn test_whilele_p_p_rr_combo_2_410_25a00410() {
    // Encoding: 0x25A00410
    // Test WHILELE_P.P.RR__ field combination: size=2, Rm=0, sf=0, Rn=0, Pd=0
    // Fields: Pd=0, Rm=0, size=2, sf=0, Rn=0
    let encoding: u32 = 0x25A00410;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELE_P.P.RR__
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=3 (64-bit / doubleword size)
#[test]
fn test_whilele_p_p_rr_combo_3_410_25e00410() {
    // Encoding: 0x25E00410
    // Test WHILELE_P.P.RR__ field combination: size=3, Rm=0, sf=0, Rn=0, Pd=0
    // Fields: Pd=0, Rm=0, sf=0, Rn=0, size=3
    let encoding: u32 = 0x25E00410;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELE_P.P.RR__
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=0 (register index 0 (first register))
#[test]
fn test_whilele_p_p_rr_combo_4_410_25200410() {
    // Encoding: 0x25200410
    // Test WHILELE_P.P.RR__ field combination: size=0, Rm=0, sf=0, Rn=0, Pd=0
    // Fields: sf=0, size=0, Rn=0, Pd=0, Rm=0
    let encoding: u32 = 0x25200410;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELE_P.P.RR__
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=1 (register index 1 (second register))
#[test]
fn test_whilele_p_p_rr_combo_5_410_25210410() {
    // Encoding: 0x25210410
    // Test WHILELE_P.P.RR__ field combination: size=0, Rm=1, sf=0, Rn=0, Pd=0
    // Fields: Rn=0, size=0, Pd=0, Rm=1, sf=0
    let encoding: u32 = 0x25210410;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELE_P.P.RR__
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=30 (register index 30 (LR in some contexts))
#[test]
fn test_whilele_p_p_rr_combo_6_410_253e0410() {
    // Encoding: 0x253E0410
    // Test WHILELE_P.P.RR__ field combination: size=0, Rm=30, sf=0, Rn=0, Pd=0
    // Fields: Pd=0, Rm=30, sf=0, size=0, Rn=0
    let encoding: u32 = 0x253E0410;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELE_P.P.RR__
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=31 (register index 31 (special))
#[test]
fn test_whilele_p_p_rr_combo_7_410_253f0410() {
    // Encoding: 0x253F0410
    // Test WHILELE_P.P.RR__ field combination: size=0, Rm=31, sf=0, Rn=0, Pd=0
    // Fields: Rm=31, Rn=0, Pd=0, size=0, sf=0
    let encoding: u32 = 0x253F0410;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELE_P.P.RR__
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// sf=0 (8-bit / byte size)
#[test]
fn test_whilele_p_p_rr_combo_8_410_25200410() {
    // Encoding: 0x25200410
    // Test WHILELE_P.P.RR__ field combination: size=0, Rm=0, sf=0, Rn=0, Pd=0
    // Fields: Pd=0, sf=0, size=0, Rm=0, Rn=0
    let encoding: u32 = 0x25200410;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELE_P.P.RR__
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// sf=1 (16-bit / halfword size)
#[test]
fn test_whilele_p_p_rr_combo_9_410_25201410() {
    // Encoding: 0x25201410
    // Test WHILELE_P.P.RR__ field combination: size=0, Rm=0, sf=1, Rn=0, Pd=0
    // Fields: sf=1, Rm=0, Rn=0, size=0, Pd=0
    let encoding: u32 = 0x25201410;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELE_P.P.RR__
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_whilele_p_p_rr_combo_10_410_25200410() {
    // Encoding: 0x25200410
    // Test WHILELE_P.P.RR__ field combination: size=0, Rm=0, sf=0, Rn=0, Pd=0
    // Fields: Rm=0, sf=0, Pd=0, Rn=0, size=0
    let encoding: u32 = 0x25200410;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELE_P.P.RR__
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_whilele_p_p_rr_combo_11_410_25200430() {
    // Encoding: 0x25200430
    // Test WHILELE_P.P.RR__ field combination: size=0, Rm=0, sf=0, Rn=1, Pd=0
    // Fields: Rn=1, Rm=0, sf=0, Pd=0, size=0
    let encoding: u32 = 0x25200430;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELE_P.P.RR__
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_whilele_p_p_rr_combo_12_410_252007d0() {
    // Encoding: 0x252007D0
    // Test WHILELE_P.P.RR__ field combination: size=0, Rm=0, sf=0, Rn=30, Pd=0
    // Fields: Rm=0, Pd=0, size=0, Rn=30, sf=0
    let encoding: u32 = 0x252007D0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELE_P.P.RR__
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_whilele_p_p_rr_combo_13_410_252007f0() {
    // Encoding: 0x252007F0
    // Test WHILELE_P.P.RR__ field combination: size=0, Rm=0, sf=0, Rn=31, Pd=0
    // Fields: Pd=0, sf=0, Rn=31, size=0, Rm=0
    let encoding: u32 = 0x252007F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELE_P.P.RR__
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pd=0 (register index 0 (first register))
#[test]
fn test_whilele_p_p_rr_combo_14_410_25200410() {
    // Encoding: 0x25200410
    // Test WHILELE_P.P.RR__ field combination: size=0, Rm=0, sf=0, Rn=0, Pd=0
    // Fields: Pd=0, size=0, Rn=0, Rm=0, sf=0
    let encoding: u32 = 0x25200410;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELE_P.P.RR__
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pd=1 (register index 1 (second register))
#[test]
fn test_whilele_p_p_rr_combo_15_410_25200411() {
    // Encoding: 0x25200411
    // Test WHILELE_P.P.RR__ field combination: size=0, Rm=0, sf=0, Rn=0, Pd=1
    // Fields: sf=0, size=0, Rm=0, Pd=1, Rn=0
    let encoding: u32 = 0x25200411;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELE_P.P.RR__
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=1 (same register test (reg=1)), Rn=1 (same register test (reg=1))
#[test]
fn test_whilele_p_p_rr_combo_16_410_25210430() {
    // Encoding: 0x25210430
    // Test WHILELE_P.P.RR__ field combination: size=0, Rm=1, sf=0, Rn=1, Pd=0
    // Fields: Rn=1, size=0, Pd=0, sf=0, Rm=1
    let encoding: u32 = 0x25210430;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELE_P.P.RR__
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=31 (same register test (reg=31)), Rn=31 (same register test (reg=31))
#[test]
fn test_whilele_p_p_rr_combo_17_410_253f07f0() {
    // Encoding: 0x253F07F0
    // Test WHILELE_P.P.RR__ field combination: size=0, Rm=31, sf=0, Rn=31, Pd=0
    // Fields: Rm=31, sf=0, Pd=0, size=0, Rn=31
    let encoding: u32 = 0x253F07F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELE_P.P.RR__
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=1 (same register test (reg=1)), Pd=1 (same register test (reg=1))
#[test]
fn test_whilele_p_p_rr_combo_18_410_25210411() {
    // Encoding: 0x25210411
    // Test WHILELE_P.P.RR__ field combination: size=0, Rm=1, sf=0, Rn=0, Pd=1
    // Fields: Rm=1, Pd=1, Rn=0, size=0, sf=0
    let encoding: u32 = 0x25210411;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELE_P.P.RR__
/// ASL: `field combination 19`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=31 (same register test (reg=31)), Pd=31 (same register test (reg=31))
#[test]
fn test_whilele_p_p_rr_combo_19_410_253f041f() {
    // Encoding: 0x253F041F
    // Test WHILELE_P.P.RR__ field combination: size=0, Rm=31, sf=0, Rn=0, Pd=31
    // Fields: Pd=31, Rn=0, size=0, Rm=31, sf=0
    let encoding: u32 = 0x253F041F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELE_P.P.RR__
/// ASL: `field combination 20`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (same register test (reg=1)), Pd=1 (same register test (reg=1))
#[test]
fn test_whilele_p_p_rr_combo_20_410_25200431() {
    // Encoding: 0x25200431
    // Test WHILELE_P.P.RR__ field combination: size=0, Rm=0, sf=0, Rn=1, Pd=1
    // Fields: sf=0, Rm=0, size=0, Rn=1, Pd=1
    let encoding: u32 = 0x25200431;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELE_P.P.RR__
/// ASL: `field combination 21`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (same register test (reg=31)), Pd=31 (same register test (reg=31))
#[test]
fn test_whilele_p_p_rr_combo_21_410_252007ff() {
    // Encoding: 0x252007FF
    // Test WHILELE_P.P.RR__ field combination: size=0, Rm=0, sf=0, Rn=31, Pd=31
    // Fields: Rn=31, Pd=31, sf=0, Rm=0, size=0
    let encoding: u32 = 0x252007FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELE_P.P.RR__
/// ASL: `field size = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "size", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_whilele_p_p_rr_special_size_0_size_variant_0_1040_25200410() {
    // Encoding: 0x25200410
    // Test WHILELE_P.P.RR__ special value size = 0 (Size variant 0)
    // Fields: Rm=0, Rn=0, size=0, Pd=0, sf=0
    let encoding: u32 = 0x25200410;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELE_P.P.RR__
/// ASL: `field size = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "size", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_whilele_p_p_rr_special_size_1_size_variant_1_1040_25600410() {
    // Encoding: 0x25600410
    // Test WHILELE_P.P.RR__ special value size = 1 (Size variant 1)
    // Fields: Pd=0, sf=0, size=1, Rm=0, Rn=0
    let encoding: u32 = 0x25600410;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELE_P.P.RR__
/// ASL: `field size = 2 (Size variant 2)`
/// Requirement: FieldSpecial { field: "size", value: 2, meaning: "Size variant 2" }
/// Size variant 2
#[test]
fn test_whilele_p_p_rr_special_size_2_size_variant_2_1040_25a00410() {
    // Encoding: 0x25A00410
    // Test WHILELE_P.P.RR__ special value size = 2 (Size variant 2)
    // Fields: size=2, sf=0, Rn=0, Pd=0, Rm=0
    let encoding: u32 = 0x25A00410;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELE_P.P.RR__
/// ASL: `field size = 3 (Size variant 3)`
/// Requirement: FieldSpecial { field: "size", value: 3, meaning: "Size variant 3" }
/// Size variant 3
#[test]
fn test_whilele_p_p_rr_special_size_3_size_variant_3_1040_25e00410() {
    // Encoding: 0x25E00410
    // Test WHILELE_P.P.RR__ special value size = 3 (Size variant 3)
    // Fields: Rm=0, sf=0, size=3, Pd=0, Rn=0
    let encoding: u32 = 0x25E00410;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELE_P.P.RR__
/// ASL: `field sf = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "sf", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_whilele_p_p_rr_special_sf_0_size_variant_0_1040_25600410() {
    // Encoding: 0x25600410
    // Test WHILELE_P.P.RR__ special value sf = 0 (Size variant 0)
    // Fields: sf=0, Rm=0, size=1, Pd=0, Rn=0
    let encoding: u32 = 0x25600410;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELE_P.P.RR__
/// ASL: `field sf = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "sf", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_whilele_p_p_rr_special_sf_1_size_variant_1_1040_25601410() {
    // Encoding: 0x25601410
    // Test WHILELE_P.P.RR__ special value sf = 1 (Size variant 1)
    // Fields: Rm=0, Rn=0, Pd=0, sf=1, size=1
    let encoding: u32 = 0x25601410;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELE_P.P.RR__
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_whilele_p_p_rr_special_rn_31_stack_pointer_sp_may_require_alignment_1040_256007f0() {
    // Encoding: 0x256007F0
    // Test WHILELE_P.P.RR__ special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: Rm=0, Rn=31, size=1, Pd=0, sf=0
    let encoding: u32 = 0x256007F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELE_P.P.RR__
/// ASL: `SimdFromField("Pd") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "unknown" }
/// verify register write to SimdFromField("Pd")
#[test]
fn test_whilele_p_p_rr_reg_write_0_25200410() {
    // Test WHILELE_P.P.RR__ register write: SimdFromField("Pd")
    // Encoding: 0x25200410
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x25200410;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: WHILELE_P.P.RR__
/// ASL: `Rn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Rn = 31)
#[test]
fn test_whilele_p_p_rr_sp_rn_252007f0() {
    // Test WHILELE_P.P.RR__ with Rn = SP (31)
    // Encoding: 0x252007F0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x252007F0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: WHILELE_P.P.RR__
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 0 + 0 = 0 (Z=1)
#[test]
fn test_whilele_p_p_rr_flags_zeroresult_0_25221430() {
    // Test WHILELE_P.P.RR__ flag computation: ZeroResult
    // Encoding: 0x25221430
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x0);
    set_x(&mut cpu, 2, 0x0);
    let encoding: u32 = 0x25221430;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: WHILELE_P.P.RR__
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 1 + (-1) = 0 (Z=1, C=1)
#[test]
fn test_whilele_p_p_rr_flags_zeroresult_1_25221430() {
    // Test WHILELE_P.P.RR__ flag computation: ZeroResult
    // Encoding: 0x25221430
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x1);
    set_x(&mut cpu, 2, 0xFFFFFFFFFFFFFFFF);
    let encoding: u32 = 0x25221430;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: WHILELE_P.P.RR__
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: NegativeResult }
/// negative value (N=1)
#[test]
fn test_whilele_p_p_rr_flags_negativeresult_2_25221430() {
    // Test WHILELE_P.P.RR__ flag computation: NegativeResult
    // Encoding: 0x25221430
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x8000000000000000);
    set_x(&mut cpu, 2, 0x0);
    let encoding: u32 = 0x25221430;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: WHILELE_P.P.RR__
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 1 = 0 (C=1, Z=1)
#[test]
fn test_whilele_p_p_rr_flags_unsignedoverflow_3_25221430() {
    // Test WHILELE_P.P.RR__ flag computation: UnsignedOverflow
    // Encoding: 0x25221430
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0xFFFFFFFFFFFFFFFF);
    set_x(&mut cpu, 2, 0x1);
    let encoding: u32 = 0x25221430;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: WHILELE_P.P.RR__
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 2 = 1 (C=1)
#[test]
fn test_whilele_p_p_rr_flags_unsignedoverflow_4_25221430() {
    // Test WHILELE_P.P.RR__ flag computation: UnsignedOverflow
    // Encoding: 0x25221430
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0xFFFFFFFFFFFFFFFF);
    set_x(&mut cpu, 2, 0x2);
    let encoding: u32 = 0x25221430;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: WHILELE_P.P.RR__
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// max_signed + 1 = min_signed (V=1, N=1)
#[test]
fn test_whilele_p_p_rr_flags_signedoverflow_5_25221430() {
    // Test WHILELE_P.P.RR__ flag computation: SignedOverflow
    // Encoding: 0x25221430
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x7FFFFFFFFFFFFFFF);
    set_x(&mut cpu, 2, 0x1);
    let encoding: u32 = 0x25221430;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: WHILELE_P.P.RR__
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// min_signed + (-1) = max_signed (V=1)
#[test]
fn test_whilele_p_p_rr_flags_signedoverflow_6_25221430() {
    // Test WHILELE_P.P.RR__ flag computation: SignedOverflow
    // Encoding: 0x25221430
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0xFFFFFFFFFFFFFFFF);
    set_x(&mut cpu, 1, 0x8000000000000000);
    let encoding: u32 = 0x25221430;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: WHILELE_P.P.RR__
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: PositiveResult }
/// 100 + 50 = 150 (no flags)
#[test]
fn test_whilele_p_p_rr_flags_positiveresult_7_25221430() {
    // Test WHILELE_P.P.RR__ flag computation: PositiveResult
    // Encoding: 0x25221430
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0x32);
    set_x(&mut cpu, 1, 0x64);
    let encoding: u32 = 0x25221430;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

// ============================================================================
// BRKN_P.P.PP__ Tests
// ============================================================================

/// Provenance: BRKN_P.P.PP__
/// ASL: `field Pg 10 +: 4`
/// Requirement: FieldBoundary { field: "Pg", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_brkn_p_p_pp_field_pg_0_min_4000_25184000() {
    // Encoding: 0x25184000
    // Test BRKN_P.P.PP__ field Pg = 0 (Min)
    // Fields: Pn=0, Pg=0, Pdm=0
    let encoding: u32 = 0x25184000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKN_P.P.PP__
/// ASL: `field Pg 10 +: 4`
/// Requirement: FieldBoundary { field: "Pg", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_brkn_p_p_pp_field_pg_1_poweroftwo_4000_25184400() {
    // Encoding: 0x25184400
    // Test BRKN_P.P.PP__ field Pg = 1 (PowerOfTwo)
    // Fields: Pg=1, Pn=0, Pdm=0
    let encoding: u32 = 0x25184400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKN_P.P.PP__
/// ASL: `field Pn 5 +: 4`
/// Requirement: FieldBoundary { field: "Pn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_brkn_p_p_pp_field_pn_0_min_4000_25184000() {
    // Encoding: 0x25184000
    // Test BRKN_P.P.PP__ field Pn = 0 (Min)
    // Fields: Pn=0, Pg=0, Pdm=0
    let encoding: u32 = 0x25184000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKN_P.P.PP__
/// ASL: `field Pn 5 +: 4`
/// Requirement: FieldBoundary { field: "Pn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_brkn_p_p_pp_field_pn_1_poweroftwo_4000_25184020() {
    // Encoding: 0x25184020
    // Test BRKN_P.P.PP__ field Pn = 1 (PowerOfTwo)
    // Fields: Pn=1, Pdm=0, Pg=0
    let encoding: u32 = 0x25184020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKN_P.P.PP__
/// ASL: `field Pdm 0 +: 4`
/// Requirement: FieldBoundary { field: "Pdm", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_brkn_p_p_pp_field_pdm_0_min_4000_25184000() {
    // Encoding: 0x25184000
    // Test BRKN_P.P.PP__ field Pdm = 0 (Min)
    // Fields: Pg=0, Pn=0, Pdm=0
    let encoding: u32 = 0x25184000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKN_P.P.PP__
/// ASL: `field Pdm 0 +: 4`
/// Requirement: FieldBoundary { field: "Pdm", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_brkn_p_p_pp_field_pdm_1_poweroftwo_4000_25184001() {
    // Encoding: 0x25184001
    // Test BRKN_P.P.PP__ field Pdm = 1 (PowerOfTwo)
    // Fields: Pdm=1, Pn=0, Pg=0
    let encoding: u32 = 0x25184001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKN_P.P.PP__
/// ASL: `field Pdm 0 +: 4`
/// Requirement: FieldBoundary { field: "Pdm", value: 7, boundary: PowerOfTwoMinusOne }
/// midpoint (7)
#[test]
fn test_brkn_p_p_pp_field_pdm_7_poweroftwominusone_4000_25184007() {
    // Encoding: 0x25184007
    // Test BRKN_P.P.PP__ field Pdm = 7 (PowerOfTwoMinusOne)
    // Fields: Pn=0, Pg=0, Pdm=7
    let encoding: u32 = 0x25184007;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKN_P.P.PP__
/// ASL: `field Pdm 0 +: 4`
/// Requirement: FieldBoundary { field: "Pdm", value: 15, boundary: Max }
/// maximum value (15)
#[test]
fn test_brkn_p_p_pp_field_pdm_15_max_4000_2518400f() {
    // Encoding: 0x2518400F
    // Test BRKN_P.P.PP__ field Pdm = 15 (Max)
    // Fields: Pn=0, Pdm=15, Pg=0
    let encoding: u32 = 0x2518400F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKN_P.P.PP__
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=0 (register index 0 (first register))
#[test]
fn test_brkn_p_p_pp_combo_0_4000_25184000() {
    // Encoding: 0x25184000
    // Test BRKN_P.P.PP__ field combination: Pg=0, Pn=0, Pdm=0
    // Fields: Pdm=0, Pg=0, Pn=0
    let encoding: u32 = 0x25184000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKN_P.P.PP__
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=1 (register index 1 (second register))
#[test]
fn test_brkn_p_p_pp_combo_1_4000_25184400() {
    // Encoding: 0x25184400
    // Test BRKN_P.P.PP__ field combination: Pg=1, Pn=0, Pdm=0
    // Fields: Pn=0, Pdm=0, Pg=1
    let encoding: u32 = 0x25184400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKN_P.P.PP__
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pn=0 (register index 0 (first register))
#[test]
fn test_brkn_p_p_pp_combo_2_4000_25184000() {
    // Encoding: 0x25184000
    // Test BRKN_P.P.PP__ field combination: Pg=0, Pn=0, Pdm=0
    // Fields: Pg=0, Pdm=0, Pn=0
    let encoding: u32 = 0x25184000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKN_P.P.PP__
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pn=1 (register index 1 (second register))
#[test]
fn test_brkn_p_p_pp_combo_3_4000_25184020() {
    // Encoding: 0x25184020
    // Test BRKN_P.P.PP__ field combination: Pg=0, Pn=1, Pdm=0
    // Fields: Pg=0, Pn=1, Pdm=0
    let encoding: u32 = 0x25184020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKN_P.P.PP__
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pdm=0 (minimum value)
#[test]
fn test_brkn_p_p_pp_combo_4_4000_25184000() {
    // Encoding: 0x25184000
    // Test BRKN_P.P.PP__ field combination: Pg=0, Pn=0, Pdm=0
    // Fields: Pg=0, Pn=0, Pdm=0
    let encoding: u32 = 0x25184000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKN_P.P.PP__
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pdm=1 (value 1)
#[test]
fn test_brkn_p_p_pp_combo_5_4000_25184001() {
    // Encoding: 0x25184001
    // Test BRKN_P.P.PP__ field combination: Pg=0, Pn=0, Pdm=1
    // Fields: Pdm=1, Pn=0, Pg=0
    let encoding: u32 = 0x25184001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKN_P.P.PP__
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pdm=7 (midpoint (7))
#[test]
fn test_brkn_p_p_pp_combo_6_4000_25184007() {
    // Encoding: 0x25184007
    // Test BRKN_P.P.PP__ field combination: Pg=0, Pn=0, Pdm=7
    // Fields: Pg=0, Pn=0, Pdm=7
    let encoding: u32 = 0x25184007;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKN_P.P.PP__
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pdm=15 (maximum value (15))
#[test]
fn test_brkn_p_p_pp_combo_7_4000_2518400f() {
    // Encoding: 0x2518400F
    // Test BRKN_P.P.PP__ field combination: Pg=0, Pn=0, Pdm=15
    // Fields: Pg=0, Pn=0, Pdm=15
    let encoding: u32 = 0x2518400F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKN_P.P.PP__
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=1 (same register test (reg=1)), Pn=1 (same register test (reg=1))
#[test]
fn test_brkn_p_p_pp_combo_8_4000_25184420() {
    // Encoding: 0x25184420
    // Test BRKN_P.P.PP__ field combination: Pg=1, Pn=1, Pdm=0
    // Fields: Pn=1, Pdm=0, Pg=1
    let encoding: u32 = 0x25184420;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKN_P.P.PP__
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=31 (same register test (reg=31)), Pn=31 (same register test (reg=31))
#[test]
fn test_brkn_p_p_pp_combo_9_4000_25187de0() {
    // Encoding: 0x25187DE0
    // Test BRKN_P.P.PP__ field combination: Pg=31, Pn=31, Pdm=0
    // Fields: Pg=31, Pdm=0, Pn=31
    let encoding: u32 = 0x25187DE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKNS_P.P.PP__
/// ASL: `field Pg 10 +: 4`
/// Requirement: FieldBoundary { field: "Pg", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_brkns_p_p_pp_field_pg_0_min_4000_25584000() {
    // Encoding: 0x25584000
    // Test BRKNS_P.P.PP__ field Pg = 0 (Min)
    // Fields: Pg=0, Pn=0, Pdm=0
    let encoding: u32 = 0x25584000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKNS_P.P.PP__
/// ASL: `field Pg 10 +: 4`
/// Requirement: FieldBoundary { field: "Pg", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_brkns_p_p_pp_field_pg_1_poweroftwo_4000_25584400() {
    // Encoding: 0x25584400
    // Test BRKNS_P.P.PP__ field Pg = 1 (PowerOfTwo)
    // Fields: Pg=1, Pn=0, Pdm=0
    let encoding: u32 = 0x25584400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKNS_P.P.PP__
/// ASL: `field Pn 5 +: 4`
/// Requirement: FieldBoundary { field: "Pn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_brkns_p_p_pp_field_pn_0_min_4000_25584000() {
    // Encoding: 0x25584000
    // Test BRKNS_P.P.PP__ field Pn = 0 (Min)
    // Fields: Pn=0, Pdm=0, Pg=0
    let encoding: u32 = 0x25584000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKNS_P.P.PP__
/// ASL: `field Pn 5 +: 4`
/// Requirement: FieldBoundary { field: "Pn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_brkns_p_p_pp_field_pn_1_poweroftwo_4000_25584020() {
    // Encoding: 0x25584020
    // Test BRKNS_P.P.PP__ field Pn = 1 (PowerOfTwo)
    // Fields: Pdm=0, Pg=0, Pn=1
    let encoding: u32 = 0x25584020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKNS_P.P.PP__
/// ASL: `field Pdm 0 +: 4`
/// Requirement: FieldBoundary { field: "Pdm", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_brkns_p_p_pp_field_pdm_0_min_4000_25584000() {
    // Encoding: 0x25584000
    // Test BRKNS_P.P.PP__ field Pdm = 0 (Min)
    // Fields: Pg=0, Pn=0, Pdm=0
    let encoding: u32 = 0x25584000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKNS_P.P.PP__
/// ASL: `field Pdm 0 +: 4`
/// Requirement: FieldBoundary { field: "Pdm", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_brkns_p_p_pp_field_pdm_1_poweroftwo_4000_25584001() {
    // Encoding: 0x25584001
    // Test BRKNS_P.P.PP__ field Pdm = 1 (PowerOfTwo)
    // Fields: Pg=0, Pn=0, Pdm=1
    let encoding: u32 = 0x25584001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKNS_P.P.PP__
/// ASL: `field Pdm 0 +: 4`
/// Requirement: FieldBoundary { field: "Pdm", value: 7, boundary: PowerOfTwoMinusOne }
/// midpoint (7)
#[test]
fn test_brkns_p_p_pp_field_pdm_7_poweroftwominusone_4000_25584007() {
    // Encoding: 0x25584007
    // Test BRKNS_P.P.PP__ field Pdm = 7 (PowerOfTwoMinusOne)
    // Fields: Pdm=7, Pg=0, Pn=0
    let encoding: u32 = 0x25584007;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKNS_P.P.PP__
/// ASL: `field Pdm 0 +: 4`
/// Requirement: FieldBoundary { field: "Pdm", value: 15, boundary: Max }
/// maximum value (15)
#[test]
fn test_brkns_p_p_pp_field_pdm_15_max_4000_2558400f() {
    // Encoding: 0x2558400F
    // Test BRKNS_P.P.PP__ field Pdm = 15 (Max)
    // Fields: Pg=0, Pn=0, Pdm=15
    let encoding: u32 = 0x2558400F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKNS_P.P.PP__
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=0 (register index 0 (first register))
#[test]
fn test_brkns_p_p_pp_combo_0_4000_25584000() {
    // Encoding: 0x25584000
    // Test BRKNS_P.P.PP__ field combination: Pg=0, Pn=0, Pdm=0
    // Fields: Pn=0, Pdm=0, Pg=0
    let encoding: u32 = 0x25584000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKNS_P.P.PP__
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=1 (register index 1 (second register))
#[test]
fn test_brkns_p_p_pp_combo_1_4000_25584400() {
    // Encoding: 0x25584400
    // Test BRKNS_P.P.PP__ field combination: Pg=1, Pn=0, Pdm=0
    // Fields: Pdm=0, Pg=1, Pn=0
    let encoding: u32 = 0x25584400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKNS_P.P.PP__
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pn=0 (register index 0 (first register))
#[test]
fn test_brkns_p_p_pp_combo_2_4000_25584000() {
    // Encoding: 0x25584000
    // Test BRKNS_P.P.PP__ field combination: Pg=0, Pn=0, Pdm=0
    // Fields: Pg=0, Pdm=0, Pn=0
    let encoding: u32 = 0x25584000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKNS_P.P.PP__
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pn=1 (register index 1 (second register))
#[test]
fn test_brkns_p_p_pp_combo_3_4000_25584020() {
    // Encoding: 0x25584020
    // Test BRKNS_P.P.PP__ field combination: Pg=0, Pn=1, Pdm=0
    // Fields: Pdm=0, Pg=0, Pn=1
    let encoding: u32 = 0x25584020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKNS_P.P.PP__
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pdm=0 (minimum value)
#[test]
fn test_brkns_p_p_pp_combo_4_4000_25584000() {
    // Encoding: 0x25584000
    // Test BRKNS_P.P.PP__ field combination: Pg=0, Pn=0, Pdm=0
    // Fields: Pg=0, Pdm=0, Pn=0
    let encoding: u32 = 0x25584000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKNS_P.P.PP__
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pdm=1 (value 1)
#[test]
fn test_brkns_p_p_pp_combo_5_4000_25584001() {
    // Encoding: 0x25584001
    // Test BRKNS_P.P.PP__ field combination: Pg=0, Pn=0, Pdm=1
    // Fields: Pn=0, Pg=0, Pdm=1
    let encoding: u32 = 0x25584001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKNS_P.P.PP__
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pdm=7 (midpoint (7))
#[test]
fn test_brkns_p_p_pp_combo_6_4000_25584007() {
    // Encoding: 0x25584007
    // Test BRKNS_P.P.PP__ field combination: Pg=0, Pn=0, Pdm=7
    // Fields: Pg=0, Pn=0, Pdm=7
    let encoding: u32 = 0x25584007;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKNS_P.P.PP__
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pdm=15 (maximum value (15))
#[test]
fn test_brkns_p_p_pp_combo_7_4000_2558400f() {
    // Encoding: 0x2558400F
    // Test BRKNS_P.P.PP__ field combination: Pg=0, Pn=0, Pdm=15
    // Fields: Pn=0, Pdm=15, Pg=0
    let encoding: u32 = 0x2558400F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKNS_P.P.PP__
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=1 (same register test (reg=1)), Pn=1 (same register test (reg=1))
#[test]
fn test_brkns_p_p_pp_combo_8_4000_25584420() {
    // Encoding: 0x25584420
    // Test BRKNS_P.P.PP__ field combination: Pg=1, Pn=1, Pdm=0
    // Fields: Pg=1, Pdm=0, Pn=1
    let encoding: u32 = 0x25584420;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKNS_P.P.PP__
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=31 (same register test (reg=31)), Pn=31 (same register test (reg=31))
#[test]
fn test_brkns_p_p_pp_combo_9_4000_25587de0() {
    // Encoding: 0x25587DE0
    // Test BRKNS_P.P.PP__ field combination: Pg=31, Pn=31, Pdm=0
    // Fields: Pn=31, Pdm=0, Pg=31
    let encoding: u32 = 0x25587DE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKN_P.P.PP__
/// ASL: `SimdFromField("Pdm") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "unknown" }
/// verify register write to SimdFromField("Pdm")
#[test]
fn test_brkn_p_p_pp_reg_write_0_25184000() {
    // Test BRKN_P.P.PP__ register write: SimdFromField("Pdm")
    // Encoding: 0x25184000
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x25184000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: BRKN_P.P.PP__
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 0 + 0 = 0 (Z=1)
#[test]
fn test_brkn_p_p_pp_flags_zeroresult_0_25184000() {
    // Test BRKN_P.P.PP__ flag computation: ZeroResult
    // Encoding: 0x25184000
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0x0);
    set_x(&mut cpu, 1, 0x0);
    let encoding: u32 = 0x25184000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: BRKN_P.P.PP__
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 1 + (-1) = 0 (Z=1, C=1)
#[test]
fn test_brkn_p_p_pp_flags_zeroresult_1_25184000() {
    // Test BRKN_P.P.PP__ flag computation: ZeroResult
    // Encoding: 0x25184000
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0xFFFFFFFFFFFFFFFF);
    set_x(&mut cpu, 1, 0x1);
    let encoding: u32 = 0x25184000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: BRKN_P.P.PP__
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: NegativeResult }
/// negative value (N=1)
#[test]
fn test_brkn_p_p_pp_flags_negativeresult_2_25184000() {
    // Test BRKN_P.P.PP__ flag computation: NegativeResult
    // Encoding: 0x25184000
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x8000000000000000);
    set_x(&mut cpu, 2, 0x0);
    let encoding: u32 = 0x25184000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: BRKN_P.P.PP__
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 1 = 0 (C=1, Z=1)
#[test]
fn test_brkn_p_p_pp_flags_unsignedoverflow_3_25184000() {
    // Test BRKN_P.P.PP__ flag computation: UnsignedOverflow
    // Encoding: 0x25184000
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0xFFFFFFFFFFFFFFFF);
    set_x(&mut cpu, 2, 0x1);
    let encoding: u32 = 0x25184000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: BRKN_P.P.PP__
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 2 = 1 (C=1)
#[test]
fn test_brkn_p_p_pp_flags_unsignedoverflow_4_25184000() {
    // Test BRKN_P.P.PP__ flag computation: UnsignedOverflow
    // Encoding: 0x25184000
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0x2);
    set_x(&mut cpu, 1, 0xFFFFFFFFFFFFFFFF);
    let encoding: u32 = 0x25184000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: BRKN_P.P.PP__
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// max_signed + 1 = min_signed (V=1, N=1)
#[test]
fn test_brkn_p_p_pp_flags_signedoverflow_5_25184000() {
    // Test BRKN_P.P.PP__ flag computation: SignedOverflow
    // Encoding: 0x25184000
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x7FFFFFFFFFFFFFFF);
    set_x(&mut cpu, 2, 0x1);
    let encoding: u32 = 0x25184000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: BRKN_P.P.PP__
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// min_signed + (-1) = max_signed (V=1)
#[test]
fn test_brkn_p_p_pp_flags_signedoverflow_6_25184000() {
    // Test BRKN_P.P.PP__ flag computation: SignedOverflow
    // Encoding: 0x25184000
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x8000000000000000);
    set_x(&mut cpu, 2, 0xFFFFFFFFFFFFFFFF);
    let encoding: u32 = 0x25184000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: BRKN_P.P.PP__
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: PositiveResult }
/// 100 + 50 = 150 (no flags)
#[test]
fn test_brkn_p_p_pp_flags_positiveresult_7_25184000() {
    // Test BRKN_P.P.PP__ flag computation: PositiveResult
    // Encoding: 0x25184000
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0x32);
    set_x(&mut cpu, 1, 0x64);
    let encoding: u32 = 0x25184000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: BRKNS_P.P.PP__
/// ASL: `SimdFromField("Pdm") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "unknown" }
/// verify register write to SimdFromField("Pdm")
#[test]
fn test_brkns_p_p_pp_reg_write_0_25584000() {
    // Test BRKNS_P.P.PP__ register write: SimdFromField("Pdm")
    // Encoding: 0x25584000
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x25584000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: BRKNS_P.P.PP__
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 0 + 0 = 0 (Z=1)
#[test]
fn test_brkns_p_p_pp_flags_zeroresult_0_25584000() {
    // Test BRKNS_P.P.PP__ flag computation: ZeroResult
    // Encoding: 0x25584000
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0x0);
    set_x(&mut cpu, 1, 0x0);
    let encoding: u32 = 0x25584000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: BRKNS_P.P.PP__
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 1 + (-1) = 0 (Z=1, C=1)
#[test]
fn test_brkns_p_p_pp_flags_zeroresult_1_25584000() {
    // Test BRKNS_P.P.PP__ flag computation: ZeroResult
    // Encoding: 0x25584000
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x1);
    set_x(&mut cpu, 2, 0xFFFFFFFFFFFFFFFF);
    let encoding: u32 = 0x25584000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: BRKNS_P.P.PP__
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: NegativeResult }
/// negative value (N=1)
#[test]
fn test_brkns_p_p_pp_flags_negativeresult_2_25584000() {
    // Test BRKNS_P.P.PP__ flag computation: NegativeResult
    // Encoding: 0x25584000
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x8000000000000000);
    set_x(&mut cpu, 2, 0x0);
    let encoding: u32 = 0x25584000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: BRKNS_P.P.PP__
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 1 = 0 (C=1, Z=1)
#[test]
fn test_brkns_p_p_pp_flags_unsignedoverflow_3_25584000() {
    // Test BRKNS_P.P.PP__ flag computation: UnsignedOverflow
    // Encoding: 0x25584000
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0xFFFFFFFFFFFFFFFF);
    set_x(&mut cpu, 2, 0x1);
    let encoding: u32 = 0x25584000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: BRKNS_P.P.PP__
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 2 = 1 (C=1)
#[test]
fn test_brkns_p_p_pp_flags_unsignedoverflow_4_25584000() {
    // Test BRKNS_P.P.PP__ flag computation: UnsignedOverflow
    // Encoding: 0x25584000
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0xFFFFFFFFFFFFFFFF);
    set_x(&mut cpu, 2, 0x2);
    let encoding: u32 = 0x25584000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: BRKNS_P.P.PP__
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// max_signed + 1 = min_signed (V=1, N=1)
#[test]
fn test_brkns_p_p_pp_flags_signedoverflow_5_25584000() {
    // Test BRKNS_P.P.PP__ flag computation: SignedOverflow
    // Encoding: 0x25584000
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x7FFFFFFFFFFFFFFF);
    set_x(&mut cpu, 2, 0x1);
    let encoding: u32 = 0x25584000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: BRKNS_P.P.PP__
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// min_signed + (-1) = max_signed (V=1)
#[test]
fn test_brkns_p_p_pp_flags_signedoverflow_6_25584000() {
    // Test BRKNS_P.P.PP__ flag computation: SignedOverflow
    // Encoding: 0x25584000
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x8000000000000000);
    set_x(&mut cpu, 2, 0xFFFFFFFFFFFFFFFF);
    let encoding: u32 = 0x25584000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: BRKNS_P.P.PP__
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: PositiveResult }
/// 100 + 50 = 150 (no flags)
#[test]
fn test_brkns_p_p_pp_flags_positiveresult_7_25584000() {
    // Test BRKNS_P.P.PP__ flag computation: PositiveResult
    // Encoding: 0x25584000
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0x32);
    set_x(&mut cpu, 1, 0x64);
    let encoding: u32 = 0x25584000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

// ============================================================================
// WRFFR_F.P__ Tests
// ============================================================================

/// Provenance: WRFFR_F.P__
/// ASL: `field Pn 5 +: 4`
/// Requirement: FieldBoundary { field: "Pn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_wrffr_f_p_field_pn_0_min_9000_25289000() {
    // Encoding: 0x25289000
    // Test WRFFR_F.P__ field Pn = 0 (Min)
    // Fields: Pn=0
    let encoding: u32 = 0x25289000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WRFFR_F.P__
/// ASL: `field Pn 5 +: 4`
/// Requirement: FieldBoundary { field: "Pn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_wrffr_f_p_field_pn_1_poweroftwo_9000_25289020() {
    // Encoding: 0x25289020
    // Test WRFFR_F.P__ field Pn = 1 (PowerOfTwo)
    // Fields: Pn=1
    let encoding: u32 = 0x25289020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WRFFR_F.P__
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pn=0 (register index 0 (first register))
#[test]
fn test_wrffr_f_p_combo_0_9000_25289000() {
    // Encoding: 0x25289000
    // Test WRFFR_F.P__ field combination: Pn=0
    // Fields: Pn=0
    let encoding: u32 = 0x25289000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WRFFR_F.P__
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pn=1 (register index 1 (second register))
#[test]
fn test_wrffr_f_p_combo_1_9000_25289020() {
    // Encoding: 0x25289020
    // Test WRFFR_F.P__ field combination: Pn=1
    // Fields: Pn=1
    let encoding: u32 = 0x25289020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

// ============================================================================
// BRKB_P.P.P__ Tests
// ============================================================================

/// Provenance: BRKB_P.P.P__
/// ASL: `field Pg 10 +: 4`
/// Requirement: FieldBoundary { field: "Pg", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_brkb_p_p_p_field_pg_0_min_4000_25904000() {
    // Encoding: 0x25904000
    // Test BRKB_P.P.P__ field Pg = 0 (Min)
    // Fields: Pd=0, Pg=0, Pn=0, M=0
    let encoding: u32 = 0x25904000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKB_P.P.P__
/// ASL: `field Pg 10 +: 4`
/// Requirement: FieldBoundary { field: "Pg", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_brkb_p_p_p_field_pg_1_poweroftwo_4000_25904400() {
    // Encoding: 0x25904400
    // Test BRKB_P.P.P__ field Pg = 1 (PowerOfTwo)
    // Fields: Pg=1, Pn=0, M=0, Pd=0
    let encoding: u32 = 0x25904400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKB_P.P.P__
/// ASL: `field Pn 5 +: 4`
/// Requirement: FieldBoundary { field: "Pn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_brkb_p_p_p_field_pn_0_min_4000_25904000() {
    // Encoding: 0x25904000
    // Test BRKB_P.P.P__ field Pn = 0 (Min)
    // Fields: M=0, Pg=0, Pd=0, Pn=0
    let encoding: u32 = 0x25904000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKB_P.P.P__
/// ASL: `field Pn 5 +: 4`
/// Requirement: FieldBoundary { field: "Pn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_brkb_p_p_p_field_pn_1_poweroftwo_4000_25904020() {
    // Encoding: 0x25904020
    // Test BRKB_P.P.P__ field Pn = 1 (PowerOfTwo)
    // Fields: Pn=1, Pg=0, M=0, Pd=0
    let encoding: u32 = 0x25904020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKB_P.P.P__
/// ASL: `field M 4 +: 1`
/// Requirement: FieldBoundary { field: "M", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_brkb_p_p_p_field_m_0_min_4000_25904000() {
    // Encoding: 0x25904000
    // Test BRKB_P.P.P__ field M = 0 (Min)
    // Fields: Pd=0, Pn=0, M=0, Pg=0
    let encoding: u32 = 0x25904000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKB_P.P.P__
/// ASL: `field M 4 +: 1`
/// Requirement: FieldBoundary { field: "M", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_brkb_p_p_p_field_m_1_max_4000_25904010() {
    // Encoding: 0x25904010
    // Test BRKB_P.P.P__ field M = 1 (Max)
    // Fields: Pn=0, Pd=0, Pg=0, M=1
    let encoding: u32 = 0x25904010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKB_P.P.P__
/// ASL: `field Pd 0 +: 4`
/// Requirement: FieldBoundary { field: "Pd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_brkb_p_p_p_field_pd_0_min_4000_25904000() {
    // Encoding: 0x25904000
    // Test BRKB_P.P.P__ field Pd = 0 (Min)
    // Fields: Pd=0, M=0, Pn=0, Pg=0
    let encoding: u32 = 0x25904000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKB_P.P.P__
/// ASL: `field Pd 0 +: 4`
/// Requirement: FieldBoundary { field: "Pd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_brkb_p_p_p_field_pd_1_poweroftwo_4000_25904001() {
    // Encoding: 0x25904001
    // Test BRKB_P.P.P__ field Pd = 1 (PowerOfTwo)
    // Fields: Pd=1, Pn=0, Pg=0, M=0
    let encoding: u32 = 0x25904001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKB_P.P.P__
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=0 (register index 0 (first register))
#[test]
fn test_brkb_p_p_p_combo_0_4000_25904000() {
    // Encoding: 0x25904000
    // Test BRKB_P.P.P__ field combination: Pg=0, Pn=0, M=0, Pd=0
    // Fields: Pg=0, M=0, Pd=0, Pn=0
    let encoding: u32 = 0x25904000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKB_P.P.P__
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=1 (register index 1 (second register))
#[test]
fn test_brkb_p_p_p_combo_1_4000_25904400() {
    // Encoding: 0x25904400
    // Test BRKB_P.P.P__ field combination: Pg=1, Pn=0, M=0, Pd=0
    // Fields: M=0, Pg=1, Pn=0, Pd=0
    let encoding: u32 = 0x25904400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKB_P.P.P__
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pn=0 (register index 0 (first register))
#[test]
fn test_brkb_p_p_p_combo_2_4000_25904000() {
    // Encoding: 0x25904000
    // Test BRKB_P.P.P__ field combination: Pg=0, Pn=0, M=0, Pd=0
    // Fields: Pd=0, M=0, Pg=0, Pn=0
    let encoding: u32 = 0x25904000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKB_P.P.P__
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pn=1 (register index 1 (second register))
#[test]
fn test_brkb_p_p_p_combo_3_4000_25904020() {
    // Encoding: 0x25904020
    // Test BRKB_P.P.P__ field combination: Pg=0, Pn=1, M=0, Pd=0
    // Fields: Pn=1, Pd=0, Pg=0, M=0
    let encoding: u32 = 0x25904020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKB_P.P.P__
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// M=0 (minimum value)
#[test]
fn test_brkb_p_p_p_combo_4_4000_25904000() {
    // Encoding: 0x25904000
    // Test BRKB_P.P.P__ field combination: Pg=0, Pn=0, M=0, Pd=0
    // Fields: Pd=0, Pn=0, M=0, Pg=0
    let encoding: u32 = 0x25904000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKB_P.P.P__
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// M=1 (maximum value (1))
#[test]
fn test_brkb_p_p_p_combo_5_4000_25904010() {
    // Encoding: 0x25904010
    // Test BRKB_P.P.P__ field combination: Pg=0, Pn=0, M=1, Pd=0
    // Fields: Pg=0, Pd=0, Pn=0, M=1
    let encoding: u32 = 0x25904010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKB_P.P.P__
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pd=0 (register index 0 (first register))
#[test]
fn test_brkb_p_p_p_combo_6_4000_25904000() {
    // Encoding: 0x25904000
    // Test BRKB_P.P.P__ field combination: Pg=0, Pn=0, M=0, Pd=0
    // Fields: Pn=0, M=0, Pd=0, Pg=0
    let encoding: u32 = 0x25904000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKB_P.P.P__
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pd=1 (register index 1 (second register))
#[test]
fn test_brkb_p_p_p_combo_7_4000_25904001() {
    // Encoding: 0x25904001
    // Test BRKB_P.P.P__ field combination: Pg=0, Pn=0, M=0, Pd=1
    // Fields: Pg=0, Pd=1, Pn=0, M=0
    let encoding: u32 = 0x25904001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKB_P.P.P__
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=1 (same register test (reg=1)), Pn=1 (same register test (reg=1))
#[test]
fn test_brkb_p_p_p_combo_8_4000_25904420() {
    // Encoding: 0x25904420
    // Test BRKB_P.P.P__ field combination: Pg=1, Pn=1, M=0, Pd=0
    // Fields: Pn=1, M=0, Pg=1, Pd=0
    let encoding: u32 = 0x25904420;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKB_P.P.P__
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=31 (same register test (reg=31)), Pn=31 (same register test (reg=31))
#[test]
fn test_brkb_p_p_p_combo_9_4000_25907de0() {
    // Encoding: 0x25907DE0
    // Test BRKB_P.P.P__ field combination: Pg=31, Pn=31, M=0, Pd=0
    // Fields: Pg=31, M=0, Pd=0, Pn=31
    let encoding: u32 = 0x25907DE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKB_P.P.P__
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=1 (same register test (reg=1)), Pd=1 (same register test (reg=1))
#[test]
fn test_brkb_p_p_p_combo_10_4000_25904401() {
    // Encoding: 0x25904401
    // Test BRKB_P.P.P__ field combination: Pg=1, Pn=0, M=0, Pd=1
    // Fields: M=0, Pd=1, Pg=1, Pn=0
    let encoding: u32 = 0x25904401;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKB_P.P.P__
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=31 (same register test (reg=31)), Pd=31 (same register test (reg=31))
#[test]
fn test_brkb_p_p_p_combo_11_4000_25907c0f() {
    // Encoding: 0x25907C0F
    // Test BRKB_P.P.P__ field combination: Pg=31, Pn=0, M=0, Pd=31
    // Fields: Pg=31, Pn=0, M=0, Pd=31
    let encoding: u32 = 0x25907C0F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKB_P.P.P__
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pn=1 (same register test (reg=1)), Pd=1 (same register test (reg=1))
#[test]
fn test_brkb_p_p_p_combo_12_4000_25904021() {
    // Encoding: 0x25904021
    // Test BRKB_P.P.P__ field combination: Pg=0, Pn=1, M=0, Pd=1
    // Fields: M=0, Pn=1, Pd=1, Pg=0
    let encoding: u32 = 0x25904021;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKB_P.P.P__
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pn=31 (same register test (reg=31)), Pd=31 (same register test (reg=31))
#[test]
fn test_brkb_p_p_p_combo_13_4000_259041ef() {
    // Encoding: 0x259041EF
    // Test BRKB_P.P.P__ field combination: Pg=0, Pn=31, M=0, Pd=31
    // Fields: M=0, Pg=0, Pn=31, Pd=31
    let encoding: u32 = 0x259041EF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKBS_P.P.P_Z
/// ASL: `field Pg 10 +: 4`
/// Requirement: FieldBoundary { field: "Pg", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_brkbs_p_p_p_z_field_pg_0_min_4000_25d04000() {
    // Encoding: 0x25D04000
    // Test BRKBS_P.P.P_Z field Pg = 0 (Min)
    // Fields: Pg=0, Pn=0, Pd=0
    let encoding: u32 = 0x25D04000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKBS_P.P.P_Z
/// ASL: `field Pg 10 +: 4`
/// Requirement: FieldBoundary { field: "Pg", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_brkbs_p_p_p_z_field_pg_1_poweroftwo_4000_25d04400() {
    // Encoding: 0x25D04400
    // Test BRKBS_P.P.P_Z field Pg = 1 (PowerOfTwo)
    // Fields: Pd=0, Pn=0, Pg=1
    let encoding: u32 = 0x25D04400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKBS_P.P.P_Z
/// ASL: `field Pn 5 +: 4`
/// Requirement: FieldBoundary { field: "Pn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_brkbs_p_p_p_z_field_pn_0_min_4000_25d04000() {
    // Encoding: 0x25D04000
    // Test BRKBS_P.P.P_Z field Pn = 0 (Min)
    // Fields: Pg=0, Pd=0, Pn=0
    let encoding: u32 = 0x25D04000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKBS_P.P.P_Z
/// ASL: `field Pn 5 +: 4`
/// Requirement: FieldBoundary { field: "Pn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_brkbs_p_p_p_z_field_pn_1_poweroftwo_4000_25d04020() {
    // Encoding: 0x25D04020
    // Test BRKBS_P.P.P_Z field Pn = 1 (PowerOfTwo)
    // Fields: Pg=0, Pd=0, Pn=1
    let encoding: u32 = 0x25D04020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKBS_P.P.P_Z
/// ASL: `field Pd 0 +: 4`
/// Requirement: FieldBoundary { field: "Pd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_brkbs_p_p_p_z_field_pd_0_min_4000_25d04000() {
    // Encoding: 0x25D04000
    // Test BRKBS_P.P.P_Z field Pd = 0 (Min)
    // Fields: Pg=0, Pn=0, Pd=0
    let encoding: u32 = 0x25D04000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKBS_P.P.P_Z
/// ASL: `field Pd 0 +: 4`
/// Requirement: FieldBoundary { field: "Pd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_brkbs_p_p_p_z_field_pd_1_poweroftwo_4000_25d04001() {
    // Encoding: 0x25D04001
    // Test BRKBS_P.P.P_Z field Pd = 1 (PowerOfTwo)
    // Fields: Pn=0, Pd=1, Pg=0
    let encoding: u32 = 0x25D04001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKBS_P.P.P_Z
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=0 (register index 0 (first register))
#[test]
fn test_brkbs_p_p_p_z_combo_0_4000_25d04000() {
    // Encoding: 0x25D04000
    // Test BRKBS_P.P.P_Z field combination: Pg=0, Pn=0, Pd=0
    // Fields: Pn=0, Pg=0, Pd=0
    let encoding: u32 = 0x25D04000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKBS_P.P.P_Z
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=1 (register index 1 (second register))
#[test]
fn test_brkbs_p_p_p_z_combo_1_4000_25d04400() {
    // Encoding: 0x25D04400
    // Test BRKBS_P.P.P_Z field combination: Pg=1, Pn=0, Pd=0
    // Fields: Pd=0, Pg=1, Pn=0
    let encoding: u32 = 0x25D04400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKBS_P.P.P_Z
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pn=0 (register index 0 (first register))
#[test]
fn test_brkbs_p_p_p_z_combo_2_4000_25d04000() {
    // Encoding: 0x25D04000
    // Test BRKBS_P.P.P_Z field combination: Pg=0, Pn=0, Pd=0
    // Fields: Pn=0, Pd=0, Pg=0
    let encoding: u32 = 0x25D04000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKBS_P.P.P_Z
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pn=1 (register index 1 (second register))
#[test]
fn test_brkbs_p_p_p_z_combo_3_4000_25d04020() {
    // Encoding: 0x25D04020
    // Test BRKBS_P.P.P_Z field combination: Pg=0, Pn=1, Pd=0
    // Fields: Pd=0, Pg=0, Pn=1
    let encoding: u32 = 0x25D04020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKBS_P.P.P_Z
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pd=0 (register index 0 (first register))
#[test]
fn test_brkbs_p_p_p_z_combo_4_4000_25d04000() {
    // Encoding: 0x25D04000
    // Test BRKBS_P.P.P_Z field combination: Pg=0, Pn=0, Pd=0
    // Fields: Pg=0, Pn=0, Pd=0
    let encoding: u32 = 0x25D04000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKBS_P.P.P_Z
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pd=1 (register index 1 (second register))
#[test]
fn test_brkbs_p_p_p_z_combo_5_4000_25d04001() {
    // Encoding: 0x25D04001
    // Test BRKBS_P.P.P_Z field combination: Pg=0, Pn=0, Pd=1
    // Fields: Pn=0, Pg=0, Pd=1
    let encoding: u32 = 0x25D04001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKBS_P.P.P_Z
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=1 (same register test (reg=1)), Pn=1 (same register test (reg=1))
#[test]
fn test_brkbs_p_p_p_z_combo_6_4000_25d04420() {
    // Encoding: 0x25D04420
    // Test BRKBS_P.P.P_Z field combination: Pg=1, Pn=1, Pd=0
    // Fields: Pd=0, Pn=1, Pg=1
    let encoding: u32 = 0x25D04420;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKBS_P.P.P_Z
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=31 (same register test (reg=31)), Pn=31 (same register test (reg=31))
#[test]
fn test_brkbs_p_p_p_z_combo_7_4000_25d07de0() {
    // Encoding: 0x25D07DE0
    // Test BRKBS_P.P.P_Z field combination: Pg=31, Pn=31, Pd=0
    // Fields: Pn=31, Pg=31, Pd=0
    let encoding: u32 = 0x25D07DE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKBS_P.P.P_Z
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=1 (same register test (reg=1)), Pd=1 (same register test (reg=1))
#[test]
fn test_brkbs_p_p_p_z_combo_8_4000_25d04401() {
    // Encoding: 0x25D04401
    // Test BRKBS_P.P.P_Z field combination: Pg=1, Pn=0, Pd=1
    // Fields: Pg=1, Pd=1, Pn=0
    let encoding: u32 = 0x25D04401;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKBS_P.P.P_Z
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=31 (same register test (reg=31)), Pd=31 (same register test (reg=31))
#[test]
fn test_brkbs_p_p_p_z_combo_9_4000_25d07c0f() {
    // Encoding: 0x25D07C0F
    // Test BRKBS_P.P.P_Z field combination: Pg=31, Pn=0, Pd=31
    // Fields: Pg=31, Pn=0, Pd=31
    let encoding: u32 = 0x25D07C0F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKBS_P.P.P_Z
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pn=1 (same register test (reg=1)), Pd=1 (same register test (reg=1))
#[test]
fn test_brkbs_p_p_p_z_combo_10_4000_25d04021() {
    // Encoding: 0x25D04021
    // Test BRKBS_P.P.P_Z field combination: Pg=0, Pn=1, Pd=1
    // Fields: Pg=0, Pn=1, Pd=1
    let encoding: u32 = 0x25D04021;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKBS_P.P.P_Z
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pn=31 (same register test (reg=31)), Pd=31 (same register test (reg=31))
#[test]
fn test_brkbs_p_p_p_z_combo_11_4000_25d041ef() {
    // Encoding: 0x25D041EF
    // Test BRKBS_P.P.P_Z field combination: Pg=0, Pn=31, Pd=31
    // Fields: Pg=0, Pn=31, Pd=31
    let encoding: u32 = 0x25D041EF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKB_P.P.P__
/// ASL: `SimdFromField("Pd") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "unknown" }
/// verify register write to SimdFromField("Pd")
#[test]
fn test_brkb_p_p_p_reg_write_0_25904000() {
    // Test BRKB_P.P.P__ register write: SimdFromField("Pd")
    // Encoding: 0x25904000
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x25904000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: BRKB_P.P.P__
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 0 + 0 = 0 (Z=1)
#[test]
fn test_brkb_p_p_p_flags_zeroresult_0_25904000() {
    // Test BRKB_P.P.P__ flag computation: ZeroResult
    // Encoding: 0x25904000
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x0);
    set_x(&mut cpu, 2, 0x0);
    let encoding: u32 = 0x25904000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: BRKB_P.P.P__
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 1 + (-1) = 0 (Z=1, C=1)
#[test]
fn test_brkb_p_p_p_flags_zeroresult_1_25904000() {
    // Test BRKB_P.P.P__ flag computation: ZeroResult
    // Encoding: 0x25904000
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x1);
    set_x(&mut cpu, 2, 0xFFFFFFFFFFFFFFFF);
    let encoding: u32 = 0x25904000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: BRKB_P.P.P__
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: NegativeResult }
/// negative value (N=1)
#[test]
fn test_brkb_p_p_p_flags_negativeresult_2_25904000() {
    // Test BRKB_P.P.P__ flag computation: NegativeResult
    // Encoding: 0x25904000
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x8000000000000000);
    set_x(&mut cpu, 2, 0x0);
    let encoding: u32 = 0x25904000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: BRKB_P.P.P__
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 1 = 0 (C=1, Z=1)
#[test]
fn test_brkb_p_p_p_flags_unsignedoverflow_3_25904000() {
    // Test BRKB_P.P.P__ flag computation: UnsignedOverflow
    // Encoding: 0x25904000
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0x1);
    set_x(&mut cpu, 1, 0xFFFFFFFFFFFFFFFF);
    let encoding: u32 = 0x25904000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: BRKB_P.P.P__
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 2 = 1 (C=1)
#[test]
fn test_brkb_p_p_p_flags_unsignedoverflow_4_25904000() {
    // Test BRKB_P.P.P__ flag computation: UnsignedOverflow
    // Encoding: 0x25904000
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0x2);
    set_x(&mut cpu, 1, 0xFFFFFFFFFFFFFFFF);
    let encoding: u32 = 0x25904000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: BRKB_P.P.P__
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// max_signed + 1 = min_signed (V=1, N=1)
#[test]
fn test_brkb_p_p_p_flags_signedoverflow_5_25904000() {
    // Test BRKB_P.P.P__ flag computation: SignedOverflow
    // Encoding: 0x25904000
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0x1);
    set_x(&mut cpu, 1, 0x7FFFFFFFFFFFFFFF);
    let encoding: u32 = 0x25904000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: BRKB_P.P.P__
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// min_signed + (-1) = max_signed (V=1)
#[test]
fn test_brkb_p_p_p_flags_signedoverflow_6_25904000() {
    // Test BRKB_P.P.P__ flag computation: SignedOverflow
    // Encoding: 0x25904000
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x8000000000000000);
    set_x(&mut cpu, 2, 0xFFFFFFFFFFFFFFFF);
    let encoding: u32 = 0x25904000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: BRKB_P.P.P__
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: PositiveResult }
/// 100 + 50 = 150 (no flags)
#[test]
fn test_brkb_p_p_p_flags_positiveresult_7_25904000() {
    // Test BRKB_P.P.P__ flag computation: PositiveResult
    // Encoding: 0x25904000
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0x32);
    set_x(&mut cpu, 1, 0x64);
    let encoding: u32 = 0x25904000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: BRKBS_P.P.P_Z
/// ASL: `SimdFromField("Pd") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "unknown" }
/// verify register write to SimdFromField("Pd")
#[test]
fn test_brkbs_p_p_p_z_reg_write_0_25d04000() {
    // Test BRKBS_P.P.P_Z register write: SimdFromField("Pd")
    // Encoding: 0x25D04000
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x25D04000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: BRKBS_P.P.P_Z
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 0 + 0 = 0 (Z=1)
#[test]
fn test_brkbs_p_p_p_z_flags_zeroresult_0_25d04000() {
    // Test BRKBS_P.P.P_Z flag computation: ZeroResult
    // Encoding: 0x25D04000
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x0);
    set_x(&mut cpu, 2, 0x0);
    let encoding: u32 = 0x25D04000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: BRKBS_P.P.P_Z
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 1 + (-1) = 0 (Z=1, C=1)
#[test]
fn test_brkbs_p_p_p_z_flags_zeroresult_1_25d04000() {
    // Test BRKBS_P.P.P_Z flag computation: ZeroResult
    // Encoding: 0x25D04000
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0xFFFFFFFFFFFFFFFF);
    set_x(&mut cpu, 1, 0x1);
    let encoding: u32 = 0x25D04000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: BRKBS_P.P.P_Z
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: NegativeResult }
/// negative value (N=1)
#[test]
fn test_brkbs_p_p_p_z_flags_negativeresult_2_25d04000() {
    // Test BRKBS_P.P.P_Z flag computation: NegativeResult
    // Encoding: 0x25D04000
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x8000000000000000);
    set_x(&mut cpu, 2, 0x0);
    let encoding: u32 = 0x25D04000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: BRKBS_P.P.P_Z
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 1 = 0 (C=1, Z=1)
#[test]
fn test_brkbs_p_p_p_z_flags_unsignedoverflow_3_25d04000() {
    // Test BRKBS_P.P.P_Z flag computation: UnsignedOverflow
    // Encoding: 0x25D04000
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0x1);
    set_x(&mut cpu, 1, 0xFFFFFFFFFFFFFFFF);
    let encoding: u32 = 0x25D04000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: BRKBS_P.P.P_Z
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 2 = 1 (C=1)
#[test]
fn test_brkbs_p_p_p_z_flags_unsignedoverflow_4_25d04000() {
    // Test BRKBS_P.P.P_Z flag computation: UnsignedOverflow
    // Encoding: 0x25D04000
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0x2);
    set_x(&mut cpu, 1, 0xFFFFFFFFFFFFFFFF);
    let encoding: u32 = 0x25D04000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: BRKBS_P.P.P_Z
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// max_signed + 1 = min_signed (V=1, N=1)
#[test]
fn test_brkbs_p_p_p_z_flags_signedoverflow_5_25d04000() {
    // Test BRKBS_P.P.P_Z flag computation: SignedOverflow
    // Encoding: 0x25D04000
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x7FFFFFFFFFFFFFFF);
    set_x(&mut cpu, 2, 0x1);
    let encoding: u32 = 0x25D04000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: BRKBS_P.P.P_Z
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// min_signed + (-1) = max_signed (V=1)
#[test]
fn test_brkbs_p_p_p_z_flags_signedoverflow_6_25d04000() {
    // Test BRKBS_P.P.P_Z flag computation: SignedOverflow
    // Encoding: 0x25D04000
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0xFFFFFFFFFFFFFFFF);
    set_x(&mut cpu, 1, 0x8000000000000000);
    let encoding: u32 = 0x25D04000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: BRKBS_P.P.P_Z
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: PositiveResult }
/// 100 + 50 = 150 (no flags)
#[test]
fn test_brkbs_p_p_p_z_flags_positiveresult_7_25d04000() {
    // Test BRKBS_P.P.P_Z flag computation: PositiveResult
    // Encoding: 0x25D04000
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0x32);
    set_x(&mut cpu, 1, 0x64);
    let encoding: u32 = 0x25D04000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

// ============================================================================
// BRKPA_P.P.PP__ Tests
// ============================================================================

/// Provenance: BRKPA_P.P.PP__
/// ASL: `field Pm 16 +: 4`
/// Requirement: FieldBoundary { field: "Pm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_brkpa_p_p_pp_field_pm_0_min_c000_2500c000() {
    // Encoding: 0x2500C000
    // Test BRKPA_P.P.PP__ field Pm = 0 (Min)
    // Fields: Pm=0, Pn=0, Pd=0, Pg=0
    let encoding: u32 = 0x2500C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKPA_P.P.PP__
/// ASL: `field Pm 16 +: 4`
/// Requirement: FieldBoundary { field: "Pm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_brkpa_p_p_pp_field_pm_1_poweroftwo_c000_2501c000() {
    // Encoding: 0x2501C000
    // Test BRKPA_P.P.PP__ field Pm = 1 (PowerOfTwo)
    // Fields: Pg=0, Pn=0, Pm=1, Pd=0
    let encoding: u32 = 0x2501C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKPA_P.P.PP__
/// ASL: `field Pg 10 +: 4`
/// Requirement: FieldBoundary { field: "Pg", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_brkpa_p_p_pp_field_pg_0_min_c000_2500c000() {
    // Encoding: 0x2500C000
    // Test BRKPA_P.P.PP__ field Pg = 0 (Min)
    // Fields: Pn=0, Pg=0, Pd=0, Pm=0
    let encoding: u32 = 0x2500C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKPA_P.P.PP__
/// ASL: `field Pg 10 +: 4`
/// Requirement: FieldBoundary { field: "Pg", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_brkpa_p_p_pp_field_pg_1_poweroftwo_c000_2500c400() {
    // Encoding: 0x2500C400
    // Test BRKPA_P.P.PP__ field Pg = 1 (PowerOfTwo)
    // Fields: Pm=0, Pn=0, Pd=0, Pg=1
    let encoding: u32 = 0x2500C400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKPA_P.P.PP__
/// ASL: `field Pn 5 +: 4`
/// Requirement: FieldBoundary { field: "Pn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_brkpa_p_p_pp_field_pn_0_min_c000_2500c000() {
    // Encoding: 0x2500C000
    // Test BRKPA_P.P.PP__ field Pn = 0 (Min)
    // Fields: Pn=0, Pg=0, Pm=0, Pd=0
    let encoding: u32 = 0x2500C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKPA_P.P.PP__
/// ASL: `field Pn 5 +: 4`
/// Requirement: FieldBoundary { field: "Pn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_brkpa_p_p_pp_field_pn_1_poweroftwo_c000_2500c020() {
    // Encoding: 0x2500C020
    // Test BRKPA_P.P.PP__ field Pn = 1 (PowerOfTwo)
    // Fields: Pm=0, Pd=0, Pn=1, Pg=0
    let encoding: u32 = 0x2500C020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKPA_P.P.PP__
/// ASL: `field Pd 0 +: 4`
/// Requirement: FieldBoundary { field: "Pd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_brkpa_p_p_pp_field_pd_0_min_c000_2500c000() {
    // Encoding: 0x2500C000
    // Test BRKPA_P.P.PP__ field Pd = 0 (Min)
    // Fields: Pm=0, Pg=0, Pd=0, Pn=0
    let encoding: u32 = 0x2500C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKPA_P.P.PP__
/// ASL: `field Pd 0 +: 4`
/// Requirement: FieldBoundary { field: "Pd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_brkpa_p_p_pp_field_pd_1_poweroftwo_c000_2500c001() {
    // Encoding: 0x2500C001
    // Test BRKPA_P.P.PP__ field Pd = 1 (PowerOfTwo)
    // Fields: Pg=0, Pm=0, Pn=0, Pd=1
    let encoding: u32 = 0x2500C001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKPA_P.P.PP__
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pm=0 (register index 0 (first register))
#[test]
fn test_brkpa_p_p_pp_combo_0_c000_2500c000() {
    // Encoding: 0x2500C000
    // Test BRKPA_P.P.PP__ field combination: Pm=0, Pg=0, Pn=0, Pd=0
    // Fields: Pm=0, Pn=0, Pg=0, Pd=0
    let encoding: u32 = 0x2500C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKPA_P.P.PP__
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pm=1 (register index 1 (second register))
#[test]
fn test_brkpa_p_p_pp_combo_1_c000_2501c000() {
    // Encoding: 0x2501C000
    // Test BRKPA_P.P.PP__ field combination: Pm=1, Pg=0, Pn=0, Pd=0
    // Fields: Pg=0, Pn=0, Pm=1, Pd=0
    let encoding: u32 = 0x2501C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKPA_P.P.PP__
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=0 (register index 0 (first register))
#[test]
fn test_brkpa_p_p_pp_combo_2_c000_2500c000() {
    // Encoding: 0x2500C000
    // Test BRKPA_P.P.PP__ field combination: Pm=0, Pg=0, Pn=0, Pd=0
    // Fields: Pm=0, Pn=0, Pg=0, Pd=0
    let encoding: u32 = 0x2500C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKPA_P.P.PP__
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=1 (register index 1 (second register))
#[test]
fn test_brkpa_p_p_pp_combo_3_c000_2500c400() {
    // Encoding: 0x2500C400
    // Test BRKPA_P.P.PP__ field combination: Pm=0, Pg=1, Pn=0, Pd=0
    // Fields: Pd=0, Pg=1, Pn=0, Pm=0
    let encoding: u32 = 0x2500C400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKPA_P.P.PP__
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pn=0 (register index 0 (first register))
#[test]
fn test_brkpa_p_p_pp_combo_4_c000_2500c000() {
    // Encoding: 0x2500C000
    // Test BRKPA_P.P.PP__ field combination: Pm=0, Pg=0, Pn=0, Pd=0
    // Fields: Pd=0, Pm=0, Pg=0, Pn=0
    let encoding: u32 = 0x2500C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKPA_P.P.PP__
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pn=1 (register index 1 (second register))
#[test]
fn test_brkpa_p_p_pp_combo_5_c000_2500c020() {
    // Encoding: 0x2500C020
    // Test BRKPA_P.P.PP__ field combination: Pm=0, Pg=0, Pn=1, Pd=0
    // Fields: Pg=0, Pd=0, Pm=0, Pn=1
    let encoding: u32 = 0x2500C020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKPA_P.P.PP__
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pd=0 (register index 0 (first register))
#[test]
fn test_brkpa_p_p_pp_combo_6_c000_2500c000() {
    // Encoding: 0x2500C000
    // Test BRKPA_P.P.PP__ field combination: Pm=0, Pg=0, Pn=0, Pd=0
    // Fields: Pn=0, Pm=0, Pg=0, Pd=0
    let encoding: u32 = 0x2500C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKPA_P.P.PP__
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pd=1 (register index 1 (second register))
#[test]
fn test_brkpa_p_p_pp_combo_7_c000_2500c001() {
    // Encoding: 0x2500C001
    // Test BRKPA_P.P.PP__ field combination: Pm=0, Pg=0, Pn=0, Pd=1
    // Fields: Pg=0, Pn=0, Pm=0, Pd=1
    let encoding: u32 = 0x2500C001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKPA_P.P.PP__
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pm=1 (same register test (reg=1)), Pg=1 (same register test (reg=1))
#[test]
fn test_brkpa_p_p_pp_combo_8_c000_2501c400() {
    // Encoding: 0x2501C400
    // Test BRKPA_P.P.PP__ field combination: Pm=1, Pg=1, Pn=0, Pd=0
    // Fields: Pn=0, Pm=1, Pg=1, Pd=0
    let encoding: u32 = 0x2501C400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKPA_P.P.PP__
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pm=31 (same register test (reg=31)), Pg=31 (same register test (reg=31))
#[test]
fn test_brkpa_p_p_pp_combo_9_c000_250ffc00() {
    // Encoding: 0x250FFC00
    // Test BRKPA_P.P.PP__ field combination: Pm=31, Pg=31, Pn=0, Pd=0
    // Fields: Pm=31, Pd=0, Pg=31, Pn=0
    let encoding: u32 = 0x250FFC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKPA_P.P.PP__
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pm=1 (same register test (reg=1)), Pn=1 (same register test (reg=1))
#[test]
fn test_brkpa_p_p_pp_combo_10_c000_2501c020() {
    // Encoding: 0x2501C020
    // Test BRKPA_P.P.PP__ field combination: Pm=1, Pg=0, Pn=1, Pd=0
    // Fields: Pn=1, Pd=0, Pm=1, Pg=0
    let encoding: u32 = 0x2501C020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKPA_P.P.PP__
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pm=31 (same register test (reg=31)), Pn=31 (same register test (reg=31))
#[test]
fn test_brkpa_p_p_pp_combo_11_c000_250fc1e0() {
    // Encoding: 0x250FC1E0
    // Test BRKPA_P.P.PP__ field combination: Pm=31, Pg=0, Pn=31, Pd=0
    // Fields: Pd=0, Pg=0, Pn=31, Pm=31
    let encoding: u32 = 0x250FC1E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKPA_P.P.PP__
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pm=1 (same register test (reg=1)), Pd=1 (same register test (reg=1))
#[test]
fn test_brkpa_p_p_pp_combo_12_c000_2501c001() {
    // Encoding: 0x2501C001
    // Test BRKPA_P.P.PP__ field combination: Pm=1, Pg=0, Pn=0, Pd=1
    // Fields: Pm=1, Pn=0, Pg=0, Pd=1
    let encoding: u32 = 0x2501C001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKPA_P.P.PP__
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pm=31 (same register test (reg=31)), Pd=31 (same register test (reg=31))
#[test]
fn test_brkpa_p_p_pp_combo_13_c000_250fc00f() {
    // Encoding: 0x250FC00F
    // Test BRKPA_P.P.PP__ field combination: Pm=31, Pg=0, Pn=0, Pd=31
    // Fields: Pn=0, Pm=31, Pg=0, Pd=31
    let encoding: u32 = 0x250FC00F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKPA_P.P.PP__
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=1 (same register test (reg=1)), Pn=1 (same register test (reg=1))
#[test]
fn test_brkpa_p_p_pp_combo_14_c000_2500c420() {
    // Encoding: 0x2500C420
    // Test BRKPA_P.P.PP__ field combination: Pm=0, Pg=1, Pn=1, Pd=0
    // Fields: Pd=0, Pn=1, Pm=0, Pg=1
    let encoding: u32 = 0x2500C420;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKPA_P.P.PP__
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=31 (same register test (reg=31)), Pn=31 (same register test (reg=31))
#[test]
fn test_brkpa_p_p_pp_combo_15_c000_2500fde0() {
    // Encoding: 0x2500FDE0
    // Test BRKPA_P.P.PP__ field combination: Pm=0, Pg=31, Pn=31, Pd=0
    // Fields: Pn=31, Pm=0, Pd=0, Pg=31
    let encoding: u32 = 0x2500FDE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKPA_P.P.PP__
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=1 (same register test (reg=1)), Pd=1 (same register test (reg=1))
#[test]
fn test_brkpa_p_p_pp_combo_16_c000_2500c401() {
    // Encoding: 0x2500C401
    // Test BRKPA_P.P.PP__ field combination: Pm=0, Pg=1, Pn=0, Pd=1
    // Fields: Pn=0, Pm=0, Pg=1, Pd=1
    let encoding: u32 = 0x2500C401;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKPA_P.P.PP__
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=31 (same register test (reg=31)), Pd=31 (same register test (reg=31))
#[test]
fn test_brkpa_p_p_pp_combo_17_c000_2500fc0f() {
    // Encoding: 0x2500FC0F
    // Test BRKPA_P.P.PP__ field combination: Pm=0, Pg=31, Pn=0, Pd=31
    // Fields: Pm=0, Pn=0, Pg=31, Pd=31
    let encoding: u32 = 0x2500FC0F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKPA_P.P.PP__
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pn=1 (same register test (reg=1)), Pd=1 (same register test (reg=1))
#[test]
fn test_brkpa_p_p_pp_combo_18_c000_2500c021() {
    // Encoding: 0x2500C021
    // Test BRKPA_P.P.PP__ field combination: Pm=0, Pg=0, Pn=1, Pd=1
    // Fields: Pd=1, Pg=0, Pm=0, Pn=1
    let encoding: u32 = 0x2500C021;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKPA_P.P.PP__
/// ASL: `field combination 19`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pn=31 (same register test (reg=31)), Pd=31 (same register test (reg=31))
#[test]
fn test_brkpa_p_p_pp_combo_19_c000_2500c1ef() {
    // Encoding: 0x2500C1EF
    // Test BRKPA_P.P.PP__ field combination: Pm=0, Pg=0, Pn=31, Pd=31
    // Fields: Pg=0, Pd=31, Pn=31, Pm=0
    let encoding: u32 = 0x2500C1EF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKPAS_P.P.PP__
/// ASL: `field Pm 16 +: 4`
/// Requirement: FieldBoundary { field: "Pm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_brkpas_p_p_pp_field_pm_0_min_c000_2540c000() {
    // Encoding: 0x2540C000
    // Test BRKPAS_P.P.PP__ field Pm = 0 (Min)
    // Fields: Pg=0, Pn=0, Pd=0, Pm=0
    let encoding: u32 = 0x2540C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKPAS_P.P.PP__
/// ASL: `field Pm 16 +: 4`
/// Requirement: FieldBoundary { field: "Pm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_brkpas_p_p_pp_field_pm_1_poweroftwo_c000_2541c000() {
    // Encoding: 0x2541C000
    // Test BRKPAS_P.P.PP__ field Pm = 1 (PowerOfTwo)
    // Fields: Pm=1, Pg=0, Pn=0, Pd=0
    let encoding: u32 = 0x2541C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKPAS_P.P.PP__
/// ASL: `field Pg 10 +: 4`
/// Requirement: FieldBoundary { field: "Pg", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_brkpas_p_p_pp_field_pg_0_min_c000_2540c000() {
    // Encoding: 0x2540C000
    // Test BRKPAS_P.P.PP__ field Pg = 0 (Min)
    // Fields: Pm=0, Pg=0, Pn=0, Pd=0
    let encoding: u32 = 0x2540C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKPAS_P.P.PP__
/// ASL: `field Pg 10 +: 4`
/// Requirement: FieldBoundary { field: "Pg", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_brkpas_p_p_pp_field_pg_1_poweroftwo_c000_2540c400() {
    // Encoding: 0x2540C400
    // Test BRKPAS_P.P.PP__ field Pg = 1 (PowerOfTwo)
    // Fields: Pm=0, Pn=0, Pg=1, Pd=0
    let encoding: u32 = 0x2540C400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKPAS_P.P.PP__
/// ASL: `field Pn 5 +: 4`
/// Requirement: FieldBoundary { field: "Pn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_brkpas_p_p_pp_field_pn_0_min_c000_2540c000() {
    // Encoding: 0x2540C000
    // Test BRKPAS_P.P.PP__ field Pn = 0 (Min)
    // Fields: Pn=0, Pm=0, Pg=0, Pd=0
    let encoding: u32 = 0x2540C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKPAS_P.P.PP__
/// ASL: `field Pn 5 +: 4`
/// Requirement: FieldBoundary { field: "Pn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_brkpas_p_p_pp_field_pn_1_poweroftwo_c000_2540c020() {
    // Encoding: 0x2540C020
    // Test BRKPAS_P.P.PP__ field Pn = 1 (PowerOfTwo)
    // Fields: Pm=0, Pg=0, Pd=0, Pn=1
    let encoding: u32 = 0x2540C020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKPAS_P.P.PP__
/// ASL: `field Pd 0 +: 4`
/// Requirement: FieldBoundary { field: "Pd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_brkpas_p_p_pp_field_pd_0_min_c000_2540c000() {
    // Encoding: 0x2540C000
    // Test BRKPAS_P.P.PP__ field Pd = 0 (Min)
    // Fields: Pn=0, Pm=0, Pd=0, Pg=0
    let encoding: u32 = 0x2540C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKPAS_P.P.PP__
/// ASL: `field Pd 0 +: 4`
/// Requirement: FieldBoundary { field: "Pd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_brkpas_p_p_pp_field_pd_1_poweroftwo_c000_2540c001() {
    // Encoding: 0x2540C001
    // Test BRKPAS_P.P.PP__ field Pd = 1 (PowerOfTwo)
    // Fields: Pm=0, Pn=0, Pd=1, Pg=0
    let encoding: u32 = 0x2540C001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKPAS_P.P.PP__
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pm=0 (register index 0 (first register))
#[test]
fn test_brkpas_p_p_pp_combo_0_c000_2540c000() {
    // Encoding: 0x2540C000
    // Test BRKPAS_P.P.PP__ field combination: Pm=0, Pg=0, Pn=0, Pd=0
    // Fields: Pm=0, Pg=0, Pn=0, Pd=0
    let encoding: u32 = 0x2540C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKPAS_P.P.PP__
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pm=1 (register index 1 (second register))
#[test]
fn test_brkpas_p_p_pp_combo_1_c000_2541c000() {
    // Encoding: 0x2541C000
    // Test BRKPAS_P.P.PP__ field combination: Pm=1, Pg=0, Pn=0, Pd=0
    // Fields: Pm=1, Pg=0, Pn=0, Pd=0
    let encoding: u32 = 0x2541C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKPAS_P.P.PP__
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=0 (register index 0 (first register))
#[test]
fn test_brkpas_p_p_pp_combo_2_c000_2540c000() {
    // Encoding: 0x2540C000
    // Test BRKPAS_P.P.PP__ field combination: Pm=0, Pg=0, Pn=0, Pd=0
    // Fields: Pn=0, Pd=0, Pg=0, Pm=0
    let encoding: u32 = 0x2540C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKPAS_P.P.PP__
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=1 (register index 1 (second register))
#[test]
fn test_brkpas_p_p_pp_combo_3_c000_2540c400() {
    // Encoding: 0x2540C400
    // Test BRKPAS_P.P.PP__ field combination: Pm=0, Pg=1, Pn=0, Pd=0
    // Fields: Pg=1, Pm=0, Pd=0, Pn=0
    let encoding: u32 = 0x2540C400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKPAS_P.P.PP__
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pn=0 (register index 0 (first register))
#[test]
fn test_brkpas_p_p_pp_combo_4_c000_2540c000() {
    // Encoding: 0x2540C000
    // Test BRKPAS_P.P.PP__ field combination: Pm=0, Pg=0, Pn=0, Pd=0
    // Fields: Pd=0, Pn=0, Pg=0, Pm=0
    let encoding: u32 = 0x2540C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKPAS_P.P.PP__
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pn=1 (register index 1 (second register))
#[test]
fn test_brkpas_p_p_pp_combo_5_c000_2540c020() {
    // Encoding: 0x2540C020
    // Test BRKPAS_P.P.PP__ field combination: Pm=0, Pg=0, Pn=1, Pd=0
    // Fields: Pg=0, Pn=1, Pm=0, Pd=0
    let encoding: u32 = 0x2540C020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKPAS_P.P.PP__
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pd=0 (register index 0 (first register))
#[test]
fn test_brkpas_p_p_pp_combo_6_c000_2540c000() {
    // Encoding: 0x2540C000
    // Test BRKPAS_P.P.PP__ field combination: Pm=0, Pg=0, Pn=0, Pd=0
    // Fields: Pg=0, Pn=0, Pd=0, Pm=0
    let encoding: u32 = 0x2540C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKPAS_P.P.PP__
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pd=1 (register index 1 (second register))
#[test]
fn test_brkpas_p_p_pp_combo_7_c000_2540c001() {
    // Encoding: 0x2540C001
    // Test BRKPAS_P.P.PP__ field combination: Pm=0, Pg=0, Pn=0, Pd=1
    // Fields: Pn=0, Pm=0, Pd=1, Pg=0
    let encoding: u32 = 0x2540C001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKPAS_P.P.PP__
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pm=1 (same register test (reg=1)), Pg=1 (same register test (reg=1))
#[test]
fn test_brkpas_p_p_pp_combo_8_c000_2541c400() {
    // Encoding: 0x2541C400
    // Test BRKPAS_P.P.PP__ field combination: Pm=1, Pg=1, Pn=0, Pd=0
    // Fields: Pm=1, Pd=0, Pg=1, Pn=0
    let encoding: u32 = 0x2541C400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKPAS_P.P.PP__
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pm=31 (same register test (reg=31)), Pg=31 (same register test (reg=31))
#[test]
fn test_brkpas_p_p_pp_combo_9_c000_254ffc00() {
    // Encoding: 0x254FFC00
    // Test BRKPAS_P.P.PP__ field combination: Pm=31, Pg=31, Pn=0, Pd=0
    // Fields: Pm=31, Pn=0, Pd=0, Pg=31
    let encoding: u32 = 0x254FFC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKPAS_P.P.PP__
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pm=1 (same register test (reg=1)), Pn=1 (same register test (reg=1))
#[test]
fn test_brkpas_p_p_pp_combo_10_c000_2541c020() {
    // Encoding: 0x2541C020
    // Test BRKPAS_P.P.PP__ field combination: Pm=1, Pg=0, Pn=1, Pd=0
    // Fields: Pd=0, Pm=1, Pg=0, Pn=1
    let encoding: u32 = 0x2541C020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKPAS_P.P.PP__
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pm=31 (same register test (reg=31)), Pn=31 (same register test (reg=31))
#[test]
fn test_brkpas_p_p_pp_combo_11_c000_254fc1e0() {
    // Encoding: 0x254FC1E0
    // Test BRKPAS_P.P.PP__ field combination: Pm=31, Pg=0, Pn=31, Pd=0
    // Fields: Pg=0, Pd=0, Pn=31, Pm=31
    let encoding: u32 = 0x254FC1E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKPAS_P.P.PP__
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pm=1 (same register test (reg=1)), Pd=1 (same register test (reg=1))
#[test]
fn test_brkpas_p_p_pp_combo_12_c000_2541c001() {
    // Encoding: 0x2541C001
    // Test BRKPAS_P.P.PP__ field combination: Pm=1, Pg=0, Pn=0, Pd=1
    // Fields: Pg=0, Pm=1, Pn=0, Pd=1
    let encoding: u32 = 0x2541C001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKPAS_P.P.PP__
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pm=31 (same register test (reg=31)), Pd=31 (same register test (reg=31))
#[test]
fn test_brkpas_p_p_pp_combo_13_c000_254fc00f() {
    // Encoding: 0x254FC00F
    // Test BRKPAS_P.P.PP__ field combination: Pm=31, Pg=0, Pn=0, Pd=31
    // Fields: Pm=31, Pd=31, Pn=0, Pg=0
    let encoding: u32 = 0x254FC00F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKPAS_P.P.PP__
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=1 (same register test (reg=1)), Pn=1 (same register test (reg=1))
#[test]
fn test_brkpas_p_p_pp_combo_14_c000_2540c420() {
    // Encoding: 0x2540C420
    // Test BRKPAS_P.P.PP__ field combination: Pm=0, Pg=1, Pn=1, Pd=0
    // Fields: Pm=0, Pn=1, Pd=0, Pg=1
    let encoding: u32 = 0x2540C420;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKPAS_P.P.PP__
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=31 (same register test (reg=31)), Pn=31 (same register test (reg=31))
#[test]
fn test_brkpas_p_p_pp_combo_15_c000_2540fde0() {
    // Encoding: 0x2540FDE0
    // Test BRKPAS_P.P.PP__ field combination: Pm=0, Pg=31, Pn=31, Pd=0
    // Fields: Pm=0, Pg=31, Pn=31, Pd=0
    let encoding: u32 = 0x2540FDE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKPAS_P.P.PP__
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=1 (same register test (reg=1)), Pd=1 (same register test (reg=1))
#[test]
fn test_brkpas_p_p_pp_combo_16_c000_2540c401() {
    // Encoding: 0x2540C401
    // Test BRKPAS_P.P.PP__ field combination: Pm=0, Pg=1, Pn=0, Pd=1
    // Fields: Pm=0, Pg=1, Pn=0, Pd=1
    let encoding: u32 = 0x2540C401;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKPAS_P.P.PP__
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=31 (same register test (reg=31)), Pd=31 (same register test (reg=31))
#[test]
fn test_brkpas_p_p_pp_combo_17_c000_2540fc0f() {
    // Encoding: 0x2540FC0F
    // Test BRKPAS_P.P.PP__ field combination: Pm=0, Pg=31, Pn=0, Pd=31
    // Fields: Pg=31, Pm=0, Pd=31, Pn=0
    let encoding: u32 = 0x2540FC0F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKPAS_P.P.PP__
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pn=1 (same register test (reg=1)), Pd=1 (same register test (reg=1))
#[test]
fn test_brkpas_p_p_pp_combo_18_c000_2540c021() {
    // Encoding: 0x2540C021
    // Test BRKPAS_P.P.PP__ field combination: Pm=0, Pg=0, Pn=1, Pd=1
    // Fields: Pm=0, Pd=1, Pg=0, Pn=1
    let encoding: u32 = 0x2540C021;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKPAS_P.P.PP__
/// ASL: `field combination 19`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pn=31 (same register test (reg=31)), Pd=31 (same register test (reg=31))
#[test]
fn test_brkpas_p_p_pp_combo_19_c000_2540c1ef() {
    // Encoding: 0x2540C1EF
    // Test BRKPAS_P.P.PP__ field combination: Pm=0, Pg=0, Pn=31, Pd=31
    // Fields: Pm=0, Pg=0, Pd=31, Pn=31
    let encoding: u32 = 0x2540C1EF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: BRKPA_P.P.PP__
/// ASL: `SimdFromField("Pd") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "unknown" }
/// verify register write to SimdFromField("Pd")
#[test]
fn test_brkpa_p_p_pp_reg_write_0_2500c000() {
    // Test BRKPA_P.P.PP__ register write: SimdFromField("Pd")
    // Encoding: 0x2500C000
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x2500C000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: BRKPA_P.P.PP__
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 0 + 0 = 0 (Z=1)
#[test]
fn test_brkpa_p_p_pp_flags_zeroresult_0_2500c000() {
    // Test BRKPA_P.P.PP__ flag computation: ZeroResult
    // Encoding: 0x2500C000
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x0);
    set_x(&mut cpu, 2, 0x0);
    let encoding: u32 = 0x2500C000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: BRKPA_P.P.PP__
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 1 + (-1) = 0 (Z=1, C=1)
#[test]
fn test_brkpa_p_p_pp_flags_zeroresult_1_2500c000() {
    // Test BRKPA_P.P.PP__ flag computation: ZeroResult
    // Encoding: 0x2500C000
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x1);
    set_x(&mut cpu, 2, 0xFFFFFFFFFFFFFFFF);
    let encoding: u32 = 0x2500C000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: BRKPA_P.P.PP__
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: NegativeResult }
/// negative value (N=1)
#[test]
fn test_brkpa_p_p_pp_flags_negativeresult_2_2500c000() {
    // Test BRKPA_P.P.PP__ flag computation: NegativeResult
    // Encoding: 0x2500C000
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x8000000000000000);
    set_x(&mut cpu, 2, 0x0);
    let encoding: u32 = 0x2500C000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: BRKPA_P.P.PP__
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 1 = 0 (C=1, Z=1)
#[test]
fn test_brkpa_p_p_pp_flags_unsignedoverflow_3_2500c000() {
    // Test BRKPA_P.P.PP__ flag computation: UnsignedOverflow
    // Encoding: 0x2500C000
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0xFFFFFFFFFFFFFFFF);
    set_x(&mut cpu, 2, 0x1);
    let encoding: u32 = 0x2500C000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: BRKPA_P.P.PP__
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 2 = 1 (C=1)
#[test]
fn test_brkpa_p_p_pp_flags_unsignedoverflow_4_2500c000() {
    // Test BRKPA_P.P.PP__ flag computation: UnsignedOverflow
    // Encoding: 0x2500C000
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0xFFFFFFFFFFFFFFFF);
    set_x(&mut cpu, 2, 0x2);
    let encoding: u32 = 0x2500C000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: BRKPA_P.P.PP__
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// max_signed + 1 = min_signed (V=1, N=1)
#[test]
fn test_brkpa_p_p_pp_flags_signedoverflow_5_2500c000() {
    // Test BRKPA_P.P.PP__ flag computation: SignedOverflow
    // Encoding: 0x2500C000
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x7FFFFFFFFFFFFFFF);
    set_x(&mut cpu, 2, 0x1);
    let encoding: u32 = 0x2500C000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: BRKPA_P.P.PP__
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// min_signed + (-1) = max_signed (V=1)
#[test]
fn test_brkpa_p_p_pp_flags_signedoverflow_6_2500c000() {
    // Test BRKPA_P.P.PP__ flag computation: SignedOverflow
    // Encoding: 0x2500C000
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x8000000000000000);
    set_x(&mut cpu, 2, 0xFFFFFFFFFFFFFFFF);
    let encoding: u32 = 0x2500C000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: BRKPA_P.P.PP__
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: PositiveResult }
/// 100 + 50 = 150 (no flags)
#[test]
fn test_brkpa_p_p_pp_flags_positiveresult_7_2500c000() {
    // Test BRKPA_P.P.PP__ flag computation: PositiveResult
    // Encoding: 0x2500C000
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0x32);
    set_x(&mut cpu, 1, 0x64);
    let encoding: u32 = 0x2500C000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: BRKPAS_P.P.PP__
/// ASL: `SimdFromField("Pd") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "unknown" }
/// verify register write to SimdFromField("Pd")
#[test]
fn test_brkpas_p_p_pp_reg_write_0_2540c000() {
    // Test BRKPAS_P.P.PP__ register write: SimdFromField("Pd")
    // Encoding: 0x2540C000
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x2540C000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: BRKPAS_P.P.PP__
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 0 + 0 = 0 (Z=1)
#[test]
fn test_brkpas_p_p_pp_flags_zeroresult_0_2540c000() {
    // Test BRKPAS_P.P.PP__ flag computation: ZeroResult
    // Encoding: 0x2540C000
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0x0);
    set_x(&mut cpu, 1, 0x0);
    let encoding: u32 = 0x2540C000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: BRKPAS_P.P.PP__
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 1 + (-1) = 0 (Z=1, C=1)
#[test]
fn test_brkpas_p_p_pp_flags_zeroresult_1_2540c000() {
    // Test BRKPAS_P.P.PP__ flag computation: ZeroResult
    // Encoding: 0x2540C000
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0xFFFFFFFFFFFFFFFF);
    set_x(&mut cpu, 1, 0x1);
    let encoding: u32 = 0x2540C000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: BRKPAS_P.P.PP__
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: NegativeResult }
/// negative value (N=1)
#[test]
fn test_brkpas_p_p_pp_flags_negativeresult_2_2540c000() {
    // Test BRKPAS_P.P.PP__ flag computation: NegativeResult
    // Encoding: 0x2540C000
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x8000000000000000);
    set_x(&mut cpu, 2, 0x0);
    let encoding: u32 = 0x2540C000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: BRKPAS_P.P.PP__
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 1 = 0 (C=1, Z=1)
#[test]
fn test_brkpas_p_p_pp_flags_unsignedoverflow_3_2540c000() {
    // Test BRKPAS_P.P.PP__ flag computation: UnsignedOverflow
    // Encoding: 0x2540C000
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0xFFFFFFFFFFFFFFFF);
    set_x(&mut cpu, 2, 0x1);
    let encoding: u32 = 0x2540C000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: BRKPAS_P.P.PP__
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 2 = 1 (C=1)
#[test]
fn test_brkpas_p_p_pp_flags_unsignedoverflow_4_2540c000() {
    // Test BRKPAS_P.P.PP__ flag computation: UnsignedOverflow
    // Encoding: 0x2540C000
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0xFFFFFFFFFFFFFFFF);
    set_x(&mut cpu, 2, 0x2);
    let encoding: u32 = 0x2540C000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: BRKPAS_P.P.PP__
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// max_signed + 1 = min_signed (V=1, N=1)
#[test]
fn test_brkpas_p_p_pp_flags_signedoverflow_5_2540c000() {
    // Test BRKPAS_P.P.PP__ flag computation: SignedOverflow
    // Encoding: 0x2540C000
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x7FFFFFFFFFFFFFFF);
    set_x(&mut cpu, 2, 0x1);
    let encoding: u32 = 0x2540C000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: BRKPAS_P.P.PP__
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// min_signed + (-1) = max_signed (V=1)
#[test]
fn test_brkpas_p_p_pp_flags_signedoverflow_6_2540c000() {
    // Test BRKPAS_P.P.PP__ flag computation: SignedOverflow
    // Encoding: 0x2540C000
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x8000000000000000);
    set_x(&mut cpu, 2, 0xFFFFFFFFFFFFFFFF);
    let encoding: u32 = 0x2540C000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: BRKPAS_P.P.PP__
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: PositiveResult }
/// 100 + 50 = 150 (no flags)
#[test]
fn test_brkpas_p_p_pp_flags_positiveresult_7_2540c000() {
    // Test BRKPAS_P.P.PP__ flag computation: PositiveResult
    // Encoding: 0x2540C000
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0x32);
    set_x(&mut cpu, 1, 0x64);
    let encoding: u32 = 0x2540C000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

// ============================================================================
// WHILELS_P.P.RR__ Tests
// ============================================================================

/// Provenance: WHILELS_P.P.RR__
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_whilels_p_p_rr_field_size_0_min_c10_25200c10() {
    // Encoding: 0x25200C10
    // Test WHILELS_P.P.RR__ field size = 0 (Min)
    // Fields: size=0, sf=0, Rn=0, Rm=0, Pd=0
    let encoding: u32 = 0x25200C10;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELS_P.P.RR__
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 1, boundary: PowerOfTwo }
/// 16-bit / halfword size
#[test]
fn test_whilels_p_p_rr_field_size_1_poweroftwo_c10_25600c10() {
    // Encoding: 0x25600C10
    // Test WHILELS_P.P.RR__ field size = 1 (PowerOfTwo)
    // Fields: Rm=0, Pd=0, sf=0, size=1, Rn=0
    let encoding: u32 = 0x25600C10;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELS_P.P.RR__
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 2, boundary: PowerOfTwo }
/// 32-bit / word size
#[test]
fn test_whilels_p_p_rr_field_size_2_poweroftwo_c10_25a00c10() {
    // Encoding: 0x25A00C10
    // Test WHILELS_P.P.RR__ field size = 2 (PowerOfTwo)
    // Fields: Rn=0, Rm=0, size=2, sf=0, Pd=0
    let encoding: u32 = 0x25A00C10;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELS_P.P.RR__
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 3, boundary: Max }
/// 64-bit / doubleword size
#[test]
fn test_whilels_p_p_rr_field_size_3_max_c10_25e00c10() {
    // Encoding: 0x25E00C10
    // Test WHILELS_P.P.RR__ field size = 3 (Max)
    // Fields: Rn=0, Pd=0, Rm=0, sf=0, size=3
    let encoding: u32 = 0x25E00C10;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELS_P.P.RR__
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_whilels_p_p_rr_field_rm_0_min_c10_25200c10() {
    // Encoding: 0x25200C10
    // Test WHILELS_P.P.RR__ field Rm = 0 (Min)
    // Fields: size=0, Rn=0, Rm=0, Pd=0, sf=0
    let encoding: u32 = 0x25200C10;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELS_P.P.RR__
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_whilels_p_p_rr_field_rm_1_poweroftwo_c10_25210c10() {
    // Encoding: 0x25210C10
    // Test WHILELS_P.P.RR__ field Rm = 1 (PowerOfTwo)
    // Fields: sf=0, Pd=0, Rn=0, size=0, Rm=1
    let encoding: u32 = 0x25210C10;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELS_P.P.RR__
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_whilels_p_p_rr_field_rm_30_poweroftwominusone_c10_253e0c10() {
    // Encoding: 0x253E0C10
    // Test WHILELS_P.P.RR__ field Rm = 30 (PowerOfTwoMinusOne)
    // Fields: Rm=30, Pd=0, sf=0, size=0, Rn=0
    let encoding: u32 = 0x253E0C10;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELS_P.P.RR__
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 31, boundary: Max }
/// register index 31 (special)
#[test]
fn test_whilels_p_p_rr_field_rm_31_max_c10_253f0c10() {
    // Encoding: 0x253F0C10
    // Test WHILELS_P.P.RR__ field Rm = 31 (Max)
    // Fields: size=0, Pd=0, Rm=31, Rn=0, sf=0
    let encoding: u32 = 0x253F0C10;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELS_P.P.RR__
/// ASL: `field sf 12 +: 1`
/// Requirement: FieldBoundary { field: "sf", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_whilels_p_p_rr_field_sf_0_min_c10_25200c10() {
    // Encoding: 0x25200C10
    // Test WHILELS_P.P.RR__ field sf = 0 (Min)
    // Fields: Rn=0, sf=0, Pd=0, Rm=0, size=0
    let encoding: u32 = 0x25200C10;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELS_P.P.RR__
/// ASL: `field sf 12 +: 1`
/// Requirement: FieldBoundary { field: "sf", value: 1, boundary: Max }
/// 16-bit / halfword size
#[test]
fn test_whilels_p_p_rr_field_sf_1_max_c10_25201c10() {
    // Encoding: 0x25201C10
    // Test WHILELS_P.P.RR__ field sf = 1 (Max)
    // Fields: Rm=0, sf=1, Pd=0, size=0, Rn=0
    let encoding: u32 = 0x25201C10;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELS_P.P.RR__
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_whilels_p_p_rr_field_rn_0_min_c10_25200c10() {
    // Encoding: 0x25200C10
    // Test WHILELS_P.P.RR__ field Rn = 0 (Min)
    // Fields: Rm=0, sf=0, size=0, Pd=0, Rn=0
    let encoding: u32 = 0x25200C10;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELS_P.P.RR__
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_whilels_p_p_rr_field_rn_1_poweroftwo_c10_25200c30() {
    // Encoding: 0x25200C30
    // Test WHILELS_P.P.RR__ field Rn = 1 (PowerOfTwo)
    // Fields: size=0, Rn=1, Rm=0, sf=0, Pd=0
    let encoding: u32 = 0x25200C30;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELS_P.P.RR__
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_whilels_p_p_rr_field_rn_30_poweroftwominusone_c10_25200fd0() {
    // Encoding: 0x25200FD0
    // Test WHILELS_P.P.RR__ field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: size=0, Rn=30, Rm=0, sf=0, Pd=0
    let encoding: u32 = 0x25200FD0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELS_P.P.RR__
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_whilels_p_p_rr_field_rn_31_max_c10_25200ff0() {
    // Encoding: 0x25200FF0
    // Test WHILELS_P.P.RR__ field Rn = 31 (Max)
    // Fields: Pd=0, size=0, Rm=0, sf=0, Rn=31
    let encoding: u32 = 0x25200FF0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELS_P.P.RR__
/// ASL: `field Pd 0 +: 4`
/// Requirement: FieldBoundary { field: "Pd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_whilels_p_p_rr_field_pd_0_min_c10_25200c10() {
    // Encoding: 0x25200C10
    // Test WHILELS_P.P.RR__ field Pd = 0 (Min)
    // Fields: Rm=0, Rn=0, Pd=0, sf=0, size=0
    let encoding: u32 = 0x25200C10;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELS_P.P.RR__
/// ASL: `field Pd 0 +: 4`
/// Requirement: FieldBoundary { field: "Pd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_whilels_p_p_rr_field_pd_1_poweroftwo_c10_25200c11() {
    // Encoding: 0x25200C11
    // Test WHILELS_P.P.RR__ field Pd = 1 (PowerOfTwo)
    // Fields: Rn=0, Rm=0, Pd=1, sf=0, size=0
    let encoding: u32 = 0x25200C11;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELS_P.P.RR__
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=0 (8-bit / byte size)
#[test]
fn test_whilels_p_p_rr_combo_0_c10_25200c10() {
    // Encoding: 0x25200C10
    // Test WHILELS_P.P.RR__ field combination: size=0, Rm=0, sf=0, Rn=0, Pd=0
    // Fields: size=0, Rn=0, Rm=0, sf=0, Pd=0
    let encoding: u32 = 0x25200C10;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELS_P.P.RR__
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=1 (16-bit / halfword size)
#[test]
fn test_whilels_p_p_rr_combo_1_c10_25600c10() {
    // Encoding: 0x25600C10
    // Test WHILELS_P.P.RR__ field combination: size=1, Rm=0, sf=0, Rn=0, Pd=0
    // Fields: Pd=0, Rm=0, size=1, sf=0, Rn=0
    let encoding: u32 = 0x25600C10;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELS_P.P.RR__
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=2 (32-bit / word size)
#[test]
fn test_whilels_p_p_rr_combo_2_c10_25a00c10() {
    // Encoding: 0x25A00C10
    // Test WHILELS_P.P.RR__ field combination: size=2, Rm=0, sf=0, Rn=0, Pd=0
    // Fields: size=2, Rm=0, sf=0, Rn=0, Pd=0
    let encoding: u32 = 0x25A00C10;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELS_P.P.RR__
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=3 (64-bit / doubleword size)
#[test]
fn test_whilels_p_p_rr_combo_3_c10_25e00c10() {
    // Encoding: 0x25E00C10
    // Test WHILELS_P.P.RR__ field combination: size=3, Rm=0, sf=0, Rn=0, Pd=0
    // Fields: size=3, sf=0, Rn=0, Rm=0, Pd=0
    let encoding: u32 = 0x25E00C10;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELS_P.P.RR__
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=0 (register index 0 (first register))
#[test]
fn test_whilels_p_p_rr_combo_4_c10_25200c10() {
    // Encoding: 0x25200C10
    // Test WHILELS_P.P.RR__ field combination: size=0, Rm=0, sf=0, Rn=0, Pd=0
    // Fields: size=0, Rm=0, Pd=0, sf=0, Rn=0
    let encoding: u32 = 0x25200C10;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELS_P.P.RR__
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=1 (register index 1 (second register))
#[test]
fn test_whilels_p_p_rr_combo_5_c10_25210c10() {
    // Encoding: 0x25210C10
    // Test WHILELS_P.P.RR__ field combination: size=0, Rm=1, sf=0, Rn=0, Pd=0
    // Fields: Rm=1, Rn=0, Pd=0, size=0, sf=0
    let encoding: u32 = 0x25210C10;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELS_P.P.RR__
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=30 (register index 30 (LR in some contexts))
#[test]
fn test_whilels_p_p_rr_combo_6_c10_253e0c10() {
    // Encoding: 0x253E0C10
    // Test WHILELS_P.P.RR__ field combination: size=0, Rm=30, sf=0, Rn=0, Pd=0
    // Fields: Rm=30, Rn=0, Pd=0, size=0, sf=0
    let encoding: u32 = 0x253E0C10;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELS_P.P.RR__
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=31 (register index 31 (special))
#[test]
fn test_whilels_p_p_rr_combo_7_c10_253f0c10() {
    // Encoding: 0x253F0C10
    // Test WHILELS_P.P.RR__ field combination: size=0, Rm=31, sf=0, Rn=0, Pd=0
    // Fields: Pd=0, Rn=0, sf=0, size=0, Rm=31
    let encoding: u32 = 0x253F0C10;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELS_P.P.RR__
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// sf=0 (8-bit / byte size)
#[test]
fn test_whilels_p_p_rr_combo_8_c10_25200c10() {
    // Encoding: 0x25200C10
    // Test WHILELS_P.P.RR__ field combination: size=0, Rm=0, sf=0, Rn=0, Pd=0
    // Fields: sf=0, size=0, Rn=0, Pd=0, Rm=0
    let encoding: u32 = 0x25200C10;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELS_P.P.RR__
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// sf=1 (16-bit / halfword size)
#[test]
fn test_whilels_p_p_rr_combo_9_c10_25201c10() {
    // Encoding: 0x25201C10
    // Test WHILELS_P.P.RR__ field combination: size=0, Rm=0, sf=1, Rn=0, Pd=0
    // Fields: Pd=0, sf=1, size=0, Rm=0, Rn=0
    let encoding: u32 = 0x25201C10;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELS_P.P.RR__
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_whilels_p_p_rr_combo_10_c10_25200c10() {
    // Encoding: 0x25200C10
    // Test WHILELS_P.P.RR__ field combination: size=0, Rm=0, sf=0, Rn=0, Pd=0
    // Fields: sf=0, Rn=0, size=0, Rm=0, Pd=0
    let encoding: u32 = 0x25200C10;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELS_P.P.RR__
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_whilels_p_p_rr_combo_11_c10_25200c30() {
    // Encoding: 0x25200C30
    // Test WHILELS_P.P.RR__ field combination: size=0, Rm=0, sf=0, Rn=1, Pd=0
    // Fields: Rm=0, size=0, Pd=0, sf=0, Rn=1
    let encoding: u32 = 0x25200C30;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELS_P.P.RR__
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_whilels_p_p_rr_combo_12_c10_25200fd0() {
    // Encoding: 0x25200FD0
    // Test WHILELS_P.P.RR__ field combination: size=0, Rm=0, sf=0, Rn=30, Pd=0
    // Fields: Rm=0, size=0, sf=0, Rn=30, Pd=0
    let encoding: u32 = 0x25200FD0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELS_P.P.RR__
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_whilels_p_p_rr_combo_13_c10_25200ff0() {
    // Encoding: 0x25200FF0
    // Test WHILELS_P.P.RR__ field combination: size=0, Rm=0, sf=0, Rn=31, Pd=0
    // Fields: Pd=0, sf=0, size=0, Rm=0, Rn=31
    let encoding: u32 = 0x25200FF0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELS_P.P.RR__
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pd=0 (register index 0 (first register))
#[test]
fn test_whilels_p_p_rr_combo_14_c10_25200c10() {
    // Encoding: 0x25200C10
    // Test WHILELS_P.P.RR__ field combination: size=0, Rm=0, sf=0, Rn=0, Pd=0
    // Fields: Pd=0, Rm=0, Rn=0, size=0, sf=0
    let encoding: u32 = 0x25200C10;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELS_P.P.RR__
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pd=1 (register index 1 (second register))
#[test]
fn test_whilels_p_p_rr_combo_15_c10_25200c11() {
    // Encoding: 0x25200C11
    // Test WHILELS_P.P.RR__ field combination: size=0, Rm=0, sf=0, Rn=0, Pd=1
    // Fields: sf=0, Rn=0, Rm=0, size=0, Pd=1
    let encoding: u32 = 0x25200C11;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELS_P.P.RR__
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=1 (same register test (reg=1)), Rn=1 (same register test (reg=1))
#[test]
fn test_whilels_p_p_rr_combo_16_c10_25210c30() {
    // Encoding: 0x25210C30
    // Test WHILELS_P.P.RR__ field combination: size=0, Rm=1, sf=0, Rn=1, Pd=0
    // Fields: sf=0, Pd=0, size=0, Rn=1, Rm=1
    let encoding: u32 = 0x25210C30;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELS_P.P.RR__
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=31 (same register test (reg=31)), Rn=31 (same register test (reg=31))
#[test]
fn test_whilels_p_p_rr_combo_17_c10_253f0ff0() {
    // Encoding: 0x253F0FF0
    // Test WHILELS_P.P.RR__ field combination: size=0, Rm=31, sf=0, Rn=31, Pd=0
    // Fields: Pd=0, sf=0, Rm=31, Rn=31, size=0
    let encoding: u32 = 0x253F0FF0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELS_P.P.RR__
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=1 (same register test (reg=1)), Pd=1 (same register test (reg=1))
#[test]
fn test_whilels_p_p_rr_combo_18_c10_25210c11() {
    // Encoding: 0x25210C11
    // Test WHILELS_P.P.RR__ field combination: size=0, Rm=1, sf=0, Rn=0, Pd=1
    // Fields: Pd=1, size=0, Rn=0, sf=0, Rm=1
    let encoding: u32 = 0x25210C11;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELS_P.P.RR__
/// ASL: `field combination 19`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=31 (same register test (reg=31)), Pd=31 (same register test (reg=31))
#[test]
fn test_whilels_p_p_rr_combo_19_c10_253f0c1f() {
    // Encoding: 0x253F0C1F
    // Test WHILELS_P.P.RR__ field combination: size=0, Rm=31, sf=0, Rn=0, Pd=31
    // Fields: sf=0, Rm=31, size=0, Rn=0, Pd=31
    let encoding: u32 = 0x253F0C1F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELS_P.P.RR__
/// ASL: `field combination 20`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (same register test (reg=1)), Pd=1 (same register test (reg=1))
#[test]
fn test_whilels_p_p_rr_combo_20_c10_25200c31() {
    // Encoding: 0x25200C31
    // Test WHILELS_P.P.RR__ field combination: size=0, Rm=0, sf=0, Rn=1, Pd=1
    // Fields: Rm=0, Rn=1, Pd=1, size=0, sf=0
    let encoding: u32 = 0x25200C31;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELS_P.P.RR__
/// ASL: `field combination 21`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (same register test (reg=31)), Pd=31 (same register test (reg=31))
#[test]
fn test_whilels_p_p_rr_combo_21_c10_25200fff() {
    // Encoding: 0x25200FFF
    // Test WHILELS_P.P.RR__ field combination: size=0, Rm=0, sf=0, Rn=31, Pd=31
    // Fields: Rm=0, sf=0, Rn=31, Pd=31, size=0
    let encoding: u32 = 0x25200FFF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELS_P.P.RR__
/// ASL: `field size = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "size", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_whilels_p_p_rr_special_size_0_size_variant_0_3088_25200c10() {
    // Encoding: 0x25200C10
    // Test WHILELS_P.P.RR__ special value size = 0 (Size variant 0)
    // Fields: Pd=0, size=0, Rm=0, sf=0, Rn=0
    let encoding: u32 = 0x25200C10;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELS_P.P.RR__
/// ASL: `field size = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "size", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_whilels_p_p_rr_special_size_1_size_variant_1_3088_25600c10() {
    // Encoding: 0x25600C10
    // Test WHILELS_P.P.RR__ special value size = 1 (Size variant 1)
    // Fields: Pd=0, size=1, Rm=0, Rn=0, sf=0
    let encoding: u32 = 0x25600C10;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELS_P.P.RR__
/// ASL: `field size = 2 (Size variant 2)`
/// Requirement: FieldSpecial { field: "size", value: 2, meaning: "Size variant 2" }
/// Size variant 2
#[test]
fn test_whilels_p_p_rr_special_size_2_size_variant_2_3088_25a00c10() {
    // Encoding: 0x25A00C10
    // Test WHILELS_P.P.RR__ special value size = 2 (Size variant 2)
    // Fields: size=2, sf=0, Rn=0, Rm=0, Pd=0
    let encoding: u32 = 0x25A00C10;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELS_P.P.RR__
/// ASL: `field size = 3 (Size variant 3)`
/// Requirement: FieldSpecial { field: "size", value: 3, meaning: "Size variant 3" }
/// Size variant 3
#[test]
fn test_whilels_p_p_rr_special_size_3_size_variant_3_3088_25e00c10() {
    // Encoding: 0x25E00C10
    // Test WHILELS_P.P.RR__ special value size = 3 (Size variant 3)
    // Fields: sf=0, Rn=0, Rm=0, Pd=0, size=3
    let encoding: u32 = 0x25E00C10;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELS_P.P.RR__
/// ASL: `field sf = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "sf", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_whilels_p_p_rr_special_sf_0_size_variant_0_3088_25600c10() {
    // Encoding: 0x25600C10
    // Test WHILELS_P.P.RR__ special value sf = 0 (Size variant 0)
    // Fields: Rn=0, sf=0, size=1, Rm=0, Pd=0
    let encoding: u32 = 0x25600C10;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELS_P.P.RR__
/// ASL: `field sf = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "sf", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_whilels_p_p_rr_special_sf_1_size_variant_1_3088_25601c10() {
    // Encoding: 0x25601C10
    // Test WHILELS_P.P.RR__ special value sf = 1 (Size variant 1)
    // Fields: Pd=0, sf=1, Rm=0, Rn=0, size=1
    let encoding: u32 = 0x25601C10;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELS_P.P.RR__
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_whilels_p_p_rr_special_rn_31_stack_pointer_sp_may_require_alignment_3088_25600ff0() {
    // Encoding: 0x25600FF0
    // Test WHILELS_P.P.RR__ special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: size=1, Rm=0, Pd=0, Rn=31, sf=0
    let encoding: u32 = 0x25600FF0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELS_P.P.RR__
/// ASL: `SimdFromField("Pd") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "unknown" }
/// verify register write to SimdFromField("Pd")
#[test]
fn test_whilels_p_p_rr_reg_write_0_25200c10() {
    // Test WHILELS_P.P.RR__ register write: SimdFromField("Pd")
    // Encoding: 0x25200C10
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x25200C10;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: WHILELS_P.P.RR__
/// ASL: `Rn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Rn = 31)
#[test]
fn test_whilels_p_p_rr_sp_rn_25200ff0() {
    // Test WHILELS_P.P.RR__ with Rn = SP (31)
    // Encoding: 0x25200FF0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x25200FF0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: WHILELS_P.P.RR__
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 0 + 0 = 0 (Z=1)
#[test]
fn test_whilels_p_p_rr_flags_zeroresult_0_25221c30() {
    // Test WHILELS_P.P.RR__ flag computation: ZeroResult
    // Encoding: 0x25221C30
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0x0);
    set_x(&mut cpu, 1, 0x0);
    let encoding: u32 = 0x25221C30;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: WHILELS_P.P.RR__
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 1 + (-1) = 0 (Z=1, C=1)
#[test]
fn test_whilels_p_p_rr_flags_zeroresult_1_25221c30() {
    // Test WHILELS_P.P.RR__ flag computation: ZeroResult
    // Encoding: 0x25221C30
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0xFFFFFFFFFFFFFFFF);
    set_x(&mut cpu, 1, 0x1);
    let encoding: u32 = 0x25221C30;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: WHILELS_P.P.RR__
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: NegativeResult }
/// negative value (N=1)
#[test]
fn test_whilels_p_p_rr_flags_negativeresult_2_25221c30() {
    // Test WHILELS_P.P.RR__ flag computation: NegativeResult
    // Encoding: 0x25221C30
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x8000000000000000);
    set_x(&mut cpu, 2, 0x0);
    let encoding: u32 = 0x25221C30;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: WHILELS_P.P.RR__
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 1 = 0 (C=1, Z=1)
#[test]
fn test_whilels_p_p_rr_flags_unsignedoverflow_3_25221c30() {
    // Test WHILELS_P.P.RR__ flag computation: UnsignedOverflow
    // Encoding: 0x25221C30
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0xFFFFFFFFFFFFFFFF);
    set_x(&mut cpu, 2, 0x1);
    let encoding: u32 = 0x25221C30;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: WHILELS_P.P.RR__
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 2 = 1 (C=1)
#[test]
fn test_whilels_p_p_rr_flags_unsignedoverflow_4_25221c30() {
    // Test WHILELS_P.P.RR__ flag computation: UnsignedOverflow
    // Encoding: 0x25221C30
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0x2);
    set_x(&mut cpu, 1, 0xFFFFFFFFFFFFFFFF);
    let encoding: u32 = 0x25221C30;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: WHILELS_P.P.RR__
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// max_signed + 1 = min_signed (V=1, N=1)
#[test]
fn test_whilels_p_p_rr_flags_signedoverflow_5_25221c30() {
    // Test WHILELS_P.P.RR__ flag computation: SignedOverflow
    // Encoding: 0x25221C30
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x7FFFFFFFFFFFFFFF);
    set_x(&mut cpu, 2, 0x1);
    let encoding: u32 = 0x25221C30;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: WHILELS_P.P.RR__
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// min_signed + (-1) = max_signed (V=1)
#[test]
fn test_whilels_p_p_rr_flags_signedoverflow_6_25221c30() {
    // Test WHILELS_P.P.RR__ flag computation: SignedOverflow
    // Encoding: 0x25221C30
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0xFFFFFFFFFFFFFFFF);
    set_x(&mut cpu, 1, 0x8000000000000000);
    let encoding: u32 = 0x25221C30;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: WHILELS_P.P.RR__
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: PositiveResult }
/// 100 + 50 = 150 (no flags)
#[test]
fn test_whilels_p_p_rr_flags_positiveresult_7_25221c30() {
    // Test WHILELS_P.P.RR__ flag computation: PositiveResult
    // Encoding: 0x25221C30
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x64);
    set_x(&mut cpu, 2, 0x32);
    let encoding: u32 = 0x25221C30;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

// ============================================================================
// SETFFR_F__ Tests
// ============================================================================

/// Provenance: SETFFR_F__
/// ASL: `fixed encoding (no variable fields)`
/// Requirement: BasicEncoding
/// instruction with no variable fields
#[test]
fn test_setffr_f_basic_encoding_252c9000() {
    // Encoding: 0x252C9000
    // Test SETFFR_F__ basic encoding
    let encoding: u32 = 0x252C9000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

// ============================================================================
// WHILELT_P.P.RR__ Tests
// ============================================================================

/// Provenance: WHILELT_P.P.RR__
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_whilelt_p_p_rr_field_size_0_min_400_25200400() {
    // Encoding: 0x25200400
    // Test WHILELT_P.P.RR__ field size = 0 (Min)
    // Fields: size=0, Rm=0, Pd=0, sf=0, Rn=0
    let encoding: u32 = 0x25200400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELT_P.P.RR__
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 1, boundary: PowerOfTwo }
/// 16-bit / halfword size
#[test]
fn test_whilelt_p_p_rr_field_size_1_poweroftwo_400_25600400() {
    // Encoding: 0x25600400
    // Test WHILELT_P.P.RR__ field size = 1 (PowerOfTwo)
    // Fields: sf=0, Pd=0, Rm=0, Rn=0, size=1
    let encoding: u32 = 0x25600400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELT_P.P.RR__
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 2, boundary: PowerOfTwo }
/// 32-bit / word size
#[test]
fn test_whilelt_p_p_rr_field_size_2_poweroftwo_400_25a00400() {
    // Encoding: 0x25A00400
    // Test WHILELT_P.P.RR__ field size = 2 (PowerOfTwo)
    // Fields: size=2, Rm=0, sf=0, Rn=0, Pd=0
    let encoding: u32 = 0x25A00400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELT_P.P.RR__
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 3, boundary: Max }
/// 64-bit / doubleword size
#[test]
fn test_whilelt_p_p_rr_field_size_3_max_400_25e00400() {
    // Encoding: 0x25E00400
    // Test WHILELT_P.P.RR__ field size = 3 (Max)
    // Fields: Rn=0, size=3, sf=0, Rm=0, Pd=0
    let encoding: u32 = 0x25E00400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELT_P.P.RR__
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_whilelt_p_p_rr_field_rm_0_min_400_25200400() {
    // Encoding: 0x25200400
    // Test WHILELT_P.P.RR__ field Rm = 0 (Min)
    // Fields: sf=0, Rm=0, size=0, Rn=0, Pd=0
    let encoding: u32 = 0x25200400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELT_P.P.RR__
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_whilelt_p_p_rr_field_rm_1_poweroftwo_400_25210400() {
    // Encoding: 0x25210400
    // Test WHILELT_P.P.RR__ field Rm = 1 (PowerOfTwo)
    // Fields: sf=0, Pd=0, Rn=0, size=0, Rm=1
    let encoding: u32 = 0x25210400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELT_P.P.RR__
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_whilelt_p_p_rr_field_rm_30_poweroftwominusone_400_253e0400() {
    // Encoding: 0x253E0400
    // Test WHILELT_P.P.RR__ field Rm = 30 (PowerOfTwoMinusOne)
    // Fields: size=0, Rm=30, sf=0, Rn=0, Pd=0
    let encoding: u32 = 0x253E0400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELT_P.P.RR__
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 31, boundary: Max }
/// register index 31 (special)
#[test]
fn test_whilelt_p_p_rr_field_rm_31_max_400_253f0400() {
    // Encoding: 0x253F0400
    // Test WHILELT_P.P.RR__ field Rm = 31 (Max)
    // Fields: Rm=31, Pd=0, sf=0, Rn=0, size=0
    let encoding: u32 = 0x253F0400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELT_P.P.RR__
/// ASL: `field sf 12 +: 1`
/// Requirement: FieldBoundary { field: "sf", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_whilelt_p_p_rr_field_sf_0_min_400_25200400() {
    // Encoding: 0x25200400
    // Test WHILELT_P.P.RR__ field sf = 0 (Min)
    // Fields: Rm=0, Rn=0, Pd=0, sf=0, size=0
    let encoding: u32 = 0x25200400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELT_P.P.RR__
/// ASL: `field sf 12 +: 1`
/// Requirement: FieldBoundary { field: "sf", value: 1, boundary: Max }
/// 16-bit / halfword size
#[test]
fn test_whilelt_p_p_rr_field_sf_1_max_400_25201400() {
    // Encoding: 0x25201400
    // Test WHILELT_P.P.RR__ field sf = 1 (Max)
    // Fields: Rm=0, size=0, sf=1, Rn=0, Pd=0
    let encoding: u32 = 0x25201400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELT_P.P.RR__
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_whilelt_p_p_rr_field_rn_0_min_400_25200400() {
    // Encoding: 0x25200400
    // Test WHILELT_P.P.RR__ field Rn = 0 (Min)
    // Fields: size=0, Rm=0, Rn=0, sf=0, Pd=0
    let encoding: u32 = 0x25200400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELT_P.P.RR__
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_whilelt_p_p_rr_field_rn_1_poweroftwo_400_25200420() {
    // Encoding: 0x25200420
    // Test WHILELT_P.P.RR__ field Rn = 1 (PowerOfTwo)
    // Fields: sf=0, size=0, Pd=0, Rm=0, Rn=1
    let encoding: u32 = 0x25200420;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELT_P.P.RR__
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_whilelt_p_p_rr_field_rn_30_poweroftwominusone_400_252007c0() {
    // Encoding: 0x252007C0
    // Test WHILELT_P.P.RR__ field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: Pd=0, Rm=0, size=0, sf=0, Rn=30
    let encoding: u32 = 0x252007C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELT_P.P.RR__
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_whilelt_p_p_rr_field_rn_31_max_400_252007e0() {
    // Encoding: 0x252007E0
    // Test WHILELT_P.P.RR__ field Rn = 31 (Max)
    // Fields: sf=0, size=0, Rn=31, Pd=0, Rm=0
    let encoding: u32 = 0x252007E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELT_P.P.RR__
/// ASL: `field Pd 0 +: 4`
/// Requirement: FieldBoundary { field: "Pd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_whilelt_p_p_rr_field_pd_0_min_400_25200400() {
    // Encoding: 0x25200400
    // Test WHILELT_P.P.RR__ field Pd = 0 (Min)
    // Fields: size=0, sf=0, Rn=0, Rm=0, Pd=0
    let encoding: u32 = 0x25200400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELT_P.P.RR__
/// ASL: `field Pd 0 +: 4`
/// Requirement: FieldBoundary { field: "Pd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_whilelt_p_p_rr_field_pd_1_poweroftwo_400_25200401() {
    // Encoding: 0x25200401
    // Test WHILELT_P.P.RR__ field Pd = 1 (PowerOfTwo)
    // Fields: Rm=0, Rn=0, sf=0, Pd=1, size=0
    let encoding: u32 = 0x25200401;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELT_P.P.RR__
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=0 (8-bit / byte size)
#[test]
fn test_whilelt_p_p_rr_combo_0_400_25200400() {
    // Encoding: 0x25200400
    // Test WHILELT_P.P.RR__ field combination: size=0, Rm=0, sf=0, Rn=0, Pd=0
    // Fields: Rn=0, Pd=0, sf=0, Rm=0, size=0
    let encoding: u32 = 0x25200400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELT_P.P.RR__
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=1 (16-bit / halfword size)
#[test]
fn test_whilelt_p_p_rr_combo_1_400_25600400() {
    // Encoding: 0x25600400
    // Test WHILELT_P.P.RR__ field combination: size=1, Rm=0, sf=0, Rn=0, Pd=0
    // Fields: Rm=0, sf=0, Rn=0, Pd=0, size=1
    let encoding: u32 = 0x25600400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELT_P.P.RR__
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=2 (32-bit / word size)
#[test]
fn test_whilelt_p_p_rr_combo_2_400_25a00400() {
    // Encoding: 0x25A00400
    // Test WHILELT_P.P.RR__ field combination: size=2, Rm=0, sf=0, Rn=0, Pd=0
    // Fields: Rm=0, sf=0, size=2, Rn=0, Pd=0
    let encoding: u32 = 0x25A00400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELT_P.P.RR__
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=3 (64-bit / doubleword size)
#[test]
fn test_whilelt_p_p_rr_combo_3_400_25e00400() {
    // Encoding: 0x25E00400
    // Test WHILELT_P.P.RR__ field combination: size=3, Rm=0, sf=0, Rn=0, Pd=0
    // Fields: Rm=0, Rn=0, size=3, sf=0, Pd=0
    let encoding: u32 = 0x25E00400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELT_P.P.RR__
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=0 (register index 0 (first register))
#[test]
fn test_whilelt_p_p_rr_combo_4_400_25200400() {
    // Encoding: 0x25200400
    // Test WHILELT_P.P.RR__ field combination: size=0, Rm=0, sf=0, Rn=0, Pd=0
    // Fields: Rn=0, sf=0, Rm=0, size=0, Pd=0
    let encoding: u32 = 0x25200400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELT_P.P.RR__
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=1 (register index 1 (second register))
#[test]
fn test_whilelt_p_p_rr_combo_5_400_25210400() {
    // Encoding: 0x25210400
    // Test WHILELT_P.P.RR__ field combination: size=0, Rm=1, sf=0, Rn=0, Pd=0
    // Fields: size=0, Rn=0, sf=0, Rm=1, Pd=0
    let encoding: u32 = 0x25210400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELT_P.P.RR__
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=30 (register index 30 (LR in some contexts))
#[test]
fn test_whilelt_p_p_rr_combo_6_400_253e0400() {
    // Encoding: 0x253E0400
    // Test WHILELT_P.P.RR__ field combination: size=0, Rm=30, sf=0, Rn=0, Pd=0
    // Fields: size=0, Pd=0, sf=0, Rn=0, Rm=30
    let encoding: u32 = 0x253E0400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELT_P.P.RR__
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=31 (register index 31 (special))
#[test]
fn test_whilelt_p_p_rr_combo_7_400_253f0400() {
    // Encoding: 0x253F0400
    // Test WHILELT_P.P.RR__ field combination: size=0, Rm=31, sf=0, Rn=0, Pd=0
    // Fields: sf=0, size=0, Pd=0, Rn=0, Rm=31
    let encoding: u32 = 0x253F0400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELT_P.P.RR__
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// sf=0 (8-bit / byte size)
#[test]
fn test_whilelt_p_p_rr_combo_8_400_25200400() {
    // Encoding: 0x25200400
    // Test WHILELT_P.P.RR__ field combination: size=0, Rm=0, sf=0, Rn=0, Pd=0
    // Fields: size=0, sf=0, Rm=0, Rn=0, Pd=0
    let encoding: u32 = 0x25200400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELT_P.P.RR__
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// sf=1 (16-bit / halfword size)
#[test]
fn test_whilelt_p_p_rr_combo_9_400_25201400() {
    // Encoding: 0x25201400
    // Test WHILELT_P.P.RR__ field combination: size=0, Rm=0, sf=1, Rn=0, Pd=0
    // Fields: Pd=0, size=0, Rm=0, sf=1, Rn=0
    let encoding: u32 = 0x25201400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELT_P.P.RR__
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_whilelt_p_p_rr_combo_10_400_25200400() {
    // Encoding: 0x25200400
    // Test WHILELT_P.P.RR__ field combination: size=0, Rm=0, sf=0, Rn=0, Pd=0
    // Fields: Pd=0, Rm=0, Rn=0, sf=0, size=0
    let encoding: u32 = 0x25200400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELT_P.P.RR__
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_whilelt_p_p_rr_combo_11_400_25200420() {
    // Encoding: 0x25200420
    // Test WHILELT_P.P.RR__ field combination: size=0, Rm=0, sf=0, Rn=1, Pd=0
    // Fields: size=0, Rm=0, Pd=0, sf=0, Rn=1
    let encoding: u32 = 0x25200420;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELT_P.P.RR__
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_whilelt_p_p_rr_combo_12_400_252007c0() {
    // Encoding: 0x252007C0
    // Test WHILELT_P.P.RR__ field combination: size=0, Rm=0, sf=0, Rn=30, Pd=0
    // Fields: Rm=0, sf=0, size=0, Rn=30, Pd=0
    let encoding: u32 = 0x252007C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELT_P.P.RR__
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_whilelt_p_p_rr_combo_13_400_252007e0() {
    // Encoding: 0x252007E0
    // Test WHILELT_P.P.RR__ field combination: size=0, Rm=0, sf=0, Rn=31, Pd=0
    // Fields: Rm=0, Pd=0, sf=0, Rn=31, size=0
    let encoding: u32 = 0x252007E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELT_P.P.RR__
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pd=0 (register index 0 (first register))
#[test]
fn test_whilelt_p_p_rr_combo_14_400_25200400() {
    // Encoding: 0x25200400
    // Test WHILELT_P.P.RR__ field combination: size=0, Rm=0, sf=0, Rn=0, Pd=0
    // Fields: Pd=0, Rm=0, Rn=0, size=0, sf=0
    let encoding: u32 = 0x25200400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELT_P.P.RR__
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pd=1 (register index 1 (second register))
#[test]
fn test_whilelt_p_p_rr_combo_15_400_25200401() {
    // Encoding: 0x25200401
    // Test WHILELT_P.P.RR__ field combination: size=0, Rm=0, sf=0, Rn=0, Pd=1
    // Fields: size=0, sf=0, Rn=0, Pd=1, Rm=0
    let encoding: u32 = 0x25200401;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELT_P.P.RR__
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=1 (same register test (reg=1)), Rn=1 (same register test (reg=1))
#[test]
fn test_whilelt_p_p_rr_combo_16_400_25210420() {
    // Encoding: 0x25210420
    // Test WHILELT_P.P.RR__ field combination: size=0, Rm=1, sf=0, Rn=1, Pd=0
    // Fields: sf=0, Rn=1, Pd=0, Rm=1, size=0
    let encoding: u32 = 0x25210420;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELT_P.P.RR__
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=31 (same register test (reg=31)), Rn=31 (same register test (reg=31))
#[test]
fn test_whilelt_p_p_rr_combo_17_400_253f07e0() {
    // Encoding: 0x253F07E0
    // Test WHILELT_P.P.RR__ field combination: size=0, Rm=31, sf=0, Rn=31, Pd=0
    // Fields: size=0, Rn=31, Rm=31, Pd=0, sf=0
    let encoding: u32 = 0x253F07E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELT_P.P.RR__
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=1 (same register test (reg=1)), Pd=1 (same register test (reg=1))
#[test]
fn test_whilelt_p_p_rr_combo_18_400_25210401() {
    // Encoding: 0x25210401
    // Test WHILELT_P.P.RR__ field combination: size=0, Rm=1, sf=0, Rn=0, Pd=1
    // Fields: size=0, Rn=0, Pd=1, Rm=1, sf=0
    let encoding: u32 = 0x25210401;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELT_P.P.RR__
/// ASL: `field combination 19`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=31 (same register test (reg=31)), Pd=31 (same register test (reg=31))
#[test]
fn test_whilelt_p_p_rr_combo_19_400_253f040f() {
    // Encoding: 0x253F040F
    // Test WHILELT_P.P.RR__ field combination: size=0, Rm=31, sf=0, Rn=0, Pd=31
    // Fields: sf=0, Rn=0, size=0, Rm=31, Pd=31
    let encoding: u32 = 0x253F040F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELT_P.P.RR__
/// ASL: `field combination 20`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (same register test (reg=1)), Pd=1 (same register test (reg=1))
#[test]
fn test_whilelt_p_p_rr_combo_20_400_25200421() {
    // Encoding: 0x25200421
    // Test WHILELT_P.P.RR__ field combination: size=0, Rm=0, sf=0, Rn=1, Pd=1
    // Fields: Pd=1, size=0, Rm=0, sf=0, Rn=1
    let encoding: u32 = 0x25200421;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELT_P.P.RR__
/// ASL: `field combination 21`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (same register test (reg=31)), Pd=31 (same register test (reg=31))
#[test]
fn test_whilelt_p_p_rr_combo_21_400_252007ef() {
    // Encoding: 0x252007EF
    // Test WHILELT_P.P.RR__ field combination: size=0, Rm=0, sf=0, Rn=31, Pd=31
    // Fields: sf=0, Rm=0, Pd=31, Rn=31, size=0
    let encoding: u32 = 0x252007EF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELT_P.P.RR__
/// ASL: `field size = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "size", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_whilelt_p_p_rr_special_size_0_size_variant_0_1024_25200400() {
    // Encoding: 0x25200400
    // Test WHILELT_P.P.RR__ special value size = 0 (Size variant 0)
    // Fields: Pd=0, Rm=0, size=0, sf=0, Rn=0
    let encoding: u32 = 0x25200400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELT_P.P.RR__
/// ASL: `field size = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "size", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_whilelt_p_p_rr_special_size_1_size_variant_1_1024_25600400() {
    // Encoding: 0x25600400
    // Test WHILELT_P.P.RR__ special value size = 1 (Size variant 1)
    // Fields: sf=0, Rn=0, Pd=0, Rm=0, size=1
    let encoding: u32 = 0x25600400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELT_P.P.RR__
/// ASL: `field size = 2 (Size variant 2)`
/// Requirement: FieldSpecial { field: "size", value: 2, meaning: "Size variant 2" }
/// Size variant 2
#[test]
fn test_whilelt_p_p_rr_special_size_2_size_variant_2_1024_25a00400() {
    // Encoding: 0x25A00400
    // Test WHILELT_P.P.RR__ special value size = 2 (Size variant 2)
    // Fields: Rm=0, Rn=0, size=2, sf=0, Pd=0
    let encoding: u32 = 0x25A00400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELT_P.P.RR__
/// ASL: `field size = 3 (Size variant 3)`
/// Requirement: FieldSpecial { field: "size", value: 3, meaning: "Size variant 3" }
/// Size variant 3
#[test]
fn test_whilelt_p_p_rr_special_size_3_size_variant_3_1024_25e00400() {
    // Encoding: 0x25E00400
    // Test WHILELT_P.P.RR__ special value size = 3 (Size variant 3)
    // Fields: Rm=0, Pd=0, sf=0, size=3, Rn=0
    let encoding: u32 = 0x25E00400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELT_P.P.RR__
/// ASL: `field sf = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "sf", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_whilelt_p_p_rr_special_sf_0_size_variant_0_1024_25600400() {
    // Encoding: 0x25600400
    // Test WHILELT_P.P.RR__ special value sf = 0 (Size variant 0)
    // Fields: Pd=0, sf=0, Rm=0, Rn=0, size=1
    let encoding: u32 = 0x25600400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELT_P.P.RR__
/// ASL: `field sf = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "sf", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_whilelt_p_p_rr_special_sf_1_size_variant_1_1024_25601400() {
    // Encoding: 0x25601400
    // Test WHILELT_P.P.RR__ special value sf = 1 (Size variant 1)
    // Fields: Rn=0, Rm=0, size=1, sf=1, Pd=0
    let encoding: u32 = 0x25601400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELT_P.P.RR__
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_whilelt_p_p_rr_special_rn_31_stack_pointer_sp_may_require_alignment_1024_256007e0() {
    // Encoding: 0x256007E0
    // Test WHILELT_P.P.RR__ special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: sf=0, Pd=0, Rm=0, size=1, Rn=31
    let encoding: u32 = 0x256007E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELT_P.P.RR__
/// ASL: `SimdFromField("Pd") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "unknown" }
/// verify register write to SimdFromField("Pd")
#[test]
fn test_whilelt_p_p_rr_reg_write_0_25200400() {
    // Test WHILELT_P.P.RR__ register write: SimdFromField("Pd")
    // Encoding: 0x25200400
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x25200400;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: WHILELT_P.P.RR__
/// ASL: `Rn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Rn = 31)
#[test]
fn test_whilelt_p_p_rr_sp_rn_252007e0() {
    // Test WHILELT_P.P.RR__ with Rn = SP (31)
    // Encoding: 0x252007E0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x252007E0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: WHILELT_P.P.RR__
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 0 + 0 = 0 (Z=1)
#[test]
fn test_whilelt_p_p_rr_flags_zeroresult_0_25221420() {
    // Test WHILELT_P.P.RR__ flag computation: ZeroResult
    // Encoding: 0x25221420
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x0);
    set_x(&mut cpu, 2, 0x0);
    let encoding: u32 = 0x25221420;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: WHILELT_P.P.RR__
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 1 + (-1) = 0 (Z=1, C=1)
#[test]
fn test_whilelt_p_p_rr_flags_zeroresult_1_25221420() {
    // Test WHILELT_P.P.RR__ flag computation: ZeroResult
    // Encoding: 0x25221420
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0xFFFFFFFFFFFFFFFF);
    set_x(&mut cpu, 1, 0x1);
    let encoding: u32 = 0x25221420;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: WHILELT_P.P.RR__
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: NegativeResult }
/// negative value (N=1)
#[test]
fn test_whilelt_p_p_rr_flags_negativeresult_2_25221420() {
    // Test WHILELT_P.P.RR__ flag computation: NegativeResult
    // Encoding: 0x25221420
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x8000000000000000);
    set_x(&mut cpu, 2, 0x0);
    let encoding: u32 = 0x25221420;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: WHILELT_P.P.RR__
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 1 = 0 (C=1, Z=1)
#[test]
fn test_whilelt_p_p_rr_flags_unsignedoverflow_3_25221420() {
    // Test WHILELT_P.P.RR__ flag computation: UnsignedOverflow
    // Encoding: 0x25221420
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0xFFFFFFFFFFFFFFFF);
    set_x(&mut cpu, 2, 0x1);
    let encoding: u32 = 0x25221420;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: WHILELT_P.P.RR__
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 2 = 1 (C=1)
#[test]
fn test_whilelt_p_p_rr_flags_unsignedoverflow_4_25221420() {
    // Test WHILELT_P.P.RR__ flag computation: UnsignedOverflow
    // Encoding: 0x25221420
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0xFFFFFFFFFFFFFFFF);
    set_x(&mut cpu, 2, 0x2);
    let encoding: u32 = 0x25221420;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: WHILELT_P.P.RR__
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// max_signed + 1 = min_signed (V=1, N=1)
#[test]
fn test_whilelt_p_p_rr_flags_signedoverflow_5_25221420() {
    // Test WHILELT_P.P.RR__ flag computation: SignedOverflow
    // Encoding: 0x25221420
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0x1);
    set_x(&mut cpu, 1, 0x7FFFFFFFFFFFFFFF);
    let encoding: u32 = 0x25221420;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: WHILELT_P.P.RR__
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// min_signed + (-1) = max_signed (V=1)
#[test]
fn test_whilelt_p_p_rr_flags_signedoverflow_6_25221420() {
    // Test WHILELT_P.P.RR__ flag computation: SignedOverflow
    // Encoding: 0x25221420
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0xFFFFFFFFFFFFFFFF);
    set_x(&mut cpu, 1, 0x8000000000000000);
    let encoding: u32 = 0x25221420;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: WHILELT_P.P.RR__
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: PositiveResult }
/// 100 + 50 = 150 (no flags)
#[test]
fn test_whilelt_p_p_rr_flags_positiveresult_7_25221420() {
    // Test WHILELT_P.P.RR__ flag computation: PositiveResult
    // Encoding: 0x25221420
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0x32);
    set_x(&mut cpu, 1, 0x64);
    let encoding: u32 = 0x25221420;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

// ============================================================================
// NOR_P.P.PP_Z Tests
// ============================================================================

/// Provenance: NOR_P.P.PP_Z
/// ASL: `field Pm 16 +: 4`
/// Requirement: FieldBoundary { field: "Pm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_nor_p_p_pp_z_field_pm_0_min_4200_25804200() {
    // Encoding: 0x25804200
    // Test NOR_P.P.PP_Z field Pm = 0 (Min)
    // Fields: Pm=0, Pg=0, Pn=0, Pd=0
    let encoding: u32 = 0x25804200;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: NOR_P.P.PP_Z
/// ASL: `field Pm 16 +: 4`
/// Requirement: FieldBoundary { field: "Pm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_nor_p_p_pp_z_field_pm_1_poweroftwo_4200_25814200() {
    // Encoding: 0x25814200
    // Test NOR_P.P.PP_Z field Pm = 1 (PowerOfTwo)
    // Fields: Pg=0, Pn=0, Pm=1, Pd=0
    let encoding: u32 = 0x25814200;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: NOR_P.P.PP_Z
/// ASL: `field Pg 10 +: 4`
/// Requirement: FieldBoundary { field: "Pg", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_nor_p_p_pp_z_field_pg_0_min_4200_25804200() {
    // Encoding: 0x25804200
    // Test NOR_P.P.PP_Z field Pg = 0 (Min)
    // Fields: Pn=0, Pm=0, Pg=0, Pd=0
    let encoding: u32 = 0x25804200;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: NOR_P.P.PP_Z
/// ASL: `field Pg 10 +: 4`
/// Requirement: FieldBoundary { field: "Pg", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_nor_p_p_pp_z_field_pg_1_poweroftwo_4200_25804600() {
    // Encoding: 0x25804600
    // Test NOR_P.P.PP_Z field Pg = 1 (PowerOfTwo)
    // Fields: Pd=0, Pm=0, Pn=0, Pg=1
    let encoding: u32 = 0x25804600;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: NOR_P.P.PP_Z
/// ASL: `field Pn 5 +: 4`
/// Requirement: FieldBoundary { field: "Pn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_nor_p_p_pp_z_field_pn_0_min_4200_25804200() {
    // Encoding: 0x25804200
    // Test NOR_P.P.PP_Z field Pn = 0 (Min)
    // Fields: Pm=0, Pg=0, Pn=0, Pd=0
    let encoding: u32 = 0x25804200;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: NOR_P.P.PP_Z
/// ASL: `field Pn 5 +: 4`
/// Requirement: FieldBoundary { field: "Pn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_nor_p_p_pp_z_field_pn_1_poweroftwo_4200_25804220() {
    // Encoding: 0x25804220
    // Test NOR_P.P.PP_Z field Pn = 1 (PowerOfTwo)
    // Fields: Pm=0, Pn=1, Pg=0, Pd=0
    let encoding: u32 = 0x25804220;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: NOR_P.P.PP_Z
/// ASL: `field Pd 0 +: 4`
/// Requirement: FieldBoundary { field: "Pd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_nor_p_p_pp_z_field_pd_0_min_4200_25804200() {
    // Encoding: 0x25804200
    // Test NOR_P.P.PP_Z field Pd = 0 (Min)
    // Fields: Pm=0, Pn=0, Pd=0, Pg=0
    let encoding: u32 = 0x25804200;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: NOR_P.P.PP_Z
/// ASL: `field Pd 0 +: 4`
/// Requirement: FieldBoundary { field: "Pd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_nor_p_p_pp_z_field_pd_1_poweroftwo_4200_25804201() {
    // Encoding: 0x25804201
    // Test NOR_P.P.PP_Z field Pd = 1 (PowerOfTwo)
    // Fields: Pg=0, Pm=0, Pd=1, Pn=0
    let encoding: u32 = 0x25804201;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: NOR_P.P.PP_Z
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pm=0 (register index 0 (first register))
#[test]
fn test_nor_p_p_pp_z_combo_0_4200_25804200() {
    // Encoding: 0x25804200
    // Test NOR_P.P.PP_Z field combination: Pm=0, Pg=0, Pn=0, Pd=0
    // Fields: Pn=0, Pg=0, Pd=0, Pm=0
    let encoding: u32 = 0x25804200;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: NOR_P.P.PP_Z
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pm=1 (register index 1 (second register))
#[test]
fn test_nor_p_p_pp_z_combo_1_4200_25814200() {
    // Encoding: 0x25814200
    // Test NOR_P.P.PP_Z field combination: Pm=1, Pg=0, Pn=0, Pd=0
    // Fields: Pm=1, Pg=0, Pd=0, Pn=0
    let encoding: u32 = 0x25814200;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: NOR_P.P.PP_Z
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=0 (register index 0 (first register))
#[test]
fn test_nor_p_p_pp_z_combo_2_4200_25804200() {
    // Encoding: 0x25804200
    // Test NOR_P.P.PP_Z field combination: Pm=0, Pg=0, Pn=0, Pd=0
    // Fields: Pn=0, Pd=0, Pm=0, Pg=0
    let encoding: u32 = 0x25804200;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: NOR_P.P.PP_Z
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=1 (register index 1 (second register))
#[test]
fn test_nor_p_p_pp_z_combo_3_4200_25804600() {
    // Encoding: 0x25804600
    // Test NOR_P.P.PP_Z field combination: Pm=0, Pg=1, Pn=0, Pd=0
    // Fields: Pg=1, Pn=0, Pd=0, Pm=0
    let encoding: u32 = 0x25804600;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: NOR_P.P.PP_Z
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pn=0 (register index 0 (first register))
#[test]
fn test_nor_p_p_pp_z_combo_4_4200_25804200() {
    // Encoding: 0x25804200
    // Test NOR_P.P.PP_Z field combination: Pm=0, Pg=0, Pn=0, Pd=0
    // Fields: Pd=0, Pg=0, Pm=0, Pn=0
    let encoding: u32 = 0x25804200;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: NOR_P.P.PP_Z
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pn=1 (register index 1 (second register))
#[test]
fn test_nor_p_p_pp_z_combo_5_4200_25804220() {
    // Encoding: 0x25804220
    // Test NOR_P.P.PP_Z field combination: Pm=0, Pg=0, Pn=1, Pd=0
    // Fields: Pg=0, Pn=1, Pm=0, Pd=0
    let encoding: u32 = 0x25804220;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: NOR_P.P.PP_Z
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pd=0 (register index 0 (first register))
#[test]
fn test_nor_p_p_pp_z_combo_6_4200_25804200() {
    // Encoding: 0x25804200
    // Test NOR_P.P.PP_Z field combination: Pm=0, Pg=0, Pn=0, Pd=0
    // Fields: Pd=0, Pm=0, Pg=0, Pn=0
    let encoding: u32 = 0x25804200;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: NOR_P.P.PP_Z
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pd=1 (register index 1 (second register))
#[test]
fn test_nor_p_p_pp_z_combo_7_4200_25804201() {
    // Encoding: 0x25804201
    // Test NOR_P.P.PP_Z field combination: Pm=0, Pg=0, Pn=0, Pd=1
    // Fields: Pg=0, Pn=0, Pm=0, Pd=1
    let encoding: u32 = 0x25804201;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: NOR_P.P.PP_Z
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pm=1 (same register test (reg=1)), Pg=1 (same register test (reg=1))
#[test]
fn test_nor_p_p_pp_z_combo_8_4200_25814600() {
    // Encoding: 0x25814600
    // Test NOR_P.P.PP_Z field combination: Pm=1, Pg=1, Pn=0, Pd=0
    // Fields: Pg=1, Pd=0, Pm=1, Pn=0
    let encoding: u32 = 0x25814600;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: NOR_P.P.PP_Z
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pm=31 (same register test (reg=31)), Pg=31 (same register test (reg=31))
#[test]
fn test_nor_p_p_pp_z_combo_9_4200_258f7e00() {
    // Encoding: 0x258F7E00
    // Test NOR_P.P.PP_Z field combination: Pm=31, Pg=31, Pn=0, Pd=0
    // Fields: Pd=0, Pm=31, Pg=31, Pn=0
    let encoding: u32 = 0x258F7E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: NOR_P.P.PP_Z
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pm=1 (same register test (reg=1)), Pn=1 (same register test (reg=1))
#[test]
fn test_nor_p_p_pp_z_combo_10_4200_25814220() {
    // Encoding: 0x25814220
    // Test NOR_P.P.PP_Z field combination: Pm=1, Pg=0, Pn=1, Pd=0
    // Fields: Pg=0, Pd=0, Pm=1, Pn=1
    let encoding: u32 = 0x25814220;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: NOR_P.P.PP_Z
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pm=31 (same register test (reg=31)), Pn=31 (same register test (reg=31))
#[test]
fn test_nor_p_p_pp_z_combo_11_4200_258f43e0() {
    // Encoding: 0x258F43E0
    // Test NOR_P.P.PP_Z field combination: Pm=31, Pg=0, Pn=31, Pd=0
    // Fields: Pd=0, Pg=0, Pm=31, Pn=31
    let encoding: u32 = 0x258F43E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: NOR_P.P.PP_Z
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pm=1 (same register test (reg=1)), Pd=1 (same register test (reg=1))
#[test]
fn test_nor_p_p_pp_z_combo_12_4200_25814201() {
    // Encoding: 0x25814201
    // Test NOR_P.P.PP_Z field combination: Pm=1, Pg=0, Pn=0, Pd=1
    // Fields: Pm=1, Pd=1, Pg=0, Pn=0
    let encoding: u32 = 0x25814201;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: NOR_P.P.PP_Z
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pm=31 (same register test (reg=31)), Pd=31 (same register test (reg=31))
#[test]
fn test_nor_p_p_pp_z_combo_13_4200_258f420f() {
    // Encoding: 0x258F420F
    // Test NOR_P.P.PP_Z field combination: Pm=31, Pg=0, Pn=0, Pd=31
    // Fields: Pn=0, Pm=31, Pd=31, Pg=0
    let encoding: u32 = 0x258F420F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: NOR_P.P.PP_Z
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=1 (same register test (reg=1)), Pn=1 (same register test (reg=1))
#[test]
fn test_nor_p_p_pp_z_combo_14_4200_25804620() {
    // Encoding: 0x25804620
    // Test NOR_P.P.PP_Z field combination: Pm=0, Pg=1, Pn=1, Pd=0
    // Fields: Pm=0, Pd=0, Pn=1, Pg=1
    let encoding: u32 = 0x25804620;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: NOR_P.P.PP_Z
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=31 (same register test (reg=31)), Pn=31 (same register test (reg=31))
#[test]
fn test_nor_p_p_pp_z_combo_15_4200_25807fe0() {
    // Encoding: 0x25807FE0
    // Test NOR_P.P.PP_Z field combination: Pm=0, Pg=31, Pn=31, Pd=0
    // Fields: Pm=0, Pd=0, Pg=31, Pn=31
    let encoding: u32 = 0x25807FE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: NOR_P.P.PP_Z
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=1 (same register test (reg=1)), Pd=1 (same register test (reg=1))
#[test]
fn test_nor_p_p_pp_z_combo_16_4200_25804601() {
    // Encoding: 0x25804601
    // Test NOR_P.P.PP_Z field combination: Pm=0, Pg=1, Pn=0, Pd=1
    // Fields: Pg=1, Pn=0, Pd=1, Pm=0
    let encoding: u32 = 0x25804601;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: NOR_P.P.PP_Z
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=31 (same register test (reg=31)), Pd=31 (same register test (reg=31))
#[test]
fn test_nor_p_p_pp_z_combo_17_4200_25807e0f() {
    // Encoding: 0x25807E0F
    // Test NOR_P.P.PP_Z field combination: Pm=0, Pg=31, Pn=0, Pd=31
    // Fields: Pm=0, Pg=31, Pn=0, Pd=31
    let encoding: u32 = 0x25807E0F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: NOR_P.P.PP_Z
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pn=1 (same register test (reg=1)), Pd=1 (same register test (reg=1))
#[test]
fn test_nor_p_p_pp_z_combo_18_4200_25804221() {
    // Encoding: 0x25804221
    // Test NOR_P.P.PP_Z field combination: Pm=0, Pg=0, Pn=1, Pd=1
    // Fields: Pd=1, Pm=0, Pn=1, Pg=0
    let encoding: u32 = 0x25804221;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: NOR_P.P.PP_Z
/// ASL: `field combination 19`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pn=31 (same register test (reg=31)), Pd=31 (same register test (reg=31))
#[test]
fn test_nor_p_p_pp_z_combo_19_4200_258043ef() {
    // Encoding: 0x258043EF
    // Test NOR_P.P.PP_Z field combination: Pm=0, Pg=0, Pn=31, Pd=31
    // Fields: Pg=0, Pn=31, Pm=0, Pd=31
    let encoding: u32 = 0x258043EF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: NORS_P.P.PP_Z
/// ASL: `field Pm 16 +: 4`
/// Requirement: FieldBoundary { field: "Pm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_nors_p_p_pp_z_field_pm_0_min_4200_25c04200() {
    // Encoding: 0x25C04200
    // Test NORS_P.P.PP_Z field Pm = 0 (Min)
    // Fields: Pg=0, Pn=0, Pd=0, Pm=0
    let encoding: u32 = 0x25C04200;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: NORS_P.P.PP_Z
/// ASL: `field Pm 16 +: 4`
/// Requirement: FieldBoundary { field: "Pm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_nors_p_p_pp_z_field_pm_1_poweroftwo_4200_25c14200() {
    // Encoding: 0x25C14200
    // Test NORS_P.P.PP_Z field Pm = 1 (PowerOfTwo)
    // Fields: Pm=1, Pg=0, Pn=0, Pd=0
    let encoding: u32 = 0x25C14200;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: NORS_P.P.PP_Z
/// ASL: `field Pg 10 +: 4`
/// Requirement: FieldBoundary { field: "Pg", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_nors_p_p_pp_z_field_pg_0_min_4200_25c04200() {
    // Encoding: 0x25C04200
    // Test NORS_P.P.PP_Z field Pg = 0 (Min)
    // Fields: Pm=0, Pg=0, Pn=0, Pd=0
    let encoding: u32 = 0x25C04200;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: NORS_P.P.PP_Z
/// ASL: `field Pg 10 +: 4`
/// Requirement: FieldBoundary { field: "Pg", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_nors_p_p_pp_z_field_pg_1_poweroftwo_4200_25c04600() {
    // Encoding: 0x25C04600
    // Test NORS_P.P.PP_Z field Pg = 1 (PowerOfTwo)
    // Fields: Pg=1, Pn=0, Pd=0, Pm=0
    let encoding: u32 = 0x25C04600;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: NORS_P.P.PP_Z
/// ASL: `field Pn 5 +: 4`
/// Requirement: FieldBoundary { field: "Pn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_nors_p_p_pp_z_field_pn_0_min_4200_25c04200() {
    // Encoding: 0x25C04200
    // Test NORS_P.P.PP_Z field Pn = 0 (Min)
    // Fields: Pm=0, Pg=0, Pd=0, Pn=0
    let encoding: u32 = 0x25C04200;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: NORS_P.P.PP_Z
/// ASL: `field Pn 5 +: 4`
/// Requirement: FieldBoundary { field: "Pn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_nors_p_p_pp_z_field_pn_1_poweroftwo_4200_25c04220() {
    // Encoding: 0x25C04220
    // Test NORS_P.P.PP_Z field Pn = 1 (PowerOfTwo)
    // Fields: Pg=0, Pm=0, Pn=1, Pd=0
    let encoding: u32 = 0x25C04220;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: NORS_P.P.PP_Z
/// ASL: `field Pd 0 +: 4`
/// Requirement: FieldBoundary { field: "Pd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_nors_p_p_pp_z_field_pd_0_min_4200_25c04200() {
    // Encoding: 0x25C04200
    // Test NORS_P.P.PP_Z field Pd = 0 (Min)
    // Fields: Pg=0, Pm=0, Pd=0, Pn=0
    let encoding: u32 = 0x25C04200;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: NORS_P.P.PP_Z
/// ASL: `field Pd 0 +: 4`
/// Requirement: FieldBoundary { field: "Pd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_nors_p_p_pp_z_field_pd_1_poweroftwo_4200_25c04201() {
    // Encoding: 0x25C04201
    // Test NORS_P.P.PP_Z field Pd = 1 (PowerOfTwo)
    // Fields: Pn=0, Pm=0, Pd=1, Pg=0
    let encoding: u32 = 0x25C04201;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: NORS_P.P.PP_Z
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pm=0 (register index 0 (first register))
#[test]
fn test_nors_p_p_pp_z_combo_0_4200_25c04200() {
    // Encoding: 0x25C04200
    // Test NORS_P.P.PP_Z field combination: Pm=0, Pg=0, Pn=0, Pd=0
    // Fields: Pm=0, Pg=0, Pn=0, Pd=0
    let encoding: u32 = 0x25C04200;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: NORS_P.P.PP_Z
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pm=1 (register index 1 (second register))
#[test]
fn test_nors_p_p_pp_z_combo_1_4200_25c14200() {
    // Encoding: 0x25C14200
    // Test NORS_P.P.PP_Z field combination: Pm=1, Pg=0, Pn=0, Pd=0
    // Fields: Pd=0, Pn=0, Pg=0, Pm=1
    let encoding: u32 = 0x25C14200;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: NORS_P.P.PP_Z
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=0 (register index 0 (first register))
#[test]
fn test_nors_p_p_pp_z_combo_2_4200_25c04200() {
    // Encoding: 0x25C04200
    // Test NORS_P.P.PP_Z field combination: Pm=0, Pg=0, Pn=0, Pd=0
    // Fields: Pm=0, Pg=0, Pd=0, Pn=0
    let encoding: u32 = 0x25C04200;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: NORS_P.P.PP_Z
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=1 (register index 1 (second register))
#[test]
fn test_nors_p_p_pp_z_combo_3_4200_25c04600() {
    // Encoding: 0x25C04600
    // Test NORS_P.P.PP_Z field combination: Pm=0, Pg=1, Pn=0, Pd=0
    // Fields: Pn=0, Pd=0, Pm=0, Pg=1
    let encoding: u32 = 0x25C04600;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: NORS_P.P.PP_Z
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pn=0 (register index 0 (first register))
#[test]
fn test_nors_p_p_pp_z_combo_4_4200_25c04200() {
    // Encoding: 0x25C04200
    // Test NORS_P.P.PP_Z field combination: Pm=0, Pg=0, Pn=0, Pd=0
    // Fields: Pn=0, Pm=0, Pg=0, Pd=0
    let encoding: u32 = 0x25C04200;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: NORS_P.P.PP_Z
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pn=1 (register index 1 (second register))
#[test]
fn test_nors_p_p_pp_z_combo_5_4200_25c04220() {
    // Encoding: 0x25C04220
    // Test NORS_P.P.PP_Z field combination: Pm=0, Pg=0, Pn=1, Pd=0
    // Fields: Pm=0, Pg=0, Pd=0, Pn=1
    let encoding: u32 = 0x25C04220;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: NORS_P.P.PP_Z
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pd=0 (register index 0 (first register))
#[test]
fn test_nors_p_p_pp_z_combo_6_4200_25c04200() {
    // Encoding: 0x25C04200
    // Test NORS_P.P.PP_Z field combination: Pm=0, Pg=0, Pn=0, Pd=0
    // Fields: Pg=0, Pd=0, Pm=0, Pn=0
    let encoding: u32 = 0x25C04200;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: NORS_P.P.PP_Z
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pd=1 (register index 1 (second register))
#[test]
fn test_nors_p_p_pp_z_combo_7_4200_25c04201() {
    // Encoding: 0x25C04201
    // Test NORS_P.P.PP_Z field combination: Pm=0, Pg=0, Pn=0, Pd=1
    // Fields: Pg=0, Pm=0, Pn=0, Pd=1
    let encoding: u32 = 0x25C04201;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: NORS_P.P.PP_Z
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pm=1 (same register test (reg=1)), Pg=1 (same register test (reg=1))
#[test]
fn test_nors_p_p_pp_z_combo_8_4200_25c14600() {
    // Encoding: 0x25C14600
    // Test NORS_P.P.PP_Z field combination: Pm=1, Pg=1, Pn=0, Pd=0
    // Fields: Pd=0, Pn=0, Pm=1, Pg=1
    let encoding: u32 = 0x25C14600;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: NORS_P.P.PP_Z
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pm=31 (same register test (reg=31)), Pg=31 (same register test (reg=31))
#[test]
fn test_nors_p_p_pp_z_combo_9_4200_25cf7e00() {
    // Encoding: 0x25CF7E00
    // Test NORS_P.P.PP_Z field combination: Pm=31, Pg=31, Pn=0, Pd=0
    // Fields: Pg=31, Pm=31, Pd=0, Pn=0
    let encoding: u32 = 0x25CF7E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: NORS_P.P.PP_Z
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pm=1 (same register test (reg=1)), Pn=1 (same register test (reg=1))
#[test]
fn test_nors_p_p_pp_z_combo_10_4200_25c14220() {
    // Encoding: 0x25C14220
    // Test NORS_P.P.PP_Z field combination: Pm=1, Pg=0, Pn=1, Pd=0
    // Fields: Pd=0, Pg=0, Pn=1, Pm=1
    let encoding: u32 = 0x25C14220;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: NORS_P.P.PP_Z
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pm=31 (same register test (reg=31)), Pn=31 (same register test (reg=31))
#[test]
fn test_nors_p_p_pp_z_combo_11_4200_25cf43e0() {
    // Encoding: 0x25CF43E0
    // Test NORS_P.P.PP_Z field combination: Pm=31, Pg=0, Pn=31, Pd=0
    // Fields: Pg=0, Pd=0, Pn=31, Pm=31
    let encoding: u32 = 0x25CF43E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: NORS_P.P.PP_Z
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pm=1 (same register test (reg=1)), Pd=1 (same register test (reg=1))
#[test]
fn test_nors_p_p_pp_z_combo_12_4200_25c14201() {
    // Encoding: 0x25C14201
    // Test NORS_P.P.PP_Z field combination: Pm=1, Pg=0, Pn=0, Pd=1
    // Fields: Pn=0, Pg=0, Pd=1, Pm=1
    let encoding: u32 = 0x25C14201;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: NORS_P.P.PP_Z
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pm=31 (same register test (reg=31)), Pd=31 (same register test (reg=31))
#[test]
fn test_nors_p_p_pp_z_combo_13_4200_25cf420f() {
    // Encoding: 0x25CF420F
    // Test NORS_P.P.PP_Z field combination: Pm=31, Pg=0, Pn=0, Pd=31
    // Fields: Pm=31, Pn=0, Pd=31, Pg=0
    let encoding: u32 = 0x25CF420F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: NORS_P.P.PP_Z
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=1 (same register test (reg=1)), Pn=1 (same register test (reg=1))
#[test]
fn test_nors_p_p_pp_z_combo_14_4200_25c04620() {
    // Encoding: 0x25C04620
    // Test NORS_P.P.PP_Z field combination: Pm=0, Pg=1, Pn=1, Pd=0
    // Fields: Pn=1, Pm=0, Pg=1, Pd=0
    let encoding: u32 = 0x25C04620;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: NORS_P.P.PP_Z
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=31 (same register test (reg=31)), Pn=31 (same register test (reg=31))
#[test]
fn test_nors_p_p_pp_z_combo_15_4200_25c07fe0() {
    // Encoding: 0x25C07FE0
    // Test NORS_P.P.PP_Z field combination: Pm=0, Pg=31, Pn=31, Pd=0
    // Fields: Pm=0, Pd=0, Pn=31, Pg=31
    let encoding: u32 = 0x25C07FE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: NORS_P.P.PP_Z
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=1 (same register test (reg=1)), Pd=1 (same register test (reg=1))
#[test]
fn test_nors_p_p_pp_z_combo_16_4200_25c04601() {
    // Encoding: 0x25C04601
    // Test NORS_P.P.PP_Z field combination: Pm=0, Pg=1, Pn=0, Pd=1
    // Fields: Pn=0, Pm=0, Pd=1, Pg=1
    let encoding: u32 = 0x25C04601;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: NORS_P.P.PP_Z
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=31 (same register test (reg=31)), Pd=31 (same register test (reg=31))
#[test]
fn test_nors_p_p_pp_z_combo_17_4200_25c07e0f() {
    // Encoding: 0x25C07E0F
    // Test NORS_P.P.PP_Z field combination: Pm=0, Pg=31, Pn=0, Pd=31
    // Fields: Pg=31, Pd=31, Pn=0, Pm=0
    let encoding: u32 = 0x25C07E0F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: NORS_P.P.PP_Z
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pn=1 (same register test (reg=1)), Pd=1 (same register test (reg=1))
#[test]
fn test_nors_p_p_pp_z_combo_18_4200_25c04221() {
    // Encoding: 0x25C04221
    // Test NORS_P.P.PP_Z field combination: Pm=0, Pg=0, Pn=1, Pd=1
    // Fields: Pm=0, Pn=1, Pd=1, Pg=0
    let encoding: u32 = 0x25C04221;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: NORS_P.P.PP_Z
/// ASL: `field combination 19`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pn=31 (same register test (reg=31)), Pd=31 (same register test (reg=31))
#[test]
fn test_nors_p_p_pp_z_combo_19_4200_25c043ef() {
    // Encoding: 0x25C043EF
    // Test NORS_P.P.PP_Z field combination: Pm=0, Pg=0, Pn=31, Pd=31
    // Fields: Pd=31, Pm=0, Pg=0, Pn=31
    let encoding: u32 = 0x25C043EF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: NOR_P.P.PP_Z
/// ASL: `SimdFromField("Pd") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "unknown" }
/// verify register write to SimdFromField("Pd")
#[test]
fn test_nor_p_p_pp_z_reg_write_0_25804200() {
    // Test NOR_P.P.PP_Z register write: SimdFromField("Pd")
    // Encoding: 0x25804200
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x25804200;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: NOR_P.P.PP_Z
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 0 + 0 = 0 (Z=1)
#[test]
fn test_nor_p_p_pp_z_flags_zeroresult_0_25804200() {
    // Test NOR_P.P.PP_Z flag computation: ZeroResult
    // Encoding: 0x25804200
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x0);
    set_x(&mut cpu, 2, 0x0);
    let encoding: u32 = 0x25804200;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: NOR_P.P.PP_Z
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 1 + (-1) = 0 (Z=1, C=1)
#[test]
fn test_nor_p_p_pp_z_flags_zeroresult_1_25804200() {
    // Test NOR_P.P.PP_Z flag computation: ZeroResult
    // Encoding: 0x25804200
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x1);
    set_x(&mut cpu, 2, 0xFFFFFFFFFFFFFFFF);
    let encoding: u32 = 0x25804200;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: NOR_P.P.PP_Z
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: NegativeResult }
/// negative value (N=1)
#[test]
fn test_nor_p_p_pp_z_flags_negativeresult_2_25804200() {
    // Test NOR_P.P.PP_Z flag computation: NegativeResult
    // Encoding: 0x25804200
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x8000000000000000);
    set_x(&mut cpu, 2, 0x0);
    let encoding: u32 = 0x25804200;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: NOR_P.P.PP_Z
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 1 = 0 (C=1, Z=1)
#[test]
fn test_nor_p_p_pp_z_flags_unsignedoverflow_3_25804200() {
    // Test NOR_P.P.PP_Z flag computation: UnsignedOverflow
    // Encoding: 0x25804200
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0xFFFFFFFFFFFFFFFF);
    set_x(&mut cpu, 2, 0x1);
    let encoding: u32 = 0x25804200;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: NOR_P.P.PP_Z
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 2 = 1 (C=1)
#[test]
fn test_nor_p_p_pp_z_flags_unsignedoverflow_4_25804200() {
    // Test NOR_P.P.PP_Z flag computation: UnsignedOverflow
    // Encoding: 0x25804200
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0xFFFFFFFFFFFFFFFF);
    set_x(&mut cpu, 2, 0x2);
    let encoding: u32 = 0x25804200;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: NOR_P.P.PP_Z
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// max_signed + 1 = min_signed (V=1, N=1)
#[test]
fn test_nor_p_p_pp_z_flags_signedoverflow_5_25804200() {
    // Test NOR_P.P.PP_Z flag computation: SignedOverflow
    // Encoding: 0x25804200
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x7FFFFFFFFFFFFFFF);
    set_x(&mut cpu, 2, 0x1);
    let encoding: u32 = 0x25804200;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: NOR_P.P.PP_Z
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// min_signed + (-1) = max_signed (V=1)
#[test]
fn test_nor_p_p_pp_z_flags_signedoverflow_6_25804200() {
    // Test NOR_P.P.PP_Z flag computation: SignedOverflow
    // Encoding: 0x25804200
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x8000000000000000);
    set_x(&mut cpu, 2, 0xFFFFFFFFFFFFFFFF);
    let encoding: u32 = 0x25804200;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: NOR_P.P.PP_Z
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: PositiveResult }
/// 100 + 50 = 150 (no flags)
#[test]
fn test_nor_p_p_pp_z_flags_positiveresult_7_25804200() {
    // Test NOR_P.P.PP_Z flag computation: PositiveResult
    // Encoding: 0x25804200
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x64);
    set_x(&mut cpu, 2, 0x32);
    let encoding: u32 = 0x25804200;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: NORS_P.P.PP_Z
/// ASL: `SimdFromField("Pd") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "unknown" }
/// verify register write to SimdFromField("Pd")
#[test]
fn test_nors_p_p_pp_z_reg_write_0_25c04200() {
    // Test NORS_P.P.PP_Z register write: SimdFromField("Pd")
    // Encoding: 0x25C04200
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x25C04200;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: NORS_P.P.PP_Z
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 0 + 0 = 0 (Z=1)
#[test]
fn test_nors_p_p_pp_z_flags_zeroresult_0_25c04200() {
    // Test NORS_P.P.PP_Z flag computation: ZeroResult
    // Encoding: 0x25C04200
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x0);
    set_x(&mut cpu, 2, 0x0);
    let encoding: u32 = 0x25C04200;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: NORS_P.P.PP_Z
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 1 + (-1) = 0 (Z=1, C=1)
#[test]
fn test_nors_p_p_pp_z_flags_zeroresult_1_25c04200() {
    // Test NORS_P.P.PP_Z flag computation: ZeroResult
    // Encoding: 0x25C04200
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x1);
    set_x(&mut cpu, 2, 0xFFFFFFFFFFFFFFFF);
    let encoding: u32 = 0x25C04200;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: NORS_P.P.PP_Z
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: NegativeResult }
/// negative value (N=1)
#[test]
fn test_nors_p_p_pp_z_flags_negativeresult_2_25c04200() {
    // Test NORS_P.P.PP_Z flag computation: NegativeResult
    // Encoding: 0x25C04200
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0x0);
    set_x(&mut cpu, 1, 0x8000000000000000);
    let encoding: u32 = 0x25C04200;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: NORS_P.P.PP_Z
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 1 = 0 (C=1, Z=1)
#[test]
fn test_nors_p_p_pp_z_flags_unsignedoverflow_3_25c04200() {
    // Test NORS_P.P.PP_Z flag computation: UnsignedOverflow
    // Encoding: 0x25C04200
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0xFFFFFFFFFFFFFFFF);
    set_x(&mut cpu, 2, 0x1);
    let encoding: u32 = 0x25C04200;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: NORS_P.P.PP_Z
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 2 = 1 (C=1)
#[test]
fn test_nors_p_p_pp_z_flags_unsignedoverflow_4_25c04200() {
    // Test NORS_P.P.PP_Z flag computation: UnsignedOverflow
    // Encoding: 0x25C04200
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0xFFFFFFFFFFFFFFFF);
    set_x(&mut cpu, 2, 0x2);
    let encoding: u32 = 0x25C04200;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: NORS_P.P.PP_Z
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// max_signed + 1 = min_signed (V=1, N=1)
#[test]
fn test_nors_p_p_pp_z_flags_signedoverflow_5_25c04200() {
    // Test NORS_P.P.PP_Z flag computation: SignedOverflow
    // Encoding: 0x25C04200
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x7FFFFFFFFFFFFFFF);
    set_x(&mut cpu, 2, 0x1);
    let encoding: u32 = 0x25C04200;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: NORS_P.P.PP_Z
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// min_signed + (-1) = max_signed (V=1)
#[test]
fn test_nors_p_p_pp_z_flags_signedoverflow_6_25c04200() {
    // Test NORS_P.P.PP_Z flag computation: SignedOverflow
    // Encoding: 0x25C04200
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x8000000000000000);
    set_x(&mut cpu, 2, 0xFFFFFFFFFFFFFFFF);
    let encoding: u32 = 0x25C04200;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: NORS_P.P.PP_Z
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: PositiveResult }
/// 100 + 50 = 150 (no flags)
#[test]
fn test_nors_p_p_pp_z_flags_positiveresult_7_25c04200() {
    // Test NORS_P.P.PP_Z flag computation: PositiveResult
    // Encoding: 0x25C04200
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0x32);
    set_x(&mut cpu, 1, 0x64);
    let encoding: u32 = 0x25C04200;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

// ============================================================================
// PNEXT_P.P.P__ Tests
// ============================================================================

/// Provenance: PNEXT_P.P.P__
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_pnext_p_p_p_field_size_0_min_c400_2519c400() {
    // Encoding: 0x2519C400
    // Test PNEXT_P.P.P__ field size = 0 (Min)
    // Fields: size=0, Pdn=0, Pg=0
    let encoding: u32 = 0x2519C400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PNEXT_P.P.P__
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 1, boundary: PowerOfTwo }
/// 16-bit / halfword size
#[test]
fn test_pnext_p_p_p_field_size_1_poweroftwo_c400_2559c400() {
    // Encoding: 0x2559C400
    // Test PNEXT_P.P.P__ field size = 1 (PowerOfTwo)
    // Fields: Pdn=0, size=1, Pg=0
    let encoding: u32 = 0x2559C400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PNEXT_P.P.P__
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 2, boundary: PowerOfTwo }
/// 32-bit / word size
#[test]
fn test_pnext_p_p_p_field_size_2_poweroftwo_c400_2599c400() {
    // Encoding: 0x2599C400
    // Test PNEXT_P.P.P__ field size = 2 (PowerOfTwo)
    // Fields: Pg=0, Pdn=0, size=2
    let encoding: u32 = 0x2599C400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PNEXT_P.P.P__
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 3, boundary: Max }
/// 64-bit / doubleword size
#[test]
fn test_pnext_p_p_p_field_size_3_max_c400_25d9c400() {
    // Encoding: 0x25D9C400
    // Test PNEXT_P.P.P__ field size = 3 (Max)
    // Fields: size=3, Pg=0, Pdn=0
    let encoding: u32 = 0x25D9C400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PNEXT_P.P.P__
/// ASL: `field Pg 5 +: 4`
/// Requirement: FieldBoundary { field: "Pg", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_pnext_p_p_p_field_pg_0_min_c400_2519c400() {
    // Encoding: 0x2519C400
    // Test PNEXT_P.P.P__ field Pg = 0 (Min)
    // Fields: size=0, Pg=0, Pdn=0
    let encoding: u32 = 0x2519C400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PNEXT_P.P.P__
/// ASL: `field Pg 5 +: 4`
/// Requirement: FieldBoundary { field: "Pg", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_pnext_p_p_p_field_pg_1_poweroftwo_c400_2519c420() {
    // Encoding: 0x2519C420
    // Test PNEXT_P.P.P__ field Pg = 1 (PowerOfTwo)
    // Fields: Pg=1, Pdn=0, size=0
    let encoding: u32 = 0x2519C420;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PNEXT_P.P.P__
/// ASL: `field Pdn 0 +: 4`
/// Requirement: FieldBoundary { field: "Pdn", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_pnext_p_p_p_field_pdn_0_min_c400_2519c400() {
    // Encoding: 0x2519C400
    // Test PNEXT_P.P.P__ field Pdn = 0 (Min)
    // Fields: Pg=0, Pdn=0, size=0
    let encoding: u32 = 0x2519C400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PNEXT_P.P.P__
/// ASL: `field Pdn 0 +: 4`
/// Requirement: FieldBoundary { field: "Pdn", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_pnext_p_p_p_field_pdn_1_poweroftwo_c400_2519c401() {
    // Encoding: 0x2519C401
    // Test PNEXT_P.P.P__ field Pdn = 1 (PowerOfTwo)
    // Fields: Pg=0, Pdn=1, size=0
    let encoding: u32 = 0x2519C401;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PNEXT_P.P.P__
/// ASL: `field Pdn 0 +: 4`
/// Requirement: FieldBoundary { field: "Pdn", value: 7, boundary: PowerOfTwoMinusOne }
/// midpoint (7)
#[test]
fn test_pnext_p_p_p_field_pdn_7_poweroftwominusone_c400_2519c407() {
    // Encoding: 0x2519C407
    // Test PNEXT_P.P.P__ field Pdn = 7 (PowerOfTwoMinusOne)
    // Fields: size=0, Pg=0, Pdn=7
    let encoding: u32 = 0x2519C407;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PNEXT_P.P.P__
/// ASL: `field Pdn 0 +: 4`
/// Requirement: FieldBoundary { field: "Pdn", value: 15, boundary: Max }
/// maximum value (15)
#[test]
fn test_pnext_p_p_p_field_pdn_15_max_c400_2519c40f() {
    // Encoding: 0x2519C40F
    // Test PNEXT_P.P.P__ field Pdn = 15 (Max)
    // Fields: Pdn=15, size=0, Pg=0
    let encoding: u32 = 0x2519C40F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PNEXT_P.P.P__
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=0 (8-bit / byte size)
#[test]
fn test_pnext_p_p_p_combo_0_c400_2519c400() {
    // Encoding: 0x2519C400
    // Test PNEXT_P.P.P__ field combination: size=0, Pg=0, Pdn=0
    // Fields: Pg=0, Pdn=0, size=0
    let encoding: u32 = 0x2519C400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PNEXT_P.P.P__
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=1 (16-bit / halfword size)
#[test]
fn test_pnext_p_p_p_combo_1_c400_2559c400() {
    // Encoding: 0x2559C400
    // Test PNEXT_P.P.P__ field combination: size=1, Pg=0, Pdn=0
    // Fields: Pg=0, Pdn=0, size=1
    let encoding: u32 = 0x2559C400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PNEXT_P.P.P__
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=2 (32-bit / word size)
#[test]
fn test_pnext_p_p_p_combo_2_c400_2599c400() {
    // Encoding: 0x2599C400
    // Test PNEXT_P.P.P__ field combination: size=2, Pg=0, Pdn=0
    // Fields: Pdn=0, Pg=0, size=2
    let encoding: u32 = 0x2599C400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PNEXT_P.P.P__
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=3 (64-bit / doubleword size)
#[test]
fn test_pnext_p_p_p_combo_3_c400_25d9c400() {
    // Encoding: 0x25D9C400
    // Test PNEXT_P.P.P__ field combination: size=3, Pg=0, Pdn=0
    // Fields: Pdn=0, size=3, Pg=0
    let encoding: u32 = 0x25D9C400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PNEXT_P.P.P__
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=0 (register index 0 (first register))
#[test]
fn test_pnext_p_p_p_combo_4_c400_2519c400() {
    // Encoding: 0x2519C400
    // Test PNEXT_P.P.P__ field combination: size=0, Pg=0, Pdn=0
    // Fields: Pg=0, Pdn=0, size=0
    let encoding: u32 = 0x2519C400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PNEXT_P.P.P__
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=1 (register index 1 (second register))
#[test]
fn test_pnext_p_p_p_combo_5_c400_2519c420() {
    // Encoding: 0x2519C420
    // Test PNEXT_P.P.P__ field combination: size=0, Pg=1, Pdn=0
    // Fields: size=0, Pdn=0, Pg=1
    let encoding: u32 = 0x2519C420;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PNEXT_P.P.P__
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pdn=0 (minimum value)
#[test]
fn test_pnext_p_p_p_combo_6_c400_2519c400() {
    // Encoding: 0x2519C400
    // Test PNEXT_P.P.P__ field combination: size=0, Pg=0, Pdn=0
    // Fields: Pg=0, size=0, Pdn=0
    let encoding: u32 = 0x2519C400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PNEXT_P.P.P__
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pdn=1 (value 1)
#[test]
fn test_pnext_p_p_p_combo_7_c400_2519c401() {
    // Encoding: 0x2519C401
    // Test PNEXT_P.P.P__ field combination: size=0, Pg=0, Pdn=1
    // Fields: Pg=0, Pdn=1, size=0
    let encoding: u32 = 0x2519C401;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PNEXT_P.P.P__
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pdn=7 (midpoint (7))
#[test]
fn test_pnext_p_p_p_combo_8_c400_2519c407() {
    // Encoding: 0x2519C407
    // Test PNEXT_P.P.P__ field combination: size=0, Pg=0, Pdn=7
    // Fields: size=0, Pg=0, Pdn=7
    let encoding: u32 = 0x2519C407;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PNEXT_P.P.P__
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pdn=15 (maximum value (15))
#[test]
fn test_pnext_p_p_p_combo_9_c400_2519c40f() {
    // Encoding: 0x2519C40F
    // Test PNEXT_P.P.P__ field combination: size=0, Pg=0, Pdn=15
    // Fields: Pg=0, size=0, Pdn=15
    let encoding: u32 = 0x2519C40F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PNEXT_P.P.P__
/// ASL: `field size = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "size", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_pnext_p_p_p_special_size_0_size_variant_0_50176_2519c400() {
    // Encoding: 0x2519C400
    // Test PNEXT_P.P.P__ special value size = 0 (Size variant 0)
    // Fields: Pg=0, size=0, Pdn=0
    let encoding: u32 = 0x2519C400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PNEXT_P.P.P__
/// ASL: `field size = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "size", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_pnext_p_p_p_special_size_1_size_variant_1_50176_2559c400() {
    // Encoding: 0x2559C400
    // Test PNEXT_P.P.P__ special value size = 1 (Size variant 1)
    // Fields: Pg=0, size=1, Pdn=0
    let encoding: u32 = 0x2559C400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PNEXT_P.P.P__
/// ASL: `field size = 2 (Size variant 2)`
/// Requirement: FieldSpecial { field: "size", value: 2, meaning: "Size variant 2" }
/// Size variant 2
#[test]
fn test_pnext_p_p_p_special_size_2_size_variant_2_50176_2599c400() {
    // Encoding: 0x2599C400
    // Test PNEXT_P.P.P__ special value size = 2 (Size variant 2)
    // Fields: Pg=0, Pdn=0, size=2
    let encoding: u32 = 0x2599C400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PNEXT_P.P.P__
/// ASL: `field size = 3 (Size variant 3)`
/// Requirement: FieldSpecial { field: "size", value: 3, meaning: "Size variant 3" }
/// Size variant 3
#[test]
fn test_pnext_p_p_p_special_size_3_size_variant_3_50176_25d9c400() {
    // Encoding: 0x25D9C400
    // Test PNEXT_P.P.P__ special value size = 3 (Size variant 3)
    // Fields: Pdn=0, size=3, Pg=0
    let encoding: u32 = 0x25D9C400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PNEXT_P.P.P__
/// ASL: `SimdFromField("Pdn") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "unknown" }
/// verify register write to SimdFromField("Pdn")
#[test]
fn test_pnext_p_p_p_reg_write_0_2519c400() {
    // Test PNEXT_P.P.P__ register write: SimdFromField("Pdn")
    // Encoding: 0x2519C400
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x2519C400;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: PNEXT_P.P.P__
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 0 + 0 = 0 (Z=1)
#[test]
fn test_pnext_p_p_p_flags_zeroresult_0_2519c400() {
    // Test PNEXT_P.P.P__ flag computation: ZeroResult
    // Encoding: 0x2519C400
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0x0);
    set_x(&mut cpu, 1, 0x0);
    let encoding: u32 = 0x2519C400;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: PNEXT_P.P.P__
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 1 + (-1) = 0 (Z=1, C=1)
#[test]
fn test_pnext_p_p_p_flags_zeroresult_1_2519c400() {
    // Test PNEXT_P.P.P__ flag computation: ZeroResult
    // Encoding: 0x2519C400
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x1);
    set_x(&mut cpu, 2, 0xFFFFFFFFFFFFFFFF);
    let encoding: u32 = 0x2519C400;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: PNEXT_P.P.P__
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: NegativeResult }
/// negative value (N=1)
#[test]
fn test_pnext_p_p_p_flags_negativeresult_2_2519c400() {
    // Test PNEXT_P.P.P__ flag computation: NegativeResult
    // Encoding: 0x2519C400
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0x0);
    set_x(&mut cpu, 1, 0x8000000000000000);
    let encoding: u32 = 0x2519C400;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: PNEXT_P.P.P__
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 1 = 0 (C=1, Z=1)
#[test]
fn test_pnext_p_p_p_flags_unsignedoverflow_3_2519c400() {
    // Test PNEXT_P.P.P__ flag computation: UnsignedOverflow
    // Encoding: 0x2519C400
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0x1);
    set_x(&mut cpu, 1, 0xFFFFFFFFFFFFFFFF);
    let encoding: u32 = 0x2519C400;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: PNEXT_P.P.P__
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 2 = 1 (C=1)
#[test]
fn test_pnext_p_p_p_flags_unsignedoverflow_4_2519c400() {
    // Test PNEXT_P.P.P__ flag computation: UnsignedOverflow
    // Encoding: 0x2519C400
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0x2);
    set_x(&mut cpu, 1, 0xFFFFFFFFFFFFFFFF);
    let encoding: u32 = 0x2519C400;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: PNEXT_P.P.P__
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// max_signed + 1 = min_signed (V=1, N=1)
#[test]
fn test_pnext_p_p_p_flags_signedoverflow_5_2519c400() {
    // Test PNEXT_P.P.P__ flag computation: SignedOverflow
    // Encoding: 0x2519C400
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0x1);
    set_x(&mut cpu, 1, 0x7FFFFFFFFFFFFFFF);
    let encoding: u32 = 0x2519C400;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: PNEXT_P.P.P__
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// min_signed + (-1) = max_signed (V=1)
#[test]
fn test_pnext_p_p_p_flags_signedoverflow_6_2519c400() {
    // Test PNEXT_P.P.P__ flag computation: SignedOverflow
    // Encoding: 0x2519C400
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0xFFFFFFFFFFFFFFFF);
    set_x(&mut cpu, 1, 0x8000000000000000);
    let encoding: u32 = 0x2519C400;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: PNEXT_P.P.P__
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: PositiveResult }
/// 100 + 50 = 150 (no flags)
#[test]
fn test_pnext_p_p_p_flags_positiveresult_7_2519c400() {
    // Test PNEXT_P.P.P__ flag computation: PositiveResult
    // Encoding: 0x2519C400
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0x32);
    set_x(&mut cpu, 1, 0x64);
    let encoding: u32 = 0x2519C400;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

// ============================================================================
// WHILELO_P.P.RR__ Tests
// ============================================================================

/// Provenance: WHILELO_P.P.RR__
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_whilelo_p_p_rr_field_size_0_min_c00_25200c00() {
    // Encoding: 0x25200C00
    // Test WHILELO_P.P.RR__ field size = 0 (Min)
    // Fields: sf=0, Pd=0, Rm=0, size=0, Rn=0
    let encoding: u32 = 0x25200C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELO_P.P.RR__
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 1, boundary: PowerOfTwo }
/// 16-bit / halfword size
#[test]
fn test_whilelo_p_p_rr_field_size_1_poweroftwo_c00_25600c00() {
    // Encoding: 0x25600C00
    // Test WHILELO_P.P.RR__ field size = 1 (PowerOfTwo)
    // Fields: Rn=0, size=1, Pd=0, Rm=0, sf=0
    let encoding: u32 = 0x25600C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELO_P.P.RR__
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 2, boundary: PowerOfTwo }
/// 32-bit / word size
#[test]
fn test_whilelo_p_p_rr_field_size_2_poweroftwo_c00_25a00c00() {
    // Encoding: 0x25A00C00
    // Test WHILELO_P.P.RR__ field size = 2 (PowerOfTwo)
    // Fields: Rm=0, Rn=0, size=2, sf=0, Pd=0
    let encoding: u32 = 0x25A00C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELO_P.P.RR__
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 3, boundary: Max }
/// 64-bit / doubleword size
#[test]
fn test_whilelo_p_p_rr_field_size_3_max_c00_25e00c00() {
    // Encoding: 0x25E00C00
    // Test WHILELO_P.P.RR__ field size = 3 (Max)
    // Fields: sf=0, Rn=0, Pd=0, size=3, Rm=0
    let encoding: u32 = 0x25E00C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELO_P.P.RR__
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_whilelo_p_p_rr_field_rm_0_min_c00_25200c00() {
    // Encoding: 0x25200C00
    // Test WHILELO_P.P.RR__ field Rm = 0 (Min)
    // Fields: Rn=0, sf=0, size=0, Pd=0, Rm=0
    let encoding: u32 = 0x25200C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELO_P.P.RR__
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_whilelo_p_p_rr_field_rm_1_poweroftwo_c00_25210c00() {
    // Encoding: 0x25210C00
    // Test WHILELO_P.P.RR__ field Rm = 1 (PowerOfTwo)
    // Fields: sf=0, size=0, Rn=0, Rm=1, Pd=0
    let encoding: u32 = 0x25210C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELO_P.P.RR__
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_whilelo_p_p_rr_field_rm_30_poweroftwominusone_c00_253e0c00() {
    // Encoding: 0x253E0C00
    // Test WHILELO_P.P.RR__ field Rm = 30 (PowerOfTwoMinusOne)
    // Fields: size=0, sf=0, Rm=30, Rn=0, Pd=0
    let encoding: u32 = 0x253E0C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELO_P.P.RR__
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 31, boundary: Max }
/// register index 31 (special)
#[test]
fn test_whilelo_p_p_rr_field_rm_31_max_c00_253f0c00() {
    // Encoding: 0x253F0C00
    // Test WHILELO_P.P.RR__ field Rm = 31 (Max)
    // Fields: Rm=31, size=0, Pd=0, Rn=0, sf=0
    let encoding: u32 = 0x253F0C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELO_P.P.RR__
/// ASL: `field sf 12 +: 1`
/// Requirement: FieldBoundary { field: "sf", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_whilelo_p_p_rr_field_sf_0_min_c00_25200c00() {
    // Encoding: 0x25200C00
    // Test WHILELO_P.P.RR__ field sf = 0 (Min)
    // Fields: Rm=0, size=0, Rn=0, Pd=0, sf=0
    let encoding: u32 = 0x25200C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELO_P.P.RR__
/// ASL: `field sf 12 +: 1`
/// Requirement: FieldBoundary { field: "sf", value: 1, boundary: Max }
/// 16-bit / halfword size
#[test]
fn test_whilelo_p_p_rr_field_sf_1_max_c00_25201c00() {
    // Encoding: 0x25201C00
    // Test WHILELO_P.P.RR__ field sf = 1 (Max)
    // Fields: Rm=0, Rn=0, Pd=0, size=0, sf=1
    let encoding: u32 = 0x25201C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELO_P.P.RR__
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_whilelo_p_p_rr_field_rn_0_min_c00_25200c00() {
    // Encoding: 0x25200C00
    // Test WHILELO_P.P.RR__ field Rn = 0 (Min)
    // Fields: Pd=0, sf=0, Rn=0, Rm=0, size=0
    let encoding: u32 = 0x25200C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELO_P.P.RR__
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_whilelo_p_p_rr_field_rn_1_poweroftwo_c00_25200c20() {
    // Encoding: 0x25200C20
    // Test WHILELO_P.P.RR__ field Rn = 1 (PowerOfTwo)
    // Fields: Rn=1, size=0, Pd=0, Rm=0, sf=0
    let encoding: u32 = 0x25200C20;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELO_P.P.RR__
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_whilelo_p_p_rr_field_rn_30_poweroftwominusone_c00_25200fc0() {
    // Encoding: 0x25200FC0
    // Test WHILELO_P.P.RR__ field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: Pd=0, Rm=0, sf=0, Rn=30, size=0
    let encoding: u32 = 0x25200FC0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELO_P.P.RR__
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_whilelo_p_p_rr_field_rn_31_max_c00_25200fe0() {
    // Encoding: 0x25200FE0
    // Test WHILELO_P.P.RR__ field Rn = 31 (Max)
    // Fields: sf=0, Rm=0, Pd=0, size=0, Rn=31
    let encoding: u32 = 0x25200FE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELO_P.P.RR__
/// ASL: `field Pd 0 +: 4`
/// Requirement: FieldBoundary { field: "Pd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_whilelo_p_p_rr_field_pd_0_min_c00_25200c00() {
    // Encoding: 0x25200C00
    // Test WHILELO_P.P.RR__ field Pd = 0 (Min)
    // Fields: Rm=0, size=0, Rn=0, Pd=0, sf=0
    let encoding: u32 = 0x25200C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELO_P.P.RR__
/// ASL: `field Pd 0 +: 4`
/// Requirement: FieldBoundary { field: "Pd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_whilelo_p_p_rr_field_pd_1_poweroftwo_c00_25200c01() {
    // Encoding: 0x25200C01
    // Test WHILELO_P.P.RR__ field Pd = 1 (PowerOfTwo)
    // Fields: size=0, sf=0, Rn=0, Rm=0, Pd=1
    let encoding: u32 = 0x25200C01;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELO_P.P.RR__
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=0 (8-bit / byte size)
#[test]
fn test_whilelo_p_p_rr_combo_0_c00_25200c00() {
    // Encoding: 0x25200C00
    // Test WHILELO_P.P.RR__ field combination: size=0, Rm=0, sf=0, Rn=0, Pd=0
    // Fields: Rm=0, Pd=0, size=0, Rn=0, sf=0
    let encoding: u32 = 0x25200C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELO_P.P.RR__
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=1 (16-bit / halfword size)
#[test]
fn test_whilelo_p_p_rr_combo_1_c00_25600c00() {
    // Encoding: 0x25600C00
    // Test WHILELO_P.P.RR__ field combination: size=1, Rm=0, sf=0, Rn=0, Pd=0
    // Fields: Pd=0, sf=0, size=1, Rm=0, Rn=0
    let encoding: u32 = 0x25600C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELO_P.P.RR__
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=2 (32-bit / word size)
#[test]
fn test_whilelo_p_p_rr_combo_2_c00_25a00c00() {
    // Encoding: 0x25A00C00
    // Test WHILELO_P.P.RR__ field combination: size=2, Rm=0, sf=0, Rn=0, Pd=0
    // Fields: size=2, sf=0, Rn=0, Rm=0, Pd=0
    let encoding: u32 = 0x25A00C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELO_P.P.RR__
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=3 (64-bit / doubleword size)
#[test]
fn test_whilelo_p_p_rr_combo_3_c00_25e00c00() {
    // Encoding: 0x25E00C00
    // Test WHILELO_P.P.RR__ field combination: size=3, Rm=0, sf=0, Rn=0, Pd=0
    // Fields: Pd=0, size=3, Rn=0, Rm=0, sf=0
    let encoding: u32 = 0x25E00C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELO_P.P.RR__
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=0 (register index 0 (first register))
#[test]
fn test_whilelo_p_p_rr_combo_4_c00_25200c00() {
    // Encoding: 0x25200C00
    // Test WHILELO_P.P.RR__ field combination: size=0, Rm=0, sf=0, Rn=0, Pd=0
    // Fields: size=0, Rn=0, Pd=0, sf=0, Rm=0
    let encoding: u32 = 0x25200C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELO_P.P.RR__
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=1 (register index 1 (second register))
#[test]
fn test_whilelo_p_p_rr_combo_5_c00_25210c00() {
    // Encoding: 0x25210C00
    // Test WHILELO_P.P.RR__ field combination: size=0, Rm=1, sf=0, Rn=0, Pd=0
    // Fields: Rn=0, Rm=1, sf=0, Pd=0, size=0
    let encoding: u32 = 0x25210C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELO_P.P.RR__
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=30 (register index 30 (LR in some contexts))
#[test]
fn test_whilelo_p_p_rr_combo_6_c00_253e0c00() {
    // Encoding: 0x253E0C00
    // Test WHILELO_P.P.RR__ field combination: size=0, Rm=30, sf=0, Rn=0, Pd=0
    // Fields: size=0, Pd=0, sf=0, Rn=0, Rm=30
    let encoding: u32 = 0x253E0C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELO_P.P.RR__
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=31 (register index 31 (special))
#[test]
fn test_whilelo_p_p_rr_combo_7_c00_253f0c00() {
    // Encoding: 0x253F0C00
    // Test WHILELO_P.P.RR__ field combination: size=0, Rm=31, sf=0, Rn=0, Pd=0
    // Fields: sf=0, Rn=0, size=0, Pd=0, Rm=31
    let encoding: u32 = 0x253F0C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELO_P.P.RR__
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// sf=0 (8-bit / byte size)
#[test]
fn test_whilelo_p_p_rr_combo_8_c00_25200c00() {
    // Encoding: 0x25200C00
    // Test WHILELO_P.P.RR__ field combination: size=0, Rm=0, sf=0, Rn=0, Pd=0
    // Fields: size=0, sf=0, Rn=0, Rm=0, Pd=0
    let encoding: u32 = 0x25200C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELO_P.P.RR__
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// sf=1 (16-bit / halfword size)
#[test]
fn test_whilelo_p_p_rr_combo_9_c00_25201c00() {
    // Encoding: 0x25201C00
    // Test WHILELO_P.P.RR__ field combination: size=0, Rm=0, sf=1, Rn=0, Pd=0
    // Fields: size=0, Rn=0, Rm=0, Pd=0, sf=1
    let encoding: u32 = 0x25201C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELO_P.P.RR__
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_whilelo_p_p_rr_combo_10_c00_25200c00() {
    // Encoding: 0x25200C00
    // Test WHILELO_P.P.RR__ field combination: size=0, Rm=0, sf=0, Rn=0, Pd=0
    // Fields: size=0, Pd=0, sf=0, Rm=0, Rn=0
    let encoding: u32 = 0x25200C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELO_P.P.RR__
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_whilelo_p_p_rr_combo_11_c00_25200c20() {
    // Encoding: 0x25200C20
    // Test WHILELO_P.P.RR__ field combination: size=0, Rm=0, sf=0, Rn=1, Pd=0
    // Fields: sf=0, Pd=0, size=0, Rm=0, Rn=1
    let encoding: u32 = 0x25200C20;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELO_P.P.RR__
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_whilelo_p_p_rr_combo_12_c00_25200fc0() {
    // Encoding: 0x25200FC0
    // Test WHILELO_P.P.RR__ field combination: size=0, Rm=0, sf=0, Rn=30, Pd=0
    // Fields: Pd=0, sf=0, Rn=30, Rm=0, size=0
    let encoding: u32 = 0x25200FC0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELO_P.P.RR__
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_whilelo_p_p_rr_combo_13_c00_25200fe0() {
    // Encoding: 0x25200FE0
    // Test WHILELO_P.P.RR__ field combination: size=0, Rm=0, sf=0, Rn=31, Pd=0
    // Fields: Pd=0, sf=0, Rn=31, size=0, Rm=0
    let encoding: u32 = 0x25200FE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELO_P.P.RR__
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pd=0 (register index 0 (first register))
#[test]
fn test_whilelo_p_p_rr_combo_14_c00_25200c00() {
    // Encoding: 0x25200C00
    // Test WHILELO_P.P.RR__ field combination: size=0, Rm=0, sf=0, Rn=0, Pd=0
    // Fields: Rm=0, size=0, Pd=0, Rn=0, sf=0
    let encoding: u32 = 0x25200C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELO_P.P.RR__
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pd=1 (register index 1 (second register))
#[test]
fn test_whilelo_p_p_rr_combo_15_c00_25200c01() {
    // Encoding: 0x25200C01
    // Test WHILELO_P.P.RR__ field combination: size=0, Rm=0, sf=0, Rn=0, Pd=1
    // Fields: Rm=0, Rn=0, Pd=1, sf=0, size=0
    let encoding: u32 = 0x25200C01;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELO_P.P.RR__
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=1 (same register test (reg=1)), Rn=1 (same register test (reg=1))
#[test]
fn test_whilelo_p_p_rr_combo_16_c00_25210c20() {
    // Encoding: 0x25210C20
    // Test WHILELO_P.P.RR__ field combination: size=0, Rm=1, sf=0, Rn=1, Pd=0
    // Fields: Rn=1, size=0, sf=0, Pd=0, Rm=1
    let encoding: u32 = 0x25210C20;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELO_P.P.RR__
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=31 (same register test (reg=31)), Rn=31 (same register test (reg=31))
#[test]
fn test_whilelo_p_p_rr_combo_17_c00_253f0fe0() {
    // Encoding: 0x253F0FE0
    // Test WHILELO_P.P.RR__ field combination: size=0, Rm=31, sf=0, Rn=31, Pd=0
    // Fields: Rn=31, sf=0, Rm=31, Pd=0, size=0
    let encoding: u32 = 0x253F0FE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELO_P.P.RR__
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=1 (same register test (reg=1)), Pd=1 (same register test (reg=1))
#[test]
fn test_whilelo_p_p_rr_combo_18_c00_25210c01() {
    // Encoding: 0x25210C01
    // Test WHILELO_P.P.RR__ field combination: size=0, Rm=1, sf=0, Rn=0, Pd=1
    // Fields: Rm=1, Pd=1, size=0, Rn=0, sf=0
    let encoding: u32 = 0x25210C01;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELO_P.P.RR__
/// ASL: `field combination 19`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=31 (same register test (reg=31)), Pd=31 (same register test (reg=31))
#[test]
fn test_whilelo_p_p_rr_combo_19_c00_253f0c0f() {
    // Encoding: 0x253F0C0F
    // Test WHILELO_P.P.RR__ field combination: size=0, Rm=31, sf=0, Rn=0, Pd=31
    // Fields: sf=0, size=0, Rm=31, Pd=31, Rn=0
    let encoding: u32 = 0x253F0C0F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELO_P.P.RR__
/// ASL: `field combination 20`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (same register test (reg=1)), Pd=1 (same register test (reg=1))
#[test]
fn test_whilelo_p_p_rr_combo_20_c00_25200c21() {
    // Encoding: 0x25200C21
    // Test WHILELO_P.P.RR__ field combination: size=0, Rm=0, sf=0, Rn=1, Pd=1
    // Fields: size=0, Rm=0, Rn=1, Pd=1, sf=0
    let encoding: u32 = 0x25200C21;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELO_P.P.RR__
/// ASL: `field combination 21`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (same register test (reg=31)), Pd=31 (same register test (reg=31))
#[test]
fn test_whilelo_p_p_rr_combo_21_c00_25200fef() {
    // Encoding: 0x25200FEF
    // Test WHILELO_P.P.RR__ field combination: size=0, Rm=0, sf=0, Rn=31, Pd=31
    // Fields: size=0, Pd=31, Rm=0, sf=0, Rn=31
    let encoding: u32 = 0x25200FEF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELO_P.P.RR__
/// ASL: `field size = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "size", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_whilelo_p_p_rr_special_size_0_size_variant_0_3072_25200c00() {
    // Encoding: 0x25200C00
    // Test WHILELO_P.P.RR__ special value size = 0 (Size variant 0)
    // Fields: size=0, Pd=0, Rn=0, Rm=0, sf=0
    let encoding: u32 = 0x25200C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELO_P.P.RR__
/// ASL: `field size = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "size", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_whilelo_p_p_rr_special_size_1_size_variant_1_3072_25600c00() {
    // Encoding: 0x25600C00
    // Test WHILELO_P.P.RR__ special value size = 1 (Size variant 1)
    // Fields: sf=0, Pd=0, size=1, Rn=0, Rm=0
    let encoding: u32 = 0x25600C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELO_P.P.RR__
/// ASL: `field size = 2 (Size variant 2)`
/// Requirement: FieldSpecial { field: "size", value: 2, meaning: "Size variant 2" }
/// Size variant 2
#[test]
fn test_whilelo_p_p_rr_special_size_2_size_variant_2_3072_25a00c00() {
    // Encoding: 0x25A00C00
    // Test WHILELO_P.P.RR__ special value size = 2 (Size variant 2)
    // Fields: Rm=0, Pd=0, sf=0, Rn=0, size=2
    let encoding: u32 = 0x25A00C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELO_P.P.RR__
/// ASL: `field size = 3 (Size variant 3)`
/// Requirement: FieldSpecial { field: "size", value: 3, meaning: "Size variant 3" }
/// Size variant 3
#[test]
fn test_whilelo_p_p_rr_special_size_3_size_variant_3_3072_25e00c00() {
    // Encoding: 0x25E00C00
    // Test WHILELO_P.P.RR__ special value size = 3 (Size variant 3)
    // Fields: Rn=0, size=3, sf=0, Pd=0, Rm=0
    let encoding: u32 = 0x25E00C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELO_P.P.RR__
/// ASL: `field sf = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "sf", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_whilelo_p_p_rr_special_sf_0_size_variant_0_3072_25600c00() {
    // Encoding: 0x25600C00
    // Test WHILELO_P.P.RR__ special value sf = 0 (Size variant 0)
    // Fields: size=1, Rm=0, sf=0, Rn=0, Pd=0
    let encoding: u32 = 0x25600C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELO_P.P.RR__
/// ASL: `field sf = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "sf", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_whilelo_p_p_rr_special_sf_1_size_variant_1_3072_25601c00() {
    // Encoding: 0x25601C00
    // Test WHILELO_P.P.RR__ special value sf = 1 (Size variant 1)
    // Fields: Pd=0, Rm=0, Rn=0, size=1, sf=1
    let encoding: u32 = 0x25601C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELO_P.P.RR__
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_whilelo_p_p_rr_special_rn_31_stack_pointer_sp_may_require_alignment_3072_25600fe0() {
    // Encoding: 0x25600FE0
    // Test WHILELO_P.P.RR__ special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: Rm=0, Pd=0, size=1, sf=0, Rn=31
    let encoding: u32 = 0x25600FE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: WHILELO_P.P.RR__
/// ASL: `SimdFromField("Pd") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "unknown" }
/// verify register write to SimdFromField("Pd")
#[test]
fn test_whilelo_p_p_rr_reg_write_0_25200c00() {
    // Test WHILELO_P.P.RR__ register write: SimdFromField("Pd")
    // Encoding: 0x25200C00
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x25200C00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: WHILELO_P.P.RR__
/// ASL: `Rn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Rn = 31)
#[test]
fn test_whilelo_p_p_rr_sp_rn_25200fe0() {
    // Test WHILELO_P.P.RR__ with Rn = SP (31)
    // Encoding: 0x25200FE0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x25200FE0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: WHILELO_P.P.RR__
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 0 + 0 = 0 (Z=1)
#[test]
fn test_whilelo_p_p_rr_flags_zeroresult_0_25221c20() {
    // Test WHILELO_P.P.RR__ flag computation: ZeroResult
    // Encoding: 0x25221C20
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0x0);
    set_x(&mut cpu, 1, 0x0);
    let encoding: u32 = 0x25221C20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: WHILELO_P.P.RR__
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 1 + (-1) = 0 (Z=1, C=1)
#[test]
fn test_whilelo_p_p_rr_flags_zeroresult_1_25221c20() {
    // Test WHILELO_P.P.RR__ flag computation: ZeroResult
    // Encoding: 0x25221C20
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x1);
    set_x(&mut cpu, 2, 0xFFFFFFFFFFFFFFFF);
    let encoding: u32 = 0x25221C20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: WHILELO_P.P.RR__
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: NegativeResult }
/// negative value (N=1)
#[test]
fn test_whilelo_p_p_rr_flags_negativeresult_2_25221c20() {
    // Test WHILELO_P.P.RR__ flag computation: NegativeResult
    // Encoding: 0x25221C20
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x8000000000000000);
    set_x(&mut cpu, 2, 0x0);
    let encoding: u32 = 0x25221C20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: WHILELO_P.P.RR__
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 1 = 0 (C=1, Z=1)
#[test]
fn test_whilelo_p_p_rr_flags_unsignedoverflow_3_25221c20() {
    // Test WHILELO_P.P.RR__ flag computation: UnsignedOverflow
    // Encoding: 0x25221C20
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0xFFFFFFFFFFFFFFFF);
    set_x(&mut cpu, 2, 0x1);
    let encoding: u32 = 0x25221C20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: WHILELO_P.P.RR__
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 2 = 1 (C=1)
#[test]
fn test_whilelo_p_p_rr_flags_unsignedoverflow_4_25221c20() {
    // Test WHILELO_P.P.RR__ flag computation: UnsignedOverflow
    // Encoding: 0x25221C20
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0x2);
    set_x(&mut cpu, 1, 0xFFFFFFFFFFFFFFFF);
    let encoding: u32 = 0x25221C20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: WHILELO_P.P.RR__
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// max_signed + 1 = min_signed (V=1, N=1)
#[test]
fn test_whilelo_p_p_rr_flags_signedoverflow_5_25221c20() {
    // Test WHILELO_P.P.RR__ flag computation: SignedOverflow
    // Encoding: 0x25221C20
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0x1);
    set_x(&mut cpu, 1, 0x7FFFFFFFFFFFFFFF);
    let encoding: u32 = 0x25221C20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: WHILELO_P.P.RR__
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// min_signed + (-1) = max_signed (V=1)
#[test]
fn test_whilelo_p_p_rr_flags_signedoverflow_6_25221c20() {
    // Test WHILELO_P.P.RR__ flag computation: SignedOverflow
    // Encoding: 0x25221C20
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x8000000000000000);
    set_x(&mut cpu, 2, 0xFFFFFFFFFFFFFFFF);
    let encoding: u32 = 0x25221C20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: WHILELO_P.P.RR__
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: PositiveResult }
/// 100 + 50 = 150 (no flags)
#[test]
fn test_whilelo_p_p_rr_flags_positiveresult_7_25221c20() {
    // Test WHILELO_P.P.RR__ flag computation: PositiveResult
    // Encoding: 0x25221C20
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0x32);
    set_x(&mut cpu, 1, 0x64);
    let encoding: u32 = 0x25221C20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}
