//! A64 integer pac tests.
//!
//! Auto-generated from ARM ASL specifications.
//! DO NOT EDIT MANUALLY.

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::generated::test_helpers::*;

// ============================================================================
// aarch64_integer_pac_strip_dp_1src Tests
// ============================================================================

/// Provenance: aarch64_integer_pac_strip_dp_1src
/// ASL: `field D 10 +: 1`
/// Requirement: FieldBoundary { field: "D", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_integer_pac_strip_dp_1src_field_d_0_min_4000_dac14000() {
    // Encoding: 0xDAC14000
    // Test aarch64_integer_pac_strip_dp_1src field D = 0 (Min)
    // Fields: Rn=0, Rd=0, D=0
    let encoding: u32 = 0xDAC14000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch64_integer_pac_strip_dp_1src
/// ASL: `field D 10 +: 1`
/// Requirement: FieldBoundary { field: "D", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_integer_pac_strip_dp_1src_field_d_1_max_4000_dac14400() {
    // Encoding: 0xDAC14400
    // Test aarch64_integer_pac_strip_dp_1src field D = 1 (Max)
    // Fields: Rn=0, D=1, Rd=0
    let encoding: u32 = 0xDAC14400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch64_integer_pac_strip_dp_1src
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_integer_pac_strip_dp_1src_field_rn_0_min_4000_dac14000() {
    // Encoding: 0xDAC14000
    // Test aarch64_integer_pac_strip_dp_1src field Rn = 0 (Min)
    // Fields: Rn=0, Rd=0, D=0
    let encoding: u32 = 0xDAC14000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch64_integer_pac_strip_dp_1src
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_integer_pac_strip_dp_1src_field_rn_1_poweroftwo_4000_dac14020() {
    // Encoding: 0xDAC14020
    // Test aarch64_integer_pac_strip_dp_1src field Rn = 1 (PowerOfTwo)
    // Fields: D=0, Rn=1, Rd=0
    let encoding: u32 = 0xDAC14020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch64_integer_pac_strip_dp_1src
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_integer_pac_strip_dp_1src_field_rn_30_poweroftwominusone_4000_dac143c0() {
    // Encoding: 0xDAC143C0
    // Test aarch64_integer_pac_strip_dp_1src field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: D=0, Rn=30, Rd=0
    let encoding: u32 = 0xDAC143C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch64_integer_pac_strip_dp_1src
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_integer_pac_strip_dp_1src_field_rn_31_max_4000_dac143e0() {
    // Encoding: 0xDAC143E0
    // Test aarch64_integer_pac_strip_dp_1src field Rn = 31 (Max)
    // Fields: Rd=0, Rn=31, D=0
    let encoding: u32 = 0xDAC143E0;
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

/// Provenance: aarch64_integer_pac_strip_dp_1src
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_integer_pac_strip_dp_1src_field_rd_0_min_4000_dac14000() {
    // Encoding: 0xDAC14000
    // Test aarch64_integer_pac_strip_dp_1src field Rd = 0 (Min)
    // Fields: Rd=0, D=0, Rn=0
    let encoding: u32 = 0xDAC14000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch64_integer_pac_strip_dp_1src
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_integer_pac_strip_dp_1src_field_rd_1_poweroftwo_4000_dac14001() {
    // Encoding: 0xDAC14001
    // Test aarch64_integer_pac_strip_dp_1src field Rd = 1 (PowerOfTwo)
    // Fields: D=0, Rn=0, Rd=1
    let encoding: u32 = 0xDAC14001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch64_integer_pac_strip_dp_1src
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_integer_pac_strip_dp_1src_field_rd_30_poweroftwominusone_4000_dac1401e() {
    // Encoding: 0xDAC1401E
    // Test aarch64_integer_pac_strip_dp_1src field Rd = 30 (PowerOfTwoMinusOne)
    // Fields: Rd=30, Rn=0, D=0
    let encoding: u32 = 0xDAC1401E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch64_integer_pac_strip_dp_1src
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 31, boundary: Max }
/// register index 31 (ZR - zero register)
#[test]
fn test_aarch64_integer_pac_strip_dp_1src_field_rd_31_max_4000_dac1401f() {
    // Encoding: 0xDAC1401F
    // Test aarch64_integer_pac_strip_dp_1src field Rd = 31 (Max)
    // Fields: Rd=31, Rn=0, D=0
    let encoding: u32 = 0xDAC1401F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch64_integer_pac_strip_dp_1src
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// D=0 (minimum value)
#[test]
fn test_aarch64_integer_pac_strip_dp_1src_combo_0_4000_dac14000() {
    // Encoding: 0xDAC14000
    // Test aarch64_integer_pac_strip_dp_1src field combination: D=0, Rn=0, Rd=0
    // Fields: Rd=0, Rn=0, D=0
    let encoding: u32 = 0xDAC14000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch64_integer_pac_strip_dp_1src
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// D=1 (maximum value (1))
#[test]
fn test_aarch64_integer_pac_strip_dp_1src_combo_1_4000_dac14400() {
    // Encoding: 0xDAC14400
    // Test aarch64_integer_pac_strip_dp_1src field combination: D=1, Rn=0, Rd=0
    // Fields: Rn=0, D=1, Rd=0
    let encoding: u32 = 0xDAC14400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch64_integer_pac_strip_dp_1src
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch64_integer_pac_strip_dp_1src_combo_2_4000_dac14000() {
    // Encoding: 0xDAC14000
    // Test aarch64_integer_pac_strip_dp_1src field combination: D=0, Rn=0, Rd=0
    // Fields: D=0, Rn=0, Rd=0
    let encoding: u32 = 0xDAC14000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch64_integer_pac_strip_dp_1src
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_aarch64_integer_pac_strip_dp_1src_combo_3_4000_dac14020() {
    // Encoding: 0xDAC14020
    // Test aarch64_integer_pac_strip_dp_1src field combination: D=0, Rn=1, Rd=0
    // Fields: Rd=0, Rn=1, D=0
    let encoding: u32 = 0xDAC14020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch64_integer_pac_strip_dp_1src
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_integer_pac_strip_dp_1src_combo_4_4000_dac143c0() {
    // Encoding: 0xDAC143C0
    // Test aarch64_integer_pac_strip_dp_1src field combination: D=0, Rn=30, Rd=0
    // Fields: D=0, Rn=30, Rd=0
    let encoding: u32 = 0xDAC143C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch64_integer_pac_strip_dp_1src
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_integer_pac_strip_dp_1src_combo_5_4000_dac143e0() {
    // Encoding: 0xDAC143E0
    // Test aarch64_integer_pac_strip_dp_1src field combination: D=0, Rn=31, Rd=0
    // Fields: D=0, Rn=31, Rd=0
    let encoding: u32 = 0xDAC143E0;
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

/// Provenance: aarch64_integer_pac_strip_dp_1src
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=0 (register index 0 (first register))
#[test]
fn test_aarch64_integer_pac_strip_dp_1src_combo_6_4000_dac14000() {
    // Encoding: 0xDAC14000
    // Test aarch64_integer_pac_strip_dp_1src field combination: D=0, Rn=0, Rd=0
    // Fields: D=0, Rn=0, Rd=0
    let encoding: u32 = 0xDAC14000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch64_integer_pac_strip_dp_1src
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=1 (register index 1 (second register))
#[test]
fn test_aarch64_integer_pac_strip_dp_1src_combo_7_4000_dac14001() {
    // Encoding: 0xDAC14001
    // Test aarch64_integer_pac_strip_dp_1src field combination: D=0, Rn=0, Rd=1
    // Fields: Rn=0, Rd=1, D=0
    let encoding: u32 = 0xDAC14001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch64_integer_pac_strip_dp_1src
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_integer_pac_strip_dp_1src_combo_8_4000_dac1401e() {
    // Encoding: 0xDAC1401E
    // Test aarch64_integer_pac_strip_dp_1src field combination: D=0, Rn=0, Rd=30
    // Fields: Rn=0, Rd=30, D=0
    let encoding: u32 = 0xDAC1401E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch64_integer_pac_strip_dp_1src
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=31 (register index 31 (ZR - zero register))
#[test]
fn test_aarch64_integer_pac_strip_dp_1src_combo_9_4000_dac1401f() {
    // Encoding: 0xDAC1401F
    // Test aarch64_integer_pac_strip_dp_1src field combination: D=0, Rn=0, Rd=31
    // Fields: Rd=31, D=0, Rn=0
    let encoding: u32 = 0xDAC1401F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch64_integer_pac_strip_dp_1src
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (same register test (reg=1)), Rd=1 (same register test (reg=1))
#[test]
fn test_aarch64_integer_pac_strip_dp_1src_combo_10_4000_dac14021() {
    // Encoding: 0xDAC14021
    // Test aarch64_integer_pac_strip_dp_1src field combination: D=0, Rn=1, Rd=1
    // Fields: D=0, Rn=1, Rd=1
    let encoding: u32 = 0xDAC14021;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch64_integer_pac_strip_dp_1src
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (same register test (reg=31)), Rd=31 (same register test (reg=31))
#[test]
fn test_aarch64_integer_pac_strip_dp_1src_combo_11_4000_dac143ff() {
    // Encoding: 0xDAC143FF
    // Test aarch64_integer_pac_strip_dp_1src field combination: D=0, Rn=31, Rd=31
    // Fields: Rd=31, Rn=31, D=0
    let encoding: u32 = 0xDAC143FF;
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

/// Provenance: aarch64_integer_pac_strip_dp_1src
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_integer_pac_strip_dp_1src_special_rn_31_stack_pointer_sp_may_require_alignment_16384_dac143e0()
 {
    // Encoding: 0xDAC143E0
    // Test aarch64_integer_pac_strip_dp_1src special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: Rd=0, Rn=31, D=0
    let encoding: u32 = 0xDAC143E0;
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

/// Provenance: aarch64_integer_pac_strip_dp_1src
/// ASL: `field Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)`
/// Requirement: FieldSpecial { field: "Rd", value: 31, meaning: "Zero register (XZR/WZR) - reads as 0, writes discarded" }
/// Zero register (XZR/WZR) - reads as 0, writes discarded
#[test]
fn test_aarch64_integer_pac_strip_dp_1src_special_rd_31_zero_register_xzr_wzr_reads_as_0_writes_discarded_16384_dac1401f()
 {
    // Encoding: 0xDAC1401F
    // Test aarch64_integer_pac_strip_dp_1src special value Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)
    // Fields: Rn=0, Rd=31, D=0
    let encoding: u32 = 0xDAC1401F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch64_integer_pac_strip_hint
/// ASL: `fixed encoding (no variable fields)`
/// Requirement: BasicEncoding
/// instruction with no variable fields
#[test]
fn test_aarch64_integer_pac_strip_hint_basic_encoding_d50320ff() {
    // Encoding: 0xD50320FF
    // Test aarch64_integer_pac_strip_hint basic encoding
    let encoding: u32 = 0xD50320FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_pac_strip_dp_1src
/// ASL: `GpFromField("d") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "d" }
/// verify register write to GpFromField("d")
#[test]
fn test_aarch64_integer_pac_strip_dp_1src_reg_write_0_dac14000() {
    // Test aarch64_integer_pac_strip_dp_1src register write: GpFromField("d")
    // Encoding: 0xDAC14000
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xDAC14000;
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

/// Provenance: aarch64_integer_pac_strip_dp_1src
/// ASL: `Rn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Rn = 31)
#[test]
fn test_aarch64_integer_pac_strip_dp_1src_sp_rn_dac143e0() {
    // Test aarch64_integer_pac_strip_dp_1src with Rn = SP (31)
    // Encoding: 0xDAC143E0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xDAC143E0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_integer_pac_strip_dp_1src
/// ASL: `Rd = 31 (ZR)`
/// Requirement: RegisterSpecial { reg: Zr, behavior: "reads as 0, writes discarded" }
/// zero register (Rd = 31)
#[test]
fn test_aarch64_integer_pac_strip_dp_1src_zr_rd_dac1401f() {
    // Test aarch64_integer_pac_strip_dp_1src with Rd = ZR (31)
    // Encoding: 0xDAC1401F
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xDAC1401F;
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

/// Provenance: aarch64_integer_pac_strip_hint
/// ASL: `GpFromField("d") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "d" }
/// verify register write to GpFromField("d")
#[test]
fn test_aarch64_integer_pac_strip_hint_reg_write_0_d50320ff() {
    // Test aarch64_integer_pac_strip_hint register write: GpFromField("d")
    // Encoding: 0xD50320FF
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xD50320FF;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

// ============================================================================
// aarch64_integer_pac_pacga_dp_2src Tests
// ============================================================================

/// Provenance: aarch64_integer_pac_pacga_dp_2src
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_integer_pac_pacga_dp_2src_field_rm_0_min_3000_9ac03000() {
    // Encoding: 0x9AC03000
    // Test aarch64_integer_pac_pacga_dp_2src field Rm = 0 (Min)
    // Fields: Rn=0, Rm=0, Rd=0
    let encoding: u32 = 0x9AC03000;
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

/// Provenance: aarch64_integer_pac_pacga_dp_2src
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_integer_pac_pacga_dp_2src_field_rm_1_poweroftwo_3000_9ac13000() {
    // Encoding: 0x9AC13000
    // Test aarch64_integer_pac_pacga_dp_2src field Rm = 1 (PowerOfTwo)
    // Fields: Rn=0, Rm=1, Rd=0
    let encoding: u32 = 0x9AC13000;
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

/// Provenance: aarch64_integer_pac_pacga_dp_2src
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_integer_pac_pacga_dp_2src_field_rm_30_poweroftwominusone_3000_9ade3000() {
    // Encoding: 0x9ADE3000
    // Test aarch64_integer_pac_pacga_dp_2src field Rm = 30 (PowerOfTwoMinusOne)
    // Fields: Rn=0, Rd=0, Rm=30
    let encoding: u32 = 0x9ADE3000;
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

/// Provenance: aarch64_integer_pac_pacga_dp_2src
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 31, boundary: Max }
/// register index 31 (special)
#[test]
fn test_aarch64_integer_pac_pacga_dp_2src_field_rm_31_max_3000_9adf3000() {
    // Encoding: 0x9ADF3000
    // Test aarch64_integer_pac_pacga_dp_2src field Rm = 31 (Max)
    // Fields: Rm=31, Rn=0, Rd=0
    let encoding: u32 = 0x9ADF3000;
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

/// Provenance: aarch64_integer_pac_pacga_dp_2src
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_integer_pac_pacga_dp_2src_field_rn_0_min_3000_9ac03000() {
    // Encoding: 0x9AC03000
    // Test aarch64_integer_pac_pacga_dp_2src field Rn = 0 (Min)
    // Fields: Rd=0, Rm=0, Rn=0
    let encoding: u32 = 0x9AC03000;
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

/// Provenance: aarch64_integer_pac_pacga_dp_2src
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_integer_pac_pacga_dp_2src_field_rn_1_poweroftwo_3000_9ac03020() {
    // Encoding: 0x9AC03020
    // Test aarch64_integer_pac_pacga_dp_2src field Rn = 1 (PowerOfTwo)
    // Fields: Rm=0, Rn=1, Rd=0
    let encoding: u32 = 0x9AC03020;
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

/// Provenance: aarch64_integer_pac_pacga_dp_2src
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_integer_pac_pacga_dp_2src_field_rn_30_poweroftwominusone_3000_9ac033c0() {
    // Encoding: 0x9AC033C0
    // Test aarch64_integer_pac_pacga_dp_2src field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: Rm=0, Rd=0, Rn=30
    let encoding: u32 = 0x9AC033C0;
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

/// Provenance: aarch64_integer_pac_pacga_dp_2src
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_integer_pac_pacga_dp_2src_field_rn_31_max_3000_9ac033e0() {
    // Encoding: 0x9AC033E0
    // Test aarch64_integer_pac_pacga_dp_2src field Rn = 31 (Max)
    // Fields: Rd=0, Rm=0, Rn=31
    let encoding: u32 = 0x9AC033E0;
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

/// Provenance: aarch64_integer_pac_pacga_dp_2src
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_integer_pac_pacga_dp_2src_field_rd_0_min_3000_9ac03000() {
    // Encoding: 0x9AC03000
    // Test aarch64_integer_pac_pacga_dp_2src field Rd = 0 (Min)
    // Fields: Rn=0, Rm=0, Rd=0
    let encoding: u32 = 0x9AC03000;
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

/// Provenance: aarch64_integer_pac_pacga_dp_2src
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_integer_pac_pacga_dp_2src_field_rd_1_poweroftwo_3000_9ac03001() {
    // Encoding: 0x9AC03001
    // Test aarch64_integer_pac_pacga_dp_2src field Rd = 1 (PowerOfTwo)
    // Fields: Rm=0, Rn=0, Rd=1
    let encoding: u32 = 0x9AC03001;
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

/// Provenance: aarch64_integer_pac_pacga_dp_2src
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_integer_pac_pacga_dp_2src_field_rd_30_poweroftwominusone_3000_9ac0301e() {
    // Encoding: 0x9AC0301E
    // Test aarch64_integer_pac_pacga_dp_2src field Rd = 30 (PowerOfTwoMinusOne)
    // Fields: Rn=0, Rm=0, Rd=30
    let encoding: u32 = 0x9AC0301E;
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

/// Provenance: aarch64_integer_pac_pacga_dp_2src
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 31, boundary: Max }
/// register index 31 (ZR - zero register)
#[test]
fn test_aarch64_integer_pac_pacga_dp_2src_field_rd_31_max_3000_9ac0301f() {
    // Encoding: 0x9AC0301F
    // Test aarch64_integer_pac_pacga_dp_2src field Rd = 31 (Max)
    // Fields: Rd=31, Rn=0, Rm=0
    let encoding: u32 = 0x9AC0301F;
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

/// Provenance: aarch64_integer_pac_pacga_dp_2src
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=0 (register index 0 (first register))
#[test]
fn test_aarch64_integer_pac_pacga_dp_2src_combo_0_3000_9ac03000() {
    // Encoding: 0x9AC03000
    // Test aarch64_integer_pac_pacga_dp_2src field combination: Rm=0, Rn=0, Rd=0
    // Fields: Rd=0, Rn=0, Rm=0
    let encoding: u32 = 0x9AC03000;
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

/// Provenance: aarch64_integer_pac_pacga_dp_2src
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=1 (register index 1 (second register))
#[test]
fn test_aarch64_integer_pac_pacga_dp_2src_combo_1_3000_9ac13000() {
    // Encoding: 0x9AC13000
    // Test aarch64_integer_pac_pacga_dp_2src field combination: Rm=1, Rn=0, Rd=0
    // Fields: Rm=1, Rn=0, Rd=0
    let encoding: u32 = 0x9AC13000;
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

/// Provenance: aarch64_integer_pac_pacga_dp_2src
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_integer_pac_pacga_dp_2src_combo_2_3000_9ade3000() {
    // Encoding: 0x9ADE3000
    // Test aarch64_integer_pac_pacga_dp_2src field combination: Rm=30, Rn=0, Rd=0
    // Fields: Rd=0, Rm=30, Rn=0
    let encoding: u32 = 0x9ADE3000;
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

/// Provenance: aarch64_integer_pac_pacga_dp_2src
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=31 (register index 31 (special))
#[test]
fn test_aarch64_integer_pac_pacga_dp_2src_combo_3_3000_9adf3000() {
    // Encoding: 0x9ADF3000
    // Test aarch64_integer_pac_pacga_dp_2src field combination: Rm=31, Rn=0, Rd=0
    // Fields: Rn=0, Rd=0, Rm=31
    let encoding: u32 = 0x9ADF3000;
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

/// Provenance: aarch64_integer_pac_pacga_dp_2src
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch64_integer_pac_pacga_dp_2src_combo_4_3000_9ac03000() {
    // Encoding: 0x9AC03000
    // Test aarch64_integer_pac_pacga_dp_2src field combination: Rm=0, Rn=0, Rd=0
    // Fields: Rn=0, Rd=0, Rm=0
    let encoding: u32 = 0x9AC03000;
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

/// Provenance: aarch64_integer_pac_pacga_dp_2src
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_aarch64_integer_pac_pacga_dp_2src_combo_5_3000_9ac03020() {
    // Encoding: 0x9AC03020
    // Test aarch64_integer_pac_pacga_dp_2src field combination: Rm=0, Rn=1, Rd=0
    // Fields: Rm=0, Rd=0, Rn=1
    let encoding: u32 = 0x9AC03020;
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

/// Provenance: aarch64_integer_pac_pacga_dp_2src
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_integer_pac_pacga_dp_2src_combo_6_3000_9ac033c0() {
    // Encoding: 0x9AC033C0
    // Test aarch64_integer_pac_pacga_dp_2src field combination: Rm=0, Rn=30, Rd=0
    // Fields: Rn=30, Rd=0, Rm=0
    let encoding: u32 = 0x9AC033C0;
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

/// Provenance: aarch64_integer_pac_pacga_dp_2src
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_integer_pac_pacga_dp_2src_combo_7_3000_9ac033e0() {
    // Encoding: 0x9AC033E0
    // Test aarch64_integer_pac_pacga_dp_2src field combination: Rm=0, Rn=31, Rd=0
    // Fields: Rm=0, Rd=0, Rn=31
    let encoding: u32 = 0x9AC033E0;
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

/// Provenance: aarch64_integer_pac_pacga_dp_2src
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=0 (register index 0 (first register))
#[test]
fn test_aarch64_integer_pac_pacga_dp_2src_combo_8_3000_9ac03000() {
    // Encoding: 0x9AC03000
    // Test aarch64_integer_pac_pacga_dp_2src field combination: Rm=0, Rn=0, Rd=0
    // Fields: Rd=0, Rn=0, Rm=0
    let encoding: u32 = 0x9AC03000;
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

/// Provenance: aarch64_integer_pac_pacga_dp_2src
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=1 (register index 1 (second register))
#[test]
fn test_aarch64_integer_pac_pacga_dp_2src_combo_9_3000_9ac03001() {
    // Encoding: 0x9AC03001
    // Test aarch64_integer_pac_pacga_dp_2src field combination: Rm=0, Rn=0, Rd=1
    // Fields: Rn=0, Rm=0, Rd=1
    let encoding: u32 = 0x9AC03001;
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

/// Provenance: aarch64_integer_pac_pacga_dp_2src
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_integer_pac_pacga_dp_2src_combo_10_3000_9ac0301e() {
    // Encoding: 0x9AC0301E
    // Test aarch64_integer_pac_pacga_dp_2src field combination: Rm=0, Rn=0, Rd=30
    // Fields: Rm=0, Rn=0, Rd=30
    let encoding: u32 = 0x9AC0301E;
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

/// Provenance: aarch64_integer_pac_pacga_dp_2src
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=31 (register index 31 (ZR - zero register))
#[test]
fn test_aarch64_integer_pac_pacga_dp_2src_combo_11_3000_9ac0301f() {
    // Encoding: 0x9AC0301F
    // Test aarch64_integer_pac_pacga_dp_2src field combination: Rm=0, Rn=0, Rd=31
    // Fields: Rd=31, Rm=0, Rn=0
    let encoding: u32 = 0x9AC0301F;
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

/// Provenance: aarch64_integer_pac_pacga_dp_2src
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=1 (same register test (reg=1)), Rn=1 (same register test (reg=1))
#[test]
fn test_aarch64_integer_pac_pacga_dp_2src_combo_12_3000_9ac13020() {
    // Encoding: 0x9AC13020
    // Test aarch64_integer_pac_pacga_dp_2src field combination: Rm=1, Rn=1, Rd=0
    // Fields: Rm=1, Rn=1, Rd=0
    let encoding: u32 = 0x9AC13020;
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

/// Provenance: aarch64_integer_pac_pacga_dp_2src
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=31 (same register test (reg=31)), Rn=31 (same register test (reg=31))
#[test]
fn test_aarch64_integer_pac_pacga_dp_2src_combo_13_3000_9adf33e0() {
    // Encoding: 0x9ADF33E0
    // Test aarch64_integer_pac_pacga_dp_2src field combination: Rm=31, Rn=31, Rd=0
    // Fields: Rd=0, Rm=31, Rn=31
    let encoding: u32 = 0x9ADF33E0;
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

/// Provenance: aarch64_integer_pac_pacga_dp_2src
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=1 (same register test (reg=1)), Rd=1 (same register test (reg=1))
#[test]
fn test_aarch64_integer_pac_pacga_dp_2src_combo_14_3000_9ac13001() {
    // Encoding: 0x9AC13001
    // Test aarch64_integer_pac_pacga_dp_2src field combination: Rm=1, Rn=0, Rd=1
    // Fields: Rm=1, Rn=0, Rd=1
    let encoding: u32 = 0x9AC13001;
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

/// Provenance: aarch64_integer_pac_pacga_dp_2src
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=31 (same register test (reg=31)), Rd=31 (same register test (reg=31))
#[test]
fn test_aarch64_integer_pac_pacga_dp_2src_combo_15_3000_9adf301f() {
    // Encoding: 0x9ADF301F
    // Test aarch64_integer_pac_pacga_dp_2src field combination: Rm=31, Rn=0, Rd=31
    // Fields: Rn=0, Rm=31, Rd=31
    let encoding: u32 = 0x9ADF301F;
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

/// Provenance: aarch64_integer_pac_pacga_dp_2src
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (same register test (reg=1)), Rd=1 (same register test (reg=1))
#[test]
fn test_aarch64_integer_pac_pacga_dp_2src_combo_16_3000_9ac03021() {
    // Encoding: 0x9AC03021
    // Test aarch64_integer_pac_pacga_dp_2src field combination: Rm=0, Rn=1, Rd=1
    // Fields: Rd=1, Rm=0, Rn=1
    let encoding: u32 = 0x9AC03021;
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

/// Provenance: aarch64_integer_pac_pacga_dp_2src
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (same register test (reg=31)), Rd=31 (same register test (reg=31))
#[test]
fn test_aarch64_integer_pac_pacga_dp_2src_combo_17_3000_9ac033ff() {
    // Encoding: 0x9AC033FF
    // Test aarch64_integer_pac_pacga_dp_2src field combination: Rm=0, Rn=31, Rd=31
    // Fields: Rd=31, Rm=0, Rn=31
    let encoding: u32 = 0x9AC033FF;
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

/// Provenance: aarch64_integer_pac_pacga_dp_2src
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_integer_pac_pacga_dp_2src_special_rn_31_stack_pointer_sp_may_require_alignment_12288_9ac033e0()
 {
    // Encoding: 0x9AC033E0
    // Test aarch64_integer_pac_pacga_dp_2src special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: Rm=0, Rn=31, Rd=0
    let encoding: u32 = 0x9AC033E0;
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

/// Provenance: aarch64_integer_pac_pacga_dp_2src
/// ASL: `field Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)`
/// Requirement: FieldSpecial { field: "Rd", value: 31, meaning: "Zero register (XZR/WZR) - reads as 0, writes discarded" }
/// Zero register (XZR/WZR) - reads as 0, writes discarded
#[test]
fn test_aarch64_integer_pac_pacga_dp_2src_special_rd_31_zero_register_xzr_wzr_reads_as_0_writes_discarded_12288_9ac0301f()
 {
    // Encoding: 0x9AC0301F
    // Test aarch64_integer_pac_pacga_dp_2src special value Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)
    // Fields: Rd=31, Rm=0, Rn=0
    let encoding: u32 = 0x9AC0301F;
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

/// Provenance: aarch64_integer_pac_pacga_dp_2src
/// ASL: `UMULH X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// small values - high bits zero
#[test]
fn test_aarch64_integer_pac_pacga_dp_2src_umulh_oracle_0_9bc27c20() {
    // Test UMULH: small values - high bits zero (oracle)
    // Encoding: 0x9BC27C20
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0x3);
    set_x(&mut cpu, 1, 0x2);
    let encoding: u32 = 0x9BC27C20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 0), 0x0, "X0 should be 0x0000000000000000");
}

/// Provenance: aarch64_integer_pac_pacga_dp_2src
/// ASL: `UMULH X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// large value * 2 - produces high bits
#[test]
fn test_aarch64_integer_pac_pacga_dp_2src_umulh_oracle_1_9bc27c20() {
    // Test UMULH: large value * 2 - produces high bits (oracle)
    // Encoding: 0x9BC27C20
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x8000000000000000);
    set_x(&mut cpu, 2, 0x2);
    let encoding: u32 = 0x9BC27C20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 0), 0x1, "X0 should be 0x0000000000000001");
}

/// Provenance: aarch64_integer_pac_pacga_dp_2src
/// ASL: `UMULH X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// max * max unsigned
#[test]
fn test_aarch64_integer_pac_pacga_dp_2src_umulh_oracle_2_9bc27c20() {
    // Test UMULH: max * max unsigned (oracle)
    // Encoding: 0x9BC27C20
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0xFFFFFFFFFFFFFFFF);
    set_x(&mut cpu, 1, 0xFFFFFFFFFFFFFFFF);
    let encoding: u32 = 0x9BC27C20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_x(&cpu, 0),
        0xFFFFFFFFFFFFFFFE,
        "X0 should be 0xFFFFFFFFFFFFFFFE"
    );
}

/// Provenance: aarch64_integer_pac_pacga_dp_2src
/// ASL: `UMULH X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// max positive * max positive
#[test]
fn test_aarch64_integer_pac_pacga_dp_2src_umulh_oracle_3_9bc27c20() {
    // Test UMULH: max positive * max positive (oracle)
    // Encoding: 0x9BC27C20
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x7FFFFFFFFFFFFFFF);
    set_x(&mut cpu, 2, 0x7FFFFFFFFFFFFFFF);
    let encoding: u32 = 0x9BC27C20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_x(&cpu, 0),
        0x3FFFFFFFFFFFFFFF,
        "X0 should be 0x3FFFFFFFFFFFFFFF"
    );
}

/// Provenance: aarch64_integer_pac_pacga_dp_2src
/// ASL: `UMULH X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// 2^32 * 2^32
#[test]
fn test_aarch64_integer_pac_pacga_dp_2src_umulh_oracle_4_9bc27c20() {
    // Test UMULH: 2^32 * 2^32 (oracle)
    // Encoding: 0x9BC27C20
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0x100000000);
    set_x(&mut cpu, 1, 0x100000000);
    let encoding: u32 = 0x9BC27C20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 0), 0x1, "X0 should be 0x0000000000000001");
}

/// Provenance: aarch64_integer_pac_pacga_dp_2src
/// ASL: `GpFromField("d") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "d" }
/// verify register write to GpFromField("d")
#[test]
fn test_aarch64_integer_pac_pacga_dp_2src_reg_write_0_9ac03000() {
    // Test aarch64_integer_pac_pacga_dp_2src register write: GpFromField("d")
    // Encoding: 0x9AC03000
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x9AC03000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_integer_pac_pacga_dp_2src
/// ASL: `GpFromField("d") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "d" }
/// verify register write to GpFromField("d")
#[test]
fn test_aarch64_integer_pac_pacga_dp_2src_reg_write_1_9ac03000() {
    // Test aarch64_integer_pac_pacga_dp_2src register write: GpFromField("d")
    // Encoding: 0x9AC03000
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x9AC03000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_integer_pac_pacga_dp_2src
/// ASL: `Rn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Rn = 31)
#[test]
fn test_aarch64_integer_pac_pacga_dp_2src_sp_rn_9ac033e0() {
    // Test aarch64_integer_pac_pacga_dp_2src with Rn = SP (31)
    // Encoding: 0x9AC033E0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x9AC033E0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_integer_pac_pacga_dp_2src
/// ASL: `Rd = 31 (ZR)`
/// Requirement: RegisterSpecial { reg: Zr, behavior: "reads as 0, writes discarded" }
/// zero register (Rd = 31)
#[test]
fn test_aarch64_integer_pac_pacga_dp_2src_zr_rd_9ac0301f() {
    // Test aarch64_integer_pac_pacga_dp_2src with Rd = ZR (31)
    // Encoding: 0x9AC0301F
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x9AC0301F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 31), 0, "XZR should always be 0");
}

// ============================================================================
// aarch64_integer_pac_pacda_dp_1src Tests
// ============================================================================

/// Provenance: aarch64_integer_pac_pacda_dp_1src
/// ASL: `field Z 13 +: 1`
/// Requirement: FieldBoundary { field: "Z", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_integer_pac_pacda_dp_1src_field_z_0_min_800_dac10800() {
    // Encoding: 0xDAC10800
    // Test aarch64_integer_pac_pacda_dp_1src field Z = 0 (Min)
    // Fields: Rd=0, Z=0, Rn=0
    let encoding: u32 = 0xDAC10800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: pacda	x0, x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_pacda_dp_1src
/// ASL: `field Z 13 +: 1`
/// Requirement: FieldBoundary { field: "Z", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_integer_pac_pacda_dp_1src_field_z_1_max_800_dac12800() {
    // Encoding: 0xDAC12800
    // Test aarch64_integer_pac_pacda_dp_1src field Z = 1 (Max)
    // Fields: Rn=0, Rd=0, Z=1
    let encoding: u32 = 0xDAC12800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch64_integer_pac_pacda_dp_1src
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_integer_pac_pacda_dp_1src_field_rn_0_min_800_dac10800() {
    // Encoding: 0xDAC10800
    // Test aarch64_integer_pac_pacda_dp_1src field Rn = 0 (Min)
    // Fields: Z=0, Rn=0, Rd=0
    let encoding: u32 = 0xDAC10800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: pacda	x0, x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_pacda_dp_1src
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_integer_pac_pacda_dp_1src_field_rn_1_poweroftwo_800_dac10820() {
    // Encoding: 0xDAC10820
    // Test aarch64_integer_pac_pacda_dp_1src field Rn = 1 (PowerOfTwo)
    // Fields: Rn=1, Rd=0, Z=0
    let encoding: u32 = 0xDAC10820;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: pacda	x0, x1
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_pacda_dp_1src
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_integer_pac_pacda_dp_1src_field_rn_30_poweroftwominusone_800_dac10bc0() {
    // Encoding: 0xDAC10BC0
    // Test aarch64_integer_pac_pacda_dp_1src field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: Rd=0, Rn=30, Z=0
    let encoding: u32 = 0xDAC10BC0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: pacda	x0, x30
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_pacda_dp_1src
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_integer_pac_pacda_dp_1src_field_rn_31_max_800_dac10be0() {
    // Encoding: 0xDAC10BE0
    // Test aarch64_integer_pac_pacda_dp_1src field Rn = 31 (Max)
    // Fields: Z=0, Rn=31, Rd=0
    let encoding: u32 = 0xDAC10BE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: pacda	x0, sp
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_pacda_dp_1src
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_integer_pac_pacda_dp_1src_field_rd_0_min_800_dac10800() {
    // Encoding: 0xDAC10800
    // Test aarch64_integer_pac_pacda_dp_1src field Rd = 0 (Min)
    // Fields: Z=0, Rn=0, Rd=0
    let encoding: u32 = 0xDAC10800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: pacda	x0, x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_pacda_dp_1src
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_integer_pac_pacda_dp_1src_field_rd_1_poweroftwo_800_dac10801() {
    // Encoding: 0xDAC10801
    // Test aarch64_integer_pac_pacda_dp_1src field Rd = 1 (PowerOfTwo)
    // Fields: Z=0, Rn=0, Rd=1
    let encoding: u32 = 0xDAC10801;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: pacda	x1, x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_pacda_dp_1src
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_integer_pac_pacda_dp_1src_field_rd_30_poweroftwominusone_800_dac1081e() {
    // Encoding: 0xDAC1081E
    // Test aarch64_integer_pac_pacda_dp_1src field Rd = 30 (PowerOfTwoMinusOne)
    // Fields: Z=0, Rn=0, Rd=30
    let encoding: u32 = 0xDAC1081E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: pacda	x30, x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_pacda_dp_1src
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 31, boundary: Max }
/// register index 31 (ZR - zero register)
#[test]
fn test_aarch64_integer_pac_pacda_dp_1src_field_rd_31_max_800_dac1081f() {
    // Encoding: 0xDAC1081F
    // Test aarch64_integer_pac_pacda_dp_1src field Rd = 31 (Max)
    // Fields: Z=0, Rn=0, Rd=31
    let encoding: u32 = 0xDAC1081F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: pacda	xzr, x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_pacda_dp_1src
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Z=0 (minimum value)
#[test]
fn test_aarch64_integer_pac_pacda_dp_1src_combo_0_800_dac10800() {
    // Encoding: 0xDAC10800
    // Test aarch64_integer_pac_pacda_dp_1src field combination: Z=0, Rn=0, Rd=0
    // Fields: Rn=0, Rd=0, Z=0
    let encoding: u32 = 0xDAC10800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: pacda	x0, x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_pacda_dp_1src
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Z=1 (maximum value (1))
#[test]
fn test_aarch64_integer_pac_pacda_dp_1src_combo_1_800_dac12800() {
    // Encoding: 0xDAC12800
    // Test aarch64_integer_pac_pacda_dp_1src field combination: Z=1, Rn=0, Rd=0
    // Fields: Z=1, Rd=0, Rn=0
    let encoding: u32 = 0xDAC12800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch64_integer_pac_pacda_dp_1src
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch64_integer_pac_pacda_dp_1src_combo_2_800_dac10800() {
    // Encoding: 0xDAC10800
    // Test aarch64_integer_pac_pacda_dp_1src field combination: Z=0, Rn=0, Rd=0
    // Fields: Rn=0, Z=0, Rd=0
    let encoding: u32 = 0xDAC10800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: pacda	x0, x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_pacda_dp_1src
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_aarch64_integer_pac_pacda_dp_1src_combo_3_800_dac10820() {
    // Encoding: 0xDAC10820
    // Test aarch64_integer_pac_pacda_dp_1src field combination: Z=0, Rn=1, Rd=0
    // Fields: Rd=0, Z=0, Rn=1
    let encoding: u32 = 0xDAC10820;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: pacda	x0, x1
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_pacda_dp_1src
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_integer_pac_pacda_dp_1src_combo_4_800_dac10bc0() {
    // Encoding: 0xDAC10BC0
    // Test aarch64_integer_pac_pacda_dp_1src field combination: Z=0, Rn=30, Rd=0
    // Fields: Rd=0, Z=0, Rn=30
    let encoding: u32 = 0xDAC10BC0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: pacda	x0, x30
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_pacda_dp_1src
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_integer_pac_pacda_dp_1src_combo_5_800_dac10be0() {
    // Encoding: 0xDAC10BE0
    // Test aarch64_integer_pac_pacda_dp_1src field combination: Z=0, Rn=31, Rd=0
    // Fields: Z=0, Rn=31, Rd=0
    let encoding: u32 = 0xDAC10BE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: pacda	x0, sp
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_pacda_dp_1src
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=0 (register index 0 (first register))
#[test]
fn test_aarch64_integer_pac_pacda_dp_1src_combo_6_800_dac10800() {
    // Encoding: 0xDAC10800
    // Test aarch64_integer_pac_pacda_dp_1src field combination: Z=0, Rn=0, Rd=0
    // Fields: Rn=0, Z=0, Rd=0
    let encoding: u32 = 0xDAC10800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: pacda	x0, x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_pacda_dp_1src
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=1 (register index 1 (second register))
#[test]
fn test_aarch64_integer_pac_pacda_dp_1src_combo_7_800_dac10801() {
    // Encoding: 0xDAC10801
    // Test aarch64_integer_pac_pacda_dp_1src field combination: Z=0, Rn=0, Rd=1
    // Fields: Rn=0, Z=0, Rd=1
    let encoding: u32 = 0xDAC10801;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: pacda	x1, x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_pacda_dp_1src
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_integer_pac_pacda_dp_1src_combo_8_800_dac1081e() {
    // Encoding: 0xDAC1081E
    // Test aarch64_integer_pac_pacda_dp_1src field combination: Z=0, Rn=0, Rd=30
    // Fields: Rn=0, Rd=30, Z=0
    let encoding: u32 = 0xDAC1081E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: pacda	x30, x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_pacda_dp_1src
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=31 (register index 31 (ZR - zero register))
#[test]
fn test_aarch64_integer_pac_pacda_dp_1src_combo_9_800_dac1081f() {
    // Encoding: 0xDAC1081F
    // Test aarch64_integer_pac_pacda_dp_1src field combination: Z=0, Rn=0, Rd=31
    // Fields: Z=0, Rn=0, Rd=31
    let encoding: u32 = 0xDAC1081F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: pacda	xzr, x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_pacda_dp_1src
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (same register test (reg=1)), Rd=1 (same register test (reg=1))
#[test]
fn test_aarch64_integer_pac_pacda_dp_1src_combo_10_800_dac10821() {
    // Encoding: 0xDAC10821
    // Test aarch64_integer_pac_pacda_dp_1src field combination: Z=0, Rn=1, Rd=1
    // Fields: Rd=1, Z=0, Rn=1
    let encoding: u32 = 0xDAC10821;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: pacda	x1, x1
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_pacda_dp_1src
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (same register test (reg=31)), Rd=31 (same register test (reg=31))
#[test]
fn test_aarch64_integer_pac_pacda_dp_1src_combo_11_800_dac10bff() {
    // Encoding: 0xDAC10BFF
    // Test aarch64_integer_pac_pacda_dp_1src field combination: Z=0, Rn=31, Rd=31
    // Fields: Rn=31, Rd=31, Z=0
    let encoding: u32 = 0xDAC10BFF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: pacda	xzr, sp
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_pacda_dp_1src
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_integer_pac_pacda_dp_1src_special_rn_31_stack_pointer_sp_may_require_alignment_2048_dac10be0()
 {
    // Encoding: 0xDAC10BE0
    // Test aarch64_integer_pac_pacda_dp_1src special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: Z=0, Rn=31, Rd=0
    let encoding: u32 = 0xDAC10BE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: pacda	x0, sp
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_pacda_dp_1src
/// ASL: `field Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)`
/// Requirement: FieldSpecial { field: "Rd", value: 31, meaning: "Zero register (XZR/WZR) - reads as 0, writes discarded" }
/// Zero register (XZR/WZR) - reads as 0, writes discarded
#[test]
fn test_aarch64_integer_pac_pacda_dp_1src_special_rd_31_zero_register_xzr_wzr_reads_as_0_writes_discarded_2048_dac1081f()
 {
    // Encoding: 0xDAC1081F
    // Test aarch64_integer_pac_pacda_dp_1src special value Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)
    // Fields: Z=0, Rd=31, Rn=0
    let encoding: u32 = 0xDAC1081F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: pacda	xzr, x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_pacda_dp_1src
/// ASL: `GpFromField("d") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "d" }
/// verify register write to GpFromField("d")
#[test]
fn test_aarch64_integer_pac_pacda_dp_1src_reg_write_0_dac10800() {
    // Test aarch64_integer_pac_pacda_dp_1src register write: GpFromField("d")
    // Encoding: 0xDAC10800
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xDAC10800;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_integer_pac_pacda_dp_1src
/// ASL: `GpFromField("d") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "d" }
/// verify register write to GpFromField("d")
#[test]
fn test_aarch64_integer_pac_pacda_dp_1src_reg_write_1_dac10800() {
    // Test aarch64_integer_pac_pacda_dp_1src register write: GpFromField("d")
    // Encoding: 0xDAC10800
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xDAC10800;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_integer_pac_pacda_dp_1src
/// ASL: `Rn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Rn = 31)
#[test]
fn test_aarch64_integer_pac_pacda_dp_1src_sp_rn_dac10be0() {
    // Test aarch64_integer_pac_pacda_dp_1src with Rn = SP (31)
    // Encoding: 0xDAC10BE0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xDAC10BE0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_integer_pac_pacda_dp_1src
/// ASL: `Rd = 31 (ZR)`
/// Requirement: RegisterSpecial { reg: Zr, behavior: "reads as 0, writes discarded" }
/// zero register (Rd = 31)
#[test]
fn test_aarch64_integer_pac_pacda_dp_1src_zr_rd_dac1081f() {
    // Test aarch64_integer_pac_pacda_dp_1src with Rd = ZR (31)
    // Encoding: 0xDAC1081F
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xDAC1081F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 31), 0, "XZR should always be 0");
}

// ============================================================================
// aarch64_integer_pac_autib_dp_1src Tests
// ============================================================================

/// Provenance: aarch64_integer_pac_autib_dp_1src
/// ASL: `field Z 13 +: 1`
/// Requirement: FieldBoundary { field: "Z", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_integer_pac_autib_dp_1src_field_z_0_min_1400_dac11400() {
    // Encoding: 0xDAC11400
    // Test aarch64_integer_pac_autib_dp_1src field Z = 0 (Min)
    // Fields: Rn=0, Z=0, Rd=0
    let encoding: u32 = 0xDAC11400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: autib	x0, x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_autib_dp_1src
/// ASL: `field Z 13 +: 1`
/// Requirement: FieldBoundary { field: "Z", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_integer_pac_autib_dp_1src_field_z_1_max_1400_dac13400() {
    // Encoding: 0xDAC13400
    // Test aarch64_integer_pac_autib_dp_1src field Z = 1 (Max)
    // Fields: Rd=0, Rn=0, Z=1
    let encoding: u32 = 0xDAC13400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch64_integer_pac_autib_dp_1src
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_integer_pac_autib_dp_1src_field_rn_0_min_1400_dac11400() {
    // Encoding: 0xDAC11400
    // Test aarch64_integer_pac_autib_dp_1src field Rn = 0 (Min)
    // Fields: Rn=0, Z=0, Rd=0
    let encoding: u32 = 0xDAC11400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: autib	x0, x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_autib_dp_1src
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_integer_pac_autib_dp_1src_field_rn_1_poweroftwo_1400_dac11420() {
    // Encoding: 0xDAC11420
    // Test aarch64_integer_pac_autib_dp_1src field Rn = 1 (PowerOfTwo)
    // Fields: Z=0, Rd=0, Rn=1
    let encoding: u32 = 0xDAC11420;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: autib	x0, x1
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_autib_dp_1src
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_integer_pac_autib_dp_1src_field_rn_30_poweroftwominusone_1400_dac117c0() {
    // Encoding: 0xDAC117C0
    // Test aarch64_integer_pac_autib_dp_1src field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: Z=0, Rn=30, Rd=0
    let encoding: u32 = 0xDAC117C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: autib	x0, x30
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_autib_dp_1src
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_integer_pac_autib_dp_1src_field_rn_31_max_1400_dac117e0() {
    // Encoding: 0xDAC117E0
    // Test aarch64_integer_pac_autib_dp_1src field Rn = 31 (Max)
    // Fields: Z=0, Rn=31, Rd=0
    let encoding: u32 = 0xDAC117E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: autib	x0, sp
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_autib_dp_1src
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_integer_pac_autib_dp_1src_field_rd_0_min_1400_dac11400() {
    // Encoding: 0xDAC11400
    // Test aarch64_integer_pac_autib_dp_1src field Rd = 0 (Min)
    // Fields: Z=0, Rn=0, Rd=0
    let encoding: u32 = 0xDAC11400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: autib	x0, x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_autib_dp_1src
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_integer_pac_autib_dp_1src_field_rd_1_poweroftwo_1400_dac11401() {
    // Encoding: 0xDAC11401
    // Test aarch64_integer_pac_autib_dp_1src field Rd = 1 (PowerOfTwo)
    // Fields: Z=0, Rn=0, Rd=1
    let encoding: u32 = 0xDAC11401;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: autib	x1, x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_autib_dp_1src
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_integer_pac_autib_dp_1src_field_rd_30_poweroftwominusone_1400_dac1141e() {
    // Encoding: 0xDAC1141E
    // Test aarch64_integer_pac_autib_dp_1src field Rd = 30 (PowerOfTwoMinusOne)
    // Fields: Rd=30, Z=0, Rn=0
    let encoding: u32 = 0xDAC1141E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: autib	x30, x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_autib_dp_1src
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 31, boundary: Max }
/// register index 31 (ZR - zero register)
#[test]
fn test_aarch64_integer_pac_autib_dp_1src_field_rd_31_max_1400_dac1141f() {
    // Encoding: 0xDAC1141F
    // Test aarch64_integer_pac_autib_dp_1src field Rd = 31 (Max)
    // Fields: Rd=31, Z=0, Rn=0
    let encoding: u32 = 0xDAC1141F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: autib	xzr, x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_autib_dp_1src
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Z=0 (minimum value)
#[test]
fn test_aarch64_integer_pac_autib_dp_1src_combo_0_1400_dac11400() {
    // Encoding: 0xDAC11400
    // Test aarch64_integer_pac_autib_dp_1src field combination: Z=0, Rn=0, Rd=0
    // Fields: Rn=0, Rd=0, Z=0
    let encoding: u32 = 0xDAC11400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: autib	x0, x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_autib_dp_1src
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Z=1 (maximum value (1))
#[test]
fn test_aarch64_integer_pac_autib_dp_1src_combo_1_1400_dac13400() {
    // Encoding: 0xDAC13400
    // Test aarch64_integer_pac_autib_dp_1src field combination: Z=1, Rn=0, Rd=0
    // Fields: Rd=0, Rn=0, Z=1
    let encoding: u32 = 0xDAC13400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch64_integer_pac_autib_dp_1src
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch64_integer_pac_autib_dp_1src_combo_2_1400_dac11400() {
    // Encoding: 0xDAC11400
    // Test aarch64_integer_pac_autib_dp_1src field combination: Z=0, Rn=0, Rd=0
    // Fields: Z=0, Rn=0, Rd=0
    let encoding: u32 = 0xDAC11400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: autib	x0, x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_autib_dp_1src
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_aarch64_integer_pac_autib_dp_1src_combo_3_1400_dac11420() {
    // Encoding: 0xDAC11420
    // Test aarch64_integer_pac_autib_dp_1src field combination: Z=0, Rn=1, Rd=0
    // Fields: Z=0, Rn=1, Rd=0
    let encoding: u32 = 0xDAC11420;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: autib	x0, x1
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_autib_dp_1src
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_integer_pac_autib_dp_1src_combo_4_1400_dac117c0() {
    // Encoding: 0xDAC117C0
    // Test aarch64_integer_pac_autib_dp_1src field combination: Z=0, Rn=30, Rd=0
    // Fields: Z=0, Rn=30, Rd=0
    let encoding: u32 = 0xDAC117C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: autib	x0, x30
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_autib_dp_1src
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_integer_pac_autib_dp_1src_combo_5_1400_dac117e0() {
    // Encoding: 0xDAC117E0
    // Test aarch64_integer_pac_autib_dp_1src field combination: Z=0, Rn=31, Rd=0
    // Fields: Rd=0, Z=0, Rn=31
    let encoding: u32 = 0xDAC117E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: autib	x0, sp
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_autib_dp_1src
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=0 (register index 0 (first register))
#[test]
fn test_aarch64_integer_pac_autib_dp_1src_combo_6_1400_dac11400() {
    // Encoding: 0xDAC11400
    // Test aarch64_integer_pac_autib_dp_1src field combination: Z=0, Rn=0, Rd=0
    // Fields: Rd=0, Z=0, Rn=0
    let encoding: u32 = 0xDAC11400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: autib	x0, x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_autib_dp_1src
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=1 (register index 1 (second register))
#[test]
fn test_aarch64_integer_pac_autib_dp_1src_combo_7_1400_dac11401() {
    // Encoding: 0xDAC11401
    // Test aarch64_integer_pac_autib_dp_1src field combination: Z=0, Rn=0, Rd=1
    // Fields: Rd=1, Z=0, Rn=0
    let encoding: u32 = 0xDAC11401;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: autib	x1, x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_autib_dp_1src
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_integer_pac_autib_dp_1src_combo_8_1400_dac1141e() {
    // Encoding: 0xDAC1141E
    // Test aarch64_integer_pac_autib_dp_1src field combination: Z=0, Rn=0, Rd=30
    // Fields: Rd=30, Rn=0, Z=0
    let encoding: u32 = 0xDAC1141E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: autib	x30, x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_autib_dp_1src
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=31 (register index 31 (ZR - zero register))
#[test]
fn test_aarch64_integer_pac_autib_dp_1src_combo_9_1400_dac1141f() {
    // Encoding: 0xDAC1141F
    // Test aarch64_integer_pac_autib_dp_1src field combination: Z=0, Rn=0, Rd=31
    // Fields: Rn=0, Rd=31, Z=0
    let encoding: u32 = 0xDAC1141F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: autib	xzr, x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_autib_dp_1src
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (same register test (reg=1)), Rd=1 (same register test (reg=1))
#[test]
fn test_aarch64_integer_pac_autib_dp_1src_combo_10_1400_dac11421() {
    // Encoding: 0xDAC11421
    // Test aarch64_integer_pac_autib_dp_1src field combination: Z=0, Rn=1, Rd=1
    // Fields: Rd=1, Rn=1, Z=0
    let encoding: u32 = 0xDAC11421;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: autib	x1, x1
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_autib_dp_1src
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (same register test (reg=31)), Rd=31 (same register test (reg=31))
#[test]
fn test_aarch64_integer_pac_autib_dp_1src_combo_11_1400_dac117ff() {
    // Encoding: 0xDAC117FF
    // Test aarch64_integer_pac_autib_dp_1src field combination: Z=0, Rn=31, Rd=31
    // Fields: Z=0, Rd=31, Rn=31
    let encoding: u32 = 0xDAC117FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: autib	xzr, sp
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_autib_dp_1src
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_integer_pac_autib_dp_1src_special_rn_31_stack_pointer_sp_may_require_alignment_5120_dac117e0()
 {
    // Encoding: 0xDAC117E0
    // Test aarch64_integer_pac_autib_dp_1src special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: Rd=0, Rn=31, Z=0
    let encoding: u32 = 0xDAC117E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: autib	x0, sp
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_autib_dp_1src
/// ASL: `field Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)`
/// Requirement: FieldSpecial { field: "Rd", value: 31, meaning: "Zero register (XZR/WZR) - reads as 0, writes discarded" }
/// Zero register (XZR/WZR) - reads as 0, writes discarded
#[test]
fn test_aarch64_integer_pac_autib_dp_1src_special_rd_31_zero_register_xzr_wzr_reads_as_0_writes_discarded_5120_dac1141f()
 {
    // Encoding: 0xDAC1141F
    // Test aarch64_integer_pac_autib_dp_1src special value Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)
    // Fields: Z=0, Rn=0, Rd=31
    let encoding: u32 = 0xDAC1141F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: autib	xzr, x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_autib_hint
/// ASL: `field CRm 8 +: 4`
/// Requirement: FieldBoundary { field: "CRm", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_integer_pac_autib_hint_field_crm_0_min_201f_d503201f() {
    // Encoding: 0xD503201F
    // Test aarch64_integer_pac_autib_hint field CRm = 0 (Min)
    // Fields: op2=0, CRm=0
    let encoding: u32 = 0xD503201F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: nop
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_autib_hint
/// ASL: `field CRm 8 +: 4`
/// Requirement: FieldBoundary { field: "CRm", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_aarch64_integer_pac_autib_hint_field_crm_1_poweroftwo_201f_d503211f() {
    // Encoding: 0xD503211F
    // Test aarch64_integer_pac_autib_hint field CRm = 1 (PowerOfTwo)
    // Fields: op2=0, CRm=1
    let encoding: u32 = 0xD503211F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: pacia1716
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_autib_hint
/// ASL: `field CRm 8 +: 4`
/// Requirement: FieldBoundary { field: "CRm", value: 7, boundary: PowerOfTwoMinusOne }
/// midpoint (7)
#[test]
fn test_aarch64_integer_pac_autib_hint_field_crm_7_poweroftwominusone_201f_d503271f() {
    // Encoding: 0xD503271F
    // Test aarch64_integer_pac_autib_hint field CRm = 7 (PowerOfTwoMinusOne)
    // Fields: CRm=7, op2=0
    let encoding: u32 = 0xD503271F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: hint	#56
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_autib_hint
/// ASL: `field CRm 8 +: 4`
/// Requirement: FieldBoundary { field: "CRm", value: 15, boundary: Max }
/// maximum value (15)
#[test]
fn test_aarch64_integer_pac_autib_hint_field_crm_15_max_201f_d5032f1f() {
    // Encoding: 0xD5032F1F
    // Test aarch64_integer_pac_autib_hint field CRm = 15 (Max)
    // Fields: CRm=15, op2=0
    let encoding: u32 = 0xD5032F1F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: hint	#120
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_autib_hint
/// ASL: `field op2 5 +: 3`
/// Requirement: FieldBoundary { field: "op2", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_integer_pac_autib_hint_field_op2_0_min_201f_d503201f() {
    // Encoding: 0xD503201F
    // Test aarch64_integer_pac_autib_hint field op2 = 0 (Min)
    // Fields: CRm=0, op2=0
    let encoding: u32 = 0xD503201F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: nop
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_autib_hint
/// ASL: `field op2 5 +: 3`
/// Requirement: FieldBoundary { field: "op2", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_aarch64_integer_pac_autib_hint_field_op2_1_poweroftwo_201f_d503203f() {
    // Encoding: 0xD503203F
    // Test aarch64_integer_pac_autib_hint field op2 = 1 (PowerOfTwo)
    // Fields: CRm=0, op2=1
    let encoding: u32 = 0xD503203F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: yield
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_autib_hint
/// ASL: `field op2 5 +: 3`
/// Requirement: FieldBoundary { field: "op2", value: 7, boundary: Max }
/// maximum value (7)
#[test]
fn test_aarch64_integer_pac_autib_hint_field_op2_7_max_201f_d50320ff() {
    // Encoding: 0xD50320FF
    // Test aarch64_integer_pac_autib_hint field op2 = 7 (Max)
    // Fields: op2=7, CRm=0
    let encoding: u32 = 0xD50320FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: xpaclri
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_autib_hint
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// CRm=0 (minimum value)
#[test]
fn test_aarch64_integer_pac_autib_hint_combo_0_201f_d503201f() {
    // Encoding: 0xD503201F
    // Test aarch64_integer_pac_autib_hint field combination: CRm=0, op2=0
    // Fields: op2=0, CRm=0
    let encoding: u32 = 0xD503201F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: nop
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_autib_hint
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// CRm=1 (value 1)
#[test]
fn test_aarch64_integer_pac_autib_hint_combo_1_201f_d503211f() {
    // Encoding: 0xD503211F
    // Test aarch64_integer_pac_autib_hint field combination: CRm=1, op2=0
    // Fields: CRm=1, op2=0
    let encoding: u32 = 0xD503211F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: pacia1716
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_autib_hint
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// CRm=7 (midpoint (7))
#[test]
fn test_aarch64_integer_pac_autib_hint_combo_2_201f_d503271f() {
    // Encoding: 0xD503271F
    // Test aarch64_integer_pac_autib_hint field combination: CRm=7, op2=0
    // Fields: CRm=7, op2=0
    let encoding: u32 = 0xD503271F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: hint	#56
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_autib_hint
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// CRm=15 (maximum value (15))
#[test]
fn test_aarch64_integer_pac_autib_hint_combo_3_201f_d5032f1f() {
    // Encoding: 0xD5032F1F
    // Test aarch64_integer_pac_autib_hint field combination: CRm=15, op2=0
    // Fields: op2=0, CRm=15
    let encoding: u32 = 0xD5032F1F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: hint	#120
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_autib_hint
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// op2=0 (minimum value)
#[test]
fn test_aarch64_integer_pac_autib_hint_combo_4_201f_d503201f() {
    // Encoding: 0xD503201F
    // Test aarch64_integer_pac_autib_hint field combination: CRm=0, op2=0
    // Fields: CRm=0, op2=0
    let encoding: u32 = 0xD503201F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: nop
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_autib_hint
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// op2=1 (value 1)
#[test]
fn test_aarch64_integer_pac_autib_hint_combo_5_201f_d503203f() {
    // Encoding: 0xD503203F
    // Test aarch64_integer_pac_autib_hint field combination: CRm=0, op2=1
    // Fields: CRm=0, op2=1
    let encoding: u32 = 0xD503203F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: yield
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_autib_hint
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// op2=7 (maximum value (7))
#[test]
fn test_aarch64_integer_pac_autib_hint_combo_6_201f_d50320ff() {
    // Encoding: 0xD50320FF
    // Test aarch64_integer_pac_autib_hint field combination: CRm=0, op2=7
    // Fields: CRm=0, op2=7
    let encoding: u32 = 0xD50320FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: xpaclri
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_autib_dp_1src
/// ASL: `GpFromField("d") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "d" }
/// verify register write to GpFromField("d")
#[test]
fn test_aarch64_integer_pac_autib_dp_1src_reg_write_0_dac11400() {
    // Test aarch64_integer_pac_autib_dp_1src register write: GpFromField("d")
    // Encoding: 0xDAC11400
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xDAC11400;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_integer_pac_autib_dp_1src
/// ASL: `GpFromField("d") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "d" }
/// verify register write to GpFromField("d")
#[test]
fn test_aarch64_integer_pac_autib_dp_1src_reg_write_1_dac11400() {
    // Test aarch64_integer_pac_autib_dp_1src register write: GpFromField("d")
    // Encoding: 0xDAC11400
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xDAC11400;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_integer_pac_autib_dp_1src
/// ASL: `Rn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Rn = 31)
#[test]
fn test_aarch64_integer_pac_autib_dp_1src_sp_rn_dac117e0() {
    // Test aarch64_integer_pac_autib_dp_1src with Rn = SP (31)
    // Encoding: 0xDAC117E0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xDAC117E0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_integer_pac_autib_dp_1src
/// ASL: `Rd = 31 (ZR)`
/// Requirement: RegisterSpecial { reg: Zr, behavior: "reads as 0, writes discarded" }
/// zero register (Rd = 31)
#[test]
fn test_aarch64_integer_pac_autib_dp_1src_zr_rd_dac1141f() {
    // Test aarch64_integer_pac_autib_dp_1src with Rd = ZR (31)
    // Encoding: 0xDAC1141F
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xDAC1141F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 31), 0, "XZR should always be 0");
}

/// Provenance: aarch64_integer_pac_autib_hint
/// ASL: `GpFromField("d") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "d" }
/// verify register write to GpFromField("d")
#[test]
fn test_aarch64_integer_pac_autib_hint_reg_write_0_d503201f() {
    // Test aarch64_integer_pac_autib_hint register write: GpFromField("d")
    // Encoding: 0xD503201F
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xD503201F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_integer_pac_autib_hint
/// ASL: `GpFromField("d") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "d" }
/// verify register write to GpFromField("d")
#[test]
fn test_aarch64_integer_pac_autib_hint_reg_write_1_d503201f() {
    // Test aarch64_integer_pac_autib_hint register write: GpFromField("d")
    // Encoding: 0xD503201F
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xD503201F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

// ============================================================================
// aarch64_integer_pac_pacdb_dp_1src Tests
// ============================================================================

/// Provenance: aarch64_integer_pac_pacdb_dp_1src
/// ASL: `field Z 13 +: 1`
/// Requirement: FieldBoundary { field: "Z", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_integer_pac_pacdb_dp_1src_field_z_0_min_c00_dac10c00() {
    // Encoding: 0xDAC10C00
    // Test aarch64_integer_pac_pacdb_dp_1src field Z = 0 (Min)
    // Fields: Rn=0, Rd=0, Z=0
    let encoding: u32 = 0xDAC10C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: pacdb	x0, x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_pacdb_dp_1src
/// ASL: `field Z 13 +: 1`
/// Requirement: FieldBoundary { field: "Z", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_integer_pac_pacdb_dp_1src_field_z_1_max_c00_dac12c00() {
    // Encoding: 0xDAC12C00
    // Test aarch64_integer_pac_pacdb_dp_1src field Z = 1 (Max)
    // Fields: Rd=0, Rn=0, Z=1
    let encoding: u32 = 0xDAC12C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch64_integer_pac_pacdb_dp_1src
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_integer_pac_pacdb_dp_1src_field_rn_0_min_c00_dac10c00() {
    // Encoding: 0xDAC10C00
    // Test aarch64_integer_pac_pacdb_dp_1src field Rn = 0 (Min)
    // Fields: Rn=0, Rd=0, Z=0
    let encoding: u32 = 0xDAC10C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: pacdb	x0, x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_pacdb_dp_1src
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_integer_pac_pacdb_dp_1src_field_rn_1_poweroftwo_c00_dac10c20() {
    // Encoding: 0xDAC10C20
    // Test aarch64_integer_pac_pacdb_dp_1src field Rn = 1 (PowerOfTwo)
    // Fields: Z=0, Rn=1, Rd=0
    let encoding: u32 = 0xDAC10C20;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: pacdb	x0, x1
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_pacdb_dp_1src
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_integer_pac_pacdb_dp_1src_field_rn_30_poweroftwominusone_c00_dac10fc0() {
    // Encoding: 0xDAC10FC0
    // Test aarch64_integer_pac_pacdb_dp_1src field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: Rd=0, Z=0, Rn=30
    let encoding: u32 = 0xDAC10FC0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: pacdb	x0, x30
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_pacdb_dp_1src
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_integer_pac_pacdb_dp_1src_field_rn_31_max_c00_dac10fe0() {
    // Encoding: 0xDAC10FE0
    // Test aarch64_integer_pac_pacdb_dp_1src field Rn = 31 (Max)
    // Fields: Rn=31, Rd=0, Z=0
    let encoding: u32 = 0xDAC10FE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: pacdb	x0, sp
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_pacdb_dp_1src
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_integer_pac_pacdb_dp_1src_field_rd_0_min_c00_dac10c00() {
    // Encoding: 0xDAC10C00
    // Test aarch64_integer_pac_pacdb_dp_1src field Rd = 0 (Min)
    // Fields: Z=0, Rd=0, Rn=0
    let encoding: u32 = 0xDAC10C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: pacdb	x0, x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_pacdb_dp_1src
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_integer_pac_pacdb_dp_1src_field_rd_1_poweroftwo_c00_dac10c01() {
    // Encoding: 0xDAC10C01
    // Test aarch64_integer_pac_pacdb_dp_1src field Rd = 1 (PowerOfTwo)
    // Fields: Rd=1, Z=0, Rn=0
    let encoding: u32 = 0xDAC10C01;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: pacdb	x1, x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_pacdb_dp_1src
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_integer_pac_pacdb_dp_1src_field_rd_30_poweroftwominusone_c00_dac10c1e() {
    // Encoding: 0xDAC10C1E
    // Test aarch64_integer_pac_pacdb_dp_1src field Rd = 30 (PowerOfTwoMinusOne)
    // Fields: Rn=0, Rd=30, Z=0
    let encoding: u32 = 0xDAC10C1E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: pacdb	x30, x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_pacdb_dp_1src
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 31, boundary: Max }
/// register index 31 (ZR - zero register)
#[test]
fn test_aarch64_integer_pac_pacdb_dp_1src_field_rd_31_max_c00_dac10c1f() {
    // Encoding: 0xDAC10C1F
    // Test aarch64_integer_pac_pacdb_dp_1src field Rd = 31 (Max)
    // Fields: Z=0, Rd=31, Rn=0
    let encoding: u32 = 0xDAC10C1F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: pacdb	xzr, x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_pacdb_dp_1src
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Z=0 (minimum value)
#[test]
fn test_aarch64_integer_pac_pacdb_dp_1src_combo_0_c00_dac10c00() {
    // Encoding: 0xDAC10C00
    // Test aarch64_integer_pac_pacdb_dp_1src field combination: Z=0, Rn=0, Rd=0
    // Fields: Rn=0, Z=0, Rd=0
    let encoding: u32 = 0xDAC10C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: pacdb	x0, x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_pacdb_dp_1src
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Z=1 (maximum value (1))
#[test]
fn test_aarch64_integer_pac_pacdb_dp_1src_combo_1_c00_dac12c00() {
    // Encoding: 0xDAC12C00
    // Test aarch64_integer_pac_pacdb_dp_1src field combination: Z=1, Rn=0, Rd=0
    // Fields: Rn=0, Rd=0, Z=1
    let encoding: u32 = 0xDAC12C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch64_integer_pac_pacdb_dp_1src
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch64_integer_pac_pacdb_dp_1src_combo_2_c00_dac10c00() {
    // Encoding: 0xDAC10C00
    // Test aarch64_integer_pac_pacdb_dp_1src field combination: Z=0, Rn=0, Rd=0
    // Fields: Rd=0, Z=0, Rn=0
    let encoding: u32 = 0xDAC10C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: pacdb	x0, x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_pacdb_dp_1src
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_aarch64_integer_pac_pacdb_dp_1src_combo_3_c00_dac10c20() {
    // Encoding: 0xDAC10C20
    // Test aarch64_integer_pac_pacdb_dp_1src field combination: Z=0, Rn=1, Rd=0
    // Fields: Rd=0, Z=0, Rn=1
    let encoding: u32 = 0xDAC10C20;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: pacdb	x0, x1
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_pacdb_dp_1src
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_integer_pac_pacdb_dp_1src_combo_4_c00_dac10fc0() {
    // Encoding: 0xDAC10FC0
    // Test aarch64_integer_pac_pacdb_dp_1src field combination: Z=0, Rn=30, Rd=0
    // Fields: Z=0, Rn=30, Rd=0
    let encoding: u32 = 0xDAC10FC0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: pacdb	x0, x30
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_pacdb_dp_1src
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_integer_pac_pacdb_dp_1src_combo_5_c00_dac10fe0() {
    // Encoding: 0xDAC10FE0
    // Test aarch64_integer_pac_pacdb_dp_1src field combination: Z=0, Rn=31, Rd=0
    // Fields: Rd=0, Rn=31, Z=0
    let encoding: u32 = 0xDAC10FE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: pacdb	x0, sp
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_pacdb_dp_1src
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=0 (register index 0 (first register))
#[test]
fn test_aarch64_integer_pac_pacdb_dp_1src_combo_6_c00_dac10c00() {
    // Encoding: 0xDAC10C00
    // Test aarch64_integer_pac_pacdb_dp_1src field combination: Z=0, Rn=0, Rd=0
    // Fields: Z=0, Rn=0, Rd=0
    let encoding: u32 = 0xDAC10C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: pacdb	x0, x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_pacdb_dp_1src
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=1 (register index 1 (second register))
#[test]
fn test_aarch64_integer_pac_pacdb_dp_1src_combo_7_c00_dac10c01() {
    // Encoding: 0xDAC10C01
    // Test aarch64_integer_pac_pacdb_dp_1src field combination: Z=0, Rn=0, Rd=1
    // Fields: Rn=0, Rd=1, Z=0
    let encoding: u32 = 0xDAC10C01;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: pacdb	x1, x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_pacdb_dp_1src
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_integer_pac_pacdb_dp_1src_combo_8_c00_dac10c1e() {
    // Encoding: 0xDAC10C1E
    // Test aarch64_integer_pac_pacdb_dp_1src field combination: Z=0, Rn=0, Rd=30
    // Fields: Z=0, Rn=0, Rd=30
    let encoding: u32 = 0xDAC10C1E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: pacdb	x30, x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_pacdb_dp_1src
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=31 (register index 31 (ZR - zero register))
#[test]
fn test_aarch64_integer_pac_pacdb_dp_1src_combo_9_c00_dac10c1f() {
    // Encoding: 0xDAC10C1F
    // Test aarch64_integer_pac_pacdb_dp_1src field combination: Z=0, Rn=0, Rd=31
    // Fields: Z=0, Rn=0, Rd=31
    let encoding: u32 = 0xDAC10C1F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: pacdb	xzr, x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_pacdb_dp_1src
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (same register test (reg=1)), Rd=1 (same register test (reg=1))
#[test]
fn test_aarch64_integer_pac_pacdb_dp_1src_combo_10_c00_dac10c21() {
    // Encoding: 0xDAC10C21
    // Test aarch64_integer_pac_pacdb_dp_1src field combination: Z=0, Rn=1, Rd=1
    // Fields: Rn=1, Rd=1, Z=0
    let encoding: u32 = 0xDAC10C21;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: pacdb	x1, x1
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_pacdb_dp_1src
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (same register test (reg=31)), Rd=31 (same register test (reg=31))
#[test]
fn test_aarch64_integer_pac_pacdb_dp_1src_combo_11_c00_dac10fff() {
    // Encoding: 0xDAC10FFF
    // Test aarch64_integer_pac_pacdb_dp_1src field combination: Z=0, Rn=31, Rd=31
    // Fields: Z=0, Rn=31, Rd=31
    let encoding: u32 = 0xDAC10FFF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: pacdb	xzr, sp
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_pacdb_dp_1src
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_integer_pac_pacdb_dp_1src_special_rn_31_stack_pointer_sp_may_require_alignment_3072_dac10fe0()
 {
    // Encoding: 0xDAC10FE0
    // Test aarch64_integer_pac_pacdb_dp_1src special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: Z=0, Rn=31, Rd=0
    let encoding: u32 = 0xDAC10FE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: pacdb	x0, sp
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_pacdb_dp_1src
/// ASL: `field Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)`
/// Requirement: FieldSpecial { field: "Rd", value: 31, meaning: "Zero register (XZR/WZR) - reads as 0, writes discarded" }
/// Zero register (XZR/WZR) - reads as 0, writes discarded
#[test]
fn test_aarch64_integer_pac_pacdb_dp_1src_special_rd_31_zero_register_xzr_wzr_reads_as_0_writes_discarded_3072_dac10c1f()
 {
    // Encoding: 0xDAC10C1F
    // Test aarch64_integer_pac_pacdb_dp_1src special value Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)
    // Fields: Rd=31, Z=0, Rn=0
    let encoding: u32 = 0xDAC10C1F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: pacdb	xzr, x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_pacdb_dp_1src
/// ASL: `GpFromField("d") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "d" }
/// verify register write to GpFromField("d")
#[test]
fn test_aarch64_integer_pac_pacdb_dp_1src_reg_write_0_dac10c00() {
    // Test aarch64_integer_pac_pacdb_dp_1src register write: GpFromField("d")
    // Encoding: 0xDAC10C00
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xDAC10C00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_integer_pac_pacdb_dp_1src
/// ASL: `GpFromField("d") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "d" }
/// verify register write to GpFromField("d")
#[test]
fn test_aarch64_integer_pac_pacdb_dp_1src_reg_write_1_dac10c00() {
    // Test aarch64_integer_pac_pacdb_dp_1src register write: GpFromField("d")
    // Encoding: 0xDAC10C00
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xDAC10C00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_integer_pac_pacdb_dp_1src
/// ASL: `Rn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Rn = 31)
#[test]
fn test_aarch64_integer_pac_pacdb_dp_1src_sp_rn_dac10fe0() {
    // Test aarch64_integer_pac_pacdb_dp_1src with Rn = SP (31)
    // Encoding: 0xDAC10FE0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xDAC10FE0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_integer_pac_pacdb_dp_1src
/// ASL: `Rd = 31 (ZR)`
/// Requirement: RegisterSpecial { reg: Zr, behavior: "reads as 0, writes discarded" }
/// zero register (Rd = 31)
#[test]
fn test_aarch64_integer_pac_pacdb_dp_1src_zr_rd_dac10c1f() {
    // Test aarch64_integer_pac_pacdb_dp_1src with Rd = ZR (31)
    // Encoding: 0xDAC10C1F
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xDAC10C1F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 31), 0, "XZR should always be 0");
}

// ============================================================================
// aarch64_integer_pac_autia_dp_1src Tests
// ============================================================================

/// Provenance: aarch64_integer_pac_autia_dp_1src
/// ASL: `field Z 13 +: 1`
/// Requirement: FieldBoundary { field: "Z", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_integer_pac_autia_dp_1src_field_z_0_min_1000_dac11000() {
    // Encoding: 0xDAC11000
    // Test aarch64_integer_pac_autia_dp_1src field Z = 0 (Min)
    // Fields: Rd=0, Z=0, Rn=0
    let encoding: u32 = 0xDAC11000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: autia	x0, x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_autia_dp_1src
/// ASL: `field Z 13 +: 1`
/// Requirement: FieldBoundary { field: "Z", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_integer_pac_autia_dp_1src_field_z_1_max_1000_dac13000() {
    // Encoding: 0xDAC13000
    // Test aarch64_integer_pac_autia_dp_1src field Z = 1 (Max)
    // Fields: Rn=0, Rd=0, Z=1
    let encoding: u32 = 0xDAC13000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch64_integer_pac_autia_dp_1src
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_integer_pac_autia_dp_1src_field_rn_0_min_1000_dac11000() {
    // Encoding: 0xDAC11000
    // Test aarch64_integer_pac_autia_dp_1src field Rn = 0 (Min)
    // Fields: Rn=0, Z=0, Rd=0
    let encoding: u32 = 0xDAC11000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: autia	x0, x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_autia_dp_1src
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_integer_pac_autia_dp_1src_field_rn_1_poweroftwo_1000_dac11020() {
    // Encoding: 0xDAC11020
    // Test aarch64_integer_pac_autia_dp_1src field Rn = 1 (PowerOfTwo)
    // Fields: Rn=1, Z=0, Rd=0
    let encoding: u32 = 0xDAC11020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: autia	x0, x1
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_autia_dp_1src
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_integer_pac_autia_dp_1src_field_rn_30_poweroftwominusone_1000_dac113c0() {
    // Encoding: 0xDAC113C0
    // Test aarch64_integer_pac_autia_dp_1src field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: Rd=0, Rn=30, Z=0
    let encoding: u32 = 0xDAC113C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: autia	x0, x30
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_autia_dp_1src
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_integer_pac_autia_dp_1src_field_rn_31_max_1000_dac113e0() {
    // Encoding: 0xDAC113E0
    // Test aarch64_integer_pac_autia_dp_1src field Rn = 31 (Max)
    // Fields: Rd=0, Z=0, Rn=31
    let encoding: u32 = 0xDAC113E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: autia	x0, sp
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_autia_dp_1src
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_integer_pac_autia_dp_1src_field_rd_0_min_1000_dac11000() {
    // Encoding: 0xDAC11000
    // Test aarch64_integer_pac_autia_dp_1src field Rd = 0 (Min)
    // Fields: Rd=0, Z=0, Rn=0
    let encoding: u32 = 0xDAC11000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: autia	x0, x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_autia_dp_1src
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_integer_pac_autia_dp_1src_field_rd_1_poweroftwo_1000_dac11001() {
    // Encoding: 0xDAC11001
    // Test aarch64_integer_pac_autia_dp_1src field Rd = 1 (PowerOfTwo)
    // Fields: Rn=0, Rd=1, Z=0
    let encoding: u32 = 0xDAC11001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: autia	x1, x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_autia_dp_1src
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_integer_pac_autia_dp_1src_field_rd_30_poweroftwominusone_1000_dac1101e() {
    // Encoding: 0xDAC1101E
    // Test aarch64_integer_pac_autia_dp_1src field Rd = 30 (PowerOfTwoMinusOne)
    // Fields: Rd=30, Rn=0, Z=0
    let encoding: u32 = 0xDAC1101E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: autia	x30, x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_autia_dp_1src
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 31, boundary: Max }
/// register index 31 (ZR - zero register)
#[test]
fn test_aarch64_integer_pac_autia_dp_1src_field_rd_31_max_1000_dac1101f() {
    // Encoding: 0xDAC1101F
    // Test aarch64_integer_pac_autia_dp_1src field Rd = 31 (Max)
    // Fields: Z=0, Rn=0, Rd=31
    let encoding: u32 = 0xDAC1101F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: autia	xzr, x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_autia_dp_1src
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Z=0 (minimum value)
#[test]
fn test_aarch64_integer_pac_autia_dp_1src_combo_0_1000_dac11000() {
    // Encoding: 0xDAC11000
    // Test aarch64_integer_pac_autia_dp_1src field combination: Z=0, Rn=0, Rd=0
    // Fields: Rn=0, Z=0, Rd=0
    let encoding: u32 = 0xDAC11000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: autia	x0, x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_autia_dp_1src
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Z=1 (maximum value (1))
#[test]
fn test_aarch64_integer_pac_autia_dp_1src_combo_1_1000_dac13000() {
    // Encoding: 0xDAC13000
    // Test aarch64_integer_pac_autia_dp_1src field combination: Z=1, Rn=0, Rd=0
    // Fields: Rn=0, Rd=0, Z=1
    let encoding: u32 = 0xDAC13000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch64_integer_pac_autia_dp_1src
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch64_integer_pac_autia_dp_1src_combo_2_1000_dac11000() {
    // Encoding: 0xDAC11000
    // Test aarch64_integer_pac_autia_dp_1src field combination: Z=0, Rn=0, Rd=0
    // Fields: Rd=0, Z=0, Rn=0
    let encoding: u32 = 0xDAC11000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: autia	x0, x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_autia_dp_1src
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_aarch64_integer_pac_autia_dp_1src_combo_3_1000_dac11020() {
    // Encoding: 0xDAC11020
    // Test aarch64_integer_pac_autia_dp_1src field combination: Z=0, Rn=1, Rd=0
    // Fields: Rd=0, Z=0, Rn=1
    let encoding: u32 = 0xDAC11020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: autia	x0, x1
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_autia_dp_1src
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_integer_pac_autia_dp_1src_combo_4_1000_dac113c0() {
    // Encoding: 0xDAC113C0
    // Test aarch64_integer_pac_autia_dp_1src field combination: Z=0, Rn=30, Rd=0
    // Fields: Z=0, Rn=30, Rd=0
    let encoding: u32 = 0xDAC113C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: autia	x0, x30
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_autia_dp_1src
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_integer_pac_autia_dp_1src_combo_5_1000_dac113e0() {
    // Encoding: 0xDAC113E0
    // Test aarch64_integer_pac_autia_dp_1src field combination: Z=0, Rn=31, Rd=0
    // Fields: Rd=0, Z=0, Rn=31
    let encoding: u32 = 0xDAC113E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: autia	x0, sp
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_autia_dp_1src
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=0 (register index 0 (first register))
#[test]
fn test_aarch64_integer_pac_autia_dp_1src_combo_6_1000_dac11000() {
    // Encoding: 0xDAC11000
    // Test aarch64_integer_pac_autia_dp_1src field combination: Z=0, Rn=0, Rd=0
    // Fields: Rd=0, Rn=0, Z=0
    let encoding: u32 = 0xDAC11000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: autia	x0, x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_autia_dp_1src
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=1 (register index 1 (second register))
#[test]
fn test_aarch64_integer_pac_autia_dp_1src_combo_7_1000_dac11001() {
    // Encoding: 0xDAC11001
    // Test aarch64_integer_pac_autia_dp_1src field combination: Z=0, Rn=0, Rd=1
    // Fields: Rn=0, Rd=1, Z=0
    let encoding: u32 = 0xDAC11001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: autia	x1, x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_autia_dp_1src
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_integer_pac_autia_dp_1src_combo_8_1000_dac1101e() {
    // Encoding: 0xDAC1101E
    // Test aarch64_integer_pac_autia_dp_1src field combination: Z=0, Rn=0, Rd=30
    // Fields: Z=0, Rn=0, Rd=30
    let encoding: u32 = 0xDAC1101E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: autia	x30, x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_autia_dp_1src
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=31 (register index 31 (ZR - zero register))
#[test]
fn test_aarch64_integer_pac_autia_dp_1src_combo_9_1000_dac1101f() {
    // Encoding: 0xDAC1101F
    // Test aarch64_integer_pac_autia_dp_1src field combination: Z=0, Rn=0, Rd=31
    // Fields: Rn=0, Rd=31, Z=0
    let encoding: u32 = 0xDAC1101F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: autia	xzr, x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_autia_dp_1src
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (same register test (reg=1)), Rd=1 (same register test (reg=1))
#[test]
fn test_aarch64_integer_pac_autia_dp_1src_combo_10_1000_dac11021() {
    // Encoding: 0xDAC11021
    // Test aarch64_integer_pac_autia_dp_1src field combination: Z=0, Rn=1, Rd=1
    // Fields: Z=0, Rd=1, Rn=1
    let encoding: u32 = 0xDAC11021;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: autia	x1, x1
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_autia_dp_1src
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (same register test (reg=31)), Rd=31 (same register test (reg=31))
#[test]
fn test_aarch64_integer_pac_autia_dp_1src_combo_11_1000_dac113ff() {
    // Encoding: 0xDAC113FF
    // Test aarch64_integer_pac_autia_dp_1src field combination: Z=0, Rn=31, Rd=31
    // Fields: Rd=31, Z=0, Rn=31
    let encoding: u32 = 0xDAC113FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: autia	xzr, sp
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_autia_dp_1src
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_integer_pac_autia_dp_1src_special_rn_31_stack_pointer_sp_may_require_alignment_4096_dac113e0()
 {
    // Encoding: 0xDAC113E0
    // Test aarch64_integer_pac_autia_dp_1src special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: Rn=31, Z=0, Rd=0
    let encoding: u32 = 0xDAC113E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: autia	x0, sp
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_autia_dp_1src
/// ASL: `field Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)`
/// Requirement: FieldSpecial { field: "Rd", value: 31, meaning: "Zero register (XZR/WZR) - reads as 0, writes discarded" }
/// Zero register (XZR/WZR) - reads as 0, writes discarded
#[test]
fn test_aarch64_integer_pac_autia_dp_1src_special_rd_31_zero_register_xzr_wzr_reads_as_0_writes_discarded_4096_dac1101f()
 {
    // Encoding: 0xDAC1101F
    // Test aarch64_integer_pac_autia_dp_1src special value Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)
    // Fields: Rn=0, Rd=31, Z=0
    let encoding: u32 = 0xDAC1101F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: autia	xzr, x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_autia_hint
/// ASL: `field CRm 8 +: 4`
/// Requirement: FieldBoundary { field: "CRm", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_integer_pac_autia_hint_field_crm_0_min_201f_d503201f() {
    // Encoding: 0xD503201F
    // Test aarch64_integer_pac_autia_hint field CRm = 0 (Min)
    // Fields: CRm=0, op2=0
    let encoding: u32 = 0xD503201F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: nop
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_autia_hint
/// ASL: `field CRm 8 +: 4`
/// Requirement: FieldBoundary { field: "CRm", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_aarch64_integer_pac_autia_hint_field_crm_1_poweroftwo_201f_d503211f() {
    // Encoding: 0xD503211F
    // Test aarch64_integer_pac_autia_hint field CRm = 1 (PowerOfTwo)
    // Fields: CRm=1, op2=0
    let encoding: u32 = 0xD503211F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: pacia1716
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_autia_hint
/// ASL: `field CRm 8 +: 4`
/// Requirement: FieldBoundary { field: "CRm", value: 7, boundary: PowerOfTwoMinusOne }
/// midpoint (7)
#[test]
fn test_aarch64_integer_pac_autia_hint_field_crm_7_poweroftwominusone_201f_d503271f() {
    // Encoding: 0xD503271F
    // Test aarch64_integer_pac_autia_hint field CRm = 7 (PowerOfTwoMinusOne)
    // Fields: op2=0, CRm=7
    let encoding: u32 = 0xD503271F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: hint	#56
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_autia_hint
/// ASL: `field CRm 8 +: 4`
/// Requirement: FieldBoundary { field: "CRm", value: 15, boundary: Max }
/// maximum value (15)
#[test]
fn test_aarch64_integer_pac_autia_hint_field_crm_15_max_201f_d5032f1f() {
    // Encoding: 0xD5032F1F
    // Test aarch64_integer_pac_autia_hint field CRm = 15 (Max)
    // Fields: op2=0, CRm=15
    let encoding: u32 = 0xD5032F1F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: hint	#120
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_autia_hint
/// ASL: `field op2 5 +: 3`
/// Requirement: FieldBoundary { field: "op2", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_integer_pac_autia_hint_field_op2_0_min_201f_d503201f() {
    // Encoding: 0xD503201F
    // Test aarch64_integer_pac_autia_hint field op2 = 0 (Min)
    // Fields: op2=0, CRm=0
    let encoding: u32 = 0xD503201F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: nop
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_autia_hint
/// ASL: `field op2 5 +: 3`
/// Requirement: FieldBoundary { field: "op2", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_aarch64_integer_pac_autia_hint_field_op2_1_poweroftwo_201f_d503203f() {
    // Encoding: 0xD503203F
    // Test aarch64_integer_pac_autia_hint field op2 = 1 (PowerOfTwo)
    // Fields: op2=1, CRm=0
    let encoding: u32 = 0xD503203F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: yield
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_autia_hint
/// ASL: `field op2 5 +: 3`
/// Requirement: FieldBoundary { field: "op2", value: 7, boundary: Max }
/// maximum value (7)
#[test]
fn test_aarch64_integer_pac_autia_hint_field_op2_7_max_201f_d50320ff() {
    // Encoding: 0xD50320FF
    // Test aarch64_integer_pac_autia_hint field op2 = 7 (Max)
    // Fields: CRm=0, op2=7
    let encoding: u32 = 0xD50320FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: xpaclri
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_autia_hint
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// CRm=0 (minimum value)
#[test]
fn test_aarch64_integer_pac_autia_hint_combo_0_201f_d503201f() {
    // Encoding: 0xD503201F
    // Test aarch64_integer_pac_autia_hint field combination: CRm=0, op2=0
    // Fields: CRm=0, op2=0
    let encoding: u32 = 0xD503201F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: nop
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_autia_hint
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// CRm=1 (value 1)
#[test]
fn test_aarch64_integer_pac_autia_hint_combo_1_201f_d503211f() {
    // Encoding: 0xD503211F
    // Test aarch64_integer_pac_autia_hint field combination: CRm=1, op2=0
    // Fields: op2=0, CRm=1
    let encoding: u32 = 0xD503211F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: pacia1716
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_autia_hint
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// CRm=7 (midpoint (7))
#[test]
fn test_aarch64_integer_pac_autia_hint_combo_2_201f_d503271f() {
    // Encoding: 0xD503271F
    // Test aarch64_integer_pac_autia_hint field combination: CRm=7, op2=0
    // Fields: op2=0, CRm=7
    let encoding: u32 = 0xD503271F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: hint	#56
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_autia_hint
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// CRm=15 (maximum value (15))
#[test]
fn test_aarch64_integer_pac_autia_hint_combo_3_201f_d5032f1f() {
    // Encoding: 0xD5032F1F
    // Test aarch64_integer_pac_autia_hint field combination: CRm=15, op2=0
    // Fields: CRm=15, op2=0
    let encoding: u32 = 0xD5032F1F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: hint	#120
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_autia_hint
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// op2=0 (minimum value)
#[test]
fn test_aarch64_integer_pac_autia_hint_combo_4_201f_d503201f() {
    // Encoding: 0xD503201F
    // Test aarch64_integer_pac_autia_hint field combination: CRm=0, op2=0
    // Fields: op2=0, CRm=0
    let encoding: u32 = 0xD503201F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: nop
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_autia_hint
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// op2=1 (value 1)
#[test]
fn test_aarch64_integer_pac_autia_hint_combo_5_201f_d503203f() {
    // Encoding: 0xD503203F
    // Test aarch64_integer_pac_autia_hint field combination: CRm=0, op2=1
    // Fields: CRm=0, op2=1
    let encoding: u32 = 0xD503203F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: yield
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_autia_hint
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// op2=7 (maximum value (7))
#[test]
fn test_aarch64_integer_pac_autia_hint_combo_6_201f_d50320ff() {
    // Encoding: 0xD50320FF
    // Test aarch64_integer_pac_autia_hint field combination: CRm=0, op2=7
    // Fields: op2=7, CRm=0
    let encoding: u32 = 0xD50320FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: xpaclri
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_autia_dp_1src
/// ASL: `GpFromField("d") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "d" }
/// verify register write to GpFromField("d")
#[test]
fn test_aarch64_integer_pac_autia_dp_1src_reg_write_0_dac11000() {
    // Test aarch64_integer_pac_autia_dp_1src register write: GpFromField("d")
    // Encoding: 0xDAC11000
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xDAC11000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_integer_pac_autia_dp_1src
/// ASL: `GpFromField("d") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "d" }
/// verify register write to GpFromField("d")
#[test]
fn test_aarch64_integer_pac_autia_dp_1src_reg_write_1_dac11000() {
    // Test aarch64_integer_pac_autia_dp_1src register write: GpFromField("d")
    // Encoding: 0xDAC11000
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xDAC11000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_integer_pac_autia_dp_1src
/// ASL: `Rn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Rn = 31)
#[test]
fn test_aarch64_integer_pac_autia_dp_1src_sp_rn_dac113e0() {
    // Test aarch64_integer_pac_autia_dp_1src with Rn = SP (31)
    // Encoding: 0xDAC113E0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xDAC113E0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_integer_pac_autia_dp_1src
/// ASL: `Rd = 31 (ZR)`
/// Requirement: RegisterSpecial { reg: Zr, behavior: "reads as 0, writes discarded" }
/// zero register (Rd = 31)
#[test]
fn test_aarch64_integer_pac_autia_dp_1src_zr_rd_dac1101f() {
    // Test aarch64_integer_pac_autia_dp_1src with Rd = ZR (31)
    // Encoding: 0xDAC1101F
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xDAC1101F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 31), 0, "XZR should always be 0");
}

/// Provenance: aarch64_integer_pac_autia_hint
/// ASL: `GpFromField("d") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "d" }
/// verify register write to GpFromField("d")
#[test]
fn test_aarch64_integer_pac_autia_hint_reg_write_0_d503201f() {
    // Test aarch64_integer_pac_autia_hint register write: GpFromField("d")
    // Encoding: 0xD503201F
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xD503201F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_integer_pac_autia_hint
/// ASL: `GpFromField("d") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "d" }
/// verify register write to GpFromField("d")
#[test]
fn test_aarch64_integer_pac_autia_hint_reg_write_1_d503201f() {
    // Test aarch64_integer_pac_autia_hint register write: GpFromField("d")
    // Encoding: 0xD503201F
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xD503201F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

// ============================================================================
// aarch64_integer_pac_autdb_dp_1src Tests
// ============================================================================

/// Provenance: aarch64_integer_pac_autdb_dp_1src
/// ASL: `field Z 13 +: 1`
/// Requirement: FieldBoundary { field: "Z", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_integer_pac_autdb_dp_1src_field_z_0_min_1c00_dac11c00() {
    // Encoding: 0xDAC11C00
    // Test aarch64_integer_pac_autdb_dp_1src field Z = 0 (Min)
    // Fields: Z=0, Rd=0, Rn=0
    let encoding: u32 = 0xDAC11C00;
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

/// Provenance: aarch64_integer_pac_autdb_dp_1src
/// ASL: `field Z 13 +: 1`
/// Requirement: FieldBoundary { field: "Z", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_integer_pac_autdb_dp_1src_field_z_1_max_1c00_dac13c00() {
    // Encoding: 0xDAC13C00
    // Test aarch64_integer_pac_autdb_dp_1src field Z = 1 (Max)
    // Fields: Z=1, Rd=0, Rn=0
    let encoding: u32 = 0xDAC13C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch64_integer_pac_autdb_dp_1src
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_integer_pac_autdb_dp_1src_field_rn_0_min_1c00_dac11c00() {
    // Encoding: 0xDAC11C00
    // Test aarch64_integer_pac_autdb_dp_1src field Rn = 0 (Min)
    // Fields: Rn=0, Rd=0, Z=0
    let encoding: u32 = 0xDAC11C00;
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

/// Provenance: aarch64_integer_pac_autdb_dp_1src
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_integer_pac_autdb_dp_1src_field_rn_1_poweroftwo_1c00_dac11c20() {
    // Encoding: 0xDAC11C20
    // Test aarch64_integer_pac_autdb_dp_1src field Rn = 1 (PowerOfTwo)
    // Fields: Rn=1, Z=0, Rd=0
    let encoding: u32 = 0xDAC11C20;
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

/// Provenance: aarch64_integer_pac_autdb_dp_1src
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_integer_pac_autdb_dp_1src_field_rn_30_poweroftwominusone_1c00_dac11fc0() {
    // Encoding: 0xDAC11FC0
    // Test aarch64_integer_pac_autdb_dp_1src field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: Rd=0, Z=0, Rn=30
    let encoding: u32 = 0xDAC11FC0;
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

/// Provenance: aarch64_integer_pac_autdb_dp_1src
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_integer_pac_autdb_dp_1src_field_rn_31_max_1c00_dac11fe0() {
    // Encoding: 0xDAC11FE0
    // Test aarch64_integer_pac_autdb_dp_1src field Rn = 31 (Max)
    // Fields: Z=0, Rd=0, Rn=31
    let encoding: u32 = 0xDAC11FE0;
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

/// Provenance: aarch64_integer_pac_autdb_dp_1src
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_integer_pac_autdb_dp_1src_field_rd_0_min_1c00_dac11c00() {
    // Encoding: 0xDAC11C00
    // Test aarch64_integer_pac_autdb_dp_1src field Rd = 0 (Min)
    // Fields: Z=0, Rd=0, Rn=0
    let encoding: u32 = 0xDAC11C00;
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

/// Provenance: aarch64_integer_pac_autdb_dp_1src
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_integer_pac_autdb_dp_1src_field_rd_1_poweroftwo_1c00_dac11c01() {
    // Encoding: 0xDAC11C01
    // Test aarch64_integer_pac_autdb_dp_1src field Rd = 1 (PowerOfTwo)
    // Fields: Rn=0, Rd=1, Z=0
    let encoding: u32 = 0xDAC11C01;
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

/// Provenance: aarch64_integer_pac_autdb_dp_1src
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_integer_pac_autdb_dp_1src_field_rd_30_poweroftwominusone_1c00_dac11c1e() {
    // Encoding: 0xDAC11C1E
    // Test aarch64_integer_pac_autdb_dp_1src field Rd = 30 (PowerOfTwoMinusOne)
    // Fields: Z=0, Rd=30, Rn=0
    let encoding: u32 = 0xDAC11C1E;
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

/// Provenance: aarch64_integer_pac_autdb_dp_1src
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 31, boundary: Max }
/// register index 31 (ZR - zero register)
#[test]
fn test_aarch64_integer_pac_autdb_dp_1src_field_rd_31_max_1c00_dac11c1f() {
    // Encoding: 0xDAC11C1F
    // Test aarch64_integer_pac_autdb_dp_1src field Rd = 31 (Max)
    // Fields: Rd=31, Z=0, Rn=0
    let encoding: u32 = 0xDAC11C1F;
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

/// Provenance: aarch64_integer_pac_autdb_dp_1src
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Z=0 (minimum value)
#[test]
fn test_aarch64_integer_pac_autdb_dp_1src_combo_0_1c00_dac11c00() {
    // Encoding: 0xDAC11C00
    // Test aarch64_integer_pac_autdb_dp_1src field combination: Z=0, Rn=0, Rd=0
    // Fields: Rd=0, Rn=0, Z=0
    let encoding: u32 = 0xDAC11C00;
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

/// Provenance: aarch64_integer_pac_autdb_dp_1src
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Z=1 (maximum value (1))
#[test]
fn test_aarch64_integer_pac_autdb_dp_1src_combo_1_1c00_dac13c00() {
    // Encoding: 0xDAC13C00
    // Test aarch64_integer_pac_autdb_dp_1src field combination: Z=1, Rn=0, Rd=0
    // Fields: Rd=0, Rn=0, Z=1
    let encoding: u32 = 0xDAC13C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch64_integer_pac_autdb_dp_1src
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch64_integer_pac_autdb_dp_1src_combo_2_1c00_dac11c00() {
    // Encoding: 0xDAC11C00
    // Test aarch64_integer_pac_autdb_dp_1src field combination: Z=0, Rn=0, Rd=0
    // Fields: Rd=0, Rn=0, Z=0
    let encoding: u32 = 0xDAC11C00;
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

/// Provenance: aarch64_integer_pac_autdb_dp_1src
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_aarch64_integer_pac_autdb_dp_1src_combo_3_1c00_dac11c20() {
    // Encoding: 0xDAC11C20
    // Test aarch64_integer_pac_autdb_dp_1src field combination: Z=0, Rn=1, Rd=0
    // Fields: Z=0, Rn=1, Rd=0
    let encoding: u32 = 0xDAC11C20;
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

/// Provenance: aarch64_integer_pac_autdb_dp_1src
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_integer_pac_autdb_dp_1src_combo_4_1c00_dac11fc0() {
    // Encoding: 0xDAC11FC0
    // Test aarch64_integer_pac_autdb_dp_1src field combination: Z=0, Rn=30, Rd=0
    // Fields: Rd=0, Z=0, Rn=30
    let encoding: u32 = 0xDAC11FC0;
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

/// Provenance: aarch64_integer_pac_autdb_dp_1src
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_integer_pac_autdb_dp_1src_combo_5_1c00_dac11fe0() {
    // Encoding: 0xDAC11FE0
    // Test aarch64_integer_pac_autdb_dp_1src field combination: Z=0, Rn=31, Rd=0
    // Fields: Z=0, Rn=31, Rd=0
    let encoding: u32 = 0xDAC11FE0;
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

/// Provenance: aarch64_integer_pac_autdb_dp_1src
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=0 (register index 0 (first register))
#[test]
fn test_aarch64_integer_pac_autdb_dp_1src_combo_6_1c00_dac11c00() {
    // Encoding: 0xDAC11C00
    // Test aarch64_integer_pac_autdb_dp_1src field combination: Z=0, Rn=0, Rd=0
    // Fields: Rd=0, Z=0, Rn=0
    let encoding: u32 = 0xDAC11C00;
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

/// Provenance: aarch64_integer_pac_autdb_dp_1src
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=1 (register index 1 (second register))
#[test]
fn test_aarch64_integer_pac_autdb_dp_1src_combo_7_1c00_dac11c01() {
    // Encoding: 0xDAC11C01
    // Test aarch64_integer_pac_autdb_dp_1src field combination: Z=0, Rn=0, Rd=1
    // Fields: Rn=0, Rd=1, Z=0
    let encoding: u32 = 0xDAC11C01;
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

/// Provenance: aarch64_integer_pac_autdb_dp_1src
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_integer_pac_autdb_dp_1src_combo_8_1c00_dac11c1e() {
    // Encoding: 0xDAC11C1E
    // Test aarch64_integer_pac_autdb_dp_1src field combination: Z=0, Rn=0, Rd=30
    // Fields: Z=0, Rd=30, Rn=0
    let encoding: u32 = 0xDAC11C1E;
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

/// Provenance: aarch64_integer_pac_autdb_dp_1src
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=31 (register index 31 (ZR - zero register))
#[test]
fn test_aarch64_integer_pac_autdb_dp_1src_combo_9_1c00_dac11c1f() {
    // Encoding: 0xDAC11C1F
    // Test aarch64_integer_pac_autdb_dp_1src field combination: Z=0, Rn=0, Rd=31
    // Fields: Z=0, Rn=0, Rd=31
    let encoding: u32 = 0xDAC11C1F;
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

/// Provenance: aarch64_integer_pac_autdb_dp_1src
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (same register test (reg=1)), Rd=1 (same register test (reg=1))
#[test]
fn test_aarch64_integer_pac_autdb_dp_1src_combo_10_1c00_dac11c21() {
    // Encoding: 0xDAC11C21
    // Test aarch64_integer_pac_autdb_dp_1src field combination: Z=0, Rn=1, Rd=1
    // Fields: Z=0, Rn=1, Rd=1
    let encoding: u32 = 0xDAC11C21;
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

/// Provenance: aarch64_integer_pac_autdb_dp_1src
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (same register test (reg=31)), Rd=31 (same register test (reg=31))
#[test]
fn test_aarch64_integer_pac_autdb_dp_1src_combo_11_1c00_dac11fff() {
    // Encoding: 0xDAC11FFF
    // Test aarch64_integer_pac_autdb_dp_1src field combination: Z=0, Rn=31, Rd=31
    // Fields: Rd=31, Rn=31, Z=0
    let encoding: u32 = 0xDAC11FFF;
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

/// Provenance: aarch64_integer_pac_autdb_dp_1src
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_integer_pac_autdb_dp_1src_special_rn_31_stack_pointer_sp_may_require_alignment_7168_dac11fe0()
 {
    // Encoding: 0xDAC11FE0
    // Test aarch64_integer_pac_autdb_dp_1src special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: Rn=31, Rd=0, Z=0
    let encoding: u32 = 0xDAC11FE0;
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

/// Provenance: aarch64_integer_pac_autdb_dp_1src
/// ASL: `field Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)`
/// Requirement: FieldSpecial { field: "Rd", value: 31, meaning: "Zero register (XZR/WZR) - reads as 0, writes discarded" }
/// Zero register (XZR/WZR) - reads as 0, writes discarded
#[test]
fn test_aarch64_integer_pac_autdb_dp_1src_special_rd_31_zero_register_xzr_wzr_reads_as_0_writes_discarded_7168_dac11c1f()
 {
    // Encoding: 0xDAC11C1F
    // Test aarch64_integer_pac_autdb_dp_1src special value Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)
    // Fields: Rn=0, Rd=31, Z=0
    let encoding: u32 = 0xDAC11C1F;
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

/// Provenance: aarch64_integer_pac_autdb_dp_1src
/// ASL: `GpFromField("d") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "d" }
/// verify register write to GpFromField("d")
#[test]
fn test_aarch64_integer_pac_autdb_dp_1src_reg_write_0_dac11c00() {
    // Test aarch64_integer_pac_autdb_dp_1src register write: GpFromField("d")
    // Encoding: 0xDAC11C00
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xDAC11C00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_integer_pac_autdb_dp_1src
/// ASL: `GpFromField("d") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "d" }
/// verify register write to GpFromField("d")
#[test]
fn test_aarch64_integer_pac_autdb_dp_1src_reg_write_1_dac11c00() {
    // Test aarch64_integer_pac_autdb_dp_1src register write: GpFromField("d")
    // Encoding: 0xDAC11C00
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xDAC11C00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_integer_pac_autdb_dp_1src
/// ASL: `Rn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Rn = 31)
#[test]
fn test_aarch64_integer_pac_autdb_dp_1src_sp_rn_dac11fe0() {
    // Test aarch64_integer_pac_autdb_dp_1src with Rn = SP (31)
    // Encoding: 0xDAC11FE0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xDAC11FE0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_integer_pac_autdb_dp_1src
/// ASL: `Rd = 31 (ZR)`
/// Requirement: RegisterSpecial { reg: Zr, behavior: "reads as 0, writes discarded" }
/// zero register (Rd = 31)
#[test]
fn test_aarch64_integer_pac_autdb_dp_1src_zr_rd_dac11c1f() {
    // Test aarch64_integer_pac_autdb_dp_1src with Rd = ZR (31)
    // Encoding: 0xDAC11C1F
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xDAC11C1F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 31), 0, "XZR should always be 0");
}

// ============================================================================
// aarch64_integer_pac_pacia_dp_1src Tests
// ============================================================================

/// Provenance: aarch64_integer_pac_pacia_dp_1src
/// ASL: `field Z 13 +: 1`
/// Requirement: FieldBoundary { field: "Z", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_integer_pac_pacia_dp_1src_field_z_0_min_0_dac10000() {
    // Encoding: 0xDAC10000
    // Test aarch64_integer_pac_pacia_dp_1src field Z = 0 (Min)
    // Fields: Rd=0, Rn=0, Z=0
    let encoding: u32 = 0xDAC10000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: pacia	x0, x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_pacia_dp_1src
/// ASL: `field Z 13 +: 1`
/// Requirement: FieldBoundary { field: "Z", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_integer_pac_pacia_dp_1src_field_z_1_max_0_dac12000() {
    // Encoding: 0xDAC12000
    // Test aarch64_integer_pac_pacia_dp_1src field Z = 1 (Max)
    // Fields: Z=1, Rn=0, Rd=0
    let encoding: u32 = 0xDAC12000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch64_integer_pac_pacia_dp_1src
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_integer_pac_pacia_dp_1src_field_rn_0_min_0_dac10000() {
    // Encoding: 0xDAC10000
    // Test aarch64_integer_pac_pacia_dp_1src field Rn = 0 (Min)
    // Fields: Rn=0, Rd=0, Z=0
    let encoding: u32 = 0xDAC10000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: pacia	x0, x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_pacia_dp_1src
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_integer_pac_pacia_dp_1src_field_rn_1_poweroftwo_0_dac10020() {
    // Encoding: 0xDAC10020
    // Test aarch64_integer_pac_pacia_dp_1src field Rn = 1 (PowerOfTwo)
    // Fields: Rn=1, Z=0, Rd=0
    let encoding: u32 = 0xDAC10020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: pacia	x0, x1
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_pacia_dp_1src
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_integer_pac_pacia_dp_1src_field_rn_30_poweroftwominusone_0_dac103c0() {
    // Encoding: 0xDAC103C0
    // Test aarch64_integer_pac_pacia_dp_1src field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: Rn=30, Rd=0, Z=0
    let encoding: u32 = 0xDAC103C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: pacia	x0, x30
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_pacia_dp_1src
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_integer_pac_pacia_dp_1src_field_rn_31_max_0_dac103e0() {
    // Encoding: 0xDAC103E0
    // Test aarch64_integer_pac_pacia_dp_1src field Rn = 31 (Max)
    // Fields: Rd=0, Rn=31, Z=0
    let encoding: u32 = 0xDAC103E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: pacia	x0, sp
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_pacia_dp_1src
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_integer_pac_pacia_dp_1src_field_rd_0_min_0_dac10000() {
    // Encoding: 0xDAC10000
    // Test aarch64_integer_pac_pacia_dp_1src field Rd = 0 (Min)
    // Fields: Rd=0, Z=0, Rn=0
    let encoding: u32 = 0xDAC10000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: pacia	x0, x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_pacia_dp_1src
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_integer_pac_pacia_dp_1src_field_rd_1_poweroftwo_0_dac10001() {
    // Encoding: 0xDAC10001
    // Test aarch64_integer_pac_pacia_dp_1src field Rd = 1 (PowerOfTwo)
    // Fields: Rn=0, Rd=1, Z=0
    let encoding: u32 = 0xDAC10001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: pacia	x1, x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_pacia_dp_1src
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_integer_pac_pacia_dp_1src_field_rd_30_poweroftwominusone_0_dac1001e() {
    // Encoding: 0xDAC1001E
    // Test aarch64_integer_pac_pacia_dp_1src field Rd = 30 (PowerOfTwoMinusOne)
    // Fields: Z=0, Rn=0, Rd=30
    let encoding: u32 = 0xDAC1001E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: pacia	x30, x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_pacia_dp_1src
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 31, boundary: Max }
/// register index 31 (ZR - zero register)
#[test]
fn test_aarch64_integer_pac_pacia_dp_1src_field_rd_31_max_0_dac1001f() {
    // Encoding: 0xDAC1001F
    // Test aarch64_integer_pac_pacia_dp_1src field Rd = 31 (Max)
    // Fields: Z=0, Rn=0, Rd=31
    let encoding: u32 = 0xDAC1001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: pacia	xzr, x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_pacia_dp_1src
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Z=0 (minimum value)
#[test]
fn test_aarch64_integer_pac_pacia_dp_1src_combo_0_0_dac10000() {
    // Encoding: 0xDAC10000
    // Test aarch64_integer_pac_pacia_dp_1src field combination: Z=0, Rn=0, Rd=0
    // Fields: Rn=0, Rd=0, Z=0
    let encoding: u32 = 0xDAC10000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: pacia	x0, x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_pacia_dp_1src
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Z=1 (maximum value (1))
#[test]
fn test_aarch64_integer_pac_pacia_dp_1src_combo_1_0_dac12000() {
    // Encoding: 0xDAC12000
    // Test aarch64_integer_pac_pacia_dp_1src field combination: Z=1, Rn=0, Rd=0
    // Fields: Z=1, Rn=0, Rd=0
    let encoding: u32 = 0xDAC12000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch64_integer_pac_pacia_dp_1src
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch64_integer_pac_pacia_dp_1src_combo_2_0_dac10000() {
    // Encoding: 0xDAC10000
    // Test aarch64_integer_pac_pacia_dp_1src field combination: Z=0, Rn=0, Rd=0
    // Fields: Z=0, Rn=0, Rd=0
    let encoding: u32 = 0xDAC10000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: pacia	x0, x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_pacia_dp_1src
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_aarch64_integer_pac_pacia_dp_1src_combo_3_0_dac10020() {
    // Encoding: 0xDAC10020
    // Test aarch64_integer_pac_pacia_dp_1src field combination: Z=0, Rn=1, Rd=0
    // Fields: Rd=0, Z=0, Rn=1
    let encoding: u32 = 0xDAC10020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: pacia	x0, x1
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_pacia_dp_1src
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_integer_pac_pacia_dp_1src_combo_4_0_dac103c0() {
    // Encoding: 0xDAC103C0
    // Test aarch64_integer_pac_pacia_dp_1src field combination: Z=0, Rn=30, Rd=0
    // Fields: Rn=30, Rd=0, Z=0
    let encoding: u32 = 0xDAC103C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: pacia	x0, x30
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_pacia_dp_1src
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_integer_pac_pacia_dp_1src_combo_5_0_dac103e0() {
    // Encoding: 0xDAC103E0
    // Test aarch64_integer_pac_pacia_dp_1src field combination: Z=0, Rn=31, Rd=0
    // Fields: Rn=31, Z=0, Rd=0
    let encoding: u32 = 0xDAC103E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: pacia	x0, sp
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_pacia_dp_1src
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=0 (register index 0 (first register))
#[test]
fn test_aarch64_integer_pac_pacia_dp_1src_combo_6_0_dac10000() {
    // Encoding: 0xDAC10000
    // Test aarch64_integer_pac_pacia_dp_1src field combination: Z=0, Rn=0, Rd=0
    // Fields: Rn=0, Z=0, Rd=0
    let encoding: u32 = 0xDAC10000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: pacia	x0, x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_pacia_dp_1src
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=1 (register index 1 (second register))
#[test]
fn test_aarch64_integer_pac_pacia_dp_1src_combo_7_0_dac10001() {
    // Encoding: 0xDAC10001
    // Test aarch64_integer_pac_pacia_dp_1src field combination: Z=0, Rn=0, Rd=1
    // Fields: Rn=0, Rd=1, Z=0
    let encoding: u32 = 0xDAC10001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: pacia	x1, x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_pacia_dp_1src
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_integer_pac_pacia_dp_1src_combo_8_0_dac1001e() {
    // Encoding: 0xDAC1001E
    // Test aarch64_integer_pac_pacia_dp_1src field combination: Z=0, Rn=0, Rd=30
    // Fields: Rd=30, Rn=0, Z=0
    let encoding: u32 = 0xDAC1001E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: pacia	x30, x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_pacia_dp_1src
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=31 (register index 31 (ZR - zero register))
#[test]
fn test_aarch64_integer_pac_pacia_dp_1src_combo_9_0_dac1001f() {
    // Encoding: 0xDAC1001F
    // Test aarch64_integer_pac_pacia_dp_1src field combination: Z=0, Rn=0, Rd=31
    // Fields: Rn=0, Z=0, Rd=31
    let encoding: u32 = 0xDAC1001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: pacia	xzr, x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_pacia_dp_1src
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (same register test (reg=1)), Rd=1 (same register test (reg=1))
#[test]
fn test_aarch64_integer_pac_pacia_dp_1src_combo_10_0_dac10021() {
    // Encoding: 0xDAC10021
    // Test aarch64_integer_pac_pacia_dp_1src field combination: Z=0, Rn=1, Rd=1
    // Fields: Z=0, Rn=1, Rd=1
    let encoding: u32 = 0xDAC10021;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: pacia	x1, x1
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_pacia_dp_1src
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (same register test (reg=31)), Rd=31 (same register test (reg=31))
#[test]
fn test_aarch64_integer_pac_pacia_dp_1src_combo_11_0_dac103ff() {
    // Encoding: 0xDAC103FF
    // Test aarch64_integer_pac_pacia_dp_1src field combination: Z=0, Rn=31, Rd=31
    // Fields: Rd=31, Z=0, Rn=31
    let encoding: u32 = 0xDAC103FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: pacia	xzr, sp
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_pacia_dp_1src
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_integer_pac_pacia_dp_1src_special_rn_31_stack_pointer_sp_may_require_alignment_0_dac103e0()
 {
    // Encoding: 0xDAC103E0
    // Test aarch64_integer_pac_pacia_dp_1src special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: Rn=31, Z=0, Rd=0
    let encoding: u32 = 0xDAC103E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: pacia	x0, sp
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_pacia_dp_1src
/// ASL: `field Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)`
/// Requirement: FieldSpecial { field: "Rd", value: 31, meaning: "Zero register (XZR/WZR) - reads as 0, writes discarded" }
/// Zero register (XZR/WZR) - reads as 0, writes discarded
#[test]
fn test_aarch64_integer_pac_pacia_dp_1src_special_rd_31_zero_register_xzr_wzr_reads_as_0_writes_discarded_0_dac1001f()
 {
    // Encoding: 0xDAC1001F
    // Test aarch64_integer_pac_pacia_dp_1src special value Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)
    // Fields: Rn=0, Rd=31, Z=0
    let encoding: u32 = 0xDAC1001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: pacia	xzr, x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_pacia_hint
/// ASL: `field CRm 8 +: 4`
/// Requirement: FieldBoundary { field: "CRm", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_integer_pac_pacia_hint_field_crm_0_min_201f_d503201f() {
    // Encoding: 0xD503201F
    // Test aarch64_integer_pac_pacia_hint field CRm = 0 (Min)
    // Fields: op2=0, CRm=0
    let encoding: u32 = 0xD503201F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: nop
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_pacia_hint
/// ASL: `field CRm 8 +: 4`
/// Requirement: FieldBoundary { field: "CRm", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_aarch64_integer_pac_pacia_hint_field_crm_1_poweroftwo_201f_d503211f() {
    // Encoding: 0xD503211F
    // Test aarch64_integer_pac_pacia_hint field CRm = 1 (PowerOfTwo)
    // Fields: CRm=1, op2=0
    let encoding: u32 = 0xD503211F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: pacia1716
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_pacia_hint
/// ASL: `field CRm 8 +: 4`
/// Requirement: FieldBoundary { field: "CRm", value: 7, boundary: PowerOfTwoMinusOne }
/// midpoint (7)
#[test]
fn test_aarch64_integer_pac_pacia_hint_field_crm_7_poweroftwominusone_201f_d503271f() {
    // Encoding: 0xD503271F
    // Test aarch64_integer_pac_pacia_hint field CRm = 7 (PowerOfTwoMinusOne)
    // Fields: op2=0, CRm=7
    let encoding: u32 = 0xD503271F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: hint	#56
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_pacia_hint
/// ASL: `field CRm 8 +: 4`
/// Requirement: FieldBoundary { field: "CRm", value: 15, boundary: Max }
/// maximum value (15)
#[test]
fn test_aarch64_integer_pac_pacia_hint_field_crm_15_max_201f_d5032f1f() {
    // Encoding: 0xD5032F1F
    // Test aarch64_integer_pac_pacia_hint field CRm = 15 (Max)
    // Fields: CRm=15, op2=0
    let encoding: u32 = 0xD5032F1F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: hint	#120
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_pacia_hint
/// ASL: `field op2 5 +: 3`
/// Requirement: FieldBoundary { field: "op2", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_integer_pac_pacia_hint_field_op2_0_min_201f_d503201f() {
    // Encoding: 0xD503201F
    // Test aarch64_integer_pac_pacia_hint field op2 = 0 (Min)
    // Fields: CRm=0, op2=0
    let encoding: u32 = 0xD503201F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: nop
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_pacia_hint
/// ASL: `field op2 5 +: 3`
/// Requirement: FieldBoundary { field: "op2", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_aarch64_integer_pac_pacia_hint_field_op2_1_poweroftwo_201f_d503203f() {
    // Encoding: 0xD503203F
    // Test aarch64_integer_pac_pacia_hint field op2 = 1 (PowerOfTwo)
    // Fields: op2=1, CRm=0
    let encoding: u32 = 0xD503203F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: yield
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_pacia_hint
/// ASL: `field op2 5 +: 3`
/// Requirement: FieldBoundary { field: "op2", value: 7, boundary: Max }
/// maximum value (7)
#[test]
fn test_aarch64_integer_pac_pacia_hint_field_op2_7_max_201f_d50320ff() {
    // Encoding: 0xD50320FF
    // Test aarch64_integer_pac_pacia_hint field op2 = 7 (Max)
    // Fields: CRm=0, op2=7
    let encoding: u32 = 0xD50320FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: xpaclri
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_pacia_hint
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// CRm=0 (minimum value)
#[test]
fn test_aarch64_integer_pac_pacia_hint_combo_0_201f_d503201f() {
    // Encoding: 0xD503201F
    // Test aarch64_integer_pac_pacia_hint field combination: CRm=0, op2=0
    // Fields: op2=0, CRm=0
    let encoding: u32 = 0xD503201F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: nop
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_pacia_hint
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// CRm=1 (value 1)
#[test]
fn test_aarch64_integer_pac_pacia_hint_combo_1_201f_d503211f() {
    // Encoding: 0xD503211F
    // Test aarch64_integer_pac_pacia_hint field combination: CRm=1, op2=0
    // Fields: CRm=1, op2=0
    let encoding: u32 = 0xD503211F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: pacia1716
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_pacia_hint
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// CRm=7 (midpoint (7))
#[test]
fn test_aarch64_integer_pac_pacia_hint_combo_2_201f_d503271f() {
    // Encoding: 0xD503271F
    // Test aarch64_integer_pac_pacia_hint field combination: CRm=7, op2=0
    // Fields: op2=0, CRm=7
    let encoding: u32 = 0xD503271F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: hint	#56
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_pacia_hint
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// CRm=15 (maximum value (15))
#[test]
fn test_aarch64_integer_pac_pacia_hint_combo_3_201f_d5032f1f() {
    // Encoding: 0xD5032F1F
    // Test aarch64_integer_pac_pacia_hint field combination: CRm=15, op2=0
    // Fields: op2=0, CRm=15
    let encoding: u32 = 0xD5032F1F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: hint	#120
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_pacia_hint
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// op2=0 (minimum value)
#[test]
fn test_aarch64_integer_pac_pacia_hint_combo_4_201f_d503201f() {
    // Encoding: 0xD503201F
    // Test aarch64_integer_pac_pacia_hint field combination: CRm=0, op2=0
    // Fields: CRm=0, op2=0
    let encoding: u32 = 0xD503201F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: nop
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_pacia_hint
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// op2=1 (value 1)
#[test]
fn test_aarch64_integer_pac_pacia_hint_combo_5_201f_d503203f() {
    // Encoding: 0xD503203F
    // Test aarch64_integer_pac_pacia_hint field combination: CRm=0, op2=1
    // Fields: CRm=0, op2=1
    let encoding: u32 = 0xD503203F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: yield
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_pacia_hint
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// op2=7 (maximum value (7))
#[test]
fn test_aarch64_integer_pac_pacia_hint_combo_6_201f_d50320ff() {
    // Encoding: 0xD50320FF
    // Test aarch64_integer_pac_pacia_hint field combination: CRm=0, op2=7
    // Fields: CRm=0, op2=7
    let encoding: u32 = 0xD50320FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: xpaclri
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_pacia_dp_1src
/// ASL: `GpFromField("d") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "d" }
/// verify register write to GpFromField("d")
#[test]
fn test_aarch64_integer_pac_pacia_dp_1src_reg_write_0_dac10000() {
    // Test aarch64_integer_pac_pacia_dp_1src register write: GpFromField("d")
    // Encoding: 0xDAC10000
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xDAC10000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_integer_pac_pacia_dp_1src
/// ASL: `GpFromField("d") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "d" }
/// verify register write to GpFromField("d")
#[test]
fn test_aarch64_integer_pac_pacia_dp_1src_reg_write_1_dac10000() {
    // Test aarch64_integer_pac_pacia_dp_1src register write: GpFromField("d")
    // Encoding: 0xDAC10000
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xDAC10000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_integer_pac_pacia_dp_1src
/// ASL: `Rn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Rn = 31)
#[test]
fn test_aarch64_integer_pac_pacia_dp_1src_sp_rn_dac103e0() {
    // Test aarch64_integer_pac_pacia_dp_1src with Rn = SP (31)
    // Encoding: 0xDAC103E0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xDAC103E0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_integer_pac_pacia_dp_1src
/// ASL: `Rd = 31 (ZR)`
/// Requirement: RegisterSpecial { reg: Zr, behavior: "reads as 0, writes discarded" }
/// zero register (Rd = 31)
#[test]
fn test_aarch64_integer_pac_pacia_dp_1src_zr_rd_dac1001f() {
    // Test aarch64_integer_pac_pacia_dp_1src with Rd = ZR (31)
    // Encoding: 0xDAC1001F
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xDAC1001F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 31), 0, "XZR should always be 0");
}

/// Provenance: aarch64_integer_pac_pacia_hint
/// ASL: `GpFromField("d") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "d" }
/// verify register write to GpFromField("d")
#[test]
fn test_aarch64_integer_pac_pacia_hint_reg_write_0_d503201f() {
    // Test aarch64_integer_pac_pacia_hint register write: GpFromField("d")
    // Encoding: 0xD503201F
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xD503201F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_integer_pac_pacia_hint
/// ASL: `GpFromField("d") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "d" }
/// verify register write to GpFromField("d")
#[test]
fn test_aarch64_integer_pac_pacia_hint_reg_write_1_d503201f() {
    // Test aarch64_integer_pac_pacia_hint register write: GpFromField("d")
    // Encoding: 0xD503201F
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xD503201F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

// ============================================================================
// aarch64_integer_pac_autda_dp_1src Tests
// ============================================================================

/// Provenance: aarch64_integer_pac_autda_dp_1src
/// ASL: `field Z 13 +: 1`
/// Requirement: FieldBoundary { field: "Z", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_integer_pac_autda_dp_1src_field_z_0_min_1800_dac11800() {
    // Encoding: 0xDAC11800
    // Test aarch64_integer_pac_autda_dp_1src field Z = 0 (Min)
    // Fields: Rd=0, Z=0, Rn=0
    let encoding: u32 = 0xDAC11800;
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

/// Provenance: aarch64_integer_pac_autda_dp_1src
/// ASL: `field Z 13 +: 1`
/// Requirement: FieldBoundary { field: "Z", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_integer_pac_autda_dp_1src_field_z_1_max_1800_dac13800() {
    // Encoding: 0xDAC13800
    // Test aarch64_integer_pac_autda_dp_1src field Z = 1 (Max)
    // Fields: Rn=0, Rd=0, Z=1
    let encoding: u32 = 0xDAC13800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch64_integer_pac_autda_dp_1src
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_integer_pac_autda_dp_1src_field_rn_0_min_1800_dac11800() {
    // Encoding: 0xDAC11800
    // Test aarch64_integer_pac_autda_dp_1src field Rn = 0 (Min)
    // Fields: Rd=0, Rn=0, Z=0
    let encoding: u32 = 0xDAC11800;
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

/// Provenance: aarch64_integer_pac_autda_dp_1src
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_integer_pac_autda_dp_1src_field_rn_1_poweroftwo_1800_dac11820() {
    // Encoding: 0xDAC11820
    // Test aarch64_integer_pac_autda_dp_1src field Rn = 1 (PowerOfTwo)
    // Fields: Z=0, Rn=1, Rd=0
    let encoding: u32 = 0xDAC11820;
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

/// Provenance: aarch64_integer_pac_autda_dp_1src
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_integer_pac_autda_dp_1src_field_rn_30_poweroftwominusone_1800_dac11bc0() {
    // Encoding: 0xDAC11BC0
    // Test aarch64_integer_pac_autda_dp_1src field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: Rn=30, Z=0, Rd=0
    let encoding: u32 = 0xDAC11BC0;
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

/// Provenance: aarch64_integer_pac_autda_dp_1src
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_integer_pac_autda_dp_1src_field_rn_31_max_1800_dac11be0() {
    // Encoding: 0xDAC11BE0
    // Test aarch64_integer_pac_autda_dp_1src field Rn = 31 (Max)
    // Fields: Rd=0, Rn=31, Z=0
    let encoding: u32 = 0xDAC11BE0;
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

/// Provenance: aarch64_integer_pac_autda_dp_1src
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_integer_pac_autda_dp_1src_field_rd_0_min_1800_dac11800() {
    // Encoding: 0xDAC11800
    // Test aarch64_integer_pac_autda_dp_1src field Rd = 0 (Min)
    // Fields: Z=0, Rn=0, Rd=0
    let encoding: u32 = 0xDAC11800;
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

/// Provenance: aarch64_integer_pac_autda_dp_1src
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_integer_pac_autda_dp_1src_field_rd_1_poweroftwo_1800_dac11801() {
    // Encoding: 0xDAC11801
    // Test aarch64_integer_pac_autda_dp_1src field Rd = 1 (PowerOfTwo)
    // Fields: Rd=1, Rn=0, Z=0
    let encoding: u32 = 0xDAC11801;
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

/// Provenance: aarch64_integer_pac_autda_dp_1src
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_integer_pac_autda_dp_1src_field_rd_30_poweroftwominusone_1800_dac1181e() {
    // Encoding: 0xDAC1181E
    // Test aarch64_integer_pac_autda_dp_1src field Rd = 30 (PowerOfTwoMinusOne)
    // Fields: Rn=0, Z=0, Rd=30
    let encoding: u32 = 0xDAC1181E;
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

/// Provenance: aarch64_integer_pac_autda_dp_1src
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 31, boundary: Max }
/// register index 31 (ZR - zero register)
#[test]
fn test_aarch64_integer_pac_autda_dp_1src_field_rd_31_max_1800_dac1181f() {
    // Encoding: 0xDAC1181F
    // Test aarch64_integer_pac_autda_dp_1src field Rd = 31 (Max)
    // Fields: Rd=31, Z=0, Rn=0
    let encoding: u32 = 0xDAC1181F;
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

/// Provenance: aarch64_integer_pac_autda_dp_1src
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Z=0 (minimum value)
#[test]
fn test_aarch64_integer_pac_autda_dp_1src_combo_0_1800_dac11800() {
    // Encoding: 0xDAC11800
    // Test aarch64_integer_pac_autda_dp_1src field combination: Z=0, Rn=0, Rd=0
    // Fields: Z=0, Rd=0, Rn=0
    let encoding: u32 = 0xDAC11800;
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

/// Provenance: aarch64_integer_pac_autda_dp_1src
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Z=1 (maximum value (1))
#[test]
fn test_aarch64_integer_pac_autda_dp_1src_combo_1_1800_dac13800() {
    // Encoding: 0xDAC13800
    // Test aarch64_integer_pac_autda_dp_1src field combination: Z=1, Rn=0, Rd=0
    // Fields: Z=1, Rd=0, Rn=0
    let encoding: u32 = 0xDAC13800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch64_integer_pac_autda_dp_1src
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch64_integer_pac_autda_dp_1src_combo_2_1800_dac11800() {
    // Encoding: 0xDAC11800
    // Test aarch64_integer_pac_autda_dp_1src field combination: Z=0, Rn=0, Rd=0
    // Fields: Z=0, Rn=0, Rd=0
    let encoding: u32 = 0xDAC11800;
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

/// Provenance: aarch64_integer_pac_autda_dp_1src
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_aarch64_integer_pac_autda_dp_1src_combo_3_1800_dac11820() {
    // Encoding: 0xDAC11820
    // Test aarch64_integer_pac_autda_dp_1src field combination: Z=0, Rn=1, Rd=0
    // Fields: Rn=1, Z=0, Rd=0
    let encoding: u32 = 0xDAC11820;
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

/// Provenance: aarch64_integer_pac_autda_dp_1src
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_integer_pac_autda_dp_1src_combo_4_1800_dac11bc0() {
    // Encoding: 0xDAC11BC0
    // Test aarch64_integer_pac_autda_dp_1src field combination: Z=0, Rn=30, Rd=0
    // Fields: Rn=30, Z=0, Rd=0
    let encoding: u32 = 0xDAC11BC0;
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

/// Provenance: aarch64_integer_pac_autda_dp_1src
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_integer_pac_autda_dp_1src_combo_5_1800_dac11be0() {
    // Encoding: 0xDAC11BE0
    // Test aarch64_integer_pac_autda_dp_1src field combination: Z=0, Rn=31, Rd=0
    // Fields: Rd=0, Rn=31, Z=0
    let encoding: u32 = 0xDAC11BE0;
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

/// Provenance: aarch64_integer_pac_autda_dp_1src
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=0 (register index 0 (first register))
#[test]
fn test_aarch64_integer_pac_autda_dp_1src_combo_6_1800_dac11800() {
    // Encoding: 0xDAC11800
    // Test aarch64_integer_pac_autda_dp_1src field combination: Z=0, Rn=0, Rd=0
    // Fields: Z=0, Rn=0, Rd=0
    let encoding: u32 = 0xDAC11800;
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

/// Provenance: aarch64_integer_pac_autda_dp_1src
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=1 (register index 1 (second register))
#[test]
fn test_aarch64_integer_pac_autda_dp_1src_combo_7_1800_dac11801() {
    // Encoding: 0xDAC11801
    // Test aarch64_integer_pac_autda_dp_1src field combination: Z=0, Rn=0, Rd=1
    // Fields: Z=0, Rd=1, Rn=0
    let encoding: u32 = 0xDAC11801;
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

/// Provenance: aarch64_integer_pac_autda_dp_1src
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_integer_pac_autda_dp_1src_combo_8_1800_dac1181e() {
    // Encoding: 0xDAC1181E
    // Test aarch64_integer_pac_autda_dp_1src field combination: Z=0, Rn=0, Rd=30
    // Fields: Z=0, Rn=0, Rd=30
    let encoding: u32 = 0xDAC1181E;
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

/// Provenance: aarch64_integer_pac_autda_dp_1src
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=31 (register index 31 (ZR - zero register))
#[test]
fn test_aarch64_integer_pac_autda_dp_1src_combo_9_1800_dac1181f() {
    // Encoding: 0xDAC1181F
    // Test aarch64_integer_pac_autda_dp_1src field combination: Z=0, Rn=0, Rd=31
    // Fields: Z=0, Rn=0, Rd=31
    let encoding: u32 = 0xDAC1181F;
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

/// Provenance: aarch64_integer_pac_autda_dp_1src
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (same register test (reg=1)), Rd=1 (same register test (reg=1))
#[test]
fn test_aarch64_integer_pac_autda_dp_1src_combo_10_1800_dac11821() {
    // Encoding: 0xDAC11821
    // Test aarch64_integer_pac_autda_dp_1src field combination: Z=0, Rn=1, Rd=1
    // Fields: Z=0, Rd=1, Rn=1
    let encoding: u32 = 0xDAC11821;
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

/// Provenance: aarch64_integer_pac_autda_dp_1src
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (same register test (reg=31)), Rd=31 (same register test (reg=31))
#[test]
fn test_aarch64_integer_pac_autda_dp_1src_combo_11_1800_dac11bff() {
    // Encoding: 0xDAC11BFF
    // Test aarch64_integer_pac_autda_dp_1src field combination: Z=0, Rn=31, Rd=31
    // Fields: Rd=31, Z=0, Rn=31
    let encoding: u32 = 0xDAC11BFF;
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

/// Provenance: aarch64_integer_pac_autda_dp_1src
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_integer_pac_autda_dp_1src_special_rn_31_stack_pointer_sp_may_require_alignment_6144_dac11be0()
 {
    // Encoding: 0xDAC11BE0
    // Test aarch64_integer_pac_autda_dp_1src special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: Rn=31, Z=0, Rd=0
    let encoding: u32 = 0xDAC11BE0;
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

/// Provenance: aarch64_integer_pac_autda_dp_1src
/// ASL: `field Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)`
/// Requirement: FieldSpecial { field: "Rd", value: 31, meaning: "Zero register (XZR/WZR) - reads as 0, writes discarded" }
/// Zero register (XZR/WZR) - reads as 0, writes discarded
#[test]
fn test_aarch64_integer_pac_autda_dp_1src_special_rd_31_zero_register_xzr_wzr_reads_as_0_writes_discarded_6144_dac1181f()
 {
    // Encoding: 0xDAC1181F
    // Test aarch64_integer_pac_autda_dp_1src special value Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)
    // Fields: Z=0, Rd=31, Rn=0
    let encoding: u32 = 0xDAC1181F;
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

/// Provenance: aarch64_integer_pac_autda_dp_1src
/// ASL: `GpFromField("d") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "d" }
/// verify register write to GpFromField("d")
#[test]
fn test_aarch64_integer_pac_autda_dp_1src_reg_write_0_dac11800() {
    // Test aarch64_integer_pac_autda_dp_1src register write: GpFromField("d")
    // Encoding: 0xDAC11800
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xDAC11800;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_integer_pac_autda_dp_1src
/// ASL: `GpFromField("d") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "d" }
/// verify register write to GpFromField("d")
#[test]
fn test_aarch64_integer_pac_autda_dp_1src_reg_write_1_dac11800() {
    // Test aarch64_integer_pac_autda_dp_1src register write: GpFromField("d")
    // Encoding: 0xDAC11800
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xDAC11800;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_integer_pac_autda_dp_1src
/// ASL: `Rn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Rn = 31)
#[test]
fn test_aarch64_integer_pac_autda_dp_1src_sp_rn_dac11be0() {
    // Test aarch64_integer_pac_autda_dp_1src with Rn = SP (31)
    // Encoding: 0xDAC11BE0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xDAC11BE0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_integer_pac_autda_dp_1src
/// ASL: `Rd = 31 (ZR)`
/// Requirement: RegisterSpecial { reg: Zr, behavior: "reads as 0, writes discarded" }
/// zero register (Rd = 31)
#[test]
fn test_aarch64_integer_pac_autda_dp_1src_zr_rd_dac1181f() {
    // Test aarch64_integer_pac_autda_dp_1src with Rd = ZR (31)
    // Encoding: 0xDAC1181F
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xDAC1181F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 31), 0, "XZR should always be 0");
}

// ============================================================================
// aarch64_integer_pac_pacib_dp_1src Tests
// ============================================================================

/// Provenance: aarch64_integer_pac_pacib_dp_1src
/// ASL: `field Z 13 +: 1`
/// Requirement: FieldBoundary { field: "Z", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_integer_pac_pacib_dp_1src_field_z_0_min_400_dac10400() {
    // Encoding: 0xDAC10400
    // Test aarch64_integer_pac_pacib_dp_1src field Z = 0 (Min)
    // Fields: Rn=0, Rd=0, Z=0
    let encoding: u32 = 0xDAC10400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: pacib	x0, x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_pacib_dp_1src
/// ASL: `field Z 13 +: 1`
/// Requirement: FieldBoundary { field: "Z", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_integer_pac_pacib_dp_1src_field_z_1_max_400_dac12400() {
    // Encoding: 0xDAC12400
    // Test aarch64_integer_pac_pacib_dp_1src field Z = 1 (Max)
    // Fields: Rn=0, Rd=0, Z=1
    let encoding: u32 = 0xDAC12400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch64_integer_pac_pacib_dp_1src
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_integer_pac_pacib_dp_1src_field_rn_0_min_400_dac10400() {
    // Encoding: 0xDAC10400
    // Test aarch64_integer_pac_pacib_dp_1src field Rn = 0 (Min)
    // Fields: Rd=0, Z=0, Rn=0
    let encoding: u32 = 0xDAC10400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: pacib	x0, x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_pacib_dp_1src
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_integer_pac_pacib_dp_1src_field_rn_1_poweroftwo_400_dac10420() {
    // Encoding: 0xDAC10420
    // Test aarch64_integer_pac_pacib_dp_1src field Rn = 1 (PowerOfTwo)
    // Fields: Z=0, Rd=0, Rn=1
    let encoding: u32 = 0xDAC10420;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: pacib	x0, x1
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_pacib_dp_1src
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_integer_pac_pacib_dp_1src_field_rn_30_poweroftwominusone_400_dac107c0() {
    // Encoding: 0xDAC107C0
    // Test aarch64_integer_pac_pacib_dp_1src field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: Rn=30, Z=0, Rd=0
    let encoding: u32 = 0xDAC107C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: pacib	x0, x30
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_pacib_dp_1src
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_integer_pac_pacib_dp_1src_field_rn_31_max_400_dac107e0() {
    // Encoding: 0xDAC107E0
    // Test aarch64_integer_pac_pacib_dp_1src field Rn = 31 (Max)
    // Fields: Z=0, Rn=31, Rd=0
    let encoding: u32 = 0xDAC107E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: pacib	x0, sp
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_pacib_dp_1src
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_integer_pac_pacib_dp_1src_field_rd_0_min_400_dac10400() {
    // Encoding: 0xDAC10400
    // Test aarch64_integer_pac_pacib_dp_1src field Rd = 0 (Min)
    // Fields: Rn=0, Rd=0, Z=0
    let encoding: u32 = 0xDAC10400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: pacib	x0, x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_pacib_dp_1src
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_integer_pac_pacib_dp_1src_field_rd_1_poweroftwo_400_dac10401() {
    // Encoding: 0xDAC10401
    // Test aarch64_integer_pac_pacib_dp_1src field Rd = 1 (PowerOfTwo)
    // Fields: Z=0, Rn=0, Rd=1
    let encoding: u32 = 0xDAC10401;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: pacib	x1, x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_pacib_dp_1src
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_integer_pac_pacib_dp_1src_field_rd_30_poweroftwominusone_400_dac1041e() {
    // Encoding: 0xDAC1041E
    // Test aarch64_integer_pac_pacib_dp_1src field Rd = 30 (PowerOfTwoMinusOne)
    // Fields: Rn=0, Rd=30, Z=0
    let encoding: u32 = 0xDAC1041E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: pacib	x30, x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_pacib_dp_1src
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 31, boundary: Max }
/// register index 31 (ZR - zero register)
#[test]
fn test_aarch64_integer_pac_pacib_dp_1src_field_rd_31_max_400_dac1041f() {
    // Encoding: 0xDAC1041F
    // Test aarch64_integer_pac_pacib_dp_1src field Rd = 31 (Max)
    // Fields: Z=0, Rn=0, Rd=31
    let encoding: u32 = 0xDAC1041F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: pacib	xzr, x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_pacib_dp_1src
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Z=0 (minimum value)
#[test]
fn test_aarch64_integer_pac_pacib_dp_1src_combo_0_400_dac10400() {
    // Encoding: 0xDAC10400
    // Test aarch64_integer_pac_pacib_dp_1src field combination: Z=0, Rn=0, Rd=0
    // Fields: Rd=0, Rn=0, Z=0
    let encoding: u32 = 0xDAC10400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: pacib	x0, x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_pacib_dp_1src
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Z=1 (maximum value (1))
#[test]
fn test_aarch64_integer_pac_pacib_dp_1src_combo_1_400_dac12400() {
    // Encoding: 0xDAC12400
    // Test aarch64_integer_pac_pacib_dp_1src field combination: Z=1, Rn=0, Rd=0
    // Fields: Rd=0, Rn=0, Z=1
    let encoding: u32 = 0xDAC12400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch64_integer_pac_pacib_dp_1src
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch64_integer_pac_pacib_dp_1src_combo_2_400_dac10400() {
    // Encoding: 0xDAC10400
    // Test aarch64_integer_pac_pacib_dp_1src field combination: Z=0, Rn=0, Rd=0
    // Fields: Rd=0, Z=0, Rn=0
    let encoding: u32 = 0xDAC10400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: pacib	x0, x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_pacib_dp_1src
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_aarch64_integer_pac_pacib_dp_1src_combo_3_400_dac10420() {
    // Encoding: 0xDAC10420
    // Test aarch64_integer_pac_pacib_dp_1src field combination: Z=0, Rn=1, Rd=0
    // Fields: Rd=0, Z=0, Rn=1
    let encoding: u32 = 0xDAC10420;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: pacib	x0, x1
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_pacib_dp_1src
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_integer_pac_pacib_dp_1src_combo_4_400_dac107c0() {
    // Encoding: 0xDAC107C0
    // Test aarch64_integer_pac_pacib_dp_1src field combination: Z=0, Rn=30, Rd=0
    // Fields: Z=0, Rn=30, Rd=0
    let encoding: u32 = 0xDAC107C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: pacib	x0, x30
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_pacib_dp_1src
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_integer_pac_pacib_dp_1src_combo_5_400_dac107e0() {
    // Encoding: 0xDAC107E0
    // Test aarch64_integer_pac_pacib_dp_1src field combination: Z=0, Rn=31, Rd=0
    // Fields: Rd=0, Rn=31, Z=0
    let encoding: u32 = 0xDAC107E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: pacib	x0, sp
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_pacib_dp_1src
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=0 (register index 0 (first register))
#[test]
fn test_aarch64_integer_pac_pacib_dp_1src_combo_6_400_dac10400() {
    // Encoding: 0xDAC10400
    // Test aarch64_integer_pac_pacib_dp_1src field combination: Z=0, Rn=0, Rd=0
    // Fields: Z=0, Rd=0, Rn=0
    let encoding: u32 = 0xDAC10400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: pacib	x0, x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_pacib_dp_1src
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=1 (register index 1 (second register))
#[test]
fn test_aarch64_integer_pac_pacib_dp_1src_combo_7_400_dac10401() {
    // Encoding: 0xDAC10401
    // Test aarch64_integer_pac_pacib_dp_1src field combination: Z=0, Rn=0, Rd=1
    // Fields: Z=0, Rd=1, Rn=0
    let encoding: u32 = 0xDAC10401;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: pacib	x1, x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_pacib_dp_1src
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_integer_pac_pacib_dp_1src_combo_8_400_dac1041e() {
    // Encoding: 0xDAC1041E
    // Test aarch64_integer_pac_pacib_dp_1src field combination: Z=0, Rn=0, Rd=30
    // Fields: Rn=0, Z=0, Rd=30
    let encoding: u32 = 0xDAC1041E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: pacib	x30, x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_pacib_dp_1src
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=31 (register index 31 (ZR - zero register))
#[test]
fn test_aarch64_integer_pac_pacib_dp_1src_combo_9_400_dac1041f() {
    // Encoding: 0xDAC1041F
    // Test aarch64_integer_pac_pacib_dp_1src field combination: Z=0, Rn=0, Rd=31
    // Fields: Z=0, Rd=31, Rn=0
    let encoding: u32 = 0xDAC1041F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: pacib	xzr, x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_pacib_dp_1src
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (same register test (reg=1)), Rd=1 (same register test (reg=1))
#[test]
fn test_aarch64_integer_pac_pacib_dp_1src_combo_10_400_dac10421() {
    // Encoding: 0xDAC10421
    // Test aarch64_integer_pac_pacib_dp_1src field combination: Z=0, Rn=1, Rd=1
    // Fields: Rn=1, Rd=1, Z=0
    let encoding: u32 = 0xDAC10421;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: pacib	x1, x1
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_pacib_dp_1src
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (same register test (reg=31)), Rd=31 (same register test (reg=31))
#[test]
fn test_aarch64_integer_pac_pacib_dp_1src_combo_11_400_dac107ff() {
    // Encoding: 0xDAC107FF
    // Test aarch64_integer_pac_pacib_dp_1src field combination: Z=0, Rn=31, Rd=31
    // Fields: Z=0, Rn=31, Rd=31
    let encoding: u32 = 0xDAC107FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: pacib	xzr, sp
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_pacib_dp_1src
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_integer_pac_pacib_dp_1src_special_rn_31_stack_pointer_sp_may_require_alignment_1024_dac107e0()
 {
    // Encoding: 0xDAC107E0
    // Test aarch64_integer_pac_pacib_dp_1src special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: Rn=31, Z=0, Rd=0
    let encoding: u32 = 0xDAC107E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: pacib	x0, sp
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_pacib_dp_1src
/// ASL: `field Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)`
/// Requirement: FieldSpecial { field: "Rd", value: 31, meaning: "Zero register (XZR/WZR) - reads as 0, writes discarded" }
/// Zero register (XZR/WZR) - reads as 0, writes discarded
#[test]
fn test_aarch64_integer_pac_pacib_dp_1src_special_rd_31_zero_register_xzr_wzr_reads_as_0_writes_discarded_1024_dac1041f()
 {
    // Encoding: 0xDAC1041F
    // Test aarch64_integer_pac_pacib_dp_1src special value Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)
    // Fields: Z=0, Rn=0, Rd=31
    let encoding: u32 = 0xDAC1041F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: pacib	xzr, x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_pacib_hint
/// ASL: `field CRm 8 +: 4`
/// Requirement: FieldBoundary { field: "CRm", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_integer_pac_pacib_hint_field_crm_0_min_201f_d503201f() {
    // Encoding: 0xD503201F
    // Test aarch64_integer_pac_pacib_hint field CRm = 0 (Min)
    // Fields: CRm=0, op2=0
    let encoding: u32 = 0xD503201F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: nop
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_pacib_hint
/// ASL: `field CRm 8 +: 4`
/// Requirement: FieldBoundary { field: "CRm", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_aarch64_integer_pac_pacib_hint_field_crm_1_poweroftwo_201f_d503211f() {
    // Encoding: 0xD503211F
    // Test aarch64_integer_pac_pacib_hint field CRm = 1 (PowerOfTwo)
    // Fields: CRm=1, op2=0
    let encoding: u32 = 0xD503211F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: pacia1716
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_pacib_hint
/// ASL: `field CRm 8 +: 4`
/// Requirement: FieldBoundary { field: "CRm", value: 7, boundary: PowerOfTwoMinusOne }
/// midpoint (7)
#[test]
fn test_aarch64_integer_pac_pacib_hint_field_crm_7_poweroftwominusone_201f_d503271f() {
    // Encoding: 0xD503271F
    // Test aarch64_integer_pac_pacib_hint field CRm = 7 (PowerOfTwoMinusOne)
    // Fields: op2=0, CRm=7
    let encoding: u32 = 0xD503271F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: hint	#56
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_pacib_hint
/// ASL: `field CRm 8 +: 4`
/// Requirement: FieldBoundary { field: "CRm", value: 15, boundary: Max }
/// maximum value (15)
#[test]
fn test_aarch64_integer_pac_pacib_hint_field_crm_15_max_201f_d5032f1f() {
    // Encoding: 0xD5032F1F
    // Test aarch64_integer_pac_pacib_hint field CRm = 15 (Max)
    // Fields: CRm=15, op2=0
    let encoding: u32 = 0xD5032F1F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: hint	#120
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_pacib_hint
/// ASL: `field op2 5 +: 3`
/// Requirement: FieldBoundary { field: "op2", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_integer_pac_pacib_hint_field_op2_0_min_201f_d503201f() {
    // Encoding: 0xD503201F
    // Test aarch64_integer_pac_pacib_hint field op2 = 0 (Min)
    // Fields: CRm=0, op2=0
    let encoding: u32 = 0xD503201F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: nop
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_pacib_hint
/// ASL: `field op2 5 +: 3`
/// Requirement: FieldBoundary { field: "op2", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_aarch64_integer_pac_pacib_hint_field_op2_1_poweroftwo_201f_d503203f() {
    // Encoding: 0xD503203F
    // Test aarch64_integer_pac_pacib_hint field op2 = 1 (PowerOfTwo)
    // Fields: CRm=0, op2=1
    let encoding: u32 = 0xD503203F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: yield
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_pacib_hint
/// ASL: `field op2 5 +: 3`
/// Requirement: FieldBoundary { field: "op2", value: 7, boundary: Max }
/// maximum value (7)
#[test]
fn test_aarch64_integer_pac_pacib_hint_field_op2_7_max_201f_d50320ff() {
    // Encoding: 0xD50320FF
    // Test aarch64_integer_pac_pacib_hint field op2 = 7 (Max)
    // Fields: CRm=0, op2=7
    let encoding: u32 = 0xD50320FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: xpaclri
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_pacib_hint
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// CRm=0 (minimum value)
#[test]
fn test_aarch64_integer_pac_pacib_hint_combo_0_201f_d503201f() {
    // Encoding: 0xD503201F
    // Test aarch64_integer_pac_pacib_hint field combination: CRm=0, op2=0
    // Fields: op2=0, CRm=0
    let encoding: u32 = 0xD503201F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: nop
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_pacib_hint
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// CRm=1 (value 1)
#[test]
fn test_aarch64_integer_pac_pacib_hint_combo_1_201f_d503211f() {
    // Encoding: 0xD503211F
    // Test aarch64_integer_pac_pacib_hint field combination: CRm=1, op2=0
    // Fields: CRm=1, op2=0
    let encoding: u32 = 0xD503211F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: pacia1716
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_pacib_hint
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// CRm=7 (midpoint (7))
#[test]
fn test_aarch64_integer_pac_pacib_hint_combo_2_201f_d503271f() {
    // Encoding: 0xD503271F
    // Test aarch64_integer_pac_pacib_hint field combination: CRm=7, op2=0
    // Fields: op2=0, CRm=7
    let encoding: u32 = 0xD503271F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: hint	#56
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_pacib_hint
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// CRm=15 (maximum value (15))
#[test]
fn test_aarch64_integer_pac_pacib_hint_combo_3_201f_d5032f1f() {
    // Encoding: 0xD5032F1F
    // Test aarch64_integer_pac_pacib_hint field combination: CRm=15, op2=0
    // Fields: op2=0, CRm=15
    let encoding: u32 = 0xD5032F1F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: hint	#120
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_pacib_hint
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// op2=0 (minimum value)
#[test]
fn test_aarch64_integer_pac_pacib_hint_combo_4_201f_d503201f() {
    // Encoding: 0xD503201F
    // Test aarch64_integer_pac_pacib_hint field combination: CRm=0, op2=0
    // Fields: op2=0, CRm=0
    let encoding: u32 = 0xD503201F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: nop
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_pacib_hint
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// op2=1 (value 1)
#[test]
fn test_aarch64_integer_pac_pacib_hint_combo_5_201f_d503203f() {
    // Encoding: 0xD503203F
    // Test aarch64_integer_pac_pacib_hint field combination: CRm=0, op2=1
    // Fields: op2=1, CRm=0
    let encoding: u32 = 0xD503203F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: yield
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_pacib_hint
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// op2=7 (maximum value (7))
#[test]
fn test_aarch64_integer_pac_pacib_hint_combo_6_201f_d50320ff() {
    // Encoding: 0xD50320FF
    // Test aarch64_integer_pac_pacib_hint field combination: CRm=0, op2=7
    // Fields: CRm=0, op2=7
    let encoding: u32 = 0xD50320FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: xpaclri
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_pac_pacib_dp_1src
/// ASL: `GpFromField("d") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "d" }
/// verify register write to GpFromField("d")
#[test]
fn test_aarch64_integer_pac_pacib_dp_1src_reg_write_0_dac10400() {
    // Test aarch64_integer_pac_pacib_dp_1src register write: GpFromField("d")
    // Encoding: 0xDAC10400
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xDAC10400;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_integer_pac_pacib_dp_1src
/// ASL: `GpFromField("d") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "d" }
/// verify register write to GpFromField("d")
#[test]
fn test_aarch64_integer_pac_pacib_dp_1src_reg_write_1_dac10400() {
    // Test aarch64_integer_pac_pacib_dp_1src register write: GpFromField("d")
    // Encoding: 0xDAC10400
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xDAC10400;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_integer_pac_pacib_dp_1src
/// ASL: `Rn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Rn = 31)
#[test]
fn test_aarch64_integer_pac_pacib_dp_1src_sp_rn_dac107e0() {
    // Test aarch64_integer_pac_pacib_dp_1src with Rn = SP (31)
    // Encoding: 0xDAC107E0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xDAC107E0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_integer_pac_pacib_dp_1src
/// ASL: `Rd = 31 (ZR)`
/// Requirement: RegisterSpecial { reg: Zr, behavior: "reads as 0, writes discarded" }
/// zero register (Rd = 31)
#[test]
fn test_aarch64_integer_pac_pacib_dp_1src_zr_rd_dac1041f() {
    // Test aarch64_integer_pac_pacib_dp_1src with Rd = ZR (31)
    // Encoding: 0xDAC1041F
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xDAC1041F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 31), 0, "XZR should always be 0");
}

/// Provenance: aarch64_integer_pac_pacib_hint
/// ASL: `GpFromField("d") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "d" }
/// verify register write to GpFromField("d")
#[test]
fn test_aarch64_integer_pac_pacib_hint_reg_write_0_d503201f() {
    // Test aarch64_integer_pac_pacib_hint register write: GpFromField("d")
    // Encoding: 0xD503201F
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xD503201F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_integer_pac_pacib_hint
/// ASL: `GpFromField("d") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "d" }
/// verify register write to GpFromField("d")
#[test]
fn test_aarch64_integer_pac_pacib_hint_reg_write_1_d503201f() {
    // Test aarch64_integer_pac_pacib_hint register write: GpFromField("d")
    // Encoding: 0xD503201F
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xD503201F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}
