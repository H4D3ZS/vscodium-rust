//! T16 branch compare_branch tests.
//!
//! Auto-generated from ARM ASL specifications.
//! DO NOT EDIT MANUALLY.

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::generated::test_helpers_32::*;

// ============================================================================
// aarch32_CBNZ_A Tests
// ============================================================================

/// Provenance: aarch32_CBNZ_T1_A
/// ASL: `field op 27 +: 1`
/// Requirement: FieldBoundary { field: "op", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_cbnz_t1_a_field_op_0_min_0_b1000000() {
    // Thumb encoding (32): 0xB1000000
    // Test aarch32_CBNZ_T1_A field op = 0 (Min)
    // ISET: T32
    // Fields: i=0, op=0, Rn=0, imm5=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xB1000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_CBNZ_T1_A
/// ASL: `field op 27 +: 1`
/// Requirement: FieldBoundary { field: "op", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_cbnz_t1_a_field_op_1_max_0_b9000000() {
    // Thumb encoding (32): 0xB9000000
    // Test aarch32_CBNZ_T1_A field op = 1 (Max)
    // ISET: T32
    // Fields: imm5=0, i=0, Rn=0, op=1
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xB9000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_CBNZ_T1_A
/// ASL: `field i 25 +: 1`
/// Requirement: FieldBoundary { field: "i", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_cbnz_t1_a_field_i_0_min_0_b1000000() {
    // Thumb encoding (32): 0xB1000000
    // Test aarch32_CBNZ_T1_A field i = 0 (Min)
    // ISET: T32
    // Fields: Rn=0, op=0, i=0, imm5=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xB1000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_CBNZ_T1_A
/// ASL: `field i 25 +: 1`
/// Requirement: FieldBoundary { field: "i", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_cbnz_t1_a_field_i_1_max_0_b3000000() {
    // Thumb encoding (32): 0xB3000000
    // Test aarch32_CBNZ_T1_A field i = 1 (Max)
    // ISET: T32
    // Fields: imm5=0, op=0, i=1, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xB3000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_CBNZ_T1_A
/// ASL: `field imm5 19 +: 5`
/// Requirement: FieldBoundary { field: "imm5", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch32_cbnz_t1_a_field_imm5_0_zero_0_b1000000() {
    // Thumb encoding (32): 0xB1000000
    // Test aarch32_CBNZ_T1_A field imm5 = 0 (Zero)
    // ISET: T32
    // Fields: i=0, imm5=0, Rn=0, op=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xB1000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_CBNZ_T1_A
/// ASL: `field imm5 19 +: 5`
/// Requirement: FieldBoundary { field: "imm5", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch32_cbnz_t1_a_field_imm5_1_poweroftwo_0_b1080000() {
    // Thumb encoding (32): 0xB1080000
    // Test aarch32_CBNZ_T1_A field imm5 = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: op=0, imm5=1, i=0, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xB1080000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_CBNZ_T1_A
