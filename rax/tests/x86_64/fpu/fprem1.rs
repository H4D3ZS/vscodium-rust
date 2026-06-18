//! Tests for the FPREM1 instruction.
//!
//! FPREM1 - Partial Remainder (IEEE 754 compliant)
//!
//! Computes the IEEE remainder obtained from dividing ST(0) by ST(1) and stores result in ST(0).
//! Remainder = ST(0) - (Q * ST(1)) where Q is rounded quotient toward nearest integer.
//! May require multiple executions if C2 flag is set (partial remainder).
//! Stores three least significant bits of quotient in C3, C1, C0.
//!
//! Opcode: D9 F5
//!
//! Flags affected:
//! - C0: Set to bit 2 (Q2) of quotient
//! - C1: Set to bit 0 (Q0) of quotient (or 0 if stack underflow)
//! - C2: Set to 0 if reduction complete, 1 if incomplete
//! - C3: Set to bit 1 (Q1) of quotient
//!
//! Reference: /Users/int/dev/rax/docs/fprem1.txt

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
// FPREM1 - Basic IEEE Remainder Operations
// ============================================================================

#[test]
fn test_fprem1_basic_positive() {
    // 7.0 % 3.0 (IEEE) - rounds to nearest
    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008] ; divisor (3.0)
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000] ; dividend (7.0)
        0xD9, 0xF5, // FPREM1
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000] ; result
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 7.0);
    write_f64(&mem, 0x2008, 3.0);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert_eq!(result, 1.0, "7.0 % 3.0 (IEEE) should be 1.0");
}

#[test]
fn test_fprem1_exact_division() {
    // 9.0 % 3.0 = 0.0
    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008] ; divisor
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000] ; dividend
        0xD9, 0xF5, // FPREM1
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
fn test_fprem1_small_dividend() {
    // 2.0 % 5.0 = 2.0
    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008] ; divisor
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000] ; dividend
        0xD9, 0xF5, // FPREM1
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
fn test_fprem1_fractional() {
    // 5.5 % 2.0 = -0.5 (rounds to nearest, Q=3)
    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008]
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xF5, // FPREM1
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 5.5);
    write_f64(&mem, 0x2008, 2.0);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    // IEEE remainder: 5.5 = 3*2.0 - 0.5, so remainder is -0.5
    assert_eq!(result, -0.5, "5.5 % 2.0 (IEEE) should be -0.5");
}

// ============================================================================
// FPREM1 - IEEE Rounding vs FPREM
// ============================================================================

#[test]
fn test_fprem1_ieee_rounding_case1() {
    // 5.0 % 3.0: Q rounds to 2 (nearest), remainder = 5.0 - 2*3.0 = -1.0
    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008]
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xF5, // FPREM1
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 5.0);
    write_f64(&mem, 0x2008, 3.0);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    // IEEE: 5.0/3.0 = 1.666..., rounds to 2, so 5.0 - 2*3.0 = -1.0
    assert_eq!(result, -1.0, "5.0 % 3.0 (IEEE) should be -1.0");
}

#[test]
fn test_fprem1_ieee_rounding_case2() {
    // 7.5 % 4.0: Q = 1.875 rounds to 2, remainder = 7.5 - 2*4.0 = -0.5
    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008]
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xF5, // FPREM1
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 7.5);
    write_f64(&mem, 0x2008, 4.0);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert_eq!(result, -0.5, "7.5 % 4.0 (IEEE) should be -0.5");
}

#[test]
fn test_fprem1_magnitude_less_than_half() {
    // IEEE remainder magnitude should be <= |modulus|/2
    // 10.0 % 7.0: Q = 1.428... rounds to 1, remainder = 10.0 - 1*7.0 = 3.0
    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008]
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xF5, // FPREM1
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 10.0);
    write_f64(&mem, 0x2008, 7.0);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert_eq!(result, 3.0, "10.0 % 7.0 (IEEE) should be 3.0");
    assert!(
        result.abs() <= 7.0 / 2.0,
        "Magnitude should be <= |modulus|/2"
    );
}

// ============================================================================
// FPREM1 - Negative Dividends
// ============================================================================

#[test]
fn test_fprem1_negative_dividend() {
    // -7.0 % 3.0
    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008]
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xF5, // FPREM1
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, -7.0);
    write_f64(&mem, 0x2008, 3.0);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    // -7.0/3.0 = -2.333... rounds to -2, so -7.0 - (-2)*3.0 = -1.0
    assert_eq!(result, -1.0, "-7.0 % 3.0 (IEEE) should be -1.0");
}

