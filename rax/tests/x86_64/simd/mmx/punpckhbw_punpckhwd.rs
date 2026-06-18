//! Tests for PUNPCKHBW and PUNPCKHWD instructions (MMX).
//!
//! PUNPCKHBW/PUNPCKHWD - Unpack High Data
//!
//! Unpacks and interleaves the high-order data elements (bytes or words) of the
//! destination and source operands into the destination operand.
//!
//! - PUNPCKHBW: Interleave high-order bytes from mm and mm/m64 into mm
//! - PUNPCKHWD: Interleave high-order words from mm and mm/m64 into mm
//!
//! Flags affected: None
//!
//! Reference: docs/punpckhbw:punpckhwd:punpckhdq:punpckhqdq.txt

use crate::common::*;
use vm_memory::GuestMemoryMmap;

// Helper to write 64-bit value to memory
fn write_mem_at_u64(mem: &GuestMemoryMmap, addr: u64, value: u64) {
    mem.write_slice(&value.to_le_bytes(), vm_memory::GuestAddress(addr))
        .unwrap();
}

// Helper to read 64-bit value from memory
fn read_mem_at_u64(mem: &GuestMemoryMmap, addr: u64) -> u64 {
    let mut buf = [0u8; 8];
    mem.read_slice(&mut buf, vm_memory::GuestAddress(addr))
        .unwrap();
    u64::from_le_bytes(buf)
}

// ============================================================================
// PUNPCKHBW mm, mm/m64 (opcode 0F 68 /r) - Interleave high-order bytes
// ============================================================================

#[test]
fn test_punpckhbw_mm_mm_basic() {
    // PUNPCKHBW MM0, MM1 - basic interleaving
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // MOVQ MM0, [0x2000]
        0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00, 0x00, // MOVQ MM1, [0x2008]
        0x0f, 0x68, 0xc1, // PUNPCKHBW MM0, MM1
        0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, // MOVQ [0x2010], MM0
        0xf4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    // MM0 = 0x0706050403020100 (high bytes: 07 06 05 04)
    write_mem_at_u64(&mem, 0x2000, 0x0706050403020100);
    // MM1 = 0x0F0E0D0C0B0A0908 (high bytes: 0F 0E 0D 0C)
    write_mem_at_u64(&mem, 0x2008, 0x0F0E0D0C0B0A0908);

    run_until_hlt(&mut vcpu).unwrap();

    // Result should interleave high bytes: 0F 07 0E 06 0D 05 0C 04
    let result = read_mem_at_u64(&mem, 0x2010);
    assert_eq!(result, 0x0F070E060D050C04, "PUNPCKHBW: basic interleaving");
}

#[test]
fn test_punpckhbw_mm_mm_zeros() {
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00,
        0x00, 0x0f, 0x68, 0xc1, 0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, 0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u64(&mem, 0x2000, 0x0000000000000000);
    write_mem_at_u64(&mem, 0x2008, 0x0000000000000000);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_mem_at_u64(&mem, 0x2010);
    assert_eq!(result, 0x0000000000000000, "PUNPCKHBW: all zeros");
}

#[test]
fn test_punpckhbw_mm_mm_ones() {
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00,
        0x00, 0x0f, 0x68, 0xc1, 0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, 0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u64(&mem, 0x2000, 0xFFFFFFFFFFFFFFFF);
    write_mem_at_u64(&mem, 0x2008, 0xFFFFFFFFFFFFFFFF);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_mem_at_u64(&mem, 0x2010);
    assert_eq!(result, 0xFFFFFFFFFFFFFFFF, "PUNPCKHBW: all ones");
}

#[test]
fn test_punpckhbw_mm_mm_alternating() {
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00,
        0x00, 0x0f, 0x68, 0xc1, 0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, 0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u64(&mem, 0x2000, 0xAAAAAAAAAAAAAAAA);
    write_mem_at_u64(&mem, 0x2008, 0x5555555555555555);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_mem_at_u64(&mem, 0x2010);
    // High bytes: AA AA AA AA and 55 55 55 55
    // Result: 55 AA 55 AA 55 AA 55 AA
    assert_eq!(result, 0x55AA55AA55AA55AA, "PUNPCKHBW: alternating pattern");
}

