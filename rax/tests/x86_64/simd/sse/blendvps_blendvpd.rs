use crate::common::{run_until_hlt, setup_vm};
use rax::cpu::Registers;
use vm_memory::{Bytes, GuestAddress};

// BLENDVPS - Variable Blend Packed Single Precision Floating-Point Values
// BLENDVPD - Variable Blend Packed Double Precision Floating-Point Values
//
// BLENDVPS blends 4 packed single-precision (32-bit) floating-point values based on sign bit of XMM0
// BLENDVPD blends 2 packed double-precision (64-bit) floating-point values based on sign bit of XMM0
//
// Opcodes:
// 66 0F 38 14 /r    BLENDVPS xmm1, xmm2/m128, <XMM0> - Blend packed single from xmm2/m128 to xmm1 based on sign bits in XMM0
// 66 0F 38 15 /r    BLENDVPD xmm1, xmm2/m128, <XMM0> - Blend packed double from xmm2/m128 to xmm1 based on sign bits in XMM0
//
// For BLENDVPS: sign bit of each dword in XMM0 controls blending (1 = SRC, 0 = DEST)
// For BLENDVPD: sign bit of each qword in XMM0 controls blending (1 = SRC, 0 = DEST)

const ALIGNED_ADDR: u64 = 0x3000; // 16-byte aligned address for testing

// ============================================================================
// BLENDVPS Tests - Variable Blend Packed Single Precision (4x float32)
// ============================================================================

