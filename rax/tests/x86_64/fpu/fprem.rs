//! Tests for the FPREM instruction.
//!
//! FPREM - Partial Remainder
//!
//! Computes the remainder from dividing ST(0) by ST(1) and stores result in ST(0).
//! Remainder = ST(0) - (Q * ST(1)) where Q is truncated quotient toward zero.
//! May require multiple executions if C2 flag is set (partial remainder).
//! Stores three least significant bits of quotient in C3, C1, C0.
//!
//! Opcode: D9 F8
//!
//! Flags affected:
//! - C0: Set to bit 2 (Q2) of quotient
//! - C1: Set to bit 0 (Q0) of quotient (or 0 if stack underflow)
//! - C2: Set to 0 if reduction complete, 1 if incomplete
//! - C3: Set to bit 1 (Q1) of quotient
//!
//! Reference: /Users/int/dev/rax/docs/fprem.txt

use crate::common::*;
use rax::cpu::Registers;
use vm_memory::{Bytes, GuestAddress};

// Helper function to write f64 to memory
fn write_f64(mem: &vm_memory::GuestMemoryMmap, addr: u64, val: f64) {
    mem.write_slice(&val.to_le_bytes(), GuestAddress(addr))
        .unwrap();
}

// Helper function to read f64 from memory
fn read_f64(mem: &vm_memory::GuestMemoryMmap, addr: u64) -> f64 {
    let mut buf = [0u8; 8];
    mem.read_slice(&mut buf, GuestAddress(addr)).unwrap();
    f64::from_le_bytes(buf)
}

// ============================================================================
// FPREM - Basic Remainder Operations
// ============================================================================

#[test]
fn test_fprem_basic_positive() {
    // 7.0 % 3.0 = 1.0
    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008] ; divisor (3.0)
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000] ; dividend (7.0)
        0xD9, 0xF8, // FPREM
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000] ; result
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 7.0);
    write_f64(&mem, 0x2008, 3.0);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert_eq!(result, 1.0, "7.0 % 3.0 should be 1.0");
}

#[test]
fn test_fprem_exact_division() {
    // 9.0 % 3.0 = 0.0
    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008] ; divisor
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000] ; dividend
        0xD9, 0xF8, // FPREM
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 9.0);
    write_f64(&mem, 0x2008, 3.0);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert_eq!(result, 0.0, "9.0 % 3.0 should be 0.0");
}

#[test]
fn test_fprem_small_dividend() {
    // 2.0 % 5.0 = 2.0
    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008] ; divisor
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000] ; dividend
        0xD9, 0xF8, // FPREM
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 2.0);
    write_f64(&mem, 0x2008, 5.0);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert_eq!(result, 2.0, "2.0 % 5.0 should be 2.0");
}

#[test]
fn test_fprem_fractional() {
    // 5.5 % 2.0 = 1.5
    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008]
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xF8, // FPREM
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 5.5);
    write_f64(&mem, 0x2008, 2.0);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert_eq!(result, 1.5, "5.5 % 2.0 should be 1.5");
}

// ============================================================================
// FPREM - Negative Dividends
// ============================================================================

#[test]
fn test_fprem_negative_dividend() {
    // -7.0 % 3.0 = -1.0 (sign follows dividend)
    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008]
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xF8, // FPREM
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, -7.0);
    write_f64(&mem, 0x2008, 3.0);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert_eq!(result, -1.0, "-7.0 % 3.0 should be -1.0");
}

#[test]
fn test_fprem_negative_divisor() {
    // 7.0 % -3.0 = 1.0 (sign follows dividend)
    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008]
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xF8, // FPREM
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 7.0);
    write_f64(&mem, 0x2008, -3.0);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert_eq!(result, 1.0, "7.0 % -3.0 should be 1.0");
}

#[test]
fn test_fprem_both_negative() {
    // -7.0 % -3.0 = -1.0
    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008]
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xF8, // FPREM
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, -7.0);
    write_f64(&mem, 0x2008, -3.0);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert_eq!(result, -1.0, "-7.0 % -3.0 should be -1.0");
}

// ============================================================================
// FPREM - Special Values: Zero
// ============================================================================

#[test]
fn test_fprem_zero_dividend() {
    // 0.0 % 5.0 = 0.0
    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008]
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xF8, // FPREM
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 0.0);
    write_f64(&mem, 0x2008, 5.0);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert_eq!(result, 0.0, "0.0 % 5.0 should be 0.0");
    assert!(!result.is_sign_negative(), "Result should be positive zero");
}

#[test]
fn test_fprem_negative_zero_dividend() {
    // -0.0 % 5.0 = -0.0
    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008]
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xF8, // FPREM
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, -0.0);
    write_f64(&mem, 0x2008, 5.0);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert_eq!(result, 0.0, "-0.0 % 5.0 should be -0.0");
    assert!(result.is_sign_negative(), "Result should be negative zero");
}

// ============================================================================
// FPREM - Divisors
// ============================================================================

#[test]
fn test_fprem_divisor_one() {
    // 5.5 % 1.0 = 0.5
    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008]
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xF8, // FPREM
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 5.5);
    write_f64(&mem, 0x2008, 1.0);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert_eq!(result, 0.5, "5.5 % 1.0 should be 0.5");
}

