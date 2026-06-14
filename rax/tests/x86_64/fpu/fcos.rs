//! Tests for the FCOS instruction.
//!
//! FCOS - Cosine
//!
//! Computes the approximate cosine of ST(0) and stores the result in ST(0).
//! The source operand must be in radians and within the range -2^63 to +2^63.
//!
//! Opcode: D9 FF
//!
//! Flags affected:
//! - C1: Set to 0 if stack underflow; set if result rounded up
//! - C2: Set to 1 if source out of range; otherwise 0
//! - C0, C3: Undefined
//!
//! Reference: /Users/int/dev/rax/docs/fcos.txt

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
// FCOS - Basic Cosine Tests
// ============================================================================

#[test]
fn test_fcos_zero() {
    // cos(0) = 1
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
    assert!(
        (result - 1.0).abs() < 1e-10,
        "cos(0) should be 1.0, got {}",
        result
    );
}

#[test]
fn test_fcos_pi_over_2() {
    // cos(π/2) ≈ 0
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
    assert!(
        result.abs() < 1e-10,
        "cos(π/2) should be ~0, got {}",
        result
    );
}

#[test]
fn test_fcos_pi() {
    // cos(π) = -1
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
    assert!(
        (result + 1.0).abs() < 1e-10,
        "cos(π) should be -1.0, got {}",
        result
    );
}

#[test]
fn test_fcos_two_pi() {
    // cos(2π) = 1
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
    assert!(
        (result - 1.0).abs() < 1e-10,
        "cos(2π) should be 1.0, got {}",
        result
    );
}

#[test]
fn test_fcos_pi_over_4() {
    // cos(π/4) ≈ √2/2 ≈ 0.7071
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
    let expected = 1.0 / 2.0_f64.sqrt();
    assert!(
        (result - expected).abs() < 1e-10,
        "cos(π/4) should be √2/2, got {}",
        result
    );
}

#[test]
fn test_fcos_pi_over_3() {
    // cos(π/3) = 0.5
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xFF, // FCOS
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, std::f64::consts::FRAC_PI_3);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert!(
        (result - 0.5).abs() < 1e-10,
        "cos(π/3) should be 0.5, got {}",
        result
    );
}

#[test]
fn test_fcos_pi_over_6() {
    // cos(π/6) ≈ √3/2 ≈ 0.866
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
    let expected = 3.0_f64.sqrt() / 2.0;
    assert!(
        (result - expected).abs() < 1e-10,
        "cos(π/6) should be √3/2, got {}",
        result
    );
}

// ============================================================================
// FCOS - Negative Angles
// ============================================================================

#[test]
fn test_fcos_negative_pi_over_2() {
    // cos(-π/2) ≈ 0
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
    assert!(
        result.abs() < 1e-10,
        "cos(-π/2) should be ~0, got {}",
        result
    );
}

#[test]
fn test_fcos_negative_pi() {
    // cos(-π) = -1
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
    assert!(
        (result + 1.0).abs() < 1e-10,
        "cos(-π) should be -1.0, got {}",
        result
    );
}

#[test]
fn test_fcos_negative_pi_over_4() {
    // cos(-π/4) = cos(π/4) ≈ √2/2
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xFF, // FCOS
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, -std::f64::consts::FRAC_PI_4);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    let expected = 1.0 / 2.0_f64.sqrt();
    assert!(
        (result - expected).abs() < 1e-10,
        "cos(-π/4) should be √2/2, got {}",
        result
    );
}

// ============================================================================
// FCOS - Small Angles
// ============================================================================

#[test]
fn test_fcos_small_positive() {
    // cos(0.1) ≈ 0.995
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xFF, // FCOS
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 0.1);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    let expected = 0.1_f64.cos();
    assert!((result - expected).abs() < 1e-10, "cos(0.1) mismatch");
}

#[test]
fn test_fcos_small_negative() {
    // cos(-0.1) ≈ 0.995
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xFF, // FCOS
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, -0.1);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    let expected = (-0.1_f64).cos();
    assert!((result - expected).abs() < 1e-10, "cos(-0.1) mismatch");
}

#[test]
fn test_fcos_very_small() {
    // cos(0.001) ≈ 1.0
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xFF, // FCOS
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 0.001);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    let expected = 0.001_f64.cos();
    assert!((result - expected).abs() < 1e-10, "cos(0.001) mismatch");
}

