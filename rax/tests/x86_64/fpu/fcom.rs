//! Tests for the FCOM, FCOMP, and FCOMPP instructions.
//!
//! FCOM/FCOMP/FCOMPP - Compare Floating-Point Values
//!
//! FCOM compares ST(0) with a source value and sets condition code flags
//! C0, C2, and C3 in the FPU status word according to the results.
//!
//! FCOMP performs the same comparison and then pops the register stack.
//! FCOMPP compares ST(0) with ST(1) and pops the stack twice.
//!
//! Comparison Results:
//! - ST(0) > SRC: C3=0, C2=0, C0=0
//! - ST(0) < SRC: C3=0, C2=0, C0=1
//! - ST(0) = SRC: C3=1, C2=0, C0=0
//! - Unordered:   C3=1, C2=1, C0=1 (NaN operand)
//!
//! Opcodes:
//! - FCOM: D8 /2 (mem), D8 D0+i (reg)
//! - FCOMP: D8 /3 (mem), D8 D8+i (reg)
//! - FCOMPP: DE D9
//!
//! Flags affected:
//! - C1: Set to 0
//! - C0, C2, C3: Set according to comparison result
//!
//! Reference: /Users/int/dev/rax/docs/fcom:fcomp:fcompp.txt

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

// FPU status word bit positions for condition codes
const C0_BIT: u16 = 0x0100; // bit 8
const C2_BIT: u16 = 0x0400; // bit 10
const C3_BIT: u16 = 0x4000; // bit 14

// Helper to extract FPU status word
// Note: This is a simplified test - actual implementation would need to
// read the FPU status word through FSTSW instruction

// ============================================================================
// FCOM - Compare ST(0) with ST(1)
// ============================================================================

#[test]
fn test_fcom_equal() {
    // Compare 5.0 with 5.0 (equal)
    // FLD qword [0x2000]  ; Load 5.0 into ST(0)
    // FLD qword [0x2008]  ; Load 5.0 into ST(0), previous ST(0) becomes ST(1)
    // FCOM ST(1)          ; D8 D1 - Compare ST(0) with ST(1)
    // FSTP qword [0x3000] ; Store result flag test helper
    // FSTP qword [0x3008] ; Clean stack
    // HLT
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008]
        0xD8, 0xD1, // FCOM ST(1)
        // For testing, we'll store both values to verify they're still on stack
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, // FSTP qword [0x3008]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 5.0);
    write_f64(&mem, 0x2008, 5.0);

    run_until_hlt(&mut vcpu).unwrap();

    // Verify values are unchanged (FCOM doesn't modify stack)
    let val1 = read_f64(&mem, 0x3000);
    let val2 = read_f64(&mem, 0x3008);
    assert_eq!(val1, 5.0, "ST(0) should be unchanged");
    assert_eq!(val2, 5.0, "ST(1) should be unchanged");
    // In actual test, would check C3=1, C2=0, C0=0 for equality
}

#[test]
fn test_fcom_greater_than() {
    // Compare 10.0 > 5.0
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000] (5.0)
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008] (10.0)
        0xD8, 0xD1, // FCOM ST(1)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, // FSTP qword [0x3008]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 5.0);
    write_f64(&mem, 0x2008, 10.0);

    run_until_hlt(&mut vcpu).unwrap();

    let val1 = read_f64(&mem, 0x3000);
    let val2 = read_f64(&mem, 0x3008);
    assert_eq!(val1, 10.0, "ST(0) should be 10.0");
    assert_eq!(val2, 5.0, "ST(1) should be 5.0");
    // Should set C3=0, C2=0, C0=0 (ST(0) > ST(1))
}

#[test]
fn test_fcom_less_than() {
    // Compare 3.0 < 7.0
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000] (7.0)
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008] (3.0)
        0xD8, 0xD1, // FCOM ST(1)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, // FSTP qword [0x3008]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 7.0);
    write_f64(&mem, 0x2008, 3.0);

    run_until_hlt(&mut vcpu).unwrap();

    let val1 = read_f64(&mem, 0x3000);
    let val2 = read_f64(&mem, 0x3008);
    assert_eq!(val1, 3.0, "ST(0) should be 3.0");
    assert_eq!(val2, 7.0, "ST(1) should be 7.0");
    // Should set C3=0, C2=0, C0=1 (ST(0) < ST(1))
}

