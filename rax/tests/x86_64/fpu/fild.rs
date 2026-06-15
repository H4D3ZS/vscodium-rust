//! Tests for the FILD instruction.
//!
//! FILD - Load Integer
//!
//! Converts the integer source operand to double extended-precision floating-point
//! format and pushes the value onto the FPU register stack. The source operand can be
//! a word integer (2 bytes), a doubleword integer (4 bytes), or a quadword integer (8 bytes).
//!
//! Opcodes:
//! - FILD m16int: DF /0
//! - FILD m32int: DB /0
//! - FILD m64int: DF /5
//!
//! Reference: /Users/int/dev/rax/docs/fild.txt

use crate::common::*;
use std::sync::Arc;
use vm_memory::{Bytes, GuestAddress, GuestMemoryMmap};

const DATA_ADDR: u64 = 0x2000;

fn write_i16(mem: &Arc<GuestMemoryMmap>, addr: u64, value: i16) {
    mem.write_slice(&value.to_le_bytes(), GuestAddress(addr))
        .unwrap();
}

fn write_i32(mem: &Arc<GuestMemoryMmap>, addr: u64, value: i32) {
    mem.write_slice(&value.to_le_bytes(), GuestAddress(addr))
        .unwrap();
}

fn write_i64(mem: &Arc<GuestMemoryMmap>, addr: u64, value: i64) {
    mem.write_slice(&value.to_le_bytes(), GuestAddress(addr))
        .unwrap();
}

fn read_f64(mem: &Arc<GuestMemoryMmap>, addr: u64) -> f64 {
    let mut buf = [0u8; 8];
    mem.read_slice(&mut buf, GuestAddress(addr)).unwrap();
    f64::from_le_bytes(buf)
}

// ============================================================================
// FILD m16int (opcode DF /0)
// ============================================================================

#[test]
fn test_fild_m16int_zero() {
    // FILD word ptr [0x2000]
    // FSTP qword ptr [0x3000]
    let code = [
        0xDF, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_i16(&mem, DATA_ADDR, 0);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 0.0);
}

#[test]
fn test_fild_m16int_positive_one() {
    let code = [
        0xDF, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_i16(&mem, DATA_ADDR, 1);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 1.0);
}

#[test]
fn test_fild_m16int_negative_one() {
    let code = [
        0xDF, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_i16(&mem, DATA_ADDR, -1);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), -1.0);
}

#[test]
fn test_fild_m16int_max() {
    let code = [
        0xDF, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_i16(&mem, DATA_ADDR, i16::MAX);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), i16::MAX as f64);
}

#[test]
fn test_fild_m16int_min() {
    let code = [
        0xDF, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_i16(&mem, DATA_ADDR, i16::MIN);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), i16::MIN as f64);
}

#[test]
fn test_fild_m16int_positive_100() {
    let code = [
        0xDF, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_i16(&mem, DATA_ADDR, 100);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 100.0);
}

#[test]
fn test_fild_m16int_negative_100() {
    let code = [
        0xDF, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_i16(&mem, DATA_ADDR, -100);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), -100.0);
}

#[test]
fn test_fild_m16int_1000() {
    let code = [
        0xDF, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_i16(&mem, DATA_ADDR, 1000);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 1000.0);
}

#[test]
fn test_fild_m16int_multiple() {
    // Load multiple integers onto stack
    let code = [
        0xDF, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FILD 10
        0xDF, 0x04, 0x25, 0x02, 0x20, 0x00, 0x00, // FILD 20
        0xDF, 0x04, 0x25, 0x04, 0x20, 0x00, 0x00, // FILD 30
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP 30
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, // FSTP 20
        0xDD, 0x1C, 0x25, 0x10, 0x30, 0x00, 0x00, // FSTP 10
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_i16(&mem, DATA_ADDR, 10);
    write_i16(&mem, DATA_ADDR + 2, 20);
    write_i16(&mem, DATA_ADDR + 4, 30);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 30.0);
    assert_eq!(read_f64(&mem, 0x3008), 20.0);
    assert_eq!(read_f64(&mem, 0x3010), 10.0);
}

#[test]
fn test_fild_m16int_arithmetic() {
    // FILD 5, FILD 3, FADDP -> 8
    let code = [
        0xDF, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDF, 0x04, 0x25, 0x02, 0x20, 0x00, 0x00, 0xDE,
        0xC1, // FADDP
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_i16(&mem, DATA_ADDR, 5);
    write_i16(&mem, DATA_ADDR + 2, 3);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 8.0);
}

