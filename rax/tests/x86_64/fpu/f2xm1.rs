//! Tests for the F2XM1 instruction.
//!
//! F2XM1 - Compute 2^X - 1
//!
//! Computes the exponential value of 2 to the power of ST(0) minus 1.
//! The source operand must lie in the range -1.0 to +1.0.
//! If the source is outside this range, the result is undefined.
//! Result is stored back in ST(0).
//!
//! Opcode: D9 F0
//!
//! Formula: ST(0) := (2^ST(0)) - 1
//!
//! Used for exponentiation: x^y := 2^(y * log2(x))
//!
//! Flags affected:
//! - C1: Set to 0 if stack underflow; set if result rounded up, cleared otherwise
//! - C0, C2, C3: Undefined
//!
//! Reference: /Users/int/dev/rax/docs/f2xm1.txt

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
// F2XM1 - Zero
// ============================================================================

#[test]
fn test_f2xm1_zero() {
    // 2^0 - 1 = 1 - 1 = 0
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xF0, // F2XM1
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 0.0);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert!((result - 0.0).abs() < 1e-10, "2^0 - 1 should be 0");
}

#[test]
fn test_f2xm1_positive_zero() {
    // Explicitly test +0.0
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xF0, // F2XM1
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 0.0);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert_eq!(result, 0.0, "2^(+0) - 1 = +0");
    assert!(!result.is_sign_negative(), "Result should be positive zero");
}

#[test]
fn test_f2xm1_negative_zero() {
    // 2^(-0) - 1 = 1 - 1 = -0
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xF0, // F2XM1
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, -0.0);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert_eq!(result, 0.0, "2^(-0) - 1 = -0");
    assert!(result.is_sign_negative(), "Result should be negative zero");
}

// ============================================================================
// F2XM1 - One
// ============================================================================

#[test]
fn test_f2xm1_one() {
    // 2^1 - 1 = 2 - 1 = 1
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xF0, // F2XM1
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 1.0);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert!((result - 1.0).abs() < 1e-10, "2^1 - 1 should be 1.0");
}

// ============================================================================
// F2XM1 - Positive Values
// ============================================================================

#[test]
fn test_f2xm1_half() {
    // 2^0.5 - 1 = sqrt(2) - 1 ≈ 0.414
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xF0, // F2XM1
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 0.5);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    let expected = 2.0f64.powf(0.5) - 1.0;
    assert!(
        (result - expected).abs() < 1e-10,
        "2^0.5 - 1 should be approximately 0.414"
    );
}

#[test]
fn test_f2xm1_quarter() {
    // 2^0.25 - 1
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xF0, // F2XM1
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 0.25);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    let expected = 2.0f64.powf(0.25) - 1.0;
    assert!((result - expected).abs() < 1e-10, "2^0.25 - 1 calculation");
}

#[test]
fn test_f2xm1_three_quarters() {
    // 2^0.75 - 1
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xF0, // F2XM1
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 0.75);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    let expected = 2.0f64.powf(0.75) - 1.0;
    assert!((result - expected).abs() < 1e-10, "2^0.75 - 1 calculation");
}

#[test]
fn test_f2xm1_small_positive() {
    // 2^0.1 - 1
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xF0, // F2XM1
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 0.1);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    let expected = 2.0f64.powf(0.1) - 1.0;
    assert!((result - expected).abs() < 1e-10, "2^0.1 - 1 calculation");
}

#[test]
fn test_f2xm1_very_small_positive() {
    // 2^0.01 - 1
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xF0, // F2XM1
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 0.01);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    let expected = 2.0f64.powf(0.01) - 1.0;
    assert!((result - expected).abs() < 1e-10, "2^0.01 - 1 calculation");
}

// ============================================================================
// F2XM1 - Negative Values
// ============================================================================

#[test]
fn test_f2xm1_negative_one() {
    // 2^(-1) - 1 = 0.5 - 1 = -0.5
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xF0, // F2XM1
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, -1.0);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert!((result - (-0.5)).abs() < 1e-10, "2^(-1) - 1 should be -0.5");
}

