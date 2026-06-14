//! Tests for the FIDIVR instruction.
//!
//! FIDIVR - Reverse divide integer by floating-point (m16int and m32int)
//!
//! Reference: /Users/int/dev/rax/docs/fdivr:fdivrp:fidivr.txt
//!
//! Opcode: DE /7 - FIDIVR m16int  ; ST(0) = m16int / ST(0)
//! Opcode: DA /7 - FIDIVR m32int  ; ST(0) = m32int / ST(0)

use crate::common::*;
use std::sync::Arc;
use vm_memory::{Bytes, GuestAddress, GuestMemoryMmap};

const DATA_ADDR: u64 = 0x2000;

fn write_f64(mem: &Arc<GuestMemoryMmap>, addr: u64, value: f64) {
    mem.write_slice(&value.to_le_bytes(), GuestAddress(addr))
        .unwrap();
}

fn write_i16(mem: &Arc<GuestMemoryMmap>, addr: u64, value: i16) {
    mem.write_slice(&value.to_le_bytes(), GuestAddress(addr))
        .unwrap();
}

fn write_i32(mem: &Arc<GuestMemoryMmap>, addr: u64, value: i32) {
    mem.write_slice(&value.to_le_bytes(), GuestAddress(addr))
        .unwrap();
}

fn read_f64(mem: &Arc<GuestMemoryMmap>, addr: u64) -> f64 {
    let mut buf = [0u8; 8];
    mem.read_slice(&mut buf, GuestAddress(addr)).unwrap();
    f64::from_le_bytes(buf)
}

#[test]
fn test_fidivr_m16int_basic() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xDE, 0x3C, 0x25, 0x08, 0x20, 0x00, 0x00, // FIDIVR word [0x2008]
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 2.0);
    write_i16(&mem, DATA_ADDR + 8, 10);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 5.0); // 10 / 2
}

#[test]
fn test_fidivr_m16int_fractional() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDE, 0x3C, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD,
        0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 3.0);
    write_i16(&mem, DATA_ADDR + 8, 1);

    run_until_hlt(&mut vcpu).unwrap();
    let result = read_f64(&mem, 0x3000);
    assert!((result - (1.0 / 3.0)).abs() < 1e-15);
}

#[test]
fn test_fidivr_m16int_negative() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDE, 0x3C, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD,
        0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, -5.0);
    write_i16(&mem, DATA_ADDR + 8, 10);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), -2.0); // 10 / -5
}

#[test]
fn test_fidivr_m32int_basic() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDA, 0x3C, 0x25, 0x08, 0x20, 0x00,
        0x00, // FIDIVR dword [0x2008]
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 10.0);
    write_i32(&mem, DATA_ADDR + 8, 1000);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 100.0); // 1000 / 10
}

#[test]
fn test_fidivr_m32int_zero_dividend() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDA, 0x3C, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDD,
        0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 5.0);
    write_i32(&mem, DATA_ADDR + 8, 0);

    run_until_hlt(&mut vcpu).unwrap();
    let result = read_f64(&mem, 0x3000);
    assert_eq!(result, 0.0);
    assert!(!result.is_sign_negative());
}
