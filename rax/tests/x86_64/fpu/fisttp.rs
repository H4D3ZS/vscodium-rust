//! Tests for the FISTTP instruction.
//!
//! FISTTP - Store Integer with Truncation and Pop
//!
//! Converts the value in ST(0) to a signed integer using truncation (round toward zero)
//! and stores the result in the destination, then pops the register stack.
//! Unlike FISTP, FISTTP always uses truncation regardless of the rounding mode in the
//! FPU control word.
//!
//! Opcodes:
//! - FISTTP m16int: DF /1
//! - FISTTP m32int: DB /1
//! - FISTTP m64int: DD /1
//!
//! Reference: /Users/int/dev/rax/docs/fisttp.txt

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

// ============================================================================
// FISTTP m16int (opcode DF /1)
// ============================================================================

#[test]
fn test_fisttp_m16int_zero() {
    // FISTTP word ptr [0x3000]
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDF, 0x0C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 0.0);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_i16(&mem, 0x3000), 0);
}

#[test]
fn test_fisttp_m16int_positive_one() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDF, 0x0C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 1.0);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_i16(&mem, 0x3000), 1);
}

#[test]
fn test_fisttp_m16int_negative_one() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDF, 0x0C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, -1.0);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_i16(&mem, 0x3000), -1);
}

#[test]
fn test_fisttp_m16int_truncate_positive() {
    // 2.9 should truncate to 2 (not round to 3)
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDF, 0x0C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 2.9);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_i16(&mem, 0x3000), 2);
}

#[test]
fn test_fisttp_m16int_truncate_negative() {
    // -2.9 should truncate to -2 (toward zero, not -3)
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDF, 0x0C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, -2.9);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_i16(&mem, 0x3000), -2);
}

#[test]
fn test_fisttp_m16int_truncate_half() {
    // 2.5 should truncate to 2 (not round)
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDF, 0x0C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 2.5);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_i16(&mem, 0x3000), 2);
}

#[test]
fn test_fisttp_m16int_truncate_negative_half() {
    // -2.5 should truncate to -2 (toward zero)
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDF, 0x0C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, -2.5);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_i16(&mem, 0x3000), -2);
}

#[test]
fn test_fisttp_m16int_truncate_small_fraction() {
    // 99.1 truncates to 99
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDF, 0x0C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 99.1);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_i16(&mem, 0x3000), 99);
}

#[test]
fn test_fisttp_m16int_truncate_large_fraction() {
    // 99.9 truncates to 99
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDF, 0x0C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 99.9);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_i16(&mem, 0x3000), 99);
}

#[test]
fn test_fisttp_m16int_max() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDF, 0x0C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, i16::MAX as f64);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_i16(&mem, 0x3000), i16::MAX);
}

#[test]
fn test_fisttp_m16int_min() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDF, 0x0C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, i16::MIN as f64);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_i16(&mem, 0x3000), i16::MIN);
}

// ============================================================================
// FISTTP m32int (opcode DB /1)
// ============================================================================

#[test]
fn test_fisttp_m32int_zero() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDB, 0x0C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 0.0);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_i32(&mem, 0x3000), 0);
}

#[test]
fn test_fisttp_m32int_positive() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDB, 0x0C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 12345.0);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_i32(&mem, 0x3000), 12345);
}

#[test]
fn test_fisttp_m32int_negative() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDB, 0x0C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, -67890.0);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_i32(&mem, 0x3000), -67890);
}

#[test]
fn test_fisttp_m32int_truncate_positive() {
    // 1234.99 truncates to 1234
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDB, 0x0C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 1234.99);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_i32(&mem, 0x3000), 1234);
}

#[test]
fn test_fisttp_m32int_truncate_negative() {
    // -1234.99 truncates to -1234
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDB, 0x0C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, -1234.99);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_i32(&mem, 0x3000), -1234);
}

#[test]
fn test_fisttp_m32int_truncate_half() {
    // 999.5 truncates to 999
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDB, 0x0C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 999.5);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_i32(&mem, 0x3000), 999);
}

