//! Tests for PACKUSWB instruction (MMX).
//!
//! PACKUSWB - Pack With Unsigned Saturation
//!
//! Converts 4 packed signed word integers from mm1 and 4 signed word integers from
//! mm2/m64 into 8 packed unsigned byte integers in mm1 using unsigned saturation.
//!
//! Saturation: Values > 0xFF become 0xFF, values < 0 become 0x00
//!
//! Flags affected: None
//!
//! Reference: docs/packuswb.txt

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

fn packuswb_expected(dst: u64, src: u64) -> u64 {
    let mut result = 0u64;
    for i in 0..4 {
        let w = ((dst >> (i * 16)) & 0xFFFF) as i16;
        let b = w.clamp(0, 255) as u8;
        result |= (b as u64) << (i * 8);
    }
    for i in 0..4 {
        let w = ((src >> (i * 16)) & 0xFFFF) as i16;
        let b = w.clamp(0, 255) as u8;
        result |= (b as u64) << ((i + 4) * 8);
    }
    result
}

// ============================================================================
// PACKUSWB mm, mm/m64 (opcode 0F 67 /r) - Pack signed words to unsigned bytes
// ============================================================================

#[test]
fn test_packuswb_mm_mm_basic() {
    // PACKUSWB MM0, MM1 - basic packing
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // MOVQ MM0, [0x2000]
        0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00, 0x00, // MOVQ MM1, [0x2008]
        0x0f, 0x67, 0xc1, // PACKUSWB MM0, MM1
        0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, // MOVQ [0x2010], MM0
        0xf4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    // MM0 = 4 words: 0x0001, 0x0002, 0x0003, 0x0004 (all in unsigned byte range)
    write_mem_at_u64(&mem, 0x2000, 0x0004000300020001);
    // MM1 = 4 words: 0x0005, 0x0006, 0x0007, 0x0008
    write_mem_at_u64(&mem, 0x2008, 0x0008000700060005);

    run_until_hlt(&mut vcpu).unwrap();

    // Result: 8 bytes from 4 words of MM0 then 4 words of MM1
    let result = read_mem_at_u64(&mem, 0x2010);
    assert_eq!(
        result,
        packuswb_expected(0x0004000300020001, 0x0008000700060005),
        "PACKUSWB: basic packing"
    );
}

#[test]
fn test_packuswb_mm_mm_positive_saturation() {
    // Test saturation for positive overflow (> 0xFF)
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00,
        0x00, 0x0f, 0x67, 0xc1, 0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, 0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    // Values above 0xFF should saturate to 0xFF
    write_mem_at_u64(&mem, 0x2000, 0x010000FF01000200);
    write_mem_at_u64(&mem, 0x2008, 0x7FFF00010000FFFF);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_mem_at_u64(&mem, 0x2010);
    // 0200->FF, 0100->FF, 00FF->FF, 0100->FF, FFFF->00, 0000->00, 0001->01, 7FFF->FF
    assert_eq!(
        result,
        packuswb_expected(0x010000FF01000200, 0x7FFF00010000FFFF),
        "PACKUSWB: positive saturation"
    );
}

#[test]
fn test_packuswb_mm_mm_negative_saturation() {
    // Test saturation for negative values (< 0) - all saturate to 0x00
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00,
        0x00, 0x0f, 0x67, 0xc1, 0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, 0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    // All negative values (signed interpretation) should become 0x00
    write_mem_at_u64(&mem, 0x2000, 0xFFFFFFFFFFFF8000);
    write_mem_at_u64(&mem, 0x2008, 0xFF00FF01FF80FFFE);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_mem_at_u64(&mem, 0x2010);
    assert_eq!(
        result,
        packuswb_expected(0xFFFFFFFFFFFF8000, 0xFF00FF01FF80FFFE),
        "PACKUSWB: negative saturation"
    );
}

