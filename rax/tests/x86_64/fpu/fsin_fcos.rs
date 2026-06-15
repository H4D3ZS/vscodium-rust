//! Tests for the FSIN and FCOS instructions.
//!
//! FSIN - Sine
//! FCOS - Cosine
//!
//! FSIN computes an approximation of the sine of the source operand in register ST(0)
//! and stores the result in ST(0). The source operand must be given in radians and must
//! be within the range -2^63 to +2^63.
//!
//! FCOS computes the approximate cosine of the source operand in register ST(0) and
//! stores the result in ST(0). The source operand must be given in radians and must
//! be within the range -2^63 to +2^63.
//!
//! Opcodes:
//! - FSIN: D9 FE
//! - FCOS: D9 FF
//!
//! Flags affected:
//! - C1: Set to 0 if stack underflow occurred; Set if result was rounded up
//! - C2: Set to 1 if outside range (-2^63 < source < +2^63); otherwise 0
//! - C0, C3: Undefined
//!
//! Reference: /Users/int/dev/rax/docs/fsin.txt, /Users/int/dev/rax/docs/fcos.txt

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
// FSIN - Sine Tests: Special Angles
// ============================================================================

#[test]
fn test_fsin_zero() {
    // FLD qword [0x2000]  ; DD 04 25 00 20 00 00
    // FSIN                ; D9 FE
    // FSTP qword [0x3000] ; DD 1C 25 00 30 00 00
    // HLT                 ; F4
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xFE, // FSIN
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 0.0);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert!((result - 0.0).abs() < 1e-15, "sin(0) should be 0");
}

#[test]
fn test_fsin_pi_over_2() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xFE, // FSIN
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, std::f64::consts::FRAC_PI_2);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert!((result - 1.0).abs() < 1e-15, "sin(π/2) should be 1");
}

#[test]
fn test_fsin_pi() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xFE, // FSIN
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, std::f64::consts::PI);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert!(result.abs() < 1e-15, "sin(π) should be approximately 0");
}

#[test]
fn test_fsin_3pi_over_2() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xFE, // FSIN
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 3.0 * std::f64::consts::FRAC_PI_2);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert!((result + 1.0).abs() < 1e-15, "sin(3π/2) should be -1");
}

#[test]
fn test_fsin_2pi() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xFE, // FSIN
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 2.0 * std::f64::consts::PI);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert!(result.abs() < 1e-15, "sin(2π) should be approximately 0");
}

#[test]
fn test_fsin_pi_over_4() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xFE, // FSIN
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, std::f64::consts::FRAC_PI_4);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    let expected = (std::f64::consts::FRAC_PI_4).sin();
    assert!((result - expected).abs() < 1e-15, "sin(π/4) should be √2/2");
}

#[test]
fn test_fsin_pi_over_6() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xFE, // FSIN
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, std::f64::consts::FRAC_PI_6);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert!((result - 0.5).abs() < 1e-15, "sin(π/6) should be 0.5");
}

// ============================================================================
// FSIN - Negative Angles
// ============================================================================

#[test]
fn test_fsin_negative_zero() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xFE, // FSIN
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, -0.0);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert!((result - 0.0).abs() < 1e-15, "sin(-0) should be -0 or 0");
}

#[test]
fn test_fsin_negative_pi_over_2() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xFE, // FSIN
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, -std::f64::consts::FRAC_PI_2);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert!((result + 1.0).abs() < 1e-15, "sin(-π/2) should be -1");
}

#[test]
fn test_fsin_negative_pi() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xFE, // FSIN
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, -std::f64::consts::PI);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert!(result.abs() < 1e-15, "sin(-π) should be approximately 0");
}

// ============================================================================
// FCOS - Cosine Tests: Special Angles
// ============================================================================

#[test]
fn test_fcos_zero() {
    // FLD qword [0x2000]  ; DD 04 25 00 20 00 00
    // FCOS                ; D9 FF
    // FSTP qword [0x3000] ; DD 1C 25 00 30 00 00
    // HLT                 ; F4
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xFF, // FCOS
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 0.0);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert!((result - 1.0).abs() < 1e-15, "cos(0) should be 1");
}

#[test]
fn test_fcos_pi_over_2() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xFF, // FCOS
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, std::f64::consts::FRAC_PI_2);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert!(result.abs() < 1e-15, "cos(π/2) should be approximately 0");
}

#[test]
fn test_fcos_pi() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xFF, // FCOS
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, std::f64::consts::PI);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert!((result + 1.0).abs() < 1e-15, "cos(π) should be -1");
}

#[test]
fn test_fcos_3pi_over_2() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xFF, // FCOS
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 3.0 * std::f64::consts::FRAC_PI_2);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert!(result.abs() < 1e-15, "cos(3π/2) should be approximately 0");
}

