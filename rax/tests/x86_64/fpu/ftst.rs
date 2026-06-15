//! Tests for the FTST instruction.
//!
//! FTST - Test ST(0)
//!
//! Compares ST(0) with 0.0 and sets condition code flags C0, C2, C3 in FPU status word.
//! This is an unordered comparison that checks the class of numbers being compared.
//! The sign of zero is ignored (-0.0 == +0.0).
//!
//! Opcode: D9 E4
//!
//! Result flags (C3 C2 C0):
//! - ST(0) > 0.0:  0 0 0
//! - ST(0) < 0.0:  0 0 1
//! - ST(0) = 0.0:  1 0 0
//! - Unordered:    1 1 1
//!
//! Flags affected:
//! - C0, C2, C3: Set according to comparison result
//! - C1: Set to 0
//!
//! Reference: /Users/int/dev/rax/docs/ftst.txt

use crate::common::*;
use rax::cpu::Registers;
use vm_memory::{Bytes, GuestAddress};

// Helper function to write f64 to memory
fn write_f64(mem: &vm_memory::GuestMemoryMmap, addr: u64, val: f64) {
    mem.write_slice(&val.to_le_bytes(), GuestAddress(addr))
        .unwrap();
}

// Helper function to read u16 from memory
fn read_u16(mem: &vm_memory::GuestMemoryMmap, addr: u64) -> u16 {
    let mut buf = [0u8; 2];
    mem.read_slice(&mut buf, GuestAddress(addr)).unwrap();
    u16::from_le_bytes(buf)
}

// ============================================================================
// FTST - Positive Values (C3=0, C2=0, C0=0)
// ============================================================================

#[test]
fn test_ftst_positive_small() {
    // Test ST(0) > 0.0
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xE4, // FTST
        0x9B, 0xDF, 0xE0, // FSTSW AX
        0x66, 0x67, 0xA3, 0x00, 0x30, 0x00, 0x00, // MOV [0x3000], AX
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 1.5);

    run_until_hlt(&mut vcpu).unwrap();

    let status = read_u16(&mem, 0x3000);
    let c3 = (status >> 14) & 1;
    let c2 = (status >> 10) & 1;
    let c1 = (status >> 9) & 1;
    let c0 = (status >> 8) & 1;

    assert_eq!(c3, 0, "C3 should be 0 for ST(0) > 0.0");
    assert_eq!(c2, 0, "C2 should be 0 for ST(0) > 0.0");
    assert_eq!(c0, 0, "C0 should be 0 for ST(0) > 0.0");
    assert_eq!(c1, 0, "C1 should be 0");
}

#[test]
fn test_ftst_positive_one() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xE4, // FTST
        0x9B, 0xDF, 0xE0, // FSTSW AX
        0x66, 0x67, 0xA3, 0x00, 0x30, 0x00, 0x00, // MOV [0x3000], AX
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 1.0);

    run_until_hlt(&mut vcpu).unwrap();

    let status = read_u16(&mem, 0x3000);
    let c3 = (status >> 14) & 1;
    let c2 = (status >> 10) & 1;
    let c0 = (status >> 8) & 1;

    assert_eq!(c3, 0, "C3 should be 0");
    assert_eq!(c2, 0, "C2 should be 0");
    assert_eq!(c0, 0, "C0 should be 0");
}

#[test]
fn test_ftst_positive_large() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xE4, // FTST
        0x9B, 0xDF, 0xE0, // FSTSW AX
        0x66, 0x67, 0xA3, 0x00, 0x30, 0x00, 0x00, // MOV [0x3000], AX
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 123456789.0);

    run_until_hlt(&mut vcpu).unwrap();

    let status = read_u16(&mem, 0x3000);
    let c3 = (status >> 14) & 1;
    let c2 = (status >> 10) & 1;
    let c0 = (status >> 8) & 1;

    assert_eq!(c3, 0, "C3 should be 0 for large positive");
    assert_eq!(c2, 0, "C2 should be 0 for large positive");
    assert_eq!(c0, 0, "C0 should be 0 for large positive");
}

