//! Tests for FDIV, FDIVP, FIDIV, FDIVR, FDIVRP, and FIDIVR instructions.
//!
//! FDIV - Divide
//! FDIVP - Divide and pop
//! FIDIV - Divide by integer
//! FDIVR - Reverse divide
//! FDIVRP - Reverse divide and pop
//! FIDIVR - Reverse divide by integer
//!
//! References: /Users/int/dev/rax/docs/fdiv:fdivp:fidiv.txt
//!             /Users/int/dev/rax/docs/fdivr:fdivrp:fidivr.txt

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
// FDIV m32fp (opcode D8 /6) - ST(0) = ST(0) / m32fp
// ============================================================================

#[test]
fn test_fdiv_m32fp_basic() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xD8, 0x34, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD,
        0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 10.0);
    write_f32(&mem, DATA_ADDR + 8, 2.0);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 5.0);
}

#[test]
fn test_fdiv_m32fp_fractional_result() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xD8, 0x34, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD,
        0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 10.0);
    write_f32(&mem, DATA_ADDR + 8, 3.0);

    run_until_hlt(&mut vcpu).unwrap();
    let result = read_f64(&mem, 0x3000);
    assert!((result - 3.333333333333333).abs() < 1e-10);
}

#[test]
fn test_fdiv_m32fp_negative_result() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xD8, 0x34, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD,
        0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, -10.0);
    write_f32(&mem, DATA_ADDR + 8, 2.0);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), -5.0);
}

#[test]
fn test_fdiv_m32fp_both_negative() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xD8, 0x34, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD,
        0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, -10.0);
    write_f32(&mem, DATA_ADDR + 8, -2.0);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 5.0);
}

// ============================================================================
// FDIV m64fp (opcode DC /6)
// ============================================================================

#[test]
fn test_fdiv_m64fp_basic() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDC, 0x34, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD,
        0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 100.0);
    write_f64(&mem, DATA_ADDR + 8, 4.0);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 25.0);
}

#[test]
fn test_fdiv_m64fp_precision() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDC, 0x34, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD,
        0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 1.0);
    write_f64(&mem, DATA_ADDR + 8, 3.0);

    run_until_hlt(&mut vcpu).unwrap();
    let result = read_f64(&mem, 0x3000);
    assert!((result - 0.3333333333333333).abs() < 1e-15);
}

// ============================================================================
// FDIV ST(0), ST(i) (opcode D8 F0+i)
// ============================================================================

#[test]
fn test_fdiv_st0_st1() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, 0xD8,
        0xF1, // FDIV ST(0), ST(1)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 4.0);
    write_f64(&mem, DATA_ADDR + 8, 20.0);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 5.0); // 20.0 / 4.0
}

// ============================================================================
// FDIV ST(i), ST(0) (opcode DC F8+i)
// ============================================================================

#[test]
fn test_fdiv_st1_st0() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDC,
        0xF9, // FDIV ST(1), ST(0)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 4.0);
    write_f64(&mem, DATA_ADDR + 8, 20.0);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 20.0); // ST(0) unchanged
    assert_eq!(read_f64(&mem, 0x3008), 0.2); // ST(1) = 4.0 / 20.0
}

// ============================================================================
// FDIVP ST(i), ST(0) (opcode DE F8+i)
// ============================================================================

#[test]
fn test_fdivp_st1_st0() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDE,
        0xF9, // FDIVP ST(1), ST(0)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 5.0);
    write_f64(&mem, DATA_ADDR + 8, 25.0);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 0.2); // 5.0 / 25.0
}

// ============================================================================
// FIDIV m16int (opcode DE /6)
// ============================================================================

#[test]
fn test_fidiv_m16int_positive() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDE, 0x34, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD,
        0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 100.0);
    write_i16(&mem, DATA_ADDR + 8, 4);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 25.0);
}

#[test]
fn test_fidiv_m16int_negative() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDE, 0x34, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD,
        0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 50.0);
    write_i16(&mem, DATA_ADDR + 8, -5);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), -10.0);
}

// ============================================================================
// FIDIV m32int (opcode DA /6)
// ============================================================================

#[test]
fn test_fidiv_m32int_positive() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDA, 0x34, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD,
        0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 1000.0);
    write_i32(&mem, DATA_ADDR + 8, 8);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 125.0);
}

// ============================================================================
// FDIVR m32fp (opcode D8 /7) - ST(0) = m32fp / ST(0)
// ============================================================================

