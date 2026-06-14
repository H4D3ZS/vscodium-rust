//! Tests for the FADD, FADDP, and FIADD instructions.
//!
//! FADD - Add floating-point
//! FADDP - Add floating-point and pop
//! FIADD - Add integer to floating-point
//!
//! Reference: /Users/int/dev/rax/docs/fadd:faddp:fiadd.txt

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
// FADD m32fp (opcode D8 /0)
// ============================================================================

#[test]
fn test_fadd_m32fp_basic() {
    // FLD qword ptr [0x2000]  ; Load 2.0
    // FADD dword ptr [0x2008] ; Add 3.0
    // FSTP qword ptr [0x3000]
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xD8, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD,
        0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 2.0);
    write_f32(&mem, DATA_ADDR + 8, 3.0);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 5.0);
}

#[test]
fn test_fadd_m32fp_zero() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xD8, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD,
        0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 5.5);
    write_f32(&mem, DATA_ADDR + 8, 0.0);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 5.5);
}

#[test]
fn test_fadd_m32fp_negative() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xD8, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD,
        0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 10.0);
    write_f32(&mem, DATA_ADDR + 8, -3.0);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 7.0);
}

#[test]
fn test_fadd_m32fp_result_zero() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xD8, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD,
        0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 5.0);
    write_f32(&mem, DATA_ADDR + 8, -5.0);

    run_until_hlt(&mut vcpu).unwrap();
    let result = read_f64(&mem, 0x3000);
    assert_eq!(result, 0.0);
    assert!(!result.is_sign_negative());
}

// ============================================================================
// FADD m64fp (opcode DC /0)
// ============================================================================

#[test]
fn test_fadd_m64fp_basic() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDC, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD,
        0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 1.5);
    write_f64(&mem, DATA_ADDR + 8, 2.5);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 4.0);
}

#[test]
fn test_fadd_m64fp_large_values() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDC, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD,
        0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 1.0e15);
    write_f64(&mem, DATA_ADDR + 8, 2.0e15);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 3.0e15);
}

#[test]
fn test_fadd_m64fp_small_values() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDC, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD,
        0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 1.0e-15);
    write_f64(&mem, DATA_ADDR + 8, 2.0e-15);

    run_until_hlt(&mut vcpu).unwrap();
    let result = read_f64(&mem, 0x3000);
    assert!((result - 3.0e-15).abs() < 1e-30);
}

#[test]
fn test_fadd_m64fp_pi_e() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDC, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD,
        0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, std::f64::consts::PI);
    write_f64(&mem, DATA_ADDR + 8, std::f64::consts::E);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        read_f64(&mem, 0x3000),
        std::f64::consts::PI + std::f64::consts::E
    );
}

// ============================================================================
// FADD ST(0), ST(i) (opcode D8 C0+i)
// ============================================================================

#[test]
fn test_fadd_st0_st1() {
    // FLD 2.0, FLD 3.0, FADD ST(0), ST(1) -> ST(0) = 5.0
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, 0xD8,
        0xC1, // FADD ST(0), ST(1)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 2.0);
    write_f64(&mem, DATA_ADDR + 8, 3.0);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 5.0);
}

#[test]
fn test_fadd_st0_st2() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD,
        0x04, 0x25, 0x10, 0x20, 0x00, 0x00, 0xD8, 0xC2, // FADD ST(0), ST(2)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 1.0);
    write_f64(&mem, DATA_ADDR + 8, 2.0);
    write_f64(&mem, DATA_ADDR + 16, 3.0);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 4.0); // 3.0 + 1.0
}

// ============================================================================
// FADD ST(i), ST(0) (opcode DC C0+i)
// ============================================================================

#[test]
fn test_fadd_st1_st0() {
    // FLD 2.0, FLD 3.0, FADD ST(1), ST(0) -> ST(1) = 5.0
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDC,
        0xC1, // FADD ST(1), ST(0)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // Pop ST(0)
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, // Pop ST(0)
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 2.0);
    write_f64(&mem, DATA_ADDR + 8, 3.0);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 3.0); // ST(0) unchanged
    assert_eq!(read_f64(&mem, 0x3008), 5.0); // ST(1) = 2.0 + 3.0
}

