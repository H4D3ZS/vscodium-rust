//! Tests for the FYL2X instruction.
//!
//! FYL2X - Compute y * log2(x)
//!
//! Computes (ST(1) * log2(ST(0))), stores result in ST(1), and pops the register stack.
//! ST(0) must be a non-zero positive number.
//! Designed with built-in multiplication to optimize logarithm calculations.
//!
//! Opcode: D9 F1
//!
//! Flags affected:
//! - C1: Set to 0 if stack underflow; set if result rounded up, cleared otherwise
//! - C0, C2, C3: Undefined
//!
//! Reference: /Users/int/dev/rax/docs/fyl2x.txt

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
// FYL2X - Basic Logarithm Calculations
// ============================================================================

#[test]
fn test_fyl2x_log2_of_2() {
    // log2(2) = 1, so 1 * log2(2) = 1
    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008] ; y = 1.0
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000] ; x = 2.0
        0xD9, 0xF1, // FYL2X ; ST(1) * log2(ST(0))
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 2.0); // x
    write_f64(&mem, 0x2008, 1.0); // y

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert!((result - 1.0).abs() < 1e-10, "1 * log2(2) should be 1.0");
}

#[test]
fn test_fyl2x_log2_of_4() {
    // log2(4) = 2, so 1 * log2(4) = 2
    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008] ; y = 1.0
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000] ; x = 4.0
        0xD9, 0xF1, // FYL2X
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 4.0); // x
    write_f64(&mem, 0x2008, 1.0); // y

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert!((result - 2.0).abs() < 1e-10, "1 * log2(4) should be 2.0");
}

#[test]
fn test_fyl2x_log2_of_8() {
    // log2(8) = 3
    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008] ; y = 1.0
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000] ; x = 8.0
        0xD9, 0xF1, // FYL2X
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 8.0); // x
    write_f64(&mem, 0x2008, 1.0); // y

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert!((result - 3.0).abs() < 1e-10, "1 * log2(8) should be 3.0");
}

#[test]
fn test_fyl2x_log2_of_1() {
    // log2(1) = 0, so y * log2(1) = 0 for any y
    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008] ; y = 5.0
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000] ; x = 1.0
        0xD9, 0xF1, // FYL2X
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 1.0); // x
    write_f64(&mem, 0x2008, 5.0); // y

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert!((result - 0.0).abs() < 1e-10, "5 * log2(1) should be 0.0");
}

// ============================================================================
// FYL2X - With Multiplier
// ============================================================================

#[test]
fn test_fyl2x_with_multiplier() {
    // 3 * log2(8) = 3 * 3 = 9
    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008] ; y = 3.0
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000] ; x = 8.0
        0xD9, 0xF1, // FYL2X
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 8.0); // x
    write_f64(&mem, 0x2008, 3.0); // y

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert!((result - 9.0).abs() < 1e-10, "3 * log2(8) should be 9.0");
}

#[test]
fn test_fyl2x_negative_multiplier() {
    // -2 * log2(4) = -2 * 2 = -4
    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008] ; y = -2.0
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000] ; x = 4.0
        0xD9, 0xF1, // FYL2X
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 4.0); // x
    write_f64(&mem, 0x2008, -2.0); // y

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert!(
        (result - (-4.0)).abs() < 1e-10,
        "-2 * log2(4) should be -4.0"
    );
}

#[test]
fn test_fyl2x_fractional_multiplier() {
    // 0.5 * log2(16) = 0.5 * 4 = 2
    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008] ; y = 0.5
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000] ; x = 16.0
        0xD9, 0xF1, // FYL2X
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 16.0); // x
    write_f64(&mem, 0x2008, 0.5); // y

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert!((result - 2.0).abs() < 1e-10, "0.5 * log2(16) should be 2.0");
}

// ============================================================================
// FYL2X - Fractional Inputs
// ============================================================================

#[test]
fn test_fyl2x_log2_of_half() {
    // log2(0.5) = -1, so 1 * log2(0.5) = -1
    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008] ; y = 1.0
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000] ; x = 0.5
        0xD9, 0xF1, // FYL2X
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 0.5); // x
    write_f64(&mem, 0x2008, 1.0); // y

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert!(
        (result - (-1.0)).abs() < 1e-10,
        "1 * log2(0.5) should be -1.0"
    );
}

#[test]
fn test_fyl2x_log2_of_quarter() {
    // log2(0.25) = -2
    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008] ; y = 1.0
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000] ; x = 0.25
        0xD9, 0xF1, // FYL2X
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 0.25); // x
    write_f64(&mem, 0x2008, 1.0); // y

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert!(
        (result - (-2.0)).abs() < 1e-10,
        "1 * log2(0.25) should be -2.0"
    );
}

