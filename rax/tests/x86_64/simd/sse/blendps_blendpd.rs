use crate::common::{run_until_hlt, setup_vm};
use rax::cpu::Registers;
use vm_memory::{Bytes, GuestAddress};

// BLENDPS - Blend Packed Single Precision Floating-Point Values
// BLENDPD - Blend Packed Double Precision Floating-Point Values
//
// BLENDPS blends 4 packed single-precision (32-bit) floating-point values based on immediate mask
// BLENDPD blends 2 packed double-precision (64-bit) floating-point values based on immediate mask
//
// Opcodes:
// 66 0F 3A 0C /r ib    BLENDPS xmm1, xmm2/m128, imm8 - Blend packed single from xmm2/m128 to xmm1 based on imm8 mask
// 66 0F 3A 0D /r ib    BLENDPD xmm1, xmm2/m128, imm8 - Blend packed double from xmm2/m128 to xmm1 based on imm8 mask
//
// For BLENDPS: bits[3:0] of imm8 control which elements are selected (1 = SRC, 0 = DEST)
// For BLENDPD: bits[1:0] of imm8 control which elements are selected (1 = SRC, 0 = DEST)

const ALIGNED_ADDR: u64 = 0x3000; // 16-byte aligned address for testing

// ============================================================================
// BLENDPS Tests - Packed Single Precision (4x float32)
// ============================================================================