#[test]
fn test_fdivr_m32fp_basic() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xD8, 0x3C, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD,
        0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 2.0);
    write_f32(&mem, DATA_ADDR + 8, 10.0);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 5.0); // 10.0 / 2.0
}

#[test]
fn test_fdivr_m32fp_negative() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xD8, 0x3C, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD,
        0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, -4.0);
    write_f32(&mem, DATA_ADDR + 8, 20.0);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), -5.0); // 20.0 / -4.0
}

// ============================================================================
// FDIVR m64fp (opcode DC /7)
// ============================================================================

#[test]
fn test_fdivr_m64fp_basic() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDC, 0x3C, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD,
        0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 4.0);
    write_f64(&mem, DATA_ADDR + 8, 100.0);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 25.0); // 100.0 / 4.0
}

// ============================================================================
// FDIVR ST(0), ST(i) (opcode D8 F8+i)
// ============================================================================

#[test]
fn test_fdivr_st0_st1() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, 0xD8,
        0xF9, // FDIVR ST(0), ST(1)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 4.0);
    write_f64(&mem, DATA_ADDR + 8, 20.0);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 0.2); // 4.0 / 20.0
}

// ============================================================================
// FDIVR ST(i), ST(0) (opcode DC F0+i)
// ============================================================================

#[test]
fn test_fdivr_st1_st0() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDC,
        0xF1, // FDIVR ST(1), ST(0)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 4.0);
    write_f64(&mem, DATA_ADDR + 8, 20.0);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 20.0); // ST(0) unchanged
    assert_eq!(read_f64(&mem, 0x3008), 5.0); // ST(1) = 20.0 / 4.0
}

// ============================================================================
// FDIVRP ST(i), ST(0) (opcode DE F0+i)
// ============================================================================

#[test]
fn test_fdivrp_st1_st0() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDE,
        0xF1, // FDIVRP ST(1), ST(0)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 5.0);
    write_f64(&mem, DATA_ADDR + 8, 25.0);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 5.0); // 25.0 / 5.0
}

// ============================================================================
// FISUBR m16int (opcode DE /7)
// ============================================================================

#[test]
fn test_fidivr_m16int_positive() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDE, 0x3C, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD,
        0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 5.0);
    write_i16(&mem, DATA_ADDR + 8, 100);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 20.0); // 100 / 5.0
}

// ============================================================================
// FIDIVR m32int (opcode DA /7)
// ============================================================================

#[test]
fn test_fidivr_m32int_positive() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDA, 0x3C, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD,
        0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 8.0);
    write_i32(&mem, DATA_ADDR + 8, 1000);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 125.0); // 1000 / 8.0
}

// ============================================================================
// Special cases
// ============================================================================

#[test]
fn test_fdiv_infinity_by_finite() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDC, 0x34, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD,
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
fn test_fdiv_finite_by_infinity() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDC, 0x34, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD,
        0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 100.0);
    write_f64(&mem, DATA_ADDR + 8, f64::INFINITY);

    run_until_hlt(&mut vcpu).unwrap();
    let result = read_f64(&mem, 0x3000);
    assert_eq!(result, 0.0);
    assert!(!result.is_sign_negative());
}

#[test]
fn test_fdiv_zero_by_nonzero() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDC, 0x34, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD,
        0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 0.0);
    write_f64(&mem, DATA_ADDR + 8, 5.0);

    run_until_hlt(&mut vcpu).unwrap();
    let result = read_f64(&mem, 0x3000);
    assert_eq!(result, 0.0);
}

#[test]
fn test_fdiv_nan_propagation() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDC, 0x34, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD,
        0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, f64::NAN);
    write_f64(&mem, DATA_ADDR + 8, 5.0);

    run_until_hlt(&mut vcpu).unwrap();
    assert!(read_f64(&mem, 0x3000).is_nan());
}

#[test]
fn test_fdiv_sign_rules() {
    // Test sign: pos/pos=pos, pos/neg=neg, neg/neg=pos
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDC, 0x34, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD,
        0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];

    // Positive / Positive
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 10.0);
    write_f64(&mem, DATA_ADDR + 8, 2.0);
    run_until_hlt(&mut vcpu).unwrap();
    let result = read_f64(&mem, 0x3000);
    assert_eq!(result, 5.0);
    assert!(!result.is_sign_negative());

    // Negative / Negative
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, -10.0);
    write_f64(&mem, DATA_ADDR + 8, -2.0);
    run_until_hlt(&mut vcpu).unwrap();
    let result = read_f64(&mem, 0x3000);
    assert_eq!(result, 5.0);
    assert!(!result.is_sign_negative());
}

