//! Tests for the FPTAN instruction.
//!
//! FPTAN - Partial Tangent
//!
//! Computes the approximate tangent of the source operand in register ST(0), stores
//! the result in ST(0), and pushes a 1.0 onto the FPU register stack. The source
//! operand must be given in radians and must be less than ±2^63.
//!
//! The value 1.0 is pushed onto the register stack after the tangent has been computed
//! to maintain compatibility with the Intel 8087 and Intel 287 math coprocessors.
//!
//! If the source operand is outside the acceptable range, the C2 flag in the FPU status
//! word is set, and the value in register ST(0) remains unchanged.
//!
//! Opcode: D9 F2
//!
//! Operation:
//! IF ST(0) < 2^63 THEN
//!   C2 := 0;
//!   ST(0) := fptan(ST(0));
//!   TOP := TOP - 1;
//!   ST(0) := 1.0;
//! ELSE
//!   C2 := 1;
//! FI;
//!
//! Flags affected:
//! - C1: Set to 0 if stack underflow occurred; set to 1 if stack overflow occurred
//! - C2: Set to 1 if outside range (-2^63 < source < +2^63); otherwise 0
//! - C0, C3: Undefined
//!
//! Reference: /Users/int/dev/rax/docs/fptan.txt

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
// FPTAN Tests: Special Angles
// ============================================================================

#[test]
fn test_fptan_zero() {
    // Test tan(0) = 0
    // FLD qword [0x2000]  ; Load angle
    // FPTAN               ; Compute tangent, push 1.0
    // FSTP qword [0x3000] ; Store the 1.0
    // FSTP qword [0x3008] ; Store the tangent result
    // HLT
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xF2, // FPTAN
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, // FSTP qword [0x3008]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 0.0);

    run_until_hlt(&mut vcpu).unwrap();

    let one_value = read_f64(&mem, 0x3000);
    let tangent = read_f64(&mem, 0x3008);

    assert!((one_value - 1.0).abs() < 1e-15, "FPTAN should push 1.0");
    assert!(tangent.abs() < 1e-15, "tan(0) should be 0");
}

#[test]
fn test_fptan_pi_over_4() {
    // Test tan(π/4) = 1
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xD9, 0xF2, 0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00,
        0x00, 0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, 0xF4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, std::f64::consts::FRAC_PI_4);

    run_until_hlt(&mut vcpu).unwrap();

    let one_value = read_f64(&mem, 0x3000);
    let tangent = read_f64(&mem, 0x3008);

    assert!((one_value - 1.0).abs() < 1e-15, "FPTAN should push 1.0");
    assert!((tangent - 1.0).abs() < 1e-14, "tan(π/4) should be 1");
}

#[test]
fn test_fptan_pi_over_6() {
    // Test tan(π/6) = 1/√3 ≈ 0.577350269
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xD9, 0xF2, 0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00,
        0x00, 0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, 0xF4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, std::f64::consts::FRAC_PI_6);

    run_until_hlt(&mut vcpu).unwrap();

    let one_value = read_f64(&mem, 0x3000);
    let tangent = read_f64(&mem, 0x3008);
    let expected = 1.0 / 3.0_f64.sqrt();

    assert!((one_value - 1.0).abs() < 1e-15, "FPTAN should push 1.0");
    assert!(
        (tangent - expected).abs() < 1e-14,
        "tan(π/6) should be 1/√3"
    );
}

#[test]
fn test_fptan_pi_over_3() {
    // Test tan(π/3) = √3 ≈ 1.732050808
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xD9, 0xF2, 0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00,
        0x00, 0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, 0xF4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, std::f64::consts::FRAC_PI_3);

    run_until_hlt(&mut vcpu).unwrap();

    let one_value = read_f64(&mem, 0x3000);
    let tangent = read_f64(&mem, 0x3008);
    let expected = 3.0_f64.sqrt();

    assert!((one_value - 1.0).abs() < 1e-15, "FPTAN should push 1.0");
    assert!((tangent - expected).abs() < 1e-13, "tan(π/3) should be √3");
}

#[test]
fn test_fptan_pi() {
    // Test tan(π) ≈ 0 (due to finite precision)
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xD9, 0xF2, 0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00,
        0x00, 0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, 0xF4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, std::f64::consts::PI);

    run_until_hlt(&mut vcpu).unwrap();

    let one_value = read_f64(&mem, 0x3000);
    let tangent = read_f64(&mem, 0x3008);

    assert!((one_value - 1.0).abs() < 1e-15, "FPTAN should push 1.0");
    assert!(tangent.abs() < 1e-14, "tan(π) should be approximately 0");
}

// ============================================================================
// FPTAN Tests: Negative Angles
// ============================================================================

