//! Tests for PADDSB and PADDSW instructions (MMX).
//!
//! PADDSB - Add Packed Signed Bytes with Saturation (MMX)
//! PADDSW - Add Packed Signed Words with Saturation (MMX)
//!
//! Adds packed signed integers with saturation.
//! If overflow occurs, result is clamped to min/max value.
//!
//! Opcodes:
//! - PADDSB: 0F EC /r
//! - PADDSW: 0F ED /r
//!
//! Flags affected: None
//!
//! Reference: /Users/int/dev/rax/docs/paddsb:paddsw.txt

use crate::common::*;

fn write_mm_via_mem(mem: &vm_memory::GuestMemoryMmap, addr: u64, value: u64) {
    write_mem_at_u64(mem, addr, value);
}

// ============================================================================
// PADDSB mm, mm/m64 - Add Packed Signed Bytes with Saturation
// ============================================================================

#[test]
fn test_paddsb_basic() {
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // MOVQ MM0, [0x2000]
        0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00, 0x00, // MOVQ MM1, [0x2008]
        0x0f, 0xec, 0xc1, // PADDSB MM0, MM1
        0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, // MOVQ [0x2010], MM0
        0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    // 1+1=2, 2+2=4, 3+3=6, 4+4=8, 5+5=10, 6+6=12, 7+7=14, 8+8=16
    write_mm_via_mem(&mem, 0x2000, 0x0807060504030201);
    write_mm_via_mem(&mem, 0x2008, 0x0807060504030201);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_mem_at_u64(&mem, 0x2010);
    assert_eq!(result, 0x100E0C0A08060402, "PADDSB: basic addition");
}

#[test]
fn test_paddsb_positive_saturation() {
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00,
        0x00, 0x0f, 0xec, 0xc1, 0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, 0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    // 127 + 1 = saturate to 127
    write_mm_via_mem(&mem, 0x2000, 0x7F7F7F7F7F7F7F7F);
    write_mm_via_mem(&mem, 0x2008, 0x0101010101010101);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_mem_at_u64(&mem, 0x2010);
    assert_eq!(result, 0x7F7F7F7F7F7F7F7F, "PADDSB: positive saturation");
}

#[test]
fn test_paddsb_negative_saturation() {
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00,
        0x00, 0x0f, 0xec, 0xc1, 0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, 0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    // -128 + (-1) = saturate to -128
    write_mm_via_mem(&mem, 0x2000, 0x8080808080808080);
    write_mm_via_mem(&mem, 0x2008, 0xFFFFFFFFFFFFFFFF);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_mem_at_u64(&mem, 0x2010);
    assert_eq!(result, 0x8080808080808080, "PADDSB: negative saturation");
}

#[test]
fn test_paddsb_mixed_signs() {
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00,
        0x00, 0x0f, 0xec, 0xc1, 0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, 0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    // 10 + (-5) = 5, -10 + 5 = -5
    write_mm_via_mem(&mem, 0x2000, 0xF60A0000F60A0000);
    write_mm_via_mem(&mem, 0x2008, 0x05FB000005FB0000);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_mem_at_u64(&mem, 0x2010);
    assert_eq!(result, 0xFB050000FB050000, "PADDSB: mixed signs");
}

#[test]
fn test_paddsb_all_zeros() {
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00,
        0x00, 0x0f, 0xec, 0xc1, 0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, 0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mm_via_mem(&mem, 0x2000, 0x0000000000000000);
    write_mm_via_mem(&mem, 0x2008, 0x0000000000000000);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_mem_at_u64(&mem, 0x2010);
    assert_eq!(result, 0x0000000000000000, "PADDSB: all zeros");
}

#[test]
fn test_paddsb_no_saturation_positive() {
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00,
        0x00, 0x0f, 0xec, 0xc1, 0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, 0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    // 60 + 60 = 120 (no saturation)
    write_mm_via_mem(&mem, 0x2000, 0x3C3C3C3C3C3C3C3C);
    write_mm_via_mem(&mem, 0x2008, 0x3C3C3C3C3C3C3C3C);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_mem_at_u64(&mem, 0x2010);
    assert_eq!(
        result, 0x7878787878787878,
        "PADDSB: no saturation (positive)"
    );
}

#[test]
fn test_paddsb_no_saturation_negative() {
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00,
        0x00, 0x0f, 0xec, 0xc1, 0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, 0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    // -60 + (-60) = -120 (no saturation)
    write_mm_via_mem(&mem, 0x2000, 0xC4C4C4C4C4C4C4C4);
    write_mm_via_mem(&mem, 0x2008, 0xC4C4C4C4C4C4C4C4);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_mem_at_u64(&mem, 0x2010);
    assert_eq!(
        result, 0x8888888888888888,
        "PADDSB: no saturation (negative)"
    );
}

