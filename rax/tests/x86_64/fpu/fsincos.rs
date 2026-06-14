//! Tests for the FSINCOS instruction.
//!
//! FSINCOS - Simultaneous Sine and Cosine
//!
//! Computes both the approximate sine and cosine of the source operand in register ST(0),
//! stores the sine in ST(0), and pushes the cosine onto the top of the FPU register stack.
//! This instruction is faster than executing FSIN and FCOS in succession.
//! The source operand must be given in radians and must be within the range -2^63 to +2^63.
//!
//! Opcode: D9 FB
//!
//! Flags affected:
//! - C1: Set to 0 if stack underflow; set to 1 if stack overflow; set if result rounded up
//! - C2: Set to 1 if outside range (-2^63 < source < +2^63); otherwise 0
//! - C0, C3: Undefined
//!
//! Reference: /Users/int/dev/rax/docs/fsincos.txt

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
// FSINCOS - Special Angles
// ============================================================================

#[test]
fn test_fsincos_zero() {
    // sin(0) = 0, cos(0) = 1
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xFB, // FSINCOS
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000] ; cosine
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, // FSTP qword [0x3008] ; sine
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 0.0);

    run_until_hlt(&mut vcpu).unwrap();

    let cosine = read_f64(&mem, 0x3000);
    let sine = read_f64(&mem, 0x3008);
    assert!((sine - 0.0).abs() < 1e-15, "sin(0) should be 0");
    assert!((cosine - 1.0).abs() < 1e-15, "cos(0) should be 1");
}

#[test]
fn test_fsincos_pi_over_2() {
    // sin(π/2) = 1, cos(π/2) = 0
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xFB, // FSINCOS
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000] ; cosine
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, // FSTP qword [0x3008] ; sine
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, std::f64::consts::FRAC_PI_2);

    run_until_hlt(&mut vcpu).unwrap();

    let cosine = read_f64(&mem, 0x3000);
    let sine = read_f64(&mem, 0x3008);
    assert!((sine - 1.0).abs() < 1e-15, "sin(π/2) should be 1");
    assert!(cosine.abs() < 1e-15, "cos(π/2) should be 0");
}

#[test]
fn test_fsincos_pi() {
    // sin(π) = 0, cos(π) = -1
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xFB, // FSINCOS
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000] ; cosine
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, // FSTP qword [0x3008] ; sine
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, std::f64::consts::PI);

    run_until_hlt(&mut vcpu).unwrap();

    let cosine = read_f64(&mem, 0x3000);
    let sine = read_f64(&mem, 0x3008);
    assert!(sine.abs() < 1e-15, "sin(π) should be 0");
    assert!((cosine + 1.0).abs() < 1e-15, "cos(π) should be -1");
}

#[test]
fn test_fsincos_3pi_over_2() {
    // sin(3π/2) = -1, cos(3π/2) = 0
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xFB, // FSINCOS
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000] ; cosine
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, // FSTP qword [0x3008] ; sine
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 3.0 * std::f64::consts::FRAC_PI_2);

    run_until_hlt(&mut vcpu).unwrap();

    let cosine = read_f64(&mem, 0x3000);
    let sine = read_f64(&mem, 0x3008);
    assert!((sine + 1.0).abs() < 1e-15, "sin(3π/2) should be -1");
    assert!(cosine.abs() < 1e-15, "cos(3π/2) should be 0");
}

#[test]
fn test_fsincos_2pi() {
    // sin(2π) = 0, cos(2π) = 1
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xFB, // FSINCOS
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000] ; cosine
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, // FSTP qword [0x3008] ; sine
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 2.0 * std::f64::consts::PI);

    run_until_hlt(&mut vcpu).unwrap();

    let cosine = read_f64(&mem, 0x3000);
    let sine = read_f64(&mem, 0x3008);
    assert!(sine.abs() < 1e-15, "sin(2π) should be 0");
    assert!((cosine - 1.0).abs() < 1e-15, "cos(2π) should be 1");
}

