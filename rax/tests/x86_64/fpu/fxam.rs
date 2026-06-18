//! Tests for the FXAM instruction.
//!
//! FXAM - Examine Floating-Point
//!
//! Examines the contents of ST(0) and sets condition code flags C0, C2, C3
//! in the FPU status word to indicate the class of value or number.
//! C1 is set to the sign bit of the value in ST(0).
//!
//! Opcode: D9 E5
//!
//! Classification (C3 C2 C0):
//! - 000: Unsupported
//! - 001: NaN
//! - 010: Normal finite number
//! - 011: Infinity
//! - 100: Zero
//! - 101: Empty
//! - 110: Denormal number
//!
//! Flags affected:
//! - C0, C2, C3: Set according to value class (see table above)
//! - C1: Set to sign bit (0 for positive, 1 for negative)
//!
//! Reference: /Users/int/dev/rax/docs/fxam.txt

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

// Helper function to read u16 from memory
fn read_u16(mem: &vm_memory::GuestMemoryMmap, addr: u64) -> u16 {
    let mut buf = [0u8; 2];
    mem.read_slice(&mut buf, GuestAddress(addr)).unwrap();
    u16::from_le_bytes(buf)
}

// ============================================================================
// FXAM - Normal Finite Numbers (C3=0, C2=1, C0=0)
// ============================================================================

#[test]
fn test_fxam_positive_normal() {
    // FXAM on positive normal number
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xE5, // FXAM
        0x9B, 0xDF, 0xE0, // FSTSW AX
        0x66, 0x67, 0xA3, 0x00, 0x30, 0x00, 0x00, // MOV [0x3000], AX
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 3.14);

    run_until_hlt(&mut vcpu).unwrap();

    let status = read_u16(&mem, 0x3000);
    let c3 = (status >> 14) & 1;
    let c2 = (status >> 10) & 1;
    let c1 = (status >> 9) & 1;
    let c0 = (status >> 8) & 1;

    assert_eq!(c3, 0, "C3 should be 0 for normal");
    assert_eq!(c2, 1, "C2 should be 1 for normal");
    assert_eq!(c0, 0, "C0 should be 0 for normal");
    assert_eq!(c1, 0, "C1 should be 0 for positive");
}

#[test]
fn test_fxam_negative_normal() {
    // FXAM on negative normal number
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xE5, // FXAM
        0x9B, 0xDF, 0xE0, // FSTSW AX
        0x66, 0x67, 0xA3, 0x00, 0x30, 0x00, 0x00, // MOV [0x3000], AX
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, -2.71);

    run_until_hlt(&mut vcpu).unwrap();

    let status = read_u16(&mem, 0x3000);
    let c3 = (status >> 14) & 1;
    let c2 = (status >> 10) & 1;
    let c1 = (status >> 9) & 1;
    let c0 = (status >> 8) & 1;

    assert_eq!(c3, 0, "C3 should be 0 for normal");
    assert_eq!(c2, 1, "C2 should be 1 for normal");
    assert_eq!(c0, 0, "C0 should be 0 for normal");
    assert_eq!(c1, 1, "C1 should be 1 for negative");
}

#[test]
fn test_fxam_large_normal() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xE5, // FXAM
        0x9B, 0xDF, 0xE0, // FSTSW AX
        0x66, 0x67, 0xA3, 0x00, 0x30, 0x00, 0x00, // MOV [0x3000], AX
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 1.0e100);

    run_until_hlt(&mut vcpu).unwrap();

    let status = read_u16(&mem, 0x3000);
    let c3 = (status >> 14) & 1;
    let c2 = (status >> 10) & 1;
    let c0 = (status >> 8) & 1;

    assert_eq!(c3, 0, "C3 should be 0 for normal");
    assert_eq!(c2, 1, "C2 should be 1 for normal");
    assert_eq!(c0, 0, "C0 should be 0 for normal");
}

#[test]
fn test_fxam_small_normal() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xE5, // FXAM
        0x9B, 0xDF, 0xE0, // FSTSW AX
        0x66, 0x67, 0xA3, 0x00, 0x30, 0x00, 0x00, // MOV [0x3000], AX
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 1.0e-100);

    run_until_hlt(&mut vcpu).unwrap();

    let status = read_u16(&mem, 0x3000);
    let c3 = (status >> 14) & 1;
    let c2 = (status >> 10) & 1;
    let c0 = (status >> 8) & 1;

    assert_eq!(c3, 0, "C3 should be 0 for normal");
    assert_eq!(c2, 1, "C2 should be 1 for normal");
    assert_eq!(c0, 0, "C0 should be 0 for normal");
}

// ============================================================================
// FXAM - Zero (C3=1, C2=0, C0=0)
// ============================================================================

