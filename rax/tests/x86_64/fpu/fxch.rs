//! Tests for the FXCH instruction.
//!
//! FXCH - Exchange Register Contents
//!
//! Exchanges the contents of ST(0) and ST(i).
//! If no source operand is specified, exchanges ST(0) and ST(1).
//!
//! Opcode: D9 C8+i (exchange ST(0) with ST(i))
//! Opcode: D9 C9 (exchange ST(0) with ST(1))
//!
//! Flags affected:
//! - C1: Set to 0
//! - C0, C2, C3: Undefined
//!
//! Reference: /Users/int/dev/rax/docs/fxch.txt

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
// FXCH - Basic Exchange with ST(1)
// ============================================================================

#[test]
fn test_fxch_st1_basic() {
    // Load two values, exchange them, store both
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000] ; ST(0) = 3.14
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00,
        0x00, // FLD qword [0x2008] ; ST(0) = 2.71, ST(1) = 3.14
        0xD9, 0xC9, // FXCH ST(1)        ; ST(0) = 3.14, ST(1) = 2.71
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000] ; store ST(0) = 3.14
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, // FSTP qword [0x3008] ; store ST(0) = 2.71
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 3.14);
    write_f64(&mem, 0x2008, 2.71);

    run_until_hlt(&mut vcpu).unwrap();

    let result1 = read_f64(&mem, 0x3000);
    let result2 = read_f64(&mem, 0x3008);
    assert_eq!(result1, 3.14, "After FXCH, ST(0) should be 3.14");
    assert_eq!(result2, 2.71, "After FXCH, ST(1) should be 2.71");
}

#[test]
fn test_fxch_default() {
    // FXCH without operand exchanges ST(0) and ST(1)
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000] ; ST(0) = 1.0
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00,
        0x00, // FLD qword [0x2008] ; ST(0) = 2.0, ST(1) = 1.0
        0xD9, 0xC9, // FXCH              ; ST(0) = 1.0, ST(1) = 2.0
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, // FSTP qword [0x3008]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 1.0);
    write_f64(&mem, 0x2008, 2.0);

    run_until_hlt(&mut vcpu).unwrap();

    let result1 = read_f64(&mem, 0x3000);
    let result2 = read_f64(&mem, 0x3008);
    assert_eq!(result1, 1.0, "FXCH exchanged ST(0)");
    assert_eq!(result2, 2.0, "FXCH exchanged ST(1)");
}

// ============================================================================
// FXCH - Exchange with Different Stack Positions
// ============================================================================

#[test]
fn test_fxch_st2() {
    // Exchange ST(0) with ST(2)
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000] ; ST(0) = 1.0
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00,
        0x00, // FLD qword [0x2008] ; ST(0) = 2.0, ST(1) = 1.0
        0xDD, 0x04, 0x25, 0x10, 0x20, 0x00,
        0x00, // FLD qword [0x2010] ; ST(0) = 3.0, ST(1) = 2.0, ST(2) = 1.0
        0xD9, 0xCA, // FXCH ST(2)        ; ST(0) = 1.0, ST(1) = 2.0, ST(2) = 3.0
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000] ; store 1.0
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, // FSTP qword [0x3008] ; store 2.0
        0xDD, 0x1C, 0x25, 0x10, 0x30, 0x00, 0x00, // FSTP qword [0x3010] ; store 3.0
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 1.0);
    write_f64(&mem, 0x2008, 2.0);
    write_f64(&mem, 0x2010, 3.0);

    run_until_hlt(&mut vcpu).unwrap();

    let result1 = read_f64(&mem, 0x3000);
    let result2 = read_f64(&mem, 0x3008);
    let result3 = read_f64(&mem, 0x3010);
    assert_eq!(result1, 1.0, "After FXCH ST(2), ST(0) should be 1.0");
    assert_eq!(result2, 2.0, "ST(1) should remain 2.0");
    assert_eq!(
        result3, 3.0,
        "After FXCH ST(2), old ST(0) should be at ST(2)"
    );
}

#[test]
fn test_fxch_st3() {
    // Exchange ST(0) with ST(3)
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000] ; ST(0) = 10.0
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008] ; ST(0) = 20.0
        0xDD, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, // FLD qword [0x2010] ; ST(0) = 30.0
        0xDD, 0x04, 0x25, 0x18, 0x20, 0x00, 0x00, // FLD qword [0x2018] ; ST(0) = 40.0
        0xD9, 0xCB, // FXCH ST(3)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 10.0);
    write_f64(&mem, 0x2008, 20.0);
    write_f64(&mem, 0x2010, 30.0);
    write_f64(&mem, 0x2018, 40.0);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert_eq!(result, 10.0, "After FXCH ST(3), ST(0) should be 10.0");
}