#[test]
fn test_ftst_positive_very_small() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xE4, // FTST
        0x9B, 0xDF, 0xE0, // FSTSW AX
        0x66, 0x67, 0xA3, 0x00, 0x30, 0x00, 0x00, // MOV [0x3000], AX
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 1e-100);

    run_until_hlt(&mut vcpu).unwrap();

    let status = read_u16(&mem, 0x3000);
    let c3 = (status >> 14) & 1;
    let c2 = (status >> 10) & 1;
    let c0 = (status >> 8) & 1;

    assert_eq!(c3, 0, "C3 should be 0 for very small positive");
    assert_eq!(c2, 0, "C2 should be 0 for very small positive");
    assert_eq!(c0, 0, "C0 should be 0 for very small positive");
}

#[test]
fn test_ftst_positive_infinity() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xE4, // FTST
        0x9B, 0xDF, 0xE0, // FSTSW AX
        0x66, 0x67, 0xA3, 0x00, 0x30, 0x00, 0x00, // MOV [0x3000], AX
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, f64::INFINITY);

    run_until_hlt(&mut vcpu).unwrap();

    let status = read_u16(&mem, 0x3000);
    let c3 = (status >> 14) & 1;
    let c2 = (status >> 10) & 1;
    let c0 = (status >> 8) & 1;

    assert_eq!(c3, 0, "C3 should be 0 for positive infinity");
    assert_eq!(c2, 0, "C2 should be 0 for positive infinity");
    assert_eq!(c0, 0, "C0 should be 0 for positive infinity");
}

// ============================================================================
// FTST - Negative Values (C3=0, C2=0, C0=1)
// ============================================================================

#[test]
fn test_ftst_negative_small() {
    // Test ST(0) < 0.0
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xE4, // FTST
        0x9B, 0xDF, 0xE0, // FSTSW AX
        0x66, 0x67, 0xA3, 0x00, 0x30, 0x00, 0x00, // MOV [0x3000], AX
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, -1.5);

    run_until_hlt(&mut vcpu).unwrap();

    let status = read_u16(&mem, 0x3000);
    let c3 = (status >> 14) & 1;
    let c2 = (status >> 10) & 1;
    let c1 = (status >> 9) & 1;
    let c0 = (status >> 8) & 1;

    assert_eq!(c3, 0, "C3 should be 0 for ST(0) < 0.0");
    assert_eq!(c2, 0, "C2 should be 0 for ST(0) < 0.0");
    assert_eq!(c0, 1, "C0 should be 1 for ST(0) < 0.0");
    assert_eq!(c1, 0, "C1 should be 0");
}

#[test]
fn test_ftst_negative_one() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xE4, // FTST
        0x9B, 0xDF, 0xE0, // FSTSW AX
        0x66, 0x67, 0xA3, 0x00, 0x30, 0x00, 0x00, // MOV [0x3000], AX
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, -1.0);

    run_until_hlt(&mut vcpu).unwrap();

    let status = read_u16(&mem, 0x3000);
    let c3 = (status >> 14) & 1;
    let c2 = (status >> 10) & 1;
    let c0 = (status >> 8) & 1;

    assert_eq!(c3, 0, "C3 should be 0");
    assert_eq!(c2, 0, "C2 should be 0");
    assert_eq!(c0, 1, "C0 should be 1");
}

#[test]
fn test_ftst_negative_large() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xE4, // FTST
        0x9B, 0xDF, 0xE0, // FSTSW AX
        0x66, 0x67, 0xA3, 0x00, 0x30, 0x00, 0x00, // MOV [0x3000], AX
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, -987654321.0);

    run_until_hlt(&mut vcpu).unwrap();

    let status = read_u16(&mem, 0x3000);
    let c3 = (status >> 14) & 1;
    let c2 = (status >> 10) & 1;
    let c0 = (status >> 8) & 1;

    assert_eq!(c3, 0, "C3 should be 0 for large negative");
    assert_eq!(c2, 0, "C2 should be 0 for large negative");
    assert_eq!(c0, 1, "C0 should be 1 for large negative");
}

