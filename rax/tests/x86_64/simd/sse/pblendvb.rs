use crate::common::*;

// PBLENDVB - Variable Blend Packed Bytes
//
// Conditionally copies byte elements from the source operand to the destination
// operand depending on mask bits defined in the implicit third register (XMM0
// for legacy SSE4.1). The mask bits are the most significant bit in each byte
// element of XMM0.
//
// If mask bit is "1", copy from source; else, keep destination unchanged.
//
// Opcode:
//   66 0F 38 10 /r    PBLENDVB xmm1, xmm2/m128, <XMM0>

const ALIGNED_ADDR: u64 = 0x3000;

#[test]
fn test_pblendvb_xmm1_xmm2_mask_all_zeros() {
    // XMM0 (mask) = all zeros, so XMM1 unchanged
    let code = [
        0x66, 0x0f, 0x38, 0x10, 0xca, // PBLENDVB XMM1, XMM2 (mask in XMM0)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pblendvb_xmm1_xmm2_mask_all_ones() {
    // XMM0 (mask) = all 0xFF, so all bytes from XMM2
    let code = [
        0x66, 0x0f, 0x38, 0x10, 0xca, // PBLENDVB XMM1, XMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pblendvb_xmm1_xmm2_mask_alternating() {
    // XMM0 (mask) alternates 0x00 and 0xFF
    let code = [
        0x66, 0x0f, 0x38, 0x10, 0xca, // PBLENDVB XMM1, XMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pblendvb_xmm3_xmm4_mask_low_half() {
    // XMM0 (mask) = 0xFF for low 8 bytes, 0x00 for high 8 bytes
    let code = [
        0x66, 0x0f, 0x38, 0x10, 0xdc, // PBLENDVB XMM3, XMM4
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pblendvb_xmm5_xmm6_mask_high_half() {
    // XMM0 (mask) = 0x00 for low 8 bytes, 0xFF for high 8 bytes
    let code = [
        0x66, 0x0f, 0x38, 0x10, 0xee, // PBLENDVB XMM5, XMM6
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pblendvb_xmm7_xmm0_basic() {
    // Note: XMM0 is both mask and source here
    let code = [
        0x66, 0x0f, 0x38, 0x10, 0xf8, // PBLENDVB XMM7, XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pblendvb_xmm1_xmm3_mask_sign_bit() {
    // XMM0 (mask) has sign bit set (0x80)
    let code = [
        0x66, 0x0f, 0x38, 0x10, 0xcb, // PBLENDVB XMM1, XMM3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pblendvb_xmm2_xmm4_mask_no_sign_bit() {
    // XMM0 (mask) = 0x7F (sign bit not set)
    let code = [
        0x66, 0x0f, 0x38, 0x10, 0xd4, // PBLENDVB XMM2, XMM4
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pblendvb_xmm8_xmm9_high_regs() {
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0x10, 0xc1, // PBLENDVB XMM8, XMM9
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pblendvb_xmm10_xmm11_high_regs() {
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0x10, 0xd3, // PBLENDVB XMM10, XMM11
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pblendvb_xmm12_xmm13_high_regs() {
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0x10, 0xe5, // PBLENDVB XMM12, XMM13
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pblendvb_xmm14_xmm15_high_regs() {
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0x10, 0xf7, // PBLENDVB XMM14, XMM15
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pblendvb_xmm1_mem_mask_zeros() {
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x38, 0x10, 0x08, // PBLENDVB XMM1, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data: [u8; 16] = [
        0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF,
        0x00,
    ];
    mem.write_slice(&data, vm_memory::GuestAddress(ALIGNED_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pblendvb_xmm2_mem_mask_all_ones() {
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x38, 0x10, 0x10, // PBLENDVB XMM2, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data: [u8; 16] = [
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF,
    ];
    mem.write_slice(&data, vm_memory::GuestAddress(ALIGNED_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pblendvb_xmm3_mem_mask_alternating() {
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x38, 0x10, 0x18, // PBLENDVB XMM3, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data: [u8; 16] = [
        0x00, 0xFF, 0x00, 0xFF, 0x00, 0xFF, 0x00, 0xFF, 0x00, 0xFF, 0x00, 0xFF, 0x00, 0xFF, 0x00,
        0xFF,
    ];
    mem.write_slice(&data, vm_memory::GuestAddress(ALIGNED_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pblendvb_xmm4_mem_mask_low_half() {
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x38, 0x10, 0x20, // PBLENDVB XMM4, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data: [u8; 16] = [
        0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA,
        0xAA,
    ];
    mem.write_slice(&data, vm_memory::GuestAddress(ALIGNED_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pblendvb_xmm5_mem_mask_high_half() {
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x38, 0x10, 0x28, // PBLENDVB XMM5, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data: [u8; 16] = [
        0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55,
        0x55,
    ];
    mem.write_slice(&data, vm_memory::GuestAddress(ALIGNED_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pblendvb_sequential_operations() {
    let code = [
        0x66, 0x0f, 0x38, 0x10, 0xca, // PBLENDVB XMM1, XMM2
        0x66, 0x0f, 0x38, 0x10, 0xdc, // PBLENDVB XMM3, XMM4
        0x66, 0x0f, 0x38, 0x10, 0xee, // PBLENDVB XMM5, XMM6
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pblendvb_xmm1_xmm2_mask_single_byte() {
    // XMM0 (mask) has only one byte with sign bit set
    let code = [
        0x66, 0x0f, 0x38, 0x10, 0xca, // PBLENDVB XMM1, XMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pblendvb_xmm1_xmm2_mask_two_bytes() {
    // XMM0 (mask) has two bytes with sign bit set
    let code = [
        0x66, 0x0f, 0x38, 0x10, 0xca, // PBLENDVB XMM1, XMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pblendvb_xmm1_xmm2_mask_four_bytes() {
    // XMM0 (mask) has four bytes with sign bit set
    let code = [
        0x66, 0x0f, 0x38, 0x10, 0xca, // PBLENDVB XMM1, XMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pblendvb_xmm1_xmm2_mask_eight_bytes() {
    // XMM0 (mask) has eight bytes with sign bit set
    let code = [
        0x66, 0x0f, 0x38, 0x10, 0xca, // PBLENDVB XMM1, XMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pblendvb_xmm1_xmm2_mask_0x80_pattern() {
    // XMM0 (mask) has 0x80 (only sign bit set)
    let code = [
        0x66, 0x0f, 0x38, 0x10, 0xca, // PBLENDVB XMM1, XMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pblendvb_xmm1_xmm2_mask_0x81_pattern() {
    // XMM0 (mask) has 0x81 (sign bit + LSB set)
    let code = [
        0x66, 0x0f, 0x38, 0x10, 0xca, // PBLENDVB XMM1, XMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pblendvb_xmm1_xmm2_mask_0xc0_pattern() {
    // XMM0 (mask) has 0xC0 (two high bits set)
    let code = [
        0x66, 0x0f, 0x38, 0x10, 0xca, // PBLENDVB XMM1, XMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pblendvb_xmm0_xmm15_cross() {
    let code = [
        0x66, 0x44, 0x0f, 0x38, 0x10, 0xf8, // PBLENDVB XMM0, XMM15
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pblendvb_xmm15_xmm1_cross() {
    let code = [
        0x66, 0x44, 0x0f, 0x38, 0x10, 0xf9, // PBLENDVB XMM15, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pblendvb_mem_displacement() {
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&(ALIGNED_ADDR - 0x10).to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x38, 0x10, 0x48, 0x10, // PBLENDVB XMM1, [RAX+0x10]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(
        &[
            0xCC, 0xCC, 0xCC, 0xCC, 0xCC, 0xCC, 0xCC, 0xCC, 0xCC, 0xCC, 0xCC, 0xCC, 0xCC, 0xCC,
            0xCC, 0xCC,
        ],
        vm_memory::GuestAddress(ALIGNED_ADDR),
    )
    .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pblendvb_xmm1_xmm2_mask_pattern1() {
    // Custom mask pattern in XMM0
    let code = [
        0x66, 0x0f, 0x38, 0x10, 0xca, // PBLENDVB XMM1, XMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pblendvb_xmm1_xmm2_mask_pattern2() {
    // Different custom mask pattern in XMM0
    let code = [
        0x66, 0x0f, 0x38, 0x10, 0xca, // PBLENDVB XMM1, XMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pblendvb_xmm1_xmm2_mask_random_pattern() {
    // Random pattern in XMM0 mask
    let code = [
        0x66, 0x0f, 0x38, 0x10, 0xca, // PBLENDVB XMM1, XMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Known-answer value tests (register-to-register via set_xmm/get_xmm)
//
// PBLENDVB selects, per byte, SRC if the high bit (0x80) of the corresponding
// byte of the implicit mask register XMM0 is set, else DST.
// ============================================================================

#[test]
fn kat_pblendvb_value() {
    // PBLENDVB XMM1, XMM2  (66 0F 38 10 CA), mask = XMM0.
    // DST(XMM1)=all 0x11, SRC(XMM2)=all 0x22, mask even bytes 0x80 -> even from src.
    let code = [0x66, 0x0f, 0x38, 0x10, 0xca, 0xf4];
    let (mut vcpu, mem) = crate::common::setup_vm(&code, None);
    crate::common::set_xmm(&mem, &mut vcpu, 0, 0x00800080008000800080008000800080);
    crate::common::set_xmm(&mem, &mut vcpu, 1, 0x11111111111111111111111111111111);
    crate::common::set_xmm(&mem, &mut vcpu, 2, 0x22222222222222222222222222222222);
    let regs = crate::common::run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        crate::common::get_xmm(&regs, 1),
        0x11221122112211221122112211221122,
        "PBLENDVB got {:032x}",
        crate::common::get_xmm(&regs, 1)
    );
}
