//! Tests for the FBLD and FBSTP instructions.
//!
//! FBLD - Load Binary Coded Decimal
//! FBSTP - Store BCD and Pop
//!
//! FBLD converts a BCD (Binary Coded Decimal) source operand to double extended-precision
//! floating-point format and pushes the value onto the FPU register stack.
//!
//! FBSTP converts the value in ST(0) to an 18-digit packed BCD integer, stores the
//! result in the destination, and pops the register stack.
//!
//! BCD Format: 10 bytes (80 bits)
//! - Bytes 0-8: 18 BCD digits (2 digits per byte)
//! - Byte 9: Sign bit (bit 7) + most significant digit
//!
//! Opcodes:
//! - FBLD m80dec: DF /4
//! - FBSTP m80bcd: DF /6
//!
//! Reference: /Users/int/dev/rax/docs/fbld.txt, /Users/int/dev/rax/docs/fbstp.txt

use crate::common::*;
use std::sync::Arc;
use vm_memory::{Bytes, GuestAddress, GuestMemoryMmap};

const DATA_ADDR: u64 = 0x2000;

fn write_f64(mem: &Arc<GuestMemoryMmap>, addr: u64, value: f64) {
    mem.write_slice(&value.to_le_bytes(), GuestAddress(addr))
        .unwrap();
}

fn read_f64(mem: &Arc<GuestMemoryMmap>, addr: u64) -> f64 {
    let mut buf = [0u8; 8];
    mem.read_slice(&mut buf, GuestAddress(addr)).unwrap();
    f64::from_le_bytes(buf)
}

// Write BCD value to memory
fn write_bcd(mem: &Arc<GuestMemoryMmap>, addr: u64, value: &[u8; 10]) {
    mem.write_slice(value, GuestAddress(addr)).unwrap();
}

// Read BCD value from memory
fn read_bcd(mem: &Arc<GuestMemoryMmap>, addr: u64) -> [u8; 10] {
    let mut buf = [0u8; 10];
    mem.read_slice(&mut buf, GuestAddress(addr)).unwrap();
    buf
}

// Helper to create BCD representation from a decimal string
// BCD format: bytes 0-8 contain digits (LSB first), byte 9 has sign bit
fn make_bcd(value: i64) -> [u8; 10] {
    let mut bcd = [0u8; 10];
    let is_negative = value < 0;
    let mut abs_value = value.abs() as u64;

    // Pack digits (2 per byte)
    for i in 0..9 {
        let low = (abs_value % 10) as u8;
        abs_value /= 10;
        let high = (abs_value % 10) as u8;
        abs_value /= 10;
        bcd[i] = (high << 4) | low;
    }

    // Byte 9: sign bit (bit 7) + most significant digit
    bcd[9] = if is_negative { 0x80 } else { 0x00 };

    bcd
}

// Helper to extract value from BCD
fn parse_bcd(bcd: &[u8; 10]) -> Option<i64> {
    let is_negative = (bcd[9] & 0x80) != 0;
    let mut value: i64 = 0;
    let mut multiplier: i64 = 1;

    for i in 0..9 {
        let low = (bcd[i] & 0x0F) as i64;
        let high = ((bcd[i] >> 4) & 0x0F) as i64;

        if low > 9 || high > 9 {
            return None; // Invalid BCD
        }

        value += low * multiplier;
        multiplier *= 10;
        value += high * multiplier;
        multiplier *= 10;
    }

    if is_negative {
        Some(-value)
    } else {
        Some(value)
    }
}

// ============================================================================
// FBLD - Load BCD
// ============================================================================

#[test]
fn test_fbld_zero() {
    // FBLD tbyte ptr [0x2000]
    // FSTP qword ptr [0x3000]
    let code = [
        0xDF, 0x24, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_bcd(&mem, DATA_ADDR, &make_bcd(0));

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 0.0);
}

#[test]
fn test_fbld_positive_one() {
    let code = [
        0xDF, 0x24, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_bcd(&mem, DATA_ADDR, &make_bcd(1));

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 1.0);
}

#[test]
fn test_fbld_negative_one() {
    let code = [
        0xDF, 0x24, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_bcd(&mem, DATA_ADDR, &make_bcd(-1));

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), -1.0);
}

#[test]
fn test_fbld_positive_123() {
    let code = [
        0xDF, 0x24, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_bcd(&mem, DATA_ADDR, &make_bcd(123));

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 123.0);
}

#[test]
fn test_fbld_negative_456() {
    let code = [
        0xDF, 0x24, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_bcd(&mem, DATA_ADDR, &make_bcd(-456));

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), -456.0);
}

#[test]
fn test_fbld_large_positive() {
    let code = [
        0xDF, 0x24, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_bcd(&mem, DATA_ADDR, &make_bcd(123456789));

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 123456789.0);
}

#[test]
fn test_fbld_large_negative() {
    let code = [
        0xDF, 0x24, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_bcd(&mem, DATA_ADDR, &make_bcd(-987654321));

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), -987654321.0);
}

