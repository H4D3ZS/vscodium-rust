use crate::common::*;
use rax::cpu::Registers;
use vm_memory::{Bytes, GuestAddress};

// VCVTTPS2DQ - Convert with Truncation Packed Single-Precision to Packed Doubleword Integers
// VCVTTPD2DQ - Convert with Truncation Packed Double-Precision to Packed Doubleword Integers
//
// VCVTTPS2DQ converts packed single-precision floating-point values to packed signed integers,
// using truncation (round toward zero).
// VCVTTPD2DQ converts packed double-precision floating-point values to packed signed integers,
// using truncation (round toward zero).
//
// Opcodes:
// VEX.128.F3.0F.WIG 5B /r VCVTTPS2DQ xmm1, xmm2/m128   - Convert with truncation float32 to int32
// VEX.256.F3.0F.WIG 5B /r VCVTTPS2DQ ymm1, ymm2/m256   - Convert with truncation float32 to int32
// VEX.128.66.0F.WIG E6 /r VCVTTPD2DQ xmm1, xmm2/m128   - Convert with truncation float64 to int32
// VEX.256.66.0F.WIG E6 /r VCVTTPD2DQ xmm1, ymm2/m256   - Convert with truncation float64 to int32

const ALIGNED_ADDR: u64 = 0x3000; // 32-byte aligned address for testing

// ============================================================================
// VCVTTPS2DQ Tests - 128-bit XMM registers (4x float32 to 4x int32)
// ============================================================================

