//! Tests for the FCHS instruction.
//!
//! FCHS - Change Sign
//!
//! Complements the sign bit of ST(0). This operation changes a positive value
//! into a negative value of equal magnitude or vice versa.
//! The operation is SignBit(ST(0)) := NOT (SignBit(ST(0)))
//!
//! Opcode: D9 E0
//!
//! Flags affected:
//! - C1: Set to 0
//! - C0, C2, C3: Undefined
//!
//! Reference: /Users/int/dev/rax/docs/fchs.txt

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
// FCHS - Change Sign: Positive to Negative
// ============================================================================

#[test]
fn test_fchs_positive_to_negative_small() {
    // Load 3.14, change sign, store result
    // FLD qword [0x2000]  ; DD 04 25 00 20 00 00
    // FCHS                ; D9 E0
    // FSTP qword [0x3000] ; DD 1C 25 00 30 00 00
    // HLT                 ; F4
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xE0, // FCHS
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 3.14);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert_eq!(result, -3.14, "FCHS of 3.14 should be -3.14");
}

#[test]
fn test_fchs_positive_one() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xE0, // FCHS
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 1.0);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert_eq!(result, -1.0, "FCHS of 1.0 should be -1.0");
}

#[test]
fn test_fchs_positive_large() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xE0, // FCHS
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 999999.999999);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert_eq!(
        result, -999999.999999,
        "FCHS of large positive should be negative"
    );
}

#[test]
fn test_fchs_positive_fraction() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xE0, // FCHS
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 0.125);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert_eq!(result, -0.125, "FCHS of 0.125 should be -0.125");
}

#[test]
fn test_fchs_positive_very_small() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xE0, // FCHS
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 1e-100);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert_eq!(
        result, -1e-100,
        "FCHS of very small positive should be negative"
    );
}

// ============================================================================
// FCHS - Change Sign: Negative to Positive
// ============================================================================

#[test]
fn test_fchs_negative_to_positive_small() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xE0, // FCHS
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, -2.718);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert_eq!(result, 2.718, "FCHS of -2.718 should be 2.718");
}

#[test]
fn test_fchs_negative_one() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xE0, // FCHS
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, -1.0);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert_eq!(result, 1.0, "FCHS of -1.0 should be 1.0");
}

#[test]
fn test_fchs_negative_large() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xE0, // FCHS
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, -123456789.0);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert_eq!(
        result, 123456789.0,
        "FCHS of large negative should be positive"
    );
}

#[test]
fn test_fchs_negative_fraction() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xE0, // FCHS
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, -0.75);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert_eq!(result, 0.75, "FCHS of -0.75 should be 0.75");
}

#[test]
fn test_fchs_negative_very_small() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xE0, // FCHS
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, -1e-50);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert_eq!(
        result, 1e-50,
        "FCHS of very small negative should be positive"
    );
}

// ============================================================================
// FCHS - Special Cases: Zero
// ============================================================================

#[test]
fn test_fchs_positive_zero_to_negative_zero() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xE0, // FCHS
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 0.0);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert_eq!(result, -0.0, "FCHS of +0.0 should be -0.0");
    assert!(result.is_sign_negative(), "Result should be negative zero");
}

#[test]
fn test_fchs_negative_zero_to_positive_zero() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xE0, // FCHS
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, -0.0);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert_eq!(result, 0.0, "FCHS of -0.0 should be +0.0");
    assert!(!result.is_sign_negative(), "Result should be positive zero");
}

// ============================================================================
// FCHS - Special Cases: Infinity
// ============================================================================

#[test]
fn test_fchs_positive_infinity_to_negative_infinity() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xE0, // FCHS
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, f64::INFINITY);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert_eq!(
        result,
        f64::NEG_INFINITY,
        "FCHS of +infinity should be -infinity"
    );
    assert!(result.is_infinite(), "Result should be infinite");
    assert!(result.is_sign_negative(), "Result should be negative");
}

#[test]
fn test_fchs_negative_infinity_to_positive_infinity() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xE0, // FCHS
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, f64::NEG_INFINITY);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert_eq!(
        result,
        f64::INFINITY,
        "FCHS of -infinity should be +infinity"
    );
    assert!(result.is_infinite(), "Result should be infinite");
    assert!(!result.is_sign_negative(), "Result should be positive");
}

// ============================================================================
// FCHS - Special Cases: NaN
// ============================================================================

#[test]
fn test_fchs_nan() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xE0, // FCHS
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, f64::NAN);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert!(result.is_nan(), "FCHS of NaN should remain NaN");
}

#[test]
fn test_fchs_negative_nan() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xE0, // FCHS
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, -f64::NAN);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert!(result.is_nan(), "FCHS of negative NaN should remain NaN");
}

// ============================================================================
// FCHS - Double Negation (Idempotent)
// ============================================================================

#[test]
fn test_fchs_double_negation_positive() {
    // FCHS twice should return to original value
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xE0, // FCHS
        0xD9, 0xE0, // FCHS (second time)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 42.0);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert_eq!(
        result, 42.0,
        "Double FCHS should return original positive value"
    );
}

#[test]
fn test_fchs_double_negation_negative() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xE0, // FCHS
        0xD9, 0xE0, // FCHS (second time)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, -42.0);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert_eq!(
        result, -42.0,
        "Double FCHS should return original negative value"
    );
}

#[test]
fn test_fchs_double_negation_zero() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xE0, // FCHS
        0xD9, 0xE0, // FCHS (second time)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 0.0);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert_eq!(result, 0.0, "Double FCHS of +0.0 should return +0.0");
    assert!(!result.is_sign_negative(), "Result should be positive zero");
}