#[test]
fn test_fxch_st4() {
    // Exchange ST(0) with ST(4)
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000] ; 1.0
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008] ; 2.0
        0xDD, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, // FLD qword [0x2010] ; 3.0
        0xDD, 0x04, 0x25, 0x18, 0x20, 0x00, 0x00, // FLD qword [0x2018] ; 4.0
        0xDD, 0x04, 0x25, 0x20, 0x20, 0x00, 0x00, // FLD qword [0x2020] ; 5.0
        0xD9, 0xCC, // FXCH ST(4)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 1.0);
    write_f64(&mem, 0x2008, 2.0);
    write_f64(&mem, 0x2010, 3.0);
    write_f64(&mem, 0x2018, 4.0);
    write_f64(&mem, 0x2020, 5.0);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert_eq!(result, 1.0, "After FXCH ST(4), ST(0) should be 1.0");
}

#[test]
fn test_fxch_st5() {
    // Exchange ST(0) with ST(5)
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000] ; 1.0
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008] ; 2.0
        0xDD, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, // FLD qword [0x2010] ; 3.0
        0xDD, 0x04, 0x25, 0x18, 0x20, 0x00, 0x00, // FLD qword [0x2018] ; 4.0
        0xDD, 0x04, 0x25, 0x20, 0x20, 0x00, 0x00, // FLD qword [0x2020] ; 5.0
        0xDD, 0x04, 0x25, 0x28, 0x20, 0x00, 0x00, // FLD qword [0x2028] ; 6.0
        0xD9, 0xCD, // FXCH ST(5)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 1.0);
    write_f64(&mem, 0x2008, 2.0);
    write_f64(&mem, 0x2010, 3.0);
    write_f64(&mem, 0x2018, 4.0);
    write_f64(&mem, 0x2020, 5.0);
    write_f64(&mem, 0x2028, 6.0);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert_eq!(result, 1.0, "After FXCH ST(5), ST(0) should be 1.0");
}

#[test]
fn test_fxch_st6() {
    // Exchange ST(0) with ST(6)
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000] ; 1.0
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008] ; 2.0
        0xDD, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, // FLD qword [0x2010] ; 3.0
        0xDD, 0x04, 0x25, 0x18, 0x20, 0x00, 0x00, // FLD qword [0x2018] ; 4.0
        0xDD, 0x04, 0x25, 0x20, 0x20, 0x00, 0x00, // FLD qword [0x2020] ; 5.0
        0xDD, 0x04, 0x25, 0x28, 0x20, 0x00, 0x00, // FLD qword [0x2028] ; 6.0
        0xDD, 0x04, 0x25, 0x30, 0x20, 0x00, 0x00, // FLD qword [0x2030] ; 7.0
        0xD9, 0xCE, // FXCH ST(6)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 1.0);
    write_f64(&mem, 0x2008, 2.0);
    write_f64(&mem, 0x2010, 3.0);
    write_f64(&mem, 0x2018, 4.0);
    write_f64(&mem, 0x2020, 5.0);
    write_f64(&mem, 0x2028, 6.0);
    write_f64(&mem, 0x2030, 7.0);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert_eq!(result, 1.0, "After FXCH ST(6), ST(0) should be 1.0");
}

#[test]
fn test_fxch_st7() {
    // Exchange ST(0) with ST(7) - maximum stack position
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000] ; 1.0
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008] ; 2.0
        0xDD, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, // FLD qword [0x2010] ; 3.0
        0xDD, 0x04, 0x25, 0x18, 0x20, 0x00, 0x00, // FLD qword [0x2018] ; 4.0
        0xDD, 0x04, 0x25, 0x20, 0x20, 0x00, 0x00, // FLD qword [0x2020] ; 5.0
        0xDD, 0x04, 0x25, 0x28, 0x20, 0x00, 0x00, // FLD qword [0x2028] ; 6.0
        0xDD, 0x04, 0x25, 0x30, 0x20, 0x00, 0x00, // FLD qword [0x2030] ; 7.0
        0xDD, 0x04, 0x25, 0x38, 0x20, 0x00, 0x00, // FLD qword [0x2038] ; 8.0
        0xD9, 0xCF, // FXCH ST(7)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 1.0);
    write_f64(&mem, 0x2008, 2.0);
    write_f64(&mem, 0x2010, 3.0);
    write_f64(&mem, 0x2018, 4.0);
    write_f64(&mem, 0x2020, 5.0);
    write_f64(&mem, 0x2028, 6.0);
    write_f64(&mem, 0x2030, 7.0);
    write_f64(&mem, 0x2038, 8.0);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert_eq!(result, 1.0, "After FXCH ST(7), ST(0) should be 1.0");
}