#[test]
fn test_fisttp_m32int_large() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDB, 0x0C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 1000000.0);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_i32(&mem, 0x3000), 1000000);
}

#[test]
fn test_fisttp_m32int_max() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDB, 0x0C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, i32::MAX as f64);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_i32(&mem, 0x3000), i32::MAX);
}

#[test]
fn test_fisttp_m32int_min() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDB, 0x0C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, i32::MIN as f64);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_i32(&mem, 0x3000), i32::MIN);
}

// ============================================================================
// FISTTP m64int (opcode DD /1)
// ============================================================================

#[test]
fn test_fisttp_m64int_zero() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDD, 0x0C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 0.0);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_i64(&mem, 0x3000), 0);
}

#[test]
fn test_fisttp_m64int_positive() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDD, 0x0C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 123456789.0);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_i64(&mem, 0x3000), 123456789);
}

#[test]
fn test_fisttp_m64int_negative() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDD, 0x0C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, -987654321.0);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_i64(&mem, 0x3000), -987654321);
}

#[test]
fn test_fisttp_m64int_truncate_positive() {
    // 999999.999 truncates to 999999
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDD, 0x0C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 999999.999);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_i64(&mem, 0x3000), 999999);
}

#[test]
fn test_fisttp_m64int_truncate_negative() {
    // -999999.999 truncates to -999999
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDD, 0x0C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, -999999.999);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_i64(&mem, 0x3000), -999999);
}

#[test]
fn test_fisttp_m64int_large() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDD, 0x0C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 1_000_000_000_000.0);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_i64(&mem, 0x3000), 1_000_000_000_000);
}

#[test]
fn test_fisttp_m64int_max_safe() {
    // Maximum exact integer in f64 is 2^53
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDD, 0x0C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    let max_safe = (1i64 << 53) - 1;
    write_f64(&mem, DATA_ADDR, max_safe as f64);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_i64(&mem, 0x3000), max_safe);
}

#[test]
fn test_fisttp_m64int_min_safe() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDD, 0x0C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    let min_safe = -((1i64 << 53) - 1);
    write_f64(&mem, DATA_ADDR, min_safe as f64);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_i64(&mem, 0x3000), min_safe);
}

// ============================================================================
// Truncation vs rounding tests
// ============================================================================

#[test]
fn test_fisttp_truncation_positive_values() {
    // Test various positive fractional values
    let test_cases = vec![
        (0.1, 0),
        (0.9, 0),
        (1.1, 1),
        (1.9, 1),
        (2.5, 2),
        (3.5, 3),
        (99.99, 99),
    ];

    for (input, expected) in test_cases {
        let code = [
            0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDB, 0x0C, 0x25, 0x00, 0x30, 0x00, 0x00,
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
fn test_fisttp_truncation_negative_values() {
    // Test various negative fractional values
    let test_cases = vec![
        (-0.1, 0),
        (-0.9, 0),
        (-1.1, -1),
        (-1.9, -1),
        (-2.5, -2),
        (-3.5, -3),
        (-99.99, -99),
    ];

    for (input, expected) in test_cases {
        let code = [
            0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDB, 0x0C, 0x25, 0x00, 0x30, 0x00, 0x00,
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
// Stack behavior tests
// ============================================================================

#[test]
fn test_fisttp_pops_stack() {
    // FISTTP should pop the stack
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD 100.5
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD 200.5
        0xDB, 0x0C, 0x25, 0x00, 0x30, 0x00, 0x00, // FISTTP (200)
        0xDB, 0x0C, 0x25, 0x04, 0x30, 0x00, 0x00, // FISTTP (100)
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 100.5);
    write_f64(&mem, DATA_ADDR + 8, 200.5);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_i32(&mem, 0x3000), 200);
    assert_eq!(read_i32(&mem, 0x3004), 100);
}

#[test]
fn test_fisttp_sequential() {
    // Multiple FISTTP operations
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD 1.7
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD 2.7
        0xDD, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, // FLD 3.7
        0xDF, 0x0C, 0x25, 0x00, 0x30, 0x00, 0x00, // FISTTP word (3)
        0xDB, 0x0C, 0x25, 0x04, 0x30, 0x00, 0x00, // FISTTP dword (2)
        0xDD, 0x0C, 0x25, 0x08, 0x30, 0x00, 0x00, // FISTTP qword (1)
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 1.7);
    write_f64(&mem, DATA_ADDR + 8, 2.7);
    write_f64(&mem, DATA_ADDR + 16, 3.7);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_i16(&mem, 0x3000), 3);
    assert_eq!(read_i32(&mem, 0x3004), 2);
    assert_eq!(read_i64(&mem, 0x3008), 1);
}

// ============================================================================
// Arithmetic integration tests
// ============================================================================

#[test]
fn test_fisttp_after_arithmetic() {
    // Compute result and truncate
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD 10.7
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD 20.3
        0xDE, 0xC1, // FADDP (31.0)
        0xDB, 0x0C, 0x25, 0x00, 0x30, 0x00, 0x00, // FISTTP
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 10.7);
    write_f64(&mem, DATA_ADDR + 8, 20.3);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_i32(&mem, 0x3000), 31);
}