#[test]
fn test_fsincos_pi_over_4() {
    // sin(π/4) = cos(π/4) = √2/2
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xFB, // FSINCOS
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000] ; cosine
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, // FSTP qword [0x3008] ; sine
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, std::f64::consts::FRAC_PI_4);

    run_until_hlt(&mut vcpu).unwrap();

    let cosine = read_f64(&mem, 0x3000);
    let sine = read_f64(&mem, 0x3008);
    let expected = (std::f64::consts::FRAC_PI_4).sin();
    assert!((sine - expected).abs() < 1e-15, "sin(π/4) should be √2/2");
    assert!((cosine - expected).abs() < 1e-15, "cos(π/4) should be √2/2");
    assert!((sine - cosine).abs() < 1e-15, "sin(π/4) = cos(π/4)");
}

#[test]
fn test_fsincos_pi_over_6() {
    // sin(π/6) = 0.5, cos(π/6) = √3/2
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xFB, // FSINCOS
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000] ; cosine
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, // FSTP qword [0x3008] ; sine
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, std::f64::consts::FRAC_PI_6);

    run_until_hlt(&mut vcpu).unwrap();

    let cosine = read_f64(&mem, 0x3000);
    let sine = read_f64(&mem, 0x3008);
    assert!((sine - 0.5).abs() < 1e-15, "sin(π/6) should be 0.5");
    let expected_cos = (std::f64::consts::FRAC_PI_6).cos();
    assert!(
        (cosine - expected_cos).abs() < 1e-15,
        "cos(π/6) should be √3/2"
    );
}

#[test]
fn test_fsincos_pi_over_3() {
    // sin(π/3) = √3/2, cos(π/3) = 0.5
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xFB, // FSINCOS
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000] ; cosine
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, // FSTP qword [0x3008] ; sine
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, std::f64::consts::PI / 3.0);

    run_until_hlt(&mut vcpu).unwrap();

    let cosine = read_f64(&mem, 0x3000);
    let sine = read_f64(&mem, 0x3008);
    let expected_sin = (std::f64::consts::PI / 3.0).sin();
    assert!(
        (sine - expected_sin).abs() < 1e-15,
        "sin(π/3) should be √3/2"
    );
    assert!((cosine - 0.5).abs() < 1e-15, "cos(π/3) should be 0.5");
}

// ============================================================================
// FSINCOS - Negative Angles
// ============================================================================

#[test]
fn test_fsincos_negative_zero() {
    // sin(-0) = -0, cos(-0) = 1
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xFB, // FSINCOS
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000] ; cosine
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, // FSTP qword [0x3008] ; sine
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, -0.0);

    run_until_hlt(&mut vcpu).unwrap();

    let cosine = read_f64(&mem, 0x3000);
    let sine = read_f64(&mem, 0x3008);
    assert!(sine.abs() < 1e-15, "sin(-0) should be 0");
    assert!((cosine - 1.0).abs() < 1e-15, "cos(-0) should be 1");
}

#[test]
fn test_fsincos_negative_pi_over_2() {
    // sin(-π/2) = -1, cos(-π/2) = 0
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xFB, // FSINCOS
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000] ; cosine
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, // FSTP qword [0x3008] ; sine
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, -std::f64::consts::FRAC_PI_2);

    run_until_hlt(&mut vcpu).unwrap();

    let cosine = read_f64(&mem, 0x3000);
    let sine = read_f64(&mem, 0x3008);
    assert!((sine + 1.0).abs() < 1e-15, "sin(-π/2) should be -1");
    assert!(cosine.abs() < 1e-15, "cos(-π/2) should be 0");
}

#[test]
fn test_fsincos_negative_pi() {
    // sin(-π) = 0, cos(-π) = -1
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xFB, // FSINCOS
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000] ; cosine
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, // FSTP qword [0x3008] ; sine
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, -std::f64::consts::PI);

    run_until_hlt(&mut vcpu).unwrap();

    let cosine = read_f64(&mem, 0x3000);
    let sine = read_f64(&mem, 0x3008);
    assert!(sine.abs() < 1e-15, "sin(-π) should be 0");
    assert!((cosine + 1.0).abs() < 1e-15, "cos(-π) should be -1");
}

#[test]
fn test_fsincos_negative_pi_over_4() {
    // sin(-π/4) = -√2/2, cos(-π/4) = √2/2
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xFB, // FSINCOS
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000] ; cosine
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, // FSTP qword [0x3008] ; sine
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, -std::f64::consts::FRAC_PI_4);

    run_until_hlt(&mut vcpu).unwrap();

    let cosine = read_f64(&mem, 0x3000);
    let sine = read_f64(&mem, 0x3008);
    let expected = (-std::f64::consts::FRAC_PI_4).sin();
    assert!((sine - expected).abs() < 1e-15, "sin(-π/4) should be -√2/2");
    assert!(
        (cosine + sine).abs() < 1e-15,
        "cos(-π/4) should equal -sin(-π/4)"
    );
}

