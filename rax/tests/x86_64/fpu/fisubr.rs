//! Tests for the FISUBR instruction.
//!
//! FISUBR - Reverse subtract integer from floating-point (m16int and m32int)
//!
//! Reference: /Users/int/dev/rax/docs/fsubr:fsubrp:fisubr.txt
//!
//! Opcode: DE /5 - FISUBR m16int  ; ST(0) = m16int - ST(0)
//! Opcode: DA /5 - FISUBR m32int  ; ST(0) = m32int - ST(0)

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
// FISUBR m16int (opcode DE /5) - ST(0) = m16int - ST(0)
// ============================================================================

#[test]
fn test_fisubr_m16int_basic() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xDE, 0x2C, 0x25, 0x08, 0x20, 0x00, 0x00, // FISUBR word [0x2008]
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 2.5); // ST(0)
    write_i16(&mem, DATA_ADDR + 8, 10); // Integer operand

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 7.5); // 10 - 2.5 = 7.5
}

#[test]
fn test_fisubr_m16int_negative_result() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDE, 0x2C, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD,
        0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 15.0);
    write_i16(&mem, DATA_ADDR + 8, 5);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), -10.0); // 5 - 15 = -10
}

#[test]
fn test_fisubr_m16int_negative_operand() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDE, 0x2C, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD,
        0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 5.0);
    write_i16(&mem, DATA_ADDR + 8, -10);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), -15.0); // -10 - 5 = -15
}

#[test]
fn test_fisubr_m16int_zero() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDE, 0x2C, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD,
        0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 3.14159);
    write_i16(&mem, DATA_ADDR + 8, 0);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), -3.14159);
}

#[test]
fn test_fisubr_m16int_result_zero() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDE, 0x2C, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD,
        0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 100.0);
    write_i16(&mem, DATA_ADDR + 8, 100);

    run_until_hlt(&mut vcpu).unwrap();
    let result = read_f64(&mem, 0x3000);
    assert_eq!(result, 0.0);
    assert!(!result.is_sign_negative());
}

#[test]
fn test_fisubr_m16int_max() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDE, 0x2C, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD,
        0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 0.5);
    write_i16(&mem, DATA_ADDR + 8, i16::MAX);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), i16::MAX as f64 - 0.5);
}

#[test]
fn test_fisubr_m16int_min() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDE, 0x2C, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD,
        0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 0.5);
    write_i16(&mem, DATA_ADDR + 8, i16::MIN);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), i16::MIN as f64 - 0.5);
}

// ============================================================================
// FISUBR m32int (opcode DA /5) - ST(0) = m32int - ST(0)
// ============================================================================

#[test]
fn test_fisubr_m32int_basic() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDA, 0x2C, 0x25, 0x08, 0x20, 0x00,
        0x00, // FISUBR dword [0x2008]
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 1.5);
    write_i32(&mem, DATA_ADDR + 8, 1000);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 998.5); // 1000 - 1.5
}

#[test]
fn test_fisubr_m32int_negative() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDA, 0x2C, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD,
        0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 150.0);
    write_i32(&mem, DATA_ADDR + 8, -50);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), -200.0); // -50 - 150
}

#[test]
fn test_fisubr_m32int_zero() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDA, 0x2C, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD,
        0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 999.999);
    write_i32(&mem, DATA_ADDR + 8, 0);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), -999.999);
}

#[test]
fn test_fisubr_m32int_large() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDA, 0x2C, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD,
        0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 0.25);
    write_i32(&mem, DATA_ADDR + 8, 1000000);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 999999.75);
}

#[test]
fn test_fisubr_m32int_max() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDA, 0x2C, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD,
        0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 0.75);
    write_i32(&mem, DATA_ADDR + 8, i32::MAX);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), i32::MAX as f64 - 0.75);
}

#[test]
fn test_fisubr_m32int_min() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDA, 0x2C, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD,
        0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 0.75);
    write_i32(&mem, DATA_ADDR + 8, i32::MIN);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), i32::MIN as f64 - 0.75);
}

#[test]
fn test_fisubr_m32int_result_zero() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDA, 0x2C, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD,
        0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 50000.0);
    write_i32(&mem, DATA_ADDR + 8, 50000);

    run_until_hlt(&mut vcpu).unwrap();
    let result = read_f64(&mem, 0x3000);
    assert_eq!(result, 0.0);
    assert!(!result.is_sign_negative());
}

// ============================================================================
// Chain tests
// ============================================================================

#[test]
fn test_fisubr_chain() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD 5.0
        0xDA, 0x2C, 0x25, 0x08, 0x20, 0x00, 0x00, // FISUBR 100 ; ST(0) = 95
        0xDA, 0x2C, 0x25, 0x0C, 0x20, 0x00, 0x00, // FISUBR 200 ; ST(0) = 105
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 5.0);
    write_i32(&mem, DATA_ADDR + 8, 100);
    write_i32(&mem, DATA_ADDR + 12, 200);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 105.0); // 200 - (100 - 5)
}

// ============================================================================
// Special cases
// ============================================================================

#[test]
fn test_fisubr_from_infinity() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDA, 0x2C, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD,
        0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, f64::INFINITY);
    write_i32(&mem, DATA_ADDR + 8, 1000);

    run_until_hlt(&mut vcpu).unwrap();
    let result = read_f64(&mem, 0x3000);
    assert!(result.is_infinite() && result.is_sign_negative());
}

#[test]
fn test_fisubr_from_neg_infinity() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDA, 0x2C, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD,
        0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, f64::NEG_INFINITY);
    write_i32(&mem, DATA_ADDR + 8, 1000);

    run_until_hlt(&mut vcpu).unwrap();
    let result = read_f64(&mem, 0x3000);
    assert!(result.is_infinite() && result.is_sign_positive());
}