// ============================================================================
// FILD m32int (opcode DB /0)
// ============================================================================

#[test]
fn test_fild_m32int_zero() {
    // FILD dword ptr [0x2000]
    // FSTP qword ptr [0x3000]
    let code = [
        0xDB, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_i32(&mem, DATA_ADDR, 0);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 0.0);
}

#[test]
fn test_fild_m32int_positive_one() {
    let code = [
        0xDB, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_i32(&mem, DATA_ADDR, 1);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 1.0);
}

#[test]
fn test_fild_m32int_negative_one() {
    let code = [
        0xDB, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_i32(&mem, DATA_ADDR, -1);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), -1.0);
}

#[test]
fn test_fild_m32int_max() {
    let code = [
        0xDB, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_i32(&mem, DATA_ADDR, i32::MAX);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), i32::MAX as f64);
}

#[test]
fn test_fild_m32int_min() {
    let code = [
        0xDB, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_i32(&mem, DATA_ADDR, i32::MIN);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), i32::MIN as f64);
}

#[test]
fn test_fild_m32int_1000000() {
    let code = [
        0xDB, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_i32(&mem, DATA_ADDR, 1000000);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 1000000.0);
}

#[test]
fn test_fild_m32int_negative_1000000() {
    let code = [
        0xDB, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_i32(&mem, DATA_ADDR, -1000000);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), -1000000.0);
}

#[test]
fn test_fild_m32int_12345() {
    let code = [
        0xDB, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_i32(&mem, DATA_ADDR, 12345);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 12345.0);
}

#[test]
fn test_fild_m32int_large_positive() {
    let code = [
        0xDB, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_i32(&mem, DATA_ADDR, 1_000_000_000);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 1_000_000_000.0);
}

#[test]
fn test_fild_m32int_large_negative() {
    let code = [
        0xDB, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_i32(&mem, DATA_ADDR, -1_000_000_000);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), -1_000_000_000.0);
}

#[test]
fn test_fild_m32int_multiple() {
    // Load multiple integers onto stack
    let code = [
        0xDB, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FILD 100
        0xDB, 0x04, 0x25, 0x04, 0x20, 0x00, 0x00, // FILD 200
        0xDB, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FILD 300
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP 300
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, // FSTP 200
        0xDD, 0x1C, 0x25, 0x10, 0x30, 0x00, 0x00, // FSTP 100
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_i32(&mem, DATA_ADDR, 100);
    write_i32(&mem, DATA_ADDR + 4, 200);
    write_i32(&mem, DATA_ADDR + 8, 300);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 300.0);
    assert_eq!(read_f64(&mem, 0x3008), 200.0);
    assert_eq!(read_f64(&mem, 0x3010), 100.0);
}

#[test]
fn test_fild_m32int_arithmetic() {
    // FILD 50, FILD 30, FSUBP -> 20
    let code = [
        0xDB, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDB, 0x04, 0x25, 0x04, 0x20, 0x00, 0x00, 0xDE,
        0xE9, // FSUBP
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_i32(&mem, DATA_ADDR, 50);
    write_i32(&mem, DATA_ADDR + 4, 30);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 20.0);
}

// ============================================================================
// FILD m64int (opcode DF /5)
// ============================================================================

#[test]
fn test_fild_m64int_zero() {
    // FILD qword ptr [0x2000]
    // FSTP qword ptr [0x3000]
    let code = [
        0xDF, 0x2C, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_i64(&mem, DATA_ADDR, 0);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 0.0);
}

#[test]
fn test_fild_m64int_positive_one() {
    let code = [
        0xDF, 0x2C, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_i64(&mem, DATA_ADDR, 1);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 1.0);
}

#[test]
fn test_fild_m64int_negative_one() {
    let code = [
        0xDF, 0x2C, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_i64(&mem, DATA_ADDR, -1);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), -1.0);
}

#[test]
fn test_fild_m64int_large_positive() {
    let code = [
        0xDF, 0x2C, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_i64(&mem, DATA_ADDR, 1_000_000_000_000i64);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 1_000_000_000_000.0);
}

#[test]
fn test_fild_m64int_large_negative() {
    let code = [
        0xDF, 0x2C, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_i64(&mem, DATA_ADDR, -1_000_000_000_000i64);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), -1_000_000_000_000.0);
}