// ============================================================================
// FXCH - Multiple Exchanges
// ============================================================================

#[test]
fn test_fxch_twice_cancels() {
    // Two FXCH operations should cancel out
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008]
        0xD9, 0xC9, // FXCH ST(1)
        0xD9, 0xC9, // FXCH ST(1) again
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, // FSTP qword [0x3008]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 5.0);
    write_f64(&mem, 0x2008, 7.0);

    run_until_hlt(&mut vcpu).unwrap();

    let result1 = read_f64(&mem, 0x3000);
    let result2 = read_f64(&mem, 0x3008);
    assert_eq!(result1, 7.0, "Two FXCH should cancel, ST(0) = 7.0");
    assert_eq!(result2, 5.0, "Two FXCH should cancel, ST(1) = 5.0");
}

#[test]
fn test_fxch_chain() {
    // Chain of FXCH operations to move value through stack
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000] ; 1.0
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008] ; 2.0
        0xDD, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, // FLD qword [0x2010] ; 3.0
        0xD9, 0xC9, // FXCH ST(1) ; swap ST(0) and ST(1)
        0xD9, 0xCA, // FXCH ST(2) ; swap ST(0) and ST(2)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 1.0);
    write_f64(&mem, 0x2008, 2.0);
    write_f64(&mem, 0x2010, 3.0);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert_eq!(result, 1.0, "After chain of FXCH");
}

// ============================================================================
// FXCH - With Arithmetic Operations
// ============================================================================

#[test]
fn test_fxch_with_fadd() {
    // Use FXCH to reorder operands for addition
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000] ; 5.0
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008] ; 3.0
        0xD9, 0xC9, // FXCH ST(1)
        0xDE, 0xC1, // FADDP ST(1), ST(0)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 5.0);
    write_f64(&mem, 0x2008, 3.0);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert_eq!(result, 8.0, "FXCH + FADDP should compute 5.0 + 3.0 = 8.0");
}

#[test]
fn test_fxch_sqrt_pattern() {
    // Example from docs: FXCH ST(3); FSQRT; FXCH ST(3)
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000] ; 16.0 (to sqrt)
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008] ; value to keep
        0xDD, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, // FLD qword [0x2010] ; value to keep
        0xDD, 0x04, 0x25, 0x18, 0x20, 0x00, 0x00, // FLD qword [0x2018] ; value to keep
        0xD9, 0xCB, // FXCH ST(3) ; bring 16.0 to top
        0xD9, 0xFA, // FSQRT
        0xD9, 0xCB, // FXCH ST(3) ; put result back
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000] ; discard top
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, // FSTP qword [0x3008]
        0xDD, 0x1C, 0x25, 0x10, 0x30, 0x00, 0x00, // FSTP qword [0x3010]
        0xDD, 0x1C, 0x25, 0x18, 0x30, 0x00, 0x00, // FSTP qword [0x3018] ; sqrt result
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 16.0);
    write_f64(&mem, 0x2008, 1.0);
    write_f64(&mem, 0x2010, 2.0);
    write_f64(&mem, 0x2018, 3.0);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3018);
    assert_eq!(result, 4.0, "SQRT(16) = 4.0 after FXCH pattern");
}

// ============================================================================
// FXCH - Special Values
// ============================================================================

#[test]
fn test_fxch_with_zero() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000] ; 0.0
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008] ; 5.0
        0xD9, 0xC9, // FXCH ST(1)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, // FSTP qword [0x3008]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 0.0);
    write_f64(&mem, 0x2008, 5.0);

    run_until_hlt(&mut vcpu).unwrap();

    let result1 = read_f64(&mem, 0x3000);
    let result2 = read_f64(&mem, 0x3008);
    assert_eq!(result1, 0.0, "FXCH with zero works");
    assert_eq!(result2, 5.0, "FXCH with zero works");
}

#[test]
fn test_fxch_with_infinity() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008]
        0xD9, 0xC9, // FXCH ST(1)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, f64::INFINITY);
    write_f64(&mem, 0x2008, 42.0);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert!(result.is_infinite(), "FXCH with infinity works");
}

#[test]
fn test_fxch_with_nan() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008]
        0xD9, 0xC9, // FXCH ST(1)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, f64::NAN);
    write_f64(&mem, 0x2008, 7.5);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert!(result.is_nan(), "FXCH with NaN works");
}

