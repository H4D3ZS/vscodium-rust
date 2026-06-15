use crate::common::{run_until_hlt, setup_vm};
use rax::cpu::Registers;
use vm_memory::{Bytes, GuestAddress};

// MASKMOVDQU - Store Selected Bytes of Double Quadword
//
// MASKMOVDQU selectively stores bytes from the source XMM register to memory location
// specified by DI/EDI/RDI using the mask in the second XMM register.
//
// The high bit of each byte in the mask determines whether the corresponding byte is written:
// - 0: no write
// - 1: write
//
// Memory location uses DS:DI/EDI/RDI (can be overridden with segment prefix)
// Non-temporal hint for cache pollution minimization
//
// Opcodes:
// 66 0F F7 /r             MASKMOVDQU xmm1, xmm2     - Store selected bytes from xmm1 to DS:[RDI] using mask in xmm2

const DEST_ADDR: u64 = 0x4000; // Destination address for masked store

// ============================================================================
// MASKMOVDQU Tests - Store Selected Bytes Using Mask
// ============================================================================

#[test]
fn test_maskmovdqu_xmm0_xmm1() {
    // MASKMOVDQU XMM0, XMM1
    let code = [
        0x66, 0x0f, 0xf7, 0xc1, // MASKMOVDQU XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_maskmovdqu_xmm1_xmm2() {
    // MASKMOVDQU XMM1, XMM2
    let code = [
        0x66, 0x0f, 0xf7, 0xca, // MASKMOVDQU XMM1, XMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_maskmovdqu_xmm2_xmm3() {
    // MASKMOVDQU XMM2, XMM3
    let code = [
        0x66, 0x0f, 0xf7, 0xd3, // MASKMOVDQU XMM2, XMM3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_maskmovdqu_xmm3_xmm4() {
    // MASKMOVDQU XMM3, XMM4
    let code = [
        0x66, 0x0f, 0xf7, 0xdc, // MASKMOVDQU XMM3, XMM4
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_maskmovdqu_xmm4_xmm5() {
    // MASKMOVDQU XMM4, XMM5
    let code = [
        0x66, 0x0f, 0xf7, 0xe5, // MASKMOVDQU XMM4, XMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_maskmovdqu_xmm5_xmm6() {
    // MASKMOVDQU XMM5, XMM6
    let code = [
        0x66, 0x0f, 0xf7, 0xee, // MASKMOVDQU XMM5, XMM6
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_maskmovdqu_xmm6_xmm7() {
    // MASKMOVDQU XMM6, XMM7
    let code = [
        0x66, 0x0f, 0xf7, 0xf7, // MASKMOVDQU XMM6, XMM7
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_maskmovdqu_xmm7_xmm0() {
    // MASKMOVDQU XMM7, XMM0
    let code = [
        0x66, 0x0f, 0xf7, 0xf8, // MASKMOVDQU XMM7, XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_maskmovdqu_xmm8_xmm9() {
    // MASKMOVDQU XMM8, XMM9 (requires REX prefix)
    let code = [
        0x66, 0x45, 0x0f, 0xf7, 0xc1, // MASKMOVDQU XMM8, XMM9
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_maskmovdqu_xmm10_xmm11() {
    // MASKMOVDQU XMM10, XMM11
    let code = [
        0x66, 0x45, 0x0f, 0xf7, 0xd3, // MASKMOVDQU XMM10, XMM11
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_maskmovdqu_xmm12_xmm13() {
    // MASKMOVDQU XMM12, XMM13
    let code = [
        0x66, 0x45, 0x0f, 0xf7, 0xe5, // MASKMOVDQU XMM12, XMM13
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_maskmovdqu_xmm14_xmm15() {
    // MASKMOVDQU XMM14, XMM15
    let code = [
        0x66, 0x45, 0x0f, 0xf7, 0xf7, // MASKMOVDQU XMM14, XMM15
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_maskmovdqu_xmm15_xmm0() {
    // MASKMOVDQU XMM15, XMM0
    let code = [
        0x66, 0x44, 0x0f, 0xf7, 0xf8, // MASKMOVDQU XMM15, XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_maskmovdqu_mask_all_zeros() {
    // Test with mask = 0 (no bytes written)
    let code = [
        0x66, 0x0f, 0xf7, 0xc1, // MASKMOVDQU XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_maskmovdqu_mask_all_ones() {
    // Test with mask = 0xFF..FF (all bytes written)
    let code = [
        0x66, 0x0f, 0xf7, 0xd3, // MASKMOVDQU XMM2, XMM3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_maskmovdqu_mask_first_byte() {
    // Test with only first byte mask bit set
    let code = [
        0x66, 0x0f, 0xf7, 0xe5, // MASKMOVDQU XMM4, XMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_maskmovdqu_mask_last_byte() {
    // Test with only last byte mask bit set
    let code = [
        0x66, 0x0f, 0xf7, 0xf7, // MASKMOVDQU XMM6, XMM7
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_maskmovdqu_mask_alternating() {
    // Test with alternating mask bits (0xAA in each byte)
    let code = [
        0x66, 0x45, 0x0f, 0xf7, 0xc1, // MASKMOVDQU XMM8, XMM9
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_maskmovdqu_mask_even_bytes() {
    // Test with even bytes masked (high bit set in even positions)
    let code = [
        0x66, 0x45, 0x0f, 0xf7, 0xd3, // MASKMOVDQU XMM10, XMM11
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_maskmovdqu_mask_odd_bytes() {
    // Test with odd bytes masked (high bit set in odd positions)
    let code = [
        0x66, 0x45, 0x0f, 0xf7, 0xe5, // MASKMOVDQU XMM12, XMM13
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_maskmovdqu_mask_first_half() {
    // Test with first 8 bytes masked
    let code = [
        0x66, 0x45, 0x0f, 0xf7, 0xf7, // MASKMOVDQU XMM14, XMM15
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_maskmovdqu_mask_second_half() {
    // Test with last 8 bytes masked
    let code = [
        0x66, 0x44, 0x0f, 0xf7, 0xf8, // MASKMOVDQU XMM15, XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_maskmovdqu_with_rdi() {
    // Test with RDI as destination pointer
    let code = [
        0x66, 0x0f, 0xf7, 0xc1, // MASKMOVDQU XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_maskmovdqu_single_byte_1() {
    // Test writing single byte at position 0
    let code = [
        0x66, 0x0f, 0xf7, 0xd3, // MASKMOVDQU XMM2, XMM3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_maskmovdqu_single_byte_2() {
    // Test writing single byte at position 7
    let code = [
        0x66, 0x0f, 0xf7, 0xe5, // MASKMOVDQU XMM4, XMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_maskmovdqu_single_byte_3() {
    // Test writing single byte at position 15
    let code = [
        0x66, 0x0f, 0xf7, 0xf7, // MASKMOVDQU XMM6, XMM7
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_maskmovdqu_non_temporal_hint() {
    // Test non-temporal hint behavior
    let code = [
        0x66, 0x45, 0x0f, 0xf7, 0xc1, // MASKMOVDQU XMM8, XMM9
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_maskmovdqu_sparse_mask_1() {
    // Test sparse mask pattern (bytes 0, 4, 8, 12)
    let code = [
        0x66, 0x45, 0x0f, 0xf7, 0xd3, // MASKMOVDQU XMM10, XMM11
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_maskmovdqu_sparse_mask_2() {
    // Test sparse mask pattern (bytes 1, 5, 9, 13)
    let code = [
        0x66, 0x45, 0x0f, 0xf7, 0xe5, // MASKMOVDQU XMM12, XMM13
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_maskmovdqu_continuous_bytes() {
    // Test continuous range of bytes (4-7)
    let code = [
        0x66, 0x45, 0x0f, 0xf7, 0xf7, // MASKMOVDQU XMM14, XMM15
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_maskmovdqu_verify_unmasked_unchanged() {
    // Test that unmasked bytes remain unchanged
    let code = [
        0x66, 0x44, 0x0f, 0xf7, 0xf8, // MASKMOVDQU XMM15, XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_maskmovdqu_same_register() {
    // Test with same register as source and mask
    let code = [
        0x66, 0x0f, 0xf7, 0xc0, // MASKMOVDQU XMM0, XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_maskmovdqu_complex_pattern_1() {
    // Test complex mask pattern
    let code = [
        0x66, 0x0f, 0xf7, 0xc1, // MASKMOVDQU XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_maskmovdqu_complex_pattern_2() {
    // Test another complex mask pattern
    let code = [
        0x66, 0x0f, 0xf7, 0xd3, // MASKMOVDQU XMM2, XMM3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_maskmovdqu_boundary_bytes() {
    // Test only boundary bytes (0 and 15)
    let code = [
        0x66, 0x0f, 0xf7, 0xe5, // MASKMOVDQU XMM4, XMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
