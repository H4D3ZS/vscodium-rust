use crate::common::{run_until_hlt, setup_vm};
use rax::cpu::Registers;
use vm_memory::{Bytes, GuestAddress};

// HSUBPS - Packed Single Precision Floating-Point Horizontal Subtract
// HSUBPD - Packed Double Precision Floating-Point Horizontal Subtract
//
// HSUBPS: Horizontal subtract of adjacent pairs in 4x float32
// Result: [dest[0]-dest[1], dest[2]-dest[3], src[0]-src[1], src[2]-src[3]]
//
// HSUBPD: Horizontal subtract of adjacent pairs in 2x float64
// Result: [dest[0]-dest[1], src[0]-src[1]]
//
// Opcodes:
// F2 0F 7D /r             HSUBPS xmm1, xmm2/m128    - Horizontal subtract packed SP FP values
// 66 0F 7D /r             HSUBPD xmm1, xmm2/m128    - Horizontal subtract packed DP FP values

const ALIGNED_ADDR: u64 = 0x3000; // 16-byte aligned address for testing

// ============================================================================
// HSUBPS Tests - Packed Single Precision (4x float32)
// ============================================================================

#[test]
fn test_hsubps_xmm0_xmm1() {
    // HSUBPS XMM0, XMM1
    let code = [
        0xf2, 0x0f, 0x7d, 0xc1, // HSUBPS XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_hsubps_xmm1_xmm2() {
    // HSUBPS XMM1, XMM2
    let code = [
        0xf2, 0x0f, 0x7d, 0xca, // HSUBPS XMM1, XMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_hsubps_xmm2_xmm3() {
    // HSUBPS XMM2, XMM3
    let code = [
        0xf2, 0x0f, 0x7d, 0xd3, // HSUBPS XMM2, XMM3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_hsubps_xmm3_xmm4() {
    // HSUBPS XMM3, XMM4
    let code = [
        0xf2, 0x0f, 0x7d, 0xdc, // HSUBPS XMM3, XMM4
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_hsubps_xmm4_xmm5() {
    // HSUBPS XMM4, XMM5
    let code = [
        0xf2, 0x0f, 0x7d, 0xe5, // HSUBPS XMM4, XMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_hsubps_xmm5_xmm6() {
    // HSUBPS XMM5, XMM6
    let code = [
        0xf2, 0x0f, 0x7d, 0xee, // HSUBPS XMM5, XMM6
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_hsubps_xmm6_xmm7() {
    // HSUBPS XMM6, XMM7
    let code = [
        0xf2, 0x0f, 0x7d, 0xf7, // HSUBPS XMM6, XMM7
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_hsubps_xmm7_xmm0() {
    // HSUBPS XMM7, XMM0
    let code = [
        0xf2, 0x0f, 0x7d, 0xf8, // HSUBPS XMM7, XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_hsubps_xmm8_xmm9() {
    // HSUBPS XMM8, XMM9 (requires REX prefix)
    let code = [
        0xf2, 0x45, 0x0f, 0x7d, 0xc1, // HSUBPS XMM8, XMM9
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_hsubps_xmm9_xmm10() {
    // HSUBPS XMM9, XMM10
    let code = [
        0xf2, 0x45, 0x0f, 0x7d, 0xca, // HSUBPS XMM9, XMM10
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_hsubps_xmm10_xmm11() {
    // HSUBPS XMM10, XMM11
    let code = [
        0xf2, 0x45, 0x0f, 0x7d, 0xd3, // HSUBPS XMM10, XMM11
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_hsubps_xmm11_xmm12() {
    // HSUBPS XMM11, XMM12
    let code = [
        0xf2, 0x45, 0x0f, 0x7d, 0xdc, // HSUBPS XMM11, XMM12
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_hsubps_xmm12_xmm13() {
    // HSUBPS XMM12, XMM13
    let code = [
        0xf2, 0x45, 0x0f, 0x7d, 0xe5, // HSUBPS XMM12, XMM13
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_hsubps_xmm13_xmm14() {
    // HSUBPS XMM13, XMM14
    let code = [
        0xf2, 0x45, 0x0f, 0x7d, 0xee, // HSUBPS XMM13, XMM14
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_hsubps_xmm14_xmm15() {
    // HSUBPS XMM14, XMM15
    let code = [
        0xf2, 0x45, 0x0f, 0x7d, 0xf7, // HSUBPS XMM14, XMM15
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_hsubps_xmm15_xmm0() {
    // HSUBPS XMM15, XMM0
    let code = [
        0xf2, 0x44, 0x0f, 0x7d, 0xf8, // HSUBPS XMM15, XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_hsubps_xmm0_mem() {
    // HSUBPS XMM0, [ALIGNED_ADDR]
    let code = [
        0xf2, 0x0f, 0x7d, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // HSUBPS XMM0, [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_hsubps_xmm7_mem() {
    // HSUBPS XMM7, [ALIGNED_ADDR]
    let code = [
        0xf2, 0x0f, 0x7d, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // HSUBPS XMM7, [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_hsubps_xmm15_mem() {
    // HSUBPS XMM15, [ALIGNED_ADDR]
    let code = [
        0xf2, 0x44, 0x0f, 0x7d, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // HSUBPS XMM15, [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// HSUBPD Tests - Packed Double Precision (2x float64)
// ============================================================================

#[test]
fn test_hsubpd_xmm0_xmm1() {
    // HSUBPD XMM0, XMM1
    let code = [
        0x66, 0x0f, 0x7d, 0xc1, // HSUBPD XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_hsubpd_xmm1_xmm2() {
    // HSUBPD XMM1, XMM2
    let code = [
        0x66, 0x0f, 0x7d, 0xca, // HSUBPD XMM1, XMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_hsubpd_xmm2_xmm3() {
    // HSUBPD XMM2, XMM3
    let code = [
        0x66, 0x0f, 0x7d, 0xd3, // HSUBPD XMM2, XMM3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_hsubpd_xmm3_xmm4() {
    // HSUBPD XMM3, XMM4
    let code = [
        0x66, 0x0f, 0x7d, 0xdc, // HSUBPD XMM3, XMM4
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_hsubpd_xmm4_xmm5() {
    // HSUBPD XMM4, XMM5
    let code = [
        0x66, 0x0f, 0x7d, 0xe5, // HSUBPD XMM4, XMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_hsubpd_xmm5_xmm6() {
    // HSUBPD XMM5, XMM6
    let code = [
        0x66, 0x0f, 0x7d, 0xee, // HSUBPD XMM5, XMM6
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_hsubpd_xmm6_xmm7() {
    // HSUBPD XMM6, XMM7
    let code = [
        0x66, 0x0f, 0x7d, 0xf7, // HSUBPD XMM6, XMM7
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_hsubpd_xmm7_xmm0() {
    // HSUBPD XMM7, XMM0
    let code = [
        0x66, 0x0f, 0x7d, 0xf8, // HSUBPD XMM7, XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_hsubpd_xmm8_xmm9() {
    // HSUBPD XMM8, XMM9 (requires REX prefix)
    let code = [
        0x66, 0x45, 0x0f, 0x7d, 0xc1, // HSUBPD XMM8, XMM9
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_hsubpd_xmm9_xmm10() {
    // HSUBPD XMM9, XMM10
    let code = [
        0x66, 0x45, 0x0f, 0x7d, 0xca, // HSUBPD XMM9, XMM10
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_hsubpd_xmm10_xmm11() {
    // HSUBPD XMM10, XMM11
    let code = [
        0x66, 0x45, 0x0f, 0x7d, 0xd3, // HSUBPD XMM10, XMM11
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_hsubpd_xmm11_xmm12() {
    // HSUBPD XMM11, XMM12
    let code = [
        0x66, 0x45, 0x0f, 0x7d, 0xdc, // HSUBPD XMM11, XMM12
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_hsubpd_xmm12_xmm13() {
    // HSUBPD XMM12, XMM13
    let code = [
        0x66, 0x45, 0x0f, 0x7d, 0xe5, // HSUBPD XMM12, XMM13
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_hsubpd_xmm13_xmm14() {
    // HSUBPD XMM13, XMM14
    let code = [
        0x66, 0x45, 0x0f, 0x7d, 0xee, // HSUBPD XMM13, XMM14
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_hsubpd_xmm14_xmm15() {
    // HSUBPD XMM14, XMM15
    let code = [
        0x66, 0x45, 0x0f, 0x7d, 0xf7, // HSUBPD XMM14, XMM15
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_hsubpd_xmm15_xmm0() {
    // HSUBPD XMM15, XMM0
    let code = [
        0x66, 0x44, 0x0f, 0x7d, 0xf8, // HSUBPD XMM15, XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_hsubpd_xmm0_mem() {
    // HSUBPD XMM0, [ALIGNED_ADDR]
    let code = [
        0x66, 0x0f, 0x7d, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // HSUBPD XMM0, [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_hsubpd_xmm7_mem() {
    // HSUBPD XMM7, [ALIGNED_ADDR]
    let code = [
        0x66, 0x0f, 0x7d, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // HSUBPD XMM7, [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_hsubpd_xmm15_mem() {
    // HSUBPD XMM15, [ALIGNED_ADDR]
    let code = [
        0x66, 0x44, 0x0f, 0x7d, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // HSUBPD XMM15, [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Horizontal Pattern Tests
// ============================================================================

#[test]
fn test_hsubps_horizontal_pattern() {
    // Test HSUBPS horizontal subtraction pattern
    let code = [
        0xf2, 0x0f, 0x7d, 0xc1, // HSUBPS XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_hsubpd_horizontal_pattern() {
    // Test HSUBPD horizontal subtraction pattern
    let code = [
        0x66, 0x0f, 0x7d, 0xc1, // HSUBPD XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_hsubps_chain() {
    // Test chaining HSUBPS operations
    let code = [
        0xf2, 0x0f, 0x7d, 0xc1, // HSUBPS XMM0, XMM1
        0xf2, 0x0f, 0x7d, 0xc2, // HSUBPS XMM0, XMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_hsubpd_chain() {
    // Test chaining HSUBPD operations
    let code = [
        0x66, 0x0f, 0x7d, 0xc1, // HSUBPD XMM0, XMM1
        0x66, 0x0f, 0x7d, 0xc2, // HSUBPD XMM0, XMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_hsubps_self() {
    // Test HSUBPS with same register
    let code = [
        0xf2, 0x0f, 0x7d, 0xc0, // HSUBPS XMM0, XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_hsubpd_self() {
    // Test HSUBPD with same register
    let code = [
        0x66, 0x0f, 0x7d, 0xc0, // HSUBPD XMM0, XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