#[test]
fn test_fxch_negative_values() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008]
        0xD9, 0xC9, // FXCH ST(1)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, // FSTP qword [0x3008]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, -3.14);
    write_f64(&mem, 0x2008, -2.71);

    run_until_hlt(&mut vcpu).unwrap();

    let result1 = read_f64(&mem, 0x3000);
    let result2 = read_f64(&mem, 0x3008);
    assert_eq!(result1, -3.14, "FXCH with negative values");
    assert_eq!(result2, -2.71, "FXCH with negative values");
}

// ============================================================================
// FXCH - Edge Cases
// ============================================================================

#[test]
fn test_fxch_st0_is_nop() {
    // FXCH ST(0) should be a no-op (exchange ST(0) with itself)
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xC8, // FXCH ST(0)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 9.9);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert_eq!(result, 9.9, "FXCH ST(0) should be no-op");
}

#[test]
fn test_fxch_preserves_precision() {
    // Verify FXCH doesn't alter precision of values
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008]
        0xD9, 0xC9, // FXCH ST(1)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    let precise_val = std::f64::consts::PI;
    write_f64(&mem, 0x2000, precise_val);
    write_f64(&mem, 0x2008, 1.0);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert_eq!(result, precise_val, "FXCH should preserve full precision");
}

#[test]
fn test_fxch_with_very_small_values() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008]
        0xD9, 0xC9, // FXCH ST(1)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 1e-100);
    write_f64(&mem, 0x2008, 1e100);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert_eq!(result, 1e-100, "FXCH with very small value");
}

#[test]
fn test_fxch_with_denormals() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008]
        0xD9, 0xC9, // FXCH ST(1)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    let denormal = f64::MIN_POSITIVE / 2.0;
    write_f64(&mem, 0x2000, denormal);
    write_f64(&mem, 0x2008, 1.0);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert_eq!(result, denormal, "FXCH with denormal value");
}

#[test]
fn test_fxch_alternating_exchanges() {
    // Alternate exchanges should rotate values
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000] ; 1.0
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008] ; 2.0
        0xDD, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, // FLD qword [0x2010] ; 3.0
        0xD9, 0xC9, // FXCH ST(1)
        0xD9, 0xCA, // FXCH ST(2)
        0xD9, 0xC9, // FXCH ST(1)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 1.0);
    write_f64(&mem, 0x2008, 2.0);
    write_f64(&mem, 0x2010, 3.0);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert_eq!(result, 3.0, "After alternating FXCH");
}

#[test]
fn test_fxch_multiple_values_deep_stack() {
    // Test with full stack depth
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000] ; 1.0
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008] ; 2.0
        0xDD, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, // FLD qword [0x2010] ; 3.0
        0xDD, 0x04, 0x25, 0x18, 0x20, 0x00, 0x00, // FLD qword [0x2018] ; 4.0
        0xDD, 0x04, 0x25, 0x20, 0x20, 0x00, 0x00, // FLD qword [0x2020] ; 5.0
        0xD9, 0xCC, // FXCH ST(4) ; bring 1.0 to top
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 1.0);
    write_f64(&mem, 0x2008, 2.0);
    write_f64(&mem, 0x2010, 3.0);
    write_f64(&mem, 0x2018, 4.0);
    write_f64(&mem, 0x2020, 5.0);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert_eq!(result, 1.0, "FXCH ST(4) should bring bottom to top");
}

#[test]
fn test_fxch_pi_and_e() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000] ; PI
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008] ; E
        0xD9, 0xC9, // FXCH ST(1)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, // FSTP qword [0x3008]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, std::f64::consts::PI);
    write_f64(&mem, 0x2008, std::f64::consts::E);

    run_until_hlt(&mut vcpu).unwrap();

    let result1 = read_f64(&mem, 0x3000);
    let result2 = read_f64(&mem, 0x3008);
    assert_eq!(result1, std::f64::consts::PI, "FXCH with PI");
    assert_eq!(result2, std::f64::consts::E, "FXCH with E");
}

#[test]
fn test_fxch_max_and_min() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008]
        0xD9, 0xC9, // FXCH ST(1)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, f64::MAX);
    write_f64(&mem, 0x2008, f64::MIN_POSITIVE);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert_eq!(result, f64::MAX, "FXCH with MAX value");
}