#[test]
fn test_packuswb_mm_m64() {
    // Test with memory operand
    let code = vec![
        0x0f, 0x6f, 0x14, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0f, 0x67, 0x14, 0x25, 0x08, 0x20, 0x00,
        0x00, // PACKUSWB MM2, [0x2008]
        0x0f, 0x7f, 0x14, 0x25, 0x10, 0x20, 0x00, 0x00, 0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u64(&mem, 0x2000, 0x0010002000300040);
    write_mem_at_u64(&mem, 0x2008, 0x0050006000700080);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_mem_at_u64(&mem, 0x2010);
    assert_eq!(
        result,
        packuswb_expected(0x0010002000300040, 0x0050006000700080),
        "PACKUSWB: memory operand"
    );
}

#[test]
fn test_packuswb_all_zeros() {
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00,
        0x00, 0x0f, 0x67, 0xc1, 0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, 0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u64(&mem, 0x2000, 0x0000000000000000);
    write_mem_at_u64(&mem, 0x2008, 0x0000000000000000);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_mem_at_u64(&mem, 0x2010);
    assert_eq!(
        result,
        packuswb_expected(0x0000000000000000, 0x0000000000000000),
        "PACKUSWB: all zeros"
    );
}

#[test]
fn test_packuswb_boundary_values() {
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00,
        0x00, 0x0f, 0x67, 0xc1, 0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, 0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    // Test exact boundaries: 0x00, 0xFF, 0x100 (saturates)
    write_mem_at_u64(&mem, 0x2000, 0x01000100000000FF);
    write_mem_at_u64(&mem, 0x2008, 0x0000FFFF00010000);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_mem_at_u64(&mem, 0x2010);
    // 00FF->FF, 0000->00, 0100->FF, 0100->FF, 0000->00, 0001->01, FFFF->00, 0000->00
    assert_eq!(
        result,
        packuswb_expected(0x01000100000000FF, 0x0000FFFF00010000),
        "PACKUSWB: boundary values"
    );
}

#[test]
fn test_packuswb_mm3_mm4() {
    let code = vec![
        0x0f, 0x6f, 0x1c, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0f, 0x6f, 0x24, 0x25, 0x08, 0x20, 0x00,
        0x00, 0x0f, 0x67, 0xdc, // PACKUSWB MM3, MM4
        0x0f, 0x7f, 0x1c, 0x25, 0x10, 0x20, 0x00, 0x00, 0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u64(&mem, 0x2000, 0x000A000B000C000D);
    write_mem_at_u64(&mem, 0x2008, 0x000E000F00100011);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_mem_at_u64(&mem, 0x2010);
    assert_eq!(
        result,
        packuswb_expected(0x000A000B000C000D, 0x000E000F00100011),
        "PACKUSWB: MM3 with MM4"
    );
}

#[test]
fn test_packuswb_max_saturation() {
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00,
        0x00, 0x0f, 0x67, 0xc1, 0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, 0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    // All large positive values - should saturate to 0xFF
    write_mem_at_u64(&mem, 0x2000, 0x7FFF7FFF7FFF7FFF);
    write_mem_at_u64(&mem, 0x2008, 0x0FFF0FFF0FFF0FFF);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_mem_at_u64(&mem, 0x2010);
    assert_eq!(
        result,
        packuswb_expected(0x7FFF7FFF7FFF7FFF, 0x0FFF0FFF0FFF0FFF),
        "PACKUSWB: all max saturation"
    );
}

#[test]
fn test_packuswb_min_saturation() {
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00,
        0x00, 0x0f, 0x67, 0xc1, 0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, 0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    // All negative values - should saturate to 0x00
    write_mem_at_u64(&mem, 0x2000, 0x8000800080008000);
    write_mem_at_u64(&mem, 0x2008, 0xFFFFFFFFFFFFFFFF);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_mem_at_u64(&mem, 0x2010);
    assert_eq!(
        result,
        packuswb_expected(0x8000800080008000, 0xFFFFFFFFFFFFFFFF),
        "PACKUSWB: all min saturation"
    );
}

#[test]
fn test_packuswb_mixed_saturation() {
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00,
        0x00, 0x0f, 0x67, 0xc1, 0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, 0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    // Mix: some saturate high, some low, some in range
    write_mem_at_u64(&mem, 0x2000, 0x010000FF8000007F);
    write_mem_at_u64(&mem, 0x2008, 0xFFFF00000FFF0001);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_mem_at_u64(&mem, 0x2010);
    // 007F->7F, 8000->00, 00FF->FF, 0100->FF, 0001->01, 0FFF->FF, 0000->00, FFFF->00
    assert_eq!(
        result,
        packuswb_expected(0x010000FF8000007F, 0xFFFF00000FFF0001),
        "PACKUSWB: mixed saturation"
    );
}

#[test]
fn test_packuswb_sequential_values() {
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00,
        0x00, 0x0f, 0x67, 0xc1, 0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, 0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u64(&mem, 0x2000, 0x0001000200030004);
    write_mem_at_u64(&mem, 0x2008, 0x0005000600070008);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_mem_at_u64(&mem, 0x2010);
    assert_eq!(
        result,
        packuswb_expected(0x0001000200030004, 0x0005000600070008),
        "PACKUSWB: sequential values"
    );
}

#[test]
fn test_packuswb_alternating_saturation() {
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00,
        0x00, 0x0f, 0x67, 0xc1, 0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, 0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    // Alternating: saturate high, in range, saturate low, in range
    write_mem_at_u64(&mem, 0x2000, 0x7FFF0001FFFF00FF);
    write_mem_at_u64(&mem, 0x2008, 0x00017FFF0000FFFF);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_mem_at_u64(&mem, 0x2010);
    // 00FF->FF, FFFF->00, 0001->01, 7FFF->FF, 0000->00, FFFF->00, 7FFF->FF, 0001->01
    assert_eq!(
        result,
        packuswb_expected(0x7FFF0001FFFF00FF, 0x00017FFF0000FFFF),
        "PACKUSWB: alternating saturation"
    );
}

#[test]
fn test_packuswb_register_independence() {
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00,
        0x00, 0x0f, 0x6f, 0x14, 0x25, 0x10, 0x20, 0x00, 0x00, 0x0f, 0x67, 0xc1, 0x0f, 0x7f, 0x04,
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
        packuswb_expected(0x0001000200030004, 0x0005000600070008),
        "PACKUSWB: MM0 result"
    );

    let mm2_result = read_mem_at_u64(&mem, 0x2020);
    assert_eq!(mm2_result, 0x1111111111111111, "PACKUSWB: MM2 unchanged");
}

#[test]
fn test_packuswb_mm5_mm6() {
    let code = vec![
        0x0f, 0x6f, 0x2c, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0f, 0x6f, 0x34, 0x25, 0x08, 0x20, 0x00,
        0x00, 0x0f, 0x67, 0xee, // PACKUSWB MM5, MM6
        0x0f, 0x7f, 0x2c, 0x25, 0x10, 0x20, 0x00, 0x00, 0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u64(&mem, 0x2000, 0x000000AB000000EF);
    write_mem_at_u64(&mem, 0x2008, 0x0000001200000056);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_mem_at_u64(&mem, 0x2010);
    assert_eq!(
        result,
        packuswb_expected(0x000000AB000000EF, 0x0000001200000056),
        "PACKUSWB: MM5 with MM6"
    );
}

#[test]
fn test_packuswb_just_in_range() {
    // Test values at exact boundaries that don't saturate
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00,
        0x00, 0x0f, 0x67, 0xc1, 0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, 0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    // Values 0x00 to 0xFF (exactly in range)
    write_mem_at_u64(&mem, 0x2000, 0x007F008000FE00FF);
    write_mem_at_u64(&mem, 0x2008, 0x000000010001007F);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_mem_at_u64(&mem, 0x2010);
    // 00FF->FF, 00FE->FE, 0080->80, 007F->7F, 007F->7F, 0001->01, 0001->01, 0000->00
    assert_eq!(
        result,
        packuswb_expected(0x007F008000FE00FF, 0x000000010001007F),
        "PACKUSWB: just in range"
    );
}

#[test]
fn test_packuswb_one_above_max() {
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00,
        0x00, 0x0f, 0x67, 0xc1, 0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, 0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    // Test 0x100 (just above max)
    write_mem_at_u64(&mem, 0x2000, 0x010000FE00FF0100);
    write_mem_at_u64(&mem, 0x2008, 0x0000000000000000);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_mem_at_u64(&mem, 0x2010);
    // 0100->FF, 00FF->FF, 00FE->FE, 0100->FF, all others 0
    assert_eq!(
        result,
        packuswb_expected(0x010000FE00FF0100, 0x0000000000000000),
        "PACKUSWB: one above max"
    );
}

#[test]
fn test_packuswb_minus_one() {
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00,
        0x00, 0x0f, 0x67, 0xc1, 0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, 0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    // FFFF is -1 in signed, should saturate to 0
    write_mem_at_u64(&mem, 0x2000, 0xFFFFFFFFFFFFFFFF);
    write_mem_at_u64(&mem, 0x2008, 0xFFFEFFFDFFFCFFFB);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_mem_at_u64(&mem, 0x2010);
    assert_eq!(
        result,
        packuswb_expected(0xFFFFFFFFFFFFFFFF, 0xFFFEFFFDFFFCFFFB),
        "PACKUSWB: negative values to zero"
    );
}

#[test]
fn test_packuswb_large_positive() {
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00,
        0x00, 0x0f, 0x67, 0xc1, 0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, 0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    // Large positive values
    write_mem_at_u64(&mem, 0x2000, 0x1000200030004000);
    write_mem_at_u64(&mem, 0x2008, 0x5000600070007FFF);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_mem_at_u64(&mem, 0x2010);
    // All saturate to FF
    assert_eq!(
        result,
        packuswb_expected(0x1000200030004000, 0x5000600070007FFF),
        "PACKUSWB: large positive"
    );
}

#[test]
fn test_packuswb_mm7_mm0() {
    let code = vec![
        0x0f, 0x6f, 0x3c, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0f, 0x6f, 0x04, 0x25, 0x08, 0x20, 0x00,
        0x00, 0x0f, 0x67, 0xf8, // PACKUSWB MM7, MM0
        0x0f, 0x7f, 0x3c, 0x25, 0x10, 0x20, 0x00, 0x00, 0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u64(&mem, 0x2000, 0x0020003000FE00FF);
    write_mem_at_u64(&mem, 0x2008, 0xFFFF0000010020AB);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_mem_at_u64(&mem, 0x2010);
    // 00FF->FF, 00FE->FE, 0030->30, 0020->20, 20AB->FF, 0100->FF, 0000->00, FFFF->00
    assert_eq!(
        result,
        packuswb_expected(0x0020003000FE00FF, 0xFFFF0000010020AB),
        "PACKUSWB: MM7 with MM0"
    );
}

#[test]
fn test_packuswb_pattern_01() {
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00,
        0x00, 0x0f, 0x67, 0xc1, 0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, 0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u64(&mem, 0x2000, 0x0001000100010001);
    write_mem_at_u64(&mem, 0x2008, 0x0001000100010001);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_mem_at_u64(&mem, 0x2010);
    assert_eq!(
        result,
        packuswb_expected(0x0001000100010001, 0x0001000100010001),
        "PACKUSWB: pattern 01"
    );
}

#[test]
fn test_packuswb_pattern_ff() {
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00,
        0x00, 0x0f, 0x67, 0xc1, 0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, 0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u64(&mem, 0x2000, 0x00FF00FF00FF00FF);
    write_mem_at_u64(&mem, 0x2008, 0x00FF00FF00FF00FF);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_mem_at_u64(&mem, 0x2010);
    assert_eq!(
        result,
        packuswb_expected(0x00FF00FF00FF00FF, 0x00FF00FF00FF00FF),
        "PACKUSWB: pattern FF"
    );
}

#[test]
fn test_packuswb_half_saturate() {
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00,
        0x00, 0x0f, 0x67, 0xc1, 0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, 0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    // Alternating in-range and saturating values
    write_mem_at_u64(&mem, 0x2000, 0x0001010000010100);
    write_mem_at_u64(&mem, 0x2008, 0x00017FFF00010100);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_mem_at_u64(&mem, 0x2010);
    // 0100->FF, 0001->01, 7FFF->FF, 0001->01, 0100->FF, 0001->01, 0100->FF, 0001->01
    assert_eq!(
        result,
        packuswb_expected(0x0001010000010100, 0x00017FFF00010100),
        "PACKUSWB: half saturate"
    );
}

#[test]
fn test_packuswb_ascending_values() {
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00,
        0x00, 0x0f, 0x67, 0xc1, 0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, 0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u64(&mem, 0x2000, 0x000F001E002D003C);
    write_mem_at_u64(&mem, 0x2008, 0x004B005A00690078);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_mem_at_u64(&mem, 0x2010);
    // 3C 2D 1E 0F from MM0, 78 69 5A 4B from MM1
    assert_eq!(
        result,
        packuswb_expected(0x000F001E002D003C, 0x004B005A00690078),
        "PACKUSWB: ascending values"
    );
}

#[test]
fn test_packuswb_descending_values() {
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00,
        0x00, 0x0f, 0x67, 0xc1, 0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, 0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u64(&mem, 0x2000, 0x00F000D000B00090);
    write_mem_at_u64(&mem, 0x2008, 0x0070005000300010);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_mem_at_u64(&mem, 0x2010);
    // 90 B0 D0 F0 from MM0, 10 30 50 70 from MM1
    assert_eq!(
        result,
        packuswb_expected(0x00F000D000B00090, 0x0070005000300010),
        "PACKUSWB: descending values"
    );
}

#[test]
fn test_packuswb_mixed_range() {
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00,
        0x00, 0x0f, 0x67, 0xc1, 0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, 0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    // Mix of small, medium, large (saturating), and negative (saturating)
    write_mem_at_u64(&mem, 0x2000, 0x0001007F0100FFFF);
    write_mem_at_u64(&mem, 0x2008, 0x00007FFF01000080);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_mem_at_u64(&mem, 0x2010);
    // FFFF->00, 0100->FF, 007F->7F, 0001->01, 0080->80, 0100->FF, 7FFF->FF, 0000->00
    assert_eq!(
        result,
        packuswb_expected(0x0001007F0100FFFF, 0x00007FFF01000080),
        "PACKUSWB: mixed range"
    );
}

#[test]
fn test_packuswb_hex_pattern() {
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00,
        0x00, 0x0f, 0x67, 0xc1, 0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, 0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u64(&mem, 0x2000, 0x00AA00BB00CC00DD);
    write_mem_at_u64(&mem, 0x2008, 0x00EE00FF00AB00CD);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_mem_at_u64(&mem, 0x2010);
    assert_eq!(
        result,
        packuswb_expected(0x00AA00BB00CC00DD, 0x00EE00FF00AB00CD),
        "PACKUSWB: hex pattern"
    );
}