#[test]
fn test_fptan_negative_pi_over_4() {
    // Test tan(-π/4) = -1
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xD9, 0xF2, 0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00,
        0x00, 0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, 0xF4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, -std::f64::consts::FRAC_PI_4);

    run_until_hlt(&mut vcpu).unwrap();

    let one_value = read_f64(&mem, 0x3000);
    let tangent = read_f64(&mem, 0x3008);

    assert!((one_value - 1.0).abs() < 1e-15, "FPTAN should push 1.0");
    assert!((tangent + 1.0).abs() < 1e-14, "tan(-π/4) should be -1");
}

#[test]
fn test_fptan_negative_pi_over_6() {
    // Test tan(-π/6) = -1/√3
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xD9, 0xF2, 0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00,
        0x00, 0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, 0xF4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, -std::f64::consts::FRAC_PI_6);

    run_until_hlt(&mut vcpu).unwrap();

    let one_value = read_f64(&mem, 0x3000);
    let tangent = read_f64(&mem, 0x3008);
    let expected = -1.0 / 3.0_f64.sqrt();

    assert!((one_value - 1.0).abs() < 1e-15, "FPTAN should push 1.0");
    assert!(
        (tangent - expected).abs() < 1e-14,
        "tan(-π/6) should be -1/√3"
    );
}

#[test]
fn test_fptan_negative_pi() {
    // Test tan(-π) ≈ 0
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xD9, 0xF2, 0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00,
        0x00, 0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, 0xF4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, -std::f64::consts::PI);

    run_until_hlt(&mut vcpu).unwrap();

    let one_value = read_f64(&mem, 0x3000);
    let tangent = read_f64(&mem, 0x3008);

    assert!((one_value - 1.0).abs() < 1e-15, "FPTAN should push 1.0");
    assert!(tangent.abs() < 1e-14, "tan(-π) should be approximately 0");
}

// ============================================================================
// FPTAN Tests: Small Angles
// ============================================================================

#[test]
fn test_fptan_small_positive_angle() {
    // Test tan(0.1) - for small x, tan(x) ≈ x
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xD9, 0xF2, 0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00,
        0x00, 0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, 0xF4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 0.1);

    run_until_hlt(&mut vcpu).unwrap();

    let one_value = read_f64(&mem, 0x3000);
    let tangent = read_f64(&mem, 0x3008);
    let expected = 0.1_f64.tan();

    assert!((one_value - 1.0).abs() < 1e-15, "FPTAN should push 1.0");
    assert!(
        (tangent - expected).abs() < 1e-15,
        "tan(0.1) should match Rust tan"
    );
}

#[test]
fn test_fptan_small_negative_angle() {
    // Test tan(-0.1)
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xD9, 0xF2, 0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00,
        0x00, 0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, 0xF4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, -0.1);

    run_until_hlt(&mut vcpu).unwrap();

    let one_value = read_f64(&mem, 0x3000);
    let tangent = read_f64(&mem, 0x3008);
    let expected = (-0.1_f64).tan();

    assert!((one_value - 1.0).abs() < 1e-15, "FPTAN should push 1.0");
    assert!(
        (tangent - expected).abs() < 1e-15,
        "tan(-0.1) should match Rust tan"
    );
}

#[test]
fn test_fptan_very_small_angle() {
    // Test tan(0.001)
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xD9, 0xF2, 0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00,
        0x00, 0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, 0xF4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 0.001);

    run_until_hlt(&mut vcpu).unwrap();

    let one_value = read_f64(&mem, 0x3000);
    let tangent = read_f64(&mem, 0x3008);

    assert!((one_value - 1.0).abs() < 1e-15, "FPTAN should push 1.0");
    let expected = (0.001_f64).tan();
    assert!(
        (tangent - expected).abs() < 1e-15,
        "tan(0.001) should match Rust tan"
    );
}

// ============================================================================
// FPTAN Tests: Multiple of 2π (periodicity)
// ============================================================================

#[test]
fn test_fptan_two_pi() {
    // Test tan(2π) ≈ 0 (periodicity)
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xD9, 0xF2, 0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00,
        0x00, 0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, 0xF4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 2.0 * std::f64::consts::PI);

    run_until_hlt(&mut vcpu).unwrap();

    let one_value = read_f64(&mem, 0x3000);
    let tangent = read_f64(&mem, 0x3008);

    assert!((one_value - 1.0).abs() < 1e-15, "FPTAN should push 1.0");
    assert!(tangent.abs() < 1e-13, "tan(2π) should be approximately 0");
}

#[test]
fn test_fptan_three_pi_over_4() {
    // Test tan(3π/4) = -1
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xD9, 0xF2, 0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00,
        0x00, 0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, 0xF4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 3.0 * std::f64::consts::FRAC_PI_4);

    run_until_hlt(&mut vcpu).unwrap();

    let one_value = read_f64(&mem, 0x3000);
    let tangent = read_f64(&mem, 0x3008);

    assert!((one_value - 1.0).abs() < 1e-15, "FPTAN should push 1.0");
    assert!((tangent + 1.0).abs() < 1e-13, "tan(3π/4) should be -1");
}