#[test]
fn test_fcom_zero_equal() {
    // Compare 0.0 with 0.0
    let code = [
        0xD9, 0xEE, // FLDZ
        0xD9, 0xEE, // FLDZ
        0xD8, 0xD1, // FCOM ST(1)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, // FSTP qword [0x3008]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    run_until_hlt(&mut vcpu).unwrap();

    let val1 = read_f64(&mem, 0x3000);
    let val2 = read_f64(&mem, 0x3008);
    assert_eq!(val1, 0.0);
    assert_eq!(val2, 0.0);
}

#[test]
fn test_fcom_positive_negative_zero() {
    // +0.0 should equal -0.0
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000] (-0.0)
        0xD9, 0xEE, // FLDZ (+0.0)
        0xD8, 0xD1, // FCOM ST(1)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, // FSTP qword [0x3008]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, -0.0);

    run_until_hlt(&mut vcpu).unwrap();

    // The sign of zero is ignored in comparison
}

// ============================================================================
// FCOMP - Compare and Pop Once
// ============================================================================

#[test]
fn test_fcomp_equal() {
    // Compare and pop - stack should have one less value
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008]
        0xD8, 0xD9, // FCOMP ST(1) - compare and pop
        // After FCOMP, only one value should be on stack
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 5.0);
    write_f64(&mem, 0x2008, 5.0);

    run_until_hlt(&mut vcpu).unwrap();

    let val = read_f64(&mem, 0x3000);
    assert_eq!(
        val, 5.0,
        "After FCOMP, ST(0) should contain the remaining value"
    );
}

#[test]
fn test_fcomp_greater() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000] (3.0)
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008] (8.0)
        0xD8, 0xD9, // FCOMP ST(1)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 3.0);
    write_f64(&mem, 0x2008, 8.0);

    run_until_hlt(&mut vcpu).unwrap();

    let val = read_f64(&mem, 0x3000);
    assert_eq!(val, 3.0);
}

#[test]
fn test_fcomp_less() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000] (9.0)
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008] (2.0)
        0xD8, 0xD9, // FCOMP ST(1)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 9.0);
    write_f64(&mem, 0x2008, 2.0);

    run_until_hlt(&mut vcpu).unwrap();

    let val = read_f64(&mem, 0x3000);
    assert_eq!(val, 9.0);
}

#[test]
fn test_fcomp_with_constant() {
    // Compare with loaded constant
    let code = [
        0xD9, 0xE8, // FLD1
        0xD9, 0xE8, // FLD1
        0xD8, 0xD9, // FCOMP ST(1)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    run_until_hlt(&mut vcpu).unwrap();

    let val = read_f64(&mem, 0x3000);
    assert_eq!(val, 1.0);
}

// ============================================================================
// FCOMPP - Compare and Pop Twice
// ============================================================================

#[test]
fn test_fcompp_equal() {
    // Compare ST(0) with ST(1) and pop both
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008]
        0xDE, 0xD9, // FCOMPP - compare and pop twice
        // Stack should now be empty, push a marker value to verify
        0xD9, 0xE8, // FLD1
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 4.0);
    write_f64(&mem, 0x2008, 4.0);

    run_until_hlt(&mut vcpu).unwrap();

    let marker = read_f64(&mem, 0x3000);
    assert_eq!(
        marker, 1.0,
        "Stack should be empty after FCOMPP, FLD1 should work"
    );
}

#[test]
fn test_fcompp_greater() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000] (2.0)
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008] (6.0)
        0xDE, 0xD9, // FCOMPP
        0xD9, 0xE8, // FLD1 (marker)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 2.0);
    write_f64(&mem, 0x2008, 6.0);

    run_until_hlt(&mut vcpu).unwrap();

    let marker = read_f64(&mem, 0x3000);
    assert_eq!(marker, 1.0);
}

#[test]
fn test_fcompp_less() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000] (8.0)
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008] (3.0)
        0xDE, 0xD9, // FCOMPP
        0xD9, 0xEE, // FLDZ (marker)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 8.0);
    write_f64(&mem, 0x2008, 3.0);

    run_until_hlt(&mut vcpu).unwrap();

    let marker = read_f64(&mem, 0x3000);
    assert_eq!(marker, 0.0);
}

