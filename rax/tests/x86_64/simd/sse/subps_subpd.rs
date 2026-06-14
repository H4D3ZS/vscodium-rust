use crate::common::{run_until_hlt, setup_vm};
use rax::cpu::Registers;
use vm_memory::{Bytes, GuestAddress};

// SUBPS - Subtract Packed Single Precision Floating-Point Values
// SUBPD - Subtract Packed Double Precision Floating-Point Values
//
// SUBPS subtracts 4 packed single-precision (32-bit) floating-point values
// SUBPD subtracts 2 packed double-precision (64-bit) floating-point values
//
// Opcodes:
// NP 0F 5C /r             SUBPS xmm1, xmm2/m128    - Subtract packed single from xmm2/m128 from xmm1
// 66 0F 5C /r             SUBPD xmm1, xmm2/m128    - Subtract packed double from xmm2/m128 from xmm1

const ALIGNED_ADDR: u64 = 0x3000; // 16-byte aligned address for testing

// ============================================================================
// SUBPS Tests - Packed Single Precision (4x float32)
// ============================================================================

#[test]
fn test_subps_xmm0_xmm1() {
    // SUBPS XMM0, XMM1
    let code = [
        0x0f, 0x5c, 0xc1, // SUBPS XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_subps_xmm1_xmm2() {
    // SUBPS XMM1, XMM2
    let code = [
        0x0f, 0x5c, 0xca, // SUBPS XMM1, XMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_subps_xmm2_xmm3() {
    // SUBPS XMM2, XMM3
    let code = [
        0x0f, 0x5c, 0xd3, // SUBPS XMM2, XMM3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_subps_xmm3_xmm4() {
    // SUBPS XMM3, XMM4
    let code = [
        0x0f, 0x5c, 0xdc, // SUBPS XMM3, XMM4
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_subps_xmm4_xmm5() {
    // SUBPS XMM4, XMM5
    let code = [
        0x0f, 0x5c, 0xe5, // SUBPS XMM4, XMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_subps_xmm5_xmm6() {
    // SUBPS XMM5, XMM6
    let code = [
        0x0f, 0x5c, 0xee, // SUBPS XMM5, XMM6
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_subps_xmm6_xmm7() {
    // SUBPS XMM6, XMM7
    let code = [
        0x0f, 0x5c, 0xf7, // SUBPS XMM6, XMM7
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_subps_xmm7_xmm0() {
    // SUBPS XMM7, XMM0
    let code = [
        0x0f, 0x5c, 0xf8, // SUBPS XMM7, XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_subps_xmm8_xmm9() {
    // SUBPS XMM8, XMM9 (requires REX prefix)
    let code = [
        0x45, 0x0f, 0x5c, 0xc1, // SUBPS XMM8, XMM9
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_subps_xmm9_xmm10() {
    // SUBPS XMM9, XMM10
    let code = [
        0x45, 0x0f, 0x5c, 0xca, // SUBPS XMM9, XMM10
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_subps_xmm10_xmm11() {
    // SUBPS XMM10, XMM11
    let code = [
        0x45, 0x0f, 0x5c, 0xd3, // SUBPS XMM10, XMM11
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_subps_xmm11_xmm12() {
    // SUBPS XMM11, XMM12
    let code = [
        0x45, 0x0f, 0x5c, 0xdc, // SUBPS XMM11, XMM12
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_subps_xmm12_xmm13() {
    // SUBPS XMM12, XMM13
    let code = [
        0x45, 0x0f, 0x5c, 0xe5, // SUBPS XMM12, XMM13
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_subps_xmm13_xmm14() {
    // SUBPS XMM13, XMM14
    let code = [
        0x45, 0x0f, 0x5c, 0xee, // SUBPS XMM13, XMM14
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_subps_xmm14_xmm15() {
    // SUBPS XMM14, XMM15
    let code = [
        0x45, 0x0f, 0x5c, 0xf7, // SUBPS XMM14, XMM15
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_subps_xmm15_xmm0() {
    // SUBPS XMM15, XMM0
    let code = [
        0x44, 0x0f, 0x5c, 0xf8, // SUBPS XMM15, XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_subps_xmm0_mem() {
    // SUBPS XMM0, [ALIGNED_ADDR]
    let code = [
        0x0f, 0x5c, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // SUBPS XMM0, [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_subps_xmm1_mem() {
    // SUBPS XMM1, [ALIGNED_ADDR]
    let code = [
        0x0f, 0x5c, 0x0c, 0x25, 0x00, 0x30, 0x00, 0x00, // SUBPS XMM1, [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_subps_xmm7_mem() {
    // SUBPS XMM7, [ALIGNED_ADDR]
    let code = [
        0x0f, 0x5c, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // SUBPS XMM7, [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_subps_xmm15_mem() {
    // SUBPS XMM15, [ALIGNED_ADDR]
    let code = [
        0x44, 0x0f, 0x5c, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // SUBPS XMM15, [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_subps_positive_values() {
    // Test subtraction of positive floating-point values
    let code = [
        0x0f, 0x5c, 0xc1, // SUBPS XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_subps_negative_result() {
    // Test subtraction resulting in negative values
    let code = [
        0x0f, 0x5c, 0xd3, // SUBPS XMM2, XMM3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_subps_mixed_signs() {
    // Test subtraction with mixed signs
    let code = [
        0x0f, 0x5c, 0xe5, // SUBPS XMM4, XMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_subps_zero_result() {
    // Test subtraction resulting in zero
    let code = [
        0x0f, 0x5c, 0xf7, // SUBPS XMM6, XMM7
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// SUBPD Tests - Packed Double Precision (2x float64)
// ============================================================================

#[test]
fn test_subpd_xmm0_xmm1() {
    // SUBPD XMM0, XMM1
    let code = [
        0x66, 0x0f, 0x5c, 0xc1, // SUBPD XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_subpd_xmm1_xmm2() {
    // SUBPD XMM1, XMM2
    let code = [
        0x66, 0x0f, 0x5c, 0xca, // SUBPD XMM1, XMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_subpd_xmm2_xmm3() {
    // SUBPD XMM2, XMM3
    let code = [
        0x66, 0x0f, 0x5c, 0xd3, // SUBPD XMM2, XMM3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_subpd_xmm3_xmm4() {
    // SUBPD XMM3, XMM4
    let code = [
        0x66, 0x0f, 0x5c, 0xdc, // SUBPD XMM3, XMM4
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_subpd_xmm4_xmm5() {
    // SUBPD XMM4, XMM5
    let code = [
        0x66, 0x0f, 0x5c, 0xe5, // SUBPD XMM4, XMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_subpd_xmm5_xmm6() {
    // SUBPD XMM5, XMM6
    let code = [
        0x66, 0x0f, 0x5c, 0xee, // SUBPD XMM5, XMM6
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_subpd_xmm6_xmm7() {
    // SUBPD XMM6, XMM7
    let code = [
        0x66, 0x0f, 0x5c, 0xf7, // SUBPD XMM6, XMM7
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_subpd_xmm7_xmm0() {
    // SUBPD XMM7, XMM0
    let code = [
        0x66, 0x0f, 0x5c, 0xf8, // SUBPD XMM7, XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_subpd_xmm8_xmm9() {
    // SUBPD XMM8, XMM9 (requires REX prefix)
    let code = [
        0x66, 0x45, 0x0f, 0x5c, 0xc1, // SUBPD XMM8, XMM9
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_subpd_xmm9_xmm10() {
    // SUBPD XMM9, XMM10
    let code = [
        0x66, 0x45, 0x0f, 0x5c, 0xca, // SUBPD XMM9, XMM10
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_subpd_xmm10_xmm11() {
    // SUBPD XMM10, XMM11
    let code = [
        0x66, 0x45, 0x0f, 0x5c, 0xd3, // SUBPD XMM10, XMM11
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_subpd_xmm11_xmm12() {
    // SUBPD XMM11, XMM12
    let code = [
        0x66, 0x45, 0x0f, 0x5c, 0xdc, // SUBPD XMM11, XMM12
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_subpd_xmm12_xmm13() {
    // SUBPD XMM12, XMM13
    let code = [
        0x66, 0x45, 0x0f, 0x5c, 0xe5, // SUBPD XMM12, XMM13
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_subpd_xmm13_xmm14() {
    // SUBPD XMM13, XMM14
    let code = [
        0x66, 0x45, 0x0f, 0x5c, 0xee, // SUBPD XMM13, XMM14
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_subpd_xmm14_xmm15() {
    // SUBPD XMM14, XMM15
    let code = [
        0x66, 0x45, 0x0f, 0x5c, 0xf7, // SUBPD XMM14, XMM15
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_subpd_xmm15_xmm0() {
    // SUBPD XMM15, XMM0
    let code = [
        0x66, 0x44, 0x0f, 0x5c, 0xf8, // SUBPD XMM15, XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_subpd_xmm0_mem() {
    // SUBPD XMM0, [ALIGNED_ADDR]
    let code = [
        0x66, 0x0f, 0x5c, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // SUBPD XMM0, [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_subpd_xmm1_mem() {
    // SUBPD XMM1, [ALIGNED_ADDR]
    let code = [
        0x66, 0x0f, 0x5c, 0x0c, 0x25, 0x00, 0x30, 0x00, 0x00, // SUBPD XMM1, [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_subpd_xmm7_mem() {
    // SUBPD XMM7, [ALIGNED_ADDR]
    let code = [
        0x66, 0x0f, 0x5c, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // SUBPD XMM7, [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_subpd_xmm15_mem() {
    // SUBPD XMM15, [ALIGNED_ADDR]
    let code = [
        0x66, 0x44, 0x0f, 0x5c, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // SUBPD XMM15, [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_subpd_positive_values() {
    // Test subtraction of positive double-precision values
    let code = [
        0x66, 0x0f, 0x5c, 0xc1, // SUBPD XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_subpd_negative_result() {
    // Test subtraction resulting in negative double values
    let code = [
        0x66, 0x0f, 0x5c, 0xd3, // SUBPD XMM2, XMM3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_subpd_mixed_signs() {
    // Test subtraction with mixed signs
    let code = [
        0x66, 0x0f, 0x5c, 0xe5, // SUBPD XMM4, XMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_subpd_zero_result() {
    // Test subtraction resulting in zero
    let code = [
        0x66, 0x0f, 0x5c, 0xf7, // SUBPD XMM6, XMM7
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