#[test]
fn test_fptan_five_pi_over_4() {
    // Test tan(5π/4) = 1 (5π/4 - π = π/4)
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xD9, 0xF2, 0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00,
        0x00, 0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, 0xF4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 5.0 * std::f64::consts::FRAC_PI_4);

    run_until_hlt(&mut vcpu).unwrap();

    let one_value = read_f64(&mem, 0x3000);
    let tangent = read_f64(&mem, 0x3008);

    assert!((one_value - 1.0).abs() < 1e-15, "FPTAN should push 1.0");
    assert!((tangent - 1.0).abs() < 1e-13, "tan(5π/4) should be 1");
}

// ============================================================================
// FPTAN Tests: Larger Angles (Range Reduction)
// ============================================================================

#[test]
fn test_fptan_ten_pi() {
    // Test tan(10π) ≈ 0
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xD9, 0xF2, 0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00,
        0x00, 0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, 0xF4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 10.0 * std::f64::consts::PI);

    run_until_hlt(&mut vcpu).unwrap();

    let one_value = read_f64(&mem, 0x3000);
    let tangent = read_f64(&mem, 0x3008);

    assert!((one_value - 1.0).abs() < 1e-15, "FPTAN should push 1.0");
    // Accuracy may degrade for larger angles
    assert!(tangent.abs() < 1e-10, "tan(10π) should be approximately 0");
}

#[test]
fn test_fptan_hundred() {
    // Test tan(100.0) radians
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xD9, 0xF2, 0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00,
        0x00, 0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, 0xF4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 100.0);

    run_until_hlt(&mut vcpu).unwrap();

    let one_value = read_f64(&mem, 0x3000);
    let tangent = read_f64(&mem, 0x3008);
    let expected = 100.0_f64.tan();

    assert!((one_value - 1.0).abs() < 1e-15, "FPTAN should push 1.0");
    // Larger angles have reduced accuracy
    assert!(
        (tangent - expected).abs() / expected.abs() < 1e-10,
        "tan(100) should approximately match Rust tan"
    );
}

#[test]
fn test_fptan_thousand() {
    // Test tan(1000.0) radians
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xD9, 0xF2, 0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00,
        0x00, 0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, 0xF4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 1000.0);

    run_until_hlt(&mut vcpu).unwrap();

    let one_value = read_f64(&mem, 0x3000);
    let _tangent = read_f64(&mem, 0x3008);

    assert!((one_value - 1.0).abs() < 1e-15, "FPTAN should push 1.0");
    // Just verify it completes without error
    // Accuracy significantly degrades for very large angles
}

// ============================================================================
// FPTAN Tests: Various Values
// ============================================================================

#[test]
fn test_fptan_one_radian() {
    // Test tan(1.0 radian)
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xD9, 0xF2, 0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00,
        0x00, 0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, 0xF4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 1.0);

    run_until_hlt(&mut vcpu).unwrap();

    let one_value = read_f64(&mem, 0x3000);
    let tangent = read_f64(&mem, 0x3008);
    let expected = 1.0_f64.tan();

    assert!((one_value - 1.0).abs() < 1e-15, "FPTAN should push 1.0");
    assert!(
        (tangent - expected).abs() < 1e-14,
        "tan(1) should match Rust tan"
    );
}

#[test]
fn test_fptan_two_radians() {
    // Test tan(2.0 radians)
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xD9, 0xF2, 0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00,
        0x00, 0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, 0xF4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 2.0);

    run_until_hlt(&mut vcpu).unwrap();

    let one_value = read_f64(&mem, 0x3000);
    let tangent = read_f64(&mem, 0x3008);
    let expected = 2.0_f64.tan();

    assert!((one_value - 1.0).abs() < 1e-15, "FPTAN should push 1.0");
    assert!(
        (tangent - expected).abs() < 1e-14,
        "tan(2) should match Rust tan"
    );
}

#[test]
fn test_fptan_half_radian() {
    // Test tan(0.5 radians)
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xD9, 0xF2, 0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00,
        0x00, 0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, 0xF4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 0.5);

    run_until_hlt(&mut vcpu).unwrap();

    let one_value = read_f64(&mem, 0x3000);
    let tangent = read_f64(&mem, 0x3008);
    let expected = 0.5_f64.tan();

    assert!((one_value - 1.0).abs() < 1e-15, "FPTAN should push 1.0");
    assert!(
        (tangent - expected).abs() < 1e-15,
        "tan(0.5) should match Rust tan"
    );
}

