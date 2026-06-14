//! Tests for FMUL, FMULP, and FIMUL instructions.
//!
//! FMUL - Multiply floating-point
//! FMULP - Multiply floating-point and pop
//! FIMUL - Multiply integer with floating-point
//!
//! Reference: /Users/int/dev/rax/docs/fmul:fmulp:fimul.txt

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
// FMUL m32fp (opcode D8 /1) - ST(0) = ST(0) * m32fp
// ============================================================================

#[test]
fn test_fmul_m32fp_basic() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xD8, 0x0C, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD,
        0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 2.5);
    write_f32(&mem, DATA_ADDR + 8, 4.0);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 10.0);
}

#[test]
fn test_fmul_m32fp_zero() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xD8, 0x0C, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD,
        0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 5.5);
    write_f32(&mem, DATA_ADDR + 8, 0.0);

    run_until_hlt(&mut vcpu).unwrap();
    let result = read_f64(&mem, 0x3000);
    assert_eq!(result, 0.0);
    assert!(!result.is_sign_negative());
}

#[test]
fn test_fmul_m32fp_negative() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xD8, 0x0C, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD,
        0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 3.0);
    write_f32(&mem, DATA_ADDR + 8, -2.0);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), -6.0);
}

#[test]
fn test_fmul_m32fp_both_negative() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xD8, 0x0C, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD,
        0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, -3.0);
    write_f32(&mem, DATA_ADDR + 8, -2.0);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 6.0);
}

// ============================================================================
// FMUL m64fp (opcode DC /1)
// ============================================================================

#[test]
fn test_fmul_m64fp_basic() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDC, 0x0C, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD,
        0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 1.5);
    write_f64(&mem, DATA_ADDR + 8, 2.0);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 3.0);
}

#[test]
fn test_fmul_m64fp_large() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDC, 0x0C, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD,
        0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 1.0e10);
    write_f64(&mem, DATA_ADDR + 8, 1.0e5);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 1.0e15);
}

#[test]
fn test_fmul_m64fp_small() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDC, 0x0C, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD,
        0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 1.0e-10);
    write_f64(&mem, DATA_ADDR + 8, 1.0e-5);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 1.0e-15);
}

// ============================================================================
// FMUL ST(0), ST(i) (opcode D8 C8+i)
// ============================================================================

#[test]
fn test_fmul_st0_st1() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, 0xD8,
        0xC9, // FMUL ST(0), ST(1)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 3.0);
    write_f64(&mem, DATA_ADDR + 8, 4.0);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 12.0);
}

#[test]
fn test_fmul_st0_st2() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD,
        0x04, 0x25, 0x10, 0x20, 0x00, 0x00, 0xD8, 0xCA, // FMUL ST(0), ST(2)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 2.0);
    write_f64(&mem, DATA_ADDR + 8, 3.0);
    write_f64(&mem, DATA_ADDR + 16, 5.0);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 10.0); // 5.0 * 2.0
}

// ============================================================================
// FMUL ST(i), ST(0) (opcode DC C8+i)
// ============================================================================

#[test]
fn test_fmul_st1_st0() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDC,
        0xC9, // FMUL ST(1), ST(0)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 2.0);
    write_f64(&mem, DATA_ADDR + 8, 7.0);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 7.0); // ST(0) unchanged
    assert_eq!(read_f64(&mem, 0x3008), 14.0); // ST(1) = 2.0 * 7.0
}

// ============================================================================
// FMULP ST(i), ST(0) (opcode DE C8+i)
// ============================================================================

#[test]
fn test_fmulp_st1_st0() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDE,
        0xC9, // FMULP ST(1), ST(0)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 3.0);
    write_f64(&mem, DATA_ADDR + 8, 5.0);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 15.0);
}

#[test]
fn test_fmulp_no_operand() {
    // FMULP with no operand = FMULP ST(1), ST(0)
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDE,
        0xC9, 0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 4.0);
    write_f64(&mem, DATA_ADDR + 8, 2.5);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 10.0);
}

// ============================================================================
// FIMUL m16int (opcode DE /1)
// ============================================================================

#[test]
fn test_fimul_m16int_positive() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDE, 0x0C, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD,
        0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 2.5);
    write_i16(&mem, DATA_ADDR + 8, 4);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 10.0);
}

#[test]
fn test_fimul_m16int_negative() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDE, 0x0C, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD,
        0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 5.0);
    write_i16(&mem, DATA_ADDR + 8, -3);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), -15.0);
}