// ============================================================================
// FSINCOS - Trigonometric Identities
// ============================================================================

#[test]
fn test_fsincos_pythagorean_identity() {
    // sin²(x) + cos²(x) = 1
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xFB, // FSINCOS
        0xD9, 0xC0, // FLD ST(0) (duplicate cosine)
        0xD8, 0xC8, // FMUL ST(0), ST(0) (cos²)
        0xD9, 0xCA, // FXCH ST(2)
        0xD8, 0xC8, // FMUL ST(0), ST(0) (sin²)
        0xDE, 0xC2, // FADDP (add sin² + cos²)
        0xDD, 0xD8, // FSTP ST(0) (pop extra)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 0.7);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert!(
        (result - 1.0).abs() < 1e-14,
        "sin²(x) + cos²(x) should equal 1"
    );
}

#[test]
fn test_fsincos_odd_even_symmetry() {
    // sin(-x) = -sin(x), cos(-x) = cos(x)
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xFB, // FSINCOS
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000] ; cosine
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, // FSTP qword [0x3008] ; sine
        0xF4, // HLT
    ];

    let angle = 0.5;

    // Compute sin(angle), cos(angle)
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, angle);
    run_until_hlt(&mut vcpu).unwrap();
    let cos_positive = read_f64(&mem, 0x3000);
    let sin_positive = read_f64(&mem, 0x3008);

    // Compute sin(-angle), cos(-angle)
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, -angle);
    run_until_hlt(&mut vcpu).unwrap();
    let cos_negative = read_f64(&mem, 0x3000);
    let sin_negative = read_f64(&mem, 0x3008);

    assert!(
        (sin_positive + sin_negative).abs() < 1e-15,
        "sin(-x) should equal -sin(x)"
    );
    assert!(
        (cos_positive - cos_negative).abs() < 1e-15,
        "cos(-x) should equal cos(x)"
    );
}

#[test]
fn test_fsincos_periodicity() {
    // sin(x + 2π) = sin(x), cos(x + 2π) = cos(x)
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xFB, // FSINCOS
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000] ; cosine
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, // FSTP qword [0x3008] ; sine
        0xF4, // HLT
    ];

    let angle = std::f64::consts::FRAC_PI_6;

    // Compute sin(angle), cos(angle)
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, angle);
    run_until_hlt(&mut vcpu).unwrap();
    let cos1 = read_f64(&mem, 0x3000);
    let sin1 = read_f64(&mem, 0x3008);

    // Compute sin(angle + 2π), cos(angle + 2π)
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, angle + 2.0 * std::f64::consts::PI);
    run_until_hlt(&mut vcpu).unwrap();
    let cos2 = read_f64(&mem, 0x3000);
    let sin2 = read_f64(&mem, 0x3008);

    assert!(
        (sin1 - sin2).abs() < 1e-14,
        "sin(x + 2π) should equal sin(x)"
    );
    assert!(
        (cos1 - cos2).abs() < 1e-14,
        "cos(x + 2π) should equal cos(x)"
    );
}

// ============================================================================
// FSINCOS - Range Reduction
// ============================================================================

#[test]
fn test_fsincos_large_positive() {
    // Test with large positive angle
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xFB, // FSINCOS
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000] ; cosine
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, // FSTP qword [0x3008] ; sine
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 100.0 * std::f64::consts::PI);

    run_until_hlt(&mut vcpu).unwrap();

    let cosine = read_f64(&mem, 0x3000);
    let sine = read_f64(&mem, 0x3008);
    assert!(sine.abs() <= 1.0, "sin(large value) should be in [-1, 1]");
    assert!(cosine.abs() <= 1.0, "cos(large value) should be in [-1, 1]");
}

