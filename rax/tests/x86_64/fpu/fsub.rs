//! Tests for FSUB, FSUBP, FISUB, FSUBR, FSUBRP, and FISUBR instructions.
//!
//! FSUB - Subtract
//! FSUBP - Subtract and pop
//! FISUB - Subtract integer
//! FSUBR - Reverse subtract
//! FSUBRP - Reverse subtract and pop
//! FISUBR - Reverse subtract integer
//!
//! References: /Users/int/dev/rax/docs/fsub:fsubp:fisub.txt
//!             /Users/int/dev/rax/docs/fsubr:fsubrp:fisubr.txt

use crate::common::*;
use std::sync::Arc;
use vm_memory::{Bytes, GuestAddress, GuestMemoryMmap};

const DATA_ADDR: u64 = 0x2000;

fn write_f32(mem: &Arc<GuestMemoryMmap>, addr: u64, value: f32) {
    mem.write_slice(&value.to_le_bytes(), GuestAddress(addr))
        .unwrap();
}

fn write_f64(mem: &Arc<GuestMemoryMmap>, addr: u64, value: f64) {
    mem.write_slice(&value.to_le_bytes(), GuestAddress(addr))
        .unwrap();
}

fn write_i16(mem: &Arc<GuestMemoryMmap>, addr: u64, value: i16) {
    mem.write_slice(&value.to_le_bytes(), GuestAddress(addr))
        .unwrap();
}

fn write_i32(mem: &Arc<GuestMemoryMmap>, addr: u64, value: i32) {
    mem.write_slice(&value.to_le_bytes(), GuestAddress(addr))
        .unwrap();
}

fn read_f64(mem: &Arc<GuestMemoryMmap>, addr: u64) -> f64 {
    let mut buf = [0u8; 8];
    mem.read_slice(&mut buf, GuestAddress(addr)).unwrap();
    f64::from_le_bytes(buf)
}

// ============================================================================
// FSUB m32fp (opcode D8 /4) - ST(0) = ST(0) - m32fp
// ============================================================================

#[test]
fn test_fsub_m32fp_basic() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xD8, 0x24, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD,
        0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 10.0);
    write_f32(&mem, DATA_ADDR + 8, 3.0);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 7.0);
}

#[test]
fn test_fsub_m32fp_negative_result() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xD8, 0x24, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD,
        0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 3.0);
    write_f32(&mem, DATA_ADDR + 8, 10.0);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), -7.0);
}

#[test]
fn test_fsub_m32fp_zero_result() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xD8, 0x24, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD,
        0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 5.5);
    write_f32(&mem, DATA_ADDR + 8, 5.5);

    run_until_hlt(&mut vcpu).unwrap();
    let result = read_f64(&mem, 0x3000);
    assert_eq!(result, 0.0);
    assert!(!result.is_sign_negative());
}

// ============================================================================
// FSUB m64fp (opcode DC /4)
// ============================================================================

#[test]
fn test_fsub_m64fp_basic() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDC, 0x24, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD,
        0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 100.5);
    write_f64(&mem, DATA_ADDR + 8, 50.25);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 50.25);
}

#[test]
fn test_fsub_m64fp_large_values() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDC, 0x24, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD,
        0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 3.0e15);
    write_f64(&mem, DATA_ADDR + 8, 1.0e15);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 2.0e15);
}

// ============================================================================
// FSUB ST(0), ST(i) (opcode D8 E0+i)
// ============================================================================

#[test]
fn test_fsub_st0_st1() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, 0xD8,
        0xE1, // FSUB ST(0), ST(1)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 5.0);
    write_f64(&mem, DATA_ADDR + 8, 12.0);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 7.0); // 12.0 - 5.0
}

// ============================================================================
// FSUB ST(i), ST(0) (opcode DC E8+i)
// ============================================================================

#[test]
fn test_fsub_st1_st0() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDC,
        0xE9, // FSUB ST(1), ST(0)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 5.0);
    write_f64(&mem, DATA_ADDR + 8, 12.0);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 12.0); // ST(0) unchanged
    assert_eq!(read_f64(&mem, 0x3008), -7.0); // ST(1) = 5.0 - 12.0
}

// ============================================================================
// FSUBP ST(i), ST(0) (opcode DE E8+i)
// ============================================================================

#[test]
fn test_fsubp_st1_st0() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDE,
        0xE9, // FSUBP ST(1), ST(0)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 3.0);
    write_f64(&mem, DATA_ADDR + 8, 10.0);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), -7.0); // 3.0 - 10.0
}