#[test]
fn test_fxam_positive_zero() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xE5, // FXAM
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

    assert_eq!(c3, 1, "C3 should be 1 for zero");
    assert_eq!(c2, 0, "C2 should be 0 for zero");
    assert_eq!(c0, 0, "C0 should be 0 for zero");
    assert_eq!(c1, 0, "C1 should be 0 for positive zero");
}

#[test]
fn test_fxam_negative_zero() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xE5, // FXAM
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
    let c1 = (status >> 9) & 1;
    let c0 = (status >> 8) & 1;

    assert_eq!(c3, 1, "C3 should be 1 for zero");
    assert_eq!(c2, 0, "C2 should be 0 for zero");
    assert_eq!(c0, 0, "C0 should be 0 for zero");
    assert_eq!(c1, 1, "C1 should be 1 for negative zero");
}

// ============================================================================
// FXAM - Infinity (C3=0, C2=1, C0=1)
// ============================================================================

#[test]
fn test_fxam_positive_infinity() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xE5, // FXAM
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
    let c1 = (status >> 9) & 1;
    let c0 = (status >> 8) & 1;

    assert_eq!(c3, 0, "C3 should be 0 for infinity");
    assert_eq!(c2, 1, "C2 should be 1 for infinity");
    assert_eq!(c0, 1, "C0 should be 1 for infinity");
    assert_eq!(c1, 0, "C1 should be 0 for positive infinity");
}

#[test]
fn test_fxam_negative_infinity() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xE5, // FXAM
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
    let c1 = (status >> 9) & 1;
    let c0 = (status >> 8) & 1;

    assert_eq!(c3, 0, "C3 should be 0 for infinity");
    assert_eq!(c2, 1, "C2 should be 1 for infinity");
    assert_eq!(c0, 1, "C0 should be 1 for infinity");
    assert_eq!(c1, 1, "C1 should be 1 for negative infinity");
}

// ============================================================================
// FXAM - NaN (C3=0, C2=0, C0=1)
// ============================================================================

#[test]
fn test_fxam_nan() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xE5, // FXAM
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

    assert_eq!(c3, 0, "C3 should be 0 for NaN");
    assert_eq!(c2, 0, "C2 should be 0 for NaN");
    assert_eq!(c0, 1, "C0 should be 1 for NaN");
}

#[test]
fn test_fxam_negative_nan() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xE5, // FXAM
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
    let c1 = (status >> 9) & 1;
    let c0 = (status >> 8) & 1;

    assert_eq!(c3, 0, "C3 should be 0 for NaN");
    assert_eq!(c2, 0, "C2 should be 0 for NaN");
    assert_eq!(c0, 1, "C0 should be 1 for NaN");
    assert_eq!(c1, 1, "C1 should be 1 for negative NaN");
}

// ============================================================================
// FXAM - Denormal Numbers (C3=1, C2=1, C0=0)
// ============================================================================

#[test]
fn test_fxam_positive_denormal() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xE5, // FXAM
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
    let c1 = (status >> 9) & 1;
    let c0 = (status >> 8) & 1;

    assert_eq!(c3, 1, "C3 should be 1 for denormal");
    assert_eq!(c2, 1, "C2 should be 1 for denormal");
    assert_eq!(c0, 0, "C0 should be 0 for denormal");
    assert_eq!(c1, 0, "C1 should be 0 for positive denormal");
}

#[test]
fn test_fxam_negative_denormal() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xE5, // FXAM
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
    let c1 = (status >> 9) & 1;
    let c0 = (status >> 8) & 1;

    assert_eq!(c3, 1, "C3 should be 1 for denormal");
    assert_eq!(c2, 1, "C2 should be 1 for denormal");
    assert_eq!(c0, 0, "C0 should be 0 for denormal");
    assert_eq!(c1, 1, "C1 should be 1 for negative denormal");
}

// ============================================================================
// FXAM - Mathematical Constants
// ============================================================================

#[test]
fn test_fxam_pi() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xE5, // FXAM
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

    assert_eq!(c3, 0, "C3 should be 0 for normal (PI)");
    assert_eq!(c2, 1, "C2 should be 1 for normal (PI)");
    assert_eq!(c0, 0, "C0 should be 0 for normal (PI)");
}

#[test]
fn test_fxam_e() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xE5, // FXAM
        0x9B, 0xDF, 0xE0, // FSTSW AX
        0x66, 0x67, 0xA3, 0x00, 0x30, 0x00, 0x00, // MOV [0x3000], AX
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, std::f64::consts::E);

    run_until_hlt(&mut vcpu).unwrap();

    let status = read_u16(&mem, 0x3000);
    let c3 = (status >> 14) & 1;
    let c2 = (status >> 10) & 1;
    let c0 = (status >> 8) & 1;

    assert_eq!(c3, 0, "C3 should be 0 for normal (E)");
    assert_eq!(c2, 1, "C2 should be 1 for normal (E)");
    assert_eq!(c0, 0, "C0 should be 0 for normal (E)");
}

