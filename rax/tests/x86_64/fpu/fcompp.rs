//! Tests for the FCOMPP instruction.
//!
//! FCOMPP - Compare floating-point and pop twice
//!
//! Reference: /Users/int/dev/rax/docs/fcom:fcomp:fcompp.txt
//!
//! Opcode: DE D9 - FCOMPP ; Compare ST(0) with ST(1) and pop twice

use crate::common::*;
use std::sync::Arc;
use vm_memory::{Bytes, GuestAddress, GuestMemoryMmap};

const DATA_ADDR: u64 = 0x2000;

fn write_f64(mem: &Arc<GuestMemoryMmap>, addr: u64, value: f64) {
    mem.write_slice(&value.to_le_bytes(), GuestAddress(addr))
        .unwrap();
}

fn read_u16(mem: &Arc<GuestMemoryMmap>, addr: u64) -> u16 {
    let mut buf = [0u8; 2];
    mem.read_slice(&mut buf, GuestAddress(addr)).unwrap();
    u16::from_le_bytes(buf)
}

#[test]
fn test_fcompp_equal() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008]
        0xDE, 0xD9, // FCOMPP
        0xDF, 0xE0, // FSTSW AX
        0x66, 0x89, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // MOV [0x3000], AX
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 10.0);
    write_f64(&mem, DATA_ADDR + 8, 10.0);

    run_until_hlt(&mut vcpu).unwrap();
    let status = read_u16(&mem, 0x3000);
    // C3=1, C2=0, C0=0 for equal (bits 14, 10, 8)
    assert_eq!(status & 0x4500, 0x4000);
}

#[test]
fn test_fcompp_greater() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDE,
        0xD9, // FCOMPP
        0xDF, 0xE0, // FSTSW AX
        0x66, 0x89, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 5.0);
    write_f64(&mem, DATA_ADDR + 8, 15.0);

    run_until_hlt(&mut vcpu).unwrap();
    let status = read_u16(&mem, 0x3000);
    // ST(0) > ST(1): 15 > 5, C3=0, C2=0, C0=0
    assert_eq!(status & 0x4500, 0x0000);
}

#[test]
fn test_fcompp_less() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDE,
        0xD9, // FCOMPP
        0xDF, 0xE0, // FSTSW AX
        0x66, 0x89, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 15.0);
    write_f64(&mem, DATA_ADDR + 8, 5.0);

    run_until_hlt(&mut vcpu).unwrap();
    let status = read_u16(&mem, 0x3000);
    // ST(0) < ST(1): 5 < 15, C3=0, C2=0, C0=1
    assert_eq!(status & 0x4500, 0x0100);
}

#[test]
fn test_fcompp_negative_numbers() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDE,
        0xD9, 0xDF, 0xE0, 0x66, 0x89, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, -10.0);
    write_f64(&mem, DATA_ADDR + 8, -5.0);

    run_until_hlt(&mut vcpu).unwrap();
    let status = read_u16(&mem, 0x3000);
    // -5 > -10, so ST(0) > ST(1)
    assert_eq!(status & 0x4500, 0x0000);
}

#[test]
fn test_fcompp_zero_comparison() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDE,
        0xD9, 0xDF, 0xE0, 0x66, 0x89, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 0.0);
    write_f64(&mem, DATA_ADDR + 8, 0.0);

    run_until_hlt(&mut vcpu).unwrap();
    let status = read_u16(&mem, 0x3000);
    // Equal
    assert_eq!(status & 0x4500, 0x4000);
}

#[test]
fn test_fcompp_infinity() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDE,
        0xD9, 0xDF, 0xE0, 0x66, 0x89, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 100.0);
    write_f64(&mem, DATA_ADDR + 8, f64::INFINITY);

    run_until_hlt(&mut vcpu).unwrap();
    let status = read_u16(&mem, 0x3000);
    // ST(0) > ST(1): infinity > 100
    assert_eq!(status & 0x4500, 0x0000);
}