#[test]
fn test_fisttp_division_truncate() {
    // Test division with truncation
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD 10.0
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD 3.0
        0xDE, 0xF9, // FDIVP (3.333...)
        0xDB, 0x0C, 0x25, 0x00, 0x30, 0x00, 0x00, // FISTTP (should be 3)
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 10.0);
    write_f64(&mem, DATA_ADDR + 8, 3.0);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_i32(&mem, 0x3000), 3);
}

// ============================================================================
// Edge cases
// ============================================================================

#[test]
fn test_fisttp_very_small_positive() {
    // Values close to zero
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDB, 0x0C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 0.00001);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_i32(&mem, 0x3000), 0);
}

#[test]
fn test_fisttp_very_small_negative() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDB, 0x0C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, -0.00001);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_i32(&mem, 0x3000), 0);
}

#[test]
fn test_fisttp_almost_one() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDB, 0x0C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 0.99999);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_i32(&mem, 0x3000), 0);
}

#[test]
fn test_fisttp_almost_minus_one() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDB, 0x0C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, -0.99999);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_i32(&mem, 0x3000), 0);
}

#[test]
fn test_fisttp_all_sizes_same_value() {
    // Store same value in all sizes
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD 1234.9
        0xD9, 0xC0, // FLD ST(0) - duplicate
        0xD9, 0xC0, // FLD ST(0) - duplicate again
        0xDF, 0x0C, 0x25, 0x00, 0x30, 0x00, 0x00, // FISTTP word
        0xDB, 0x0C, 0x25, 0x04, 0x30, 0x00, 0x00, // FISTTP dword
        0xDD, 0x0C, 0x25, 0x08, 0x30, 0x00, 0x00, // FISTTP qword
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 1234.9);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_i16(&mem, 0x3000), 1234);
    assert_eq!(read_i32(&mem, 0x3004), 1234);
    assert_eq!(read_i64(&mem, 0x3008), 1234);
}

#[test]
fn test_fisttp_pi() {
    // Truncate π to 3
    let code = [
        0xD9, 0xEB, // FLDPI
        0xDB, 0x0C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_i32(&mem, 0x3000), 3);
}

#[test]
fn test_fisttp_e() {
    // Truncate e to 2
    let code = [
        0xD9, 0xE8, // FLD1
        0xD9, 0xEA, // FLDL2E
        0xD9, 0xED, // FLDLN2
        0xDE, 0xC9, // FMULP (log2(e) * ln(2) ≈ 1)
        0xDE, 0xC1, // FADDP (1 + 1 ≈ 2)
        0xD9, 0xE8, // FLD1
        0xDE, 0xC1, // FADDP (2 + 1 ≈ 3, but let's just use simple value)
        // Actually, let's just load e directly via calculation
        0xDD, 0xD8, // FSTP ST(0) - cleanup
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // Load e value
        0xDB, 0x0C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, std::f64::consts::E);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_i32(&mem, 0x3000), 2);
}
