//! Tests for the FRNDINT instruction.
//!
//! FRNDINT - Round to Integer
//!
//! Rounds the source value in ST(0) to the nearest integral value according to
//! the current rounding mode (RC field of FPU control word).
//! If the source value is infinity, it remains unchanged.
//! If the value is not integral, the inexact-result exception (#P) is generated.
//!
//! Opcode: D9 FC
//!
//! Flags affected:
//! - C1: Set to 0 if stack underflow; set if result rounded up, cleared otherwise
//! - C0, C2, C3: Undefined
//!
//! Reference: /Users/int/dev/rax/docs/frndint.txt

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

// Helper function to write FPU control word
fn write_fpu_control_word(mem: &vm_memory::GuestMemoryMmap, addr: u64, cw: u16) {
    mem.write_slice(&cw.to_le_bytes(), GuestAddress(addr))
        .unwrap();
}

// ============================================================================
// FRNDINT - Round to Nearest (Default Rounding Mode)
// ============================================================================

#[test]
fn test_frndint_positive_round_down() {
    // Test 3.2 rounds to 3.0
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xFC, // FRNDINT
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 3.2);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert_eq!(result, 3.0, "FRNDINT of 3.2 should round to 3.0");
}

#[test]
fn test_frndint_positive_round_up() {
    // Test 3.8 rounds to 4.0
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xFC, // FRNDINT
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 3.8);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert_eq!(result, 4.0, "FRNDINT of 3.8 should round to 4.0");
}

#[test]
fn test_frndint_positive_half() {
    // Test 2.5 rounds to nearest even (2.0)
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xFC, // FRNDINT
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 2.5);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert_eq!(result, 2.0, "FRNDINT of 2.5 should round to even (2.0)");
}

#[test]
fn test_frndint_positive_half_odd() {
    // Test 3.5 rounds to nearest even (4.0)
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xFC, // FRNDINT
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 3.5);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert_eq!(result, 4.0, "FRNDINT of 3.5 should round to even (4.0)");
}

// ============================================================================
// FRNDINT - Negative Values
// ============================================================================

#[test]
fn test_frndint_negative_round_up() {
    // Test -3.2 rounds to -3.0
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xFC, // FRNDINT
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, -3.2);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert_eq!(result, -3.0, "FRNDINT of -3.2 should round to -3.0");
}

#[test]
fn test_frndint_negative_round_down() {
    // Test -3.8 rounds to -4.0
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xFC, // FRNDINT
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, -3.8);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert_eq!(result, -4.0, "FRNDINT of -3.8 should round to -4.0");
}

#[test]
fn test_frndint_negative_half() {
    // Test -2.5 rounds to nearest even (-2.0)
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xFC, // FRNDINT
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, -2.5);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert_eq!(result, -2.0, "FRNDINT of -2.5 should round to even (-2.0)");
}

#[test]
fn test_frndint_negative_half_odd() {
    // Test -3.5 rounds to nearest even (-4.0)
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xFC, // FRNDINT
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, -3.5);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert_eq!(result, -4.0, "FRNDINT of -3.5 should round to even (-4.0)");
}

// ============================================================================
// FRNDINT - Values Already Integer
// ============================================================================

#[test]
fn test_frndint_already_integer_positive() {
    // Test 5.0 remains 5.0
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xFC, // FRNDINT
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 5.0);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert_eq!(result, 5.0, "FRNDINT of 5.0 should remain 5.0");
}

#[test]
fn test_frndint_already_integer_negative() {
    // Test -7.0 remains -7.0
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xFC, // FRNDINT
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, -7.0);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert_eq!(result, -7.0, "FRNDINT of -7.0 should remain -7.0");
}

#[test]
fn test_frndint_already_integer_large() {
    // Test large integer value
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xFC, // FRNDINT
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 123456789.0);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert_eq!(
        result, 123456789.0,
        "FRNDINT of large integer should remain unchanged"
    );
}

// ============================================================================
// FRNDINT - Special Values
// ============================================================================

#[test]
fn test_frndint_positive_zero() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xFC, // FRNDINT
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 0.0);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert_eq!(result, 0.0, "FRNDINT of +0.0 should be +0.0");
    assert!(!result.is_sign_negative(), "Result should be positive zero");
}

#[test]
fn test_frndint_negative_zero() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xFC, // FRNDINT
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, -0.0);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert_eq!(result, 0.0, "FRNDINT of -0.0 should be -0.0");
    assert!(result.is_sign_negative(), "Result should be negative zero");
}

