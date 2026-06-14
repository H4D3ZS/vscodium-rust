//! Tests for the FIST and FISTP instructions.
//!
//! FIST - Store Integer
//! FISTP - Store Integer and Pop
//!
//! Converts the value in ST(0) to a signed integer and stores the result in the destination.
//! FISTP also pops the register stack. The value is rounded to an integer according to the
//! rounding mode specified in the FPU control word.
//!
//! Opcodes:
//! - FIST m16int: DF /2
//! - FIST m32int: DB /2
//! - FISTP m16int: DF /3
//! - FISTP m32int: DB /3
//! - FISTP m64int: DF /7
//!
//! Reference: /Users/int/dev/rax/docs/fist:fistp.txt

use crate::common::*;
use std::sync::Arc;
use vm_memory::{Bytes, GuestAddress, GuestMemoryMmap};

const DATA_ADDR: u64 = 0x2000;

fn write_f64(mem: &Arc<GuestMemoryMmap>, addr: u64, value: f64) {
    mem.write_slice(&value.to_le_bytes(), GuestAddress(addr))
        .unwrap();
}

fn read_i16(mem: &Arc<GuestMemoryMmap>, addr: u64) -> i16 {
    let mut buf = [0u8; 2];
    mem.read_slice(&mut buf, GuestAddress(addr)).unwrap();
    i16::from_le_bytes(buf)
}

fn read_i32(mem: &Arc<GuestMemoryMmap>, addr: u64) -> i32 {
    let mut buf = [0u8; 4];
    mem.read_slice(&mut buf, GuestAddress(addr)).unwrap();
    i32::from_le_bytes(buf)
}

fn read_i64(mem: &Arc<GuestMemoryMmap>, addr: u64) -> i64 {
    let mut buf = [0u8; 8];
    mem.read_slice(&mut buf, GuestAddress(addr)).unwrap();
    i64::from_le_bytes(buf)
}

fn read_f64(mem: &Arc<GuestMemoryMmap>, addr: u64) -> f64 {
    let mut buf = [0u8; 8];
    mem.read_slice(&mut buf, GuestAddress(addr)).unwrap();
    f64::from_le_bytes(buf)
}

// ============================================================================
// FIST m16int (opcode DF /2)
// ============================================================================

#[test]
fn test_fist_m16int_zero() {
    // FLD qword ptr [0x2000]
    // FIST word ptr [0x3000]
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDF, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, 0xDD,
        0xD8, // FSTP ST(0) to clean up
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 0.0);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_i16(&mem, 0x3000), 0);
}

#[test]
fn test_fist_m16int_positive_one() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDF, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, 0xDD,
        0xD8, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 1.0);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_i16(&mem, 0x3000), 1);
}

#[test]
fn test_fist_m16int_negative_one() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDF, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, 0xDD,
        0xD8, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, -1.0);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_i16(&mem, 0x3000), -1);
}

#[test]
fn test_fist_m16int_100() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDF, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, 0xDD,
        0xD8, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 100.0);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_i16(&mem, 0x3000), 100);
}

#[test]
fn test_fist_m16int_negative_100() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDF, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, 0xDD,
        0xD8, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, -100.0);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_i16(&mem, 0x3000), -100);
}

#[test]
fn test_fist_m16int_rounding_down() {
    // 2.3 should round to 2 (default rounding mode is round-to-nearest-even)
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDF, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, 0xDD,
        0xD8, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 2.3);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_i16(&mem, 0x3000), 2);
}

#[test]
fn test_fist_m16int_rounding_up() {
    // 2.7 should round to 3
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDF, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, 0xDD,
        0xD8, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 2.7);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_i16(&mem, 0x3000), 3);
}

#[test]
fn test_fist_m16int_half_round_even() {
    // 2.5 should round to 2 (even)
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDF, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, 0xDD,
        0xD8, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 2.5);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_i16(&mem, 0x3000), 2);
}

#[test]
fn test_fist_m16int_preserves_st0() {
    // FIST should not pop the stack
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDF, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, 0xDD,
        0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, // FSTP to verify value still there
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 42.0);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_i16(&mem, 0x3000), 42);
    assert_eq!(read_f64(&mem, 0x3008), 42.0);
}