// ============================================================================
// PADDSW mm, mm/m64 - Add Packed Signed Words with Saturation
// ============================================================================

#[test]
fn test_paddsw_basic() {
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // MOVQ MM0, [0x2000]
        0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00, 0x00, // MOVQ MM1, [0x2008]
        0x0f, 0xed, 0xc1, // PADDSW MM0, MM1
        0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, // MOVQ [0x2010], MM0
        0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    // 100+100=200, 200+200=400, 300+300=600, 400+400=800
    write_mm_via_mem(&mem, 0x2000, 0x0190019001000064);
    write_mm_via_mem(&mem, 0x2008, 0x0190012C00C80064);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_mem_at_u64(&mem, 0x2010);
    assert_eq!(result, 0x032002BC01C800C8, "PADDSW: basic addition");
}

#[test]
fn test_paddsw_positive_saturation() {
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00,
        0x00, 0x0f, 0xed, 0xc1, 0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, 0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    // 32767 + 1 = saturate to 32767
    write_mm_via_mem(&mem, 0x2000, 0x7FFF7FFF7FFF7FFF);
    write_mm_via_mem(&mem, 0x2008, 0x0001000100010001);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_mem_at_u64(&mem, 0x2010);
    assert_eq!(result, 0x7FFF7FFF7FFF7FFF, "PADDSW: positive saturation");
}

#[test]
fn test_paddsw_negative_saturation() {
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00,
        0x00, 0x0f, 0xed, 0xc1, 0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, 0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    // -32768 + (-1) = saturate to -32768
    write_mm_via_mem(&mem, 0x2000, 0x8000800080008000);
    write_mm_via_mem(&mem, 0x2008, 0xFFFFFFFFFFFFFFFF);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_mem_at_u64(&mem, 0x2010);
    assert_eq!(result, 0x8000800080008000, "PADDSW: negative saturation");
}

#[test]
fn test_paddsw_mixed_signs() {
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00,
        0x00, 0x0f, 0xed, 0xc1, 0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, 0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    // 1000 + (-500) = 500, -1000 + 500 = -500
    write_mm_via_mem(&mem, 0x2000, 0xFC1803E8FC1803E8);
    write_mm_via_mem(&mem, 0x2008, 0x01F4FE0C01F4FE0C);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_mem_at_u64(&mem, 0x2010);
    assert_eq!(result, 0xFE0C01F4FE0C01F4, "PADDSW: mixed signs");
}

#[test]
fn test_paddsw_all_zeros() {
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00,
        0x00, 0x0f, 0xed, 0xc1, 0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, 0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mm_via_mem(&mem, 0x2000, 0x0000000000000000);
    write_mm_via_mem(&mem, 0x2008, 0x0000000000000000);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_mem_at_u64(&mem, 0x2010);
    assert_eq!(result, 0x0000000000000000, "PADDSW: all zeros");
}

#[test]
fn test_paddsw_no_saturation_positive() {
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00,
        0x00, 0x0f, 0xed, 0xc1, 0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, 0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    // 16000 + 16000 = 32000 (no saturation)
    write_mm_via_mem(&mem, 0x2000, 0x3E803E803E803E80);
    write_mm_via_mem(&mem, 0x2008, 0x3E803E803E803E80);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_mem_at_u64(&mem, 0x2010);
    assert_eq!(
        result, 0x7D007D007D007D00,
        "PADDSW: no saturation (positive)"
    );
}

#[test]
fn test_paddsw_no_saturation_negative() {
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00,
        0x00, 0x0f, 0xed, 0xc1, 0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, 0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    // -16000 + (-16000) = -32000 (no saturation)
    write_mm_via_mem(&mem, 0x2000, 0xC180C180C180C180);
    write_mm_via_mem(&mem, 0x2008, 0xC180C180C180C180);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_mem_at_u64(&mem, 0x2010);
    assert_eq!(
        result, 0x8300830083008300,
        "PADDSW: no saturation (negative)"
    );
}

#[test]
fn test_paddsw_edge_saturation_positive() {
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00,
        0x00, 0x0f, 0xed, 0xc1, 0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, 0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    // 32767 + 32767 = saturate to 32767
    write_mm_via_mem(&mem, 0x2000, 0x7FFF7FFF7FFF7FFF);
    write_mm_via_mem(&mem, 0x2008, 0x7FFF7FFF7FFF7FFF);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_mem_at_u64(&mem, 0x2010);
    assert_eq!(
        result, 0x7FFF7FFF7FFF7FFF,
        "PADDSW: edge saturation (positive)"
    );
}