#[test]
fn test_frndint_positive_infinity() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xFC, // FRNDINT
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, f64::INFINITY);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert!(
        result.is_infinite(),
        "FRNDINT of +infinity should remain +infinity"
    );
    assert!(
        !result.is_sign_negative(),
        "Result should be positive infinity"
    );
}

#[test]
fn test_frndint_negative_infinity() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xFC, // FRNDINT
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, f64::NEG_INFINITY);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert!(
        result.is_infinite(),
        "FRNDINT of -infinity should remain -infinity"
    );
    assert!(
        result.is_sign_negative(),
        "Result should be negative infinity"
    );
}

#[test]
fn test_frndint_nan() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xFC, // FRNDINT
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, f64::NAN);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert!(result.is_nan(), "FRNDINT of NaN should remain NaN");
}

// ============================================================================
// FRNDINT - Small Fractional Values
// ============================================================================

#[test]
fn test_frndint_small_positive_fraction() {
    // Test 0.1 rounds to 0.0
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xFC, // FRNDINT
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 0.1);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert_eq!(result, 0.0, "FRNDINT of 0.1 should round to 0.0");
}

#[test]
fn test_frndint_small_negative_fraction() {
    // Test -0.1 rounds to -0.0
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xFC, // FRNDINT
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, -0.1);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert_eq!(result, 0.0, "FRNDINT of -0.1 should round to -0.0");
}

#[test]
fn test_frndint_near_one() {
    // Test 0.9 rounds to 1.0
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xFC, // FRNDINT
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 0.9);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert_eq!(result, 1.0, "FRNDINT of 0.9 should round to 1.0");
}

#[test]
fn test_frndint_near_negative_one() {
    // Test -0.9 rounds to -1.0
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xFC, // FRNDINT
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, -0.9);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert_eq!(result, -1.0, "FRNDINT of -0.9 should round to -1.0");
}

// ============================================================================
// FRNDINT - Large Values
// ============================================================================

#[test]
fn test_frndint_large_positive() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xFC, // FRNDINT
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 123456789.7);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert_eq!(
        result, 123456790.0,
        "FRNDINT of large value should round correctly"
    );
}

#[test]
fn test_frndint_large_negative() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xFC, // FRNDINT
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, -987654321.3);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert_eq!(
        result, -987654321.0,
        "FRNDINT of large negative should round correctly"
    );
}

// ============================================================================
// FRNDINT - Mathematical Constants
// ============================================================================

#[test]
fn test_frndint_pi() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xFC, // FRNDINT
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, std::f64::consts::PI);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert_eq!(result, 3.0, "FRNDINT of PI should round to 3.0");
}

#[test]
fn test_frndint_e() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xFC, // FRNDINT
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, std::f64::consts::E);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert_eq!(result, 3.0, "FRNDINT of E should round to 3.0");
}

// ============================================================================
// FRNDINT - Multiple Operations
// ============================================================================

#[test]
fn test_frndint_sequence() {
    // Test multiple FRNDINT operations in sequence
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xFC, // FRNDINT
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008]
        0xD9, 0xFC, // FRNDINT
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, // FSTP qword [0x3008]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 1.7);
    write_f64(&mem, 0x2008, -2.3);

    run_until_hlt(&mut vcpu).unwrap();

    let result1 = read_f64(&mem, 0x3000);
    let result2 = read_f64(&mem, 0x3008);
    assert_eq!(result1, 2.0, "First FRNDINT result");
    assert_eq!(result2, -2.0, "Second FRNDINT result");
}

#[test]
fn test_frndint_idempotent() {
    // FRNDINT on an integer should be idempotent
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xFC, // FRNDINT
        0xD9, 0xFC, // FRNDINT (second time)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 4.9);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert_eq!(result, 5.0, "Double FRNDINT should give same result");
}

// ============================================================================
// FRNDINT - Edge Cases
// ============================================================================

#[test]
fn test_frndint_very_large_value() {
    // Values so large they are already integers
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xFC, // FRNDINT
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 1e100);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert_eq!(result, 1e100, "Very large value should remain unchanged");
}

#[test]
fn test_frndint_one_half() {
    // Test exact 0.5 (rounds to even, which is 0.0)
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xFC, // FRNDINT
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 0.5);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert_eq!(result, 0.0, "FRNDINT of 0.5 should round to even (0.0)");
}

#[test]
fn test_frndint_one_and_half() {
    // Test 1.5 (rounds to even, which is 2.0)
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xFC, // FRNDINT
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 1.5);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert_eq!(result, 2.0, "FRNDINT of 1.5 should round to even (2.0)");
}