// ============================================================================
// FCOS - Multiple Periods
// ============================================================================

#[test]
fn test_fcos_three_pi_over_2() {
    // cos(3π/2) ≈ 0
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
    assert!(
        result.abs() < 1e-10,
        "cos(3π/2) should be ~0, got {}",
        result
    );
}

#[test]
fn test_fcos_four_pi() {
    // cos(4π) = 1
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xFF, // FCOS
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 4.0 * std::f64::consts::PI);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert!(
        (result - 1.0).abs() < 1e-10,
        "cos(4π) should be 1.0, got {}",
        result
    );
}

#[test]
fn test_fcos_six_pi() {
    // cos(6π) = 1
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xFF, // FCOS
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 6.0 * std::f64::consts::PI);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert!(
        (result - 1.0).abs() < 1e-10,
        "cos(6π) should be 1.0, got {}",
        result
    );
}

// ============================================================================
// FCOS - Larger Angles
// ============================================================================

#[test]
fn test_fcos_ten() {
    // cos(10)
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xFF, // FCOS
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 10.0);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    let expected = 10.0_f64.cos();
    assert!((result - expected).abs() < 1e-10, "cos(10) mismatch");
}

#[test]
fn test_fcos_hundred() {
    // cos(100)
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xFF, // FCOS
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 100.0);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    let expected = 100.0_f64.cos();
    assert!((result - expected).abs() < 1e-9, "cos(100) mismatch");
}

#[test]
fn test_fcos_thousand() {
    // cos(1000)
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xFF, // FCOS
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 1000.0);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    let expected = 1000.0_f64.cos();
    assert!((result - expected).abs() < 1e-8, "cos(1000) mismatch");
}

// ============================================================================
// FCOS - Symmetry Tests (cosine is even function)
// ============================================================================

#[test]
fn test_fcos_symmetry_1() {
    // cos(x) = cos(-x)
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xFF, // FCOS
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let angle = 1.234;

    // Test positive angle
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, angle);
    run_until_hlt(&mut vcpu).unwrap();
    let result_pos = read_f64(&mem, 0x3000);

    // Test negative angle
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, -angle);
    run_until_hlt(&mut vcpu).unwrap();
    let result_neg = read_f64(&mem, 0x3000);

    assert!(
        (result_pos - result_neg).abs() < 1e-10,
        "cos(x) should equal cos(-x)"
    );
}

#[test]
fn test_fcos_symmetry_pi_3() {
    // cos(x) = cos(-x) for x = π/3
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xFF, // FCOS
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let angle = std::f64::consts::FRAC_PI_3;

    // Test positive angle
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, angle);
    run_until_hlt(&mut vcpu).unwrap();
    let result_pos = read_f64(&mem, 0x3000);

    // Test negative angle
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, -angle);
    run_until_hlt(&mut vcpu).unwrap();
    let result_neg = read_f64(&mem, 0x3000);

    assert!(
        (result_pos - result_neg).abs() < 1e-10,
        "cos(π/3) should equal cos(-π/3)"
    );
}

// ============================================================================
// FCOS - Range Tests
// ============================================================================

#[test]
fn test_fcos_result_in_range() {
    // Cosine results should always be in [-1, 1]
    let test_values = vec![0.0, 0.5, 1.0, 1.5, 2.0, 3.0, 5.0, 10.0, -0.5, -1.0, -2.0];

    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xFF, // FCOS
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    for val in test_values {
        let (mut vcpu, mem) = setup_vm(&code, None);
        write_f64(&mem, 0x2000, val);
        run_until_hlt(&mut vcpu).unwrap();
        let result = read_f64(&mem, 0x3000);

        assert!(
            result >= -1.0 && result <= 1.0,
            "cos({}) = {} should be in range [-1, 1]",
            val,
            result
        );
    }
}

// ============================================================================
// FCOS - Special Quadrant Tests
// ============================================================================

#[test]
fn test_fcos_quadrant_1() {
    // First quadrant: 0 to π/2, cosine decreases from 1 to 0
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xFF, // FCOS
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, std::f64::consts::FRAC_PI_8);
    run_until_hlt(&mut vcpu).unwrap();
    let result = read_f64(&mem, 0x3000);

    assert!(result > 0.0 && result < 1.0, "cos(π/8) should be in (0, 1)");
}