// ============================================================================
// FADDP ST(i), ST(0) (opcode DE C0+i)
// ============================================================================

#[test]
fn test_faddp_st1_st0() {
    // FADDP adds and pops
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDE,
        0xC1, // FADDP ST(1), ST(0)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 4.0);
    write_f64(&mem, DATA_ADDR + 8, 6.0);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 10.0);
}

#[test]
fn test_faddp_no_operand() {
    // FADDP with no operand = FADDP ST(1), ST(0)
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDE,
        0xC1, // FADDP
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 7.5);
    write_f64(&mem, DATA_ADDR + 8, 2.5);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 10.0);
}

// ============================================================================
// FIADD m16int (opcode DE /0)
// ============================================================================

#[test]
fn test_fiadd_m16int_positive() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDE, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD,
        0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 2.5);
    write_i16(&mem, DATA_ADDR + 8, 10);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 12.5);
}

#[test]
fn test_fiadd_m16int_negative() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDE, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD,
        0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 10.0);
    write_i16(&mem, DATA_ADDR + 8, -5);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 5.0);
}

#[test]
fn test_fiadd_m16int_zero() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDE, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD,
        0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 3.14159);
    write_i16(&mem, DATA_ADDR + 8, 0);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 3.14159);
}

#[test]
fn test_fiadd_m16int_max() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDE, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD,
        0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 0.5);
    write_i16(&mem, DATA_ADDR + 8, i16::MAX);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), i16::MAX as f64 + 0.5);
}

#[test]
fn test_fiadd_m16int_min() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDE, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD,
        0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 0.5);
    write_i16(&mem, DATA_ADDR + 8, i16::MIN);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), i16::MIN as f64 + 0.5);
}

// ============================================================================
// FIADD m32int (opcode DA /0)
// ============================================================================

#[test]
fn test_fiadd_m32int_positive() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDA, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD,
        0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 1.5);
    write_i32(&mem, DATA_ADDR + 8, 1000);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 1001.5);
}

#[test]
fn test_fiadd_m32int_negative() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDA, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD,
        0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 100.0);
    write_i32(&mem, DATA_ADDR + 8, -50);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 50.0);
}

#[test]
fn test_fiadd_m32int_large() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDA, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD,
        0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 0.25);
    write_i32(&mem, DATA_ADDR + 8, 1000000);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 1000000.25);
}

// ============================================================================
// Special cases
// ============================================================================

#[test]
fn test_fadd_infinity_finite() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDC, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD,
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
fn test_fadd_neg_infinity_finite() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDC, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD,
        0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, f64::NEG_INFINITY);
    write_f64(&mem, DATA_ADDR + 8, 100.0);

    run_until_hlt(&mut vcpu).unwrap();
    let result = read_f64(&mem, 0x3000);
    assert!(result.is_infinite() && result.is_sign_negative());
}

#[test]
fn test_fadd_same_infinity() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDC, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD,
        0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, f64::INFINITY);
    write_f64(&mem, DATA_ADDR + 8, f64::INFINITY);

    run_until_hlt(&mut vcpu).unwrap();
    let result = read_f64(&mem, 0x3000);
    assert!(result.is_infinite() && result.is_sign_positive());
}

#[test]
fn test_fadd_nan_propagation() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDC, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD,
        0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, f64::NAN);
    write_f64(&mem, DATA_ADDR + 8, 5.0);

    run_until_hlt(&mut vcpu).unwrap();
    assert!(read_f64(&mem, 0x3000).is_nan());
}

#[test]
fn test_fadd_zero_plus_zero() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDC, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD,
        0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 0.0);
    write_f64(&mem, DATA_ADDR + 8, 0.0);

    run_until_hlt(&mut vcpu).unwrap();
    let result = read_f64(&mem, 0x3000);
    assert_eq!(result, 0.0);
    assert!(!result.is_sign_negative());
}

#[test]
fn test_fadd_negative_zero_handling() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDC, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD,
        0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 0.0);
    write_f64(&mem, DATA_ADDR + 8, -0.0);

    run_until_hlt(&mut vcpu).unwrap();
    let result = read_f64(&mem, 0x3000);
    assert_eq!(result, 0.0);
}