#[test]
fn test_fprem_small_divisor() {
    // 10.0 % 0.5 = 0.0
    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008]
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xF8, // FPREM
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 10.0);
    write_f64(&mem, 0x2008, 0.5);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert_eq!(result, 0.0, "10.0 % 0.5 should be 0.0");
}

#[test]
fn test_fprem_large_divisor() {
    // 5.0 % 10.0 = 5.0
    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008]
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xF8, // FPREM
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 5.0);
    write_f64(&mem, 0x2008, 10.0);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert_eq!(result, 5.0, "5.0 % 10.0 should be 5.0");
}

#[test]
fn test_fprem_fractional_divisor() {
    // 7.0 % 1.5 = 1.0
    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008]
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xF8, // FPREM
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 7.0);
    write_f64(&mem, 0x2008, 1.5);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert_eq!(result, 1.0, "7.0 % 1.5 should be 1.0");
}

// ============================================================================
// FPREM - Special Values: Infinity
// ============================================================================

#[test]
fn test_fprem_finite_mod_infinity() {
    // 5.0 % infinity = 5.0
    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008]
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xF8, // FPREM
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 5.0);
    write_f64(&mem, 0x2008, f64::INFINITY);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert_eq!(result, 5.0, "Finite % infinity should be the finite value");
}

// ============================================================================
// FPREM - Mathematical Constants
// ============================================================================

#[test]
fn test_fprem_pi_modulo() {
    // Test PI % 1.0
    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008]
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xF8, // FPREM
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, std::f64::consts::PI);
    write_f64(&mem, 0x2008, 1.0);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    let expected = std::f64::consts::PI - 3.0;
    assert!((result - expected).abs() < 1e-10, "PI % 1.0 computation");
}

#[test]
fn test_fprem_angle_reduction() {
    // Common use: reduce angle to unit circle
    // 10.0 % (2*PI) for angle reduction
    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008]
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xF8, // FPREM
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    let angle = 10.0;
    let two_pi = 2.0 * std::f64::consts::PI;
    write_f64(&mem, 0x2000, angle);
    write_f64(&mem, 0x2008, two_pi);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert!(
        result >= 0.0 && result < two_pi,
        "Angle should be reduced to [0, 2π)"
    );
}

// ============================================================================
// FPREM - Multiple Operations
// ============================================================================

#[test]
fn test_fprem_sequence() {
    // Test multiple FPREM operations
    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008]
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xF8, // FPREM
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xDD, 0x04, 0x25, 0x18, 0x20, 0x00, 0x00, // FLD qword [0x2018]
        0xDD, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, // FLD qword [0x2010]
        0xD9, 0xF8, // FPREM
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, // FSTP qword [0x3008]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 10.0);
    write_f64(&mem, 0x2008, 3.0);
    write_f64(&mem, 0x2010, 15.0);
    write_f64(&mem, 0x2018, 4.0);

    run_until_hlt(&mut vcpu).unwrap();

    let result1 = read_f64(&mem, 0x3000);
    let result2 = read_f64(&mem, 0x3008);
    assert_eq!(result1, 1.0, "10.0 % 3.0 = 1.0");
    assert_eq!(result2, 3.0, "15.0 % 4.0 = 3.0");
}

// ============================================================================
// FPREM - Edge Cases
// ============================================================================

#[test]
fn test_fprem_very_large_dividend() {
    // Large % small
    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008]
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xF8, // FPREM
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 1000000.0);
    write_f64(&mem, 0x2008, 7.0);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert_eq!(result, 1.0, "1000000.0 % 7.0 = 1.0");
}

#[test]
fn test_fprem_very_small_values() {
    // Small % smaller
    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008]
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xF8, // FPREM
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 1e-10);
    write_f64(&mem, 0x2008, 3e-11);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    let expected = 1e-10 % 3e-11;
    assert!((result - expected).abs() < 1e-20, "Very small values");
}

#[test]
fn test_fprem_same_values() {
    // x % x = 0
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xF8, // FPREM
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 5.5);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert_eq!(result, 0.0, "x % x should be 0");
}

#[test]
fn test_fprem_preserves_divisor() {
    // FPREM only modifies ST(0), ST(1) (divisor) remains unchanged
    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008] ; divisor
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000] ; dividend
        0xD9, 0xF8, // FPREM
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000] ; remainder
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, // FSTP qword [0x3008] ; divisor
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 10.0);
    write_f64(&mem, 0x2008, 3.0);

    run_until_hlt(&mut vcpu).unwrap();

    let remainder = read_f64(&mem, 0x3000);
    let divisor = read_f64(&mem, 0x3008);
    assert_eq!(remainder, 1.0, "Remainder should be 1.0");
    assert_eq!(divisor, 3.0, "Divisor should remain 3.0");
}

// ============================================================================
// FPREM - Partial Remainder (C2 Flag)
// ============================================================================

