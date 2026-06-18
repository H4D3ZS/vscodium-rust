//! Tests for the FSCALE instruction.
//!
//! FSCALE - Scale
//!
//! Truncates the value in the source operand (toward 0) to an integral value and
//! adds that value to the exponent of the destination operand. The destination and
//! source operands are floating-point values located in registers ST(0) and ST(1),
//! respectively. This instruction provides rapid multiplication or division by integral
//! powers of 2.
//!
//! In most cases, only the exponent is changed and the mantissa (significand) remains
//! unchanged. However, when the value being scaled in ST(0) is a denormal value, the
//! mantissa is also changed and the result may turn out to be a normalized number.
//! Similarly, if overflow or underflow results from a scale operation, the resulting
//! mantissa will differ from the source's mantissa.
//!
//! Opcode: D9 FD
//!
//! Operation: ST(0) := ST(0) * 2^RoundTowardZero(ST(1))
//!
//! Flags affected:
//! - C1: Set to 0 if stack underflow occurred; Set if result was rounded up
//! - C0, C2, C3: Undefined
//!
//! Reference: /Users/int/dev/rax/docs/fscale.txt

use crate::common::*;
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
// FSCALE Tests: Scaling by Positive Integer Powers
// ============================================================================

#[test]
fn test_fscale_multiply_by_2() {
    // Test FSCALE with scale factor 1 (multiply by 2^1 = 2)
    // FLD qword [0x2000]  ; Load value to scale (ST(0))
    // FLD qword [0x2008]  ; Load scale factor (ST(0), value becomes ST(1))
    // FSCALE              ; ST(0) = ST(1) * 2^trunc(ST(0))
    // FSTP qword [0x3000] ; Store result
    // HLT
    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008]
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xFD, // FSCALE
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xDD, 0xD8, // FSTP ST(0) (clean stack)
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 5.0); // Value to scale
    write_f64(&mem, 0x2008, 1.0); // Scale factor

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert!((result - 10.0).abs() < 1e-15, "5.0 * 2^1 should be 10.0");
}

#[test]
fn test_fscale_multiply_by_4() {
    // Test FSCALE with scale factor 2 (multiply by 2^2 = 4)
    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xD9,
        0xFD, 0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xDD, 0xD8, 0xF4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 3.0);
    write_f64(&mem, 0x2008, 2.0);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert!((result - 12.0).abs() < 1e-15, "3.0 * 2^2 should be 12.0");
}

#[test]
fn test_fscale_multiply_by_8() {
    // Test FSCALE with scale factor 3 (multiply by 2^3 = 8)
    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xD9,
        0xFD, 0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xDD, 0xD8, 0xF4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 7.5);
    write_f64(&mem, 0x2008, 3.0);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert!((result - 60.0).abs() < 1e-14, "7.5 * 2^3 should be 60.0");
}

#[test]
fn test_fscale_multiply_by_1024() {
    // Test FSCALE with scale factor 10 (multiply by 2^10 = 1024)
    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xD9,
        0xFD, 0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xDD, 0xD8, 0xF4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 1.0);
    write_f64(&mem, 0x2008, 10.0);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert!(
        (result - 1024.0).abs() < 1e-13,
        "1.0 * 2^10 should be 1024.0"
    );
}

// ============================================================================
// FSCALE Tests: Scaling by Negative Integer Powers (Division)
// ============================================================================

#[test]
fn test_fscale_divide_by_2() {
    // Test FSCALE with scale factor -1 (divide by 2^1 = 2)
    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xD9,
        0xFD, 0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xDD, 0xD8, 0xF4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 10.0);
    write_f64(&mem, 0x2008, -1.0);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert!((result - 5.0).abs() < 1e-15, "10.0 * 2^-1 should be 5.0");
}

#[test]
fn test_fscale_divide_by_4() {
    // Test FSCALE with scale factor -2 (divide by 2^2 = 4)
    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xD9,
        0xFD, 0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xDD, 0xD8, 0xF4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 20.0);
    write_f64(&mem, 0x2008, -2.0);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert!((result - 5.0).abs() < 1e-15, "20.0 * 2^-2 should be 5.0");
}

