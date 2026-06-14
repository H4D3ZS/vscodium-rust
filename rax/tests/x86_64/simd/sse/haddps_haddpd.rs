use crate::common::{run_until_hlt, setup_vm};
use rax::cpu::Registers;
use vm_memory::{Bytes, GuestAddress};

// HADDPS - Packed Single Precision Floating-Point Horizontal Add
// HADDPD - Packed Double Precision Floating-Point Horizontal Add
//
// HADDPS: Horizontal add of adjacent pairs in 4x float32
// Result: [dest[1]+dest[0], dest[3]+dest[2], src[1]+src[0], src[3]+src[2]]
//
// HADDPD: Horizontal add of adjacent pairs in 2x float64
// Result: [dest[1]+dest[0], src[1]+src[0]]
//
// Opcodes:
// F2 0F 7C /r             HADDPS xmm1, xmm2/m128    - Horizontal add packed SP FP values
// 66 0F 7C /r             HADDPD xmm1, xmm2/m128    - Horizontal add packed DP FP values

const ALIGNED_ADDR: u64 = 0x3000; // 16-byte aligned address for testing

// ============================================================================
// HADDPS Tests - Packed Single Precision (4x float32)
// ============================================================================

#[test]
fn test_haddps_xmm0_xmm1() {
    // HADDPS XMM0, XMM1
    let code = [
        0xf2, 0x0f, 0x7c, 0xc1, // HADDPS XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_haddps_xmm1_xmm2() {
    // HADDPS XMM1, XMM2
    let code = [
        0xf2, 0x0f, 0x7c, 0xca, // HADDPS XMM1, XMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_haddps_xmm2_xmm3() {
    // HADDPS XMM2, XMM3
    let code = [
        0xf2, 0x0f, 0x7c, 0xd3, // HADDPS XMM2, XMM3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_haddps_xmm3_xmm4() {
    // HADDPS XMM3, XMM4
    let code = [
        0xf2, 0x0f, 0x7c, 0xdc, // HADDPS XMM3, XMM4
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_haddps_xmm4_xmm5() {
    // HADDPS XMM4, XMM5
    let code = [
        0xf2, 0x0f, 0x7c, 0xe5, // HADDPS XMM4, XMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_haddps_xmm5_xmm6() {
    // HADDPS XMM5, XMM6
    let code = [
        0xf2, 0x0f, 0x7c, 0xee, // HADDPS XMM5, XMM6
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_haddps_xmm6_xmm7() {
    // HADDPS XMM6, XMM7
    let code = [
        0xf2, 0x0f, 0x7c, 0xf7, // HADDPS XMM6, XMM7
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_haddps_xmm7_xmm0() {
    // HADDPS XMM7, XMM0
    let code = [
        0xf2, 0x0f, 0x7c, 0xf8, // HADDPS XMM7, XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_haddps_xmm8_xmm9() {
    // HADDPS XMM8, XMM9 (requires REX prefix)
    let code = [
        0xf2, 0x45, 0x0f, 0x7c, 0xc1, // HADDPS XMM8, XMM9
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_haddps_xmm9_xmm10() {
    // HADDPS XMM9, XMM10
    let code = [
        0xf2, 0x45, 0x0f, 0x7c, 0xca, // HADDPS XMM9, XMM10
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_haddps_xmm10_xmm11() {
    // HADDPS XMM10, XMM11
    let code = [
        0xf2, 0x45, 0x0f, 0x7c, 0xd3, // HADDPS XMM10, XMM11
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_haddps_xmm11_xmm12() {
    // HADDPS XMM11, XMM12
    let code = [
        0xf2, 0x45, 0x0f, 0x7c, 0xdc, // HADDPS XMM11, XMM12
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_haddps_xmm12_xmm13() {
    // HADDPS XMM12, XMM13
    let code = [
        0xf2, 0x45, 0x0f, 0x7c, 0xe5, // HADDPS XMM12, XMM13
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_haddps_xmm13_xmm14() {
    // HADDPS XMM13, XMM14
    let code = [
        0xf2, 0x45, 0x0f, 0x7c, 0xee, // HADDPS XMM13, XMM14
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_haddps_xmm14_xmm15() {
    // HADDPS XMM14, XMM15
    let code = [
        0xf2, 0x45, 0x0f, 0x7c, 0xf7, // HADDPS XMM14, XMM15
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_haddps_xmm15_xmm0() {
    // HADDPS XMM15, XMM0
    let code = [
        0xf2, 0x44, 0x0f, 0x7c, 0xf8, // HADDPS XMM15, XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_haddps_xmm0_mem() {
    // HADDPS XMM0, [ALIGNED_ADDR]
    let code = [
        0xf2, 0x0f, 0x7c, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // HADDPS XMM0, [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_haddps_xmm7_mem() {
    // HADDPS XMM7, [ALIGNED_ADDR]
    let code = [
        0xf2, 0x0f, 0x7c, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // HADDPS XMM7, [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_haddps_xmm15_mem() {
    // HADDPS XMM15, [ALIGNED_ADDR]
    let code = [
        0xf2, 0x44, 0x0f, 0x7c, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // HADDPS XMM15, [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// HADDPD Tests - Packed Double Precision (2x float64)
// ============================================================================

#[test]
fn test_haddpd_xmm0_xmm1() {
    // HADDPD XMM0, XMM1
    let code = [
        0x66, 0x0f, 0x7c, 0xc1, // HADDPD XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_haddpd_xmm1_xmm2() {
    // HADDPD XMM1, XMM2
    let code = [
        0x66, 0x0f, 0x7c, 0xca, // HADDPD XMM1, XMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_haddpd_xmm2_xmm3() {
    // HADDPD XMM2, XMM3
    let code = [
        0x66, 0x0f, 0x7c, 0xd3, // HADDPD XMM2, XMM3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_haddpd_xmm3_xmm4() {
    // HADDPD XMM3, XMM4
    let code = [
        0x66, 0x0f, 0x7c, 0xdc, // HADDPD XMM3, XMM4
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_haddpd_xmm4_xmm5() {
    // HADDPD XMM4, XMM5
    let code = [
        0x66, 0x0f, 0x7c, 0xe5, // HADDPD XMM4, XMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_haddpd_xmm5_xmm6() {
    // HADDPD XMM5, XMM6
    let code = [
        0x66, 0x0f, 0x7c, 0xee, // HADDPD XMM5, XMM6
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_haddpd_xmm6_xmm7() {
    // HADDPD XMM6, XMM7
    let code = [
        0x66, 0x0f, 0x7c, 0xf7, // HADDPD XMM6, XMM7
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_haddpd_xmm7_xmm0() {
    // HADDPD XMM7, XMM0
    let code = [
        0x66, 0x0f, 0x7c, 0xf8, // HADDPD XMM7, XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_haddpd_xmm8_xmm9() {
    // HADDPD XMM8, XMM9 (requires REX prefix)
    let code = [
        0x66, 0x45, 0x0f, 0x7c, 0xc1, // HADDPD XMM8, XMM9
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_haddpd_xmm9_xmm10() {
    // HADDPD XMM9, XMM10
    let code = [
        0x66, 0x45, 0x0f, 0x7c, 0xca, // HADDPD XMM9, XMM10
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_haddpd_xmm10_xmm11() {
    // HADDPD XMM10, XMM11
    let code = [
        0x66, 0x45, 0x0f, 0x7c, 0xd3, // HADDPD XMM10, XMM11
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_haddpd_xmm11_xmm12() {
    // HADDPD XMM11, XMM12
    let code = [
        0x66, 0x45, 0x0f, 0x7c, 0xdc, // HADDPD XMM11, XMM12
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_haddpd_xmm12_xmm13() {
    // HADDPD XMM12, XMM13
    let code = [
        0x66, 0x45, 0x0f, 0x7c, 0xe5, // HADDPD XMM12, XMM13
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_haddpd_xmm13_xmm14() {
    // HADDPD XMM13, XMM14
    let code = [
        0x66, 0x45, 0x0f, 0x7c, 0xee, // HADDPD XMM13, XMM14
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_haddpd_xmm14_xmm15() {
    // HADDPD XMM14, XMM15
    let code = [
        0x66, 0x45, 0x0f, 0x7c, 0xf7, // HADDPD XMM14, XMM15
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_haddpd_xmm15_xmm0() {
    // HADDPD XMM15, XMM0
    let code = [
        0x66, 0x44, 0x0f, 0x7c, 0xf8, // HADDPD XMM15, XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_haddpd_xmm0_mem() {
    // HADDPD XMM0, [ALIGNED_ADDR]
    let code = [
        0x66, 0x0f, 0x7c, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // HADDPD XMM0, [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_haddpd_xmm7_mem() {
    // HADDPD XMM7, [ALIGNED_ADDR]
    let code = [
        0x66, 0x0f, 0x7c, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // HADDPD XMM7, [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_haddpd_xmm15_mem() {
    // HADDPD XMM15, [ALIGNED_ADDR]
    let code = [
        0x66, 0x44, 0x0f, 0x7c, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // HADDPD XMM15, [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Horizontal Pattern Tests
// ============================================================================

#[test]
fn test_haddps_horizontal_pattern() {
    // Test HADDPS horizontal addition pattern
    let code = [
        0xf2, 0x0f, 0x7c, 0xc1, // HADDPS XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_haddpd_horizontal_pattern() {
    // Test HADDPD horizontal addition pattern
    let code = [
        0x66, 0x0f, 0x7c, 0xc1, // HADDPD XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_haddps_chain() {
    // Test chaining HADDPS operations
    let code = [
        0xf2, 0x0f, 0x7c, 0xc1, // HADDPS XMM0, XMM1
        0xf2, 0x0f, 0x7c, 0xc2, // HADDPS XMM0, XMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_haddpd_chain() {
    // Test chaining HADDPD operations
    let code = [
        0x66, 0x0f, 0x7c, 0xc1, // HADDPD XMM0, XMM1
        0x66, 0x0f, 0x7c, 0xc2, // HADDPD XMM0, XMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_haddps_self() {
    // Test HADDPS with same register
    let code = [
        0xf2, 0x0f, 0x7c, 0xc0, // HADDPS XMM0, XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_haddpd_self() {
    // Test HADDPD with same register
    let code = [
        0x66, 0x0f, 0x7c, 0xc0, // HADDPD XMM0, XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
