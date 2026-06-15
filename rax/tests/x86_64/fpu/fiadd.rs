//! Tests for the FIADD instruction.
//!
//! FIADD - Add integer to floating-point (m16int and m32int)
//!
//! Reference: /Users/int/dev/rax/docs/fadd:faddp:fiadd.txt
//!
//! Opcode: DE /0 - FIADD m16int
//! Opcode: DA /0 - FIADD m32int

use crate::common::*;
use std::sync::Arc;
use vm_memory::{Bytes, GuestAddress, GuestMemoryMmap};

const DATA_ADDR: u64 = 0x2000;

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
// FIADD m16int (opcode DE /0)
// ============================================================================

#[test]
fn test_fiadd_m16int_positive_basic() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xDE, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FIADD word [0x2008]
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xf4, // HLT
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

#[test]
fn test_fiadd_m16int_result_zero() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDE, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD,
        0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, -100.0);
    write_i16(&mem, DATA_ADDR + 8, 100);

    run_until_hlt(&mut vcpu).unwrap();
    let result = read_f64(&mem, 0x3000);
    assert_eq!(result, 0.0);
    assert!(!result.is_sign_negative());
}

#[test]
fn test_fiadd_m16int_fractional_base() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDE, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD,
        0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 0.123);
    write_i16(&mem, DATA_ADDR + 8, 5);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 5.123);
}

#[test]
fn test_fiadd_m16int_large_positive() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDE, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD,
        0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 30000.0);
    write_i16(&mem, DATA_ADDR + 8, 2000);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 32000.0);
}

#[test]
fn test_fiadd_m16int_large_negative() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDE, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD,
        0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, -30000.0);
    write_i16(&mem, DATA_ADDR + 8, -2000);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), -32000.0);
}

// ============================================================================
// FIADD m32int (opcode DA /0)
// ============================================================================

#[test]
fn test_fiadd_m32int_positive_basic() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDA, 0x04, 0x25, 0x08, 0x20, 0x00,
        0x00, // FIADD dword [0x2008]
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
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
fn test_fiadd_m32int_zero() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDA, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD,
        0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 999.999);
    write_i32(&mem, DATA_ADDR + 8, 0);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 999.999);
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

#[test]
fn test_fiadd_m32int_max() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDA, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD,
        0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 0.75);
    write_i32(&mem, DATA_ADDR + 8, i32::MAX);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), i32::MAX as f64 + 0.75);
}

#[test]
fn test_fiadd_m32int_min() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDA, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD,
        0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 0.75);
    write_i32(&mem, DATA_ADDR + 8, i32::MIN);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), i32::MIN as f64 + 0.75);
}

#[test]
fn test_fiadd_m32int_result_zero() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDA, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD,
        0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, -50000.0);
    write_i32(&mem, DATA_ADDR + 8, 50000);

    run_until_hlt(&mut vcpu).unwrap();
    let result = read_f64(&mem, 0x3000);
    assert_eq!(result, 0.0);
    assert!(!result.is_sign_negative());
}

#[test]
fn test_fiadd_m32int_billion() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDA, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD,
        0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 0.5);
    write_i32(&mem, DATA_ADDR + 8, 1_000_000_000);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 1_000_000_000.5);
}

// ============================================================================
// Multiple operations and chain tests
// ============================================================================

#[test]
fn test_fiadd_multiple_m16() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD 1.5
        0xDE, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FIADD 10
        0xDE, 0x04, 0x25, 0x0A, 0x20, 0x00, 0x00, // FIADD 20
        0xDE, 0x04, 0x25, 0x0C, 0x20, 0x00, 0x00, // FIADD 30
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 1.5);
    write_i16(&mem, DATA_ADDR + 8, 10);
    write_i16(&mem, DATA_ADDR + 10, 20);
    write_i16(&mem, DATA_ADDR + 12, 30);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 61.5);
}

#[test]
fn test_fiadd_multiple_m32() {
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

#[test]
fn test_fiadd_mixed_m16_m32() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD 1.0
        0xDE, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FIADD word 100
        0xDA, 0x04, 0x25, 0x0A, 0x20, 0x00, 0x00, // FIADD dword 1000
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 1.0);
    write_i16(&mem, DATA_ADDR + 8, 100);
    write_i32(&mem, DATA_ADDR + 10, 1000);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 1101.0);
}