// ============================================================================
// Precision tests
// ============================================================================

#[test]
fn test_fadd_precision_accumulation() {
    // Add multiple small values
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDC, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDC,
        0x04, 0x25, 0x10, 0x20, 0x00, 0x00, 0xDC, 0x04, 0x25, 0x18, 0x20, 0x00, 0x00, 0xDD, 0x1C,
        0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 0.1);
    write_f64(&mem, DATA_ADDR + 8, 0.2);
    write_f64(&mem, DATA_ADDR + 16, 0.3);
    write_f64(&mem, DATA_ADDR + 24, 0.4);

    run_until_hlt(&mut vcpu).unwrap();
    let result = read_f64(&mem, 0x3000);
    assert!((result - 1.0).abs() < 1e-10);
}

#[test]
fn test_fadd_mixed_magnitude() {
    // Large + small
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDC, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD,
        0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 1.0e100);
    write_f64(&mem, DATA_ADDR + 8, 1.0);

    run_until_hlt(&mut vcpu).unwrap();
    let result = read_f64(&mem, 0x3000);
    assert_eq!(result, 1.0e100);
}

#[test]
fn test_fadd_commutative() {
    // a + b should equal b + a
    let code1 = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDC, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD,
        0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let code2 = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDC, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDD,
        0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];

    let (mut vcpu1, mem1) = setup_vm(&code1, None);
    write_f64(&mem1, DATA_ADDR, 123.456);
    write_f64(&mem1, DATA_ADDR + 8, 789.012);
    run_until_hlt(&mut vcpu1).unwrap();
    let result1 = read_f64(&mem1, 0x3000);

    let (mut vcpu2, mem2) = setup_vm(&code2, None);
    write_f64(&mem2, DATA_ADDR, 123.456);
    write_f64(&mem2, DATA_ADDR + 8, 789.012);
    run_until_hlt(&mut vcpu2).unwrap();
    let result2 = read_f64(&mem2, 0x3000);

    assert_eq!(result1, result2);
}

#[test]
fn test_fadd_associative_property() {
    // Test (a + b) + c
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDC, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDC,
        0x04, 0x25, 0x10, 0x20, 0x00, 0x00, 0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 1.0);
    write_f64(&mem, DATA_ADDR + 8, 2.0);
    write_f64(&mem, DATA_ADDR + 16, 3.0);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 6.0);
}

#[test]
fn test_fadd_chain_operations() {
    // Chain multiple FADD operations
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD 1.0
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD 2.0
        0xDE, 0xC1, // FADDP
        0xDD, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, // FLD 3.0
        0xDE, 0xC1, // FADDP
        0xDD, 0x04, 0x25, 0x18, 0x20, 0x00, 0x00, // FLD 4.0
        0xDE, 0xC1, // FADDP
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 1.0);
    write_f64(&mem, DATA_ADDR + 8, 2.0);
    write_f64(&mem, DATA_ADDR + 16, 3.0);
    write_f64(&mem, DATA_ADDR + 24, 4.0);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 10.0);
}

#[test]
fn test_fadd_stack_preservation() {
    // Verify FADD ST(i), ST(0) preserves ST(0)
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD 5.0
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD 3.0
        0xDC, 0xC1, // FADD ST(1), ST(0) ; ST(1) = 8.0, ST(0) = 3.0
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP [0x3000] ; 3.0
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, // FSTP [0x3008] ; 8.0
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 5.0);
    write_f64(&mem, DATA_ADDR + 8, 3.0);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 3.0);
    assert_eq!(read_f64(&mem, 0x3008), 8.0);
}

#[test]
fn test_fiadd_multiple_integers() {
    // Add multiple integers to floating point
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD 1.5
        0xDA, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FIADD 10
        0xDA, 0x04, 0x25, 0x0C, 0x20, 0x00, 0x00, // FIADD 20
        0xDA, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, // FIADD 30
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 1.5);
    write_i32(&mem, DATA_ADDR + 8, 10);
    write_i32(&mem, DATA_ADDR + 12, 20);
    write_i32(&mem, DATA_ADDR + 16, 30);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 61.5);
}
