//! Tests for the FXTRACT instruction.
//!
//! FXTRACT - Extract Exponent and Significand
//!
//! Separates the source value in the ST(0) register into its exponent and significand,
//! stores the exponent in ST(0), and pushes the significand onto the register stack.
//! Following this operation, the new top-of-stack register ST(0) contains the value
//! of the original significand expressed as a floating-point value. The sign and
//! significand of this value are the same as those found in the source operand, and
//! the exponent is 3FFFH (biased value for a true exponent of zero). The ST(1) register
//! contains the value of the original operand's true (unbiased) exponent expressed as
//! a floating-point value.
//!
//! Opcode: D9 F4
//!
//! Operation:
//! TEMP := Significand(ST(0));
//! ST(0) := Exponent(ST(0));
//! TOP := TOP - 1;
//! ST(0) := TEMP;
//!
//! Flags affected:
//! - C1: Set to 0 if stack underflow occurred; set to 1 if stack overflow occurred
//! - C0, C2, C3: Undefined
//!
//! Reference: /Users/int/dev/rax/docs/fxtract.txt

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
// FXTRACT Tests: Powers of Two
// ============================================================================

#[test]
fn test_fxtract_one() {
    // Test FXTRACT on 1.0 (2^0)
    // 1.0 has exponent 0 and significand 1.0
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xF4, // FXTRACT
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000] (significand)
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, // FSTP qword [0x3008] (exponent)
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 1.0);

    run_until_hlt(&mut vcpu).unwrap();

    let significand = read_f64(&mem, 0x3000);
    let exponent = read_f64(&mem, 0x3008);

    assert!(
        (significand - 1.0).abs() < 1e-15,
        "Significand of 1.0 should be 1.0"
    );
    assert!(
        (exponent - 0.0).abs() < 1e-15,
        "Exponent of 1.0 should be 0.0"
    );
}

#[test]
fn test_fxtract_two() {
    // Test FXTRACT on 2.0 (2^1)
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xD9, 0xF4, 0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00,
        0x00, 0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, 0xF4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 2.0);

    run_until_hlt(&mut vcpu).unwrap();

    let significand = read_f64(&mem, 0x3000);
    let exponent = read_f64(&mem, 0x3008);

    assert!(
        (significand - 1.0).abs() < 1e-15,
        "Significand of 2.0 should be 1.0"
    );
    assert!(
        (exponent - 1.0).abs() < 1e-15,
        "Exponent of 2.0 should be 1.0"
    );
}

#[test]
fn test_fxtract_four() {
    // Test FXTRACT on 4.0 (2^2)
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xD9, 0xF4, 0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00,
        0x00, 0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, 0xF4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 4.0);

    run_until_hlt(&mut vcpu).unwrap();

    let significand = read_f64(&mem, 0x3000);
    let exponent = read_f64(&mem, 0x3008);

    assert!(
        (significand - 1.0).abs() < 1e-15,
        "Significand of 4.0 should be 1.0"
    );
    assert!(
        (exponent - 2.0).abs() < 1e-15,
        "Exponent of 4.0 should be 2.0"
    );
}

#[test]
fn test_fxtract_eight() {
    // Test FXTRACT on 8.0 (2^3)
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xD9, 0xF4, 0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00,
        0x00, 0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, 0xF4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 8.0);

    run_until_hlt(&mut vcpu).unwrap();

    let significand = read_f64(&mem, 0x3000);
    let exponent = read_f64(&mem, 0x3008);

    assert!(
        (significand - 1.0).abs() < 1e-15,
        "Significand of 8.0 should be 1.0"
    );
    assert!(
        (exponent - 3.0).abs() < 1e-15,
        "Exponent of 8.0 should be 3.0"
    );
}

#[test]
fn test_fxtract_large_power_of_two() {
    // Test FXTRACT on 1024.0 (2^10)
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xD9, 0xF4, 0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00,
        0x00, 0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, 0xF4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 1024.0);

    run_until_hlt(&mut vcpu).unwrap();

    let significand = read_f64(&mem, 0x3000);
    let exponent = read_f64(&mem, 0x3008);

    assert!(
        (significand - 1.0).abs() < 1e-15,
        "Significand of 1024.0 should be 1.0"
    );
    assert!(
        (exponent - 10.0).abs() < 1e-15,
        "Exponent of 1024.0 should be 10.0"
    );
}