#[test]
fn test_fcompp_with_constants() {
    // Compare two constants
    let code = [
        0xD9, 0xEE, // FLDZ
        0xD9, 0xE8, // FLD1
        0xDE, 0xD9, // FCOMPP
        0xD9, 0xEB, // FLDPI (marker)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    run_until_hlt(&mut vcpu).unwrap();

    let marker = read_f64(&mem, 0x3000);
    assert!((marker - std::f64::consts::PI).abs() < 1e-15);
}

// ============================================================================
// FCOM - Special Values
// ============================================================================

#[test]
fn test_fcom_infinity_greater() {
    // +infinity > finite
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000] (100.0)
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008] (+inf)
        0xD8, 0xD1, // FCOM ST(1)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, // FSTP qword [0x3008]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 100.0);
    write_f64(&mem, 0x2008, f64::INFINITY);

    run_until_hlt(&mut vcpu).unwrap();

    let val1 = read_f64(&mem, 0x3000);
    let val2 = read_f64(&mem, 0x3008);
    assert_eq!(val1, f64::INFINITY);
    assert_eq!(val2, 100.0);
}

#[test]
fn test_fcom_negative_infinity_less() {
    // -infinity < finite
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000] (0.0)
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008] (-inf)
        0xD8, 0xD1, // FCOM ST(1)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, // FSTP qword [0x3008]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 0.0);
    write_f64(&mem, 0x2008, f64::NEG_INFINITY);

    run_until_hlt(&mut vcpu).unwrap();

    let val1 = read_f64(&mem, 0x3000);
    let val2 = read_f64(&mem, 0x3008);
    assert_eq!(val1, f64::NEG_INFINITY);
    assert_eq!(val2, 0.0);
}

#[test]
fn test_fcom_infinities_equal() {
    // +inf == +inf
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000] (+inf)
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008] (+inf)
        0xD8, 0xD1, // FCOM ST(1)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, // FSTP qword [0x3008]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, f64::INFINITY);
    write_f64(&mem, 0x2008, f64::INFINITY);

    run_until_hlt(&mut vcpu).unwrap();

    let val1 = read_f64(&mem, 0x3000);
    let val2 = read_f64(&mem, 0x3008);
    assert_eq!(val1, f64::INFINITY);
    assert_eq!(val2, f64::INFINITY);
}

#[test]
fn test_fcom_infinities_different() {
    // +inf > -inf
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000] (-inf)
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008] (+inf)
        0xD8, 0xD1, // FCOM ST(1)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, // FSTP qword [0x3008]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, f64::NEG_INFINITY);
    write_f64(&mem, 0x2008, f64::INFINITY);

    run_until_hlt(&mut vcpu).unwrap();

    let val1 = read_f64(&mem, 0x3000);
    let val2 = read_f64(&mem, 0x3008);
    assert_eq!(val1, f64::INFINITY);
    assert_eq!(val2, f64::NEG_INFINITY);
}

// ============================================================================
// FCOM - NaN Comparisons (Unordered)
// ============================================================================

#[test]
fn test_fcom_nan_vs_number() {
    // NaN compared with any number is unordered
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000] (5.0)
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008] (NaN)
        0xD8, 0xD1, // FCOM ST(1)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, // FSTP qword [0x3008]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 5.0);
    write_f64(&mem, 0x2008, f64::NAN);

    run_until_hlt(&mut vcpu).unwrap();

    let val1 = read_f64(&mem, 0x3000);
    let val2 = read_f64(&mem, 0x3008);
    assert!(val1.is_nan());
    assert_eq!(val2, 5.0);
    // Should set C3=1, C2=1, C0=1 (unordered)
}

#[test]
fn test_fcom_number_vs_nan() {
    // Number compared with NaN is unordered
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000] (NaN)
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008] (10.0)
        0xD8, 0xD1, // FCOM ST(1)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, // FSTP qword [0x3008]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, f64::NAN);
    write_f64(&mem, 0x2008, 10.0);

    run_until_hlt(&mut vcpu).unwrap();

    let val1 = read_f64(&mem, 0x3000);
    let val2 = read_f64(&mem, 0x3008);
    assert_eq!(val1, 10.0);
    assert!(val2.is_nan());
}