#[test]
fn test_fcos_2pi() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xFF, // FCOS
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 2.0 * std::f64::consts::PI);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert!((result - 1.0).abs() < 1e-15, "cos(2π) should be 1");
}

#[test]
fn test_fcos_pi_over_4() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xFF, // FCOS
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, std::f64::consts::FRAC_PI_4);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    let expected = (std::f64::consts::FRAC_PI_4).cos();
    assert!((result - expected).abs() < 1e-15, "cos(π/4) should be √2/2");
}

#[test]
fn test_fcos_pi_over_6() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xFF, // FCOS
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, std::f64::consts::FRAC_PI_6);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    let expected = (std::f64::consts::FRAC_PI_6).cos();
    assert!((result - expected).abs() < 1e-15, "cos(π/6) should be √3/2");
}

// ============================================================================
// FCOS - Negative Angles
// ============================================================================

#[test]
fn test_fcos_negative_zero() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xFF, // FCOS
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, -0.0);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert!((result - 1.0).abs() < 1e-15, "cos(-0) should be 1");
}

#[test]
fn test_fcos_negative_pi_over_2() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xFF, // FCOS
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, -std::f64::consts::FRAC_PI_2);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert!(result.abs() < 1e-15, "cos(-π/2) should be approximately 0");
}

#[test]
fn test_fcos_negative_pi() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xFF, // FCOS
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, -std::f64::consts::PI);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert!((result + 1.0).abs() < 1e-15, "cos(-π) should be -1");
}

// ============================================================================
// FSIN/FCOS - Range Reduction
// ============================================================================

#[test]
fn test_fsin_large_positive() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xFE, // FSIN
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 100.0 * std::f64::consts::PI);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert!(result.abs() <= 1.0, "sin(large value) should be in [-1, 1]");
}

#[test]
fn test_fcos_large_positive() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xFF, // FCOS
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 100.0 * std::f64::consts::PI);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert!(result.abs() <= 1.0, "cos(large value) should be in [-1, 1]");
}

#[test]
fn test_fsin_multiple_periods() {
    // sin(x + 2πn) = sin(x)
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xFE, // FSIN
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    let angle = std::f64::consts::FRAC_PI_6 + 10.0 * 2.0 * std::f64::consts::PI;
    write_f64(&mem, 0x2000, angle);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    let expected = (std::f64::consts::FRAC_PI_6).sin();
    assert!((result - expected).abs() < 1e-10, "sin should be periodic");
}

#[test]
fn test_fcos_multiple_periods() {
    // cos(x + 2πn) = cos(x)
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xFF, // FCOS
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    let angle = std::f64::consts::FRAC_PI_6 + 10.0 * 2.0 * std::f64::consts::PI;
    write_f64(&mem, 0x2000, angle);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    let expected = (std::f64::consts::FRAC_PI_6).cos();
    assert!((result - expected).abs() < 1e-10, "cos should be periodic");
}

// ============================================================================
// FSIN/FCOS - Trigonometric Identities
// ============================================================================

#[test]
fn test_sin_cos_pythagorean_identity() {
    // sin²(x) + cos²(x) = 1
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xC0, // FLD ST(0) (duplicate)
        0xD9, 0xFE, // FSIN
        0xD8, 0xC8, // FMUL ST(0), ST(0) (sin²)
        0xD9, 0xC9, // FXCH ST(1)
        0xD9, 0xFF, // FCOS
        0xD8, 0xC8, // FMUL ST(0), ST(0) (cos²)
        0xDE, 0xC1, // FADDP (add sin² + cos²)
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
fn test_sin_odd_function() {
    // sin(-x) = -sin(x)
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xFE, // FSIN
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let angle = 0.5;

    // Compute sin(angle)
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, angle);
    run_until_hlt(&mut vcpu).unwrap();
    let sin_positive = read_f64(&mem, 0x3000);

    // Compute sin(-angle)
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, -angle);
    run_until_hlt(&mut vcpu).unwrap();
    let sin_negative = read_f64(&mem, 0x3000);

    assert!(
        (sin_positive + sin_negative).abs() < 1e-15,
        "sin(-x) should equal -sin(x)"
    );
}

#[test]
fn test_cos_even_function() {
    // cos(-x) = cos(x)
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xFF, // FCOS
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let angle = 0.5;

    // Compute cos(angle)
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, angle);
    run_until_hlt(&mut vcpu).unwrap();
    let cos_positive = read_f64(&mem, 0x3000);

    // Compute cos(-angle)
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, -angle);
    run_until_hlt(&mut vcpu).unwrap();
    let cos_negative = read_f64(&mem, 0x3000);

    assert!(
        (cos_positive - cos_negative).abs() < 1e-15,
        "cos(-x) should equal cos(x)"
    );
}

// ============================================================================
// FSIN/FCOS - Small Angles
// ============================================================================