// ============================================================================
// FISUB m16int (opcode DE /4)
// ============================================================================

#[test]
fn test_fisub_m16int_positive() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDE, 0x24, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD,
        0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 100.5);
    write_i16(&mem, DATA_ADDR + 8, 25);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 75.5);
}

#[test]
fn test_fisub_m16int_negative() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDE, 0x24, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD,
        0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 50.0);
    write_i16(&mem, DATA_ADDR + 8, -10);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 60.0);
}

// ============================================================================
// FISUB m32int (opcode DA /4)
// ============================================================================

#[test]
fn test_fisub_m32int_positive() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDA, 0x24, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD,
        0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 1000.5);
    write_i32(&mem, DATA_ADDR + 8, 250);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 750.5);
}

// ============================================================================
// FSUBR m32fp (opcode D8 /5) - ST(0) = m32fp - ST(0)
// ============================================================================

#[test]
fn test_fsubr_m32fp_basic() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xD8, 0x2C, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD,
        0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 3.0);
    write_f32(&mem, DATA_ADDR + 8, 10.0);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 7.0); // 10.0 - 3.0
}

#[test]
fn test_fsubr_m32fp_negative_result() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xD8, 0x2C, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD,
        0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 10.0);
    write_f32(&mem, DATA_ADDR + 8, 3.0);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), -7.0); // 3.0 - 10.0
}

// ============================================================================
// FSUBR m64fp (opcode DC /5)
// ============================================================================

#[test]
fn test_fsubr_m64fp_basic() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDC, 0x2C, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD,
        0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 25.5);
    write_f64(&mem, DATA_ADDR + 8, 100.5);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 75.0); // 100.5 - 25.5
}

// ============================================================================
// FSUBR ST(0), ST(i) (opcode D8 E8+i)
// ============================================================================

#[test]
fn test_fsubr_st0_st1() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, 0xD8,
        0xE9, // FSUBR ST(0), ST(1)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 5.0);
    write_f64(&mem, DATA_ADDR + 8, 12.0);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), -7.0); // 5.0 - 12.0
}

// ============================================================================
// FSUBR ST(i), ST(0) (opcode DC E0+i)
// ============================================================================

#[test]
fn test_fsubr_st1_st0() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDC,
        0xE1, // FSUBR ST(1), ST(0)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 5.0);
    write_f64(&mem, DATA_ADDR + 8, 12.0);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 12.0); // ST(0) unchanged
    assert_eq!(read_f64(&mem, 0x3008), 7.0); // ST(1) = 12.0 - 5.0
}

// ============================================================================
// FSUBRP ST(i), ST(0) (opcode DE E0+i)
// ============================================================================

#[test]
fn test_fsubrp_st1_st0() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDE,
        0xE1, // FSUBRP ST(1), ST(0)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 3.0);
    write_f64(&mem, DATA_ADDR + 8, 10.0);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 7.0); // 10.0 - 3.0
}

// ============================================================================
// FISUBR m16int (opcode DE /5)
// ============================================================================

#[test]
fn test_fisubr_m16int_positive() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDE, 0x2C, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD,
        0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 25.5);
    write_i16(&mem, DATA_ADDR + 8, 100);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 74.5); // 100 - 25.5
}

// ============================================================================
// FISUBR m32int (opcode DA /5)
// ============================================================================

#[test]
fn test_fisubr_m32int_positive() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDA, 0x2C, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD,
        0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 250.5);
    write_i32(&mem, DATA_ADDR + 8, 1000);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 749.5); // 1000 - 250.5
}

// ============================================================================
// Special cases and edge tests
// ============================================================================

#[test]
fn test_fsub_infinity_handling() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDC, 0x24, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD,
        0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, f64::INFINITY);
    write_f64(&mem, DATA_ADDR + 8, 100.0);

    run_until_hlt(&mut vcpu).unwrap();
    let result = read_f64(&mem, 0x3000);
    assert!(result.is_infinite() && result.is_sign_positive());
}

#[test]
fn test_fsub_from_infinity() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDC, 0x24, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD,
        0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 100.0);
    write_f64(&mem, DATA_ADDR + 8, f64::INFINITY);

    run_until_hlt(&mut vcpu).unwrap();
    let result = read_f64(&mem, 0x3000);
    assert!(result.is_infinite() && result.is_sign_negative());
}

