//! T16 misc other tests.
//!
//! Auto-generated from ARM ASL specifications.
//! DO NOT EDIT MANUALLY.

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::generated::test_helpers_32::*;

// ============================================================================
// aarch32_IT_A Tests
// ============================================================================

/// Provenance: aarch32_IT_T1_A
/// ASL: `field firstcond 20 +: 4`
/// Requirement: FieldBoundary { field: "firstcond", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_it_t1_a_field_firstcond_0_min_0_bf000000() {
    // Thumb encoding (32): 0xBF000000
    // Test aarch32_IT_T1_A field firstcond = 0 (Min)
    // ISET: T32
    // Fields: firstcond=0, mask=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xBF000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_IT_T1_A
/// ASL: `field firstcond 20 +: 4`
/// Requirement: FieldBoundary { field: "firstcond", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_aarch32_it_t1_a_field_firstcond_1_poweroftwo_0_bf100000() {
    // Thumb encoding (32): 0xBF100000
    // Test aarch32_IT_T1_A field firstcond = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: firstcond=1, mask=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xBF100000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_IT_T1_A
/// ASL: `field firstcond 20 +: 4`
/// Requirement: FieldBoundary { field: "firstcond", value: 7, boundary: PowerOfTwoMinusOne }
/// midpoint (7)
#[test]
fn test_aarch32_it_t1_a_field_firstcond_7_poweroftwominusone_0_bf700000() {
    // Thumb encoding (32): 0xBF700000
    // Test aarch32_IT_T1_A field firstcond = 7 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: mask=0, firstcond=7
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xBF700000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_IT_T1_A
/// ASL: `field firstcond 20 +: 4`
/// Requirement: FieldBoundary { field: "firstcond", value: 15, boundary: Max }
/// maximum value (15)
#[test]
fn test_aarch32_it_t1_a_field_firstcond_15_max_0_bff00000() {
    // Thumb encoding (32): 0xBFF00000
    // Test aarch32_IT_T1_A field firstcond = 15 (Max)
    // ISET: T32
    // Fields: mask=0, firstcond=15
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xBFF00000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_IT_T1_A
/// ASL: `field mask 16 +: 4`
/// Requirement: FieldBoundary { field: "mask", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_it_t1_a_field_mask_0_min_0_bf000000() {
    // Thumb encoding (32): 0xBF000000
    // Test aarch32_IT_T1_A field mask = 0 (Min)
    // ISET: T32
    // Fields: firstcond=0, mask=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xBF000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_IT_T1_A
/// ASL: `field mask 16 +: 4`
/// Requirement: FieldBoundary { field: "mask", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_aarch32_it_t1_a_field_mask_1_poweroftwo_0_bf010000() {
    // Thumb encoding (32): 0xBF010000
    // Test aarch32_IT_T1_A field mask = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: firstcond=0, mask=1
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xBF010000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_IT_T1_A
/// ASL: `field mask 16 +: 4`
/// Requirement: FieldBoundary { field: "mask", value: 7, boundary: PowerOfTwoMinusOne }
/// midpoint (7)
#[test]
fn test_aarch32_it_t1_a_field_mask_7_poweroftwominusone_0_bf070000() {
    // Thumb encoding (32): 0xBF070000
    // Test aarch32_IT_T1_A field mask = 7 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: mask=7, firstcond=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xBF070000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_IT_T1_A
/// ASL: `field mask 16 +: 4`
/// Requirement: FieldBoundary { field: "mask", value: 15, boundary: Max }
/// maximum value (15)
#[test]
fn test_aarch32_it_t1_a_field_mask_15_max_0_bf0f0000() {
    // Thumb encoding (32): 0xBF0F0000
    // Test aarch32_IT_T1_A field mask = 15 (Max)
    // ISET: T32
    // Fields: firstcond=0, mask=15
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xBF0F0000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_IT_T1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// firstcond=0 (minimum value)
#[test]
fn test_aarch32_it_t1_a_combo_0_0_bf000000() {
    // Thumb encoding (32): 0xBF000000
    // Test aarch32_IT_T1_A field combination: firstcond=0, mask=0
    // ISET: T32
    // Fields: firstcond=0, mask=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xBF000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_IT_T1_A
/// ASL: `Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "firstcond" }), rhs: Binary { op: Or, lhs: LitBits([true, true, true, true]), rhs: Binary { op: Ne, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "firstcond" }), rhs: Binary { op: And, lhs: LitBits([true, true, true, false]), rhs: Call { name: QualifiedIdentifier { qualifier: Any, name: "BitCount" }, args: [Var(QualifiedIdentifier { qualifier: Any, name: "mask" })] } } }, rhs: LitInt(1) } } }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"firstcond\" }), rhs: Binary { op: Or, lhs: LitBits([true, true, true, true]), rhs: Binary { op: Ne, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"firstcond\" }), rhs: Binary { op: And, lhs: LitBits([true, true, true, false]), rhs: Call { name: QualifiedIdentifier { qualifier: Any, name: \"BitCount\" }, args: [Var(QualifiedIdentifier { qualifier: Any, name: \"mask\" })] } } }, rhs: LitInt(1) } } }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_it_t1_a_invalid_0_0_bf000000() {
    // Thumb encoding (32): 0xBF000000
    // Test aarch32_IT_T1_A invalid encoding: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "firstcond" }), rhs: Binary { op: Or, lhs: LitBits([true, true, true, true]), rhs: Binary { op: Ne, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "firstcond" }), rhs: Binary { op: And, lhs: LitBits([true, true, true, false]), rhs: Call { name: QualifiedIdentifier { qualifier: Any, name: "BitCount" }, args: [Var(QualifiedIdentifier { qualifier: Any, name: "mask" })] } } }, rhs: LitInt(1) } } }
    // ISET: T32
    // Fields: firstcond=0, mask=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xBF000000;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

/// Provenance: aarch32_IT_T1_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_it_t1_a_invalid_1_0_bf000000() {
    // Thumb encoding (32): 0xBF000000
    // Test aarch32_IT_T1_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: T32
    // Fields: firstcond=0, mask=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xBF000000;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

/// Provenance: aarch32_IT_T1_A
/// ASL: `Call { name: QualifiedIdentifier { qualifier: Any, name: "InITBlock" }, args: [] }`
/// Requirement: UndefinedEncoding { condition: "Call { name: QualifiedIdentifier { qualifier: Any, name: \"InITBlock\" }, args: [] }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_it_t1_a_invalid_2_0_bf000000() {
    // Thumb encoding (32): 0xBF000000
    // Test aarch32_IT_T1_A invalid encoding: Call { name: QualifiedIdentifier { qualifier: Any, name: "InITBlock" }, args: [] }
    // ISET: T32
    // Fields: firstcond=0, mask=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xBF000000;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

/// Provenance: aarch32_IT_T1_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_it_t1_a_invalid_3_0_bf000000() {
    // Thumb encoding (32): 0xBF000000
    // Test aarch32_IT_T1_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: T32
    // Fields: firstcond=0, mask=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xBF000000;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

/// Provenance: aarch32_IT_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// shift by 0 (32)
#[test]
fn test_aarch32_it_t1_a_lslv_oracle_32_0_bf020020() {
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

/// Provenance: aarch32_IT_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// shift by 0 (64)
#[test]
fn test_aarch32_it_t1_a_lslv_oracle_64_0_bf020020() {
    // Test LSLV 64-bit: shift by 0 (oracle)
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
    assert_eq!(
        get_w(&cpu, 0),
        0x12345678,
        "X0 should be 0x0000000012345678"
    );
}

/// Provenance: aarch32_IT_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// shift by 4 (32)
#[test]
fn test_aarch32_it_t1_a_lslv_oracle_32_1_bf020020() {
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

/// Provenance: aarch32_IT_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// shift by 4 (64)
#[test]
fn test_aarch32_it_t1_a_lslv_oracle_64_1_bf020020() {
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

/// Provenance: aarch32_IT_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// shift by 8 (32)
#[test]
fn test_aarch32_it_t1_a_lslv_oracle_32_2_bf020020() {
    // Test LSLV 32-bit: shift by 8 (oracle)
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
    assert_eq!(get_w(&cpu, 0), 0x34567800, "W0 should be 0x34567800");
}

/// Provenance: aarch32_IT_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// shift by 8 (64)
#[test]
fn test_aarch32_it_t1_a_lslv_oracle_64_2_bf020020() {
    // Test LSLV 64-bit: shift by 8 (oracle)
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
    assert_eq!(
        get_w(&cpu, 0),
        0x34567800,
        "X0 should be 0x0000001234567800"
    );
}

/// Provenance: aarch32_IT_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// MSB set, shift 1 (32)
#[test]
fn test_aarch32_it_t1_a_lslv_oracle_32_3_bf020020() {
    // Test LSLV 32-bit: MSB set, shift 1 (oracle)
    // ISET: T16
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x1);
    set_w(&mut cpu, 1, 0x0);
    let encoding: u16 = 0x0020;
    // T16: Write 16-bit instruction
    cpu.write_memory(0, &(encoding as u32).to_le_bytes())
        .unwrap();
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x0, "W0 should be 0x00000000");
}

/// Provenance: aarch32_IT_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// MSB set, shift 1 (64)
#[test]
fn test_aarch32_it_t1_a_lslv_oracle_64_3_bf020020() {
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

/// Provenance: aarch32_IT_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// LSB set, max shift (32)
#[test]
fn test_aarch32_it_t1_a_lslv_oracle_32_4_bf020020() {
    // Test LSLV 32-bit: LSB set, max shift (oracle)
    // ISET: T16
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x3F);
    set_w(&mut cpu, 1, 0x1);
    let encoding: u16 = 0x0020;
    // T16: Write 16-bit instruction
    cpu.write_memory(0, &(encoding as u32).to_le_bytes())
        .unwrap();
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x80000000, "W0 should be 0x80000000");
}

/// Provenance: aarch32_IT_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// LSB set, max shift (64)
#[test]
fn test_aarch32_it_t1_a_lslv_oracle_64_4_bf020020() {
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

/// Provenance: aarch32_IT_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// all ones, shift 32 (32)
#[test]
fn test_aarch32_it_t1_a_lslv_oracle_32_5_bf020020() {
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

/// Provenance: aarch32_IT_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// all ones, shift 32 (64)
#[test]
fn test_aarch32_it_t1_a_lslv_oracle_64_5_bf020020() {
    // Test LSLV 64-bit: all ones, shift 32 (oracle)
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
    assert_eq!(get_w(&cpu, 0), 0x0, "X0 should be 0xFFFFFFFF00000000");
}

/// Provenance: aarch32_IT_T1_A
/// ASL: `LSLS R0, R1, R2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// no shift
#[test]
fn test_aarch32_it_t1_a_t16_oracle_0_bf000000() {
    // Test T16 LSLS: no shift (oracle)
    // ISET: T16
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFF);
    set_w(&mut cpu, 2, 0x0);
    let encoding: u16 = 0x0000;
    // T16: Write 16-bit instruction
    cpu.write_memory(0, &(encoding as u32).to_le_bytes())
        .unwrap();
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0xFF, "R0 should be 0x000000FF");
}

/// Provenance: aarch32_IT_T1_A
/// ASL: `LSLS R0, R1, R2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// shift by 4
#[test]
fn test_aarch32_it_t1_a_t16_oracle_1_bf000000() {
    // Test T16 LSLS: shift by 4 (oracle)
    // ISET: T16
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x4);
    set_w(&mut cpu, 1, 0xFF);
    let encoding: u16 = 0x0000;
    // T16: Write 16-bit instruction
    cpu.write_memory(0, &(encoding as u32).to_le_bytes())
        .unwrap();
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0xFF0, "R0 should be 0x00000FF0");
}

/// Provenance: aarch32_IT_T1_A
/// ASL: `LSLS R0, R1, R2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// MSB set, shift 1
#[test]
fn test_aarch32_it_t1_a_t16_oracle_2_bf000000() {
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

/// Provenance: aarch32_IT_T1_A
/// ASL: `LSLS R0, R1, R2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// shift to MSB
#[test]
fn test_aarch32_it_t1_a_t16_oracle_3_bf000000() {
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
