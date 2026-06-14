//! Tests for EMMS instruction.
//!
//! EMMS - Empty MMX Technology State
//!
//! Sets x87 FPU tag word to empty (all 1s). This marks the MMX/x87 registers
//! as available for x87 floating-point use. Must be called after MMX operations
//! before using x87 FP instructions.
//!
//! Flags affected: None
//!
//! Reference: docs/emms.txt

use crate::common::*;

fn write_mm_via_mem(mem: &vm_memory::GuestMemoryMmap, addr: u64, value: u64) {
    write_mem_at_u64(mem, addr, value);
}

// ============================================================================
// EMMS (opcode 0F 77)
// ============================================================================

#[test]
fn test_emms_basic() {
    // EMMS after a simple MMX operation
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // MOVQ MM0, [0x2000]
        0x0f, 0x77, // EMMS
        0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mm_via_mem(&mem, 0x2000, 0x1234567890ABCDEF);

    let result = run_until_hlt(&mut vcpu);
    assert!(result.is_ok(), "EMMS: basic execution");
}

#[test]
fn test_emms_after_paddb() {
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00,
        0x00, 0x0f, 0xfc, 0xc1, // PADDB MM0, MM1
        0x0f, 0x77, // EMMS
        0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mm_via_mem(&mem, 0x2000, 0x0102030405060708);
    write_mm_via_mem(&mem, 0x2008, 0x0101010101010101);

    let result = run_until_hlt(&mut vcpu);
    assert!(result.is_ok(), "EMMS: after PADDB");
}

#[test]
fn test_emms_after_paddw() {
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00,
        0x00, 0x0f, 0xfd, 0xc1, // PADDW MM0, MM1
        0x0f, 0x77, // EMMS
        0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mm_via_mem(&mem, 0x2000, 0x0001000200030004);
    write_mm_via_mem(&mem, 0x2008, 0x0001000100010001);

    let result = run_until_hlt(&mut vcpu);
    assert!(result.is_ok(), "EMMS: after PADDW");
}

#[test]
fn test_emms_after_paddd() {
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00,
        0x00, 0x0f, 0xfe, 0xc1, // PADDD MM0, MM1
        0x0f, 0x77, // EMMS
        0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mm_via_mem(&mem, 0x2000, 0x0000000100000002);
    write_mm_via_mem(&mem, 0x2008, 0x0000000100000001);

    let result = run_until_hlt(&mut vcpu);
    assert!(result.is_ok(), "EMMS: after PADDD");
}

#[test]
fn test_emms_after_psubb() {
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00,
        0x00, 0x0f, 0xf8, 0xc1, // PSUBB MM0, MM1
        0x0f, 0x77, // EMMS
        0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mm_via_mem(&mem, 0x2000, 0x0A09080706050403);
    write_mm_via_mem(&mem, 0x2008, 0x0101010101010101);

    let result = run_until_hlt(&mut vcpu);
    assert!(result.is_ok(), "EMMS: after PSUBB");
}

#[test]
fn test_emms_after_psubw() {
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00,
        0x00, 0x0f, 0xf9, 0xc1, // PSUBW MM0, MM1
        0x0f, 0x77, // EMMS
        0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mm_via_mem(&mem, 0x2000, 0x000A0009000800007);
    write_mm_via_mem(&mem, 0x2008, 0x0001000100010001);

    let result = run_until_hlt(&mut vcpu);
    assert!(result.is_ok(), "EMMS: after PSUBW");
}

#[test]
fn test_emms_after_psubd() {
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00,
        0x00, 0x0f, 0xfa, 0xc1, // PSUBD MM0, MM1
        0x0f, 0x77, // EMMS
        0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mm_via_mem(&mem, 0x2000, 0x0000000A00000009);
    write_mm_via_mem(&mem, 0x2008, 0x0000000100000001);

    let result = run_until_hlt(&mut vcpu);
    assert!(result.is_ok(), "EMMS: after PSUBD");
}

#[test]
fn test_emms_after_pmullw() {
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00,
        0x00, 0x0f, 0xd5, 0xc1, // PMULLW MM0, MM1
        0x0f, 0x77, // EMMS
        0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mm_via_mem(&mem, 0x2000, 0x0004000300020001);
    write_mm_via_mem(&mem, 0x2008, 0x0005000400030002);

    let result = run_until_hlt(&mut vcpu);
    assert!(result.is_ok(), "EMMS: after PMULLW");
}