// ============================================================================
// FXAM - Special Finite Values
// ============================================================================

#[test]
fn test_fxam_one() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xE5, // FXAM
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

    assert_eq!(c3, 0, "C3 should be 0 for normal (1.0)");
    assert_eq!(c2, 1, "C2 should be 1 for normal (1.0)");
    assert_eq!(c0, 0, "C0 should be 0 for normal (1.0)");
}

#[test]
fn test_fxam_negative_one() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xE5, // FXAM
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
    let c1 = (status >> 9) & 1;
    let c0 = (status >> 8) & 1;

    assert_eq!(c3, 0, "C3 should be 0 for normal (-1.0)");
    assert_eq!(c2, 1, "C2 should be 1 for normal (-1.0)");
    assert_eq!(c0, 0, "C0 should be 0 for normal (-1.0)");
    assert_eq!(c1, 1, "C1 should be 1 for negative");
}

#[test]
fn test_fxam_max_value() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xE5, // FXAM
        0x9B, 0xDF, 0xE0, // FSTSW AX
        0x66, 0x67, 0xA3, 0x00, 0x30, 0x00, 0x00, // MOV [0x3000], AX
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, f64::MAX);

    run_until_hlt(&mut vcpu).unwrap();

    let status = read_u16(&mem, 0x3000);
    let c3 = (status >> 14) & 1;
    let c2 = (status >> 10) & 1;
    let c0 = (status >> 8) & 1;

    assert_eq!(c3, 0, "C3 should be 0 for normal (MAX)");
    assert_eq!(c2, 1, "C2 should be 1 for normal (MAX)");
    assert_eq!(c0, 0, "C0 should be 0 for normal (MAX)");
}

#[test]
fn test_fxam_min_positive() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xE5, // FXAM
        0x9B, 0xDF, 0xE0, // FSTSW AX
        0x66, 0x67, 0xA3, 0x00, 0x30, 0x00, 0x00, // MOV [0x3000], AX
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, f64::MIN_POSITIVE);

    run_until_hlt(&mut vcpu).unwrap();

    let status = read_u16(&mem, 0x3000);
    let c3 = (status >> 14) & 1;
    let c2 = (status >> 10) & 1;
    let c0 = (status >> 8) & 1;

    assert_eq!(c3, 0, "C3 should be 0 for normal (MIN_POSITIVE)");
    assert_eq!(c2, 1, "C2 should be 1 for normal (MIN_POSITIVE)");
    assert_eq!(c0, 0, "C0 should be 0 for normal (MIN_POSITIVE)");
}

// ============================================================================
// FXAM - Sign Detection
// ============================================================================

#[test]
fn test_fxam_sign_positive_values() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xE5, // FXAM
        0x9B, 0xDF, 0xE0, // FSTSW AX
        0x66, 0x67, 0xA3, 0x00, 0x30, 0x00, 0x00, // MOV [0x3000], AX
        0xF4, // HLT
    ];

    let test_values = vec![1.0, 100.0, 0.5, f64::MAX, f64::INFINITY];

    for val in test_values {
        let (mut vcpu, mem) = setup_vm(&code, None);
        write_f64(&mem, 0x2000, val);

        run_until_hlt(&mut vcpu).unwrap();

        let status = read_u16(&mem, 0x3000);
        let c1 = (status >> 9) & 1;

        assert_eq!(c1, 0, "C1 should be 0 for positive value {}", val);
    }
}

#[test]
fn test_fxam_sign_negative_values() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xE5, // FXAM
        0x9B, 0xDF, 0xE0, // FSTSW AX
        0x66, 0x67, 0xA3, 0x00, 0x30, 0x00, 0x00, // MOV [0x3000], AX
        0xF4, // HLT
    ];

    let test_values = vec![-1.0, -100.0, -0.5, -f64::MAX, f64::NEG_INFINITY];

    for val in test_values {
        let (mut vcpu, mem) = setup_vm(&code, None);
        write_f64(&mem, 0x2000, val);

        run_until_hlt(&mut vcpu).unwrap();

        let status = read_u16(&mem, 0x3000);
        let c1 = (status >> 9) & 1;

        assert_eq!(c1, 1, "C1 should be 1 for negative value {}", val);
    }
}

// ============================================================================
// FXAM - Multiple Examinations
// ============================================================================