#[test]
fn test_fsincos_large_negative() {
    // Test with large negative angle
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xFB, // FSINCOS
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000] ; cosine
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, // FSTP qword [0x3008] ; sine
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, -100.0 * std::f64::consts::PI);

    run_until_hlt(&mut vcpu).unwrap();

    let cosine = read_f64(&mem, 0x3000);
    let sine = read_f64(&mem, 0x3008);
    assert!(
        sine.abs() <= 1.0,
        "sin(large negative) should be in [-1, 1]"
    );
    assert!(
        cosine.abs() <= 1.0,
        "cos(large negative) should be in [-1, 1]"
    );
}

// ============================================================================
// FSINCOS - Small Angles
// ============================================================================

#[test]
fn test_fsincos_small_angle() {
    // For small angles, sin(x) ≈ x and cos(x) ≈ 1
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xFB, // FSINCOS
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000] ; cosine
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, // FSTP qword [0x3008] ; sine
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    let small_angle = 0.001;
    write_f64(&mem, 0x2000, small_angle);

    run_until_hlt(&mut vcpu).unwrap();

    let cosine = read_f64(&mem, 0x3000);
    let sine = read_f64(&mem, 0x3008);
    assert!((sine - small_angle).abs() < 1e-7, "sin(small x) ≈ x");
    assert!((cosine - 1.0).abs() < 1e-5, "cos(small x) ≈ 1");
}

// ============================================================================
// FSINCOS - Various Angles
// ============================================================================

#[test]
fn test_fsincos_various_angles() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xFB, // FSINCOS
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000] ; cosine
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, // FSTP qword [0x3008] ; sine
        0xF4, // HLT
    ];

    let test_angles = vec![0.1, 0.2, 0.5, 1.0, 1.5, 2.0, 2.5, 3.0];

    for angle in test_angles {
        let (mut vcpu, mem) = setup_vm(&code, None);
        write_f64(&mem, 0x2000, angle);

        run_until_hlt(&mut vcpu).unwrap();

        let cosine = read_f64(&mem, 0x3000);
        let sine = read_f64(&mem, 0x3008);
        let expected_sin = angle.sin();
        let expected_cos = angle.cos();
        assert!(
            (sine - expected_sin).abs() < 1e-14,
            "sin({}) error too large",
            angle
        );
        assert!(
            (cosine - expected_cos).abs() < 1e-14,
            "cos({}) error too large",
            angle
        );
    }
}

// ============================================================================
// FSINCOS - Special Values
// ============================================================================

#[test]
fn test_fsincos_infinity() {
    // FSINCOS of infinity should produce NaN
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xFB, // FSINCOS
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000] ; cosine
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, // FSTP qword [0x3008] ; sine
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, f64::INFINITY);

    run_until_hlt(&mut vcpu).unwrap();

    let cosine = read_f64(&mem, 0x3000);
    let sine = read_f64(&mem, 0x3008);
    assert!(sine.is_nan(), "sin(infinity) should produce NaN");
    assert!(cosine.is_nan(), "cos(infinity) should produce NaN");
}

#[test]
fn test_fsincos_neg_infinity() {
    // FSINCOS of -infinity should produce NaN
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xFB, // FSINCOS
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000] ; cosine
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, // FSTP qword [0x3008] ; sine
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, f64::NEG_INFINITY);

    run_until_hlt(&mut vcpu).unwrap();

    let cosine = read_f64(&mem, 0x3000);
    let sine = read_f64(&mem, 0x3008);
    assert!(sine.is_nan(), "sin(-infinity) should produce NaN");
    assert!(cosine.is_nan(), "cos(-infinity) should produce NaN");
}

#[test]
fn test_fsincos_nan() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xFB, // FSINCOS
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000] ; cosine
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, // FSTP qword [0x3008] ; sine
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, f64::NAN);

    run_until_hlt(&mut vcpu).unwrap();

    let cosine = read_f64(&mem, 0x3000);
    let sine = read_f64(&mem, 0x3008);
    assert!(sine.is_nan(), "sin(NaN) should be NaN");
    assert!(cosine.is_nan(), "cos(NaN) should be NaN");
}

// ============================================================================
// FSINCOS - Performance Benefits
// ============================================================================

