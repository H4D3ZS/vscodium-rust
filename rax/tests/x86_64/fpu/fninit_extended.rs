//! Extended tests for the FNINIT instruction in various states.
//!
//! FNINIT - Initialize Floating-Point Unit (non-waiting)
//!
//! This file contains comprehensive tests for FNINIT covering:
//! - Initialization from various FPU states
//! - Stack handling
//! - Control word initialization
//! - Status word clearing
//! - Tag word reset
//!
//! FNINIT sets the FPU to its default state:
//! - Control Word: 0x037F
//! - Status Word: 0x0000
//! - Tag Word: 0xFFFF (all empty)
//! - Clears exception flags
//!
//! Opcode: DB E3
//!
//! Reference: /Users/int/dev/rax/docs/finit:fninit.txt

use crate::common::*;
use vm_memory::{Bytes, GuestAddress};

fn write_f64(mem: &vm_memory::GuestMemoryMmap, addr: u64, val: f64) {
    mem.write_slice(&val.to_le_bytes(), GuestAddress(addr))
        .unwrap();
}

fn read_f64(mem: &vm_memory::GuestMemoryMmap, addr: u64) -> f64 {
    let mut buf = [0u8; 8];
    mem.read_slice(&mut buf, GuestAddress(addr)).unwrap();
    f64::from_le_bytes(buf)
}

fn write_u16(mem: &vm_memory::GuestMemoryMmap, addr: u64, val: u16) {
    mem.write_slice(&val.to_le_bytes(), GuestAddress(addr))
        .unwrap();
}

fn read_u16(mem: &vm_memory::GuestMemoryMmap, addr: u64) -> u16 {
    let mut buf = [0u8; 2];
    mem.read_slice(&mut buf, GuestAddress(addr)).unwrap();
    u16::from_le_bytes(buf)
}

#[test]
fn test_fninit_basic() {
    let code = [
        0xDB, 0xE3, // FNINIT
        0xF4, // HLT
    ];
    let (mut vcpu, _mem) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_fninit_clears_stack() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xDB, 0xE3, // FNINIT
        0xD9, 0xE8, // FLD1 (should work on empty stack)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 99.99);
    run_until_hlt(&mut vcpu).unwrap();
    let result = read_f64(&mem, 0x3000);
    assert_eq!(result, 1.0);
}

#[test]
fn test_fninit_resets_control_word() {
    let code = [
        0xD9, 0x2C, 0x25, 0x00, 0x20, 0x00, 0x00, // FLDCW [0x2000]
        0xDB, 0xE3, // FNINIT
        0xD9, 0x3C, 0x25, 0x00, 0x30, 0x00, 0x00, // FNSTCW [0x3000]
        0xF4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_u16(&mem, 0x2000, 0x0C7F);
    run_until_hlt(&mut vcpu).unwrap();
    let cw = read_u16(&mem, 0x3000);
    assert_eq!(cw, 0x037F, "Control word should be reset to default");
}

#[test]
fn test_fninit_with_full_stack() {
    let code = [
        0xD9, 0xE8, // FLD1 x8
        0xD9, 0xE8, 0xD9, 0xE8, 0xD9, 0xE8, 0xD9, 0xE8, 0xD9, 0xE8, 0xD9, 0xE8, 0xD9, 0xE8, 0xDB,
        0xE3, // FNINIT
        0xD9, 0xE8, // FLD1
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
    let result = read_f64(&mem, 0x3000);
    assert_eq!(result, 1.0);
}

#[test]
fn test_fninit_multiple_times() {
    let code = [
        0xDB, 0xE3, // FNINIT
        0xD9, 0xE8, // FLD1
        0xDB, 0xE3, // FNINIT
        0xD9, 0xE8, // FLD1
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xF4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
    let result = read_f64(&mem, 0x3000);
    assert_eq!(result, 1.0);
}

#[test]
fn test_fninit_after_arithmetic() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDE,
        0xC1, // FADDP
        0xDB, 0xE3, // FNINIT
        0xD9, 0xEE, // FLDZ
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xF4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 10.0);
    write_f64(&mem, 0x2008, 20.0);
    run_until_hlt(&mut vcpu).unwrap();
    let result = read_f64(&mem, 0x3000);
    assert_eq!(result, 0.0);
}

#[test]
fn test_fninit_clears_status() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, 0xD8,
        0xD1, // FCOM
        0xDB, 0xE3, // FNINIT
        0xDF, 0xE0, // FNSTSW AX
        0x89, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // MOV [0x3000], EAX
        0xF4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 1.0);
    write_f64(&mem, 0x2008, 2.0);
    run_until_hlt(&mut vcpu).unwrap();
    let status = read_u16(&mem, 0x3000);
    assert_eq!(status & 0x3FFF, 0, "Status should be cleared");
}