#[test]
fn test_fscale_divide_by_8() {
    // Test FSCALE with scale factor -3 (divide by 2^3 = 8)
    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xD9,
        0xFD, 0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xDD, 0xD8, 0xF4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 64.0);
    write_f64(&mem, 0x2008, -3.0);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert!((result - 8.0).abs() < 1e-15, "64.0 * 2^-3 should be 8.0");
}

#[test]
fn test_fscale_divide_by_1024() {
    // Test FSCALE with scale factor -10 (divide by 2^10 = 1024)
    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xD9,
        0xFD, 0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xDD, 0xD8, 0xF4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 2048.0);
    write_f64(&mem, 0x2008, -10.0);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert!((result - 2.0).abs() < 1e-14, "2048.0 * 2^-10 should be 2.0");
}

// ============================================================================
// FSCALE Tests: Scale Factor Zero
// ============================================================================

#[test]
fn test_fscale_zero_scale_factor() {
    // Test FSCALE with scale factor 0 (no change)
    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xD9,
        0xFD, 0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xDD, 0xD8, 0xF4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 42.0);
    write_f64(&mem, 0x2008, 0.0);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert!((result - 42.0).abs() < 1e-15, "42.0 * 2^0 should be 42.0");
}

#[test]
fn test_fscale_positive_zero() {
    // Test FSCALE with +0.0 scale factor
    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xD9,
        0xFD, 0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xDD, 0xD8, 0xF4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 99.5);
    write_f64(&mem, 0x2008, 0.0);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert!(
        (result - 99.5).abs() < 1e-14,
        "Value should remain unchanged"
    );
}

#[test]
fn test_fscale_negative_zero() {
    // Test FSCALE with -0.0 scale factor
    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xD9,
        0xFD, 0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xDD, 0xD8, 0xF4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 77.7);
    write_f64(&mem, 0x2008, -0.0);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert!(
        (result - 77.7).abs() < 1e-14,
        "Value should remain unchanged"
    );
}

// ============================================================================
// FSCALE Tests: Fractional Scale Factors (Truncation)
// ============================================================================

#[test]
fn test_fscale_truncate_positive_fraction() {
    // Test FSCALE with scale factor 2.9 (truncates to 2)
    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xD9,
        0xFD, 0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xDD, 0xD8, 0xF4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 1.0);
    write_f64(&mem, 0x2008, 2.9);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    // 2.9 truncates to 2, so 1.0 * 2^2 = 4.0
    assert!(
        (result - 4.0).abs() < 1e-15,
        "Scale factor 2.9 should truncate to 2"
    );
}

#[test]
fn test_fscale_truncate_negative_fraction() {
    // Test FSCALE with scale factor -2.9 (truncates to -2)
    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xD9,
        0xFD, 0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xDD, 0xD8, 0xF4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 16.0);
    write_f64(&mem, 0x2008, -2.9);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    // -2.9 truncates to -2, so 16.0 * 2^-2 = 4.0
    assert!(
        (result - 4.0).abs() < 1e-15,
        "Scale factor -2.9 should truncate to -2"
    );
}

#[test]
fn test_fscale_truncate_small_positive_fraction() {
    // Test FSCALE with scale factor 0.9 (truncates to 0)
    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xD9,
        0xFD, 0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xDD, 0xD8, 0xF4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 5.0);
    write_f64(&mem, 0x2008, 0.9);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    // 0.9 truncates to 0, so 5.0 * 2^0 = 5.0
    assert!(
        (result - 5.0).abs() < 1e-15,
        "Scale factor 0.9 should truncate to 0"
    );
}

#[test]
fn test_fscale_truncate_small_negative_fraction() {
    // Test FSCALE with scale factor -0.9 (truncates to 0)
    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xD9,
        0xFD, 0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xDD, 0xD8, 0xF4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 7.0);
    write_f64(&mem, 0x2008, -0.9);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    // -0.9 truncates to 0, so 7.0 * 2^0 = 7.0
    assert!(
        (result - 7.0).abs() < 1e-15,
        "Scale factor -0.9 should truncate to 0"
    );
}

// ============================================================================
// FSCALE Tests: Negative Values
// ============================================================================