#[test]
fn test_fild_m64int_max_safe_integer() {
    // Maximum integer that can be represented exactly in f64 (2^53)
    let code = [
        0xDF, 0x2C, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    let max_safe = (1i64 << 53) - 1;
    write_i64(&mem, DATA_ADDR, max_safe);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), max_safe as f64);
}

#[test]
fn test_fild_m64int_min_safe_integer() {
    let code = [
        0xDF, 0x2C, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    let min_safe = -((1i64 << 53) - 1);
    write_i64(&mem, DATA_ADDR, min_safe);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), min_safe as f64);
}

#[test]
fn test_fild_m64int_123456789() {
    let code = [
        0xDF, 0x2C, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_i64(&mem, DATA_ADDR, 123456789);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 123456789.0);
}

#[test]
fn test_fild_m64int_negative_123456789() {
    let code = [
        0xDF, 0x2C, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_i64(&mem, DATA_ADDR, -123456789);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), -123456789.0);
}

#[test]
fn test_fild_m64int_multiple() {
    // Load multiple integers onto stack
    let code = [
        0xDF, 0x2C, 0x25, 0x00, 0x20, 0x00, 0x00, // FILD 1000
        0xDF, 0x2C, 0x25, 0x08, 0x20, 0x00, 0x00, // FILD 2000
        0xDF, 0x2C, 0x25, 0x10, 0x20, 0x00, 0x00, // FILD 3000
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP 3000
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, // FSTP 2000
        0xDD, 0x1C, 0x25, 0x10, 0x30, 0x00, 0x00, // FSTP 1000
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_i64(&mem, DATA_ADDR, 1000);
    write_i64(&mem, DATA_ADDR + 8, 2000);
    write_i64(&mem, DATA_ADDR + 16, 3000);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 3000.0);
    assert_eq!(read_f64(&mem, 0x3008), 2000.0);
    assert_eq!(read_f64(&mem, 0x3010), 1000.0);
}

#[test]
fn test_fild_m64int_arithmetic() {
    // FILD 100, FILD 50, FMULP -> 5000
    let code = [
        0xDF, 0x2C, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDF, 0x2C, 0x25, 0x08, 0x20, 0x00, 0x00, 0xDE,
        0xC9, // FMULP
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_i64(&mem, DATA_ADDR, 100);
    write_i64(&mem, DATA_ADDR + 8, 50);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 5000.0);
}

// ============================================================================
// Mixed size tests
// ============================================================================

#[test]
fn test_fild_mixed_sizes() {
    // Load different integer sizes
    let code = [
        0xDF, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FILD word 10
        0xDB, 0x04, 0x25, 0x02, 0x20, 0x00, 0x00, // FILD dword 20
        0xDF, 0x2C, 0x25, 0x08, 0x20, 0x00, 0x00, // FILD qword 30
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP 30
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, // FSTP 20
        0xDD, 0x1C, 0x25, 0x10, 0x30, 0x00, 0x00, // FSTP 10
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_i16(&mem, DATA_ADDR, 10);
    write_i32(&mem, DATA_ADDR + 2, 20);
    write_i64(&mem, DATA_ADDR + 8, 30);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 30.0);
    assert_eq!(read_f64(&mem, 0x3008), 20.0);
    assert_eq!(read_f64(&mem, 0x3010), 10.0);
}

#[test]
fn test_fild_all_sizes_arithmetic() {
    // Sum different sizes: 100 + 1000 + 10000 = 11100
    let code = [
        0xDF, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FILD word 100
        0xDB, 0x04, 0x25, 0x02, 0x20, 0x00, 0x00, // FILD dword 1000
        0xDE, 0xC1, // FADDP
        0xDF, 0x2C, 0x25, 0x08, 0x20, 0x00, 0x00, // FILD qword 10000
        0xDE, 0xC1, // FADDP
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_i16(&mem, DATA_ADDR, 100);
    write_i32(&mem, DATA_ADDR + 2, 1000);
    write_i64(&mem, DATA_ADDR + 8, 10000);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 11100.0);
}

// ============================================================================
// Edge cases and special values
// ============================================================================

#[test]
fn test_fild_m16int_power_of_two() {
    let code = [
        0xDF, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_i16(&mem, DATA_ADDR, 1024);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 1024.0);
}