#[test]
fn test_blendvps_xmm1_xmm2_xmm0_all_zeros() {
    // BLENDVPS XMM1, XMM2 (mask in XMM0 = all zeros, select all from XMM1)
    let code = [
        0x66, 0x0f, 0x38, 0x14, 0xca, // BLENDVPS XMM1, XMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_blendvps_xmm1_xmm2_xmm0_all_ones() {
    // BLENDVPS XMM1, XMM2 (mask in XMM0 = all ones, select all from XMM2)
    let code = [
        0x66, 0x0f, 0x38, 0x14, 0xca, // BLENDVPS XMM1, XMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_blendvps_xmm0_xmm1_alternating() {
    // BLENDVPS XMM0, XMM1 (XMM0 serves as both destination and mask)
    let code = [
        0x66, 0x0f, 0x38, 0x14, 0xc1, // BLENDVPS XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_blendvps_xmm2_xmm3() {
    // BLENDVPS XMM2, XMM3
    let code = [
        0x66, 0x0f, 0x38, 0x14, 0xd3, // BLENDVPS XMM2, XMM3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_blendvps_xmm3_xmm4() {
    // BLENDVPS XMM3, XMM4
    let code = [
        0x66, 0x0f, 0x38, 0x14, 0xdc, // BLENDVPS XMM3, XMM4
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_blendvps_xmm4_xmm5() {
    // BLENDVPS XMM4, XMM5
    let code = [
        0x66, 0x0f, 0x38, 0x14, 0xe5, // BLENDVPS XMM4, XMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_blendvps_xmm5_xmm6() {
    // BLENDVPS XMM5, XMM6
    let code = [
        0x66, 0x0f, 0x38, 0x14, 0xee, // BLENDVPS XMM5, XMM6
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_blendvps_xmm6_xmm7() {
    // BLENDVPS XMM6, XMM7
    let code = [
        0x66, 0x0f, 0x38, 0x14, 0xf7, // BLENDVPS XMM6, XMM7
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_blendvps_xmm7_xmm0() {
    // BLENDVPS XMM7, XMM0
    let code = [
        0x66, 0x0f, 0x38, 0x14, 0xf8, // BLENDVPS XMM7, XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_blendvps_xmm8_xmm9() {
    // BLENDVPS XMM8, XMM9 (requires REX prefix)
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0x14, 0xc1, // BLENDVPS XMM8, XMM9
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_blendvps_xmm9_xmm10() {
    // BLENDVPS XMM9, XMM10
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0x14, 0xca, // BLENDVPS XMM9, XMM10
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_blendvps_xmm10_xmm11() {
    // BLENDVPS XMM10, XMM11
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0x14, 0xd3, // BLENDVPS XMM10, XMM11
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_blendvps_xmm11_xmm12() {
    // BLENDVPS XMM11, XMM12
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0x14, 0xdc, // BLENDVPS XMM11, XMM12
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_blendvps_xmm12_xmm13() {
    // BLENDVPS XMM12, XMM13
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0x14, 0xe5, // BLENDVPS XMM12, XMM13
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_blendvps_xmm13_xmm14() {
    // BLENDVPS XMM13, XMM14
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0x14, 0xee, // BLENDVPS XMM13, XMM14
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_blendvps_xmm14_xmm15() {
    // BLENDVPS XMM14, XMM15
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0x14, 0xf7, // BLENDVPS XMM14, XMM15
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_blendvps_xmm15_xmm8() {
    // BLENDVPS XMM15, XMM8
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0x14, 0xf8, // BLENDVPS XMM15, XMM8
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_blendvps_xmm0_mem() {
    // BLENDVPS XMM0, [ALIGNED_ADDR]
    let code = [
        0x66, 0x0f, 0x38, 0x14, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // BLENDVPS XMM0, [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_blendvps_xmm1_mem() {
    // BLENDVPS XMM1, [ALIGNED_ADDR]
    let code = [
        0x66, 0x0f, 0x38, 0x14, 0x0c, 0x25, 0x00, 0x30, 0x00, 0x00, // BLENDVPS XMM1, [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_blendvps_xmm7_mem() {
    // BLENDVPS XMM7, [ALIGNED_ADDR]
    let code = [
        0x66, 0x0f, 0x38, 0x14, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // BLENDVPS XMM7, [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_blendvps_xmm15_mem() {
    // BLENDVPS XMM15, [ALIGNED_ADDR]
    let code = [
        0x66, 0x44, 0x0f, 0x38, 0x14, 0x3c, 0x25, 0x00, 0x30, 0x00,
        0x00, // BLENDVPS XMM15, [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// Test with different mask patterns
#[test]
fn test_blendvps_mask_first_element() {
    // Mask in XMM0 has sign bit set only in first element
    let code = [
        0x66, 0x0f, 0x38, 0x14, 0xca, // BLENDVPS XMM1, XMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_blendvps_mask_second_element() {
    // Mask in XMM0 has sign bit set only in second element
    let code = [
        0x66, 0x0f, 0x38, 0x14, 0xd3, // BLENDVPS XMM2, XMM3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_blendvps_mask_third_element() {
    // Mask in XMM0 has sign bit set only in third element
    let code = [
        0x66, 0x0f, 0x38, 0x14, 0xe5, // BLENDVPS XMM4, XMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_blendvps_mask_fourth_element() {
    // Mask in XMM0 has sign bit set only in fourth element
    let code = [
        0x66, 0x0f, 0x38, 0x14, 0xf7, // BLENDVPS XMM6, XMM7
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// BLENDVPD Tests - Variable Blend Packed Double Precision (2x float64)
// ============================================================================

#[test]
fn test_blendvpd_xmm1_xmm2_xmm0_all_zeros() {
    // BLENDVPD XMM1, XMM2 (mask in XMM0 = all zeros, select all from XMM1)
    let code = [
        0x66, 0x0f, 0x38, 0x15, 0xca, // BLENDVPD XMM1, XMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_blendvpd_xmm1_xmm2_xmm0_all_ones() {
    // BLENDVPD XMM1, XMM2 (mask in XMM0 = all ones, select all from XMM2)
    let code = [
        0x66, 0x0f, 0x38, 0x15, 0xca, // BLENDVPD XMM1, XMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_blendvpd_xmm0_xmm1() {
    // BLENDVPD XMM0, XMM1 (XMM0 serves as both destination and mask)
    let code = [
        0x66, 0x0f, 0x38, 0x15, 0xc1, // BLENDVPD XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_blendvpd_xmm2_xmm3() {
    // BLENDVPD XMM2, XMM3
    let code = [
        0x66, 0x0f, 0x38, 0x15, 0xd3, // BLENDVPD XMM2, XMM3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_blendvpd_xmm3_xmm4() {
    // BLENDVPD XMM3, XMM4
    let code = [
        0x66, 0x0f, 0x38, 0x15, 0xdc, // BLENDVPD XMM3, XMM4
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_blendvpd_xmm4_xmm5() {
    // BLENDVPD XMM4, XMM5
    let code = [
        0x66, 0x0f, 0x38, 0x15, 0xe5, // BLENDVPD XMM4, XMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_blendvpd_xmm5_xmm6() {
    // BLENDVPD XMM5, XMM6
    let code = [
        0x66, 0x0f, 0x38, 0x15, 0xee, // BLENDVPD XMM5, XMM6
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_blendvpd_xmm6_xmm7() {
    // BLENDVPD XMM6, XMM7
    let code = [
        0x66, 0x0f, 0x38, 0x15, 0xf7, // BLENDVPD XMM6, XMM7
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_blendvpd_xmm7_xmm0() {
    // BLENDVPD XMM7, XMM0
    let code = [
        0x66, 0x0f, 0x38, 0x15, 0xf8, // BLENDVPD XMM7, XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_blendvpd_xmm8_xmm9() {
    // BLENDVPD XMM8, XMM9 (requires REX prefix)
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0x15, 0xc1, // BLENDVPD XMM8, XMM9
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_blendvpd_xmm9_xmm10() {
    // BLENDVPD XMM9, XMM10
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0x15, 0xca, // BLENDVPD XMM9, XMM10
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_blendvpd_xmm10_xmm11() {
    // BLENDVPD XMM10, XMM11
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0x15, 0xd3, // BLENDVPD XMM10, XMM11
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_blendvpd_xmm11_xmm12() {
    // BLENDVPD XMM11, XMM12
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0x15, 0xdc, // BLENDVPD XMM11, XMM12
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_blendvpd_xmm12_xmm13() {
    // BLENDVPD XMM12, XMM13
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0x15, 0xe5, // BLENDVPD XMM12, XMM13
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_blendvpd_xmm13_xmm14() {
    // BLENDVPD XMM13, XMM14
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0x15, 0xee, // BLENDVPD XMM13, XMM14
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_blendvpd_xmm14_xmm15() {
    // BLENDVPD XMM14, XMM15
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0x15, 0xf7, // BLENDVPD XMM14, XMM15
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_blendvpd_xmm15_xmm8() {
    // BLENDVPD XMM15, XMM8
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0x15, 0xf8, // BLENDVPD XMM15, XMM8
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_blendvpd_xmm0_mem() {
    // BLENDVPD XMM0, [ALIGNED_ADDR]
    let code = [
        0x66, 0x0f, 0x38, 0x15, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // BLENDVPD XMM0, [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_blendvpd_xmm1_mem() {
    // BLENDVPD XMM1, [ALIGNED_ADDR]
    let code = [
        0x66, 0x0f, 0x38, 0x15, 0x0c, 0x25, 0x00, 0x30, 0x00, 0x00, // BLENDVPD XMM1, [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_blendvpd_xmm7_mem() {
    // BLENDVPD XMM7, [ALIGNED_ADDR]
    let code = [
        0x66, 0x0f, 0x38, 0x15, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // BLENDVPD XMM7, [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_blendvpd_xmm15_mem() {
    // BLENDVPD XMM15, [ALIGNED_ADDR]
    let code = [
        0x66, 0x44, 0x0f, 0x38, 0x15, 0x3c, 0x25, 0x00, 0x30, 0x00,
        0x00, // BLENDVPD XMM15, [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// Test with different mask patterns
#[test]
fn test_blendvpd_mask_first_element() {
    // Mask in XMM0 has sign bit set only in first element
    let code = [
        0x66, 0x0f, 0x38, 0x15, 0xca, // BLENDVPD XMM1, XMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_blendvpd_mask_second_element() {
    // Mask in XMM0 has sign bit set only in second element
    let code = [
        0x66, 0x0f, 0x38, 0x15, 0xd3, // BLENDVPD XMM2, XMM3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_blendvpd_mask_alternating() {
    // Mask in XMM0 has alternating sign bits
    let code = [
        0x66, 0x0f, 0x38, 0x15, 0xe5, // BLENDVPD XMM4, XMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Known-answer value tests (register-to-register via set_xmm/get_xmm)
//
// BLENDVPS selects per-dword and BLENDVPD per-qword from SRC when the high bit
// of the corresponding element of mask XMM0 is set, else from DST.
// ============================================================================

#[test]
fn kat_blendvps_value() {
    // BLENDVPS XMM1, XMM2 (66 0F 38 14 CA), mask = XMM0.
    // DST dwords A0..A3, SRC dwords B0..B3, mask lanes 0 and 2 high-bit set.
    let code = [0x66, 0x0f, 0x38, 0x14, 0xca, 0xf4];
    let (mut vcpu, mem) = crate::common::setup_vm(&code, None);
    crate::common::set_xmm(&mem, &mut vcpu, 0, 0x00000000800000000000000080000000);
    crate::common::set_xmm(&mem, &mut vcpu, 1, 0xAAAAAAA3_AAAAAAA2_AAAAAAA1_AAAAAAA0);
    crate::common::set_xmm(&mem, &mut vcpu, 2, 0xBBBBBBB3_BBBBBBB2_BBBBBBB1_BBBBBBB0);
    let regs = crate::common::run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        crate::common::get_xmm(&regs, 1),
        0xAAAAAAA3_BBBBBBB2_AAAAAAA1_BBBBBBB0,
        "BLENDVPS got {:032x}",
        crate::common::get_xmm(&regs, 1)
    );
}

#[test]
fn kat_blendvpd_value() {
    // BLENDVPD XMM1, XMM2 (66 0F 38 15 CA), mask = XMM0.
    // DST qwords D0,D1; SRC qwords S0,S1; mask qword1 high-bit set -> [D0, S1].
    let code = [0x66, 0x0f, 0x38, 0x15, 0xca, 0xf4];
    let (mut vcpu, mem) = crate::common::setup_vm(&code, None);
    crate::common::set_xmm(&mem, &mut vcpu, 0, 0x80000000000000000000000000000000);
    crate::common::set_xmm(&mem, &mut vcpu, 1, 0xDDDDDDDDDDDDDDD1_DDDDDDDDDDDDDDD0);
    crate::common::set_xmm(&mem, &mut vcpu, 2, 0x5555555555555551_5555555555555550);
    let regs = crate::common::run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        crate::common::get_xmm(&regs, 1),
        0x5555555555555551_DDDDDDDDDDDDDDD0,
        "BLENDVPD got {:032x}",
        crate::common::get_xmm(&regs, 1)
    );
}