#[test]
fn test_fxam_sequence() {
    // Test FXAM on multiple values
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xE5, // FXAM
        0x9B, 0xDF, 0xE0, // FSTSW AX
        0x66, 0x67, 0xA3, 0x00, 0x30, 0x00, 0x00, // MOV [0x3000], AX
        0xDD, 0xD8, // FSTP ST(0) ; pop
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008]
        0xD9, 0xE5, // FXAM
        0x9B, 0xDF, 0xE0, // FSTSW AX
        0x66, 0x67, 0xA3, 0x08, 0x30, 0x00, 0x00, // MOV [0x3008], AX
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 5.0);
    write_f64(&mem, 0x2008, 0.0);

    run_until_hlt(&mut vcpu).unwrap();

    let status1 = read_u16(&mem, 0x3000);
    let status2 = read_u16(&mem, 0x3008);

    // First: normal
    let c3_1 = (status1 >> 14) & 1;
    let c2_1 = (status1 >> 10) & 1;
    let c0_1 = (status1 >> 8) & 1;
    assert_eq!(c3_1, 0, "First should be normal");
    assert_eq!(c2_1, 1, "First should be normal");
    assert_eq!(c0_1, 0, "First should be normal");

    // Second: zero
    let c3_2 = (status2 >> 14) & 1;
    let c2_2 = (status2 >> 10) & 1;
    let c0_2 = (status2 >> 8) & 1;
    assert_eq!(c3_2, 1, "Second should be zero");
    assert_eq!(c2_2, 0, "Second should be zero");
    assert_eq!(c0_2, 0, "Second should be zero");
}

#[test]
fn test_fxam_does_not_modify_value() {
    // FXAM should not modify ST(0)
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xE5, // FXAM
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 3.14159);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert_eq!(result, 3.14159, "FXAM should not modify ST(0)");
}

// ============================================================================
// FXAM - Edge Cases
// ============================================================================

#[test]
fn test_fxam_very_small_denormal() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xE5, // FXAM
        0x9B, 0xDF, 0xE0, // FSTSW AX
        0x66, 0x67, 0xA3, 0x00, 0x30, 0x00, 0x00, // MOV [0x3000], AX
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    let very_small_denormal = f64::MIN_POSITIVE / 1000.0;
    write_f64(&mem, 0x2000, very_small_denormal);

    run_until_hlt(&mut vcpu).unwrap();

    let status = read_u16(&mem, 0x3000);
    let c3 = (status >> 14) & 1;
    let c2 = (status >> 10) & 1;
    let c0 = (status >> 8) & 1;

    assert_eq!(c3, 1, "C3 should be 1 for denormal");
    assert_eq!(c2, 1, "C2 should be 1 for denormal");
    assert_eq!(c0, 0, "C0 should be 0 for denormal");
}

#[test]
fn test_fxam_fractional_values() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xE5, // FXAM
        0x9B, 0xDF, 0xE0, // FSTSW AX
        0x66, 0x67, 0xA3, 0x00, 0x30, 0x00, 0x00, // MOV [0x3000], AX
        0xF4, // HLT
    ];

    let test_values = vec![0.5, 0.25, 0.125, 0.1, 0.01];

    for val in test_values {
        let (mut vcpu, mem) = setup_vm(&code, None);
        write_f64(&mem, 0x2000, val);

        run_until_hlt(&mut vcpu).unwrap();

        let status = read_u16(&mem, 0x3000);
        let c3 = (status >> 14) & 1;
        let c2 = (status >> 10) & 1;
        let c0 = (status >> 8) & 1;

        assert_eq!(c3, 0, "C3 should be 0 for normal ({})", val);
        assert_eq!(c2, 1, "C2 should be 1 for normal ({})", val);
        assert_eq!(c0, 0, "C0 should be 0 for normal ({})", val);
    }
}

#[test]
fn test_fxam_power_of_two() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xE5, // FXAM
        0x9B, 0xDF, 0xE0, // FSTSW AX
        0x66, 0x67, 0xA3, 0x00, 0x30, 0x00, 0x00, // MOV [0x3000], AX
        0xF4, // HLT
    ];

    let test_values = vec![2.0, 4.0, 8.0, 16.0, 1024.0];

    for val in test_values {
        let (mut vcpu, mem) = setup_vm(&code, None);
        write_f64(&mem, 0x2000, val);

        run_until_hlt(&mut vcpu).unwrap();

        let status = read_u16(&mem, 0x3000);
        let c3 = (status >> 14) & 1;
        let c2 = (status >> 10) & 1;
        let c0 = (status >> 8) & 1;

        assert_eq!(c3, 0, "C3 should be 0 for normal ({})", val);
        assert_eq!(c2, 1, "C2 should be 1 for normal ({})", val);
        assert_eq!(c0, 0, "C0 should be 0 for normal ({})", val);
    }
}