// ============================================================================
// FXTRACT Tests: Fractional Powers of Two
// ============================================================================

#[test]
fn test_fxtract_half() {
    // Test FXTRACT on 0.5 (2^-1)
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xD9, 0xF4, 0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00,
        0x00, 0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, 0xF4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 0.5);

    run_until_hlt(&mut vcpu).unwrap();

    let significand = read_f64(&mem, 0x3000);
    let exponent = read_f64(&mem, 0x3008);

    assert!(
        (significand - 1.0).abs() < 1e-15,
        "Significand of 0.5 should be 1.0"
    );
    assert!(
        (exponent - (-1.0)).abs() < 1e-15,
        "Exponent of 0.5 should be -1.0"
    );
}

#[test]
fn test_fxtract_quarter() {
    // Test FXTRACT on 0.25 (2^-2)
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xD9, 0xF4, 0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00,
        0x00, 0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, 0xF4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 0.25);

    run_until_hlt(&mut vcpu).unwrap();

    let significand = read_f64(&mem, 0x3000);
    let exponent = read_f64(&mem, 0x3008);

    assert!(
        (significand - 1.0).abs() < 1e-15,
        "Significand of 0.25 should be 1.0"
    );
    assert!(
        (exponent - (-2.0)).abs() < 1e-15,
        "Exponent of 0.25 should be -2.0"
    );
}

#[test]
fn test_fxtract_small_power_of_two() {
    // Test FXTRACT on 0.0009765625 (2^-10)
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xD9, 0xF4, 0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00,
        0x00, 0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, 0xF4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 1.0 / 1024.0);

    run_until_hlt(&mut vcpu).unwrap();

    let significand = read_f64(&mem, 0x3000);
    let exponent = read_f64(&mem, 0x3008);

    assert!(
        (significand - 1.0).abs() < 1e-15,
        "Significand of 2^-10 should be 1.0"
    );
    assert!(
        (exponent - (-10.0)).abs() < 1e-15,
        "Exponent of 2^-10 should be -10.0"
    );
}

// ============================================================================
// FXTRACT Tests: Non-Power-of-Two Values
// ============================================================================

#[test]
fn test_fxtract_three() {
    // Test FXTRACT on 3.0
    // 3.0 = 1.5 * 2^1
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xD9, 0xF4, 0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00,
        0x00, 0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, 0xF4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 3.0);

    run_until_hlt(&mut vcpu).unwrap();

    let significand = read_f64(&mem, 0x3000);
    let exponent = read_f64(&mem, 0x3008);

    assert!(
        (significand - 1.5).abs() < 1e-15,
        "Significand of 3.0 should be 1.5"
    );
    assert!(
        (exponent - 1.0).abs() < 1e-15,
        "Exponent of 3.0 should be 1.0"
    );
}

#[test]
fn test_fxtract_five() {
    // Test FXTRACT on 5.0
    // 5.0 = 1.25 * 2^2
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xD9, 0xF4, 0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00,
        0x00, 0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, 0xF4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 5.0);

    run_until_hlt(&mut vcpu).unwrap();

    let significand = read_f64(&mem, 0x3000);
    let exponent = read_f64(&mem, 0x3008);

    assert!(
        (significand - 1.25).abs() < 1e-15,
        "Significand of 5.0 should be 1.25"
    );
    assert!(
        (exponent - 2.0).abs() < 1e-15,
        "Exponent of 5.0 should be 2.0"
    );
}

#[test]
fn test_fxtract_six() {
    // Test FXTRACT on 6.0
    // 6.0 = 1.5 * 2^2
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xD9, 0xF4, 0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00,
        0x00, 0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, 0xF4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 6.0);

    run_until_hlt(&mut vcpu).unwrap();

    let significand = read_f64(&mem, 0x3000);
    let exponent = read_f64(&mem, 0x3008);

    assert!(
        (significand - 1.5).abs() < 1e-15,
        "Significand of 6.0 should be 1.5"
    );
    assert!(
        (exponent - 2.0).abs() < 1e-15,
        "Exponent of 6.0 should be 2.0"
    );
}

