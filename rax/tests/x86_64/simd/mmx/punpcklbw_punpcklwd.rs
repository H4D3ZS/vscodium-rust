//! Tests for PUNPCKLBW and PUNPCKLWD instructions (MMX).
//!
//! PUNPCKLBW/PUNPCKLWD - Unpack Low Data
//!
//! Unpacks and interleaves the low-order data elements (bytes or words) of the
//! destination and source operands into the destination operand.
//!
//! - PUNPCKLBW: Interleave low-order bytes from mm and mm/m32 into mm
//! - PUNPCKLWD: Interleave low-order words from mm and mm/m32 into mm
//!
//! Flags affected: None
//!
//! Reference: docs/punpcklbw:punpcklwd:punpckldq:punpcklqdq.txt

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
// PUNPCKLBW mm, mm/m32 (opcode 0F 60 /r) - Interleave low-order bytes
// ============================================================================

#[test]
fn test_punpcklbw_mm_mm_basic() {
    // PUNPCKLBW MM0, MM1 - basic interleaving
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // MOVQ MM0, [0x2000]
        0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00, 0x00, // MOVQ MM1, [0x2008]
        0x0f, 0x60, 0xc1, // PUNPCKLBW MM0, MM1
        0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, // MOVQ [0x2010], MM0
        0xf4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    // MM0 = 0x0706050403020100 (low bytes: 03 02 01 00)
    write_mem_at_u64(&mem, 0x2000, 0x0706050403020100);
    // MM1 = 0x0F0E0D0C0B0A0908 (low bytes: 0B 0A 09 08)
    write_mem_at_u64(&mem, 0x2008, 0x0F0E0D0C0B0A0908);

    run_until_hlt(&mut vcpu).unwrap();

    // Result should interleave: 0B 03 0A 02 09 01 08 00
    let result = read_mem_at_u64(&mem, 0x2010);
    assert_eq!(result, 0x0B030A020901_0800, "PUNPCKLBW: basic interleaving");
}

#[test]
fn test_punpcklbw_mm_mm_zeros() {
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00,
        0x00, 0x0f, 0x60, 0xc1, 0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, 0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u64(&mem, 0x2000, 0x0000000000000000);
    write_mem_at_u64(&mem, 0x2008, 0x0000000000000000);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_mem_at_u64(&mem, 0x2010);
    assert_eq!(result, 0x0000000000000000, "PUNPCKLBW: all zeros");
}

#[test]
fn test_punpcklbw_mm_mm_ones() {
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00,
        0x00, 0x0f, 0x60, 0xc1, 0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, 0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u64(&mem, 0x2000, 0xFFFFFFFFFFFFFFFF);
    write_mem_at_u64(&mem, 0x2008, 0xFFFFFFFFFFFFFFFF);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_mem_at_u64(&mem, 0x2010);
    assert_eq!(result, 0xFFFFFFFFFFFFFFFF, "PUNPCKLBW: all ones");
}

#[test]
fn test_punpcklbw_mm_mm_alternating() {
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00,
        0x00, 0x0f, 0x60, 0xc1, 0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, 0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    // Dest low bytes: AA AA AA AA
    write_mem_at_u64(&mem, 0x2000, 0xAAAAAAAAAAAAAAAA);
    // Src low bytes: 55 55 55 55
    write_mem_at_u64(&mem, 0x2008, 0x5555555555555555);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_mem_at_u64(&mem, 0x2010);
    // Interleaved: 55 AA 55 AA 55 AA 55 AA
    assert_eq!(result, 0x55AA55AA55AA55AA, "PUNPCKLBW: alternating pattern");
}