#[test]
fn test_fprem1_negative_divisor() {
    // 7.0 % -3.0
    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008]
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xF5, // FPREM1
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 7.0);
    write_f64(&mem, 0x2008, -3.0);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    // 7.0/-3.0 = -2.333... rounds to -2, so 7.0 - (-2)*(-3.0) = 1.0
    assert_eq!(result, 1.0, "7.0 % -3.0 (IEEE) should be 1.0");
}

#[test]
fn test_fprem1_both_negative() {
    // -7.0 % -3.0
    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008]
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xF5, // FPREM1
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, -7.0);
    write_f64(&mem, 0x2008, -3.0);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    // -7.0/-3.0 = 2.333... rounds to 2, so -7.0 - 2*(-3.0) = -1.0
    assert_eq!(result, -1.0, "-7.0 % -3.0 (IEEE) should be -1.0");
}

#[test]
fn test_fprem1_negative_half_case() {
    // -5.0 % 3.0: rounds to nearest even
    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008]
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xF5, // FPREM1
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, -5.0);
    write_f64(&mem, 0x2008, 3.0);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    // -5.0/3.0 = -1.666... rounds to -2, so -5.0 - (-2)*3.0 = 1.0
    assert_eq!(result, 1.0, "-5.0 % 3.0 (IEEE) should be 1.0");
}

// ============================================================================
// FPREM1 - Special Values: Zero
// ============================================================================

#[test]
fn test_fprem1_zero_dividend() {
    // 0.0 % 5.0 = 0.0
    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008]
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xF5, // FPREM1
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
fn test_fprem1_negative_zero_dividend() {
    // -0.0 % 5.0 = -0.0
    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008]
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xF5, // FPREM1
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
// FPREM1 - Divisors
// ============================================================================

#[test]
fn test_fprem1_divisor_one() {
    // 5.5 % 1.0 = 0.5
    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008]
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xF5, // FPREM1
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 5.5);
    write_f64(&mem, 0x2008, 1.0);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    // 5.5/1.0 = 5.5 rounds to 6 (even), so 5.5 - 6*1.0 = -0.5
    assert_eq!(result, -0.5, "5.5 % 1.0 (IEEE) should be -0.5");
}

#[test]
fn test_fprem1_small_divisor() {
    // 10.0 % 0.5 = 0.0
    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008]
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xF5, // FPREM1
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
fn test_fprem1_large_divisor() {
    // 5.0 % 10.0 = 5.0
    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008]
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xF5, // FPREM1
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
fn test_fprem1_fractional_divisor() {
    // 7.0 % 1.5
    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008]
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xF5, // FPREM1
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 7.0);
    write_f64(&mem, 0x2008, 1.5);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    // 7.0/1.5 = 4.666... rounds to 5, so 7.0 - 5*1.5 = -0.5
    assert_eq!(result, -0.5, "7.0 % 1.5 (IEEE) should be -0.5");
}

// ============================================================================
// FPREM1 - Special Values: Infinity
// ============================================================================

#[test]
fn test_fprem1_finite_mod_infinity() {
    // 5.0 % infinity = 5.0
    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008]
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xF5, // FPREM1
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

#[test]
fn test_fprem1_finite_mod_neg_infinity() {
    // 5.0 % -infinity = 5.0
    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008]
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xF5, // FPREM1
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 5.0);
    write_f64(&mem, 0x2008, f64::NEG_INFINITY);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert_eq!(result, 5.0, "Finite % -infinity should be the finite value");
}

// ============================================================================
// FPREM1 - Angle Reduction for Tangent
// ============================================================================

#[test]
fn test_fprem1_pi_over_4_reduction() {
    // Common use: reduce angle to π/4 for tangent
    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008]
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xF5, // FPREM1
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    let angle = 10.0;
    let pi_over_4 = std::f64::consts::FRAC_PI_4;
    write_f64(&mem, 0x2000, angle);
    write_f64(&mem, 0x2008, pi_over_4);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert!(
        result.abs() <= pi_over_4 / 2.0,
        "Reduced angle magnitude should be <= π/8"
    );
}