#[test]
fn test_frndint_negative_one_half() {
    // Test -0.5 (rounds to even, which is -0.0)
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xFC, // FRNDINT
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, -0.5);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert_eq!(result, 0.0, "FRNDINT of -0.5 should round to even (-0.0)");
}

#[test]
fn test_frndint_negative_one_and_half() {
    // Test -1.5 (rounds to even, which is -2.0)
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xFC, // FRNDINT
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, -1.5);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert_eq!(result, -2.0, "FRNDINT of -1.5 should round to even (-2.0)");
}

#[test]
fn test_frndint_various_fractions() {
    // Test a range of fractional values
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xFC, // FRNDINT
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let test_cases = vec![
        (0.25, 0.0),
        (0.75, 1.0),
        (1.25, 1.0),
        (1.75, 2.0),
        (-0.25, 0.0),
        (-0.75, -1.0),
        (-1.25, -1.0),
        (-1.75, -2.0),
    ];

    for (input, expected) in test_cases {
        let (mut vcpu, mem) = setup_vm(&code, None);
        write_f64(&mem, 0x2000, input);

        run_until_hlt(&mut vcpu).unwrap();

        let result = read_f64(&mem, 0x3000);
        assert_eq!(
            result, expected,
            "FRNDINT of {} should be {}",
            input, expected
        );
    }
}

// ============================================================================
// Known-answer FRNDINT tests under EACH rounding mode (set via FLDCW).
//
// RC field (control word bits 10-11):
//   00 = round to nearest even (CW 0x037F)
//   01 = round down toward -inf  (CW 0x077F)
//   10 = round up toward +inf    (CW 0x0B7F)
//   11 = round toward zero/trunc (CW 0x0F7F)
// ============================================================================

/// Load control word `cw` via FLDCW, then FRNDINT the given value, then store.
fn kat_frndint_rc(value: f64, cw: u16) -> f64 {
    let code = [
        0xD9, 0x2C, 0x25, 0x10, 0x20, 0x00, 0x00, // FLDCW [0x2010]
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xFC, // FRNDINT
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, value);
    write_fpu_control_word(&mem, 0x2010, cw);
    run_until_hlt(&mut vcpu).unwrap();
    read_f64(&mem, 0x3000)
}

const RC_NEAREST: u16 = 0x037F;
const RC_DOWN: u16 = 0x077F;
const RC_UP: u16 = 0x0B7F;
const RC_TRUNC: u16 = 0x0F7F;

#[test]
fn test_frndint_round_nearest_even_mode() {
    assert_eq!(
        kat_frndint_rc(2.5, RC_NEAREST),
        2.0,
        "2.5 ties to even -> 2"
    );
    assert_eq!(
        kat_frndint_rc(3.5, RC_NEAREST),
        4.0,
        "3.5 ties to even -> 4"
    );
    assert_eq!(kat_frndint_rc(2.4, RC_NEAREST), 2.0);
    assert_eq!(kat_frndint_rc(2.6, RC_NEAREST), 3.0);
}

#[test]
fn test_frndint_round_down_mode() {
    assert_eq!(kat_frndint_rc(2.9, RC_DOWN), 2.0, "floor(2.9)");
    assert_eq!(kat_frndint_rc(2.1, RC_DOWN), 2.0);
    assert_eq!(kat_frndint_rc(-2.1, RC_DOWN), -3.0, "floor(-2.1)");
    assert_eq!(kat_frndint_rc(-2.9, RC_DOWN), -3.0);
}

#[test]
fn test_frndint_round_up_mode() {
    assert_eq!(kat_frndint_rc(2.1, RC_UP), 3.0, "ceil(2.1)");
    assert_eq!(kat_frndint_rc(2.9, RC_UP), 3.0);
    assert_eq!(kat_frndint_rc(-2.9, RC_UP), -2.0, "ceil(-2.9)");
    assert_eq!(kat_frndint_rc(-2.1, RC_UP), -2.0);
}

#[test]
fn test_frndint_round_truncate_mode() {
    assert_eq!(kat_frndint_rc(2.9, RC_TRUNC), 2.0, "trunc(2.9)");
    assert_eq!(kat_frndint_rc(-2.9, RC_TRUNC), -2.0, "trunc(-2.9)");
    assert_eq!(kat_frndint_rc(2.1, RC_TRUNC), 2.0);
    assert_eq!(kat_frndint_rc(-2.1, RC_TRUNC), -2.0);
}