#[test]
fn test_fcom_nan_vs_nan() {
    // NaN compared with NaN is unordered
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000] (NaN)
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008] (NaN)
        0xD8, 0xD1, // FCOM ST(1)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, // FSTP qword [0x3008]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, f64::NAN);
    write_f64(&mem, 0x2008, f64::NAN);

    run_until_hlt(&mut vcpu).unwrap();

    let val1 = read_f64(&mem, 0x3000);
    let val2 = read_f64(&mem, 0x3008);
    assert!(val1.is_nan());
    assert!(val2.is_nan());
}

// ============================================================================
// FCOM - Various Numeric Comparisons
// ============================================================================

#[test]
fn test_fcom_very_close_numbers() {
    // Test comparison of very close numbers
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008]
        0xD8, 0xD1, // FCOM ST(1)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, // FSTP qword [0x3008]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 1.0);
    write_f64(&mem, 0x2008, 1.0 + 1e-15);

    run_until_hlt(&mut vcpu).unwrap();

    // Should detect the difference
}

#[test]
fn test_fcom_negative_numbers() {
    // -5.0 > -10.0
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000] (-10.0)
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008] (-5.0)
        0xD8, 0xD1, // FCOM ST(1)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, // FSTP qword [0x3008]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, -10.0);
    write_f64(&mem, 0x2008, -5.0);

    run_until_hlt(&mut vcpu).unwrap();

    let val1 = read_f64(&mem, 0x3000);
    let val2 = read_f64(&mem, 0x3008);
    assert_eq!(val1, -5.0);
    assert_eq!(val2, -10.0);
    // -5.0 > -10.0, so C3=0, C2=0, C0=0
}

#[test]
fn test_fcom_mixed_signs() {
    // Positive > Negative
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000] (-3.0)
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008] (3.0)
        0xD8, 0xD1, // FCOM ST(1)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, // FSTP qword [0x3008]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, -3.0);
    write_f64(&mem, 0x2008, 3.0);

    run_until_hlt(&mut vcpu).unwrap();

    let val1 = read_f64(&mem, 0x3000);
    let val2 = read_f64(&mem, 0x3008);
    assert_eq!(val1, 3.0);
    assert_eq!(val2, -3.0);
}

#[test]
fn test_fcom_tiny_numbers() {
    // Compare very small numbers
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008]
        0xD8, 0xD1, // FCOM ST(1)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, // FSTP qword [0x3008]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 1e-100);
    write_f64(&mem, 0x2008, 2e-100);

    run_until_hlt(&mut vcpu).unwrap();

    let val1 = read_f64(&mem, 0x3000);
    let val2 = read_f64(&mem, 0x3008);
    assert_eq!(val1, 2e-100);
    assert_eq!(val2, 1e-100);
}

#[test]
fn test_fcom_huge_numbers() {
    // Compare very large numbers
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008]
        0xD8, 0xD1, // FCOM ST(1)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, // FSTP qword [0x3008]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 1e100);
    write_f64(&mem, 0x2008, 2e100);

    run_until_hlt(&mut vcpu).unwrap();

    let val1 = read_f64(&mem, 0x3000);
    let val2 = read_f64(&mem, 0x3008);
    assert_eq!(val1, 2e100);
    assert_eq!(val2, 1e100);
}

// ============================================================================
// FCOM - Sequence of Comparisons
// ============================================================================

#[test]
fn test_fcom_sequence() {
    // Multiple comparisons in sequence
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000] (1.0)
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008] (2.0)
        0xD8, 0xD1, // FCOM ST(1) - compare 2.0 vs 1.0
        0xDD, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, // FLD qword [0x2010] (3.0)
        0xD8, 0xD1, // FCOM ST(1) - compare 3.0 vs 2.0
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, // FSTP qword [0x3008]
        0xDD, 0x1C, 0x25, 0x10, 0x30, 0x00, 0x00, // FSTP qword [0x3010]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 1.0);
    write_f64(&mem, 0x2008, 2.0);
    write_f64(&mem, 0x2010, 3.0);

    run_until_hlt(&mut vcpu).unwrap();

    let val1 = read_f64(&mem, 0x3000);
    let val2 = read_f64(&mem, 0x3008);
    let val3 = read_f64(&mem, 0x3010);
    assert_eq!(val1, 3.0);
    assert_eq!(val2, 2.0);
    assert_eq!(val3, 1.0);
}