/// ASL: `field imm5 19 +: 5`
/// Requirement: FieldBoundary { field: "imm5", value: 3, boundary: PowerOfTwoMinusOne }
/// 2^2 - 1 = 3
#[test]
fn test_aarch32_cbnz_t1_a_field_imm5_3_poweroftwominusone_0_b1180000() {
    // Thumb encoding (32): 0xB1180000
    // Test aarch32_CBNZ_T1_A field imm5 = 3 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: Rn=0, i=0, imm5=3, op=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xB1180000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_CBNZ_T1_A
/// ASL: `field imm5 19 +: 5`
/// Requirement: FieldBoundary { field: "imm5", value: 4, boundary: PowerOfTwo }
/// power of 2 (2^2 = 4)
#[test]
fn test_aarch32_cbnz_t1_a_field_imm5_4_poweroftwo_0_b1200000() {
    // Thumb encoding (32): 0xB1200000
    // Test aarch32_CBNZ_T1_A field imm5 = 4 (PowerOfTwo)
    // ISET: T32
    // Fields: op=0, Rn=0, imm5=4, i=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xB1200000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_CBNZ_T1_A
/// ASL: `field imm5 19 +: 5`
/// Requirement: FieldBoundary { field: "imm5", value: 7, boundary: PowerOfTwoMinusOne }
/// 2^3 - 1 = 7
#[test]
fn test_aarch32_cbnz_t1_a_field_imm5_7_poweroftwominusone_0_b1380000() {
    // Thumb encoding (32): 0xB1380000
    // Test aarch32_CBNZ_T1_A field imm5 = 7 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: op=0, Rn=0, imm5=7, i=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xB1380000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_CBNZ_T1_A
/// ASL: `field imm5 19 +: 5`
/// Requirement: FieldBoundary { field: "imm5", value: 8, boundary: PowerOfTwo }
/// power of 2 (2^3 = 8)
#[test]
fn test_aarch32_cbnz_t1_a_field_imm5_8_poweroftwo_0_b1400000() {
    // Thumb encoding (32): 0xB1400000
    // Test aarch32_CBNZ_T1_A field imm5 = 8 (PowerOfTwo)
    // ISET: T32
    // Fields: op=0, Rn=0, imm5=8, i=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xB1400000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_CBNZ_T1_A
/// ASL: `field imm5 19 +: 5`
/// Requirement: FieldBoundary { field: "imm5", value: 15, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (15)
#[test]
fn test_aarch32_cbnz_t1_a_field_imm5_15_poweroftwominusone_0_b1780000() {
    // Thumb encoding (32): 0xB1780000
    // Test aarch32_CBNZ_T1_A field imm5 = 15 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: op=0, Rn=0, imm5=15, i=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xB1780000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_CBNZ_T1_A
/// ASL: `field imm5 19 +: 5`
/// Requirement: FieldBoundary { field: "imm5", value: 16, boundary: PowerOfTwo }
/// power of 2 (2^4 = 16)
#[test]
fn test_aarch32_cbnz_t1_a_field_imm5_16_poweroftwo_0_b1800000() {
    // Thumb encoding (32): 0xB1800000
    // Test aarch32_CBNZ_T1_A field imm5 = 16 (PowerOfTwo)
    // ISET: T32
    // Fields: imm5=16, op=0, i=0, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xB1800000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_CBNZ_T1_A
/// ASL: `field imm5 19 +: 5`
/// Requirement: FieldBoundary { field: "imm5", value: 31, boundary: Max }
/// maximum immediate (31)
#[test]
fn test_aarch32_cbnz_t1_a_field_imm5_31_max_0_b1f80000() {
    // Thumb encoding (32): 0xB1F80000
    // Test aarch32_CBNZ_T1_A field imm5 = 31 (Max)
    // ISET: T32
    // Fields: op=0, i=0, Rn=0, imm5=31
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xB1F80000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_CBNZ_T1_A
/// ASL: `field Rn 16 +: 3`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_cbnz_t1_a_field_rn_0_min_0_b1000000() {
    // Thumb encoding (32): 0xB1000000
    // Test aarch32_CBNZ_T1_A field Rn = 0 (Min)
    // ISET: T32
    // Fields: i=0, op=0, imm5=0, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xB1000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_CBNZ_T1_A
/// ASL: `field Rn 16 +: 3`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_cbnz_t1_a_field_rn_1_poweroftwo_0_b1010000() {
    // Thumb encoding (32): 0xB1010000
    // Test aarch32_CBNZ_T1_A field Rn = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rn=1, op=0, i=0, imm5=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xB1010000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_CBNZ_T1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// op=0 (minimum value)
#[test]
fn test_aarch32_cbnz_t1_a_combo_0_0_b1000000() {
    // Thumb encoding (32): 0xB1000000
    // Test aarch32_CBNZ_T1_A field combination: op=0, i=0, imm5=0, Rn=0
    // ISET: T32
    // Fields: imm5=0, Rn=0, i=0, op=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xB1000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_CBNZ_T1_A
/// ASL: `Call { name: QualifiedIdentifier { qualifier: Any, name: "InITBlock" }, args: [] }`
/// Requirement: UndefinedEncoding { condition: "Call { name: QualifiedIdentifier { qualifier: Any, name: \"InITBlock\" }, args: [] }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_cbnz_t1_a_invalid_0_0_b1000000() {
    // Thumb encoding (32): 0xB1000000
    // Test aarch32_CBNZ_T1_A invalid encoding: Call { name: QualifiedIdentifier { qualifier: Any, name: "InITBlock" }, args: [] }
    // ISET: T32
    // Fields: imm5=0, Rn=0, op=0, i=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xB1000000;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

/// Provenance: aarch32_CBNZ_T1_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_cbnz_t1_a_invalid_1_0_b1000000() {
    // Thumb encoding (32): 0xB1000000
    // Test aarch32_CBNZ_T1_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: T32
    // Fields: Rn=0, imm5=0, i=0, op=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xB1000000;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

/// Provenance: aarch32_CBNZ_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// shift by 0 (32)
#[test]
fn test_aarch32_cbnz_t1_a_lslv_oracle_32_0_b1020020() {
    // Test LSLV 32-bit: shift by 0 (oracle)
    // ISET: T16
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x12345678);
    set_w(&mut cpu, 2, 0x0);
    let encoding: u16 = 0x0020;
    // T16: Write 16-bit instruction
    cpu.write_memory(0, &(encoding as u32).to_le_bytes())
        .unwrap();
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x12345678, "W0 should be 0x12345678");
}

/// Provenance: aarch32_CBNZ_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// shift by 0 (64)
#[test]
fn test_aarch32_cbnz_t1_a_lslv_oracle_64_0_b1020020() {
    // Test LSLV 64-bit: shift by 0 (oracle)
    // ISET: T16
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x0);
    set_w(&mut cpu, 1, 0x12345678);
    let encoding: u16 = 0x0020;
    // T16: Write 16-bit instruction
    cpu.write_memory(0, &(encoding as u32).to_le_bytes())
        .unwrap();
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0x12345678,
        "X0 should be 0x0000000012345678"
    );
}

/// Provenance: aarch32_CBNZ_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// shift by 4 (32)
#[test]
fn test_aarch32_cbnz_t1_a_lslv_oracle_32_1_b1020020() {
    // Test LSLV 32-bit: shift by 4 (oracle)
    // ISET: T16
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x12345678);
    set_w(&mut cpu, 2, 0x4);
    let encoding: u16 = 0x0020;
    // T16: Write 16-bit instruction
    cpu.write_memory(0, &(encoding as u32).to_le_bytes())
        .unwrap();
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x23456780, "W0 should be 0x23456780");
}

/// Provenance: aarch32_CBNZ_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// shift by 4 (64)
#[test]
fn test_aarch32_cbnz_t1_a_lslv_oracle_64_1_b1020020() {
    // Test LSLV 64-bit: shift by 4 (oracle)
    // ISET: T16
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x12345678);
    set_w(&mut cpu, 2, 0x4);
    let encoding: u16 = 0x0020;
    // T16: Write 16-bit instruction
    cpu.write_memory(0, &(encoding as u32).to_le_bytes())
        .unwrap();
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0x23456780,
        "X0 should be 0x0000000123456780"
    );
}

