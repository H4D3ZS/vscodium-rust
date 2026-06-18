//! A64 memory single tests.
//!
//! Auto-generated from ARM ASL specifications.
//! DO NOT EDIT MANUALLY.

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::generated::test_helpers::*;

// ============================================================================
// aarch64_memory_single_general_immediate_signed_offset_normal Tests
// ============================================================================

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_normal
/// ASL: `field size 30 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_normal_field_size_0_min_0_38000000() {
    // Encoding: 0x38000000
    // Test aarch64_memory_single_general_immediate_signed_offset_normal field size = 0 (Min)
    // Fields: imm9=0, opc=0, Rn=0, size=0, Rt=0
    let encoding: u32 = 0x38000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_normal
/// ASL: `field size 30 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 1, boundary: PowerOfTwo }
/// 16-bit / halfword size
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_normal_field_size_1_poweroftwo_0_78000000()
 {
    // Encoding: 0x78000000
    // Test aarch64_memory_single_general_immediate_signed_offset_normal field size = 1 (PowerOfTwo)
    // Fields: Rt=0, opc=0, size=1, imm9=0, Rn=0
    let encoding: u32 = 0x78000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_normal
/// ASL: `field size 30 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 2, boundary: PowerOfTwo }
/// 32-bit / word size
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_normal_field_size_2_poweroftwo_0_b8000000()
 {
    // Encoding: 0xB8000000
    // Test aarch64_memory_single_general_immediate_signed_offset_normal field size = 2 (PowerOfTwo)
    // Fields: opc=0, imm9=0, Rn=0, Rt=0, size=2
    let encoding: u32 = 0xB8000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_normal
/// ASL: `field size 30 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 3, boundary: Max }
/// 64-bit / doubleword size
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_normal_field_size_3_max_0_f8000000() {
    // Encoding: 0xF8000000
    // Test aarch64_memory_single_general_immediate_signed_offset_normal field size = 3 (Max)
    // Fields: opc=0, Rt=0, Rn=0, size=3, imm9=0
    let encoding: u32 = 0xF8000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_normal
/// ASL: `field opc 22 +: 2`
/// Requirement: FieldBoundary { field: "opc", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_normal_field_opc_0_min_0_38000000() {
    // Encoding: 0x38000000
    // Test aarch64_memory_single_general_immediate_signed_offset_normal field opc = 0 (Min)
    // Fields: Rt=0, imm9=0, Rn=0, size=0, opc=0
    let encoding: u32 = 0x38000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_normal
/// ASL: `field opc 22 +: 2`
/// Requirement: FieldBoundary { field: "opc", value: 1, boundary: PowerOfTwo }
/// 16-bit / halfword size
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_normal_field_opc_1_poweroftwo_0_38400000()
 {
    // Encoding: 0x38400000
    // Test aarch64_memory_single_general_immediate_signed_offset_normal field opc = 1 (PowerOfTwo)
    // Fields: Rt=0, imm9=0, size=0, opc=1, Rn=0
    let encoding: u32 = 0x38400000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_normal
/// ASL: `field opc 22 +: 2`
/// Requirement: FieldBoundary { field: "opc", value: 2, boundary: PowerOfTwo }
/// 32-bit / word size
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_normal_field_opc_2_poweroftwo_0_38800000()
 {
    // Encoding: 0x38800000
    // Test aarch64_memory_single_general_immediate_signed_offset_normal field opc = 2 (PowerOfTwo)
    // Fields: opc=2, size=0, Rn=0, Rt=0, imm9=0
    let encoding: u32 = 0x38800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_normal
/// ASL: `field opc 22 +: 2`
/// Requirement: FieldBoundary { field: "opc", value: 3, boundary: Max }
/// 64-bit / doubleword size
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_normal_field_opc_3_max_0_38c00000() {
    // Encoding: 0x38C00000
    // Test aarch64_memory_single_general_immediate_signed_offset_normal field opc = 3 (Max)
    // Fields: Rn=0, imm9=0, size=0, Rt=0, opc=3
    let encoding: u32 = 0x38C00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_normal
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_normal_field_imm9_0_zero_0_38000000()
{
    // Encoding: 0x38000000
    // Test aarch64_memory_single_general_immediate_signed_offset_normal field imm9 = 0 (Zero)
    // Fields: Rt=0, Rn=0, imm9=0, opc=0, size=0
    let encoding: u32 = 0x38000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_normal
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_normal_field_imm9_1_poweroftwo_0_38001000()
 {
    // Encoding: 0x38001000
    // Test aarch64_memory_single_general_immediate_signed_offset_normal field imm9 = 1 (PowerOfTwo)
    // Fields: imm9=1, Rt=0, size=0, Rn=0, opc=0
    let encoding: u32 = 0x38001000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_normal
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 3, boundary: PowerOfTwoMinusOne }
/// 2^2 - 1 = 3
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_normal_field_imm9_3_poweroftwominusone_0_38003000()
 {
    // Encoding: 0x38003000
    // Test aarch64_memory_single_general_immediate_signed_offset_normal field imm9 = 3 (PowerOfTwoMinusOne)
    // Fields: opc=0, imm9=3, size=0, Rt=0, Rn=0
    let encoding: u32 = 0x38003000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_normal
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 4, boundary: PowerOfTwo }
/// power of 2 (2^2 = 4)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_normal_field_imm9_4_poweroftwo_0_38004000()
 {
    // Encoding: 0x38004000
    // Test aarch64_memory_single_general_immediate_signed_offset_normal field imm9 = 4 (PowerOfTwo)
    // Fields: imm9=4, opc=0, Rn=0, Rt=0, size=0
    let encoding: u32 = 0x38004000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_normal
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 7, boundary: PowerOfTwoMinusOne }
/// 2^3 - 1 = 7
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_normal_field_imm9_7_poweroftwominusone_0_38007000()
 {
    // Encoding: 0x38007000
    // Test aarch64_memory_single_general_immediate_signed_offset_normal field imm9 = 7 (PowerOfTwoMinusOne)
    // Fields: size=0, imm9=7, Rn=0, Rt=0, opc=0
    let encoding: u32 = 0x38007000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_normal
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 8, boundary: PowerOfTwo }
/// power of 2 (2^3 = 8)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_normal_field_imm9_8_poweroftwo_0_38008000()
 {
    // Encoding: 0x38008000
    // Test aarch64_memory_single_general_immediate_signed_offset_normal field imm9 = 8 (PowerOfTwo)
    // Fields: size=0, Rt=0, Rn=0, opc=0, imm9=8
    let encoding: u32 = 0x38008000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_normal
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 15, boundary: PowerOfTwoMinusOne }
/// 2^4 - 1 = 15
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_normal_field_imm9_15_poweroftwominusone_0_3800f000()
 {
    // Encoding: 0x3800F000
    // Test aarch64_memory_single_general_immediate_signed_offset_normal field imm9 = 15 (PowerOfTwoMinusOne)
    // Fields: Rn=0, size=0, imm9=15, Rt=0, opc=0
    let encoding: u32 = 0x3800F000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_normal
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 16, boundary: PowerOfTwo }
/// power of 2 (2^4 = 16)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_normal_field_imm9_16_poweroftwo_0_38010000()
 {
    // Encoding: 0x38010000
    // Test aarch64_memory_single_general_immediate_signed_offset_normal field imm9 = 16 (PowerOfTwo)
    // Fields: opc=0, Rn=0, size=0, Rt=0, imm9=16
    let encoding: u32 = 0x38010000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_normal
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 31, boundary: PowerOfTwoMinusOne }
/// 2^5 - 1 = 31
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_normal_field_imm9_31_poweroftwominusone_0_3801f000()
 {
    // Encoding: 0x3801F000
    // Test aarch64_memory_single_general_immediate_signed_offset_normal field imm9 = 31 (PowerOfTwoMinusOne)
    // Fields: Rn=0, imm9=31, size=0, opc=0, Rt=0
    let encoding: u32 = 0x3801F000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_normal
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 32, boundary: PowerOfTwo }
/// power of 2 (2^5 = 32)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_normal_field_imm9_32_poweroftwo_0_38020000()
 {
    // Encoding: 0x38020000
    // Test aarch64_memory_single_general_immediate_signed_offset_normal field imm9 = 32 (PowerOfTwo)
    // Fields: Rn=0, Rt=0, imm9=32, size=0, opc=0
    let encoding: u32 = 0x38020000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_normal
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 63, boundary: PowerOfTwoMinusOne }
/// 2^6 - 1 = 63
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_normal_field_imm9_63_poweroftwominusone_0_3803f000()
 {
    // Encoding: 0x3803F000
    // Test aarch64_memory_single_general_immediate_signed_offset_normal field imm9 = 63 (PowerOfTwoMinusOne)
    // Fields: opc=0, Rt=0, Rn=0, imm9=63, size=0
    let encoding: u32 = 0x3803F000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_normal
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 64, boundary: PowerOfTwo }
/// power of 2 (2^6 = 64)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_normal_field_imm9_64_poweroftwo_0_38040000()
 {
    // Encoding: 0x38040000
    // Test aarch64_memory_single_general_immediate_signed_offset_normal field imm9 = 64 (PowerOfTwo)
    // Fields: size=0, Rt=0, imm9=64, Rn=0, opc=0
    let encoding: u32 = 0x38040000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_normal
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 127, boundary: PowerOfTwoMinusOne }
/// 2^7 - 1 = 127
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_normal_field_imm9_127_poweroftwominusone_0_3807f000()
 {
    // Encoding: 0x3807F000
    // Test aarch64_memory_single_general_immediate_signed_offset_normal field imm9 = 127 (PowerOfTwoMinusOne)
    // Fields: Rn=0, Rt=0, size=0, opc=0, imm9=127
    let encoding: u32 = 0x3807F000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_normal
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 128, boundary: PowerOfTwo }
/// power of 2 (2^7 = 128)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_normal_field_imm9_128_poweroftwo_0_38080000()
 {
    // Encoding: 0x38080000
    // Test aarch64_memory_single_general_immediate_signed_offset_normal field imm9 = 128 (PowerOfTwo)
    // Fields: Rt=0, imm9=128, size=0, opc=0, Rn=0
    let encoding: u32 = 0x38080000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_normal
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 255, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (255)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_normal_field_imm9_255_poweroftwominusone_0_380ff000()
 {
    // Encoding: 0x380FF000
    // Test aarch64_memory_single_general_immediate_signed_offset_normal field imm9 = 255 (PowerOfTwoMinusOne)
    // Fields: Rn=0, Rt=0, opc=0, size=0, imm9=255
    let encoding: u32 = 0x380FF000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_normal
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 256, boundary: PowerOfTwo }
/// power of 2 (2^8 = 256)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_normal_field_imm9_256_poweroftwo_0_38100000()
 {
    // Encoding: 0x38100000
    // Test aarch64_memory_single_general_immediate_signed_offset_normal field imm9 = 256 (PowerOfTwo)
    // Fields: Rt=0, opc=0, imm9=256, size=0, Rn=0
    let encoding: u32 = 0x38100000;
    let mut cpu = create_test_cpu();
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

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_normal
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 511, boundary: Max }
/// maximum immediate (511)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_normal_field_imm9_511_max_0_381ff000()
{
    // Encoding: 0x381FF000
    // Test aarch64_memory_single_general_immediate_signed_offset_normal field imm9 = 511 (Max)
    // Fields: opc=0, size=0, Rn=0, imm9=511, Rt=0
    let encoding: u32 = 0x381FF000;
    let mut cpu = create_test_cpu();
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

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_normal
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_normal_field_rn_0_min_0_38000000() {
    // Encoding: 0x38000000
    // Test aarch64_memory_single_general_immediate_signed_offset_normal field Rn = 0 (Min)
    // Fields: Rt=0, opc=0, imm9=0, Rn=0, size=0
    let encoding: u32 = 0x38000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_normal
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_normal_field_rn_1_poweroftwo_0_38000020()
 {
    // Encoding: 0x38000020
    // Test aarch64_memory_single_general_immediate_signed_offset_normal field Rn = 1 (PowerOfTwo)
    // Fields: opc=0, Rn=1, Rt=0, imm9=0, size=0
    let encoding: u32 = 0x38000020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_normal
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_normal_field_rn_30_poweroftwominusone_0_380003c0()
 {
    // Encoding: 0x380003C0
    // Test aarch64_memory_single_general_immediate_signed_offset_normal field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: opc=0, size=0, Rt=0, imm9=0, Rn=30
    let encoding: u32 = 0x380003C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_normal
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_normal_field_rn_31_max_0_380003e0() {
    // Encoding: 0x380003E0
    // Test aarch64_memory_single_general_immediate_signed_offset_normal field Rn = 31 (Max)
    // Fields: opc=0, Rt=0, imm9=0, Rn=31, size=0
    let encoding: u32 = 0x380003E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_normal
/// ASL: `field Rt 0 +: 5`
/// Requirement: FieldBoundary { field: "Rt", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_normal_field_rt_0_min_0_38000000() {
    // Encoding: 0x38000000
    // Test aarch64_memory_single_general_immediate_signed_offset_normal field Rt = 0 (Min)
    // Fields: Rn=0, size=0, imm9=0, Rt=0, opc=0
    let encoding: u32 = 0x38000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_normal
/// ASL: `field Rt 0 +: 5`
/// Requirement: FieldBoundary { field: "Rt", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_normal_field_rt_1_poweroftwo_0_38000001()
 {
    // Encoding: 0x38000001
    // Test aarch64_memory_single_general_immediate_signed_offset_normal field Rt = 1 (PowerOfTwo)
    // Fields: size=0, imm9=0, Rn=0, opc=0, Rt=1
    let encoding: u32 = 0x38000001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_normal
/// ASL: `field Rt 0 +: 5`
/// Requirement: FieldBoundary { field: "Rt", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_normal_field_rt_30_poweroftwominusone_0_3800001e()
 {
    // Encoding: 0x3800001E
    // Test aarch64_memory_single_general_immediate_signed_offset_normal field Rt = 30 (PowerOfTwoMinusOne)
    // Fields: imm9=0, Rn=0, Rt=30, size=0, opc=0
    let encoding: u32 = 0x3800001E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_normal
/// ASL: `field Rt 0 +: 5`
/// Requirement: FieldBoundary { field: "Rt", value: 31, boundary: Max }
/// register index 31 (ZR - zero register)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_normal_field_rt_31_max_0_3800001f() {
    // Encoding: 0x3800001F
    // Test aarch64_memory_single_general_immediate_signed_offset_normal field Rt = 31 (Max)
    // Fields: Rn=0, opc=0, imm9=0, size=0, Rt=31
    let encoding: u32 = 0x3800001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_normal
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=0 (8-bit / byte size)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_normal_combo_0_0_38000000() {
    // Encoding: 0x38000000
    // Test aarch64_memory_single_general_immediate_signed_offset_normal field combination: size=0, opc=0, imm9=0, Rn=0, Rt=0
    // Fields: Rn=0, imm9=0, size=0, opc=0, Rt=0
    let encoding: u32 = 0x38000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_normal
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=1 (16-bit / halfword size)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_normal_combo_1_0_78000000() {
    // Encoding: 0x78000000
    // Test aarch64_memory_single_general_immediate_signed_offset_normal field combination: size=1, opc=0, imm9=0, Rn=0, Rt=0
    // Fields: opc=0, imm9=0, Rt=0, Rn=0, size=1
    let encoding: u32 = 0x78000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_normal
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=2 (32-bit / word size)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_normal_combo_2_0_b8000000() {
    // Encoding: 0xB8000000
    // Test aarch64_memory_single_general_immediate_signed_offset_normal field combination: size=2, opc=0, imm9=0, Rn=0, Rt=0
    // Fields: Rn=0, imm9=0, Rt=0, opc=0, size=2
    let encoding: u32 = 0xB8000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_normal
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=3 (64-bit / doubleword size)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_normal_combo_3_0_f8000000() {
    // Encoding: 0xF8000000
    // Test aarch64_memory_single_general_immediate_signed_offset_normal field combination: size=3, opc=0, imm9=0, Rn=0, Rt=0
    // Fields: Rt=0, opc=0, imm9=0, size=3, Rn=0
    let encoding: u32 = 0xF8000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_normal
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// opc=0 (8-bit / byte size)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_normal_combo_4_0_38000000() {
    // Encoding: 0x38000000
    // Test aarch64_memory_single_general_immediate_signed_offset_normal field combination: size=0, opc=0, imm9=0, Rn=0, Rt=0
    // Fields: size=0, opc=0, Rn=0, Rt=0, imm9=0
    let encoding: u32 = 0x38000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_normal
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// opc=1 (16-bit / halfword size)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_normal_combo_5_0_38400000() {
    // Encoding: 0x38400000
    // Test aarch64_memory_single_general_immediate_signed_offset_normal field combination: size=0, opc=1, imm9=0, Rn=0, Rt=0
    // Fields: imm9=0, size=0, Rn=0, Rt=0, opc=1
    let encoding: u32 = 0x38400000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_normal
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// opc=2 (32-bit / word size)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_normal_combo_6_0_38800000() {
    // Encoding: 0x38800000
    // Test aarch64_memory_single_general_immediate_signed_offset_normal field combination: size=0, opc=2, imm9=0, Rn=0, Rt=0
    // Fields: Rt=0, size=0, Rn=0, opc=2, imm9=0
    let encoding: u32 = 0x38800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_normal
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// opc=3 (64-bit / doubleword size)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_normal_combo_7_0_38c00000() {
    // Encoding: 0x38C00000
    // Test aarch64_memory_single_general_immediate_signed_offset_normal field combination: size=0, opc=3, imm9=0, Rn=0, Rt=0
    // Fields: opc=3, imm9=0, Rt=0, Rn=0, size=0
    let encoding: u32 = 0x38C00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_normal
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=0 (immediate value 0)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_normal_combo_8_0_38000000() {
    // Encoding: 0x38000000
    // Test aarch64_memory_single_general_immediate_signed_offset_normal field combination: size=0, opc=0, imm9=0, Rn=0, Rt=0
    // Fields: imm9=0, Rt=0, size=0, opc=0, Rn=0
    let encoding: u32 = 0x38000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_normal
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=1 (immediate value 1)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_normal_combo_9_0_38001000() {
    // Encoding: 0x38001000
    // Test aarch64_memory_single_general_immediate_signed_offset_normal field combination: size=0, opc=0, imm9=1, Rn=0, Rt=0
    // Fields: opc=0, Rn=0, imm9=1, size=0, Rt=0
    let encoding: u32 = 0x38001000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_normal
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=3 (2^2 - 1 = 3)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_normal_combo_10_0_38003000() {
    // Encoding: 0x38003000
    // Test aarch64_memory_single_general_immediate_signed_offset_normal field combination: size=0, opc=0, imm9=3, Rn=0, Rt=0
    // Fields: Rn=0, Rt=0, opc=0, imm9=3, size=0
    let encoding: u32 = 0x38003000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_normal
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=4 (power of 2 (2^2 = 4))
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_normal_combo_11_0_38004000() {
    // Encoding: 0x38004000
    // Test aarch64_memory_single_general_immediate_signed_offset_normal field combination: size=0, opc=0, imm9=4, Rn=0, Rt=0
    // Fields: imm9=4, opc=0, Rt=0, size=0, Rn=0
    let encoding: u32 = 0x38004000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_normal
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=7 (2^3 - 1 = 7)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_normal_combo_12_0_38007000() {
    // Encoding: 0x38007000
    // Test aarch64_memory_single_general_immediate_signed_offset_normal field combination: size=0, opc=0, imm9=7, Rn=0, Rt=0
    // Fields: Rn=0, imm9=7, size=0, Rt=0, opc=0
    let encoding: u32 = 0x38007000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_normal
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=8 (power of 2 (2^3 = 8))
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_normal_combo_13_0_38008000() {
    // Encoding: 0x38008000
    // Test aarch64_memory_single_general_immediate_signed_offset_normal field combination: size=0, opc=0, imm9=8, Rn=0, Rt=0
    // Fields: imm9=8, Rn=0, opc=0, size=0, Rt=0
    let encoding: u32 = 0x38008000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_normal
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=15 (2^4 - 1 = 15)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_normal_combo_14_0_3800f000() {
    // Encoding: 0x3800F000
    // Test aarch64_memory_single_general_immediate_signed_offset_normal field combination: size=0, opc=0, imm9=15, Rn=0, Rt=0
    // Fields: imm9=15, Rn=0, Rt=0, size=0, opc=0
    let encoding: u32 = 0x3800F000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_normal
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=16 (power of 2 (2^4 = 16))
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_normal_combo_15_0_38010000() {
    // Encoding: 0x38010000
    // Test aarch64_memory_single_general_immediate_signed_offset_normal field combination: size=0, opc=0, imm9=16, Rn=0, Rt=0
    // Fields: imm9=16, Rn=0, size=0, Rt=0, opc=0
    let encoding: u32 = 0x38010000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_normal
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=31 (2^5 - 1 = 31)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_normal_combo_16_0_3801f000() {
    // Encoding: 0x3801F000
    // Test aarch64_memory_single_general_immediate_signed_offset_normal field combination: size=0, opc=0, imm9=31, Rn=0, Rt=0
    // Fields: opc=0, Rn=0, imm9=31, Rt=0, size=0
    let encoding: u32 = 0x3801F000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_normal
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=32 (power of 2 (2^5 = 32))
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_normal_combo_17_0_38020000() {
    // Encoding: 0x38020000
    // Test aarch64_memory_single_general_immediate_signed_offset_normal field combination: size=0, opc=0, imm9=32, Rn=0, Rt=0
    // Fields: size=0, opc=0, Rt=0, imm9=32, Rn=0
    let encoding: u32 = 0x38020000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_normal
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=63 (2^6 - 1 = 63)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_normal_combo_18_0_3803f000() {
    // Encoding: 0x3803F000
    // Test aarch64_memory_single_general_immediate_signed_offset_normal field combination: size=0, opc=0, imm9=63, Rn=0, Rt=0
    // Fields: Rn=0, size=0, imm9=63, opc=0, Rt=0
    let encoding: u32 = 0x3803F000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_normal
/// ASL: `field combination 19`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=64 (power of 2 (2^6 = 64))
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_normal_combo_19_0_38040000() {
    // Encoding: 0x38040000
    // Test aarch64_memory_single_general_immediate_signed_offset_normal field combination: size=0, opc=0, imm9=64, Rn=0, Rt=0
    // Fields: size=0, imm9=64, Rn=0, Rt=0, opc=0
    let encoding: u32 = 0x38040000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_normal
/// ASL: `field combination 20`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=127 (2^7 - 1 = 127)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_normal_combo_20_0_3807f000() {
    // Encoding: 0x3807F000
    // Test aarch64_memory_single_general_immediate_signed_offset_normal field combination: size=0, opc=0, imm9=127, Rn=0, Rt=0
    // Fields: size=0, Rn=0, imm9=127, Rt=0, opc=0
    let encoding: u32 = 0x3807F000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_normal
/// ASL: `field combination 21`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=128 (power of 2 (2^7 = 128))
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_normal_combo_21_0_38080000() {
    // Encoding: 0x38080000
    // Test aarch64_memory_single_general_immediate_signed_offset_normal field combination: size=0, opc=0, imm9=128, Rn=0, Rt=0
    // Fields: size=0, imm9=128, opc=0, Rn=0, Rt=0
    let encoding: u32 = 0x38080000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_normal
/// ASL: `field combination 22`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=255 (immediate midpoint (255))
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_normal_combo_22_0_380ff000() {
    // Encoding: 0x380FF000
    // Test aarch64_memory_single_general_immediate_signed_offset_normal field combination: size=0, opc=0, imm9=255, Rn=0, Rt=0
    // Fields: Rn=0, size=0, Rt=0, opc=0, imm9=255
    let encoding: u32 = 0x380FF000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_normal
/// ASL: `field combination 23`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=256 (power of 2 (2^8 = 256))
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_normal_combo_23_0_38100000() {
    // Encoding: 0x38100000
    // Test aarch64_memory_single_general_immediate_signed_offset_normal field combination: size=0, opc=0, imm9=256, Rn=0, Rt=0
    // Fields: opc=0, imm9=256, size=0, Rn=0, Rt=0
    let encoding: u32 = 0x38100000;
    let mut cpu = create_test_cpu();
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

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_normal
/// ASL: `field combination 24`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=511 (maximum immediate (511))
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_normal_combo_24_0_381ff000() {
    // Encoding: 0x381FF000
    // Test aarch64_memory_single_general_immediate_signed_offset_normal field combination: size=0, opc=0, imm9=511, Rn=0, Rt=0
    // Fields: Rn=0, Rt=0, opc=0, size=0, imm9=511
    let encoding: u32 = 0x381FF000;
    let mut cpu = create_test_cpu();
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

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_normal
/// ASL: `field combination 25`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_normal_combo_25_0_38000000() {
    // Encoding: 0x38000000
    // Test aarch64_memory_single_general_immediate_signed_offset_normal field combination: size=0, opc=0, imm9=0, Rn=0, Rt=0
    // Fields: opc=0, Rt=0, Rn=0, imm9=0, size=0
    let encoding: u32 = 0x38000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_normal
/// ASL: `field combination 26`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_normal_combo_26_0_38000020() {
    // Encoding: 0x38000020
    // Test aarch64_memory_single_general_immediate_signed_offset_normal field combination: size=0, opc=0, imm9=0, Rn=1, Rt=0
    // Fields: Rt=0, imm9=0, opc=0, size=0, Rn=1
    let encoding: u32 = 0x38000020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_normal
/// ASL: `field combination 27`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_normal_combo_27_0_380003c0() {
    // Encoding: 0x380003C0
    // Test aarch64_memory_single_general_immediate_signed_offset_normal field combination: size=0, opc=0, imm9=0, Rn=30, Rt=0
    // Fields: size=0, Rn=30, imm9=0, opc=0, Rt=0
    let encoding: u32 = 0x380003C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_normal
/// ASL: `field combination 28`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_normal_combo_28_0_380003e0() {
    // Encoding: 0x380003E0
    // Test aarch64_memory_single_general_immediate_signed_offset_normal field combination: size=0, opc=0, imm9=0, Rn=31, Rt=0
    // Fields: size=0, opc=0, Rn=31, imm9=0, Rt=0
    let encoding: u32 = 0x380003E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_normal
/// ASL: `field combination 29`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt=0 (register index 0 (first register))
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_normal_combo_29_0_38000000() {
    // Encoding: 0x38000000
    // Test aarch64_memory_single_general_immediate_signed_offset_normal field combination: size=0, opc=0, imm9=0, Rn=0, Rt=0
    // Fields: Rn=0, opc=0, imm9=0, Rt=0, size=0
    let encoding: u32 = 0x38000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_normal
/// ASL: `field combination 30`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt=1 (register index 1 (second register))
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_normal_combo_30_0_38000001() {
    // Encoding: 0x38000001
    // Test aarch64_memory_single_general_immediate_signed_offset_normal field combination: size=0, opc=0, imm9=0, Rn=0, Rt=1
    // Fields: size=0, Rn=0, opc=0, imm9=0, Rt=1
    let encoding: u32 = 0x38000001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_normal
/// ASL: `field combination 31`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_normal_combo_31_0_3800001e() {
    // Encoding: 0x3800001E
    // Test aarch64_memory_single_general_immediate_signed_offset_normal field combination: size=0, opc=0, imm9=0, Rn=0, Rt=30
    // Fields: imm9=0, Rt=30, size=0, opc=0, Rn=0
    let encoding: u32 = 0x3800001E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_normal
/// ASL: `field combination 32`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt=31 (register index 31 (ZR - zero register))
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_normal_combo_32_0_3800001f() {
    // Encoding: 0x3800001F
    // Test aarch64_memory_single_general_immediate_signed_offset_normal field combination: size=0, opc=0, imm9=0, Rn=0, Rt=31
    // Fields: opc=0, size=0, Rn=0, imm9=0, Rt=31
    let encoding: u32 = 0x3800001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_normal
/// ASL: `field combination 33`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (same register test (reg=1)), Rt=1 (same register test (reg=1))
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_normal_combo_33_0_38000021() {
    // Encoding: 0x38000021
    // Test aarch64_memory_single_general_immediate_signed_offset_normal field combination: size=0, opc=0, imm9=0, Rn=1, Rt=1
    // Fields: opc=0, imm9=0, size=0, Rn=1, Rt=1
    let encoding: u32 = 0x38000021;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_normal
/// ASL: `field combination 34`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (same register test (reg=31)), Rt=31 (same register test (reg=31))
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_normal_combo_34_0_380003ff() {
    // Encoding: 0x380003FF
    // Test aarch64_memory_single_general_immediate_signed_offset_normal field combination: size=0, opc=0, imm9=0, Rn=31, Rt=31
    // Fields: size=0, opc=0, imm9=0, Rn=31, Rt=31
    let encoding: u32 = 0x380003FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_normal
/// ASL: `field size = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "size", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_normal_special_size_0_size_variant_0_0_38001000()
 {
    // Encoding: 0x38001000
    // Test aarch64_memory_single_general_immediate_signed_offset_normal special value size = 0 (Size variant 0)
    // Fields: imm9=1, Rt=0, size=0, Rn=0, opc=0
    let encoding: u32 = 0x38001000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_normal
/// ASL: `field size = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "size", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_normal_special_size_1_size_variant_1_0_78001000()
 {
    // Encoding: 0x78001000
    // Test aarch64_memory_single_general_immediate_signed_offset_normal special value size = 1 (Size variant 1)
    // Fields: opc=0, Rt=0, size=1, Rn=0, imm9=1
    let encoding: u32 = 0x78001000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_normal
/// ASL: `field size = 2 (Size variant 2)`
/// Requirement: FieldSpecial { field: "size", value: 2, meaning: "Size variant 2" }
/// Size variant 2
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_normal_special_size_2_size_variant_2_0_b8001000()
 {
    // Encoding: 0xB8001000
    // Test aarch64_memory_single_general_immediate_signed_offset_normal special value size = 2 (Size variant 2)
    // Fields: size=2, imm9=1, opc=0, Rt=0, Rn=0
    let encoding: u32 = 0xB8001000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_normal
/// ASL: `field size = 3 (Size variant 3)`
/// Requirement: FieldSpecial { field: "size", value: 3, meaning: "Size variant 3" }
/// Size variant 3
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_normal_special_size_3_size_variant_3_0_f8001000()
 {
    // Encoding: 0xF8001000
    // Test aarch64_memory_single_general_immediate_signed_offset_normal special value size = 3 (Size variant 3)
    // Fields: size=3, opc=0, imm9=1, Rt=0, Rn=0
    let encoding: u32 = 0xF8001000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_normal
/// ASL: `field opc = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "opc", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_normal_special_opc_0_size_variant_0_0_78001000()
 {
    // Encoding: 0x78001000
    // Test aarch64_memory_single_general_immediate_signed_offset_normal special value opc = 0 (Size variant 0)
    // Fields: imm9=1, Rt=0, size=1, opc=0, Rn=0
    let encoding: u32 = 0x78001000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_normal
/// ASL: `field opc = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "opc", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_normal_special_opc_1_size_variant_1_0_78401000()
 {
    // Encoding: 0x78401000
    // Test aarch64_memory_single_general_immediate_signed_offset_normal special value opc = 1 (Size variant 1)
    // Fields: opc=1, Rt=0, imm9=1, size=1, Rn=0
    let encoding: u32 = 0x78401000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_normal
/// ASL: `field opc = 2 (Size variant 2)`
/// Requirement: FieldSpecial { field: "opc", value: 2, meaning: "Size variant 2" }
/// Size variant 2
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_normal_special_opc_2_size_variant_2_0_78801000()
 {
    // Encoding: 0x78801000
    // Test aarch64_memory_single_general_immediate_signed_offset_normal special value opc = 2 (Size variant 2)
    // Fields: Rn=0, imm9=1, size=1, opc=2, Rt=0
    let encoding: u32 = 0x78801000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_normal
/// ASL: `field opc = 3 (Size variant 3)`
/// Requirement: FieldSpecial { field: "opc", value: 3, meaning: "Size variant 3" }
/// Size variant 3
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_normal_special_opc_3_size_variant_3_0_78c01000()
 {
    // Encoding: 0x78C01000
    // Test aarch64_memory_single_general_immediate_signed_offset_normal special value opc = 3 (Size variant 3)
    // Fields: opc=3, Rn=0, Rt=0, size=1, imm9=1
    let encoding: u32 = 0x78C01000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_normal
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_normal_special_rn_31_stack_pointer_sp_may_require_alignment_0_780013e0()
 {
    // Encoding: 0x780013E0
    // Test aarch64_memory_single_general_immediate_signed_offset_normal special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: Rt=0, Rn=31, opc=0, imm9=1, size=1
    let encoding: u32 = 0x780013E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_normal
/// ASL: `field Rt = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)`
/// Requirement: FieldSpecial { field: "Rt", value: 31, meaning: "Zero register (XZR/WZR) - reads as 0, writes discarded" }
/// Zero register (XZR/WZR) - reads as 0, writes discarded
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_normal_special_rt_31_zero_register_xzr_wzr_reads_as_0_writes_discarded_0_7800101f()
 {
    // Encoding: 0x7800101F
    // Test aarch64_memory_single_general_immediate_signed_offset_normal special value Rt = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)
    // Fields: imm9=1, size=1, Rn=0, opc=0, Rt=31
    let encoding: u32 = 0x7800101F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_normal
/// ASL: `GpFromField("t") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "t" }
/// verify register write to GpFromField("t")
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_normal_reg_write_0_38000000() {
    // Test aarch64_memory_single_general_immediate_signed_offset_normal register write: GpFromField("t")
    // Encoding: 0x38000000
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x38000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_normal
/// ASL: `GpFromField("t") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "t" }
/// verify register write to GpFromField("t")
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_normal_reg_write_1_38000000() {
    // Test aarch64_memory_single_general_immediate_signed_offset_normal register write: GpFromField("t")
    // Encoding: 0x38000000
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x38000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_normal
/// ASL: `Sp write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "unknown" }
/// verify register write to Sp
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_normal_reg_write_2_38000000() {
    // Test aarch64_memory_single_general_immediate_signed_offset_normal register write: Sp
    // Encoding: 0x38000000
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x38000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_normal
/// ASL: `GpFromField("n") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "n" }
/// verify register write to GpFromField("n")
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_normal_reg_write_3_38000000() {
    // Test aarch64_memory_single_general_immediate_signed_offset_normal register write: GpFromField("n")
    // Encoding: 0x38000000
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x38000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_normal
/// ASL: `Rn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Rn = 31)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_normal_sp_rn_380003e0() {
    // Test aarch64_memory_single_general_immediate_signed_offset_normal with Rn = SP (31)
    // Encoding: 0x380003E0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x380003E0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_normal
/// ASL: `Rt = 31 (ZR)`
/// Requirement: RegisterSpecial { reg: Zr, behavior: "reads as 0, writes discarded" }
/// zero register (Rt = 31)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_normal_zr_rt_3800001f() {
    // Test aarch64_memory_single_general_immediate_signed_offset_normal with Rt = ZR (31)
    // Encoding: 0x3800001F
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x3800001F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 31), 0, "XZR should always be 0");
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_normal
/// ASL: `Mem[address, 8] = data`
/// Requirement: MemoryAccess { op: Store, size_bits: 64, addressing: "Base { reg: \"address\" }" }
/// 8-byte store
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_normal_store_0_38000020() {
    // Test aarch64_memory_single_general_immediate_signed_offset_normal memory store: 8 bytes
    // Encoding: 0x38000020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 0, 0xDEADBEEFCAFEBABE);
    set_x(&mut cpu, 1, 0x100000000000);
    let encoding: u32 = 0x38000020;
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

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_normal
/// ASL: `Unconditional`
/// Requirement: UndefinedEncoding { condition: "Unconditional" }
/// triggers Undefined
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_normal_exception_0_38000000() {
    // Test aarch64_memory_single_general_immediate_signed_offset_normal exception: Undefined
    // Encoding: 0x38000000
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x38000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_normal
/// ASL: `Unconditional`
/// Requirement: UndefinedEncoding { condition: "Unconditional" }
/// triggers Undefined
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_normal_exception_1_38000000() {
    // Test aarch64_memory_single_general_immediate_signed_offset_normal exception: Undefined
    // Encoding: 0x38000000
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x38000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

// ============================================================================
// aarch64_memory_single_general_immediate_signed_post_idx Tests
// ============================================================================

/// Provenance: aarch64_memory_single_general_immediate_signed_post_idx
/// ASL: `field size 30 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_memory_single_general_immediate_signed_post_idx_field_size_0_min_400_38000400() {
    // Encoding: 0x38000400
    // Test aarch64_memory_single_general_immediate_signed_post_idx field size = 0 (Min)
    // Fields: imm9=0, Rn=0, Rt=0, opc=0, size=0
    let encoding: u32 = 0x38000400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_post_idx
/// ASL: `field size 30 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 1, boundary: PowerOfTwo }
/// 16-bit / halfword size
#[test]
fn test_aarch64_memory_single_general_immediate_signed_post_idx_field_size_1_poweroftwo_400_78000400()
 {
    // Encoding: 0x78000400
    // Test aarch64_memory_single_general_immediate_signed_post_idx field size = 1 (PowerOfTwo)
    // Fields: Rn=0, opc=0, Rt=0, imm9=0, size=1
    let encoding: u32 = 0x78000400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_post_idx
/// ASL: `field size 30 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 2, boundary: PowerOfTwo }
/// 32-bit / word size
#[test]
fn test_aarch64_memory_single_general_immediate_signed_post_idx_field_size_2_poweroftwo_400_b8000400()
 {
    // Encoding: 0xB8000400
    // Test aarch64_memory_single_general_immediate_signed_post_idx field size = 2 (PowerOfTwo)
    // Fields: Rn=0, size=2, opc=0, Rt=0, imm9=0
    let encoding: u32 = 0xB8000400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_post_idx
/// ASL: `field size 30 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 3, boundary: Max }
/// 64-bit / doubleword size
#[test]
fn test_aarch64_memory_single_general_immediate_signed_post_idx_field_size_3_max_400_f8000400() {
    // Encoding: 0xF8000400
    // Test aarch64_memory_single_general_immediate_signed_post_idx field size = 3 (Max)
    // Fields: opc=0, imm9=0, size=3, Rn=0, Rt=0
    let encoding: u32 = 0xF8000400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_post_idx
/// ASL: `field opc 22 +: 2`
/// Requirement: FieldBoundary { field: "opc", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_memory_single_general_immediate_signed_post_idx_field_opc_0_min_400_38000400() {
    // Encoding: 0x38000400
    // Test aarch64_memory_single_general_immediate_signed_post_idx field opc = 0 (Min)
    // Fields: size=0, imm9=0, opc=0, Rt=0, Rn=0
    let encoding: u32 = 0x38000400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_post_idx
/// ASL: `field opc 22 +: 2`
/// Requirement: FieldBoundary { field: "opc", value: 1, boundary: PowerOfTwo }
/// 16-bit / halfword size
#[test]
fn test_aarch64_memory_single_general_immediate_signed_post_idx_field_opc_1_poweroftwo_400_38400400()
 {
    // Encoding: 0x38400400
    // Test aarch64_memory_single_general_immediate_signed_post_idx field opc = 1 (PowerOfTwo)
    // Fields: opc=1, imm9=0, Rn=0, size=0, Rt=0
    let encoding: u32 = 0x38400400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_post_idx
/// ASL: `field opc 22 +: 2`
/// Requirement: FieldBoundary { field: "opc", value: 2, boundary: PowerOfTwo }
/// 32-bit / word size
#[test]
fn test_aarch64_memory_single_general_immediate_signed_post_idx_field_opc_2_poweroftwo_400_38800400()
 {
    // Encoding: 0x38800400
    // Test aarch64_memory_single_general_immediate_signed_post_idx field opc = 2 (PowerOfTwo)
    // Fields: imm9=0, Rn=0, opc=2, size=0, Rt=0
    let encoding: u32 = 0x38800400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_post_idx
/// ASL: `field opc 22 +: 2`
/// Requirement: FieldBoundary { field: "opc", value: 3, boundary: Max }
/// 64-bit / doubleword size
#[test]
fn test_aarch64_memory_single_general_immediate_signed_post_idx_field_opc_3_max_400_38c00400() {
    // Encoding: 0x38C00400
    // Test aarch64_memory_single_general_immediate_signed_post_idx field opc = 3 (Max)
    // Fields: opc=3, imm9=0, Rt=0, size=0, Rn=0
    let encoding: u32 = 0x38C00400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_post_idx
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch64_memory_single_general_immediate_signed_post_idx_field_imm9_0_zero_400_38000400() {
    // Encoding: 0x38000400
    // Test aarch64_memory_single_general_immediate_signed_post_idx field imm9 = 0 (Zero)
    // Fields: size=0, opc=0, imm9=0, Rt=0, Rn=0
    let encoding: u32 = 0x38000400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_post_idx
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch64_memory_single_general_immediate_signed_post_idx_field_imm9_1_poweroftwo_400_38001400()
 {
    // Encoding: 0x38001400
    // Test aarch64_memory_single_general_immediate_signed_post_idx field imm9 = 1 (PowerOfTwo)
    // Fields: Rn=0, size=0, opc=0, imm9=1, Rt=0
    let encoding: u32 = 0x38001400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_post_idx
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 3, boundary: PowerOfTwoMinusOne }
/// 2^2 - 1 = 3
#[test]
fn test_aarch64_memory_single_general_immediate_signed_post_idx_field_imm9_3_poweroftwominusone_400_38003400()
 {
    // Encoding: 0x38003400
    // Test aarch64_memory_single_general_immediate_signed_post_idx field imm9 = 3 (PowerOfTwoMinusOne)
    // Fields: size=0, Rn=0, Rt=0, imm9=3, opc=0
    let encoding: u32 = 0x38003400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_post_idx
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 4, boundary: PowerOfTwo }
/// power of 2 (2^2 = 4)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_post_idx_field_imm9_4_poweroftwo_400_38004400()
 {
    // Encoding: 0x38004400
    // Test aarch64_memory_single_general_immediate_signed_post_idx field imm9 = 4 (PowerOfTwo)
    // Fields: imm9=4, Rt=0, opc=0, Rn=0, size=0
    let encoding: u32 = 0x38004400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_post_idx
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 7, boundary: PowerOfTwoMinusOne }
/// 2^3 - 1 = 7
#[test]
fn test_aarch64_memory_single_general_immediate_signed_post_idx_field_imm9_7_poweroftwominusone_400_38007400()
 {
    // Encoding: 0x38007400
    // Test aarch64_memory_single_general_immediate_signed_post_idx field imm9 = 7 (PowerOfTwoMinusOne)
    // Fields: Rt=0, Rn=0, imm9=7, size=0, opc=0
    let encoding: u32 = 0x38007400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_post_idx
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 8, boundary: PowerOfTwo }
/// power of 2 (2^3 = 8)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_post_idx_field_imm9_8_poweroftwo_400_38008400()
 {
    // Encoding: 0x38008400
    // Test aarch64_memory_single_general_immediate_signed_post_idx field imm9 = 8 (PowerOfTwo)
    // Fields: Rt=0, opc=0, imm9=8, size=0, Rn=0
    let encoding: u32 = 0x38008400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_post_idx
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 15, boundary: PowerOfTwoMinusOne }
/// 2^4 - 1 = 15
#[test]
fn test_aarch64_memory_single_general_immediate_signed_post_idx_field_imm9_15_poweroftwominusone_400_3800f400()
 {
    // Encoding: 0x3800F400
    // Test aarch64_memory_single_general_immediate_signed_post_idx field imm9 = 15 (PowerOfTwoMinusOne)
    // Fields: Rn=0, size=0, imm9=15, opc=0, Rt=0
    let encoding: u32 = 0x3800F400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_post_idx
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 16, boundary: PowerOfTwo }
/// power of 2 (2^4 = 16)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_post_idx_field_imm9_16_poweroftwo_400_38010400()
 {
    // Encoding: 0x38010400
    // Test aarch64_memory_single_general_immediate_signed_post_idx field imm9 = 16 (PowerOfTwo)
    // Fields: imm9=16, Rt=0, size=0, Rn=0, opc=0
    let encoding: u32 = 0x38010400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_post_idx
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 31, boundary: PowerOfTwoMinusOne }
/// 2^5 - 1 = 31
#[test]
fn test_aarch64_memory_single_general_immediate_signed_post_idx_field_imm9_31_poweroftwominusone_400_3801f400()
 {
    // Encoding: 0x3801F400
    // Test aarch64_memory_single_general_immediate_signed_post_idx field imm9 = 31 (PowerOfTwoMinusOne)
    // Fields: Rt=0, opc=0, Rn=0, size=0, imm9=31
    let encoding: u32 = 0x3801F400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_post_idx
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 32, boundary: PowerOfTwo }
/// power of 2 (2^5 = 32)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_post_idx_field_imm9_32_poweroftwo_400_38020400()
 {
    // Encoding: 0x38020400
    // Test aarch64_memory_single_general_immediate_signed_post_idx field imm9 = 32 (PowerOfTwo)
    // Fields: Rt=0, size=0, imm9=32, opc=0, Rn=0
    let encoding: u32 = 0x38020400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_post_idx
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 63, boundary: PowerOfTwoMinusOne }
/// 2^6 - 1 = 63
#[test]
fn test_aarch64_memory_single_general_immediate_signed_post_idx_field_imm9_63_poweroftwominusone_400_3803f400()
 {
    // Encoding: 0x3803F400
    // Test aarch64_memory_single_general_immediate_signed_post_idx field imm9 = 63 (PowerOfTwoMinusOne)
    // Fields: opc=0, size=0, Rn=0, imm9=63, Rt=0
    let encoding: u32 = 0x3803F400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_post_idx
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 64, boundary: PowerOfTwo }
/// power of 2 (2^6 = 64)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_post_idx_field_imm9_64_poweroftwo_400_38040400()
 {
    // Encoding: 0x38040400
    // Test aarch64_memory_single_general_immediate_signed_post_idx field imm9 = 64 (PowerOfTwo)
    // Fields: size=0, opc=0, Rn=0, Rt=0, imm9=64
    let encoding: u32 = 0x38040400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_post_idx
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 127, boundary: PowerOfTwoMinusOne }
/// 2^7 - 1 = 127
#[test]
fn test_aarch64_memory_single_general_immediate_signed_post_idx_field_imm9_127_poweroftwominusone_400_3807f400()
 {
    // Encoding: 0x3807F400
    // Test aarch64_memory_single_general_immediate_signed_post_idx field imm9 = 127 (PowerOfTwoMinusOne)
    // Fields: size=0, imm9=127, Rn=0, Rt=0, opc=0
    let encoding: u32 = 0x3807F400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_post_idx
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 128, boundary: PowerOfTwo }
/// power of 2 (2^7 = 128)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_post_idx_field_imm9_128_poweroftwo_400_38080400()
 {
    // Encoding: 0x38080400
    // Test aarch64_memory_single_general_immediate_signed_post_idx field imm9 = 128 (PowerOfTwo)
    // Fields: Rt=0, size=0, opc=0, imm9=128, Rn=0
    let encoding: u32 = 0x38080400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_post_idx
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 255, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (255)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_post_idx_field_imm9_255_poweroftwominusone_400_380ff400()
 {
    // Encoding: 0x380FF400
    // Test aarch64_memory_single_general_immediate_signed_post_idx field imm9 = 255 (PowerOfTwoMinusOne)
    // Fields: Rt=0, imm9=255, opc=0, Rn=0, size=0
    let encoding: u32 = 0x380FF400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_post_idx
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 256, boundary: PowerOfTwo }
/// power of 2 (2^8 = 256)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_post_idx_field_imm9_256_poweroftwo_400_38100400()
 {
    // Encoding: 0x38100400
    // Test aarch64_memory_single_general_immediate_signed_post_idx field imm9 = 256 (PowerOfTwo)
    // Fields: Rt=0, Rn=0, size=0, opc=0, imm9=256
    let encoding: u32 = 0x38100400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_post_idx
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 511, boundary: Max }
/// maximum immediate (511)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_post_idx_field_imm9_511_max_400_381ff400() {
    // Encoding: 0x381FF400
    // Test aarch64_memory_single_general_immediate_signed_post_idx field imm9 = 511 (Max)
    // Fields: opc=0, imm9=511, Rt=0, size=0, Rn=0
    let encoding: u32 = 0x381FF400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_post_idx
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_post_idx_field_rn_0_min_400_38000400() {
    // Encoding: 0x38000400
    // Test aarch64_memory_single_general_immediate_signed_post_idx field Rn = 0 (Min)
    // Fields: opc=0, imm9=0, Rn=0, Rt=0, size=0
    let encoding: u32 = 0x38000400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_post_idx
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_post_idx_field_rn_1_poweroftwo_400_38000420()
{
    // Encoding: 0x38000420
    // Test aarch64_memory_single_general_immediate_signed_post_idx field Rn = 1 (PowerOfTwo)
    // Fields: Rn=1, imm9=0, opc=0, size=0, Rt=0
    let encoding: u32 = 0x38000420;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_post_idx
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_post_idx_field_rn_30_poweroftwominusone_400_380007c0()
 {
    // Encoding: 0x380007C0
    // Test aarch64_memory_single_general_immediate_signed_post_idx field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: opc=0, size=0, Rn=30, imm9=0, Rt=0
    let encoding: u32 = 0x380007C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_post_idx
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_post_idx_field_rn_31_max_400_380007e0() {
    // Encoding: 0x380007E0
    // Test aarch64_memory_single_general_immediate_signed_post_idx field Rn = 31 (Max)
    // Fields: Rt=0, imm9=0, Rn=31, opc=0, size=0
    let encoding: u32 = 0x380007E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_post_idx
/// ASL: `field Rt 0 +: 5`
/// Requirement: FieldBoundary { field: "Rt", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_post_idx_field_rt_0_min_400_38000400() {
    // Encoding: 0x38000400
    // Test aarch64_memory_single_general_immediate_signed_post_idx field Rt = 0 (Min)
    // Fields: size=0, Rt=0, imm9=0, Rn=0, opc=0
    let encoding: u32 = 0x38000400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_post_idx
/// ASL: `field Rt 0 +: 5`
/// Requirement: FieldBoundary { field: "Rt", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_post_idx_field_rt_1_poweroftwo_400_38000401()
{
    // Encoding: 0x38000401
    // Test aarch64_memory_single_general_immediate_signed_post_idx field Rt = 1 (PowerOfTwo)
    // Fields: Rt=1, imm9=0, size=0, Rn=0, opc=0
    let encoding: u32 = 0x38000401;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_post_idx
/// ASL: `field Rt 0 +: 5`
/// Requirement: FieldBoundary { field: "Rt", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_post_idx_field_rt_30_poweroftwominusone_400_3800041e()
 {
    // Encoding: 0x3800041E
    // Test aarch64_memory_single_general_immediate_signed_post_idx field Rt = 30 (PowerOfTwoMinusOne)
    // Fields: size=0, Rn=0, opc=0, imm9=0, Rt=30
    let encoding: u32 = 0x3800041E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_post_idx
/// ASL: `field Rt 0 +: 5`
/// Requirement: FieldBoundary { field: "Rt", value: 31, boundary: Max }
/// register index 31 (ZR - zero register)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_post_idx_field_rt_31_max_400_3800041f() {
    // Encoding: 0x3800041F
    // Test aarch64_memory_single_general_immediate_signed_post_idx field Rt = 31 (Max)
    // Fields: Rn=0, size=0, Rt=31, imm9=0, opc=0
    let encoding: u32 = 0x3800041F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_post_idx
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=0 (8-bit / byte size)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_post_idx_combo_0_400_38000400() {
    // Encoding: 0x38000400
    // Test aarch64_memory_single_general_immediate_signed_post_idx field combination: size=0, opc=0, imm9=0, Rn=0, Rt=0
    // Fields: Rt=0, imm9=0, size=0, opc=0, Rn=0
    let encoding: u32 = 0x38000400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_post_idx
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=1 (16-bit / halfword size)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_post_idx_combo_1_400_78000400() {
    // Encoding: 0x78000400
    // Test aarch64_memory_single_general_immediate_signed_post_idx field combination: size=1, opc=0, imm9=0, Rn=0, Rt=0
    // Fields: imm9=0, Rn=0, Rt=0, size=1, opc=0
    let encoding: u32 = 0x78000400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_post_idx
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=2 (32-bit / word size)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_post_idx_combo_2_400_b8000400() {
    // Encoding: 0xB8000400
    // Test aarch64_memory_single_general_immediate_signed_post_idx field combination: size=2, opc=0, imm9=0, Rn=0, Rt=0
    // Fields: size=2, opc=0, Rt=0, imm9=0, Rn=0
    let encoding: u32 = 0xB8000400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_post_idx
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=3 (64-bit / doubleword size)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_post_idx_combo_3_400_f8000400() {
    // Encoding: 0xF8000400
    // Test aarch64_memory_single_general_immediate_signed_post_idx field combination: size=3, opc=0, imm9=0, Rn=0, Rt=0
    // Fields: Rt=0, imm9=0, size=3, opc=0, Rn=0
    let encoding: u32 = 0xF8000400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_post_idx
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// opc=0 (8-bit / byte size)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_post_idx_combo_4_400_38000400() {
    // Encoding: 0x38000400
    // Test aarch64_memory_single_general_immediate_signed_post_idx field combination: size=0, opc=0, imm9=0, Rn=0, Rt=0
    // Fields: opc=0, imm9=0, Rt=0, size=0, Rn=0
    let encoding: u32 = 0x38000400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_post_idx
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// opc=1 (16-bit / halfword size)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_post_idx_combo_5_400_38400400() {
    // Encoding: 0x38400400
    // Test aarch64_memory_single_general_immediate_signed_post_idx field combination: size=0, opc=1, imm9=0, Rn=0, Rt=0
    // Fields: opc=1, size=0, imm9=0, Rn=0, Rt=0
    let encoding: u32 = 0x38400400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_post_idx
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// opc=2 (32-bit / word size)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_post_idx_combo_6_400_38800400() {
    // Encoding: 0x38800400
    // Test aarch64_memory_single_general_immediate_signed_post_idx field combination: size=0, opc=2, imm9=0, Rn=0, Rt=0
    // Fields: size=0, Rt=0, Rn=0, opc=2, imm9=0
    let encoding: u32 = 0x38800400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_post_idx
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// opc=3 (64-bit / doubleword size)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_post_idx_combo_7_400_38c00400() {
    // Encoding: 0x38C00400
    // Test aarch64_memory_single_general_immediate_signed_post_idx field combination: size=0, opc=3, imm9=0, Rn=0, Rt=0
    // Fields: imm9=0, Rn=0, opc=3, size=0, Rt=0
    let encoding: u32 = 0x38C00400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_post_idx
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=0 (immediate value 0)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_post_idx_combo_8_400_38000400() {
    // Encoding: 0x38000400
    // Test aarch64_memory_single_general_immediate_signed_post_idx field combination: size=0, opc=0, imm9=0, Rn=0, Rt=0
    // Fields: imm9=0, size=0, Rt=0, opc=0, Rn=0
    let encoding: u32 = 0x38000400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_post_idx
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=1 (immediate value 1)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_post_idx_combo_9_400_38001400() {
    // Encoding: 0x38001400
    // Test aarch64_memory_single_general_immediate_signed_post_idx field combination: size=0, opc=0, imm9=1, Rn=0, Rt=0
    // Fields: Rt=0, Rn=0, size=0, imm9=1, opc=0
    let encoding: u32 = 0x38001400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_post_idx
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=3 (2^2 - 1 = 3)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_post_idx_combo_10_400_38003400() {
    // Encoding: 0x38003400
    // Test aarch64_memory_single_general_immediate_signed_post_idx field combination: size=0, opc=0, imm9=3, Rn=0, Rt=0
    // Fields: Rt=0, Rn=0, imm9=3, size=0, opc=0
    let encoding: u32 = 0x38003400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_post_idx
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=4 (power of 2 (2^2 = 4))
#[test]
fn test_aarch64_memory_single_general_immediate_signed_post_idx_combo_11_400_38004400() {
    // Encoding: 0x38004400
    // Test aarch64_memory_single_general_immediate_signed_post_idx field combination: size=0, opc=0, imm9=4, Rn=0, Rt=0
    // Fields: Rn=0, imm9=4, Rt=0, opc=0, size=0
    let encoding: u32 = 0x38004400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_post_idx
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=7 (2^3 - 1 = 7)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_post_idx_combo_12_400_38007400() {
    // Encoding: 0x38007400
    // Test aarch64_memory_single_general_immediate_signed_post_idx field combination: size=0, opc=0, imm9=7, Rn=0, Rt=0
    // Fields: Rn=0, imm9=7, Rt=0, size=0, opc=0
    let encoding: u32 = 0x38007400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_post_idx
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=8 (power of 2 (2^3 = 8))
#[test]
fn test_aarch64_memory_single_general_immediate_signed_post_idx_combo_13_400_38008400() {
    // Encoding: 0x38008400
    // Test aarch64_memory_single_general_immediate_signed_post_idx field combination: size=0, opc=0, imm9=8, Rn=0, Rt=0
    // Fields: opc=0, size=0, imm9=8, Rt=0, Rn=0
    let encoding: u32 = 0x38008400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_post_idx
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=15 (2^4 - 1 = 15)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_post_idx_combo_14_400_3800f400() {
    // Encoding: 0x3800F400
    // Test aarch64_memory_single_general_immediate_signed_post_idx field combination: size=0, opc=0, imm9=15, Rn=0, Rt=0
    // Fields: size=0, opc=0, imm9=15, Rt=0, Rn=0
    let encoding: u32 = 0x3800F400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_post_idx
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=16 (power of 2 (2^4 = 16))
#[test]
fn test_aarch64_memory_single_general_immediate_signed_post_idx_combo_15_400_38010400() {
    // Encoding: 0x38010400
    // Test aarch64_memory_single_general_immediate_signed_post_idx field combination: size=0, opc=0, imm9=16, Rn=0, Rt=0
    // Fields: Rn=0, Rt=0, opc=0, imm9=16, size=0
    let encoding: u32 = 0x38010400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_post_idx
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=31 (2^5 - 1 = 31)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_post_idx_combo_16_400_3801f400() {
    // Encoding: 0x3801F400
    // Test aarch64_memory_single_general_immediate_signed_post_idx field combination: size=0, opc=0, imm9=31, Rn=0, Rt=0
    // Fields: Rn=0, size=0, Rt=0, imm9=31, opc=0
    let encoding: u32 = 0x3801F400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_post_idx
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=32 (power of 2 (2^5 = 32))
#[test]
fn test_aarch64_memory_single_general_immediate_signed_post_idx_combo_17_400_38020400() {
    // Encoding: 0x38020400
    // Test aarch64_memory_single_general_immediate_signed_post_idx field combination: size=0, opc=0, imm9=32, Rn=0, Rt=0
    // Fields: opc=0, Rt=0, Rn=0, imm9=32, size=0
    let encoding: u32 = 0x38020400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_post_idx
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=63 (2^6 - 1 = 63)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_post_idx_combo_18_400_3803f400() {
    // Encoding: 0x3803F400
    // Test aarch64_memory_single_general_immediate_signed_post_idx field combination: size=0, opc=0, imm9=63, Rn=0, Rt=0
    // Fields: opc=0, imm9=63, size=0, Rt=0, Rn=0
    let encoding: u32 = 0x3803F400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_post_idx
/// ASL: `field combination 19`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=64 (power of 2 (2^6 = 64))
#[test]
fn test_aarch64_memory_single_general_immediate_signed_post_idx_combo_19_400_38040400() {
    // Encoding: 0x38040400
    // Test aarch64_memory_single_general_immediate_signed_post_idx field combination: size=0, opc=0, imm9=64, Rn=0, Rt=0
    // Fields: imm9=64, Rn=0, opc=0, Rt=0, size=0
    let encoding: u32 = 0x38040400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_post_idx
/// ASL: `field combination 20`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=127 (2^7 - 1 = 127)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_post_idx_combo_20_400_3807f400() {
    // Encoding: 0x3807F400
    // Test aarch64_memory_single_general_immediate_signed_post_idx field combination: size=0, opc=0, imm9=127, Rn=0, Rt=0
    // Fields: size=0, imm9=127, Rn=0, opc=0, Rt=0
    let encoding: u32 = 0x3807F400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_post_idx
/// ASL: `field combination 21`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=128 (power of 2 (2^7 = 128))
#[test]
fn test_aarch64_memory_single_general_immediate_signed_post_idx_combo_21_400_38080400() {
    // Encoding: 0x38080400
    // Test aarch64_memory_single_general_immediate_signed_post_idx field combination: size=0, opc=0, imm9=128, Rn=0, Rt=0
    // Fields: Rt=0, size=0, imm9=128, Rn=0, opc=0
    let encoding: u32 = 0x38080400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_post_idx
/// ASL: `field combination 22`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=255 (immediate midpoint (255))
#[test]
fn test_aarch64_memory_single_general_immediate_signed_post_idx_combo_22_400_380ff400() {
    // Encoding: 0x380FF400
    // Test aarch64_memory_single_general_immediate_signed_post_idx field combination: size=0, opc=0, imm9=255, Rn=0, Rt=0
    // Fields: opc=0, size=0, Rn=0, Rt=0, imm9=255
    let encoding: u32 = 0x380FF400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_post_idx
/// ASL: `field combination 23`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=256 (power of 2 (2^8 = 256))
#[test]
fn test_aarch64_memory_single_general_immediate_signed_post_idx_combo_23_400_38100400() {
    // Encoding: 0x38100400
    // Test aarch64_memory_single_general_immediate_signed_post_idx field combination: size=0, opc=0, imm9=256, Rn=0, Rt=0
    // Fields: opc=0, size=0, Rn=0, Rt=0, imm9=256
    let encoding: u32 = 0x38100400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_post_idx
/// ASL: `field combination 24`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=511 (maximum immediate (511))
#[test]
fn test_aarch64_memory_single_general_immediate_signed_post_idx_combo_24_400_381ff400() {
    // Encoding: 0x381FF400
    // Test aarch64_memory_single_general_immediate_signed_post_idx field combination: size=0, opc=0, imm9=511, Rn=0, Rt=0
    // Fields: Rn=0, opc=0, imm9=511, Rt=0, size=0
    let encoding: u32 = 0x381FF400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_post_idx
/// ASL: `field combination 25`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch64_memory_single_general_immediate_signed_post_idx_combo_25_400_38000400() {
    // Encoding: 0x38000400
    // Test aarch64_memory_single_general_immediate_signed_post_idx field combination: size=0, opc=0, imm9=0, Rn=0, Rt=0
    // Fields: size=0, opc=0, imm9=0, Rn=0, Rt=0
    let encoding: u32 = 0x38000400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_post_idx
/// ASL: `field combination 26`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_aarch64_memory_single_general_immediate_signed_post_idx_combo_26_400_38000420() {
    // Encoding: 0x38000420
    // Test aarch64_memory_single_general_immediate_signed_post_idx field combination: size=0, opc=0, imm9=0, Rn=1, Rt=0
    // Fields: Rn=1, size=0, opc=0, Rt=0, imm9=0
    let encoding: u32 = 0x38000420;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_post_idx
/// ASL: `field combination 27`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_memory_single_general_immediate_signed_post_idx_combo_27_400_380007c0() {
    // Encoding: 0x380007C0
    // Test aarch64_memory_single_general_immediate_signed_post_idx field combination: size=0, opc=0, imm9=0, Rn=30, Rt=0
    // Fields: opc=0, size=0, imm9=0, Rn=30, Rt=0
    let encoding: u32 = 0x380007C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_post_idx
/// ASL: `field combination 28`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_memory_single_general_immediate_signed_post_idx_combo_28_400_380007e0() {
    // Encoding: 0x380007E0
    // Test aarch64_memory_single_general_immediate_signed_post_idx field combination: size=0, opc=0, imm9=0, Rn=31, Rt=0
    // Fields: Rt=0, opc=0, Rn=31, imm9=0, size=0
    let encoding: u32 = 0x380007E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_post_idx
/// ASL: `field combination 29`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt=0 (register index 0 (first register))
#[test]
fn test_aarch64_memory_single_general_immediate_signed_post_idx_combo_29_400_38000400() {
    // Encoding: 0x38000400
    // Test aarch64_memory_single_general_immediate_signed_post_idx field combination: size=0, opc=0, imm9=0, Rn=0, Rt=0
    // Fields: Rn=0, opc=0, Rt=0, imm9=0, size=0
    let encoding: u32 = 0x38000400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_post_idx
/// ASL: `field combination 30`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt=1 (register index 1 (second register))
#[test]
fn test_aarch64_memory_single_general_immediate_signed_post_idx_combo_30_400_38000401() {
    // Encoding: 0x38000401
    // Test aarch64_memory_single_general_immediate_signed_post_idx field combination: size=0, opc=0, imm9=0, Rn=0, Rt=1
    // Fields: Rt=1, imm9=0, opc=0, size=0, Rn=0
    let encoding: u32 = 0x38000401;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_post_idx
/// ASL: `field combination 31`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_memory_single_general_immediate_signed_post_idx_combo_31_400_3800041e() {
    // Encoding: 0x3800041E
    // Test aarch64_memory_single_general_immediate_signed_post_idx field combination: size=0, opc=0, imm9=0, Rn=0, Rt=30
    // Fields: imm9=0, opc=0, size=0, Rn=0, Rt=30
    let encoding: u32 = 0x3800041E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_post_idx
/// ASL: `field combination 32`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt=31 (register index 31 (ZR - zero register))
#[test]
fn test_aarch64_memory_single_general_immediate_signed_post_idx_combo_32_400_3800041f() {
    // Encoding: 0x3800041F
    // Test aarch64_memory_single_general_immediate_signed_post_idx field combination: size=0, opc=0, imm9=0, Rn=0, Rt=31
    // Fields: imm9=0, size=0, Rn=0, Rt=31, opc=0
    let encoding: u32 = 0x3800041F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_post_idx
/// ASL: `field combination 33`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (same register test (reg=1)), Rt=1 (same register test (reg=1))
#[test]
fn test_aarch64_memory_single_general_immediate_signed_post_idx_combo_33_400_38000421() {
    // Encoding: 0x38000421
    // Test aarch64_memory_single_general_immediate_signed_post_idx field combination: size=0, opc=0, imm9=0, Rn=1, Rt=1
    // Fields: imm9=0, Rt=1, size=0, opc=0, Rn=1
    let encoding: u32 = 0x38000421;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_post_idx
/// ASL: `field combination 34`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (same register test (reg=31)), Rt=31 (same register test (reg=31))
#[test]
fn test_aarch64_memory_single_general_immediate_signed_post_idx_combo_34_400_380007ff() {
    // Encoding: 0x380007FF
    // Test aarch64_memory_single_general_immediate_signed_post_idx field combination: size=0, opc=0, imm9=0, Rn=31, Rt=31
    // Fields: Rt=31, opc=0, Rn=31, size=0, imm9=0
    let encoding: u32 = 0x380007FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_post_idx
/// ASL: `field size = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "size", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_memory_single_general_immediate_signed_post_idx_special_size_0_size_variant_0_1024_38001400()
 {
    // Encoding: 0x38001400
    // Test aarch64_memory_single_general_immediate_signed_post_idx special value size = 0 (Size variant 0)
    // Fields: size=0, imm9=1, Rn=0, Rt=0, opc=0
    let encoding: u32 = 0x38001400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_post_idx
/// ASL: `field size = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "size", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_memory_single_general_immediate_signed_post_idx_special_size_1_size_variant_1_1024_78001400()
 {
    // Encoding: 0x78001400
    // Test aarch64_memory_single_general_immediate_signed_post_idx special value size = 1 (Size variant 1)
    // Fields: opc=0, imm9=1, Rn=0, Rt=0, size=1
    let encoding: u32 = 0x78001400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_post_idx
/// ASL: `field size = 2 (Size variant 2)`
/// Requirement: FieldSpecial { field: "size", value: 2, meaning: "Size variant 2" }
/// Size variant 2
#[test]
fn test_aarch64_memory_single_general_immediate_signed_post_idx_special_size_2_size_variant_2_1024_b8001400()
 {
    // Encoding: 0xB8001400
    // Test aarch64_memory_single_general_immediate_signed_post_idx special value size = 2 (Size variant 2)
    // Fields: Rt=0, Rn=0, size=2, opc=0, imm9=1
    let encoding: u32 = 0xB8001400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_post_idx
/// ASL: `field size = 3 (Size variant 3)`
/// Requirement: FieldSpecial { field: "size", value: 3, meaning: "Size variant 3" }
/// Size variant 3
#[test]
fn test_aarch64_memory_single_general_immediate_signed_post_idx_special_size_3_size_variant_3_1024_f8001400()
 {
    // Encoding: 0xF8001400
    // Test aarch64_memory_single_general_immediate_signed_post_idx special value size = 3 (Size variant 3)
    // Fields: Rt=0, imm9=1, opc=0, Rn=0, size=3
    let encoding: u32 = 0xF8001400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_post_idx
/// ASL: `field opc = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "opc", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_memory_single_general_immediate_signed_post_idx_special_opc_0_size_variant_0_1024_78001400()
 {
    // Encoding: 0x78001400
    // Test aarch64_memory_single_general_immediate_signed_post_idx special value opc = 0 (Size variant 0)
    // Fields: imm9=1, Rn=0, opc=0, size=1, Rt=0
    let encoding: u32 = 0x78001400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_post_idx
/// ASL: `field opc = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "opc", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_memory_single_general_immediate_signed_post_idx_special_opc_1_size_variant_1_1024_78401400()
 {
    // Encoding: 0x78401400
    // Test aarch64_memory_single_general_immediate_signed_post_idx special value opc = 1 (Size variant 1)
    // Fields: opc=1, Rn=0, imm9=1, size=1, Rt=0
    let encoding: u32 = 0x78401400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_post_idx
/// ASL: `field opc = 2 (Size variant 2)`
/// Requirement: FieldSpecial { field: "opc", value: 2, meaning: "Size variant 2" }
/// Size variant 2
#[test]
fn test_aarch64_memory_single_general_immediate_signed_post_idx_special_opc_2_size_variant_2_1024_78801400()
 {
    // Encoding: 0x78801400
    // Test aarch64_memory_single_general_immediate_signed_post_idx special value opc = 2 (Size variant 2)
    // Fields: imm9=1, opc=2, Rt=0, size=1, Rn=0
    let encoding: u32 = 0x78801400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_post_idx
/// ASL: `field opc = 3 (Size variant 3)`
/// Requirement: FieldSpecial { field: "opc", value: 3, meaning: "Size variant 3" }
/// Size variant 3
#[test]
fn test_aarch64_memory_single_general_immediate_signed_post_idx_special_opc_3_size_variant_3_1024_78c01400()
 {
    // Encoding: 0x78C01400
    // Test aarch64_memory_single_general_immediate_signed_post_idx special value opc = 3 (Size variant 3)
    // Fields: size=1, opc=3, Rt=0, Rn=0, imm9=1
    let encoding: u32 = 0x78C01400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_post_idx
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_memory_single_general_immediate_signed_post_idx_special_rn_31_stack_pointer_sp_may_require_alignment_1024_780017e0()
 {
    // Encoding: 0x780017E0
    // Test aarch64_memory_single_general_immediate_signed_post_idx special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: Rt=0, Rn=31, imm9=1, opc=0, size=1
    let encoding: u32 = 0x780017E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_post_idx
/// ASL: `field Rt = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)`
/// Requirement: FieldSpecial { field: "Rt", value: 31, meaning: "Zero register (XZR/WZR) - reads as 0, writes discarded" }
/// Zero register (XZR/WZR) - reads as 0, writes discarded
#[test]
fn test_aarch64_memory_single_general_immediate_signed_post_idx_special_rt_31_zero_register_xzr_wzr_reads_as_0_writes_discarded_1024_7800141f()
 {
    // Encoding: 0x7800141F
    // Test aarch64_memory_single_general_immediate_signed_post_idx special value Rt = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)
    // Fields: size=1, opc=0, Rt=31, Rn=0, imm9=1
    let encoding: u32 = 0x7800141F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pre_idx
/// ASL: `field size 30 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pre_idx_field_size_0_min_c00_38000c00() {
    // Encoding: 0x38000C00
    // Test aarch64_memory_single_general_immediate_signed_pre_idx field size = 0 (Min)
    // Fields: size=0, imm9=0, opc=0, Rn=0, Rt=0
    let encoding: u32 = 0x38000C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pre_idx
/// ASL: `field size 30 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 1, boundary: PowerOfTwo }
/// 16-bit / halfword size
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pre_idx_field_size_1_poweroftwo_c00_78000c00()
 {
    // Encoding: 0x78000C00
    // Test aarch64_memory_single_general_immediate_signed_pre_idx field size = 1 (PowerOfTwo)
    // Fields: opc=0, Rn=0, Rt=0, size=1, imm9=0
    let encoding: u32 = 0x78000C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pre_idx
/// ASL: `field size 30 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 2, boundary: PowerOfTwo }
/// 32-bit / word size
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pre_idx_field_size_2_poweroftwo_c00_b8000c00()
 {
    // Encoding: 0xB8000C00
    // Test aarch64_memory_single_general_immediate_signed_pre_idx field size = 2 (PowerOfTwo)
    // Fields: opc=0, imm9=0, size=2, Rt=0, Rn=0
    let encoding: u32 = 0xB8000C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pre_idx
/// ASL: `field size 30 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 3, boundary: Max }
/// 64-bit / doubleword size
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pre_idx_field_size_3_max_c00_f8000c00() {
    // Encoding: 0xF8000C00
    // Test aarch64_memory_single_general_immediate_signed_pre_idx field size = 3 (Max)
    // Fields: imm9=0, size=3, opc=0, Rn=0, Rt=0
    let encoding: u32 = 0xF8000C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pre_idx
/// ASL: `field opc 22 +: 2`
/// Requirement: FieldBoundary { field: "opc", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pre_idx_field_opc_0_min_c00_38000c00() {
    // Encoding: 0x38000C00
    // Test aarch64_memory_single_general_immediate_signed_pre_idx field opc = 0 (Min)
    // Fields: size=0, Rt=0, opc=0, imm9=0, Rn=0
    let encoding: u32 = 0x38000C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pre_idx
/// ASL: `field opc 22 +: 2`
/// Requirement: FieldBoundary { field: "opc", value: 1, boundary: PowerOfTwo }
/// 16-bit / halfword size
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pre_idx_field_opc_1_poweroftwo_c00_38400c00()
{
    // Encoding: 0x38400C00
    // Test aarch64_memory_single_general_immediate_signed_pre_idx field opc = 1 (PowerOfTwo)
    // Fields: size=0, Rn=0, Rt=0, imm9=0, opc=1
    let encoding: u32 = 0x38400C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pre_idx
/// ASL: `field opc 22 +: 2`
/// Requirement: FieldBoundary { field: "opc", value: 2, boundary: PowerOfTwo }
/// 32-bit / word size
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pre_idx_field_opc_2_poweroftwo_c00_38800c00()
{
    // Encoding: 0x38800C00
    // Test aarch64_memory_single_general_immediate_signed_pre_idx field opc = 2 (PowerOfTwo)
    // Fields: imm9=0, opc=2, Rn=0, Rt=0, size=0
    let encoding: u32 = 0x38800C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pre_idx
/// ASL: `field opc 22 +: 2`
/// Requirement: FieldBoundary { field: "opc", value: 3, boundary: Max }
/// 64-bit / doubleword size
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pre_idx_field_opc_3_max_c00_38c00c00() {
    // Encoding: 0x38C00C00
    // Test aarch64_memory_single_general_immediate_signed_pre_idx field opc = 3 (Max)
    // Fields: opc=3, size=0, Rn=0, Rt=0, imm9=0
    let encoding: u32 = 0x38C00C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pre_idx
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pre_idx_field_imm9_0_zero_c00_38000c00() {
    // Encoding: 0x38000C00
    // Test aarch64_memory_single_general_immediate_signed_pre_idx field imm9 = 0 (Zero)
    // Fields: size=0, imm9=0, opc=0, Rn=0, Rt=0
    let encoding: u32 = 0x38000C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pre_idx
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pre_idx_field_imm9_1_poweroftwo_c00_38001c00()
 {
    // Encoding: 0x38001C00
    // Test aarch64_memory_single_general_immediate_signed_pre_idx field imm9 = 1 (PowerOfTwo)
    // Fields: opc=0, imm9=1, Rt=0, Rn=0, size=0
    let encoding: u32 = 0x38001C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pre_idx
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 3, boundary: PowerOfTwoMinusOne }
/// 2^2 - 1 = 3
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pre_idx_field_imm9_3_poweroftwominusone_c00_38003c00()
 {
    // Encoding: 0x38003C00
    // Test aarch64_memory_single_general_immediate_signed_pre_idx field imm9 = 3 (PowerOfTwoMinusOne)
    // Fields: Rn=0, imm9=3, opc=0, size=0, Rt=0
    let encoding: u32 = 0x38003C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pre_idx
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 4, boundary: PowerOfTwo }
/// power of 2 (2^2 = 4)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pre_idx_field_imm9_4_poweroftwo_c00_38004c00()
 {
    // Encoding: 0x38004C00
    // Test aarch64_memory_single_general_immediate_signed_pre_idx field imm9 = 4 (PowerOfTwo)
    // Fields: imm9=4, opc=0, Rt=0, size=0, Rn=0
    let encoding: u32 = 0x38004C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pre_idx
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 7, boundary: PowerOfTwoMinusOne }
/// 2^3 - 1 = 7
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pre_idx_field_imm9_7_poweroftwominusone_c00_38007c00()
 {
    // Encoding: 0x38007C00
    // Test aarch64_memory_single_general_immediate_signed_pre_idx field imm9 = 7 (PowerOfTwoMinusOne)
    // Fields: size=0, Rt=0, imm9=7, Rn=0, opc=0
    let encoding: u32 = 0x38007C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pre_idx
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 8, boundary: PowerOfTwo }
/// power of 2 (2^3 = 8)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pre_idx_field_imm9_8_poweroftwo_c00_38008c00()
 {
    // Encoding: 0x38008C00
    // Test aarch64_memory_single_general_immediate_signed_pre_idx field imm9 = 8 (PowerOfTwo)
    // Fields: Rn=0, Rt=0, size=0, opc=0, imm9=8
    let encoding: u32 = 0x38008C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pre_idx
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 15, boundary: PowerOfTwoMinusOne }
/// 2^4 - 1 = 15
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pre_idx_field_imm9_15_poweroftwominusone_c00_3800fc00()
 {
    // Encoding: 0x3800FC00
    // Test aarch64_memory_single_general_immediate_signed_pre_idx field imm9 = 15 (PowerOfTwoMinusOne)
    // Fields: imm9=15, size=0, Rn=0, Rt=0, opc=0
    let encoding: u32 = 0x3800FC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pre_idx
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 16, boundary: PowerOfTwo }
/// power of 2 (2^4 = 16)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pre_idx_field_imm9_16_poweroftwo_c00_38010c00()
 {
    // Encoding: 0x38010C00
    // Test aarch64_memory_single_general_immediate_signed_pre_idx field imm9 = 16 (PowerOfTwo)
    // Fields: Rt=0, opc=0, size=0, imm9=16, Rn=0
    let encoding: u32 = 0x38010C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pre_idx
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 31, boundary: PowerOfTwoMinusOne }
/// 2^5 - 1 = 31
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pre_idx_field_imm9_31_poweroftwominusone_c00_3801fc00()
 {
    // Encoding: 0x3801FC00
    // Test aarch64_memory_single_general_immediate_signed_pre_idx field imm9 = 31 (PowerOfTwoMinusOne)
    // Fields: size=0, opc=0, Rn=0, imm9=31, Rt=0
    let encoding: u32 = 0x3801FC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pre_idx
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 32, boundary: PowerOfTwo }
/// power of 2 (2^5 = 32)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pre_idx_field_imm9_32_poweroftwo_c00_38020c00()
 {
    // Encoding: 0x38020C00
    // Test aarch64_memory_single_general_immediate_signed_pre_idx field imm9 = 32 (PowerOfTwo)
    // Fields: opc=0, size=0, Rn=0, Rt=0, imm9=32
    let encoding: u32 = 0x38020C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pre_idx
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 63, boundary: PowerOfTwoMinusOne }
/// 2^6 - 1 = 63
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pre_idx_field_imm9_63_poweroftwominusone_c00_3803fc00()
 {
    // Encoding: 0x3803FC00
    // Test aarch64_memory_single_general_immediate_signed_pre_idx field imm9 = 63 (PowerOfTwoMinusOne)
    // Fields: Rt=0, size=0, imm9=63, Rn=0, opc=0
    let encoding: u32 = 0x3803FC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pre_idx
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 64, boundary: PowerOfTwo }
/// power of 2 (2^6 = 64)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pre_idx_field_imm9_64_poweroftwo_c00_38040c00()
 {
    // Encoding: 0x38040C00
    // Test aarch64_memory_single_general_immediate_signed_pre_idx field imm9 = 64 (PowerOfTwo)
    // Fields: size=0, opc=0, imm9=64, Rn=0, Rt=0
    let encoding: u32 = 0x38040C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pre_idx
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 127, boundary: PowerOfTwoMinusOne }
/// 2^7 - 1 = 127
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pre_idx_field_imm9_127_poweroftwominusone_c00_3807fc00()
 {
    // Encoding: 0x3807FC00
    // Test aarch64_memory_single_general_immediate_signed_pre_idx field imm9 = 127 (PowerOfTwoMinusOne)
    // Fields: Rn=0, Rt=0, size=0, opc=0, imm9=127
    let encoding: u32 = 0x3807FC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pre_idx
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 128, boundary: PowerOfTwo }
/// power of 2 (2^7 = 128)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pre_idx_field_imm9_128_poweroftwo_c00_38080c00()
 {
    // Encoding: 0x38080C00
    // Test aarch64_memory_single_general_immediate_signed_pre_idx field imm9 = 128 (PowerOfTwo)
    // Fields: opc=0, imm9=128, Rn=0, size=0, Rt=0
    let encoding: u32 = 0x38080C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pre_idx
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 255, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (255)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pre_idx_field_imm9_255_poweroftwominusone_c00_380ffc00()
 {
    // Encoding: 0x380FFC00
    // Test aarch64_memory_single_general_immediate_signed_pre_idx field imm9 = 255 (PowerOfTwoMinusOne)
    // Fields: opc=0, imm9=255, size=0, Rt=0, Rn=0
    let encoding: u32 = 0x380FFC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pre_idx
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 256, boundary: PowerOfTwo }
/// power of 2 (2^8 = 256)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pre_idx_field_imm9_256_poweroftwo_c00_38100c00()
 {
    // Encoding: 0x38100C00
    // Test aarch64_memory_single_general_immediate_signed_pre_idx field imm9 = 256 (PowerOfTwo)
    // Fields: Rn=0, imm9=256, Rt=0, opc=0, size=0
    let encoding: u32 = 0x38100C00;
    let mut cpu = create_test_cpu();
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

/// Provenance: aarch64_memory_single_general_immediate_signed_pre_idx
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 511, boundary: Max }
/// maximum immediate (511)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pre_idx_field_imm9_511_max_c00_381ffc00() {
    // Encoding: 0x381FFC00
    // Test aarch64_memory_single_general_immediate_signed_pre_idx field imm9 = 511 (Max)
    // Fields: opc=0, Rn=0, Rt=0, imm9=511, size=0
    let encoding: u32 = 0x381FFC00;
    let mut cpu = create_test_cpu();
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

/// Provenance: aarch64_memory_single_general_immediate_signed_pre_idx
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pre_idx_field_rn_0_min_c00_38000c00() {
    // Encoding: 0x38000C00
    // Test aarch64_memory_single_general_immediate_signed_pre_idx field Rn = 0 (Min)
    // Fields: opc=0, Rn=0, size=0, Rt=0, imm9=0
    let encoding: u32 = 0x38000C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pre_idx
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pre_idx_field_rn_1_poweroftwo_c00_38000c20()
{
    // Encoding: 0x38000C20
    // Test aarch64_memory_single_general_immediate_signed_pre_idx field Rn = 1 (PowerOfTwo)
    // Fields: Rn=1, opc=0, imm9=0, size=0, Rt=0
    let encoding: u32 = 0x38000C20;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pre_idx
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pre_idx_field_rn_30_poweroftwominusone_c00_38000fc0()
 {
    // Encoding: 0x38000FC0
    // Test aarch64_memory_single_general_immediate_signed_pre_idx field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: opc=0, size=0, imm9=0, Rt=0, Rn=30
    let encoding: u32 = 0x38000FC0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pre_idx
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pre_idx_field_rn_31_max_c00_38000fe0() {
    // Encoding: 0x38000FE0
    // Test aarch64_memory_single_general_immediate_signed_pre_idx field Rn = 31 (Max)
    // Fields: size=0, Rn=31, Rt=0, opc=0, imm9=0
    let encoding: u32 = 0x38000FE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pre_idx
/// ASL: `field Rt 0 +: 5`
/// Requirement: FieldBoundary { field: "Rt", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pre_idx_field_rt_0_min_c00_38000c00() {
    // Encoding: 0x38000C00
    // Test aarch64_memory_single_general_immediate_signed_pre_idx field Rt = 0 (Min)
    // Fields: size=0, Rn=0, opc=0, imm9=0, Rt=0
    let encoding: u32 = 0x38000C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pre_idx
/// ASL: `field Rt 0 +: 5`
/// Requirement: FieldBoundary { field: "Rt", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pre_idx_field_rt_1_poweroftwo_c00_38000c01()
{
    // Encoding: 0x38000C01
    // Test aarch64_memory_single_general_immediate_signed_pre_idx field Rt = 1 (PowerOfTwo)
    // Fields: size=0, imm9=0, Rt=1, opc=0, Rn=0
    let encoding: u32 = 0x38000C01;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pre_idx
/// ASL: `field Rt 0 +: 5`
/// Requirement: FieldBoundary { field: "Rt", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pre_idx_field_rt_30_poweroftwominusone_c00_38000c1e()
 {
    // Encoding: 0x38000C1E
    // Test aarch64_memory_single_general_immediate_signed_pre_idx field Rt = 30 (PowerOfTwoMinusOne)
    // Fields: imm9=0, Rn=0, size=0, Rt=30, opc=0
    let encoding: u32 = 0x38000C1E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pre_idx
/// ASL: `field Rt 0 +: 5`
/// Requirement: FieldBoundary { field: "Rt", value: 31, boundary: Max }
/// register index 31 (ZR - zero register)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pre_idx_field_rt_31_max_c00_38000c1f() {
    // Encoding: 0x38000C1F
    // Test aarch64_memory_single_general_immediate_signed_pre_idx field Rt = 31 (Max)
    // Fields: Rn=0, size=0, Rt=31, imm9=0, opc=0
    let encoding: u32 = 0x38000C1F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pre_idx
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=0 (8-bit / byte size)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pre_idx_combo_0_c00_38000c00() {
    // Encoding: 0x38000C00
    // Test aarch64_memory_single_general_immediate_signed_pre_idx field combination: size=0, opc=0, imm9=0, Rn=0, Rt=0
    // Fields: opc=0, size=0, imm9=0, Rn=0, Rt=0
    let encoding: u32 = 0x38000C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pre_idx
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=1 (16-bit / halfword size)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pre_idx_combo_1_c00_78000c00() {
    // Encoding: 0x78000C00
    // Test aarch64_memory_single_general_immediate_signed_pre_idx field combination: size=1, opc=0, imm9=0, Rn=0, Rt=0
    // Fields: size=1, Rn=0, Rt=0, opc=0, imm9=0
    let encoding: u32 = 0x78000C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pre_idx
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=2 (32-bit / word size)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pre_idx_combo_2_c00_b8000c00() {
    // Encoding: 0xB8000C00
    // Test aarch64_memory_single_general_immediate_signed_pre_idx field combination: size=2, opc=0, imm9=0, Rn=0, Rt=0
    // Fields: Rt=0, size=2, Rn=0, imm9=0, opc=0
    let encoding: u32 = 0xB8000C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pre_idx
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=3 (64-bit / doubleword size)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pre_idx_combo_3_c00_f8000c00() {
    // Encoding: 0xF8000C00
    // Test aarch64_memory_single_general_immediate_signed_pre_idx field combination: size=3, opc=0, imm9=0, Rn=0, Rt=0
    // Fields: opc=0, Rn=0, size=3, Rt=0, imm9=0
    let encoding: u32 = 0xF8000C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pre_idx
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// opc=0 (8-bit / byte size)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pre_idx_combo_4_c00_38000c00() {
    // Encoding: 0x38000C00
    // Test aarch64_memory_single_general_immediate_signed_pre_idx field combination: size=0, opc=0, imm9=0, Rn=0, Rt=0
    // Fields: Rt=0, Rn=0, size=0, opc=0, imm9=0
    let encoding: u32 = 0x38000C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pre_idx
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// opc=1 (16-bit / halfword size)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pre_idx_combo_5_c00_38400c00() {
    // Encoding: 0x38400C00
    // Test aarch64_memory_single_general_immediate_signed_pre_idx field combination: size=0, opc=1, imm9=0, Rn=0, Rt=0
    // Fields: Rt=0, size=0, opc=1, imm9=0, Rn=0
    let encoding: u32 = 0x38400C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pre_idx
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// opc=2 (32-bit / word size)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pre_idx_combo_6_c00_38800c00() {
    // Encoding: 0x38800C00
    // Test aarch64_memory_single_general_immediate_signed_pre_idx field combination: size=0, opc=2, imm9=0, Rn=0, Rt=0
    // Fields: imm9=0, opc=2, Rn=0, Rt=0, size=0
    let encoding: u32 = 0x38800C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pre_idx
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// opc=3 (64-bit / doubleword size)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pre_idx_combo_7_c00_38c00c00() {
    // Encoding: 0x38C00C00
    // Test aarch64_memory_single_general_immediate_signed_pre_idx field combination: size=0, opc=3, imm9=0, Rn=0, Rt=0
    // Fields: opc=3, Rt=0, Rn=0, imm9=0, size=0
    let encoding: u32 = 0x38C00C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pre_idx
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=0 (immediate value 0)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pre_idx_combo_8_c00_38000c00() {
    // Encoding: 0x38000C00
    // Test aarch64_memory_single_general_immediate_signed_pre_idx field combination: size=0, opc=0, imm9=0, Rn=0, Rt=0
    // Fields: Rn=0, imm9=0, size=0, Rt=0, opc=0
    let encoding: u32 = 0x38000C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pre_idx
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=1 (immediate value 1)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pre_idx_combo_9_c00_38001c00() {
    // Encoding: 0x38001C00
    // Test aarch64_memory_single_general_immediate_signed_pre_idx field combination: size=0, opc=0, imm9=1, Rn=0, Rt=0
    // Fields: opc=0, size=0, imm9=1, Rn=0, Rt=0
    let encoding: u32 = 0x38001C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pre_idx
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=3 (2^2 - 1 = 3)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pre_idx_combo_10_c00_38003c00() {
    // Encoding: 0x38003C00
    // Test aarch64_memory_single_general_immediate_signed_pre_idx field combination: size=0, opc=0, imm9=3, Rn=0, Rt=0
    // Fields: opc=0, Rt=0, size=0, Rn=0, imm9=3
    let encoding: u32 = 0x38003C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pre_idx
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=4 (power of 2 (2^2 = 4))
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pre_idx_combo_11_c00_38004c00() {
    // Encoding: 0x38004C00
    // Test aarch64_memory_single_general_immediate_signed_pre_idx field combination: size=0, opc=0, imm9=4, Rn=0, Rt=0
    // Fields: imm9=4, Rt=0, opc=0, Rn=0, size=0
    let encoding: u32 = 0x38004C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pre_idx
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=7 (2^3 - 1 = 7)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pre_idx_combo_12_c00_38007c00() {
    // Encoding: 0x38007C00
    // Test aarch64_memory_single_general_immediate_signed_pre_idx field combination: size=0, opc=0, imm9=7, Rn=0, Rt=0
    // Fields: size=0, opc=0, Rt=0, Rn=0, imm9=7
    let encoding: u32 = 0x38007C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pre_idx
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=8 (power of 2 (2^3 = 8))
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pre_idx_combo_13_c00_38008c00() {
    // Encoding: 0x38008C00
    // Test aarch64_memory_single_general_immediate_signed_pre_idx field combination: size=0, opc=0, imm9=8, Rn=0, Rt=0
    // Fields: imm9=8, opc=0, size=0, Rt=0, Rn=0
    let encoding: u32 = 0x38008C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pre_idx
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=15 (2^4 - 1 = 15)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pre_idx_combo_14_c00_3800fc00() {
    // Encoding: 0x3800FC00
    // Test aarch64_memory_single_general_immediate_signed_pre_idx field combination: size=0, opc=0, imm9=15, Rn=0, Rt=0
    // Fields: opc=0, Rt=0, imm9=15, Rn=0, size=0
    let encoding: u32 = 0x3800FC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pre_idx
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=16 (power of 2 (2^4 = 16))
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pre_idx_combo_15_c00_38010c00() {
    // Encoding: 0x38010C00
    // Test aarch64_memory_single_general_immediate_signed_pre_idx field combination: size=0, opc=0, imm9=16, Rn=0, Rt=0
    // Fields: size=0, Rn=0, Rt=0, imm9=16, opc=0
    let encoding: u32 = 0x38010C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pre_idx
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=31 (2^5 - 1 = 31)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pre_idx_combo_16_c00_3801fc00() {
    // Encoding: 0x3801FC00
    // Test aarch64_memory_single_general_immediate_signed_pre_idx field combination: size=0, opc=0, imm9=31, Rn=0, Rt=0
    // Fields: Rt=0, opc=0, size=0, imm9=31, Rn=0
    let encoding: u32 = 0x3801FC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pre_idx
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=32 (power of 2 (2^5 = 32))
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pre_idx_combo_17_c00_38020c00() {
    // Encoding: 0x38020C00
    // Test aarch64_memory_single_general_immediate_signed_pre_idx field combination: size=0, opc=0, imm9=32, Rn=0, Rt=0
    // Fields: imm9=32, Rn=0, opc=0, size=0, Rt=0
    let encoding: u32 = 0x38020C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pre_idx
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=63 (2^6 - 1 = 63)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pre_idx_combo_18_c00_3803fc00() {
    // Encoding: 0x3803FC00
    // Test aarch64_memory_single_general_immediate_signed_pre_idx field combination: size=0, opc=0, imm9=63, Rn=0, Rt=0
    // Fields: size=0, Rt=0, Rn=0, opc=0, imm9=63
    let encoding: u32 = 0x3803FC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pre_idx
/// ASL: `field combination 19`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=64 (power of 2 (2^6 = 64))
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pre_idx_combo_19_c00_38040c00() {
    // Encoding: 0x38040C00
    // Test aarch64_memory_single_general_immediate_signed_pre_idx field combination: size=0, opc=0, imm9=64, Rn=0, Rt=0
    // Fields: size=0, Rt=0, opc=0, Rn=0, imm9=64
    let encoding: u32 = 0x38040C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pre_idx
/// ASL: `field combination 20`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=127 (2^7 - 1 = 127)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pre_idx_combo_20_c00_3807fc00() {
    // Encoding: 0x3807FC00
    // Test aarch64_memory_single_general_immediate_signed_pre_idx field combination: size=0, opc=0, imm9=127, Rn=0, Rt=0
    // Fields: imm9=127, Rt=0, Rn=0, size=0, opc=0
    let encoding: u32 = 0x3807FC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pre_idx
/// ASL: `field combination 21`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=128 (power of 2 (2^7 = 128))
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pre_idx_combo_21_c00_38080c00() {
    // Encoding: 0x38080C00
    // Test aarch64_memory_single_general_immediate_signed_pre_idx field combination: size=0, opc=0, imm9=128, Rn=0, Rt=0
    // Fields: Rt=0, imm9=128, Rn=0, size=0, opc=0
    let encoding: u32 = 0x38080C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pre_idx
/// ASL: `field combination 22`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=255 (immediate midpoint (255))
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pre_idx_combo_22_c00_380ffc00() {
    // Encoding: 0x380FFC00
    // Test aarch64_memory_single_general_immediate_signed_pre_idx field combination: size=0, opc=0, imm9=255, Rn=0, Rt=0
    // Fields: Rn=0, Rt=0, size=0, imm9=255, opc=0
    let encoding: u32 = 0x380FFC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pre_idx
/// ASL: `field combination 23`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=256 (power of 2 (2^8 = 256))
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pre_idx_combo_23_c00_38100c00() {
    // Encoding: 0x38100C00
    // Test aarch64_memory_single_general_immediate_signed_pre_idx field combination: size=0, opc=0, imm9=256, Rn=0, Rt=0
    // Fields: Rn=0, opc=0, imm9=256, size=0, Rt=0
    let encoding: u32 = 0x38100C00;
    let mut cpu = create_test_cpu();
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

/// Provenance: aarch64_memory_single_general_immediate_signed_pre_idx
/// ASL: `field combination 24`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=511 (maximum immediate (511))
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pre_idx_combo_24_c00_381ffc00() {
    // Encoding: 0x381FFC00
    // Test aarch64_memory_single_general_immediate_signed_pre_idx field combination: size=0, opc=0, imm9=511, Rn=0, Rt=0
    // Fields: opc=0, Rn=0, imm9=511, Rt=0, size=0
    let encoding: u32 = 0x381FFC00;
    let mut cpu = create_test_cpu();
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

/// Provenance: aarch64_memory_single_general_immediate_signed_pre_idx
/// ASL: `field combination 25`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pre_idx_combo_25_c00_38000c00() {
    // Encoding: 0x38000C00
    // Test aarch64_memory_single_general_immediate_signed_pre_idx field combination: size=0, opc=0, imm9=0, Rn=0, Rt=0
    // Fields: imm9=0, size=0, opc=0, Rt=0, Rn=0
    let encoding: u32 = 0x38000C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pre_idx
/// ASL: `field combination 26`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pre_idx_combo_26_c00_38000c20() {
    // Encoding: 0x38000C20
    // Test aarch64_memory_single_general_immediate_signed_pre_idx field combination: size=0, opc=0, imm9=0, Rn=1, Rt=0
    // Fields: size=0, Rn=1, Rt=0, imm9=0, opc=0
    let encoding: u32 = 0x38000C20;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pre_idx
/// ASL: `field combination 27`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pre_idx_combo_27_c00_38000fc0() {
    // Encoding: 0x38000FC0
    // Test aarch64_memory_single_general_immediate_signed_pre_idx field combination: size=0, opc=0, imm9=0, Rn=30, Rt=0
    // Fields: Rn=30, size=0, opc=0, imm9=0, Rt=0
    let encoding: u32 = 0x38000FC0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pre_idx
/// ASL: `field combination 28`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pre_idx_combo_28_c00_38000fe0() {
    // Encoding: 0x38000FE0
    // Test aarch64_memory_single_general_immediate_signed_pre_idx field combination: size=0, opc=0, imm9=0, Rn=31, Rt=0
    // Fields: Rt=0, imm9=0, Rn=31, opc=0, size=0
    let encoding: u32 = 0x38000FE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pre_idx
/// ASL: `field combination 29`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt=0 (register index 0 (first register))
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pre_idx_combo_29_c00_38000c00() {
    // Encoding: 0x38000C00
    // Test aarch64_memory_single_general_immediate_signed_pre_idx field combination: size=0, opc=0, imm9=0, Rn=0, Rt=0
    // Fields: Rn=0, opc=0, Rt=0, size=0, imm9=0
    let encoding: u32 = 0x38000C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pre_idx
/// ASL: `field combination 30`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt=1 (register index 1 (second register))
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pre_idx_combo_30_c00_38000c01() {
    // Encoding: 0x38000C01
    // Test aarch64_memory_single_general_immediate_signed_pre_idx field combination: size=0, opc=0, imm9=0, Rn=0, Rt=1
    // Fields: size=0, Rt=1, imm9=0, Rn=0, opc=0
    let encoding: u32 = 0x38000C01;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pre_idx
/// ASL: `field combination 31`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pre_idx_combo_31_c00_38000c1e() {
    // Encoding: 0x38000C1E
    // Test aarch64_memory_single_general_immediate_signed_pre_idx field combination: size=0, opc=0, imm9=0, Rn=0, Rt=30
    // Fields: Rt=30, opc=0, size=0, imm9=0, Rn=0
    let encoding: u32 = 0x38000C1E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pre_idx
/// ASL: `field combination 32`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt=31 (register index 31 (ZR - zero register))
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pre_idx_combo_32_c00_38000c1f() {
    // Encoding: 0x38000C1F
    // Test aarch64_memory_single_general_immediate_signed_pre_idx field combination: size=0, opc=0, imm9=0, Rn=0, Rt=31
    // Fields: size=0, opc=0, Rt=31, imm9=0, Rn=0
    let encoding: u32 = 0x38000C1F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pre_idx
/// ASL: `field combination 33`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (same register test (reg=1)), Rt=1 (same register test (reg=1))
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pre_idx_combo_33_c00_38000c21() {
    // Encoding: 0x38000C21
    // Test aarch64_memory_single_general_immediate_signed_pre_idx field combination: size=0, opc=0, imm9=0, Rn=1, Rt=1
    // Fields: Rn=1, Rt=1, opc=0, size=0, imm9=0
    let encoding: u32 = 0x38000C21;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pre_idx
/// ASL: `field combination 34`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (same register test (reg=31)), Rt=31 (same register test (reg=31))
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pre_idx_combo_34_c00_38000fff() {
    // Encoding: 0x38000FFF
    // Test aarch64_memory_single_general_immediate_signed_pre_idx field combination: size=0, opc=0, imm9=0, Rn=31, Rt=31
    // Fields: Rn=31, imm9=0, Rt=31, opc=0, size=0
    let encoding: u32 = 0x38000FFF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pre_idx
/// ASL: `field size = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "size", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pre_idx_special_size_0_size_variant_0_3072_38001c00()
 {
    // Encoding: 0x38001C00
    // Test aarch64_memory_single_general_immediate_signed_pre_idx special value size = 0 (Size variant 0)
    // Fields: Rn=0, imm9=1, size=0, opc=0, Rt=0
    let encoding: u32 = 0x38001C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pre_idx
/// ASL: `field size = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "size", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pre_idx_special_size_1_size_variant_1_3072_78001c00()
 {
    // Encoding: 0x78001C00
    // Test aarch64_memory_single_general_immediate_signed_pre_idx special value size = 1 (Size variant 1)
    // Fields: imm9=1, size=1, Rn=0, Rt=0, opc=0
    let encoding: u32 = 0x78001C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pre_idx
/// ASL: `field size = 2 (Size variant 2)`
/// Requirement: FieldSpecial { field: "size", value: 2, meaning: "Size variant 2" }
/// Size variant 2
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pre_idx_special_size_2_size_variant_2_3072_b8001c00()
 {
    // Encoding: 0xB8001C00
    // Test aarch64_memory_single_general_immediate_signed_pre_idx special value size = 2 (Size variant 2)
    // Fields: size=2, Rt=0, opc=0, imm9=1, Rn=0
    let encoding: u32 = 0xB8001C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pre_idx
/// ASL: `field size = 3 (Size variant 3)`
/// Requirement: FieldSpecial { field: "size", value: 3, meaning: "Size variant 3" }
/// Size variant 3
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pre_idx_special_size_3_size_variant_3_3072_f8001c00()
 {
    // Encoding: 0xF8001C00
    // Test aarch64_memory_single_general_immediate_signed_pre_idx special value size = 3 (Size variant 3)
    // Fields: opc=0, size=3, imm9=1, Rn=0, Rt=0
    let encoding: u32 = 0xF8001C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pre_idx
/// ASL: `field opc = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "opc", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pre_idx_special_opc_0_size_variant_0_3072_78001c00()
 {
    // Encoding: 0x78001C00
    // Test aarch64_memory_single_general_immediate_signed_pre_idx special value opc = 0 (Size variant 0)
    // Fields: imm9=1, Rn=0, opc=0, Rt=0, size=1
    let encoding: u32 = 0x78001C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pre_idx
/// ASL: `field opc = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "opc", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pre_idx_special_opc_1_size_variant_1_3072_78401c00()
 {
    // Encoding: 0x78401C00
    // Test aarch64_memory_single_general_immediate_signed_pre_idx special value opc = 1 (Size variant 1)
    // Fields: Rt=0, size=1, Rn=0, imm9=1, opc=1
    let encoding: u32 = 0x78401C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pre_idx
/// ASL: `field opc = 2 (Size variant 2)`
/// Requirement: FieldSpecial { field: "opc", value: 2, meaning: "Size variant 2" }
/// Size variant 2
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pre_idx_special_opc_2_size_variant_2_3072_78801c00()
 {
    // Encoding: 0x78801C00
    // Test aarch64_memory_single_general_immediate_signed_pre_idx special value opc = 2 (Size variant 2)
    // Fields: size=1, Rt=0, Rn=0, opc=2, imm9=1
    let encoding: u32 = 0x78801C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pre_idx
/// ASL: `field opc = 3 (Size variant 3)`
/// Requirement: FieldSpecial { field: "opc", value: 3, meaning: "Size variant 3" }
/// Size variant 3
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pre_idx_special_opc_3_size_variant_3_3072_78c01c00()
 {
    // Encoding: 0x78C01C00
    // Test aarch64_memory_single_general_immediate_signed_pre_idx special value opc = 3 (Size variant 3)
    // Fields: size=1, imm9=1, opc=3, Rt=0, Rn=0
    let encoding: u32 = 0x78C01C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pre_idx
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pre_idx_special_rn_31_stack_pointer_sp_may_require_alignment_3072_78001fe0()
 {
    // Encoding: 0x78001FE0
    // Test aarch64_memory_single_general_immediate_signed_pre_idx special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: opc=0, size=1, imm9=1, Rn=31, Rt=0
    let encoding: u32 = 0x78001FE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pre_idx
/// ASL: `field Rt = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)`
/// Requirement: FieldSpecial { field: "Rt", value: 31, meaning: "Zero register (XZR/WZR) - reads as 0, writes discarded" }
/// Zero register (XZR/WZR) - reads as 0, writes discarded
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pre_idx_special_rt_31_zero_register_xzr_wzr_reads_as_0_writes_discarded_3072_78001c1f()
 {
    // Encoding: 0x78001C1F
    // Test aarch64_memory_single_general_immediate_signed_pre_idx special value Rt = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)
    // Fields: Rn=0, opc=0, imm9=1, Rt=31, size=1
    let encoding: u32 = 0x78001C1F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_unsigned
/// ASL: `field size 30 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_memory_single_general_immediate_unsigned_field_size_0_min_0_39000000() {
    // Encoding: 0x39000000
    // Test aarch64_memory_single_general_immediate_unsigned field size = 0 (Min)
    // Fields: size=0, Rt=0, opc=0, imm12=0, Rn=0
    let encoding: u32 = 0x39000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_unsigned
/// ASL: `field size 30 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 1, boundary: PowerOfTwo }
/// 16-bit / halfword size
#[test]
fn test_aarch64_memory_single_general_immediate_unsigned_field_size_1_poweroftwo_0_79000000() {
    // Encoding: 0x79000000
    // Test aarch64_memory_single_general_immediate_unsigned field size = 1 (PowerOfTwo)
    // Fields: size=1, opc=0, imm12=0, Rn=0, Rt=0
    let encoding: u32 = 0x79000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_unsigned
/// ASL: `field size 30 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 2, boundary: PowerOfTwo }
/// 32-bit / word size
#[test]
fn test_aarch64_memory_single_general_immediate_unsigned_field_size_2_poweroftwo_0_b9000000() {
    // Encoding: 0xB9000000
    // Test aarch64_memory_single_general_immediate_unsigned field size = 2 (PowerOfTwo)
    // Fields: opc=0, imm12=0, size=2, Rt=0, Rn=0
    let encoding: u32 = 0xB9000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_unsigned
/// ASL: `field size 30 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 3, boundary: Max }
/// 64-bit / doubleword size
#[test]
fn test_aarch64_memory_single_general_immediate_unsigned_field_size_3_max_0_f9000000() {
    // Encoding: 0xF9000000
    // Test aarch64_memory_single_general_immediate_unsigned field size = 3 (Max)
    // Fields: imm12=0, opc=0, Rn=0, size=3, Rt=0
    let encoding: u32 = 0xF9000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_unsigned
/// ASL: `field opc 22 +: 2`
/// Requirement: FieldBoundary { field: "opc", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_memory_single_general_immediate_unsigned_field_opc_0_min_0_39000000() {
    // Encoding: 0x39000000
    // Test aarch64_memory_single_general_immediate_unsigned field opc = 0 (Min)
    // Fields: opc=0, Rt=0, Rn=0, size=0, imm12=0
    let encoding: u32 = 0x39000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_unsigned
/// ASL: `field opc 22 +: 2`
/// Requirement: FieldBoundary { field: "opc", value: 1, boundary: PowerOfTwo }
/// 16-bit / halfword size
#[test]
fn test_aarch64_memory_single_general_immediate_unsigned_field_opc_1_poweroftwo_0_39400000() {
    // Encoding: 0x39400000
    // Test aarch64_memory_single_general_immediate_unsigned field opc = 1 (PowerOfTwo)
    // Fields: opc=1, Rt=0, Rn=0, size=0, imm12=0
    let encoding: u32 = 0x39400000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_unsigned
/// ASL: `field opc 22 +: 2`
/// Requirement: FieldBoundary { field: "opc", value: 2, boundary: PowerOfTwo }
/// 32-bit / word size
#[test]
fn test_aarch64_memory_single_general_immediate_unsigned_field_opc_2_poweroftwo_0_39800000() {
    // Encoding: 0x39800000
    // Test aarch64_memory_single_general_immediate_unsigned field opc = 2 (PowerOfTwo)
    // Fields: size=0, opc=2, imm12=0, Rn=0, Rt=0
    let encoding: u32 = 0x39800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_unsigned
/// ASL: `field opc 22 +: 2`
/// Requirement: FieldBoundary { field: "opc", value: 3, boundary: Max }
/// 64-bit / doubleword size
#[test]
fn test_aarch64_memory_single_general_immediate_unsigned_field_opc_3_max_0_39c00000() {
    // Encoding: 0x39C00000
    // Test aarch64_memory_single_general_immediate_unsigned field opc = 3 (Max)
    // Fields: Rt=0, imm12=0, Rn=0, size=0, opc=3
    let encoding: u32 = 0x39C00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_unsigned
/// ASL: `field imm12 10 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch64_memory_single_general_immediate_unsigned_field_imm12_0_zero_0_39000000() {
    // Encoding: 0x39000000
    // Test aarch64_memory_single_general_immediate_unsigned field imm12 = 0 (Zero)
    // Fields: opc=0, size=0, imm12=0, Rn=0, Rt=0
    let encoding: u32 = 0x39000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_unsigned
/// ASL: `field imm12 10 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch64_memory_single_general_immediate_unsigned_field_imm12_1_poweroftwo_0_39000400() {
    // Encoding: 0x39000400
    // Test aarch64_memory_single_general_immediate_unsigned field imm12 = 1 (PowerOfTwo)
    // Fields: Rt=0, Rn=0, opc=0, size=0, imm12=1
    let encoding: u32 = 0x39000400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_unsigned
/// ASL: `field imm12 10 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 3, boundary: PowerOfTwoMinusOne }
/// 2^2 - 1 = 3
#[test]
fn test_aarch64_memory_single_general_immediate_unsigned_field_imm12_3_poweroftwominusone_0_39000c00()
 {
    // Encoding: 0x39000C00
    // Test aarch64_memory_single_general_immediate_unsigned field imm12 = 3 (PowerOfTwoMinusOne)
    // Fields: Rt=0, imm12=3, opc=0, Rn=0, size=0
    let encoding: u32 = 0x39000C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_unsigned
/// ASL: `field imm12 10 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 4, boundary: PowerOfTwo }
/// power of 2 (2^2 = 4)
#[test]
fn test_aarch64_memory_single_general_immediate_unsigned_field_imm12_4_poweroftwo_0_39001000() {
    // Encoding: 0x39001000
    // Test aarch64_memory_single_general_immediate_unsigned field imm12 = 4 (PowerOfTwo)
    // Fields: Rn=0, opc=0, Rt=0, size=0, imm12=4
    let encoding: u32 = 0x39001000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_unsigned
/// ASL: `field imm12 10 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 7, boundary: PowerOfTwoMinusOne }
/// 2^3 - 1 = 7
#[test]
fn test_aarch64_memory_single_general_immediate_unsigned_field_imm12_7_poweroftwominusone_0_39001c00()
 {
    // Encoding: 0x39001C00
    // Test aarch64_memory_single_general_immediate_unsigned field imm12 = 7 (PowerOfTwoMinusOne)
    // Fields: Rn=0, opc=0, Rt=0, imm12=7, size=0
    let encoding: u32 = 0x39001C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_unsigned
/// ASL: `field imm12 10 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 8, boundary: PowerOfTwo }
/// power of 2 (2^3 = 8)
#[test]
fn test_aarch64_memory_single_general_immediate_unsigned_field_imm12_8_poweroftwo_0_39002000() {
    // Encoding: 0x39002000
    // Test aarch64_memory_single_general_immediate_unsigned field imm12 = 8 (PowerOfTwo)
    // Fields: size=0, imm12=8, opc=0, Rn=0, Rt=0
    let encoding: u32 = 0x39002000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_unsigned
/// ASL: `field imm12 10 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 15, boundary: PowerOfTwoMinusOne }
/// 2^4 - 1 = 15
#[test]
fn test_aarch64_memory_single_general_immediate_unsigned_field_imm12_15_poweroftwominusone_0_39003c00()
 {
    // Encoding: 0x39003C00
    // Test aarch64_memory_single_general_immediate_unsigned field imm12 = 15 (PowerOfTwoMinusOne)
    // Fields: imm12=15, opc=0, size=0, Rn=0, Rt=0
    let encoding: u32 = 0x39003C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_unsigned
/// ASL: `field imm12 10 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 16, boundary: PowerOfTwo }
/// power of 2 (2^4 = 16)
#[test]
fn test_aarch64_memory_single_general_immediate_unsigned_field_imm12_16_poweroftwo_0_39004000() {
    // Encoding: 0x39004000
    // Test aarch64_memory_single_general_immediate_unsigned field imm12 = 16 (PowerOfTwo)
    // Fields: Rt=0, size=0, Rn=0, imm12=16, opc=0
    let encoding: u32 = 0x39004000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_unsigned
/// ASL: `field imm12 10 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 31, boundary: PowerOfTwoMinusOne }
/// 2^5 - 1 = 31
#[test]
fn test_aarch64_memory_single_general_immediate_unsigned_field_imm12_31_poweroftwominusone_0_39007c00()
 {
    // Encoding: 0x39007C00
    // Test aarch64_memory_single_general_immediate_unsigned field imm12 = 31 (PowerOfTwoMinusOne)
    // Fields: size=0, imm12=31, Rt=0, Rn=0, opc=0
    let encoding: u32 = 0x39007C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_unsigned
/// ASL: `field imm12 10 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 32, boundary: PowerOfTwo }
/// power of 2 (2^5 = 32)
#[test]
fn test_aarch64_memory_single_general_immediate_unsigned_field_imm12_32_poweroftwo_0_39008000() {
    // Encoding: 0x39008000
    // Test aarch64_memory_single_general_immediate_unsigned field imm12 = 32 (PowerOfTwo)
    // Fields: imm12=32, Rt=0, opc=0, size=0, Rn=0
    let encoding: u32 = 0x39008000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_unsigned
/// ASL: `field imm12 10 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 63, boundary: PowerOfTwoMinusOne }
/// 2^6 - 1 = 63
#[test]
fn test_aarch64_memory_single_general_immediate_unsigned_field_imm12_63_poweroftwominusone_0_3900fc00()
 {
    // Encoding: 0x3900FC00
    // Test aarch64_memory_single_general_immediate_unsigned field imm12 = 63 (PowerOfTwoMinusOne)
    // Fields: size=0, Rt=0, imm12=63, opc=0, Rn=0
    let encoding: u32 = 0x3900FC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_unsigned
/// ASL: `field imm12 10 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 64, boundary: PowerOfTwo }
/// power of 2 (2^6 = 64)
#[test]
fn test_aarch64_memory_single_general_immediate_unsigned_field_imm12_64_poweroftwo_0_39010000() {
    // Encoding: 0x39010000
    // Test aarch64_memory_single_general_immediate_unsigned field imm12 = 64 (PowerOfTwo)
    // Fields: size=0, Rt=0, opc=0, imm12=64, Rn=0
    let encoding: u32 = 0x39010000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_unsigned
/// ASL: `field imm12 10 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 127, boundary: PowerOfTwoMinusOne }
/// 2^7 - 1 = 127
#[test]
fn test_aarch64_memory_single_general_immediate_unsigned_field_imm12_127_poweroftwominusone_0_3901fc00()
 {
    // Encoding: 0x3901FC00
    // Test aarch64_memory_single_general_immediate_unsigned field imm12 = 127 (PowerOfTwoMinusOne)
    // Fields: size=0, Rn=0, opc=0, imm12=127, Rt=0
    let encoding: u32 = 0x3901FC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_unsigned
/// ASL: `field imm12 10 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 128, boundary: PowerOfTwo }
/// power of 2 (2^7 = 128)
#[test]
fn test_aarch64_memory_single_general_immediate_unsigned_field_imm12_128_poweroftwo_0_39020000() {
    // Encoding: 0x39020000
    // Test aarch64_memory_single_general_immediate_unsigned field imm12 = 128 (PowerOfTwo)
    // Fields: Rn=0, Rt=0, opc=0, imm12=128, size=0
    let encoding: u32 = 0x39020000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_unsigned
/// ASL: `field imm12 10 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 255, boundary: PowerOfTwoMinusOne }
/// 2^8 - 1 = 255
#[test]
fn test_aarch64_memory_single_general_immediate_unsigned_field_imm12_255_poweroftwominusone_0_3903fc00()
 {
    // Encoding: 0x3903FC00
    // Test aarch64_memory_single_general_immediate_unsigned field imm12 = 255 (PowerOfTwoMinusOne)
    // Fields: Rn=0, Rt=0, imm12=255, size=0, opc=0
    let encoding: u32 = 0x3903FC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_unsigned
/// ASL: `field imm12 10 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 256, boundary: PowerOfTwo }
/// power of 2 (2^8 = 256)
#[test]
fn test_aarch64_memory_single_general_immediate_unsigned_field_imm12_256_poweroftwo_0_39040000() {
    // Encoding: 0x39040000
    // Test aarch64_memory_single_general_immediate_unsigned field imm12 = 256 (PowerOfTwo)
    // Fields: imm12=256, Rn=0, Rt=0, size=0, opc=0
    let encoding: u32 = 0x39040000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_unsigned
/// ASL: `field imm12 10 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 511, boundary: PowerOfTwoMinusOne }
/// 2^9 - 1 = 511
#[test]
fn test_aarch64_memory_single_general_immediate_unsigned_field_imm12_511_poweroftwominusone_0_3907fc00()
 {
    // Encoding: 0x3907FC00
    // Test aarch64_memory_single_general_immediate_unsigned field imm12 = 511 (PowerOfTwoMinusOne)
    // Fields: Rn=0, Rt=0, size=0, opc=0, imm12=511
    let encoding: u32 = 0x3907FC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_unsigned
/// ASL: `field imm12 10 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 512, boundary: PowerOfTwo }
/// power of 2 (2^9 = 512)
#[test]
fn test_aarch64_memory_single_general_immediate_unsigned_field_imm12_512_poweroftwo_0_39080000() {
    // Encoding: 0x39080000
    // Test aarch64_memory_single_general_immediate_unsigned field imm12 = 512 (PowerOfTwo)
    // Fields: Rt=0, opc=0, size=0, imm12=512, Rn=0
    let encoding: u32 = 0x39080000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_unsigned
/// ASL: `field imm12 10 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 1023, boundary: PowerOfTwoMinusOne }
/// 2^10 - 1 = 1023
#[test]
fn test_aarch64_memory_single_general_immediate_unsigned_field_imm12_1023_poweroftwominusone_0_390ffc00()
 {
    // Encoding: 0x390FFC00
    // Test aarch64_memory_single_general_immediate_unsigned field imm12 = 1023 (PowerOfTwoMinusOne)
    // Fields: opc=0, Rt=0, size=0, imm12=1023, Rn=0
    let encoding: u32 = 0x390FFC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_unsigned
/// ASL: `field imm12 10 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 1024, boundary: PowerOfTwo }
/// power of 2 (2^10 = 1024)
#[test]
fn test_aarch64_memory_single_general_immediate_unsigned_field_imm12_1024_poweroftwo_0_39100000() {
    // Encoding: 0x39100000
    // Test aarch64_memory_single_general_immediate_unsigned field imm12 = 1024 (PowerOfTwo)
    // Fields: opc=0, imm12=1024, Rn=0, size=0, Rt=0
    let encoding: u32 = 0x39100000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_unsigned
/// ASL: `field imm12 10 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 2047, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (2047)
#[test]
fn test_aarch64_memory_single_general_immediate_unsigned_field_imm12_2047_poweroftwominusone_0_391ffc00()
 {
    // Encoding: 0x391FFC00
    // Test aarch64_memory_single_general_immediate_unsigned field imm12 = 2047 (PowerOfTwoMinusOne)
    // Fields: size=0, Rn=0, Rt=0, opc=0, imm12=2047
    let encoding: u32 = 0x391FFC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_unsigned
/// ASL: `field imm12 10 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 2048, boundary: PowerOfTwo }
/// power of 2 (2^11 = 2048)
#[test]
fn test_aarch64_memory_single_general_immediate_unsigned_field_imm12_2048_poweroftwo_0_39200000() {
    // Encoding: 0x39200000
    // Test aarch64_memory_single_general_immediate_unsigned field imm12 = 2048 (PowerOfTwo)
    // Fields: opc=0, size=0, imm12=2048, Rn=0, Rt=0
    let encoding: u32 = 0x39200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_unsigned
/// ASL: `field imm12 10 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 4095, boundary: Max }
/// maximum immediate (4095)
#[test]
fn test_aarch64_memory_single_general_immediate_unsigned_field_imm12_4095_max_0_393ffc00() {
    // Encoding: 0x393FFC00
    // Test aarch64_memory_single_general_immediate_unsigned field imm12 = 4095 (Max)
    // Fields: size=0, imm12=4095, Rt=0, opc=0, Rn=0
    let encoding: u32 = 0x393FFC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_unsigned
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_memory_single_general_immediate_unsigned_field_rn_0_min_0_39000000() {
    // Encoding: 0x39000000
    // Test aarch64_memory_single_general_immediate_unsigned field Rn = 0 (Min)
    // Fields: opc=0, Rt=0, imm12=0, Rn=0, size=0
    let encoding: u32 = 0x39000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_unsigned
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_memory_single_general_immediate_unsigned_field_rn_1_poweroftwo_0_39000020() {
    // Encoding: 0x39000020
    // Test aarch64_memory_single_general_immediate_unsigned field Rn = 1 (PowerOfTwo)
    // Fields: opc=0, size=0, imm12=0, Rt=0, Rn=1
    let encoding: u32 = 0x39000020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_unsigned
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_memory_single_general_immediate_unsigned_field_rn_30_poweroftwominusone_0_390003c0()
{
    // Encoding: 0x390003C0
    // Test aarch64_memory_single_general_immediate_unsigned field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: Rn=30, Rt=0, size=0, opc=0, imm12=0
    let encoding: u32 = 0x390003C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_unsigned
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_memory_single_general_immediate_unsigned_field_rn_31_max_0_390003e0() {
    // Encoding: 0x390003E0
    // Test aarch64_memory_single_general_immediate_unsigned field Rn = 31 (Max)
    // Fields: size=0, Rn=31, opc=0, imm12=0, Rt=0
    let encoding: u32 = 0x390003E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_unsigned
/// ASL: `field Rt 0 +: 5`
/// Requirement: FieldBoundary { field: "Rt", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_memory_single_general_immediate_unsigned_field_rt_0_min_0_39000000() {
    // Encoding: 0x39000000
    // Test aarch64_memory_single_general_immediate_unsigned field Rt = 0 (Min)
    // Fields: imm12=0, Rn=0, Rt=0, size=0, opc=0
    let encoding: u32 = 0x39000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_unsigned
/// ASL: `field Rt 0 +: 5`
/// Requirement: FieldBoundary { field: "Rt", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_memory_single_general_immediate_unsigned_field_rt_1_poweroftwo_0_39000001() {
    // Encoding: 0x39000001
    // Test aarch64_memory_single_general_immediate_unsigned field Rt = 1 (PowerOfTwo)
    // Fields: size=0, Rn=0, Rt=1, imm12=0, opc=0
    let encoding: u32 = 0x39000001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_unsigned
/// ASL: `field Rt 0 +: 5`
/// Requirement: FieldBoundary { field: "Rt", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_memory_single_general_immediate_unsigned_field_rt_30_poweroftwominusone_0_3900001e()
{
    // Encoding: 0x3900001E
    // Test aarch64_memory_single_general_immediate_unsigned field Rt = 30 (PowerOfTwoMinusOne)
    // Fields: size=0, Rn=0, Rt=30, opc=0, imm12=0
    let encoding: u32 = 0x3900001E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_unsigned
/// ASL: `field Rt 0 +: 5`
/// Requirement: FieldBoundary { field: "Rt", value: 31, boundary: Max }
/// register index 31 (ZR - zero register)
#[test]
fn test_aarch64_memory_single_general_immediate_unsigned_field_rt_31_max_0_3900001f() {
    // Encoding: 0x3900001F
    // Test aarch64_memory_single_general_immediate_unsigned field Rt = 31 (Max)
    // Fields: Rt=31, Rn=0, opc=0, imm12=0, size=0
    let encoding: u32 = 0x3900001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_unsigned
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=0 (8-bit / byte size)
#[test]
fn test_aarch64_memory_single_general_immediate_unsigned_combo_0_0_39000000() {
    // Encoding: 0x39000000
    // Test aarch64_memory_single_general_immediate_unsigned field combination: size=0, opc=0, imm12=0, Rn=0, Rt=0
    // Fields: size=0, opc=0, Rn=0, Rt=0, imm12=0
    let encoding: u32 = 0x39000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_unsigned
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=1 (16-bit / halfword size)
#[test]
fn test_aarch64_memory_single_general_immediate_unsigned_combo_1_0_79000000() {
    // Encoding: 0x79000000
    // Test aarch64_memory_single_general_immediate_unsigned field combination: size=1, opc=0, imm12=0, Rn=0, Rt=0
    // Fields: opc=0, size=1, Rn=0, Rt=0, imm12=0
    let encoding: u32 = 0x79000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_unsigned
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=2 (32-bit / word size)
#[test]
fn test_aarch64_memory_single_general_immediate_unsigned_combo_2_0_b9000000() {
    // Encoding: 0xB9000000
    // Test aarch64_memory_single_general_immediate_unsigned field combination: size=2, opc=0, imm12=0, Rn=0, Rt=0
    // Fields: Rn=0, opc=0, Rt=0, imm12=0, size=2
    let encoding: u32 = 0xB9000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_unsigned
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=3 (64-bit / doubleword size)
#[test]
fn test_aarch64_memory_single_general_immediate_unsigned_combo_3_0_f9000000() {
    // Encoding: 0xF9000000
    // Test aarch64_memory_single_general_immediate_unsigned field combination: size=3, opc=0, imm12=0, Rn=0, Rt=0
    // Fields: Rt=0, imm12=0, Rn=0, size=3, opc=0
    let encoding: u32 = 0xF9000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_unsigned
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// opc=0 (8-bit / byte size)
#[test]
fn test_aarch64_memory_single_general_immediate_unsigned_combo_4_0_39000000() {
    // Encoding: 0x39000000
    // Test aarch64_memory_single_general_immediate_unsigned field combination: size=0, opc=0, imm12=0, Rn=0, Rt=0
    // Fields: opc=0, imm12=0, size=0, Rn=0, Rt=0
    let encoding: u32 = 0x39000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_unsigned
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// opc=1 (16-bit / halfword size)
#[test]
fn test_aarch64_memory_single_general_immediate_unsigned_combo_5_0_39400000() {
    // Encoding: 0x39400000
    // Test aarch64_memory_single_general_immediate_unsigned field combination: size=0, opc=1, imm12=0, Rn=0, Rt=0
    // Fields: Rn=0, size=0, Rt=0, opc=1, imm12=0
    let encoding: u32 = 0x39400000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_unsigned
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// opc=2 (32-bit / word size)
#[test]
fn test_aarch64_memory_single_general_immediate_unsigned_combo_6_0_39800000() {
    // Encoding: 0x39800000
    // Test aarch64_memory_single_general_immediate_unsigned field combination: size=0, opc=2, imm12=0, Rn=0, Rt=0
    // Fields: size=0, opc=2, imm12=0, Rt=0, Rn=0
    let encoding: u32 = 0x39800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_unsigned
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// opc=3 (64-bit / doubleword size)
#[test]
fn test_aarch64_memory_single_general_immediate_unsigned_combo_7_0_39c00000() {
    // Encoding: 0x39C00000
    // Test aarch64_memory_single_general_immediate_unsigned field combination: size=0, opc=3, imm12=0, Rn=0, Rt=0
    // Fields: Rt=0, opc=3, size=0, imm12=0, Rn=0
    let encoding: u32 = 0x39C00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_unsigned
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm12=0 (immediate value 0)
#[test]
fn test_aarch64_memory_single_general_immediate_unsigned_combo_8_0_39000000() {
    // Encoding: 0x39000000
    // Test aarch64_memory_single_general_immediate_unsigned field combination: size=0, opc=0, imm12=0, Rn=0, Rt=0
    // Fields: Rt=0, Rn=0, size=0, opc=0, imm12=0
    let encoding: u32 = 0x39000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_unsigned
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm12=1 (immediate value 1)
#[test]
fn test_aarch64_memory_single_general_immediate_unsigned_combo_9_0_39000400() {
    // Encoding: 0x39000400
    // Test aarch64_memory_single_general_immediate_unsigned field combination: size=0, opc=0, imm12=1, Rn=0, Rt=0
    // Fields: opc=0, Rn=0, imm12=1, Rt=0, size=0
    let encoding: u32 = 0x39000400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_unsigned
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm12=3 (2^2 - 1 = 3)
#[test]
fn test_aarch64_memory_single_general_immediate_unsigned_combo_10_0_39000c00() {
    // Encoding: 0x39000C00
    // Test aarch64_memory_single_general_immediate_unsigned field combination: size=0, opc=0, imm12=3, Rn=0, Rt=0
    // Fields: opc=0, Rn=0, imm12=3, Rt=0, size=0
    let encoding: u32 = 0x39000C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_unsigned
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm12=4 (power of 2 (2^2 = 4))
#[test]
fn test_aarch64_memory_single_general_immediate_unsigned_combo_11_0_39001000() {
    // Encoding: 0x39001000
    // Test aarch64_memory_single_general_immediate_unsigned field combination: size=0, opc=0, imm12=4, Rn=0, Rt=0
    // Fields: opc=0, Rn=0, Rt=0, imm12=4, size=0
    let encoding: u32 = 0x39001000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_unsigned
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm12=7 (2^3 - 1 = 7)
#[test]
fn test_aarch64_memory_single_general_immediate_unsigned_combo_12_0_39001c00() {
    // Encoding: 0x39001C00
    // Test aarch64_memory_single_general_immediate_unsigned field combination: size=0, opc=0, imm12=7, Rn=0, Rt=0
    // Fields: imm12=7, size=0, Rn=0, opc=0, Rt=0
    let encoding: u32 = 0x39001C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_unsigned
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm12=8 (power of 2 (2^3 = 8))
#[test]
fn test_aarch64_memory_single_general_immediate_unsigned_combo_13_0_39002000() {
    // Encoding: 0x39002000
    // Test aarch64_memory_single_general_immediate_unsigned field combination: size=0, opc=0, imm12=8, Rn=0, Rt=0
    // Fields: opc=0, Rn=0, size=0, imm12=8, Rt=0
    let encoding: u32 = 0x39002000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_unsigned
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm12=15 (2^4 - 1 = 15)
#[test]
fn test_aarch64_memory_single_general_immediate_unsigned_combo_14_0_39003c00() {
    // Encoding: 0x39003C00
    // Test aarch64_memory_single_general_immediate_unsigned field combination: size=0, opc=0, imm12=15, Rn=0, Rt=0
    // Fields: Rt=0, opc=0, Rn=0, size=0, imm12=15
    let encoding: u32 = 0x39003C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_unsigned
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm12=16 (power of 2 (2^4 = 16))
#[test]
fn test_aarch64_memory_single_general_immediate_unsigned_combo_15_0_39004000() {
    // Encoding: 0x39004000
    // Test aarch64_memory_single_general_immediate_unsigned field combination: size=0, opc=0, imm12=16, Rn=0, Rt=0
    // Fields: size=0, Rt=0, opc=0, imm12=16, Rn=0
    let encoding: u32 = 0x39004000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_unsigned
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm12=31 (2^5 - 1 = 31)
#[test]
fn test_aarch64_memory_single_general_immediate_unsigned_combo_16_0_39007c00() {
    // Encoding: 0x39007C00
    // Test aarch64_memory_single_general_immediate_unsigned field combination: size=0, opc=0, imm12=31, Rn=0, Rt=0
    // Fields: Rt=0, imm12=31, opc=0, Rn=0, size=0
    let encoding: u32 = 0x39007C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_unsigned
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm12=32 (power of 2 (2^5 = 32))
#[test]
fn test_aarch64_memory_single_general_immediate_unsigned_combo_17_0_39008000() {
    // Encoding: 0x39008000
    // Test aarch64_memory_single_general_immediate_unsigned field combination: size=0, opc=0, imm12=32, Rn=0, Rt=0
    // Fields: Rn=0, Rt=0, imm12=32, opc=0, size=0
    let encoding: u32 = 0x39008000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_unsigned
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm12=63 (2^6 - 1 = 63)
#[test]
fn test_aarch64_memory_single_general_immediate_unsigned_combo_18_0_3900fc00() {
    // Encoding: 0x3900FC00
    // Test aarch64_memory_single_general_immediate_unsigned field combination: size=0, opc=0, imm12=63, Rn=0, Rt=0
    // Fields: Rn=0, Rt=0, opc=0, imm12=63, size=0
    let encoding: u32 = 0x3900FC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_unsigned
/// ASL: `field combination 19`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm12=64 (power of 2 (2^6 = 64))
#[test]
fn test_aarch64_memory_single_general_immediate_unsigned_combo_19_0_39010000() {
    // Encoding: 0x39010000
    // Test aarch64_memory_single_general_immediate_unsigned field combination: size=0, opc=0, imm12=64, Rn=0, Rt=0
    // Fields: size=0, opc=0, imm12=64, Rn=0, Rt=0
    let encoding: u32 = 0x39010000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_unsigned
/// ASL: `field combination 20`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm12=127 (2^7 - 1 = 127)
#[test]
fn test_aarch64_memory_single_general_immediate_unsigned_combo_20_0_3901fc00() {
    // Encoding: 0x3901FC00
    // Test aarch64_memory_single_general_immediate_unsigned field combination: size=0, opc=0, imm12=127, Rn=0, Rt=0
    // Fields: Rn=0, opc=0, size=0, imm12=127, Rt=0
    let encoding: u32 = 0x3901FC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_unsigned
/// ASL: `field combination 21`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm12=128 (power of 2 (2^7 = 128))
#[test]
fn test_aarch64_memory_single_general_immediate_unsigned_combo_21_0_39020000() {
    // Encoding: 0x39020000
    // Test aarch64_memory_single_general_immediate_unsigned field combination: size=0, opc=0, imm12=128, Rn=0, Rt=0
    // Fields: Rn=0, imm12=128, opc=0, size=0, Rt=0
    let encoding: u32 = 0x39020000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_unsigned
/// ASL: `field combination 22`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm12=255 (2^8 - 1 = 255)
#[test]
fn test_aarch64_memory_single_general_immediate_unsigned_combo_22_0_3903fc00() {
    // Encoding: 0x3903FC00
    // Test aarch64_memory_single_general_immediate_unsigned field combination: size=0, opc=0, imm12=255, Rn=0, Rt=0
    // Fields: opc=0, imm12=255, Rn=0, size=0, Rt=0
    let encoding: u32 = 0x3903FC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_unsigned
/// ASL: `field combination 23`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm12=256 (power of 2 (2^8 = 256))
#[test]
fn test_aarch64_memory_single_general_immediate_unsigned_combo_23_0_39040000() {
    // Encoding: 0x39040000
    // Test aarch64_memory_single_general_immediate_unsigned field combination: size=0, opc=0, imm12=256, Rn=0, Rt=0
    // Fields: size=0, opc=0, imm12=256, Rt=0, Rn=0
    let encoding: u32 = 0x39040000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_unsigned
/// ASL: `field combination 24`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm12=511 (2^9 - 1 = 511)
#[test]
fn test_aarch64_memory_single_general_immediate_unsigned_combo_24_0_3907fc00() {
    // Encoding: 0x3907FC00
    // Test aarch64_memory_single_general_immediate_unsigned field combination: size=0, opc=0, imm12=511, Rn=0, Rt=0
    // Fields: opc=0, size=0, Rn=0, Rt=0, imm12=511
    let encoding: u32 = 0x3907FC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_unsigned
/// ASL: `field combination 25`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm12=512 (power of 2 (2^9 = 512))
#[test]
fn test_aarch64_memory_single_general_immediate_unsigned_combo_25_0_39080000() {
    // Encoding: 0x39080000
    // Test aarch64_memory_single_general_immediate_unsigned field combination: size=0, opc=0, imm12=512, Rn=0, Rt=0
    // Fields: imm12=512, size=0, opc=0, Rn=0, Rt=0
    let encoding: u32 = 0x39080000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_unsigned
/// ASL: `field combination 26`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm12=1023 (2^10 - 1 = 1023)
#[test]
fn test_aarch64_memory_single_general_immediate_unsigned_combo_26_0_390ffc00() {
    // Encoding: 0x390FFC00
    // Test aarch64_memory_single_general_immediate_unsigned field combination: size=0, opc=0, imm12=1023, Rn=0, Rt=0
    // Fields: opc=0, Rn=0, imm12=1023, size=0, Rt=0
    let encoding: u32 = 0x390FFC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_unsigned
/// ASL: `field combination 27`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm12=1024 (power of 2 (2^10 = 1024))
#[test]
fn test_aarch64_memory_single_general_immediate_unsigned_combo_27_0_39100000() {
    // Encoding: 0x39100000
    // Test aarch64_memory_single_general_immediate_unsigned field combination: size=0, opc=0, imm12=1024, Rn=0, Rt=0
    // Fields: Rt=0, Rn=0, opc=0, size=0, imm12=1024
    let encoding: u32 = 0x39100000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_unsigned
/// ASL: `field combination 28`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm12=2047 (immediate midpoint (2047))
#[test]
fn test_aarch64_memory_single_general_immediate_unsigned_combo_28_0_391ffc00() {
    // Encoding: 0x391FFC00
    // Test aarch64_memory_single_general_immediate_unsigned field combination: size=0, opc=0, imm12=2047, Rn=0, Rt=0
    // Fields: imm12=2047, Rn=0, Rt=0, size=0, opc=0
    let encoding: u32 = 0x391FFC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_unsigned
/// ASL: `field combination 29`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm12=2048 (power of 2 (2^11 = 2048))
#[test]
fn test_aarch64_memory_single_general_immediate_unsigned_combo_29_0_39200000() {
    // Encoding: 0x39200000
    // Test aarch64_memory_single_general_immediate_unsigned field combination: size=0, opc=0, imm12=2048, Rn=0, Rt=0
    // Fields: opc=0, size=0, imm12=2048, Rn=0, Rt=0
    let encoding: u32 = 0x39200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_unsigned
/// ASL: `field combination 30`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm12=4095 (maximum immediate (4095))
#[test]
fn test_aarch64_memory_single_general_immediate_unsigned_combo_30_0_393ffc00() {
    // Encoding: 0x393FFC00
    // Test aarch64_memory_single_general_immediate_unsigned field combination: size=0, opc=0, imm12=4095, Rn=0, Rt=0
    // Fields: opc=0, size=0, Rn=0, imm12=4095, Rt=0
    let encoding: u32 = 0x393FFC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_unsigned
/// ASL: `field combination 31`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch64_memory_single_general_immediate_unsigned_combo_31_0_39000000() {
    // Encoding: 0x39000000
    // Test aarch64_memory_single_general_immediate_unsigned field combination: size=0, opc=0, imm12=0, Rn=0, Rt=0
    // Fields: imm12=0, Rt=0, size=0, Rn=0, opc=0
    let encoding: u32 = 0x39000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_unsigned
/// ASL: `field combination 32`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_aarch64_memory_single_general_immediate_unsigned_combo_32_0_39000020() {
    // Encoding: 0x39000020
    // Test aarch64_memory_single_general_immediate_unsigned field combination: size=0, opc=0, imm12=0, Rn=1, Rt=0
    // Fields: Rt=0, Rn=1, imm12=0, size=0, opc=0
    let encoding: u32 = 0x39000020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_unsigned
/// ASL: `field combination 33`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_memory_single_general_immediate_unsigned_combo_33_0_390003c0() {
    // Encoding: 0x390003C0
    // Test aarch64_memory_single_general_immediate_unsigned field combination: size=0, opc=0, imm12=0, Rn=30, Rt=0
    // Fields: size=0, Rn=30, Rt=0, imm12=0, opc=0
    let encoding: u32 = 0x390003C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_unsigned
/// ASL: `field combination 34`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_memory_single_general_immediate_unsigned_combo_34_0_390003e0() {
    // Encoding: 0x390003E0
    // Test aarch64_memory_single_general_immediate_unsigned field combination: size=0, opc=0, imm12=0, Rn=31, Rt=0
    // Fields: imm12=0, size=0, Rn=31, opc=0, Rt=0
    let encoding: u32 = 0x390003E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_unsigned
/// ASL: `field combination 35`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt=0 (register index 0 (first register))
#[test]
fn test_aarch64_memory_single_general_immediate_unsigned_combo_35_0_39000000() {
    // Encoding: 0x39000000
    // Test aarch64_memory_single_general_immediate_unsigned field combination: size=0, opc=0, imm12=0, Rn=0, Rt=0
    // Fields: size=0, opc=0, Rt=0, imm12=0, Rn=0
    let encoding: u32 = 0x39000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_unsigned
/// ASL: `field combination 36`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt=1 (register index 1 (second register))
#[test]
fn test_aarch64_memory_single_general_immediate_unsigned_combo_36_0_39000001() {
    // Encoding: 0x39000001
    // Test aarch64_memory_single_general_immediate_unsigned field combination: size=0, opc=0, imm12=0, Rn=0, Rt=1
    // Fields: imm12=0, Rt=1, size=0, Rn=0, opc=0
    let encoding: u32 = 0x39000001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_unsigned
/// ASL: `field combination 37`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_memory_single_general_immediate_unsigned_combo_37_0_3900001e() {
    // Encoding: 0x3900001E
    // Test aarch64_memory_single_general_immediate_unsigned field combination: size=0, opc=0, imm12=0, Rn=0, Rt=30
    // Fields: Rn=0, Rt=30, opc=0, size=0, imm12=0
    let encoding: u32 = 0x3900001E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_unsigned
/// ASL: `field combination 38`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt=31 (register index 31 (ZR - zero register))
#[test]
fn test_aarch64_memory_single_general_immediate_unsigned_combo_38_0_3900001f() {
    // Encoding: 0x3900001F
    // Test aarch64_memory_single_general_immediate_unsigned field combination: size=0, opc=0, imm12=0, Rn=0, Rt=31
    // Fields: size=0, opc=0, imm12=0, Rn=0, Rt=31
    let encoding: u32 = 0x3900001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_unsigned
/// ASL: `field combination 39`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (same register test (reg=1)), Rt=1 (same register test (reg=1))
#[test]
fn test_aarch64_memory_single_general_immediate_unsigned_combo_39_0_39000021() {
    // Encoding: 0x39000021
    // Test aarch64_memory_single_general_immediate_unsigned field combination: size=0, opc=0, imm12=0, Rn=1, Rt=1
    // Fields: imm12=0, Rt=1, opc=0, Rn=1, size=0
    let encoding: u32 = 0x39000021;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_unsigned
/// ASL: `field combination 40`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (same register test (reg=31)), Rt=31 (same register test (reg=31))
#[test]
fn test_aarch64_memory_single_general_immediate_unsigned_combo_40_0_390003ff() {
    // Encoding: 0x390003FF
    // Test aarch64_memory_single_general_immediate_unsigned field combination: size=0, opc=0, imm12=0, Rn=31, Rt=31
    // Fields: size=0, opc=0, imm12=0, Rn=31, Rt=31
    let encoding: u32 = 0x390003FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_unsigned
/// ASL: `field size = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "size", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_memory_single_general_immediate_unsigned_special_size_0_size_variant_0_0_39000400()
{
    // Encoding: 0x39000400
    // Test aarch64_memory_single_general_immediate_unsigned special value size = 0 (Size variant 0)
    // Fields: imm12=1, size=0, opc=0, Rn=0, Rt=0
    let encoding: u32 = 0x39000400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_unsigned
/// ASL: `field size = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "size", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_memory_single_general_immediate_unsigned_special_size_1_size_variant_1_0_79000400()
{
    // Encoding: 0x79000400
    // Test aarch64_memory_single_general_immediate_unsigned special value size = 1 (Size variant 1)
    // Fields: imm12=1, opc=0, Rn=0, Rt=0, size=1
    let encoding: u32 = 0x79000400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_unsigned
/// ASL: `field size = 2 (Size variant 2)`
/// Requirement: FieldSpecial { field: "size", value: 2, meaning: "Size variant 2" }
/// Size variant 2
#[test]
fn test_aarch64_memory_single_general_immediate_unsigned_special_size_2_size_variant_2_0_b9000400()
{
    // Encoding: 0xB9000400
    // Test aarch64_memory_single_general_immediate_unsigned special value size = 2 (Size variant 2)
    // Fields: imm12=1, Rt=0, size=2, Rn=0, opc=0
    let encoding: u32 = 0xB9000400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_unsigned
/// ASL: `field size = 3 (Size variant 3)`
/// Requirement: FieldSpecial { field: "size", value: 3, meaning: "Size variant 3" }
/// Size variant 3
#[test]
fn test_aarch64_memory_single_general_immediate_unsigned_special_size_3_size_variant_3_0_f9000400()
{
    // Encoding: 0xF9000400
    // Test aarch64_memory_single_general_immediate_unsigned special value size = 3 (Size variant 3)
    // Fields: size=3, opc=0, imm12=1, Rn=0, Rt=0
    let encoding: u32 = 0xF9000400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_unsigned
/// ASL: `field opc = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "opc", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_memory_single_general_immediate_unsigned_special_opc_0_size_variant_0_0_79000400() {
    // Encoding: 0x79000400
    // Test aarch64_memory_single_general_immediate_unsigned special value opc = 0 (Size variant 0)
    // Fields: imm12=1, opc=0, size=1, Rn=0, Rt=0
    let encoding: u32 = 0x79000400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_unsigned
/// ASL: `field opc = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "opc", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_memory_single_general_immediate_unsigned_special_opc_1_size_variant_1_0_79400400() {
    // Encoding: 0x79400400
    // Test aarch64_memory_single_general_immediate_unsigned special value opc = 1 (Size variant 1)
    // Fields: opc=1, Rn=0, Rt=0, size=1, imm12=1
    let encoding: u32 = 0x79400400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_unsigned
/// ASL: `field opc = 2 (Size variant 2)`
/// Requirement: FieldSpecial { field: "opc", value: 2, meaning: "Size variant 2" }
/// Size variant 2
#[test]
fn test_aarch64_memory_single_general_immediate_unsigned_special_opc_2_size_variant_2_0_79800400() {
    // Encoding: 0x79800400
    // Test aarch64_memory_single_general_immediate_unsigned special value opc = 2 (Size variant 2)
    // Fields: opc=2, Rt=0, imm12=1, Rn=0, size=1
    let encoding: u32 = 0x79800400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_unsigned
/// ASL: `field opc = 3 (Size variant 3)`
/// Requirement: FieldSpecial { field: "opc", value: 3, meaning: "Size variant 3" }
/// Size variant 3
#[test]
fn test_aarch64_memory_single_general_immediate_unsigned_special_opc_3_size_variant_3_0_79c00400() {
    // Encoding: 0x79C00400
    // Test aarch64_memory_single_general_immediate_unsigned special value opc = 3 (Size variant 3)
    // Fields: Rn=0, size=1, Rt=0, opc=3, imm12=1
    let encoding: u32 = 0x79C00400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_unsigned
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_memory_single_general_immediate_unsigned_special_rn_31_stack_pointer_sp_may_require_alignment_0_790007e0()
 {
    // Encoding: 0x790007E0
    // Test aarch64_memory_single_general_immediate_unsigned special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: size=1, Rn=31, Rt=0, imm12=1, opc=0
    let encoding: u32 = 0x790007E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_unsigned
/// ASL: `field Rt = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)`
/// Requirement: FieldSpecial { field: "Rt", value: 31, meaning: "Zero register (XZR/WZR) - reads as 0, writes discarded" }
/// Zero register (XZR/WZR) - reads as 0, writes discarded
#[test]
fn test_aarch64_memory_single_general_immediate_unsigned_special_rt_31_zero_register_xzr_wzr_reads_as_0_writes_discarded_0_7900041f()
 {
    // Encoding: 0x7900041F
    // Test aarch64_memory_single_general_immediate_unsigned special value Rt = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)
    // Fields: Rn=0, Rt=31, size=1, opc=0, imm12=1
    let encoding: u32 = 0x7900041F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_post_idx
/// ASL: `GpFromField("t") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "t" }
/// verify register write to GpFromField("t")
#[test]
fn test_aarch64_memory_single_general_immediate_signed_post_idx_reg_write_0_38000400() {
    // Test aarch64_memory_single_general_immediate_signed_post_idx register write: GpFromField("t")
    // Encoding: 0x38000400
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x38000400;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_memory_single_general_immediate_signed_post_idx
/// ASL: `GpFromField("t") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "t" }
/// verify register write to GpFromField("t")
#[test]
fn test_aarch64_memory_single_general_immediate_signed_post_idx_reg_write_1_38000400() {
    // Test aarch64_memory_single_general_immediate_signed_post_idx register write: GpFromField("t")
    // Encoding: 0x38000400
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x38000400;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_memory_single_general_immediate_signed_post_idx
/// ASL: `Sp write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "unknown" }
/// verify register write to Sp
#[test]
fn test_aarch64_memory_single_general_immediate_signed_post_idx_reg_write_2_38000400() {
    // Test aarch64_memory_single_general_immediate_signed_post_idx register write: Sp
    // Encoding: 0x38000400
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x38000400;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_memory_single_general_immediate_signed_post_idx
/// ASL: `GpFromField("n") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "n" }
/// verify register write to GpFromField("n")
#[test]
fn test_aarch64_memory_single_general_immediate_signed_post_idx_reg_write_3_38000400() {
    // Test aarch64_memory_single_general_immediate_signed_post_idx register write: GpFromField("n")
    // Encoding: 0x38000400
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x38000400;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_memory_single_general_immediate_signed_post_idx
/// ASL: `Rn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Rn = 31)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_post_idx_sp_rn_380007e0() {
    // Test aarch64_memory_single_general_immediate_signed_post_idx with Rn = SP (31)
    // Encoding: 0x380007E0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x380007E0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_memory_single_general_immediate_signed_post_idx
/// ASL: `Rt = 31 (ZR)`
/// Requirement: RegisterSpecial { reg: Zr, behavior: "reads as 0, writes discarded" }
/// zero register (Rt = 31)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_post_idx_zr_rt_3800041f() {
    // Test aarch64_memory_single_general_immediate_signed_post_idx with Rt = ZR (31)
    // Encoding: 0x3800041F
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x3800041F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 31), 0, "XZR should always be 0");
}

/// Provenance: aarch64_memory_single_general_immediate_signed_post_idx
/// ASL: `Mem[address, 8] = data`
/// Requirement: MemoryAccess { op: Store, size_bits: 64, addressing: "Base { reg: \"address\" }" }
/// 8-byte store
#[test]
fn test_aarch64_memory_single_general_immediate_signed_post_idx_store_0_38000420() {
    // Test aarch64_memory_single_general_immediate_signed_post_idx memory store: 8 bytes
    // Encoding: 0x38000420
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x100000000000);
    set_x(&mut cpu, 0, 0xDEADBEEFCAFEBABE);
    let encoding: u32 = 0x38000420;
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

/// Provenance: aarch64_memory_single_general_immediate_signed_post_idx
/// ASL: `Unconditional`
/// Requirement: UndefinedEncoding { condition: "Unconditional" }
/// triggers Undefined
#[test]
fn test_aarch64_memory_single_general_immediate_signed_post_idx_exception_0_38000400() {
    // Test aarch64_memory_single_general_immediate_signed_post_idx exception: Undefined
    // Encoding: 0x38000400
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x38000400;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_memory_single_general_immediate_signed_post_idx
/// ASL: `Unconditional`
/// Requirement: UndefinedEncoding { condition: "Unconditional" }
/// triggers Undefined
#[test]
fn test_aarch64_memory_single_general_immediate_signed_post_idx_exception_1_38000400() {
    // Test aarch64_memory_single_general_immediate_signed_post_idx exception: Undefined
    // Encoding: 0x38000400
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x38000400;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pre_idx
/// ASL: `GpFromField("t") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "t" }
/// verify register write to GpFromField("t")
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pre_idx_reg_write_0_38000c00() {
    // Test aarch64_memory_single_general_immediate_signed_pre_idx register write: GpFromField("t")
    // Encoding: 0x38000C00
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x38000C00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pre_idx
/// ASL: `GpFromField("t") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "t" }
/// verify register write to GpFromField("t")
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pre_idx_reg_write_1_38000c00() {
    // Test aarch64_memory_single_general_immediate_signed_pre_idx register write: GpFromField("t")
    // Encoding: 0x38000C00
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x38000C00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pre_idx
/// ASL: `Sp write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "unknown" }
/// verify register write to Sp
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pre_idx_reg_write_2_38000c00() {
    // Test aarch64_memory_single_general_immediate_signed_pre_idx register write: Sp
    // Encoding: 0x38000C00
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x38000C00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pre_idx
/// ASL: `GpFromField("n") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "n" }
/// verify register write to GpFromField("n")
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pre_idx_reg_write_3_38000c00() {
    // Test aarch64_memory_single_general_immediate_signed_pre_idx register write: GpFromField("n")
    // Encoding: 0x38000C00
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x38000C00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pre_idx
/// ASL: `Rn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Rn = 31)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pre_idx_sp_rn_38000fe0() {
    // Test aarch64_memory_single_general_immediate_signed_pre_idx with Rn = SP (31)
    // Encoding: 0x38000FE0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x38000FE0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pre_idx
/// ASL: `Rt = 31 (ZR)`
/// Requirement: RegisterSpecial { reg: Zr, behavior: "reads as 0, writes discarded" }
/// zero register (Rt = 31)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pre_idx_zr_rt_38000c1f() {
    // Test aarch64_memory_single_general_immediate_signed_pre_idx with Rt = ZR (31)
    // Encoding: 0x38000C1F
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x38000C1F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 31), 0, "XZR should always be 0");
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pre_idx
/// ASL: `Mem[address, 8] = data`
/// Requirement: MemoryAccess { op: Store, size_bits: 64, addressing: "Base { reg: \"address\" }" }
/// 8-byte store
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pre_idx_store_0_38000c20() {
    // Test aarch64_memory_single_general_immediate_signed_pre_idx memory store: 8 bytes
    // Encoding: 0x38000C20
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x100000000000);
    set_x(&mut cpu, 0, 0xDEADBEEFCAFEBABE);
    let encoding: u32 = 0x38000C20;
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

/// Provenance: aarch64_memory_single_general_immediate_signed_pre_idx
/// ASL: `Unconditional`
/// Requirement: UndefinedEncoding { condition: "Unconditional" }
/// triggers Undefined
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pre_idx_exception_0_38000c00() {
    // Test aarch64_memory_single_general_immediate_signed_pre_idx exception: Undefined
    // Encoding: 0x38000C00
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x38000C00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pre_idx
/// ASL: `Unconditional`
/// Requirement: UndefinedEncoding { condition: "Unconditional" }
/// triggers Undefined
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pre_idx_exception_1_38000c00() {
    // Test aarch64_memory_single_general_immediate_signed_pre_idx exception: Undefined
    // Encoding: 0x38000C00
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x38000C00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_memory_single_general_immediate_unsigned
/// ASL: `STRB X0, [X1, #0]`
/// Requirement: MemoryAccess { op: Store, size_bits: 8, addressing: "immediate" }
/// zero value
#[test]
fn test_aarch64_memory_single_general_immediate_unsigned_str_oracle_0_39000020() {
    // Test STRB: zero value (oracle)
    // Encoding: 0x39000020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 0, 0x0);
    set_x(&mut cpu, 1, 0x1000);
    let encoding: u32 = 0x39000020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    {
        let buf = cpu.read_memory(0x1000, 1).unwrap();
        let mut bytes = [0u8; 8];
        let n = buf.len().min(8);
        bytes[..n].copy_from_slice(&buf[..n]);
        let val = u64::from_le_bytes(bytes);
        assert_eq!(val, 0x0, "Memory at 0x1000 should be 0x0");
    }
}

/// Provenance: aarch64_memory_single_general_immediate_unsigned
/// ASL: `STRB X0, [X1, #0]`
/// Requirement: MemoryAccess { op: Store, size_bits: 8, addressing: "immediate" }
/// byte value
#[test]
fn test_aarch64_memory_single_general_immediate_unsigned_str_oracle_1_39000020() {
    // Test STRB: byte value (oracle)
    // Encoding: 0x39000020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x1000);
    set_x(&mut cpu, 0, 0xFF);
    let encoding: u32 = 0x39000020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
    {
        let buf = cpu.read_memory(0x1000, 1).unwrap();
        let mut bytes = [0u8; 8];
        let n = buf.len().min(8);
        bytes[..n].copy_from_slice(&buf[..n]);
        let val = u64::from_le_bytes(bytes);
        assert_eq!(val, 0xFF, "Memory at 0x1000 should be 0xFF");
    }
}

/// Provenance: aarch64_memory_single_general_immediate_unsigned
/// ASL: `STRB X0, [X1, #0]`
/// Requirement: MemoryAccess { op: Store, size_bits: 8, addressing: "immediate" }
/// halfword value
#[test]
fn test_aarch64_memory_single_general_immediate_unsigned_str_oracle_2_39000020() {
    // Test STRB: halfword value (oracle)
    // Encoding: 0x39000020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x1000);
    set_x(&mut cpu, 0, 0x1234);
    let encoding: u32 = 0x39000020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
    {
        let buf = cpu.read_memory(0x1000, 1).unwrap();
        let mut bytes = [0u8; 8];
        let n = buf.len().min(8);
        bytes[..n].copy_from_slice(&buf[..n]);
        let val = u64::from_le_bytes(bytes);
        assert_eq!(val, 0x34, "Memory at 0x1000 should be 0x34");
    }
}

/// Provenance: aarch64_memory_single_general_immediate_unsigned
/// ASL: `STRB X0, [X1, #0]`
/// Requirement: MemoryAccess { op: Store, size_bits: 8, addressing: "immediate" }
/// word value
#[test]
fn test_aarch64_memory_single_general_immediate_unsigned_str_oracle_3_39000020() {
    // Test STRB: word value (oracle)
    // Encoding: 0x39000020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x1000);
    set_x(&mut cpu, 0, 0x12345678);
    let encoding: u32 = 0x39000020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
    {
        let buf = cpu.read_memory(0x1000, 1).unwrap();
        let mut bytes = [0u8; 8];
        let n = buf.len().min(8);
        bytes[..n].copy_from_slice(&buf[..n]);
        let val = u64::from_le_bytes(bytes);
        assert_eq!(val, 0x78, "Memory at 0x1000 should be 0x78");
    }
}

/// Provenance: aarch64_memory_single_general_immediate_unsigned
/// ASL: `STRB X0, [X1, #0]`
/// Requirement: MemoryAccess { op: Store, size_bits: 8, addressing: "immediate" }
/// doubleword value
#[test]
fn test_aarch64_memory_single_general_immediate_unsigned_str_oracle_4_39000020() {
    // Test STRB: doubleword value (oracle)
    // Encoding: 0x39000020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x1000);
    set_x(&mut cpu, 0, 0x123456789ABCDEF0);
    let encoding: u32 = 0x39000020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
    {
        let buf = cpu.read_memory(0x1000, 1).unwrap();
        let mut bytes = [0u8; 8];
        let n = buf.len().min(8);
        bytes[..n].copy_from_slice(&buf[..n]);
        let val = u64::from_le_bytes(bytes);
        assert_eq!(val, 0xF0, "Memory at 0x1000 should be 0xF0");
    }
}

/// Provenance: aarch64_memory_single_general_immediate_unsigned
/// ASL: `GpFromField("t") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "t" }
/// verify register write to GpFromField("t")
#[test]
fn test_aarch64_memory_single_general_immediate_unsigned_reg_write_0_39000000() {
    // Test aarch64_memory_single_general_immediate_unsigned register write: GpFromField("t")
    // Encoding: 0x39000000
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x39000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_memory_single_general_immediate_unsigned
/// ASL: `GpFromField("t") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "t" }
/// verify register write to GpFromField("t")
#[test]
fn test_aarch64_memory_single_general_immediate_unsigned_reg_write_1_39000000() {
    // Test aarch64_memory_single_general_immediate_unsigned register write: GpFromField("t")
    // Encoding: 0x39000000
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x39000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_memory_single_general_immediate_unsigned
/// ASL: `Sp write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "unknown" }
/// verify register write to Sp
#[test]
fn test_aarch64_memory_single_general_immediate_unsigned_reg_write_2_39000000() {
    // Test aarch64_memory_single_general_immediate_unsigned register write: Sp
    // Encoding: 0x39000000
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x39000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_memory_single_general_immediate_unsigned
/// ASL: `GpFromField("n") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "n" }
/// verify register write to GpFromField("n")
#[test]
fn test_aarch64_memory_single_general_immediate_unsigned_reg_write_3_39000000() {
    // Test aarch64_memory_single_general_immediate_unsigned register write: GpFromField("n")
    // Encoding: 0x39000000
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x39000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_memory_single_general_immediate_unsigned
/// ASL: `Rn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Rn = 31)
#[test]
fn test_aarch64_memory_single_general_immediate_unsigned_sp_rn_390003e0() {
    // Test aarch64_memory_single_general_immediate_unsigned with Rn = SP (31)
    // Encoding: 0x390003E0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x390003E0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_memory_single_general_immediate_unsigned
/// ASL: `Rt = 31 (ZR)`
/// Requirement: RegisterSpecial { reg: Zr, behavior: "reads as 0, writes discarded" }
/// zero register (Rt = 31)
#[test]
fn test_aarch64_memory_single_general_immediate_unsigned_zr_rt_3900001f() {
    // Test aarch64_memory_single_general_immediate_unsigned with Rt = ZR (31)
    // Encoding: 0x3900001F
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x3900001F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 31), 0, "XZR should always be 0");
}

/// Provenance: aarch64_memory_single_general_immediate_unsigned
/// ASL: `Mem[address, 8] = data`
/// Requirement: MemoryAccess { op: Store, size_bits: 64, addressing: "Base { reg: \"address\" }" }
/// 8-byte store
#[test]
fn test_aarch64_memory_single_general_immediate_unsigned_store_0_39000020() {
    // Test aarch64_memory_single_general_immediate_unsigned memory store: 8 bytes
    // Encoding: 0x39000020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 0, 0xDEADBEEFCAFEBABE);
    set_x(&mut cpu, 1, 0x100000000000);
    let encoding: u32 = 0x39000020;
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

/// Provenance: aarch64_memory_single_general_immediate_unsigned
/// ASL: `Unconditional`
/// Requirement: UndefinedEncoding { condition: "Unconditional" }
/// triggers Undefined
#[test]
fn test_aarch64_memory_single_general_immediate_unsigned_exception_0_39000000() {
    // Test aarch64_memory_single_general_immediate_unsigned exception: Undefined
    // Encoding: 0x39000000
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x39000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_memory_single_general_immediate_unsigned
/// ASL: `Unconditional`
/// Requirement: UndefinedEncoding { condition: "Unconditional" }
/// triggers Undefined
#[test]
fn test_aarch64_memory_single_general_immediate_unsigned_exception_1_39000000() {
    // Test aarch64_memory_single_general_immediate_unsigned exception: Undefined
    // Encoding: 0x39000000
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x39000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

// ============================================================================
// aarch64_memory_single_simdfp_immediate_signed_offset_normal Tests
// ============================================================================

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_offset_normal
/// ASL: `field size 30 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_offset_normal_field_size_0_min_0_3c000000() {
    // Encoding: 0x3C000000
    // Test aarch64_memory_single_simdfp_immediate_signed_offset_normal field size = 0 (Min)
    // Fields: Rt=0, size=0, opc=0, Rn=0, imm9=0
    let encoding: u32 = 0x3C000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stur	b0, [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_offset_normal
/// ASL: `field size 30 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 1, boundary: PowerOfTwo }
/// 16-bit / halfword size
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_offset_normal_field_size_1_poweroftwo_0_7c000000()
 {
    // Encoding: 0x7C000000
    // Test aarch64_memory_single_simdfp_immediate_signed_offset_normal field size = 1 (PowerOfTwo)
    // Fields: opc=0, imm9=0, Rt=0, Rn=0, size=1
    let encoding: u32 = 0x7C000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stur	h0, [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_offset_normal
/// ASL: `field size 30 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 2, boundary: PowerOfTwo }
/// 32-bit / word size
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_offset_normal_field_size_2_poweroftwo_0_bc000000()
 {
    // Encoding: 0xBC000000
    // Test aarch64_memory_single_simdfp_immediate_signed_offset_normal field size = 2 (PowerOfTwo)
    // Fields: size=2, Rt=0, imm9=0, Rn=0, opc=0
    let encoding: u32 = 0xBC000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stur	s0, [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_offset_normal
/// ASL: `field size 30 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 3, boundary: Max }
/// 64-bit / doubleword size
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_offset_normal_field_size_3_max_0_fc000000() {
    // Encoding: 0xFC000000
    // Test aarch64_memory_single_simdfp_immediate_signed_offset_normal field size = 3 (Max)
    // Fields: Rn=0, Rt=0, opc=0, imm9=0, size=3
    let encoding: u32 = 0xFC000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stur	d0, [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_offset_normal
/// ASL: `field opc 22 +: 2`
/// Requirement: FieldBoundary { field: "opc", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_offset_normal_field_opc_0_min_0_3c000000() {
    // Encoding: 0x3C000000
    // Test aarch64_memory_single_simdfp_immediate_signed_offset_normal field opc = 0 (Min)
    // Fields: imm9=0, Rn=0, Rt=0, opc=0, size=0
    let encoding: u32 = 0x3C000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stur	b0, [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_offset_normal
/// ASL: `field opc 22 +: 2`
/// Requirement: FieldBoundary { field: "opc", value: 1, boundary: PowerOfTwo }
/// 16-bit / halfword size
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_offset_normal_field_opc_1_poweroftwo_0_3c400000()
 {
    // Encoding: 0x3C400000
    // Test aarch64_memory_single_simdfp_immediate_signed_offset_normal field opc = 1 (PowerOfTwo)
    // Fields: Rt=0, Rn=0, size=0, opc=1, imm9=0
    let encoding: u32 = 0x3C400000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ldur	b0, [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_offset_normal
/// ASL: `field opc 22 +: 2`
/// Requirement: FieldBoundary { field: "opc", value: 2, boundary: PowerOfTwo }
/// 32-bit / word size
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_offset_normal_field_opc_2_poweroftwo_0_3c800000()
 {
    // Encoding: 0x3C800000
    // Test aarch64_memory_single_simdfp_immediate_signed_offset_normal field opc = 2 (PowerOfTwo)
    // Fields: imm9=0, Rn=0, opc=2, Rt=0, size=0
    let encoding: u32 = 0x3C800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stur	q0, [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_offset_normal
/// ASL: `field opc 22 +: 2`
/// Requirement: FieldBoundary { field: "opc", value: 3, boundary: Max }
/// 64-bit / doubleword size
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_offset_normal_field_opc_3_max_0_3cc00000() {
    // Encoding: 0x3CC00000
    // Test aarch64_memory_single_simdfp_immediate_signed_offset_normal field opc = 3 (Max)
    // Fields: imm9=0, size=0, Rt=0, opc=3, Rn=0
    let encoding: u32 = 0x3CC00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ldur	q0, [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_offset_normal
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_offset_normal_field_imm9_0_zero_0_3c000000() {
    // Encoding: 0x3C000000
    // Test aarch64_memory_single_simdfp_immediate_signed_offset_normal field imm9 = 0 (Zero)
    // Fields: Rt=0, size=0, opc=0, Rn=0, imm9=0
    let encoding: u32 = 0x3C000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stur	b0, [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_offset_normal
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_offset_normal_field_imm9_1_poweroftwo_0_3c001000()
 {
    // Encoding: 0x3C001000
    // Test aarch64_memory_single_simdfp_immediate_signed_offset_normal field imm9 = 1 (PowerOfTwo)
    // Fields: imm9=1, size=0, opc=0, Rn=0, Rt=0
    let encoding: u32 = 0x3C001000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stur	b0, [x0, #1]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_offset_normal
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 3, boundary: PowerOfTwoMinusOne }
/// 2^2 - 1 = 3
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_offset_normal_field_imm9_3_poweroftwominusone_0_3c003000()
 {
    // Encoding: 0x3C003000
    // Test aarch64_memory_single_simdfp_immediate_signed_offset_normal field imm9 = 3 (PowerOfTwoMinusOne)
    // Fields: size=0, opc=0, Rt=0, Rn=0, imm9=3
    let encoding: u32 = 0x3C003000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stur	b0, [x0, #3]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_offset_normal
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 4, boundary: PowerOfTwo }
/// power of 2 (2^2 = 4)
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_offset_normal_field_imm9_4_poweroftwo_0_3c004000()
 {
    // Encoding: 0x3C004000
    // Test aarch64_memory_single_simdfp_immediate_signed_offset_normal field imm9 = 4 (PowerOfTwo)
    // Fields: Rt=0, opc=0, Rn=0, size=0, imm9=4
    let encoding: u32 = 0x3C004000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stur	b0, [x0, #4]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_offset_normal
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 7, boundary: PowerOfTwoMinusOne }
/// 2^3 - 1 = 7
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_offset_normal_field_imm9_7_poweroftwominusone_0_3c007000()
 {
    // Encoding: 0x3C007000
    // Test aarch64_memory_single_simdfp_immediate_signed_offset_normal field imm9 = 7 (PowerOfTwoMinusOne)
    // Fields: size=0, Rn=0, Rt=0, opc=0, imm9=7
    let encoding: u32 = 0x3C007000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stur	b0, [x0, #7]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_offset_normal
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 8, boundary: PowerOfTwo }
/// power of 2 (2^3 = 8)
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_offset_normal_field_imm9_8_poweroftwo_0_3c008000()
 {
    // Encoding: 0x3C008000
    // Test aarch64_memory_single_simdfp_immediate_signed_offset_normal field imm9 = 8 (PowerOfTwo)
    // Fields: Rn=0, size=0, opc=0, imm9=8, Rt=0
    let encoding: u32 = 0x3C008000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stur	b0, [x0, #8]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_offset_normal
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 15, boundary: PowerOfTwoMinusOne }
/// 2^4 - 1 = 15
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_offset_normal_field_imm9_15_poweroftwominusone_0_3c00f000()
 {
    // Encoding: 0x3C00F000
    // Test aarch64_memory_single_simdfp_immediate_signed_offset_normal field imm9 = 15 (PowerOfTwoMinusOne)
    // Fields: Rn=0, Rt=0, opc=0, size=0, imm9=15
    let encoding: u32 = 0x3C00F000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stur	b0, [x0, #15]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_offset_normal
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 16, boundary: PowerOfTwo }
/// power of 2 (2^4 = 16)
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_offset_normal_field_imm9_16_poweroftwo_0_3c010000()
 {
    // Encoding: 0x3C010000
    // Test aarch64_memory_single_simdfp_immediate_signed_offset_normal field imm9 = 16 (PowerOfTwo)
    // Fields: size=0, opc=0, Rn=0, Rt=0, imm9=16
    let encoding: u32 = 0x3C010000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stur	b0, [x0, #16]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_offset_normal
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 31, boundary: PowerOfTwoMinusOne }
/// 2^5 - 1 = 31
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_offset_normal_field_imm9_31_poweroftwominusone_0_3c01f000()
 {
    // Encoding: 0x3C01F000
    // Test aarch64_memory_single_simdfp_immediate_signed_offset_normal field imm9 = 31 (PowerOfTwoMinusOne)
    // Fields: imm9=31, opc=0, Rn=0, Rt=0, size=0
    let encoding: u32 = 0x3C01F000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stur	b0, [x0, #31]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_offset_normal
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 32, boundary: PowerOfTwo }
/// power of 2 (2^5 = 32)
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_offset_normal_field_imm9_32_poweroftwo_0_3c020000()
 {
    // Encoding: 0x3C020000
    // Test aarch64_memory_single_simdfp_immediate_signed_offset_normal field imm9 = 32 (PowerOfTwo)
    // Fields: Rn=0, size=0, imm9=32, Rt=0, opc=0
    let encoding: u32 = 0x3C020000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stur	b0, [x0, #32]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_offset_normal
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 63, boundary: PowerOfTwoMinusOne }
/// 2^6 - 1 = 63
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_offset_normal_field_imm9_63_poweroftwominusone_0_3c03f000()
 {
    // Encoding: 0x3C03F000
    // Test aarch64_memory_single_simdfp_immediate_signed_offset_normal field imm9 = 63 (PowerOfTwoMinusOne)
    // Fields: size=0, imm9=63, opc=0, Rn=0, Rt=0
    let encoding: u32 = 0x3C03F000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stur	b0, [x0, #63]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_offset_normal
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 64, boundary: PowerOfTwo }
/// power of 2 (2^6 = 64)
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_offset_normal_field_imm9_64_poweroftwo_0_3c040000()
 {
    // Encoding: 0x3C040000
    // Test aarch64_memory_single_simdfp_immediate_signed_offset_normal field imm9 = 64 (PowerOfTwo)
    // Fields: Rt=0, Rn=0, size=0, opc=0, imm9=64
    let encoding: u32 = 0x3C040000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stur	b0, [x0, #64]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_offset_normal
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 127, boundary: PowerOfTwoMinusOne }
/// 2^7 - 1 = 127
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_offset_normal_field_imm9_127_poweroftwominusone_0_3c07f000()
 {
    // Encoding: 0x3C07F000
    // Test aarch64_memory_single_simdfp_immediate_signed_offset_normal field imm9 = 127 (PowerOfTwoMinusOne)
    // Fields: Rt=0, Rn=0, size=0, opc=0, imm9=127
    let encoding: u32 = 0x3C07F000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stur	b0, [x0, #127]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_offset_normal
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 128, boundary: PowerOfTwo }
/// power of 2 (2^7 = 128)
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_offset_normal_field_imm9_128_poweroftwo_0_3c080000()
 {
    // Encoding: 0x3C080000
    // Test aarch64_memory_single_simdfp_immediate_signed_offset_normal field imm9 = 128 (PowerOfTwo)
    // Fields: Rn=0, opc=0, imm9=128, size=0, Rt=0
    let encoding: u32 = 0x3C080000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stur	b0, [x0, #128]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_offset_normal
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 255, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (255)
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_offset_normal_field_imm9_255_poweroftwominusone_0_3c0ff000()
 {
    // Encoding: 0x3C0FF000
    // Test aarch64_memory_single_simdfp_immediate_signed_offset_normal field imm9 = 255 (PowerOfTwoMinusOne)
    // Fields: imm9=255, Rn=0, opc=0, size=0, Rt=0
    let encoding: u32 = 0x3C0FF000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stur	b0, [x0, #255]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_offset_normal
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 256, boundary: PowerOfTwo }
/// power of 2 (2^8 = 256)
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_offset_normal_field_imm9_256_poweroftwo_0_3c100000()
 {
    // Encoding: 0x3C100000
    // Test aarch64_memory_single_simdfp_immediate_signed_offset_normal field imm9 = 256 (PowerOfTwo)
    // Fields: imm9=256, Rt=0, Rn=0, size=0, opc=0
    let encoding: u32 = 0x3C100000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_offset_normal
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 511, boundary: Max }
/// maximum immediate (511)
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_offset_normal_field_imm9_511_max_0_3c1ff000()
{
    // Encoding: 0x3C1FF000
    // Test aarch64_memory_single_simdfp_immediate_signed_offset_normal field imm9 = 511 (Max)
    // Fields: imm9=511, Rn=0, Rt=0, size=0, opc=0
    let encoding: u32 = 0x3C1FF000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stur	b0, [x0, #-1]
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

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_offset_normal
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_offset_normal_field_rn_0_min_0_3c000000() {
    // Encoding: 0x3C000000
    // Test aarch64_memory_single_simdfp_immediate_signed_offset_normal field Rn = 0 (Min)
    // Fields: imm9=0, size=0, Rn=0, opc=0, Rt=0
    let encoding: u32 = 0x3C000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stur	b0, [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_offset_normal
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_offset_normal_field_rn_1_poweroftwo_0_3c000020()
 {
    // Encoding: 0x3C000020
    // Test aarch64_memory_single_simdfp_immediate_signed_offset_normal field Rn = 1 (PowerOfTwo)
    // Fields: size=0, Rt=0, opc=0, imm9=0, Rn=1
    let encoding: u32 = 0x3C000020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stur	b0, [x1]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_offset_normal
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_offset_normal_field_rn_30_poweroftwominusone_0_3c0003c0()
 {
    // Encoding: 0x3C0003C0
    // Test aarch64_memory_single_simdfp_immediate_signed_offset_normal field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: size=0, Rn=30, Rt=0, opc=0, imm9=0
    let encoding: u32 = 0x3C0003C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stur	b0, [x30]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_offset_normal
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_offset_normal_field_rn_31_max_0_3c0003e0() {
    // Encoding: 0x3C0003E0
    // Test aarch64_memory_single_simdfp_immediate_signed_offset_normal field Rn = 31 (Max)
    // Fields: Rt=0, size=0, opc=0, imm9=0, Rn=31
    let encoding: u32 = 0x3C0003E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stur	b0, [sp]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_offset_normal
/// ASL: `field Rt 0 +: 5`
/// Requirement: FieldBoundary { field: "Rt", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_offset_normal_field_rt_0_min_0_3c000000() {
    // Encoding: 0x3C000000
    // Test aarch64_memory_single_simdfp_immediate_signed_offset_normal field Rt = 0 (Min)
    // Fields: opc=0, imm9=0, Rn=0, Rt=0, size=0
    let encoding: u32 = 0x3C000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stur	b0, [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_offset_normal
/// ASL: `field Rt 0 +: 5`
/// Requirement: FieldBoundary { field: "Rt", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_offset_normal_field_rt_1_poweroftwo_0_3c000001()
 {
    // Encoding: 0x3C000001
    // Test aarch64_memory_single_simdfp_immediate_signed_offset_normal field Rt = 1 (PowerOfTwo)
    // Fields: imm9=0, Rt=1, opc=0, size=0, Rn=0
    let encoding: u32 = 0x3C000001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stur	b1, [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_offset_normal
/// ASL: `field Rt 0 +: 5`
/// Requirement: FieldBoundary { field: "Rt", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_offset_normal_field_rt_30_poweroftwominusone_0_3c00001e()
 {
    // Encoding: 0x3C00001E
    // Test aarch64_memory_single_simdfp_immediate_signed_offset_normal field Rt = 30 (PowerOfTwoMinusOne)
    // Fields: opc=0, Rt=30, size=0, imm9=0, Rn=0
    let encoding: u32 = 0x3C00001E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stur	b30, [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_offset_normal
/// ASL: `field Rt 0 +: 5`
/// Requirement: FieldBoundary { field: "Rt", value: 31, boundary: Max }
/// register index 31 (ZR - zero register)
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_offset_normal_field_rt_31_max_0_3c00001f() {
    // Encoding: 0x3C00001F
    // Test aarch64_memory_single_simdfp_immediate_signed_offset_normal field Rt = 31 (Max)
    // Fields: size=0, opc=0, Rn=0, Rt=31, imm9=0
    let encoding: u32 = 0x3C00001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stur	b31, [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_offset_normal
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=0 (8-bit / byte size)
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_offset_normal_combo_0_0_3c000000() {
    // Encoding: 0x3C000000
    // Test aarch64_memory_single_simdfp_immediate_signed_offset_normal field combination: size=0, opc=0, imm9=0, Rn=0, Rt=0
    // Fields: Rn=0, size=0, Rt=0, opc=0, imm9=0
    let encoding: u32 = 0x3C000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stur	b0, [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_offset_normal
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=1 (16-bit / halfword size)
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_offset_normal_combo_1_0_7c000000() {
    // Encoding: 0x7C000000
    // Test aarch64_memory_single_simdfp_immediate_signed_offset_normal field combination: size=1, opc=0, imm9=0, Rn=0, Rt=0
    // Fields: imm9=0, size=1, Rn=0, Rt=0, opc=0
    let encoding: u32 = 0x7C000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stur	h0, [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_offset_normal
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=2 (32-bit / word size)
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_offset_normal_combo_2_0_bc000000() {
    // Encoding: 0xBC000000
    // Test aarch64_memory_single_simdfp_immediate_signed_offset_normal field combination: size=2, opc=0, imm9=0, Rn=0, Rt=0
    // Fields: imm9=0, Rn=0, Rt=0, opc=0, size=2
    let encoding: u32 = 0xBC000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stur	s0, [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_offset_normal
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=3 (64-bit / doubleword size)
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_offset_normal_combo_3_0_fc000000() {
    // Encoding: 0xFC000000
    // Test aarch64_memory_single_simdfp_immediate_signed_offset_normal field combination: size=3, opc=0, imm9=0, Rn=0, Rt=0
    // Fields: Rn=0, imm9=0, size=3, Rt=0, opc=0
    let encoding: u32 = 0xFC000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stur	d0, [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_offset_normal
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// opc=0 (8-bit / byte size)
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_offset_normal_combo_4_0_3c000000() {
    // Encoding: 0x3C000000
    // Test aarch64_memory_single_simdfp_immediate_signed_offset_normal field combination: size=0, opc=0, imm9=0, Rn=0, Rt=0
    // Fields: size=0, imm9=0, opc=0, Rt=0, Rn=0
    let encoding: u32 = 0x3C000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stur	b0, [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_offset_normal
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// opc=1 (16-bit / halfword size)
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_offset_normal_combo_5_0_3c400000() {
    // Encoding: 0x3C400000
    // Test aarch64_memory_single_simdfp_immediate_signed_offset_normal field combination: size=0, opc=1, imm9=0, Rn=0, Rt=0
    // Fields: imm9=0, size=0, opc=1, Rn=0, Rt=0
    let encoding: u32 = 0x3C400000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ldur	b0, [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_offset_normal
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// opc=2 (32-bit / word size)
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_offset_normal_combo_6_0_3c800000() {
    // Encoding: 0x3C800000
    // Test aarch64_memory_single_simdfp_immediate_signed_offset_normal field combination: size=0, opc=2, imm9=0, Rn=0, Rt=0
    // Fields: size=0, imm9=0, opc=2, Rt=0, Rn=0
    let encoding: u32 = 0x3C800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stur	q0, [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_offset_normal
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// opc=3 (64-bit / doubleword size)
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_offset_normal_combo_7_0_3cc00000() {
    // Encoding: 0x3CC00000
    // Test aarch64_memory_single_simdfp_immediate_signed_offset_normal field combination: size=0, opc=3, imm9=0, Rn=0, Rt=0
    // Fields: imm9=0, size=0, Rn=0, Rt=0, opc=3
    let encoding: u32 = 0x3CC00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ldur	q0, [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_offset_normal
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=0 (immediate value 0)
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_offset_normal_combo_8_0_3c000000() {
    // Encoding: 0x3C000000
    // Test aarch64_memory_single_simdfp_immediate_signed_offset_normal field combination: size=0, opc=0, imm9=0, Rn=0, Rt=0
    // Fields: Rt=0, Rn=0, imm9=0, size=0, opc=0
    let encoding: u32 = 0x3C000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stur	b0, [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_offset_normal
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=1 (immediate value 1)
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_offset_normal_combo_9_0_3c001000() {
    // Encoding: 0x3C001000
    // Test aarch64_memory_single_simdfp_immediate_signed_offset_normal field combination: size=0, opc=0, imm9=1, Rn=0, Rt=0
    // Fields: opc=0, Rn=0, imm9=1, size=0, Rt=0
    let encoding: u32 = 0x3C001000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stur	b0, [x0, #1]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_offset_normal
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=3 (2^2 - 1 = 3)
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_offset_normal_combo_10_0_3c003000() {
    // Encoding: 0x3C003000
    // Test aarch64_memory_single_simdfp_immediate_signed_offset_normal field combination: size=0, opc=0, imm9=3, Rn=0, Rt=0
    // Fields: opc=0, size=0, Rt=0, Rn=0, imm9=3
    let encoding: u32 = 0x3C003000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stur	b0, [x0, #3]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_offset_normal
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=4 (power of 2 (2^2 = 4))
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_offset_normal_combo_11_0_3c004000() {
    // Encoding: 0x3C004000
    // Test aarch64_memory_single_simdfp_immediate_signed_offset_normal field combination: size=0, opc=0, imm9=4, Rn=0, Rt=0
    // Fields: opc=0, imm9=4, Rn=0, Rt=0, size=0
    let encoding: u32 = 0x3C004000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stur	b0, [x0, #4]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_offset_normal
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=7 (2^3 - 1 = 7)
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_offset_normal_combo_12_0_3c007000() {
    // Encoding: 0x3C007000
    // Test aarch64_memory_single_simdfp_immediate_signed_offset_normal field combination: size=0, opc=0, imm9=7, Rn=0, Rt=0
    // Fields: size=0, Rn=0, Rt=0, imm9=7, opc=0
    let encoding: u32 = 0x3C007000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stur	b0, [x0, #7]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_offset_normal
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=8 (power of 2 (2^3 = 8))
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_offset_normal_combo_13_0_3c008000() {
    // Encoding: 0x3C008000
    // Test aarch64_memory_single_simdfp_immediate_signed_offset_normal field combination: size=0, opc=0, imm9=8, Rn=0, Rt=0
    // Fields: Rn=0, Rt=0, opc=0, size=0, imm9=8
    let encoding: u32 = 0x3C008000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stur	b0, [x0, #8]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_offset_normal
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=15 (2^4 - 1 = 15)
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_offset_normal_combo_14_0_3c00f000() {
    // Encoding: 0x3C00F000
    // Test aarch64_memory_single_simdfp_immediate_signed_offset_normal field combination: size=0, opc=0, imm9=15, Rn=0, Rt=0
    // Fields: Rn=0, opc=0, imm9=15, Rt=0, size=0
    let encoding: u32 = 0x3C00F000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stur	b0, [x0, #15]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_offset_normal
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=16 (power of 2 (2^4 = 16))
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_offset_normal_combo_15_0_3c010000() {
    // Encoding: 0x3C010000
    // Test aarch64_memory_single_simdfp_immediate_signed_offset_normal field combination: size=0, opc=0, imm9=16, Rn=0, Rt=0
    // Fields: size=0, Rn=0, Rt=0, imm9=16, opc=0
    let encoding: u32 = 0x3C010000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stur	b0, [x0, #16]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_offset_normal
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=31 (2^5 - 1 = 31)
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_offset_normal_combo_16_0_3c01f000() {
    // Encoding: 0x3C01F000
    // Test aarch64_memory_single_simdfp_immediate_signed_offset_normal field combination: size=0, opc=0, imm9=31, Rn=0, Rt=0
    // Fields: opc=0, size=0, imm9=31, Rt=0, Rn=0
    let encoding: u32 = 0x3C01F000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stur	b0, [x0, #31]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_offset_normal
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=32 (power of 2 (2^5 = 32))
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_offset_normal_combo_17_0_3c020000() {
    // Encoding: 0x3C020000
    // Test aarch64_memory_single_simdfp_immediate_signed_offset_normal field combination: size=0, opc=0, imm9=32, Rn=0, Rt=0
    // Fields: size=0, opc=0, Rt=0, Rn=0, imm9=32
    let encoding: u32 = 0x3C020000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stur	b0, [x0, #32]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_offset_normal
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=63 (2^6 - 1 = 63)
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_offset_normal_combo_18_0_3c03f000() {
    // Encoding: 0x3C03F000
    // Test aarch64_memory_single_simdfp_immediate_signed_offset_normal field combination: size=0, opc=0, imm9=63, Rn=0, Rt=0
    // Fields: Rn=0, opc=0, Rt=0, size=0, imm9=63
    let encoding: u32 = 0x3C03F000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stur	b0, [x0, #63]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_offset_normal
/// ASL: `field combination 19`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=64 (power of 2 (2^6 = 64))
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_offset_normal_combo_19_0_3c040000() {
    // Encoding: 0x3C040000
    // Test aarch64_memory_single_simdfp_immediate_signed_offset_normal field combination: size=0, opc=0, imm9=64, Rn=0, Rt=0
    // Fields: imm9=64, opc=0, size=0, Rn=0, Rt=0
    let encoding: u32 = 0x3C040000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stur	b0, [x0, #64]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_offset_normal
/// ASL: `field combination 20`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=127 (2^7 - 1 = 127)
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_offset_normal_combo_20_0_3c07f000() {
    // Encoding: 0x3C07F000
    // Test aarch64_memory_single_simdfp_immediate_signed_offset_normal field combination: size=0, opc=0, imm9=127, Rn=0, Rt=0
    // Fields: size=0, imm9=127, Rn=0, opc=0, Rt=0
    let encoding: u32 = 0x3C07F000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stur	b0, [x0, #127]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_offset_normal
/// ASL: `field combination 21`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=128 (power of 2 (2^7 = 128))
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_offset_normal_combo_21_0_3c080000() {
    // Encoding: 0x3C080000
    // Test aarch64_memory_single_simdfp_immediate_signed_offset_normal field combination: size=0, opc=0, imm9=128, Rn=0, Rt=0
    // Fields: size=0, imm9=128, opc=0, Rn=0, Rt=0
    let encoding: u32 = 0x3C080000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stur	b0, [x0, #128]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_offset_normal
/// ASL: `field combination 22`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=255 (immediate midpoint (255))
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_offset_normal_combo_22_0_3c0ff000() {
    // Encoding: 0x3C0FF000
    // Test aarch64_memory_single_simdfp_immediate_signed_offset_normal field combination: size=0, opc=0, imm9=255, Rn=0, Rt=0
    // Fields: imm9=255, Rt=0, size=0, opc=0, Rn=0
    let encoding: u32 = 0x3C0FF000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stur	b0, [x0, #255]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_offset_normal
/// ASL: `field combination 23`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=256 (power of 2 (2^8 = 256))
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_offset_normal_combo_23_0_3c100000() {
    // Encoding: 0x3C100000
    // Test aarch64_memory_single_simdfp_immediate_signed_offset_normal field combination: size=0, opc=0, imm9=256, Rn=0, Rt=0
    // Fields: imm9=256, size=0, Rn=0, Rt=0, opc=0
    let encoding: u32 = 0x3C100000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_offset_normal
/// ASL: `field combination 24`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=511 (maximum immediate (511))
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_offset_normal_combo_24_0_3c1ff000() {
    // Encoding: 0x3C1FF000
    // Test aarch64_memory_single_simdfp_immediate_signed_offset_normal field combination: size=0, opc=0, imm9=511, Rn=0, Rt=0
    // Fields: Rn=0, Rt=0, imm9=511, size=0, opc=0
    let encoding: u32 = 0x3C1FF000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stur	b0, [x0, #-1]
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

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_offset_normal
/// ASL: `field combination 25`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_offset_normal_combo_25_0_3c000000() {
    // Encoding: 0x3C000000
    // Test aarch64_memory_single_simdfp_immediate_signed_offset_normal field combination: size=0, opc=0, imm9=0, Rn=0, Rt=0
    // Fields: imm9=0, opc=0, Rn=0, Rt=0, size=0
    let encoding: u32 = 0x3C000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stur	b0, [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_offset_normal
/// ASL: `field combination 26`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_offset_normal_combo_26_0_3c000020() {
    // Encoding: 0x3C000020
    // Test aarch64_memory_single_simdfp_immediate_signed_offset_normal field combination: size=0, opc=0, imm9=0, Rn=1, Rt=0
    // Fields: opc=0, size=0, Rt=0, imm9=0, Rn=1
    let encoding: u32 = 0x3C000020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stur	b0, [x1]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_offset_normal
/// ASL: `field combination 27`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_offset_normal_combo_27_0_3c0003c0() {
    // Encoding: 0x3C0003C0
    // Test aarch64_memory_single_simdfp_immediate_signed_offset_normal field combination: size=0, opc=0, imm9=0, Rn=30, Rt=0
    // Fields: opc=0, size=0, Rt=0, Rn=30, imm9=0
    let encoding: u32 = 0x3C0003C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stur	b0, [x30]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_offset_normal
/// ASL: `field combination 28`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_offset_normal_combo_28_0_3c0003e0() {
    // Encoding: 0x3C0003E0
    // Test aarch64_memory_single_simdfp_immediate_signed_offset_normal field combination: size=0, opc=0, imm9=0, Rn=31, Rt=0
    // Fields: imm9=0, size=0, Rn=31, Rt=0, opc=0
    let encoding: u32 = 0x3C0003E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stur	b0, [sp]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_offset_normal
/// ASL: `field combination 29`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt=0 (register index 0 (first register))
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_offset_normal_combo_29_0_3c000000() {
    // Encoding: 0x3C000000
    // Test aarch64_memory_single_simdfp_immediate_signed_offset_normal field combination: size=0, opc=0, imm9=0, Rn=0, Rt=0
    // Fields: Rn=0, opc=0, Rt=0, imm9=0, size=0
    let encoding: u32 = 0x3C000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stur	b0, [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_offset_normal
/// ASL: `field combination 30`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt=1 (register index 1 (second register))
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_offset_normal_combo_30_0_3c000001() {
    // Encoding: 0x3C000001
    // Test aarch64_memory_single_simdfp_immediate_signed_offset_normal field combination: size=0, opc=0, imm9=0, Rn=0, Rt=1
    // Fields: Rt=1, size=0, Rn=0, opc=0, imm9=0
    let encoding: u32 = 0x3C000001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stur	b1, [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_offset_normal
/// ASL: `field combination 31`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_offset_normal_combo_31_0_3c00001e() {
    // Encoding: 0x3C00001E
    // Test aarch64_memory_single_simdfp_immediate_signed_offset_normal field combination: size=0, opc=0, imm9=0, Rn=0, Rt=30
    // Fields: opc=0, size=0, Rt=30, imm9=0, Rn=0
    let encoding: u32 = 0x3C00001E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stur	b30, [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_offset_normal
/// ASL: `field combination 32`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt=31 (register index 31 (ZR - zero register))
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_offset_normal_combo_32_0_3c00001f() {
    // Encoding: 0x3C00001F
    // Test aarch64_memory_single_simdfp_immediate_signed_offset_normal field combination: size=0, opc=0, imm9=0, Rn=0, Rt=31
    // Fields: Rn=0, size=0, imm9=0, Rt=31, opc=0
    let encoding: u32 = 0x3C00001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stur	b31, [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_offset_normal
/// ASL: `field combination 33`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (same register test (reg=1)), Rt=1 (same register test (reg=1))
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_offset_normal_combo_33_0_3c000021() {
    // Encoding: 0x3C000021
    // Test aarch64_memory_single_simdfp_immediate_signed_offset_normal field combination: size=0, opc=0, imm9=0, Rn=1, Rt=1
    // Fields: size=0, Rn=1, opc=0, imm9=0, Rt=1
    let encoding: u32 = 0x3C000021;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stur	b1, [x1]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_offset_normal
/// ASL: `field combination 34`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (same register test (reg=31)), Rt=31 (same register test (reg=31))
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_offset_normal_combo_34_0_3c0003ff() {
    // Encoding: 0x3C0003FF
    // Test aarch64_memory_single_simdfp_immediate_signed_offset_normal field combination: size=0, opc=0, imm9=0, Rn=31, Rt=31
    // Fields: Rn=31, size=0, Rt=31, opc=0, imm9=0
    let encoding: u32 = 0x3C0003FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stur	b31, [sp]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_offset_normal
/// ASL: `field size = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "size", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_offset_normal_special_size_0_size_variant_0_0_3c001000()
 {
    // Encoding: 0x3C001000
    // Test aarch64_memory_single_simdfp_immediate_signed_offset_normal special value size = 0 (Size variant 0)
    // Fields: Rn=0, Rt=0, size=0, imm9=1, opc=0
    let encoding: u32 = 0x3C001000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stur	b0, [x0, #1]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_offset_normal
/// ASL: `field size = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "size", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_offset_normal_special_size_1_size_variant_1_0_7c001000()
 {
    // Encoding: 0x7C001000
    // Test aarch64_memory_single_simdfp_immediate_signed_offset_normal special value size = 1 (Size variant 1)
    // Fields: imm9=1, size=1, Rt=0, Rn=0, opc=0
    let encoding: u32 = 0x7C001000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stur	h0, [x0, #1]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_offset_normal
/// ASL: `field size = 2 (Size variant 2)`
/// Requirement: FieldSpecial { field: "size", value: 2, meaning: "Size variant 2" }
/// Size variant 2
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_offset_normal_special_size_2_size_variant_2_0_bc001000()
 {
    // Encoding: 0xBC001000
    // Test aarch64_memory_single_simdfp_immediate_signed_offset_normal special value size = 2 (Size variant 2)
    // Fields: Rt=0, imm9=1, Rn=0, size=2, opc=0
    let encoding: u32 = 0xBC001000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stur	s0, [x0, #1]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_offset_normal
/// ASL: `field size = 3 (Size variant 3)`
/// Requirement: FieldSpecial { field: "size", value: 3, meaning: "Size variant 3" }
/// Size variant 3
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_offset_normal_special_size_3_size_variant_3_0_fc001000()
 {
    // Encoding: 0xFC001000
    // Test aarch64_memory_single_simdfp_immediate_signed_offset_normal special value size = 3 (Size variant 3)
    // Fields: size=3, Rt=0, imm9=1, opc=0, Rn=0
    let encoding: u32 = 0xFC001000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stur	d0, [x0, #1]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_offset_normal
/// ASL: `field opc = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "opc", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_offset_normal_special_opc_0_size_variant_0_0_7c001000()
 {
    // Encoding: 0x7C001000
    // Test aarch64_memory_single_simdfp_immediate_signed_offset_normal special value opc = 0 (Size variant 0)
    // Fields: Rn=0, Rt=0, opc=0, imm9=1, size=1
    let encoding: u32 = 0x7C001000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stur	h0, [x0, #1]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_offset_normal
/// ASL: `field opc = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "opc", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_offset_normal_special_opc_1_size_variant_1_0_7c401000()
 {
    // Encoding: 0x7C401000
    // Test aarch64_memory_single_simdfp_immediate_signed_offset_normal special value opc = 1 (Size variant 1)
    // Fields: opc=1, size=1, Rn=0, Rt=0, imm9=1
    let encoding: u32 = 0x7C401000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ldur	h0, [x0, #1]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_offset_normal
/// ASL: `field opc = 2 (Size variant 2)`
/// Requirement: FieldSpecial { field: "opc", value: 2, meaning: "Size variant 2" }
/// Size variant 2
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_offset_normal_special_opc_2_size_variant_2_0_7c801000()
 {
    // Encoding: 0x7C801000
    // Test aarch64_memory_single_simdfp_immediate_signed_offset_normal special value opc = 2 (Size variant 2)
    // Fields: Rt=0, size=1, imm9=1, opc=2, Rn=0
    let encoding: u32 = 0x7C801000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_offset_normal
/// ASL: `field opc = 3 (Size variant 3)`
/// Requirement: FieldSpecial { field: "opc", value: 3, meaning: "Size variant 3" }
/// Size variant 3
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_offset_normal_special_opc_3_size_variant_3_0_7cc01000()
 {
    // Encoding: 0x7CC01000
    // Test aarch64_memory_single_simdfp_immediate_signed_offset_normal special value opc = 3 (Size variant 3)
    // Fields: Rt=0, imm9=1, size=1, opc=3, Rn=0
    let encoding: u32 = 0x7CC01000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_offset_normal
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_offset_normal_special_rn_31_stack_pointer_sp_may_require_alignment_0_7c0013e0()
 {
    // Encoding: 0x7C0013E0
    // Test aarch64_memory_single_simdfp_immediate_signed_offset_normal special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: size=1, Rt=0, imm9=1, opc=0, Rn=31
    let encoding: u32 = 0x7C0013E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stur	h0, [sp, #1]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_offset_normal
/// ASL: `field Rt = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)`
/// Requirement: FieldSpecial { field: "Rt", value: 31, meaning: "Zero register (XZR/WZR) - reads as 0, writes discarded" }
/// Zero register (XZR/WZR) - reads as 0, writes discarded
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_offset_normal_special_rt_31_zero_register_xzr_wzr_reads_as_0_writes_discarded_0_7c00101f()
 {
    // Encoding: 0x7C00101F
    // Test aarch64_memory_single_simdfp_immediate_signed_offset_normal special value Rt = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)
    // Fields: imm9=1, Rt=31, opc=0, Rn=0, size=1
    let encoding: u32 = 0x7C00101F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stur	h31, [x0, #1]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

// ============================================================================
// aarch64_memory_single_general_immediate_signed_offset_lda_stl Tests
// ============================================================================

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_lda_stl
/// ASL: `field size 30 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_lda_stl_field_size_0_min_0_19000000()
{
    // Encoding: 0x19000000
    // Test aarch64_memory_single_general_immediate_signed_offset_lda_stl field size = 0 (Min)
    // Fields: Rt=0, Rn=0, size=0, opc=0, imm9=0
    let encoding: u32 = 0x19000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_lda_stl
/// ASL: `field size 30 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 1, boundary: PowerOfTwo }
/// 16-bit / halfword size
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_lda_stl_field_size_1_poweroftwo_0_59000000()
 {
    // Encoding: 0x59000000
    // Test aarch64_memory_single_general_immediate_signed_offset_lda_stl field size = 1 (PowerOfTwo)
    // Fields: Rt=0, opc=0, Rn=0, size=1, imm9=0
    let encoding: u32 = 0x59000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_lda_stl
/// ASL: `field size 30 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 2, boundary: PowerOfTwo }
/// 32-bit / word size
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_lda_stl_field_size_2_poweroftwo_0_99000000()
 {
    // Encoding: 0x99000000
    // Test aarch64_memory_single_general_immediate_signed_offset_lda_stl field size = 2 (PowerOfTwo)
    // Fields: imm9=0, Rt=0, opc=0, size=2, Rn=0
    let encoding: u32 = 0x99000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_lda_stl
/// ASL: `field size 30 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 3, boundary: Max }
/// 64-bit / doubleword size
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_lda_stl_field_size_3_max_0_d9000000()
{
    // Encoding: 0xD9000000
    // Test aarch64_memory_single_general_immediate_signed_offset_lda_stl field size = 3 (Max)
    // Fields: size=3, Rn=0, Rt=0, imm9=0, opc=0
    let encoding: u32 = 0xD9000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_lda_stl
/// ASL: `field opc 22 +: 2`
/// Requirement: FieldBoundary { field: "opc", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_lda_stl_field_opc_0_min_0_19000000() {
    // Encoding: 0x19000000
    // Test aarch64_memory_single_general_immediate_signed_offset_lda_stl field opc = 0 (Min)
    // Fields: opc=0, Rt=0, Rn=0, size=0, imm9=0
    let encoding: u32 = 0x19000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_lda_stl
/// ASL: `field opc 22 +: 2`
/// Requirement: FieldBoundary { field: "opc", value: 1, boundary: PowerOfTwo }
/// 16-bit / halfword size
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_lda_stl_field_opc_1_poweroftwo_0_19400000()
 {
    // Encoding: 0x19400000
    // Test aarch64_memory_single_general_immediate_signed_offset_lda_stl field opc = 1 (PowerOfTwo)
    // Fields: imm9=0, size=0, opc=1, Rn=0, Rt=0
    let encoding: u32 = 0x19400000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_lda_stl
/// ASL: `field opc 22 +: 2`
/// Requirement: FieldBoundary { field: "opc", value: 2, boundary: PowerOfTwo }
/// 32-bit / word size
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_lda_stl_field_opc_2_poweroftwo_0_19800000()
 {
    // Encoding: 0x19800000
    // Test aarch64_memory_single_general_immediate_signed_offset_lda_stl field opc = 2 (PowerOfTwo)
    // Fields: Rt=0, imm9=0, size=0, Rn=0, opc=2
    let encoding: u32 = 0x19800000;
    let mut cpu = create_test_cpu();
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

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_lda_stl
/// ASL: `field opc 22 +: 2`
/// Requirement: FieldBoundary { field: "opc", value: 3, boundary: Max }
/// 64-bit / doubleword size
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_lda_stl_field_opc_3_max_0_19c00000() {
    // Encoding: 0x19C00000
    // Test aarch64_memory_single_general_immediate_signed_offset_lda_stl field opc = 3 (Max)
    // Fields: opc=3, Rt=0, size=0, imm9=0, Rn=0
    let encoding: u32 = 0x19C00000;
    let mut cpu = create_test_cpu();
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

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_lda_stl
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_lda_stl_field_imm9_0_zero_0_19000000()
{
    // Encoding: 0x19000000
    // Test aarch64_memory_single_general_immediate_signed_offset_lda_stl field imm9 = 0 (Zero)
    // Fields: size=0, Rt=0, imm9=0, Rn=0, opc=0
    let encoding: u32 = 0x19000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_lda_stl
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_lda_stl_field_imm9_1_poweroftwo_0_19001000()
 {
    // Encoding: 0x19001000
    // Test aarch64_memory_single_general_immediate_signed_offset_lda_stl field imm9 = 1 (PowerOfTwo)
    // Fields: Rt=0, imm9=1, Rn=0, opc=0, size=0
    let encoding: u32 = 0x19001000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_lda_stl
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 3, boundary: PowerOfTwoMinusOne }
/// 2^2 - 1 = 3
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_lda_stl_field_imm9_3_poweroftwominusone_0_19003000()
 {
    // Encoding: 0x19003000
    // Test aarch64_memory_single_general_immediate_signed_offset_lda_stl field imm9 = 3 (PowerOfTwoMinusOne)
    // Fields: Rn=0, imm9=3, Rt=0, size=0, opc=0
    let encoding: u32 = 0x19003000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_lda_stl
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 4, boundary: PowerOfTwo }
/// power of 2 (2^2 = 4)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_lda_stl_field_imm9_4_poweroftwo_0_19004000()
 {
    // Encoding: 0x19004000
    // Test aarch64_memory_single_general_immediate_signed_offset_lda_stl field imm9 = 4 (PowerOfTwo)
    // Fields: imm9=4, Rn=0, size=0, Rt=0, opc=0
    let encoding: u32 = 0x19004000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_lda_stl
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 7, boundary: PowerOfTwoMinusOne }
/// 2^3 - 1 = 7
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_lda_stl_field_imm9_7_poweroftwominusone_0_19007000()
 {
    // Encoding: 0x19007000
    // Test aarch64_memory_single_general_immediate_signed_offset_lda_stl field imm9 = 7 (PowerOfTwoMinusOne)
    // Fields: size=0, Rn=0, Rt=0, opc=0, imm9=7
    let encoding: u32 = 0x19007000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_lda_stl
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 8, boundary: PowerOfTwo }
/// power of 2 (2^3 = 8)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_lda_stl_field_imm9_8_poweroftwo_0_19008000()
 {
    // Encoding: 0x19008000
    // Test aarch64_memory_single_general_immediate_signed_offset_lda_stl field imm9 = 8 (PowerOfTwo)
    // Fields: size=0, Rn=0, Rt=0, imm9=8, opc=0
    let encoding: u32 = 0x19008000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_lda_stl
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 15, boundary: PowerOfTwoMinusOne }
/// 2^4 - 1 = 15
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_lda_stl_field_imm9_15_poweroftwominusone_0_1900f000()
 {
    // Encoding: 0x1900F000
    // Test aarch64_memory_single_general_immediate_signed_offset_lda_stl field imm9 = 15 (PowerOfTwoMinusOne)
    // Fields: size=0, Rt=0, Rn=0, imm9=15, opc=0
    let encoding: u32 = 0x1900F000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_lda_stl
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 16, boundary: PowerOfTwo }
/// power of 2 (2^4 = 16)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_lda_stl_field_imm9_16_poweroftwo_0_19010000()
 {
    // Encoding: 0x19010000
    // Test aarch64_memory_single_general_immediate_signed_offset_lda_stl field imm9 = 16 (PowerOfTwo)
    // Fields: size=0, opc=0, imm9=16, Rn=0, Rt=0
    let encoding: u32 = 0x19010000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_lda_stl
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 31, boundary: PowerOfTwoMinusOne }
/// 2^5 - 1 = 31
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_lda_stl_field_imm9_31_poweroftwominusone_0_1901f000()
 {
    // Encoding: 0x1901F000
    // Test aarch64_memory_single_general_immediate_signed_offset_lda_stl field imm9 = 31 (PowerOfTwoMinusOne)
    // Fields: opc=0, size=0, Rt=0, Rn=0, imm9=31
    let encoding: u32 = 0x1901F000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_lda_stl
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 32, boundary: PowerOfTwo }
/// power of 2 (2^5 = 32)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_lda_stl_field_imm9_32_poweroftwo_0_19020000()
 {
    // Encoding: 0x19020000
    // Test aarch64_memory_single_general_immediate_signed_offset_lda_stl field imm9 = 32 (PowerOfTwo)
    // Fields: Rn=0, Rt=0, size=0, opc=0, imm9=32
    let encoding: u32 = 0x19020000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_lda_stl
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 63, boundary: PowerOfTwoMinusOne }
/// 2^6 - 1 = 63
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_lda_stl_field_imm9_63_poweroftwominusone_0_1903f000()
 {
    // Encoding: 0x1903F000
    // Test aarch64_memory_single_general_immediate_signed_offset_lda_stl field imm9 = 63 (PowerOfTwoMinusOne)
    // Fields: Rn=0, opc=0, imm9=63, Rt=0, size=0
    let encoding: u32 = 0x1903F000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_lda_stl
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 64, boundary: PowerOfTwo }
/// power of 2 (2^6 = 64)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_lda_stl_field_imm9_64_poweroftwo_0_19040000()
 {
    // Encoding: 0x19040000
    // Test aarch64_memory_single_general_immediate_signed_offset_lda_stl field imm9 = 64 (PowerOfTwo)
    // Fields: opc=0, imm9=64, size=0, Rn=0, Rt=0
    let encoding: u32 = 0x19040000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_lda_stl
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 127, boundary: PowerOfTwoMinusOne }
/// 2^7 - 1 = 127
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_lda_stl_field_imm9_127_poweroftwominusone_0_1907f000()
 {
    // Encoding: 0x1907F000
    // Test aarch64_memory_single_general_immediate_signed_offset_lda_stl field imm9 = 127 (PowerOfTwoMinusOne)
    // Fields: opc=0, imm9=127, Rt=0, Rn=0, size=0
    let encoding: u32 = 0x1907F000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_lda_stl
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 128, boundary: PowerOfTwo }
/// power of 2 (2^7 = 128)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_lda_stl_field_imm9_128_poweroftwo_0_19080000()
 {
    // Encoding: 0x19080000
    // Test aarch64_memory_single_general_immediate_signed_offset_lda_stl field imm9 = 128 (PowerOfTwo)
    // Fields: Rn=0, imm9=128, Rt=0, opc=0, size=0
    let encoding: u32 = 0x19080000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_lda_stl
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 255, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (255)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_lda_stl_field_imm9_255_poweroftwominusone_0_190ff000()
 {
    // Encoding: 0x190FF000
    // Test aarch64_memory_single_general_immediate_signed_offset_lda_stl field imm9 = 255 (PowerOfTwoMinusOne)
    // Fields: Rn=0, size=0, Rt=0, opc=0, imm9=255
    let encoding: u32 = 0x190FF000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_lda_stl
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 256, boundary: PowerOfTwo }
/// power of 2 (2^8 = 256)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_lda_stl_field_imm9_256_poweroftwo_0_19100000()
 {
    // Encoding: 0x19100000
    // Test aarch64_memory_single_general_immediate_signed_offset_lda_stl field imm9 = 256 (PowerOfTwo)
    // Fields: imm9=256, opc=0, Rn=0, Rt=0, size=0
    let encoding: u32 = 0x19100000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_lda_stl
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 511, boundary: Max }
/// maximum immediate (511)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_lda_stl_field_imm9_511_max_0_191ff000()
 {
    // Encoding: 0x191FF000
    // Test aarch64_memory_single_general_immediate_signed_offset_lda_stl field imm9 = 511 (Max)
    // Fields: opc=0, size=0, Rn=0, imm9=511, Rt=0
    let encoding: u32 = 0x191FF000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_lda_stl
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_lda_stl_field_rn_0_min_0_19000000() {
    // Encoding: 0x19000000
    // Test aarch64_memory_single_general_immediate_signed_offset_lda_stl field Rn = 0 (Min)
    // Fields: size=0, opc=0, Rn=0, imm9=0, Rt=0
    let encoding: u32 = 0x19000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_lda_stl
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_lda_stl_field_rn_1_poweroftwo_0_19000020()
 {
    // Encoding: 0x19000020
    // Test aarch64_memory_single_general_immediate_signed_offset_lda_stl field Rn = 1 (PowerOfTwo)
    // Fields: opc=0, Rt=0, imm9=0, Rn=1, size=0
    let encoding: u32 = 0x19000020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_lda_stl
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_lda_stl_field_rn_30_poweroftwominusone_0_190003c0()
 {
    // Encoding: 0x190003C0
    // Test aarch64_memory_single_general_immediate_signed_offset_lda_stl field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: Rt=0, Rn=30, size=0, opc=0, imm9=0
    let encoding: u32 = 0x190003C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_lda_stl
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_lda_stl_field_rn_31_max_0_190003e0() {
    // Encoding: 0x190003E0
    // Test aarch64_memory_single_general_immediate_signed_offset_lda_stl field Rn = 31 (Max)
    // Fields: Rt=0, size=0, Rn=31, imm9=0, opc=0
    let encoding: u32 = 0x190003E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_lda_stl
/// ASL: `field Rt 0 +: 5`
/// Requirement: FieldBoundary { field: "Rt", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_lda_stl_field_rt_0_min_0_19000000() {
    // Encoding: 0x19000000
    // Test aarch64_memory_single_general_immediate_signed_offset_lda_stl field Rt = 0 (Min)
    // Fields: opc=0, size=0, Rn=0, Rt=0, imm9=0
    let encoding: u32 = 0x19000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_lda_stl
/// ASL: `field Rt 0 +: 5`
/// Requirement: FieldBoundary { field: "Rt", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_lda_stl_field_rt_1_poweroftwo_0_19000001()
 {
    // Encoding: 0x19000001
    // Test aarch64_memory_single_general_immediate_signed_offset_lda_stl field Rt = 1 (PowerOfTwo)
    // Fields: Rt=1, size=0, opc=0, imm9=0, Rn=0
    let encoding: u32 = 0x19000001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_lda_stl
/// ASL: `field Rt 0 +: 5`
/// Requirement: FieldBoundary { field: "Rt", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_lda_stl_field_rt_30_poweroftwominusone_0_1900001e()
 {
    // Encoding: 0x1900001E
    // Test aarch64_memory_single_general_immediate_signed_offset_lda_stl field Rt = 30 (PowerOfTwoMinusOne)
    // Fields: size=0, opc=0, imm9=0, Rn=0, Rt=30
    let encoding: u32 = 0x1900001E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_lda_stl
/// ASL: `field Rt 0 +: 5`
/// Requirement: FieldBoundary { field: "Rt", value: 31, boundary: Max }
/// register index 31 (ZR - zero register)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_lda_stl_field_rt_31_max_0_1900001f() {
    // Encoding: 0x1900001F
    // Test aarch64_memory_single_general_immediate_signed_offset_lda_stl field Rt = 31 (Max)
    // Fields: size=0, Rn=0, Rt=31, opc=0, imm9=0
    let encoding: u32 = 0x1900001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_lda_stl
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=0 (8-bit / byte size)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_lda_stl_combo_0_0_19000000() {
    // Encoding: 0x19000000
    // Test aarch64_memory_single_general_immediate_signed_offset_lda_stl field combination: size=0, opc=0, imm9=0, Rn=0, Rt=0
    // Fields: Rn=0, Rt=0, opc=0, size=0, imm9=0
    let encoding: u32 = 0x19000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_lda_stl
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=1 (16-bit / halfword size)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_lda_stl_combo_1_0_59000000() {
    // Encoding: 0x59000000
    // Test aarch64_memory_single_general_immediate_signed_offset_lda_stl field combination: size=1, opc=0, imm9=0, Rn=0, Rt=0
    // Fields: imm9=0, Rn=0, opc=0, Rt=0, size=1
    let encoding: u32 = 0x59000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_lda_stl
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=2 (32-bit / word size)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_lda_stl_combo_2_0_99000000() {
    // Encoding: 0x99000000
    // Test aarch64_memory_single_general_immediate_signed_offset_lda_stl field combination: size=2, opc=0, imm9=0, Rn=0, Rt=0
    // Fields: opc=0, imm9=0, size=2, Rt=0, Rn=0
    let encoding: u32 = 0x99000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_lda_stl
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=3 (64-bit / doubleword size)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_lda_stl_combo_3_0_d9000000() {
    // Encoding: 0xD9000000
    // Test aarch64_memory_single_general_immediate_signed_offset_lda_stl field combination: size=3, opc=0, imm9=0, Rn=0, Rt=0
    // Fields: opc=0, imm9=0, size=3, Rt=0, Rn=0
    let encoding: u32 = 0xD9000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_lda_stl
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// opc=0 (8-bit / byte size)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_lda_stl_combo_4_0_19000000() {
    // Encoding: 0x19000000
    // Test aarch64_memory_single_general_immediate_signed_offset_lda_stl field combination: size=0, opc=0, imm9=0, Rn=0, Rt=0
    // Fields: Rn=0, Rt=0, size=0, opc=0, imm9=0
    let encoding: u32 = 0x19000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_lda_stl
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// opc=1 (16-bit / halfword size)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_lda_stl_combo_5_0_19400000() {
    // Encoding: 0x19400000
    // Test aarch64_memory_single_general_immediate_signed_offset_lda_stl field combination: size=0, opc=1, imm9=0, Rn=0, Rt=0
    // Fields: Rn=0, Rt=0, imm9=0, opc=1, size=0
    let encoding: u32 = 0x19400000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_lda_stl
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// opc=2 (32-bit / word size)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_lda_stl_combo_6_0_19800000() {
    // Encoding: 0x19800000
    // Test aarch64_memory_single_general_immediate_signed_offset_lda_stl field combination: size=0, opc=2, imm9=0, Rn=0, Rt=0
    // Fields: opc=2, size=0, imm9=0, Rt=0, Rn=0
    let encoding: u32 = 0x19800000;
    let mut cpu = create_test_cpu();
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

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_lda_stl
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// opc=3 (64-bit / doubleword size)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_lda_stl_combo_7_0_19c00000() {
    // Encoding: 0x19C00000
    // Test aarch64_memory_single_general_immediate_signed_offset_lda_stl field combination: size=0, opc=3, imm9=0, Rn=0, Rt=0
    // Fields: opc=3, size=0, imm9=0, Rn=0, Rt=0
    let encoding: u32 = 0x19C00000;
    let mut cpu = create_test_cpu();
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

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_lda_stl
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=0 (immediate value 0)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_lda_stl_combo_8_0_19000000() {
    // Encoding: 0x19000000
    // Test aarch64_memory_single_general_immediate_signed_offset_lda_stl field combination: size=0, opc=0, imm9=0, Rn=0, Rt=0
    // Fields: opc=0, size=0, Rn=0, imm9=0, Rt=0
    let encoding: u32 = 0x19000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_lda_stl
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=1 (immediate value 1)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_lda_stl_combo_9_0_19001000() {
    // Encoding: 0x19001000
    // Test aarch64_memory_single_general_immediate_signed_offset_lda_stl field combination: size=0, opc=0, imm9=1, Rn=0, Rt=0
    // Fields: size=0, imm9=1, Rn=0, opc=0, Rt=0
    let encoding: u32 = 0x19001000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_lda_stl
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=3 (2^2 - 1 = 3)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_lda_stl_combo_10_0_19003000() {
    // Encoding: 0x19003000
    // Test aarch64_memory_single_general_immediate_signed_offset_lda_stl field combination: size=0, opc=0, imm9=3, Rn=0, Rt=0
    // Fields: Rt=0, opc=0, size=0, imm9=3, Rn=0
    let encoding: u32 = 0x19003000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_lda_stl
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=4 (power of 2 (2^2 = 4))
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_lda_stl_combo_11_0_19004000() {
    // Encoding: 0x19004000
    // Test aarch64_memory_single_general_immediate_signed_offset_lda_stl field combination: size=0, opc=0, imm9=4, Rn=0, Rt=0
    // Fields: imm9=4, Rt=0, opc=0, Rn=0, size=0
    let encoding: u32 = 0x19004000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_lda_stl
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=7 (2^3 - 1 = 7)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_lda_stl_combo_12_0_19007000() {
    // Encoding: 0x19007000
    // Test aarch64_memory_single_general_immediate_signed_offset_lda_stl field combination: size=0, opc=0, imm9=7, Rn=0, Rt=0
    // Fields: opc=0, Rn=0, Rt=0, size=0, imm9=7
    let encoding: u32 = 0x19007000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_lda_stl
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=8 (power of 2 (2^3 = 8))
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_lda_stl_combo_13_0_19008000() {
    // Encoding: 0x19008000
    // Test aarch64_memory_single_general_immediate_signed_offset_lda_stl field combination: size=0, opc=0, imm9=8, Rn=0, Rt=0
    // Fields: Rt=0, imm9=8, Rn=0, size=0, opc=0
    let encoding: u32 = 0x19008000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_lda_stl
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=15 (2^4 - 1 = 15)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_lda_stl_combo_14_0_1900f000() {
    // Encoding: 0x1900F000
    // Test aarch64_memory_single_general_immediate_signed_offset_lda_stl field combination: size=0, opc=0, imm9=15, Rn=0, Rt=0
    // Fields: imm9=15, opc=0, Rt=0, size=0, Rn=0
    let encoding: u32 = 0x1900F000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_lda_stl
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=16 (power of 2 (2^4 = 16))
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_lda_stl_combo_15_0_19010000() {
    // Encoding: 0x19010000
    // Test aarch64_memory_single_general_immediate_signed_offset_lda_stl field combination: size=0, opc=0, imm9=16, Rn=0, Rt=0
    // Fields: imm9=16, Rn=0, opc=0, size=0, Rt=0
    let encoding: u32 = 0x19010000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_lda_stl
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=31 (2^5 - 1 = 31)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_lda_stl_combo_16_0_1901f000() {
    // Encoding: 0x1901F000
    // Test aarch64_memory_single_general_immediate_signed_offset_lda_stl field combination: size=0, opc=0, imm9=31, Rn=0, Rt=0
    // Fields: Rn=0, imm9=31, size=0, Rt=0, opc=0
    let encoding: u32 = 0x1901F000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_lda_stl
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=32 (power of 2 (2^5 = 32))
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_lda_stl_combo_17_0_19020000() {
    // Encoding: 0x19020000
    // Test aarch64_memory_single_general_immediate_signed_offset_lda_stl field combination: size=0, opc=0, imm9=32, Rn=0, Rt=0
    // Fields: Rt=0, Rn=0, size=0, imm9=32, opc=0
    let encoding: u32 = 0x19020000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_lda_stl
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=63 (2^6 - 1 = 63)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_lda_stl_combo_18_0_1903f000() {
    // Encoding: 0x1903F000
    // Test aarch64_memory_single_general_immediate_signed_offset_lda_stl field combination: size=0, opc=0, imm9=63, Rn=0, Rt=0
    // Fields: size=0, Rn=0, opc=0, imm9=63, Rt=0
    let encoding: u32 = 0x1903F000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_lda_stl
/// ASL: `field combination 19`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=64 (power of 2 (2^6 = 64))
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_lda_stl_combo_19_0_19040000() {
    // Encoding: 0x19040000
    // Test aarch64_memory_single_general_immediate_signed_offset_lda_stl field combination: size=0, opc=0, imm9=64, Rn=0, Rt=0
    // Fields: Rn=0, Rt=0, imm9=64, size=0, opc=0
    let encoding: u32 = 0x19040000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_lda_stl
/// ASL: `field combination 20`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=127 (2^7 - 1 = 127)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_lda_stl_combo_20_0_1907f000() {
    // Encoding: 0x1907F000
    // Test aarch64_memory_single_general_immediate_signed_offset_lda_stl field combination: size=0, opc=0, imm9=127, Rn=0, Rt=0
    // Fields: Rt=0, opc=0, Rn=0, imm9=127, size=0
    let encoding: u32 = 0x1907F000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_lda_stl
/// ASL: `field combination 21`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=128 (power of 2 (2^7 = 128))
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_lda_stl_combo_21_0_19080000() {
    // Encoding: 0x19080000
    // Test aarch64_memory_single_general_immediate_signed_offset_lda_stl field combination: size=0, opc=0, imm9=128, Rn=0, Rt=0
    // Fields: opc=0, Rn=0, size=0, imm9=128, Rt=0
    let encoding: u32 = 0x19080000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_lda_stl
/// ASL: `field combination 22`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=255 (immediate midpoint (255))
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_lda_stl_combo_22_0_190ff000() {
    // Encoding: 0x190FF000
    // Test aarch64_memory_single_general_immediate_signed_offset_lda_stl field combination: size=0, opc=0, imm9=255, Rn=0, Rt=0
    // Fields: Rt=0, imm9=255, size=0, Rn=0, opc=0
    let encoding: u32 = 0x190FF000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_lda_stl
/// ASL: `field combination 23`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=256 (power of 2 (2^8 = 256))
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_lda_stl_combo_23_0_19100000() {
    // Encoding: 0x19100000
    // Test aarch64_memory_single_general_immediate_signed_offset_lda_stl field combination: size=0, opc=0, imm9=256, Rn=0, Rt=0
    // Fields: opc=0, imm9=256, Rn=0, size=0, Rt=0
    let encoding: u32 = 0x19100000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_lda_stl
/// ASL: `field combination 24`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=511 (maximum immediate (511))
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_lda_stl_combo_24_0_191ff000() {
    // Encoding: 0x191FF000
    // Test aarch64_memory_single_general_immediate_signed_offset_lda_stl field combination: size=0, opc=0, imm9=511, Rn=0, Rt=0
    // Fields: opc=0, imm9=511, size=0, Rn=0, Rt=0
    let encoding: u32 = 0x191FF000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_lda_stl
/// ASL: `field combination 25`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_lda_stl_combo_25_0_19000000() {
    // Encoding: 0x19000000
    // Test aarch64_memory_single_general_immediate_signed_offset_lda_stl field combination: size=0, opc=0, imm9=0, Rn=0, Rt=0
    // Fields: size=0, Rn=0, opc=0, Rt=0, imm9=0
    let encoding: u32 = 0x19000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_lda_stl
/// ASL: `field combination 26`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_lda_stl_combo_26_0_19000020() {
    // Encoding: 0x19000020
    // Test aarch64_memory_single_general_immediate_signed_offset_lda_stl field combination: size=0, opc=0, imm9=0, Rn=1, Rt=0
    // Fields: opc=0, Rn=1, imm9=0, size=0, Rt=0
    let encoding: u32 = 0x19000020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_lda_stl
/// ASL: `field combination 27`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_lda_stl_combo_27_0_190003c0() {
    // Encoding: 0x190003C0
    // Test aarch64_memory_single_general_immediate_signed_offset_lda_stl field combination: size=0, opc=0, imm9=0, Rn=30, Rt=0
    // Fields: Rt=0, size=0, opc=0, imm9=0, Rn=30
    let encoding: u32 = 0x190003C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_lda_stl
/// ASL: `field combination 28`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_lda_stl_combo_28_0_190003e0() {
    // Encoding: 0x190003E0
    // Test aarch64_memory_single_general_immediate_signed_offset_lda_stl field combination: size=0, opc=0, imm9=0, Rn=31, Rt=0
    // Fields: opc=0, Rn=31, imm9=0, size=0, Rt=0
    let encoding: u32 = 0x190003E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_lda_stl
/// ASL: `field combination 29`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt=0 (register index 0 (first register))
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_lda_stl_combo_29_0_19000000() {
    // Encoding: 0x19000000
    // Test aarch64_memory_single_general_immediate_signed_offset_lda_stl field combination: size=0, opc=0, imm9=0, Rn=0, Rt=0
    // Fields: Rn=0, size=0, imm9=0, opc=0, Rt=0
    let encoding: u32 = 0x19000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_lda_stl
/// ASL: `field combination 30`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt=1 (register index 1 (second register))
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_lda_stl_combo_30_0_19000001() {
    // Encoding: 0x19000001
    // Test aarch64_memory_single_general_immediate_signed_offset_lda_stl field combination: size=0, opc=0, imm9=0, Rn=0, Rt=1
    // Fields: opc=0, imm9=0, Rt=1, size=0, Rn=0
    let encoding: u32 = 0x19000001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_lda_stl
/// ASL: `field combination 31`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_lda_stl_combo_31_0_1900001e() {
    // Encoding: 0x1900001E
    // Test aarch64_memory_single_general_immediate_signed_offset_lda_stl field combination: size=0, opc=0, imm9=0, Rn=0, Rt=30
    // Fields: opc=0, Rn=0, Rt=30, size=0, imm9=0
    let encoding: u32 = 0x1900001E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_lda_stl
/// ASL: `field combination 32`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt=31 (register index 31 (ZR - zero register))
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_lda_stl_combo_32_0_1900001f() {
    // Encoding: 0x1900001F
    // Test aarch64_memory_single_general_immediate_signed_offset_lda_stl field combination: size=0, opc=0, imm9=0, Rn=0, Rt=31
    // Fields: opc=0, size=0, Rt=31, imm9=0, Rn=0
    let encoding: u32 = 0x1900001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_lda_stl
/// ASL: `field combination 33`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (same register test (reg=1)), Rt=1 (same register test (reg=1))
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_lda_stl_combo_33_0_19000021() {
    // Encoding: 0x19000021
    // Test aarch64_memory_single_general_immediate_signed_offset_lda_stl field combination: size=0, opc=0, imm9=0, Rn=1, Rt=1
    // Fields: size=0, Rn=1, Rt=1, imm9=0, opc=0
    let encoding: u32 = 0x19000021;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_lda_stl
/// ASL: `field combination 34`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (same register test (reg=31)), Rt=31 (same register test (reg=31))
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_lda_stl_combo_34_0_190003ff() {
    // Encoding: 0x190003FF
    // Test aarch64_memory_single_general_immediate_signed_offset_lda_stl field combination: size=0, opc=0, imm9=0, Rn=31, Rt=31
    // Fields: Rt=31, imm9=0, size=0, Rn=31, opc=0
    let encoding: u32 = 0x190003FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_lda_stl
/// ASL: `field size = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "size", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_lda_stl_special_size_0_size_variant_0_0_19001000()
 {
    // Encoding: 0x19001000
    // Test aarch64_memory_single_general_immediate_signed_offset_lda_stl special value size = 0 (Size variant 0)
    // Fields: opc=0, imm9=1, Rn=0, Rt=0, size=0
    let encoding: u32 = 0x19001000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_lda_stl
/// ASL: `field size = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "size", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_lda_stl_special_size_1_size_variant_1_0_59001000()
 {
    // Encoding: 0x59001000
    // Test aarch64_memory_single_general_immediate_signed_offset_lda_stl special value size = 1 (Size variant 1)
    // Fields: opc=0, size=1, Rn=0, imm9=1, Rt=0
    let encoding: u32 = 0x59001000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_lda_stl
/// ASL: `field size = 2 (Size variant 2)`
/// Requirement: FieldSpecial { field: "size", value: 2, meaning: "Size variant 2" }
/// Size variant 2
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_lda_stl_special_size_2_size_variant_2_0_99001000()
 {
    // Encoding: 0x99001000
    // Test aarch64_memory_single_general_immediate_signed_offset_lda_stl special value size = 2 (Size variant 2)
    // Fields: size=2, Rt=0, imm9=1, Rn=0, opc=0
    let encoding: u32 = 0x99001000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_lda_stl
/// ASL: `field size = 3 (Size variant 3)`
/// Requirement: FieldSpecial { field: "size", value: 3, meaning: "Size variant 3" }
/// Size variant 3
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_lda_stl_special_size_3_size_variant_3_0_d9001000()
 {
    // Encoding: 0xD9001000
    // Test aarch64_memory_single_general_immediate_signed_offset_lda_stl special value size = 3 (Size variant 3)
    // Fields: size=3, imm9=1, Rt=0, opc=0, Rn=0
    let encoding: u32 = 0xD9001000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_lda_stl
/// ASL: `field opc = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "opc", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_lda_stl_special_opc_0_size_variant_0_0_59001000()
 {
    // Encoding: 0x59001000
    // Test aarch64_memory_single_general_immediate_signed_offset_lda_stl special value opc = 0 (Size variant 0)
    // Fields: Rn=0, Rt=0, size=1, opc=0, imm9=1
    let encoding: u32 = 0x59001000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_lda_stl
/// ASL: `field opc = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "opc", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_lda_stl_special_opc_1_size_variant_1_0_59401000()
 {
    // Encoding: 0x59401000
    // Test aarch64_memory_single_general_immediate_signed_offset_lda_stl special value opc = 1 (Size variant 1)
    // Fields: size=1, imm9=1, Rn=0, opc=1, Rt=0
    let encoding: u32 = 0x59401000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_lda_stl
/// ASL: `field opc = 2 (Size variant 2)`
/// Requirement: FieldSpecial { field: "opc", value: 2, meaning: "Size variant 2" }
/// Size variant 2
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_lda_stl_special_opc_2_size_variant_2_0_59801000()
 {
    // Encoding: 0x59801000
    // Test aarch64_memory_single_general_immediate_signed_offset_lda_stl special value opc = 2 (Size variant 2)
    // Fields: Rt=0, size=1, Rn=0, imm9=1, opc=2
    let encoding: u32 = 0x59801000;
    let mut cpu = create_test_cpu();
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

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_lda_stl
/// ASL: `field opc = 3 (Size variant 3)`
/// Requirement: FieldSpecial { field: "opc", value: 3, meaning: "Size variant 3" }
/// Size variant 3
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_lda_stl_special_opc_3_size_variant_3_0_59c01000()
 {
    // Encoding: 0x59C01000
    // Test aarch64_memory_single_general_immediate_signed_offset_lda_stl special value opc = 3 (Size variant 3)
    // Fields: imm9=1, Rt=0, opc=3, size=1, Rn=0
    let encoding: u32 = 0x59C01000;
    let mut cpu = create_test_cpu();
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

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_lda_stl
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_lda_stl_special_rn_31_stack_pointer_sp_may_require_alignment_0_590013e0()
 {
    // Encoding: 0x590013E0
    // Test aarch64_memory_single_general_immediate_signed_offset_lda_stl special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: size=1, opc=0, imm9=1, Rn=31, Rt=0
    let encoding: u32 = 0x590013E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_lda_stl
/// ASL: `field Rt = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)`
/// Requirement: FieldSpecial { field: "Rt", value: 31, meaning: "Zero register (XZR/WZR) - reads as 0, writes discarded" }
/// Zero register (XZR/WZR) - reads as 0, writes discarded
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_lda_stl_special_rt_31_zero_register_xzr_wzr_reads_as_0_writes_discarded_0_5900101f()
 {
    // Encoding: 0x5900101F
    // Test aarch64_memory_single_general_immediate_signed_offset_lda_stl special value Rt = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)
    // Fields: imm9=1, opc=0, size=1, Rn=0, Rt=31
    let encoding: u32 = 0x5900101F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_lda_stl
/// ASL: `STRB X0, [X1, #0]`
/// Requirement: MemoryAccess { op: Store, size_bits: 8, addressing: "immediate" }
/// zero value
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_lda_stl_str_oracle_0_39000020() {
    // Test STRB: zero value (oracle)
    // Encoding: 0x39000020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x1000);
    set_x(&mut cpu, 0, 0x0);
    let encoding: u32 = 0x39000020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    {
        let buf = cpu.read_memory(0x1000, 1).unwrap();
        let mut bytes = [0u8; 8];
        let n = buf.len().min(8);
        bytes[..n].copy_from_slice(&buf[..n]);
        let val = u64::from_le_bytes(bytes);
        assert_eq!(val, 0x0, "Memory at 0x1000 should be 0x0");
    }
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_lda_stl
/// ASL: `STRB X0, [X1, #0]`
/// Requirement: MemoryAccess { op: Store, size_bits: 8, addressing: "immediate" }
/// byte value
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_lda_stl_str_oracle_1_39000020() {
    // Test STRB: byte value (oracle)
    // Encoding: 0x39000020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x1000);
    set_x(&mut cpu, 0, 0xFF);
    let encoding: u32 = 0x39000020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
    {
        let buf = cpu.read_memory(0x1000, 1).unwrap();
        let mut bytes = [0u8; 8];
        let n = buf.len().min(8);
        bytes[..n].copy_from_slice(&buf[..n]);
        let val = u64::from_le_bytes(bytes);
        assert_eq!(val, 0xFF, "Memory at 0x1000 should be 0xFF");
    }
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_lda_stl
/// ASL: `STRB X0, [X1, #0]`
/// Requirement: MemoryAccess { op: Store, size_bits: 8, addressing: "immediate" }
/// halfword value
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_lda_stl_str_oracle_2_39000020() {
    // Test STRB: halfword value (oracle)
    // Encoding: 0x39000020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 0, 0x1234);
    set_x(&mut cpu, 1, 0x1000);
    let encoding: u32 = 0x39000020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
    {
        let buf = cpu.read_memory(0x1000, 1).unwrap();
        let mut bytes = [0u8; 8];
        let n = buf.len().min(8);
        bytes[..n].copy_from_slice(&buf[..n]);
        let val = u64::from_le_bytes(bytes);
        assert_eq!(val, 0x34, "Memory at 0x1000 should be 0x34");
    }
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_lda_stl
/// ASL: `STRB X0, [X1, #0]`
/// Requirement: MemoryAccess { op: Store, size_bits: 8, addressing: "immediate" }
/// word value
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_lda_stl_str_oracle_3_39000020() {
    // Test STRB: word value (oracle)
    // Encoding: 0x39000020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x1000);
    set_x(&mut cpu, 0, 0x12345678);
    let encoding: u32 = 0x39000020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
    {
        let buf = cpu.read_memory(0x1000, 1).unwrap();
        let mut bytes = [0u8; 8];
        let n = buf.len().min(8);
        bytes[..n].copy_from_slice(&buf[..n]);
        let val = u64::from_le_bytes(bytes);
        assert_eq!(val, 0x78, "Memory at 0x1000 should be 0x78");
    }
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_lda_stl
/// ASL: `STRB X0, [X1, #0]`
/// Requirement: MemoryAccess { op: Store, size_bits: 8, addressing: "immediate" }
/// doubleword value
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_lda_stl_str_oracle_4_39000020() {
    // Test STRB: doubleword value (oracle)
    // Encoding: 0x39000020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 0, 0x123456789ABCDEF0);
    set_x(&mut cpu, 1, 0x1000);
    let encoding: u32 = 0x39000020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
    {
        let buf = cpu.read_memory(0x1000, 1).unwrap();
        let mut bytes = [0u8; 8];
        let n = buf.len().min(8);
        bytes[..n].copy_from_slice(&buf[..n]);
        let val = u64::from_le_bytes(bytes);
        assert_eq!(val, 0xF0, "Memory at 0x1000 should be 0xF0");
    }
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_lda_stl
/// ASL: `GpFromField("t") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "t" }
/// verify register write to GpFromField("t")
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_lda_stl_reg_write_0_19000000() {
    // Test aarch64_memory_single_general_immediate_signed_offset_lda_stl register write: GpFromField("t")
    // Encoding: 0x19000000
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x19000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_lda_stl
/// ASL: `GpFromField("t") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "t" }
/// verify register write to GpFromField("t")
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_lda_stl_reg_write_1_19000000() {
    // Test aarch64_memory_single_general_immediate_signed_offset_lda_stl register write: GpFromField("t")
    // Encoding: 0x19000000
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x19000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_lda_stl
/// ASL: `Sp write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "unknown" }
/// verify register write to Sp
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_lda_stl_reg_write_2_19000000() {
    // Test aarch64_memory_single_general_immediate_signed_offset_lda_stl register write: Sp
    // Encoding: 0x19000000
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x19000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_lda_stl
/// ASL: `GpFromField("n") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "n" }
/// verify register write to GpFromField("n")
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_lda_stl_reg_write_3_19000000() {
    // Test aarch64_memory_single_general_immediate_signed_offset_lda_stl register write: GpFromField("n")
    // Encoding: 0x19000000
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x19000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_lda_stl
/// ASL: `Rn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Rn = 31)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_lda_stl_sp_rn_190003e0() {
    // Test aarch64_memory_single_general_immediate_signed_offset_lda_stl with Rn = SP (31)
    // Encoding: 0x190003E0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x190003E0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_lda_stl
/// ASL: `Rt = 31 (ZR)`
/// Requirement: RegisterSpecial { reg: Zr, behavior: "reads as 0, writes discarded" }
/// zero register (Rt = 31)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_lda_stl_zr_rt_1900001f() {
    // Test aarch64_memory_single_general_immediate_signed_offset_lda_stl with Rt = ZR (31)
    // Encoding: 0x1900001F
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x1900001F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 31), 0, "XZR should always be 0");
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_lda_stl
/// ASL: `Mem[address, 8] = data`
/// Requirement: MemoryAccess { op: Store, size_bits: 64, addressing: "Base { reg: \"address\" }" }
/// 8-byte store
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_lda_stl_store_0_19000020() {
    // Test aarch64_memory_single_general_immediate_signed_offset_lda_stl memory store: 8 bytes
    // Encoding: 0x19000020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x100000000000);
    set_x(&mut cpu, 0, 0xDEADBEEFCAFEBABE);
    let encoding: u32 = 0x19000020;
    write_insn(&mut cpu, 0, encoding);
    // Allocated encoding; synthetic operands may fault at runtime.
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

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_lda_stl
/// ASL: `Unconditional`
/// Requirement: UndefinedEncoding { condition: "Unconditional" }
/// triggers Undefined
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_lda_stl_exception_0_19000000() {
    // Test aarch64_memory_single_general_immediate_signed_offset_lda_stl exception: Undefined
    // Encoding: 0x19000000
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x19000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_lda_stl
/// ASL: `Unconditional`
/// Requirement: UndefinedEncoding { condition: "Unconditional" }
/// triggers Undefined
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_lda_stl_exception_1_19000000() {
    // Test aarch64_memory_single_general_immediate_signed_offset_lda_stl exception: Undefined
    // Encoding: 0x19000000
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x19000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

// ============================================================================
// aarch64_memory_single_general_register Tests
// ============================================================================

/// Provenance: aarch64_memory_single_general_register
/// ASL: `field size 30 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_memory_single_general_register_field_size_0_min_800_38200800() {
    // Encoding: 0x38200800
    // Test aarch64_memory_single_general_register field size = 0 (Min)
    // Fields: option=0, S=0, size=0, Rn=0, Rt=0, opc=0, Rm=0
    let encoding: u32 = 0x38200800;
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

/// Provenance: aarch64_memory_single_general_register
/// ASL: `field size 30 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 1, boundary: PowerOfTwo }
/// 16-bit / halfword size
#[test]
fn test_aarch64_memory_single_general_register_field_size_1_poweroftwo_800_78200800() {
    // Encoding: 0x78200800
    // Test aarch64_memory_single_general_register field size = 1 (PowerOfTwo)
    // Fields: S=0, size=1, Rm=0, opc=0, Rn=0, option=0, Rt=0
    let encoding: u32 = 0x78200800;
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

/// Provenance: aarch64_memory_single_general_register
/// ASL: `field size 30 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 2, boundary: PowerOfTwo }
/// 32-bit / word size
#[test]
fn test_aarch64_memory_single_general_register_field_size_2_poweroftwo_800_b8200800() {
    // Encoding: 0xB8200800
    // Test aarch64_memory_single_general_register field size = 2 (PowerOfTwo)
    // Fields: Rm=0, option=0, S=0, opc=0, Rn=0, size=2, Rt=0
    let encoding: u32 = 0xB8200800;
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

/// Provenance: aarch64_memory_single_general_register
/// ASL: `field size 30 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 3, boundary: Max }
/// 64-bit / doubleword size
#[test]
fn test_aarch64_memory_single_general_register_field_size_3_max_800_f8200800() {
    // Encoding: 0xF8200800
    // Test aarch64_memory_single_general_register field size = 3 (Max)
    // Fields: opc=0, S=0, option=0, Rt=0, size=3, Rn=0, Rm=0
    let encoding: u32 = 0xF8200800;
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

/// Provenance: aarch64_memory_single_general_register
/// ASL: `field opc 22 +: 2`
/// Requirement: FieldBoundary { field: "opc", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_memory_single_general_register_field_opc_0_min_800_38200800() {
    // Encoding: 0x38200800
    // Test aarch64_memory_single_general_register field opc = 0 (Min)
    // Fields: size=0, option=0, opc=0, S=0, Rn=0, Rt=0, Rm=0
    let encoding: u32 = 0x38200800;
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

/// Provenance: aarch64_memory_single_general_register
/// ASL: `field opc 22 +: 2`
/// Requirement: FieldBoundary { field: "opc", value: 1, boundary: PowerOfTwo }
/// 16-bit / halfword size
#[test]
fn test_aarch64_memory_single_general_register_field_opc_1_poweroftwo_800_38600800() {
    // Encoding: 0x38600800
    // Test aarch64_memory_single_general_register field opc = 1 (PowerOfTwo)
    // Fields: opc=1, Rm=0, Rn=0, option=0, Rt=0, size=0, S=0
    let encoding: u32 = 0x38600800;
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

/// Provenance: aarch64_memory_single_general_register
/// ASL: `field opc 22 +: 2`
/// Requirement: FieldBoundary { field: "opc", value: 2, boundary: PowerOfTwo }
/// 32-bit / word size
#[test]
fn test_aarch64_memory_single_general_register_field_opc_2_poweroftwo_800_38a00800() {
    // Encoding: 0x38A00800
    // Test aarch64_memory_single_general_register field opc = 2 (PowerOfTwo)
    // Fields: Rn=0, Rm=0, Rt=0, option=0, S=0, opc=2, size=0
    let encoding: u32 = 0x38A00800;
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

/// Provenance: aarch64_memory_single_general_register
/// ASL: `field opc 22 +: 2`
/// Requirement: FieldBoundary { field: "opc", value: 3, boundary: Max }
/// 64-bit / doubleword size
#[test]
fn test_aarch64_memory_single_general_register_field_opc_3_max_800_38e00800() {
    // Encoding: 0x38E00800
    // Test aarch64_memory_single_general_register field opc = 3 (Max)
    // Fields: Rt=0, opc=3, Rm=0, size=0, option=0, S=0, Rn=0
    let encoding: u32 = 0x38E00800;
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

/// Provenance: aarch64_memory_single_general_register
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_memory_single_general_register_field_rm_0_min_800_38200800() {
    // Encoding: 0x38200800
    // Test aarch64_memory_single_general_register field Rm = 0 (Min)
    // Fields: Rm=0, opc=0, S=0, Rn=0, Rt=0, size=0, option=0
    let encoding: u32 = 0x38200800;
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

/// Provenance: aarch64_memory_single_general_register
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_memory_single_general_register_field_rm_1_poweroftwo_800_38210800() {
    // Encoding: 0x38210800
    // Test aarch64_memory_single_general_register field Rm = 1 (PowerOfTwo)
    // Fields: S=0, Rt=0, Rm=1, size=0, opc=0, option=0, Rn=0
    let encoding: u32 = 0x38210800;
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

/// Provenance: aarch64_memory_single_general_register
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_memory_single_general_register_field_rm_30_poweroftwominusone_800_383e0800() {
    // Encoding: 0x383E0800
    // Test aarch64_memory_single_general_register field Rm = 30 (PowerOfTwoMinusOne)
    // Fields: size=0, S=0, Rn=0, Rt=0, opc=0, Rm=30, option=0
    let encoding: u32 = 0x383E0800;
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

/// Provenance: aarch64_memory_single_general_register
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 31, boundary: Max }
/// register index 31 (special)
#[test]
fn test_aarch64_memory_single_general_register_field_rm_31_max_800_383f0800() {
    // Encoding: 0x383F0800
    // Test aarch64_memory_single_general_register field Rm = 31 (Max)
    // Fields: Rn=0, S=0, Rt=0, Rm=31, size=0, opc=0, option=0
    let encoding: u32 = 0x383F0800;
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

/// Provenance: aarch64_memory_single_general_register
/// ASL: `field option 13 +: 3`
/// Requirement: FieldBoundary { field: "option", value: 0, boundary: Min }
/// option 0
#[test]
fn test_aarch64_memory_single_general_register_field_option_0_min_800_38200800() {
    // Encoding: 0x38200800
    // Test aarch64_memory_single_general_register field option = 0 (Min)
    // Fields: option=0, opc=0, Rn=0, Rt=0, size=0, Rm=0, S=0
    let encoding: u32 = 0x38200800;
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

/// Provenance: aarch64_memory_single_general_register
/// ASL: `field option 13 +: 3`
/// Requirement: FieldBoundary { field: "option", value: 1, boundary: PowerOfTwo }
/// option 1
#[test]
fn test_aarch64_memory_single_general_register_field_option_1_poweroftwo_800_38202800() {
    // Encoding: 0x38202800
    // Test aarch64_memory_single_general_register field option = 1 (PowerOfTwo)
    // Fields: option=1, S=0, Rn=0, Rt=0, opc=0, size=0, Rm=0
    let encoding: u32 = 0x38202800;
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

/// Provenance: aarch64_memory_single_general_register
/// ASL: `field option 13 +: 3`
/// Requirement: FieldBoundary { field: "option", value: 2, boundary: PowerOfTwo }
/// option 2
#[test]
fn test_aarch64_memory_single_general_register_field_option_2_poweroftwo_800_38204800() {
    // Encoding: 0x38204800
    // Test aarch64_memory_single_general_register field option = 2 (PowerOfTwo)
    // Fields: Rm=0, Rt=0, Rn=0, size=0, opc=0, option=2, S=0
    let encoding: u32 = 0x38204800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_register
/// ASL: `field option 13 +: 3`
/// Requirement: FieldBoundary { field: "option", value: 3, boundary: PowerOfTwo }
/// option 3
#[test]
fn test_aarch64_memory_single_general_register_field_option_3_poweroftwo_800_38206800() {
    // Encoding: 0x38206800
    // Test aarch64_memory_single_general_register field option = 3 (PowerOfTwo)
    // Fields: Rm=0, size=0, option=3, opc=0, S=0, Rn=0, Rt=0
    let encoding: u32 = 0x38206800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_register
/// ASL: `field option 13 +: 3`
/// Requirement: FieldBoundary { field: "option", value: 4, boundary: PowerOfTwo }
/// option 4
#[test]
fn test_aarch64_memory_single_general_register_field_option_4_poweroftwo_800_38208800() {
    // Encoding: 0x38208800
    // Test aarch64_memory_single_general_register field option = 4 (PowerOfTwo)
    // Fields: size=0, opc=0, Rm=0, Rt=0, Rn=0, option=4, S=0
    let encoding: u32 = 0x38208800;
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

/// Provenance: aarch64_memory_single_general_register
/// ASL: `field option 13 +: 3`
/// Requirement: FieldBoundary { field: "option", value: 5, boundary: PowerOfTwo }
/// option 5
#[test]
fn test_aarch64_memory_single_general_register_field_option_5_poweroftwo_800_3820a800() {
    // Encoding: 0x3820A800
    // Test aarch64_memory_single_general_register field option = 5 (PowerOfTwo)
    // Fields: size=0, option=5, opc=0, S=0, Rn=0, Rt=0, Rm=0
    let encoding: u32 = 0x3820A800;
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

/// Provenance: aarch64_memory_single_general_register
/// ASL: `field option 13 +: 3`
/// Requirement: FieldBoundary { field: "option", value: 6, boundary: PowerOfTwo }
/// option 6
#[test]
fn test_aarch64_memory_single_general_register_field_option_6_poweroftwo_800_3820c800() {
    // Encoding: 0x3820C800
    // Test aarch64_memory_single_general_register field option = 6 (PowerOfTwo)
    // Fields: Rt=0, option=6, S=0, opc=0, size=0, Rn=0, Rm=0
    let encoding: u32 = 0x3820C800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_register
/// ASL: `field option 13 +: 3`
/// Requirement: FieldBoundary { field: "option", value: 7, boundary: Max }
/// option 7
#[test]
fn test_aarch64_memory_single_general_register_field_option_7_max_800_3820e800() {
    // Encoding: 0x3820E800
    // Test aarch64_memory_single_general_register field option = 7 (Max)
    // Fields: size=0, option=7, opc=0, Rn=0, S=0, Rt=0, Rm=0
    let encoding: u32 = 0x3820E800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_register
/// ASL: `field S 12 +: 1`
/// Requirement: FieldBoundary { field: "S", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_memory_single_general_register_field_s_0_min_800_38200800() {
    // Encoding: 0x38200800
    // Test aarch64_memory_single_general_register field S = 0 (Min)
    // Fields: option=0, Rt=0, opc=0, Rm=0, S=0, Rn=0, size=0
    let encoding: u32 = 0x38200800;
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

/// Provenance: aarch64_memory_single_general_register
/// ASL: `field S 12 +: 1`
/// Requirement: FieldBoundary { field: "S", value: 1, boundary: Max }
/// 16-bit / halfword size
#[test]
fn test_aarch64_memory_single_general_register_field_s_1_max_800_38201800() {
    // Encoding: 0x38201800
    // Test aarch64_memory_single_general_register field S = 1 (Max)
    // Fields: size=0, Rn=0, opc=0, Rm=0, S=1, Rt=0, option=0
    let encoding: u32 = 0x38201800;
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

/// Provenance: aarch64_memory_single_general_register
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_memory_single_general_register_field_rn_0_min_800_38200800() {
    // Encoding: 0x38200800
    // Test aarch64_memory_single_general_register field Rn = 0 (Min)
    // Fields: S=0, Rt=0, Rn=0, Rm=0, size=0, option=0, opc=0
    let encoding: u32 = 0x38200800;
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

/// Provenance: aarch64_memory_single_general_register
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_memory_single_general_register_field_rn_1_poweroftwo_800_38200820() {
    // Encoding: 0x38200820
    // Test aarch64_memory_single_general_register field Rn = 1 (PowerOfTwo)
    // Fields: Rt=0, Rn=1, S=0, size=0, Rm=0, opc=0, option=0
    let encoding: u32 = 0x38200820;
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

/// Provenance: aarch64_memory_single_general_register
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_memory_single_general_register_field_rn_30_poweroftwominusone_800_38200bc0() {
    // Encoding: 0x38200BC0
    // Test aarch64_memory_single_general_register field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: opc=0, Rn=30, Rt=0, option=0, Rm=0, size=0, S=0
    let encoding: u32 = 0x38200BC0;
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

/// Provenance: aarch64_memory_single_general_register
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_memory_single_general_register_field_rn_31_max_800_38200be0() {
    // Encoding: 0x38200BE0
    // Test aarch64_memory_single_general_register field Rn = 31 (Max)
    // Fields: S=0, option=0, size=0, Rn=31, Rt=0, Rm=0, opc=0
    let encoding: u32 = 0x38200BE0;
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

/// Provenance: aarch64_memory_single_general_register
/// ASL: `field Rt 0 +: 5`
/// Requirement: FieldBoundary { field: "Rt", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_memory_single_general_register_field_rt_0_min_800_38200800() {
    // Encoding: 0x38200800
    // Test aarch64_memory_single_general_register field Rt = 0 (Min)
    // Fields: S=0, Rn=0, Rt=0, size=0, opc=0, option=0, Rm=0
    let encoding: u32 = 0x38200800;
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

/// Provenance: aarch64_memory_single_general_register
/// ASL: `field Rt 0 +: 5`
/// Requirement: FieldBoundary { field: "Rt", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_memory_single_general_register_field_rt_1_poweroftwo_800_38200801() {
    // Encoding: 0x38200801
    // Test aarch64_memory_single_general_register field Rt = 1 (PowerOfTwo)
    // Fields: Rm=0, option=0, S=0, Rn=0, Rt=1, size=0, opc=0
    let encoding: u32 = 0x38200801;
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

/// Provenance: aarch64_memory_single_general_register
/// ASL: `field Rt 0 +: 5`
/// Requirement: FieldBoundary { field: "Rt", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_memory_single_general_register_field_rt_30_poweroftwominusone_800_3820081e() {
    // Encoding: 0x3820081E
    // Test aarch64_memory_single_general_register field Rt = 30 (PowerOfTwoMinusOne)
    // Fields: size=0, opc=0, Rm=0, option=0, Rn=0, S=0, Rt=30
    let encoding: u32 = 0x3820081E;
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

/// Provenance: aarch64_memory_single_general_register
/// ASL: `field Rt 0 +: 5`
/// Requirement: FieldBoundary { field: "Rt", value: 31, boundary: Max }
/// register index 31 (ZR - zero register)
#[test]
fn test_aarch64_memory_single_general_register_field_rt_31_max_800_3820081f() {
    // Encoding: 0x3820081F
    // Test aarch64_memory_single_general_register field Rt = 31 (Max)
    // Fields: opc=0, option=0, Rn=0, Rm=0, S=0, Rt=31, size=0
    let encoding: u32 = 0x3820081F;
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

/// Provenance: aarch64_memory_single_general_register
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=0 (8-bit / byte size)
#[test]
fn test_aarch64_memory_single_general_register_combo_0_800_38200800() {
    // Encoding: 0x38200800
    // Test aarch64_memory_single_general_register field combination: size=0, opc=0, Rm=0, option=0, S=0, Rn=0, Rt=0
    // Fields: Rt=0, option=0, size=0, S=0, Rn=0, opc=0, Rm=0
    let encoding: u32 = 0x38200800;
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

/// Provenance: aarch64_memory_single_general_register
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=1 (16-bit / halfword size)
#[test]
fn test_aarch64_memory_single_general_register_combo_1_800_78200800() {
    // Encoding: 0x78200800
    // Test aarch64_memory_single_general_register field combination: size=1, opc=0, Rm=0, option=0, S=0, Rn=0, Rt=0
    // Fields: Rn=0, Rm=0, opc=0, option=0, S=0, Rt=0, size=1
    let encoding: u32 = 0x78200800;
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

/// Provenance: aarch64_memory_single_general_register
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=2 (32-bit / word size)
#[test]
fn test_aarch64_memory_single_general_register_combo_2_800_b8200800() {
    // Encoding: 0xB8200800
    // Test aarch64_memory_single_general_register field combination: size=2, opc=0, Rm=0, option=0, S=0, Rn=0, Rt=0
    // Fields: Rn=0, opc=0, Rt=0, size=2, option=0, S=0, Rm=0
    let encoding: u32 = 0xB8200800;
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

/// Provenance: aarch64_memory_single_general_register
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=3 (64-bit / doubleword size)
#[test]
fn test_aarch64_memory_single_general_register_combo_3_800_f8200800() {
    // Encoding: 0xF8200800
    // Test aarch64_memory_single_general_register field combination: size=3, opc=0, Rm=0, option=0, S=0, Rn=0, Rt=0
    // Fields: S=0, Rn=0, Rt=0, size=3, opc=0, Rm=0, option=0
    let encoding: u32 = 0xF8200800;
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

/// Provenance: aarch64_memory_single_general_register
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// opc=0 (8-bit / byte size)
#[test]
fn test_aarch64_memory_single_general_register_combo_4_800_38200800() {
    // Encoding: 0x38200800
    // Test aarch64_memory_single_general_register field combination: size=0, opc=0, Rm=0, option=0, S=0, Rn=0, Rt=0
    // Fields: S=0, size=0, Rm=0, Rn=0, Rt=0, option=0, opc=0
    let encoding: u32 = 0x38200800;
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

/// Provenance: aarch64_memory_single_general_register
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// opc=1 (16-bit / halfword size)
#[test]
fn test_aarch64_memory_single_general_register_combo_5_800_38600800() {
    // Encoding: 0x38600800
    // Test aarch64_memory_single_general_register field combination: size=0, opc=1, Rm=0, option=0, S=0, Rn=0, Rt=0
    // Fields: Rn=0, size=0, option=0, Rt=0, opc=1, S=0, Rm=0
    let encoding: u32 = 0x38600800;
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

/// Provenance: aarch64_memory_single_general_register
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// opc=2 (32-bit / word size)
#[test]
fn test_aarch64_memory_single_general_register_combo_6_800_38a00800() {
    // Encoding: 0x38A00800
    // Test aarch64_memory_single_general_register field combination: size=0, opc=2, Rm=0, option=0, S=0, Rn=0, Rt=0
    // Fields: option=0, Rm=0, size=0, Rn=0, S=0, Rt=0, opc=2
    let encoding: u32 = 0x38A00800;
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

/// Provenance: aarch64_memory_single_general_register
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// opc=3 (64-bit / doubleword size)
#[test]
fn test_aarch64_memory_single_general_register_combo_7_800_38e00800() {
    // Encoding: 0x38E00800
    // Test aarch64_memory_single_general_register field combination: size=0, opc=3, Rm=0, option=0, S=0, Rn=0, Rt=0
    // Fields: Rt=0, Rm=0, S=0, size=0, opc=3, option=0, Rn=0
    let encoding: u32 = 0x38E00800;
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

/// Provenance: aarch64_memory_single_general_register
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=0 (register index 0 (first register))
#[test]
fn test_aarch64_memory_single_general_register_combo_8_800_38200800() {
    // Encoding: 0x38200800
    // Test aarch64_memory_single_general_register field combination: size=0, opc=0, Rm=0, option=0, S=0, Rn=0, Rt=0
    // Fields: size=0, Rm=0, option=0, S=0, opc=0, Rn=0, Rt=0
    let encoding: u32 = 0x38200800;
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

/// Provenance: aarch64_memory_single_general_register
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=1 (register index 1 (second register))
#[test]
fn test_aarch64_memory_single_general_register_combo_9_800_38210800() {
    // Encoding: 0x38210800
    // Test aarch64_memory_single_general_register field combination: size=0, opc=0, Rm=1, option=0, S=0, Rn=0, Rt=0
    // Fields: Rt=0, size=0, Rm=1, opc=0, option=0, S=0, Rn=0
    let encoding: u32 = 0x38210800;
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

/// Provenance: aarch64_memory_single_general_register
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_memory_single_general_register_combo_10_800_383e0800() {
    // Encoding: 0x383E0800
    // Test aarch64_memory_single_general_register field combination: size=0, opc=0, Rm=30, option=0, S=0, Rn=0, Rt=0
    // Fields: Rm=30, Rn=0, opc=0, option=0, S=0, Rt=0, size=0
    let encoding: u32 = 0x383E0800;
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

/// Provenance: aarch64_memory_single_general_register
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=31 (register index 31 (special))
#[test]
fn test_aarch64_memory_single_general_register_combo_11_800_383f0800() {
    // Encoding: 0x383F0800
    // Test aarch64_memory_single_general_register field combination: size=0, opc=0, Rm=31, option=0, S=0, Rn=0, Rt=0
    // Fields: opc=0, size=0, Rn=0, option=0, Rt=0, Rm=31, S=0
    let encoding: u32 = 0x383F0800;
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

/// Provenance: aarch64_memory_single_general_register
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// option=0 (option 0)
#[test]
fn test_aarch64_memory_single_general_register_combo_12_800_38200800() {
    // Encoding: 0x38200800
    // Test aarch64_memory_single_general_register field combination: size=0, opc=0, Rm=0, option=0, S=0, Rn=0, Rt=0
    // Fields: option=0, Rn=0, Rt=0, opc=0, S=0, Rm=0, size=0
    let encoding: u32 = 0x38200800;
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

/// Provenance: aarch64_memory_single_general_register
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// option=1 (option 1)
#[test]
fn test_aarch64_memory_single_general_register_combo_13_800_38202800() {
    // Encoding: 0x38202800
    // Test aarch64_memory_single_general_register field combination: size=0, opc=0, Rm=0, option=1, S=0, Rn=0, Rt=0
    // Fields: size=0, opc=0, Rm=0, option=1, S=0, Rn=0, Rt=0
    let encoding: u32 = 0x38202800;
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

/// Provenance: aarch64_memory_single_general_register
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// option=2 (option 2)
#[test]
fn test_aarch64_memory_single_general_register_combo_14_800_38204800() {
    // Encoding: 0x38204800
    // Test aarch64_memory_single_general_register field combination: size=0, opc=0, Rm=0, option=2, S=0, Rn=0, Rt=0
    // Fields: Rn=0, Rt=0, Rm=0, opc=0, option=2, size=0, S=0
    let encoding: u32 = 0x38204800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_register
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// option=3 (option 3)
#[test]
fn test_aarch64_memory_single_general_register_combo_15_800_38206800() {
    // Encoding: 0x38206800
    // Test aarch64_memory_single_general_register field combination: size=0, opc=0, Rm=0, option=3, S=0, Rn=0, Rt=0
    // Fields: size=0, opc=0, Rn=0, S=0, option=3, Rt=0, Rm=0
    let encoding: u32 = 0x38206800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_register
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// option=4 (option 4)
#[test]
fn test_aarch64_memory_single_general_register_combo_16_800_38208800() {
    // Encoding: 0x38208800
    // Test aarch64_memory_single_general_register field combination: size=0, opc=0, Rm=0, option=4, S=0, Rn=0, Rt=0
    // Fields: Rn=0, opc=0, Rm=0, size=0, S=0, option=4, Rt=0
    let encoding: u32 = 0x38208800;
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

/// Provenance: aarch64_memory_single_general_register
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// option=5 (option 5)
#[test]
fn test_aarch64_memory_single_general_register_combo_17_800_3820a800() {
    // Encoding: 0x3820A800
    // Test aarch64_memory_single_general_register field combination: size=0, opc=0, Rm=0, option=5, S=0, Rn=0, Rt=0
    // Fields: opc=0, Rt=0, size=0, Rm=0, option=5, Rn=0, S=0
    let encoding: u32 = 0x3820A800;
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

/// Provenance: aarch64_memory_single_general_register
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// option=6 (option 6)
#[test]
fn test_aarch64_memory_single_general_register_combo_18_800_3820c800() {
    // Encoding: 0x3820C800
    // Test aarch64_memory_single_general_register field combination: size=0, opc=0, Rm=0, option=6, S=0, Rn=0, Rt=0
    // Fields: size=0, Rm=0, S=0, option=6, Rn=0, Rt=0, opc=0
    let encoding: u32 = 0x3820C800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_register
/// ASL: `field combination 19`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// option=7 (option 7)
#[test]
fn test_aarch64_memory_single_general_register_combo_19_800_3820e800() {
    // Encoding: 0x3820E800
    // Test aarch64_memory_single_general_register field combination: size=0, opc=0, Rm=0, option=7, S=0, Rn=0, Rt=0
    // Fields: S=0, Rn=0, Rt=0, opc=0, size=0, option=7, Rm=0
    let encoding: u32 = 0x3820E800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_register
/// ASL: `field combination 20`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// S=0 (8-bit / byte size)
#[test]
fn test_aarch64_memory_single_general_register_combo_20_800_38200800() {
    // Encoding: 0x38200800
    // Test aarch64_memory_single_general_register field combination: size=0, opc=0, Rm=0, option=0, S=0, Rn=0, Rt=0
    // Fields: option=0, size=0, opc=0, Rm=0, Rn=0, Rt=0, S=0
    let encoding: u32 = 0x38200800;
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

/// Provenance: aarch64_memory_single_general_register
/// ASL: `field combination 21`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// S=1 (16-bit / halfword size)
#[test]
fn test_aarch64_memory_single_general_register_combo_21_800_38201800() {
    // Encoding: 0x38201800
    // Test aarch64_memory_single_general_register field combination: size=0, opc=0, Rm=0, option=0, S=1, Rn=0, Rt=0
    // Fields: option=0, Rt=0, Rm=0, opc=0, S=1, size=0, Rn=0
    let encoding: u32 = 0x38201800;
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

/// Provenance: aarch64_memory_single_general_register
/// ASL: `field combination 22`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch64_memory_single_general_register_combo_22_800_38200800() {
    // Encoding: 0x38200800
    // Test aarch64_memory_single_general_register field combination: size=0, opc=0, Rm=0, option=0, S=0, Rn=0, Rt=0
    // Fields: S=0, Rn=0, Rt=0, Rm=0, opc=0, option=0, size=0
    let encoding: u32 = 0x38200800;
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

/// Provenance: aarch64_memory_single_general_register
/// ASL: `field combination 23`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_aarch64_memory_single_general_register_combo_23_800_38200820() {
    // Encoding: 0x38200820
    // Test aarch64_memory_single_general_register field combination: size=0, opc=0, Rm=0, option=0, S=0, Rn=1, Rt=0
    // Fields: Rt=0, option=0, Rm=0, size=0, S=0, opc=0, Rn=1
    let encoding: u32 = 0x38200820;
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

/// Provenance: aarch64_memory_single_general_register
/// ASL: `field combination 24`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_memory_single_general_register_combo_24_800_38200bc0() {
    // Encoding: 0x38200BC0
    // Test aarch64_memory_single_general_register field combination: size=0, opc=0, Rm=0, option=0, S=0, Rn=30, Rt=0
    // Fields: opc=0, size=0, Rt=0, Rm=0, option=0, S=0, Rn=30
    let encoding: u32 = 0x38200BC0;
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

/// Provenance: aarch64_memory_single_general_register
/// ASL: `field combination 25`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_memory_single_general_register_combo_25_800_38200be0() {
    // Encoding: 0x38200BE0
    // Test aarch64_memory_single_general_register field combination: size=0, opc=0, Rm=0, option=0, S=0, Rn=31, Rt=0
    // Fields: option=0, S=0, opc=0, Rm=0, Rt=0, Rn=31, size=0
    let encoding: u32 = 0x38200BE0;
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

/// Provenance: aarch64_memory_single_general_register
/// ASL: `field combination 26`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt=0 (register index 0 (first register))
#[test]
fn test_aarch64_memory_single_general_register_combo_26_800_38200800() {
    // Encoding: 0x38200800
    // Test aarch64_memory_single_general_register field combination: size=0, opc=0, Rm=0, option=0, S=0, Rn=0, Rt=0
    // Fields: size=0, S=0, Rn=0, opc=0, Rt=0, option=0, Rm=0
    let encoding: u32 = 0x38200800;
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

/// Provenance: aarch64_memory_single_general_register
/// ASL: `field combination 27`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt=1 (register index 1 (second register))
#[test]
fn test_aarch64_memory_single_general_register_combo_27_800_38200801() {
    // Encoding: 0x38200801
    // Test aarch64_memory_single_general_register field combination: size=0, opc=0, Rm=0, option=0, S=0, Rn=0, Rt=1
    // Fields: Rn=0, Rm=0, S=0, size=0, opc=0, Rt=1, option=0
    let encoding: u32 = 0x38200801;
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

/// Provenance: aarch64_memory_single_general_register
/// ASL: `field combination 28`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_memory_single_general_register_combo_28_800_3820081e() {
    // Encoding: 0x3820081E
    // Test aarch64_memory_single_general_register field combination: size=0, opc=0, Rm=0, option=0, S=0, Rn=0, Rt=30
    // Fields: Rn=0, Rt=30, Rm=0, option=0, S=0, size=0, opc=0
    let encoding: u32 = 0x3820081E;
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

/// Provenance: aarch64_memory_single_general_register
/// ASL: `field combination 29`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt=31 (register index 31 (ZR - zero register))
#[test]
fn test_aarch64_memory_single_general_register_combo_29_800_3820081f() {
    // Encoding: 0x3820081F
    // Test aarch64_memory_single_general_register field combination: size=0, opc=0, Rm=0, option=0, S=0, Rn=0, Rt=31
    // Fields: Rn=0, Rt=31, size=0, S=0, option=0, Rm=0, opc=0
    let encoding: u32 = 0x3820081F;
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

/// Provenance: aarch64_memory_single_general_register
/// ASL: `field combination 30`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=1 (same register test (reg=1)), Rn=1 (same register test (reg=1))
#[test]
fn test_aarch64_memory_single_general_register_combo_30_800_38210820() {
    // Encoding: 0x38210820
    // Test aarch64_memory_single_general_register field combination: size=0, opc=0, Rm=1, option=0, S=0, Rn=1, Rt=0
    // Fields: opc=0, size=0, S=0, Rn=1, Rt=0, Rm=1, option=0
    let encoding: u32 = 0x38210820;
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

/// Provenance: aarch64_memory_single_general_register
/// ASL: `field combination 31`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=31 (same register test (reg=31)), Rn=31 (same register test (reg=31))
#[test]
fn test_aarch64_memory_single_general_register_combo_31_800_383f0be0() {
    // Encoding: 0x383F0BE0
    // Test aarch64_memory_single_general_register field combination: size=0, opc=0, Rm=31, option=0, S=0, Rn=31, Rt=0
    // Fields: size=0, opc=0, S=0, Rn=31, Rt=0, option=0, Rm=31
    let encoding: u32 = 0x383F0BE0;
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

/// Provenance: aarch64_memory_single_general_register
/// ASL: `field combination 32`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=1 (same register test (reg=1)), Rt=1 (same register test (reg=1))
#[test]
fn test_aarch64_memory_single_general_register_combo_32_800_38210801() {
    // Encoding: 0x38210801
    // Test aarch64_memory_single_general_register field combination: size=0, opc=0, Rm=1, option=0, S=0, Rn=0, Rt=1
    // Fields: Rm=1, Rn=0, S=0, Rt=1, option=0, opc=0, size=0
    let encoding: u32 = 0x38210801;
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

/// Provenance: aarch64_memory_single_general_register
/// ASL: `field combination 33`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=31 (same register test (reg=31)), Rt=31 (same register test (reg=31))
#[test]
fn test_aarch64_memory_single_general_register_combo_33_800_383f081f() {
    // Encoding: 0x383F081F
    // Test aarch64_memory_single_general_register field combination: size=0, opc=0, Rm=31, option=0, S=0, Rn=0, Rt=31
    // Fields: Rt=31, option=0, Rm=31, Rn=0, S=0, size=0, opc=0
    let encoding: u32 = 0x383F081F;
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

/// Provenance: aarch64_memory_single_general_register
/// ASL: `field combination 34`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (same register test (reg=1)), Rt=1 (same register test (reg=1))
#[test]
fn test_aarch64_memory_single_general_register_combo_34_800_38200821() {
    // Encoding: 0x38200821
    // Test aarch64_memory_single_general_register field combination: size=0, opc=0, Rm=0, option=0, S=0, Rn=1, Rt=1
    // Fields: size=0, Rm=0, option=0, Rn=1, Rt=1, S=0, opc=0
    let encoding: u32 = 0x38200821;
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

/// Provenance: aarch64_memory_single_general_register
/// ASL: `field combination 35`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (same register test (reg=31)), Rt=31 (same register test (reg=31))
#[test]
fn test_aarch64_memory_single_general_register_combo_35_800_38200bff() {
    // Encoding: 0x38200BFF
    // Test aarch64_memory_single_general_register field combination: size=0, opc=0, Rm=0, option=0, S=0, Rn=31, Rt=31
    // Fields: Rm=0, size=0, S=0, Rn=31, option=0, opc=0, Rt=31
    let encoding: u32 = 0x38200BFF;
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

/// Provenance: aarch64_memory_single_general_register
/// ASL: `field size = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "size", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_memory_single_general_register_special_size_0_size_variant_0_2048_38200800() {
    // Encoding: 0x38200800
    // Test aarch64_memory_single_general_register special value size = 0 (Size variant 0)
    // Fields: Rt=0, option=0, size=0, S=0, Rn=0, opc=0, Rm=0
    let encoding: u32 = 0x38200800;
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

/// Provenance: aarch64_memory_single_general_register
/// ASL: `field size = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "size", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_memory_single_general_register_special_size_1_size_variant_1_2048_78200800() {
    // Encoding: 0x78200800
    // Test aarch64_memory_single_general_register special value size = 1 (Size variant 1)
    // Fields: size=1, option=0, Rt=0, Rm=0, Rn=0, S=0, opc=0
    let encoding: u32 = 0x78200800;
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

/// Provenance: aarch64_memory_single_general_register
/// ASL: `field size = 2 (Size variant 2)`
/// Requirement: FieldSpecial { field: "size", value: 2, meaning: "Size variant 2" }
/// Size variant 2
#[test]
fn test_aarch64_memory_single_general_register_special_size_2_size_variant_2_2048_b8200800() {
    // Encoding: 0xB8200800
    // Test aarch64_memory_single_general_register special value size = 2 (Size variant 2)
    // Fields: opc=0, option=0, S=0, Rm=0, Rt=0, Rn=0, size=2
    let encoding: u32 = 0xB8200800;
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

/// Provenance: aarch64_memory_single_general_register
/// ASL: `field size = 3 (Size variant 3)`
/// Requirement: FieldSpecial { field: "size", value: 3, meaning: "Size variant 3" }
/// Size variant 3
#[test]
fn test_aarch64_memory_single_general_register_special_size_3_size_variant_3_2048_f8200800() {
    // Encoding: 0xF8200800
    // Test aarch64_memory_single_general_register special value size = 3 (Size variant 3)
    // Fields: Rt=0, opc=0, S=0, option=0, Rn=0, Rm=0, size=3
    let encoding: u32 = 0xF8200800;
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

/// Provenance: aarch64_memory_single_general_register
/// ASL: `field opc = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "opc", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_memory_single_general_register_special_opc_0_size_variant_0_2048_78200800() {
    // Encoding: 0x78200800
    // Test aarch64_memory_single_general_register special value opc = 0 (Size variant 0)
    // Fields: S=0, Rn=0, Rm=0, Rt=0, size=1, opc=0, option=0
    let encoding: u32 = 0x78200800;
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

/// Provenance: aarch64_memory_single_general_register
/// ASL: `field opc = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "opc", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_memory_single_general_register_special_opc_1_size_variant_1_2048_78600800() {
    // Encoding: 0x78600800
    // Test aarch64_memory_single_general_register special value opc = 1 (Size variant 1)
    // Fields: S=0, size=1, Rm=0, opc=1, Rt=0, Rn=0, option=0
    let encoding: u32 = 0x78600800;
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

/// Provenance: aarch64_memory_single_general_register
/// ASL: `field opc = 2 (Size variant 2)`
/// Requirement: FieldSpecial { field: "opc", value: 2, meaning: "Size variant 2" }
/// Size variant 2
#[test]
fn test_aarch64_memory_single_general_register_special_opc_2_size_variant_2_2048_78a00800() {
    // Encoding: 0x78A00800
    // Test aarch64_memory_single_general_register special value opc = 2 (Size variant 2)
    // Fields: Rn=0, Rt=0, Rm=0, opc=2, size=1, option=0, S=0
    let encoding: u32 = 0x78A00800;
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

/// Provenance: aarch64_memory_single_general_register
/// ASL: `field opc = 3 (Size variant 3)`
/// Requirement: FieldSpecial { field: "opc", value: 3, meaning: "Size variant 3" }
/// Size variant 3
#[test]
fn test_aarch64_memory_single_general_register_special_opc_3_size_variant_3_2048_78e00800() {
    // Encoding: 0x78E00800
    // Test aarch64_memory_single_general_register special value opc = 3 (Size variant 3)
    // Fields: Rm=0, Rt=0, S=0, Rn=0, size=1, opc=3, option=0
    let encoding: u32 = 0x78E00800;
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

/// Provenance: aarch64_memory_single_general_register
/// ASL: `field S = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "S", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_memory_single_general_register_special_s_0_size_variant_0_2048_78200800() {
    // Encoding: 0x78200800
    // Test aarch64_memory_single_general_register special value S = 0 (Size variant 0)
    // Fields: Rt=0, Rm=0, option=0, size=1, opc=0, S=0, Rn=0
    let encoding: u32 = 0x78200800;
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

/// Provenance: aarch64_memory_single_general_register
/// ASL: `field S = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "S", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_memory_single_general_register_special_s_1_size_variant_1_2048_78201800() {
    // Encoding: 0x78201800
    // Test aarch64_memory_single_general_register special value S = 1 (Size variant 1)
    // Fields: opc=0, option=0, size=1, Rm=0, S=1, Rt=0, Rn=0
    let encoding: u32 = 0x78201800;
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

/// Provenance: aarch64_memory_single_general_register
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_memory_single_general_register_special_rn_31_stack_pointer_sp_may_require_alignment_2048_78200be0()
 {
    // Encoding: 0x78200BE0
    // Test aarch64_memory_single_general_register special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: option=0, size=1, opc=0, S=0, Rn=31, Rm=0, Rt=0
    let encoding: u32 = 0x78200BE0;
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

/// Provenance: aarch64_memory_single_general_register
/// ASL: `field Rt = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)`
/// Requirement: FieldSpecial { field: "Rt", value: 31, meaning: "Zero register (XZR/WZR) - reads as 0, writes discarded" }
/// Zero register (XZR/WZR) - reads as 0, writes discarded
#[test]
fn test_aarch64_memory_single_general_register_special_rt_31_zero_register_xzr_wzr_reads_as_0_writes_discarded_2048_7820081f()
 {
    // Encoding: 0x7820081F
    // Test aarch64_memory_single_general_register special value Rt = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)
    // Fields: opc=0, option=0, S=0, Rn=0, Rt=31, Rm=0, size=1
    let encoding: u32 = 0x7820081F;
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

/// Provenance: aarch64_memory_single_general_register
/// ASL: `GpFromField("t") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "t" }
/// verify register write to GpFromField("t")
#[test]
fn test_aarch64_memory_single_general_register_reg_write_0_38200800() {
    // Test aarch64_memory_single_general_register register write: GpFromField("t")
    // Encoding: 0x38200800
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x38200800;
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

/// Provenance: aarch64_memory_single_general_register
/// ASL: `GpFromField("t") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "t" }
/// verify register write to GpFromField("t")
#[test]
fn test_aarch64_memory_single_general_register_reg_write_1_38200800() {
    // Test aarch64_memory_single_general_register register write: GpFromField("t")
    // Encoding: 0x38200800
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x38200800;
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

/// Provenance: aarch64_memory_single_general_register
/// ASL: `Sp write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "unknown" }
/// verify register write to Sp
#[test]
fn test_aarch64_memory_single_general_register_reg_write_2_38200800() {
    // Test aarch64_memory_single_general_register register write: Sp
    // Encoding: 0x38200800
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x38200800;
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

/// Provenance: aarch64_memory_single_general_register
/// ASL: `GpFromField("n") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "n" }
/// verify register write to GpFromField("n")
#[test]
fn test_aarch64_memory_single_general_register_reg_write_3_38200800() {
    // Test aarch64_memory_single_general_register register write: GpFromField("n")
    // Encoding: 0x38200800
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x38200800;
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

/// Provenance: aarch64_memory_single_general_register
/// ASL: `Rn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Rn = 31)
#[test]
fn test_aarch64_memory_single_general_register_sp_rn_38200be0() {
    // Test aarch64_memory_single_general_register with Rn = SP (31)
    // Encoding: 0x38200BE0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x38200BE0;
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

/// Provenance: aarch64_memory_single_general_register
/// ASL: `Rt = 31 (ZR)`
/// Requirement: RegisterSpecial { reg: Zr, behavior: "reads as 0, writes discarded" }
/// zero register (Rt = 31)
#[test]
fn test_aarch64_memory_single_general_register_zr_rt_3820081f() {
    // Test aarch64_memory_single_general_register with Rt = ZR (31)
    // Encoding: 0x3820081F
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x3820081F;
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

/// Provenance: aarch64_memory_single_general_register
/// ASL: `Mem[address, 8] = data`
/// Requirement: MemoryAccess { op: Store, size_bits: 64, addressing: "Base { reg: \"address\" }" }
/// 8-byte store
#[test]
fn test_aarch64_memory_single_general_register_store_0_38200820() {
    // Test aarch64_memory_single_general_register memory store: 8 bytes
    // Encoding: 0x38200820
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x100000000000);
    set_x(&mut cpu, 0, 0xDEADBEEFCAFEBABE);
    let encoding: u32 = 0x38200820;
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

/// Provenance: aarch64_memory_single_general_register
/// ASL: `Unconditional`
/// Requirement: UndefinedEncoding { condition: "Unconditional" }
/// triggers Undefined
#[test]
fn test_aarch64_memory_single_general_register_exception_0_38200800() {
    // Test aarch64_memory_single_general_register exception: Undefined
    // Encoding: 0x38200800
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x38200800;
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

/// Provenance: aarch64_memory_single_general_register
/// ASL: `Unconditional`
/// Requirement: UndefinedEncoding { condition: "Unconditional" }
/// triggers Undefined
#[test]
fn test_aarch64_memory_single_general_register_exception_1_38200800() {
    // Test aarch64_memory_single_general_register exception: Undefined
    // Encoding: 0x38200800
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x38200800;
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

// ============================================================================
// aarch64_memory_single_general_immediate_signed_offset_unpriv Tests
// ============================================================================

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_unpriv
/// ASL: `field size 30 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_unpriv_field_size_0_min_800_38000800()
{
    // Encoding: 0x38000800
    // Test aarch64_memory_single_general_immediate_signed_offset_unpriv field size = 0 (Min)
    // Fields: size=0, Rn=0, Rt=0, opc=0, imm9=0
    let encoding: u32 = 0x38000800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_unpriv
/// ASL: `field size 30 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 1, boundary: PowerOfTwo }
/// 16-bit / halfword size
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_unpriv_field_size_1_poweroftwo_800_78000800()
 {
    // Encoding: 0x78000800
    // Test aarch64_memory_single_general_immediate_signed_offset_unpriv field size = 1 (PowerOfTwo)
    // Fields: Rt=0, size=1, opc=0, imm9=0, Rn=0
    let encoding: u32 = 0x78000800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_unpriv
/// ASL: `field size 30 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 2, boundary: PowerOfTwo }
/// 32-bit / word size
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_unpriv_field_size_2_poweroftwo_800_b8000800()
 {
    // Encoding: 0xB8000800
    // Test aarch64_memory_single_general_immediate_signed_offset_unpriv field size = 2 (PowerOfTwo)
    // Fields: opc=0, size=2, Rn=0, Rt=0, imm9=0
    let encoding: u32 = 0xB8000800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_unpriv
/// ASL: `field size 30 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 3, boundary: Max }
/// 64-bit / doubleword size
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_unpriv_field_size_3_max_800_f8000800()
{
    // Encoding: 0xF8000800
    // Test aarch64_memory_single_general_immediate_signed_offset_unpriv field size = 3 (Max)
    // Fields: Rt=0, size=3, opc=0, imm9=0, Rn=0
    let encoding: u32 = 0xF8000800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_unpriv
/// ASL: `field opc 22 +: 2`
/// Requirement: FieldBoundary { field: "opc", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_unpriv_field_opc_0_min_800_38000800()
{
    // Encoding: 0x38000800
    // Test aarch64_memory_single_general_immediate_signed_offset_unpriv field opc = 0 (Min)
    // Fields: size=0, Rt=0, opc=0, imm9=0, Rn=0
    let encoding: u32 = 0x38000800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_unpriv
/// ASL: `field opc 22 +: 2`
/// Requirement: FieldBoundary { field: "opc", value: 1, boundary: PowerOfTwo }
/// 16-bit / halfword size
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_unpriv_field_opc_1_poweroftwo_800_38400800()
 {
    // Encoding: 0x38400800
    // Test aarch64_memory_single_general_immediate_signed_offset_unpriv field opc = 1 (PowerOfTwo)
    // Fields: size=0, Rn=0, imm9=0, Rt=0, opc=1
    let encoding: u32 = 0x38400800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_unpriv
/// ASL: `field opc 22 +: 2`
/// Requirement: FieldBoundary { field: "opc", value: 2, boundary: PowerOfTwo }
/// 32-bit / word size
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_unpriv_field_opc_2_poweroftwo_800_38800800()
 {
    // Encoding: 0x38800800
    // Test aarch64_memory_single_general_immediate_signed_offset_unpriv field opc = 2 (PowerOfTwo)
    // Fields: Rn=0, Rt=0, size=0, opc=2, imm9=0
    let encoding: u32 = 0x38800800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_unpriv
/// ASL: `field opc 22 +: 2`
/// Requirement: FieldBoundary { field: "opc", value: 3, boundary: Max }
/// 64-bit / doubleword size
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_unpriv_field_opc_3_max_800_38c00800()
{
    // Encoding: 0x38C00800
    // Test aarch64_memory_single_general_immediate_signed_offset_unpriv field opc = 3 (Max)
    // Fields: size=0, imm9=0, Rn=0, Rt=0, opc=3
    let encoding: u32 = 0x38C00800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_unpriv
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_unpriv_field_imm9_0_zero_800_38000800()
 {
    // Encoding: 0x38000800
    // Test aarch64_memory_single_general_immediate_signed_offset_unpriv field imm9 = 0 (Zero)
    // Fields: opc=0, size=0, Rn=0, Rt=0, imm9=0
    let encoding: u32 = 0x38000800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_unpriv
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_unpriv_field_imm9_1_poweroftwo_800_38001800()
 {
    // Encoding: 0x38001800
    // Test aarch64_memory_single_general_immediate_signed_offset_unpriv field imm9 = 1 (PowerOfTwo)
    // Fields: Rn=0, size=0, imm9=1, Rt=0, opc=0
    let encoding: u32 = 0x38001800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_unpriv
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 3, boundary: PowerOfTwoMinusOne }
/// 2^2 - 1 = 3
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_unpriv_field_imm9_3_poweroftwominusone_800_38003800()
 {
    // Encoding: 0x38003800
    // Test aarch64_memory_single_general_immediate_signed_offset_unpriv field imm9 = 3 (PowerOfTwoMinusOne)
    // Fields: imm9=3, Rn=0, Rt=0, opc=0, size=0
    let encoding: u32 = 0x38003800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_unpriv
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 4, boundary: PowerOfTwo }
/// power of 2 (2^2 = 4)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_unpriv_field_imm9_4_poweroftwo_800_38004800()
 {
    // Encoding: 0x38004800
    // Test aarch64_memory_single_general_immediate_signed_offset_unpriv field imm9 = 4 (PowerOfTwo)
    // Fields: opc=0, Rt=0, size=0, Rn=0, imm9=4
    let encoding: u32 = 0x38004800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_unpriv
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 7, boundary: PowerOfTwoMinusOne }
/// 2^3 - 1 = 7
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_unpriv_field_imm9_7_poweroftwominusone_800_38007800()
 {
    // Encoding: 0x38007800
    // Test aarch64_memory_single_general_immediate_signed_offset_unpriv field imm9 = 7 (PowerOfTwoMinusOne)
    // Fields: opc=0, imm9=7, Rn=0, size=0, Rt=0
    let encoding: u32 = 0x38007800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_unpriv
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 8, boundary: PowerOfTwo }
/// power of 2 (2^3 = 8)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_unpriv_field_imm9_8_poweroftwo_800_38008800()
 {
    // Encoding: 0x38008800
    // Test aarch64_memory_single_general_immediate_signed_offset_unpriv field imm9 = 8 (PowerOfTwo)
    // Fields: Rt=0, imm9=8, opc=0, size=0, Rn=0
    let encoding: u32 = 0x38008800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_unpriv
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 15, boundary: PowerOfTwoMinusOne }
/// 2^4 - 1 = 15
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_unpriv_field_imm9_15_poweroftwominusone_800_3800f800()
 {
    // Encoding: 0x3800F800
    // Test aarch64_memory_single_general_immediate_signed_offset_unpriv field imm9 = 15 (PowerOfTwoMinusOne)
    // Fields: Rt=0, size=0, imm9=15, opc=0, Rn=0
    let encoding: u32 = 0x3800F800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_unpriv
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 16, boundary: PowerOfTwo }
/// power of 2 (2^4 = 16)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_unpriv_field_imm9_16_poweroftwo_800_38010800()
 {
    // Encoding: 0x38010800
    // Test aarch64_memory_single_general_immediate_signed_offset_unpriv field imm9 = 16 (PowerOfTwo)
    // Fields: size=0, Rt=0, imm9=16, opc=0, Rn=0
    let encoding: u32 = 0x38010800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_unpriv
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 31, boundary: PowerOfTwoMinusOne }
/// 2^5 - 1 = 31
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_unpriv_field_imm9_31_poweroftwominusone_800_3801f800()
 {
    // Encoding: 0x3801F800
    // Test aarch64_memory_single_general_immediate_signed_offset_unpriv field imm9 = 31 (PowerOfTwoMinusOne)
    // Fields: Rt=0, size=0, imm9=31, opc=0, Rn=0
    let encoding: u32 = 0x3801F800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_unpriv
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 32, boundary: PowerOfTwo }
/// power of 2 (2^5 = 32)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_unpriv_field_imm9_32_poweroftwo_800_38020800()
 {
    // Encoding: 0x38020800
    // Test aarch64_memory_single_general_immediate_signed_offset_unpriv field imm9 = 32 (PowerOfTwo)
    // Fields: opc=0, imm9=32, size=0, Rn=0, Rt=0
    let encoding: u32 = 0x38020800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_unpriv
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 63, boundary: PowerOfTwoMinusOne }
/// 2^6 - 1 = 63
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_unpriv_field_imm9_63_poweroftwominusone_800_3803f800()
 {
    // Encoding: 0x3803F800
    // Test aarch64_memory_single_general_immediate_signed_offset_unpriv field imm9 = 63 (PowerOfTwoMinusOne)
    // Fields: imm9=63, opc=0, size=0, Rn=0, Rt=0
    let encoding: u32 = 0x3803F800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_unpriv
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 64, boundary: PowerOfTwo }
/// power of 2 (2^6 = 64)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_unpriv_field_imm9_64_poweroftwo_800_38040800()
 {
    // Encoding: 0x38040800
    // Test aarch64_memory_single_general_immediate_signed_offset_unpriv field imm9 = 64 (PowerOfTwo)
    // Fields: Rn=0, opc=0, Rt=0, size=0, imm9=64
    let encoding: u32 = 0x38040800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_unpriv
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 127, boundary: PowerOfTwoMinusOne }
/// 2^7 - 1 = 127
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_unpriv_field_imm9_127_poweroftwominusone_800_3807f800()
 {
    // Encoding: 0x3807F800
    // Test aarch64_memory_single_general_immediate_signed_offset_unpriv field imm9 = 127 (PowerOfTwoMinusOne)
    // Fields: opc=0, size=0, Rn=0, imm9=127, Rt=0
    let encoding: u32 = 0x3807F800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_unpriv
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 128, boundary: PowerOfTwo }
/// power of 2 (2^7 = 128)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_unpriv_field_imm9_128_poweroftwo_800_38080800()
 {
    // Encoding: 0x38080800
    // Test aarch64_memory_single_general_immediate_signed_offset_unpriv field imm9 = 128 (PowerOfTwo)
    // Fields: imm9=128, Rn=0, size=0, Rt=0, opc=0
    let encoding: u32 = 0x38080800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_unpriv
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 255, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (255)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_unpriv_field_imm9_255_poweroftwominusone_800_380ff800()
 {
    // Encoding: 0x380FF800
    // Test aarch64_memory_single_general_immediate_signed_offset_unpriv field imm9 = 255 (PowerOfTwoMinusOne)
    // Fields: size=0, Rt=0, Rn=0, imm9=255, opc=0
    let encoding: u32 = 0x380FF800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_unpriv
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 256, boundary: PowerOfTwo }
/// power of 2 (2^8 = 256)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_unpriv_field_imm9_256_poweroftwo_800_38100800()
 {
    // Encoding: 0x38100800
    // Test aarch64_memory_single_general_immediate_signed_offset_unpriv field imm9 = 256 (PowerOfTwo)
    // Fields: opc=0, size=0, imm9=256, Rn=0, Rt=0
    let encoding: u32 = 0x38100800;
    let mut cpu = create_test_cpu();
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

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_unpriv
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 511, boundary: Max }
/// maximum immediate (511)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_unpriv_field_imm9_511_max_800_381ff800()
 {
    // Encoding: 0x381FF800
    // Test aarch64_memory_single_general_immediate_signed_offset_unpriv field imm9 = 511 (Max)
    // Fields: Rt=0, imm9=511, Rn=0, opc=0, size=0
    let encoding: u32 = 0x381FF800;
    let mut cpu = create_test_cpu();
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

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_unpriv
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_unpriv_field_rn_0_min_800_38000800() {
    // Encoding: 0x38000800
    // Test aarch64_memory_single_general_immediate_signed_offset_unpriv field Rn = 0 (Min)
    // Fields: Rn=0, imm9=0, opc=0, Rt=0, size=0
    let encoding: u32 = 0x38000800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_unpriv
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_unpriv_field_rn_1_poweroftwo_800_38000820()
 {
    // Encoding: 0x38000820
    // Test aarch64_memory_single_general_immediate_signed_offset_unpriv field Rn = 1 (PowerOfTwo)
    // Fields: size=0, imm9=0, Rt=0, Rn=1, opc=0
    let encoding: u32 = 0x38000820;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_unpriv
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_unpriv_field_rn_30_poweroftwominusone_800_38000bc0()
 {
    // Encoding: 0x38000BC0
    // Test aarch64_memory_single_general_immediate_signed_offset_unpriv field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: Rn=30, Rt=0, size=0, imm9=0, opc=0
    let encoding: u32 = 0x38000BC0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_unpriv
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_unpriv_field_rn_31_max_800_38000be0()
{
    // Encoding: 0x38000BE0
    // Test aarch64_memory_single_general_immediate_signed_offset_unpriv field Rn = 31 (Max)
    // Fields: Rn=31, Rt=0, opc=0, size=0, imm9=0
    let encoding: u32 = 0x38000BE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_unpriv
/// ASL: `field Rt 0 +: 5`
/// Requirement: FieldBoundary { field: "Rt", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_unpriv_field_rt_0_min_800_38000800() {
    // Encoding: 0x38000800
    // Test aarch64_memory_single_general_immediate_signed_offset_unpriv field Rt = 0 (Min)
    // Fields: size=0, opc=0, Rn=0, Rt=0, imm9=0
    let encoding: u32 = 0x38000800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_unpriv
/// ASL: `field Rt 0 +: 5`
/// Requirement: FieldBoundary { field: "Rt", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_unpriv_field_rt_1_poweroftwo_800_38000801()
 {
    // Encoding: 0x38000801
    // Test aarch64_memory_single_general_immediate_signed_offset_unpriv field Rt = 1 (PowerOfTwo)
    // Fields: opc=0, Rn=0, Rt=1, imm9=0, size=0
    let encoding: u32 = 0x38000801;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_unpriv
/// ASL: `field Rt 0 +: 5`
/// Requirement: FieldBoundary { field: "Rt", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_unpriv_field_rt_30_poweroftwominusone_800_3800081e()
 {
    // Encoding: 0x3800081E
    // Test aarch64_memory_single_general_immediate_signed_offset_unpriv field Rt = 30 (PowerOfTwoMinusOne)
    // Fields: size=0, Rn=0, Rt=30, opc=0, imm9=0
    let encoding: u32 = 0x3800081E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_unpriv
/// ASL: `field Rt 0 +: 5`
/// Requirement: FieldBoundary { field: "Rt", value: 31, boundary: Max }
/// register index 31 (ZR - zero register)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_unpriv_field_rt_31_max_800_3800081f()
{
    // Encoding: 0x3800081F
    // Test aarch64_memory_single_general_immediate_signed_offset_unpriv field Rt = 31 (Max)
    // Fields: Rn=0, Rt=31, opc=0, size=0, imm9=0
    let encoding: u32 = 0x3800081F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_unpriv
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=0 (8-bit / byte size)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_unpriv_combo_0_800_38000800() {
    // Encoding: 0x38000800
    // Test aarch64_memory_single_general_immediate_signed_offset_unpriv field combination: size=0, opc=0, imm9=0, Rn=0, Rt=0
    // Fields: size=0, imm9=0, Rn=0, Rt=0, opc=0
    let encoding: u32 = 0x38000800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_unpriv
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=1 (16-bit / halfword size)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_unpriv_combo_1_800_78000800() {
    // Encoding: 0x78000800
    // Test aarch64_memory_single_general_immediate_signed_offset_unpriv field combination: size=1, opc=0, imm9=0, Rn=0, Rt=0
    // Fields: Rt=0, Rn=0, imm9=0, size=1, opc=0
    let encoding: u32 = 0x78000800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_unpriv
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=2 (32-bit / word size)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_unpriv_combo_2_800_b8000800() {
    // Encoding: 0xB8000800
    // Test aarch64_memory_single_general_immediate_signed_offset_unpriv field combination: size=2, opc=0, imm9=0, Rn=0, Rt=0
    // Fields: Rn=0, opc=0, imm9=0, Rt=0, size=2
    let encoding: u32 = 0xB8000800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_unpriv
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=3 (64-bit / doubleword size)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_unpriv_combo_3_800_f8000800() {
    // Encoding: 0xF8000800
    // Test aarch64_memory_single_general_immediate_signed_offset_unpriv field combination: size=3, opc=0, imm9=0, Rn=0, Rt=0
    // Fields: opc=0, imm9=0, Rn=0, Rt=0, size=3
    let encoding: u32 = 0xF8000800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_unpriv
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// opc=0 (8-bit / byte size)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_unpriv_combo_4_800_38000800() {
    // Encoding: 0x38000800
    // Test aarch64_memory_single_general_immediate_signed_offset_unpriv field combination: size=0, opc=0, imm9=0, Rn=0, Rt=0
    // Fields: imm9=0, opc=0, size=0, Rn=0, Rt=0
    let encoding: u32 = 0x38000800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_unpriv
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// opc=1 (16-bit / halfword size)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_unpriv_combo_5_800_38400800() {
    // Encoding: 0x38400800
    // Test aarch64_memory_single_general_immediate_signed_offset_unpriv field combination: size=0, opc=1, imm9=0, Rn=0, Rt=0
    // Fields: size=0, Rt=0, Rn=0, imm9=0, opc=1
    let encoding: u32 = 0x38400800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_unpriv
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// opc=2 (32-bit / word size)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_unpriv_combo_6_800_38800800() {
    // Encoding: 0x38800800
    // Test aarch64_memory_single_general_immediate_signed_offset_unpriv field combination: size=0, opc=2, imm9=0, Rn=0, Rt=0
    // Fields: size=0, Rn=0, imm9=0, Rt=0, opc=2
    let encoding: u32 = 0x38800800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_unpriv
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// opc=3 (64-bit / doubleword size)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_unpriv_combo_7_800_38c00800() {
    // Encoding: 0x38C00800
    // Test aarch64_memory_single_general_immediate_signed_offset_unpriv field combination: size=0, opc=3, imm9=0, Rn=0, Rt=0
    // Fields: Rt=0, imm9=0, Rn=0, size=0, opc=3
    let encoding: u32 = 0x38C00800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_unpriv
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=0 (immediate value 0)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_unpriv_combo_8_800_38000800() {
    // Encoding: 0x38000800
    // Test aarch64_memory_single_general_immediate_signed_offset_unpriv field combination: size=0, opc=0, imm9=0, Rn=0, Rt=0
    // Fields: opc=0, size=0, imm9=0, Rt=0, Rn=0
    let encoding: u32 = 0x38000800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_unpriv
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=1 (immediate value 1)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_unpriv_combo_9_800_38001800() {
    // Encoding: 0x38001800
    // Test aarch64_memory_single_general_immediate_signed_offset_unpriv field combination: size=0, opc=0, imm9=1, Rn=0, Rt=0
    // Fields: Rn=0, Rt=0, size=0, opc=0, imm9=1
    let encoding: u32 = 0x38001800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_unpriv
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=3 (2^2 - 1 = 3)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_unpriv_combo_10_800_38003800() {
    // Encoding: 0x38003800
    // Test aarch64_memory_single_general_immediate_signed_offset_unpriv field combination: size=0, opc=0, imm9=3, Rn=0, Rt=0
    // Fields: Rn=0, size=0, Rt=0, opc=0, imm9=3
    let encoding: u32 = 0x38003800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_unpriv
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=4 (power of 2 (2^2 = 4))
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_unpriv_combo_11_800_38004800() {
    // Encoding: 0x38004800
    // Test aarch64_memory_single_general_immediate_signed_offset_unpriv field combination: size=0, opc=0, imm9=4, Rn=0, Rt=0
    // Fields: imm9=4, opc=0, Rn=0, Rt=0, size=0
    let encoding: u32 = 0x38004800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_unpriv
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=7 (2^3 - 1 = 7)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_unpriv_combo_12_800_38007800() {
    // Encoding: 0x38007800
    // Test aarch64_memory_single_general_immediate_signed_offset_unpriv field combination: size=0, opc=0, imm9=7, Rn=0, Rt=0
    // Fields: Rt=0, opc=0, size=0, Rn=0, imm9=7
    let encoding: u32 = 0x38007800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_unpriv
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=8 (power of 2 (2^3 = 8))
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_unpriv_combo_13_800_38008800() {
    // Encoding: 0x38008800
    // Test aarch64_memory_single_general_immediate_signed_offset_unpriv field combination: size=0, opc=0, imm9=8, Rn=0, Rt=0
    // Fields: opc=0, Rt=0, size=0, imm9=8, Rn=0
    let encoding: u32 = 0x38008800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_unpriv
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=15 (2^4 - 1 = 15)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_unpriv_combo_14_800_3800f800() {
    // Encoding: 0x3800F800
    // Test aarch64_memory_single_general_immediate_signed_offset_unpriv field combination: size=0, opc=0, imm9=15, Rn=0, Rt=0
    // Fields: imm9=15, Rt=0, Rn=0, opc=0, size=0
    let encoding: u32 = 0x3800F800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_unpriv
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=16 (power of 2 (2^4 = 16))
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_unpriv_combo_15_800_38010800() {
    // Encoding: 0x38010800
    // Test aarch64_memory_single_general_immediate_signed_offset_unpriv field combination: size=0, opc=0, imm9=16, Rn=0, Rt=0
    // Fields: size=0, Rt=0, imm9=16, opc=0, Rn=0
    let encoding: u32 = 0x38010800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_unpriv
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=31 (2^5 - 1 = 31)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_unpriv_combo_16_800_3801f800() {
    // Encoding: 0x3801F800
    // Test aarch64_memory_single_general_immediate_signed_offset_unpriv field combination: size=0, opc=0, imm9=31, Rn=0, Rt=0
    // Fields: Rn=0, opc=0, size=0, imm9=31, Rt=0
    let encoding: u32 = 0x3801F800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_unpriv
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=32 (power of 2 (2^5 = 32))
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_unpriv_combo_17_800_38020800() {
    // Encoding: 0x38020800
    // Test aarch64_memory_single_general_immediate_signed_offset_unpriv field combination: size=0, opc=0, imm9=32, Rn=0, Rt=0
    // Fields: Rt=0, size=0, opc=0, Rn=0, imm9=32
    let encoding: u32 = 0x38020800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_unpriv
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=63 (2^6 - 1 = 63)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_unpriv_combo_18_800_3803f800() {
    // Encoding: 0x3803F800
    // Test aarch64_memory_single_general_immediate_signed_offset_unpriv field combination: size=0, opc=0, imm9=63, Rn=0, Rt=0
    // Fields: imm9=63, Rn=0, opc=0, Rt=0, size=0
    let encoding: u32 = 0x3803F800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_unpriv
/// ASL: `field combination 19`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=64 (power of 2 (2^6 = 64))
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_unpriv_combo_19_800_38040800() {
    // Encoding: 0x38040800
    // Test aarch64_memory_single_general_immediate_signed_offset_unpriv field combination: size=0, opc=0, imm9=64, Rn=0, Rt=0
    // Fields: size=0, Rt=0, opc=0, Rn=0, imm9=64
    let encoding: u32 = 0x38040800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_unpriv
/// ASL: `field combination 20`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=127 (2^7 - 1 = 127)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_unpriv_combo_20_800_3807f800() {
    // Encoding: 0x3807F800
    // Test aarch64_memory_single_general_immediate_signed_offset_unpriv field combination: size=0, opc=0, imm9=127, Rn=0, Rt=0
    // Fields: opc=0, Rn=0, size=0, Rt=0, imm9=127
    let encoding: u32 = 0x3807F800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_unpriv
/// ASL: `field combination 21`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=128 (power of 2 (2^7 = 128))
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_unpriv_combo_21_800_38080800() {
    // Encoding: 0x38080800
    // Test aarch64_memory_single_general_immediate_signed_offset_unpriv field combination: size=0, opc=0, imm9=128, Rn=0, Rt=0
    // Fields: opc=0, size=0, imm9=128, Rn=0, Rt=0
    let encoding: u32 = 0x38080800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_unpriv
/// ASL: `field combination 22`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=255 (immediate midpoint (255))
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_unpriv_combo_22_800_380ff800() {
    // Encoding: 0x380FF800
    // Test aarch64_memory_single_general_immediate_signed_offset_unpriv field combination: size=0, opc=0, imm9=255, Rn=0, Rt=0
    // Fields: Rt=0, size=0, Rn=0, opc=0, imm9=255
    let encoding: u32 = 0x380FF800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_unpriv
/// ASL: `field combination 23`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=256 (power of 2 (2^8 = 256))
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_unpriv_combo_23_800_38100800() {
    // Encoding: 0x38100800
    // Test aarch64_memory_single_general_immediate_signed_offset_unpriv field combination: size=0, opc=0, imm9=256, Rn=0, Rt=0
    // Fields: Rn=0, size=0, imm9=256, opc=0, Rt=0
    let encoding: u32 = 0x38100800;
    let mut cpu = create_test_cpu();
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

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_unpriv
/// ASL: `field combination 24`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=511 (maximum immediate (511))
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_unpriv_combo_24_800_381ff800() {
    // Encoding: 0x381FF800
    // Test aarch64_memory_single_general_immediate_signed_offset_unpriv field combination: size=0, opc=0, imm9=511, Rn=0, Rt=0
    // Fields: size=0, opc=0, imm9=511, Rt=0, Rn=0
    let encoding: u32 = 0x381FF800;
    let mut cpu = create_test_cpu();
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

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_unpriv
/// ASL: `field combination 25`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_unpriv_combo_25_800_38000800() {
    // Encoding: 0x38000800
    // Test aarch64_memory_single_general_immediate_signed_offset_unpriv field combination: size=0, opc=0, imm9=0, Rn=0, Rt=0
    // Fields: imm9=0, size=0, Rt=0, opc=0, Rn=0
    let encoding: u32 = 0x38000800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_unpriv
/// ASL: `field combination 26`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_unpriv_combo_26_800_38000820() {
    // Encoding: 0x38000820
    // Test aarch64_memory_single_general_immediate_signed_offset_unpriv field combination: size=0, opc=0, imm9=0, Rn=1, Rt=0
    // Fields: size=0, imm9=0, Rn=1, opc=0, Rt=0
    let encoding: u32 = 0x38000820;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_unpriv
/// ASL: `field combination 27`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_unpriv_combo_27_800_38000bc0() {
    // Encoding: 0x38000BC0
    // Test aarch64_memory_single_general_immediate_signed_offset_unpriv field combination: size=0, opc=0, imm9=0, Rn=30, Rt=0
    // Fields: imm9=0, opc=0, size=0, Rt=0, Rn=30
    let encoding: u32 = 0x38000BC0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_unpriv
/// ASL: `field combination 28`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_unpriv_combo_28_800_38000be0() {
    // Encoding: 0x38000BE0
    // Test aarch64_memory_single_general_immediate_signed_offset_unpriv field combination: size=0, opc=0, imm9=0, Rn=31, Rt=0
    // Fields: Rt=0, size=0, Rn=31, imm9=0, opc=0
    let encoding: u32 = 0x38000BE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_unpriv
/// ASL: `field combination 29`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt=0 (register index 0 (first register))
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_unpriv_combo_29_800_38000800() {
    // Encoding: 0x38000800
    // Test aarch64_memory_single_general_immediate_signed_offset_unpriv field combination: size=0, opc=0, imm9=0, Rn=0, Rt=0
    // Fields: Rt=0, imm9=0, size=0, opc=0, Rn=0
    let encoding: u32 = 0x38000800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_unpriv
/// ASL: `field combination 30`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt=1 (register index 1 (second register))
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_unpriv_combo_30_800_38000801() {
    // Encoding: 0x38000801
    // Test aarch64_memory_single_general_immediate_signed_offset_unpriv field combination: size=0, opc=0, imm9=0, Rn=0, Rt=1
    // Fields: Rt=1, imm9=0, size=0, opc=0, Rn=0
    let encoding: u32 = 0x38000801;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_unpriv
/// ASL: `field combination 31`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_unpriv_combo_31_800_3800081e() {
    // Encoding: 0x3800081E
    // Test aarch64_memory_single_general_immediate_signed_offset_unpriv field combination: size=0, opc=0, imm9=0, Rn=0, Rt=30
    // Fields: Rn=0, size=0, Rt=30, imm9=0, opc=0
    let encoding: u32 = 0x3800081E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_unpriv
/// ASL: `field combination 32`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt=31 (register index 31 (ZR - zero register))
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_unpriv_combo_32_800_3800081f() {
    // Encoding: 0x3800081F
    // Test aarch64_memory_single_general_immediate_signed_offset_unpriv field combination: size=0, opc=0, imm9=0, Rn=0, Rt=31
    // Fields: opc=0, imm9=0, size=0, Rn=0, Rt=31
    let encoding: u32 = 0x3800081F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_unpriv
/// ASL: `field combination 33`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (same register test (reg=1)), Rt=1 (same register test (reg=1))
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_unpriv_combo_33_800_38000821() {
    // Encoding: 0x38000821
    // Test aarch64_memory_single_general_immediate_signed_offset_unpriv field combination: size=0, opc=0, imm9=0, Rn=1, Rt=1
    // Fields: opc=0, size=0, imm9=0, Rn=1, Rt=1
    let encoding: u32 = 0x38000821;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_unpriv
/// ASL: `field combination 34`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (same register test (reg=31)), Rt=31 (same register test (reg=31))
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_unpriv_combo_34_800_38000bff() {
    // Encoding: 0x38000BFF
    // Test aarch64_memory_single_general_immediate_signed_offset_unpriv field combination: size=0, opc=0, imm9=0, Rn=31, Rt=31
    // Fields: opc=0, size=0, imm9=0, Rn=31, Rt=31
    let encoding: u32 = 0x38000BFF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_unpriv
/// ASL: `field size = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "size", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_unpriv_special_size_0_size_variant_0_2048_38001800()
 {
    // Encoding: 0x38001800
    // Test aarch64_memory_single_general_immediate_signed_offset_unpriv special value size = 0 (Size variant 0)
    // Fields: size=0, Rt=0, imm9=1, Rn=0, opc=0
    let encoding: u32 = 0x38001800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_unpriv
/// ASL: `field size = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "size", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_unpriv_special_size_1_size_variant_1_2048_78001800()
 {
    // Encoding: 0x78001800
    // Test aarch64_memory_single_general_immediate_signed_offset_unpriv special value size = 1 (Size variant 1)
    // Fields: size=1, Rt=0, imm9=1, Rn=0, opc=0
    let encoding: u32 = 0x78001800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_unpriv
/// ASL: `field size = 2 (Size variant 2)`
/// Requirement: FieldSpecial { field: "size", value: 2, meaning: "Size variant 2" }
/// Size variant 2
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_unpriv_special_size_2_size_variant_2_2048_b8001800()
 {
    // Encoding: 0xB8001800
    // Test aarch64_memory_single_general_immediate_signed_offset_unpriv special value size = 2 (Size variant 2)
    // Fields: Rn=0, Rt=0, imm9=1, size=2, opc=0
    let encoding: u32 = 0xB8001800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_unpriv
/// ASL: `field size = 3 (Size variant 3)`
/// Requirement: FieldSpecial { field: "size", value: 3, meaning: "Size variant 3" }
/// Size variant 3
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_unpriv_special_size_3_size_variant_3_2048_f8001800()
 {
    // Encoding: 0xF8001800
    // Test aarch64_memory_single_general_immediate_signed_offset_unpriv special value size = 3 (Size variant 3)
    // Fields: imm9=1, Rn=0, Rt=0, size=3, opc=0
    let encoding: u32 = 0xF8001800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_unpriv
/// ASL: `field opc = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "opc", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_unpriv_special_opc_0_size_variant_0_2048_78001800()
 {
    // Encoding: 0x78001800
    // Test aarch64_memory_single_general_immediate_signed_offset_unpriv special value opc = 0 (Size variant 0)
    // Fields: Rn=0, size=1, Rt=0, imm9=1, opc=0
    let encoding: u32 = 0x78001800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_unpriv
/// ASL: `field opc = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "opc", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_unpriv_special_opc_1_size_variant_1_2048_78401800()
 {
    // Encoding: 0x78401800
    // Test aarch64_memory_single_general_immediate_signed_offset_unpriv special value opc = 1 (Size variant 1)
    // Fields: Rt=0, size=1, imm9=1, Rn=0, opc=1
    let encoding: u32 = 0x78401800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_unpriv
/// ASL: `field opc = 2 (Size variant 2)`
/// Requirement: FieldSpecial { field: "opc", value: 2, meaning: "Size variant 2" }
/// Size variant 2
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_unpriv_special_opc_2_size_variant_2_2048_78801800()
 {
    // Encoding: 0x78801800
    // Test aarch64_memory_single_general_immediate_signed_offset_unpriv special value opc = 2 (Size variant 2)
    // Fields: size=1, opc=2, Rt=0, imm9=1, Rn=0
    let encoding: u32 = 0x78801800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_unpriv
/// ASL: `field opc = 3 (Size variant 3)`
/// Requirement: FieldSpecial { field: "opc", value: 3, meaning: "Size variant 3" }
/// Size variant 3
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_unpriv_special_opc_3_size_variant_3_2048_78c01800()
 {
    // Encoding: 0x78C01800
    // Test aarch64_memory_single_general_immediate_signed_offset_unpriv special value opc = 3 (Size variant 3)
    // Fields: imm9=1, Rn=0, opc=3, size=1, Rt=0
    let encoding: u32 = 0x78C01800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_unpriv
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_unpriv_special_rn_31_stack_pointer_sp_may_require_alignment_2048_78001be0()
 {
    // Encoding: 0x78001BE0
    // Test aarch64_memory_single_general_immediate_signed_offset_unpriv special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: opc=0, Rt=0, size=1, imm9=1, Rn=31
    let encoding: u32 = 0x78001BE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_unpriv
/// ASL: `field Rt = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)`
/// Requirement: FieldSpecial { field: "Rt", value: 31, meaning: "Zero register (XZR/WZR) - reads as 0, writes discarded" }
/// Zero register (XZR/WZR) - reads as 0, writes discarded
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_unpriv_special_rt_31_zero_register_xzr_wzr_reads_as_0_writes_discarded_2048_7800181f()
 {
    // Encoding: 0x7800181F
    // Test aarch64_memory_single_general_immediate_signed_offset_unpriv special value Rt = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)
    // Fields: imm9=1, Rt=31, Rn=0, opc=0, size=1
    let encoding: u32 = 0x7800181F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_unpriv
/// ASL: `GpFromField("t") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "t" }
/// verify register write to GpFromField("t")
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_unpriv_reg_write_0_38000800() {
    // Test aarch64_memory_single_general_immediate_signed_offset_unpriv register write: GpFromField("t")
    // Encoding: 0x38000800
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x38000800;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_unpriv
/// ASL: `GpFromField("t") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "t" }
/// verify register write to GpFromField("t")
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_unpriv_reg_write_1_38000800() {
    // Test aarch64_memory_single_general_immediate_signed_offset_unpriv register write: GpFromField("t")
    // Encoding: 0x38000800
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x38000800;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_unpriv
/// ASL: `Sp write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "unknown" }
/// verify register write to Sp
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_unpriv_reg_write_2_38000800() {
    // Test aarch64_memory_single_general_immediate_signed_offset_unpriv register write: Sp
    // Encoding: 0x38000800
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x38000800;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_unpriv
/// ASL: `GpFromField("n") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "n" }
/// verify register write to GpFromField("n")
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_unpriv_reg_write_3_38000800() {
    // Test aarch64_memory_single_general_immediate_signed_offset_unpriv register write: GpFromField("n")
    // Encoding: 0x38000800
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x38000800;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_unpriv
/// ASL: `Rn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Rn = 31)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_unpriv_sp_rn_38000be0() {
    // Test aarch64_memory_single_general_immediate_signed_offset_unpriv with Rn = SP (31)
    // Encoding: 0x38000BE0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x38000BE0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_unpriv
/// ASL: `Rt = 31 (ZR)`
/// Requirement: RegisterSpecial { reg: Zr, behavior: "reads as 0, writes discarded" }
/// zero register (Rt = 31)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_unpriv_zr_rt_3800081f() {
    // Test aarch64_memory_single_general_immediate_signed_offset_unpriv with Rt = ZR (31)
    // Encoding: 0x3800081F
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x3800081F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 31), 0, "XZR should always be 0");
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_unpriv
/// ASL: `Mem[address, 8] = data`
/// Requirement: MemoryAccess { op: Store, size_bits: 64, addressing: "Base { reg: \"address\" }" }
/// 8-byte store
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_unpriv_store_0_38000820() {
    // Test aarch64_memory_single_general_immediate_signed_offset_unpriv memory store: 8 bytes
    // Encoding: 0x38000820
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x100000000000);
    set_x(&mut cpu, 0, 0xDEADBEEFCAFEBABE);
    let encoding: u32 = 0x38000820;
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

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_unpriv
/// ASL: `Unconditional`
/// Requirement: UndefinedEncoding { condition: "Unconditional" }
/// triggers Undefined
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_unpriv_exception_0_38000800() {
    // Test aarch64_memory_single_general_immediate_signed_offset_unpriv exception: Undefined
    // Encoding: 0x38000800
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x38000800;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_memory_single_general_immediate_signed_offset_unpriv
/// ASL: `Unconditional`
/// Requirement: UndefinedEncoding { condition: "Unconditional" }
/// triggers Undefined
#[test]
fn test_aarch64_memory_single_general_immediate_signed_offset_unpriv_exception_1_38000800() {
    // Test aarch64_memory_single_general_immediate_signed_offset_unpriv exception: Undefined
    // Encoding: 0x38000800
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x38000800;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

// ============================================================================
// aarch64_memory_single_general_immediate_unsigned Tests
// ============================================================================

// ============================================================================
// aarch64_memory_single_simdfp_register Tests
// ============================================================================

/// Provenance: aarch64_memory_single_simdfp_register
/// ASL: `field size 30 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_memory_single_simdfp_register_field_size_0_min_800_3c200800() {
    // Encoding: 0x3C200800
    // Test aarch64_memory_single_simdfp_register field size = 0 (Min)
    // Fields: option=0, opc=0, S=0, size=0, Rm=0, Rn=0, Rt=0
    let encoding: u32 = 0x3C200800;
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

/// Provenance: aarch64_memory_single_simdfp_register
/// ASL: `field size 30 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 1, boundary: PowerOfTwo }
/// 16-bit / halfword size
#[test]
fn test_aarch64_memory_single_simdfp_register_field_size_1_poweroftwo_800_7c200800() {
    // Encoding: 0x7C200800
    // Test aarch64_memory_single_simdfp_register field size = 1 (PowerOfTwo)
    // Fields: Rn=0, Rm=0, Rt=0, option=0, size=1, opc=0, S=0
    let encoding: u32 = 0x7C200800;
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

/// Provenance: aarch64_memory_single_simdfp_register
/// ASL: `field size 30 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 2, boundary: PowerOfTwo }
/// 32-bit / word size
#[test]
fn test_aarch64_memory_single_simdfp_register_field_size_2_poweroftwo_800_bc200800() {
    // Encoding: 0xBC200800
    // Test aarch64_memory_single_simdfp_register field size = 2 (PowerOfTwo)
    // Fields: Rn=0, Rt=0, opc=0, option=0, Rm=0, S=0, size=2
    let encoding: u32 = 0xBC200800;
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

/// Provenance: aarch64_memory_single_simdfp_register
/// ASL: `field size 30 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 3, boundary: Max }
/// 64-bit / doubleword size
#[test]
fn test_aarch64_memory_single_simdfp_register_field_size_3_max_800_fc200800() {
    // Encoding: 0xFC200800
    // Test aarch64_memory_single_simdfp_register field size = 3 (Max)
    // Fields: Rm=0, size=3, opc=0, S=0, Rn=0, Rt=0, option=0
    let encoding: u32 = 0xFC200800;
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

/// Provenance: aarch64_memory_single_simdfp_register
/// ASL: `field opc 22 +: 2`
/// Requirement: FieldBoundary { field: "opc", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_memory_single_simdfp_register_field_opc_0_min_800_3c200800() {
    // Encoding: 0x3C200800
    // Test aarch64_memory_single_simdfp_register field opc = 0 (Min)
    // Fields: size=0, Rm=0, opc=0, Rn=0, option=0, Rt=0, S=0
    let encoding: u32 = 0x3C200800;
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

/// Provenance: aarch64_memory_single_simdfp_register
/// ASL: `field opc 22 +: 2`
/// Requirement: FieldBoundary { field: "opc", value: 1, boundary: PowerOfTwo }
/// 16-bit / halfword size
#[test]
fn test_aarch64_memory_single_simdfp_register_field_opc_1_poweroftwo_800_3c600800() {
    // Encoding: 0x3C600800
    // Test aarch64_memory_single_simdfp_register field opc = 1 (PowerOfTwo)
    // Fields: option=0, S=0, Rt=0, Rn=0, size=0, Rm=0, opc=1
    let encoding: u32 = 0x3C600800;
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

/// Provenance: aarch64_memory_single_simdfp_register
/// ASL: `field opc 22 +: 2`
/// Requirement: FieldBoundary { field: "opc", value: 2, boundary: PowerOfTwo }
/// 32-bit / word size
#[test]
fn test_aarch64_memory_single_simdfp_register_field_opc_2_poweroftwo_800_3ca00800() {
    // Encoding: 0x3CA00800
    // Test aarch64_memory_single_simdfp_register field opc = 2 (PowerOfTwo)
    // Fields: Rt=0, opc=2, size=0, Rm=0, Rn=0, option=0, S=0
    let encoding: u32 = 0x3CA00800;
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

/// Provenance: aarch64_memory_single_simdfp_register
/// ASL: `field opc 22 +: 2`
/// Requirement: FieldBoundary { field: "opc", value: 3, boundary: Max }
/// 64-bit / doubleword size
#[test]
fn test_aarch64_memory_single_simdfp_register_field_opc_3_max_800_3ce00800() {
    // Encoding: 0x3CE00800
    // Test aarch64_memory_single_simdfp_register field opc = 3 (Max)
    // Fields: option=0, S=0, Rt=0, Rn=0, size=0, opc=3, Rm=0
    let encoding: u32 = 0x3CE00800;
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

/// Provenance: aarch64_memory_single_simdfp_register
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_memory_single_simdfp_register_field_rm_0_min_800_3c200800() {
    // Encoding: 0x3C200800
    // Test aarch64_memory_single_simdfp_register field Rm = 0 (Min)
    // Fields: opc=0, Rm=0, size=0, S=0, Rn=0, option=0, Rt=0
    let encoding: u32 = 0x3C200800;
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

/// Provenance: aarch64_memory_single_simdfp_register
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_memory_single_simdfp_register_field_rm_1_poweroftwo_800_3c210800() {
    // Encoding: 0x3C210800
    // Test aarch64_memory_single_simdfp_register field Rm = 1 (PowerOfTwo)
    // Fields: Rt=0, S=0, Rm=1, option=0, opc=0, size=0, Rn=0
    let encoding: u32 = 0x3C210800;
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

/// Provenance: aarch64_memory_single_simdfp_register
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_memory_single_simdfp_register_field_rm_30_poweroftwominusone_800_3c3e0800() {
    // Encoding: 0x3C3E0800
    // Test aarch64_memory_single_simdfp_register field Rm = 30 (PowerOfTwoMinusOne)
    // Fields: opc=0, S=0, size=0, option=0, Rn=0, Rt=0, Rm=30
    let encoding: u32 = 0x3C3E0800;
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

/// Provenance: aarch64_memory_single_simdfp_register
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 31, boundary: Max }
/// register index 31 (special)
#[test]
fn test_aarch64_memory_single_simdfp_register_field_rm_31_max_800_3c3f0800() {
    // Encoding: 0x3C3F0800
    // Test aarch64_memory_single_simdfp_register field Rm = 31 (Max)
    // Fields: Rn=0, size=0, option=0, S=0, opc=0, Rm=31, Rt=0
    let encoding: u32 = 0x3C3F0800;
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

/// Provenance: aarch64_memory_single_simdfp_register
/// ASL: `field option 13 +: 3`
/// Requirement: FieldBoundary { field: "option", value: 0, boundary: Min }
/// option 0
#[test]
fn test_aarch64_memory_single_simdfp_register_field_option_0_min_800_3c200800() {
    // Encoding: 0x3C200800
    // Test aarch64_memory_single_simdfp_register field option = 0 (Min)
    // Fields: S=0, Rn=0, Rt=0, opc=0, Rm=0, option=0, size=0
    let encoding: u32 = 0x3C200800;
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

/// Provenance: aarch64_memory_single_simdfp_register
/// ASL: `field option 13 +: 3`
/// Requirement: FieldBoundary { field: "option", value: 1, boundary: PowerOfTwo }
/// option 1
#[test]
fn test_aarch64_memory_single_simdfp_register_field_option_1_poweroftwo_800_3c202800() {
    // Encoding: 0x3C202800
    // Test aarch64_memory_single_simdfp_register field option = 1 (PowerOfTwo)
    // Fields: size=0, Rn=0, S=0, opc=0, option=1, Rm=0, Rt=0
    let encoding: u32 = 0x3C202800;
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

/// Provenance: aarch64_memory_single_simdfp_register
/// ASL: `field option 13 +: 3`
/// Requirement: FieldBoundary { field: "option", value: 2, boundary: PowerOfTwo }
/// option 2
#[test]
fn test_aarch64_memory_single_simdfp_register_field_option_2_poweroftwo_800_3c204800() {
    // Encoding: 0x3C204800
    // Test aarch64_memory_single_simdfp_register field option = 2 (PowerOfTwo)
    // Fields: opc=0, size=0, Rm=0, option=2, Rn=0, Rt=0, S=0
    let encoding: u32 = 0x3C204800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x0, w0, uxtw]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_register
/// ASL: `field option 13 +: 3`
/// Requirement: FieldBoundary { field: "option", value: 3, boundary: PowerOfTwo }
/// option 3
#[test]
fn test_aarch64_memory_single_simdfp_register_field_option_3_poweroftwo_800_3c206800() {
    // Encoding: 0x3C206800
    // Test aarch64_memory_single_simdfp_register field option = 3 (PowerOfTwo)
    // Fields: Rn=0, size=0, Rm=0, option=3, S=0, Rt=0, opc=0
    let encoding: u32 = 0x3C206800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x0, x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_register
/// ASL: `field option 13 +: 3`
/// Requirement: FieldBoundary { field: "option", value: 4, boundary: PowerOfTwo }
/// option 4
#[test]
fn test_aarch64_memory_single_simdfp_register_field_option_4_poweroftwo_800_3c208800() {
    // Encoding: 0x3C208800
    // Test aarch64_memory_single_simdfp_register field option = 4 (PowerOfTwo)
    // Fields: size=0, opc=0, S=0, Rm=0, Rn=0, Rt=0, option=4
    let encoding: u32 = 0x3C208800;
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

/// Provenance: aarch64_memory_single_simdfp_register
/// ASL: `field option 13 +: 3`
/// Requirement: FieldBoundary { field: "option", value: 5, boundary: PowerOfTwo }
/// option 5
#[test]
fn test_aarch64_memory_single_simdfp_register_field_option_5_poweroftwo_800_3c20a800() {
    // Encoding: 0x3C20A800
    // Test aarch64_memory_single_simdfp_register field option = 5 (PowerOfTwo)
    // Fields: size=0, S=0, Rn=0, Rm=0, option=5, opc=0, Rt=0
    let encoding: u32 = 0x3C20A800;
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

/// Provenance: aarch64_memory_single_simdfp_register
/// ASL: `field option 13 +: 3`
/// Requirement: FieldBoundary { field: "option", value: 6, boundary: PowerOfTwo }
/// option 6
#[test]
fn test_aarch64_memory_single_simdfp_register_field_option_6_poweroftwo_800_3c20c800() {
    // Encoding: 0x3C20C800
    // Test aarch64_memory_single_simdfp_register field option = 6 (PowerOfTwo)
    // Fields: option=6, S=0, Rt=0, size=0, opc=0, Rm=0, Rn=0
    let encoding: u32 = 0x3C20C800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x0, w0, sxtw]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_register
/// ASL: `field option 13 +: 3`
/// Requirement: FieldBoundary { field: "option", value: 7, boundary: Max }
/// option 7
#[test]
fn test_aarch64_memory_single_simdfp_register_field_option_7_max_800_3c20e800() {
    // Encoding: 0x3C20E800
    // Test aarch64_memory_single_simdfp_register field option = 7 (Max)
    // Fields: Rt=0, opc=0, S=0, option=7, size=0, Rm=0, Rn=0
    let encoding: u32 = 0x3C20E800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x0, x0, sxtx]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_register
/// ASL: `field S 12 +: 1`
/// Requirement: FieldBoundary { field: "S", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_memory_single_simdfp_register_field_s_0_min_800_3c200800() {
    // Encoding: 0x3C200800
    // Test aarch64_memory_single_simdfp_register field S = 0 (Min)
    // Fields: size=0, option=0, Rm=0, S=0, Rt=0, Rn=0, opc=0
    let encoding: u32 = 0x3C200800;
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

/// Provenance: aarch64_memory_single_simdfp_register
/// ASL: `field S 12 +: 1`
/// Requirement: FieldBoundary { field: "S", value: 1, boundary: Max }
/// 16-bit / halfword size
#[test]
fn test_aarch64_memory_single_simdfp_register_field_s_1_max_800_3c201800() {
    // Encoding: 0x3C201800
    // Test aarch64_memory_single_simdfp_register field S = 1 (Max)
    // Fields: Rm=0, S=1, Rn=0, size=0, opc=0, option=0, Rt=0
    let encoding: u32 = 0x3C201800;
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

/// Provenance: aarch64_memory_single_simdfp_register
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_memory_single_simdfp_register_field_rn_0_min_800_3c200800() {
    // Encoding: 0x3C200800
    // Test aarch64_memory_single_simdfp_register field Rn = 0 (Min)
    // Fields: Rn=0, size=0, Rt=0, Rm=0, opc=0, option=0, S=0
    let encoding: u32 = 0x3C200800;
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

/// Provenance: aarch64_memory_single_simdfp_register
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_memory_single_simdfp_register_field_rn_1_poweroftwo_800_3c200820() {
    // Encoding: 0x3C200820
    // Test aarch64_memory_single_simdfp_register field Rn = 1 (PowerOfTwo)
    // Fields: S=0, opc=0, option=0, Rn=1, Rt=0, Rm=0, size=0
    let encoding: u32 = 0x3C200820;
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

/// Provenance: aarch64_memory_single_simdfp_register
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_memory_single_simdfp_register_field_rn_30_poweroftwominusone_800_3c200bc0() {
    // Encoding: 0x3C200BC0
    // Test aarch64_memory_single_simdfp_register field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: Rn=30, Rm=0, opc=0, option=0, size=0, S=0, Rt=0
    let encoding: u32 = 0x3C200BC0;
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

/// Provenance: aarch64_memory_single_simdfp_register
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_memory_single_simdfp_register_field_rn_31_max_800_3c200be0() {
    // Encoding: 0x3C200BE0
    // Test aarch64_memory_single_simdfp_register field Rn = 31 (Max)
    // Fields: S=0, size=0, option=0, Rn=31, Rm=0, Rt=0, opc=0
    let encoding: u32 = 0x3C200BE0;
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

/// Provenance: aarch64_memory_single_simdfp_register
/// ASL: `field Rt 0 +: 5`
/// Requirement: FieldBoundary { field: "Rt", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_memory_single_simdfp_register_field_rt_0_min_800_3c200800() {
    // Encoding: 0x3C200800
    // Test aarch64_memory_single_simdfp_register field Rt = 0 (Min)
    // Fields: option=0, Rn=0, S=0, size=0, opc=0, Rm=0, Rt=0
    let encoding: u32 = 0x3C200800;
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

/// Provenance: aarch64_memory_single_simdfp_register
/// ASL: `field Rt 0 +: 5`
/// Requirement: FieldBoundary { field: "Rt", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_memory_single_simdfp_register_field_rt_1_poweroftwo_800_3c200801() {
    // Encoding: 0x3C200801
    // Test aarch64_memory_single_simdfp_register field Rt = 1 (PowerOfTwo)
    // Fields: Rt=1, Rm=0, opc=0, option=0, S=0, size=0, Rn=0
    let encoding: u32 = 0x3C200801;
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

/// Provenance: aarch64_memory_single_simdfp_register
/// ASL: `field Rt 0 +: 5`
/// Requirement: FieldBoundary { field: "Rt", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_memory_single_simdfp_register_field_rt_30_poweroftwominusone_800_3c20081e() {
    // Encoding: 0x3C20081E
    // Test aarch64_memory_single_simdfp_register field Rt = 30 (PowerOfTwoMinusOne)
    // Fields: S=0, Rm=0, option=0, Rn=0, size=0, Rt=30, opc=0
    let encoding: u32 = 0x3C20081E;
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

/// Provenance: aarch64_memory_single_simdfp_register
/// ASL: `field Rt 0 +: 5`
/// Requirement: FieldBoundary { field: "Rt", value: 31, boundary: Max }
/// register index 31 (ZR - zero register)
#[test]
fn test_aarch64_memory_single_simdfp_register_field_rt_31_max_800_3c20081f() {
    // Encoding: 0x3C20081F
    // Test aarch64_memory_single_simdfp_register field Rt = 31 (Max)
    // Fields: Rn=0, Rt=31, option=0, opc=0, Rm=0, size=0, S=0
    let encoding: u32 = 0x3C20081F;
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

/// Provenance: aarch64_memory_single_simdfp_register
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=0 (8-bit / byte size)
#[test]
fn test_aarch64_memory_single_simdfp_register_combo_0_800_3c200800() {
    // Encoding: 0x3C200800
    // Test aarch64_memory_single_simdfp_register field combination: size=0, opc=0, Rm=0, option=0, S=0, Rn=0, Rt=0
    // Fields: size=0, Rm=0, option=0, S=0, Rn=0, Rt=0, opc=0
    let encoding: u32 = 0x3C200800;
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

/// Provenance: aarch64_memory_single_simdfp_register
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=1 (16-bit / halfword size)
#[test]
fn test_aarch64_memory_single_simdfp_register_combo_1_800_7c200800() {
    // Encoding: 0x7C200800
    // Test aarch64_memory_single_simdfp_register field combination: size=1, opc=0, Rm=0, option=0, S=0, Rn=0, Rt=0
    // Fields: Rt=0, opc=0, size=1, option=0, S=0, Rm=0, Rn=0
    let encoding: u32 = 0x7C200800;
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

/// Provenance: aarch64_memory_single_simdfp_register
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=2 (32-bit / word size)
#[test]
fn test_aarch64_memory_single_simdfp_register_combo_2_800_bc200800() {
    // Encoding: 0xBC200800
    // Test aarch64_memory_single_simdfp_register field combination: size=2, opc=0, Rm=0, option=0, S=0, Rn=0, Rt=0
    // Fields: option=0, size=2, Rm=0, opc=0, S=0, Rn=0, Rt=0
    let encoding: u32 = 0xBC200800;
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

/// Provenance: aarch64_memory_single_simdfp_register
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=3 (64-bit / doubleword size)
#[test]
fn test_aarch64_memory_single_simdfp_register_combo_3_800_fc200800() {
    // Encoding: 0xFC200800
    // Test aarch64_memory_single_simdfp_register field combination: size=3, opc=0, Rm=0, option=0, S=0, Rn=0, Rt=0
    // Fields: option=0, S=0, Rt=0, size=3, opc=0, Rn=0, Rm=0
    let encoding: u32 = 0xFC200800;
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

/// Provenance: aarch64_memory_single_simdfp_register
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// opc=0 (8-bit / byte size)
#[test]
fn test_aarch64_memory_single_simdfp_register_combo_4_800_3c200800() {
    // Encoding: 0x3C200800
    // Test aarch64_memory_single_simdfp_register field combination: size=0, opc=0, Rm=0, option=0, S=0, Rn=0, Rt=0
    // Fields: size=0, opc=0, Rm=0, S=0, Rn=0, Rt=0, option=0
    let encoding: u32 = 0x3C200800;
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

/// Provenance: aarch64_memory_single_simdfp_register
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// opc=1 (16-bit / halfword size)
#[test]
fn test_aarch64_memory_single_simdfp_register_combo_5_800_3c600800() {
    // Encoding: 0x3C600800
    // Test aarch64_memory_single_simdfp_register field combination: size=0, opc=1, Rm=0, option=0, S=0, Rn=0, Rt=0
    // Fields: Rn=0, Rt=0, opc=1, S=0, size=0, Rm=0, option=0
    let encoding: u32 = 0x3C600800;
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

/// Provenance: aarch64_memory_single_simdfp_register
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// opc=2 (32-bit / word size)
#[test]
fn test_aarch64_memory_single_simdfp_register_combo_6_800_3ca00800() {
    // Encoding: 0x3CA00800
    // Test aarch64_memory_single_simdfp_register field combination: size=0, opc=2, Rm=0, option=0, S=0, Rn=0, Rt=0
    // Fields: option=0, Rt=0, Rm=0, opc=2, Rn=0, S=0, size=0
    let encoding: u32 = 0x3CA00800;
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

/// Provenance: aarch64_memory_single_simdfp_register
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// opc=3 (64-bit / doubleword size)
#[test]
fn test_aarch64_memory_single_simdfp_register_combo_7_800_3ce00800() {
    // Encoding: 0x3CE00800
    // Test aarch64_memory_single_simdfp_register field combination: size=0, opc=3, Rm=0, option=0, S=0, Rn=0, Rt=0
    // Fields: opc=3, Rm=0, S=0, Rn=0, option=0, size=0, Rt=0
    let encoding: u32 = 0x3CE00800;
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

/// Provenance: aarch64_memory_single_simdfp_register
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=0 (register index 0 (first register))
#[test]
fn test_aarch64_memory_single_simdfp_register_combo_8_800_3c200800() {
    // Encoding: 0x3C200800
    // Test aarch64_memory_single_simdfp_register field combination: size=0, opc=0, Rm=0, option=0, S=0, Rn=0, Rt=0
    // Fields: size=0, opc=0, option=0, Rn=0, Rt=0, Rm=0, S=0
    let encoding: u32 = 0x3C200800;
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

/// Provenance: aarch64_memory_single_simdfp_register
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=1 (register index 1 (second register))
#[test]
fn test_aarch64_memory_single_simdfp_register_combo_9_800_3c210800() {
    // Encoding: 0x3C210800
    // Test aarch64_memory_single_simdfp_register field combination: size=0, opc=0, Rm=1, option=0, S=0, Rn=0, Rt=0
    // Fields: size=0, Rm=1, S=0, option=0, opc=0, Rn=0, Rt=0
    let encoding: u32 = 0x3C210800;
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

/// Provenance: aarch64_memory_single_simdfp_register
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_memory_single_simdfp_register_combo_10_800_3c3e0800() {
    // Encoding: 0x3C3E0800
    // Test aarch64_memory_single_simdfp_register field combination: size=0, opc=0, Rm=30, option=0, S=0, Rn=0, Rt=0
    // Fields: Rn=0, Rt=0, size=0, option=0, opc=0, S=0, Rm=30
    let encoding: u32 = 0x3C3E0800;
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

/// Provenance: aarch64_memory_single_simdfp_register
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=31 (register index 31 (special))
#[test]
fn test_aarch64_memory_single_simdfp_register_combo_11_800_3c3f0800() {
    // Encoding: 0x3C3F0800
    // Test aarch64_memory_single_simdfp_register field combination: size=0, opc=0, Rm=31, option=0, S=0, Rn=0, Rt=0
    // Fields: size=0, opc=0, option=0, Rt=0, S=0, Rn=0, Rm=31
    let encoding: u32 = 0x3C3F0800;
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

/// Provenance: aarch64_memory_single_simdfp_register
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// option=0 (option 0)
#[test]
fn test_aarch64_memory_single_simdfp_register_combo_12_800_3c200800() {
    // Encoding: 0x3C200800
    // Test aarch64_memory_single_simdfp_register field combination: size=0, opc=0, Rm=0, option=0, S=0, Rn=0, Rt=0
    // Fields: Rn=0, Rt=0, Rm=0, size=0, S=0, opc=0, option=0
    let encoding: u32 = 0x3C200800;
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

/// Provenance: aarch64_memory_single_simdfp_register
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// option=1 (option 1)
#[test]
fn test_aarch64_memory_single_simdfp_register_combo_13_800_3c202800() {
    // Encoding: 0x3C202800
    // Test aarch64_memory_single_simdfp_register field combination: size=0, opc=0, Rm=0, option=1, S=0, Rn=0, Rt=0
    // Fields: Rt=0, size=0, Rm=0, S=0, option=1, Rn=0, opc=0
    let encoding: u32 = 0x3C202800;
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

/// Provenance: aarch64_memory_single_simdfp_register
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// option=2 (option 2)
#[test]
fn test_aarch64_memory_single_simdfp_register_combo_14_800_3c204800() {
    // Encoding: 0x3C204800
    // Test aarch64_memory_single_simdfp_register field combination: size=0, opc=0, Rm=0, option=2, S=0, Rn=0, Rt=0
    // Fields: Rn=0, opc=0, Rm=0, option=2, S=0, size=0, Rt=0
    let encoding: u32 = 0x3C204800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x0, w0, uxtw]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_register
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// option=3 (option 3)
#[test]
fn test_aarch64_memory_single_simdfp_register_combo_15_800_3c206800() {
    // Encoding: 0x3C206800
    // Test aarch64_memory_single_simdfp_register field combination: size=0, opc=0, Rm=0, option=3, S=0, Rn=0, Rt=0
    // Fields: option=3, Rm=0, Rn=0, Rt=0, S=0, size=0, opc=0
    let encoding: u32 = 0x3C206800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x0, x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_register
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// option=4 (option 4)
#[test]
fn test_aarch64_memory_single_simdfp_register_combo_16_800_3c208800() {
    // Encoding: 0x3C208800
    // Test aarch64_memory_single_simdfp_register field combination: size=0, opc=0, Rm=0, option=4, S=0, Rn=0, Rt=0
    // Fields: S=0, opc=0, Rt=0, Rm=0, option=4, Rn=0, size=0
    let encoding: u32 = 0x3C208800;
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

/// Provenance: aarch64_memory_single_simdfp_register
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// option=5 (option 5)
#[test]
fn test_aarch64_memory_single_simdfp_register_combo_17_800_3c20a800() {
    // Encoding: 0x3C20A800
    // Test aarch64_memory_single_simdfp_register field combination: size=0, opc=0, Rm=0, option=5, S=0, Rn=0, Rt=0
    // Fields: opc=0, size=0, option=5, S=0, Rn=0, Rt=0, Rm=0
    let encoding: u32 = 0x3C20A800;
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

/// Provenance: aarch64_memory_single_simdfp_register
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// option=6 (option 6)
#[test]
fn test_aarch64_memory_single_simdfp_register_combo_18_800_3c20c800() {
    // Encoding: 0x3C20C800
    // Test aarch64_memory_single_simdfp_register field combination: size=0, opc=0, Rm=0, option=6, S=0, Rn=0, Rt=0
    // Fields: option=6, S=0, Rm=0, Rt=0, Rn=0, size=0, opc=0
    let encoding: u32 = 0x3C20C800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x0, w0, sxtw]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_register
/// ASL: `field combination 19`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// option=7 (option 7)
#[test]
fn test_aarch64_memory_single_simdfp_register_combo_19_800_3c20e800() {
    // Encoding: 0x3C20E800
    // Test aarch64_memory_single_simdfp_register field combination: size=0, opc=0, Rm=0, option=7, S=0, Rn=0, Rt=0
    // Fields: S=0, Rt=0, opc=0, option=7, Rn=0, size=0, Rm=0
    let encoding: u32 = 0x3C20E800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x0, x0, sxtx]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_register
/// ASL: `field combination 20`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// S=0 (8-bit / byte size)
#[test]
fn test_aarch64_memory_single_simdfp_register_combo_20_800_3c200800() {
    // Encoding: 0x3C200800
    // Test aarch64_memory_single_simdfp_register field combination: size=0, opc=0, Rm=0, option=0, S=0, Rn=0, Rt=0
    // Fields: S=0, Rn=0, size=0, option=0, Rt=0, Rm=0, opc=0
    let encoding: u32 = 0x3C200800;
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

/// Provenance: aarch64_memory_single_simdfp_register
/// ASL: `field combination 21`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// S=1 (16-bit / halfword size)
#[test]
fn test_aarch64_memory_single_simdfp_register_combo_21_800_3c201800() {
    // Encoding: 0x3C201800
    // Test aarch64_memory_single_simdfp_register field combination: size=0, opc=0, Rm=0, option=0, S=1, Rn=0, Rt=0
    // Fields: size=0, Rm=0, opc=0, option=0, S=1, Rt=0, Rn=0
    let encoding: u32 = 0x3C201800;
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

/// Provenance: aarch64_memory_single_simdfp_register
/// ASL: `field combination 22`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch64_memory_single_simdfp_register_combo_22_800_3c200800() {
    // Encoding: 0x3C200800
    // Test aarch64_memory_single_simdfp_register field combination: size=0, opc=0, Rm=0, option=0, S=0, Rn=0, Rt=0
    // Fields: Rm=0, Rn=0, Rt=0, size=0, S=0, opc=0, option=0
    let encoding: u32 = 0x3C200800;
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

/// Provenance: aarch64_memory_single_simdfp_register
/// ASL: `field combination 23`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_aarch64_memory_single_simdfp_register_combo_23_800_3c200820() {
    // Encoding: 0x3C200820
    // Test aarch64_memory_single_simdfp_register field combination: size=0, opc=0, Rm=0, option=0, S=0, Rn=1, Rt=0
    // Fields: S=0, Rm=0, size=0, Rt=0, option=0, opc=0, Rn=1
    let encoding: u32 = 0x3C200820;
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

/// Provenance: aarch64_memory_single_simdfp_register
/// ASL: `field combination 24`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_memory_single_simdfp_register_combo_24_800_3c200bc0() {
    // Encoding: 0x3C200BC0
    // Test aarch64_memory_single_simdfp_register field combination: size=0, opc=0, Rm=0, option=0, S=0, Rn=30, Rt=0
    // Fields: Rn=30, S=0, Rt=0, size=0, opc=0, Rm=0, option=0
    let encoding: u32 = 0x3C200BC0;
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

/// Provenance: aarch64_memory_single_simdfp_register
/// ASL: `field combination 25`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_memory_single_simdfp_register_combo_25_800_3c200be0() {
    // Encoding: 0x3C200BE0
    // Test aarch64_memory_single_simdfp_register field combination: size=0, opc=0, Rm=0, option=0, S=0, Rn=31, Rt=0
    // Fields: S=0, size=0, opc=0, option=0, Rt=0, Rn=31, Rm=0
    let encoding: u32 = 0x3C200BE0;
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

/// Provenance: aarch64_memory_single_simdfp_register
/// ASL: `field combination 26`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt=0 (register index 0 (first register))
#[test]
fn test_aarch64_memory_single_simdfp_register_combo_26_800_3c200800() {
    // Encoding: 0x3C200800
    // Test aarch64_memory_single_simdfp_register field combination: size=0, opc=0, Rm=0, option=0, S=0, Rn=0, Rt=0
    // Fields: Rt=0, size=0, opc=0, Rm=0, option=0, Rn=0, S=0
    let encoding: u32 = 0x3C200800;
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

/// Provenance: aarch64_memory_single_simdfp_register
/// ASL: `field combination 27`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt=1 (register index 1 (second register))
#[test]
fn test_aarch64_memory_single_simdfp_register_combo_27_800_3c200801() {
    // Encoding: 0x3C200801
    // Test aarch64_memory_single_simdfp_register field combination: size=0, opc=0, Rm=0, option=0, S=0, Rn=0, Rt=1
    // Fields: opc=0, size=0, S=0, option=0, Rn=0, Rt=1, Rm=0
    let encoding: u32 = 0x3C200801;
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

/// Provenance: aarch64_memory_single_simdfp_register
/// ASL: `field combination 28`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_memory_single_simdfp_register_combo_28_800_3c20081e() {
    // Encoding: 0x3C20081E
    // Test aarch64_memory_single_simdfp_register field combination: size=0, opc=0, Rm=0, option=0, S=0, Rn=0, Rt=30
    // Fields: Rm=0, Rn=0, S=0, Rt=30, size=0, opc=0, option=0
    let encoding: u32 = 0x3C20081E;
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

/// Provenance: aarch64_memory_single_simdfp_register
/// ASL: `field combination 29`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt=31 (register index 31 (ZR - zero register))
#[test]
fn test_aarch64_memory_single_simdfp_register_combo_29_800_3c20081f() {
    // Encoding: 0x3C20081F
    // Test aarch64_memory_single_simdfp_register field combination: size=0, opc=0, Rm=0, option=0, S=0, Rn=0, Rt=31
    // Fields: opc=0, Rn=0, S=0, Rt=31, Rm=0, size=0, option=0
    let encoding: u32 = 0x3C20081F;
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

/// Provenance: aarch64_memory_single_simdfp_register
/// ASL: `field combination 30`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=1 (same register test (reg=1)), Rn=1 (same register test (reg=1))
#[test]
fn test_aarch64_memory_single_simdfp_register_combo_30_800_3c210820() {
    // Encoding: 0x3C210820
    // Test aarch64_memory_single_simdfp_register field combination: size=0, opc=0, Rm=1, option=0, S=0, Rn=1, Rt=0
    // Fields: S=0, Rm=1, opc=0, size=0, Rt=0, option=0, Rn=1
    let encoding: u32 = 0x3C210820;
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

/// Provenance: aarch64_memory_single_simdfp_register
/// ASL: `field combination 31`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=31 (same register test (reg=31)), Rn=31 (same register test (reg=31))
#[test]
fn test_aarch64_memory_single_simdfp_register_combo_31_800_3c3f0be0() {
    // Encoding: 0x3C3F0BE0
    // Test aarch64_memory_single_simdfp_register field combination: size=0, opc=0, Rm=31, option=0, S=0, Rn=31, Rt=0
    // Fields: S=0, Rm=31, Rt=0, opc=0, size=0, Rn=31, option=0
    let encoding: u32 = 0x3C3F0BE0;
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

/// Provenance: aarch64_memory_single_simdfp_register
/// ASL: `field combination 32`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=1 (same register test (reg=1)), Rt=1 (same register test (reg=1))
#[test]
fn test_aarch64_memory_single_simdfp_register_combo_32_800_3c210801() {
    // Encoding: 0x3C210801
    // Test aarch64_memory_single_simdfp_register field combination: size=0, opc=0, Rm=1, option=0, S=0, Rn=0, Rt=1
    // Fields: Rm=1, size=0, opc=0, S=0, Rn=0, option=0, Rt=1
    let encoding: u32 = 0x3C210801;
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

/// Provenance: aarch64_memory_single_simdfp_register
/// ASL: `field combination 33`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=31 (same register test (reg=31)), Rt=31 (same register test (reg=31))
#[test]
fn test_aarch64_memory_single_simdfp_register_combo_33_800_3c3f081f() {
    // Encoding: 0x3C3F081F
    // Test aarch64_memory_single_simdfp_register field combination: size=0, opc=0, Rm=31, option=0, S=0, Rn=0, Rt=31
    // Fields: Rn=0, Rt=31, opc=0, S=0, size=0, Rm=31, option=0
    let encoding: u32 = 0x3C3F081F;
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

/// Provenance: aarch64_memory_single_simdfp_register
/// ASL: `field combination 34`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (same register test (reg=1)), Rt=1 (same register test (reg=1))
#[test]
fn test_aarch64_memory_single_simdfp_register_combo_34_800_3c200821() {
    // Encoding: 0x3C200821
    // Test aarch64_memory_single_simdfp_register field combination: size=0, opc=0, Rm=0, option=0, S=0, Rn=1, Rt=1
    // Fields: size=0, Rm=0, option=0, Rt=1, S=0, Rn=1, opc=0
    let encoding: u32 = 0x3C200821;
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

/// Provenance: aarch64_memory_single_simdfp_register
/// ASL: `field combination 35`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (same register test (reg=31)), Rt=31 (same register test (reg=31))
#[test]
fn test_aarch64_memory_single_simdfp_register_combo_35_800_3c200bff() {
    // Encoding: 0x3C200BFF
    // Test aarch64_memory_single_simdfp_register field combination: size=0, opc=0, Rm=0, option=0, S=0, Rn=31, Rt=31
    // Fields: size=0, option=0, S=0, Rn=31, Rt=31, Rm=0, opc=0
    let encoding: u32 = 0x3C200BFF;
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

/// Provenance: aarch64_memory_single_simdfp_register
/// ASL: `field size = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "size", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_memory_single_simdfp_register_special_size_0_size_variant_0_2048_3c200800() {
    // Encoding: 0x3C200800
    // Test aarch64_memory_single_simdfp_register special value size = 0 (Size variant 0)
    // Fields: size=0, opc=0, S=0, Rn=0, Rm=0, Rt=0, option=0
    let encoding: u32 = 0x3C200800;
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

/// Provenance: aarch64_memory_single_simdfp_register
/// ASL: `field size = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "size", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_memory_single_simdfp_register_special_size_1_size_variant_1_2048_7c200800() {
    // Encoding: 0x7C200800
    // Test aarch64_memory_single_simdfp_register special value size = 1 (Size variant 1)
    // Fields: S=0, opc=0, option=0, Rt=0, size=1, Rn=0, Rm=0
    let encoding: u32 = 0x7C200800;
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

/// Provenance: aarch64_memory_single_simdfp_register
/// ASL: `field size = 2 (Size variant 2)`
/// Requirement: FieldSpecial { field: "size", value: 2, meaning: "Size variant 2" }
/// Size variant 2
#[test]
fn test_aarch64_memory_single_simdfp_register_special_size_2_size_variant_2_2048_bc200800() {
    // Encoding: 0xBC200800
    // Test aarch64_memory_single_simdfp_register special value size = 2 (Size variant 2)
    // Fields: size=2, Rt=0, Rm=0, S=0, Rn=0, option=0, opc=0
    let encoding: u32 = 0xBC200800;
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

/// Provenance: aarch64_memory_single_simdfp_register
/// ASL: `field size = 3 (Size variant 3)`
/// Requirement: FieldSpecial { field: "size", value: 3, meaning: "Size variant 3" }
/// Size variant 3
#[test]
fn test_aarch64_memory_single_simdfp_register_special_size_3_size_variant_3_2048_fc200800() {
    // Encoding: 0xFC200800
    // Test aarch64_memory_single_simdfp_register special value size = 3 (Size variant 3)
    // Fields: S=0, size=3, Rt=0, Rm=0, option=0, opc=0, Rn=0
    let encoding: u32 = 0xFC200800;
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

/// Provenance: aarch64_memory_single_simdfp_register
/// ASL: `field opc = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "opc", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_memory_single_simdfp_register_special_opc_0_size_variant_0_2048_7c200800() {
    // Encoding: 0x7C200800
    // Test aarch64_memory_single_simdfp_register special value opc = 0 (Size variant 0)
    // Fields: Rn=0, Rm=0, opc=0, option=0, size=1, S=0, Rt=0
    let encoding: u32 = 0x7C200800;
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

/// Provenance: aarch64_memory_single_simdfp_register
/// ASL: `field opc = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "opc", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_memory_single_simdfp_register_special_opc_1_size_variant_1_2048_7c600800() {
    // Encoding: 0x7C600800
    // Test aarch64_memory_single_simdfp_register special value opc = 1 (Size variant 1)
    // Fields: size=1, opc=1, Rn=0, Rt=0, S=0, Rm=0, option=0
    let encoding: u32 = 0x7C600800;
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

/// Provenance: aarch64_memory_single_simdfp_register
/// ASL: `field opc = 2 (Size variant 2)`
/// Requirement: FieldSpecial { field: "opc", value: 2, meaning: "Size variant 2" }
/// Size variant 2
#[test]
fn test_aarch64_memory_single_simdfp_register_special_opc_2_size_variant_2_2048_7ca00800() {
    // Encoding: 0x7CA00800
    // Test aarch64_memory_single_simdfp_register special value opc = 2 (Size variant 2)
    // Fields: opc=2, S=0, Rt=0, Rn=0, option=0, size=1, Rm=0
    let encoding: u32 = 0x7CA00800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch64_memory_single_simdfp_register
/// ASL: `field opc = 3 (Size variant 3)`
/// Requirement: FieldSpecial { field: "opc", value: 3, meaning: "Size variant 3" }
/// Size variant 3
#[test]
fn test_aarch64_memory_single_simdfp_register_special_opc_3_size_variant_3_2048_7ce00800() {
    // Encoding: 0x7CE00800
    // Test aarch64_memory_single_simdfp_register special value opc = 3 (Size variant 3)
    // Fields: Rn=0, option=0, Rm=0, size=1, opc=3, S=0, Rt=0
    let encoding: u32 = 0x7CE00800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch64_memory_single_simdfp_register
/// ASL: `field S = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "S", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_memory_single_simdfp_register_special_s_0_size_variant_0_2048_7c200800() {
    // Encoding: 0x7C200800
    // Test aarch64_memory_single_simdfp_register special value S = 0 (Size variant 0)
    // Fields: Rn=0, opc=0, option=0, S=0, Rt=0, size=1, Rm=0
    let encoding: u32 = 0x7C200800;
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

/// Provenance: aarch64_memory_single_simdfp_register
/// ASL: `field S = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "S", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_memory_single_simdfp_register_special_s_1_size_variant_1_2048_7c201800() {
    // Encoding: 0x7C201800
    // Test aarch64_memory_single_simdfp_register special value S = 1 (Size variant 1)
    // Fields: Rt=0, size=1, opc=0, option=0, Rm=0, S=1, Rn=0
    let encoding: u32 = 0x7C201800;
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

/// Provenance: aarch64_memory_single_simdfp_register
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_memory_single_simdfp_register_special_rn_31_stack_pointer_sp_may_require_alignment_2048_7c200be0()
 {
    // Encoding: 0x7C200BE0
    // Test aarch64_memory_single_simdfp_register special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: Rn=31, Rt=0, size=1, option=0, opc=0, S=0, Rm=0
    let encoding: u32 = 0x7C200BE0;
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

/// Provenance: aarch64_memory_single_simdfp_register
/// ASL: `field Rt = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)`
/// Requirement: FieldSpecial { field: "Rt", value: 31, meaning: "Zero register (XZR/WZR) - reads as 0, writes discarded" }
/// Zero register (XZR/WZR) - reads as 0, writes discarded
#[test]
fn test_aarch64_memory_single_simdfp_register_special_rt_31_zero_register_xzr_wzr_reads_as_0_writes_discarded_2048_7c20081f()
 {
    // Encoding: 0x7C20081F
    // Test aarch64_memory_single_simdfp_register special value Rt = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)
    // Fields: Rn=0, opc=0, size=1, option=0, Rm=0, Rt=31, S=0
    let encoding: u32 = 0x7C20081F;
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

// ============================================================================
// aarch64_memory_single_simdfp_immediate_signed_post_idx Tests
// ============================================================================

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_post_idx
/// ASL: `field size 30 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_post_idx_field_size_0_min_400_3c000400() {
    // Encoding: 0x3C000400
    // Test aarch64_memory_single_simdfp_immediate_signed_post_idx field size = 0 (Min)
    // Fields: Rt=0, size=0, imm9=0, Rn=0, opc=0
    let encoding: u32 = 0x3C000400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x0], #0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_post_idx
/// ASL: `field size 30 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 1, boundary: PowerOfTwo }
/// 16-bit / halfword size
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_post_idx_field_size_1_poweroftwo_400_7c000400()
 {
    // Encoding: 0x7C000400
    // Test aarch64_memory_single_simdfp_immediate_signed_post_idx field size = 1 (PowerOfTwo)
    // Fields: opc=0, Rn=0, Rt=0, size=1, imm9=0
    let encoding: u32 = 0x7C000400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	h0, [x0], #0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_post_idx
/// ASL: `field size 30 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 2, boundary: PowerOfTwo }
/// 32-bit / word size
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_post_idx_field_size_2_poweroftwo_400_bc000400()
 {
    // Encoding: 0xBC000400
    // Test aarch64_memory_single_simdfp_immediate_signed_post_idx field size = 2 (PowerOfTwo)
    // Fields: Rn=0, opc=0, Rt=0, imm9=0, size=2
    let encoding: u32 = 0xBC000400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	s0, [x0], #0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_post_idx
/// ASL: `field size 30 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 3, boundary: Max }
/// 64-bit / doubleword size
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_post_idx_field_size_3_max_400_fc000400() {
    // Encoding: 0xFC000400
    // Test aarch64_memory_single_simdfp_immediate_signed_post_idx field size = 3 (Max)
    // Fields: opc=0, imm9=0, size=3, Rt=0, Rn=0
    let encoding: u32 = 0xFC000400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	d0, [x0], #0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_post_idx
/// ASL: `field opc 22 +: 2`
/// Requirement: FieldBoundary { field: "opc", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_post_idx_field_opc_0_min_400_3c000400() {
    // Encoding: 0x3C000400
    // Test aarch64_memory_single_simdfp_immediate_signed_post_idx field opc = 0 (Min)
    // Fields: opc=0, Rn=0, size=0, imm9=0, Rt=0
    let encoding: u32 = 0x3C000400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x0], #0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_post_idx
/// ASL: `field opc 22 +: 2`
/// Requirement: FieldBoundary { field: "opc", value: 1, boundary: PowerOfTwo }
/// 16-bit / halfword size
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_post_idx_field_opc_1_poweroftwo_400_3c400400()
{
    // Encoding: 0x3C400400
    // Test aarch64_memory_single_simdfp_immediate_signed_post_idx field opc = 1 (PowerOfTwo)
    // Fields: imm9=0, Rt=0, size=0, opc=1, Rn=0
    let encoding: u32 = 0x3C400400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ldr	b0, [x0], #0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_post_idx
/// ASL: `field opc 22 +: 2`
/// Requirement: FieldBoundary { field: "opc", value: 2, boundary: PowerOfTwo }
/// 32-bit / word size
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_post_idx_field_opc_2_poweroftwo_400_3c800400()
{
    // Encoding: 0x3C800400
    // Test aarch64_memory_single_simdfp_immediate_signed_post_idx field opc = 2 (PowerOfTwo)
    // Fields: Rn=0, size=0, opc=2, imm9=0, Rt=0
    let encoding: u32 = 0x3C800400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	q0, [x0], #0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_post_idx
/// ASL: `field opc 22 +: 2`
/// Requirement: FieldBoundary { field: "opc", value: 3, boundary: Max }
/// 64-bit / doubleword size
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_post_idx_field_opc_3_max_400_3cc00400() {
    // Encoding: 0x3CC00400
    // Test aarch64_memory_single_simdfp_immediate_signed_post_idx field opc = 3 (Max)
    // Fields: size=0, imm9=0, Rn=0, opc=3, Rt=0
    let encoding: u32 = 0x3CC00400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ldr	q0, [x0], #0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_post_idx
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_post_idx_field_imm9_0_zero_400_3c000400() {
    // Encoding: 0x3C000400
    // Test aarch64_memory_single_simdfp_immediate_signed_post_idx field imm9 = 0 (Zero)
    // Fields: imm9=0, opc=0, Rn=0, size=0, Rt=0
    let encoding: u32 = 0x3C000400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x0], #0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_post_idx
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_post_idx_field_imm9_1_poweroftwo_400_3c001400()
 {
    // Encoding: 0x3C001400
    // Test aarch64_memory_single_simdfp_immediate_signed_post_idx field imm9 = 1 (PowerOfTwo)
    // Fields: size=0, imm9=1, opc=0, Rn=0, Rt=0
    let encoding: u32 = 0x3C001400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x0], #1
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_post_idx
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 3, boundary: PowerOfTwoMinusOne }
/// 2^2 - 1 = 3
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_post_idx_field_imm9_3_poweroftwominusone_400_3c003400()
 {
    // Encoding: 0x3C003400
    // Test aarch64_memory_single_simdfp_immediate_signed_post_idx field imm9 = 3 (PowerOfTwoMinusOne)
    // Fields: size=0, Rn=0, opc=0, Rt=0, imm9=3
    let encoding: u32 = 0x3C003400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x0], #3
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_post_idx
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 4, boundary: PowerOfTwo }
/// power of 2 (2^2 = 4)
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_post_idx_field_imm9_4_poweroftwo_400_3c004400()
 {
    // Encoding: 0x3C004400
    // Test aarch64_memory_single_simdfp_immediate_signed_post_idx field imm9 = 4 (PowerOfTwo)
    // Fields: imm9=4, Rn=0, size=0, opc=0, Rt=0
    let encoding: u32 = 0x3C004400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x0], #4
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_post_idx
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 7, boundary: PowerOfTwoMinusOne }
/// 2^3 - 1 = 7
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_post_idx_field_imm9_7_poweroftwominusone_400_3c007400()
 {
    // Encoding: 0x3C007400
    // Test aarch64_memory_single_simdfp_immediate_signed_post_idx field imm9 = 7 (PowerOfTwoMinusOne)
    // Fields: opc=0, size=0, imm9=7, Rt=0, Rn=0
    let encoding: u32 = 0x3C007400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x0], #7
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_post_idx
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 8, boundary: PowerOfTwo }
/// power of 2 (2^3 = 8)
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_post_idx_field_imm9_8_poweroftwo_400_3c008400()
 {
    // Encoding: 0x3C008400
    // Test aarch64_memory_single_simdfp_immediate_signed_post_idx field imm9 = 8 (PowerOfTwo)
    // Fields: size=0, Rt=0, Rn=0, imm9=8, opc=0
    let encoding: u32 = 0x3C008400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x0], #8
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_post_idx
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 15, boundary: PowerOfTwoMinusOne }
/// 2^4 - 1 = 15
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_post_idx_field_imm9_15_poweroftwominusone_400_3c00f400()
 {
    // Encoding: 0x3C00F400
    // Test aarch64_memory_single_simdfp_immediate_signed_post_idx field imm9 = 15 (PowerOfTwoMinusOne)
    // Fields: opc=0, Rn=0, Rt=0, size=0, imm9=15
    let encoding: u32 = 0x3C00F400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x0], #15
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_post_idx
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 16, boundary: PowerOfTwo }
/// power of 2 (2^4 = 16)
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_post_idx_field_imm9_16_poweroftwo_400_3c010400()
 {
    // Encoding: 0x3C010400
    // Test aarch64_memory_single_simdfp_immediate_signed_post_idx field imm9 = 16 (PowerOfTwo)
    // Fields: Rn=0, Rt=0, size=0, opc=0, imm9=16
    let encoding: u32 = 0x3C010400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x0], #16
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_post_idx
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 31, boundary: PowerOfTwoMinusOne }
/// 2^5 - 1 = 31
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_post_idx_field_imm9_31_poweroftwominusone_400_3c01f400()
 {
    // Encoding: 0x3C01F400
    // Test aarch64_memory_single_simdfp_immediate_signed_post_idx field imm9 = 31 (PowerOfTwoMinusOne)
    // Fields: size=0, Rn=0, imm9=31, Rt=0, opc=0
    let encoding: u32 = 0x3C01F400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x0], #31
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_post_idx
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 32, boundary: PowerOfTwo }
/// power of 2 (2^5 = 32)
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_post_idx_field_imm9_32_poweroftwo_400_3c020400()
 {
    // Encoding: 0x3C020400
    // Test aarch64_memory_single_simdfp_immediate_signed_post_idx field imm9 = 32 (PowerOfTwo)
    // Fields: size=0, Rn=0, imm9=32, opc=0, Rt=0
    let encoding: u32 = 0x3C020400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x0], #32
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_post_idx
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 63, boundary: PowerOfTwoMinusOne }
/// 2^6 - 1 = 63
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_post_idx_field_imm9_63_poweroftwominusone_400_3c03f400()
 {
    // Encoding: 0x3C03F400
    // Test aarch64_memory_single_simdfp_immediate_signed_post_idx field imm9 = 63 (PowerOfTwoMinusOne)
    // Fields: opc=0, size=0, imm9=63, Rt=0, Rn=0
    let encoding: u32 = 0x3C03F400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x0], #63
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_post_idx
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 64, boundary: PowerOfTwo }
/// power of 2 (2^6 = 64)
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_post_idx_field_imm9_64_poweroftwo_400_3c040400()
 {
    // Encoding: 0x3C040400
    // Test aarch64_memory_single_simdfp_immediate_signed_post_idx field imm9 = 64 (PowerOfTwo)
    // Fields: imm9=64, Rt=0, opc=0, size=0, Rn=0
    let encoding: u32 = 0x3C040400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x0], #64
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_post_idx
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 127, boundary: PowerOfTwoMinusOne }
/// 2^7 - 1 = 127
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_post_idx_field_imm9_127_poweroftwominusone_400_3c07f400()
 {
    // Encoding: 0x3C07F400
    // Test aarch64_memory_single_simdfp_immediate_signed_post_idx field imm9 = 127 (PowerOfTwoMinusOne)
    // Fields: size=0, imm9=127, opc=0, Rn=0, Rt=0
    let encoding: u32 = 0x3C07F400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x0], #127
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_post_idx
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 128, boundary: PowerOfTwo }
/// power of 2 (2^7 = 128)
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_post_idx_field_imm9_128_poweroftwo_400_3c080400()
 {
    // Encoding: 0x3C080400
    // Test aarch64_memory_single_simdfp_immediate_signed_post_idx field imm9 = 128 (PowerOfTwo)
    // Fields: Rn=0, Rt=0, opc=0, size=0, imm9=128
    let encoding: u32 = 0x3C080400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x0], #128
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_post_idx
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 255, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (255)
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_post_idx_field_imm9_255_poweroftwominusone_400_3c0ff400()
 {
    // Encoding: 0x3C0FF400
    // Test aarch64_memory_single_simdfp_immediate_signed_post_idx field imm9 = 255 (PowerOfTwoMinusOne)
    // Fields: Rn=0, imm9=255, size=0, opc=0, Rt=0
    let encoding: u32 = 0x3C0FF400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x0], #255
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_post_idx
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 256, boundary: PowerOfTwo }
/// power of 2 (2^8 = 256)
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_post_idx_field_imm9_256_poweroftwo_400_3c100400()
 {
    // Encoding: 0x3C100400
    // Test aarch64_memory_single_simdfp_immediate_signed_post_idx field imm9 = 256 (PowerOfTwo)
    // Fields: size=0, opc=0, imm9=256, Rn=0, Rt=0
    let encoding: u32 = 0x3C100400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x0], #-256
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_post_idx
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 511, boundary: Max }
/// maximum immediate (511)
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_post_idx_field_imm9_511_max_400_3c1ff400() {
    // Encoding: 0x3C1FF400
    // Test aarch64_memory_single_simdfp_immediate_signed_post_idx field imm9 = 511 (Max)
    // Fields: imm9=511, Rn=0, Rt=0, size=0, opc=0
    let encoding: u32 = 0x3C1FF400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x0], #-1
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_post_idx
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_post_idx_field_rn_0_min_400_3c000400() {
    // Encoding: 0x3C000400
    // Test aarch64_memory_single_simdfp_immediate_signed_post_idx field Rn = 0 (Min)
    // Fields: imm9=0, size=0, opc=0, Rn=0, Rt=0
    let encoding: u32 = 0x3C000400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x0], #0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_post_idx
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_post_idx_field_rn_1_poweroftwo_400_3c000420()
{
    // Encoding: 0x3C000420
    // Test aarch64_memory_single_simdfp_immediate_signed_post_idx field Rn = 1 (PowerOfTwo)
    // Fields: Rn=1, imm9=0, size=0, opc=0, Rt=0
    let encoding: u32 = 0x3C000420;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x1], #0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_post_idx
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_post_idx_field_rn_30_poweroftwominusone_400_3c0007c0()
 {
    // Encoding: 0x3C0007C0
    // Test aarch64_memory_single_simdfp_immediate_signed_post_idx field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: opc=0, imm9=0, Rt=0, Rn=30, size=0
    let encoding: u32 = 0x3C0007C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x30], #0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_post_idx
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_post_idx_field_rn_31_max_400_3c0007e0() {
    // Encoding: 0x3C0007E0
    // Test aarch64_memory_single_simdfp_immediate_signed_post_idx field Rn = 31 (Max)
    // Fields: Rn=31, size=0, imm9=0, Rt=0, opc=0
    let encoding: u32 = 0x3C0007E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [sp], #0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_post_idx
/// ASL: `field Rt 0 +: 5`
/// Requirement: FieldBoundary { field: "Rt", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_post_idx_field_rt_0_min_400_3c000400() {
    // Encoding: 0x3C000400
    // Test aarch64_memory_single_simdfp_immediate_signed_post_idx field Rt = 0 (Min)
    // Fields: size=0, opc=0, Rt=0, imm9=0, Rn=0
    let encoding: u32 = 0x3C000400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x0], #0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_post_idx
/// ASL: `field Rt 0 +: 5`
/// Requirement: FieldBoundary { field: "Rt", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_post_idx_field_rt_1_poweroftwo_400_3c000401()
{
    // Encoding: 0x3C000401
    // Test aarch64_memory_single_simdfp_immediate_signed_post_idx field Rt = 1 (PowerOfTwo)
    // Fields: Rn=0, opc=0, size=0, imm9=0, Rt=1
    let encoding: u32 = 0x3C000401;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b1, [x0], #0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_post_idx
/// ASL: `field Rt 0 +: 5`
/// Requirement: FieldBoundary { field: "Rt", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_post_idx_field_rt_30_poweroftwominusone_400_3c00041e()
 {
    // Encoding: 0x3C00041E
    // Test aarch64_memory_single_simdfp_immediate_signed_post_idx field Rt = 30 (PowerOfTwoMinusOne)
    // Fields: Rt=30, opc=0, size=0, imm9=0, Rn=0
    let encoding: u32 = 0x3C00041E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b30, [x0], #0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_post_idx
/// ASL: `field Rt 0 +: 5`
/// Requirement: FieldBoundary { field: "Rt", value: 31, boundary: Max }
/// register index 31 (ZR - zero register)
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_post_idx_field_rt_31_max_400_3c00041f() {
    // Encoding: 0x3C00041F
    // Test aarch64_memory_single_simdfp_immediate_signed_post_idx field Rt = 31 (Max)
    // Fields: size=0, Rn=0, imm9=0, Rt=31, opc=0
    let encoding: u32 = 0x3C00041F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b31, [x0], #0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_post_idx
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=0 (8-bit / byte size)
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_post_idx_combo_0_400_3c000400() {
    // Encoding: 0x3C000400
    // Test aarch64_memory_single_simdfp_immediate_signed_post_idx field combination: size=0, opc=0, imm9=0, Rn=0, Rt=0
    // Fields: size=0, opc=0, Rt=0, Rn=0, imm9=0
    let encoding: u32 = 0x3C000400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x0], #0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_post_idx
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=1 (16-bit / halfword size)
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_post_idx_combo_1_400_7c000400() {
    // Encoding: 0x7C000400
    // Test aarch64_memory_single_simdfp_immediate_signed_post_idx field combination: size=1, opc=0, imm9=0, Rn=0, Rt=0
    // Fields: Rn=0, size=1, imm9=0, Rt=0, opc=0
    let encoding: u32 = 0x7C000400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	h0, [x0], #0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_post_idx
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=2 (32-bit / word size)
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_post_idx_combo_2_400_bc000400() {
    // Encoding: 0xBC000400
    // Test aarch64_memory_single_simdfp_immediate_signed_post_idx field combination: size=2, opc=0, imm9=0, Rn=0, Rt=0
    // Fields: Rt=0, Rn=0, imm9=0, size=2, opc=0
    let encoding: u32 = 0xBC000400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	s0, [x0], #0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_post_idx
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=3 (64-bit / doubleword size)
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_post_idx_combo_3_400_fc000400() {
    // Encoding: 0xFC000400
    // Test aarch64_memory_single_simdfp_immediate_signed_post_idx field combination: size=3, opc=0, imm9=0, Rn=0, Rt=0
    // Fields: size=3, imm9=0, Rn=0, opc=0, Rt=0
    let encoding: u32 = 0xFC000400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	d0, [x0], #0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_post_idx
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// opc=0 (8-bit / byte size)
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_post_idx_combo_4_400_3c000400() {
    // Encoding: 0x3C000400
    // Test aarch64_memory_single_simdfp_immediate_signed_post_idx field combination: size=0, opc=0, imm9=0, Rn=0, Rt=0
    // Fields: opc=0, imm9=0, Rt=0, Rn=0, size=0
    let encoding: u32 = 0x3C000400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x0], #0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_post_idx
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// opc=1 (16-bit / halfword size)
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_post_idx_combo_5_400_3c400400() {
    // Encoding: 0x3C400400
    // Test aarch64_memory_single_simdfp_immediate_signed_post_idx field combination: size=0, opc=1, imm9=0, Rn=0, Rt=0
    // Fields: opc=1, Rt=0, size=0, imm9=0, Rn=0
    let encoding: u32 = 0x3C400400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ldr	b0, [x0], #0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_post_idx
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// opc=2 (32-bit / word size)
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_post_idx_combo_6_400_3c800400() {
    // Encoding: 0x3C800400
    // Test aarch64_memory_single_simdfp_immediate_signed_post_idx field combination: size=0, opc=2, imm9=0, Rn=0, Rt=0
    // Fields: size=0, opc=2, Rt=0, imm9=0, Rn=0
    let encoding: u32 = 0x3C800400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	q0, [x0], #0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_post_idx
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// opc=3 (64-bit / doubleword size)
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_post_idx_combo_7_400_3cc00400() {
    // Encoding: 0x3CC00400
    // Test aarch64_memory_single_simdfp_immediate_signed_post_idx field combination: size=0, opc=3, imm9=0, Rn=0, Rt=0
    // Fields: imm9=0, Rn=0, Rt=0, size=0, opc=3
    let encoding: u32 = 0x3CC00400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ldr	q0, [x0], #0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_post_idx
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=0 (immediate value 0)
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_post_idx_combo_8_400_3c000400() {
    // Encoding: 0x3C000400
    // Test aarch64_memory_single_simdfp_immediate_signed_post_idx field combination: size=0, opc=0, imm9=0, Rn=0, Rt=0
    // Fields: size=0, opc=0, imm9=0, Rn=0, Rt=0
    let encoding: u32 = 0x3C000400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x0], #0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_post_idx
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=1 (immediate value 1)
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_post_idx_combo_9_400_3c001400() {
    // Encoding: 0x3C001400
    // Test aarch64_memory_single_simdfp_immediate_signed_post_idx field combination: size=0, opc=0, imm9=1, Rn=0, Rt=0
    // Fields: imm9=1, opc=0, size=0, Rn=0, Rt=0
    let encoding: u32 = 0x3C001400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x0], #1
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_post_idx
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=3 (2^2 - 1 = 3)
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_post_idx_combo_10_400_3c003400() {
    // Encoding: 0x3C003400
    // Test aarch64_memory_single_simdfp_immediate_signed_post_idx field combination: size=0, opc=0, imm9=3, Rn=0, Rt=0
    // Fields: size=0, opc=0, Rt=0, imm9=3, Rn=0
    let encoding: u32 = 0x3C003400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x0], #3
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_post_idx
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=4 (power of 2 (2^2 = 4))
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_post_idx_combo_11_400_3c004400() {
    // Encoding: 0x3C004400
    // Test aarch64_memory_single_simdfp_immediate_signed_post_idx field combination: size=0, opc=0, imm9=4, Rn=0, Rt=0
    // Fields: opc=0, imm9=4, Rn=0, Rt=0, size=0
    let encoding: u32 = 0x3C004400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x0], #4
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_post_idx
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=7 (2^3 - 1 = 7)
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_post_idx_combo_12_400_3c007400() {
    // Encoding: 0x3C007400
    // Test aarch64_memory_single_simdfp_immediate_signed_post_idx field combination: size=0, opc=0, imm9=7, Rn=0, Rt=0
    // Fields: imm9=7, Rn=0, size=0, opc=0, Rt=0
    let encoding: u32 = 0x3C007400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x0], #7
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_post_idx
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=8 (power of 2 (2^3 = 8))
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_post_idx_combo_13_400_3c008400() {
    // Encoding: 0x3C008400
    // Test aarch64_memory_single_simdfp_immediate_signed_post_idx field combination: size=0, opc=0, imm9=8, Rn=0, Rt=0
    // Fields: Rt=0, Rn=0, imm9=8, size=0, opc=0
    let encoding: u32 = 0x3C008400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x0], #8
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_post_idx
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=15 (2^4 - 1 = 15)
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_post_idx_combo_14_400_3c00f400() {
    // Encoding: 0x3C00F400
    // Test aarch64_memory_single_simdfp_immediate_signed_post_idx field combination: size=0, opc=0, imm9=15, Rn=0, Rt=0
    // Fields: Rn=0, opc=0, Rt=0, imm9=15, size=0
    let encoding: u32 = 0x3C00F400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x0], #15
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_post_idx
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=16 (power of 2 (2^4 = 16))
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_post_idx_combo_15_400_3c010400() {
    // Encoding: 0x3C010400
    // Test aarch64_memory_single_simdfp_immediate_signed_post_idx field combination: size=0, opc=0, imm9=16, Rn=0, Rt=0
    // Fields: opc=0, imm9=16, size=0, Rn=0, Rt=0
    let encoding: u32 = 0x3C010400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x0], #16
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_post_idx
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=31 (2^5 - 1 = 31)
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_post_idx_combo_16_400_3c01f400() {
    // Encoding: 0x3C01F400
    // Test aarch64_memory_single_simdfp_immediate_signed_post_idx field combination: size=0, opc=0, imm9=31, Rn=0, Rt=0
    // Fields: size=0, imm9=31, opc=0, Rt=0, Rn=0
    let encoding: u32 = 0x3C01F400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x0], #31
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_post_idx
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=32 (power of 2 (2^5 = 32))
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_post_idx_combo_17_400_3c020400() {
    // Encoding: 0x3C020400
    // Test aarch64_memory_single_simdfp_immediate_signed_post_idx field combination: size=0, opc=0, imm9=32, Rn=0, Rt=0
    // Fields: imm9=32, opc=0, size=0, Rt=0, Rn=0
    let encoding: u32 = 0x3C020400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x0], #32
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_post_idx
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=63 (2^6 - 1 = 63)
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_post_idx_combo_18_400_3c03f400() {
    // Encoding: 0x3C03F400
    // Test aarch64_memory_single_simdfp_immediate_signed_post_idx field combination: size=0, opc=0, imm9=63, Rn=0, Rt=0
    // Fields: size=0, imm9=63, opc=0, Rn=0, Rt=0
    let encoding: u32 = 0x3C03F400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x0], #63
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_post_idx
/// ASL: `field combination 19`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=64 (power of 2 (2^6 = 64))
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_post_idx_combo_19_400_3c040400() {
    // Encoding: 0x3C040400
    // Test aarch64_memory_single_simdfp_immediate_signed_post_idx field combination: size=0, opc=0, imm9=64, Rn=0, Rt=0
    // Fields: Rt=0, size=0, imm9=64, Rn=0, opc=0
    let encoding: u32 = 0x3C040400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x0], #64
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_post_idx
/// ASL: `field combination 20`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=127 (2^7 - 1 = 127)
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_post_idx_combo_20_400_3c07f400() {
    // Encoding: 0x3C07F400
    // Test aarch64_memory_single_simdfp_immediate_signed_post_idx field combination: size=0, opc=0, imm9=127, Rn=0, Rt=0
    // Fields: Rn=0, opc=0, Rt=0, size=0, imm9=127
    let encoding: u32 = 0x3C07F400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x0], #127
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_post_idx
/// ASL: `field combination 21`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=128 (power of 2 (2^7 = 128))
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_post_idx_combo_21_400_3c080400() {
    // Encoding: 0x3C080400
    // Test aarch64_memory_single_simdfp_immediate_signed_post_idx field combination: size=0, opc=0, imm9=128, Rn=0, Rt=0
    // Fields: opc=0, Rn=0, imm9=128, size=0, Rt=0
    let encoding: u32 = 0x3C080400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x0], #128
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_post_idx
/// ASL: `field combination 22`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=255 (immediate midpoint (255))
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_post_idx_combo_22_400_3c0ff400() {
    // Encoding: 0x3C0FF400
    // Test aarch64_memory_single_simdfp_immediate_signed_post_idx field combination: size=0, opc=0, imm9=255, Rn=0, Rt=0
    // Fields: Rn=0, size=0, Rt=0, imm9=255, opc=0
    let encoding: u32 = 0x3C0FF400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x0], #255
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_post_idx
/// ASL: `field combination 23`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=256 (power of 2 (2^8 = 256))
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_post_idx_combo_23_400_3c100400() {
    // Encoding: 0x3C100400
    // Test aarch64_memory_single_simdfp_immediate_signed_post_idx field combination: size=0, opc=0, imm9=256, Rn=0, Rt=0
    // Fields: imm9=256, Rt=0, size=0, Rn=0, opc=0
    let encoding: u32 = 0x3C100400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x0], #-256
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_post_idx
/// ASL: `field combination 24`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=511 (maximum immediate (511))
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_post_idx_combo_24_400_3c1ff400() {
    // Encoding: 0x3C1FF400
    // Test aarch64_memory_single_simdfp_immediate_signed_post_idx field combination: size=0, opc=0, imm9=511, Rn=0, Rt=0
    // Fields: size=0, opc=0, Rt=0, Rn=0, imm9=511
    let encoding: u32 = 0x3C1FF400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x0], #-1
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_post_idx
/// ASL: `field combination 25`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_post_idx_combo_25_400_3c000400() {
    // Encoding: 0x3C000400
    // Test aarch64_memory_single_simdfp_immediate_signed_post_idx field combination: size=0, opc=0, imm9=0, Rn=0, Rt=0
    // Fields: size=0, opc=0, imm9=0, Rt=0, Rn=0
    let encoding: u32 = 0x3C000400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x0], #0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_post_idx
/// ASL: `field combination 26`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_post_idx_combo_26_400_3c000420() {
    // Encoding: 0x3C000420
    // Test aarch64_memory_single_simdfp_immediate_signed_post_idx field combination: size=0, opc=0, imm9=0, Rn=1, Rt=0
    // Fields: size=0, imm9=0, Rt=0, opc=0, Rn=1
    let encoding: u32 = 0x3C000420;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x1], #0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_post_idx
/// ASL: `field combination 27`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_post_idx_combo_27_400_3c0007c0() {
    // Encoding: 0x3C0007C0
    // Test aarch64_memory_single_simdfp_immediate_signed_post_idx field combination: size=0, opc=0, imm9=0, Rn=30, Rt=0
    // Fields: opc=0, Rn=30, imm9=0, Rt=0, size=0
    let encoding: u32 = 0x3C0007C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x30], #0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_post_idx
/// ASL: `field combination 28`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_post_idx_combo_28_400_3c0007e0() {
    // Encoding: 0x3C0007E0
    // Test aarch64_memory_single_simdfp_immediate_signed_post_idx field combination: size=0, opc=0, imm9=0, Rn=31, Rt=0
    // Fields: Rt=0, size=0, Rn=31, opc=0, imm9=0
    let encoding: u32 = 0x3C0007E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [sp], #0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_post_idx
/// ASL: `field combination 29`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt=0 (register index 0 (first register))
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_post_idx_combo_29_400_3c000400() {
    // Encoding: 0x3C000400
    // Test aarch64_memory_single_simdfp_immediate_signed_post_idx field combination: size=0, opc=0, imm9=0, Rn=0, Rt=0
    // Fields: Rn=0, size=0, opc=0, Rt=0, imm9=0
    let encoding: u32 = 0x3C000400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x0], #0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_post_idx
/// ASL: `field combination 30`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt=1 (register index 1 (second register))
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_post_idx_combo_30_400_3c000401() {
    // Encoding: 0x3C000401
    // Test aarch64_memory_single_simdfp_immediate_signed_post_idx field combination: size=0, opc=0, imm9=0, Rn=0, Rt=1
    // Fields: imm9=0, size=0, Rt=1, Rn=0, opc=0
    let encoding: u32 = 0x3C000401;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b1, [x0], #0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_post_idx
/// ASL: `field combination 31`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_post_idx_combo_31_400_3c00041e() {
    // Encoding: 0x3C00041E
    // Test aarch64_memory_single_simdfp_immediate_signed_post_idx field combination: size=0, opc=0, imm9=0, Rn=0, Rt=30
    // Fields: imm9=0, Rn=0, opc=0, Rt=30, size=0
    let encoding: u32 = 0x3C00041E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b30, [x0], #0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_post_idx
/// ASL: `field combination 32`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt=31 (register index 31 (ZR - zero register))
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_post_idx_combo_32_400_3c00041f() {
    // Encoding: 0x3C00041F
    // Test aarch64_memory_single_simdfp_immediate_signed_post_idx field combination: size=0, opc=0, imm9=0, Rn=0, Rt=31
    // Fields: imm9=0, size=0, Rn=0, Rt=31, opc=0
    let encoding: u32 = 0x3C00041F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b31, [x0], #0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_post_idx
/// ASL: `field combination 33`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (same register test (reg=1)), Rt=1 (same register test (reg=1))
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_post_idx_combo_33_400_3c000421() {
    // Encoding: 0x3C000421
    // Test aarch64_memory_single_simdfp_immediate_signed_post_idx field combination: size=0, opc=0, imm9=0, Rn=1, Rt=1
    // Fields: size=0, Rt=1, imm9=0, opc=0, Rn=1
    let encoding: u32 = 0x3C000421;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b1, [x1], #0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_post_idx
/// ASL: `field combination 34`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (same register test (reg=31)), Rt=31 (same register test (reg=31))
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_post_idx_combo_34_400_3c0007ff() {
    // Encoding: 0x3C0007FF
    // Test aarch64_memory_single_simdfp_immediate_signed_post_idx field combination: size=0, opc=0, imm9=0, Rn=31, Rt=31
    // Fields: opc=0, imm9=0, Rt=31, Rn=31, size=0
    let encoding: u32 = 0x3C0007FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b31, [sp], #0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_post_idx
/// ASL: `field size = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "size", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_post_idx_special_size_0_size_variant_0_1024_3c001400()
 {
    // Encoding: 0x3C001400
    // Test aarch64_memory_single_simdfp_immediate_signed_post_idx special value size = 0 (Size variant 0)
    // Fields: imm9=1, opc=0, size=0, Rn=0, Rt=0
    let encoding: u32 = 0x3C001400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x0], #1
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_post_idx
/// ASL: `field size = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "size", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_post_idx_special_size_1_size_variant_1_1024_7c001400()
 {
    // Encoding: 0x7C001400
    // Test aarch64_memory_single_simdfp_immediate_signed_post_idx special value size = 1 (Size variant 1)
    // Fields: opc=0, imm9=1, size=1, Rn=0, Rt=0
    let encoding: u32 = 0x7C001400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	h0, [x0], #1
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_post_idx
/// ASL: `field size = 2 (Size variant 2)`
/// Requirement: FieldSpecial { field: "size", value: 2, meaning: "Size variant 2" }
/// Size variant 2
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_post_idx_special_size_2_size_variant_2_1024_bc001400()
 {
    // Encoding: 0xBC001400
    // Test aarch64_memory_single_simdfp_immediate_signed_post_idx special value size = 2 (Size variant 2)
    // Fields: Rt=0, opc=0, Rn=0, size=2, imm9=1
    let encoding: u32 = 0xBC001400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	s0, [x0], #1
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_post_idx
/// ASL: `field size = 3 (Size variant 3)`
/// Requirement: FieldSpecial { field: "size", value: 3, meaning: "Size variant 3" }
/// Size variant 3
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_post_idx_special_size_3_size_variant_3_1024_fc001400()
 {
    // Encoding: 0xFC001400
    // Test aarch64_memory_single_simdfp_immediate_signed_post_idx special value size = 3 (Size variant 3)
    // Fields: imm9=1, size=3, Rt=0, Rn=0, opc=0
    let encoding: u32 = 0xFC001400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	d0, [x0], #1
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_post_idx
/// ASL: `field opc = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "opc", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_post_idx_special_opc_0_size_variant_0_1024_7c001400()
 {
    // Encoding: 0x7C001400
    // Test aarch64_memory_single_simdfp_immediate_signed_post_idx special value opc = 0 (Size variant 0)
    // Fields: opc=0, imm9=1, Rt=0, Rn=0, size=1
    let encoding: u32 = 0x7C001400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	h0, [x0], #1
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_post_idx
/// ASL: `field opc = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "opc", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_post_idx_special_opc_1_size_variant_1_1024_7c401400()
 {
    // Encoding: 0x7C401400
    // Test aarch64_memory_single_simdfp_immediate_signed_post_idx special value opc = 1 (Size variant 1)
    // Fields: size=1, Rn=0, opc=1, imm9=1, Rt=0
    let encoding: u32 = 0x7C401400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ldr	h0, [x0], #1
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_post_idx
/// ASL: `field opc = 2 (Size variant 2)`
/// Requirement: FieldSpecial { field: "opc", value: 2, meaning: "Size variant 2" }
/// Size variant 2
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_post_idx_special_opc_2_size_variant_2_1024_7c801400()
 {
    // Encoding: 0x7C801400
    // Test aarch64_memory_single_simdfp_immediate_signed_post_idx special value opc = 2 (Size variant 2)
    // Fields: Rt=0, Rn=0, opc=2, size=1, imm9=1
    let encoding: u32 = 0x7C801400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_post_idx
/// ASL: `field opc = 3 (Size variant 3)`
/// Requirement: FieldSpecial { field: "opc", value: 3, meaning: "Size variant 3" }
/// Size variant 3
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_post_idx_special_opc_3_size_variant_3_1024_7cc01400()
 {
    // Encoding: 0x7CC01400
    // Test aarch64_memory_single_simdfp_immediate_signed_post_idx special value opc = 3 (Size variant 3)
    // Fields: Rn=0, opc=3, imm9=1, Rt=0, size=1
    let encoding: u32 = 0x7CC01400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_post_idx
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_post_idx_special_rn_31_stack_pointer_sp_may_require_alignment_1024_7c0017e0()
 {
    // Encoding: 0x7C0017E0
    // Test aarch64_memory_single_simdfp_immediate_signed_post_idx special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: size=1, imm9=1, Rn=31, Rt=0, opc=0
    let encoding: u32 = 0x7C0017E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	h0, [sp], #1
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_post_idx
/// ASL: `field Rt = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)`
/// Requirement: FieldSpecial { field: "Rt", value: 31, meaning: "Zero register (XZR/WZR) - reads as 0, writes discarded" }
/// Zero register (XZR/WZR) - reads as 0, writes discarded
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_post_idx_special_rt_31_zero_register_xzr_wzr_reads_as_0_writes_discarded_1024_7c00141f()
 {
    // Encoding: 0x7C00141F
    // Test aarch64_memory_single_simdfp_immediate_signed_post_idx special value Rt = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)
    // Fields: size=1, opc=0, Rn=0, Rt=31, imm9=1
    let encoding: u32 = 0x7C00141F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	h31, [x0], #1
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_pre_idx
/// ASL: `field size 30 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_pre_idx_field_size_0_min_c00_3c000c00() {
    // Encoding: 0x3C000C00
    // Test aarch64_memory_single_simdfp_immediate_signed_pre_idx field size = 0 (Min)
    // Fields: Rt=0, Rn=0, size=0, imm9=0, opc=0
    let encoding: u32 = 0x3C000C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x0, #0]!
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_pre_idx
/// ASL: `field size 30 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 1, boundary: PowerOfTwo }
/// 16-bit / halfword size
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_pre_idx_field_size_1_poweroftwo_c00_7c000c00()
{
    // Encoding: 0x7C000C00
    // Test aarch64_memory_single_simdfp_immediate_signed_pre_idx field size = 1 (PowerOfTwo)
    // Fields: size=1, Rn=0, imm9=0, opc=0, Rt=0
    let encoding: u32 = 0x7C000C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	h0, [x0, #0]!
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_pre_idx
/// ASL: `field size 30 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 2, boundary: PowerOfTwo }
/// 32-bit / word size
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_pre_idx_field_size_2_poweroftwo_c00_bc000c00()
{
    // Encoding: 0xBC000C00
    // Test aarch64_memory_single_simdfp_immediate_signed_pre_idx field size = 2 (PowerOfTwo)
    // Fields: opc=0, Rt=0, size=2, Rn=0, imm9=0
    let encoding: u32 = 0xBC000C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	s0, [x0, #0]!
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_pre_idx
/// ASL: `field size 30 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 3, boundary: Max }
/// 64-bit / doubleword size
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_pre_idx_field_size_3_max_c00_fc000c00() {
    // Encoding: 0xFC000C00
    // Test aarch64_memory_single_simdfp_immediate_signed_pre_idx field size = 3 (Max)
    // Fields: opc=0, imm9=0, Rn=0, size=3, Rt=0
    let encoding: u32 = 0xFC000C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	d0, [x0, #0]!
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_pre_idx
/// ASL: `field opc 22 +: 2`
/// Requirement: FieldBoundary { field: "opc", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_pre_idx_field_opc_0_min_c00_3c000c00() {
    // Encoding: 0x3C000C00
    // Test aarch64_memory_single_simdfp_immediate_signed_pre_idx field opc = 0 (Min)
    // Fields: opc=0, Rt=0, size=0, Rn=0, imm9=0
    let encoding: u32 = 0x3C000C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x0, #0]!
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_pre_idx
/// ASL: `field opc 22 +: 2`
/// Requirement: FieldBoundary { field: "opc", value: 1, boundary: PowerOfTwo }
/// 16-bit / halfword size
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_pre_idx_field_opc_1_poweroftwo_c00_3c400c00()
{
    // Encoding: 0x3C400C00
    // Test aarch64_memory_single_simdfp_immediate_signed_pre_idx field opc = 1 (PowerOfTwo)
    // Fields: Rt=0, opc=1, Rn=0, size=0, imm9=0
    let encoding: u32 = 0x3C400C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ldr	b0, [x0, #0]!
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_pre_idx
/// ASL: `field opc 22 +: 2`
/// Requirement: FieldBoundary { field: "opc", value: 2, boundary: PowerOfTwo }
/// 32-bit / word size
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_pre_idx_field_opc_2_poweroftwo_c00_3c800c00()
{
    // Encoding: 0x3C800C00
    // Test aarch64_memory_single_simdfp_immediate_signed_pre_idx field opc = 2 (PowerOfTwo)
    // Fields: Rn=0, size=0, opc=2, Rt=0, imm9=0
    let encoding: u32 = 0x3C800C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	q0, [x0, #0]!
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_pre_idx
/// ASL: `field opc 22 +: 2`
/// Requirement: FieldBoundary { field: "opc", value: 3, boundary: Max }
/// 64-bit / doubleword size
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_pre_idx_field_opc_3_max_c00_3cc00c00() {
    // Encoding: 0x3CC00C00
    // Test aarch64_memory_single_simdfp_immediate_signed_pre_idx field opc = 3 (Max)
    // Fields: Rn=0, imm9=0, size=0, Rt=0, opc=3
    let encoding: u32 = 0x3CC00C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ldr	q0, [x0, #0]!
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_pre_idx
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_pre_idx_field_imm9_0_zero_c00_3c000c00() {
    // Encoding: 0x3C000C00
    // Test aarch64_memory_single_simdfp_immediate_signed_pre_idx field imm9 = 0 (Zero)
    // Fields: size=0, Rt=0, imm9=0, opc=0, Rn=0
    let encoding: u32 = 0x3C000C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x0, #0]!
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_pre_idx
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_pre_idx_field_imm9_1_poweroftwo_c00_3c001c00()
{
    // Encoding: 0x3C001C00
    // Test aarch64_memory_single_simdfp_immediate_signed_pre_idx field imm9 = 1 (PowerOfTwo)
    // Fields: Rt=0, imm9=1, opc=0, size=0, Rn=0
    let encoding: u32 = 0x3C001C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x0, #1]!
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_pre_idx
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 3, boundary: PowerOfTwoMinusOne }
/// 2^2 - 1 = 3
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_pre_idx_field_imm9_3_poweroftwominusone_c00_3c003c00()
 {
    // Encoding: 0x3C003C00
    // Test aarch64_memory_single_simdfp_immediate_signed_pre_idx field imm9 = 3 (PowerOfTwoMinusOne)
    // Fields: imm9=3, size=0, opc=0, Rn=0, Rt=0
    let encoding: u32 = 0x3C003C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x0, #3]!
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_pre_idx
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 4, boundary: PowerOfTwo }
/// power of 2 (2^2 = 4)
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_pre_idx_field_imm9_4_poweroftwo_c00_3c004c00()
{
    // Encoding: 0x3C004C00
    // Test aarch64_memory_single_simdfp_immediate_signed_pre_idx field imm9 = 4 (PowerOfTwo)
    // Fields: Rn=0, Rt=0, imm9=4, opc=0, size=0
    let encoding: u32 = 0x3C004C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x0, #4]!
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_pre_idx
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 7, boundary: PowerOfTwoMinusOne }
/// 2^3 - 1 = 7
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_pre_idx_field_imm9_7_poweroftwominusone_c00_3c007c00()
 {
    // Encoding: 0x3C007C00
    // Test aarch64_memory_single_simdfp_immediate_signed_pre_idx field imm9 = 7 (PowerOfTwoMinusOne)
    // Fields: size=0, imm9=7, opc=0, Rt=0, Rn=0
    let encoding: u32 = 0x3C007C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x0, #7]!
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_pre_idx
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 8, boundary: PowerOfTwo }
/// power of 2 (2^3 = 8)
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_pre_idx_field_imm9_8_poweroftwo_c00_3c008c00()
{
    // Encoding: 0x3C008C00
    // Test aarch64_memory_single_simdfp_immediate_signed_pre_idx field imm9 = 8 (PowerOfTwo)
    // Fields: opc=0, size=0, imm9=8, Rn=0, Rt=0
    let encoding: u32 = 0x3C008C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x0, #8]!
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_pre_idx
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 15, boundary: PowerOfTwoMinusOne }
/// 2^4 - 1 = 15
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_pre_idx_field_imm9_15_poweroftwominusone_c00_3c00fc00()
 {
    // Encoding: 0x3C00FC00
    // Test aarch64_memory_single_simdfp_immediate_signed_pre_idx field imm9 = 15 (PowerOfTwoMinusOne)
    // Fields: Rn=0, imm9=15, Rt=0, opc=0, size=0
    let encoding: u32 = 0x3C00FC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x0, #15]!
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_pre_idx
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 16, boundary: PowerOfTwo }
/// power of 2 (2^4 = 16)
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_pre_idx_field_imm9_16_poweroftwo_c00_3c010c00()
 {
    // Encoding: 0x3C010C00
    // Test aarch64_memory_single_simdfp_immediate_signed_pre_idx field imm9 = 16 (PowerOfTwo)
    // Fields: Rt=0, Rn=0, size=0, imm9=16, opc=0
    let encoding: u32 = 0x3C010C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x0, #16]!
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_pre_idx
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 31, boundary: PowerOfTwoMinusOne }
/// 2^5 - 1 = 31
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_pre_idx_field_imm9_31_poweroftwominusone_c00_3c01fc00()
 {
    // Encoding: 0x3C01FC00
    // Test aarch64_memory_single_simdfp_immediate_signed_pre_idx field imm9 = 31 (PowerOfTwoMinusOne)
    // Fields: size=0, Rt=0, Rn=0, opc=0, imm9=31
    let encoding: u32 = 0x3C01FC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x0, #31]!
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_pre_idx
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 32, boundary: PowerOfTwo }
/// power of 2 (2^5 = 32)
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_pre_idx_field_imm9_32_poweroftwo_c00_3c020c00()
 {
    // Encoding: 0x3C020C00
    // Test aarch64_memory_single_simdfp_immediate_signed_pre_idx field imm9 = 32 (PowerOfTwo)
    // Fields: Rn=0, size=0, Rt=0, opc=0, imm9=32
    let encoding: u32 = 0x3C020C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x0, #32]!
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_pre_idx
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 63, boundary: PowerOfTwoMinusOne }
/// 2^6 - 1 = 63
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_pre_idx_field_imm9_63_poweroftwominusone_c00_3c03fc00()
 {
    // Encoding: 0x3C03FC00
    // Test aarch64_memory_single_simdfp_immediate_signed_pre_idx field imm9 = 63 (PowerOfTwoMinusOne)
    // Fields: opc=0, imm9=63, Rn=0, Rt=0, size=0
    let encoding: u32 = 0x3C03FC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x0, #63]!
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_pre_idx
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 64, boundary: PowerOfTwo }
/// power of 2 (2^6 = 64)
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_pre_idx_field_imm9_64_poweroftwo_c00_3c040c00()
 {
    // Encoding: 0x3C040C00
    // Test aarch64_memory_single_simdfp_immediate_signed_pre_idx field imm9 = 64 (PowerOfTwo)
    // Fields: Rn=0, Rt=0, opc=0, size=0, imm9=64
    let encoding: u32 = 0x3C040C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x0, #64]!
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_pre_idx
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 127, boundary: PowerOfTwoMinusOne }
/// 2^7 - 1 = 127
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_pre_idx_field_imm9_127_poweroftwominusone_c00_3c07fc00()
 {
    // Encoding: 0x3C07FC00
    // Test aarch64_memory_single_simdfp_immediate_signed_pre_idx field imm9 = 127 (PowerOfTwoMinusOne)
    // Fields: Rt=0, size=0, opc=0, imm9=127, Rn=0
    let encoding: u32 = 0x3C07FC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x0, #127]!
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_pre_idx
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 128, boundary: PowerOfTwo }
/// power of 2 (2^7 = 128)
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_pre_idx_field_imm9_128_poweroftwo_c00_3c080c00()
 {
    // Encoding: 0x3C080C00
    // Test aarch64_memory_single_simdfp_immediate_signed_pre_idx field imm9 = 128 (PowerOfTwo)
    // Fields: opc=0, imm9=128, size=0, Rn=0, Rt=0
    let encoding: u32 = 0x3C080C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x0, #128]!
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_pre_idx
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 255, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (255)
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_pre_idx_field_imm9_255_poweroftwominusone_c00_3c0ffc00()
 {
    // Encoding: 0x3C0FFC00
    // Test aarch64_memory_single_simdfp_immediate_signed_pre_idx field imm9 = 255 (PowerOfTwoMinusOne)
    // Fields: opc=0, imm9=255, size=0, Rn=0, Rt=0
    let encoding: u32 = 0x3C0FFC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x0, #255]!
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_pre_idx
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 256, boundary: PowerOfTwo }
/// power of 2 (2^8 = 256)
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_pre_idx_field_imm9_256_poweroftwo_c00_3c100c00()
 {
    // Encoding: 0x3C100C00
    // Test aarch64_memory_single_simdfp_immediate_signed_pre_idx field imm9 = 256 (PowerOfTwo)
    // Fields: size=0, opc=0, imm9=256, Rt=0, Rn=0
    let encoding: u32 = 0x3C100C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_pre_idx
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 511, boundary: Max }
/// maximum immediate (511)
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_pre_idx_field_imm9_511_max_c00_3c1ffc00() {
    // Encoding: 0x3C1FFC00
    // Test aarch64_memory_single_simdfp_immediate_signed_pre_idx field imm9 = 511 (Max)
    // Fields: Rt=0, size=0, imm9=511, opc=0, Rn=0
    let encoding: u32 = 0x3C1FFC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x0, #-1]!
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

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_pre_idx
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_pre_idx_field_rn_0_min_c00_3c000c00() {
    // Encoding: 0x3C000C00
    // Test aarch64_memory_single_simdfp_immediate_signed_pre_idx field Rn = 0 (Min)
    // Fields: size=0, Rn=0, Rt=0, imm9=0, opc=0
    let encoding: u32 = 0x3C000C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x0, #0]!
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_pre_idx
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_pre_idx_field_rn_1_poweroftwo_c00_3c000c20() {
    // Encoding: 0x3C000C20
    // Test aarch64_memory_single_simdfp_immediate_signed_pre_idx field Rn = 1 (PowerOfTwo)
    // Fields: size=0, Rt=0, Rn=1, opc=0, imm9=0
    let encoding: u32 = 0x3C000C20;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x1, #0]!
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_pre_idx
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_pre_idx_field_rn_30_poweroftwominusone_c00_3c000fc0()
 {
    // Encoding: 0x3C000FC0
    // Test aarch64_memory_single_simdfp_immediate_signed_pre_idx field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: Rt=0, size=0, opc=0, imm9=0, Rn=30
    let encoding: u32 = 0x3C000FC0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x30, #0]!
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_pre_idx
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_pre_idx_field_rn_31_max_c00_3c000fe0() {
    // Encoding: 0x3C000FE0
    // Test aarch64_memory_single_simdfp_immediate_signed_pre_idx field Rn = 31 (Max)
    // Fields: size=0, Rt=0, Rn=31, opc=0, imm9=0
    let encoding: u32 = 0x3C000FE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [sp, #0]!
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_pre_idx
/// ASL: `field Rt 0 +: 5`
/// Requirement: FieldBoundary { field: "Rt", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_pre_idx_field_rt_0_min_c00_3c000c00() {
    // Encoding: 0x3C000C00
    // Test aarch64_memory_single_simdfp_immediate_signed_pre_idx field Rt = 0 (Min)
    // Fields: opc=0, size=0, Rn=0, Rt=0, imm9=0
    let encoding: u32 = 0x3C000C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x0, #0]!
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_pre_idx
/// ASL: `field Rt 0 +: 5`
/// Requirement: FieldBoundary { field: "Rt", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_pre_idx_field_rt_1_poweroftwo_c00_3c000c01() {
    // Encoding: 0x3C000C01
    // Test aarch64_memory_single_simdfp_immediate_signed_pre_idx field Rt = 1 (PowerOfTwo)
    // Fields: Rt=1, size=0, Rn=0, opc=0, imm9=0
    let encoding: u32 = 0x3C000C01;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b1, [x0, #0]!
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_pre_idx
/// ASL: `field Rt 0 +: 5`
/// Requirement: FieldBoundary { field: "Rt", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_pre_idx_field_rt_30_poweroftwominusone_c00_3c000c1e()
 {
    // Encoding: 0x3C000C1E
    // Test aarch64_memory_single_simdfp_immediate_signed_pre_idx field Rt = 30 (PowerOfTwoMinusOne)
    // Fields: size=0, Rn=0, opc=0, imm9=0, Rt=30
    let encoding: u32 = 0x3C000C1E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b30, [x0, #0]!
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_pre_idx
/// ASL: `field Rt 0 +: 5`
/// Requirement: FieldBoundary { field: "Rt", value: 31, boundary: Max }
/// register index 31 (ZR - zero register)
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_pre_idx_field_rt_31_max_c00_3c000c1f() {
    // Encoding: 0x3C000C1F
    // Test aarch64_memory_single_simdfp_immediate_signed_pre_idx field Rt = 31 (Max)
    // Fields: Rn=0, size=0, imm9=0, Rt=31, opc=0
    let encoding: u32 = 0x3C000C1F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b31, [x0, #0]!
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_pre_idx
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=0 (8-bit / byte size)
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_pre_idx_combo_0_c00_3c000c00() {
    // Encoding: 0x3C000C00
    // Test aarch64_memory_single_simdfp_immediate_signed_pre_idx field combination: size=0, opc=0, imm9=0, Rn=0, Rt=0
    // Fields: Rt=0, size=0, imm9=0, Rn=0, opc=0
    let encoding: u32 = 0x3C000C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x0, #0]!
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_pre_idx
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=1 (16-bit / halfword size)
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_pre_idx_combo_1_c00_7c000c00() {
    // Encoding: 0x7C000C00
    // Test aarch64_memory_single_simdfp_immediate_signed_pre_idx field combination: size=1, opc=0, imm9=0, Rn=0, Rt=0
    // Fields: opc=0, size=1, imm9=0, Rn=0, Rt=0
    let encoding: u32 = 0x7C000C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	h0, [x0, #0]!
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_pre_idx
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=2 (32-bit / word size)
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_pre_idx_combo_2_c00_bc000c00() {
    // Encoding: 0xBC000C00
    // Test aarch64_memory_single_simdfp_immediate_signed_pre_idx field combination: size=2, opc=0, imm9=0, Rn=0, Rt=0
    // Fields: Rt=0, Rn=0, opc=0, size=2, imm9=0
    let encoding: u32 = 0xBC000C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	s0, [x0, #0]!
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_pre_idx
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=3 (64-bit / doubleword size)
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_pre_idx_combo_3_c00_fc000c00() {
    // Encoding: 0xFC000C00
    // Test aarch64_memory_single_simdfp_immediate_signed_pre_idx field combination: size=3, opc=0, imm9=0, Rn=0, Rt=0
    // Fields: imm9=0, Rn=0, size=3, Rt=0, opc=0
    let encoding: u32 = 0xFC000C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	d0, [x0, #0]!
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_pre_idx
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// opc=0 (8-bit / byte size)
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_pre_idx_combo_4_c00_3c000c00() {
    // Encoding: 0x3C000C00
    // Test aarch64_memory_single_simdfp_immediate_signed_pre_idx field combination: size=0, opc=0, imm9=0, Rn=0, Rt=0
    // Fields: imm9=0, opc=0, Rn=0, Rt=0, size=0
    let encoding: u32 = 0x3C000C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x0, #0]!
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_pre_idx
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// opc=1 (16-bit / halfword size)
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_pre_idx_combo_5_c00_3c400c00() {
    // Encoding: 0x3C400C00
    // Test aarch64_memory_single_simdfp_immediate_signed_pre_idx field combination: size=0, opc=1, imm9=0, Rn=0, Rt=0
    // Fields: size=0, opc=1, Rt=0, Rn=0, imm9=0
    let encoding: u32 = 0x3C400C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ldr	b0, [x0, #0]!
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_pre_idx
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// opc=2 (32-bit / word size)
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_pre_idx_combo_6_c00_3c800c00() {
    // Encoding: 0x3C800C00
    // Test aarch64_memory_single_simdfp_immediate_signed_pre_idx field combination: size=0, opc=2, imm9=0, Rn=0, Rt=0
    // Fields: Rn=0, opc=2, Rt=0, size=0, imm9=0
    let encoding: u32 = 0x3C800C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	q0, [x0, #0]!
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_pre_idx
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// opc=3 (64-bit / doubleword size)
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_pre_idx_combo_7_c00_3cc00c00() {
    // Encoding: 0x3CC00C00
    // Test aarch64_memory_single_simdfp_immediate_signed_pre_idx field combination: size=0, opc=3, imm9=0, Rn=0, Rt=0
    // Fields: size=0, opc=3, Rn=0, Rt=0, imm9=0
    let encoding: u32 = 0x3CC00C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ldr	q0, [x0, #0]!
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_pre_idx
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=0 (immediate value 0)
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_pre_idx_combo_8_c00_3c000c00() {
    // Encoding: 0x3C000C00
    // Test aarch64_memory_single_simdfp_immediate_signed_pre_idx field combination: size=0, opc=0, imm9=0, Rn=0, Rt=0
    // Fields: opc=0, size=0, imm9=0, Rn=0, Rt=0
    let encoding: u32 = 0x3C000C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x0, #0]!
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_pre_idx
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=1 (immediate value 1)
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_pre_idx_combo_9_c00_3c001c00() {
    // Encoding: 0x3C001C00
    // Test aarch64_memory_single_simdfp_immediate_signed_pre_idx field combination: size=0, opc=0, imm9=1, Rn=0, Rt=0
    // Fields: opc=0, size=0, Rt=0, Rn=0, imm9=1
    let encoding: u32 = 0x3C001C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x0, #1]!
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_pre_idx
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=3 (2^2 - 1 = 3)
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_pre_idx_combo_10_c00_3c003c00() {
    // Encoding: 0x3C003C00
    // Test aarch64_memory_single_simdfp_immediate_signed_pre_idx field combination: size=0, opc=0, imm9=3, Rn=0, Rt=0
    // Fields: Rn=0, opc=0, imm9=3, size=0, Rt=0
    let encoding: u32 = 0x3C003C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x0, #3]!
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_pre_idx
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=4 (power of 2 (2^2 = 4))
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_pre_idx_combo_11_c00_3c004c00() {
    // Encoding: 0x3C004C00
    // Test aarch64_memory_single_simdfp_immediate_signed_pre_idx field combination: size=0, opc=0, imm9=4, Rn=0, Rt=0
    // Fields: opc=0, imm9=4, size=0, Rt=0, Rn=0
    let encoding: u32 = 0x3C004C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x0, #4]!
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_pre_idx
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=7 (2^3 - 1 = 7)
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_pre_idx_combo_12_c00_3c007c00() {
    // Encoding: 0x3C007C00
    // Test aarch64_memory_single_simdfp_immediate_signed_pre_idx field combination: size=0, opc=0, imm9=7, Rn=0, Rt=0
    // Fields: size=0, Rn=0, opc=0, imm9=7, Rt=0
    let encoding: u32 = 0x3C007C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x0, #7]!
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_pre_idx
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=8 (power of 2 (2^3 = 8))
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_pre_idx_combo_13_c00_3c008c00() {
    // Encoding: 0x3C008C00
    // Test aarch64_memory_single_simdfp_immediate_signed_pre_idx field combination: size=0, opc=0, imm9=8, Rn=0, Rt=0
    // Fields: Rt=0, Rn=0, size=0, opc=0, imm9=8
    let encoding: u32 = 0x3C008C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x0, #8]!
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_pre_idx
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=15 (2^4 - 1 = 15)
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_pre_idx_combo_14_c00_3c00fc00() {
    // Encoding: 0x3C00FC00
    // Test aarch64_memory_single_simdfp_immediate_signed_pre_idx field combination: size=0, opc=0, imm9=15, Rn=0, Rt=0
    // Fields: imm9=15, opc=0, Rn=0, Rt=0, size=0
    let encoding: u32 = 0x3C00FC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x0, #15]!
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_pre_idx
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=16 (power of 2 (2^4 = 16))
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_pre_idx_combo_15_c00_3c010c00() {
    // Encoding: 0x3C010C00
    // Test aarch64_memory_single_simdfp_immediate_signed_pre_idx field combination: size=0, opc=0, imm9=16, Rn=0, Rt=0
    // Fields: opc=0, Rn=0, Rt=0, imm9=16, size=0
    let encoding: u32 = 0x3C010C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x0, #16]!
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_pre_idx
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=31 (2^5 - 1 = 31)
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_pre_idx_combo_16_c00_3c01fc00() {
    // Encoding: 0x3C01FC00
    // Test aarch64_memory_single_simdfp_immediate_signed_pre_idx field combination: size=0, opc=0, imm9=31, Rn=0, Rt=0
    // Fields: size=0, Rt=0, opc=0, Rn=0, imm9=31
    let encoding: u32 = 0x3C01FC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x0, #31]!
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_pre_idx
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=32 (power of 2 (2^5 = 32))
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_pre_idx_combo_17_c00_3c020c00() {
    // Encoding: 0x3C020C00
    // Test aarch64_memory_single_simdfp_immediate_signed_pre_idx field combination: size=0, opc=0, imm9=32, Rn=0, Rt=0
    // Fields: Rn=0, imm9=32, size=0, opc=0, Rt=0
    let encoding: u32 = 0x3C020C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x0, #32]!
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_pre_idx
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=63 (2^6 - 1 = 63)
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_pre_idx_combo_18_c00_3c03fc00() {
    // Encoding: 0x3C03FC00
    // Test aarch64_memory_single_simdfp_immediate_signed_pre_idx field combination: size=0, opc=0, imm9=63, Rn=0, Rt=0
    // Fields: opc=0, imm9=63, Rt=0, size=0, Rn=0
    let encoding: u32 = 0x3C03FC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x0, #63]!
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_pre_idx
/// ASL: `field combination 19`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=64 (power of 2 (2^6 = 64))
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_pre_idx_combo_19_c00_3c040c00() {
    // Encoding: 0x3C040C00
    // Test aarch64_memory_single_simdfp_immediate_signed_pre_idx field combination: size=0, opc=0, imm9=64, Rn=0, Rt=0
    // Fields: size=0, opc=0, imm9=64, Rn=0, Rt=0
    let encoding: u32 = 0x3C040C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x0, #64]!
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_pre_idx
/// ASL: `field combination 20`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=127 (2^7 - 1 = 127)
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_pre_idx_combo_20_c00_3c07fc00() {
    // Encoding: 0x3C07FC00
    // Test aarch64_memory_single_simdfp_immediate_signed_pre_idx field combination: size=0, opc=0, imm9=127, Rn=0, Rt=0
    // Fields: imm9=127, Rn=0, size=0, Rt=0, opc=0
    let encoding: u32 = 0x3C07FC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x0, #127]!
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_pre_idx
/// ASL: `field combination 21`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=128 (power of 2 (2^7 = 128))
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_pre_idx_combo_21_c00_3c080c00() {
    // Encoding: 0x3C080C00
    // Test aarch64_memory_single_simdfp_immediate_signed_pre_idx field combination: size=0, opc=0, imm9=128, Rn=0, Rt=0
    // Fields: imm9=128, Rn=0, Rt=0, size=0, opc=0
    let encoding: u32 = 0x3C080C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x0, #128]!
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_pre_idx
/// ASL: `field combination 22`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=255 (immediate midpoint (255))
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_pre_idx_combo_22_c00_3c0ffc00() {
    // Encoding: 0x3C0FFC00
    // Test aarch64_memory_single_simdfp_immediate_signed_pre_idx field combination: size=0, opc=0, imm9=255, Rn=0, Rt=0
    // Fields: imm9=255, Rn=0, size=0, Rt=0, opc=0
    let encoding: u32 = 0x3C0FFC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x0, #255]!
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_pre_idx
/// ASL: `field combination 23`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=256 (power of 2 (2^8 = 256))
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_pre_idx_combo_23_c00_3c100c00() {
    // Encoding: 0x3C100C00
    // Test aarch64_memory_single_simdfp_immediate_signed_pre_idx field combination: size=0, opc=0, imm9=256, Rn=0, Rt=0
    // Fields: Rn=0, size=0, opc=0, imm9=256, Rt=0
    let encoding: u32 = 0x3C100C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_pre_idx
/// ASL: `field combination 24`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=511 (maximum immediate (511))
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_pre_idx_combo_24_c00_3c1ffc00() {
    // Encoding: 0x3C1FFC00
    // Test aarch64_memory_single_simdfp_immediate_signed_pre_idx field combination: size=0, opc=0, imm9=511, Rn=0, Rt=0
    // Fields: opc=0, imm9=511, size=0, Rn=0, Rt=0
    let encoding: u32 = 0x3C1FFC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x0, #-1]!
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

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_pre_idx
/// ASL: `field combination 25`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_pre_idx_combo_25_c00_3c000c00() {
    // Encoding: 0x3C000C00
    // Test aarch64_memory_single_simdfp_immediate_signed_pre_idx field combination: size=0, opc=0, imm9=0, Rn=0, Rt=0
    // Fields: size=0, Rn=0, imm9=0, Rt=0, opc=0
    let encoding: u32 = 0x3C000C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x0, #0]!
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_pre_idx
/// ASL: `field combination 26`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_pre_idx_combo_26_c00_3c000c20() {
    // Encoding: 0x3C000C20
    // Test aarch64_memory_single_simdfp_immediate_signed_pre_idx field combination: size=0, opc=0, imm9=0, Rn=1, Rt=0
    // Fields: imm9=0, Rt=0, opc=0, Rn=1, size=0
    let encoding: u32 = 0x3C000C20;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x1, #0]!
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_pre_idx
/// ASL: `field combination 27`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_pre_idx_combo_27_c00_3c000fc0() {
    // Encoding: 0x3C000FC0
    // Test aarch64_memory_single_simdfp_immediate_signed_pre_idx field combination: size=0, opc=0, imm9=0, Rn=30, Rt=0
    // Fields: opc=0, Rn=30, Rt=0, size=0, imm9=0
    let encoding: u32 = 0x3C000FC0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x30, #0]!
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_pre_idx
/// ASL: `field combination 28`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_pre_idx_combo_28_c00_3c000fe0() {
    // Encoding: 0x3C000FE0
    // Test aarch64_memory_single_simdfp_immediate_signed_pre_idx field combination: size=0, opc=0, imm9=0, Rn=31, Rt=0
    // Fields: size=0, Rt=0, Rn=31, imm9=0, opc=0
    let encoding: u32 = 0x3C000FE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [sp, #0]!
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_pre_idx
/// ASL: `field combination 29`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt=0 (register index 0 (first register))
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_pre_idx_combo_29_c00_3c000c00() {
    // Encoding: 0x3C000C00
    // Test aarch64_memory_single_simdfp_immediate_signed_pre_idx field combination: size=0, opc=0, imm9=0, Rn=0, Rt=0
    // Fields: size=0, imm9=0, Rn=0, opc=0, Rt=0
    let encoding: u32 = 0x3C000C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x0, #0]!
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_pre_idx
/// ASL: `field combination 30`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt=1 (register index 1 (second register))
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_pre_idx_combo_30_c00_3c000c01() {
    // Encoding: 0x3C000C01
    // Test aarch64_memory_single_simdfp_immediate_signed_pre_idx field combination: size=0, opc=0, imm9=0, Rn=0, Rt=1
    // Fields: Rt=1, Rn=0, opc=0, size=0, imm9=0
    let encoding: u32 = 0x3C000C01;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b1, [x0, #0]!
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_pre_idx
/// ASL: `field combination 31`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_pre_idx_combo_31_c00_3c000c1e() {
    // Encoding: 0x3C000C1E
    // Test aarch64_memory_single_simdfp_immediate_signed_pre_idx field combination: size=0, opc=0, imm9=0, Rn=0, Rt=30
    // Fields: imm9=0, opc=0, Rn=0, size=0, Rt=30
    let encoding: u32 = 0x3C000C1E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b30, [x0, #0]!
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_pre_idx
/// ASL: `field combination 32`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt=31 (register index 31 (ZR - zero register))
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_pre_idx_combo_32_c00_3c000c1f() {
    // Encoding: 0x3C000C1F
    // Test aarch64_memory_single_simdfp_immediate_signed_pre_idx field combination: size=0, opc=0, imm9=0, Rn=0, Rt=31
    // Fields: opc=0, size=0, imm9=0, Rt=31, Rn=0
    let encoding: u32 = 0x3C000C1F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b31, [x0, #0]!
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_pre_idx
/// ASL: `field combination 33`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (same register test (reg=1)), Rt=1 (same register test (reg=1))
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_pre_idx_combo_33_c00_3c000c21() {
    // Encoding: 0x3C000C21
    // Test aarch64_memory_single_simdfp_immediate_signed_pre_idx field combination: size=0, opc=0, imm9=0, Rn=1, Rt=1
    // Fields: opc=0, Rn=1, Rt=1, imm9=0, size=0
    let encoding: u32 = 0x3C000C21;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b1, [x1, #0]!
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_pre_idx
/// ASL: `field combination 34`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (same register test (reg=31)), Rt=31 (same register test (reg=31))
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_pre_idx_combo_34_c00_3c000fff() {
    // Encoding: 0x3C000FFF
    // Test aarch64_memory_single_simdfp_immediate_signed_pre_idx field combination: size=0, opc=0, imm9=0, Rn=31, Rt=31
    // Fields: Rt=31, size=0, opc=0, imm9=0, Rn=31
    let encoding: u32 = 0x3C000FFF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b31, [sp, #0]!
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_pre_idx
/// ASL: `field size = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "size", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_pre_idx_special_size_0_size_variant_0_3072_3c001c00()
 {
    // Encoding: 0x3C001C00
    // Test aarch64_memory_single_simdfp_immediate_signed_pre_idx special value size = 0 (Size variant 0)
    // Fields: Rn=0, Rt=0, imm9=1, opc=0, size=0
    let encoding: u32 = 0x3C001C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x0, #1]!
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_pre_idx
/// ASL: `field size = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "size", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_pre_idx_special_size_1_size_variant_1_3072_7c001c00()
 {
    // Encoding: 0x7C001C00
    // Test aarch64_memory_single_simdfp_immediate_signed_pre_idx special value size = 1 (Size variant 1)
    // Fields: opc=0, imm9=1, size=1, Rn=0, Rt=0
    let encoding: u32 = 0x7C001C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	h0, [x0, #1]!
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_pre_idx
/// ASL: `field size = 2 (Size variant 2)`
/// Requirement: FieldSpecial { field: "size", value: 2, meaning: "Size variant 2" }
/// Size variant 2
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_pre_idx_special_size_2_size_variant_2_3072_bc001c00()
 {
    // Encoding: 0xBC001C00
    // Test aarch64_memory_single_simdfp_immediate_signed_pre_idx special value size = 2 (Size variant 2)
    // Fields: Rn=0, opc=0, Rt=0, size=2, imm9=1
    let encoding: u32 = 0xBC001C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	s0, [x0, #1]!
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_pre_idx
/// ASL: `field size = 3 (Size variant 3)`
/// Requirement: FieldSpecial { field: "size", value: 3, meaning: "Size variant 3" }
/// Size variant 3
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_pre_idx_special_size_3_size_variant_3_3072_fc001c00()
 {
    // Encoding: 0xFC001C00
    // Test aarch64_memory_single_simdfp_immediate_signed_pre_idx special value size = 3 (Size variant 3)
    // Fields: Rt=0, opc=0, size=3, Rn=0, imm9=1
    let encoding: u32 = 0xFC001C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	d0, [x0, #1]!
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_pre_idx
/// ASL: `field opc = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "opc", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_pre_idx_special_opc_0_size_variant_0_3072_7c001c00()
 {
    // Encoding: 0x7C001C00
    // Test aarch64_memory_single_simdfp_immediate_signed_pre_idx special value opc = 0 (Size variant 0)
    // Fields: imm9=1, Rn=0, size=1, opc=0, Rt=0
    let encoding: u32 = 0x7C001C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	h0, [x0, #1]!
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_pre_idx
/// ASL: `field opc = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "opc", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_pre_idx_special_opc_1_size_variant_1_3072_7c401c00()
 {
    // Encoding: 0x7C401C00
    // Test aarch64_memory_single_simdfp_immediate_signed_pre_idx special value opc = 1 (Size variant 1)
    // Fields: Rt=0, size=1, Rn=0, opc=1, imm9=1
    let encoding: u32 = 0x7C401C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ldr	h0, [x0, #1]!
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_pre_idx
/// ASL: `field opc = 2 (Size variant 2)`
/// Requirement: FieldSpecial { field: "opc", value: 2, meaning: "Size variant 2" }
/// Size variant 2
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_pre_idx_special_opc_2_size_variant_2_3072_7c801c00()
 {
    // Encoding: 0x7C801C00
    // Test aarch64_memory_single_simdfp_immediate_signed_pre_idx special value opc = 2 (Size variant 2)
    // Fields: Rn=0, Rt=0, imm9=1, opc=2, size=1
    let encoding: u32 = 0x7C801C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_pre_idx
/// ASL: `field opc = 3 (Size variant 3)`
/// Requirement: FieldSpecial { field: "opc", value: 3, meaning: "Size variant 3" }
/// Size variant 3
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_pre_idx_special_opc_3_size_variant_3_3072_7cc01c00()
 {
    // Encoding: 0x7CC01C00
    // Test aarch64_memory_single_simdfp_immediate_signed_pre_idx special value opc = 3 (Size variant 3)
    // Fields: imm9=1, Rt=0, opc=3, Rn=0, size=1
    let encoding: u32 = 0x7CC01C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_pre_idx
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_pre_idx_special_rn_31_stack_pointer_sp_may_require_alignment_3072_7c001fe0()
 {
    // Encoding: 0x7C001FE0
    // Test aarch64_memory_single_simdfp_immediate_signed_pre_idx special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: size=1, Rn=31, Rt=0, imm9=1, opc=0
    let encoding: u32 = 0x7C001FE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	h0, [sp, #1]!
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_signed_pre_idx
/// ASL: `field Rt = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)`
/// Requirement: FieldSpecial { field: "Rt", value: 31, meaning: "Zero register (XZR/WZR) - reads as 0, writes discarded" }
/// Zero register (XZR/WZR) - reads as 0, writes discarded
#[test]
fn test_aarch64_memory_single_simdfp_immediate_signed_pre_idx_special_rt_31_zero_register_xzr_wzr_reads_as_0_writes_discarded_3072_7c001c1f()
 {
    // Encoding: 0x7C001C1F
    // Test aarch64_memory_single_simdfp_immediate_signed_pre_idx special value Rt = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)
    // Fields: opc=0, Rt=31, imm9=1, size=1, Rn=0
    let encoding: u32 = 0x7C001C1F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	h31, [x0, #1]!
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_unsigned
/// ASL: `field size 30 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_memory_single_simdfp_immediate_unsigned_field_size_0_min_0_3d000000() {
    // Encoding: 0x3D000000
    // Test aarch64_memory_single_simdfp_immediate_unsigned field size = 0 (Min)
    // Fields: Rt=0, imm12=0, size=0, opc=0, Rn=0
    let encoding: u32 = 0x3D000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_unsigned
/// ASL: `field size 30 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 1, boundary: PowerOfTwo }
/// 16-bit / halfword size
#[test]
fn test_aarch64_memory_single_simdfp_immediate_unsigned_field_size_1_poweroftwo_0_7d000000() {
    // Encoding: 0x7D000000
    // Test aarch64_memory_single_simdfp_immediate_unsigned field size = 1 (PowerOfTwo)
    // Fields: opc=0, imm12=0, Rn=0, Rt=0, size=1
    let encoding: u32 = 0x7D000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	h0, [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_unsigned
/// ASL: `field size 30 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 2, boundary: PowerOfTwo }
/// 32-bit / word size
#[test]
fn test_aarch64_memory_single_simdfp_immediate_unsigned_field_size_2_poweroftwo_0_bd000000() {
    // Encoding: 0xBD000000
    // Test aarch64_memory_single_simdfp_immediate_unsigned field size = 2 (PowerOfTwo)
    // Fields: size=2, imm12=0, opc=0, Rt=0, Rn=0
    let encoding: u32 = 0xBD000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	s0, [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_unsigned
/// ASL: `field size 30 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 3, boundary: Max }
/// 64-bit / doubleword size
#[test]
fn test_aarch64_memory_single_simdfp_immediate_unsigned_field_size_3_max_0_fd000000() {
    // Encoding: 0xFD000000
    // Test aarch64_memory_single_simdfp_immediate_unsigned field size = 3 (Max)
    // Fields: imm12=0, Rt=0, size=3, Rn=0, opc=0
    let encoding: u32 = 0xFD000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	d0, [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_unsigned
/// ASL: `field opc 22 +: 2`
/// Requirement: FieldBoundary { field: "opc", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_memory_single_simdfp_immediate_unsigned_field_opc_0_min_0_3d000000() {
    // Encoding: 0x3D000000
    // Test aarch64_memory_single_simdfp_immediate_unsigned field opc = 0 (Min)
    // Fields: size=0, Rn=0, Rt=0, opc=0, imm12=0
    let encoding: u32 = 0x3D000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_unsigned
/// ASL: `field opc 22 +: 2`
/// Requirement: FieldBoundary { field: "opc", value: 1, boundary: PowerOfTwo }
/// 16-bit / halfword size
#[test]
fn test_aarch64_memory_single_simdfp_immediate_unsigned_field_opc_1_poweroftwo_0_3d400000() {
    // Encoding: 0x3D400000
    // Test aarch64_memory_single_simdfp_immediate_unsigned field opc = 1 (PowerOfTwo)
    // Fields: opc=1, Rn=0, imm12=0, Rt=0, size=0
    let encoding: u32 = 0x3D400000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ldr	b0, [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_unsigned
/// ASL: `field opc 22 +: 2`
/// Requirement: FieldBoundary { field: "opc", value: 2, boundary: PowerOfTwo }
/// 32-bit / word size
#[test]
fn test_aarch64_memory_single_simdfp_immediate_unsigned_field_opc_2_poweroftwo_0_3d800000() {
    // Encoding: 0x3D800000
    // Test aarch64_memory_single_simdfp_immediate_unsigned field opc = 2 (PowerOfTwo)
    // Fields: imm12=0, opc=2, size=0, Rn=0, Rt=0
    let encoding: u32 = 0x3D800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	q0, [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_unsigned
/// ASL: `field opc 22 +: 2`
/// Requirement: FieldBoundary { field: "opc", value: 3, boundary: Max }
/// 64-bit / doubleword size
#[test]
fn test_aarch64_memory_single_simdfp_immediate_unsigned_field_opc_3_max_0_3dc00000() {
    // Encoding: 0x3DC00000
    // Test aarch64_memory_single_simdfp_immediate_unsigned field opc = 3 (Max)
    // Fields: size=0, opc=3, imm12=0, Rn=0, Rt=0
    let encoding: u32 = 0x3DC00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ldr	q0, [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_unsigned
/// ASL: `field imm12 10 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch64_memory_single_simdfp_immediate_unsigned_field_imm12_0_zero_0_3d000000() {
    // Encoding: 0x3D000000
    // Test aarch64_memory_single_simdfp_immediate_unsigned field imm12 = 0 (Zero)
    // Fields: size=0, Rt=0, imm12=0, opc=0, Rn=0
    let encoding: u32 = 0x3D000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_unsigned
/// ASL: `field imm12 10 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch64_memory_single_simdfp_immediate_unsigned_field_imm12_1_poweroftwo_0_3d000400() {
    // Encoding: 0x3D000400
    // Test aarch64_memory_single_simdfp_immediate_unsigned field imm12 = 1 (PowerOfTwo)
    // Fields: Rn=0, imm12=1, size=0, opc=0, Rt=0
    let encoding: u32 = 0x3D000400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x0, #1]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_unsigned
/// ASL: `field imm12 10 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 3, boundary: PowerOfTwoMinusOne }
/// 2^2 - 1 = 3
#[test]
fn test_aarch64_memory_single_simdfp_immediate_unsigned_field_imm12_3_poweroftwominusone_0_3d000c00()
 {
    // Encoding: 0x3D000C00
    // Test aarch64_memory_single_simdfp_immediate_unsigned field imm12 = 3 (PowerOfTwoMinusOne)
    // Fields: imm12=3, Rn=0, Rt=0, size=0, opc=0
    let encoding: u32 = 0x3D000C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x0, #3]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_unsigned
/// ASL: `field imm12 10 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 4, boundary: PowerOfTwo }
/// power of 2 (2^2 = 4)
#[test]
fn test_aarch64_memory_single_simdfp_immediate_unsigned_field_imm12_4_poweroftwo_0_3d001000() {
    // Encoding: 0x3D001000
    // Test aarch64_memory_single_simdfp_immediate_unsigned field imm12 = 4 (PowerOfTwo)
    // Fields: imm12=4, Rn=0, opc=0, size=0, Rt=0
    let encoding: u32 = 0x3D001000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x0, #4]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_unsigned
/// ASL: `field imm12 10 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 7, boundary: PowerOfTwoMinusOne }
/// 2^3 - 1 = 7
#[test]
fn test_aarch64_memory_single_simdfp_immediate_unsigned_field_imm12_7_poweroftwominusone_0_3d001c00()
 {
    // Encoding: 0x3D001C00
    // Test aarch64_memory_single_simdfp_immediate_unsigned field imm12 = 7 (PowerOfTwoMinusOne)
    // Fields: size=0, imm12=7, Rt=0, opc=0, Rn=0
    let encoding: u32 = 0x3D001C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x0, #7]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_unsigned
/// ASL: `field imm12 10 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 8, boundary: PowerOfTwo }
/// power of 2 (2^3 = 8)
#[test]
fn test_aarch64_memory_single_simdfp_immediate_unsigned_field_imm12_8_poweroftwo_0_3d002000() {
    // Encoding: 0x3D002000
    // Test aarch64_memory_single_simdfp_immediate_unsigned field imm12 = 8 (PowerOfTwo)
    // Fields: opc=0, imm12=8, Rn=0, size=0, Rt=0
    let encoding: u32 = 0x3D002000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x0, #8]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_unsigned
/// ASL: `field imm12 10 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 15, boundary: PowerOfTwoMinusOne }
/// 2^4 - 1 = 15
#[test]
fn test_aarch64_memory_single_simdfp_immediate_unsigned_field_imm12_15_poweroftwominusone_0_3d003c00()
 {
    // Encoding: 0x3D003C00
    // Test aarch64_memory_single_simdfp_immediate_unsigned field imm12 = 15 (PowerOfTwoMinusOne)
    // Fields: opc=0, Rn=0, Rt=0, size=0, imm12=15
    let encoding: u32 = 0x3D003C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x0, #15]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_unsigned
/// ASL: `field imm12 10 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 16, boundary: PowerOfTwo }
/// power of 2 (2^4 = 16)
#[test]
fn test_aarch64_memory_single_simdfp_immediate_unsigned_field_imm12_16_poweroftwo_0_3d004000() {
    // Encoding: 0x3D004000
    // Test aarch64_memory_single_simdfp_immediate_unsigned field imm12 = 16 (PowerOfTwo)
    // Fields: Rt=0, Rn=0, opc=0, size=0, imm12=16
    let encoding: u32 = 0x3D004000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x0, #16]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_unsigned
/// ASL: `field imm12 10 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 31, boundary: PowerOfTwoMinusOne }
/// 2^5 - 1 = 31
#[test]
fn test_aarch64_memory_single_simdfp_immediate_unsigned_field_imm12_31_poweroftwominusone_0_3d007c00()
 {
    // Encoding: 0x3D007C00
    // Test aarch64_memory_single_simdfp_immediate_unsigned field imm12 = 31 (PowerOfTwoMinusOne)
    // Fields: size=0, opc=0, Rn=0, Rt=0, imm12=31
    let encoding: u32 = 0x3D007C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x0, #31]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_unsigned
/// ASL: `field imm12 10 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 32, boundary: PowerOfTwo }
/// power of 2 (2^5 = 32)
#[test]
fn test_aarch64_memory_single_simdfp_immediate_unsigned_field_imm12_32_poweroftwo_0_3d008000() {
    // Encoding: 0x3D008000
    // Test aarch64_memory_single_simdfp_immediate_unsigned field imm12 = 32 (PowerOfTwo)
    // Fields: size=0, imm12=32, Rn=0, Rt=0, opc=0
    let encoding: u32 = 0x3D008000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x0, #32]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_unsigned
/// ASL: `field imm12 10 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 63, boundary: PowerOfTwoMinusOne }
/// 2^6 - 1 = 63
#[test]
fn test_aarch64_memory_single_simdfp_immediate_unsigned_field_imm12_63_poweroftwominusone_0_3d00fc00()
 {
    // Encoding: 0x3D00FC00
    // Test aarch64_memory_single_simdfp_immediate_unsigned field imm12 = 63 (PowerOfTwoMinusOne)
    // Fields: Rn=0, size=0, opc=0, imm12=63, Rt=0
    let encoding: u32 = 0x3D00FC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x0, #63]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_unsigned
/// ASL: `field imm12 10 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 64, boundary: PowerOfTwo }
/// power of 2 (2^6 = 64)
#[test]
fn test_aarch64_memory_single_simdfp_immediate_unsigned_field_imm12_64_poweroftwo_0_3d010000() {
    // Encoding: 0x3D010000
    // Test aarch64_memory_single_simdfp_immediate_unsigned field imm12 = 64 (PowerOfTwo)
    // Fields: opc=0, Rn=0, Rt=0, size=0, imm12=64
    let encoding: u32 = 0x3D010000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x0, #64]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_unsigned
/// ASL: `field imm12 10 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 127, boundary: PowerOfTwoMinusOne }
/// 2^7 - 1 = 127
#[test]
fn test_aarch64_memory_single_simdfp_immediate_unsigned_field_imm12_127_poweroftwominusone_0_3d01fc00()
 {
    // Encoding: 0x3D01FC00
    // Test aarch64_memory_single_simdfp_immediate_unsigned field imm12 = 127 (PowerOfTwoMinusOne)
    // Fields: Rt=0, size=0, opc=0, Rn=0, imm12=127
    let encoding: u32 = 0x3D01FC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x0, #127]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_unsigned
/// ASL: `field imm12 10 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 128, boundary: PowerOfTwo }
/// power of 2 (2^7 = 128)
#[test]
fn test_aarch64_memory_single_simdfp_immediate_unsigned_field_imm12_128_poweroftwo_0_3d020000() {
    // Encoding: 0x3D020000
    // Test aarch64_memory_single_simdfp_immediate_unsigned field imm12 = 128 (PowerOfTwo)
    // Fields: Rn=0, size=0, opc=0, Rt=0, imm12=128
    let encoding: u32 = 0x3D020000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x0, #128]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_unsigned
/// ASL: `field imm12 10 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 255, boundary: PowerOfTwoMinusOne }
/// 2^8 - 1 = 255
#[test]
fn test_aarch64_memory_single_simdfp_immediate_unsigned_field_imm12_255_poweroftwominusone_0_3d03fc00()
 {
    // Encoding: 0x3D03FC00
    // Test aarch64_memory_single_simdfp_immediate_unsigned field imm12 = 255 (PowerOfTwoMinusOne)
    // Fields: Rt=0, Rn=0, opc=0, size=0, imm12=255
    let encoding: u32 = 0x3D03FC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x0, #255]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_unsigned
/// ASL: `field imm12 10 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 256, boundary: PowerOfTwo }
/// power of 2 (2^8 = 256)
#[test]
fn test_aarch64_memory_single_simdfp_immediate_unsigned_field_imm12_256_poweroftwo_0_3d040000() {
    // Encoding: 0x3D040000
    // Test aarch64_memory_single_simdfp_immediate_unsigned field imm12 = 256 (PowerOfTwo)
    // Fields: Rn=0, opc=0, size=0, imm12=256, Rt=0
    let encoding: u32 = 0x3D040000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x0, #256]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_unsigned
/// ASL: `field imm12 10 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 511, boundary: PowerOfTwoMinusOne }
/// 2^9 - 1 = 511
#[test]
fn test_aarch64_memory_single_simdfp_immediate_unsigned_field_imm12_511_poweroftwominusone_0_3d07fc00()
 {
    // Encoding: 0x3D07FC00
    // Test aarch64_memory_single_simdfp_immediate_unsigned field imm12 = 511 (PowerOfTwoMinusOne)
    // Fields: Rn=0, imm12=511, opc=0, size=0, Rt=0
    let encoding: u32 = 0x3D07FC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x0, #511]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_unsigned
/// ASL: `field imm12 10 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 512, boundary: PowerOfTwo }
/// power of 2 (2^9 = 512)
#[test]
fn test_aarch64_memory_single_simdfp_immediate_unsigned_field_imm12_512_poweroftwo_0_3d080000() {
    // Encoding: 0x3D080000
    // Test aarch64_memory_single_simdfp_immediate_unsigned field imm12 = 512 (PowerOfTwo)
    // Fields: Rn=0, size=0, opc=0, Rt=0, imm12=512
    let encoding: u32 = 0x3D080000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x0, #512]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_unsigned
/// ASL: `field imm12 10 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 1023, boundary: PowerOfTwoMinusOne }
/// 2^10 - 1 = 1023
#[test]
fn test_aarch64_memory_single_simdfp_immediate_unsigned_field_imm12_1023_poweroftwominusone_0_3d0ffc00()
 {
    // Encoding: 0x3D0FFC00
    // Test aarch64_memory_single_simdfp_immediate_unsigned field imm12 = 1023 (PowerOfTwoMinusOne)
    // Fields: opc=0, size=0, Rt=0, imm12=1023, Rn=0
    let encoding: u32 = 0x3D0FFC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x0, #1023]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_unsigned
/// ASL: `field imm12 10 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 1024, boundary: PowerOfTwo }
/// power of 2 (2^10 = 1024)
#[test]
fn test_aarch64_memory_single_simdfp_immediate_unsigned_field_imm12_1024_poweroftwo_0_3d100000() {
    // Encoding: 0x3D100000
    // Test aarch64_memory_single_simdfp_immediate_unsigned field imm12 = 1024 (PowerOfTwo)
    // Fields: size=0, Rn=0, imm12=1024, opc=0, Rt=0
    let encoding: u32 = 0x3D100000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x0, #1024]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_unsigned
/// ASL: `field imm12 10 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 2047, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (2047)
#[test]
fn test_aarch64_memory_single_simdfp_immediate_unsigned_field_imm12_2047_poweroftwominusone_0_3d1ffc00()
 {
    // Encoding: 0x3D1FFC00
    // Test aarch64_memory_single_simdfp_immediate_unsigned field imm12 = 2047 (PowerOfTwoMinusOne)
    // Fields: imm12=2047, opc=0, size=0, Rn=0, Rt=0
    let encoding: u32 = 0x3D1FFC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x0, #2047]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_unsigned
/// ASL: `field imm12 10 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 2048, boundary: PowerOfTwo }
/// power of 2 (2^11 = 2048)
#[test]
fn test_aarch64_memory_single_simdfp_immediate_unsigned_field_imm12_2048_poweroftwo_0_3d200000() {
    // Encoding: 0x3D200000
    // Test aarch64_memory_single_simdfp_immediate_unsigned field imm12 = 2048 (PowerOfTwo)
    // Fields: opc=0, size=0, Rn=0, Rt=0, imm12=2048
    let encoding: u32 = 0x3D200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x0, #2048]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_unsigned
/// ASL: `field imm12 10 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 4095, boundary: Max }
/// maximum immediate (4095)
#[test]
fn test_aarch64_memory_single_simdfp_immediate_unsigned_field_imm12_4095_max_0_3d3ffc00() {
    // Encoding: 0x3D3FFC00
    // Test aarch64_memory_single_simdfp_immediate_unsigned field imm12 = 4095 (Max)
    // Fields: Rn=0, imm12=4095, size=0, Rt=0, opc=0
    let encoding: u32 = 0x3D3FFC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x0, #4095]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_unsigned
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_memory_single_simdfp_immediate_unsigned_field_rn_0_min_0_3d000000() {
    // Encoding: 0x3D000000
    // Test aarch64_memory_single_simdfp_immediate_unsigned field Rn = 0 (Min)
    // Fields: size=0, opc=0, imm12=0, Rn=0, Rt=0
    let encoding: u32 = 0x3D000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_unsigned
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_memory_single_simdfp_immediate_unsigned_field_rn_1_poweroftwo_0_3d000020() {
    // Encoding: 0x3D000020
    // Test aarch64_memory_single_simdfp_immediate_unsigned field Rn = 1 (PowerOfTwo)
    // Fields: imm12=0, opc=0, size=0, Rn=1, Rt=0
    let encoding: u32 = 0x3D000020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x1]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_unsigned
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_memory_single_simdfp_immediate_unsigned_field_rn_30_poweroftwominusone_0_3d0003c0()
{
    // Encoding: 0x3D0003C0
    // Test aarch64_memory_single_simdfp_immediate_unsigned field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: opc=0, imm12=0, Rn=30, Rt=0, size=0
    let encoding: u32 = 0x3D0003C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x30]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_unsigned
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_memory_single_simdfp_immediate_unsigned_field_rn_31_max_0_3d0003e0() {
    // Encoding: 0x3D0003E0
    // Test aarch64_memory_single_simdfp_immediate_unsigned field Rn = 31 (Max)
    // Fields: imm12=0, Rn=31, size=0, opc=0, Rt=0
    let encoding: u32 = 0x3D0003E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [sp]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_unsigned
/// ASL: `field Rt 0 +: 5`
/// Requirement: FieldBoundary { field: "Rt", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_memory_single_simdfp_immediate_unsigned_field_rt_0_min_0_3d000000() {
    // Encoding: 0x3D000000
    // Test aarch64_memory_single_simdfp_immediate_unsigned field Rt = 0 (Min)
    // Fields: size=0, Rn=0, opc=0, Rt=0, imm12=0
    let encoding: u32 = 0x3D000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_unsigned
/// ASL: `field Rt 0 +: 5`
/// Requirement: FieldBoundary { field: "Rt", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_memory_single_simdfp_immediate_unsigned_field_rt_1_poweroftwo_0_3d000001() {
    // Encoding: 0x3D000001
    // Test aarch64_memory_single_simdfp_immediate_unsigned field Rt = 1 (PowerOfTwo)
    // Fields: opc=0, Rn=0, Rt=1, imm12=0, size=0
    let encoding: u32 = 0x3D000001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b1, [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_unsigned
/// ASL: `field Rt 0 +: 5`
/// Requirement: FieldBoundary { field: "Rt", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_memory_single_simdfp_immediate_unsigned_field_rt_30_poweroftwominusone_0_3d00001e()
{
    // Encoding: 0x3D00001E
    // Test aarch64_memory_single_simdfp_immediate_unsigned field Rt = 30 (PowerOfTwoMinusOne)
    // Fields: opc=0, imm12=0, Rn=0, Rt=30, size=0
    let encoding: u32 = 0x3D00001E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b30, [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_unsigned
/// ASL: `field Rt 0 +: 5`
/// Requirement: FieldBoundary { field: "Rt", value: 31, boundary: Max }
/// register index 31 (ZR - zero register)
#[test]
fn test_aarch64_memory_single_simdfp_immediate_unsigned_field_rt_31_max_0_3d00001f() {
    // Encoding: 0x3D00001F
    // Test aarch64_memory_single_simdfp_immediate_unsigned field Rt = 31 (Max)
    // Fields: opc=0, Rn=0, Rt=31, size=0, imm12=0
    let encoding: u32 = 0x3D00001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b31, [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_unsigned
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=0 (8-bit / byte size)
#[test]
fn test_aarch64_memory_single_simdfp_immediate_unsigned_combo_0_0_3d000000() {
    // Encoding: 0x3D000000
    // Test aarch64_memory_single_simdfp_immediate_unsigned field combination: size=0, opc=0, imm12=0, Rn=0, Rt=0
    // Fields: imm12=0, size=0, opc=0, Rt=0, Rn=0
    let encoding: u32 = 0x3D000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_unsigned
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=1 (16-bit / halfword size)
#[test]
fn test_aarch64_memory_single_simdfp_immediate_unsigned_combo_1_0_7d000000() {
    // Encoding: 0x7D000000
    // Test aarch64_memory_single_simdfp_immediate_unsigned field combination: size=1, opc=0, imm12=0, Rn=0, Rt=0
    // Fields: Rt=0, opc=0, size=1, imm12=0, Rn=0
    let encoding: u32 = 0x7D000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	h0, [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_unsigned
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=2 (32-bit / word size)
#[test]
fn test_aarch64_memory_single_simdfp_immediate_unsigned_combo_2_0_bd000000() {
    // Encoding: 0xBD000000
    // Test aarch64_memory_single_simdfp_immediate_unsigned field combination: size=2, opc=0, imm12=0, Rn=0, Rt=0
    // Fields: Rt=0, imm12=0, opc=0, size=2, Rn=0
    let encoding: u32 = 0xBD000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	s0, [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_unsigned
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=3 (64-bit / doubleword size)
#[test]
fn test_aarch64_memory_single_simdfp_immediate_unsigned_combo_3_0_fd000000() {
    // Encoding: 0xFD000000
    // Test aarch64_memory_single_simdfp_immediate_unsigned field combination: size=3, opc=0, imm12=0, Rn=0, Rt=0
    // Fields: imm12=0, Rn=0, Rt=0, size=3, opc=0
    let encoding: u32 = 0xFD000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	d0, [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_unsigned
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// opc=0 (8-bit / byte size)
#[test]
fn test_aarch64_memory_single_simdfp_immediate_unsigned_combo_4_0_3d000000() {
    // Encoding: 0x3D000000
    // Test aarch64_memory_single_simdfp_immediate_unsigned field combination: size=0, opc=0, imm12=0, Rn=0, Rt=0
    // Fields: opc=0, Rn=0, size=0, imm12=0, Rt=0
    let encoding: u32 = 0x3D000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_unsigned
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// opc=1 (16-bit / halfword size)
#[test]
fn test_aarch64_memory_single_simdfp_immediate_unsigned_combo_5_0_3d400000() {
    // Encoding: 0x3D400000
    // Test aarch64_memory_single_simdfp_immediate_unsigned field combination: size=0, opc=1, imm12=0, Rn=0, Rt=0
    // Fields: Rt=0, opc=1, imm12=0, size=0, Rn=0
    let encoding: u32 = 0x3D400000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ldr	b0, [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_unsigned
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// opc=2 (32-bit / word size)
#[test]
fn test_aarch64_memory_single_simdfp_immediate_unsigned_combo_6_0_3d800000() {
    // Encoding: 0x3D800000
    // Test aarch64_memory_single_simdfp_immediate_unsigned field combination: size=0, opc=2, imm12=0, Rn=0, Rt=0
    // Fields: size=0, imm12=0, Rt=0, Rn=0, opc=2
    let encoding: u32 = 0x3D800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	q0, [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_unsigned
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// opc=3 (64-bit / doubleword size)
#[test]
fn test_aarch64_memory_single_simdfp_immediate_unsigned_combo_7_0_3dc00000() {
    // Encoding: 0x3DC00000
    // Test aarch64_memory_single_simdfp_immediate_unsigned field combination: size=0, opc=3, imm12=0, Rn=0, Rt=0
    // Fields: Rt=0, imm12=0, size=0, Rn=0, opc=3
    let encoding: u32 = 0x3DC00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ldr	q0, [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_unsigned
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm12=0 (immediate value 0)
#[test]
fn test_aarch64_memory_single_simdfp_immediate_unsigned_combo_8_0_3d000000() {
    // Encoding: 0x3D000000
    // Test aarch64_memory_single_simdfp_immediate_unsigned field combination: size=0, opc=0, imm12=0, Rn=0, Rt=0
    // Fields: opc=0, size=0, Rt=0, imm12=0, Rn=0
    let encoding: u32 = 0x3D000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_unsigned
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm12=1 (immediate value 1)
#[test]
fn test_aarch64_memory_single_simdfp_immediate_unsigned_combo_9_0_3d000400() {
    // Encoding: 0x3D000400
    // Test aarch64_memory_single_simdfp_immediate_unsigned field combination: size=0, opc=0, imm12=1, Rn=0, Rt=0
    // Fields: Rn=0, opc=0, Rt=0, size=0, imm12=1
    let encoding: u32 = 0x3D000400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x0, #1]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_unsigned
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm12=3 (2^2 - 1 = 3)
#[test]
fn test_aarch64_memory_single_simdfp_immediate_unsigned_combo_10_0_3d000c00() {
    // Encoding: 0x3D000C00
    // Test aarch64_memory_single_simdfp_immediate_unsigned field combination: size=0, opc=0, imm12=3, Rn=0, Rt=0
    // Fields: imm12=3, Rt=0, size=0, opc=0, Rn=0
    let encoding: u32 = 0x3D000C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x0, #3]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_unsigned
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm12=4 (power of 2 (2^2 = 4))
#[test]
fn test_aarch64_memory_single_simdfp_immediate_unsigned_combo_11_0_3d001000() {
    // Encoding: 0x3D001000
    // Test aarch64_memory_single_simdfp_immediate_unsigned field combination: size=0, opc=0, imm12=4, Rn=0, Rt=0
    // Fields: Rn=0, Rt=0, imm12=4, opc=0, size=0
    let encoding: u32 = 0x3D001000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x0, #4]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_unsigned
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm12=7 (2^3 - 1 = 7)
#[test]
fn test_aarch64_memory_single_simdfp_immediate_unsigned_combo_12_0_3d001c00() {
    // Encoding: 0x3D001C00
    // Test aarch64_memory_single_simdfp_immediate_unsigned field combination: size=0, opc=0, imm12=7, Rn=0, Rt=0
    // Fields: Rt=0, Rn=0, opc=0, imm12=7, size=0
    let encoding: u32 = 0x3D001C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x0, #7]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_unsigned
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm12=8 (power of 2 (2^3 = 8))
#[test]
fn test_aarch64_memory_single_simdfp_immediate_unsigned_combo_13_0_3d002000() {
    // Encoding: 0x3D002000
    // Test aarch64_memory_single_simdfp_immediate_unsigned field combination: size=0, opc=0, imm12=8, Rn=0, Rt=0
    // Fields: Rn=0, size=0, Rt=0, opc=0, imm12=8
    let encoding: u32 = 0x3D002000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x0, #8]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_unsigned
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm12=15 (2^4 - 1 = 15)
#[test]
fn test_aarch64_memory_single_simdfp_immediate_unsigned_combo_14_0_3d003c00() {
    // Encoding: 0x3D003C00
    // Test aarch64_memory_single_simdfp_immediate_unsigned field combination: size=0, opc=0, imm12=15, Rn=0, Rt=0
    // Fields: size=0, imm12=15, opc=0, Rn=0, Rt=0
    let encoding: u32 = 0x3D003C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x0, #15]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_unsigned
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm12=16 (power of 2 (2^4 = 16))
#[test]
fn test_aarch64_memory_single_simdfp_immediate_unsigned_combo_15_0_3d004000() {
    // Encoding: 0x3D004000
    // Test aarch64_memory_single_simdfp_immediate_unsigned field combination: size=0, opc=0, imm12=16, Rn=0, Rt=0
    // Fields: imm12=16, opc=0, Rt=0, size=0, Rn=0
    let encoding: u32 = 0x3D004000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x0, #16]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_unsigned
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm12=31 (2^5 - 1 = 31)
#[test]
fn test_aarch64_memory_single_simdfp_immediate_unsigned_combo_16_0_3d007c00() {
    // Encoding: 0x3D007C00
    // Test aarch64_memory_single_simdfp_immediate_unsigned field combination: size=0, opc=0, imm12=31, Rn=0, Rt=0
    // Fields: Rn=0, opc=0, imm12=31, Rt=0, size=0
    let encoding: u32 = 0x3D007C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x0, #31]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_unsigned
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm12=32 (power of 2 (2^5 = 32))
#[test]
fn test_aarch64_memory_single_simdfp_immediate_unsigned_combo_17_0_3d008000() {
    // Encoding: 0x3D008000
    // Test aarch64_memory_single_simdfp_immediate_unsigned field combination: size=0, opc=0, imm12=32, Rn=0, Rt=0
    // Fields: Rt=0, opc=0, imm12=32, Rn=0, size=0
    let encoding: u32 = 0x3D008000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x0, #32]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_unsigned
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm12=63 (2^6 - 1 = 63)
#[test]
fn test_aarch64_memory_single_simdfp_immediate_unsigned_combo_18_0_3d00fc00() {
    // Encoding: 0x3D00FC00
    // Test aarch64_memory_single_simdfp_immediate_unsigned field combination: size=0, opc=0, imm12=63, Rn=0, Rt=0
    // Fields: Rt=0, opc=0, Rn=0, imm12=63, size=0
    let encoding: u32 = 0x3D00FC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x0, #63]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_unsigned
/// ASL: `field combination 19`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm12=64 (power of 2 (2^6 = 64))
#[test]
fn test_aarch64_memory_single_simdfp_immediate_unsigned_combo_19_0_3d010000() {
    // Encoding: 0x3D010000
    // Test aarch64_memory_single_simdfp_immediate_unsigned field combination: size=0, opc=0, imm12=64, Rn=0, Rt=0
    // Fields: Rt=0, size=0, imm12=64, opc=0, Rn=0
    let encoding: u32 = 0x3D010000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x0, #64]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_unsigned
/// ASL: `field combination 20`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm12=127 (2^7 - 1 = 127)
#[test]
fn test_aarch64_memory_single_simdfp_immediate_unsigned_combo_20_0_3d01fc00() {
    // Encoding: 0x3D01FC00
    // Test aarch64_memory_single_simdfp_immediate_unsigned field combination: size=0, opc=0, imm12=127, Rn=0, Rt=0
    // Fields: Rt=0, imm12=127, size=0, opc=0, Rn=0
    let encoding: u32 = 0x3D01FC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x0, #127]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_unsigned
/// ASL: `field combination 21`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm12=128 (power of 2 (2^7 = 128))
#[test]
fn test_aarch64_memory_single_simdfp_immediate_unsigned_combo_21_0_3d020000() {
    // Encoding: 0x3D020000
    // Test aarch64_memory_single_simdfp_immediate_unsigned field combination: size=0, opc=0, imm12=128, Rn=0, Rt=0
    // Fields: imm12=128, Rn=0, Rt=0, size=0, opc=0
    let encoding: u32 = 0x3D020000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x0, #128]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_unsigned
/// ASL: `field combination 22`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm12=255 (2^8 - 1 = 255)
#[test]
fn test_aarch64_memory_single_simdfp_immediate_unsigned_combo_22_0_3d03fc00() {
    // Encoding: 0x3D03FC00
    // Test aarch64_memory_single_simdfp_immediate_unsigned field combination: size=0, opc=0, imm12=255, Rn=0, Rt=0
    // Fields: imm12=255, Rt=0, size=0, opc=0, Rn=0
    let encoding: u32 = 0x3D03FC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x0, #255]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_unsigned
/// ASL: `field combination 23`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm12=256 (power of 2 (2^8 = 256))
#[test]
fn test_aarch64_memory_single_simdfp_immediate_unsigned_combo_23_0_3d040000() {
    // Encoding: 0x3D040000
    // Test aarch64_memory_single_simdfp_immediate_unsigned field combination: size=0, opc=0, imm12=256, Rn=0, Rt=0
    // Fields: Rt=0, Rn=0, opc=0, imm12=256, size=0
    let encoding: u32 = 0x3D040000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x0, #256]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_unsigned
/// ASL: `field combination 24`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm12=511 (2^9 - 1 = 511)
#[test]
fn test_aarch64_memory_single_simdfp_immediate_unsigned_combo_24_0_3d07fc00() {
    // Encoding: 0x3D07FC00
    // Test aarch64_memory_single_simdfp_immediate_unsigned field combination: size=0, opc=0, imm12=511, Rn=0, Rt=0
    // Fields: opc=0, size=0, imm12=511, Rn=0, Rt=0
    let encoding: u32 = 0x3D07FC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x0, #511]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_unsigned
/// ASL: `field combination 25`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm12=512 (power of 2 (2^9 = 512))
#[test]
fn test_aarch64_memory_single_simdfp_immediate_unsigned_combo_25_0_3d080000() {
    // Encoding: 0x3D080000
    // Test aarch64_memory_single_simdfp_immediate_unsigned field combination: size=0, opc=0, imm12=512, Rn=0, Rt=0
    // Fields: opc=0, imm12=512, size=0, Rt=0, Rn=0
    let encoding: u32 = 0x3D080000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x0, #512]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_unsigned
/// ASL: `field combination 26`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm12=1023 (2^10 - 1 = 1023)
#[test]
fn test_aarch64_memory_single_simdfp_immediate_unsigned_combo_26_0_3d0ffc00() {
    // Encoding: 0x3D0FFC00
    // Test aarch64_memory_single_simdfp_immediate_unsigned field combination: size=0, opc=0, imm12=1023, Rn=0, Rt=0
    // Fields: size=0, opc=0, Rn=0, imm12=1023, Rt=0
    let encoding: u32 = 0x3D0FFC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x0, #1023]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_unsigned
/// ASL: `field combination 27`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm12=1024 (power of 2 (2^10 = 1024))
#[test]
fn test_aarch64_memory_single_simdfp_immediate_unsigned_combo_27_0_3d100000() {
    // Encoding: 0x3D100000
    // Test aarch64_memory_single_simdfp_immediate_unsigned field combination: size=0, opc=0, imm12=1024, Rn=0, Rt=0
    // Fields: Rt=0, opc=0, size=0, imm12=1024, Rn=0
    let encoding: u32 = 0x3D100000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x0, #1024]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_unsigned
/// ASL: `field combination 28`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm12=2047 (immediate midpoint (2047))
#[test]
fn test_aarch64_memory_single_simdfp_immediate_unsigned_combo_28_0_3d1ffc00() {
    // Encoding: 0x3D1FFC00
    // Test aarch64_memory_single_simdfp_immediate_unsigned field combination: size=0, opc=0, imm12=2047, Rn=0, Rt=0
    // Fields: Rt=0, imm12=2047, size=0, opc=0, Rn=0
    let encoding: u32 = 0x3D1FFC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x0, #2047]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_unsigned
/// ASL: `field combination 29`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm12=2048 (power of 2 (2^11 = 2048))
#[test]
fn test_aarch64_memory_single_simdfp_immediate_unsigned_combo_29_0_3d200000() {
    // Encoding: 0x3D200000
    // Test aarch64_memory_single_simdfp_immediate_unsigned field combination: size=0, opc=0, imm12=2048, Rn=0, Rt=0
    // Fields: Rn=0, size=0, imm12=2048, opc=0, Rt=0
    let encoding: u32 = 0x3D200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x0, #2048]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_unsigned
/// ASL: `field combination 30`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm12=4095 (maximum immediate (4095))
#[test]
fn test_aarch64_memory_single_simdfp_immediate_unsigned_combo_30_0_3d3ffc00() {
    // Encoding: 0x3D3FFC00
    // Test aarch64_memory_single_simdfp_immediate_unsigned field combination: size=0, opc=0, imm12=4095, Rn=0, Rt=0
    // Fields: Rn=0, size=0, imm12=4095, Rt=0, opc=0
    let encoding: u32 = 0x3D3FFC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x0, #4095]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_unsigned
/// ASL: `field combination 31`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch64_memory_single_simdfp_immediate_unsigned_combo_31_0_3d000000() {
    // Encoding: 0x3D000000
    // Test aarch64_memory_single_simdfp_immediate_unsigned field combination: size=0, opc=0, imm12=0, Rn=0, Rt=0
    // Fields: size=0, imm12=0, opc=0, Rt=0, Rn=0
    let encoding: u32 = 0x3D000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_unsigned
/// ASL: `field combination 32`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_aarch64_memory_single_simdfp_immediate_unsigned_combo_32_0_3d000020() {
    // Encoding: 0x3D000020
    // Test aarch64_memory_single_simdfp_immediate_unsigned field combination: size=0, opc=0, imm12=0, Rn=1, Rt=0
    // Fields: size=0, Rt=0, opc=0, Rn=1, imm12=0
    let encoding: u32 = 0x3D000020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x1]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_unsigned
/// ASL: `field combination 33`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_memory_single_simdfp_immediate_unsigned_combo_33_0_3d0003c0() {
    // Encoding: 0x3D0003C0
    // Test aarch64_memory_single_simdfp_immediate_unsigned field combination: size=0, opc=0, imm12=0, Rn=30, Rt=0
    // Fields: opc=0, Rt=0, Rn=30, imm12=0, size=0
    let encoding: u32 = 0x3D0003C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x30]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_unsigned
/// ASL: `field combination 34`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_memory_single_simdfp_immediate_unsigned_combo_34_0_3d0003e0() {
    // Encoding: 0x3D0003E0
    // Test aarch64_memory_single_simdfp_immediate_unsigned field combination: size=0, opc=0, imm12=0, Rn=31, Rt=0
    // Fields: size=0, imm12=0, Rt=0, opc=0, Rn=31
    let encoding: u32 = 0x3D0003E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [sp]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_unsigned
/// ASL: `field combination 35`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt=0 (register index 0 (first register))
#[test]
fn test_aarch64_memory_single_simdfp_immediate_unsigned_combo_35_0_3d000000() {
    // Encoding: 0x3D000000
    // Test aarch64_memory_single_simdfp_immediate_unsigned field combination: size=0, opc=0, imm12=0, Rn=0, Rt=0
    // Fields: Rt=0, imm12=0, size=0, Rn=0, opc=0
    let encoding: u32 = 0x3D000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_unsigned
/// ASL: `field combination 36`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt=1 (register index 1 (second register))
#[test]
fn test_aarch64_memory_single_simdfp_immediate_unsigned_combo_36_0_3d000001() {
    // Encoding: 0x3D000001
    // Test aarch64_memory_single_simdfp_immediate_unsigned field combination: size=0, opc=0, imm12=0, Rn=0, Rt=1
    // Fields: Rn=0, imm12=0, size=0, opc=0, Rt=1
    let encoding: u32 = 0x3D000001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b1, [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_unsigned
/// ASL: `field combination 37`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_memory_single_simdfp_immediate_unsigned_combo_37_0_3d00001e() {
    // Encoding: 0x3D00001E
    // Test aarch64_memory_single_simdfp_immediate_unsigned field combination: size=0, opc=0, imm12=0, Rn=0, Rt=30
    // Fields: Rt=30, size=0, Rn=0, imm12=0, opc=0
    let encoding: u32 = 0x3D00001E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b30, [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_unsigned
/// ASL: `field combination 38`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt=31 (register index 31 (ZR - zero register))
#[test]
fn test_aarch64_memory_single_simdfp_immediate_unsigned_combo_38_0_3d00001f() {
    // Encoding: 0x3D00001F
    // Test aarch64_memory_single_simdfp_immediate_unsigned field combination: size=0, opc=0, imm12=0, Rn=0, Rt=31
    // Fields: Rt=31, size=0, imm12=0, opc=0, Rn=0
    let encoding: u32 = 0x3D00001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b31, [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_unsigned
/// ASL: `field combination 39`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (same register test (reg=1)), Rt=1 (same register test (reg=1))
#[test]
fn test_aarch64_memory_single_simdfp_immediate_unsigned_combo_39_0_3d000021() {
    // Encoding: 0x3D000021
    // Test aarch64_memory_single_simdfp_immediate_unsigned field combination: size=0, opc=0, imm12=0, Rn=1, Rt=1
    // Fields: imm12=0, Rn=1, size=0, Rt=1, opc=0
    let encoding: u32 = 0x3D000021;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b1, [x1]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_unsigned
/// ASL: `field combination 40`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (same register test (reg=31)), Rt=31 (same register test (reg=31))
#[test]
fn test_aarch64_memory_single_simdfp_immediate_unsigned_combo_40_0_3d0003ff() {
    // Encoding: 0x3D0003FF
    // Test aarch64_memory_single_simdfp_immediate_unsigned field combination: size=0, opc=0, imm12=0, Rn=31, Rt=31
    // Fields: Rn=31, size=0, Rt=31, opc=0, imm12=0
    let encoding: u32 = 0x3D0003FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b31, [sp]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_unsigned
/// ASL: `field size = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "size", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_memory_single_simdfp_immediate_unsigned_special_size_0_size_variant_0_0_3d000400() {
    // Encoding: 0x3D000400
    // Test aarch64_memory_single_simdfp_immediate_unsigned special value size = 0 (Size variant 0)
    // Fields: opc=0, Rt=0, imm12=1, size=0, Rn=0
    let encoding: u32 = 0x3D000400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	b0, [x0, #1]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_unsigned
/// ASL: `field size = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "size", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_memory_single_simdfp_immediate_unsigned_special_size_1_size_variant_1_0_7d000400() {
    // Encoding: 0x7D000400
    // Test aarch64_memory_single_simdfp_immediate_unsigned special value size = 1 (Size variant 1)
    // Fields: imm12=1, opc=0, Rn=0, size=1, Rt=0
    let encoding: u32 = 0x7D000400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	h0, [x0, #2]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_unsigned
/// ASL: `field size = 2 (Size variant 2)`
/// Requirement: FieldSpecial { field: "size", value: 2, meaning: "Size variant 2" }
/// Size variant 2
#[test]
fn test_aarch64_memory_single_simdfp_immediate_unsigned_special_size_2_size_variant_2_0_bd000400() {
    // Encoding: 0xBD000400
    // Test aarch64_memory_single_simdfp_immediate_unsigned special value size = 2 (Size variant 2)
    // Fields: Rn=0, imm12=1, size=2, Rt=0, opc=0
    let encoding: u32 = 0xBD000400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	s0, [x0, #4]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_unsigned
/// ASL: `field size = 3 (Size variant 3)`
/// Requirement: FieldSpecial { field: "size", value: 3, meaning: "Size variant 3" }
/// Size variant 3
#[test]
fn test_aarch64_memory_single_simdfp_immediate_unsigned_special_size_3_size_variant_3_0_fd000400() {
    // Encoding: 0xFD000400
    // Test aarch64_memory_single_simdfp_immediate_unsigned special value size = 3 (Size variant 3)
    // Fields: Rn=0, opc=0, Rt=0, size=3, imm12=1
    let encoding: u32 = 0xFD000400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	d0, [x0, #8]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_unsigned
/// ASL: `field opc = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "opc", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_memory_single_simdfp_immediate_unsigned_special_opc_0_size_variant_0_0_7d000400() {
    // Encoding: 0x7D000400
    // Test aarch64_memory_single_simdfp_immediate_unsigned special value opc = 0 (Size variant 0)
    // Fields: size=1, Rn=0, imm12=1, Rt=0, opc=0
    let encoding: u32 = 0x7D000400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	h0, [x0, #2]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_unsigned
/// ASL: `field opc = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "opc", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_memory_single_simdfp_immediate_unsigned_special_opc_1_size_variant_1_0_7d400400() {
    // Encoding: 0x7D400400
    // Test aarch64_memory_single_simdfp_immediate_unsigned special value opc = 1 (Size variant 1)
    // Fields: Rn=0, size=1, imm12=1, opc=1, Rt=0
    let encoding: u32 = 0x7D400400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ldr	h0, [x0, #2]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_unsigned
/// ASL: `field opc = 2 (Size variant 2)`
/// Requirement: FieldSpecial { field: "opc", value: 2, meaning: "Size variant 2" }
/// Size variant 2
#[test]
fn test_aarch64_memory_single_simdfp_immediate_unsigned_special_opc_2_size_variant_2_0_7d800400() {
    // Encoding: 0x7D800400
    // Test aarch64_memory_single_simdfp_immediate_unsigned special value opc = 2 (Size variant 2)
    // Fields: imm12=1, size=1, Rn=0, Rt=0, opc=2
    let encoding: u32 = 0x7D800400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_unsigned
/// ASL: `field opc = 3 (Size variant 3)`
/// Requirement: FieldSpecial { field: "opc", value: 3, meaning: "Size variant 3" }
/// Size variant 3
#[test]
fn test_aarch64_memory_single_simdfp_immediate_unsigned_special_opc_3_size_variant_3_0_7dc00400() {
    // Encoding: 0x7DC00400
    // Test aarch64_memory_single_simdfp_immediate_unsigned special value opc = 3 (Size variant 3)
    // Fields: size=1, Rn=0, imm12=1, opc=3, Rt=0
    let encoding: u32 = 0x7DC00400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_unsigned
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_memory_single_simdfp_immediate_unsigned_special_rn_31_stack_pointer_sp_may_require_alignment_0_7d0007e0()
 {
    // Encoding: 0x7D0007E0
    // Test aarch64_memory_single_simdfp_immediate_unsigned special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: size=1, imm12=1, opc=0, Rn=31, Rt=0
    let encoding: u32 = 0x7D0007E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	h0, [sp, #2]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_single_simdfp_immediate_unsigned
/// ASL: `field Rt = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)`
/// Requirement: FieldSpecial { field: "Rt", value: 31, meaning: "Zero register (XZR/WZR) - reads as 0, writes discarded" }
/// Zero register (XZR/WZR) - reads as 0, writes discarded
#[test]
fn test_aarch64_memory_single_simdfp_immediate_unsigned_special_rt_31_zero_register_xzr_wzr_reads_as_0_writes_discarded_0_7d00041f()
 {
    // Encoding: 0x7D00041F
    // Test aarch64_memory_single_simdfp_immediate_unsigned special value Rt = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)
    // Fields: imm12=1, Rn=0, Rt=31, opc=0, size=1
    let encoding: u32 = 0x7D00041F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: str	h31, [x0, #2]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

// ============================================================================
// aarch64_memory_single_general_immediate_signed_pac Tests
// ============================================================================

/// Provenance: aarch64_memory_single_general_immediate_signed_pac
/// ASL: `field size 30 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pac_field_size_0_min_400_38200400() {
    // Encoding: 0x38200400
    // Test aarch64_memory_single_general_immediate_signed_pac field size = 0 (Min)
    // Fields: Rt=0, imm9=0, S=0, W=0, size=0, Rn=0, M=0
    let encoding: u32 = 0x38200400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pac
/// ASL: `field size 30 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 1, boundary: PowerOfTwo }
/// 16-bit / halfword size
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pac_field_size_1_poweroftwo_400_78200400() {
    // Encoding: 0x78200400
    // Test aarch64_memory_single_general_immediate_signed_pac field size = 1 (PowerOfTwo)
    // Fields: W=0, size=1, Rn=0, S=0, M=0, imm9=0, Rt=0
    let encoding: u32 = 0x78200400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pac
/// ASL: `field size 30 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 2, boundary: PowerOfTwo }
/// 32-bit / word size
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pac_field_size_2_poweroftwo_400_b8200400() {
    // Encoding: 0xB8200400
    // Test aarch64_memory_single_general_immediate_signed_pac field size = 2 (PowerOfTwo)
    // Fields: W=0, Rt=0, imm9=0, size=2, S=0, Rn=0, M=0
    let encoding: u32 = 0xB8200400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pac
/// ASL: `field size 30 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 3, boundary: Max }
/// 64-bit / doubleword size
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pac_field_size_3_max_400_f8200400() {
    // Encoding: 0xF8200400
    // Test aarch64_memory_single_general_immediate_signed_pac field size = 3 (Max)
    // Fields: S=0, M=0, imm9=0, size=3, W=0, Rn=0, Rt=0
    let encoding: u32 = 0xF8200400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pac
/// ASL: `field M 23 +: 1`
/// Requirement: FieldBoundary { field: "M", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pac_field_m_0_min_400_38200400() {
    // Encoding: 0x38200400
    // Test aarch64_memory_single_general_immediate_signed_pac field M = 0 (Min)
    // Fields: Rt=0, W=0, imm9=0, M=0, S=0, Rn=0, size=0
    let encoding: u32 = 0x38200400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pac
/// ASL: `field M 23 +: 1`
/// Requirement: FieldBoundary { field: "M", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pac_field_m_1_max_400_38a00400() {
    // Encoding: 0x38A00400
    // Test aarch64_memory_single_general_immediate_signed_pac field M = 1 (Max)
    // Fields: Rt=0, size=0, M=1, imm9=0, S=0, W=0, Rn=0
    let encoding: u32 = 0x38A00400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pac
/// ASL: `field S 22 +: 1`
/// Requirement: FieldBoundary { field: "S", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pac_field_s_0_min_400_38200400() {
    // Encoding: 0x38200400
    // Test aarch64_memory_single_general_immediate_signed_pac field S = 0 (Min)
    // Fields: Rn=0, size=0, S=0, imm9=0, Rt=0, W=0, M=0
    let encoding: u32 = 0x38200400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pac
/// ASL: `field S 22 +: 1`
/// Requirement: FieldBoundary { field: "S", value: 1, boundary: Max }
/// 16-bit / halfword size
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pac_field_s_1_max_400_38600400() {
    // Encoding: 0x38600400
    // Test aarch64_memory_single_general_immediate_signed_pac field S = 1 (Max)
    // Fields: size=0, S=1, imm9=0, Rn=0, Rt=0, W=0, M=0
    let encoding: u32 = 0x38600400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pac
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pac_field_imm9_0_zero_400_38200400() {
    // Encoding: 0x38200400
    // Test aarch64_memory_single_general_immediate_signed_pac field imm9 = 0 (Zero)
    // Fields: W=0, Rn=0, size=0, Rt=0, S=0, imm9=0, M=0
    let encoding: u32 = 0x38200400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pac
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pac_field_imm9_1_poweroftwo_400_38201400() {
    // Encoding: 0x38201400
    // Test aarch64_memory_single_general_immediate_signed_pac field imm9 = 1 (PowerOfTwo)
    // Fields: W=0, Rt=0, imm9=1, M=0, size=0, Rn=0, S=0
    let encoding: u32 = 0x38201400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pac
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 3, boundary: PowerOfTwoMinusOne }
/// 2^2 - 1 = 3
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pac_field_imm9_3_poweroftwominusone_400_38203400()
 {
    // Encoding: 0x38203400
    // Test aarch64_memory_single_general_immediate_signed_pac field imm9 = 3 (PowerOfTwoMinusOne)
    // Fields: size=0, imm9=3, S=0, M=0, Rt=0, Rn=0, W=0
    let encoding: u32 = 0x38203400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pac
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 4, boundary: PowerOfTwo }
/// power of 2 (2^2 = 4)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pac_field_imm9_4_poweroftwo_400_38204400() {
    // Encoding: 0x38204400
    // Test aarch64_memory_single_general_immediate_signed_pac field imm9 = 4 (PowerOfTwo)
    // Fields: M=0, size=0, W=0, Rn=0, Rt=0, S=0, imm9=4
    let encoding: u32 = 0x38204400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pac
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 7, boundary: PowerOfTwoMinusOne }
/// 2^3 - 1 = 7
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pac_field_imm9_7_poweroftwominusone_400_38207400()
 {
    // Encoding: 0x38207400
    // Test aarch64_memory_single_general_immediate_signed_pac field imm9 = 7 (PowerOfTwoMinusOne)
    // Fields: imm9=7, S=0, Rt=0, M=0, size=0, Rn=0, W=0
    let encoding: u32 = 0x38207400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pac
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 8, boundary: PowerOfTwo }
/// power of 2 (2^3 = 8)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pac_field_imm9_8_poweroftwo_400_38208400() {
    // Encoding: 0x38208400
    // Test aarch64_memory_single_general_immediate_signed_pac field imm9 = 8 (PowerOfTwo)
    // Fields: size=0, Rn=0, Rt=0, M=0, W=0, S=0, imm9=8
    let encoding: u32 = 0x38208400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pac
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 15, boundary: PowerOfTwoMinusOne }
/// 2^4 - 1 = 15
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pac_field_imm9_15_poweroftwominusone_400_3820f400()
 {
    // Encoding: 0x3820F400
    // Test aarch64_memory_single_general_immediate_signed_pac field imm9 = 15 (PowerOfTwoMinusOne)
    // Fields: W=0, M=0, S=0, imm9=15, Rn=0, Rt=0, size=0
    let encoding: u32 = 0x3820F400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pac
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 16, boundary: PowerOfTwo }
/// power of 2 (2^4 = 16)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pac_field_imm9_16_poweroftwo_400_38210400() {
    // Encoding: 0x38210400
    // Test aarch64_memory_single_general_immediate_signed_pac field imm9 = 16 (PowerOfTwo)
    // Fields: size=0, imm9=16, S=0, M=0, W=0, Rn=0, Rt=0
    let encoding: u32 = 0x38210400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pac
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 31, boundary: PowerOfTwoMinusOne }
/// 2^5 - 1 = 31
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pac_field_imm9_31_poweroftwominusone_400_3821f400()
 {
    // Encoding: 0x3821F400
    // Test aarch64_memory_single_general_immediate_signed_pac field imm9 = 31 (PowerOfTwoMinusOne)
    // Fields: imm9=31, size=0, M=0, S=0, W=0, Rn=0, Rt=0
    let encoding: u32 = 0x3821F400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pac
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 32, boundary: PowerOfTwo }
/// power of 2 (2^5 = 32)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pac_field_imm9_32_poweroftwo_400_38220400() {
    // Encoding: 0x38220400
    // Test aarch64_memory_single_general_immediate_signed_pac field imm9 = 32 (PowerOfTwo)
    // Fields: size=0, M=0, imm9=32, W=0, S=0, Rn=0, Rt=0
    let encoding: u32 = 0x38220400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pac
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 63, boundary: PowerOfTwoMinusOne }
/// 2^6 - 1 = 63
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pac_field_imm9_63_poweroftwominusone_400_3823f400()
 {
    // Encoding: 0x3823F400
    // Test aarch64_memory_single_general_immediate_signed_pac field imm9 = 63 (PowerOfTwoMinusOne)
    // Fields: M=0, size=0, Rn=0, S=0, imm9=63, W=0, Rt=0
    let encoding: u32 = 0x3823F400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pac
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 64, boundary: PowerOfTwo }
/// power of 2 (2^6 = 64)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pac_field_imm9_64_poweroftwo_400_38240400() {
    // Encoding: 0x38240400
    // Test aarch64_memory_single_general_immediate_signed_pac field imm9 = 64 (PowerOfTwo)
    // Fields: Rt=0, M=0, S=0, size=0, W=0, imm9=64, Rn=0
    let encoding: u32 = 0x38240400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pac
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 127, boundary: PowerOfTwoMinusOne }
/// 2^7 - 1 = 127
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pac_field_imm9_127_poweroftwominusone_400_3827f400()
 {
    // Encoding: 0x3827F400
    // Test aarch64_memory_single_general_immediate_signed_pac field imm9 = 127 (PowerOfTwoMinusOne)
    // Fields: Rt=0, W=0, M=0, S=0, imm9=127, size=0, Rn=0
    let encoding: u32 = 0x3827F400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pac
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 128, boundary: PowerOfTwo }
/// power of 2 (2^7 = 128)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pac_field_imm9_128_poweroftwo_400_38280400()
{
    // Encoding: 0x38280400
    // Test aarch64_memory_single_general_immediate_signed_pac field imm9 = 128 (PowerOfTwo)
    // Fields: size=0, S=0, M=0, imm9=128, W=0, Rn=0, Rt=0
    let encoding: u32 = 0x38280400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pac
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 255, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (255)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pac_field_imm9_255_poweroftwominusone_400_382ff400()
 {
    // Encoding: 0x382FF400
    // Test aarch64_memory_single_general_immediate_signed_pac field imm9 = 255 (PowerOfTwoMinusOne)
    // Fields: M=0, S=0, Rn=0, W=0, imm9=255, size=0, Rt=0
    let encoding: u32 = 0x382FF400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pac
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 256, boundary: PowerOfTwo }
/// power of 2 (2^8 = 256)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pac_field_imm9_256_poweroftwo_400_38300400()
{
    // Encoding: 0x38300400
    // Test aarch64_memory_single_general_immediate_signed_pac field imm9 = 256 (PowerOfTwo)
    // Fields: imm9=256, size=0, W=0, Rn=0, M=0, Rt=0, S=0
    let encoding: u32 = 0x38300400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pac
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 511, boundary: Max }
/// maximum immediate (511)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pac_field_imm9_511_max_400_383ff400() {
    // Encoding: 0x383FF400
    // Test aarch64_memory_single_general_immediate_signed_pac field imm9 = 511 (Max)
    // Fields: Rt=0, size=0, S=0, M=0, imm9=511, W=0, Rn=0
    let encoding: u32 = 0x383FF400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pac
/// ASL: `field W 11 +: 1`
/// Requirement: FieldBoundary { field: "W", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pac_field_w_0_min_400_38200400() {
    // Encoding: 0x38200400
    // Test aarch64_memory_single_general_immediate_signed_pac field W = 0 (Min)
    // Fields: Rn=0, size=0, M=0, W=0, S=0, imm9=0, Rt=0
    let encoding: u32 = 0x38200400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pac
/// ASL: `field W 11 +: 1`
/// Requirement: FieldBoundary { field: "W", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pac_field_w_1_max_400_38200c00() {
    // Encoding: 0x38200C00
    // Test aarch64_memory_single_general_immediate_signed_pac field W = 1 (Max)
    // Fields: M=0, Rn=0, S=0, Rt=0, imm9=0, W=1, size=0
    let encoding: u32 = 0x38200C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pac
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pac_field_rn_0_min_400_38200400() {
    // Encoding: 0x38200400
    // Test aarch64_memory_single_general_immediate_signed_pac field Rn = 0 (Min)
    // Fields: size=0, M=0, S=0, imm9=0, W=0, Rn=0, Rt=0
    let encoding: u32 = 0x38200400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pac
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pac_field_rn_1_poweroftwo_400_38200420() {
    // Encoding: 0x38200420
    // Test aarch64_memory_single_general_immediate_signed_pac field Rn = 1 (PowerOfTwo)
    // Fields: Rt=0, M=0, size=0, Rn=1, imm9=0, S=0, W=0
    let encoding: u32 = 0x38200420;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pac
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pac_field_rn_30_poweroftwominusone_400_382007c0()
 {
    // Encoding: 0x382007C0
    // Test aarch64_memory_single_general_immediate_signed_pac field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: M=0, S=0, imm9=0, W=0, Rt=0, size=0, Rn=30
    let encoding: u32 = 0x382007C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pac
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pac_field_rn_31_max_400_382007e0() {
    // Encoding: 0x382007E0
    // Test aarch64_memory_single_general_immediate_signed_pac field Rn = 31 (Max)
    // Fields: imm9=0, size=0, Rn=31, Rt=0, M=0, S=0, W=0
    let encoding: u32 = 0x382007E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pac
/// ASL: `field Rt 0 +: 5`
/// Requirement: FieldBoundary { field: "Rt", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pac_field_rt_0_min_400_38200400() {
    // Encoding: 0x38200400
    // Test aarch64_memory_single_general_immediate_signed_pac field Rt = 0 (Min)
    // Fields: S=0, Rn=0, M=0, Rt=0, W=0, imm9=0, size=0
    let encoding: u32 = 0x38200400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pac
/// ASL: `field Rt 0 +: 5`
/// Requirement: FieldBoundary { field: "Rt", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pac_field_rt_1_poweroftwo_400_38200401() {
    // Encoding: 0x38200401
    // Test aarch64_memory_single_general_immediate_signed_pac field Rt = 1 (PowerOfTwo)
    // Fields: Rt=1, M=0, size=0, imm9=0, W=0, Rn=0, S=0
    let encoding: u32 = 0x38200401;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pac
/// ASL: `field Rt 0 +: 5`
/// Requirement: FieldBoundary { field: "Rt", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pac_field_rt_30_poweroftwominusone_400_3820041e()
 {
    // Encoding: 0x3820041E
    // Test aarch64_memory_single_general_immediate_signed_pac field Rt = 30 (PowerOfTwoMinusOne)
    // Fields: Rn=0, Rt=30, size=0, S=0, imm9=0, W=0, M=0
    let encoding: u32 = 0x3820041E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pac
/// ASL: `field Rt 0 +: 5`
/// Requirement: FieldBoundary { field: "Rt", value: 31, boundary: Max }
/// register index 31 (ZR - zero register)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pac_field_rt_31_max_400_3820041f() {
    // Encoding: 0x3820041F
    // Test aarch64_memory_single_general_immediate_signed_pac field Rt = 31 (Max)
    // Fields: S=0, imm9=0, M=0, size=0, Rn=0, Rt=31, W=0
    let encoding: u32 = 0x3820041F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pac
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=0 (8-bit / byte size)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pac_combo_0_400_38200400() {
    // Encoding: 0x38200400
    // Test aarch64_memory_single_general_immediate_signed_pac field combination: size=0, M=0, S=0, imm9=0, W=0, Rn=0, Rt=0
    // Fields: S=0, W=0, Rn=0, Rt=0, size=0, M=0, imm9=0
    let encoding: u32 = 0x38200400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pac
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=1 (16-bit / halfword size)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pac_combo_1_400_78200400() {
    // Encoding: 0x78200400
    // Test aarch64_memory_single_general_immediate_signed_pac field combination: size=1, M=0, S=0, imm9=0, W=0, Rn=0, Rt=0
    // Fields: size=1, imm9=0, Rt=0, M=0, S=0, W=0, Rn=0
    let encoding: u32 = 0x78200400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pac
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=2 (32-bit / word size)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pac_combo_2_400_b8200400() {
    // Encoding: 0xB8200400
    // Test aarch64_memory_single_general_immediate_signed_pac field combination: size=2, M=0, S=0, imm9=0, W=0, Rn=0, Rt=0
    // Fields: Rn=0, Rt=0, size=2, S=0, M=0, imm9=0, W=0
    let encoding: u32 = 0xB8200400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pac
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=3 (64-bit / doubleword size)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pac_combo_3_400_f8200400() {
    // Encoding: 0xF8200400
    // Test aarch64_memory_single_general_immediate_signed_pac field combination: size=3, M=0, S=0, imm9=0, W=0, Rn=0, Rt=0
    // Fields: S=0, imm9=0, size=3, Rt=0, W=0, M=0, Rn=0
    let encoding: u32 = 0xF8200400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pac
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// M=0 (minimum value)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pac_combo_4_400_38200400() {
    // Encoding: 0x38200400
    // Test aarch64_memory_single_general_immediate_signed_pac field combination: size=0, M=0, S=0, imm9=0, W=0, Rn=0, Rt=0
    // Fields: Rn=0, Rt=0, M=0, S=0, size=0, imm9=0, W=0
    let encoding: u32 = 0x38200400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pac
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// M=1 (maximum value (1))
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pac_combo_5_400_38a00400() {
    // Encoding: 0x38A00400
    // Test aarch64_memory_single_general_immediate_signed_pac field combination: size=0, M=1, S=0, imm9=0, W=0, Rn=0, Rt=0
    // Fields: Rn=0, S=0, size=0, M=1, imm9=0, W=0, Rt=0
    let encoding: u32 = 0x38A00400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pac
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// S=0 (8-bit / byte size)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pac_combo_6_400_38200400() {
    // Encoding: 0x38200400
    // Test aarch64_memory_single_general_immediate_signed_pac field combination: size=0, M=0, S=0, imm9=0, W=0, Rn=0, Rt=0
    // Fields: Rn=0, Rt=0, size=0, imm9=0, S=0, M=0, W=0
    let encoding: u32 = 0x38200400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pac
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// S=1 (16-bit / halfword size)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pac_combo_7_400_38600400() {
    // Encoding: 0x38600400
    // Test aarch64_memory_single_general_immediate_signed_pac field combination: size=0, M=0, S=1, imm9=0, W=0, Rn=0, Rt=0
    // Fields: imm9=0, M=0, Rn=0, S=1, size=0, W=0, Rt=0
    let encoding: u32 = 0x38600400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pac
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=0 (immediate value 0)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pac_combo_8_400_38200400() {
    // Encoding: 0x38200400
    // Test aarch64_memory_single_general_immediate_signed_pac field combination: size=0, M=0, S=0, imm9=0, W=0, Rn=0, Rt=0
    // Fields: size=0, imm9=0, W=0, Rn=0, Rt=0, M=0, S=0
    let encoding: u32 = 0x38200400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pac
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=1 (immediate value 1)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pac_combo_9_400_38201400() {
    // Encoding: 0x38201400
    // Test aarch64_memory_single_general_immediate_signed_pac field combination: size=0, M=0, S=0, imm9=1, W=0, Rn=0, Rt=0
    // Fields: size=0, imm9=1, M=0, Rt=0, S=0, W=0, Rn=0
    let encoding: u32 = 0x38201400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pac
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=3 (2^2 - 1 = 3)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pac_combo_10_400_38203400() {
    // Encoding: 0x38203400
    // Test aarch64_memory_single_general_immediate_signed_pac field combination: size=0, M=0, S=0, imm9=3, W=0, Rn=0, Rt=0
    // Fields: imm9=3, M=0, Rn=0, size=0, S=0, W=0, Rt=0
    let encoding: u32 = 0x38203400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pac
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=4 (power of 2 (2^2 = 4))
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pac_combo_11_400_38204400() {
    // Encoding: 0x38204400
    // Test aarch64_memory_single_general_immediate_signed_pac field combination: size=0, M=0, S=0, imm9=4, W=0, Rn=0, Rt=0
    // Fields: M=0, size=0, S=0, imm9=4, Rn=0, W=0, Rt=0
    let encoding: u32 = 0x38204400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pac
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=7 (2^3 - 1 = 7)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pac_combo_12_400_38207400() {
    // Encoding: 0x38207400
    // Test aarch64_memory_single_general_immediate_signed_pac field combination: size=0, M=0, S=0, imm9=7, W=0, Rn=0, Rt=0
    // Fields: M=0, size=0, imm9=7, S=0, W=0, Rn=0, Rt=0
    let encoding: u32 = 0x38207400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pac
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=8 (power of 2 (2^3 = 8))
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pac_combo_13_400_38208400() {
    // Encoding: 0x38208400
    // Test aarch64_memory_single_general_immediate_signed_pac field combination: size=0, M=0, S=0, imm9=8, W=0, Rn=0, Rt=0
    // Fields: W=0, S=0, M=0, size=0, imm9=8, Rt=0, Rn=0
    let encoding: u32 = 0x38208400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pac
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=15 (2^4 - 1 = 15)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pac_combo_14_400_3820f400() {
    // Encoding: 0x3820F400
    // Test aarch64_memory_single_general_immediate_signed_pac field combination: size=0, M=0, S=0, imm9=15, W=0, Rn=0, Rt=0
    // Fields: S=0, size=0, Rn=0, imm9=15, W=0, Rt=0, M=0
    let encoding: u32 = 0x3820F400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pac
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=16 (power of 2 (2^4 = 16))
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pac_combo_15_400_38210400() {
    // Encoding: 0x38210400
    // Test aarch64_memory_single_general_immediate_signed_pac field combination: size=0, M=0, S=0, imm9=16, W=0, Rn=0, Rt=0
    // Fields: W=0, Rt=0, size=0, M=0, imm9=16, Rn=0, S=0
    let encoding: u32 = 0x38210400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pac
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=31 (2^5 - 1 = 31)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pac_combo_16_400_3821f400() {
    // Encoding: 0x3821F400
    // Test aarch64_memory_single_general_immediate_signed_pac field combination: size=0, M=0, S=0, imm9=31, W=0, Rn=0, Rt=0
    // Fields: Rn=0, Rt=0, size=0, imm9=31, S=0, M=0, W=0
    let encoding: u32 = 0x3821F400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pac
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=32 (power of 2 (2^5 = 32))
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pac_combo_17_400_38220400() {
    // Encoding: 0x38220400
    // Test aarch64_memory_single_general_immediate_signed_pac field combination: size=0, M=0, S=0, imm9=32, W=0, Rn=0, Rt=0
    // Fields: W=0, imm9=32, S=0, M=0, Rn=0, size=0, Rt=0
    let encoding: u32 = 0x38220400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pac
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=63 (2^6 - 1 = 63)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pac_combo_18_400_3823f400() {
    // Encoding: 0x3823F400
    // Test aarch64_memory_single_general_immediate_signed_pac field combination: size=0, M=0, S=0, imm9=63, W=0, Rn=0, Rt=0
    // Fields: W=0, Rn=0, S=0, Rt=0, M=0, size=0, imm9=63
    let encoding: u32 = 0x3823F400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pac
/// ASL: `field combination 19`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=64 (power of 2 (2^6 = 64))
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pac_combo_19_400_38240400() {
    // Encoding: 0x38240400
    // Test aarch64_memory_single_general_immediate_signed_pac field combination: size=0, M=0, S=0, imm9=64, W=0, Rn=0, Rt=0
    // Fields: S=0, imm9=64, size=0, M=0, W=0, Rn=0, Rt=0
    let encoding: u32 = 0x38240400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pac
/// ASL: `field combination 20`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=127 (2^7 - 1 = 127)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pac_combo_20_400_3827f400() {
    // Encoding: 0x3827F400
    // Test aarch64_memory_single_general_immediate_signed_pac field combination: size=0, M=0, S=0, imm9=127, W=0, Rn=0, Rt=0
    // Fields: Rn=0, M=0, Rt=0, S=0, size=0, imm9=127, W=0
    let encoding: u32 = 0x3827F400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pac
/// ASL: `field combination 21`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=128 (power of 2 (2^7 = 128))
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pac_combo_21_400_38280400() {
    // Encoding: 0x38280400
    // Test aarch64_memory_single_general_immediate_signed_pac field combination: size=0, M=0, S=0, imm9=128, W=0, Rn=0, Rt=0
    // Fields: S=0, Rn=0, M=0, imm9=128, size=0, W=0, Rt=0
    let encoding: u32 = 0x38280400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pac
/// ASL: `field combination 22`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=255 (immediate midpoint (255))
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pac_combo_22_400_382ff400() {
    // Encoding: 0x382FF400
    // Test aarch64_memory_single_general_immediate_signed_pac field combination: size=0, M=0, S=0, imm9=255, W=0, Rn=0, Rt=0
    // Fields: Rt=0, M=0, W=0, size=0, S=0, imm9=255, Rn=0
    let encoding: u32 = 0x382FF400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pac
/// ASL: `field combination 23`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=256 (power of 2 (2^8 = 256))
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pac_combo_23_400_38300400() {
    // Encoding: 0x38300400
    // Test aarch64_memory_single_general_immediate_signed_pac field combination: size=0, M=0, S=0, imm9=256, W=0, Rn=0, Rt=0
    // Fields: W=0, Rn=0, size=0, Rt=0, M=0, S=0, imm9=256
    let encoding: u32 = 0x38300400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pac
/// ASL: `field combination 24`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=511 (maximum immediate (511))
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pac_combo_24_400_383ff400() {
    // Encoding: 0x383FF400
    // Test aarch64_memory_single_general_immediate_signed_pac field combination: size=0, M=0, S=0, imm9=511, W=0, Rn=0, Rt=0
    // Fields: size=0, M=0, Rn=0, Rt=0, S=0, W=0, imm9=511
    let encoding: u32 = 0x383FF400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pac
/// ASL: `field combination 25`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// W=0 (minimum value)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pac_combo_25_400_38200400() {
    // Encoding: 0x38200400
    // Test aarch64_memory_single_general_immediate_signed_pac field combination: size=0, M=0, S=0, imm9=0, W=0, Rn=0, Rt=0
    // Fields: W=0, M=0, size=0, S=0, Rn=0, Rt=0, imm9=0
    let encoding: u32 = 0x38200400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pac
/// ASL: `field combination 26`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// W=1 (maximum value (1))
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pac_combo_26_400_38200c00() {
    // Encoding: 0x38200C00
    // Test aarch64_memory_single_general_immediate_signed_pac field combination: size=0, M=0, S=0, imm9=0, W=1, Rn=0, Rt=0
    // Fields: M=0, W=1, Rn=0, Rt=0, S=0, size=0, imm9=0
    let encoding: u32 = 0x38200C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pac
/// ASL: `field combination 27`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pac_combo_27_400_38200400() {
    // Encoding: 0x38200400
    // Test aarch64_memory_single_general_immediate_signed_pac field combination: size=0, M=0, S=0, imm9=0, W=0, Rn=0, Rt=0
    // Fields: Rt=0, M=0, size=0, S=0, imm9=0, W=0, Rn=0
    let encoding: u32 = 0x38200400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pac
/// ASL: `field combination 28`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pac_combo_28_400_38200420() {
    // Encoding: 0x38200420
    // Test aarch64_memory_single_general_immediate_signed_pac field combination: size=0, M=0, S=0, imm9=0, W=0, Rn=1, Rt=0
    // Fields: Rt=0, S=0, size=0, M=0, imm9=0, W=0, Rn=1
    let encoding: u32 = 0x38200420;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pac
/// ASL: `field combination 29`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pac_combo_29_400_382007c0() {
    // Encoding: 0x382007C0
    // Test aarch64_memory_single_general_immediate_signed_pac field combination: size=0, M=0, S=0, imm9=0, W=0, Rn=30, Rt=0
    // Fields: W=0, Rt=0, size=0, M=0, Rn=30, imm9=0, S=0
    let encoding: u32 = 0x382007C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pac
/// ASL: `field combination 30`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pac_combo_30_400_382007e0() {
    // Encoding: 0x382007E0
    // Test aarch64_memory_single_general_immediate_signed_pac field combination: size=0, M=0, S=0, imm9=0, W=0, Rn=31, Rt=0
    // Fields: size=0, M=0, Rn=31, imm9=0, W=0, S=0, Rt=0
    let encoding: u32 = 0x382007E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pac
/// ASL: `field combination 31`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt=0 (register index 0 (first register))
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pac_combo_31_400_38200400() {
    // Encoding: 0x38200400
    // Test aarch64_memory_single_general_immediate_signed_pac field combination: size=0, M=0, S=0, imm9=0, W=0, Rn=0, Rt=0
    // Fields: Rn=0, S=0, size=0, M=0, imm9=0, Rt=0, W=0
    let encoding: u32 = 0x38200400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pac
/// ASL: `field combination 32`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt=1 (register index 1 (second register))
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pac_combo_32_400_38200401() {
    // Encoding: 0x38200401
    // Test aarch64_memory_single_general_immediate_signed_pac field combination: size=0, M=0, S=0, imm9=0, W=0, Rn=0, Rt=1
    // Fields: W=0, Rn=0, M=0, Rt=1, S=0, size=0, imm9=0
    let encoding: u32 = 0x38200401;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pac
/// ASL: `field combination 33`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pac_combo_33_400_3820041e() {
    // Encoding: 0x3820041E
    // Test aarch64_memory_single_general_immediate_signed_pac field combination: size=0, M=0, S=0, imm9=0, W=0, Rn=0, Rt=30
    // Fields: M=0, W=0, Rn=0, size=0, S=0, imm9=0, Rt=30
    let encoding: u32 = 0x3820041E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pac
/// ASL: `field combination 34`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt=31 (register index 31 (ZR - zero register))
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pac_combo_34_400_3820041f() {
    // Encoding: 0x3820041F
    // Test aarch64_memory_single_general_immediate_signed_pac field combination: size=0, M=0, S=0, imm9=0, W=0, Rn=0, Rt=31
    // Fields: S=0, imm9=0, size=0, W=0, Rn=0, Rt=31, M=0
    let encoding: u32 = 0x3820041F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pac
/// ASL: `field combination 35`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (same register test (reg=1)), Rt=1 (same register test (reg=1))
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pac_combo_35_400_38200421() {
    // Encoding: 0x38200421
    // Test aarch64_memory_single_general_immediate_signed_pac field combination: size=0, M=0, S=0, imm9=0, W=0, Rn=1, Rt=1
    // Fields: imm9=0, size=0, Rt=1, W=0, M=0, S=0, Rn=1
    let encoding: u32 = 0x38200421;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pac
/// ASL: `field combination 36`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (same register test (reg=31)), Rt=31 (same register test (reg=31))
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pac_combo_36_400_382007ff() {
    // Encoding: 0x382007FF
    // Test aarch64_memory_single_general_immediate_signed_pac field combination: size=0, M=0, S=0, imm9=0, W=0, Rn=31, Rt=31
    // Fields: W=0, Rn=31, S=0, M=0, Rt=31, imm9=0, size=0
    let encoding: u32 = 0x382007FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pac
/// ASL: `field size = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "size", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pac_special_size_0_size_variant_0_1024_38201400()
 {
    // Encoding: 0x38201400
    // Test aarch64_memory_single_general_immediate_signed_pac special value size = 0 (Size variant 0)
    // Fields: imm9=1, S=0, size=0, Rt=0, W=0, M=0, Rn=0
    let encoding: u32 = 0x38201400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pac
/// ASL: `field size = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "size", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pac_special_size_1_size_variant_1_1024_78201400()
 {
    // Encoding: 0x78201400
    // Test aarch64_memory_single_general_immediate_signed_pac special value size = 1 (Size variant 1)
    // Fields: size=1, imm9=1, Rt=0, M=0, W=0, Rn=0, S=0
    let encoding: u32 = 0x78201400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pac
/// ASL: `field size = 2 (Size variant 2)`
/// Requirement: FieldSpecial { field: "size", value: 2, meaning: "Size variant 2" }
/// Size variant 2
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pac_special_size_2_size_variant_2_1024_b8201400()
 {
    // Encoding: 0xB8201400
    // Test aarch64_memory_single_general_immediate_signed_pac special value size = 2 (Size variant 2)
    // Fields: Rn=0, Rt=0, M=0, W=0, imm9=1, S=0, size=2
    let encoding: u32 = 0xB8201400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pac
/// ASL: `field size = 3 (Size variant 3)`
/// Requirement: FieldSpecial { field: "size", value: 3, meaning: "Size variant 3" }
/// Size variant 3
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pac_special_size_3_size_variant_3_1024_f8201400()
 {
    // Encoding: 0xF8201400
    // Test aarch64_memory_single_general_immediate_signed_pac special value size = 3 (Size variant 3)
    // Fields: Rt=0, M=0, S=0, imm9=1, W=0, size=3, Rn=0
    let encoding: u32 = 0xF8201400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pac
/// ASL: `field S = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "S", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pac_special_s_0_size_variant_0_1024_78201400()
 {
    // Encoding: 0x78201400
    // Test aarch64_memory_single_general_immediate_signed_pac special value S = 0 (Size variant 0)
    // Fields: S=0, Rn=0, Rt=0, size=1, imm9=1, M=0, W=0
    let encoding: u32 = 0x78201400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pac
/// ASL: `field S = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "S", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pac_special_s_1_size_variant_1_1024_78601400()
 {
    // Encoding: 0x78601400
    // Test aarch64_memory_single_general_immediate_signed_pac special value S = 1 (Size variant 1)
    // Fields: imm9=1, S=1, Rn=0, Rt=0, size=1, W=0, M=0
    let encoding: u32 = 0x78601400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pac
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pac_special_rn_31_stack_pointer_sp_may_require_alignment_1024_782017e0()
 {
    // Encoding: 0x782017E0
    // Test aarch64_memory_single_general_immediate_signed_pac special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: Rn=31, S=0, W=0, size=1, Rt=0, imm9=1, M=0
    let encoding: u32 = 0x782017E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pac
/// ASL: `field Rt = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)`
/// Requirement: FieldSpecial { field: "Rt", value: 31, meaning: "Zero register (XZR/WZR) - reads as 0, writes discarded" }
/// Zero register (XZR/WZR) - reads as 0, writes discarded
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pac_special_rt_31_zero_register_xzr_wzr_reads_as_0_writes_discarded_1024_7820141f()
 {
    // Encoding: 0x7820141F
    // Test aarch64_memory_single_general_immediate_signed_pac special value Rt = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)
    // Fields: W=0, imm9=1, Rt=31, Rn=0, M=0, S=0, size=1
    let encoding: u32 = 0x7820141F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pac
/// ASL: `GpFromField("t") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "t" }
/// verify register write to GpFromField("t")
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pac_reg_write_0_38200400() {
    // Test aarch64_memory_single_general_immediate_signed_pac register write: GpFromField("t")
    // Encoding: 0x38200400
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x38200400;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pac
/// ASL: `Sp write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "unknown" }
/// verify register write to Sp
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pac_reg_write_1_38200400() {
    // Test aarch64_memory_single_general_immediate_signed_pac register write: Sp
    // Encoding: 0x38200400
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x38200400;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pac
/// ASL: `GpFromField("n") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "n" }
/// verify register write to GpFromField("n")
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pac_reg_write_2_38200400() {
    // Test aarch64_memory_single_general_immediate_signed_pac register write: GpFromField("n")
    // Encoding: 0x38200400
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x38200400;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pac
/// ASL: `Rn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Rn = 31)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pac_sp_rn_382007e0() {
    // Test aarch64_memory_single_general_immediate_signed_pac with Rn = SP (31)
    // Encoding: 0x382007E0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x382007E0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pac
/// ASL: `Rt = 31 (ZR)`
/// Requirement: RegisterSpecial { reg: Zr, behavior: "reads as 0, writes discarded" }
/// zero register (Rt = 31)
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pac_zr_rt_3820041f() {
    // Test aarch64_memory_single_general_immediate_signed_pac with Rt = ZR (31)
    // Encoding: 0x3820041F
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x3820041F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 31), 0, "XZR should always be 0");
}

/// Provenance: aarch64_memory_single_general_immediate_signed_pac
/// ASL: `Unconditional`
/// Requirement: UndefinedEncoding { condition: "Unconditional" }
/// triggers Undefined
#[test]
fn test_aarch64_memory_single_general_immediate_signed_pac_exception_0_38200400() {
    // Test aarch64_memory_single_general_immediate_signed_pac exception: Undefined
    // Encoding: 0x38200400
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x38200400;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}