#[test]
fn test_fdiv_by_one() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDC, 0x34, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD,
        0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 123.456);
    write_f64(&mem, DATA_ADDR + 8, 1.0);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 123.456);
}

#[test]
fn test_fdivr_vs_fdiv() {
    // Verify FDIVR reverses the operands
    let code1 = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDC, 0x34, 0x25, 0x08, 0x20, 0x00,
        0x00, // FDIV
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let code2 = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDC, 0x3C, 0x25, 0x00, 0x20, 0x00,
        0x00, // FDIVR
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];

    let (mut vcpu1, mem1) = setup_vm(&code1, None);
    write_f64(&mem1, DATA_ADDR, 10.0);
    write_f64(&mem1, DATA_ADDR + 8, 2.0);
    run_until_hlt(&mut vcpu1).unwrap();
    let result1 = read_f64(&mem1, 0x3000);

    let (mut vcpu2, mem2) = setup_vm(&code2, None);
    write_f64(&mem2, DATA_ADDR, 10.0);
    write_f64(&mem2, DATA_ADDR + 8, 2.0);
    run_until_hlt(&mut vcpu2).unwrap();
    let result2 = read_f64(&mem2, 0x3000);

    assert_eq!(result1, result2);
}

#[test]
fn test_fdiv_chain() {
    // Chain multiple divisions
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD 1000.0
        0xDC, 0x34, 0x25, 0x08, 0x20, 0x00, 0x00, // FDIV 10.0
        0xDC, 0x34, 0x25, 0x10, 0x20, 0x00, 0x00, // FDIV 5.0
        0xDC, 0x34, 0x25, 0x18, 0x20, 0x00, 0x00, // FDIV 2.0
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 1000.0);
    write_f64(&mem, DATA_ADDR + 8, 10.0);
    write_f64(&mem, DATA_ADDR + 16, 5.0);
    write_f64(&mem, DATA_ADDR + 24, 2.0);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 10.0); // 1000/10/5/2
}

#[test]
fn test_fidiv_precision() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDA, 0x34, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD,
        0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 100.5);
    write_i32(&mem, DATA_ADDR + 8, 2);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 50.25);
}

#[test]
fn test_fidivr_precision() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDA, 0x3C, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD,
        0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 5.0);
    write_i32(&mem, DATA_ADDR + 8, 100);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 20.0); // 100 / 5.0
}

#[test]
fn test_fdiv_very_small() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDC, 0x34, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD,
        0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 1.0e-100);
    write_f64(&mem, DATA_ADDR + 8, 1.0e100);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 1.0e-200);
}

#[test]
fn test_fdiv_very_large() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDC, 0x34, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD,
        0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 1.0e100);
    write_f64(&mem, DATA_ADDR + 8, 1.0e-100);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 1.0e200);
}

#[test]
fn test_fdivp_stack_behavior() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD 8.0
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD 4.0
        0xDD, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, // FLD 16.0
        0xDE, 0xF9, // FDIVP ST(1), ST(0) ; ST(1) = 4.0 / 16.0, pop
        0xDE, 0xF9, // FDIVP ST(1), ST(0) ; ST(1) = 8.0 / 0.25, pop
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 8.0);
    write_f64(&mem, DATA_ADDR + 8, 4.0);
    write_f64(&mem, DATA_ADDR + 16, 16.0);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 32.0); // 8.0 / 0.25
}

#[test]
fn test_fdivrp_stack_behavior() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD 10.0
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD 50.0
        0xDE, 0xF1, // FDIVRP ST(1), ST(0) ; ST(1) = 50.0 / 10.0, pop
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 10.0);
    write_f64(&mem, DATA_ADDR + 8, 50.0);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 5.0);
}

#[test]
fn test_fdiv_reciprocal() {
    // 1.0 / x gives reciprocal
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDC, 0x34, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD,
        0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 1.0);
    write_f64(&mem, DATA_ADDR + 8, 8.0);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 0.125);
}

#[test]
fn test_fdivr_inverse_symmetry() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDC, 0x3C, 0x25, 0x08, 0x20, 0x00,
        0x00, // FDIVR
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 4.0);
    write_f64(&mem, DATA_ADDR + 8, 20.0);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 5.0); // 20.0 / 4.0
}