#[test]
fn test_vcvttps2dq_xmm0_xmm1() {
    // VCVTTPS2DQ XMM0, XMM1
    let code = [
        0xc5, 0xfa, 0x5b, 0xc1, // VCVTTPS2DQ XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vcvttps2dq_xmm1_xmm2() {
    // VCVTTPS2DQ XMM1, XMM2
    let code = [
        0xc5, 0xfa, 0x5b, 0xca, // VCVTTPS2DQ XMM1, XMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vcvttps2dq_xmm2_xmm3() {
    // VCVTTPS2DQ XMM2, XMM3
    let code = [
        0xc5, 0xfa, 0x5b, 0xd3, // VCVTTPS2DQ XMM2, XMM3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vcvttps2dq_xmm3_xmm4() {
    // VCVTTPS2DQ XMM3, XMM4
    let code = [
        0xc5, 0xfa, 0x5b, 0xdc, // VCVTTPS2DQ XMM3, XMM4
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vcvttps2dq_xmm4_xmm5() {
    // VCVTTPS2DQ XMM4, XMM5
    let code = [
        0xc5, 0xfa, 0x5b, 0xe5, // VCVTTPS2DQ XMM4, XMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vcvttps2dq_xmm5_xmm6() {
    // VCVTTPS2DQ XMM5, XMM6
    let code = [
        0xc5, 0xfa, 0x5b, 0xee, // VCVTTPS2DQ XMM5, XMM6
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vcvttps2dq_xmm6_xmm7() {
    // VCVTTPS2DQ XMM6, XMM7
    let code = [
        0xc5, 0xfa, 0x5b, 0xf7, // VCVTTPS2DQ XMM6, XMM7
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vcvttps2dq_xmm7_xmm0() {
    // VCVTTPS2DQ XMM7, XMM0
    let code = [
        0xc5, 0xfa, 0x5b, 0xf8, // VCVTTPS2DQ XMM7, XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// VCVTTPS2DQ Tests - Extended XMM registers (XMM8-XMM15)
// ============================================================================

#[test]
fn test_vcvttps2dq_xmm8_xmm9() {
    // VCVTTPS2DQ XMM8, XMM9
    let code = [
        0xc4, 0x41, 0x7a, 0x5b, 0xc1, // VCVTTPS2DQ XMM8, XMM9
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vcvttps2dq_xmm9_xmm10() {
    // VCVTTPS2DQ XMM9, XMM10
    let code = [
        0xc4, 0x41, 0x7a, 0x5b, 0xca, // VCVTTPS2DQ XMM9, XMM10
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vcvttps2dq_xmm10_xmm11() {
    // VCVTTPS2DQ XMM10, XMM11
    let code = [
        0xc4, 0x41, 0x7a, 0x5b, 0xd3, // VCVTTPS2DQ XMM10, XMM11
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vcvttps2dq_xmm11_xmm12() {
    // VCVTTPS2DQ XMM11, XMM12
    let code = [
        0xc4, 0x41, 0x7a, 0x5b, 0xdc, // VCVTTPS2DQ XMM11, XMM12
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vcvttps2dq_xmm12_xmm13() {
    // VCVTTPS2DQ XMM12, XMM13
    let code = [
        0xc4, 0x41, 0x7a, 0x5b, 0xe5, // VCVTTPS2DQ XMM12, XMM13
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vcvttps2dq_xmm13_xmm14() {
    // VCVTTPS2DQ XMM13, XMM14
    let code = [
        0xc4, 0x41, 0x7a, 0x5b, 0xee, // VCVTTPS2DQ XMM13, XMM14
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vcvttps2dq_xmm14_xmm15() {
    // VCVTTPS2DQ XMM14, XMM15
    let code = [
        0xc4, 0x41, 0x7a, 0x5b, 0xf7, // VCVTTPS2DQ XMM14, XMM15
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vcvttps2dq_xmm15_xmm8() {
    // VCVTTPS2DQ XMM15, XMM8
    let code = [
        0xc4, 0x41, 0x7a, 0x5b, 0xf8, // VCVTTPS2DQ XMM15, XMM8
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// VCVTTPS2DQ Tests - Cross-domain XMM registers
// ============================================================================

#[test]
fn test_vcvttps2dq_xmm0_xmm8() {
    // VCVTTPS2DQ XMM0, XMM8
    let code = [
        0xc4, 0xc1, 0x7a, 0x5b, 0xc0, // VCVTTPS2DQ XMM0, XMM8
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vcvttps2dq_xmm8_xmm0() {
    // VCVTTPS2DQ XMM8, XMM0
    let code = [
        0xc4, 0xc1, 0x7a, 0x5b, 0xc0, // VCVTTPS2DQ XMM8, XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vcvttps2dq_xmm7_xmm15() {
    // VCVTTPS2DQ XMM7, XMM15
    let code = [
        0xc4, 0xc1, 0x7a, 0x5b, 0xff, // VCVTTPS2DQ XMM7, XMM15
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// VCVTTPS2DQ Tests - 256-bit YMM registers (8x float32 to 8x int32)
// ============================================================================

#[test]
fn test_vcvttps2dq_ymm0_ymm1() {
    // VCVTTPS2DQ YMM0, YMM1
    let code = [
        0xc5, 0xfe, 0x5b, 0xc1, // VCVTTPS2DQ YMM0, YMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vcvttps2dq_ymm1_ymm2() {
    // VCVTTPS2DQ YMM1, YMM2
    let code = [
        0xc5, 0xfe, 0x5b, 0xca, // VCVTTPS2DQ YMM1, YMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vcvttps2dq_ymm2_ymm3() {
    // VCVTTPS2DQ YMM2, YMM3
    let code = [
        0xc5, 0xfe, 0x5b, 0xd3, // VCVTTPS2DQ YMM2, YMM3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vcvttps2dq_ymm3_ymm4() {
    // VCVTTPS2DQ YMM3, YMM4
    let code = [
        0xc5, 0xfe, 0x5b, 0xdc, // VCVTTPS2DQ YMM3, YMM4
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vcvttps2dq_ymm4_ymm5() {
    // VCVTTPS2DQ YMM4, YMM5
    let code = [
        0xc5, 0xfe, 0x5b, 0xe5, // VCVTTPS2DQ YMM4, YMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vcvttps2dq_ymm5_ymm6() {
    // VCVTTPS2DQ YMM5, YMM6
    let code = [
        0xc5, 0xfe, 0x5b, 0xee, // VCVTTPS2DQ YMM5, YMM6
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vcvttps2dq_ymm6_ymm7() {
    // VCVTTPS2DQ YMM6, YMM7
    let code = [
        0xc5, 0xfe, 0x5b, 0xf7, // VCVTTPS2DQ YMM6, YMM7
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vcvttps2dq_ymm7_ymm0() {
    // VCVTTPS2DQ YMM7, YMM0
    let code = [
        0xc5, 0xfe, 0x5b, 0xf8, // VCVTTPS2DQ YMM7, YMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// VCVTTPS2DQ Tests - Extended YMM registers
// ============================================================================

#[test]
fn test_vcvttps2dq_ymm8_ymm9() {
    // VCVTTPS2DQ YMM8, YMM9
    let code = [
        0xc4, 0x41, 0x7e, 0x5b, 0xc1, // VCVTTPS2DQ YMM8, YMM9
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vcvttps2dq_ymm9_ymm10() {
    // VCVTTPS2DQ YMM9, YMM10
    let code = [
        0xc4, 0x41, 0x7e, 0x5b, 0xca, // VCVTTPS2DQ YMM9, YMM10
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vcvttps2dq_ymm10_ymm11() {
    // VCVTTPS2DQ YMM10, YMM11
    let code = [
        0xc4, 0x41, 0x7e, 0x5b, 0xd3, // VCVTTPS2DQ YMM10, YMM11
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vcvttps2dq_ymm11_ymm12() {
    // VCVTTPS2DQ YMM11, YMM12
    let code = [
        0xc4, 0x41, 0x7e, 0x5b, 0xdc, // VCVTTPS2DQ YMM11, YMM12
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vcvttps2dq_ymm12_ymm13() {
    // VCVTTPS2DQ YMM12, YMM13
    let code = [
        0xc4, 0x41, 0x7e, 0x5b, 0xe5, // VCVTTPS2DQ YMM12, YMM13
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vcvttps2dq_ymm13_ymm14() {
    // VCVTTPS2DQ YMM13, YMM14
    let code = [
        0xc4, 0x41, 0x7e, 0x5b, 0xee, // VCVTTPS2DQ YMM13, YMM14
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vcvttps2dq_ymm14_ymm15() {
    // VCVTTPS2DQ YMM14, YMM15
    let code = [
        0xc4, 0x41, 0x7e, 0x5b, 0xf7, // VCVTTPS2DQ YMM14, YMM15
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vcvttps2dq_ymm15_ymm8() {
    // VCVTTPS2DQ YMM15, YMM8
    let code = [
        0xc4, 0x41, 0x7e, 0x5b, 0xf8, // VCVTTPS2DQ YMM15, YMM8
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// VCVTTPS2DQ Tests - Memory operands (128-bit)
// ============================================================================

#[test]
fn test_vcvttps2dq_xmm0_mem() {
    // VCVTTPS2DQ XMM0, [mem]
    let code = [
        0xc5, 0xfa, 0x5b, 0x05, 0x00, 0x40, 0x00, 0x00, // VCVTTPS2DQ XMM0, [rip + 0x4000]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    // Initialize memory with 4 single-precision floats
    let test_data: [u8; 16] = [
        0x9a, 0x99, 0x99, 0x3f, // 1.2
        0x66, 0x66, 0x26, 0x40, // 2.6
        0xcd, 0xcc, 0x4c, 0x40, // 3.2
        0x33, 0x33, 0x73, 0x40, // 3.8
    ];
    mem.write_slice(&test_data, GuestAddress(ALIGNED_ADDR))
        .unwrap();

    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vcvttps2dq_xmm8_mem() {
    // VCVTTPS2DQ XMM8, [mem]
    let code = [
        0xc4, 0x41, 0x7a, 0x5b, 0x05, 0x00, 0x40, 0x00,
        0x00, // VCVTTPS2DQ XMM8, [rip + 0x4000]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    let test_data: [u8; 16] = [
        0x9a, 0x99, 0x99, 0xbf, // -1.2 -> -1
        0x66, 0x66, 0x26, 0xc0, // -2.6 -> -2
        0x9a, 0x99, 0xc9, 0x42, // 100.8 -> 100
        0x33, 0x33, 0xc9, 0xc2, // -100.6 -> -100
    ];
    mem.write_slice(&test_data, GuestAddress(ALIGNED_ADDR))
        .unwrap();

    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// VCVTTPS2DQ Tests - Memory operands (256-bit)
// ============================================================================

#[test]
fn test_vcvttps2dq_ymm0_mem() {
    // VCVTTPS2DQ YMM0, [mem]
    let code = [
        0xc5, 0xfe, 0x5b, 0x05, 0x00, 0x40, 0x00, 0x00, // VCVTTPS2DQ YMM0, [rip + 0x4000]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    // Initialize memory with 8 single-precision floats
    let test_data: [u8; 32] = [
        0x9a, 0x99, 0x99, 0x3f, // 1.2
        0x66, 0x66, 0x26, 0x40, // 2.6
        0xcd, 0xcc, 0x4c, 0x40, // 3.2
        0x33, 0x33, 0x73, 0x40, // 3.8
        0x9a, 0x99, 0x99, 0xbf, // -1.2
        0x66, 0x66, 0x26, 0xc0, // -2.6
        0xcd, 0xcc, 0x4c, 0xc0, // -3.2
        0x33, 0x33, 0x73, 0xc0, // -3.8
    ];
    mem.write_slice(&test_data, GuestAddress(ALIGNED_ADDR))
        .unwrap();

    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vcvttps2dq_ymm8_mem() {
    // VCVTTPS2DQ YMM8, [mem]
    let code = [
        0xc4, 0x41, 0x7e, 0x5b, 0x05, 0x00, 0x40, 0x00,
        0x00, // VCVTTPS2DQ YMM8, [rip + 0x4000]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    let test_data: [u8; 32] = [
        0x00, 0x00, 0x80, 0x3f, // 1.0
        0x9a, 0x99, 0x19, 0x3f, // 0.6 -> 0
        0x33, 0x33, 0xb3, 0x3f, // 1.4 -> 1
        0xcd, 0xcc, 0x8c, 0x3f, // 1.1 -> 1
        0xcd, 0xcc, 0xcc, 0x3f, // 1.6 -> 1
        0x66, 0x66, 0x06, 0x40, // 2.1 -> 2
        0x9a, 0x99, 0x19, 0x40, // 2.4 -> 2
        0x66, 0x66, 0x26, 0x40, // 2.6 -> 2
    ];
    mem.write_slice(&test_data, GuestAddress(ALIGNED_ADDR))
        .unwrap();

    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// VCVTTPD2DQ Tests - 128-bit (convert 2x float64 to 2x int32)
// ============================================================================

#[test]
fn test_vcvttpd2dq_xmm0_xmm1() {
    // VCVTTPD2DQ XMM0, XMM1
    let code = [
        0xc5, 0xf9, 0xe6, 0xc1, // VCVTTPD2DQ XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vcvttpd2dq_xmm1_xmm2() {
    // VCVTTPD2DQ XMM1, XMM2
    let code = [
        0xc5, 0xf9, 0xe6, 0xca, // VCVTTPD2DQ XMM1, XMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vcvttpd2dq_xmm2_xmm3() {
    // VCVTTPD2DQ XMM2, XMM3
    let code = [
        0xc5, 0xf9, 0xe6, 0xd3, // VCVTTPD2DQ XMM2, XMM3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vcvttpd2dq_xmm3_xmm4() {
    // VCVTTPD2DQ XMM3, XMM4
    let code = [
        0xc5, 0xf9, 0xe6, 0xdc, // VCVTTPD2DQ XMM3, XMM4
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vcvttpd2dq_xmm4_xmm5() {
    // VCVTTPD2DQ XMM4, XMM5
    let code = [
        0xc5, 0xf9, 0xe6, 0xe5, // VCVTTPD2DQ XMM4, XMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vcvttpd2dq_xmm5_xmm6() {
    // VCVTTPD2DQ XMM5, XMM6
    let code = [
        0xc5, 0xf9, 0xe6, 0xee, // VCVTTPD2DQ XMM5, XMM6
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vcvttpd2dq_xmm6_xmm7() {
    // VCVTTPD2DQ XMM6, XMM7
    let code = [
        0xc5, 0xf9, 0xe6, 0xf7, // VCVTTPD2DQ XMM6, XMM7
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vcvttpd2dq_xmm7_xmm0() {
    // VCVTTPD2DQ XMM7, XMM0
    let code = [
        0xc5, 0xf9, 0xe6, 0xf8, // VCVTTPD2DQ XMM7, XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// VCVTTPD2DQ Tests - Extended XMM registers
// ============================================================================

#[test]
fn test_vcvttpd2dq_xmm8_xmm9() {
    // VCVTTPD2DQ XMM8, XMM9
    let code = [
        0xc4, 0x41, 0x79, 0xe6, 0xc1, // VCVTTPD2DQ XMM8, XMM9
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vcvttpd2dq_xmm9_xmm10() {
    // VCVTTPD2DQ XMM9, XMM10
    let code = [
        0xc4, 0x41, 0x79, 0xe6, 0xca, // VCVTTPD2DQ XMM9, XMM10
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vcvttpd2dq_xmm10_xmm11() {
    // VCVTTPD2DQ XMM10, XMM11
    let code = [
        0xc4, 0x41, 0x79, 0xe6, 0xd3, // VCVTTPD2DQ XMM10, XMM11
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vcvttpd2dq_xmm11_xmm12() {
    // VCVTTPD2DQ XMM11, XMM12
    let code = [
        0xc4, 0x41, 0x79, 0xe6, 0xdc, // VCVTTPD2DQ XMM11, XMM12
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vcvttpd2dq_xmm12_xmm13() {
    // VCVTTPD2DQ XMM12, XMM13
    let code = [
        0xc4, 0x41, 0x79, 0xe6, 0xe5, // VCVTTPD2DQ XMM12, XMM13
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vcvttpd2dq_xmm13_xmm14() {
    // VCVTTPD2DQ XMM13, XMM14
    let code = [
        0xc4, 0x41, 0x79, 0xe6, 0xee, // VCVTTPD2DQ XMM13, XMM14
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vcvttpd2dq_xmm14_xmm15() {
    // VCVTTPD2DQ XMM14, XMM15
    let code = [
        0xc4, 0x41, 0x79, 0xe6, 0xf7, // VCVTTPD2DQ XMM14, XMM15
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vcvttpd2dq_xmm15_xmm8() {
    // VCVTTPD2DQ XMM15, XMM8
    let code = [
        0xc4, 0x41, 0x79, 0xe6, 0xf8, // VCVTTPD2DQ XMM15, XMM8
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// VCVTTPD2DQ Tests - 256-bit (convert 4x float64 to 4x int32)
// ============================================================================

#[test]
fn test_vcvttpd2dq_xmm0_ymm1() {
    // VCVTTPD2DQ XMM0, YMM1
    let code = [
        0xc5, 0xfd, 0xe6, 0xc1, // VCVTTPD2DQ XMM0, YMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vcvttpd2dq_xmm1_ymm2() {
    // VCVTTPD2DQ XMM1, YMM2
    let code = [
        0xc5, 0xfd, 0xe6, 0xca, // VCVTTPD2DQ XMM1, YMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vcvttpd2dq_xmm2_ymm3() {
    // VCVTTPD2DQ XMM2, YMM3
    let code = [
        0xc5, 0xfd, 0xe6, 0xd3, // VCVTTPD2DQ XMM2, YMM3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vcvttpd2dq_xmm3_ymm4() {
    // VCVTTPD2DQ XMM3, YMM4
    let code = [
        0xc5, 0xfd, 0xe6, 0xdc, // VCVTTPD2DQ XMM3, YMM4
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vcvttpd2dq_xmm4_ymm5() {
    // VCVTTPD2DQ XMM4, YMM5
    let code = [
        0xc5, 0xfd, 0xe6, 0xe5, // VCVTTPD2DQ XMM4, YMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vcvttpd2dq_xmm5_ymm6() {
    // VCVTTPD2DQ XMM5, YMM6
    let code = [
        0xc5, 0xfd, 0xe6, 0xee, // VCVTTPD2DQ XMM5, YMM6
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vcvttpd2dq_xmm6_ymm7() {
    // VCVTTPD2DQ XMM6, YMM7
    let code = [
        0xc5, 0xfd, 0xe6, 0xf7, // VCVTTPD2DQ XMM6, YMM7
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vcvttpd2dq_xmm7_ymm0() {
    // VCVTTPD2DQ XMM7, YMM0
    let code = [
        0xc5, 0xfd, 0xe6, 0xf8, // VCVTTPD2DQ XMM7, YMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// VCVTTPD2DQ Tests - Extended YMM registers
// ============================================================================

#[test]
fn test_vcvttpd2dq_xmm8_ymm9() {
    // VCVTTPD2DQ XMM8, YMM9
    let code = [
        0xc4, 0x41, 0x7d, 0xe6, 0xc1, // VCVTTPD2DQ XMM8, YMM9
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vcvttpd2dq_xmm9_ymm10() {
    // VCVTTPD2DQ XMM9, YMM10
    let code = [
        0xc4, 0x41, 0x7d, 0xe6, 0xca, // VCVTTPD2DQ XMM9, YMM10
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vcvttpd2dq_xmm10_ymm11() {
    // VCVTTPD2DQ XMM10, YMM11
    let code = [
        0xc4, 0x41, 0x7d, 0xe6, 0xd3, // VCVTTPD2DQ XMM10, YMM11
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vcvttpd2dq_xmm11_ymm12() {
    // VCVTTPD2DQ XMM11, YMM12
    let code = [
        0xc4, 0x41, 0x7d, 0xe6, 0xdc, // VCVTTPD2DQ XMM11, YMM12
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vcvttpd2dq_xmm12_ymm13() {
    // VCVTTPD2DQ XMM12, YMM13
    let code = [
        0xc4, 0x41, 0x7d, 0xe6, 0xe5, // VCVTTPD2DQ XMM12, YMM13
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vcvttpd2dq_xmm13_ymm14() {
    // VCVTTPD2DQ XMM13, YMM14
    let code = [
        0xc4, 0x41, 0x7d, 0xe6, 0xee, // VCVTTPD2DQ XMM13, YMM14
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vcvttpd2dq_xmm14_ymm15() {
    // VCVTTPD2DQ XMM14, YMM15
    let code = [
        0xc4, 0x41, 0x7d, 0xe6, 0xf7, // VCVTTPD2DQ XMM14, YMM15
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vcvttpd2dq_xmm15_ymm8() {
    // VCVTTPD2DQ XMM15, YMM8
    let code = [
        0xc4, 0x41, 0x7d, 0xe6, 0xf8, // VCVTTPD2DQ XMM15, YMM8
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// VCVTTPD2DQ Tests - Memory operands (128-bit)
// ============================================================================

#[test]
fn test_vcvttpd2dq_xmm0_mem() {
    // VCVTTPD2DQ XMM0, [mem]
    let code = [
        0xc5, 0xf9, 0xe6, 0x05, 0x00, 0x40, 0x00, 0x00, // VCVTTPD2DQ XMM0, [rip + 0x4000]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    // Initialize memory with 2 double-precision floats
    let test_data: [u8; 16] = [
        0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0xf3, 0x3f, // 1.2
        0x9a, 0x99, 0x99, 0x99, 0x99, 0x99, 0x04, 0x40, // 2.6
    ];
    mem.write_slice(&test_data, GuestAddress(ALIGNED_ADDR))
        .unwrap();

    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vcvttpd2dq_xmm8_mem() {
    // VCVTTPD2DQ XMM8, [mem]
    let code = [
        0xc4, 0x41, 0x79, 0xe6, 0x05, 0x00, 0x40, 0x00,
        0x00, // VCVTTPD2DQ XMM8, [rip + 0x4000]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    let test_data: [u8; 16] = [
        0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0xf3, 0xbf, // -1.2 -> -1
        0x9a, 0x99, 0x99, 0x99, 0x99, 0x99, 0x04, 0xc0, // -2.6 -> -2
    ];
    mem.write_slice(&test_data, GuestAddress(ALIGNED_ADDR))
        .unwrap();

    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// VCVTTPD2DQ Tests - Memory operands (256-bit)
// ============================================================================

#[test]
fn test_vcvttpd2dq_xmm0_mem256() {
    // VCVTTPD2DQ XMM0, [mem]
    let code = [
        0xc5, 0xfd, 0xe6, 0x05, 0x00, 0x40, 0x00, 0x00, // VCVTTPD2DQ XMM0, [rip + 0x4000]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    // Initialize memory with 4 double-precision floats
    let test_data: [u8; 32] = [
        0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0xf3, 0x3f, // 1.2
        0x9a, 0x99, 0x99, 0x99, 0x99, 0x99, 0x04, 0x40, // 2.6
        0x9a, 0x99, 0x99, 0x99, 0x99, 0x99, 0x09, 0x40, // 3.2
        0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x0e, 0x40, // 3.8
    ];
    mem.write_slice(&test_data, GuestAddress(ALIGNED_ADDR))
        .unwrap();

    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vcvttpd2dq_xmm8_mem256() {
    // VCVTTPD2DQ XMM8, [mem]
    let code = [
        0xc4, 0x41, 0x7d, 0xe6, 0x05, 0x00, 0x40, 0x00,
        0x00, // VCVTTPD2DQ XMM8, [rip + 0x4000]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    let test_data: [u8; 32] = [
        0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0xf3, 0xbf, // -1.2 -> -1
        0x9a, 0x99, 0x99, 0x99, 0x99, 0x99, 0x04, 0xc0, // -2.6 -> -2
        0x9a, 0x99, 0x99, 0x99, 0x99, 0x99, 0x09, 0xc0, // -3.2 -> -3
        0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x0e, 0xc0, // -3.8 -> -3
    ];
    mem.write_slice(&test_data, GuestAddress(ALIGNED_ADDR))
        .unwrap();

    run_until_hlt(&mut vcpu).unwrap();
}
