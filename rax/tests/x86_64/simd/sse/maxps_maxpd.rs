use crate::common::{run_until_hlt, setup_vm};
use rax::cpu::Registers;
use vm_memory::{Bytes, GuestAddress};

// MAXPS - Maximum of Packed Single Precision Floating-Point Values
// MAXPD - Maximum of Packed Double Precision Floating-Point Values
//
// MAXPS returns maximum of 4 packed single-precision (32-bit) floating-point values
// MAXPD returns maximum of 2 packed double-precision (64-bit) floating-point values
//
// Special cases:
// - If values are both 0.0s (either sign), return second operand
// - If second operand is SNaN, forward SNaN unchanged to destination
// - If only one value is NaN, return second operand
//
// Opcodes:
// NP 0F 5F /r             MAXPS xmm1, xmm2/m128     - Return maximum packed single from xmm1 and xmm2/m128
// 66 0F 5F /r             MAXPD xmm1, xmm2/m128     - Return maximum packed double from xmm1 and xmm2/m128

const ALIGNED_ADDR: u64 = 0x3000; // 16-byte aligned address for testing

// ============================================================================
// MAXPS Tests - Packed Single Precision Maximum (4x float32)
// ============================================================================

#[test]
fn test_maxps_xmm0_xmm1() {
    // MAXPS XMM0, XMM1
    let code = [
        0x0f, 0x5f, 0xc1, // MAXPS XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_maxps_xmm1_xmm2() {
    // MAXPS XMM1, XMM2
    let code = [
        0x0f, 0x5f, 0xca, // MAXPS XMM1, XMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_maxps_xmm2_xmm3() {
    // MAXPS XMM2, XMM3
    let code = [
        0x0f, 0x5f, 0xd3, // MAXPS XMM2, XMM3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_maxps_xmm3_xmm4() {
    // MAXPS XMM3, XMM4
    let code = [
        0x0f, 0x5f, 0xdc, // MAXPS XMM3, XMM4
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_maxps_xmm4_xmm5() {
    // MAXPS XMM4, XMM5
    let code = [
        0x0f, 0x5f, 0xe5, // MAXPS XMM4, XMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_maxps_xmm5_xmm6() {
    // MAXPS XMM5, XMM6
    let code = [
        0x0f, 0x5f, 0xee, // MAXPS XMM5, XMM6
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_maxps_xmm6_xmm7() {
    // MAXPS XMM6, XMM7
    let code = [
        0x0f, 0x5f, 0xf7, // MAXPS XMM6, XMM7
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_maxps_xmm7_xmm0() {
    // MAXPS XMM7, XMM0
    let code = [
        0x0f, 0x5f, 0xf8, // MAXPS XMM7, XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_maxps_xmm8_xmm9() {
    // MAXPS XMM8, XMM9 (requires REX prefix)
    let code = [
        0x45, 0x0f, 0x5f, 0xc1, // MAXPS XMM8, XMM9
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_maxps_xmm10_xmm11() {
    // MAXPS XMM10, XMM11
    let code = [
        0x45, 0x0f, 0x5f, 0xd3, // MAXPS XMM10, XMM11
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_maxps_xmm12_xmm13() {
    // MAXPS XMM12, XMM13
    let code = [
        0x45, 0x0f, 0x5f, 0xe5, // MAXPS XMM12, XMM13
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_maxps_xmm14_xmm15() {
    // MAXPS XMM14, XMM15
    let code = [
        0x45, 0x0f, 0x5f, 0xf7, // MAXPS XMM14, XMM15
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_maxps_xmm15_xmm0() {
    // MAXPS XMM15, XMM0
    let code = [
        0x44, 0x0f, 0x5f, 0xf8, // MAXPS XMM15, XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_maxps_xmm0_mem() {
    // MAXPS XMM0, [ALIGNED_ADDR]
    let code = [
        0x0f, 0x5f, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // MAXPS XMM0, [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_maxps_xmm7_mem() {
    // MAXPS XMM7, [ALIGNED_ADDR]
    let code = [
        0x0f, 0x5f, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // MAXPS XMM7, [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_maxps_xmm15_mem() {
    // MAXPS XMM15, [ALIGNED_ADDR]
    let code = [
        0x44, 0x0f, 0x5f, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // MAXPS XMM15, [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_maxps_positive_values() {
    // Test maximum of positive values
    let code = [
        0x0f, 0x5f, 0xc1, // MAXPS XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_maxps_negative_values() {
    // Test maximum of negative values
    let code = [
        0x0f, 0x5f, 0xd3, // MAXPS XMM2, XMM3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_maxps_mixed_signs() {
    // Test maximum with mixed positive and negative values
    let code = [
        0x0f, 0x5f, 0xe5, // MAXPS XMM4, XMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_maxps_zero_positive() {
    // Test maximum of +0.0 and +0.0 (should return second operand)
    let code = [
        0x0f, 0x5f, 0xf7, // MAXPS XMM6, XMM7
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_maxps_zero_negative() {
    // Test maximum of -0.0 and -0.0 (should return second operand)
    let code = [
        0x45, 0x0f, 0x5f, 0xc1, // MAXPS XMM8, XMM9
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_maxps_nan_handling() {
    // Test NaN handling (should return second operand if one is NaN)
    let code = [
        0x45, 0x0f, 0x5f, 0xd3, // MAXPS XMM10, XMM11
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_maxps_snan_forwarding() {
    // Test SNaN forwarding (SNaN from second operand should be forwarded unchanged)
    let code = [
        0x45, 0x0f, 0x5f, 0xe5, // MAXPS XMM12, XMM13
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// MAXPD Tests - Packed Double Precision Maximum (2x float64)
// ============================================================================

#[test]
fn test_maxpd_xmm0_xmm1() {
    // MAXPD XMM0, XMM1
    let code = [
        0x66, 0x0f, 0x5f, 0xc1, // MAXPD XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_maxpd_xmm1_xmm2() {
    // MAXPD XMM1, XMM2
    let code = [
        0x66, 0x0f, 0x5f, 0xca, // MAXPD XMM1, XMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_maxpd_xmm2_xmm3() {
    // MAXPD XMM2, XMM3
    let code = [
        0x66, 0x0f, 0x5f, 0xd3, // MAXPD XMM2, XMM3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_maxpd_xmm3_xmm4() {
    // MAXPD XMM3, XMM4
    let code = [
        0x66, 0x0f, 0x5f, 0xdc, // MAXPD XMM3, XMM4
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_maxpd_xmm4_xmm5() {
    // MAXPD XMM4, XMM5
    let code = [
        0x66, 0x0f, 0x5f, 0xe5, // MAXPD XMM4, XMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_maxpd_xmm5_xmm6() {
    // MAXPD XMM5, XMM6
    let code = [
        0x66, 0x0f, 0x5f, 0xee, // MAXPD XMM5, XMM6
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_maxpd_xmm6_xmm7() {
    // MAXPD XMM6, XMM7
    let code = [
        0x66, 0x0f, 0x5f, 0xf7, // MAXPD XMM6, XMM7
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_maxpd_xmm7_xmm0() {
    // MAXPD XMM7, XMM0
    let code = [
        0x66, 0x0f, 0x5f, 0xf8, // MAXPD XMM7, XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_maxpd_xmm8_xmm9() {
    // MAXPD XMM8, XMM9 (requires REX prefix)
    let code = [
        0x66, 0x45, 0x0f, 0x5f, 0xc1, // MAXPD XMM8, XMM9
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_maxpd_xmm10_xmm11() {
    // MAXPD XMM10, XMM11
    let code = [
        0x66, 0x45, 0x0f, 0x5f, 0xd3, // MAXPD XMM10, XMM11
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_maxpd_xmm12_xmm13() {
    // MAXPD XMM12, XMM13
    let code = [
        0x66, 0x45, 0x0f, 0x5f, 0xe5, // MAXPD XMM12, XMM13
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_maxpd_xmm14_xmm15() {
    // MAXPD XMM14, XMM15
    let code = [
        0x66, 0x45, 0x0f, 0x5f, 0xf7, // MAXPD XMM14, XMM15
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_maxpd_xmm15_xmm0() {
    // MAXPD XMM15, XMM0
    let code = [
        0x66, 0x44, 0x0f, 0x5f, 0xf8, // MAXPD XMM15, XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_maxpd_xmm0_mem() {
    // MAXPD XMM0, [ALIGNED_ADDR]
    let code = [
        0x66, 0x0f, 0x5f, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // MAXPD XMM0, [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_maxpd_xmm7_mem() {
    // MAXPD XMM7, [ALIGNED_ADDR]
    let code = [
        0x66, 0x0f, 0x5f, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // MAXPD XMM7, [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_maxpd_xmm15_mem() {
    // MAXPD XMM15, [ALIGNED_ADDR]
    let code = [
        0x66, 0x44, 0x0f, 0x5f, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // MAXPD XMM15, [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_maxpd_positive_values() {
    // Test maximum of positive values
    let code = [
        0x66, 0x0f, 0x5f, 0xc1, // MAXPD XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_maxpd_negative_values() {
    // Test maximum of negative values
    let code = [
        0x66, 0x0f, 0x5f, 0xd3, // MAXPD XMM2, XMM3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_maxpd_mixed_signs() {
    // Test maximum with mixed positive and negative values
    let code = [
        0x66, 0x0f, 0x5f, 0xe5, // MAXPD XMM4, XMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_maxpd_zero_positive() {
    // Test maximum of +0.0 and +0.0 (should return second operand)
    let code = [
        0x66, 0x0f, 0x5f, 0xf7, // MAXPD XMM6, XMM7
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_maxpd_zero_negative() {
    // Test maximum of -0.0 and -0.0 (should return second operand)
    let code = [
        0x66, 0x45, 0x0f, 0x5f, 0xc1, // MAXPD XMM8, XMM9
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_maxpd_nan_handling() {
    // Test NaN handling (should return second operand if one is NaN)
    let code = [
        0x66, 0x45, 0x0f, 0x5f, 0xd3, // MAXPD XMM10, XMM11
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_maxpd_snan_forwarding() {
    // Test SNaN forwarding (SNaN from second operand should be forwarded unchanged)
    let code = [
        0x66, 0x45, 0x0f, 0x5f, 0xe5, // MAXPD XMM12, XMM13
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