#[test]
fn test_mixed_compare_operations() {
    // Mix FCOM, FCOMP, and FCOMPP
    let code = [
        0xD9, 0xE8, // FLD1 (1.0)
        0xD9, 0xE8, // FLD1 (1.0)
        0xD8, 0xD1, // FCOM ST(1)
        0xD9, 0xE8, // FLD1 (1.0)
        0xD8, 0xD9, // FCOMP ST(1) - pop once
        0xDE, 0xD9, // FCOMPP - pop twice
        0xD9, 0xEE, // FLDZ (marker - stack should be empty)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    run_until_hlt(&mut vcpu).unwrap();

    let marker = read_f64(&mem, 0x3000);
    assert_eq!(marker, 0.0);
}

#[test]
fn test_fcom_with_arithmetic() {
    // Compare after arithmetic operation
    let code = [
        0xD9, 0xE8, // FLD1
        0xD9, 0xE8, // FLD1
        0xDE, 0xC1, // FADDP (1 + 1 = 2)
        0xD9, 0xE8, // FLD1
        0xD9, 0xE8, // FLD1
        0xDE, 0xC1, // FADDP (1 + 1 = 2)
        0xD8, 0xD1, // FCOM ST(1) - compare 2 vs 2
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, // FSTP qword [0x3008]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    run_until_hlt(&mut vcpu).unwrap();

    let val1 = read_f64(&mem, 0x3000);
    let val2 = read_f64(&mem, 0x3008);
    assert_eq!(val1, 2.0);
    assert_eq!(val2, 2.0);
}

#[test]
fn test_fcom_fractions() {
    // Compare fractions
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008]
        0xD8, 0xD1, // FCOM ST(1)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, // FSTP qword [0x3008]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 0.25);
    write_f64(&mem, 0x2008, 0.5);

    run_until_hlt(&mut vcpu).unwrap();

    let val1 = read_f64(&mem, 0x3000);
    let val2 = read_f64(&mem, 0x3008);
    assert_eq!(val1, 0.5);
    assert_eq!(val2, 0.25);
}

#[test]
fn test_fcomp_negative_values() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008]
        0xD8, 0xD9, // FCOMP ST(1)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, -10.0);
    write_f64(&mem, 0x2008, -5.0);

    run_until_hlt(&mut vcpu).unwrap();

    let val = read_f64(&mem, 0x3000);
    assert_eq!(val, -10.0);
}

#[test]
fn test_fcompp_pi_comparison() {
    // Compare two π values
    let code = [
        0xD9, 0xEB, // FLDPI
        0xD9, 0xEB, // FLDPI
        0xDE, 0xD9, // FCOMPP
        0xD9, 0xE8, // FLD1 (marker)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    run_until_hlt(&mut vcpu).unwrap();

    let marker = read_f64(&mem, 0x3000);
    assert_eq!(marker, 1.0);
}

#[test]
fn test_fcom_denormal_numbers() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008]
        0xD8, 0xD1, // FCOM ST(1)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, // FSTP qword [0x3008]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    let denormal1 = f64::MIN_POSITIVE / 2.0;
    let denormal2 = f64::MIN_POSITIVE / 4.0;
    write_f64(&mem, 0x2000, denormal2);
    write_f64(&mem, 0x2008, denormal1);

    run_until_hlt(&mut vcpu).unwrap();

    // denormal1 > denormal2
}

#[test]
fn test_fcom_max_values() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008]
        0xD8, 0xD1, // FCOM ST(1)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, // FSTP qword [0x3008]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, f64::MAX);
    write_f64(&mem, 0x2008, f64::MAX);

    run_until_hlt(&mut vcpu).unwrap();

    let val1 = read_f64(&mem, 0x3000);
    let val2 = read_f64(&mem, 0x3008);
    assert_eq!(val1, f64::MAX);
    assert_eq!(val2, f64::MAX);
}

#[test]
fn test_fcomp_sequential() {
    // Multiple FCOMP operations
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000] (1.0)
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008] (2.0)
        0xD8, 0xD9, // FCOMP ST(1)
        0xDD, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, // FLD qword [0x2010] (3.0)
        0xD8, 0xD9, // FCOMP ST(1)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 1.0);
    write_f64(&mem, 0x2008, 2.0);
    write_f64(&mem, 0x2010, 3.0);

    run_until_hlt(&mut vcpu).unwrap();

    // Stack should have been popped twice
}

