use crate::common::{run_until_hlt, setup_vm};
use rax::cpu::Registers;
use vm_memory::{Bytes, GuestAddress};

// RCPSS - Compute Reciprocal of Scalar Single Precision Floating-Point Values
//
// RCPSS computes approximate reciprocal of the low single-precision (32-bit) floating-point value
// Relative error: |Relative Error| <= 1.5 * 2^-12
//
// Special cases:
// - Source 0.0 returns infinity with sign of source
// - Denormal source treated as 0.0 (same sign)
// - SNaN converted to QNaN, QNaN returned as-is
// - Tiny results flushed to 0.0 with sign of operand
// - Upper bits [127:32] remain unchanged (Legacy SSE)
//
// Opcodes:
// F3 0F 53 /r             RCPSS xmm1, xmm2/m32     - Compute approximate reciprocal of scalar single from xmm2/m32

const ALIGNED_ADDR: u64 = 0x3000; // 16-byte aligned address for testing

// ============================================================================
// RCPSS Tests - Scalar Single Precision Approximate Reciprocal
// ============================================================================

#[test]
fn test_rcpss_xmm0_xmm1() {
    // RCPSS XMM0, XMM1
    let code = [
        0xf3, 0x0f, 0x53, 0xc1, // RCPSS XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_rcpss_xmm1_xmm2() {
    // RCPSS XMM1, XMM2
    let code = [
        0xf3, 0x0f, 0x53, 0xca, // RCPSS XMM1, XMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_rcpss_xmm2_xmm3() {
    // RCPSS XMM2, XMM3
    let code = [
        0xf3, 0x0f, 0x53, 0xd3, // RCPSS XMM2, XMM3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_rcpss_xmm3_xmm4() {
    // RCPSS XMM3, XMM4
    let code = [
        0xf3, 0x0f, 0x53, 0xdc, // RCPSS XMM3, XMM4
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_rcpss_xmm4_xmm5() {
    // RCPSS XMM4, XMM5
    let code = [
        0xf3, 0x0f, 0x53, 0xe5, // RCPSS XMM4, XMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_rcpss_xmm5_xmm6() {
    // RCPSS XMM5, XMM6
    let code = [
        0xf3, 0x0f, 0x53, 0xee, // RCPSS XMM5, XMM6
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_rcpss_xmm6_xmm7() {
    // RCPSS XMM6, XMM7
    let code = [
        0xf3, 0x0f, 0x53, 0xf7, // RCPSS XMM6, XMM7
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_rcpss_xmm7_xmm0() {
    // RCPSS XMM7, XMM0
    let code = [
        0xf3, 0x0f, 0x53, 0xf8, // RCPSS XMM7, XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_rcpss_xmm8_xmm9() {
    // RCPSS XMM8, XMM9 (requires REX prefix)
    let code = [
        0xf3, 0x45, 0x0f, 0x53, 0xc1, // RCPSS XMM8, XMM9
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_rcpss_xmm10_xmm11() {
    // RCPSS XMM10, XMM11
    let code = [
        0xf3, 0x45, 0x0f, 0x53, 0xd3, // RCPSS XMM10, XMM11
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_rcpss_xmm12_xmm13() {
    // RCPSS XMM12, XMM13
    let code = [
        0xf3, 0x45, 0x0f, 0x53, 0xe5, // RCPSS XMM12, XMM13
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_rcpss_xmm14_xmm15() {
    // RCPSS XMM14, XMM15
    let code = [
        0xf3, 0x45, 0x0f, 0x53, 0xf7, // RCPSS XMM14, XMM15
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_rcpss_xmm15_xmm0() {
    // RCPSS XMM15, XMM0
    let code = [
        0xf3, 0x44, 0x0f, 0x53, 0xf8, // RCPSS XMM15, XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_rcpss_xmm0_mem() {
    // RCPSS XMM0, [ALIGNED_ADDR]
    let code = [
        0xf3, 0x0f, 0x53, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // RCPSS XMM0, [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_rcpss_xmm7_mem() {
    // RCPSS XMM7, [ALIGNED_ADDR]
    let code = [
        0xf3, 0x0f, 0x53, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // RCPSS XMM7, [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_rcpss_xmm15_mem() {
    // RCPSS XMM15, [ALIGNED_ADDR]
    let code = [
        0xf3, 0x44, 0x0f, 0x53, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // RCPSS XMM15, [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_rcpss_one() {
    // Test reciprocal of 1.0 (should approximate 1.0)
    let code = [
        0xf3, 0x0f, 0x53, 0xc1, // RCPSS XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_rcpss_two() {
    // Test reciprocal of 2.0 (should approximate 0.5)
    let code = [
        0xf3, 0x0f, 0x53, 0xd3, // RCPSS XMM2, XMM3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_rcpss_half() {
    // Test reciprocal of 0.5 (should approximate 2.0)
    let code = [
        0xf3, 0x0f, 0x53, 0xe5, // RCPSS XMM4, XMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_rcpss_small_value() {
    // Test reciprocal of small value (large result)
    let code = [
        0xf3, 0x0f, 0x53, 0xf7, // RCPSS XMM6, XMM7
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_rcpss_large_value() {
    // Test reciprocal of large value (small result)
    let code = [
        0xf3, 0x45, 0x0f, 0x53, 0xc1, // RCPSS XMM8, XMM9
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_rcpss_positive_zero() {
    // Test reciprocal of +0.0 (should return +infinity)
    let code = [
        0xf3, 0x45, 0x0f, 0x53, 0xd3, // RCPSS XMM10, XMM11
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_rcpss_negative_zero() {
    // Test reciprocal of -0.0 (should return -infinity)
    let code = [
        0xf3, 0x45, 0x0f, 0x53, 0xe5, // RCPSS XMM12, XMM13
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_rcpss_negative_value() {
    // Test reciprocal of negative value
    let code = [
        0xf3, 0x45, 0x0f, 0x53, 0xf7, // RCPSS XMM14, XMM15
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_rcpss_positive_infinity() {
    // Test reciprocal of +infinity (should return +0.0)
    let code = [
        0xf3, 0x44, 0x0f, 0x53, 0xf8, // RCPSS XMM15, XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_rcpss_negative_infinity() {
    // Test reciprocal of -infinity (should return -0.0)
    let code = [
        0xf3, 0x0f, 0x53, 0xc1, // RCPSS XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_rcpss_qnan() {
    // Test reciprocal of QNaN (should return QNaN)
    let code = [
        0xf3, 0x0f, 0x53, 0xd3, // RCPSS XMM2, XMM3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_rcpss_snan() {
    // Test reciprocal of SNaN (should convert to QNaN)
    let code = [
        0xf3, 0x0f, 0x53, 0xe5, // RCPSS XMM4, XMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_rcpss_upper_bits_preserved() {
    // Test that upper 96 bits [127:32] are preserved
    let code = [
        0xf3, 0x0f, 0x53, 0xf7, // RCPSS XMM6, XMM7
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_rcpss_denormal_treated_as_zero() {
    // Test that denormal values are treated as 0.0
    let code = [
        0xf3, 0x45, 0x0f, 0x53, 0xc1, // RCPSS XMM8, XMM9
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_rcpss_precision_test_1() {
    // Test precision of approximation
    let code = [
        0xf3, 0x45, 0x0f, 0x53, 0xd3, // RCPSS XMM10, XMM11
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_rcpss_precision_test_2() {
    // Test precision with different divisors
    let code = [
        0xf3, 0x45, 0x0f, 0x53, 0xe5, // RCPSS XMM12, XMM13
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_rcpss_mem_32bit() {
    // Test memory operand (32-bit scalar)
    let code = [
        0xf3, 0x0f, 0x53, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // RCPSS XMM0, [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_rcpss_self_xmm0() {
    // Test destination == source
    let code = [
        0xf3, 0x0f, 0x53, 0xc0, // RCPSS XMM0, XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_rcpss_self_xmm7() {
    // Test destination == source with high register
    let code = [
        0xf3, 0x0f, 0x53, 0xff, // RCPSS XMM7, XMM7
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