#[test]
fn test_fsub_nan_propagation() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDC, 0x24, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD,
        0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, f64::NAN);
    write_f64(&mem, DATA_ADDR + 8, 5.0);

    run_until_hlt(&mut vcpu).unwrap();
    assert!(read_f64(&mem, 0x3000).is_nan());
}

#[test]
fn test_fsub_zero_handling() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDC, 0x24, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD,
        0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 5.0);
    write_f64(&mem, DATA_ADDR + 8, 0.0);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 5.0);
}

#[test]
fn test_fsub_negative_zero() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDC, 0x24, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD,
        0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, -0.0);
    write_f64(&mem, DATA_ADDR + 8, -0.0);

    run_until_hlt(&mut vcpu).unwrap();
    let result = read_f64(&mem, 0x3000);
    assert_eq!(result, 0.0);
}

#[test]
fn test_fsubr_vs_fsub() {
    // Verify FSUBR reverses the operands
    let code1 = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDC, 0x24, 0x25, 0x08, 0x20, 0x00,
        0x00, // FSUB
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let code2 = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDC, 0x2C, 0x25, 0x00, 0x20, 0x00,
        0x00, // FSUBR
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];

    let (mut vcpu1, mem1) = setup_vm(&code1, None);
    write_f64(&mem1, DATA_ADDR, 10.0);
    write_f64(&mem1, DATA_ADDR + 8, 3.0);
    run_until_hlt(&mut vcpu1).unwrap();
    let result1 = read_f64(&mem1, 0x3000);

    let (mut vcpu2, mem2) = setup_vm(&code2, None);
    write_f64(&mem2, DATA_ADDR, 10.0);
    write_f64(&mem2, DATA_ADDR + 8, 3.0);
    run_until_hlt(&mut vcpu2).unwrap();
    let result2 = read_f64(&mem2, 0x3000);

    assert_eq!(result1, result2);
}

#[test]
fn test_fsub_chain() {
    // Chain multiple subtractions
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD 100.0
        0xDC, 0x24, 0x25, 0x08, 0x20, 0x00, 0x00, // FSUB 10.0
        0xDC, 0x24, 0x25, 0x10, 0x20, 0x00, 0x00, // FSUB 20.0
        0xDC, 0x24, 0x25, 0x18, 0x20, 0x00, 0x00, // FSUB 30.0
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 100.0);
    write_f64(&mem, DATA_ADDR + 8, 10.0);
    write_f64(&mem, DATA_ADDR + 16, 20.0);
    write_f64(&mem, DATA_ADDR + 24, 30.0);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 40.0);
}

#[test]
fn test_fisub_precision() {
    // Integer subtraction preserves fractional part
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDA, 0x24, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD,
        0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 100.75);
    write_i32(&mem, DATA_ADDR + 8, 50);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 50.75);
}

#[test]
fn test_fisubr_precision() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDA, 0x2C, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD,
        0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 25.25);
    write_i32(&mem, DATA_ADDR + 8, 100);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 74.75); // 100 - 25.25
}

#[test]
fn test_fsub_small_from_large() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDC, 0x24, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD,
        0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 1.0e100);
    write_f64(&mem, DATA_ADDR + 8, 1.0);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 1.0e100);
}

#[test]
fn test_fsubp_stack_behavior() {
    // Verify FSUBP pops
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD 10.0
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD 20.0
        0xDD, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, // FLD 30.0
        0xDE, 0xE9, // FSUBP ST(1), ST(0) ; ST(1) = 20.0 - 30.0, pop
        0xDE, 0xE9, // FSUBP ST(1), ST(0) ; ST(1) = 10.0 - (-10.0), pop
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 10.0);
    write_f64(&mem, DATA_ADDR + 8, 20.0);
    write_f64(&mem, DATA_ADDR + 16, 30.0);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 20.0); // 10.0 - (-10.0)
}

#[test]
fn test_fsubrp_stack_behavior() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD 5.0
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD 15.0
        0xDE, 0xE1, // FSUBRP ST(1), ST(0) ; ST(1) = 15.0 - 5.0, pop
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 5.0);
    write_f64(&mem, DATA_ADDR + 8, 15.0);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 10.0);
}

#[test]
fn test_fsub_alternating_signs() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDC, 0x24, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD,
        0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, -10.5);
    write_f64(&mem, DATA_ADDR + 8, -5.5);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), -5.0);
}