#[test]
fn test_fcom_powers_of_ten() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008]
        0xD8, 0xD1, // FCOM ST(1)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, // FSTP qword [0x3008]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 1e10);
    write_f64(&mem, 0x2008, 1e20);

    run_until_hlt(&mut vcpu).unwrap();

    let val1 = read_f64(&mem, 0x3000);
    let val2 = read_f64(&mem, 0x3008);
    assert_eq!(val1, 1e20);
    assert_eq!(val2, 1e10);
}

#[test]
fn test_fcompp_negative_infinity() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008]
        0xDE, 0xD9, // FCOMPP
        0xD9, 0xEB, // FLDPI (marker)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, f64::NEG_INFINITY);
    write_f64(&mem, 0x2008, f64::NEG_INFINITY);

    run_until_hlt(&mut vcpu).unwrap();

    let marker = read_f64(&mem, 0x3000);
    assert!((marker - std::f64::consts::PI).abs() < 1e-15);
}

#[test]
fn test_fcom_epsilon() {
    // Test comparison near epsilon
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008]
        0xD8, 0xD1, // FCOM ST(1)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, // FSTP qword [0x3008]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 1.0);
    write_f64(&mem, 0x2008, 1.0 + f64::EPSILON);

    run_until_hlt(&mut vcpu).unwrap();

    // Should detect the tiny difference
}

#[test]
fn test_fcom_mixed_sign_small() {
    // Small positive vs small negative
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008]
        0xD8, 0xD1, // FCOM ST(1)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, // FSTP qword [0x3008]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, -0.001);
    write_f64(&mem, 0x2008, 0.001);

    run_until_hlt(&mut vcpu).unwrap();

    let val1 = read_f64(&mem, 0x3000);
    let val2 = read_f64(&mem, 0x3008);
    assert_eq!(val1, 0.001);
    assert_eq!(val2, -0.001);
}

#[test]
fn test_fcompp_after_arithmetic() {
    // Compare results of arithmetic
    let code = [
        0xD9, 0xE8, // FLD1
        0xD9, 0xE8, // FLD1
        0xDE, 0xC1, // FADDP (2)
        0xD9, 0xE8, // FLD1
        0xD9, 0xE8, // FLD1
        0xDE, 0xC9, // FMULP (1)
        0xDE, 0xD9, // FCOMPP (compare 2 vs 1)
        0xD9, 0xEE, // FLDZ (marker)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    run_until_hlt(&mut vcpu).unwrap();

    let marker = read_f64(&mem, 0x3000);
    assert_eq!(marker, 0.0);
}
#[test]
fn test_fcom_constants_comparison() {
    // Compare various constant combinations
    let code = [
        0xD9, 0xEB, // FLDPI
        0xD9, 0xEA, // FLDL2E
        0xD8, 0xD1, // FCOM ST(1)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, // FSTP qword [0x3008]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    run_until_hlt(&mut vcpu).unwrap();

    let val1 = read_f64(&mem, 0x3000);
    let val2 = read_f64(&mem, 0x3008);
    assert!((val1 - std::f64::consts::LOG2_E).abs() < 1e-15);
    assert!((val2 - std::f64::consts::PI).abs() < 1e-15);
}

// ============================================================================
// Known-answer status-word (C0/C2/C3) assertions for FCOM.
//
// These actually read the FPU status word back via FNSTSW m16 and assert the
// exact condition-code bits, which the comparison tests above only described in
// comments. C3 behaves like the x86 ZF (equal), C0 like CF (less-than), and C2
// is set only for unordered (NaN) results.
// ============================================================================

/// Read the FPU status word stored to memory at `addr` by FNSTSW.
fn kat_read_sw(mem: &vm_memory::GuestMemoryMmap, addr: u64) -> u16 {
    let mut buf = [0u8; 2];
    mem.read_slice(&mut buf, GuestAddress(addr)).unwrap();
    u16::from_le_bytes(buf)
}

/// FLD [0x2000], FLD [0x2008], FCOM ST(1), FNSTSW [0x3100], HLT.
/// Returns the status word so condition codes can be inspected.
fn kat_fcom_sw(a: f64, b: f64) -> u16 {
    // ST(0) ends up = b (loaded last), ST(1) = a. FCOM ST(1) compares ST(0):b vs ST(1):a.
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008]
        0xD8, 0xD1, // FCOM ST(1)
        0xDD, 0x3C, 0x25, 0x00, 0x31, 0x00, 0x00, // FNSTSW [0x3100]
        0xF4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, a);
    write_f64(&mem, 0x2008, b);
    run_until_hlt(&mut vcpu).unwrap();
    kat_read_sw(&mem, 0x3100)
}

