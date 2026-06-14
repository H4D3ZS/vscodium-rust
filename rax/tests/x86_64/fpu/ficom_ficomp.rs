//! Tests for the FICOM and FICOMP instructions.
//!
//! FICOM - Compare integer with floating-point (m16int and m32int)
//! FICOMP - Compare integer and pop
//!
//! Reference: /Users/int/dev/rax/docs/ficom:ficomp.txt
//!
//! Opcode: DE /2 - FICOM m16int
//! Opcode: DA /2 - FICOM m32int
//! Opcode: DE /3 - FICOMP m16int
//! Opcode: DA /3 - FICOMP m32int

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

fn read_u16(mem: &Arc<GuestMemoryMmap>, addr: u64) -> u16 {
    let mut buf = [0u8; 2];
    mem.read_slice(&mut buf, GuestAddress(addr)).unwrap();
    u16::from_le_bytes(buf)
}

#[test]
fn test_ficom_m16int_equal() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xDE, 0x14, 0x25, 0x08, 0x20, 0x00, 0x00, // FICOM word [0x2008]
        0xDF, 0xE0, // FSTSW AX
        0x66, 0x89, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // MOV [0x3000], AX
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 10.0);
    write_i16(&mem, DATA_ADDR + 8, 10);

    run_until_hlt(&mut vcpu).unwrap();
    let status = read_u16(&mem, 0x3000);
    // C3=1, C2=0, C0=0 for equal (bits 14, 10, 8)
    assert_eq!(status & 0x4500, 0x4000);
}

#[test]
fn test_ficom_m16int_greater() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDE, 0x14, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDF,
        0xE0, // FSTSW AX
        0x66, 0x89, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 15.0);
    write_i16(&mem, DATA_ADDR + 8, 10);

    run_until_hlt(&mut vcpu).unwrap();
    let status = read_u16(&mem, 0x3000);
    // C3=0, C2=0, C0=0 for greater
    assert_eq!(status & 0x4500, 0x0000);
}

#[test]
fn test_ficom_m16int_less() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDE, 0x14, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDF,
        0xE0, // FSTSW AX
        0x66, 0x89, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 5.0);
    write_i16(&mem, DATA_ADDR + 8, 10);

    run_until_hlt(&mut vcpu).unwrap();
    let status = read_u16(&mem, 0x3000);
    // C3=0, C2=0, C0=1 for less
    assert_eq!(status & 0x4500, 0x0100);
}

#[test]
fn test_ficom_m32int_equal() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDA, 0x14, 0x25, 0x08, 0x20, 0x00,
        0x00, // FICOM dword [0x2008]
        0xDF, 0xE0, 0x66, 0x89, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 1000.0);
    write_i32(&mem, DATA_ADDR + 8, 1000);

    run_until_hlt(&mut vcpu).unwrap();
    let status = read_u16(&mem, 0x3000);
    assert_eq!(status & 0x4500, 0x4000);
}

#[test]
fn test_ficom_m32int_negative() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDA, 0x14, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDF,
        0xE0, 0x66, 0x89, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, -10.0);
    write_i32(&mem, DATA_ADDR + 8, 10);

    run_until_hlt(&mut vcpu).unwrap();
    let status = read_u16(&mem, 0x3000);
    // -10 < 10, so C0=1
    assert_eq!(status & 0x4500, 0x0100);
}

#[test]
fn test_ficomp_m16int_pops_stack() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xDE, 0x1C, 0x25, 0x08, 0x20, 0x00, 0x00, // FICOMP word [0x2008]
        0xDF, 0xE0, // FSTSW AX
        0x66, 0x89, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 100.0);
    write_i16(&mem, DATA_ADDR + 8, 100);

    run_until_hlt(&mut vcpu).unwrap();
    let status = read_u16(&mem, 0x3000);
    // Should be equal
    assert_eq!(status & 0x4500, 0x4000);
}

#[test]
fn test_ficomp_m32int_pops_stack() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDA, 0x1C, 0x25, 0x08, 0x20, 0x00,
        0x00, // FICOMP dword [0x2008]
        0xDF, 0xE0, 0x66, 0x89, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 500.0);
    write_i32(&mem, DATA_ADDR + 8, 500);

    run_until_hlt(&mut vcpu).unwrap();
    let status = read_u16(&mem, 0x3000);
    assert_eq!(status & 0x4500, 0x4000);
}