#[test]
fn test_fscale_negative_value_positive_scale() {
    // Test FSCALE with negative value and positive scale
    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xD9,
        0xFD, 0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xDD, 0xD8, 0xF4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, -3.0);
    write_f64(&mem, 0x2008, 2.0);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert!(
        (result - (-12.0)).abs() < 1e-15,
        "-3.0 * 2^2 should be -12.0"
    );
}

#[test]
fn test_fscale_negative_value_negative_scale() {
    // Test FSCALE with negative value and negative scale
    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xD9,
        0xFD, 0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xDD, 0xD8, 0xF4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, -8.0);
    write_f64(&mem, 0x2008, -2.0);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert!(
        (result - (-2.0)).abs() < 1e-15,
        "-8.0 * 2^-2 should be -2.0"
    );
}

// ============================================================================
// FSCALE Tests: Special Cases with Zeros
// ============================================================================

#[test]
fn test_fscale_zero_value() {
    // Test FSCALE with value = 0
    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xD9,
        0xFD, 0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xDD, 0xD8, 0xF4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 0.0);
    write_f64(&mem, 0x2008, 5.0);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert!(
        result == 0.0 && !result.is_sign_negative(),
        "0.0 * 2^5 should be +0.0"
    );
}

#[test]
fn test_fscale_negative_zero_value() {
    // Test FSCALE with value = -0
    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xD9,
        0xFD, 0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xDD, 0xD8, 0xF4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, -0.0);
    write_f64(&mem, 0x2008, 5.0);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert!(
        result == 0.0 && result.is_sign_negative(),
        "-0.0 * 2^5 should be -0.0"
    );
}

// ============================================================================
// FSCALE Tests: FXTRACT Reversal
// ============================================================================

#[test]
fn test_fscale_fxtract_reversal() {
    // Test that FXTRACT followed by FSCALE restores original value
    // As documented: FXTRACT; FSCALE; FSTP ST(1);
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xF4, // FXTRACT (ST(0)=sig, ST(1)=exp)
        0xD9, 0xFD, // FSCALE (ST(0) = sig * 2^exp)
        0xDD, 0xD9, // FSTP ST(1) (pop exponent)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    let original = 123.456;
    write_f64(&mem, 0x2000, original);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert!(
        (result - original).abs() < 1e-14,
        "FXTRACT followed by FSCALE should restore original value"
    );
}

#[test]
fn test_fscale_fxtract_reversal_negative() {
    // Test FXTRACT + FSCALE reversal with negative value
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xD9, 0xF4, 0xD9, 0xFD, 0xDD, 0xD9, 0xDD, 0x1C,
        0x25, 0x00, 0x30, 0x00, 0x00, 0xF4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    let original = -789.123;
    write_f64(&mem, 0x2000, original);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert!(
        (result - original).abs() < 1e-13,
        "FXTRACT followed by FSCALE should restore negative value"
    );
}

#[test]
fn test_fscale_fxtract_reversal_small() {
    // Test FXTRACT + FSCALE reversal with small value
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xD9, 0xF4, 0xD9, 0xFD, 0xDD, 0xD9, 0xDD, 0x1C,
        0x25, 0x00, 0x30, 0x00, 0x00, 0xF4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    let original = 0.00123;
    write_f64(&mem, 0x2000, original);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert!(
        (result - original).abs() < 1e-17,
        "FXTRACT followed by FSCALE should restore small value"
    );
}

// ============================================================================
// FSCALE Tests: Large Scale Factors
// ============================================================================

#[test]
fn test_fscale_large_positive_scale() {
    // Test FSCALE with large positive scale factor
    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xD9,
        0xFD, 0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xDD, 0xD8, 0xF4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 1.0);
    write_f64(&mem, 0x2008, 100.0);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    let expected = 2.0_f64.powf(100.0);
    assert!(
        (result - expected).abs() / expected < 1e-15,
        "1.0 * 2^100 should match"
    );
}

#[test]
fn test_fscale_large_negative_scale() {
    // Test FSCALE with large negative scale factor
    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xD9,
        0xFD, 0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xDD, 0xD8, 0xF4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 1.0);
    write_f64(&mem, 0x2008, -100.0);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    let expected = 2.0_f64.powf(-100.0);
    assert!(
        (result - expected).abs() / expected < 1e-15,
        "1.0 * 2^-100 should match"
    );
}

