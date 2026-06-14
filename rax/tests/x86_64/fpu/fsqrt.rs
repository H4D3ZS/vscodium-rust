//! Tests for the FSQRT instruction.
//!
//! FSQRT - Square Root
//!
//! Computes the square root of the source value in the ST(0) register and
//! stores the result in ST(0).
//! The operation is ST(0) := SquareRoot(ST(0))
//!
//! Opcode: D9 FA
//!
//! Flags affected:
//! - C1: Set to 0 if stack underflow occurred; Set if result was rounded up
//! - C0, C2, C3: Undefined
//!
//! Exceptions:
//! - #IA: Source operand is a negative value (except -0), SNaN, or unsupported format
//! - #D: Source operand is a denormal value
//! - #P: Value cannot be represented exactly in destination format
//!
//! Reference: /Users/int/dev/rax/docs/fsqrt.txt

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
// FSQRT - Perfect Squares
// ============================================================================

#[test]
fn test_fsqrt_perfect_square_4() {
    // Load 4.0, compute square root, store result
    // FLD qword [0x2000]  ; DD 04 25 00 20 00 00
    // FSQRT               ; D9 FA
    // FSTP qword [0x3000] ; DD 1C 25 00 30 00 00
    // HLT                 ; F4
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xFA, // FSQRT
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 4.0);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert_eq!(result, 2.0, "FSQRT of 4.0 should be 2.0");
}

#[test]
fn test_fsqrt_perfect_square_9() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xFA, // FSQRT
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 9.0);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert_eq!(result, 3.0, "FSQRT of 9.0 should be 3.0");
}

#[test]
fn test_fsqrt_perfect_square_16() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xFA, // FSQRT
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 16.0);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert_eq!(result, 4.0, "FSQRT of 16.0 should be 4.0");
}

#[test]
fn test_fsqrt_perfect_square_25() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xFA, // FSQRT
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 25.0);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert_eq!(result, 5.0, "FSQRT of 25.0 should be 5.0");
}

#[test]
fn test_fsqrt_perfect_square_100() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xFA, // FSQRT
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 100.0);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert_eq!(result, 10.0, "FSQRT of 100.0 should be 10.0");
}

#[test]
fn test_fsqrt_perfect_square_144() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xFA, // FSQRT
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 144.0);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert_eq!(result, 12.0, "FSQRT of 144.0 should be 12.0");
}

// ============================================================================
// FSQRT - Non-Perfect Squares
// ============================================================================

#[test]
fn test_fsqrt_two() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xFA, // FSQRT
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 2.0);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    let expected = 2.0_f64.sqrt();
    assert!(
        (result - expected).abs() < 1e-15,
        "FSQRT of 2.0 should be approximately {}",
        expected
    );
}

#[test]
fn test_fsqrt_three() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xFA, // FSQRT
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 3.0);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    let expected = 3.0_f64.sqrt();
    assert!(
        (result - expected).abs() < 1e-15,
        "FSQRT of 3.0 should be approximately {}",
        expected
    );
}

#[test]
fn test_fsqrt_five() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xFA, // FSQRT
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 5.0);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    let expected = 5.0_f64.sqrt();
    assert!(
        (result - expected).abs() < 1e-15,
        "FSQRT of 5.0 should be approximately {}",
        expected
    );
}

#[test]
fn test_fsqrt_pi() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xFA, // FSQRT
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, std::f64::consts::PI);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    let expected = std::f64::consts::PI.sqrt();
    assert!(
        (result - expected).abs() < 1e-15,
        "FSQRT of PI should be approximately {}",
        expected
    );
}

#[test]
fn test_fsqrt_e() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xFA, // FSQRT
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, std::f64::consts::E);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    let expected = std::f64::consts::E.sqrt();
    assert!(
        (result - expected).abs() < 1e-15,
        "FSQRT of E should be approximately {}",
        expected
    );
}

// ============================================================================
// FSQRT - Special Cases: Zero
// ============================================================================

#[test]
fn test_fsqrt_positive_zero() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xFA, // FSQRT
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 0.0);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert_eq!(result, 0.0, "FSQRT of +0.0 should be +0.0");
    assert!(!result.is_sign_negative(), "Result should be positive zero");
}

#[test]
fn test_fsqrt_negative_zero() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xFA, // FSQRT
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, -0.0);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert_eq!(result, -0.0, "FSQRT of -0.0 should be -0.0");
    assert!(result.is_sign_negative(), "Result should be negative zero");
}

