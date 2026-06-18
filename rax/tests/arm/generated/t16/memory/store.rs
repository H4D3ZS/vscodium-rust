//! T16 memory store tests.
//!
//! Auto-generated from ARM ASL specifications.
//! DO NOT EDIT MANUALLY.

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::generated::test_helpers_32::*;

// ============================================================================
// aarch32_PUSH_A Tests
// ============================================================================

/// Provenance: aarch32_PUSH_T1_A
/// ASL: `field M 24 +: 1`
/// Requirement: FieldBoundary { field: "M", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_push_t1_a_field_m_0_min_0_b4000000() {
    // Thumb encoding (32): 0xB4000000
    // Test aarch32_PUSH_T1_A field M = 0 (Min)
    // ISET: T32
    // Fields: register_list=0, M=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xB4000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_PUSH_T1_A
/// ASL: `field M 24 +: 1`
/// Requirement: FieldBoundary { field: "M", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_push_t1_a_field_m_1_max_0_b5000000() {
    // Thumb encoding (32): 0xB5000000
    // Test aarch32_PUSH_T1_A field M = 1 (Max)
    // ISET: T32
    // Fields: register_list=0, M=1
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xB5000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_PUSH_T1_A
/// ASL: `field register_list 16 +: 8`
/// Requirement: FieldBoundary { field: "register_list", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_push_t1_a_field_register_list_0_min_0_b4000000() {
    // Thumb encoding (32): 0xB4000000
    // Test aarch32_PUSH_T1_A field register_list = 0 (Min)
    // ISET: T32
    // Fields: M=0, register_list=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xB4000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_PUSH_T1_A
/// ASL: `field register_list 16 +: 8`
/// Requirement: FieldBoundary { field: "register_list", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_aarch32_push_t1_a_field_register_list_1_poweroftwo_0_b4010000() {
    // Thumb encoding (32): 0xB4010000
    // Test aarch32_PUSH_T1_A field register_list = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: register_list=1, M=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xB4010000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_PUSH_T1_A