// ============================================================================
// FISTP m16int (opcode DF /3)
// ============================================================================

#[test]
fn test_fistp_m16int_zero() {
    // FISTP word ptr [0x3000]
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDF, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 0.0);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_i16(&mem, 0x3000), 0);
}

#[test]
fn test_fistp_m16int_positive() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDF, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 123.0);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_i16(&mem, 0x3000), 123);
}

#[test]
fn test_fistp_m16int_negative() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDF, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, -456.0);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_i16(&mem, 0x3000), -456);
}

#[test]
fn test_fistp_m16int_rounding() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDF, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 99.6);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_i16(&mem, 0x3000), 100);
}

#[test]
fn test_fistp_m16int_max_value() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDF, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, i16::MAX as f64);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_i16(&mem, 0x3000), i16::MAX);
}

#[test]
fn test_fistp_m16int_min_value() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDF, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, i16::MIN as f64);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_i16(&mem, 0x3000), i16::MIN);
}

// ============================================================================
// FIST m32int (opcode DB /2)
// ============================================================================

#[test]
fn test_fist_m32int_zero() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDB, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, 0xDD,
        0xD8, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 0.0);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_i32(&mem, 0x3000), 0);
}

#[test]
fn test_fist_m32int_positive() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDB, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, 0xDD,
        0xD8, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 12345.0);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_i32(&mem, 0x3000), 12345);
}

#[test]
fn test_fist_m32int_negative() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDB, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, 0xDD,
        0xD8, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, -67890.0);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_i32(&mem, 0x3000), -67890);
}

#[test]
fn test_fist_m32int_large() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDB, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, 0xDD,
        0xD8, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 1000000.0);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_i32(&mem, 0x3000), 1000000);
}

#[test]
fn test_fist_m32int_rounding() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDB, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, 0xDD,
        0xD8, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 1234.8);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_i32(&mem, 0x3000), 1235);
}

#[test]
fn test_fist_m32int_preserves_st0() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDB, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, 0xDD,
        0x1C, 0x25, 0x04, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 9999.0);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_i32(&mem, 0x3000), 9999);
    assert_eq!(read_f64(&mem, 0x3004), 9999.0);
}

// ============================================================================
// FISTP m32int (opcode DB /3)
// ============================================================================

#[test]
fn test_fistp_m32int_zero() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDB, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 0.0);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_i32(&mem, 0x3000), 0);
}

#[test]
fn test_fistp_m32int_positive() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDB, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 987654.0);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_i32(&mem, 0x3000), 987654);
}

#[test]
fn test_fistp_m32int_negative() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDB, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, -123456.0);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_i32(&mem, 0x3000), -123456);
}

#[test]
fn test_fistp_m32int_max_value() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDB, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, i32::MAX as f64);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_i32(&mem, 0x3000), i32::MAX);
}

#[test]
fn test_fistp_m32int_min_value() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDB, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, i32::MIN as f64);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_i32(&mem, 0x3000), i32::MIN);
}

// ============================================================================
// FISTP m64int (opcode DF /7)
// ============================================================================

#[test]
fn test_fistp_m64int_zero() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDF, 0x3C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 0.0);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_i64(&mem, 0x3000), 0);
}

#[test]
fn test_fistp_m64int_positive() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDF, 0x3C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 123456789.0);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_i64(&mem, 0x3000), 123456789);
}

#[test]
fn test_fistp_m64int_negative() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDF, 0x3C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, -987654321.0);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_i64(&mem, 0x3000), -987654321);
}

#[test]
fn test_fistp_m64int_large() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDF, 0x3C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 1_000_000_000_000.0);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_i64(&mem, 0x3000), 1_000_000_000_000);
}

#[test]
fn test_fistp_m64int_rounding() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDF, 0x3C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 999999.9);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_i64(&mem, 0x3000), 1000000);
}

#[test]
fn test_fistp_m64int_max_safe_integer() {
    // Maximum exact integer in f64 is 2^53
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDF, 0x3C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    let max_safe = (1i64 << 53) - 1;
    write_f64(&mem, DATA_ADDR, max_safe as f64);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_i64(&mem, 0x3000), max_safe);
}

#[test]
fn test_fistp_m64int_min_safe_integer() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDF, 0x3C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    let min_safe = -((1i64 << 53) - 1);
    write_f64(&mem, DATA_ADDR, min_safe as f64);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_i64(&mem, 0x3000), min_safe);
}