#[test]
fn test_ftst_negative_very_small() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xE4, // FTST
        0x9B, 0xDF, 0xE0, // FSTSW AX
        0x66, 0x67, 0xA3, 0x00, 0x30, 0x00, 0x00, // MOV [0x3000], AX
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, -1e-100);

    run_until_hlt(&mut vcpu).unwrap();

    let status = read_u16(&mem, 0x3000);
    let c3 = (status >> 14) & 1;
    let c2 = (status >> 10) & 1;
    let c0 = (status >> 8) & 1;

    assert_eq!(c3, 0, "C3 should be 0 for very small negative");
    assert_eq!(c2, 0, "C2 should be 0 for very small negative");
    assert_eq!(c0, 1, "C0 should be 1 for very small negative");
}

#[test]
fn test_ftst_negative_infinity() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xE4, // FTST
        0x9B, 0xDF, 0xE0, // FSTSW AX
        0x66, 0x67, 0xA3, 0x00, 0x30, 0x00, 0x00, // MOV [0x3000], AX
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, f64::NEG_INFINITY);

    run_until_hlt(&mut vcpu).unwrap();

    let status = read_u16(&mem, 0x3000);
    let c3 = (status >> 14) & 1;
    let c2 = (status >> 10) & 1;
    let c0 = (status >> 8) & 1;

    assert_eq!(c3, 0, "C3 should be 0 for negative infinity");
    assert_eq!(c2, 0, "C2 should be 0 for negative infinity");
    assert_eq!(c0, 1, "C0 should be 1 for negative infinity");
}

// ============================================================================
// FTST - Zero Values (C3=1, C2=0, C0=0)
// ============================================================================

#[test]
fn test_ftst_positive_zero() {
    // Test ST(0) = 0.0
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xE4, // FTST
        0x9B, 0xDF, 0xE0, // FSTSW AX
        0x66, 0x67, 0xA3, 0x00, 0x30, 0x00, 0x00, // MOV [0x3000], AX
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 0.0);

    run_until_hlt(&mut vcpu).unwrap();

    let status = read_u16(&mem, 0x3000);
    let c3 = (status >> 14) & 1;
    let c2 = (status >> 10) & 1;
    let c1 = (status >> 9) & 1;
    let c0 = (status >> 8) & 1;

    assert_eq!(c3, 1, "C3 should be 1 for ST(0) = 0.0");
    assert_eq!(c2, 0, "C2 should be 0 for ST(0) = 0.0");
    assert_eq!(c0, 0, "C0 should be 0 for ST(0) = 0.0");
    assert_eq!(c1, 0, "C1 should be 0");
}

#[test]
fn test_ftst_negative_zero() {
    // -0.0 should be equal to +0.0
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xE4, // FTST
        0x9B, 0xDF, 0xE0, // FSTSW AX
        0x66, 0x67, 0xA3, 0x00, 0x30, 0x00, 0x00, // MOV [0x3000], AX
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, -0.0);

    run_until_hlt(&mut vcpu).unwrap();

    let status = read_u16(&mem, 0x3000);
    let c3 = (status >> 14) & 1;
    let c2 = (status >> 10) & 1;
    let c0 = (status >> 8) & 1;

    assert_eq!(c3, 1, "C3 should be 1 for ST(0) = -0.0");
    assert_eq!(c2, 0, "C2 should be 0 for ST(0) = -0.0");
    assert_eq!(c0, 0, "C0 should be 0 for ST(0) = -0.0 (sign ignored)");
}

// ============================================================================
// FTST - NaN (Unordered: C3=1, C2=1, C0=1)
// ============================================================================

#[test]
fn test_ftst_nan() {
    // NaN comparison is unordered
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xE4, // FTST
        0x9B, 0xDF, 0xE0, // FSTSW AX
        0x66, 0x67, 0xA3, 0x00, 0x30, 0x00, 0x00, // MOV [0x3000], AX
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, f64::NAN);

    run_until_hlt(&mut vcpu).unwrap();

    let status = read_u16(&mem, 0x3000);
    let c3 = (status >> 14) & 1;
    let c2 = (status >> 10) & 1;
    let c0 = (status >> 8) & 1;

    assert_eq!(c3, 1, "C3 should be 1 for unordered (NaN)");
    assert_eq!(c2, 1, "C2 should be 1 for unordered (NaN)");
    assert_eq!(c0, 1, "C0 should be 1 for unordered (NaN)");
}

