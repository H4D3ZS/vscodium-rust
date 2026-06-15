use crate::common::{run_until_hlt, setup_vm};
use rax::cpu::Registers;
use vm_memory::{Bytes, GuestAddress};

// ADDSUBPS - Packed Single Precision Floating-Point Add/Subtract
// ADDSUBPD - Packed Double Precision Floating-Point Add/Subtract
//
// ADDSUBPS: Alternating add/subtract on 4x float32
// - Even indices (0,2): subtract
// - Odd indices (1,3): add
//
// ADDSUBPD: Alternating add/subtract on 2x float64
// - Even index (0): subtract
// - Odd index (1): add
//
// Opcodes:
// F2 0F D0 /r             ADDSUBPS xmm1, xmm2/m128    - Add/subtract packed SP FP values
// 66 0F D0 /r             ADDSUBPD xmm1, xmm2/m128    - Add/subtract packed DP FP values

const ALIGNED_ADDR: u64 = 0x3000; // 16-byte aligned address for testing

// ============================================================================
// ADDSUBPS Tests - Packed Single Precision (4x float32)
// ============================================================================

#[test]
fn test_addsubps_xmm0_xmm1() {
    // ADDSUBPS XMM0, XMM1
    let code = [
        0xf2, 0x0f, 0xd0, 0xc1, // ADDSUBPS XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_addsubps_xmm1_xmm2() {
    // ADDSUBPS XMM1, XMM2
    let code = [
        0xf2, 0x0f, 0xd0, 0xca, // ADDSUBPS XMM1, XMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_addsubps_xmm2_xmm3() {
    // ADDSUBPS XMM2, XMM3
    let code = [
        0xf2, 0x0f, 0xd0, 0xd3, // ADDSUBPS XMM2, XMM3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_addsubps_xmm3_xmm4() {
    // ADDSUBPS XMM3, XMM4
    let code = [
        0xf2, 0x0f, 0xd0, 0xdc, // ADDSUBPS XMM3, XMM4
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_addsubps_xmm4_xmm5() {
    // ADDSUBPS XMM4, XMM5
    let code = [
        0xf2, 0x0f, 0xd0, 0xe5, // ADDSUBPS XMM4, XMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_addsubps_xmm5_xmm6() {
    // ADDSUBPS XMM5, XMM6
    let code = [
        0xf2, 0x0f, 0xd0, 0xee, // ADDSUBPS XMM5, XMM6
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_addsubps_xmm6_xmm7() {
    // ADDSUBPS XMM6, XMM7
    let code = [
        0xf2, 0x0f, 0xd0, 0xf7, // ADDSUBPS XMM6, XMM7
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_addsubps_xmm7_xmm0() {
    // ADDSUBPS XMM7, XMM0
    let code = [
        0xf2, 0x0f, 0xd0, 0xf8, // ADDSUBPS XMM7, XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_addsubps_xmm8_xmm9() {
    // ADDSUBPS XMM8, XMM9 (requires REX prefix)
    let code = [
        0xf2, 0x45, 0x0f, 0xd0, 0xc1, // ADDSUBPS XMM8, XMM9
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_addsubps_xmm9_xmm10() {
    // ADDSUBPS XMM9, XMM10
    let code = [
        0xf2, 0x45, 0x0f, 0xd0, 0xca, // ADDSUBPS XMM9, XMM10
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_addsubps_xmm10_xmm11() {
    // ADDSUBPS XMM10, XMM11
    let code = [
        0xf2, 0x45, 0x0f, 0xd0, 0xd3, // ADDSUBPS XMM10, XMM11
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_addsubps_xmm11_xmm12() {
    // ADDSUBPS XMM11, XMM12
    let code = [
        0xf2, 0x45, 0x0f, 0xd0, 0xdc, // ADDSUBPS XMM11, XMM12
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_addsubps_xmm12_xmm13() {
    // ADDSUBPS XMM12, XMM13
    let code = [
        0xf2, 0x45, 0x0f, 0xd0, 0xe5, // ADDSUBPS XMM12, XMM13
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_addsubps_xmm13_xmm14() {
    // ADDSUBPS XMM13, XMM14
    let code = [
        0xf2, 0x45, 0x0f, 0xd0, 0xee, // ADDSUBPS XMM13, XMM14
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_addsubps_xmm14_xmm15() {
    // ADDSUBPS XMM14, XMM15
    let code = [
        0xf2, 0x45, 0x0f, 0xd0, 0xf7, // ADDSUBPS XMM14, XMM15
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_addsubps_xmm15_xmm0() {
    // ADDSUBPS XMM15, XMM0
    let code = [
        0xf2, 0x44, 0x0f, 0xd0, 0xf8, // ADDSUBPS XMM15, XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_addsubps_xmm0_mem() {
    // ADDSUBPS XMM0, [ALIGNED_ADDR]
    let code = [
        0xf2, 0x0f, 0xd0, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // ADDSUBPS XMM0, [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_addsubps_xmm7_mem() {
    // ADDSUBPS XMM7, [ALIGNED_ADDR]
    let code = [
        0xf2, 0x0f, 0xd0, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // ADDSUBPS XMM7, [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_addsubps_xmm15_mem() {
    // ADDSUBPS XMM15, [ALIGNED_ADDR]
    let code = [
        0xf2, 0x44, 0x0f, 0xd0, 0x3c, 0x25, 0x00, 0x30, 0x00,
        0x00, // ADDSUBPS XMM15, [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// ADDSUBPD Tests - Packed Double Precision (2x float64)
// ============================================================================

#[test]
fn test_addsubpd_xmm0_xmm1() {
    // ADDSUBPD XMM0, XMM1
    let code = [
        0x66, 0x0f, 0xd0, 0xc1, // ADDSUBPD XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_addsubpd_xmm1_xmm2() {
    // ADDSUBPD XMM1, XMM2
    let code = [
        0x66, 0x0f, 0xd0, 0xca, // ADDSUBPD XMM1, XMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_addsubpd_xmm2_xmm3() {
    // ADDSUBPD XMM2, XMM3
    let code = [
        0x66, 0x0f, 0xd0, 0xd3, // ADDSUBPD XMM2, XMM3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_addsubpd_xmm3_xmm4() {
    // ADDSUBPD XMM3, XMM4
    let code = [
        0x66, 0x0f, 0xd0, 0xdc, // ADDSUBPD XMM3, XMM4
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_addsubpd_xmm4_xmm5() {
    // ADDSUBPD XMM4, XMM5
    let code = [
        0x66, 0x0f, 0xd0, 0xe5, // ADDSUBPD XMM4, XMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_addsubpd_xmm5_xmm6() {
    // ADDSUBPD XMM5, XMM6
    let code = [
        0x66, 0x0f, 0xd0, 0xee, // ADDSUBPD XMM5, XMM6
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_addsubpd_xmm6_xmm7() {
    // ADDSUBPD XMM6, XMM7
    let code = [
        0x66, 0x0f, 0xd0, 0xf7, // ADDSUBPD XMM6, XMM7
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_addsubpd_xmm7_xmm0() {
    // ADDSUBPD XMM7, XMM0
    let code = [
        0x66, 0x0f, 0xd0, 0xf8, // ADDSUBPD XMM7, XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_addsubpd_xmm8_xmm9() {
    // ADDSUBPD XMM8, XMM9 (requires REX prefix)
    let code = [
        0x66, 0x45, 0x0f, 0xd0, 0xc1, // ADDSUBPD XMM8, XMM9
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_addsubpd_xmm9_xmm10() {
    // ADDSUBPD XMM9, XMM10
    let code = [
        0x66, 0x45, 0x0f, 0xd0, 0xca, // ADDSUBPD XMM9, XMM10
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_addsubpd_xmm10_xmm11() {
    // ADDSUBPD XMM10, XMM11
    let code = [
        0x66, 0x45, 0x0f, 0xd0, 0xd3, // ADDSUBPD XMM10, XMM11
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_addsubpd_xmm11_xmm12() {
    // ADDSUBPD XMM11, XMM12
    let code = [
        0x66, 0x45, 0x0f, 0xd0, 0xdc, // ADDSUBPD XMM11, XMM12
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_addsubpd_xmm12_xmm13() {
    // ADDSUBPD XMM12, XMM13
    let code = [
        0x66, 0x45, 0x0f, 0xd0, 0xe5, // ADDSUBPD XMM12, XMM13
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_addsubpd_xmm13_xmm14() {
    // ADDSUBPD XMM13, XMM14
    let code = [
        0x66, 0x45, 0x0f, 0xd0, 0xee, // ADDSUBPD XMM13, XMM14
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_addsubpd_xmm14_xmm15() {
    // ADDSUBPD XMM14, XMM15
    let code = [
        0x66, 0x45, 0x0f, 0xd0, 0xf7, // ADDSUBPD XMM14, XMM15
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_addsubpd_xmm15_xmm0() {
    // ADDSUBPD XMM15, XMM0
    let code = [
        0x66, 0x44, 0x0f, 0xd0, 0xf8, // ADDSUBPD XMM15, XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_addsubpd_xmm0_mem() {
    // ADDSUBPD XMM0, [ALIGNED_ADDR]
    let code = [
        0x66, 0x0f, 0xd0, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // ADDSUBPD XMM0, [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_addsubpd_xmm7_mem() {
    // ADDSUBPD XMM7, [ALIGNED_ADDR]
    let code = [
        0x66, 0x0f, 0xd0, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // ADDSUBPD XMM7, [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_addsubpd_xmm15_mem() {
    // ADDSUBPD XMM15, [ALIGNED_ADDR]
    let code = [
        0x66, 0x44, 0x0f, 0xd0, 0x3c, 0x25, 0x00, 0x30, 0x00,
        0x00, // ADDSUBPD XMM15, [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Pattern Tests
// ============================================================================

#[test]
fn test_addsubps_alternating_pattern() {
    // Test ADDSUBPS alternating pattern
    let code = [
        0xf2, 0x0f, 0xd0, 0xc1, // ADDSUBPS XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_addsubpd_alternating_pattern() {
    // Test ADDSUBPD alternating pattern
    let code = [
        0x66, 0x0f, 0xd0, 0xc1, // ADDSUBPD XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_addsubps_chain() {
    // Test chaining ADDSUBPS operations
    let code = [
        0xf2, 0x0f, 0xd0, 0xc1, // ADDSUBPS XMM0, XMM1
        0xf2, 0x0f, 0xd0, 0xc2, // ADDSUBPS XMM0, XMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_addsubpd_chain() {
    // Test chaining ADDSUBPD operations
    let code = [
        0x66, 0x0f, 0xd0, 0xc1, // ADDSUBPD XMM0, XMM1
        0x66, 0x0f, 0xd0, 0xc2, // ADDSUBPD XMM0, XMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