#[test]
fn test_fild_m32int_power_of_two() {
    let code = [
        0xDB, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_i32(&mem, DATA_ADDR, 1 << 20); // 1048576

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 1048576.0);
}

#[test]
fn test_fild_m64int_power_of_two() {
    let code = [
        0xDF, 0x2C, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_i64(&mem, DATA_ADDR, 1i64 << 40);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), (1i64 << 40) as f64);
}

#[test]
fn test_fild_m16int_all_bits_set() {
    // -1 in two's complement
    let code = [
        0xDF, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_i16(&mem, DATA_ADDR, -1);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), -1.0);
}

#[test]
fn test_fild_m32int_all_bits_set() {
    let code = [
        0xDB, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_i32(&mem, DATA_ADDR, -1);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), -1.0);
}

#[test]
fn test_fild_m64int_all_bits_set() {
    let code = [
        0xDF, 0x2C, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_i64(&mem, DATA_ADDR, -1);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), -1.0);
}

#[test]
fn test_fild_stack_operations() {
    // Test stack behavior with FILD
    let code = [
        0xDF, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FILD 1
        0xDF, 0x04, 0x25, 0x02, 0x20, 0x00, 0x00, // FILD 2
        0xDF, 0x04, 0x25, 0x04, 0x20, 0x00, 0x00, // FILD 3
        0xDF, 0x04, 0x25, 0x06, 0x20, 0x00, 0x00, // FILD 4
        0xDE, 0xC1, // FADDP ST(1), ST(0) -> 7
        0xDE, 0xC1, // FADDP ST(1), ST(0) -> 9
        0xDE, 0xC1, // FADDP ST(1), ST(0) -> 10
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_i16(&mem, DATA_ADDR, 1);
    write_i16(&mem, DATA_ADDR + 2, 2);
    write_i16(&mem, DATA_ADDR + 4, 3);
    write_i16(&mem, DATA_ADDR + 6, 4);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 10.0);
}

#[test]
fn test_fild_conversion_exact() {
    // Verify exact conversion for small integers
    let code = [
        0xDB, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_i32(&mem, DATA_ADDR, 42);

    run_until_hlt(&mut vcpu).unwrap();
    let result = read_f64(&mem, 0x3000);
    assert_eq!(result.to_bits(), 42.0_f64.to_bits());
}

// ============================================================================
// Known-answer FILD integer->float conversion tests (exact, all widths).
// FILD m16int=DF /0, FILD m32int=DB /0, FILD m64int=DF /5.
// ============================================================================

#[test]
fn test_fild_m16_signed_values_exact() {
    let code = [
        0xDF, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FILD word [0x2000]
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xf4,
    ];
    for v in [0_i16, 1, -1, 100, -100, i16::MAX, i16::MIN] {
        let (mut vcpu, mem) = setup_vm(&code, None);
        write_i16(&mem, DATA_ADDR, v);
        run_until_hlt(&mut vcpu).unwrap();
        assert_eq!(read_f64(&mem, 0x3000), v as f64, "FILD m16 of {v}");
    }
}

#[test]
fn test_fild_m32_signed_values_exact() {
    let code = [
        0xDB, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FILD dword [0x2000]
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xf4,
    ];
    for v in [0_i32, 1, -1, 1_000_000, -1_000_000, i32::MAX, i32::MIN] {
        let (mut vcpu, mem) = setup_vm(&code, None);
        write_i32(&mem, DATA_ADDR, v);
        run_until_hlt(&mut vcpu).unwrap();
        assert_eq!(read_f64(&mem, 0x3000), v as f64, "FILD m32 of {v}");
    }
}

#[test]
fn test_fild_m64_exact_when_representable() {
    let code = [
        0xDF, 0x2C, 0x25, 0x00, 0x20, 0x00, 0x00, // FILD qword [0x2000]
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xf4,
    ];
    // Values exactly representable in f64 (|v| < 2^53 or low bits zero).
    for v in [0_i64, 1, -1, 1_000_000_000_000, -(1_i64 << 52), 1_i64 << 52] {
        let (mut vcpu, mem) = setup_vm(&code, None);
        write_i64(&mem, DATA_ADDR, v);
        run_until_hlt(&mut vcpu).unwrap();
        assert_eq!(read_f64(&mem, 0x3000), v as f64, "FILD m64 of {v}");
    }
}
