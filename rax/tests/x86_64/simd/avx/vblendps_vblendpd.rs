use crate::common::*;
use rax::cpu::Registers;
use vm_memory::{Bytes, GuestAddress};

// VBLENDPS - Blend Packed Single-Precision Floating-Point Values
// VBLENDPD - Blend Packed Double-Precision Floating-Point Values
//
// VBLENDPS conditionally copies each element from the second source or first source
// to the destination based on an 8-bit immediate mask. Each bit controls one element.
// VBLENDPD performs the same operation for double-precision values.
//
// For 128-bit: VBLENDPS uses bits [3:0], VBLENDPD uses bits [1:0]
// For 256-bit: VBLENDPS uses bits [7:0], VBLENDPD uses bits [3:0]
//
// Opcodes:
// VEX.128.66.0F.3A.W0 0C /r ib    VBLENDPS xmm1, xmm2, xmm3/m128, imm8
// VEX.256.66.0F.3A.W0 0C /r ib    VBLENDPS ymm1, ymm2, ymm3/m256, imm8
// VEX.128.66.0F.3A.W0 0D /r ib    VBLENDPD xmm1, xmm2, xmm3/m128, imm8
// VEX.256.66.0F.3A.W0 0D /r ib    VBLENDPD ymm1, ymm2, ymm3/m256, imm8

const ALIGNED_ADDR: u64 = 0x3000; // 32-byte aligned address for testing

// ============================================================================
// VBLENDPS 128-bit Tests - All 16 mask combinations
// ============================================================================