#[test]
fn test_fxtract_ten() {
    // Test FXTRACT on 10.0
    // 10.0 = 1.25 * 2^3
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xD9, 0xF4, 0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00,
        0x00, 0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, 0xF4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 10.0);

    run_until_hlt(&mut vcpu).unwrap();

    let significand = read_f64(&mem, 0x3000);
    let exponent = read_f64(&mem, 0x3008);

    assert!(
        (significand - 1.25).abs() < 1e-15,
        "Significand of 10.0 should be 1.25"
    );
    assert!(
        (exponent - 3.0).abs() < 1e-15,
        "Exponent of 10.0 should be 3.0"
    );
}

#[test]
fn test_fxtract_pi() {
    // Test FXTRACT on π
    // π ≈ 3.14159... = 1.5708... * 2^1
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xD9, 0xF4, 0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00,
        0x00, 0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, 0xF4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, std::f64::consts::PI);

    run_until_hlt(&mut vcpu).unwrap();

    let significand = read_f64(&mem, 0x3000);
    let exponent = read_f64(&mem, 0x3008);

    // π / 2 ≈ 1.5707963...
    assert!(
        (significand - std::f64::consts::PI / 2.0).abs() < 1e-15,
        "Significand of π should be π/2"
    );
    assert!(
        (exponent - 1.0).abs() < 1e-15,
        "Exponent of π should be 1.0"
    );
}

// ============================================================================
// FXTRACT Tests: Negative Values
// ============================================================================

#[test]
fn test_fxtract_negative_one() {
    // Test FXTRACT on -1.0
    // Sign is preserved in significand
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xD9, 0xF4, 0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00,
        0x00, 0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, 0xF4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, -1.0);

    run_until_hlt(&mut vcpu).unwrap();

    let significand = read_f64(&mem, 0x3000);
    let exponent = read_f64(&mem, 0x3008);

    assert!(
        (significand - (-1.0)).abs() < 1e-15,
        "Significand of -1.0 should be -1.0"
    );
    assert!(
        (exponent - 0.0).abs() < 1e-15,
        "Exponent of -1.0 should be 0.0"
    );
}

#[test]
fn test_fxtract_negative_two() {
    // Test FXTRACT on -2.0
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xD9, 0xF4, 0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00,
        0x00, 0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, 0xF4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, -2.0);

    run_until_hlt(&mut vcpu).unwrap();

    let significand = read_f64(&mem, 0x3000);
    let exponent = read_f64(&mem, 0x3008);

    assert!(
        (significand - (-1.0)).abs() < 1e-15,
        "Significand of -2.0 should be -1.0"
    );
    assert!(
        (exponent - 1.0).abs() < 1e-15,
        "Exponent of -2.0 should be 1.0"
    );
}

#[test]
fn test_fxtract_negative_half() {
    // Test FXTRACT on -0.5
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xD9, 0xF4, 0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00,
        0x00, 0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, 0xF4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, -0.5);

    run_until_hlt(&mut vcpu).unwrap();

    let significand = read_f64(&mem, 0x3000);
    let exponent = read_f64(&mem, 0x3008);

    assert!(
        (significand - (-1.0)).abs() < 1e-15,
        "Significand of -0.5 should be -1.0"
    );
    assert!(
        (exponent - (-1.0)).abs() < 1e-15,
        "Exponent of -0.5 should be -1.0"
    );
}

#[test]
fn test_fxtract_negative_pi() {
    // Test FXTRACT on -π
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xD9, 0xF4, 0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00,
        0x00, 0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, 0xF4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, -std::f64::consts::PI);

    run_until_hlt(&mut vcpu).unwrap();

    let significand = read_f64(&mem, 0x3000);
    let exponent = read_f64(&mem, 0x3008);

    assert!(
        (significand - (-std::f64::consts::PI / 2.0)).abs() < 1e-15,
        "Significand of -π should be -π/2"
    );
    assert!(
        (exponent - 1.0).abs() < 1e-15,
        "Exponent of -π should be 1.0"
    );
}

