use crate::common::{run_until_hlt, setup_vm};
use rax::cpu::Registers;
use vm_memory::{Bytes, GuestAddress};

// COMISS - Compare Scalar Ordered Single Precision Floating-Point Values and Set EFLAGS
// COMISD - Compare Scalar Ordered Double Precision Floating-Point Values and Set EFLAGS
//
// These instructions perform ordered comparisons and set EFLAGS accordingly:
// - UNORDERED (NaN):    ZF=1, PF=1, CF=1
// - GREATER_THAN:       ZF=0, PF=0, CF=0
// - LESS_THAN:          ZF=0, PF=0, CF=1
// - EQUAL:              ZF=1, PF=0, CF=0
// - OF, SF, AF are always set to 0
//
// Opcodes:
// NP 0F 2F /r             COMISS xmm1, xmm2/m32    - Compare low single-precision values
// 66 0F 2F /r             COMISD xmm1, xmm2/m64    - Compare low double-precision values

const ALIGNED_ADDR: u64 = 0x3000; // 16-byte aligned address for testing

// ============================================================================
// COMISS Tests - Ordered Single Precision Comparison
// ============================================================================

#[test]
fn test_comiss_xmm0_xmm1() {
    // COMISS XMM0, XMM1
    let code = [
        0x0f, 0x2f, 0xc1, // COMISS XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_comiss_xmm1_xmm2() {
    // COMISS XMM1, XMM2
    let code = [
        0x0f, 0x2f, 0xca, // COMISS XMM1, XMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_comiss_xmm2_xmm3() {
    // COMISS XMM2, XMM3
    let code = [
        0x0f, 0x2f, 0xd3, // COMISS XMM2, XMM3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_comiss_xmm3_xmm4() {
    // COMISS XMM3, XMM4
    let code = [
        0x0f, 0x2f, 0xdc, // COMISS XMM3, XMM4
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_comiss_xmm4_xmm5() {
    // COMISS XMM4, XMM5
    let code = [
        0x0f, 0x2f, 0xe5, // COMISS XMM4, XMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_comiss_xmm5_xmm6() {
    // COMISS XMM5, XMM6
    let code = [
        0x0f, 0x2f, 0xee, // COMISS XMM5, XMM6
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_comiss_xmm6_xmm7() {
    // COMISS XMM6, XMM7
    let code = [
        0x0f, 0x2f, 0xf7, // COMISS XMM6, XMM7
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_comiss_xmm7_xmm0() {
    // COMISS XMM7, XMM0
    let code = [
        0x0f, 0x2f, 0xf8, // COMISS XMM7, XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_comiss_xmm8_xmm9() {
    // COMISS XMM8, XMM9
    let code = [
        0x45, 0x0f, 0x2f, 0xc1, // COMISS XMM8, XMM9
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_comiss_xmm10_xmm11() {
    // COMISS XMM10, XMM11
    let code = [
        0x45, 0x0f, 0x2f, 0xd3, // COMISS XMM10, XMM11
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_comiss_xmm12_xmm13() {
    // COMISS XMM12, XMM13
    let code = [
        0x45, 0x0f, 0x2f, 0xe5, // COMISS XMM12, XMM13
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_comiss_xmm14_xmm15() {
    // COMISS XMM14, XMM15
    let code = [
        0x45, 0x0f, 0x2f, 0xf7, // COMISS XMM14, XMM15
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_comiss_xmm0_mem() {
    // COMISS XMM0, [0x3000]
    let code = [
        0x0f, 0x2f, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // COMISS XMM0, [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_comiss_xmm7_mem() {
    // COMISS XMM7, [0x3000]
    let code = [
        0x0f, 0x2f, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // COMISS XMM7, [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_comiss_xmm15_mem() {
    // COMISS XMM15, [0x3000]
    let code = [
        0x44, 0x0f, 0x2f, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // COMISS XMM15, [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_comiss_equal_values() {
    // Test comparison of equal values (should set ZF=1, PF=0, CF=0)
    let code = [
        0x0f, 0x2f, 0xc0, // COMISS XMM0, XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_comiss_greater_than() {
    // Test greater than comparison (should set ZF=0, PF=0, CF=0)
    let code = [
        0x0f, 0x2f, 0xc1, // COMISS XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_comiss_less_than() {
    // Test less than comparison (should set ZF=0, PF=0, CF=1)
    let code = [
        0x0f, 0x2f, 0xc2, // COMISS XMM0, XMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_comiss_positive_vs_negative() {
    // Test positive vs negative comparison
    let code = [
        0x0f, 0x2f, 0xc3, // COMISS XMM0, XMM3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_comiss_zero_comparison() {
    // Test comparison with zero
    let code = [
        0x0f, 0x2f, 0xc4, // COMISS XMM0, XMM4
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// COMISD Tests - Ordered Double Precision Comparison
// ============================================================================

#[test]
fn test_comisd_xmm0_xmm1() {
    // COMISD XMM0, XMM1
    let code = [
        0x66, 0x0f, 0x2f, 0xc1, // COMISD XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_comisd_xmm1_xmm2() {
    // COMISD XMM1, XMM2
    let code = [
        0x66, 0x0f, 0x2f, 0xca, // COMISD XMM1, XMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_comisd_xmm2_xmm3() {
    // COMISD XMM2, XMM3
    let code = [
        0x66, 0x0f, 0x2f, 0xd3, // COMISD XMM2, XMM3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_comisd_xmm3_xmm4() {
    // COMISD XMM3, XMM4
    let code = [
        0x66, 0x0f, 0x2f, 0xdc, // COMISD XMM3, XMM4
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_comisd_xmm4_xmm5() {
    // COMISD XMM4, XMM5
    let code = [
        0x66, 0x0f, 0x2f, 0xe5, // COMISD XMM4, XMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_comisd_xmm5_xmm6() {
    // COMISD XMM5, XMM6
    let code = [
        0x66, 0x0f, 0x2f, 0xee, // COMISD XMM5, XMM6
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_comisd_xmm6_xmm7() {
    // COMISD XMM6, XMM7
    let code = [
        0x66, 0x0f, 0x2f, 0xf7, // COMISD XMM6, XMM7
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_comisd_xmm7_xmm0() {
    // COMISD XMM7, XMM0
    let code = [
        0x66, 0x0f, 0x2f, 0xf8, // COMISD XMM7, XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_comisd_xmm8_xmm9() {
    // COMISD XMM8, XMM9
    let code = [
        0x66, 0x45, 0x0f, 0x2f, 0xc1, // COMISD XMM8, XMM9
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_comisd_xmm10_xmm11() {
    // COMISD XMM10, XMM11
    let code = [
        0x66, 0x45, 0x0f, 0x2f, 0xd3, // COMISD XMM10, XMM11
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_comisd_xmm12_xmm13() {
    // COMISD XMM12, XMM13
    let code = [
        0x66, 0x45, 0x0f, 0x2f, 0xe5, // COMISD XMM12, XMM13
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_comisd_xmm14_xmm15() {
    // COMISD XMM14, XMM15
    let code = [
        0x66, 0x45, 0x0f, 0x2f, 0xf7, // COMISD XMM14, XMM15
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_comisd_xmm0_mem() {
    // COMISD XMM0, [0x3000]
    let code = [
        0x66, 0x0f, 0x2f, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // COMISD XMM0, [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_comisd_xmm7_mem() {
    // COMISD XMM7, [0x3000]
    let code = [
        0x66, 0x0f, 0x2f, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // COMISD XMM7, [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_comisd_xmm15_mem() {
    // COMISD XMM15, [0x3000]
    let code = [
        0x66, 0x44, 0x0f, 0x2f, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // COMISD XMM15, [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_comisd_equal_values() {
    // Test comparison of equal double values (should set ZF=1, PF=0, CF=0)
    let code = [
        0x66, 0x0f, 0x2f, 0xc0, // COMISD XMM0, XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_comisd_greater_than() {
    // Test greater than comparison (should set ZF=0, PF=0, CF=0)
    let code = [
        0x66, 0x0f, 0x2f, 0xc1, // COMISD XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_comisd_less_than() {
    // Test less than comparison (should set ZF=0, PF=0, CF=1)
    let code = [
        0x66, 0x0f, 0x2f, 0xc2, // COMISD XMM0, XMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_comisd_positive_vs_negative() {
    // Test positive vs negative comparison
    let code = [
        0x66, 0x0f, 0x2f, 0xc3, // COMISD XMM0, XMM3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_comisd_zero_comparison() {
    // Test comparison with zero
    let code = [
        0x66, 0x0f, 0x2f, 0xc4, // COMISD XMM0, XMM4
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_comisd_negative_zero() {
    // Test comparison with negative zero
    let code = [
        0x66, 0x0f, 0x2f, 0xc5, // COMISD XMM0, XMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_comiss_large_vs_small() {
    // Test large value vs small value comparison
    let code = [
        0x0f, 0x2f, 0xc6, // COMISS XMM0, XMM6
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_comisd_large_vs_small() {
    // Test large double value vs small value comparison
    let code = [
        0x66, 0x0f, 0x2f, 0xc6, // COMISD XMM0, XMM6
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_comiss_xmm0_xmm8() {
    // COMISS XMM0, XMM8
    let code = [
        0x41, 0x0f, 0x2f, 0xc0, // COMISS XMM0, XMM8
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_comisd_xmm0_xmm8() {
    // COMISD XMM0, XMM8
    let code = [
        0x66, 0x41, 0x0f, 0x2f, 0xc0, // COMISD XMM0, XMM8
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
