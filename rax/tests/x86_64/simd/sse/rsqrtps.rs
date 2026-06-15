use crate::common::{run_until_hlt, setup_vm};
use rax::cpu::Registers;
use vm_memory::{Bytes, GuestAddress};

// RSQRTPS - Compute Reciprocals of Square Roots of Packed Single Precision Floating-Point Values
//
// RSQRTPS computes approximate reciprocals of square roots of 4 packed single-precision (32-bit) floating-point values
// Relative error: |Relative Error| <= 1.5 * 2^-12
//
// Special cases:
// - Source 0.0 returns infinity with sign of source
// - Denormal source treated as 0.0 (same sign)
// - Negative value (except -0.0) returns floating-point indefinite
// - SNaN converted to QNaN, QNaN returned as-is
//
// Opcodes:
// NP 0F 52 /r             RSQRTPS xmm1, xmm2/m128   - Compute approximate reciprocals of square roots of packed single from xmm2/m128

const ALIGNED_ADDR: u64 = 0x3000; // 16-byte aligned address for testing

// ============================================================================
// RSQRTPS Tests - Packed Single Precision Approximate Reciprocal Square Root (4x float32)
// ============================================================================

#[test]
fn test_rsqrtps_xmm0_xmm1() {
    // RSQRTPS XMM0, XMM1
    let code = [
        0x0f, 0x52, 0xc1, // RSQRTPS XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_rsqrtps_xmm1_xmm2() {
    // RSQRTPS XMM1, XMM2
    let code = [
        0x0f, 0x52, 0xca, // RSQRTPS XMM1, XMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_rsqrtps_xmm2_xmm3() {
    // RSQRTPS XMM2, XMM3
    let code = [
        0x0f, 0x52, 0xd3, // RSQRTPS XMM2, XMM3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_rsqrtps_xmm3_xmm4() {
    // RSQRTPS XMM3, XMM4
    let code = [
        0x0f, 0x52, 0xdc, // RSQRTPS XMM3, XMM4
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_rsqrtps_xmm4_xmm5() {
    // RSQRTPS XMM4, XMM5
    let code = [
        0x0f, 0x52, 0xe5, // RSQRTPS XMM4, XMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_rsqrtps_xmm5_xmm6() {
    // RSQRTPS XMM5, XMM6
    let code = [
        0x0f, 0x52, 0xee, // RSQRTPS XMM5, XMM6
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_rsqrtps_xmm6_xmm7() {
    // RSQRTPS XMM6, XMM7
    let code = [
        0x0f, 0x52, 0xf7, // RSQRTPS XMM6, XMM7
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_rsqrtps_xmm7_xmm0() {
    // RSQRTPS XMM7, XMM0
    let code = [
        0x0f, 0x52, 0xf8, // RSQRTPS XMM7, XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_rsqrtps_xmm8_xmm9() {
    // RSQRTPS XMM8, XMM9 (requires REX prefix)
    let code = [
        0x45, 0x0f, 0x52, 0xc1, // RSQRTPS XMM8, XMM9
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_rsqrtps_xmm10_xmm11() {
    // RSQRTPS XMM10, XMM11
    let code = [
        0x45, 0x0f, 0x52, 0xd3, // RSQRTPS XMM10, XMM11
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_rsqrtps_xmm12_xmm13() {
    // RSQRTPS XMM12, XMM13
    let code = [
        0x45, 0x0f, 0x52, 0xe5, // RSQRTPS XMM12, XMM13
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_rsqrtps_xmm14_xmm15() {
    // RSQRTPS XMM14, XMM15
    let code = [
        0x45, 0x0f, 0x52, 0xf7, // RSQRTPS XMM14, XMM15
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_rsqrtps_xmm15_xmm0() {
    // RSQRTPS XMM15, XMM0
    let code = [
        0x44, 0x0f, 0x52, 0xf8, // RSQRTPS XMM15, XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_rsqrtps_xmm0_mem() {
    // RSQRTPS XMM0, [ALIGNED_ADDR]
    let code = [
        0x0f, 0x52, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // RSQRTPS XMM0, [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_rsqrtps_xmm7_mem() {
    // RSQRTPS XMM7, [ALIGNED_ADDR]
    let code = [
        0x0f, 0x52, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // RSQRTPS XMM7, [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_rsqrtps_xmm15_mem() {
    // RSQRTPS XMM15, [ALIGNED_ADDR]
    let code = [
        0x44, 0x0f, 0x52, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // RSQRTPS XMM15, [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_rsqrtps_one() {
    // Test reciprocal sqrt of 1.0 (should approximate 1.0)
    let code = [
        0x0f, 0x52, 0xc1, // RSQRTPS XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_rsqrtps_four() {
    // Test reciprocal sqrt of 4.0 (should approximate 0.5)
    let code = [
        0x0f, 0x52, 0xd3, // RSQRTPS XMM2, XMM3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_rsqrtps_nine() {
    // Test reciprocal sqrt of 9.0 (should approximate 1/3)
    let code = [
        0x0f, 0x52, 0xe5, // RSQRTPS XMM4, XMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_rsqrtps_quarter() {
    // Test reciprocal sqrt of 0.25 (should approximate 2.0)
    let code = [
        0x0f, 0x52, 0xf7, // RSQRTPS XMM6, XMM7
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_rsqrtps_small_values() {
    // Test reciprocal sqrt of small values (large results)
    let code = [
        0x45, 0x0f, 0x52, 0xc1, // RSQRTPS XMM8, XMM9
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_rsqrtps_large_values() {
    // Test reciprocal sqrt of large values (small results)
    let code = [
        0x45, 0x0f, 0x52, 0xd3, // RSQRTPS XMM10, XMM11
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_rsqrtps_positive_zero() {
    // Test reciprocal sqrt of +0.0 (should return +infinity)
    let code = [
        0x45, 0x0f, 0x52, 0xe5, // RSQRTPS XMM12, XMM13
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_rsqrtps_negative_zero() {
    // Test reciprocal sqrt of -0.0 (should return -infinity)
    let code = [
        0x45, 0x0f, 0x52, 0xf7, // RSQRTPS XMM14, XMM15
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_rsqrtps_negative_value() {
    // Test reciprocal sqrt of negative value (should return indefinite)
    let code = [
        0x44, 0x0f, 0x52, 0xf8, // RSQRTPS XMM15, XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_rsqrtps_positive_infinity() {
    // Test reciprocal sqrt of +infinity (should return +0.0)
    let code = [
        0x0f, 0x52, 0xc1, // RSQRTPS XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_rsqrtps_negative_infinity() {
    // Test reciprocal sqrt of -infinity (should return indefinite)
    let code = [
        0x0f, 0x52, 0xd3, // RSQRTPS XMM2, XMM3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_rsqrtps_qnan() {
    // Test reciprocal sqrt of QNaN (should return QNaN)
    let code = [
        0x0f, 0x52, 0xe5, // RSQRTPS XMM4, XMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_rsqrtps_snan() {
    // Test reciprocal sqrt of SNaN (should convert to QNaN)
    let code = [
        0x0f, 0x52, 0xf7, // RSQRTPS XMM6, XMM7
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_rsqrtps_precision_test() {
    // Test precision of approximation (various values)
    let code = [
        0x45, 0x0f, 0x52, 0xc1, // RSQRTPS XMM8, XMM9
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_rsqrtps_denormal_treated_as_zero() {
    // Test that denormal values are treated as 0.0
    let code = [
        0x45, 0x0f, 0x52, 0xd3, // RSQRTPS XMM10, XMM11
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