#[test]
fn test_fcos_quadrant_2() {
    // Second quadrant: π/2 to π, cosine decreases from 0 to -1
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xFF, // FCOS
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 3.0 * std::f64::consts::FRAC_PI_4);
    run_until_hlt(&mut vcpu).unwrap();
    let result = read_f64(&mem, 0x3000);

    assert!(
        result < 0.0 && result > -1.0,
        "cos(3π/4) should be in (-1, 0)"
    );
}

#[test]
fn test_fcos_quadrant_3() {
    // Third quadrant: π to 3π/2, cosine increases from -1 to 0
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xFF, // FCOS
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 5.0 * std::f64::consts::FRAC_PI_4);
    run_until_hlt(&mut vcpu).unwrap();
    let result = read_f64(&mem, 0x3000);

    assert!(
        result < 0.0 && result > -1.0,
        "cos(5π/4) should be in (-1, 0)"
    );
}

#[test]
fn test_fcos_quadrant_4() {
    // Fourth quadrant: 3π/2 to 2π, cosine increases from 0 to 1
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xFF, // FCOS
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 7.0 * std::f64::consts::FRAC_PI_4);
    run_until_hlt(&mut vcpu).unwrap();
    let result = read_f64(&mem, 0x3000);

    assert!(
        result > 0.0 && result < 1.0,
        "cos(7π/4) should be in (0, 1)"
    );
}

// ============================================================================
// FCOS - Sequence Tests
// ============================================================================

#[test]
fn test_fcos_sequence() {
    // Test multiple FCOS operations
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xFF, // FCOS
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008]
        0xD9, 0xFF, // FCOS
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, // FSTP qword [0x3008]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 0.0);
    write_f64(&mem, 0x2008, std::f64::consts::PI);

    run_until_hlt(&mut vcpu).unwrap();

    let result1 = read_f64(&mem, 0x3000);
    let result2 = read_f64(&mem, 0x3008);
    assert!((result1 - 1.0).abs() < 1e-10, "First FCOS result");
    assert!((result2 + 1.0).abs() < 1e-10, "Second FCOS result");
}

#[test]
fn test_fcos_twice() {
    // cos(cos(x))
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xFF, // FCOS
        0xD9, 0xFF, // FCOS (again)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 1.0);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    let expected = 1.0_f64.cos().cos();
    assert!((result - expected).abs() < 1e-10, "cos(cos(1.0)) mismatch");
}

// ============================================================================
// FCOS - Edge Cases
// ============================================================================

#[test]
fn test_fcos_positive_zero() {
    // cos(+0) = 1
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
    assert!(
        (result - 1.0).abs() < 1e-15,
        "cos(+0) should be exactly 1.0"
    );
}

#[test]
fn test_fcos_negative_zero() {
    // cos(-0) = 1
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
    assert!(
        (result - 1.0).abs() < 1e-15,
        "cos(-0) should be exactly 1.0"
    );
}

#[test]
fn test_fcos_various_angles() {
    // Test a variety of angle values
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xFF, // FCOS
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let test_angles = vec![0.25, 0.75, 1.25, 2.5, 3.5, 4.5, 5.5];

    for angle in test_angles {
        let (mut vcpu, mem) = setup_vm(&code, None);
        write_f64(&mem, 0x2000, angle);
        run_until_hlt(&mut vcpu).unwrap();
        let result = read_f64(&mem, 0x3000);
        let expected = angle.cos();
        assert!(
            (result - expected).abs() < 1e-10,
            "cos({}) mismatch: expected {}, got {}",
            angle,
            expected,
            result
        );
    }
}

#[test]
fn test_fcos_arbitrary_values() {
    // Test with arbitrary floating-point values
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xFF, // FCOS
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let test_values = vec![0.12345, 1.98765, 2.71828, 3.33333, 5.55555];

    for val in test_values {
        let (mut vcpu, mem) = setup_vm(&code, None);
        write_f64(&mem, 0x2000, val);
        run_until_hlt(&mut vcpu).unwrap();
        let result = read_f64(&mem, 0x3000);
        let expected = val.cos();
        assert!((result - expected).abs() < 1e-10, "cos({}) mismatch", val);
    }
}
