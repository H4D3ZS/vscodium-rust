//! Comprehensive tests for XLAT (Table Lookup Translation) instruction
//!
//! Tests XLAT/XLATB instruction in various scenarios.

use crate::common::*;

// ============================================================================
// XLAT/XLATB - Table Lookup Translation
// ============================================================================

#[test]
fn test_xlat_basic() {
    // XLAT translates AL using table at RBX
    let code = &[
        0xD7, // XLAT
        0xF4, // HLT
    ];
    let mut cpu = create_test_cpu(code);
    let mem = cpu.get_memory();

    // Set up translation table at 0x2000
    let table: [u8; 16] = [
        0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B, 0x1C, 0x1D, 0x1E,
        0x1F,
    ];
    mem.write_slice(&table, GuestAddress(0x2000)).unwrap();

    cpu.set_rbx(0x2000); // Table base
    cpu.set_rax(0x05); // Index (AL=5)

    run_test(&mut cpu);

    assert_eq!(cpu.get_rax() & 0xFF, 0x15, "XLAT: AL = table[5] = 0x15");
}

#[test]
fn test_xlat_index_zero() {
    // XLAT with AL=0 (first element)
    let code = &[
        0xD7, // XLAT
        0xF4, // HLT
    ];
    let mut cpu = create_test_cpu(code);
    let mem = cpu.get_memory();

    let table: [u8; 10] = [0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0x00, 0x11, 0x22, 0x33];
    mem.write_slice(&table, GuestAddress(0x3000)).unwrap();

    cpu.set_rbx(0x3000);
    cpu.set_rax(0x00);

    run_test(&mut cpu);

    assert_eq!(cpu.get_rax() & 0xFF, 0xAA, "XLAT: AL = table[0] = 0xAA");
}

#[test]
fn test_xlat_index_255() {
    // XLAT with AL=255 (maximum index)
    let code = &[
        0xD7, // XLAT
        0xF4, // HLT
    ];
    let mut cpu = create_test_cpu(code);
    let mem = cpu.get_memory();

    // Create 256-byte table
    let mut table = vec![0u8; 256];
    for i in 0..256 {
        table[i] = (i ^ 0xFF) as u8; // Complement pattern
    }
    mem.write_slice(&table, GuestAddress(0x2000)).unwrap();

    cpu.set_rbx(0x2000);
    cpu.set_rax(0xFF);

    run_test(&mut cpu);

    assert_eq!(cpu.get_rax() & 0xFF, 0x00, "XLAT: AL = table[255] = 0x00");
}

#[test]
fn test_xlat_ascii_uppercase_to_lowercase() {
    // XLAT for ASCII uppercase to lowercase conversion
    let code = &[
        0xD7, // XLAT
        0xF4, // HLT
    ];
    let mut cpu = create_test_cpu(code);
    let mem = cpu.get_memory();

    // Create translation table for ASCII characters
    let mut table = vec![0u8; 256];
    for i in 0..256 {
        table[i] = i as u8;
    }
    // Uppercase A-Z (65-90) -> lowercase a-z (97-122)
    for i in 65..=90 {
        table[i] = (i + 32) as u8;
    }
    mem.write_slice(&table, GuestAddress(0x4000)).unwrap();

    cpu.set_rbx(0x4000);
    cpu.set_rax(b'A' as u64); // 65

    run_test(&mut cpu);

    assert_eq!(cpu.get_rax() & 0xFF, b'a' as u64, "XLAT: 'A' -> 'a'");
}

#[test]
fn test_xlat_preserves_high_bits_of_rax() {
    // XLAT should only modify AL, not the rest of RAX
    let code = &[
        0xD7, // XLAT
        0xF4, // HLT
    ];
    let mut cpu = create_test_cpu(code);
    let mem = cpu.get_memory();

    let table: [u8; 10] = [0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99];
    mem.write_slice(&table, GuestAddress(0x2000)).unwrap();

    cpu.set_rbx(0x2000);
    cpu.set_rax(0x12345678_9ABCDE03); // AL=3

    run_test(&mut cpu);

    assert_eq!(
        cpu.get_rax(),
        0x12345678_9ABCDE33,
        "XLAT preserves high bits of RAX"
    );
}

#[test]
fn test_xlat_hex_digit_conversion() {
    // XLAT for hex digit to ASCII conversion
    let code = &[
        0xD7, // XLAT
        0xF4, // HLT
    ];
    let mut cpu = create_test_cpu(code);
    let mem = cpu.get_memory();

    // Hex digit to ASCII table: 0-9 -> '0'-'9', 10-15 -> 'A'-'F'
    let table: [u8; 16] = [
        b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'A', b'B', b'C', b'D', b'E',
        b'F',
    ];
    mem.write_slice(&table, GuestAddress(0x5000)).unwrap();

    cpu.set_rbx(0x5000);
    cpu.set_rax(0x0A); // 10 (hex A)

    run_test(&mut cpu);

    assert_eq!(cpu.get_rax() & 0xFF, b'A' as u64, "XLAT: 0x0A -> 'A'");
}

#[test]
fn test_xlat_identity_mapping() {
    // XLAT with identity mapping (table[i] = i)
    let code = &[
        0xD7, // XLAT
        0xF4, // HLT
    ];
    let mut cpu = create_test_cpu(code);
    let mem = cpu.get_memory();

    let mut table = vec![0u8; 256];
    for i in 0..256 {
        table[i] = i as u8;
    }
    mem.write_slice(&table, GuestAddress(0x2000)).unwrap();

    cpu.set_rbx(0x2000);
    cpu.set_rax(0x42);

    run_test(&mut cpu);

    assert_eq!(cpu.get_rax() & 0xFF, 0x42, "XLAT with identity mapping");
}