#[test]
fn test_vblendps_xmm0_xmm1_xmm2_mask0x0() {
    // VBLENDPS XMM0, XMM1, XMM2, 0x0 - all from XMM1
    let code = [
        0xc4, 0xe3, 0x71, 0x0c, 0xc2, 0x00, // VBLENDPS XMM0, XMM1, XMM2, 0x0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vblendps_xmm0_xmm1_xmm2_mask0x1() {
    // VBLENDPS XMM0, XMM1, XMM2, 0x1 - blend element 0 from XMM2
    let code = [
        0xc4, 0xe3, 0x71, 0x0c, 0xc2, 0x01, // VBLENDPS XMM0, XMM1, XMM2, 0x1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vblendps_xmm0_xmm1_xmm2_mask0x2() {
    // VBLENDPS XMM0, XMM1, XMM2, 0x2 - blend element 1 from XMM2
    let code = [
        0xc4, 0xe3, 0x71, 0x0c, 0xc2, 0x02, // VBLENDPS XMM0, XMM1, XMM2, 0x2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vblendps_xmm0_xmm1_xmm2_mask0x3() {
    // VBLENDPS XMM0, XMM1, XMM2, 0x3 - blend elements 0-1 from XMM2
    let code = [
        0xc4, 0xe3, 0x71, 0x0c, 0xc2, 0x03, // VBLENDPS XMM0, XMM1, XMM2, 0x3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vblendps_xmm0_xmm1_xmm2_mask0x4() {
    // VBLENDPS XMM0, XMM1, XMM2, 0x4 - blend element 2 from XMM2
    let code = [
        0xc4, 0xe3, 0x71, 0x0c, 0xc2, 0x04, // VBLENDPS XMM0, XMM1, XMM2, 0x4
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vblendps_xmm0_xmm1_xmm2_mask0x5() {
    // VBLENDPS XMM0, XMM1, XMM2, 0x5 - blend elements 0,2 from XMM2
    let code = [
        0xc4, 0xe3, 0x71, 0x0c, 0xc2, 0x05, // VBLENDPS XMM0, XMM1, XMM2, 0x5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vblendps_xmm0_xmm1_xmm2_mask0x6() {
    // VBLENDPS XMM0, XMM1, XMM2, 0x6 - blend elements 1,2 from XMM2
    let code = [
        0xc4, 0xe3, 0x71, 0x0c, 0xc2, 0x06, // VBLENDPS XMM0, XMM1, XMM2, 0x6
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vblendps_xmm0_xmm1_xmm2_mask0x7() {
    // VBLENDPS XMM0, XMM1, XMM2, 0x7 - blend elements 0-2 from XMM2
    let code = [
        0xc4, 0xe3, 0x71, 0x0c, 0xc2, 0x07, // VBLENDPS XMM0, XMM1, XMM2, 0x7
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vblendps_xmm0_xmm1_xmm2_mask0x8() {
    // VBLENDPS XMM0, XMM1, XMM2, 0x8 - blend element 3 from XMM2
    let code = [
        0xc4, 0xe3, 0x71, 0x0c, 0xc2, 0x08, // VBLENDPS XMM0, XMM1, XMM2, 0x8
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vblendps_xmm0_xmm1_xmm2_mask0x9() {
    // VBLENDPS XMM0, XMM1, XMM2, 0x9 - blend elements 0,3 from XMM2
    let code = [
        0xc4, 0xe3, 0x71, 0x0c, 0xc2, 0x09, // VBLENDPS XMM0, XMM1, XMM2, 0x9
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vblendps_xmm0_xmm1_xmm2_mask0xa() {
    // VBLENDPS XMM0, XMM1, XMM2, 0xA - blend elements 1,3 from XMM2
    let code = [
        0xc4, 0xe3, 0x71, 0x0c, 0xc2, 0x0a, // VBLENDPS XMM0, XMM1, XMM2, 0xA
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vblendps_xmm0_xmm1_xmm2_mask0xb() {
    // VBLENDPS XMM0, XMM1, XMM2, 0xB - blend elements 0,1,3 from XMM2
    let code = [
        0xc4, 0xe3, 0x71, 0x0c, 0xc2, 0x0b, // VBLENDPS XMM0, XMM1, XMM2, 0xB
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vblendps_xmm0_xmm1_xmm2_mask0xc() {
    // VBLENDPS XMM0, XMM1, XMM2, 0xC - blend elements 2,3 from XMM2
    let code = [
        0xc4, 0xe3, 0x71, 0x0c, 0xc2, 0x0c, // VBLENDPS XMM0, XMM1, XMM2, 0xC
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vblendps_xmm0_xmm1_xmm2_mask0xd() {
    // VBLENDPS XMM0, XMM1, XMM2, 0xD - blend elements 0,2,3 from XMM2
    let code = [
        0xc4, 0xe3, 0x71, 0x0c, 0xc2, 0x0d, // VBLENDPS XMM0, XMM1, XMM2, 0xD
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vblendps_xmm0_xmm1_xmm2_mask0xe() {
    // VBLENDPS XMM0, XMM1, XMM2, 0xE - blend elements 1,2,3 from XMM2
    let code = [
        0xc4, 0xe3, 0x71, 0x0c, 0xc2, 0x0e, // VBLENDPS XMM0, XMM1, XMM2, 0xE
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vblendps_xmm0_xmm1_xmm2_mask0xf() {
    // VBLENDPS XMM0, XMM1, XMM2, 0xF - all from XMM2
    let code = [
        0xc4, 0xe3, 0x71, 0x0c, 0xc2, 0x0f, // VBLENDPS XMM0, XMM1, XMM2, 0xF
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// VBLENDPS 128-bit Tests - Different register combinations
// ============================================================================

#[test]
fn test_vblendps_xmm1_xmm2_xmm3_mask0x5() {
    // VBLENDPS XMM1, XMM2, XMM3, 0x5
    let code = [
        0xc4, 0xe3, 0x69, 0x0c, 0xcb, 0x05, // VBLENDPS XMM1, XMM2, XMM3, 0x5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vblendps_xmm7_xmm0_xmm1_mask0xa() {
    // VBLENDPS XMM7, XMM0, XMM1, 0xA
    let code = [
        0xc4, 0xe3, 0x79, 0x0c, 0xf9, 0x0a, // VBLENDPS XMM7, XMM0, XMM1, 0xA
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vblendps_xmm8_xmm9_xmm10_mask0xc() {
    // VBLENDPS XMM8, XMM9, XMM10, 0xC
    let code = [
        0xc4, 0xc3, 0x31, 0x0c, 0xc2, 0x0c, // VBLENDPS XMM8, XMM9, XMM10, 0xC
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vblendps_xmm15_xmm14_xmm13_mask0x3() {
    // VBLENDPS XMM15, XMM14, XMM13, 0x3
    let code = [
        0xc4, 0xc3, 0x09, 0x0c, 0xfd, 0x03, // VBLENDPS XMM15, XMM14, XMM13, 0x3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// VBLENDPS 256-bit Tests - All significant mask patterns
// ============================================================================

#[test]
fn test_vblendps_ymm0_ymm1_ymm2_mask0x00() {
    // VBLENDPS YMM0, YMM1, YMM2, 0x00 - all from YMM1
    let code = [
        0xc4, 0xe3, 0x75, 0x0c, 0xc2, 0x00, // VBLENDPS YMM0, YMM1, YMM2, 0x00
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vblendps_ymm0_ymm1_ymm2_mask0xff() {
    // VBLENDPS YMM0, YMM1, YMM2, 0xFF - all from YMM2
    let code = [
        0xc4, 0xe3, 0x75, 0x0c, 0xc2, 0xff, // VBLENDPS YMM0, YMM1, YMM2, 0xFF
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vblendps_ymm0_ymm1_ymm2_mask0x0f() {
    // VBLENDPS YMM0, YMM1, YMM2, 0x0F - lower half from YMM2
    let code = [
        0xc4, 0xe3, 0x75, 0x0c, 0xc2, 0x0f, // VBLENDPS YMM0, YMM1, YMM2, 0x0F
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vblendps_ymm0_ymm1_ymm2_mask0xf0() {
    // VBLENDPS YMM0, YMM1, YMM2, 0xF0 - upper half from YMM2
    let code = [
        0xc4, 0xe3, 0x75, 0x0c, 0xc2, 0xf0, // VBLENDPS YMM0, YMM1, YMM2, 0xF0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vblendps_ymm0_ymm1_ymm2_mask0xaa() {
    // VBLENDPS YMM0, YMM1, YMM2, 0xAA - alternating pattern
    let code = [
        0xc4, 0xe3, 0x75, 0x0c, 0xc2, 0xaa, // VBLENDPS YMM0, YMM1, YMM2, 0xAA
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vblendps_ymm0_ymm1_ymm2_mask0x55() {
    // VBLENDPS YMM0, YMM1, YMM2, 0x55 - alternating pattern (opposite)
    let code = [
        0xc4, 0xe3, 0x75, 0x0c, 0xc2, 0x55, // VBLENDPS YMM0, YMM1, YMM2, 0x55
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vblendps_ymm0_ymm1_ymm2_mask0xcc() {
    // VBLENDPS YMM0, YMM1, YMM2, 0xCC
    let code = [
        0xc4, 0xe3, 0x75, 0x0c, 0xc2, 0xcc, // VBLENDPS YMM0, YMM1, YMM2, 0xCC
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vblendps_ymm0_ymm1_ymm2_mask0x33() {
    // VBLENDPS YMM0, YMM1, YMM2, 0x33
    let code = [
        0xc4, 0xe3, 0x75, 0x0c, 0xc2, 0x33, // VBLENDPS YMM0, YMM1, YMM2, 0x33
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vblendps_ymm1_ymm2_ymm3_mask0xe4() {
    // VBLENDPS YMM1, YMM2, YMM3, 0xE4
    let code = [
        0xc4, 0xe3, 0x6d, 0x0c, 0xcb, 0xe4, // VBLENDPS YMM1, YMM2, YMM3, 0xE4
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vblendps_ymm8_ymm9_ymm10_mask0x3c() {
    // VBLENDPS YMM8, YMM9, YMM10, 0x3C
    let code = [
        0xc4, 0xc3, 0x35, 0x0c, 0xc2, 0x3c, // VBLENDPS YMM8, YMM9, YMM10, 0x3C
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// VBLENDPD 128-bit Tests - All 4 mask combinations
// ============================================================================

#[test]
fn test_vblendpd_xmm0_xmm1_xmm2_mask0x0() {
    // VBLENDPD XMM0, XMM1, XMM2, 0x0 - all from XMM1
    let code = [
        0xc4, 0xe3, 0x71, 0x0d, 0xc2, 0x00, // VBLENDPD XMM0, XMM1, XMM2, 0x0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vblendpd_xmm0_xmm1_xmm2_mask0x1() {
    // VBLENDPD XMM0, XMM1, XMM2, 0x1 - blend element 0 from XMM2
    let code = [
        0xc4, 0xe3, 0x71, 0x0d, 0xc2, 0x01, // VBLENDPD XMM0, XMM1, XMM2, 0x1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vblendpd_xmm0_xmm1_xmm2_mask0x2() {
    // VBLENDPD XMM0, XMM1, XMM2, 0x2 - blend element 1 from XMM2
    let code = [
        0xc4, 0xe3, 0x71, 0x0d, 0xc2, 0x02, // VBLENDPD XMM0, XMM1, XMM2, 0x2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vblendpd_xmm0_xmm1_xmm2_mask0x3() {
    // VBLENDPD XMM0, XMM1, XMM2, 0x3 - all from XMM2
    let code = [
        0xc4, 0xe3, 0x71, 0x0d, 0xc2, 0x03, // VBLENDPD XMM0, XMM1, XMM2, 0x3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vblendpd_xmm1_xmm2_xmm3_mask0x1() {
    // VBLENDPD XMM1, XMM2, XMM3, 0x1
    let code = [
        0xc4, 0xe3, 0x69, 0x0d, 0xcb, 0x01, // VBLENDPD XMM1, XMM2, XMM3, 0x1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vblendpd_xmm2_xmm3_xmm4_mask0x2() {
    // VBLENDPD XMM2, XMM3, XMM4, 0x2
    let code = [
        0xc4, 0xe3, 0x61, 0x0d, 0xd4, 0x02, // VBLENDPD XMM2, XMM3, XMM4, 0x2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vblendpd_xmm8_xmm9_xmm10_mask0x3() {
    // VBLENDPD XMM8, XMM9, XMM10, 0x3
    let code = [
        0xc4, 0xc3, 0x31, 0x0d, 0xc2, 0x03, // VBLENDPD XMM8, XMM9, XMM10, 0x3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vblendpd_xmm12_xmm13_xmm14_mask0x1() {
    // VBLENDPD XMM12, XMM13, XMM14, 0x1
    let code = [
        0xc4, 0xc3, 0x11, 0x0d, 0xe6, 0x01, // VBLENDPD XMM12, XMM13, XMM14, 0x1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// VBLENDPD 256-bit Tests - All 16 mask combinations
// ============================================================================

#[test]
fn test_vblendpd_ymm0_ymm1_ymm2_mask0x0() {
    // VBLENDPD YMM0, YMM1, YMM2, 0x0 - all from YMM1
    let code = [
        0xc4, 0xe3, 0x75, 0x0d, 0xc2, 0x00, // VBLENDPD YMM0, YMM1, YMM2, 0x0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vblendpd_ymm0_ymm1_ymm2_mask0x1() {
    // VBLENDPD YMM0, YMM1, YMM2, 0x1
    let code = [
        0xc4, 0xe3, 0x75, 0x0d, 0xc2, 0x01, // VBLENDPD YMM0, YMM1, YMM2, 0x1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vblendpd_ymm0_ymm1_ymm2_mask0x2() {
    // VBLENDPD YMM0, YMM1, YMM2, 0x2
    let code = [
        0xc4, 0xe3, 0x75, 0x0d, 0xc2, 0x02, // VBLENDPD YMM0, YMM1, YMM2, 0x2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vblendpd_ymm0_ymm1_ymm2_mask0x3() {
    // VBLENDPD YMM0, YMM1, YMM2, 0x3
    let code = [
        0xc4, 0xe3, 0x75, 0x0d, 0xc2, 0x03, // VBLENDPD YMM0, YMM1, YMM2, 0x3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vblendpd_ymm0_ymm1_ymm2_mask0x4() {
    // VBLENDPD YMM0, YMM1, YMM2, 0x4
    let code = [
        0xc4, 0xe3, 0x75, 0x0d, 0xc2, 0x04, // VBLENDPD YMM0, YMM1, YMM2, 0x4
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vblendpd_ymm0_ymm1_ymm2_mask0x5() {
    // VBLENDPD YMM0, YMM1, YMM2, 0x5
    let code = [
        0xc4, 0xe3, 0x75, 0x0d, 0xc2, 0x05, // VBLENDPD YMM0, YMM1, YMM2, 0x5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vblendpd_ymm0_ymm1_ymm2_mask0x6() {
    // VBLENDPD YMM0, YMM1, YMM2, 0x6
    let code = [
        0xc4, 0xe3, 0x75, 0x0d, 0xc2, 0x06, // VBLENDPD YMM0, YMM1, YMM2, 0x6
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vblendpd_ymm0_ymm1_ymm2_mask0x7() {
    // VBLENDPD YMM0, YMM1, YMM2, 0x7
    let code = [
        0xc4, 0xe3, 0x75, 0x0d, 0xc2, 0x07, // VBLENDPD YMM0, YMM1, YMM2, 0x7
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vblendpd_ymm0_ymm1_ymm2_mask0x8() {
    // VBLENDPD YMM0, YMM1, YMM2, 0x8
    let code = [
        0xc4, 0xe3, 0x75, 0x0d, 0xc2, 0x08, // VBLENDPD YMM0, YMM1, YMM2, 0x8
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vblendpd_ymm0_ymm1_ymm2_mask0x9() {
    // VBLENDPD YMM0, YMM1, YMM2, 0x9
    let code = [
        0xc4, 0xe3, 0x75, 0x0d, 0xc2, 0x09, // VBLENDPD YMM0, YMM1, YMM2, 0x9
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vblendpd_ymm0_ymm1_ymm2_mask0xa() {
    // VBLENDPD YMM0, YMM1, YMM2, 0xA
    let code = [
        0xc4, 0xe3, 0x75, 0x0d, 0xc2, 0x0a, // VBLENDPD YMM0, YMM1, YMM2, 0xA
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vblendpd_ymm0_ymm1_ymm2_mask0xb() {
    // VBLENDPD YMM0, YMM1, YMM2, 0xB
    let code = [
        0xc4, 0xe3, 0x75, 0x0d, 0xc2, 0x0b, // VBLENDPD YMM0, YMM1, YMM2, 0xB
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vblendpd_ymm0_ymm1_ymm2_mask0xc() {
    // VBLENDPD YMM0, YMM1, YMM2, 0xC
    let code = [
        0xc4, 0xe3, 0x75, 0x0d, 0xc2, 0x0c, // VBLENDPD YMM0, YMM1, YMM2, 0xC
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vblendpd_ymm0_ymm1_ymm2_mask0xd() {
    // VBLENDPD YMM0, YMM1, YMM2, 0xD
    let code = [
        0xc4, 0xe3, 0x75, 0x0d, 0xc2, 0x0d, // VBLENDPD YMM0, YMM1, YMM2, 0xD
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vblendpd_ymm0_ymm1_ymm2_mask0xe() {
    // VBLENDPD YMM0, YMM1, YMM2, 0xE
    let code = [
        0xc4, 0xe3, 0x75, 0x0d, 0xc2, 0x0e, // VBLENDPD YMM0, YMM1, YMM2, 0xE
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vblendpd_ymm0_ymm1_ymm2_mask0xf() {
    // VBLENDPD YMM0, YMM1, YMM2, 0xF - all from YMM2
    let code = [
        0xc4, 0xe3, 0x75, 0x0d, 0xc2, 0x0f, // VBLENDPD YMM0, YMM1, YMM2, 0xF
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vblendpd_ymm8_ymm9_ymm10_mask0x6() {
    // VBLENDPD YMM8, YMM9, YMM10, 0x6
    let code = [
        0xc4, 0xc3, 0x35, 0x0d, 0xc2, 0x06, // VBLENDPD YMM8, YMM9, YMM10, 0x6
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Memory Tests
// ============================================================================

#[test]
fn test_vblendps_xmm0_xmm1_mem128_mask0x5() {
    // VBLENDPS XMM0, XMM1, [mem128], 0x5
    let code = [
        0xc4, 0xe3, 0x71, 0x0c, 0x05, 0x00, 0x40, 0x00, 0x00,
        0x05, // VBLENDPS XMM0, XMM1, [rip + 0x4000], 0x5
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    let test_data: [u8; 16] = [
        0x00, 0x00, 0x80, 0x3f, 0x00, 0x00, 0x00, 0x40, 0x00, 0x00, 0x40, 0x40, 0x00, 0x00, 0x80,
        0x40,
    ];
    mem.write_slice(&test_data, GuestAddress(ALIGNED_ADDR))
        .unwrap();

    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vblendps_ymm0_ymm1_mem256_mask0xaa() {
    // VBLENDPS YMM0, YMM1, [mem256], 0xAA
    let code = [
        0xc4, 0xe3, 0x75, 0x0c, 0x05, 0x00, 0x40, 0x00, 0x00,
        0xaa, // VBLENDPS YMM0, YMM1, [rip + 0x4000], 0xAA
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    let test_data: [u8; 32] = [
        0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa,
        0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa,
        0xaa, 0xaa,
    ];
    mem.write_slice(&test_data, GuestAddress(ALIGNED_ADDR))
        .unwrap();

    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vblendpd_xmm0_xmm1_mem128_mask0x2() {
    // VBLENDPD XMM0, XMM1, [mem128], 0x2
    let code = [
        0xc4, 0xe3, 0x71, 0x0d, 0x05, 0x00, 0x40, 0x00, 0x00,
        0x02, // VBLENDPD XMM0, XMM1, [rip + 0x4000], 0x2
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    let test_data: [u8; 16] = [
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xf0, 0x3f, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x40,
    ];
    mem.write_slice(&test_data, GuestAddress(ALIGNED_ADDR))
        .unwrap();

    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vblendpd_ymm0_ymm1_mem256_mask0xc() {
    // VBLENDPD YMM0, YMM1, [mem256], 0xC
    let code = [
        0xc4, 0xe3, 0x75, 0x0d, 0x05, 0x00, 0x40, 0x00, 0x00,
        0x0c, // VBLENDPD YMM0, YMM1, [rip + 0x4000], 0xC
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    let test_data: [u8; 32] = [
        0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc,
        0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc,
        0xcc, 0xcc,
    ];
    mem.write_slice(&test_data, GuestAddress(ALIGNED_ADDR))
        .unwrap();

    run_until_hlt(&mut vcpu).unwrap();
}