// ============================================================================
// Stack behavior tests
// ============================================================================

#[test]
fn test_fist_does_not_pop() {
    // FIST should preserve stack
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD
        0xDB, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // FIST
        0xDB, 0x14, 0x25, 0x04, 0x30, 0x00, 0x00, // FIST again
        0xDD, 0xD8, // Clean up
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 777.0);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_i32(&mem, 0x3000), 777);
    assert_eq!(read_i32(&mem, 0x3004), 777);
}

#[test]
fn test_fistp_pops_stack() {
    // FISTP should pop the stack
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD 100
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD 200
        0xDB, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FISTP (should store 200)
        0xDB, 0x1C, 0x25, 0x04, 0x30, 0x00, 0x00, // FISTP (should store 100)
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 100.0);
    write_f64(&mem, DATA_ADDR + 8, 200.0);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_i32(&mem, 0x3000), 200);
    assert_eq!(read_i32(&mem, 0x3004), 100);
}

// ============================================================================
// Multiple size tests
// ============================================================================

#[test]
fn test_fistp_all_sizes() {
    // Store same value in all three sizes
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD
        0xD9, 0xC0, // FLD ST(0) - duplicate
        0xD9, 0xC0, // FLD ST(0) - duplicate again
        0xDF, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FISTP word
        0xDB, 0x1C, 0x25, 0x04, 0x30, 0x00, 0x00, // FISTP dword
        0xDF, 0x3C, 0x25, 0x08, 0x30, 0x00, 0x00, // FISTP qword
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 1234.0);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_i16(&mem, 0x3000), 1234);
    assert_eq!(read_i32(&mem, 0x3004), 1234);
    assert_eq!(read_i64(&mem, 0x3008), 1234);
}

#[test]
fn test_fist_fistp_mixed() {
    // Mix FIST and FISTP operations
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD 500
        0xDB, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // FIST (preserve)
        0xDB, 0x1C, 0x25, 0x04, 0x30, 0x00, 0x00, // FISTP (pop)
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 500.0);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_i32(&mem, 0x3000), 500);
    assert_eq!(read_i32(&mem, 0x3004), 500);
}

// ============================================================================
// Rounding mode tests
// ============================================================================

#[test]
fn test_fistp_round_to_nearest_even_positive() {
    // Test round-to-nearest-even (default)
    let test_cases = vec![
        (0.5, 0), // 0.5 rounds to 0 (even)
        (1.5, 2), // 1.5 rounds to 2 (even)
        (2.5, 2), // 2.5 rounds to 2 (even)
        (3.5, 4), // 3.5 rounds to 4 (even)
    ];

    for (input, expected) in test_cases {
        let code = [
            0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDB, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
            0xf4,
        ];
        let (mut vcpu, mem) = setup_vm(&code, None);
        write_f64(&mem, DATA_ADDR, input);

        run_until_hlt(&mut vcpu).unwrap();
        assert_eq!(
            read_i32(&mem, 0x3000),
            expected,
            "Failed for input {}",
            input
        );
    }
}

#[test]
fn test_fistp_round_to_nearest_even_negative() {
    let test_cases = vec![
        (-0.5, 0),  // -0.5 rounds to 0 (even)
        (-1.5, -2), // -1.5 rounds to -2 (even)
        (-2.5, -2), // -2.5 rounds to -2 (even)
        (-3.5, -4), // -3.5 rounds to -4 (even)
    ];

    for (input, expected) in test_cases {
        let code = [
            0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDB, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00,
            0xf4,
        ];
        let (mut vcpu, mem) = setup_vm(&code, None);
        write_f64(&mem, DATA_ADDR, input);

        run_until_hlt(&mut vcpu).unwrap();
        assert_eq!(
            read_i32(&mem, 0x3000),
            expected,
            "Failed for input {}",
            input
        );
    }
}

// ============================================================================
// Arithmetic integration tests
// ============================================================================

#[test]
fn test_fistp_after_arithmetic() {
    // Compute and store result
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD 10.5
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD 20.3
        0xDE, 0xC1, // FADDP (30.8)
        0xDB, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FISTP (should be 31)
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 10.5);
    write_f64(&mem, DATA_ADDR + 8, 20.3);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_i32(&mem, 0x3000), 31);
}