// ============================================================================
// FXTRACT Tests: Special Cases
// ============================================================================

#[test]
fn test_fxtract_positive_zero() {
    // Test FXTRACT on +0.0
    // Should return exponent of -∞ and significand of +0
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xD9, 0xF4, 0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00,
        0x00, 0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, 0xF4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 0.0);

    run_until_hlt(&mut vcpu).unwrap();

    let significand = read_f64(&mem, 0x3000);
    let exponent = read_f64(&mem, 0x3008);

    assert!(
        significand == 0.0 && !significand.is_sign_negative(),
        "Significand of +0 should be +0"
    );
    assert!(
        exponent.is_infinite() && exponent.is_sign_negative(),
        "Exponent of +0 should be -∞"
    );
}

#[test]
fn test_fxtract_negative_zero() {
    // Test FXTRACT on -0.0
    // Should return exponent of -∞ and significand of -0
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xD9, 0xF4, 0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00,
        0x00, 0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, 0xF4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, -0.0);

    run_until_hlt(&mut vcpu).unwrap();

    let significand = read_f64(&mem, 0x3000);
    let exponent = read_f64(&mem, 0x3008);

    assert!(
        significand == 0.0 && significand.is_sign_negative(),
        "Significand of -0 should be -0"
    );
    assert!(
        exponent.is_infinite() && exponent.is_sign_negative(),
        "Exponent of -0 should be -∞"
    );
}

// ============================================================================
// FXTRACT Tests: Large Values
// ============================================================================

#[test]
fn test_fxtract_large_value() {
    // Test FXTRACT on 1.0e100
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xD9, 0xF4, 0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00,
        0x00, 0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, 0xF4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 1.0e100);

    run_until_hlt(&mut vcpu).unwrap();

    let significand = read_f64(&mem, 0x3000);
    let exponent = read_f64(&mem, 0x3008);

    // Verify significand is in range [1.0, 2.0)
    assert!(
        significand >= 1.0 && significand < 2.0,
        "Significand should be in [1.0, 2.0)"
    );
    // Verify reconstruction
    let reconstructed = significand * 2.0_f64.powf(exponent);
    assert!(
        (reconstructed - 1.0e100).abs() / 1.0e100 < 1e-15,
        "Reconstruction should match original value"
    );
}

#[test]
fn test_fxtract_small_value() {
    // Test FXTRACT on 1.0e-100
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xD9, 0xF4, 0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00,
        0x00, 0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, 0xF4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 1.0e-100);

    run_until_hlt(&mut vcpu).unwrap();

    let significand = read_f64(&mem, 0x3000);
    let exponent = read_f64(&mem, 0x3008);

    // Verify significand is in range [1.0, 2.0)
    assert!(
        significand >= 1.0 && significand < 2.0,
        "Significand should be in [1.0, 2.0)"
    );
    // Verify reconstruction
    let reconstructed = significand * 2.0_f64.powf(exponent);
    assert!(
        (reconstructed - 1.0e-100).abs() / 1.0e-100 < 1e-15,
        "Reconstruction should match original value"
    );
}

// ============================================================================
// FXTRACT Tests: Reconstruction (FXTRACT + FSCALE reversal)
// ============================================================================