// ============================================================================
// FSCALE Tests: Various Values
// ============================================================================

#[test]
fn test_fscale_pi_scale_by_4() {
    // Test scaling π by 2^4 = 16
    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xD9,
        0xFD, 0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xDD, 0xD8, 0xF4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, std::f64::consts::PI);
    write_f64(&mem, 0x2008, 4.0);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    let expected = std::f64::consts::PI * 16.0;
    assert!(
        (result - expected).abs() < 1e-13,
        "π * 2^4 should be π * 16"
    );
}

#[test]
fn test_fscale_e_scale_by_minus_3() {
    // Test scaling e by 2^-3 = 1/8
    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xD9,
        0xFD, 0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xDD, 0xD8, 0xF4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, std::f64::consts::E);
    write_f64(&mem, 0x2008, -3.0);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    let expected = std::f64::consts::E / 8.0;
    assert!(
        (result - expected).abs() < 1e-15,
        "e * 2^-3 should be e / 8"
    );
}

#[test]
fn test_fscale_series() {
    // Test a series of scale operations
    let test_cases = [
        (1.0, 0.0, 1.0),
        (1.0, 1.0, 2.0),
        (1.0, 2.0, 4.0),
        (1.0, 3.0, 8.0),
        (2.0, 2.0, 8.0),
        (3.0, 1.0, 6.0),
        (5.0, -1.0, 2.5),
        (100.0, -3.0, 12.5),
    ];

    for &(value, scale, expected) in &test_cases {
        let code = [
            0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
            0xD9, 0xFD, 0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xDD, 0xD8, 0xF4,
        ];

        let (mut vcpu, mem) = setup_vm(&code, None);
        write_f64(&mem, 0x2000, value);
        write_f64(&mem, 0x2008, scale);

        run_until_hlt(&mut vcpu).unwrap();

        let result = read_f64(&mem, 0x3000);
        assert!(
            (result - expected).abs() < 1e-14,
            "{} * 2^{} should be {}, got {}",
            value,
            scale,
            expected,
            result
        );
    }
}

// ============================================================================
// Known-answer FSCALE tests: ST(0) = ST(0) * 2^trunc(ST(1)), EXACT for
// power-of-two friendly inputs. ST(1) is truncated toward zero first.
// ============================================================================

/// ST(1)=exponent (loaded first), ST(0)=value (loaded second), FSCALE, store.
fn kat_fscale(value: f64, exp: f64) -> f64 {
    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008] (exp -> ST(1))
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000] (value -> ST(0))
        0xD9, 0xFD, // FSCALE
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, value);
    write_f64(&mem, 0x2008, exp);
    run_until_hlt(&mut vcpu).unwrap();
    read_f64(&mem, 0x3000)
}

#[test]
fn test_fscale_exact_powers() {
    assert_eq!(kat_fscale(3.0, 0.0), 3.0);
    assert_eq!(kat_fscale(3.0, 1.0), 6.0);
    assert_eq!(kat_fscale(3.0, 4.0), 48.0);
    assert_eq!(kat_fscale(1.0, 10.0), 1024.0);
    assert_eq!(kat_fscale(8.0, -3.0), 1.0);
    assert_eq!(kat_fscale(1.0, -1.0), 0.5);
}

#[test]
fn test_fscale_truncates_exponent() {
    // ST(1) is truncated toward zero: 2.9 -> 2, -2.9 -> -2.
    assert_eq!(kat_fscale(1.0, 2.9), 4.0, "2.9 trunc -> 2 -> *4");
    assert_eq!(kat_fscale(1.0, -2.9), 0.25, "-2.9 trunc -> -2 -> *0.25");
    assert_eq!(kat_fscale(5.0, 1.5), 10.0, "1.5 trunc -> 1 -> *2");
}

#[test]
fn test_fscale_zero_and_sign() {
    assert_eq!(kat_fscale(0.0, 5.0), 0.0);
    assert_eq!(kat_fscale(-3.0, 2.0), -12.0);
}