// ============================================================================
// FSQRT - Special Cases: Infinity
// ============================================================================

#[test]
fn test_fsqrt_positive_infinity() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xFA, // FSQRT
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, f64::INFINITY);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert_eq!(
        result,
        f64::INFINITY,
        "FSQRT of +infinity should be +infinity"
    );
}

#[test]
fn test_fsqrt_negative_infinity() {
    // FSQRT of negative infinity should produce NaN (invalid operation)
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xFA, // FSQRT
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, f64::NEG_INFINITY);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert!(result.is_nan(), "FSQRT of -infinity should produce NaN");
}

// ============================================================================
// FSQRT - Special Cases: NaN
// ============================================================================

#[test]
fn test_fsqrt_nan() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xFA, // FSQRT
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, f64::NAN);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert!(result.is_nan(), "FSQRT of NaN should remain NaN");
}

// ============================================================================
// FSQRT - Negative Numbers (Invalid Operation)
// ============================================================================

#[test]
fn test_fsqrt_negative_one() {
    // FSQRT of negative number (except -0) should produce NaN
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xFA, // FSQRT
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, -1.0);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert!(
        result.is_nan(),
        "FSQRT of -1.0 should produce NaN (invalid operation)"
    );
}

#[test]
fn test_fsqrt_negative_small() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xFA, // FSQRT
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, -0.5);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert!(result.is_nan(), "FSQRT of -0.5 should produce NaN");
}

#[test]
fn test_fsqrt_negative_large() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xFA, // FSQRT
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, -100.0);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert!(result.is_nan(), "FSQRT of -100.0 should produce NaN");
}

// ============================================================================
// FSQRT - Fractional Values
// ============================================================================

#[test]
fn test_fsqrt_one_quarter() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xFA, // FSQRT
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 0.25);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert_eq!(result, 0.5, "FSQRT of 0.25 should be 0.5");
}

#[test]
fn test_fsqrt_one_sixteenth() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xFA, // FSQRT
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 0.0625);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert_eq!(result, 0.25, "FSQRT of 0.0625 should be 0.25");
}

#[test]
fn test_fsqrt_decimal_fraction() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xFA, // FSQRT
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 0.01);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert_eq!(result, 0.1, "FSQRT of 0.01 should be 0.1");
}

// ============================================================================
// FSQRT - Large Values
// ============================================================================

#[test]
fn test_fsqrt_large_perfect_square() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xFA, // FSQRT
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 10000.0);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert_eq!(result, 100.0, "FSQRT of 10000.0 should be 100.0");
}

#[test]
fn test_fsqrt_very_large() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xFA, // FSQRT
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 1e100);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    let expected = 1e50;
    assert_eq!(result, expected, "FSQRT of 1e100 should be 1e50");
}

// ============================================================================
// FSQRT - Small Values
// ============================================================================

#[test]
fn test_fsqrt_very_small() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xFA, // FSQRT
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 1e-100);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    let expected = 1e-50;
    assert!(
        (result - expected).abs() < 1e-60,
        "FSQRT of 1e-100 should be approximately 1e-50"
    );
}

#[test]
fn test_fsqrt_min_positive() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xFA, // FSQRT
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, f64::MIN_POSITIVE);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert!(result > 0.0, "FSQRT of MIN_POSITIVE should be positive");
    assert!(result.is_finite(), "Result should be finite");
}

// ============================================================================
// FSQRT - Precision Tests
// ============================================================================

#[test]
fn test_fsqrt_precision_1() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xFA, // FSQRT
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 1.0);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert_eq!(result, 1.0, "FSQRT of 1.0 should be exactly 1.0");
}

#[test]
fn test_fsqrt_precision_check() {
    // Test that FSQRT is accurate to double precision
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xFA, // FSQRT
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let test_values = vec![2.0, 3.0, 5.0, 7.0, 10.0, 50.0, 123.456];

    for val in test_values {
        let (mut vcpu, mem) = setup_vm(&code, None);
        write_f64(&mem, 0x2000, val);

        run_until_hlt(&mut vcpu).unwrap();

        let result = read_f64(&mem, 0x3000);
        let expected = val.sqrt();
        let rel_error = ((result - expected) / expected).abs();
        assert!(
            rel_error < 1e-15,
            "FSQRT of {} has relative error {} (expected < 1e-15)",
            val,
            rel_error
        );
    }
}