#[test]
fn test_punpckhbw_mm_m64() {
    // PUNPCKHBW MM2, [memory]
    let code = vec![
        0x0f, 0x6f, 0x14, 0x25, 0x00, 0x20, 0x00, 0x00, // MOVQ MM2, [0x2000]
        0x0f, 0x68, 0x14, 0x25, 0x08, 0x20, 0x00, 0x00, // PUNPCKHBW MM2, [0x2008]
        0x0f, 0x7f, 0x14, 0x25, 0x10, 0x20, 0x00, 0x00, // MOVQ [0x2010], MM2
        0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u64(&mem, 0x2000, 0x8877665544332211);
    write_mem_at_u64(&mem, 0x2008, 0xFFEEDDCCBBAAA099);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_mem_at_u64(&mem, 0x2010);
    // High bytes of MM2: 88 77 66 55
    // High bytes of mem: FF EE DD CC
    // Interleaved: FF 88 EE 77 DD 66 CC 55
    assert_eq!(result, 0xFF88EE77DD66CC55, "PUNPCKHBW: memory operand");
}

#[test]
fn test_punpckhbw_sequential_bytes() {
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00,
        0x00, 0x0f, 0x68, 0xc1, 0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, 0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u64(&mem, 0x2000, 0x0706050403020100);
    write_mem_at_u64(&mem, 0x2008, 0x0F0E0D0C0B0A0908);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_mem_at_u64(&mem, 0x2010);
    // High 4 bytes of dest: 07 06 05 04
    // High 4 bytes of src:  0F 0E 0D 0C
    // Result: 0F 07 0E 06 0D 05 0C 04
    assert_eq!(result, 0x0F070E060D050C04, "PUNPCKHBW: sequential bytes");
}

#[test]
fn test_punpckhbw_mm3_mm4() {
    let code = vec![
        0x0f, 0x6f, 0x1c, 0x25, 0x00, 0x20, 0x00, 0x00, // MOVQ MM3, [0x2000]
        0x0f, 0x6f, 0x24, 0x25, 0x08, 0x20, 0x00, 0x00, // MOVQ MM4, [0x2008]
        0x0f, 0x68, 0xdc, // PUNPCKHBW MM3, MM4
        0x0f, 0x7f, 0x1c, 0x25, 0x10, 0x20, 0x00, 0x00, // MOVQ [0x2010], MM3
        0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u64(&mem, 0x2000, 0x1111111111111111);
    write_mem_at_u64(&mem, 0x2008, 0x2222222222222222);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_mem_at_u64(&mem, 0x2010);
    assert_eq!(result, 0x2211221122112211, "PUNPCKHBW: MM3 with MM4");
}

#[test]
fn test_punpckhbw_zero_extension() {
    // Test zero extension use case (source all zeros)
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00,
        0x00, 0x0f, 0x68, 0xc1, 0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, 0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u64(&mem, 0x2000, 0xFF00FF00FF00FF00);
    write_mem_at_u64(&mem, 0x2008, 0x0000000000000000);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_mem_at_u64(&mem, 0x2010);
    // High bytes of dest: FF 00 FF 00
    // High bytes of src:  00 00 00 00
    // Result: 00 FF 00 00 00 FF 00 00
    assert_eq!(result, 0x00FF000000FF0000, "PUNPCKHBW: zero extension");
}

#[test]
fn test_punpckhbw_low_bits_ignored() {
    // Verify low bits are not used
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00,
        0x00, 0x0f, 0x68, 0xc1, 0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, 0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    // Low bytes different but high bytes same
    write_mem_at_u64(&mem, 0x2000, 0x12345678FFFFFFFF);
    write_mem_at_u64(&mem, 0x2008, 0x1234567800000000);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_mem_at_u64(&mem, 0x2010);
    // Both have same high 4 bytes: 12 34 56 78
    // Result: 12 12 34 34 56 56 78 78
    assert_eq!(result, 0x1212343456567878, "PUNPCKHBW: low bits ignored");
}

#[test]
fn test_punpckhbw_mm5_mm6() {
    let code = vec![
        0x0f, 0x6f, 0x2c, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0f, 0x6f, 0x34, 0x25, 0x08, 0x20, 0x00,
        0x00, 0x0f, 0x68, 0xee, 0x0f, 0x7f, 0x2c, 0x25, 0x10, 0x20, 0x00, 0x00, 0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u64(&mem, 0x2000, 0xDEADBEEF00000000);
    write_mem_at_u64(&mem, 0x2008, 0xCAFEBABE00000000);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_mem_at_u64(&mem, 0x2010);
    // High bytes: DE AD BE EF and CA FE BA BE
    // Result: CA DE FE AD BA BE BE EF
    assert_eq!(result, 0xCADEFEADBABEBEEF, "PUNPCKHBW: MM5 with MM6");
}

// ============================================================================
// PUNPCKHWD mm, mm/m64 (opcode 0F 69 /r) - Interleave high-order words
// ============================================================================

#[test]
fn test_punpckhwd_mm_mm_basic() {
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00,
        0x00, 0x0f, 0x69, 0xc1, // PUNPCKHWD MM0, MM1
        0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, 0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    // MM0 = 0x0004000300020001 (high words: 0004 0003)
    write_mem_at_u64(&mem, 0x2000, 0x0004000300020001);
    // MM1 = 0x0008000700060005 (high words: 0008 0007)
    write_mem_at_u64(&mem, 0x2008, 0x0008000700060005);

    run_until_hlt(&mut vcpu).unwrap();

    // Result: interleave high 2 words: 0008 0004 0007 0003
    let result = read_mem_at_u64(&mem, 0x2010);
    assert_eq!(result, 0x0008000400070003, "PUNPCKHWD: basic interleaving");
}

#[test]
fn test_punpckhwd_mm_mm_zeros() {
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00,
        0x00, 0x0f, 0x69, 0xc1, 0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, 0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u64(&mem, 0x2000, 0x0000000000000000);
    write_mem_at_u64(&mem, 0x2008, 0x0000000000000000);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_mem_at_u64(&mem, 0x2010);
    assert_eq!(result, 0x0000000000000000, "PUNPCKHWD: all zeros");
}

#[test]
fn test_punpckhwd_mm_mm_ones() {
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00,
        0x00, 0x0f, 0x69, 0xc1, 0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, 0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u64(&mem, 0x2000, 0xFFFFFFFFFFFFFFFF);
    write_mem_at_u64(&mem, 0x2008, 0xFFFFFFFFFFFFFFFF);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_mem_at_u64(&mem, 0x2010);
    assert_eq!(result, 0xFFFFFFFFFFFFFFFF, "PUNPCKHWD: all ones");
}

#[test]
fn test_punpckhwd_mm_mm_alternating() {
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00,
        0x00, 0x0f, 0x69, 0xc1, 0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, 0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u64(&mem, 0x2000, 0xAAAAAAAAAAAAAAAA);
    write_mem_at_u64(&mem, 0x2008, 0x5555555555555555);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_mem_at_u64(&mem, 0x2010);
    // High words: AAAA AAAA and 5555 5555
    // Result: 5555 AAAA 5555 AAAA
    assert_eq!(result, 0x5555AAAA5555AAAA, "PUNPCKHWD: alternating pattern");
}

#[test]
fn test_punpckhwd_mm_m64() {
    let code = vec![
        0x0f, 0x6f, 0x14, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0f, 0x69, 0x14, 0x25, 0x08, 0x20, 0x00,
        0x00, // PUNPCKHWD MM2, [0x2008]
        0x0f, 0x7f, 0x14, 0x25, 0x10, 0x20, 0x00, 0x00, 0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u64(&mem, 0x2000, 0x8888777766665555);
    write_mem_at_u64(&mem, 0x2008, 0xFFFFEEEEDDDDCCCC);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_mem_at_u64(&mem, 0x2010);
    // High words of MM2: 8888 7777
    // High words of mem: FFFF EEEE
    // Result: FFFF 8888 EEEE 7777
    assert_eq!(result, 0xFFFF8888EEEE7777, "PUNPCKHWD: memory operand");
}

#[test]
fn test_punpckhwd_sequential_words() {
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00,
        0x00, 0x0f, 0x69, 0xc1, 0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, 0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u64(&mem, 0x2000, 0x0004000300020001);
    write_mem_at_u64(&mem, 0x2008, 0x0008000700060005);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_mem_at_u64(&mem, 0x2010);
    // High words: 0004 0003 and 0008 0007
    // Result: 0008 0004 0007 0003
    assert_eq!(result, 0x0008000400070003, "PUNPCKHWD: sequential words");
}

#[test]
fn test_punpckhwd_mm3_mm7() {
    let code = vec![
        0x0f, 0x6f, 0x1c, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0f, 0x6f, 0x3c, 0x25, 0x08, 0x20, 0x00,
        0x00, 0x0f, 0x69, 0xdf, // PUNPCKHWD MM3, MM7
        0x0f, 0x7f, 0x1c, 0x25, 0x10, 0x20, 0x00, 0x00, 0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u64(&mem, 0x2000, 0x1111111122222222);
    write_mem_at_u64(&mem, 0x2008, 0x3333333344444444);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_mem_at_u64(&mem, 0x2010);
    // High words: 1111 1111 and 3333 3333
    // Result: 3333 1111 3333 1111
    assert_eq!(result, 0x3333111133331111, "PUNPCKHWD: MM3 with MM7");
}

#[test]
fn test_punpckhwd_zero_extension() {
    // Test zero extension use case
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00,
        0x00, 0x0f, 0x69, 0xc1, 0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, 0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u64(&mem, 0x2000, 0xFFFF0000FFFF0000);
    write_mem_at_u64(&mem, 0x2008, 0x0000000000000000);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_mem_at_u64(&mem, 0x2010);
    // High words: FFFF 0000 and 0000 0000
    // Result: 0000 FFFF 0000 0000
    assert_eq!(result, 0x0000FFFF00000000, "PUNPCKHWD: zero extension");
}

#[test]
fn test_punpckhwd_low_words_ignored() {
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00,
        0x00, 0x0f, 0x69, 0xc1, 0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, 0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u64(&mem, 0x2000, 0x12345678FFFFFFFF);
    write_mem_at_u64(&mem, 0x2008, 0x1234567800000000);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_mem_at_u64(&mem, 0x2010);
    // Both have high words: 1234 5678
    // Result: 1234 1234 5678 5678
    assert_eq!(result, 0x1234123456785678, "PUNPCKHWD: low words ignored");
}

#[test]
fn test_punpckhwd_mm4_mm5() {
    let code = vec![
        0x0f, 0x6f, 0x24, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0f, 0x6f, 0x2c, 0x25, 0x08, 0x20, 0x00,
        0x00, 0x0f, 0x69, 0xe5, // PUNPCKHWD MM4, MM5
        0x0f, 0x7f, 0x24, 0x25, 0x10, 0x20, 0x00, 0x00, 0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u64(&mem, 0x2000, 0xDEADBEEF00000000);
    write_mem_at_u64(&mem, 0x2008, 0xCAFEBABE00000000);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_mem_at_u64(&mem, 0x2010);
    // High words: DEAD BEEF and CAFE BABE
    // Result: CAFE DEAD BABE BEEF
    assert_eq!(result, 0xCAFEDEADBABEBEEF, "PUNPCKHWD: MM4 with MM5");
}

// ============================================================================
// Additional edge cases and comprehensive tests
// ============================================================================

#[test]
fn test_punpckhbw_single_byte_pattern() {
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00,
        0x00, 0x0f, 0x68, 0xc1, 0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, 0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u64(&mem, 0x2000, 0x0101010100000000);
    write_mem_at_u64(&mem, 0x2008, 0x0202020200000000);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_mem_at_u64(&mem, 0x2010);
    assert_eq!(result, 0x0201020102010201, "PUNPCKHBW: repeating bytes");
}

#[test]
fn test_punpckhwd_single_word_pattern() {
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00,
        0x00, 0x0f, 0x69, 0xc1, 0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, 0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u64(&mem, 0x2000, 0x1111111100000000);
    write_mem_at_u64(&mem, 0x2008, 0x2222222200000000);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_mem_at_u64(&mem, 0x2010);
    assert_eq!(result, 0x2222111122221111, "PUNPCKHWD: repeating words");
}

#[test]
fn test_punpckhbw_max_min_values() {
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00,
        0x00, 0x0f, 0x68, 0xc1, 0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, 0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u64(&mem, 0x2000, 0xFF00FF0000000000);
    write_mem_at_u64(&mem, 0x2008, 0x00FF00FF00000000);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_mem_at_u64(&mem, 0x2010);
    // High bytes: FF 00 FF 00 and 00 FF 00 FF
    // Result: 00 FF FF 00 00 FF FF 00
    assert_eq!(result, 0x00FFFF0000FFFF00, "PUNPCKHBW: max/min values");
}

#[test]
fn test_punpckhwd_max_min_values() {
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00,
        0x00, 0x0f, 0x69, 0xc1, 0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, 0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u64(&mem, 0x2000, 0xFFFF000000000000);
    write_mem_at_u64(&mem, 0x2008, 0x0000FFFF00000000);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_mem_at_u64(&mem, 0x2010);
    // High words: FFFF 0000 and 0000 FFFF
    // Result: 0000 FFFF FFFF 0000
    assert_eq!(result, 0x0000FFFFFFFF0000, "PUNPCKHWD: max/min values");
}

#[test]
fn test_punpckhbw_mixed_registers_mm2_mm3() {
    let code = vec![
        0x0f, 0x6f, 0x14, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0f, 0x6f, 0x1c, 0x25, 0x08, 0x20, 0x00,
        0x00, 0x0f, 0x68, 0xd3, 0x0f, 0x7f, 0x14, 0x25, 0x10, 0x20, 0x00, 0x00, 0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u64(&mem, 0x2000, 0xBBBBBBBB00000000);
    write_mem_at_u64(&mem, 0x2008, 0xDDDDDDDD00000000);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_mem_at_u64(&mem, 0x2010);
    assert_eq!(result, 0xDDBBDDBBDDBBDDBB, "PUNPCKHBW: MM2 with MM3");
}

#[test]
fn test_punpckhwd_mixed_registers_mm6_mm7() {
    let code = vec![
        0x0f, 0x6f, 0x34, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0f, 0x6f, 0x3c, 0x25, 0x08, 0x20, 0x00,
        0x00, 0x0f, 0x69, 0xf7, 0x0f, 0x7f, 0x34, 0x25, 0x10, 0x20, 0x00, 0x00, 0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u64(&mem, 0x2000, 0xBBBBBBBB00000000);
    write_mem_at_u64(&mem, 0x2008, 0xDDDDDDDD00000000);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_mem_at_u64(&mem, 0x2010);
    assert_eq!(result, 0xDDDDBBBBDDDDBBBB, "PUNPCKHWD: MM6 with MM7");
}

#[test]
fn test_punpckhbw_incremental_values() {
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00,
        0x00, 0x0f, 0x68, 0xc1, 0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, 0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u64(&mem, 0x2000, 0x1011121300000000);
    write_mem_at_u64(&mem, 0x2008, 0x1415161700000000);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_mem_at_u64(&mem, 0x2010);
    // High bytes: 10 11 12 13 and 14 15 16 17
    // Result: 14 10 15 11 16 12 17 13
    assert_eq!(result, 0x1410151116121713, "PUNPCKHBW: incremental values");
}

#[test]
fn test_punpckhwd_incremental_values() {
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00,
        0x00, 0x0f, 0x69, 0xc1, 0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, 0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u64(&mem, 0x2000, 0x1011121300000000);
    write_mem_at_u64(&mem, 0x2008, 0x1415161700000000);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_mem_at_u64(&mem, 0x2010);
    // High words: 1011 1213 and 1415 1617
    // Result: 1415 1011 1617 1213
    assert_eq!(result, 0x1415101116171213, "PUNPCKHWD: incremental values");
}

#[test]
fn test_punpckhbw_byte_boundary() {
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00,
        0x00, 0x0f, 0x68, 0xc1, 0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, 0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u64(&mem, 0x2000, 0x7F807F8000000000);
    write_mem_at_u64(&mem, 0x2008, 0xFF00FF0100000000);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_mem_at_u64(&mem, 0x2010);
    // High bytes: 7F 80 7F 80 and FF 00 FF 01
    // Result: FF 7F 00 80 FF 7F 01 80
    assert_eq!(result, 0xFF7F0080FF7F0180, "PUNPCKHBW: byte boundaries");
}

#[test]
fn test_punpckhwd_word_boundary() {
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00,
        0x00, 0x0f, 0x69, 0xc1, 0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, 0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u64(&mem, 0x2000, 0x7FFF800000000000);
    write_mem_at_u64(&mem, 0x2008, 0xFFFF000100000000);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_mem_at_u64(&mem, 0x2010);
    // High words: 7FFF 8000 and FFFF 0001
    // Result: FFFF 7FFF 0001 8000
    assert_eq!(result, 0xFFFF7FFF00018000, "PUNPCKHWD: word boundaries");
}

#[test]
fn test_punpckhbw_register_independence() {
    // Verify operation doesn't affect other registers
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00,
        0x00, 0x0f, 0x6f, 0x14, 0x25, 0x10, 0x20, 0x00, 0x00, 0x0f, 0x68,
        0xc1, // PUNPCKHBW MM0, MM1
        0x0f, 0x7f, 0x04, 0x25, 0x18, 0x20, 0x00, 0x00, 0x0f, 0x7f, 0x14, 0x25, 0x20, 0x20, 0x00,
        0x00, 0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u64(&mem, 0x2000, 0x1111111100000000);
    write_mem_at_u64(&mem, 0x2008, 0x2222222200000000);
    write_mem_at_u64(&mem, 0x2010, 0x3333333333333333);

    run_until_hlt(&mut vcpu).unwrap();

    let mm0_result = read_mem_at_u64(&mem, 0x2018);
    assert_eq!(mm0_result, 0x2211221122112211, "PUNPCKHBW: MM0 result");

    let mm2_result = read_mem_at_u64(&mem, 0x2020);
    assert_eq!(mm2_result, 0x3333333333333333, "PUNPCKHBW: MM2 unchanged");
}

#[test]
fn test_punpckhwd_register_independence() {
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00,
        0x00, 0x0f, 0x6f, 0x14, 0x25, 0x10, 0x20, 0x00, 0x00, 0x0f, 0x69,
        0xc1, // PUNPCKHWD MM0, MM1
        0x0f, 0x7f, 0x04, 0x25, 0x18, 0x20, 0x00, 0x00, 0x0f, 0x7f, 0x14, 0x25, 0x20, 0x20, 0x00,
        0x00, 0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u64(&mem, 0x2000, 0x1111111100000000);
    write_mem_at_u64(&mem, 0x2008, 0x2222222200000000);
    write_mem_at_u64(&mem, 0x2010, 0x3333333333333333);

    run_until_hlt(&mut vcpu).unwrap();

    let mm0_result = read_mem_at_u64(&mem, 0x2018);
    assert_eq!(mm0_result, 0x2222111122221111, "PUNPCKHWD: MM0 result");

    let mm2_result = read_mem_at_u64(&mem, 0x2020);
    assert_eq!(mm2_result, 0x3333333333333333, "PUNPCKHWD: MM2 unchanged");
}

#[test]
fn test_punpckhbw_memory_alignment() {
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0f, 0x68, 0x04, 0x25, 0x08, 0x20, 0x00,
        0x00, // PUNPCKHBW MM0, [0x2008]
        0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, 0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u64(&mem, 0x2000, 0x0123456700000000);
    write_mem_at_u64(&mem, 0x2008, 0x89ABCDEF00000000);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_mem_at_u64(&mem, 0x2010);
    // High bytes: 01 23 45 67 and 89 AB CD EF
    // Result: 89 01 AB 23 CD 45 EF 67
    assert_eq!(result, 0x8901AB23CD45EF67, "PUNPCKHBW: memory alignment");
}

#[test]
fn test_punpckhwd_memory_alignment() {
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0f, 0x69, 0x04, 0x25, 0x08, 0x20, 0x00,
        0x00, // PUNPCKHWD MM0, [0x2008]
        0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, 0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u64(&mem, 0x2000, 0x0123456700000000);
    write_mem_at_u64(&mem, 0x2008, 0x89ABCDEF00000000);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_mem_at_u64(&mem, 0x2010);
    // High words: 0123 4567 and 89AB CDEF
    // Result: 89AB 0123 CDEF 4567
    assert_eq!(result, 0x89AB0123CDEF4567, "PUNPCKHWD: memory alignment");
}