#[test]
fn test_punpcklbw_mm_m32() {
    // PUNPCKLBW MM2, [memory] - only low 32 bits of memory accessed
    let code = vec![
        0x0f, 0x6f, 0x14, 0x25, 0x00, 0x20, 0x00, 0x00, // MOVQ MM2, [0x2000]
        0x0f, 0x60, 0x14, 0x25, 0x08, 0x20, 0x00, 0x00, // PUNPCKLBW MM2, [0x2008]
        0x0f, 0x7f, 0x14, 0x25, 0x10, 0x20, 0x00, 0x00, // MOVQ [0x2010], MM2
        0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u64(&mem, 0x2000, 0x8877665544332211);
    write_mem_at_u64(&mem, 0x2008, 0xFFFFFFFFCCBBAA99);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_mem_at_u64(&mem, 0x2010);
    // Low 32 bits of MM2: 44 33 22 11
    // Low 32 bits of mem: CC BB AA 99
    // Interleaved: CC 44 BB 33 AA 22 99 11
    assert_eq!(result, 0xCC44BB33AA229911, "PUNPCKLBW: memory operand");
}

#[test]
fn test_punpcklbw_sequential_bytes() {
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00,
        0x00, 0x0f, 0x60, 0xc1, 0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, 0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u64(&mem, 0x2000, 0x0706050403020100);
    write_mem_at_u64(&mem, 0x2008, 0x0F0E0D0C0B0A0908);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_mem_at_u64(&mem, 0x2010);
    // Low 4 bytes of dest: 03 02 01 00
    // Low 4 bytes of src:  0B 0A 09 08
    // Result: 0B 03 0A 02 09 01 08 00
    assert_eq!(result, 0x0B030A0209010800, "PUNPCKLBW: sequential bytes");
}

#[test]
fn test_punpcklbw_mm3_mm4() {
    let code = vec![
        0x0f, 0x6f, 0x1c, 0x25, 0x00, 0x20, 0x00, 0x00, // MOVQ MM3, [0x2000]
        0x0f, 0x6f, 0x24, 0x25, 0x08, 0x20, 0x00, 0x00, // MOVQ MM4, [0x2008]
        0x0f, 0x60, 0xdc, // PUNPCKLBW MM3, MM4
        0x0f, 0x7f, 0x1c, 0x25, 0x10, 0x20, 0x00, 0x00, // MOVQ [0x2010], MM3
        0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u64(&mem, 0x2000, 0x1111111111111111);
    write_mem_at_u64(&mem, 0x2008, 0x2222222222222222);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_mem_at_u64(&mem, 0x2010);
    assert_eq!(result, 0x2211221122112211, "PUNPCKLBW: MM3 with MM4");
}

#[test]
fn test_punpcklbw_zero_extension() {
    // Test zero extension use case (source all zeros)
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00,
        0x00, 0x0f, 0x60, 0xc1, 0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, 0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u64(&mem, 0x2000, 0xFF00FF00FF00FF00);
    write_mem_at_u64(&mem, 0x2008, 0x0000000000000000);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_mem_at_u64(&mem, 0x2010);
    // Low bytes of dest: FF 00 FF 00
    // Low bytes of src:  00 00 00 00
    // Result: 00 FF 00 00 00 FF 00 00
    assert_eq!(result, 0x00FF000000FF0000, "PUNPCKLBW: zero extension");
}

#[test]
fn test_punpcklbw_high_bits_ignored() {
    // Verify high bits are not used
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00,
        0x00, 0x0f, 0x60, 0xc1, 0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, 0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    // High bytes different but low bytes same
    write_mem_at_u64(&mem, 0x2000, 0xFFFFFFFF12345678);
    write_mem_at_u64(&mem, 0x2008, 0x0000000012345678);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_mem_at_u64(&mem, 0x2010);
    // Both have same low 4 bytes: 78 56 34 12
    // Result: 12 12 34 34 56 56 78 78
    assert_eq!(result, 0x1212343456567878, "PUNPCKLBW: high bits ignored");
}

#[test]
fn test_punpcklbw_mm5_mm6() {
    let code = vec![
        0x0f, 0x6f, 0x2c, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0f, 0x6f, 0x34, 0x25, 0x08, 0x20, 0x00,
        0x00, 0x0f, 0x60, 0xee, 0x0f, 0x7f, 0x2c, 0x25, 0x10, 0x20, 0x00, 0x00, 0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u64(&mem, 0x2000, 0x00000000DEADBEEF);
    write_mem_at_u64(&mem, 0x2008, 0x00000000CAFEBABE);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_mem_at_u64(&mem, 0x2010);
    // Low bytes: EF BE AD DE and BE BA FE CA
    // Result: CA DE FE AD BA BE BE EF
    assert_eq!(result, 0xCADEFEADBABEBEEF, "PUNPCKLBW: MM5 with MM6");
}

// ============================================================================
// PUNPCKLWD mm, mm/m32 (opcode 0F 61 /r) - Interleave low-order words
// ============================================================================

#[test]
fn test_punpcklwd_mm_mm_basic() {
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00,
        0x00, 0x0f, 0x61, 0xc1, // PUNPCKLWD MM0, MM1
        0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, 0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    // MM0 = 0x0003000200010000 (low words: 0002 0001)
    write_mem_at_u64(&mem, 0x2000, 0x0004000300020001);
    // MM1 = 0x0007000600050004 (low words: 0006 0005)
    write_mem_at_u64(&mem, 0x2008, 0x0008000700060005);

    run_until_hlt(&mut vcpu).unwrap();

    // Result: interleave low 2 words: 0006 0002 0005 0001
    let result = read_mem_at_u64(&mem, 0x2010);
    assert_eq!(result, 0x0006000200050001, "PUNPCKLWD: basic interleaving");
}

#[test]
fn test_punpcklwd_mm_mm_zeros() {
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00,
        0x00, 0x0f, 0x61, 0xc1, 0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, 0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u64(&mem, 0x2000, 0x0000000000000000);
    write_mem_at_u64(&mem, 0x2008, 0x0000000000000000);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_mem_at_u64(&mem, 0x2010);
    assert_eq!(result, 0x0000000000000000, "PUNPCKLWD: all zeros");
}

#[test]
fn test_punpcklwd_mm_mm_ones() {
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00,
        0x00, 0x0f, 0x61, 0xc1, 0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, 0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u64(&mem, 0x2000, 0xFFFFFFFFFFFFFFFF);
    write_mem_at_u64(&mem, 0x2008, 0xFFFFFFFFFFFFFFFF);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_mem_at_u64(&mem, 0x2010);
    assert_eq!(result, 0xFFFFFFFFFFFFFFFF, "PUNPCKLWD: all ones");
}

#[test]
fn test_punpcklwd_mm_mm_alternating() {
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00,
        0x00, 0x0f, 0x61, 0xc1, 0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, 0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u64(&mem, 0x2000, 0xAAAAAAAAAAAAAAAA);
    write_mem_at_u64(&mem, 0x2008, 0x5555555555555555);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_mem_at_u64(&mem, 0x2010);
    // Low words: AAAA AAAA and 5555 5555
    // Result: 5555 AAAA 5555 AAAA
    assert_eq!(result, 0x5555AAAA5555AAAA, "PUNPCKLWD: alternating pattern");
}

#[test]
fn test_punpcklwd_mm_m32() {
    let code = vec![
        0x0f, 0x6f, 0x14, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0f, 0x61, 0x14, 0x25, 0x08, 0x20, 0x00,
        0x00, // PUNPCKLWD MM2, [0x2008]
        0x0f, 0x7f, 0x14, 0x25, 0x10, 0x20, 0x00, 0x00, 0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u64(&mem, 0x2000, 0x8888777766665555);
    write_mem_at_u64(&mem, 0x2008, 0xFFFFFFFFCCCCBBBB);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_mem_at_u64(&mem, 0x2010);
    // Low words of MM2: 7777 6666
    // Low words of mem: CCCC BBBB
    // Result: CCCC 6666 BBBB 5555
    assert_eq!(result, 0xCCCC6666BBBB5555, "PUNPCKLWD: memory operand");
}

#[test]
fn test_punpcklwd_sequential_words() {
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00,
        0x00, 0x0f, 0x61, 0xc1, 0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, 0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u64(&mem, 0x2000, 0x0004000300020001);
    write_mem_at_u64(&mem, 0x2008, 0x0008000700060005);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_mem_at_u64(&mem, 0x2010);
    // Low words: 0002 0001 and 0006 0005
    // Result: 0006 0002 0005 0001
    assert_eq!(result, 0x0006000200050001, "PUNPCKLWD: sequential words");
}

#[test]
fn test_punpcklwd_mm3_mm7() {
    let code = vec![
        0x0f, 0x6f, 0x1c, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0f, 0x6f, 0x3c, 0x25, 0x08, 0x20, 0x00,
        0x00, 0x0f, 0x61, 0xdf, // PUNPCKLWD MM3, MM7
        0x0f, 0x7f, 0x1c, 0x25, 0x10, 0x20, 0x00, 0x00, 0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u64(&mem, 0x2000, 0x1111111122222222);
    write_mem_at_u64(&mem, 0x2008, 0x3333333344444444);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_mem_at_u64(&mem, 0x2010);
    // Low words: 1111 2222 and 3333 4444
    // Result: 4444 2222 4444 2222
    assert_eq!(result, 0x4444222244442222, "PUNPCKLWD: MM3 with MM7");
}

#[test]
fn test_punpcklwd_zero_extension() {
    // Test zero extension use case
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00,
        0x00, 0x0f, 0x61, 0xc1, 0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, 0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u64(&mem, 0x2000, 0xFFFF0000FFFF0000);
    write_mem_at_u64(&mem, 0x2008, 0x0000000000000000);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_mem_at_u64(&mem, 0x2010);
    // Low words: 0000 FFFF and 0000 0000
    // Result: 0000 FFFF 0000 0000
    assert_eq!(result, 0x0000FFFF00000000, "PUNPCKLWD: zero extension");
}

#[test]
fn test_punpcklwd_high_words_ignored() {
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00,
        0x00, 0x0f, 0x61, 0xc1, 0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, 0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u64(&mem, 0x2000, 0xFFFFFFFF12345678);
    write_mem_at_u64(&mem, 0x2008, 0x0000000012345678);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_mem_at_u64(&mem, 0x2010);
    // Both have low words: 1234 5678
    // Result: 1234 1234 5678 5678
    assert_eq!(result, 0x1234123456785678, "PUNPCKLWD: high words ignored");
}

#[test]
fn test_punpcklwd_mm4_mm5() {
    let code = vec![
        0x0f, 0x6f, 0x24, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0f, 0x6f, 0x2c, 0x25, 0x08, 0x20, 0x00,
        0x00, 0x0f, 0x61, 0xe5, // PUNPCKLWD MM4, MM5
        0x0f, 0x7f, 0x24, 0x25, 0x10, 0x20, 0x00, 0x00, 0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u64(&mem, 0x2000, 0x00000000DEADBEEF);
    write_mem_at_u64(&mem, 0x2008, 0x00000000CAFEBABE);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_mem_at_u64(&mem, 0x2010);
    // Low words: DEAD BEEF and CAFE BABE
    // Result: CAFE DEAD BABE BEEF
    assert_eq!(result, 0xCAFEDEADBABEBEEF, "PUNPCKLWD: MM4 with MM5");
}

// ============================================================================
// Additional edge cases and comprehensive tests
// ============================================================================

#[test]
fn test_punpcklbw_single_byte_pattern() {
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00,
        0x00, 0x0f, 0x60, 0xc1, 0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, 0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u64(&mem, 0x2000, 0x0000000001010101);
    write_mem_at_u64(&mem, 0x2008, 0x0000000002020202);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_mem_at_u64(&mem, 0x2010);
    assert_eq!(result, 0x0201020102010201, "PUNPCKLBW: repeating bytes");
}

#[test]
fn test_punpcklwd_single_word_pattern() {
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00,
        0x00, 0x0f, 0x61, 0xc1, 0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, 0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u64(&mem, 0x2000, 0x0000000011111111);
    write_mem_at_u64(&mem, 0x2008, 0x0000000022222222);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_mem_at_u64(&mem, 0x2010);
    assert_eq!(result, 0x2222111122221111, "PUNPCKLWD: repeating words");
}

#[test]
fn test_punpcklbw_max_min_values() {
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00,
        0x00, 0x0f, 0x60, 0xc1, 0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, 0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u64(&mem, 0x2000, 0x00000000FF00FF00);
    write_mem_at_u64(&mem, 0x2008, 0x0000000000FF00FF);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_mem_at_u64(&mem, 0x2010);
    // Low bytes: FF 00 FF 00 and 00 FF 00 FF
    // Result: 00 FF FF 00 00 FF FF 00
    assert_eq!(result, 0x00FFFF0000FFFF00, "PUNPCKLBW: max/min values");
}

#[test]
fn test_punpcklwd_max_min_values() {
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00,
        0x00, 0x0f, 0x61, 0xc1, 0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, 0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u64(&mem, 0x2000, 0x00000000FFFF0000);
    write_mem_at_u64(&mem, 0x2008, 0x000000000000FFFF);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_mem_at_u64(&mem, 0x2010);
    // Low words: FFFF 0000 and 0000 FFFF
    // Result: 0000 FFFF FFFF 0000
    assert_eq!(result, 0x0000FFFFFFFF0000, "PUNPCKLWD: max/min values");
}

#[test]
fn test_punpcklbw_mixed_registers_mm2_mm3() {
    let code = vec![
        0x0f, 0x6f, 0x14, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0f, 0x6f, 0x1c, 0x25, 0x08, 0x20, 0x00,
        0x00, 0x0f, 0x60, 0xd3, 0x0f, 0x7f, 0x14, 0x25, 0x10, 0x20, 0x00, 0x00, 0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u64(&mem, 0x2000, 0xAAAAAAAABBBBBBBB);
    write_mem_at_u64(&mem, 0x2008, 0xCCCCCCCCDDDDDDDD);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_mem_at_u64(&mem, 0x2010);
    assert_eq!(result, 0xDDBBDDBBDDBBDDBB, "PUNPCKLBW: MM2 with MM3");
}

#[test]
fn test_punpcklwd_mixed_registers_mm6_mm7() {
    let code = vec![
        0x0f, 0x6f, 0x34, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0f, 0x6f, 0x3c, 0x25, 0x08, 0x20, 0x00,
        0x00, 0x0f, 0x61, 0xf7, 0x0f, 0x7f, 0x34, 0x25, 0x10, 0x20, 0x00, 0x00, 0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u64(&mem, 0x2000, 0xAAAAAAAABBBBBBBB);
    write_mem_at_u64(&mem, 0x2008, 0xCCCCCCCCDDDDDDDD);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_mem_at_u64(&mem, 0x2010);
    assert_eq!(result, 0xDDDDBBBBDDDDBBBB, "PUNPCKLWD: MM6 with MM7");
}

#[test]
fn test_punpcklbw_incremental_values() {
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00,
        0x00, 0x0f, 0x60, 0xc1, 0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, 0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u64(&mem, 0x2000, 0xFFFFFFFF10111213);
    write_mem_at_u64(&mem, 0x2008, 0xFFFFFFFF14151617);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_mem_at_u64(&mem, 0x2010);
    // Low bytes: 10 11 12 13 and 14 15 16 17
    // Result: 14 10 15 11 16 12 17 13
    assert_eq!(result, 0x1410151116121713, "PUNPCKLBW: incremental values");
}

#[test]
fn test_punpcklwd_incremental_values() {
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00,
        0x00, 0x0f, 0x61, 0xc1, 0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, 0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u64(&mem, 0x2000, 0xFFFFFFFF10111213);
    write_mem_at_u64(&mem, 0x2008, 0xFFFFFFFF14151617);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_mem_at_u64(&mem, 0x2010);
    // Low words: 1011 1213 and 1415 1617
    // Result: 1415 1011 1617 1213
    assert_eq!(result, 0x1415101116171213, "PUNPCKLWD: incremental values");
}

#[test]
fn test_punpcklbw_byte_boundary() {
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00,
        0x00, 0x0f, 0x60, 0xc1, 0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, 0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u64(&mem, 0x2000, 0x000000007F807F80);
    write_mem_at_u64(&mem, 0x2008, 0x00000000FF00FF01);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_mem_at_u64(&mem, 0x2010);
    // Low bytes: 7F 80 7F 80 and FF 00 FF 01
    // Result: FF 7F 00 80 FF 7F 01 80
    assert_eq!(result, 0xFF7F0080FF7F0180, "PUNPCKLBW: byte boundaries");
}

#[test]
fn test_punpcklwd_word_boundary() {
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00,
        0x00, 0x0f, 0x61, 0xc1, 0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, 0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u64(&mem, 0x2000, 0x000000007FFF8000);
    write_mem_at_u64(&mem, 0x2008, 0x00000000FFFF0001);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_mem_at_u64(&mem, 0x2010);
    // Low words: 7FFF 8000 and FFFF 0001
    // Result: FFFF 7FFF 0001 8000
    assert_eq!(result, 0xFFFF7FFF00018000, "PUNPCKLWD: word boundaries");
}

#[test]
fn test_punpcklbw_register_independence() {
    // Verify operation doesn't affect other registers
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // MOVQ MM0, [0x2000]
        0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00, 0x00, // MOVQ MM1, [0x2008]
        0x0f, 0x6f, 0x14, 0x25, 0x10, 0x20, 0x00, 0x00, // MOVQ MM2, [0x2010]
        0x0f, 0x60, 0xc1, // PUNPCKLBW MM0, MM1
        0x0f, 0x7f, 0x04, 0x25, 0x18, 0x20, 0x00, 0x00, // MOVQ [0x2018], MM0
        0x0f, 0x7f, 0x14, 0x25, 0x20, 0x20, 0x00, 0x00, // MOVQ [0x2020], MM2
        0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u64(&mem, 0x2000, 0x1111111111111111);
    write_mem_at_u64(&mem, 0x2008, 0x2222222222222222);
    write_mem_at_u64(&mem, 0x2010, 0x3333333333333333);

    run_until_hlt(&mut vcpu).unwrap();

    // MM0 was modified
    let mm0_result = read_mem_at_u64(&mem, 0x2018);
    assert_eq!(mm0_result, 0x2211221122112211, "PUNPCKLBW: MM0 result");

    // MM2 should be unchanged
    let mm2_result = read_mem_at_u64(&mem, 0x2020);
    assert_eq!(mm2_result, 0x3333333333333333, "PUNPCKLBW: MM2 unchanged");
}

#[test]
fn test_punpcklwd_register_independence() {
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00,
        0x00, 0x0f, 0x6f, 0x14, 0x25, 0x10, 0x20, 0x00, 0x00, 0x0f, 0x61,
        0xc1, // PUNPCKLWD MM0, MM1
        0x0f, 0x7f, 0x04, 0x25, 0x18, 0x20, 0x00, 0x00, 0x0f, 0x7f, 0x14, 0x25, 0x20, 0x20, 0x00,
        0x00, 0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u64(&mem, 0x2000, 0x1111111111111111);
    write_mem_at_u64(&mem, 0x2008, 0x2222222222222222);
    write_mem_at_u64(&mem, 0x2010, 0x3333333333333333);

    run_until_hlt(&mut vcpu).unwrap();

    let mm0_result = read_mem_at_u64(&mem, 0x2018);
    assert_eq!(mm0_result, 0x2222111122221111, "PUNPCKLWD: MM0 result");

    let mm2_result = read_mem_at_u64(&mem, 0x2020);
    assert_eq!(mm2_result, 0x3333333333333333, "PUNPCKLWD: MM2 unchanged");
}

#[test]
fn test_punpcklbw_memory_alignment() {
    // Test with memory operand
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0f, 0x60, 0x04, 0x25, 0x08, 0x20, 0x00,
        0x00, // PUNPCKLBW MM0, [0x2008]
        0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, 0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u64(&mem, 0x2000, 0xFFFFFFFF01234567);
    write_mem_at_u64(&mem, 0x2008, 0xFFFFFFFF89ABCDEF);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_mem_at_u64(&mem, 0x2010);
    // Low 4 bytes: 01 23 45 67 and 89 AB CD EF
    // Result: 89 01 AB 23 CD 45 EF 67
    assert_eq!(result, 0x8901AB23CD45EF67, "PUNPCKLBW: memory alignment");
}

#[test]
fn test_punpcklwd_memory_alignment() {
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0f, 0x61, 0x04, 0x25, 0x08, 0x20, 0x00,
        0x00, // PUNPCKLWD MM0, [0x2008]
        0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, 0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u64(&mem, 0x2000, 0xFFFFFFFF01234567);
    write_mem_at_u64(&mem, 0x2008, 0xFFFFFFFF89ABCDEF);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_mem_at_u64(&mem, 0x2010);
    // Low 2 words: 0123 4567 and 89AB CDEF
    // Result: 89AB 0123 CDEF 4567
    assert_eq!(result, 0x89AB0123CDEF4567, "PUNPCKLWD: memory alignment");
}