#[test]
fn test_fbld_max_digits() {
    // Test with maximum representable value (18 digits)
    let code = [
        0xDF, 0x24, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_bcd(&mem, DATA_ADDR, &make_bcd(999999999999999999));

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 999999999999999999.0);
}

#[test]
fn test_fbld_pushes_to_stack() {
    // FBLD should push value onto stack
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // FLD existing value
        0xDF, 0x24, 0x25, 0x00, 0x20, 0x00, 0x00, // FBLD
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, // FSTP new value
        0xDD, 0x1C, 0x25, 0x10, 0x30, 0x00, 0x00, // FSTP old value
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x3000, 99.0);
    write_bcd(&mem, DATA_ADDR, &make_bcd(42));

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3008), 42.0); // New value on top
    assert_eq!(read_f64(&mem, 0x3010), 99.0); // Old value below
}

#[test]
fn test_fbld_multiple() {
    // Load multiple BCD values
    let code = [
        0xDF, 0x24, 0x25, 0x00, 0x20, 0x00, 0x00, // FBLD 10
        0xDF, 0x24, 0x25, 0x0A, 0x20, 0x00, 0x00, // FBLD 20
        0xDF, 0x24, 0x25, 0x14, 0x20, 0x00, 0x00, // FBLD 30
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP 30
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, // FSTP 20
        0xDD, 0x1C, 0x25, 0x10, 0x30, 0x00, 0x00, // FSTP 10
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_bcd(&mem, DATA_ADDR, &make_bcd(10));
    write_bcd(&mem, DATA_ADDR + 10, &make_bcd(20));
    write_bcd(&mem, DATA_ADDR + 20, &make_bcd(30));

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 30.0);
    assert_eq!(read_f64(&mem, 0x3008), 20.0);
    assert_eq!(read_f64(&mem, 0x3010), 10.0);
}

// ============================================================================
// FBSTP - Store BCD and Pop
// ============================================================================

#[test]
fn test_fbstp_zero() {
    // FBSTP tbyte ptr [0x3000]
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDF, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 0.0);

    run_until_hlt(&mut vcpu).unwrap();
    let bcd = read_bcd(&mem, 0x3000);
    assert_eq!(parse_bcd(&bcd), Some(0));
}

#[test]
fn test_fbstp_positive_one() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDF, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 1.0);

    run_until_hlt(&mut vcpu).unwrap();
    let bcd = read_bcd(&mem, 0x3000);
    assert_eq!(parse_bcd(&bcd), Some(1));
}

#[test]
fn test_fbstp_negative_one() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDF, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, -1.0);

    run_until_hlt(&mut vcpu).unwrap();
    let bcd = read_bcd(&mem, 0x3000);
    assert_eq!(parse_bcd(&bcd), Some(-1));
}

#[test]
fn test_fbstp_positive_123() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDF, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 123.0);

    run_until_hlt(&mut vcpu).unwrap();
    let bcd = read_bcd(&mem, 0x3000);
    assert_eq!(parse_bcd(&bcd), Some(123));
}

#[test]
fn test_fbstp_negative_456() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDF, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, -456.0);

    run_until_hlt(&mut vcpu).unwrap();
    let bcd = read_bcd(&mem, 0x3000);
    assert_eq!(parse_bcd(&bcd), Some(-456));
}

#[test]
fn test_fbstp_large_positive() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDF, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 123456789.0);

    run_until_hlt(&mut vcpu).unwrap();
    let bcd = read_bcd(&mem, 0x3000);
    assert_eq!(parse_bcd(&bcd), Some(123456789));
}

#[test]
fn test_fbstp_large_negative() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDF, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, -987654321.0);

    run_until_hlt(&mut vcpu).unwrap();
    let bcd = read_bcd(&mem, 0x3000);
    assert_eq!(parse_bcd(&bcd), Some(-987654321));
}

#[test]
fn test_fbstp_rounds_down() {
    // 123.4 should round to 123
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDF, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 123.4);

    run_until_hlt(&mut vcpu).unwrap();
    let bcd = read_bcd(&mem, 0x3000);
    assert_eq!(parse_bcd(&bcd), Some(123));
}

#[test]
fn test_fbstp_rounds_up() {
    // 123.6 should round to 124
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDF, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 123.6);

    run_until_hlt(&mut vcpu).unwrap();
    let bcd = read_bcd(&mem, 0x3000);
    assert_eq!(parse_bcd(&bcd), Some(124));
}

#[test]
fn test_fbstp_pops_stack() {
    // FBSTP should pop the stack
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD 100
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD 200
        0xDF, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00, // FBSTP 200
        0xDF, 0x34, 0x25, 0x0A, 0x30, 0x00, 0x00, // FBSTP 100
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 100.0);
    write_f64(&mem, DATA_ADDR + 8, 200.0);

    run_until_hlt(&mut vcpu).unwrap();
    let bcd1 = read_bcd(&mem, 0x3000);
    let bcd2 = read_bcd(&mem, 0x300A);
    assert_eq!(parse_bcd(&bcd1), Some(200));
    assert_eq!(parse_bcd(&bcd2), Some(100));
}

// ============================================================================
// Round-trip tests
// ============================================================================