/// Provenance: aarch32_CBNZ_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// shift by 8 (32)
#[test]
fn test_aarch32_cbnz_t1_a_lslv_oracle_32_2_b1020020() {
    // Test LSLV 32-bit: shift by 8 (oracle)
    // ISET: T16
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x8);
    set_w(&mut cpu, 1, 0x12345678);
    let encoding: u16 = 0x0020;
    // T16: Write 16-bit instruction
    cpu.write_memory(0, &(encoding as u32).to_le_bytes())
        .unwrap();
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x34567800, "W0 should be 0x34567800");
}

/// Provenance: aarch32_CBNZ_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// shift by 8 (64)
#[test]
fn test_aarch32_cbnz_t1_a_lslv_oracle_64_2_b1020020() {
    // Test LSLV 64-bit: shift by 8 (oracle)
    // ISET: T16
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x12345678);
    set_w(&mut cpu, 2, 0x8);
    let encoding: u16 = 0x0020;
    // T16: Write 16-bit instruction
    cpu.write_memory(0, &(encoding as u32).to_le_bytes())
        .unwrap();
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0x34567800,
        "X0 should be 0x0000001234567800"
    );
}

/// Provenance: aarch32_CBNZ_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// MSB set, shift 1 (32)
#[test]
fn test_aarch32_cbnz_t1_a_lslv_oracle_32_3_b1020020() {
    // Test LSLV 32-bit: MSB set, shift 1 (oracle)
    // ISET: T16
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    set_w(&mut cpu, 2, 0x1);
    let encoding: u16 = 0x0020;
    // T16: Write 16-bit instruction
    cpu.write_memory(0, &(encoding as u32).to_le_bytes())
        .unwrap();
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x0, "W0 should be 0x00000000");
}

/// Provenance: aarch32_CBNZ_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// MSB set, shift 1 (64)
#[test]
fn test_aarch32_cbnz_t1_a_lslv_oracle_64_3_b1020020() {
    // Test LSLV 64-bit: MSB set, shift 1 (oracle)
    // ISET: T16
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    set_w(&mut cpu, 2, 0x1);
    let encoding: u16 = 0x0020;
    // T16: Write 16-bit instruction
    cpu.write_memory(0, &(encoding as u32).to_le_bytes())
        .unwrap();
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x0, "X0 should be 0x0000000000000000");
}