#[test]
fn test_fprem_completion() {
    // For small exponent differences, should complete in one iteration
    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008]
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xF8, // FPREM
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 100.0);
    write_f64(&mem, 0x2008, 7.0);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert_eq!(result, 2.0, "100.0 % 7.0 = 2.0");
}

// ============================================================================
// FPREM - Various Combinations
// ============================================================================

#[test]
fn test_fprem_power_of_two_divisor() {
    // Modulo by power of 2
    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008]
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xF8, // FPREM
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 17.0);
    write_f64(&mem, 0x2008, 8.0);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert_eq!(result, 1.0, "17.0 % 8.0 = 1.0");
}

#[test]
fn test_fprem_various_divisors() {
    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008]
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xF8, // FPREM
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let test_cases = vec![
        (10.0, 3.0, 1.0),
        (10.0, 4.0, 2.0),
        (10.0, 7.0, 3.0),
        (20.0, 6.0, 2.0),
        (100.0, 11.0, 1.0),
    ];

    for (dividend, divisor, expected) in test_cases {
        let (mut vcpu, mem) = setup_vm(&code, None);
        write_f64(&mem, 0x2000, dividend);
        write_f64(&mem, 0x2008, divisor);

        run_until_hlt(&mut vcpu).unwrap();

        let result = read_f64(&mem, 0x3000);
        assert_eq!(
            result, expected,
            "{} % {} should be {}",
            dividend, divisor, expected
        );
    }
}

#[test]
fn test_fprem_decimal_values() {
    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008]
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xF8, // FPREM
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let test_cases = vec![(3.14, 1.0, 0.14), (2.71, 0.5, 0.21), (9.99, 2.0, 1.99)];

    for (dividend, divisor, expected) in test_cases {
        let (mut vcpu, mem) = setup_vm(&code, None);
        write_f64(&mem, 0x2000, dividend);
        write_f64(&mem, 0x2008, divisor);

        run_until_hlt(&mut vcpu).unwrap();

        let result = read_f64(&mem, 0x3000);
        assert!(
            (result - expected).abs() < 1e-10,
            "{} % {} should be approximately {}",
            dividend,
            divisor,
            expected
        );
    }
}

#[test]
fn test_fprem_irrational_dividend() {
    // Test with sqrt(2)
    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008]
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xF8, // FPREM
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    let sqrt2 = 2.0f64.sqrt();
    write_f64(&mem, 0x2000, sqrt2);
    write_f64(&mem, 0x2008, 1.0);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    let expected = sqrt2 - 1.0;
    assert!((result - expected).abs() < 1e-10, "sqrt(2) % 1.0");
}

// ============================================================================
// Known-answer FPREM tests: remainder = ST(0) - trunc(ST(0)/ST(1))*ST(1),
// and the quotient low 3 bits land in C0/C3/C1 of the status word.
// ============================================================================

/// ST(1)=divisor (loaded first), ST(0)=dividend, FPREM, store remainder + SW.
fn kat_fprem(dividend: f64, divisor: f64) -> (f64, u16) {
    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008] (divisor -> ST(1))
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000] (dividend -> ST(0))
        0xD9, 0xF8, // FPREM
        0xDD, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // FST qword [0x3000] (remainder, no pop)
        0xDD, 0x3C, 0x25, 0x10, 0x30, 0x00, 0x00, // FNSTSW [0x3010]
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, dividend);
    write_f64(&mem, 0x2008, divisor);
    run_until_hlt(&mut vcpu).unwrap();
    let mut buf = [0u8; 2];
    mem.read_slice(&mut buf, GuestAddress(0x3010)).unwrap();
    (read_f64(&mem, 0x3000), u16::from_le_bytes(buf))
}

#[test]
fn test_fprem_exact_remainders() {
    assert_eq!(kat_fprem(7.0, 3.0).0, 1.0); // 7 = 2*3 + 1
    assert_eq!(kat_fprem(9.0, 3.0).0, 0.0); // exact
    assert_eq!(kat_fprem(10.0, 4.0).0, 2.0); // 10 = 2*4 + 2
    assert_eq!(kat_fprem(1.0, 0.5).0, 0.0);
    assert_eq!(kat_fprem(5.5, 2.0).0, 1.5); // 5.5 = 2*2 + 1.5
}

#[test]
fn test_fprem_negative_dividend_sign() {
    // FPREM remainder takes the sign of the dividend.
    assert_eq!(kat_fprem(-7.0, 3.0).0, -1.0);
    assert_eq!(kat_fprem(7.0, -3.0).0, 1.0);
}

#[test]
fn test_fprem_quotient_bits_in_status_word() {
    // 7/3 truncates to quotient 2 (binary 010) -> Q0=0(C1), Q1=1(C3), Q2=0(C0).
    let (_rem, sw) = kat_fprem(7.0, 3.0);
    const C0: u16 = 0x0100;
    const C1: u16 = 0x0200;
    const C3: u16 = 0x4000;
    assert_eq!(sw & C1, 0, "Q0 (C1) should be 0 for quotient 2");
    assert_ne!(sw & C3, 0, "Q1 (C3) should be 1 for quotient 2");
    assert_eq!(sw & C0, 0, "Q2 (C0) should be 0 for quotient 2");
}
