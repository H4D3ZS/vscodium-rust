use crate::common::{run_until_hlt, setup_vm};
use rax::cpu::Registers;
use vm_memory::{Bytes, GuestAddress};

// VADDSS - Add Scalar Single-Precision Floating-Point Values
// VADDSD - Add Scalar Double-Precision Floating-Point Values
//
// VADDSS adds the low single-precision floating-point value from the second source
// operand and the low single-precision floating-point value of the third source operand,
// and stores the result in the low doubleword of the destination operand.
//
// VADDSD adds the low double-precision floating-point value from the second source
// operand and the low double-precision floating-point value of the third source operand,
// and stores the result in the low quadword of the destination operand.
//
// Opcodes:
// VEX.LIG.F3.0F.WIG 58 /r    VADDSS xmm1, xmm2, xmm3/m32   - Add scalar single from xmm3/mem to xmm2
// VEX.LIG.F2.0F.WIG 58 /r    VADDSD xmm1, xmm2, xmm3/m64   - Add scalar double from xmm3/mem to xmm2

const ALIGNED_ADDR: u64 = 0x3000; // 32-byte aligned address for testing

// ============================================================================
// VADDSS Tests - Scalar Single-Precision
// ============================================================================

#[test]
fn test_vaddss_xmm0_xmm1_xmm2() {
    // VADDSS XMM0, XMM1, XMM2
    let code = [
        0xc5, 0xf2, 0x58, 0xc2, // VADDSS XMM0, XMM1, XMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vaddss_xmm1_xmm2_xmm3() {
    // VADDSS XMM1, XMM2, XMM3
    let code = [
        0xc5, 0xea, 0x58, 0xcb, // VADDSS XMM1, XMM2, XMM3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vaddss_xmm2_xmm3_xmm4() {
    // VADDSS XMM2, XMM3, XMM4
    let code = [
        0xc5, 0xe2, 0x58, 0xd4, // VADDSS XMM2, XMM3, XMM4
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vaddss_xmm3_xmm4_xmm5() {
    // VADDSS XMM3, XMM4, XMM5
    let code = [
        0xc5, 0xda, 0x58, 0xdd, // VADDSS XMM3, XMM4, XMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vaddss_xmm4_xmm5_xmm6() {
    // VADDSS XMM4, XMM5, XMM6
    let code = [
        0xc5, 0xd2, 0x58, 0xe6, // VADDSS XMM4, XMM5, XMM6
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vaddss_xmm5_xmm6_xmm7() {
    // VADDSS XMM5, XMM6, XMM7
    let code = [
        0xc5, 0xca, 0x58, 0xef, // VADDSS XMM5, XMM6, XMM7
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vaddss_xmm6_xmm7_xmm8() {
    // VADDSS XMM6, XMM7, XMM8
    let code = [
        0xc4, 0xc1, 0x42, 0x58, 0xf0, // VADDSS XMM6, XMM7, XMM8
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vaddss_xmm7_xmm8_xmm9() {
    // VADDSS XMM7, XMM8, XMM9
    let code = [
        0xc4, 0xc1, 0x3a, 0x58, 0xf9, // VADDSS XMM7, XMM8, XMM9
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vaddss_xmm8_xmm9_xmm10() {
    // VADDSS XMM8, XMM9, XMM10
    let code = [
        0xc4, 0x41, 0x32, 0x58, 0xc2, // VADDSS XMM8, XMM9, XMM10
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vaddss_xmm9_xmm10_xmm11() {
    // VADDSS XMM9, XMM10, XMM11
    let code = [
        0xc4, 0x41, 0x2a, 0x58, 0xcb, // VADDSS XMM9, XMM10, XMM11
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vaddss_xmm10_xmm11_xmm12() {
    // VADDSS XMM10, XMM11, XMM12
    let code = [
        0xc4, 0x41, 0x22, 0x58, 0xd4, // VADDSS XMM10, XMM11, XMM12
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vaddss_xmm11_xmm12_xmm13() {
    // VADDSS XMM11, XMM12, XMM13
    let code = [
        0xc4, 0x41, 0x1a, 0x58, 0xdd, // VADDSS XMM11, XMM12, XMM13
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vaddss_xmm12_xmm13_xmm14() {
    // VADDSS XMM12, XMM13, XMM14
    let code = [
        0xc4, 0x41, 0x12, 0x58, 0xe6, // VADDSS XMM12, XMM13, XMM14
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vaddss_xmm13_xmm14_xmm15() {
    // VADDSS XMM13, XMM14, XMM15
    let code = [
        0xc4, 0x41, 0x0a, 0x58, 0xef, // VADDSS XMM13, XMM14, XMM15
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vaddss_xmm14_xmm15_xmm0() {
    // VADDSS XMM14, XMM15, XMM0
    let code = [
        0xc4, 0x61, 0x02, 0x58, 0xf0, // VADDSS XMM14, XMM15, XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vaddss_xmm15_xmm0_xmm1() {
    // VADDSS XMM15, XMM0, XMM1
    let code = [
        0xc4, 0x61, 0x7a, 0x58, 0xf9, // VADDSS XMM15, XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// Memory operand tests
#[test]
fn test_vaddss_xmm0_xmm1_mem32() {
    // VADDSS XMM0, XMM1, [0x3000]
    let code = [
        0xc5, 0xf2, 0x58, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // VADDSS XMM0, XMM1, [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, vm_memory) = setup_vm(&code, None);

    // Write test data to memory (4 bytes for float32)
    let test_data = [0x00, 0x00, 0x80, 0x3f]; // 1.0f in IEEE 754
    vm_memory
        .write(&test_data, GuestAddress(ALIGNED_ADDR))
        .unwrap();

    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vaddss_xmm2_xmm3_mem32() {
    // VADDSS XMM2, XMM3, [0x3000]
    let code = [
        0xc5, 0xe2, 0x58, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // VADDSS XMM2, XMM3, [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, vm_memory) = setup_vm(&code, None);

    let test_data = [0x00, 0x00, 0x00, 0x40]; // 2.0f in IEEE 754
    vm_memory
        .write(&test_data, GuestAddress(ALIGNED_ADDR))
        .unwrap();

    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// VADDSD Tests - Scalar Double-Precision
// ============================================================================

#[test]
fn test_vaddsd_xmm0_xmm1_xmm2() {
    // VADDSD XMM0, XMM1, XMM2
    let code = [
        0xc5, 0xf3, 0x58, 0xc2, // VADDSD XMM0, XMM1, XMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vaddsd_xmm1_xmm2_xmm3() {
    // VADDSD XMM1, XMM2, XMM3
    let code = [
        0xc5, 0xeb, 0x58, 0xcb, // VADDSD XMM1, XMM2, XMM3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vaddsd_xmm2_xmm3_xmm4() {
    // VADDSD XMM2, XMM3, XMM4
    let code = [
        0xc5, 0xe3, 0x58, 0xd4, // VADDSD XMM2, XMM3, XMM4
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vaddsd_xmm3_xmm4_xmm5() {
    // VADDSD XMM3, XMM4, XMM5
    let code = [
        0xc5, 0xdb, 0x58, 0xdd, // VADDSD XMM3, XMM4, XMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vaddsd_xmm4_xmm5_xmm6() {
    // VADDSD XMM4, XMM5, XMM6
    let code = [
        0xc5, 0xd3, 0x58, 0xe6, // VADDSD XMM4, XMM5, XMM6
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vaddsd_xmm5_xmm6_xmm7() {
    // VADDSD XMM5, XMM6, XMM7
    let code = [
        0xc5, 0xcb, 0x58, 0xef, // VADDSD XMM5, XMM6, XMM7
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vaddsd_xmm6_xmm7_xmm8() {
    // VADDSD XMM6, XMM7, XMM8
    let code = [
        0xc4, 0xc1, 0x43, 0x58, 0xf0, // VADDSD XMM6, XMM7, XMM8
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vaddsd_xmm7_xmm8_xmm9() {
    // VADDSD XMM7, XMM8, XMM9
    let code = [
        0xc4, 0xc1, 0x3b, 0x58, 0xf9, // VADDSD XMM7, XMM8, XMM9
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vaddsd_xmm8_xmm9_xmm10() {
    // VADDSD XMM8, XMM9, XMM10
    let code = [
        0xc4, 0x41, 0x33, 0x58, 0xc2, // VADDSD XMM8, XMM9, XMM10
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vaddsd_xmm9_xmm10_xmm11() {
    // VADDSD XMM9, XMM10, XMM11
    let code = [
        0xc4, 0x41, 0x2b, 0x58, 0xcb, // VADDSD XMM9, XMM10, XMM11
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vaddsd_xmm10_xmm11_xmm12() {
    // VADDSD XMM10, XMM11, XMM12
    let code = [
        0xc4, 0x41, 0x23, 0x58, 0xd4, // VADDSD XMM10, XMM11, XMM12
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vaddsd_xmm11_xmm12_xmm13() {
    // VADDSD XMM11, XMM12, XMM13
    let code = [
        0xc4, 0x41, 0x1b, 0x58, 0xdd, // VADDSD XMM11, XMM12, XMM13
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vaddsd_xmm12_xmm13_xmm14() {
    // VADDSD XMM12, XMM13, XMM14
    let code = [
        0xc4, 0x41, 0x13, 0x58, 0xe6, // VADDSD XMM12, XMM13, XMM14
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vaddsd_xmm13_xmm14_xmm15() {
    // VADDSD XMM13, XMM14, XMM15
    let code = [
        0xc4, 0x41, 0x0b, 0x58, 0xef, // VADDSD XMM13, XMM14, XMM15
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vaddsd_xmm14_xmm15_xmm0() {
    // VADDSD XMM14, XMM15, XMM0
    let code = [
        0xc4, 0x61, 0x03, 0x58, 0xf0, // VADDSD XMM14, XMM15, XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vaddsd_xmm15_xmm0_xmm1() {
    // VADDSD XMM15, XMM0, XMM1
    let code = [
        0xc4, 0x61, 0x7b, 0x58, 0xf9, // VADDSD XMM15, XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// Memory operand tests
#[test]
fn test_vaddsd_xmm0_xmm1_mem64() {
    // VADDSD XMM0, XMM1, [0x3000]
    let code = [
        0xc5, 0xf3, 0x58, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // VADDSD XMM0, XMM1, [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, vm_memory) = setup_vm(&code, None);

    // Write test data to memory (8 bytes for float64)
    let test_data = [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xf0, 0x3f]; // 1.0 in IEEE 754
    vm_memory
        .write(&test_data, GuestAddress(ALIGNED_ADDR))
        .unwrap();

    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vaddsd_xmm2_xmm3_mem64() {
    // VADDSD XMM2, XMM3, [0x3000]
    let code = [
        0xc5, 0xe3, 0x58, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // VADDSD XMM2, XMM3, [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, vm_memory) = setup_vm(&code, None);

    let test_data = [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x40]; // 2.0 in IEEE 754
    vm_memory
        .write(&test_data, GuestAddress(ALIGNED_ADDR))
        .unwrap();

    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vaddsd_xmm4_xmm5_mem64() {
    // VADDSD XMM4, XMM5, [0x3000]
    let code = [
        0xc5, 0xd3, 0x58, 0x24, 0x25, 0x00, 0x30, 0x00, 0x00, // VADDSD XMM4, XMM5, [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, vm_memory) = setup_vm(&code, None);

    let test_data = [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x08, 0x40]; // 3.0 in IEEE 754
    vm_memory
        .write(&test_data, GuestAddress(ALIGNED_ADDR))
        .unwrap();

    run_until_hlt(&mut vcpu).unwrap();
}