#[test]
fn test_ftst_negative_nan() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xE4, // FTST
        0x9B, 0xDF, 0xE0, // FSTSW AX
        0x66, 0x67, 0xA3, 0x00, 0x30, 0x00, 0x00, // MOV [0x3000], AX
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, -f64::NAN);

    run_until_hlt(&mut vcpu).unwrap();

    let status = read_u16(&mem, 0x3000);
    let c3 = (status >> 14) & 1;
    let c2 = (status >> 10) & 1;
    let c0 = (status >> 8) & 1;

    assert_eq!(c3, 1, "C3 should be 1 for unordered (NaN)");
    assert_eq!(c2, 1, "C2 should be 1 for unordered (NaN)");
    assert_eq!(c0, 1, "C0 should be 1 for unordered (NaN)");
}

// ============================================================================
// FTST - Mathematical Constants
// ============================================================================

#[test]
fn test_ftst_pi() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xE4, // FTST
        0x9B, 0xDF, 0xE0, // FSTSW AX
        0x66, 0x67, 0xA3, 0x00, 0x30, 0x00, 0x00, // MOV [0x3000], AX
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, std::f64::consts::PI);

    run_until_hlt(&mut vcpu).unwrap();

    let status = read_u16(&mem, 0x3000);
    let c3 = (status >> 14) & 1;
    let c2 = (status >> 10) & 1;
    let c0 = (status >> 8) & 1;

    assert_eq!(c3, 0, "PI > 0");
    assert_eq!(c2, 0, "PI > 0");
    assert_eq!(c0, 0, "PI > 0");
}

#[test]
fn test_ftst_negative_e() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xE4, // FTST
        0x9B, 0xDF, 0xE0, // FSTSW AX
        0x66, 0x67, 0xA3, 0x00, 0x30, 0x00, 0x00, // MOV [0x3000], AX
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, -std::f64::consts::E);

    run_until_hlt(&mut vcpu).unwrap();

    let status = read_u16(&mem, 0x3000);
    let c3 = (status >> 14) & 1;
    let c2 = (status >> 10) & 1;
    let c0 = (status >> 8) & 1;

    assert_eq!(c3, 0, "-E < 0");
    assert_eq!(c2, 0, "-E < 0");
    assert_eq!(c0, 1, "-E < 0");
}

// ============================================================================
// FTST - Multiple Tests
// ============================================================================

#[test]
fn test_ftst_sequence() {
    // Test FTST on multiple values in sequence
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xE4, // FTST
        0x9B, 0xDF, 0xE0, // FSTSW AX
        0x66, 0x67, 0xA3, 0x00, 0x30, 0x00, 0x00, // MOV [0x3000], AX
        0xDD, 0xD8, // FSTP ST(0)
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008]
        0xD9, 0xE4, // FTST
        0x9B, 0xDF, 0xE0, // FSTSW AX
        0x66, 0x67, 0xA3, 0x08, 0x30, 0x00, 0x00, // MOV [0x3008], AX
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 5.0);
    write_f64(&mem, 0x2008, -3.0);

    run_until_hlt(&mut vcpu).unwrap();

    let status1 = read_u16(&mem, 0x3000);
    let status2 = read_u16(&mem, 0x3008);

    // First: positive
    let c0_1 = (status1 >> 8) & 1;
    assert_eq!(c0_1, 0, "5.0 > 0");

    // Second: negative
    let c0_2 = (status2 >> 8) & 1;
    assert_eq!(c0_2, 1, "-3.0 < 0");
}

// ============================================================================
// FTST - Denormal Values
// ============================================================================

#[test]
fn test_ftst_positive_denormal() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xE4, // FTST
        0x9B, 0xDF, 0xE0, // FSTSW AX
        0x66, 0x67, 0xA3, 0x00, 0x30, 0x00, 0x00, // MOV [0x3000], AX
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    let denormal = f64::MIN_POSITIVE / 2.0;
    write_f64(&mem, 0x2000, denormal);

    run_until_hlt(&mut vcpu).unwrap();

    let status = read_u16(&mem, 0x3000);
    let c3 = (status >> 14) & 1;
    let c2 = (status >> 10) & 1;
    let c0 = (status >> 8) & 1;

    assert_eq!(c3, 0, "Positive denormal > 0");
    assert_eq!(c2, 0, "Positive denormal > 0");
    assert_eq!(c0, 0, "Positive denormal > 0");
}