// Tests with different pre-init states
#[test]
fn test_fninit_after_division_by_zero() {
    let code = [
        0xD9, 0xE8, // FLD1
        0xD9, 0xEE, // FLDZ
        0xDE, 0xF9, // FDIVP (may set exception)
        0xDB, 0xE3, // FNINIT
        0xD9, 0xE8, // FLD1
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xF4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
    let result = read_f64(&mem, 0x3000);
    assert_eq!(result, 1.0);
}

#[test]
fn test_fninit_after_sqrt_negative() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xD9, 0xFA, // FSQRT (may produce NaN)
        0xDB, 0xE3, // FNINIT
        0xD9, 0xE8, // FLD1
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xF4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, -1.0);
    run_until_hlt(&mut vcpu).unwrap();
    let result = read_f64(&mem, 0x3000);
    assert_eq!(result, 1.0);
}

#[test]
fn test_fninit_sequence_operations() {
    let code = [
        0xD9, 0xE8, // FLD1
        0xDB, 0xE3, // FNINIT
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD
        0xDB, 0xE3, // FNINIT
        0xD9, 0xEE, // FLDZ
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xF4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 42.0);
    run_until_hlt(&mut vcpu).unwrap();
    let result = read_f64(&mem, 0x3000);
    assert_eq!(result, 0.0);
}

// Comprehensive state reset tests
macro_rules! fninit_state_test {
    ($name:ident, $setup:expr) => {
        #[test]
        fn $name() {
            let mut code = Vec::from($setup);
            code.extend_from_slice(&[
                0xDB, 0xE3, // FNINIT
                0xD9, 0xE8, // FLD1
                0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP [0x3000]
                0xF4, // HLT
            ]);
            let (mut vcpu, mem) = setup_vm(&code, None);
            write_f64(&mem, 0x2000, 100.0);
            write_f64(&mem, 0x2008, 200.0);
            run_until_hlt(&mut vcpu).unwrap();
            let result = read_f64(&mem, 0x3000);
            assert_eq!(result, 1.0, "Stack should be cleared");
        }
    };
}

fninit_state_test!(
    test_fninit_after_fld,
    &[
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD
    ]
);

fninit_state_test!(
    test_fninit_after_fadd,
    &[
        0xD9, 0xE8, // FLD1
        0xD9, 0xE8, // FLD1
        0xDE, 0xC1, // FADDP
    ]
);

fninit_state_test!(
    test_fninit_after_fmul,
    &[
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDE,
        0xC9, // FMULP
    ]
);

fninit_state_test!(
    test_fninit_after_fsub,
    &[
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDE,
        0xE9, // FSUBP
    ]
);

fninit_state_test!(
    test_fninit_after_fdiv,
    &[
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDE,
        0xF9, // FDIVP
    ]
);

fninit_state_test!(
    test_fninit_after_fsqrt,
    &[
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xD9, 0xFA, // FSQRT
    ]
);

fninit_state_test!(
    test_fninit_after_fabs,
    &[
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xD9, 0xE1, // FABS
    ]
);

fninit_state_test!(
    test_fninit_after_fchs,
    &[
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xD9, 0xE0, // FCHS
    ]
);

fninit_state_test!(
    test_fninit_after_fcom,
    &[
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, 0xD8,
        0xD1, // FCOM
    ]
);

fninit_state_test!(
    test_fninit_after_fxch,
    &[
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, 0xD9,
        0xC9, // FXCH
    ]
);