#[test]
fn test_fsubr_symmetry() {
    // FSUBR should give negative of FSUB when swapping operands
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDC, 0x2C, 0x25, 0x08, 0x20, 0x00,
        0x00, // FSUBR
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 7.5);
    write_f64(&mem, DATA_ADDR + 8, 12.5);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 5.0); // 12.5 - 7.5
}

// ============================================================================
// Known-answer operand-order tests for FSUB / FSUBR.
//
// Operand order is the classic place a reverse-subtract implementation goes
// wrong, so these pin down EXACT results for every encoding:
//   FSUB  ST(0),ST(i): ST(0) = ST(0) - ST(i)
//   FSUB  ST(i),ST(0): ST(i) = ST(i) - ST(0)
//   FSUBR ST(0),ST(i): ST(0) = ST(i) - ST(0)
//   FSUBR ST(i),ST(0): ST(i) = ST(0) - ST(i)
//   FSUB  m:           ST(0) = ST(0) - m
//   FSUBR m:           ST(0) = m - ST(0)
// ============================================================================

#[test]
fn test_fsub_st0_sti_exact_order() {
    // FLD 10.0, FLD 3.0  -> ST(0)=3.0, ST(1)=10.0
    // FSUB ST(0),ST(1) -> ST(0) = 3.0 - 10.0 = -7.0
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, 0xD8,
        0xE1, // FSUB ST(0), ST(1)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 10.0);
    write_f64(&mem, DATA_ADDR + 8, 3.0);
    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), -7.0);
}

#[test]
fn test_fsubr_st0_sti_exact_order() {
    // FLD 10.0, FLD 3.0 -> ST(0)=3.0, ST(1)=10.0
    // FSUBR ST(0),ST(1) -> ST(0) = ST(1) - ST(0) = 10.0 - 3.0 = 7.0
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, 0xD8,
        0xE9, // FSUBR ST(0), ST(1)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 10.0);
    write_f64(&mem, DATA_ADDR + 8, 3.0);
    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 7.0);
}

#[test]
fn test_fsub_sti_st0_exact_order() {
    // FLD 10.0, FLD 3.0 -> ST(0)=3.0, ST(1)=10.0
    // FSUB ST(1),ST(0) -> ST(1) = ST(1) - ST(0) = 10.0 - 3.0 = 7.0
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDC,
        0xE9, // FSUB ST(1), ST(0)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // pop ST(0)=3.0
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, // pop ST(1)=7.0
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 10.0);
    write_f64(&mem, DATA_ADDR + 8, 3.0);
    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 3.0);
    assert_eq!(read_f64(&mem, 0x3008), 7.0);
}

#[test]
fn test_fsubr_sti_st0_exact_order() {
    // FLD 10.0, FLD 3.0 -> ST(0)=3.0, ST(1)=10.0
    // FSUBR ST(1),ST(0) -> ST(1) = ST(0) - ST(1) = 3.0 - 10.0 = -7.0
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDC,
        0xE1, // FSUBR ST(1), ST(0)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // pop ST(0)=3.0
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, // pop ST(1)=-7.0
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 10.0);
    write_f64(&mem, DATA_ADDR + 8, 3.0);
    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 3.0);
    assert_eq!(read_f64(&mem, 0x3008), -7.0);
}

#[test]
fn test_fsub_m64_exact_order() {
    // FLD 10.0, FSUB [3.0] -> ST(0) = 10.0 - 3.0 = 7.0
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDC, 0x24, 0x25, 0x08, 0x20, 0x00,
        0x00, // FSUB m64
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 10.0);
    write_f64(&mem, DATA_ADDR + 8, 3.0);
    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 7.0);
}

#[test]
fn test_fsubr_m64_exact_order() {
    // FLD 10.0, FSUBR [3.0] -> ST(0) = 3.0 - 10.0 = -7.0
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDC, 0x2C, 0x25, 0x08, 0x20, 0x00,
        0x00, // FSUBR m64
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 10.0);
    write_f64(&mem, DATA_ADDR + 8, 3.0);
    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), -7.0);
}

#[test]
fn test_fsubrp_exact_order() {
    // FLD 10.0, FLD 3.0 -> ST(0)=3.0, ST(1)=10.0
    // FSUBRP ST(1),ST(0) -> ST(1) = ST(0) - ST(1) = 3.0 - 10.0 = -7.0, then pop
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDE,
        0xE1, // FSUBRP ST(1), ST(0)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 10.0);
    write_f64(&mem, DATA_ADDR + 8, 3.0);
    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), -7.0);
}