#[test]
fn test_fimul_m16int_zero() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDE, 0x0C, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD,
        0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 123.456);
    write_i16(&mem, DATA_ADDR + 8, 0);

    run_until_hlt(&mut vcpu).unwrap();
    let result = read_f64(&mem, 0x3000);
    assert_eq!(result, 0.0);
    assert!(!result.is_sign_negative());
}

// ============================================================================
// FIMUL m32int (opcode DA /1)
// ============================================================================

#[test]
fn test_fimul_m32int_positive() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDA, 0x0C, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD,
        0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 1.5);
    write_i32(&mem, DATA_ADDR + 8, 100);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 150.0);
}

#[test]
fn test_fimul_m32int_negative() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDA, 0x0C, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD,
        0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 2.5);
    write_i32(&mem, DATA_ADDR + 8, -40);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), -100.0);
}

#[test]
fn test_fimul_m32int_large() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDA, 0x0C, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD,
        0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 0.5);
    write_i32(&mem, DATA_ADDR + 8, 1000000);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 500000.0);
}

// ============================================================================
// Special cases
// ============================================================================

#[test]
fn test_fmul_infinity_finite() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDC, 0x0C, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD,
        0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, f64::INFINITY);
    write_f64(&mem, DATA_ADDR + 8, 2.0);

    run_until_hlt(&mut vcpu).unwrap();
    let result = read_f64(&mem, 0x3000);
    assert!(result.is_infinite() && result.is_sign_positive());
}

#[test]
fn test_fmul_neg_infinity_finite() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDC, 0x0C, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD,
        0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, f64::NEG_INFINITY);
    write_f64(&mem, DATA_ADDR + 8, 2.0);

    run_until_hlt(&mut vcpu).unwrap();
    let result = read_f64(&mem, 0x3000);
    assert!(result.is_infinite() && result.is_sign_negative());
}

#[test]
fn test_fmul_infinity_negative() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDC, 0x0C, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD,
        0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, f64::INFINITY);
    write_f64(&mem, DATA_ADDR + 8, -2.0);

    run_until_hlt(&mut vcpu).unwrap();
    let result = read_f64(&mem, 0x3000);
    assert!(result.is_infinite() && result.is_sign_negative());
}

#[test]
fn test_fmul_nan_propagation() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDC, 0x0C, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD,
        0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, f64::NAN);
    write_f64(&mem, DATA_ADDR + 8, 5.0);

    run_until_hlt(&mut vcpu).unwrap();
    assert!(read_f64(&mem, 0x3000).is_nan());
}

#[test]
fn test_fmul_sign_rules() {
    // Test sign XOR: pos * pos = pos, pos * neg = neg, neg * neg = pos
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDC, 0x0C, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD,
        0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];

    // Positive * Positive
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 3.0);
    write_f64(&mem, DATA_ADDR + 8, 2.0);
    run_until_hlt(&mut vcpu).unwrap();
    let result = read_f64(&mem, 0x3000);
    assert_eq!(result, 6.0);
    assert!(!result.is_sign_negative());

    // Negative * Negative
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, -3.0);
    write_f64(&mem, DATA_ADDR + 8, -2.0);
    run_until_hlt(&mut vcpu).unwrap();
    let result = read_f64(&mem, 0x3000);
    assert_eq!(result, 6.0);
    assert!(!result.is_sign_negative());
}

#[test]
fn test_fmul_zero_sign() {
    // 0.0 * x should preserve sign based on XOR rule
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDC, 0x0C, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD,
        0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 0.0);
    write_f64(&mem, DATA_ADDR + 8, 5.0);

    run_until_hlt(&mut vcpu).unwrap();
    let result = read_f64(&mem, 0x3000);
    assert_eq!(result, 0.0);
    assert!(!result.is_sign_negative());
}

#[test]
fn test_fmul_negative_zero() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDC, 0x0C, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD,
        0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, -0.0);
    write_f64(&mem, DATA_ADDR + 8, 5.0);

    run_until_hlt(&mut vcpu).unwrap();
    let result = read_f64(&mem, 0x3000);
    assert!(result == 0.0 && result.is_sign_negative());
}

#[test]
fn test_fmul_commutative() {
    // a * b should equal b * a
    let code1 = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDC, 0x0C, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD,
        0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let code2 = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDC, 0x0C, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDD,
        0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];

    let (mut vcpu1, mem1) = setup_vm(&code1, None);
    write_f64(&mem1, DATA_ADDR, 7.5);
    write_f64(&mem1, DATA_ADDR + 8, 3.2);
    run_until_hlt(&mut vcpu1).unwrap();
    let result1 = read_f64(&mem1, 0x3000);

    let (mut vcpu2, mem2) = setup_vm(&code2, None);
    write_f64(&mem2, DATA_ADDR, 7.5);
    write_f64(&mem2, DATA_ADDR + 8, 3.2);
    run_until_hlt(&mut vcpu2).unwrap();
    let result2 = read_f64(&mem2, 0x3000);

    assert_eq!(result1, result2);
}