#[test]
fn test_blendps_xmm0_xmm1_mask_0x0() {
    // BLENDPS XMM0, XMM1, 0x0 (all from dest)
    let code = [
        0x66, 0x0f, 0x3a, 0x0c, 0xc1, 0x00, // BLENDPS XMM0, XMM1, 0x0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_blendps_xmm0_xmm1_mask_0x1() {
    // BLENDPS XMM0, XMM1, 0x1 (blend first element)
    let code = [
        0x66, 0x0f, 0x3a, 0x0c, 0xc1, 0x01, // BLENDPS XMM0, XMM1, 0x1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_blendps_xmm0_xmm1_mask_0x2() {
    // BLENDPS XMM0, XMM1, 0x2 (blend second element)
    let code = [
        0x66, 0x0f, 0x3a, 0x0c, 0xc1, 0x02, // BLENDPS XMM0, XMM1, 0x2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_blendps_xmm0_xmm1_mask_0x3() {
    // BLENDPS XMM0, XMM1, 0x3 (blend first two elements)
    let code = [
        0x66, 0x0f, 0x3a, 0x0c, 0xc1, 0x03, // BLENDPS XMM0, XMM1, 0x3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_blendps_xmm0_xmm1_mask_0x4() {
    // BLENDPS XMM0, XMM1, 0x4 (blend third element)
    let code = [
        0x66, 0x0f, 0x3a, 0x0c, 0xc1, 0x04, // BLENDPS XMM0, XMM1, 0x4
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_blendps_xmm0_xmm1_mask_0x5() {
    // BLENDPS XMM0, XMM1, 0x5 (blend first and third elements)
    let code = [
        0x66, 0x0f, 0x3a, 0x0c, 0xc1, 0x05, // BLENDPS XMM0, XMM1, 0x5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_blendps_xmm0_xmm1_mask_0x6() {
    // BLENDPS XMM0, XMM1, 0x6 (blend second and third elements)
    let code = [
        0x66, 0x0f, 0x3a, 0x0c, 0xc1, 0x06, // BLENDPS XMM0, XMM1, 0x6
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_blendps_xmm0_xmm1_mask_0x7() {
    // BLENDPS XMM0, XMM1, 0x7 (blend first three elements)
    let code = [
        0x66, 0x0f, 0x3a, 0x0c, 0xc1, 0x07, // BLENDPS XMM0, XMM1, 0x7
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_blendps_xmm0_xmm1_mask_0x8() {
    // BLENDPS XMM0, XMM1, 0x8 (blend fourth element)
    let code = [
        0x66, 0x0f, 0x3a, 0x0c, 0xc1, 0x08, // BLENDPS XMM0, XMM1, 0x8
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_blendps_xmm0_xmm1_mask_0x9() {
    // BLENDPS XMM0, XMM1, 0x9 (blend first and fourth elements)
    let code = [
        0x66, 0x0f, 0x3a, 0x0c, 0xc1, 0x09, // BLENDPS XMM0, XMM1, 0x9
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_blendps_xmm0_xmm1_mask_0xa() {
    // BLENDPS XMM0, XMM1, 0xA (blend second and fourth elements)
    let code = [
        0x66, 0x0f, 0x3a, 0x0c, 0xc1, 0x0a, // BLENDPS XMM0, XMM1, 0xA
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_blendps_xmm0_xmm1_mask_0xb() {
    // BLENDPS XMM0, XMM1, 0xB (blend first, second, and fourth elements)
    let code = [
        0x66, 0x0f, 0x3a, 0x0c, 0xc1, 0x0b, // BLENDPS XMM0, XMM1, 0xB
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_blendps_xmm0_xmm1_mask_0xc() {
    // BLENDPS XMM0, XMM1, 0xC (blend third and fourth elements)
    let code = [
        0x66, 0x0f, 0x3a, 0x0c, 0xc1, 0x0c, // BLENDPS XMM0, XMM1, 0xC
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_blendps_xmm0_xmm1_mask_0xd() {
    // BLENDPS XMM0, XMM1, 0xD (blend first, third, and fourth elements)
    let code = [
        0x66, 0x0f, 0x3a, 0x0c, 0xc1, 0x0d, // BLENDPS XMM0, XMM1, 0xD
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_blendps_xmm0_xmm1_mask_0xe() {
    // BLENDPS XMM0, XMM1, 0xE (blend second, third, and fourth elements)
    let code = [
        0x66, 0x0f, 0x3a, 0x0c, 0xc1, 0x0e, // BLENDPS XMM0, XMM1, 0xE
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_blendps_xmm0_xmm1_mask_0xf() {
    // BLENDPS XMM0, XMM1, 0xF (all from source)
    let code = [
        0x66, 0x0f, 0x3a, 0x0c, 0xc1, 0x0f, // BLENDPS XMM0, XMM1, 0xF
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_blendps_xmm2_xmm3_mask_0x5() {
    // BLENDPS XMM2, XMM3, 0x5
    let code = [
        0x66, 0x0f, 0x3a, 0x0c, 0xd3, 0x05, // BLENDPS XMM2, XMM3, 0x5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_blendps_xmm4_xmm5_mask_0xa() {
    // BLENDPS XMM4, XMM5, 0xA
    let code = [
        0x66, 0x0f, 0x3a, 0x0c, 0xe5, 0x0a, // BLENDPS XMM4, XMM5, 0xA
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_blendps_xmm6_xmm7_mask_0xc() {
    // BLENDPS XMM6, XMM7, 0xC
    let code = [
        0x66, 0x0f, 0x3a, 0x0c, 0xf7, 0x0c, // BLENDPS XMM6, XMM7, 0xC
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_blendps_xmm8_xmm9_mask_0x3() {
    // BLENDPS XMM8, XMM9, 0x3 (requires REX prefix)
    let code = [
        0x66, 0x45, 0x0f, 0x3a, 0x0c, 0xc1, 0x03, // BLENDPS XMM8, XMM9, 0x3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_blendps_xmm10_xmm11_mask_0x6() {
    // BLENDPS XMM10, XMM11, 0x6
    let code = [
        0x66, 0x45, 0x0f, 0x3a, 0x0c, 0xd3, 0x06, // BLENDPS XMM10, XMM11, 0x6
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_blendps_xmm12_xmm13_mask_0x9() {
    // BLENDPS XMM12, XMM13, 0x9
    let code = [
        0x66, 0x45, 0x0f, 0x3a, 0x0c, 0xe5, 0x09, // BLENDPS XMM12, XMM13, 0x9
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_blendps_xmm14_xmm15_mask_0xf() {
    // BLENDPS XMM14, XMM15, 0xF
    let code = [
        0x66, 0x45, 0x0f, 0x3a, 0x0c, 0xf7, 0x0f, // BLENDPS XMM14, XMM15, 0xF
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_blendps_xmm0_mem_mask_0x5() {
    // BLENDPS XMM0, [ALIGNED_ADDR], 0x5
    let code = [
        0x66, 0x0f, 0x3a, 0x0c, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00,
        0x05, // BLENDPS XMM0, [0x3000], 0x5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_blendps_xmm1_mem_mask_0xa() {
    // BLENDPS XMM1, [ALIGNED_ADDR], 0xA
    let code = [
        0x66, 0x0f, 0x3a, 0x0c, 0x0c, 0x25, 0x00, 0x30, 0x00, 0x00,
        0x0a, // BLENDPS XMM1, [0x3000], 0xA
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_blendps_xmm7_mem_mask_0xf() {
    // BLENDPS XMM7, [ALIGNED_ADDR], 0xF
    let code = [
        0x66, 0x0f, 0x3a, 0x0c, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00,
        0x0f, // BLENDPS XMM7, [0x3000], 0xF
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_blendps_xmm15_mem_mask_0xc() {
    // BLENDPS XMM15, [ALIGNED_ADDR], 0xC
    let code = [
        0x66, 0x44, 0x0f, 0x3a, 0x0c, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00,
        0x0c, // BLENDPS XMM15, [0x3000], 0xC
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// BLENDPD Tests - Packed Double Precision (2x float64)
// ============================================================================

#[test]
fn test_blendpd_xmm0_xmm1_mask_0x0() {
    // BLENDPD XMM0, XMM1, 0x0 (all from dest)
    let code = [
        0x66, 0x0f, 0x3a, 0x0d, 0xc1, 0x00, // BLENDPD XMM0, XMM1, 0x0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_blendpd_xmm0_xmm1_mask_0x1() {
    // BLENDPD XMM0, XMM1, 0x1 (blend first element)
    let code = [
        0x66, 0x0f, 0x3a, 0x0d, 0xc1, 0x01, // BLENDPD XMM0, XMM1, 0x1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_blendpd_xmm0_xmm1_mask_0x2() {
    // BLENDPD XMM0, XMM1, 0x2 (blend second element)
    let code = [
        0x66, 0x0f, 0x3a, 0x0d, 0xc1, 0x02, // BLENDPD XMM0, XMM1, 0x2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_blendpd_xmm0_xmm1_mask_0x3() {
    // BLENDPD XMM0, XMM1, 0x3 (all from source)
    let code = [
        0x66, 0x0f, 0x3a, 0x0d, 0xc1, 0x03, // BLENDPD XMM0, XMM1, 0x3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_blendpd_xmm2_xmm3_mask_0x1() {
    // BLENDPD XMM2, XMM3, 0x1
    let code = [
        0x66, 0x0f, 0x3a, 0x0d, 0xd3, 0x01, // BLENDPD XMM2, XMM3, 0x1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_blendpd_xmm4_xmm5_mask_0x2() {
    // BLENDPD XMM4, XMM5, 0x2
    let code = [
        0x66, 0x0f, 0x3a, 0x0d, 0xe5, 0x02, // BLENDPD XMM4, XMM5, 0x2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_blendpd_xmm6_xmm7_mask_0x3() {
    // BLENDPD XMM6, XMM7, 0x3
    let code = [
        0x66, 0x0f, 0x3a, 0x0d, 0xf7, 0x03, // BLENDPD XMM6, XMM7, 0x3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_blendpd_xmm8_xmm9_mask_0x1() {
    // BLENDPD XMM8, XMM9, 0x1 (requires REX prefix)
    let code = [
        0x66, 0x45, 0x0f, 0x3a, 0x0d, 0xc1, 0x01, // BLENDPD XMM8, XMM9, 0x1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_blendpd_xmm10_xmm11_mask_0x2() {
    // BLENDPD XMM10, XMM11, 0x2
    let code = [
        0x66, 0x45, 0x0f, 0x3a, 0x0d, 0xd3, 0x02, // BLENDPD XMM10, XMM11, 0x2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_blendpd_xmm12_xmm13_mask_0x3() {
    // BLENDPD XMM12, XMM13, 0x3
    let code = [
        0x66, 0x45, 0x0f, 0x3a, 0x0d, 0xe5, 0x03, // BLENDPD XMM12, XMM13, 0x3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_blendpd_xmm14_xmm15_mask_0x0() {
    // BLENDPD XMM14, XMM15, 0x0
    let code = [
        0x66, 0x45, 0x0f, 0x3a, 0x0d, 0xf7, 0x00, // BLENDPD XMM14, XMM15, 0x0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_blendpd_xmm0_mem_mask_0x1() {
    // BLENDPD XMM0, [ALIGNED_ADDR], 0x1
    let code = [
        0x66, 0x0f, 0x3a, 0x0d, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00,
        0x01, // BLENDPD XMM0, [0x3000], 0x1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_blendpd_xmm1_mem_mask_0x2() {
    // BLENDPD XMM1, [ALIGNED_ADDR], 0x2
    let code = [
        0x66, 0x0f, 0x3a, 0x0d, 0x0c, 0x25, 0x00, 0x30, 0x00, 0x00,
        0x02, // BLENDPD XMM1, [0x3000], 0x2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_blendpd_xmm7_mem_mask_0x3() {
    // BLENDPD XMM7, [ALIGNED_ADDR], 0x3
    let code = [
        0x66, 0x0f, 0x3a, 0x0d, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00,
        0x03, // BLENDPD XMM7, [0x3000], 0x3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_blendpd_xmm15_mem_mask_0x0() {
    // BLENDPD XMM15, [ALIGNED_ADDR], 0x0
    let code = [
        0x66, 0x44, 0x0f, 0x3a, 0x0d, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00,
        0x00, // BLENDPD XMM15, [0x3000], 0x0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// Additional register combinations
#[test]
fn test_blendps_xmm1_xmm0_mask_0x6() {
    // BLENDPS XMM1, XMM0, 0x6
    let code = [
        0x66, 0x0f, 0x3a, 0x0c, 0xc8, 0x06, // BLENDPS XMM1, XMM0, 0x6
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_blendps_xmm3_xmm2_mask_0x9() {
    // BLENDPS XMM3, XMM2, 0x9
    let code = [
        0x66, 0x0f, 0x3a, 0x0c, 0xda, 0x09, // BLENDPS XMM3, XMM2, 0x9
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_blendps_xmm5_xmm4_mask_0xb() {
    // BLENDPS XMM5, XMM4, 0xB
    let code = [
        0x66, 0x0f, 0x3a, 0x0c, 0xec, 0x0b, // BLENDPS XMM5, XMM4, 0xB
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_blendpd_xmm1_xmm0_mask_0x2() {
    // BLENDPD XMM1, XMM0, 0x2
    let code = [
        0x66, 0x0f, 0x3a, 0x0d, 0xc8, 0x02, // BLENDPD XMM1, XMM0, 0x2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_blendpd_xmm3_xmm2_mask_0x1() {
    // BLENDPD XMM3, XMM2, 0x1
    let code = [
        0x66, 0x0f, 0x3a, 0x0d, 0xda, 0x01, // BLENDPD XMM3, XMM2, 0x1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_blendpd_xmm5_xmm4_mask_0x3() {
    // BLENDPD XMM5, XMM4, 0x3
    let code = [
        0x66, 0x0f, 0x3a, 0x0d, 0xec, 0x03, // BLENDPD XMM5, XMM4, 0x3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Known-answer value tests (register-to-register via set_xmm/get_xmm)
//
// BLENDPS uses imm8[3:0] to pick per-dword from SRC (bit=1) or DST (bit=0).
// BLENDPD uses imm8[1:0] to pick per-qword.
// ============================================================================

#[test]
fn kat_blendps_value() {
    // BLENDPS XMM0, XMM1, 0x0A (66 0F 3A 0C C1 0A): lanes 1 and 3 from src.
    let code = [0x66, 0x0f, 0x3a, 0x0c, 0xc1, 0x0a, 0xf4];
    let (mut vcpu, mem) = crate::common::setup_vm(&code, None);
    crate::common::set_xmm(&mem, &mut vcpu, 0, 0xAAAAAAA3_AAAAAAA2_AAAAAAA1_AAAAAAA0);
    crate::common::set_xmm(&mem, &mut vcpu, 1, 0xBBBBBBB3_BBBBBBB2_BBBBBBB1_BBBBBBB0);
    let regs = crate::common::run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        crate::common::get_xmm(&regs, 0),
        0xBBBBBBB3_AAAAAAA2_BBBBBBB1_AAAAAAA0,
        "BLENDPS got {:032x}",
        crate::common::get_xmm(&regs, 0)
    );
}

#[test]
fn kat_blendpd_value() {
    // BLENDPD XMM0, XMM1, 0x02 (66 0F 3A 0D C1 02): high qword from src.
    let code = [0x66, 0x0f, 0x3a, 0x0d, 0xc1, 0x02, 0xf4];
    let (mut vcpu, mem) = crate::common::setup_vm(&code, None);
    crate::common::set_xmm(&mem, &mut vcpu, 0, 0xDDDDDDDDDDDDDDD1_DDDDDDDDDDDDDDD0);
    crate::common::set_xmm(&mem, &mut vcpu, 1, 0x5555555555555551_5555555555555550);
    let regs = crate::common::run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        crate::common::get_xmm(&regs, 0),
        0x5555555555555551_DDDDDDDDDDDDDDD0,
        "BLENDPD got {:032x}",
        crate::common::get_xmm(&regs, 0)
    );
}