// ============================================================================
// FYL2X - Logarithm Base Conversion
// ============================================================================

#[test]
fn test_fyl2x_log10_conversion() {
    // To compute log10(x), use: log10(x) = log2(x) / log2(10)
    // So: (1/log2(10)) * log2(x)
    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008] ; y = 1/log2(10)
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000] ; x = 100.0
        0xD9, 0xF1, // FYL2X
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 100.0); // x
    let log2_10 = 10.0f64.log2();
    write_f64(&mem, 0x2008, 1.0 / log2_10); // y = 1/log2(10)

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    let expected = 100.0f64.log10();
    assert!(
        (result - expected).abs() < 1e-10,
        "log10(100) should be approximately 2.0"
    );
}

#[test]
fn test_fyl2x_natural_log_conversion() {
    // To compute ln(x), use: ln(x) = log2(x) / log2(e)
    // So: (1/log2(e)) * log2(x)
    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008] ; y = 1/log2(e)
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000] ; x
        0xD9, 0xF1, // FYL2X
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, std::f64::consts::E); // x = e
    let log2_e = std::f64::consts::E.log2();
    write_f64(&mem, 0x2008, 1.0 / log2_e); // y = 1/log2(e)

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert!((result - 1.0).abs() < 1e-10, "ln(e) should be 1.0");
}

// ============================================================================
// FYL2X - Large and Small Values
// ============================================================================

#[test]
fn test_fyl2x_large_power_of_2() {
    // log2(1024) = 10
    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008] ; y = 1.0
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000] ; x = 1024.0
        0xD9, 0xF1, // FYL2X
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 1024.0); // x
    write_f64(&mem, 0x2008, 1.0); // y

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert!(
        (result - 10.0).abs() < 1e-10,
        "1 * log2(1024) should be 10.0"
    );
}

#[test]
fn test_fyl2x_very_large_value() {
    // log2(2^20) = 20
    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008] ; y = 1.0
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000] ; x
        0xD9, 0xF1, // FYL2X
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    let x = 2.0f64.powi(20);
    write_f64(&mem, 0x2000, x); // x = 2^20
    write_f64(&mem, 0x2008, 1.0); // y

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert!((result - 20.0).abs() < 1e-10, "log2(2^20) should be 20.0");
}

#[test]
fn test_fyl2x_very_small_value() {
    // log2(2^-10) = -10
    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008] ; y = 1.0
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000] ; x
        0xD9, 0xF1, // FYL2X
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    let x = 2.0f64.powi(-10);
    write_f64(&mem, 0x2000, x); // x = 2^-10
    write_f64(&mem, 0x2008, 1.0); // y

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert!(
        (result - (-10.0)).abs() < 1e-10,
        "log2(2^-10) should be -10.0"
    );
}

// ============================================================================
// FYL2X - Non-Power-of-2 Values
// ============================================================================

#[test]
fn test_fyl2x_log2_of_3() {
    // log2(3) ≈ 1.585
    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008] ; y = 1.0
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000] ; x = 3.0
        0xD9, 0xF1, // FYL2X
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 3.0); // x
    write_f64(&mem, 0x2008, 1.0); // y

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    let expected = 3.0f64.log2();
    assert!((result - expected).abs() < 1e-10, "log2(3) should match");
}

#[test]
fn test_fyl2x_log2_of_10() {
    // log2(10) ≈ 3.322
    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008] ; y = 1.0
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000] ; x = 10.0
        0xD9, 0xF1, // FYL2X
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 10.0); // x
    write_f64(&mem, 0x2008, 1.0); // y

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    let expected = 10.0f64.log2();
    assert!((result - expected).abs() < 1e-10, "log2(10) should match");
}

// ============================================================================
// FYL2X - Special Values
// ============================================================================

#[test]
fn test_fyl2x_with_infinity_x() {
    // log2(+infinity) = +infinity, y * infinity = infinity (for positive y)
    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008] ; y = 1.0
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000] ; x = +infinity
        0xD9, 0xF1, // FYL2X
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, f64::INFINITY); // x
    write_f64(&mem, 0x2008, 1.0); // y

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert!(
        result.is_infinite() && !result.is_sign_negative(),
        "log2(+infinity) * 1 should be +infinity"
    );
}