// Tests with constants
#[test]
fn test_fninit_after_fldpi() {
    let code = [
        0xD9, 0xEB, // FLDPI
        0xDB, 0xE3, // FNINIT
        0xD9, 0xE8, // FLD1
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xF4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
    let result = read_f64(&mem, 0x3000);
    assert_eq!(result, 1.0);
}

#[test]
fn test_fninit_after_fldz() {
    let code = [
        0xD9, 0xEE, // FLDZ
        0xDB, 0xE3, // FNINIT
        0xD9, 0xE8, // FLD1
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xF4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
    let result = read_f64(&mem, 0x3000);
    assert_eq!(result, 1.0);
}

#[test]
fn test_fninit_after_fld1() {
    let code = [
        0xD9, 0xE8, // FLD1
        0xD9, 0xE8, // FLD1
        0xD9, 0xE8, // FLD1
        0xDB, 0xE3, // FNINIT
        0xD9, 0xEE, // FLDZ
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xF4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
    let result = read_f64(&mem, 0x3000);
    assert_eq!(result, 0.0);
}

// Edge cases
#[test]
fn test_fninit_with_infinity() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDB, 0xE3, 0xD9, 0xE8, 0xDD, 0x1C, 0x25, 0x00,
        0x30, 0x00, 0x00, 0xF4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, f64::INFINITY);
    run_until_hlt(&mut vcpu).unwrap();
    let result = read_f64(&mem, 0x3000);
    assert_eq!(result, 1.0);
}

#[test]
fn test_fninit_with_nan() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDB, 0xE3, 0xD9, 0xE8, 0xDD, 0x1C, 0x25, 0x00,
        0x30, 0x00, 0x00, 0xF4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, f64::NAN);
    run_until_hlt(&mut vcpu).unwrap();
    let result = read_f64(&mem, 0x3000);
    assert_eq!(result, 1.0);
}

// Consistency tests
#[test]
fn test_fninit_idempotent() {
    let code = [
        0xDB, 0xE3, // FNINIT
        0xD9, 0x3C, 0x25, 0x00, 0x30, 0x00, 0x00, // FNSTCW [0x3000]
        0xDB, 0xE3, // FNINIT again
        0xD9, 0x3C, 0x25, 0x08, 0x30, 0x00, 0x00, // FNSTCW [0x3008]
        0xF4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
    let cw1 = read_u16(&mem, 0x3000);
    let cw2 = read_u16(&mem, 0x3008);
    assert_eq!(cw1, cw2, "FNINIT should be idempotent");
}

#[test]
fn test_fninit_after_complex_operations() {
    let code = [
        0xD9, 0xE8, // FLD1
        0xD9, 0xEB, // FLDPI
        0xDE, 0xC1, // FADDP
        0xD9, 0xE8, // FLD1
        0xDE, 0xC9, // FMULP
        0xDB, 0xE3, // FNINIT
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP
        0xF4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 7.5);
    run_until_hlt(&mut vcpu).unwrap();
    let result = read_f64(&mem, 0x3000);
    assert_eq!(result, 7.5);
}

// Stress tests
#[test]
fn test_fninit_rapid_succession() {
    let code = [
        0xDB, 0xE3, 0xDB, 0xE3, 0xDB, 0xE3, 0xDB, 0xE3, 0xDB, 0xE3, 0xD9, 0xE8, 0xDD, 0x1C, 0x25,
        0x00, 0x30, 0x00, 0x00, 0xF4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
    let result = read_f64(&mem, 0x3000);
    assert_eq!(result, 1.0);
}

#[test]
fn test_fninit_alternating_operations() {
    let code = [
        0xD9, 0xE8, // FLD1
        0xDB, 0xE3, // FNINIT
        0xD9, 0xEE, // FLDZ
        0xDB, 0xE3, // FNINIT
        0xD9, 0xEB, // FLDPI
        0xDB, 0xE3, // FNINIT
        0xD9, 0xE8, // FLD1
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xF4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
    let result = read_f64(&mem, 0x3000);
    assert_eq!(result, 1.0);
}
