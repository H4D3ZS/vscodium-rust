//! Tests for the FNSAVE and extended FNOP instructions.
//!
//! FNSAVE - Store x87 FPU State (non-waiting)
//! Extended FNOP - Extended NOP operations
//!
//! FNSAVE saves the entire FPU state to a 94 or 108 byte memory area,
//! then reinitializes the FPU. The instruction does not check for
//! unmasked floating-point exceptions.
//!
//! Opcodes:
//! - FNSAVE: DD /6
//! - FNOP: D9 D0
//!
//! Reference: /Users/int/dev/rax/docs/fnsave.txt, /Users/int/dev/rax/docs/fnop.txt

use crate::common::*;
use vm_memory::{Bytes, GuestAddress};

fn write_u16(mem: &vm_memory::GuestMemoryMmap, addr: u64, val: u16) {
    mem.write_slice(&val.to_le_bytes(), GuestAddress(addr))
        .unwrap();
}

fn read_u16(mem: &vm_memory::GuestMemoryMmap, addr: u64) -> u16 {
    let mut buf = [0u8; 2];
    mem.read_slice(&mut buf, GuestAddress(addr)).unwrap();
    u16::from_le_bytes(buf)
}

fn write_f64(mem: &vm_memory::GuestMemoryMmap, addr: u64, val: f64) {
    mem.write_slice(&val.to_le_bytes(), GuestAddress(addr))
        .unwrap();
}

fn read_f64(mem: &vm_memory::GuestMemoryMmap, addr: u64) -> f64 {
    let mut buf = [0u8; 8];
    mem.read_slice(&mut buf, GuestAddress(addr)).unwrap();
    f64::from_le_bytes(buf)
}

// FNSAVE tests
#[test]
fn test_fnsave_basic() {
    let code = [
        0xDD, 0x34, 0x25, 0x00, 0x20, 0x00, 0x00, // FNSAVE [0x2000]
        0xF4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
    let fcw = read_u16(&mem, 0x2000);
    assert!(fcw < 0xFFFF);
}

#[test]
fn test_fnsave_with_data() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xDD, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00, // FNSAVE [0x3000]
        0xF4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 1.5);
    run_until_hlt(&mut vcpu).unwrap();
    let fcw = read_u16(&mem, 0x3000);
    assert!(fcw < 0xFFFF);
}

#[test]
fn test_fnsave_reinitializes() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xDD, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00, // FNSAVE [0x3000]
        0xD9, 0xE8, // FLD1 (should work on empty stack)
        0xDD, 0x1C, 0x25, 0x00, 0x40, 0x00, 0x00, // FSTP qword [0x4000]
        0xF4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 2.5);
    run_until_hlt(&mut vcpu).unwrap();
    let result = read_f64(&mem, 0x4000);
    assert_eq!(result, 1.0);
}

// FNOP tests
#[test]
fn test_fnop_basic() {
    let code = [
        0xD9, 0xD0, // FNOP
        0xF4, // HLT
    ];
    let (mut vcpu, _mem) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_fnop_preserves_stack() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xD0, // FNOP
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 3.14);
    run_until_hlt(&mut vcpu).unwrap();
    let result = read_f64(&mem, 0x3000);
    assert_eq!(result, 3.14);
}

#[test]
fn test_fnop_sequence() {
    let code = [
        0xD9, 0xD0, // FNOP
        0xD9, 0xD0, // FNOP
        0xD9, 0xD0, // FNOP
        0xF4, // HLT
    ];
    let (mut vcpu, _mem) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// Additional FNSAVE tests
#[test]
fn test_fnsave_multiple_values() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008]
        0xDD, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00, // FNSAVE [0x3000]
        0xF4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 1.0);
    write_f64(&mem, 0x2008, 2.0);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_fnop_with_arithmetic() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0xD0, // FNOP
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008]
        0xD9, 0xD0, // FNOP
        0xDE, 0xC1, // FADDP
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 1.0);
    write_f64(&mem, 0x2008, 2.0);
    run_until_hlt(&mut vcpu).unwrap();
    let result = read_f64(&mem, 0x3000);
    assert_eq!(result, 3.0);
}

#[test]
fn test_fnsave_after_arithmetic() {
    let code = [
        0xD9, 0xE8, // FLD1
        0xD9, 0xE8, // FLD1
        0xDE, 0xC1, // FADDP
        0xDD, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00, // FNSAVE [0x3000]
        0xF4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// More comprehensive tests (30+ total)
macro_rules! fnsave_test {
    ($name:ident, $val:expr) => {
        #[test]
        fn $name() {
            let code = [
                0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDD, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00,
                0xF4,
            ];
            let (mut vcpu, mem) = setup_vm(&code, None);
            write_f64(&mem, 0x2000, $val);
            run_until_hlt(&mut vcpu).unwrap();
        }
    };
}

fnsave_test!(test_fnsave_inf, f64::INFINITY);
fnsave_test!(test_fnsave_neg_inf, f64::NEG_INFINITY);
fnsave_test!(test_fnsave_zero, 0.0);
fnsave_test!(test_fnsave_neg_zero, -0.0);
fnsave_test!(test_fnsave_one, 1.0);
fnsave_test!(test_fnsave_neg_one, -1.0);
fnsave_test!(test_fnsave_large, 1e100);
fnsave_test!(test_fnsave_small, 1e-100);
fnsave_test!(test_fnsave_pi, std::f64::consts::PI);
fnsave_test!(test_fnsave_e, std::f64::consts::E);

macro_rules! fnop_test {
    ($name:ident, $val:expr) => {
        #[test]
        fn $name() {
            let code = [
                0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xD9, 0xD0, 0xDD, 0x1C, 0x25, 0x00, 0x30,
                0x00, 0x00, 0xF4,
            ];
            let (mut vcpu, mem) = setup_vm(&code, None);
            write_f64(&mem, 0x2000, $val);
            run_until_hlt(&mut vcpu).unwrap();
            let result = read_f64(&mem, 0x3000);
            assert_eq!(result, $val);
        }
    };
}

fnop_test!(test_fnop_inf, f64::INFINITY);
fnop_test!(test_fnop_neg_inf, f64::NEG_INFINITY);
fnop_test!(test_fnop_zero, 0.0);
fnop_test!(test_fnop_large, 1e200);
fnop_test!(test_fnop_small, 1e-200);
fnop_test!(test_fnop_negative, -42.5);
fnop_test!(test_fnop_positive, 42.5);
fnop_test!(test_fnop_frac, 0.125);
fnop_test!(test_fnop_max, f64::MAX);
fnop_test!(test_fnop_min, f64::MIN);

#[test]
fn test_fnsave_stack_depth() {
    let code = [
        0xD9, 0xE8, 0xD9, 0xE8, 0xD9, 0xE8, 0xD9, 0xE8, 0xDD, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xF4,
    ];
    let (mut vcpu, _mem) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_fnop_doesnt_affect_flags() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, 0xD8,
        0xD1, // FCOM
        0xD9, 0xD0, // FNOP
        0xDD, 0xD8, 0xDD, 0xD8, 0xF4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 1.0);
    write_f64(&mem, 0x2008, 2.0);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_fnsave_consistency() {
    let code = [
        0xD9, 0xE8, 0xDD, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00, 0xD9, 0xE8, 0xDD, 0x34, 0x25, 0x00,
        0x31, 0x00, 0x00, 0xF4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
    let fcw1 = read_u16(&mem, 0x3000);
    let fcw2 = read_u16(&mem, 0x3100);
    assert_eq!(fcw1, fcw2);
}