#[test]
fn test_fxch_mixed_signs() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008]
        0xDD, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, // FLD qword [0x2010]
        0xD9, 0xCA, // FXCH ST(2)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 100.0);
    write_f64(&mem, 0x2008, -50.0);
    write_f64(&mem, 0x2010, 25.0);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert_eq!(result, 100.0, "FXCH with mixed signs");
}

#[test]
fn test_fxch_fractional_values() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008]
        0xD9, 0xC9, // FXCH ST(1)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 0.123456789);
    write_f64(&mem, 0x2008, 0.987654321);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert_eq!(result, 0.123456789, "FXCH preserves fractional precision");
}

#[test]
fn test_fxch_power_of_two_values() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008]
        0xD9, 0xC9, // FXCH ST(1)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 1024.0);
    write_f64(&mem, 0x2008, 2048.0);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert_eq!(result, 1024.0, "FXCH with powers of 2");
}

#[test]
fn test_fxch_after_arithmetic() {
    // FXCH after arithmetic operation
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000] ; 10.0
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008] ; 5.0
        0xDE, 0xC1, // FADDP ; ST(0) = 15.0
        0xDD, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, // FLD qword [0x2010] ; 3.0
        0xD9, 0xC9, // FXCH ST(1) ; swap 3.0 and 15.0
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000] ; store 15.0
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 10.0);
    write_f64(&mem, 0x2008, 5.0);
    write_f64(&mem, 0x2010, 3.0);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert_eq!(result, 15.0, "FXCH after arithmetic");
}

#[test]
fn test_fxch_before_comparison() {
    // Use FXCH to prepare for comparison
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008]
        0xD9, 0xC9, // FXCH ST(1)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 7.5);
    write_f64(&mem, 0x2008, 3.5);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert_eq!(result, 7.5, "FXCH for comparison setup");
}

#[test]
fn test_fxch_zero_and_nonzero() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008]
        0xD9, 0xC9, // FXCH ST(1)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, // FSTP qword [0x3008]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 0.0);
    write_f64(&mem, 0x2008, 100.0);

    run_until_hlt(&mut vcpu).unwrap();

    let result1 = read_f64(&mem, 0x3000);
    let result2 = read_f64(&mem, 0x3008);
    assert_eq!(result1, 0.0, "Zero value exchanged");
    assert_eq!(result2, 100.0, "Non-zero value exchanged");
}

// ============================================================================
// Known-answer FXCH tests: exact swap of stack slots.
// ============================================================================

#[test]
fn test_fxch_st1_exact_swap() {
    // FLD 1.0, FLD 2.0, FLD 3.0 -> ST(0)=3, ST(1)=2, ST(2)=1
    // FXCH ST(1) -> ST(0)=2, ST(1)=3
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD,
        0x04, 0x25, 0x10, 0x20, 0x00, 0x00, 0xD9, 0xC9, // FXCH ST(1)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // pop -> 2.0
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, // pop -> 3.0
        0xDD, 0x1C, 0x25, 0x10, 0x30, 0x00, 0x00, // pop -> 1.0
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 1.0);
    write_f64(&mem, 0x2008, 2.0);
    write_f64(&mem, 0x2010, 3.0);
    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 2.0);
    assert_eq!(read_f64(&mem, 0x3008), 3.0);
    assert_eq!(read_f64(&mem, 0x3010), 1.0);
}

#[test]
fn test_fxch_st2_exact_swap() {
    // FLD 1.0, FLD 2.0, FLD 3.0 -> ST(0)=3, ST(1)=2, ST(2)=1
    // FXCH ST(2) -> ST(0)=1, ST(2)=3
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD,
        0x04, 0x25, 0x10, 0x20, 0x00, 0x00, 0xD9, 0xCA, // FXCH ST(2)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // pop -> 1.0
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, // pop -> 2.0
        0xDD, 0x1C, 0x25, 0x10, 0x30, 0x00, 0x00, // pop -> 3.0
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 1.0);
    write_f64(&mem, 0x2008, 2.0);
    write_f64(&mem, 0x2010, 3.0);
    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 1.0);
    assert_eq!(read_f64(&mem, 0x3008), 2.0);
    assert_eq!(read_f64(&mem, 0x3010), 3.0);
}

#[test]
fn test_fxch_twice_is_identity() {
    // Two FXCH ST(1) in a row leaves the stack unchanged.
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, 0xD9,
        0xC9, // FXCH ST(1)
        0xD9, 0xC9, // FXCH ST(1)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // pop -> 2.0
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, // pop -> 1.0
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 1.0);
    write_f64(&mem, 0x2008, 2.0);
    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 2.0);
    assert_eq!(read_f64(&mem, 0x3008), 1.0);
}