#[test]
fn test_ftst_negative_denormal() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xE4, // FTST
        0x9B, 0xDF, 0xE0, // FSTSW AX
        0x66, 0x67, 0xA3, 0x00, 0x30, 0x00, 0x00, // MOV [0x3000], AX
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    let denormal = -f64::MIN_POSITIVE / 2.0;
    write_f64(&mem, 0x2000, denormal);

    run_until_hlt(&mut vcpu).unwrap();

    let status = read_u16(&mem, 0x3000);
    let c3 = (status >> 14) & 1;
    let c2 = (status >> 10) & 1;
    let c0 = (status >> 8) & 1;

    assert_eq!(c3, 0, "Negative denormal < 0");
    assert_eq!(c2, 0, "Negative denormal < 0");
    assert_eq!(c0, 1, "Negative denormal < 0");
}

// ============================================================================
// FTST - Edge Cases
// ============================================================================

#[test]
fn test_ftst_max_value() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xE4, // FTST
        0x9B, 0xDF, 0xE0, // FSTSW AX
        0x66, 0x67, 0xA3, 0x00, 0x30, 0x00, 0x00, // MOV [0x3000], AX
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, f64::MAX);

    run_until_hlt(&mut vcpu).unwrap();

    let status = read_u16(&mem, 0x3000);
    let c0 = (status >> 8) & 1;

    assert_eq!(c0, 0, "MAX > 0");
}

#[test]
fn test_ftst_min_value() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xE4, // FTST
        0x9B, 0xDF, 0xE0, // FSTSW AX
        0x66, 0x67, 0xA3, 0x00, 0x30, 0x00, 0x00, // MOV [0x3000], AX
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, -f64::MAX);

    run_until_hlt(&mut vcpu).unwrap();

    let status = read_u16(&mem, 0x3000);
    let c0 = (status >> 8) & 1;

    assert_eq!(c0, 1, "-MAX < 0");
}

#[test]
fn test_ftst_various_positive() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xE4, // FTST
        0x9B, 0xDF, 0xE0, // FSTSW AX
        0x66, 0x67, 0xA3, 0x00, 0x30, 0x00, 0x00, // MOV [0x3000], AX
        0xF4, // HLT
    ];

    let test_values = vec![0.1, 0.5, 1.0, 10.0, 100.0, 1e10, 1e100];

    for val in test_values {
        let (mut vcpu, mem) = setup_vm(&code, None);
        write_f64(&mem, 0x2000, val);

        run_until_hlt(&mut vcpu).unwrap();

        let status = read_u16(&mem, 0x3000);
        let c3 = (status >> 14) & 1;
        let c2 = (status >> 10) & 1;
        let c0 = (status >> 8) & 1;

        assert_eq!(c3, 0, "{} > 0", val);
        assert_eq!(c2, 0, "{} > 0", val);
        assert_eq!(c0, 0, "{} > 0", val);
    }
}

#[test]
fn test_ftst_various_negative() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xE4, // FTST
        0x9B, 0xDF, 0xE0, // FSTSW AX
        0x66, 0x67, 0xA3, 0x00, 0x30, 0x00, 0x00, // MOV [0x3000], AX
        0xF4, // HLT
    ];

    let test_values = vec![-0.1, -0.5, -1.0, -10.0, -100.0, -1e10, -1e100];

    for val in test_values {
        let (mut vcpu, mem) = setup_vm(&code, None);
        write_f64(&mem, 0x2000, val);

        run_until_hlt(&mut vcpu).unwrap();

        let status = read_u16(&mem, 0x3000);
        let c3 = (status >> 14) & 1;
        let c2 = (status >> 10) & 1;
        let c0 = (status >> 8) & 1;

        assert_eq!(c3, 0, "{} < 0", val);
        assert_eq!(c2, 0, "{} < 0", val);
        assert_eq!(c0, 1, "{} < 0", val);
    }
}