#[test]
fn test_emms_after_pmulhw() {
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00,
        0x00, 0x0f, 0xe5, 0xc1, // PMULHW MM0, MM1
        0x0f, 0x77, // EMMS
        0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mm_via_mem(&mem, 0x2000, 0x1000100010001000);
    write_mm_via_mem(&mem, 0x2008, 0x1000100010001000);

    let result = run_until_hlt(&mut vcpu);
    assert!(result.is_ok(), "EMMS: after PMULHW");
}

#[test]
fn test_emms_after_pand() {
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00,
        0x00, 0x0f, 0xdb, 0xc1, // PAND MM0, MM1
        0x0f, 0x77, // EMMS
        0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mm_via_mem(&mem, 0x2000, 0xFFFFFFFF00000000);
    write_mm_via_mem(&mem, 0x2008, 0xFF00FF00FF00FF00);

    let result = run_until_hlt(&mut vcpu);
    assert!(result.is_ok(), "EMMS: after PAND");
}

#[test]
fn test_emms_after_por() {
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00,
        0x00, 0x0f, 0xeb, 0xc1, // POR MM0, MM1
        0x0f, 0x77, // EMMS
        0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mm_via_mem(&mem, 0x2000, 0xFF000000FF000000);
    write_mm_via_mem(&mem, 0x2008, 0x00FF000000FF0000);

    let result = run_until_hlt(&mut vcpu);
    assert!(result.is_ok(), "EMMS: after POR");
}

#[test]
fn test_emms_after_pxor() {
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00,
        0x00, 0x0f, 0xef, 0xc1, // PXOR MM0, MM1
        0x0f, 0x77, // EMMS
        0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mm_via_mem(&mem, 0x2000, 0xFF00FF00FF00FF00);
    write_mm_via_mem(&mem, 0x2008, 0xFFFF0000FFFF0000);

    let result = run_until_hlt(&mut vcpu);
    assert!(result.is_ok(), "EMMS: after PXOR");
}

#[test]
fn test_emms_after_pcmpeqb() {
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00,
        0x00, 0x0f, 0x74, 0xc1, // PCMPEQB MM0, MM1
        0x0f, 0x77, // EMMS
        0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mm_via_mem(&mem, 0x2000, 0x0102030405060708);
    write_mm_via_mem(&mem, 0x2008, 0x0102030405060708);

    let result = run_until_hlt(&mut vcpu);
    assert!(result.is_ok(), "EMMS: after PCMPEQB");
}

#[test]
fn test_emms_after_pcmpeqw() {
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00,
        0x00, 0x0f, 0x75, 0xc1, // PCMPEQW MM0, MM1
        0x0f, 0x77, // EMMS
        0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mm_via_mem(&mem, 0x2000, 0x1234567890ABCDEF);
    write_mm_via_mem(&mem, 0x2008, 0x1234567890ABCDEF);

    let result = run_until_hlt(&mut vcpu);
    assert!(result.is_ok(), "EMMS: after PCMPEQW");
}

#[test]
fn test_emms_after_pcmpeqd() {
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00,
        0x00, 0x0f, 0x76, 0xc1, // PCMPEQD MM0, MM1
        0x0f, 0x77, // EMMS
        0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mm_via_mem(&mem, 0x2000, 0x1234567890ABCDEF);
    write_mm_via_mem(&mem, 0x2008, 0x1234567890ABCDEF);

    let result = run_until_hlt(&mut vcpu);
    assert!(result.is_ok(), "EMMS: after PCMPEQD");
}

#[test]
fn test_emms_multiple_ops() {
    // EMMS after sequence of operations
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00,
        0x00, 0x0f, 0xfc, 0xc1, // PADDB MM0, MM1
        0x0f, 0xd5, 0xc1, // PMULLW MM0, MM1
        0x0f, 0xdb, 0xc1, // PAND MM0, MM1
        0x0f, 0x77, // EMMS
        0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mm_via_mem(&mem, 0x2000, 0x0102030405060708);
    write_mm_via_mem(&mem, 0x2008, 0x0101010101010101);

    let result = run_until_hlt(&mut vcpu);
    assert!(result.is_ok(), "EMMS: after multiple ops");
}

#[test]
fn test_emms_all_registers() {
    // Use all MM registers then EMMS
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // MOVQ MM0
        0x0f, 0x6f, 0x0c, 0x25, 0x00, 0x20, 0x00, 0x00, // MOVQ MM1
        0x0f, 0x6f, 0x14, 0x25, 0x00, 0x20, 0x00, 0x00, // MOVQ MM2
        0x0f, 0x6f, 0x1c, 0x25, 0x00, 0x20, 0x00, 0x00, // MOVQ MM3
        0x0f, 0x6f, 0x24, 0x25, 0x00, 0x20, 0x00, 0x00, // MOVQ MM4
        0x0f, 0x6f, 0x2c, 0x25, 0x00, 0x20, 0x00, 0x00, // MOVQ MM5
        0x0f, 0x6f, 0x34, 0x25, 0x00, 0x20, 0x00, 0x00, // MOVQ MM6
        0x0f, 0x6f, 0x3c, 0x25, 0x00, 0x20, 0x00, 0x00, // MOVQ MM7
        0x0f, 0x77, // EMMS
        0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mm_via_mem(&mem, 0x2000, 0x1234567890ABCDEF);

    let result = run_until_hlt(&mut vcpu);
    assert!(result.is_ok(), "EMMS: all registers used");
}