#[test]
fn test_fistp_sequential() {
    // Sequential FISTP operations
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD 1
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD 2
        0xDD, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, // FLD 3
        0xDF, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FISTP word (3)
        0xDB, 0x1C, 0x25, 0x04, 0x30, 0x00, 0x00, // FISTP dword (2)
        0xDF, 0x3C, 0x25, 0x08, 0x30, 0x00, 0x00, // FISTP qword (1)
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 1.0);
    write_f64(&mem, DATA_ADDR + 8, 2.0);
    write_f64(&mem, DATA_ADDR + 16, 3.0);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_i16(&mem, 0x3000), 3);
    assert_eq!(read_i32(&mem, 0x3004), 2);
    assert_eq!(read_i64(&mem, 0x3008), 1);
}

// ============================================================================
// Known-answer FIST/FISTP rounding tests.
//
// With the default control word (0x037F = round-to-nearest, ties-to-even),
// FIST/FISTP must round halfway cases to the nearest EVEN integer:
//   0.5 -> 0, 1.5 -> 2, 2.5 -> 2, 3.5 -> 4, -2.5 -> -2.
// ============================================================================

/// FLD [m64], FISTP dword [0x3000] -> returns the stored i32.
fn kat_fistp_m32(value: f64) -> i32 {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xDB, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FISTP dword [0x3000]
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, value);
    run_until_hlt(&mut vcpu).unwrap();
    read_i32(&mem, 0x3000)
}

#[test]
fn test_fistp_round_nearest_even_halfway() {
    assert_eq!(kat_fistp_m32(0.5), 0, "0.5 ties to even -> 0");
    assert_eq!(kat_fistp_m32(1.5), 2, "1.5 ties to even -> 2");
    assert_eq!(kat_fistp_m32(2.5), 2, "2.5 ties to even -> 2");
    assert_eq!(kat_fistp_m32(3.5), 4, "3.5 ties to even -> 4");
    assert_eq!(kat_fistp_m32(-0.5), 0, "-0.5 ties to even -> 0");
    assert_eq!(kat_fistp_m32(-1.5), -2, "-1.5 ties to even -> -2");
    assert_eq!(kat_fistp_m32(-2.5), -2, "-2.5 ties to even -> -2");
}

#[test]
fn test_fistp_round_nearest_nonhalfway() {
    assert_eq!(kat_fistp_m32(2.4), 2);
    assert_eq!(kat_fistp_m32(2.6), 3);
    assert_eq!(kat_fistp_m32(-2.4), -2);
    assert_eq!(kat_fistp_m32(-2.6), -3);
    assert_eq!(kat_fistp_m32(0.0), 0);
    assert_eq!(kat_fistp_m32(7.0), 7);
}

#[test]
fn test_fist_m32_does_not_pop() {
    // FIST m32 (DB /2) stores but does not pop; a second store must succeed.
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xDB, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // FIST dword [0x3000] (no pop)
        0xDB, 0x1C, 0x25, 0x04, 0x30, 0x00, 0x00, // FISTP dword [0x3004]
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 12.0);
    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_i32(&mem, 0x3000), 12);
    assert_eq!(read_i32(&mem, 0x3004), 12, "FIST must not pop ST(0)");
}

#[test]
fn test_filp_fistp_m64_roundtrip() {
    // FILD m64 then FISTP m64 must reproduce the integer exactly.
    let code = [
        0xDF, 0x2C, 0x25, 0x00, 0x20, 0x00, 0x00, // FILD qword [0x2000]
        0xDF, 0x3C, 0x25, 0x00, 0x30, 0x00, 0x00, // FISTP qword [0x3000]
        0xf4,
    ];
    for v in [0_i64, 1, -1, 123456789, -123456789] {
        let (mut vcpu, mem) = setup_vm(&code, None);
        mem.write_slice(&v.to_le_bytes(), GuestAddress(DATA_ADDR))
            .unwrap();
        run_until_hlt(&mut vcpu).unwrap();
        assert_eq!(
            read_i64(&mem, 0x3000),
            v,
            "FILD/FISTP m64 round-trip for {v}"
        );
    }
}
