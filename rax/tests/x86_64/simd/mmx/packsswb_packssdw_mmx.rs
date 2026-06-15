//! Tests for PACKSSWB and PACKSSDW instructions (MMX).
//!
//! PACKSSWB/PACKSSDW - Pack With Signed Saturation
//!
//! Converts packed signed word/dword integers from the destination and source operands
//! into packed signed byte/word integers using signed saturation to handle overflow.
//!
//! - PACKSSWB: Pack 4 signed words from mm1 and 4 from mm2/m64 into 8 signed bytes in mm1
//! - PACKSSDW: Pack 2 signed dwords from mm1 and 2 from mm2/m64 into 4 signed words in mm1
//!
//! Saturation: Values > 0x7F/0x7FFF become 0x7F/0x7FFF, values < 0x80/0x8000 become 0x80/0x8000
//!
//! Flags affected: None
//!
//! Reference: docs/packsswb:packssdw.txt

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

fn packsswb_expected(dst: u64, src: u64) -> u64 {
    let mut result = 0u64;
    for i in 0..4 {
        let w = ((dst >> (i * 16)) & 0xFFFF) as i16;
        let b = w.clamp(-128, 127) as i8 as u8;
        result |= (b as u64) << (i * 8);
    }
    for i in 0..4 {
        let w = ((src >> (i * 16)) & 0xFFFF) as i16;
        let b = w.clamp(-128, 127) as i8 as u8;
        result |= (b as u64) << ((i + 4) * 8);
    }
    result
}

fn packssdw_expected(dst: u64, src: u64) -> u64 {
    let mut result = 0u64;
    for i in 0..2 {
        let d = ((dst >> (i * 32)) & 0xFFFF_FFFF) as i32;
        let w = d.clamp(-32768, 32767) as i16 as u16;
        result |= (w as u64) << (i * 16);
    }
    for i in 0..2 {
        let d = ((src >> (i * 32)) & 0xFFFF_FFFF) as i32;
        let w = d.clamp(-32768, 32767) as i16 as u16;
        result |= (w as u64) << ((i + 2) * 16);
    }
    result
}

// ============================================================================
// PACKSSWB mm, mm/m64 (opcode 0F 63 /r) - Pack signed words to signed bytes
// ============================================================================

#[test]
fn test_packsswb_mm_mm_basic() {
    // PACKSSWB MM0, MM1 - basic packing
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // MOVQ MM0, [0x2000]
        0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00, 0x00, // MOVQ MM1, [0x2008]
        0x0f, 0x63, 0xc1, // PACKSSWB MM0, MM1
        0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, // MOVQ [0x2010], MM0
        0xf4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    // MM0 = 4 words: 0x0001, 0x0002, 0x0003, 0x0004 (all in byte range)
    write_mem_at_u64(&mem, 0x2000, 0x0004000300020001);
    // MM1 = 4 words: 0x0005, 0x0006, 0x0007, 0x0008
    write_mem_at_u64(&mem, 0x2008, 0x0008000700060005);

    run_until_hlt(&mut vcpu).unwrap();

    // Result: 8 bytes from 4 words of MM0 then 4 words of MM1
    let result = read_mem_at_u64(&mem, 0x2010);
    assert_eq!(
        result,
        packsswb_expected(0x0004000300020001, 0x0008000700060005),
        "PACKSSWB: basic packing"
    );
}

#[test]
fn test_packsswb_mm_mm_positive_saturation() {
    // Test saturation for positive overflow (> 0x7F)
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00,
        0x00, 0x0f, 0x63, 0xc1, 0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, 0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    // Values above 0x7F should saturate to 0x7F
    write_mem_at_u64(&mem, 0x2000, 0x0100007F00800200);
    write_mem_at_u64(&mem, 0x2008, 0x7FFF00010000FFFF);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_mem_at_u64(&mem, 0x2010);
    // 0200 -> 7F, 0080 -> 7F, 007F -> 7F, 0100 -> 7F, FFFF -> 80, 0000 -> 00, 0001 -> 01, 7FFF -> 7F
    assert_eq!(
        result,
        packsswb_expected(0x0100007F00800200, 0x7FFF00010000FFFF),
        "PACKSSWB: positive saturation"
    );
}