#[test]
fn test_f2xm1_negative_half() {
    // 2^(-0.5) - 1 = 1/sqrt(2) - 1 ≈ -0.293
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xF0, // F2XM1
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, -0.5);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    let expected = 2.0f64.powf(-0.5) - 1.0;
    assert!(
        (result - expected).abs() < 1e-10,
        "2^(-0.5) - 1 calculation"
    );
}

#[test]
fn test_f2xm1_negative_quarter() {
    // 2^(-0.25) - 1
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xF0, // F2XM1
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, -0.25);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    let expected = 2.0f64.powf(-0.25) - 1.0;
    assert!(
        (result - expected).abs() < 1e-10,
        "2^(-0.25) - 1 calculation"
    );
}

#[test]
fn test_f2xm1_small_negative() {
    // 2^(-0.1) - 1
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xF0, // F2XM1
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, -0.1);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    let expected = 2.0f64.powf(-0.1) - 1.0;
    assert!(
        (result - expected).abs() < 1e-10,
        "2^(-0.1) - 1 calculation"
    );
}

#[test]
fn test_f2xm1_very_small_negative() {
    // 2^(-0.01) - 1
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xF0, // F2XM1
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, -0.01);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    let expected = 2.0f64.powf(-0.01) - 1.0;
    assert!(
        (result - expected).abs() < 1e-10,
        "2^(-0.01) - 1 calculation"
    );
}

// ============================================================================
// F2XM1 - Range Limits
// ============================================================================

#[test]
fn test_f2xm1_upper_limit() {
    // Test at upper limit of valid range (1.0)
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xF0, // F2XM1
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 1.0);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    let expected = 2.0f64.powf(1.0) - 1.0;
    assert!(
        (result - expected).abs() < 1e-10,
        "Upper limit: 2^1 - 1 = 1.0"
    );
}

#[test]
fn test_f2xm1_lower_limit() {
    // Test at lower limit of valid range (-1.0)
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xF0, // F2XM1
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, -1.0);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    let expected = 2.0f64.powf(-1.0) - 1.0;
    assert!(
        (result - expected).abs() < 1e-10,
        "Lower limit: 2^(-1) - 1 = -0.5"
    );
}

#[test]
fn test_f2xm1_near_upper_limit() {
    // Test near upper limit
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xF0, // F2XM1
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 0.99);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    let expected = 2.0f64.powf(0.99) - 1.0;
    assert!((result - expected).abs() < 1e-10, "2^0.99 - 1 calculation");
}

#[test]
fn test_f2xm1_near_lower_limit() {
    // Test near lower limit
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xF0, // F2XM1
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, -0.99);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    let expected = 2.0f64.powf(-0.99) - 1.0;
    assert!(
        (result - expected).abs() < 1e-10,
        "2^(-0.99) - 1 calculation"
    );
}

// ============================================================================
// F2XM1 - Precision Tests
// ============================================================================

#[test]
fn test_f2xm1_various_values() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xF0, // F2XM1
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let test_values = vec![-0.9, -0.75, -0.5, -0.25, -0.1, 0.1, 0.25, 0.5, 0.75, 0.9];

    for val in test_values {
        let (mut vcpu, mem) = setup_vm(&code, None);
        write_f64(&mem, 0x2000, val);

        run_until_hlt(&mut vcpu).unwrap();

        let result = read_f64(&mem, 0x3000);
        let expected = 2.0f64.powf(val) - 1.0;
        assert!(
            (result - expected).abs() < 1e-10,
            "2^{} - 1 should match",
            val
        );
    }
}

#[test]
fn test_f2xm1_symmetric_values() {
    // Test that F2XM1 works correctly for symmetric values
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xF0, // F2XM1
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let test_pairs = vec![(0.5, -0.5), (0.25, -0.25), (0.75, -0.75)];

    for (pos, neg) in test_pairs {
        let (mut vcpu_pos, mem_pos) = setup_vm(&code, None);
        write_f64(&mem_pos, 0x2000, pos);
        run_until_hlt(&mut vcpu_pos).unwrap();
        let result_pos = read_f64(&mem_pos, 0x3000);

        let (mut vcpu_neg, mem_neg) = setup_vm(&code, None);
        write_f64(&mem_neg, 0x2000, neg);
        run_until_hlt(&mut vcpu_neg).unwrap();
        let result_neg = read_f64(&mem_neg, 0x3000);

        let expected_pos = 2.0f64.powf(pos) - 1.0;
        let expected_neg = 2.0f64.powf(neg) - 1.0;

        assert!((result_pos - expected_pos).abs() < 1e-10, "2^{} - 1", pos);
        assert!((result_neg - expected_neg).abs() < 1e-10, "2^{} - 1", neg);
    }
}