#[test]
fn test_fyl2x_with_zero_y() {
    // 0 * log2(x) = 0 (for finite x > 0)
    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008] ; y = 0.0
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000] ; x = 2.0
        0xD9, 0xF1, // FYL2X
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 2.0); // x
    write_f64(&mem, 0x2008, 0.0); // y

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert_eq!(result, 0.0, "0 * log2(2) should be 0");
}

// ============================================================================
// FYL2X - Multiple Operations
// ============================================================================

#[test]
fn test_fyl2x_sequence() {
    // Test multiple FYL2X operations
    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008] ; y1
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000] ; x1
        0xD9, 0xF1, // FYL2X
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xDD, 0x04, 0x25, 0x18, 0x20, 0x00, 0x00, // FLD qword [0x2018] ; y2
        0xDD, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, // FLD qword [0x2010] ; x2
        0xD9, 0xF1, // FYL2X
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, // FSTP qword [0x3008]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 2.0); // x1
    write_f64(&mem, 0x2008, 1.0); // y1
    write_f64(&mem, 0x2010, 8.0); // x2
    write_f64(&mem, 0x2018, 2.0); // y2

    run_until_hlt(&mut vcpu).unwrap();

    let result1 = read_f64(&mem, 0x3000);
    let result2 = read_f64(&mem, 0x3008);
    assert!((result1 - 1.0).abs() < 1e-10, "1 * log2(2) = 1");
    assert!((result2 - 6.0).abs() < 1e-10, "2 * log2(8) = 6");
}

// ============================================================================
// FYL2X - Precision Tests
// ============================================================================

#[test]
fn test_fyl2x_precision() {
    // Test with value that requires precision
    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008] ; y
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000] ; x
        0xD9, 0xF1, // FYL2X
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, std::f64::consts::PI); // x = PI
    write_f64(&mem, 0x2008, 1.0); // y

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    let expected = std::f64::consts::PI.log2();
    assert!(
        (result - expected).abs() < 1e-10,
        "log2(PI) should be precise"
    );
}

#[test]
fn test_fyl2x_powers_of_two_exact() {
    // Powers of 2 should give exact integer results
    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008] ; y = 1.0
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000] ; x
        0xD9, 0xF1, // FYL2X
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let test_cases = vec![
        (2.0, 1.0),
        (4.0, 2.0),
        (16.0, 4.0),
        (32.0, 5.0),
        (64.0, 6.0),
        (128.0, 7.0),
        (256.0, 8.0),
        (512.0, 9.0),
    ];

    for (x, expected_log) in test_cases {
        let (mut vcpu, mem) = setup_vm(&code, None);
        write_f64(&mem, 0x2000, x);
        write_f64(&mem, 0x2008, 1.0);

        run_until_hlt(&mut vcpu).unwrap();

        let result = read_f64(&mem, 0x3000);
        assert!(
            (result - expected_log).abs() < 1e-15,
            "log2({}) should be exactly {}",
            x,
            expected_log
        );
    }
}

#[test]
fn test_fyl2x_sqrt_2() {
    // log2(sqrt(2)) = 0.5
    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008] ; y = 1.0
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000] ; x = sqrt(2)
        0xD9, 0xF1, // FYL2X
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 2.0f64.sqrt()); // x = sqrt(2)
    write_f64(&mem, 0x2008, 1.0); // y

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert!((result - 0.5).abs() < 1e-10, "log2(sqrt(2)) should be 0.5");
}

// ============================================================================
// Known-answer FYL2X tests (transcendental; small tolerance).
// Result = ST(1)=y * log2(ST(0)=x), then pop.
// ============================================================================

/// ST(1)=y (loaded first), ST(0)=x (loaded second), FYL2X, store.
fn kat_fyl2x(y: f64, x: f64) -> f64 {
    let code = [
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008] (y -> ST(1))
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000] (x -> ST(0))
        0xD9, 0xF1, // FYL2X
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, x);
    write_f64(&mem, 0x2008, y);
    run_until_hlt(&mut vcpu).unwrap();
    read_f64(&mem, 0x3000)
}

#[test]
fn test_fyl2x_known_values() {
    let tol = 1e-12;
    assert!((kat_fyl2x(1.0, 2.0) - 1.0).abs() < tol, "1*log2(2)=1");
    assert!((kat_fyl2x(1.0, 8.0) - 3.0).abs() < tol, "1*log2(8)=3");
    assert!((kat_fyl2x(3.0, 4.0) - 6.0).abs() < tol, "3*log2(4)=6");
    assert!((kat_fyl2x(1.0, 1.0)).abs() < tol, "log2(1)=0");
    assert!((kat_fyl2x(2.0, 0.5) + 2.0).abs() < tol, "2*log2(0.5)=-2");
}