#[test]
fn test_packsswb_mm_mm_negative_saturation() {
    // Test saturation for negative overflow (< -128 = 0xFF80)
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00,
        0x00, 0x0f, 0x63, 0xc1, 0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, 0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    // Negative values: FF80 = -128 (fits), FF7F = -129 (saturate to 80)
    write_mem_at_u64(&mem, 0x2000, 0xFF00FF7FFF80FFFF);
    write_mem_at_u64(&mem, 0x2008, 0x8000FFFFFFFF81FF);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_mem_at_u64(&mem, 0x2010);
    // FFFF -> FF, FF80 -> 80, FF7F -> 80, FF00 -> 80, FF81 -> 81, FFFF -> FF, FFFF -> FF, 8000 -> 80
    assert_eq!(
        result,
        packsswb_expected(0xFF00FF7FFF80FFFF, 0x8000FFFFFFFF81FF),
        "PACKSSWB: negative saturation"
    );
}

#[test]
fn test_packsswb_mm_m64() {
    // Test with memory operand
    let code = vec![
        0x0f, 0x6f, 0x14, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0f, 0x63, 0x14, 0x25, 0x08, 0x20, 0x00,
        0x00, // PACKSSWB MM2, [0x2008]
        0x0f, 0x7f, 0x14, 0x25, 0x10, 0x20, 0x00, 0x00, 0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u64(&mem, 0x2000, 0x0010002000300040);
    write_mem_at_u64(&mem, 0x2008, 0x0050006000700080);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_mem_at_u64(&mem, 0x2010);
    // 40 30 20 10 from MM2, then 80 (sat) 70 60 50 from memory
    assert_eq!(
        result,
        packsswb_expected(0x0010002000300040, 0x0050006000700080),
        "PACKSSWB: memory operand"
    );
}

#[test]
fn test_packsswb_all_zeros() {
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00,
        0x00, 0x0f, 0x63, 0xc1, 0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, 0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u64(&mem, 0x2000, 0x0000000000000000);
    write_mem_at_u64(&mem, 0x2008, 0x0000000000000000);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_mem_at_u64(&mem, 0x2010);
    assert_eq!(
        result,
        packsswb_expected(0x0000000000000000, 0x0000000000000000),
        "PACKSSWB: all zeros"
    );
}

#[test]
fn test_packsswb_boundary_values() {
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00,
        0x00, 0x0f, 0x63, 0xc1, 0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, 0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    // Test exact boundaries: 0x7F, 0x80 (in signed), FF80 (-128)
    write_mem_at_u64(&mem, 0x2000, 0x007F0080FF80FF81);
    write_mem_at_u64(&mem, 0x2008, 0x00000001FFFFFFFF);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_mem_at_u64(&mem, 0x2010);
    // FF81 -> 81, FF80 -> 80, 0080 -> 7F (sat), 007F -> 7F, FFFF -> FF, 0001 -> 01, 0000 -> 00
    assert_eq!(
        result,
        packsswb_expected(0x007F0080FF80FF81, 0x00000001FFFFFFFF),
        "PACKSSWB: boundary values"
    );
}

#[test]
fn test_packsswb_mm3_mm4() {
    let code = vec![
        0x0f, 0x6f, 0x1c, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0f, 0x6f, 0x24, 0x25, 0x08, 0x20, 0x00,
        0x00, 0x0f, 0x63, 0xdc, // PACKSSWB MM3, MM4
        0x0f, 0x7f, 0x1c, 0x25, 0x10, 0x20, 0x00, 0x00, 0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u64(&mem, 0x2000, 0x000A000B000C000D);
    write_mem_at_u64(&mem, 0x2008, 0x000E000F00100011);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_mem_at_u64(&mem, 0x2010);
    assert_eq!(
        result,
        packsswb_expected(0x000A000B000C000D, 0x000E000F00100011),
        "PACKSSWB: MM3 with MM4"
    );
}

#[test]
fn test_packsswb_max_saturation() {
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00,
        0x00, 0x0f, 0x63, 0xc1, 0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, 0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    // All max values
    write_mem_at_u64(&mem, 0x2000, 0x7FFF7FFF7FFF7FFF);
    write_mem_at_u64(&mem, 0x2008, 0x7FFF7FFF7FFF7FFF);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_mem_at_u64(&mem, 0x2010);
    assert_eq!(
        result,
        packsswb_expected(0x7FFF7FFF7FFF7FFF, 0x7FFF7FFF7FFF7FFF),
        "PACKSSWB: all max saturation"
    );
}

#[test]
fn test_packsswb_min_saturation() {
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00,
        0x00, 0x0f, 0x63, 0xc1, 0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, 0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    // All min values
    write_mem_at_u64(&mem, 0x2000, 0x8000800080008000);
    write_mem_at_u64(&mem, 0x2008, 0x8000800080008000);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_mem_at_u64(&mem, 0x2010);
    assert_eq!(
        result,
        packsswb_expected(0x8000800080008000, 0x8000800080008000),
        "PACKSSWB: all min saturation"
    );
}

#[test]
fn test_packsswb_mixed_saturation() {
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00,
        0x00, 0x0f, 0x63, 0xc1, 0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, 0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    // Mix of values: some saturate, some don't
    write_mem_at_u64(&mem, 0x2000, 0x0100007FFF800001);
    write_mem_at_u64(&mem, 0x2008, 0x80007FFF0000FFFF);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_mem_at_u64(&mem, 0x2010);
    // 0001->01, FF80->80, 007F->7F, 0100->7F, FFFF->FF, 0000->00, 7FFF->7F, 8000->80
    assert_eq!(
        result,
        packsswb_expected(0x0100007FFF800001, 0x80007FFF0000FFFF),
        "PACKSSWB: mixed saturation"
    );
}

// ============================================================================
// PACKSSDW mm, mm/m64 (opcode 0F 6B /r) - Pack signed dwords to signed words
// ============================================================================

#[test]
fn test_packssdw_mm_mm_basic() {
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00,
        0x00, 0x0f, 0x6b, 0xc1, // PACKSSDW MM0, MM1
        0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, 0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    // MM0 = 2 dwords: 0x00000001, 0x00000002 (all in word range)
    write_mem_at_u64(&mem, 0x2000, 0x0000000200000001);
    // MM1 = 2 dwords: 0x00000003, 0x00000004
    write_mem_at_u64(&mem, 0x2008, 0x0000000400000003);

    run_until_hlt(&mut vcpu).unwrap();

    // Result: 4 words from 2 dwords of MM0 then 2 dwords of MM1
    let result = read_mem_at_u64(&mem, 0x2010);
    assert_eq!(
        result,
        packssdw_expected(0x0000000200000001, 0x0000000400000003),
        "PACKSSDW: basic packing"
    );
}

#[test]
fn test_packssdw_mm_mm_positive_saturation() {
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00,
        0x00, 0x0f, 0x6b, 0xc1, 0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, 0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    // Values above 0x7FFF should saturate to 0x7FFF
    write_mem_at_u64(&mem, 0x2000, 0x00010000000080000);
    write_mem_at_u64(&mem, 0x2008, 0x7FFFFFFF00000001);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_mem_at_u64(&mem, 0x2010);
    // 0x8000->7FFF, 0x10000->7FFF, 0x1->0001, 0x7FFFFFFF->7FFF
    assert_eq!(
        result,
        packssdw_expected(0x00010000000080000, 0x7FFFFFFF00000001),
        "PACKSSDW: positive saturation"
    );
}

#[test]
fn test_packssdw_mm_mm_negative_saturation() {
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00,
        0x00, 0x0f, 0x6b, 0xc1, 0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, 0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    // Negative values: FFFF8000 = -32768 (fits), FFFF7FFF = -32769 (saturate)
    write_mem_at_u64(&mem, 0x2000, 0xFFFF0000FFFF8000);
    write_mem_at_u64(&mem, 0x2008, 0x80000000FFFFFFFF);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_mem_at_u64(&mem, 0x2010);
    // FFFF8000->8000, FFFF0000->8000, FFFFFFFF->FFFF, 80000000->8000
    assert_eq!(
        result,
        packssdw_expected(0xFFFF0000FFFF8000, 0x80000000FFFFFFFF),
        "PACKSSDW: negative saturation"
    );
}

#[test]
fn test_packssdw_mm_m64() {
    let code = vec![
        0x0f, 0x6f, 0x14, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0f, 0x6b, 0x14, 0x25, 0x08, 0x20, 0x00,
        0x00, // PACKSSDW MM2, [0x2008]
        0x0f, 0x7f, 0x14, 0x25, 0x10, 0x20, 0x00, 0x00, 0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u64(&mem, 0x2000, 0x0000100000002000);
    write_mem_at_u64(&mem, 0x2008, 0x0000300000004000);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_mem_at_u64(&mem, 0x2010);
    assert_eq!(
        result,
        packssdw_expected(0x0000100000002000, 0x0000300000004000),
        "PACKSSDW: memory operand"
    );
}

#[test]
fn test_packssdw_all_zeros() {
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00,
        0x00, 0x0f, 0x6b, 0xc1, 0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, 0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u64(&mem, 0x2000, 0x0000000000000000);
    write_mem_at_u64(&mem, 0x2008, 0x0000000000000000);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_mem_at_u64(&mem, 0x2010);
    assert_eq!(
        result,
        packssdw_expected(0x0000000000000000, 0x0000000000000000),
        "PACKSSDW: all zeros"
    );
}

#[test]
fn test_packssdw_boundary_values() {
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00,
        0x00, 0x0f, 0x6b, 0xc1, 0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, 0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    // Test exact boundaries: 0x7FFF, 0x8000 (in signed), FFFF8000 (-32768)
    write_mem_at_u64(&mem, 0x2000, 0x00007FFFFFF8000);
    write_mem_at_u64(&mem, 0x2008, 0x00000001FFFFFFFF);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_mem_at_u64(&mem, 0x2010);
    // FFFF8000->8000, 0x7FFF->7FFF, FFFFFFFF->FFFF, 0x1->0001
    assert_eq!(
        result,
        packssdw_expected(0x00007FFFFFF8000, 0x00000001FFFFFFFF),
        "PACKSSDW: boundary values"
    );
}

#[test]
fn test_packssdw_mm5_mm6() {
    let code = vec![
        0x0f, 0x6f, 0x2c, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0f, 0x6f, 0x34, 0x25, 0x08, 0x20, 0x00,
        0x00, 0x0f, 0x6b, 0xee, // PACKSSDW MM5, MM6
        0x0f, 0x7f, 0x2c, 0x25, 0x10, 0x20, 0x00, 0x00, 0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u64(&mem, 0x2000, 0x00000ABC00000DEF);
    write_mem_at_u64(&mem, 0x2008, 0x0000123400005678);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_mem_at_u64(&mem, 0x2010);
    assert_eq!(
        result,
        packssdw_expected(0x00000ABC00000DEF, 0x0000123400005678),
        "PACKSSDW: MM5 with MM6"
    );
}

#[test]
fn test_packssdw_max_saturation() {
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00,
        0x00, 0x0f, 0x6b, 0xc1, 0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, 0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    // All max values
    write_mem_at_u64(&mem, 0x2000, 0x7FFFFFFF7FFFFFFF);
    write_mem_at_u64(&mem, 0x2008, 0x7FFFFFFF7FFFFFFF);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_mem_at_u64(&mem, 0x2010);
    assert_eq!(
        result,
        packssdw_expected(0x7FFFFFFF7FFFFFFF, 0x7FFFFFFF7FFFFFFF),
        "PACKSSDW: all max saturation"
    );
}

#[test]
fn test_packssdw_min_saturation() {
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00,
        0x00, 0x0f, 0x6b, 0xc1, 0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, 0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    // All min values
    write_mem_at_u64(&mem, 0x2000, 0x8000000080000000);
    write_mem_at_u64(&mem, 0x2008, 0x8000000080000000);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_mem_at_u64(&mem, 0x2010);
    assert_eq!(
        result,
        packssdw_expected(0x8000000080000000, 0x8000000080000000),
        "PACKSSDW: all min saturation"
    );
}

#[test]
fn test_packssdw_mixed_saturation() {
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00,
        0x00, 0x0f, 0x6b, 0xc1, 0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, 0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u64(&mem, 0x2000, 0x0001000000007FFF);
    write_mem_at_u64(&mem, 0x2008, 0x80000000FFFFFFFF);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_mem_at_u64(&mem, 0x2010);
    // 0x7FFF->7FFF, 0->0000, FFFFFFFF->FFFF, 80000000->8000
    assert_eq!(
        result,
        packssdw_expected(0x0001000000007FFF, 0x80000000FFFFFFFF),
        "PACKSSDW: mixed saturation"
    );
}

// ============================================================================
// Additional comprehensive tests
// ============================================================================

#[test]
fn test_packsswb_sequential_values() {
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00,
        0x00, 0x0f, 0x63, 0xc1, 0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, 0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u64(&mem, 0x2000, 0x0001000200030004);
    write_mem_at_u64(&mem, 0x2008, 0x0005000600070008);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_mem_at_u64(&mem, 0x2010);
    assert_eq!(
        result,
        packsswb_expected(0x0001000200030004, 0x0005000600070008),
        "PACKSSWB: sequential values"
    );
}

#[test]
fn test_packssdw_sequential_values() {
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00,
        0x00, 0x0f, 0x6b, 0xc1, 0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, 0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u64(&mem, 0x2000, 0x0000000100000002);
    write_mem_at_u64(&mem, 0x2008, 0x0000000300000004);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_mem_at_u64(&mem, 0x2010);
    assert_eq!(
        result,
        packssdw_expected(0x0000000100000002, 0x0000000300000004),
        "PACKSSDW: sequential values"
    );
}

#[test]
fn test_packsswb_alternating_saturation() {
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00,
        0x00, 0x0f, 0x63, 0xc1, 0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, 0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    // Alternating saturate/no saturate
    write_mem_at_u64(&mem, 0x2000, 0x7FFF0001800000FF);
    write_mem_at_u64(&mem, 0x2008, 0x00017FFFFFFF8000);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_mem_at_u64(&mem, 0x2010);
    // 00FF->FF, 8000->80, 0001->01, 7FFF->7F, 8000->80, FFFF->FF, 7FFF->7F, 0001->01
    assert_eq!(
        result,
        packsswb_expected(0x7FFF0001800000FF, 0x00017FFFFFFF8000),
        "PACKSSWB: alternating saturation"
    );
}

#[test]
fn test_packssdw_alternating_saturation() {
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00,
        0x00, 0x0f, 0x6b, 0xc1, 0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, 0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u64(&mem, 0x2000, 0x7FFFFFFF00000001);
    write_mem_at_u64(&mem, 0x2008, 0x0000000180000000);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_mem_at_u64(&mem, 0x2010);
    // 0x1->0001, 0x7FFFFFFF->7FFF, 0x80000000->8000, 0x1->0001
    assert_eq!(
        result,
        packssdw_expected(0x7FFFFFFF00000001, 0x0000000180000000),
        "PACKSSDW: alternating saturation"
    );
}

#[test]
fn test_packsswb_register_independence() {
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00,
        0x00, 0x0f, 0x6f, 0x14, 0x25, 0x10, 0x20, 0x00, 0x00, 0x0f, 0x63, 0xc1, 0x0f, 0x7f, 0x04,
        0x25, 0x18, 0x20, 0x00, 0x00, 0x0f, 0x7f, 0x14, 0x25, 0x20, 0x20, 0x00, 0x00, 0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u64(&mem, 0x2000, 0x0001000200030004);
    write_mem_at_u64(&mem, 0x2008, 0x0005000600070008);
    write_mem_at_u64(&mem, 0x2010, 0x1111111111111111);

    run_until_hlt(&mut vcpu).unwrap();

    let mm0_result = read_mem_at_u64(&mem, 0x2018);
    assert_eq!(
        mm0_result,
        packsswb_expected(0x0001000200030004, 0x0005000600070008),
        "PACKSSWB: MM0 result"
    );

    let mm2_result = read_mem_at_u64(&mem, 0x2020);
    assert_eq!(mm2_result, 0x1111111111111111, "PACKSSWB: MM2 unchanged");
}

#[test]
fn test_packssdw_register_independence() {
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00,
        0x00, 0x0f, 0x6f, 0x14, 0x25, 0x10, 0x20, 0x00, 0x00, 0x0f, 0x6b, 0xc1, 0x0f, 0x7f, 0x04,
        0x25, 0x18, 0x20, 0x00, 0x00, 0x0f, 0x7f, 0x14, 0x25, 0x20, 0x20, 0x00, 0x00, 0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u64(&mem, 0x2000, 0x0000000100000002);
    write_mem_at_u64(&mem, 0x2008, 0x0000000300000004);
    write_mem_at_u64(&mem, 0x2010, 0x2222222222222222);

    run_until_hlt(&mut vcpu).unwrap();

    let mm0_result = read_mem_at_u64(&mem, 0x2018);
    assert_eq!(
        mm0_result,
        packssdw_expected(0x0000000100000002, 0x0000000300000004),
        "PACKSSDW: MM0 result"
    );

    let mm2_result = read_mem_at_u64(&mem, 0x2020);
    assert_eq!(mm2_result, 0x2222222222222222, "PACKSSDW: MM2 unchanged");
}

#[test]
fn test_packsswb_mm7_mm0() {
    let code = vec![
        0x0f, 0x6f, 0x3c, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0f, 0x6f, 0x04, 0x25, 0x08, 0x20, 0x00,
        0x00, 0x0f, 0x63, 0xf8, 0x0f, 0x7f, 0x3c, 0x25, 0x10, 0x20, 0x00, 0x00, 0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u64(&mem, 0x2000, 0x00200030007F0080);
    write_mem_at_u64(&mem, 0x2008, 0xFF80FF00010020FF);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_mem_at_u64(&mem, 0x2010);
    // 0080->7F, 007F->7F, 0030->30, 0020->20, 20FF->7F (sat), 0100->7F (sat), FF00->80 (sat), FF80->80
    assert_eq!(
        result,
        packsswb_expected(0x00200030007F0080, 0xFF80FF00010020FF),
        "PACKSSWB: MM7 with MM0"
    );
}

#[test]
fn test_packssdw_mm7_mm0() {
    let code = vec![
        0x0f, 0x6f, 0x3c, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0f, 0x6f, 0x04, 0x25, 0x08, 0x20, 0x00,
        0x00, 0x0f, 0x6b, 0xf8, 0x0f, 0x7f, 0x3c, 0x25, 0x10, 0x20, 0x00, 0x00, 0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u64(&mem, 0x2000, 0x0001000000007FFF);
    write_mem_at_u64(&mem, 0x2008, 0xFFFF8000FFFFFFFF);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_mem_at_u64(&mem, 0x2010);
    // 0x7FFF->7FFF, 0->0000, FFFFFFFF->FFFF, FFFF8000->8000
    assert_eq!(
        result,
        packssdw_expected(0x0001000000007FFF, 0xFFFF8000FFFFFFFF),
        "PACKSSDW: MM7 with MM0"
    );
}