#[test]
fn test_fdiv_zero_sign_preservation() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDC, 0x34, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD,
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
fn test_fdiv_self() {
    // x / x should equal 1.0
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xD9, 0xC0, // FLD ST(0) - duplicate
        0xDE, 0xF9, // FDIVP
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 123.456);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 1.0);
}

// ============================================================================
// Known-answer operand-order tests for FDIV / FDIVR.
//
// Pin down EXACT quotients for every encoding. Note the x87 quirk that the DC
// register encodings swap FDIV<->FDIVR relative to the D8 encodings:
//   FDIV  ST(0),ST(i): ST(0) = ST(0) / ST(i)
//   FDIVR ST(0),ST(i): ST(0) = ST(i) / ST(0)
//   FDIV  ST(i),ST(0): ST(i) = ST(i) / ST(0)
//   FDIVR ST(i),ST(0): ST(i) = ST(0) / ST(i)
//   FDIV  m:           ST(0) = ST(0) / m
//   FDIVR m:           ST(0) = m / ST(0)
// Values chosen so results are exact in f64 (powers-of-two friendly).
// ============================================================================

#[test]
fn test_fdiv_st0_sti_exact_order() {
    // FLD 2.0, FLD 8.0 -> ST(0)=8.0, ST(1)=2.0; FDIV ST(0),ST(1) -> 8.0/2.0 = 4.0
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, 0xD8,
        0xF1, // FDIV ST(0), ST(1)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 2.0);
    write_f64(&mem, DATA_ADDR + 8, 8.0);
    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 4.0);
}

#[test]
fn test_fdivr_st0_sti_exact_order() {
    // FLD 2.0, FLD 8.0 -> ST(0)=8.0, ST(1)=2.0; FDIVR ST(0),ST(1) -> 2.0/8.0 = 0.25
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, 0xD8,
        0xF9, // FDIVR ST(0), ST(1)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 2.0);
    write_f64(&mem, DATA_ADDR + 8, 8.0);
    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 0.25);
}

#[test]
fn test_fdiv_sti_st0_exact_order() {
    // FLD 2.0, FLD 8.0 -> ST(0)=8.0, ST(1)=2.0; FDIV ST(1),ST(0) -> ST(1)/ST(0) = 0.25
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDC,
        0xF9, // FDIV ST(1), ST(0)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // pop ST(0)=8.0
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, // pop ST(1)=0.25
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 2.0);
    write_f64(&mem, DATA_ADDR + 8, 8.0);
    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 8.0);
    assert_eq!(read_f64(&mem, 0x3008), 0.25);
}

#[test]
fn test_fdivr_sti_st0_exact_order() {
    // FLD 2.0, FLD 8.0 -> ST(0)=8.0, ST(1)=2.0; FDIVR ST(1),ST(0) -> ST(0)/ST(1) = 4.0
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDC,
        0xF1, // FDIVR ST(1), ST(0)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // pop ST(0)=8.0
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, // pop ST(1)=4.0
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 2.0);
    write_f64(&mem, DATA_ADDR + 8, 8.0);
    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 8.0);
    assert_eq!(read_f64(&mem, 0x3008), 4.0);
}

#[test]
fn test_fdiv_m64_exact_order() {
    // FLD 8.0, FDIV [2.0] -> 8.0 / 2.0 = 4.0
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDC, 0x34, 0x25, 0x08, 0x20, 0x00,
        0x00, // FDIV m64
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 8.0);
    write_f64(&mem, DATA_ADDR + 8, 2.0);
    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 4.0);
}

#[test]
fn test_fdivr_m64_exact_order() {
    // FLD 8.0, FDIVR [2.0] -> 2.0 / 8.0 = 0.25
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDC, 0x3C, 0x25, 0x08, 0x20, 0x00,
        0x00, // FDIVR m64
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 8.0);
    write_f64(&mem, DATA_ADDR + 8, 2.0);
    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 0.25);
}

#[test]
fn test_fdiv_by_zero_yields_infinity() {
    // FLD 1.0, FDIV [0.0] -> +inf (exceptions masked by default)
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDC, 0x34, 0x25, 0x08, 0x20, 0x00,
        0x00, // FDIV m64
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 1.0);
    write_f64(&mem, DATA_ADDR + 8, 0.0);
    run_until_hlt(&mut vcpu).unwrap();
    let r = read_f64(&mem, 0x3000);
    assert!(r.is_infinite() && r.is_sign_positive());
}