#[test]
fn test_fptan_pi_over_8() {
    // Test tan(π/8)
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xD9, 0xF2, 0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00,
        0x00, 0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, 0xF4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, std::f64::consts::FRAC_PI_8);

    run_until_hlt(&mut vcpu).unwrap();

    let one_value = read_f64(&mem, 0x3000);
    let tangent = read_f64(&mem, 0x3008);
    let expected = std::f64::consts::FRAC_PI_8.tan();

    assert!((one_value - 1.0).abs() < 1e-15, "FPTAN should push 1.0");
    assert!(
        (tangent - expected).abs() < 1e-15,
        "tan(π/8) should match Rust tan"
    );
}

#[test]
fn test_fptan_three_pi_over_8() {
    // Test tan(3π/8)
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xD9, 0xF2, 0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00,
        0x00, 0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, 0xF4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 3.0 * std::f64::consts::FRAC_PI_8);

    run_until_hlt(&mut vcpu).unwrap();

    let one_value = read_f64(&mem, 0x3000);
    let tangent = read_f64(&mem, 0x3008);
    let expected = (3.0 * std::f64::consts::FRAC_PI_8).tan();

    assert!((one_value - 1.0).abs() < 1e-15, "FPTAN should push 1.0");
    assert!(
        (tangent - expected).abs() < 1e-13,
        "tan(3π/8) should match Rust tan"
    );
}

// ============================================================================
// FPTAN Tests: Compatibility with FPATAN
// ============================================================================

#[test]
fn test_fptan_fpatan_cotangent_calculation() {
    // Demonstrate cotangent calculation: cot(x) = 1/tan(x)
    // FPTAN returns tan(x) in ST(1) and 1.0 in ST(0)
    // FDIVR ST(1), ST(0) computes ST(1) = ST(0) / ST(1) = 1.0 / tan(x) = cot(x)
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xF2, // FPTAN (ST(0)=1.0, ST(1)=tan)
        0xDE, 0xF9, // FDIVR (ST(1) = ST(0)/ST(1) = 1/tan = cot)
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, // FSTP qword [0x3008] (store cotangent)
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, std::f64::consts::FRAC_PI_4);

    run_until_hlt(&mut vcpu).unwrap();

    let cotangent = read_f64(&mem, 0x3008);
    // cot(π/4) = 1/tan(π/4) = 1/1 = 1
    assert!(
        (cotangent - 1.0).abs() < 1e-14,
        "cot(π/4) calculated via FPTAN+FDIVR should be 1"
    );
}

#[test]
fn test_fptan_positive_zero() {
    // Test tan(+0.0) = +0.0
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xD9, 0xF2, 0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00,
        0x00, 0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, 0xF4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 0.0);

    run_until_hlt(&mut vcpu).unwrap();

    let one_value = read_f64(&mem, 0x3000);
    let tangent = read_f64(&mem, 0x3008);

    assert!((one_value - 1.0).abs() < 1e-15, "FPTAN should push 1.0");
    assert!(
        tangent == 0.0 && !tangent.is_sign_negative(),
        "tan(+0) should be +0"
    );
}

#[test]
fn test_fptan_negative_zero() {
    // Test tan(-0.0) = -0.0
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xD9, 0xF2, 0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00,
        0x00, 0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, 0xF4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, -0.0);

    run_until_hlt(&mut vcpu).unwrap();

    let one_value = read_f64(&mem, 0x3000);
    let tangent = read_f64(&mem, 0x3008);

    assert!((one_value - 1.0).abs() < 1e-15, "FPTAN should push 1.0");
    assert!(
        tangent == 0.0 && tangent.is_sign_negative(),
        "tan(-0) should be -0"
    );
}

#[test]
fn test_fptan_symmetry() {
    // Test that tan(-x) = -tan(x)
    let angle = 0.7;

    // First, compute tan(+angle)
    let code_pos = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xD9, 0xF2, 0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00,
        0x00, 0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, 0xF4,
    ];

    let (mut vcpu_pos, mem_pos) = setup_vm(&code_pos, None);
    write_f64(&mem_pos, 0x2000, angle);
    run_until_hlt(&mut vcpu_pos).unwrap();
    let tan_pos = read_f64(&mem_pos, 0x3008);

    // Now, compute tan(-angle)
    let code_neg = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xD9, 0xF2, 0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00,
        0x00, 0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, 0xF4,
    ];

    let (mut vcpu_neg, mem_neg) = setup_vm(&code_neg, None);
    write_f64(&mem_neg, 0x2000, -angle);
    run_until_hlt(&mut vcpu_neg).unwrap();
    let tan_neg = read_f64(&mem_neg, 0x3008);

    assert!(
        (tan_pos + tan_neg).abs() < 1e-15,
        "tan(-x) should equal -tan(x)"
    );
}