#[test]
fn test_fsin_small_angle() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xFE, // FSIN
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    let small_angle = 0.001;
    write_f64(&mem, 0x2000, small_angle);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    // For small angles, sin(x) ≈ x
    assert!((result - small_angle).abs() < 1e-7, "sin(small x) ≈ x");
}

#[test]
fn test_fcos_small_angle() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xFF, // FCOS
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    let small_angle = 0.001;
    write_f64(&mem, 0x2000, small_angle);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    // For small angles, cos(x) ≈ 1
    assert!((result - 1.0).abs() < 1e-5, "cos(small x) ≈ 1");
}

// ============================================================================
// FSIN/FCOS - Various Angles
// ============================================================================

#[test]
fn test_fsin_various_angles() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xFE, // FSIN
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let test_angles = vec![0.1, 0.2, 0.5, 1.0, 1.5, 2.0, 2.5, 3.0];

    for angle in test_angles {
        let (mut vcpu, mem) = setup_vm(&code, None);
        write_f64(&mem, 0x2000, angle);

        run_until_hlt(&mut vcpu).unwrap();

        let result = read_f64(&mem, 0x3000);
        let expected = angle.sin();
        assert!(
            (result - expected).abs() < 1e-14,
            "sin({}) error too large",
            angle
        );
    }
}

#[test]
fn test_fcos_various_angles() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xFF, // FCOS
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let test_angles = vec![0.1, 0.2, 0.5, 1.0, 1.5, 2.0, 2.5, 3.0];

    for angle in test_angles {
        let (mut vcpu, mem) = setup_vm(&code, None);
        write_f64(&mem, 0x2000, angle);

        run_until_hlt(&mut vcpu).unwrap();

        let result = read_f64(&mem, 0x3000);
        let expected = angle.cos();
        assert!(
            (result - expected).abs() < 1e-14,
            "cos({}) error too large",
            angle
        );
    }
}

// ============================================================================
// FSIN/FCOS - Special Values
// ============================================================================

#[test]
fn test_fsin_infinity() {
    // FSIN of infinity should produce NaN (invalid operation) and set C2
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xFE, // FSIN
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, f64::INFINITY);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert!(result.is_nan(), "sin(infinity) should produce NaN");
}

#[test]
fn test_fcos_infinity() {
    // FCOS of infinity should produce NaN (invalid operation) and set C2
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xFF, // FCOS
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, f64::INFINITY);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert!(result.is_nan(), "cos(infinity) should produce NaN");
}

#[test]
fn test_fsin_nan() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xFE, // FSIN
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, f64::NAN);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert!(result.is_nan(), "sin(NaN) should be NaN");
}

#[test]
fn test_fcos_nan() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xFF, // FCOS
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, f64::NAN);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert!(result.is_nan(), "cos(NaN) should be NaN");
}

#[test]
fn test_fsin_pi_over_3() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xFE, // FSIN
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, std::f64::consts::PI / 3.0);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    let expected = (std::f64::consts::PI / 3.0).sin();
    assert!((result - expected).abs() < 1e-15, "sin(π/3) should be √3/2");
}

#[test]
fn test_fcos_pi_over_3() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xFF, // FCOS
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, std::f64::consts::PI / 3.0);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    let expected = (std::f64::consts::PI / 3.0).cos();
    assert!((result - expected).abs() < 1e-15, "cos(π/3) should be 0.5");
}

#[test]
fn test_fsin_negative_pi_over_4() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xFE, // FSIN
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, -std::f64::consts::FRAC_PI_4);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    let expected = (-std::f64::consts::FRAC_PI_4).sin();
    assert!(
        (result - expected).abs() < 1e-15,
        "sin(-π/4) should be -√2/2"
    );
}

#[test]
fn test_fcos_2pi_plus_pi_over_6() {
    // cos(2π + π/6) = cos(π/6)
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xFF, // FCOS
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    let angle = 2.0 * std::f64::consts::PI + std::f64::consts::FRAC_PI_6;
    write_f64(&mem, 0x2000, angle);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    let expected = (std::f64::consts::FRAC_PI_6).cos();
    assert!(
        (result - expected).abs() < 1e-14,
        "cos(2π + π/6) should equal cos(π/6)"
    );
}

#[test]
fn test_fsin_bounds_check() {
    // sin should always be in [-1, 1]
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xFE, // FSIN
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let test_angles = vec![0.1, 0.5, 1.0, 2.0, 3.0, 5.0, 10.0];

    for angle in test_angles {
        let (mut vcpu, mem) = setup_vm(&code, None);
        write_f64(&mem, 0x2000, angle);

        run_until_hlt(&mut vcpu).unwrap();

        let result = read_f64(&mem, 0x3000);
        assert!(
            result >= -1.0 && result <= 1.0,
            "sin({}) must be in [-1, 1], got {}",
            angle,
            result
        );
    }
}