#[test]
fn test_fprem1_pi_modulo() {
    // Test PI % 1.0
    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008]
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xF5, // FPREM1
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, std::f64::consts::PI);
    write_f64(&mem, 0x2008, 1.0);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    // PI/1.0 = 3.14159... rounds to 3, so PI - 3*1.0 = 0.14159...
    let expected = std::f64::consts::PI - 3.0;
    assert!(
        (result - expected).abs() < 1e-10,
        "PI % 1.0 (IEEE) computation"
    );
}

// ============================================================================
// FPREM1 - Multiple Operations
// ============================================================================

#[test]
fn test_fprem1_sequence() {
    // Test multiple FPREM1 operations
    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008]
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xF5, // FPREM1
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xDD, 0x04, 0x25, 0x18, 0x20, 0x00, 0x00, // FLD qword [0x2018]
        0xDD, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, // FLD qword [0x2010]
        0xD9, 0xF5, // FPREM1
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
    // 10.0/3.0 = 3.333... rounds to 3, so 10.0 - 3*3.0 = 1.0
    assert_eq!(result1, 1.0, "10.0 % 3.0 (IEEE) = 1.0");
    // 15.0/4.0 = 3.75 rounds to 4, so 15.0 - 4*4.0 = -1.0
    assert_eq!(result2, -1.0, "15.0 % 4.0 (IEEE) = -1.0");
}

// ============================================================================
// FPREM1 - Edge Cases
// ============================================================================

#[test]
fn test_fprem1_very_large_dividend() {
    // Large % small
    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008]
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xF5, // FPREM1
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 1000000.0);
    write_f64(&mem, 0x2008, 7.0);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    // 1000000/7 = 142857.142... rounds to 142857, so 1000000 - 142857*7 = 1
    assert!(result.abs() <= 3.5, "Magnitude should be <= 7/2");
}

#[test]
fn test_fprem1_very_small_values() {
    // Small % smaller
    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008]
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xF5, // FPREM1
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 1e-10);
    write_f64(&mem, 0x2008, 3e-11);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert!(
        result.abs() <= 1.5e-11,
        "Magnitude should be <= |modulus|/2"
    );
}

#[test]
fn test_fprem1_same_values() {
    // x % x = 0
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xF5, // FPREM1
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
fn test_fprem1_preserves_divisor() {
    // FPREM1 only modifies ST(0), ST(1) (divisor) remains unchanged
    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008] ; divisor
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000] ; dividend
        0xD9, 0xF5, // FPREM1
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
// FPREM1 - Various Combinations
// ============================================================================

#[test]
fn test_fprem1_power_of_two_divisor() {
    // Modulo by power of 2
    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008]
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xF5, // FPREM1
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 17.0);
    write_f64(&mem, 0x2008, 8.0);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    // 17.0/8.0 = 2.125 rounds to 2, so 17.0 - 2*8.0 = 1.0
    assert_eq!(result, 1.0, "17.0 % 8.0 (IEEE) = 1.0");
}

#[test]
fn test_fprem1_irrational_dividend() {
    // Test with sqrt(2)
    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008]
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xF5, // FPREM1
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    let sqrt2 = 2.0f64.sqrt();
    write_f64(&mem, 0x2000, sqrt2);
    write_f64(&mem, 0x2008, 1.0);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    // sqrt(2)/1.0 = 1.414... rounds to 1, so sqrt(2) - 1*1.0 = 0.414...
    let expected = sqrt2 - 1.0;
    assert!((result - expected).abs() < 1e-10, "sqrt(2) % 1.0 (IEEE)");
}

#[test]
fn test_fprem1_halfway_cases() {
    // Test rounding to even when exactly halfway
    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008]
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xF5, // FPREM1
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 2.5);
    write_f64(&mem, 0x2008, 1.0);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    // 2.5/1.0 = 2.5, rounds to 2 (even), so 2.5 - 2*1.0 = 0.5
    assert_eq!(result, 0.5, "2.5 % 1.0 (IEEE) should round to even");
}

#[test]
fn test_fprem1_completion() {
    // For small exponent differences, should complete in one iteration
    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008]
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xF5, // FPREM1
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 100.0);
    write_f64(&mem, 0x2008, 7.0);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    // 100.0/7.0 = 14.285... rounds to 14, so 100.0 - 14*7.0 = 2.0
    assert_eq!(result, 2.0, "100.0 % 7.0 (IEEE) = 2.0");
}