// ============================================================================
// F2XM1 - Multiple Operations
// ============================================================================

#[test]
fn test_f2xm1_sequence() {
    // Test multiple F2XM1 operations in sequence
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xF0, // F2XM1
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008]
        0xD9, 0xF0, // F2XM1
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, // FSTP qword [0x3008]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 0.5);
    write_f64(&mem, 0x2008, -0.5);

    run_until_hlt(&mut vcpu).unwrap();

    let result1 = read_f64(&mem, 0x3000);
    let result2 = read_f64(&mem, 0x3008);
    let expected1 = 2.0f64.powf(0.5) - 1.0;
    let expected2 = 2.0f64.powf(-0.5) - 1.0;

    assert!((result1 - expected1).abs() < 1e-10, "First F2XM1");
    assert!((result2 - expected2).abs() < 1e-10, "Second F2XM1");
}

// ============================================================================
// F2XM1 - Edge Cases
// ============================================================================

#[test]
fn test_f2xm1_tiny_value() {
    // For very small x, 2^x - 1 ≈ x * ln(2)
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xF0, // F2XM1
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 0.001);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    let expected = 2.0f64.powf(0.001) - 1.0;
    assert!((result - expected).abs() < 1e-10, "2^0.001 - 1 calculation");
}

#[test]
fn test_f2xm1_precision_boundary() {
    // Test values at precision boundaries
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xF0, // F2XM1
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let test_values = vec![0.125, 0.375, 0.625, 0.875];

    for val in test_values {
        let (mut vcpu, mem) = setup_vm(&code, None);
        write_f64(&mem, 0x2000, val);

        run_until_hlt(&mut vcpu).unwrap();

        let result = read_f64(&mem, 0x3000);
        let expected = 2.0f64.powf(val) - 1.0;
        assert!((result - expected).abs() < 1e-10, "2^{} - 1", val);
    }
}

#[test]
fn test_f2xm1_fractional_precision() {
    // Test various fractional values for precision
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xF0, // F2XM1
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let test_values = vec![0.2, 0.3, 0.4, 0.6, 0.7, 0.8];

    for val in test_values {
        let (mut vcpu, mem) = setup_vm(&code, None);
        write_f64(&mem, 0x2000, val);

        run_until_hlt(&mut vcpu).unwrap();

        let result = read_f64(&mem, 0x3000);
        let expected = 2.0f64.powf(val) - 1.0;
        assert!(
            (result - expected).abs() < 1e-10,
            "2^{} - 1 should match",
            val
        );
    }
}

// ============================================================================
// Known-answer F2XM1 tests (transcendental; small tolerance).
// Result = 2^ST(0) - 1, ST(0) replaced in place. Valid domain is [-1, +1].
// ============================================================================

/// FLD [m64], F2XM1, FSTP [m64] -> returns result.
fn kat_f2xm1(value: f64) -> f64 {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xF0, // F2XM1
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, value);
    run_until_hlt(&mut vcpu).unwrap();
    read_f64(&mem, 0x3000)
}

#[test]
fn test_f2xm1_known_values() {
    let tol = 1e-12;
    assert!(kat_f2xm1(0.0).abs() < tol, "2^0 - 1 = 0");
    assert!((kat_f2xm1(1.0) - 1.0).abs() < tol, "2^1 - 1 = 1");
    assert!((kat_f2xm1(-1.0) + 0.5).abs() < tol, "2^-1 - 1 = -0.5");
    assert!(
        (kat_f2xm1(0.5) - (std::f64::consts::SQRT_2 - 1.0)).abs() < tol,
        "2^0.5 - 1"
    );
}

#[test]
fn test_f2xm1_signed_zero_preserved() {
    // The implementation preserves the sign of a zero input.
    assert_eq!(kat_f2xm1(-0.0).to_bits(), (-0.0_f64).to_bits());
    assert_eq!(kat_f2xm1(0.0).to_bits(), (0.0_f64).to_bits());
}