#[test]
fn test_fchs_triple_negation() {
    // Three FCHSs should be equivalent to one FCHS
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xE0, // FCHS
        0xD9, 0xE0, // FCHS (second)
        0xD9, 0xE0, // FCHS (third)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 17.5);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert_eq!(result, -17.5, "Triple FCHS should negate the value");
}

// ============================================================================
// FCHS - Edge Cases
// ============================================================================

#[test]
fn test_fchs_max_value() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xE0, // FCHS
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, f64::MAX);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert_eq!(result, -f64::MAX, "FCHS of MAX should be -MAX");
}

#[test]
fn test_fchs_min_positive() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xE0, // FCHS
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, f64::MIN_POSITIVE);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert_eq!(
        result,
        -f64::MIN_POSITIVE,
        "FCHS of MIN_POSITIVE should be -MIN_POSITIVE"
    );
}

#[test]
fn test_fchs_subnormal_positive() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xE0, // FCHS
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    let subnormal = f64::MIN_POSITIVE / 2.0;
    write_f64(&mem, 0x2000, subnormal);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert!(
        result.is_sign_negative(),
        "FCHS of positive subnormal should be negative"
    );
    assert_eq!(result, -subnormal, "Magnitude should be preserved");
}

#[test]
fn test_fchs_subnormal_negative() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xE0, // FCHS
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    let subnormal = -f64::MIN_POSITIVE / 2.0;
    write_f64(&mem, 0x2000, subnormal);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert!(
        !result.is_sign_negative(),
        "FCHS of negative subnormal should be positive"
    );
    assert_eq!(result, -subnormal, "Magnitude should be preserved");
}

// ============================================================================
// FCHS - Mathematical Constants
// ============================================================================

#[test]
fn test_fchs_pi() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xE0, // FCHS
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, std::f64::consts::PI);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert_eq!(result, -std::f64::consts::PI, "FCHS of PI should be -PI");
}

#[test]
fn test_fchs_e() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xE0, // FCHS
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, std::f64::consts::E);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert_eq!(result, -std::f64::consts::E, "FCHS of E should be -E");
}

// ============================================================================
// FCHS - Multiple Operations and Sequences
// ============================================================================

#[test]
fn test_fchs_sequence() {
    // Test FCHS with multiple values
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xE0, // FCHS
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008]
        0xD9, 0xE0, // FCHS
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, // FSTP qword [0x3008]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 10.5);
    write_f64(&mem, 0x2008, -20.5);

    run_until_hlt(&mut vcpu).unwrap();

    let result1 = read_f64(&mem, 0x3000);
    let result2 = read_f64(&mem, 0x3008);
    assert_eq!(result1, -10.5, "First FCHS result");
    assert_eq!(result2, 20.5, "Second FCHS result");
}

#[test]
fn test_fchs_with_arithmetic() {
    // Test FCHS combined with addition
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xE0, // FCHS
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008]
        0xDE, 0xC1, // FADDP (add and pop)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 5.0);
    write_f64(&mem, 0x2008, 3.0);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert_eq!(result, -2.0, "FCHS(5.0) + 3.0 = -5.0 + 3.0 = -2.0");
}

#[test]
fn test_fchs_various_magnitudes() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xE0, // FCHS
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let test_values = vec![
        1e-100, 1e-50, 1e-10, 0.001, 0.1, 10.0, 100.0, 1e10, 1e50, 1e100,
    ];

    for val in test_values {
        let (mut vcpu, mem) = setup_vm(&code, None);
        write_f64(&mem, 0x2000, val);

        run_until_hlt(&mut vcpu).unwrap();

        let result = read_f64(&mem, 0x3000);
        assert_eq!(result, -val, "FCHS of {} should be {}", val, -val);
    }
}

#[test]
fn test_fchs_quadruple_negation() {
    // Four FCHSs should return to original
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xE0, // FCHS
        0xD9, 0xE0, // FCHS
        0xD9, 0xE0, // FCHS
        0xD9, 0xE0, // FCHS
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 23.5);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert_eq!(result, 23.5, "Quadruple FCHS should return original value");
}

// ============================================================================
// Known-answer FCHS tests: FCHS flips the sign bit only (exact bit checks).
// ============================================================================

/// FLD [m64], FCHS, FSTP [m64] -> returns resulting bits.
fn kat_fchs_bits(value: f64) -> u64 {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xE0, // FCHS
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, value);
    run_until_hlt(&mut vcpu).unwrap();
    read_f64(&mem, 0x3000).to_bits()
}

#[test]
fn test_fchs_flips_sign_exact() {
    assert_eq!(kat_fchs_bits(1.0), (-1.0_f64).to_bits());
    assert_eq!(kat_fchs_bits(-2.5), (2.5_f64).to_bits());
    // FCHS toggles the sign bit of +0.0 to produce -0.0
    assert_eq!(kat_fchs_bits(0.0), (-0.0_f64).to_bits());
    assert_eq!(kat_fchs_bits(-0.0), (0.0_f64).to_bits());
}

#[test]
fn test_fchs_infinity_and_nan_sign() {
    assert_eq!(kat_fchs_bits(f64::INFINITY), f64::NEG_INFINITY.to_bits());
    assert_eq!(kat_fchs_bits(f64::NEG_INFINITY), f64::INFINITY.to_bits());
    // FCHS only flips the sign bit; NaN payload/quiet bit are unchanged.
    let nan = f64::from_bits(0x7FF8_0000_0000_0001);
    assert_eq!(kat_fchs_bits(nan), nan.to_bits() ^ (1u64 << 63));
}