// ============================================================================
// FSQRT - Idempotency and Combinations
// ============================================================================

#[test]
fn test_fsqrt_twice() {
    // FSQRT(FSQRT(16)) should be 2
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xFA, // FSQRT
        0xD9, 0xFA, // FSQRT (second time)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 16.0);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert_eq!(result, 2.0, "FSQRT(FSQRT(16)) should be 2");
}

#[test]
fn test_fsqrt_sequence() {
    // Test multiple FSQRT operations
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xFA, // FSQRT
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008]
        0xD9, 0xFA, // FSQRT
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, // FSTP qword [0x3008]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 64.0);
    write_f64(&mem, 0x2008, 81.0);

    run_until_hlt(&mut vcpu).unwrap();

    let result1 = read_f64(&mem, 0x3000);
    let result2 = read_f64(&mem, 0x3008);
    assert_eq!(result1, 8.0, "FSQRT(64) should be 8");
    assert_eq!(result2, 9.0, "FSQRT(81) should be 9");
}

#[test]
fn test_fsqrt_with_multiplication() {
    // Test FSQRT(4) * 3 = 2 * 3 = 6
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xFA, // FSQRT
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008]
        0xDE, 0xC9, // FMULP (multiply and pop)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 4.0);
    write_f64(&mem, 0x2008, 3.0);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert_eq!(result, 6.0, "FSQRT(4) * 3 should be 6");
}

// ============================================================================
// FSQRT - Additional Edge Cases
// ============================================================================

#[test]
fn test_fsqrt_max_value() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xFA, // FSQRT
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, f64::MAX);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert!(result.is_finite(), "FSQRT of MAX should be finite");
    assert!(result > 0.0, "FSQRT of MAX should be positive");
}

#[test]
fn test_fsqrt_inverse_property() {
    // Test that FSQRT(x) * FSQRT(x) = x
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xFA, // FSQRT
        0xD8, 0xC8, // FMUL ST(0), ST(0) (square the result)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    let test_val = 42.0;
    write_f64(&mem, 0x2000, test_val);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert!(
        (result - test_val).abs() < 1e-14,
        "FSQRT(x)^2 should equal x"
    );
}

#[test]
fn test_fsqrt_power_of_two() {
    // FSQRT(256) = 16
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xFA, // FSQRT
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 256.0);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert_eq!(result, 16.0, "FSQRT(256) should be 16");
}

#[test]
fn test_fsqrt_nested() {
    // FSQRT(FSQRT(256)) should be 4
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xFA, // FSQRT
        0xD9, 0xFA, // FSQRT (second)
        0xD9, 0xFA, // FSQRT (third)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 256.0);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert_eq!(result, 2.0, "FSQRT(FSQRT(FSQRT(256))) should be 2");
}

// ============================================================================
// Known-answer FSQRT tests: perfect squares give EXACT integer roots.
// ============================================================================

/// FLD [m64], FSQRT, FSTP [m64] -> returns result.
fn kat_fsqrt(value: f64) -> f64 {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xFA, // FSQRT
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, value);
    run_until_hlt(&mut vcpu).unwrap();
    read_f64(&mem, 0x3000)
}

#[test]
fn test_fsqrt_perfect_squares_exact() {
    assert_eq!(kat_fsqrt(0.0), 0.0);
    assert_eq!(kat_fsqrt(1.0), 1.0);
    assert_eq!(kat_fsqrt(4.0), 2.0);
    assert_eq!(kat_fsqrt(9.0), 3.0);
    assert_eq!(kat_fsqrt(144.0), 12.0);
    assert_eq!(kat_fsqrt(0.25), 0.5);
    assert_eq!(kat_fsqrt(1e8), 1e4);
}

#[test]
fn test_fsqrt_irrational_tolerance() {
    assert!((kat_fsqrt(2.0) - std::f64::consts::SQRT_2).abs() < 1e-15);
    assert!((kat_fsqrt(3.0) - 3.0_f64.sqrt()).abs() < 1e-15);
}

#[test]
fn test_fsqrt_special_values() {
    // sqrt(+inf) = +inf; sqrt(-1) = NaN (invalid, masked); sqrt(-0) = -0.
    assert!(kat_fsqrt(f64::INFINITY).is_infinite());
    assert!(kat_fsqrt(-1.0).is_nan());
    assert_eq!(kat_fsqrt(-0.0).to_bits(), (-0.0_f64).to_bits());
}