#[test]
fn test_fsincos_multiple_angles() {
    // Test computing sin and cos for multiple angles
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xFB, // FSINCOS
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000] ; cos1
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, // FSTP qword [0x3008] ; sin1
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008]
        0xD9, 0xFB, // FSINCOS
        0xDD, 0x1C, 0x25, 0x10, 0x30, 0x00, 0x00, // FSTP qword [0x3010] ; cos2
        0xDD, 0x1C, 0x25, 0x18, 0x30, 0x00, 0x00, // FSTP qword [0x3018] ; sin2
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, std::f64::consts::FRAC_PI_4);
    write_f64(&mem, 0x2008, std::f64::consts::FRAC_PI_3);

    run_until_hlt(&mut vcpu).unwrap();

    let cos1 = read_f64(&mem, 0x3000);
    let sin1 = read_f64(&mem, 0x3008);
    let cos2 = read_f64(&mem, 0x3010);
    let sin2 = read_f64(&mem, 0x3018);

    let expected_sin1 = (std::f64::consts::FRAC_PI_4).sin();
    let expected_cos1 = (std::f64::consts::FRAC_PI_4).cos();
    let expected_sin2 = (std::f64::consts::PI / 3.0).sin();
    let expected_cos2 = (std::f64::consts::PI / 3.0).cos();

    assert!((sin1 - expected_sin1).abs() < 1e-15, "sin(π/4)");
    assert!((cos1 - expected_cos1).abs() < 1e-15, "cos(π/4)");
    assert!((sin2 - expected_sin2).abs() < 1e-15, "sin(π/3)");
    assert!((cos2 - expected_cos2).abs() < 1e-15, "cos(π/3)");
}

// ============================================================================
// FSINCOS - All Quadrants
// ============================================================================

#[test]
fn test_fsincos_all_quadrants() {
    // Test an angle from each quadrant
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xFB, // FSINCOS
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000] ; cosine
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, // FSTP qword [0x3008] ; sine
        0xF4, // HLT
    ];

    // Quadrant I: 0 < θ < π/2
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, std::f64::consts::FRAC_PI_6);
    run_until_hlt(&mut vcpu).unwrap();
    let cos_q1 = read_f64(&mem, 0x3000);
    let sin_q1 = read_f64(&mem, 0x3008);
    assert!(
        sin_q1 > 0.0 && cos_q1 > 0.0,
        "Quadrant I: both should be positive"
    );

    // Quadrant II: π/2 < θ < π
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 2.0 * std::f64::consts::PI / 3.0);
    run_until_hlt(&mut vcpu).unwrap();
    let cos_q2 = read_f64(&mem, 0x3000);
    let sin_q2 = read_f64(&mem, 0x3008);
    assert!(sin_q2 > 0.0 && cos_q2 < 0.0, "Quadrant II: sin>0, cos<0");

    // Quadrant III: π < θ < 3π/2
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 4.0 * std::f64::consts::PI / 3.0);
    run_until_hlt(&mut vcpu).unwrap();
    let cos_q3 = read_f64(&mem, 0x3000);
    let sin_q3 = read_f64(&mem, 0x3008);
    assert!(
        sin_q3 < 0.0 && cos_q3 < 0.0,
        "Quadrant III: both should be negative"
    );

    // Quadrant IV: 3π/2 < θ < 2π
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 5.0 * std::f64::consts::PI / 3.0);
    run_until_hlt(&mut vcpu).unwrap();
    let cos_q4 = read_f64(&mem, 0x3000);
    let sin_q4 = read_f64(&mem, 0x3008);
    assert!(sin_q4 < 0.0 && cos_q4 > 0.0, "Quadrant IV: sin<0, cos>0");
}

#[test]
fn test_fsincos_bound_check() {
    // Both sin and cos should always be in [-1, 1]
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xFB, // FSINCOS
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000] ; cosine
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, // FSTP qword [0x3008] ; sine
        0xF4, // HLT
    ];

    let test_angles = vec![0.1, 0.5, 1.0, 2.0, 3.0, 5.0, 10.0, 50.0];

    for angle in test_angles {
        let (mut vcpu, mem) = setup_vm(&code, None);
        write_f64(&mem, 0x2000, angle);

        run_until_hlt(&mut vcpu).unwrap();

        let cosine = read_f64(&mem, 0x3000);
        let sine = read_f64(&mem, 0x3008);
        assert!(
            sine >= -1.0 && sine <= 1.0,
            "sin({}) must be in [-1, 1], got {}",
            angle,
            sine
        );
        assert!(
            cosine >= -1.0 && cosine <= 1.0,
            "cos({}) must be in [-1, 1], got {}",
            angle,
            cosine
        );
    }
}