/// ASL: `field register_list 16 +: 8`
/// Requirement: FieldBoundary { field: "register_list", value: 127, boundary: PowerOfTwoMinusOne }
/// midpoint (127)
#[test]
fn test_aarch32_push_t1_a_field_register_list_127_poweroftwominusone_0_b47f0000() {
    // Thumb encoding (32): 0xB47F0000
    // Test aarch32_PUSH_T1_A field register_list = 127 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: register_list=127, M=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xB47F0000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_PUSH_T1_A
/// ASL: `field register_list 16 +: 8`
/// Requirement: FieldBoundary { field: "register_list", value: 255, boundary: Max }
/// maximum value (255)
#[test]
fn test_aarch32_push_t1_a_field_register_list_255_max_0_b4ff0000() {
    // Thumb encoding (32): 0xB4FF0000
    // Test aarch32_PUSH_T1_A field register_list = 255 (Max)
    // ISET: T32
    // Fields: M=0, register_list=255
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xB4FF0000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_PUSH_T1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// M=0 (minimum value)
#[test]
fn test_aarch32_push_t1_a_combo_0_0_b4000000() {
    // Thumb encoding (32): 0xB4000000
    // Test aarch32_PUSH_T1_A field combination: M=0, register_list=0
    // ISET: T32
    // Fields: register_list=0, M=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xB4000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_PUSH_T1_A
/// ASL: `Binary { op: Lt, lhs: Call { name: QualifiedIdentifier { qualifier: Any, name: "BitCount" }, args: [Var(QualifiedIdentifier { qualifier: Any, name: "registers" })] }, rhs: LitInt(1) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Lt, lhs: Call { name: QualifiedIdentifier { qualifier: Any, name: \"BitCount\" }, args: [Var(QualifiedIdentifier { qualifier: Any, name: \"registers\" })] }, rhs: LitInt(1) }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_push_t1_a_invalid_0_0_b4000000() {
    // Thumb encoding (32): 0xB4000000
    // Test aarch32_PUSH_T1_A invalid encoding: Binary { op: Lt, lhs: Call { name: QualifiedIdentifier { qualifier: Any, name: "BitCount" }, args: [Var(QualifiedIdentifier { qualifier: Any, name: "registers" })] }, rhs: LitInt(1) }
    // ISET: T32
    // Fields: register_list=0, M=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xB4000000;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

/// Provenance: aarch32_PUSH_T1_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_push_t1_a_invalid_1_0_b4000000() {
    // Thumb encoding (32): 0xB4000000
    // Test aarch32_PUSH_T1_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: T32
    // Fields: M=0, register_list=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xB4000000;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

/// Provenance: aarch32_PUSH_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// shift by 0 (32)
#[test]
fn test_aarch32_push_t1_a_lslv_oracle_32_0_b4020020() {
    // Test LSLV 32-bit: shift by 0 (oracle)
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
    assert_eq!(get_w(&cpu, 0), 0x12345678, "W0 should be 0x12345678");
}

/// Provenance: aarch32_PUSH_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// shift by 0 (64)
#[test]
fn test_aarch32_push_t1_a_lslv_oracle_64_0_b4020020() {
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

/// Provenance: aarch32_PUSH_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// shift by 4 (32)
#[test]
fn test_aarch32_push_t1_a_lslv_oracle_32_1_b4020020() {
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

/// Provenance: aarch32_PUSH_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// shift by 4 (64)
#[test]
fn test_aarch32_push_t1_a_lslv_oracle_64_1_b4020020() {
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

/// Provenance: aarch32_PUSH_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// shift by 8 (32)
#[test]
fn test_aarch32_push_t1_a_lslv_oracle_32_2_b4020020() {
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

/// Provenance: aarch32_PUSH_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// shift by 8 (64)
#[test]
fn test_aarch32_push_t1_a_lslv_oracle_64_2_b4020020() {
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

/// Provenance: aarch32_PUSH_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// MSB set, shift 1 (32)
#[test]
fn test_aarch32_push_t1_a_lslv_oracle_32_3_b4020020() {
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

/// Provenance: aarch32_PUSH_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// MSB set, shift 1 (64)
#[test]
fn test_aarch32_push_t1_a_lslv_oracle_64_3_b4020020() {
    // Test LSLV 64-bit: MSB set, shift 1 (oracle)
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
    assert_eq!(get_w(&cpu, 0), 0x0, "X0 should be 0x0000000000000000");
}

/// Provenance: aarch32_PUSH_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// LSB set, max shift (32)
#[test]
fn test_aarch32_push_t1_a_lslv_oracle_32_4_b4020020() {
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

/// Provenance: aarch32_PUSH_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// LSB set, max shift (64)
#[test]
fn test_aarch32_push_t1_a_lslv_oracle_64_4_b4020020() {
    // Test LSLV 64-bit: LSB set, max shift (oracle)
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
    assert_eq!(get_w(&cpu, 0), 0x0, "X0 should be 0x8000000000000000");
}

/// Provenance: aarch32_PUSH_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// all ones, shift 32 (32)
#[test]
fn test_aarch32_push_t1_a_lslv_oracle_32_5_b4020020() {
    // Test LSLV 32-bit: all ones, shift 32 (oracle)
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
    assert_eq!(get_w(&cpu, 0), 0xFFFFFFFF, "W0 should be 0xFFFFFFFF");
}

/// Provenance: aarch32_PUSH_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// all ones, shift 32 (64)
#[test]
fn test_aarch32_push_t1_a_lslv_oracle_64_5_b4020020() {
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

/// Provenance: aarch32_PUSH_T1_A
/// ASL: `LSLS R0, R1, R2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// no shift
#[test]
fn test_aarch32_push_t1_a_t16_oracle_0_b4000000() {
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

/// Provenance: aarch32_PUSH_T1_A
/// ASL: `LSLS R0, R1, R2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// shift by 4
#[test]
fn test_aarch32_push_t1_a_t16_oracle_1_b4000000() {
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

/// Provenance: aarch32_PUSH_T1_A
/// ASL: `LSLS R0, R1, R2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// MSB set, shift 1
#[test]
fn test_aarch32_push_t1_a_t16_oracle_2_b4000000() {
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

/// Provenance: aarch32_PUSH_T1_A
/// ASL: `LSLS R0, R1, R2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// shift to MSB
#[test]
fn test_aarch32_push_t1_a_t16_oracle_3_b4000000() {
    // Test T16 LSLS: shift to MSB (oracle)
    // ISET: T16
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x1);
    set_w(&mut cpu, 2, 0x1F);
    let encoding: u16 = 0x0000;
    // T16: Write 16-bit instruction
    cpu.write_memory(0, &(encoding as u32).to_le_bytes())
        .unwrap();
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x80000000, "R0 should be 0x80000000");
}

/// Provenance: aarch32_PUSH_T1_A
/// ASL: `Mem[address, 8] = data`
/// Requirement: MemoryAccess { op: Store, size_bits: 64, addressing: "Base { reg: \"address\" }" }
/// 8-byte store
#[test]
fn test_aarch32_push_t1_a_store_0_b4000000() {
    // Test aarch32_PUSH_T1_A memory store: 8 bytes
    // ISET: T16
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 0, 0xCAFEBABE);
    set_w(&mut cpu, 1, 0x0);
    let encoding: u16 = 0x0000;
    // T16: Write 16-bit instruction
    cpu.write_memory(0, &(encoding as u32).to_le_bytes())
        .unwrap();
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch32_PUSH_T1_A
/// ASL: `Mem[address, 8] = data`
/// Requirement: MemoryAccess { op: Store, size_bits: 64, addressing: "Base { reg: \"address\" }" }
/// 8-byte store
#[test]
fn test_aarch32_push_t1_a_store_1_b4000000() {
    // Test aarch32_PUSH_T1_A memory store: 8 bytes
    // ISET: T16
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    set_w(&mut cpu, 0, 0xCAFEBABE);
    let encoding: u16 = 0x0000;
    // T16: Write 16-bit instruction
    cpu.write_memory(0, &(encoding as u32).to_le_bytes())
        .unwrap();
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch32_PUSH_T1_A
/// ASL: `Mem[address, 8] = data`
/// Requirement: MemoryAccess { op: Store, size_bits: 64, addressing: "Base { reg: \"address\" }" }
/// 8-byte store
#[test]
fn test_aarch32_push_t1_a_store_2_b4000000() {
    // Test aarch32_PUSH_T1_A memory store: 8 bytes
    // ISET: T16
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    set_w(&mut cpu, 0, 0xCAFEBABE);
    let encoding: u16 = 0x0000;
    // T16: Write 16-bit instruction
    cpu.write_memory(0, &(encoding as u32).to_le_bytes())
        .unwrap();
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch32_PUSH_T1_A
/// ASL: `Mem[address, 8] = data`
/// Requirement: MemoryAccess { op: Store, size_bits: 64, addressing: "Base { reg: \"address\" }" }
/// 8-byte store
#[test]
fn test_aarch32_push_t1_a_store_3_b4000000() {
    // Test aarch32_PUSH_T1_A memory store: 8 bytes
    // ISET: T16
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    set_w(&mut cpu, 0, 0xCAFEBABE);
    let encoding: u16 = 0x0000;
    // T16: Write 16-bit instruction
    cpu.write_memory(0, &(encoding as u32).to_le_bytes())
        .unwrap();
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch32_PUSH_T1_A
/// ASL: `Mem[address, 8] = data`
/// Requirement: MemoryAccess { op: Store, size_bits: 64, addressing: "Base { reg: \"address\" }" }
/// 8-byte store
#[test]
fn test_aarch32_push_t1_a_store_4_b4000000() {
    // Test aarch32_PUSH_T1_A memory store: 8 bytes
    // ISET: T16
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    set_w(&mut cpu, 0, 0xCAFEBABE);
    let encoding: u16 = 0x0000;
    // T16: Write 16-bit instruction
    cpu.write_memory(0, &(encoding as u32).to_le_bytes())
        .unwrap();
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}