#[test]
fn test_paddsw_edge_saturation_negative() {
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00,
        0x00, 0x0f, 0xed, 0xc1, 0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, 0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    // -32768 + (-32768) = saturate to -32768
    write_mm_via_mem(&mem, 0x2000, 0x8000800080008000);
    write_mm_via_mem(&mem, 0x2008, 0x8000800080008000);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_mem_at_u64(&mem, 0x2010);
    assert_eq!(
        result, 0x8000800080008000,
        "PADDSW: edge saturation (negative)"
    );
}

// Additional tests for PADDSB

#[test]
fn test_paddsb_boundary_values() {
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00,
        0x00, 0x0f, 0xec, 0xc1, 0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, 0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    // Test boundary values: 126+1=127, 127+0=127
    write_mm_via_mem(&mem, 0x2000, 0x7E7F00000000007E);
    write_mm_via_mem(&mem, 0x2008, 0x0100000000000001);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_mem_at_u64(&mem, 0x2010);
    assert_eq!(result, 0x7F7F00000000007F, "PADDSB: boundary values");
}

#[test]
fn test_paddsb_mm_m64() {
    let code = vec![
        0x0f, 0x6f, 0x14, 0x25, 0x00, 0x20, 0x00, 0x00, // MOVQ MM2, [0x2000]
        0x0f, 0xec, 0x14, 0x25, 0x08, 0x20, 0x00, 0x00, // PADDSB MM2, [0x2008]
        0x0f, 0x7f, 0x14, 0x25, 0x10, 0x20, 0x00, 0x00, // MOVQ [0x2010], MM2
        0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mm_via_mem(&mem, 0x2000, 0x0102030405060708);
    write_mm_via_mem(&mem, 0x2008, 0x0102030405060708);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_mem_at_u64(&mem, 0x2010);
    assert_eq!(result, 0x020406080A0C0E10, "PADDSB: memory operand");
}

#[test]
fn test_paddsw_mm_m64() {
    let code = vec![
        0x0f, 0x6f, 0x14, 0x25, 0x00, 0x20, 0x00, 0x00, // MOVQ MM2, [0x2000]
        0x0f, 0xed, 0x14, 0x25, 0x08, 0x20, 0x00, 0x00, // PADDSW MM2, [0x2008]
        0x0f, 0x7f, 0x14, 0x25, 0x10, 0x20, 0x00, 0x00, // MOVQ [0x2010], MM2
        0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mm_via_mem(&mem, 0x2000, 0x0001000200030004);
    write_mm_via_mem(&mem, 0x2008, 0x0001000200030004);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_mem_at_u64(&mem, 0x2010);
    assert_eq!(result, 0x0002000400060008, "PADDSW: memory operand");
}

#[test]
fn test_paddsb_sequential() {
    // Multiple PADDSB operations
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00,
        0x00, 0x0f, 0xec, 0xc1, // PADDSB MM0, MM1
        0x0f, 0xec, 0xc1, // PADDSB MM0, MM1 (again)
        0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, 0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mm_via_mem(&mem, 0x2000, 0x0A0A0A0A0A0A0A0A);
    write_mm_via_mem(&mem, 0x2008, 0x0505050505050505);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_mem_at_u64(&mem, 0x2010);
    // First: 10+5=15, Second: 15+5=20
    assert_eq!(result, 0x1414141414141414, "PADDSB: sequential operations");
}

#[test]
fn test_paddsw_sequential() {
    // Multiple PADDSW operations
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00,
        0x00, 0x0f, 0xed, 0xc1, // PADDSW MM0, MM1
        0x0f, 0xed, 0xc1, // PADDSW MM0, MM1 (again)
        0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, 0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mm_via_mem(&mem, 0x2000, 0x03E803E803E803E8); // 1000
    write_mm_via_mem(&mem, 0x2008, 0x01F401F401F401F4); // 500

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_mem_at_u64(&mem, 0x2010);
    // First: 1000+500=1500, Second: 1500+500=2000
    assert_eq!(result, 0x07D007D007D007D0, "PADDSW: sequential operations");
}

#[test]
fn test_paddsb_alternating_pattern() {
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00,
        0x00, 0x0f, 0xec, 0xc1, 0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, 0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    // Alternating positive and negative
    write_mm_via_mem(&mem, 0x2000, 0x0AF60AF60AF60AF6);
    write_mm_via_mem(&mem, 0x2008, 0xF60AF60AF60AF60A);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_mem_at_u64(&mem, 0x2010);
    assert_eq!(result, 0x0000000000000000, "PADDSB: alternating pattern");
}

#[test]
fn test_paddsw_alternating_pattern() {
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00,
        0x00, 0x0f, 0xed, 0xc1, 0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, 0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    // Alternating positive and negative
    write_mm_via_mem(&mem, 0x2000, 0x03E8FC1803E8FC18);
    write_mm_via_mem(&mem, 0x2008, 0xFC1803E8FC1803E8);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_mem_at_u64(&mem, 0x2010);
    assert_eq!(result, 0x0000000000000000, "PADDSW: alternating pattern");
}

#[test]
fn test_paddsb_saturation_mix() {
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00,
        0x00, 0x0f, 0xec, 0xc1, 0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, 0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    // Mix of saturating and non-saturating results
    write_mm_via_mem(&mem, 0x2000, 0x7F80010100000001);
    write_mm_via_mem(&mem, 0x2008, 0x01FF010100000001);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_mem_at_u64(&mem, 0x2010);
    // 1+1=2, 0+0=0, 1+1=2, 1+1=2, -128+(-1)=-128 (sat), 127+1=127 (sat)
    assert_eq!(result, 0x7F80020200000002, "PADDSB: saturation mix");
}

#[test]
fn test_paddsw_saturation_mix() {
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00,
        0x00, 0x0f, 0xed, 0xc1, 0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, 0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    // Mix of saturating and non-saturating results
    write_mm_via_mem(&mem, 0x2000, 0x7FFF800000010001);
    write_mm_via_mem(&mem, 0x2008, 0x0001FFFF00010001);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_mem_at_u64(&mem, 0x2010);
    // 1+1=2, 1+1=2, -32768+(-1)=-32768 (sat), 32767+1=32767 (sat)
    assert_eq!(result, 0x7FFF800000020002, "PADDSW: saturation mix");
}

#[test]
fn test_paddsb_all_mm_registers() {
    let code = vec![
        0x0f, 0x6f, 0x2c, 0x25, 0x00, 0x20, 0x00, 0x00, // MOVQ MM5, [0x2000]
        0x0f, 0x6f, 0x34, 0x25, 0x08, 0x20, 0x00, 0x00, // MOVQ MM6, [0x2008]
        0x0f, 0xec, 0xee, // PADDSB MM5, MM6
        0x0f, 0x7f, 0x2c, 0x25, 0x10, 0x20, 0x00, 0x00, // MOVQ [0x2010], MM5
        0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mm_via_mem(&mem, 0x2000, 0x0102030405060708);
    write_mm_via_mem(&mem, 0x2008, 0x0102030405060708);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_mem_at_u64(&mem, 0x2010);
    assert_eq!(result, 0x020406080A0C0E10, "PADDSB: MM5 and MM6");
}

#[test]
fn test_paddsw_all_mm_registers() {
    let code = vec![
        0x0f, 0x6f, 0x3c, 0x25, 0x00, 0x20, 0x00, 0x00, // MOVQ MM7, [0x2000]
        0x0f, 0x6f, 0x1c, 0x25, 0x08, 0x20, 0x00, 0x00, // MOVQ MM3, [0x2008]
        0x0f, 0xed, 0xfb, // PADDSW MM7, MM3
        0x0f, 0x7f, 0x3c, 0x25, 0x10, 0x20, 0x00, 0x00, // MOVQ [0x2010], MM7
        0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mm_via_mem(&mem, 0x2000, 0x0001000200030004);
    write_mm_via_mem(&mem, 0x2008, 0x0001000200030004);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_mem_at_u64(&mem, 0x2010);
    assert_eq!(result, 0x0002000400060008, "PADDSW: MM7 and MM3");
}

#[test]
fn test_paddsb_random_values() {
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00,
        0x00, 0x0f, 0xec, 0xc1, 0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, 0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    // Per-byte signed saturating add. Lanes (low->high):
    //   100+50=150  -> +127 (pos sat)    -100+-50=-150 -> -128 (neg sat)
    //   10+20=30                          -10+-20=-30
    //   127+127     -> +127 (pos sat)    -128+-128     -> -128 (neg sat)
    //   0+0=0                             64+(-64)=0
    write_mm_via_mem(&mem, 0x2000, 0x4000807FF60A9C64);
    write_mm_via_mem(&mem, 0x2008, 0xC000807FEC14CE32);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_mem_at_u64(&mem, 0x2010);
    assert_eq!(
        result, 0x0000807FE21E807F,
        "PADDSB: mixed values with saturation"
    );
}

#[test]
fn test_paddsw_random_values() {
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00,
        0x00, 0x0f, 0xed, 0xc1, 0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, 0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    // Per-word signed saturating add. Lanes (low->high):
    //   30000+5000=35000   -> +32767 (pos sat)
    //   -30000+-5000       -> -32768 (neg sat)
    //   1000+2000=3000
    //   -1000+500=-500
    write_mm_via_mem(&mem, 0x2000, 0xFC1803E88AD07530);
    write_mm_via_mem(&mem, 0x2008, 0x01F407D0EC781388);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_mem_at_u64(&mem, 0x2010);
    assert_eq!(
        result, 0xFE0C0BB880007FFF,
        "PADDSW: mixed values with saturation"
    );
}