#[test]
fn test_fbld_fbstp_roundtrip_positive() {
    // FBLD followed by FBSTP should preserve value
    let code = [
        0xDF, 0x24, 0x25, 0x00, 0x20, 0x00, 0x00, // FBLD
        0xDF, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00, // FBSTP
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    let original_bcd = make_bcd(12345);
    write_bcd(&mem, DATA_ADDR, &original_bcd);

    run_until_hlt(&mut vcpu).unwrap();
    let result_bcd = read_bcd(&mem, 0x3000);
    assert_eq!(parse_bcd(&result_bcd), Some(12345));
}

#[test]
fn test_fbld_fbstp_roundtrip_negative() {
    let code = [
        0xDF, 0x24, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDF, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    let original_bcd = make_bcd(-67890);
    write_bcd(&mem, DATA_ADDR, &original_bcd);

    run_until_hlt(&mut vcpu).unwrap();
    let result_bcd = read_bcd(&mem, 0x3000);
    assert_eq!(parse_bcd(&result_bcd), Some(-67890));
}

#[test]
fn test_fbld_fbstp_roundtrip_zero() {
    let code = [
        0xDF, 0x24, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDF, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    let original_bcd = make_bcd(0);
    write_bcd(&mem, DATA_ADDR, &original_bcd);

    run_until_hlt(&mut vcpu).unwrap();
    let result_bcd = read_bcd(&mem, 0x3000);
    assert_eq!(parse_bcd(&result_bcd), Some(0));
}

#[test]
fn test_fbld_fbstp_roundtrip_large() {
    let code = [
        0xDF, 0x24, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDF, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    let original_bcd = make_bcd(999999999999);
    write_bcd(&mem, DATA_ADDR, &original_bcd);

    run_until_hlt(&mut vcpu).unwrap();
    let result_bcd = read_bcd(&mem, 0x3000);
    assert_eq!(parse_bcd(&result_bcd), Some(999999999999));
}

// ============================================================================
// Arithmetic integration tests
// ============================================================================

#[test]
fn test_fbld_arithmetic() {
    // FBLD two values and add them
    let code = [
        0xDF, 0x24, 0x25, 0x00, 0x20, 0x00, 0x00, // FBLD 100
        0xDF, 0x24, 0x25, 0x0A, 0x20, 0x00, 0x00, // FBLD 200
        0xDE, 0xC1, // FADDP
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP result
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_bcd(&mem, DATA_ADDR, &make_bcd(100));
    write_bcd(&mem, DATA_ADDR + 10, &make_bcd(200));

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 300.0);
}

#[test]
fn test_fbstp_after_arithmetic() {
    // Arithmetic followed by FBSTP
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD 50.0
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD 30.0
        0xDE, 0xC1, // FADDP (80.0)
        0xDF, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00, // FBSTP
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 50.0);
    write_f64(&mem, DATA_ADDR + 8, 30.0);

    run_until_hlt(&mut vcpu).unwrap();
    let bcd = read_bcd(&mem, 0x3000);
    assert_eq!(parse_bcd(&bcd), Some(80));
}

#[test]
fn test_fbld_fbstp_sequence() {
    // Sequential FBLD/FBSTP operations
    let code = [
        0xDF, 0x24, 0x25, 0x00, 0x20, 0x00, 0x00, // FBLD 111
        0xDF, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00, // FBSTP 111
        0xDF, 0x24, 0x25, 0x0A, 0x20, 0x00, 0x00, // FBLD 222
        0xDF, 0x34, 0x25, 0x0A, 0x30, 0x00, 0x00, // FBSTP 222
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_bcd(&mem, DATA_ADDR, &make_bcd(111));
    write_bcd(&mem, DATA_ADDR + 10, &make_bcd(222));

    run_until_hlt(&mut vcpu).unwrap();
    let bcd1 = read_bcd(&mem, 0x3000);
    let bcd2 = read_bcd(&mem, 0x300A);
    assert_eq!(parse_bcd(&bcd1), Some(111));
    assert_eq!(parse_bcd(&bcd2), Some(222));
}

// ============================================================================
// Edge cases
// ============================================================================

#[test]
fn test_fbstp_very_large() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDF, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 999999999999999.0);

    run_until_hlt(&mut vcpu).unwrap();
    let bcd = read_bcd(&mem, 0x3000);
    assert_eq!(parse_bcd(&bcd), Some(999999999999999));
}

#[test]
fn test_fbld_all_nines() {
    let code = [
        0xDF, 0x24, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_bcd(&mem, DATA_ADDR, &make_bcd(999999));

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 999999.0);
}

#[test]
fn test_fbstp_rounding_half() {
    // Test rounding of .5
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDF, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 99.5);

    run_until_hlt(&mut vcpu).unwrap();
    let bcd = read_bcd(&mem, 0x3000);
    // Should round to nearest even (100)
    assert_eq!(parse_bcd(&bcd), Some(100));
}

#[test]
fn test_fbstp_negative_rounding() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xDF, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, -99.7);

    run_until_hlt(&mut vcpu).unwrap();
    let bcd = read_bcd(&mem, 0x3000);
    assert_eq!(parse_bcd(&bcd), Some(-100));
}