#[test]
fn test_fxtract_reconstruction_with_fscale() {
    // Test that FXTRACT followed by FSCALE restores original value
    // This demonstrates the documented example:
    // FXTRACT; FSCALE; FSTP ST(1);
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
fn test_fxtract_reconstruction_manually() {
    // Test manual reconstruction: value = significand * 2^exponent
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xD9, 0xF4, 0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00,
        0x00, 0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, 0xF4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    let original = 7.5;
    write_f64(&mem, 0x2000, original);

    run_until_hlt(&mut vcpu).unwrap();

    let significand = read_f64(&mem, 0x3000);
    let exponent = read_f64(&mem, 0x3008);

    let reconstructed = significand * 2.0_f64.powf(exponent);
    assert!(
        (reconstructed - original).abs() < 1e-15,
        "Manual reconstruction should match original: {} * 2^{} = {}",
        significand,
        exponent,
        reconstructed
    );
}

// ============================================================================
// FXTRACT Tests: Various Values
// ============================================================================

#[test]
fn test_fxtract_hundred() {
    // Test FXTRACT on 100.0
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xD9, 0xF4, 0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00,
        0x00, 0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, 0xF4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 100.0);

    run_until_hlt(&mut vcpu).unwrap();

    let significand = read_f64(&mem, 0x3000);
    let exponent = read_f64(&mem, 0x3008);

    // 100 = 1.5625 * 2^6
    assert!((significand - 1.5625).abs() < 1e-15, "Significand of 100.0");
    assert!((exponent - 6.0).abs() < 1e-15, "Exponent of 100.0");
}

#[test]
fn test_fxtract_point_one() {
    // Test FXTRACT on 0.1
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xD9, 0xF4, 0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00,
        0x00, 0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, 0xF4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 0.1);

    run_until_hlt(&mut vcpu).unwrap();

    let significand = read_f64(&mem, 0x3000);
    let exponent = read_f64(&mem, 0x3008);

    // Verify reconstruction
    let reconstructed = significand * 2.0_f64.powf(exponent);
    assert!(
        (reconstructed - 0.1).abs() < 1e-16,
        "Reconstruction of 0.1 should match"
    );
}

#[test]
fn test_fxtract_e() {
    // Test FXTRACT on e (Euler's number)
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xD9, 0xF4, 0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00,
        0x00, 0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, 0xF4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, std::f64::consts::E);

    run_until_hlt(&mut vcpu).unwrap();

    let significand = read_f64(&mem, 0x3000);
    let exponent = read_f64(&mem, 0x3008);

    // e ≈ 2.718... = 1.359... * 2^1
    assert!((exponent - 1.0).abs() < 1e-15, "Exponent of e should be 1");
    let reconstructed = significand * 2.0_f64.powf(exponent);
    assert!(
        (reconstructed - std::f64::consts::E).abs() < 1e-15,
        "Reconstruction of e should match"
    );
}

#[test]
fn test_fxtract_sqrt_two() {
    // Test FXTRACT on √2
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xD9, 0xF4, 0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00,
        0x00, 0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, 0xF4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, std::f64::consts::SQRT_2);

    run_until_hlt(&mut vcpu).unwrap();

    let significand = read_f64(&mem, 0x3000);
    let exponent = read_f64(&mem, 0x3008);

    // √2 ≈ 1.414... = 1.414... * 2^0
    assert!((exponent - 0.0).abs() < 1e-15, "Exponent of √2 should be 0");
    assert!(
        (significand - std::f64::consts::SQRT_2).abs() < 1e-15,
        "Significand of √2 should be √2 itself"
    );
}

#[test]
fn test_fxtract_range_verification() {
    // Test that significand is always in [1.0, 2.0) for positive values
    let test_values = [1.5, 3.7, 9.9, 15.3, 27.8, 50.5, 99.9, 127.5];

    for &value in &test_values {
        let code = [
            0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xD9, 0xF4, 0xDD, 0x1C, 0x25, 0x00, 0x30,
            0x00, 0x00, 0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, 0xF4,
        ];

        let (mut vcpu, mem) = setup_vm(&code, None);
        write_f64(&mem, 0x2000, value);

        run_until_hlt(&mut vcpu).unwrap();

        let significand = read_f64(&mem, 0x3000);
        let exponent = read_f64(&mem, 0x3008);

        assert!(
            significand >= 1.0 && significand < 2.0,
            "Significand of {} should be in [1.0, 2.0), got {}",
            value,
            significand
        );

        let reconstructed = significand * 2.0_f64.powf(exponent);
        assert!(
            (reconstructed - value).abs() < 1e-14,
            "Reconstruction of {} failed",
            value
        );
    }
}