#[test]
fn test_emms_standalone() {
    // EMMS without prior MMX operations (should still work)
    let code = vec![
        0x0f, 0x77, // EMMS
        0xf4,
    ];

    let (mut vcpu, _) = setup_vm(&code, None);

    let result = run_until_hlt(&mut vcpu);
    assert!(result.is_ok(), "EMMS: standalone");
}

#[test]
fn test_emms_double() {
    // Double EMMS (redundant but valid)
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0f, 0x77, // EMMS
        0x0f, 0x77, // EMMS again
        0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mm_via_mem(&mem, 0x2000, 0x1234567890ABCDEF);

    let result = run_until_hlt(&mut vcpu);
    assert!(result.is_ok(), "EMMS: double");
}

#[test]
fn test_emms_with_data_preservation() {
    // Verify that EMMS doesn't destroy MMX register data (only tag word)
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0f, 0x77, // EMMS
        0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, // MOVQ [0x2010], MM0
        0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mm_via_mem(&mem, 0x2000, 0x1234567890ABCDEF);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_mem_at_u64(&mem, 0x2010);
    assert_eq!(result, 0x1234567890ABCDEF, "EMMS: data preserved");
}

#[test]
fn test_emms_after_complex_sequence() {
    // Complex sequence of different MMX operations
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00,
        0x00, 0x0f, 0x6f, 0x14, 0x25, 0x10, 0x20, 0x00, 0x00, 0x0f, 0xfc, 0xc1, // PADDB
        0x0f, 0xfd, 0xc1, // PADDW
        0x0f, 0xfe, 0xc1, // PADDD
        0x0f, 0xf8, 0xca, // PSUBB
        0x0f, 0xd5, 0xca, // PMULLW
        0x0f, 0xdb, 0xca, // PAND
        0x0f, 0xeb, 0xca, // POR
        0x0f, 0xef, 0xca, // PXOR
        0x0f, 0x77, // EMMS
        0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mm_via_mem(&mem, 0x2000, 0x0102030405060708);
    write_mm_via_mem(&mem, 0x2008, 0x0101010101010101);
    write_mm_via_mem(&mem, 0x2010, 0xFFFFFFFFFFFFFFFF);

    let result = run_until_hlt(&mut vcpu);
    assert!(result.is_ok(), "EMMS: complex sequence");
}

#[test]
fn test_emms_after_compares() {
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00,
        0x00, 0x0f, 0x74, 0xc1, // PCMPEQB
        0x0f, 0x75, 0xc1, // PCMPEQW
        0x0f, 0x76, 0xc1, // PCMPEQD
        0x0f, 0x77, // EMMS
        0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mm_via_mem(&mem, 0x2000, 0x1234567890ABCDEF);
    write_mm_via_mem(&mem, 0x2008, 0x1234567890ABCDEF);

    let result = run_until_hlt(&mut vcpu);
    assert!(result.is_ok(), "EMMS: after compares");
}

#[test]
fn test_emms_in_loop_pattern() {
    // Simulates pattern of MMX op -> EMMS in a loop
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00,
        0x00, 0x0f, 0xfc, 0xc1, // PADDB
        0x0f, 0x77, // EMMS
        0x0f, 0x6f, 0x14, 0x25, 0x10, 0x20, 0x00, 0x00, 0x0f, 0xfd, 0xd1, // PADDW
        0x0f, 0x77, // EMMS
        0x0f, 0x6f, 0x1c, 0x25, 0x18, 0x20, 0x00, 0x00, 0x0f, 0xfe, 0xda, // PADDD
        0x0f, 0x77, // EMMS
        0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mm_via_mem(&mem, 0x2000, 0x0101010101010101);
    write_mm_via_mem(&mem, 0x2008, 0x0202020202020202);
    write_mm_via_mem(&mem, 0x2010, 0x0003000300030003);
    write_mm_via_mem(&mem, 0x2018, 0x0000000400000004);

    let result = run_until_hlt(&mut vcpu);
    assert!(result.is_ok(), "EMMS: loop pattern");
}
