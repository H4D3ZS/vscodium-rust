use crate::common::{run_until_hlt, setup_vm};
use rax::cpu::Registers;
use vm_memory::{Bytes, GuestAddress};

// SQRTSS - Compute Square Root of Scalar Single Precision Value
// SQRTSD - Compute Square Root of Scalar Double Precision Value
//
// SQRTSS computes square root of the low single-precision (32-bit) floating-point value
// SQRTSD computes square root of the low double-precision (64-bit) floating-point value
//
// Upper bits are preserved from the first source operand
//
// Opcodes:
// F3 0F 51 /r             SQRTSS xmm1, xmm2/m32     - Compute square root of low single from xmm2/m32 to xmm1
// F2 0F 51 /r             SQRTSD xmm1, xmm2/m64     - Compute square root of low double from xmm2/m64 to xmm1

const ALIGNED_ADDR: u64 = 0x3000; // 16-byte aligned address for testing

// ============================================================================
// SQRTSS Tests - Scalar Single Precision Square Root
// ============================================================================

#[test]
fn test_sqrtss_xmm0_xmm1() {
    // SQRTSS XMM0, XMM1
    let code = [
        0xf3, 0x0f, 0x51, 0xc1, // SQRTSS XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sqrtss_xmm1_xmm2() {
    // SQRTSS XMM1, XMM2
    let code = [
        0xf3, 0x0f, 0x51, 0xca, // SQRTSS XMM1, XMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sqrtss_xmm2_xmm3() {
    // SQRTSS XMM2, XMM3
    let code = [
        0xf3, 0x0f, 0x51, 0xd3, // SQRTSS XMM2, XMM3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sqrtss_xmm3_xmm4() {
    // SQRTSS XMM3, XMM4
    let code = [
        0xf3, 0x0f, 0x51, 0xdc, // SQRTSS XMM3, XMM4
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sqrtss_xmm4_xmm5() {
    // SQRTSS XMM4, XMM5
    let code = [
        0xf3, 0x0f, 0x51, 0xe5, // SQRTSS XMM4, XMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sqrtss_xmm5_xmm6() {
    // SQRTSS XMM5, XMM6
    let code = [
        0xf3, 0x0f, 0x51, 0xee, // SQRTSS XMM5, XMM6
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sqrtss_xmm6_xmm7() {
    // SQRTSS XMM6, XMM7
    let code = [
        0xf3, 0x0f, 0x51, 0xf7, // SQRTSS XMM6, XMM7
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sqrtss_xmm7_xmm0() {
    // SQRTSS XMM7, XMM0
    let code = [
        0xf3, 0x0f, 0x51, 0xf8, // SQRTSS XMM7, XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sqrtss_xmm8_xmm9() {
    // SQRTSS XMM8, XMM9 (requires REX prefix)
    let code = [
        0xf3, 0x45, 0x0f, 0x51, 0xc1, // SQRTSS XMM8, XMM9
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sqrtss_xmm10_xmm11() {
    // SQRTSS XMM10, XMM11
    let code = [
        0xf3, 0x45, 0x0f, 0x51, 0xd3, // SQRTSS XMM10, XMM11
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sqrtss_xmm12_xmm13() {
    // SQRTSS XMM12, XMM13
    let code = [
        0xf3, 0x45, 0x0f, 0x51, 0xe5, // SQRTSS XMM12, XMM13
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sqrtss_xmm14_xmm15() {
    // SQRTSS XMM14, XMM15
    let code = [
        0xf3, 0x45, 0x0f, 0x51, 0xf7, // SQRTSS XMM14, XMM15
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sqrtss_xmm15_xmm0() {
    // SQRTSS XMM15, XMM0
    let code = [
        0xf3, 0x44, 0x0f, 0x51, 0xf8, // SQRTSS XMM15, XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sqrtss_xmm0_mem() {
    // SQRTSS XMM0, [ALIGNED_ADDR]
    let code = [
        0xf3, 0x0f, 0x51, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // SQRTSS XMM0, [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sqrtss_xmm7_mem() {
    // SQRTSS XMM7, [ALIGNED_ADDR]
    let code = [
        0xf3, 0x0f, 0x51, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // SQRTSS XMM7, [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sqrtss_xmm15_mem() {
    // SQRTSS XMM15, [ALIGNED_ADDR]
    let code = [
        0xf3, 0x44, 0x0f, 0x51, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // SQRTSS XMM15, [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sqrtss_perfect_square() {
    // Test square root of perfect square (4.0 -> 2.0)
    let code = [
        0xf3, 0x0f, 0x51, 0xc1, // SQRTSS XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sqrtss_non_perfect_square() {
    // Test square root of non-perfect square (2.0)
    let code = [
        0xf3, 0x0f, 0x51, 0xd3, // SQRTSS XMM2, XMM3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sqrtss_zero() {
    // Test square root of zero
    let code = [
        0xf3, 0x0f, 0x51, 0xe5, // SQRTSS XMM4, XMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sqrtss_upper_bits_preserved() {
    // Test that upper bits [127:32] are preserved from first source
    let code = [
        0xf3, 0x0f, 0x51, 0xf7, // SQRTSS XMM6, XMM7
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// SQRTSD Tests - Scalar Double Precision Square Root
// ============================================================================

#[test]
fn test_sqrtsd_xmm0_xmm1() {
    // SQRTSD XMM0, XMM1
    let code = [
        0xf2, 0x0f, 0x51, 0xc1, // SQRTSD XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sqrtsd_xmm1_xmm2() {
    // SQRTSD XMM1, XMM2
    let code = [
        0xf2, 0x0f, 0x51, 0xca, // SQRTSD XMM1, XMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sqrtsd_xmm2_xmm3() {
    // SQRTSD XMM2, XMM3
    let code = [
        0xf2, 0x0f, 0x51, 0xd3, // SQRTSD XMM2, XMM3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sqrtsd_xmm3_xmm4() {
    // SQRTSD XMM3, XMM4
    let code = [
        0xf2, 0x0f, 0x51, 0xdc, // SQRTSD XMM3, XMM4
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sqrtsd_xmm4_xmm5() {
    // SQRTSD XMM4, XMM5
    let code = [
        0xf2, 0x0f, 0x51, 0xe5, // SQRTSD XMM4, XMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sqrtsd_xmm5_xmm6() {
    // SQRTSD XMM5, XMM6
    let code = [
        0xf2, 0x0f, 0x51, 0xee, // SQRTSD XMM5, XMM6
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sqrtsd_xmm6_xmm7() {
    // SQRTSD XMM6, XMM7
    let code = [
        0xf2, 0x0f, 0x51, 0xf7, // SQRTSD XMM6, XMM7
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sqrtsd_xmm7_xmm0() {
    // SQRTSD XMM7, XMM0
    let code = [
        0xf2, 0x0f, 0x51, 0xf8, // SQRTSD XMM7, XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sqrtsd_xmm8_xmm9() {
    // SQRTSD XMM8, XMM9 (requires REX prefix)
    let code = [
        0xf2, 0x45, 0x0f, 0x51, 0xc1, // SQRTSD XMM8, XMM9
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sqrtsd_xmm10_xmm11() {
    // SQRTSD XMM10, XMM11
    let code = [
        0xf2, 0x45, 0x0f, 0x51, 0xd3, // SQRTSD XMM10, XMM11
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sqrtsd_xmm12_xmm13() {
    // SQRTSD XMM12, XMM13
    let code = [
        0xf2, 0x45, 0x0f, 0x51, 0xe5, // SQRTSD XMM12, XMM13
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sqrtsd_xmm14_xmm15() {
    // SQRTSD XMM14, XMM15
    let code = [
        0xf2, 0x45, 0x0f, 0x51, 0xf7, // SQRTSD XMM14, XMM15
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sqrtsd_xmm15_xmm0() {
    // SQRTSD XMM15, XMM0
    let code = [
        0xf2, 0x44, 0x0f, 0x51, 0xf8, // SQRTSD XMM15, XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sqrtsd_xmm0_mem() {
    // SQRTSD XMM0, [ALIGNED_ADDR]
    let code = [
        0xf2, 0x0f, 0x51, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // SQRTSD XMM0, [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sqrtsd_xmm7_mem() {
    // SQRTSD XMM7, [ALIGNED_ADDR]
    let code = [
        0xf2, 0x0f, 0x51, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // SQRTSD XMM7, [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sqrtsd_xmm15_mem() {
    // SQRTSD XMM15, [ALIGNED_ADDR]
    let code = [
        0xf2, 0x44, 0x0f, 0x51, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // SQRTSD XMM15, [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sqrtsd_perfect_square() {
    // Test square root of perfect square (9.0 -> 3.0)
    let code = [
        0xf2, 0x0f, 0x51, 0xc1, // SQRTSD XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sqrtsd_non_perfect_square() {
    // Test square root of non-perfect square (2.0)
    let code = [
        0xf2, 0x0f, 0x51, 0xd3, // SQRTSD XMM2, XMM3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sqrtsd_zero() {
    // Test square root of zero
    let code = [
        0xf2, 0x0f, 0x51, 0xe5, // SQRTSD XMM4, XMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sqrtsd_upper_bits_preserved() {
    // Test that upper bits [127:64] are preserved from first source
    let code = [
        0xf2, 0x0f, 0x51, 0xf7, // SQRTSD XMM6, XMM7
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