#[test]
fn test_fcom_sw_equal_sets_c3_only() {
    // ST(0)=5.0 == ST(1)=5.0  -> C3=1, C2=0, C0=0
    let sw = kat_fcom_sw(5.0, 5.0);
    assert_ne!(sw & C3_BIT, 0, "C3 must be set for equal");
    assert_eq!(sw & C2_BIT, 0, "C2 must be clear for ordered");
    assert_eq!(sw & C0_BIT, 0, "C0 must be clear for equal");
}

#[test]
fn test_fcom_sw_greater_clears_all() {
    // ST(1)=5.0, ST(0)=10.0 -> ST(0) > src -> C3=0, C2=0, C0=0
    let sw = kat_fcom_sw(5.0, 10.0);
    assert_eq!(sw & C3_BIT, 0, "C3 must be clear for greater");
    assert_eq!(sw & C2_BIT, 0, "C2 must be clear for ordered");
    assert_eq!(sw & C0_BIT, 0, "C0 must be clear for greater");
}

#[test]
fn test_fcom_sw_less_sets_c0_only() {
    // ST(1)=7.0, ST(0)=3.0 -> ST(0) < src -> C3=0, C2=0, C0=1
    let sw = kat_fcom_sw(7.0, 3.0);
    assert_eq!(sw & C3_BIT, 0, "C3 must be clear for less");
    assert_eq!(sw & C2_BIT, 0, "C2 must be clear for ordered");
    assert_ne!(sw & C0_BIT, 0, "C0 must be set for less");
}

#[test]
fn test_fcom_sw_nan_unordered_sets_all() {
    // NaN operand -> unordered -> C3=1, C2=1, C0=1
    let sw = kat_fcom_sw(1.0, f64::NAN);
    assert_ne!(sw & C3_BIT, 0, "C3 must be set for unordered");
    assert_ne!(sw & C2_BIT, 0, "C2 must be set for unordered");
    assert_ne!(sw & C0_BIT, 0, "C0 must be set for unordered");
}

#[test]
fn test_fcom_sw_c3_acts_like_zf() {
    // FNSTSW AX then SAHF maps C3->ZF; here we just confirm C3 reflects equality.
    assert_ne!(kat_fcom_sw(2.5, 2.5) & C3_BIT, 0, "equal -> C3(ZF-like)=1");
    assert_eq!(kat_fcom_sw(2.5, 3.5) & C3_BIT, 0, "unequal(less) -> C3=0");
    assert_eq!(
        kat_fcom_sw(3.5, 2.5) & C3_BIT,
        0,
        "unequal(greater) -> C3=0"
    );
}

#[test]
fn test_fcom_sw_neg_zero_equals_pos_zero() {
    // -0.0 compares equal to +0.0 -> C3=1
    let sw = kat_fcom_sw(0.0, -0.0);
    assert_ne!(sw & C3_BIT, 0, "+0.0 == -0.0 -> C3=1");
    assert_eq!(sw & C0_BIT, 0);
    assert_eq!(sw & C2_BIT, 0);
}

#[test]
fn test_fcom_sw_infinity_ordering() {
    // +inf vs finite: ST(0)=+inf > src=1.0 -> all clear
    let sw = kat_fcom_sw(1.0, f64::INFINITY);
    assert_eq!(sw & C3_BIT, 0);
    assert_eq!(sw & C2_BIT, 0);
    assert_eq!(sw & C0_BIT, 0);
    // -inf vs finite: ST(0)=-inf < src=1.0 -> C0 set
    let sw2 = kat_fcom_sw(1.0, f64::NEG_INFINITY);
    assert_ne!(sw2 & C0_BIT, 0);
    assert_eq!(sw2 & C3_BIT, 0);
    assert_eq!(sw2 & C2_BIT, 0);
}