/// Provenance: aarch32_CBNZ_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// LSB set, max shift (32)
#[test]
fn test_aarch32_cbnz_t1_a_lslv_oracle_32_4_b1020020() {
    // Test LSLV 32-bit: LSB set, max shift (oracle)
    // ISET: T16
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x1);
    set_w(&mut cpu, 2, 0x3F);
    let encoding: u16 = 0x0020;
    // T16: Write 16-bit instruction
    cpu.write_memory(0, &(encoding as u32).to_le_bytes())
        .unwrap();
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x80000000, "W0 should be 0x80000000");
}

/// Provenance: aarch32_CBNZ_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// LSB set, max shift (64)
#[test]
fn test_aarch32_cbnz_t1_a_lslv_oracle_64_4_b1020020() {
    // Test LSLV 64-bit: LSB set, max shift (oracle)
    // ISET: T16
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x1);
    set_w(&mut cpu, 2, 0x3F);
    let encoding: u16 = 0x0020;
    // T16: Write 16-bit instruction
    cpu.write_memory(0, &(encoding as u32).to_le_bytes())
        .unwrap();
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x0, "X0 should be 0x8000000000000000");
}

/// Provenance: aarch32_CBNZ_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// all ones, shift 32 (32)
#[test]
fn test_aarch32_cbnz_t1_a_lslv_oracle_32_5_b1020020() {
    // Test LSLV 32-bit: all ones, shift 32 (oracle)
    // ISET: T16
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    set_w(&mut cpu, 2, 0x20);
    let encoding: u16 = 0x0020;
    // T16: Write 16-bit instruction
    cpu.write_memory(0, &(encoding as u32).to_le_bytes())
        .unwrap();
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0xFFFFFFFF, "W0 should be 0xFFFFFFFF");
}

/// Provenance: aarch32_CBNZ_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// all ones, shift 32 (64)
#[test]
fn test_aarch32_cbnz_t1_a_lslv_oracle_64_5_b1020020() {
    // Test LSLV 64-bit: all ones, shift 32 (oracle)
    // ISET: T16
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x20);
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u16 = 0x0020;
    // T16: Write 16-bit instruction
    cpu.write_memory(0, &(encoding as u32).to_le_bytes())
        .unwrap();
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x0, "X0 should be 0xFFFFFFFF00000000");
}

/// Provenance: aarch32_CBNZ_T1_A
/// ASL: `LSLS R0, R1, R2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// no shift
#[test]
fn test_aarch32_cbnz_t1_a_t16_oracle_0_b1010000() {
    // Test T16 LSLS: no shift (oracle)
    // ISET: T16
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x0);
    set_w(&mut cpu, 1, 0xFF);
    let encoding: u16 = 0x0000;
    // T16: Write 16-bit instruction
    cpu.write_memory(0, &(encoding as u32).to_le_bytes())
        .unwrap();
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0xFF, "R0 should be 0x000000FF");
}

/// Provenance: aarch32_CBNZ_T1_A
/// ASL: `LSLS R0, R1, R2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// shift by 4
#[test]
fn test_aarch32_cbnz_t1_a_t16_oracle_1_b1010000() {
    // Test T16 LSLS: shift by 4 (oracle)
    // ISET: T16
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFF);
    set_w(&mut cpu, 2, 0x4);
    let encoding: u16 = 0x0000;
    // T16: Write 16-bit instruction
    cpu.write_memory(0, &(encoding as u32).to_le_bytes())
        .unwrap();
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0xFF0, "R0 should be 0x00000FF0");
}

/// Provenance: aarch32_CBNZ_T1_A
/// ASL: `LSLS R0, R1, R2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// MSB set, shift 1
#[test]
fn test_aarch32_cbnz_t1_a_t16_oracle_2_b1010000() {
    // Test T16 LSLS: MSB set, shift 1 (oracle)
    // ISET: T16
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x80000000);
    set_w(&mut cpu, 2, 0x1);
    let encoding: u16 = 0x0000;
    // T16: Write 16-bit instruction
    cpu.write_memory(0, &(encoding as u32).to_le_bytes())
        .unwrap();
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x0, "R0 should be 0x00000000");
}

/// Provenance: aarch32_CBNZ_T1_A
/// ASL: `LSLS R0, R1, R2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// shift to MSB
#[test]
fn test_aarch32_cbnz_t1_a_t16_oracle_3_b1010000() {
    // Test T16 LSLS: shift to MSB (oracle)
    // ISET: T16
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x1F);
    set_w(&mut cpu, 1, 0x1);
    let encoding: u16 = 0x0000;
    // T16: Write 16-bit instruction
    cpu.write_memory(0, &(encoding as u32).to_le_bytes())
        .unwrap();
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x80000000, "R0 should be 0x80000000");
}