#[test]
fn test_fiadd_alternating_signs() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD 0.0
        0xDA, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FIADD 100
        0xDA, 0x04, 0x25, 0x0C, 0x20, 0x00, 0x00, // FIADD -50
        0xDA, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, // FIADD 75
        0xDA, 0x04, 0x25, 0x14, 0x20, 0x00, 0x00, // FIADD -25
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 0.0);
    write_i32(&mem, DATA_ADDR + 8, 100);
    write_i32(&mem, DATA_ADDR + 12, -50);
    write_i32(&mem, DATA_ADDR + 16, 75);
    write_i32(&mem, DATA_ADDR + 20, -25);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 100.0);
}

// ============================================================================
// Special cases with infinity
// ============================================================================

#[test]
fn test_fiadd_m32_to_infinity() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDA, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD,
        0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, f64::INFINITY);
    write_i32(&mem, DATA_ADDR + 8, 1000);

    run_until_hlt(&mut vcpu).unwrap();
    let result = read_f64(&mem, 0x3000);
    assert!(result.is_infinite() && result.is_sign_positive());
}

#[test]
fn test_fiadd_m32_to_neg_infinity() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDA, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD,
        0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, f64::NEG_INFINITY);
    write_i32(&mem, DATA_ADDR + 8, 1000);

    run_until_hlt(&mut vcpu).unwrap();
    let result = read_f64(&mem, 0x3000);
    assert!(result.is_infinite() && result.is_sign_negative());
}

#[test]
fn test_fiadd_m16_to_infinity() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDE, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD,
        0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, f64::INFINITY);
    write_i16(&mem, DATA_ADDR + 8, 100);

    run_until_hlt(&mut vcpu).unwrap();
    let result = read_f64(&mem, 0x3000);
    assert!(result.is_infinite() && result.is_sign_positive());
}

// ============================================================================
// Precision tests
// ============================================================================

#[test]
fn test_fiadd_precision_small_int_to_large_float() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDA, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD,
        0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 1.0e100);
    write_i32(&mem, DATA_ADDR + 8, 1);

    run_until_hlt(&mut vcpu).unwrap();
    let result = read_f64(&mem, 0x3000);
    assert_eq!(result, 1.0e100);
}

#[test]
fn test_fiadd_precision_accumulation() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD 0.1
        0xDA, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FIADD 1
        0xDA, 0x04, 0x25, 0x0C, 0x20, 0x00, 0x00, // FIADD 2
        0xDA, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, // FIADD 3
        0xDA, 0x04, 0x25, 0x14, 0x20, 0x00, 0x00, // FIADD 4
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 0.1);
    write_i32(&mem, DATA_ADDR + 8, 1);
    write_i32(&mem, DATA_ADDR + 12, 2);
    write_i32(&mem, DATA_ADDR + 16, 3);
    write_i32(&mem, DATA_ADDR + 20, 4);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 10.1);
}

// ============================================================================
// Boundary and edge cases
// ============================================================================

#[test]
fn test_fiadd_m16_one() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDE, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD,
        0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 0.0);
    write_i16(&mem, DATA_ADDR + 8, 1);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 1.0);
}

#[test]
fn test_fiadd_m32_one() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDA, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD,
        0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 0.0);
    write_i32(&mem, DATA_ADDR + 8, 1);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 1.0);
}

#[test]
fn test_fiadd_m16_negative_one() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDE, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD,
        0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 0.0);
    write_i16(&mem, DATA_ADDR + 8, -1);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), -1.0);
}

#[test]
fn test_fiadd_m32_negative_one() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDA, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD,
        0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 0.0);
    write_i32(&mem, DATA_ADDR + 8, -1);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), -1.0);
}

#[test]
fn test_fiadd_very_small_float_with_large_int() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDA, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD,
        0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 1.0e-100);
    write_i32(&mem, DATA_ADDR + 8, 1000000);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 1000000.0);
}

#[test]
fn test_fiadd_negative_float_positive_int() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDA, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD,
        0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, -3.5);
    write_i32(&mem, DATA_ADDR + 8, 10);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 6.5);
}

#[test]
fn test_fiadd_positive_float_negative_int() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDA, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD,
        0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 10.5);
    write_i32(&mem, DATA_ADDR + 8, -3);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 7.5);
}

#[test]
fn test_fiadd_commutative_with_int() {
    // 5.5 + 10 should equal conceptually the same as 10 + 5.5
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDA, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD,
        0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 5.5);
    write_i32(&mem, DATA_ADDR + 8, 10);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 15.5);
}