#[test]
fn test_fmul_associative() {
    // Test (a * b) * c
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDC, 0x0C, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDC,
        0x0C, 0x25, 0x10, 0x20, 0x00, 0x00, 0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 2.0);
    write_f64(&mem, DATA_ADDR + 8, 3.0);
    write_f64(&mem, DATA_ADDR + 16, 4.0);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 24.0);
}

#[test]
fn test_fmul_by_one() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDC, 0x0C, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD,
        0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 123.456);
    write_f64(&mem, DATA_ADDR + 8, 1.0);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 123.456);
}

#[test]
fn test_fmul_chain() {
    // Chain multiple multiplications
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD 2.0
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD 3.0
        0xDE, 0xC9, // FMULP
        0xDD, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, // FLD 4.0
        0xDE, 0xC9, // FMULP
        0xDD, 0x04, 0x25, 0x18, 0x20, 0x00, 0x00, // FLD 5.0
        0xDE, 0xC9, // FMULP
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 2.0);
    write_f64(&mem, DATA_ADDR + 8, 3.0);
    write_f64(&mem, DATA_ADDR + 16, 4.0);
    write_f64(&mem, DATA_ADDR + 24, 5.0);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 120.0); // 2*3*4*5
}

#[test]
fn test_fimul_preserves_fraction() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDA, 0x0C, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD,
        0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 2.5);
    write_i32(&mem, DATA_ADDR + 8, 3);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 7.5);
}

#[test]
fn test_fmul_small_large() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDC, 0x0C, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD,
        0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 1.0e-100);
    write_f64(&mem, DATA_ADDR + 8, 1.0e100);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 1.0);
}

#[test]
fn test_fmulp_stack_behavior() {
    // Verify FMULP pops the stack
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD 2.0
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD 3.0
        0xDD, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, // FLD 4.0
        0xDE, 0xC9, // FMULP ST(1), ST(0) ; ST(1) = 3.0 * 4.0, pop
        0xDE, 0xC9, // FMULP ST(1), ST(0) ; ST(1) = 2.0 * 12.0, pop
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 2.0);
    write_f64(&mem, DATA_ADDR + 8, 3.0);
    write_f64(&mem, DATA_ADDR + 16, 4.0);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 24.0);
}

#[test]
fn test_fmul_fractions() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDC, 0x0C, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD,
        0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 0.5);
    write_f64(&mem, DATA_ADDR + 8, 0.25);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 0.125);
}

// ============================================================================
// Known-answer exact-product tests for FMUL (register + memory forms).
// All products are exactly representable in f64.
// ============================================================================

#[test]
fn test_fmul_st0_st1_exact() {
    // FLD 6.0, FLD 7.0 -> ST(0)=7.0, ST(1)=6.0; FMUL ST(0),ST(1) -> 42.0
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, 0xD8,
        0xC9, // FMUL ST(0), ST(1)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 6.0);
    write_f64(&mem, DATA_ADDR + 8, 7.0);
    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 42.0);
}

#[test]
fn test_fmul_sti_st0_exact() {
    // FLD 6.0, FLD 7.0 -> ST(0)=7.0, ST(1)=6.0; FMUL ST(1),ST(0) -> ST(1)=42.0
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDC,
        0xC9, // FMUL ST(1), ST(0)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // pop ST(0)=7.0
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, // pop ST(1)=42.0
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 6.0);
    write_f64(&mem, DATA_ADDR + 8, 7.0);
    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 7.0);
    assert_eq!(read_f64(&mem, 0x3008), 42.0);
}

#[test]
fn test_fmul_m64_exact() {
    // FLD 1.5, FMUL [4.0] -> 6.0
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDC, 0x0C, 0x25, 0x08, 0x20, 0x00,
        0x00, // FMUL m64
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 1.5);
    write_f64(&mem, DATA_ADDR + 8, 4.0);
    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 6.0);
}

#[test]
fn test_fmul_by_negative_sign() {
    // 3.0 * -2.0 = -6.0
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDC, 0x0C, 0x25, 0x08, 0x20, 0x00,
        0x00, // FMUL m64
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 3.0);
    write_f64(&mem, DATA_ADDR + 8, -2.0);
    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), -6.0);
}
