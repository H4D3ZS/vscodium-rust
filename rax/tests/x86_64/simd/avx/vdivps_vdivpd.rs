use crate::common::*;
use rax::cpu::Registers;
use vm_memory::{Bytes, GuestAddress};

// VDIVPS - Divide Packed Single-Precision Floating-Point Values
// VDIVPD - Divide Packed Double-Precision Floating-Point Values
//
// VDIVPS performs element-wise division of packed single-precision floating-point values.
// VDIVPD performs element-wise division of packed double-precision floating-point values.
//
// Opcodes:
// VEX.128.0F.WIG 5E /r    VDIVPS xmm1, xmm2, xmm3/m128   - Divide xmm2 by packed single from xmm3/mem
// VEX.256.0F.WIG 5E /r    VDIVPS ymm1, ymm2, ymm3/m256   - Divide ymm2 by packed single from ymm3/mem
// VEX.128.66.0F.WIG 5E /r VDIVPD xmm1, xmm2, xmm3/m128   - Divide xmm2 by packed double from xmm3/mem
// VEX.256.66.0F.WIG 5E /r VDIVPD ymm1, ymm2, ymm3/m256   - Divide ymm2 by packed double from ymm3/mem

const ALIGNED_ADDR: u64 = 0x3000; // 32-byte aligned address for testing

// ============================================================================
// VDIVPS Tests - 128-bit XMM registers (4x float32)
// ============================================================================

#[test]
fn test_vdivps_xmm0_xmm1_xmm2() {
    // VDIVPS XMM0, XMM1, XMM2
    let code = [
        0xc5, 0xf0, 0x5e, 0xc2, // VDIVPS XMM0, XMM1, XMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vdivps_xmm1_xmm2_xmm3() {
    // VDIVPS XMM1, XMM2, XMM3
    let code = [
        0xc5, 0xe8, 0x5e, 0xcb, // VDIVPS XMM1, XMM2, XMM3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vdivps_xmm2_xmm3_xmm4() {
    // VDIVPS XMM2, XMM3, XMM4
    let code = [
        0xc5, 0xe0, 0x5e, 0xd4, // VDIVPS XMM2, XMM3, XMM4
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vdivps_xmm3_xmm4_xmm5() {
    // VDIVPS XMM3, XMM4, XMM5
    let code = [
        0xc5, 0xd8, 0x5e, 0xdd, // VDIVPS XMM3, XMM4, XMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vdivps_xmm4_xmm5_xmm6() {
    // VDIVPS XMM4, XMM5, XMM6
    let code = [
        0xc5, 0xd0, 0x5e, 0xe6, // VDIVPS XMM4, XMM5, XMM6
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vdivps_xmm5_xmm6_xmm7() {
    // VDIVPS XMM5, XMM6, XMM7
    let code = [
        0xc5, 0xc8, 0x5e, 0xef, // VDIVPS XMM5, XMM6, XMM7
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vdivps_xmm6_xmm7_xmm0() {
    // VDIVPS XMM6, XMM7, XMM0
    let code = [
        0xc5, 0xc0, 0x5e, 0xf0, // VDIVPS XMM6, XMM7, XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vdivps_xmm7_xmm0_xmm1() {
    // VDIVPS XMM7, XMM0, XMM1
    let code = [
        0xc5, 0xf8, 0x5e, 0xf9, // VDIVPS XMM7, XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// VDIVPS Tests - Extended XMM registers (XMM8-XMM15)
// ============================================================================

#[test]
fn test_vdivps_xmm8_xmm9_xmm10() {
    // VDIVPS XMM8, XMM9, XMM10
    let code = [
        0xc4, 0x41, 0x30, 0x5e, 0xc2, // VDIVPS XMM8, XMM9, XMM10
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vdivps_xmm9_xmm10_xmm11() {
    // VDIVPS XMM9, XMM10, XMM11
    let code = [
        0xc4, 0x41, 0x28, 0x5e, 0xcb, // VDIVPS XMM9, XMM10, XMM11
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vdivps_xmm10_xmm11_xmm12() {
    // VDIVPS XMM10, XMM11, XMM12
    let code = [
        0xc4, 0x41, 0x20, 0x5e, 0xd4, // VDIVPS XMM10, XMM11, XMM12
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vdivps_xmm11_xmm12_xmm13() {
    // VDIVPS XMM11, XMM12, XMM13
    let code = [
        0xc4, 0x41, 0x18, 0x5e, 0xdd, // VDIVPS XMM11, XMM12, XMM13
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vdivps_xmm12_xmm13_xmm14() {
    // VDIVPS XMM12, XMM13, XMM14
    let code = [
        0xc4, 0x41, 0x10, 0x5e, 0xe6, // VDIVPS XMM12, XMM13, XMM14
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vdivps_xmm13_xmm14_xmm15() {
    // VDIVPS XMM13, XMM14, XMM15
    let code = [
        0xc4, 0x41, 0x08, 0x5e, 0xef, // VDIVPS XMM13, XMM14, XMM15
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vdivps_xmm14_xmm15_xmm8() {
    // VDIVPS XMM14, XMM15, XMM8
    let code = [
        0xc4, 0x41, 0x00, 0x5e, 0xf0, // VDIVPS XMM14, XMM15, XMM8
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vdivps_xmm15_xmm8_xmm9() {
    // VDIVPS XMM15, XMM8, XMM9
    let code = [
        0xc4, 0x41, 0x38, 0x5e, 0xf9, // VDIVPS XMM15, XMM8, XMM9
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// VDIVPS Tests - Cross-domain (mixing low and high XMM registers)
// ============================================================================

#[test]
fn test_vdivps_xmm0_xmm8_xmm15() {
    // VDIVPS XMM0, XMM8, XMM15
    let code = [
        0xc4, 0xc1, 0x38, 0x5e, 0xc7, // VDIVPS XMM0, XMM8, XMM15
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vdivps_xmm8_xmm0_xmm7() {
    // VDIVPS XMM8, XMM0, XMM7
    let code = [
        0xc4, 0xc1, 0x78, 0x5e, 0xc7, // VDIVPS XMM8, XMM0, XMM7
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vdivps_xmm7_xmm8_xmm0() {
    // VDIVPS XMM7, XMM8, XMM0
    let code = [
        0xc4, 0xc1, 0x38, 0x5e, 0xf8, // VDIVPS XMM7, XMM8, XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// VDIVPS Tests - 256-bit YMM registers (8x float32)
// ============================================================================

#[test]
fn test_vdivps_ymm0_ymm1_ymm2() {
    // VDIVPS YMM0, YMM1, YMM2
    let code = [
        0xc5, 0xf4, 0x5e, 0xc2, // VDIVPS YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vdivps_ymm1_ymm2_ymm3() {
    // VDIVPS YMM1, YMM2, YMM3
    let code = [
        0xc5, 0xec, 0x5e, 0xcb, // VDIVPS YMM1, YMM2, YMM3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vdivps_ymm2_ymm3_ymm4() {
    // VDIVPS YMM2, YMM3, YMM4
    let code = [
        0xc5, 0xe4, 0x5e, 0xd4, // VDIVPS YMM2, YMM3, YMM4
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vdivps_ymm3_ymm4_ymm5() {
    // VDIVPS YMM3, YMM4, YMM5
    let code = [
        0xc5, 0xdc, 0x5e, 0xdd, // VDIVPS YMM3, YMM4, YMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vdivps_ymm4_ymm5_ymm6() {
    // VDIVPS YMM4, YMM5, YMM6
    let code = [
        0xc5, 0xd4, 0x5e, 0xe6, // VDIVPS YMM4, YMM5, YMM6
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vdivps_ymm5_ymm6_ymm7() {
    // VDIVPS YMM5, YMM6, YMM7
    let code = [
        0xc5, 0xcc, 0x5e, 0xef, // VDIVPS YMM5, YMM6, YMM7
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vdivps_ymm6_ymm7_ymm0() {
    // VDIVPS YMM6, YMM7, YMM0
    let code = [
        0xc5, 0xc4, 0x5e, 0xf0, // VDIVPS YMM6, YMM7, YMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vdivps_ymm7_ymm0_ymm1() {
    // VDIVPS YMM7, YMM0, YMM1
    let code = [
        0xc5, 0xfc, 0x5e, 0xf9, // VDIVPS YMM7, YMM0, YMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// VDIVPS Tests - Extended YMM registers (YMM8-YMM15)
// ============================================================================

#[test]
fn test_vdivps_ymm8_ymm9_ymm10() {
    // VDIVPS YMM8, YMM9, YMM10
    let code = [
        0xc4, 0x41, 0x34, 0x5e, 0xc2, // VDIVPS YMM8, YMM9, YMM10
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vdivps_ymm9_ymm10_ymm11() {
    // VDIVPS YMM9, YMM10, YMM11
    let code = [
        0xc4, 0x41, 0x2c, 0x5e, 0xcb, // VDIVPS YMM9, YMM10, YMM11
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vdivps_ymm10_ymm11_ymm12() {
    // VDIVPS YMM10, YMM11, YMM12
    let code = [
        0xc4, 0x41, 0x24, 0x5e, 0xd4, // VDIVPS YMM10, YMM11, YMM12
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vdivps_ymm11_ymm12_ymm13() {
    // VDIVPS YMM11, YMM12, YMM13
    let code = [
        0xc4, 0x41, 0x1c, 0x5e, 0xdd, // VDIVPS YMM11, YMM12, YMM13
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vdivps_ymm12_ymm13_ymm14() {
    // VDIVPS YMM12, YMM13, YMM14
    let code = [
        0xc4, 0x41, 0x14, 0x5e, 0xe6, // VDIVPS YMM12, YMM13, YMM14
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vdivps_ymm13_ymm14_ymm15() {
    // VDIVPS YMM13, YMM14, YMM15
    let code = [
        0xc4, 0x41, 0x0c, 0x5e, 0xef, // VDIVPS YMM13, YMM14, YMM15
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vdivps_ymm14_ymm15_ymm8() {
    // VDIVPS YMM14, YMM15, YMM8
    let code = [
        0xc4, 0x41, 0x04, 0x5e, 0xf0, // VDIVPS YMM14, YMM15, YMM8
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vdivps_ymm15_ymm8_ymm9() {
    // VDIVPS YMM15, YMM8, YMM9
    let code = [
        0xc4, 0x41, 0x3c, 0x5e, 0xf9, // VDIVPS YMM15, YMM8, YMM9
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// VDIVPS Tests - Cross-domain YMM registers
// ============================================================================

#[test]
fn test_vdivps_ymm0_ymm8_ymm15() {
    // VDIVPS YMM0, YMM8, YMM15
    let code = [
        0xc4, 0xc1, 0x3c, 0x5e, 0xc7, // VDIVPS YMM0, YMM8, YMM15
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vdivps_ymm8_ymm0_ymm7() {
    // VDIVPS YMM8, YMM0, YMM7
    let code = [
        0xc4, 0xc1, 0x7c, 0x5e, 0xc7, // VDIVPS YMM8, YMM0, YMM7
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vdivps_ymm15_ymm0_ymm1() {
    // VDIVPS YMM15, YMM0, YMM1
    let code = [
        0xc4, 0xc1, 0x7c, 0x5e, 0xf9, // VDIVPS YMM15, YMM0, YMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// VDIVPS Tests - Memory operands (128-bit)
// ============================================================================

#[test]
fn test_vdivps_xmm0_xmm1_mem() {
    // VDIVPS XMM0, XMM1, [mem]
    let code = [
        0xc5, 0xf0, 0x5e, 0x05, 0x00, 0x40, 0x00, 0x00, // VDIVPS XMM0, XMM1, [rip + 0x4000]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    // Initialize memory with test data (4 single-precision floats)
    let test_data: [u8; 16] = [
        0x00, 0x00, 0x80, 0x3f, // 1.0
        0x00, 0x00, 0x00, 0x40, // 2.0
        0x00, 0x00, 0x40, 0x40, // 3.0
        0x00, 0x00, 0x80, 0x40, // 4.0
    ];
    mem.write_slice(&test_data, GuestAddress(ALIGNED_ADDR))
        .unwrap();

    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vdivps_xmm8_xmm9_mem() {
    // VDIVPS XMM8, XMM9, [mem]
    let code = [
        0xc4, 0x41, 0x30, 0x5e, 0x05, 0x00, 0x40, 0x00,
        0x00, // VDIVPS XMM8, XMM9, [rip + 0x4000]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    let test_data: [u8; 16] = [
        0x00, 0x00, 0xa0, 0x40, // 5.0
        0x00, 0x00, 0xc0, 0x40, // 6.0
        0x00, 0x00, 0xe0, 0x40, // 7.0
        0x00, 0x00, 0x00, 0x41, // 8.0
    ];
    mem.write_slice(&test_data, GuestAddress(ALIGNED_ADDR))
        .unwrap();

    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// VDIVPS Tests - Memory operands (256-bit)
// ============================================================================

#[test]
fn test_vdivps_ymm0_ymm1_mem() {
    // VDIVPS YMM0, YMM1, [mem]
    let code = [
        0xc5, 0xf4, 0x5e, 0x05, 0x00, 0x40, 0x00, 0x00, // VDIVPS YMM0, YMM1, [rip + 0x4000]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    // Initialize memory with test data (8 single-precision floats)
    let test_data: [u8; 32] = [
        0x00, 0x00, 0x80, 0x3f, // 1.0
        0x00, 0x00, 0x00, 0x40, // 2.0
        0x00, 0x00, 0x40, 0x40, // 3.0
        0x00, 0x00, 0x80, 0x40, // 4.0
        0x00, 0x00, 0xa0, 0x40, // 5.0
        0x00, 0x00, 0xc0, 0x40, // 6.0
        0x00, 0x00, 0xe0, 0x40, // 7.0
        0x00, 0x00, 0x00, 0x41, // 8.0
    ];
    mem.write_slice(&test_data, GuestAddress(ALIGNED_ADDR))
        .unwrap();

    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vdivps_ymm8_ymm9_mem() {
    // VDIVPS YMM8, YMM9, [mem]
    let code = [
        0xc4, 0x41, 0x34, 0x5e, 0x05, 0x00, 0x40, 0x00,
        0x00, // VDIVPS YMM8, YMM9, [rip + 0x4000]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    let test_data: [u8; 32] = [
        0x00, 0x00, 0x10, 0x41, // 9.0
        0x00, 0x00, 0x20, 0x41, // 10.0
        0x00, 0x00, 0x30, 0x41, // 11.0
        0x00, 0x00, 0x40, 0x41, // 12.0
        0x00, 0x00, 0x50, 0x41, // 13.0
        0x00, 0x00, 0x60, 0x41, // 14.0
        0x00, 0x00, 0x70, 0x41, // 15.0
        0x00, 0x00, 0x80, 0x41, // 16.0
    ];
    mem.write_slice(&test_data, GuestAddress(ALIGNED_ADDR))
        .unwrap();

    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// VDIVPD Tests - 128-bit XMM registers (2x float64)
// ============================================================================

#[test]
fn test_vdivpd_xmm0_xmm1_xmm2() {
    // VDIVPD XMM0, XMM1, XMM2
    let code = [
        0xc5, 0xf1, 0x5e, 0xc2, // VDIVPD XMM0, XMM1, XMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vdivpd_xmm1_xmm2_xmm3() {
    // VDIVPD XMM1, XMM2, XMM3
    let code = [
        0xc5, 0xe9, 0x5e, 0xcb, // VDIVPD XMM1, XMM2, XMM3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vdivpd_xmm2_xmm3_xmm4() {
    // VDIVPD XMM2, XMM3, XMM4
    let code = [
        0xc5, 0xe1, 0x5e, 0xd4, // VDIVPD XMM2, XMM3, XMM4
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vdivpd_xmm3_xmm4_xmm5() {
    // VDIVPD XMM3, XMM4, XMM5
    let code = [
        0xc5, 0xd9, 0x5e, 0xdd, // VDIVPD XMM3, XMM4, XMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vdivpd_xmm4_xmm5_xmm6() {
    // VDIVPD XMM4, XMM5, XMM6
    let code = [
        0xc5, 0xd1, 0x5e, 0xe6, // VDIVPD XMM4, XMM5, XMM6
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vdivpd_xmm5_xmm6_xmm7() {
    // VDIVPD XMM5, XMM6, XMM7
    let code = [
        0xc5, 0xc9, 0x5e, 0xef, // VDIVPD XMM5, XMM6, XMM7
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vdivpd_xmm6_xmm7_xmm0() {
    // VDIVPD XMM6, XMM7, XMM0
    let code = [
        0xc5, 0xc1, 0x5e, 0xf0, // VDIVPD XMM6, XMM7, XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vdivpd_xmm7_xmm0_xmm1() {
    // VDIVPD XMM7, XMM0, XMM1
    let code = [
        0xc5, 0xf9, 0x5e, 0xf9, // VDIVPD XMM7, XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// VDIVPD Tests - Extended XMM registers (XMM8-XMM15)
// ============================================================================

#[test]
fn test_vdivpd_xmm8_xmm9_xmm10() {
    // VDIVPD XMM8, XMM9, XMM10
    let code = [
        0xc4, 0x41, 0x31, 0x5e, 0xc2, // VDIVPD XMM8, XMM9, XMM10
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vdivpd_xmm9_xmm10_xmm11() {
    // VDIVPD XMM9, XMM10, XMM11
    let code = [
        0xc4, 0x41, 0x29, 0x5e, 0xcb, // VDIVPD XMM9, XMM10, XMM11
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vdivpd_xmm10_xmm11_xmm12() {
    // VDIVPD XMM10, XMM11, XMM12
    let code = [
        0xc4, 0x41, 0x21, 0x5e, 0xd4, // VDIVPD XMM10, XMM11, XMM12
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vdivpd_xmm11_xmm12_xmm13() {
    // VDIVPD XMM11, XMM12, XMM13
    let code = [
        0xc4, 0x41, 0x19, 0x5e, 0xdd, // VDIVPD XMM11, XMM12, XMM13
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vdivpd_xmm12_xmm13_xmm14() {
    // VDIVPD XMM12, XMM13, XMM14
    let code = [
        0xc4, 0x41, 0x11, 0x5e, 0xe6, // VDIVPD XMM12, XMM13, XMM14
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vdivpd_xmm13_xmm14_xmm15() {
    // VDIVPD XMM13, XMM14, XMM15
    let code = [
        0xc4, 0x41, 0x09, 0x5e, 0xef, // VDIVPD XMM13, XMM14, XMM15
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vdivpd_xmm14_xmm15_xmm8() {
    // VDIVPD XMM14, XMM15, XMM8
    let code = [
        0xc4, 0x41, 0x01, 0x5e, 0xf0, // VDIVPD XMM14, XMM15, XMM8
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vdivpd_xmm15_xmm8_xmm9() {
    // VDIVPD XMM15, XMM8, XMM9
    let code = [
        0xc4, 0x41, 0x39, 0x5e, 0xf9, // VDIVPD XMM15, XMM8, XMM9
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// VDIVPD Tests - 256-bit YMM registers (4x float64)
// ============================================================================

#[test]
fn test_vdivpd_ymm0_ymm1_ymm2() {
    // VDIVPD YMM0, YMM1, YMM2
    let code = [
        0xc5, 0xf5, 0x5e, 0xc2, // VDIVPD YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vdivpd_ymm1_ymm2_ymm3() {
    // VDIVPD YMM1, YMM2, YMM3
    let code = [
        0xc5, 0xed, 0x5e, 0xcb, // VDIVPD YMM1, YMM2, YMM3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vdivpd_ymm2_ymm3_ymm4() {
    // VDIVPD YMM2, YMM3, YMM4
    let code = [
        0xc5, 0xe5, 0x5e, 0xd4, // VDIVPD YMM2, YMM3, YMM4
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vdivpd_ymm3_ymm4_ymm5() {
    // VDIVPD YMM3, YMM4, YMM5
    let code = [
        0xc5, 0xdd, 0x5e, 0xdd, // VDIVPD YMM3, YMM4, YMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vdivpd_ymm4_ymm5_ymm6() {
    // VDIVPD YMM4, YMM5, YMM6
    let code = [
        0xc5, 0xd5, 0x5e, 0xe6, // VDIVPD YMM4, YMM5, YMM6
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vdivpd_ymm5_ymm6_ymm7() {
    // VDIVPD YMM5, YMM6, YMM7
    let code = [
        0xc5, 0xcd, 0x5e, 0xef, // VDIVPD YMM5, YMM6, YMM7
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vdivpd_ymm6_ymm7_ymm0() {
    // VDIVPD YMM6, YMM7, YMM0
    let code = [
        0xc5, 0xc5, 0x5e, 0xf0, // VDIVPD YMM6, YMM7, YMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vdivpd_ymm7_ymm0_ymm1() {
    // VDIVPD YMM7, YMM0, YMM1
    let code = [
        0xc5, 0xfd, 0x5e, 0xf9, // VDIVPD YMM7, YMM0, YMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// VDIVPD Tests - Extended YMM registers (YMM8-YMM15)
// ============================================================================

#[test]
fn test_vdivpd_ymm8_ymm9_ymm10() {
    // VDIVPD YMM8, YMM9, YMM10
    let code = [
        0xc4, 0x41, 0x35, 0x5e, 0xc2, // VDIVPD YMM8, YMM9, YMM10
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vdivpd_ymm9_ymm10_ymm11() {
    // VDIVPD YMM9, YMM10, YMM11
    let code = [
        0xc4, 0x41, 0x2d, 0x5e, 0xcb, // VDIVPD YMM9, YMM10, YMM11
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vdivpd_ymm10_ymm11_ymm12() {
    // VDIVPD YMM10, YMM11, YMM12
    let code = [
        0xc4, 0x41, 0x25, 0x5e, 0xd4, // VDIVPD YMM10, YMM11, YMM12
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vdivpd_ymm11_ymm12_ymm13() {
    // VDIVPD YMM11, YMM12, YMM13
    let code = [
        0xc4, 0x41, 0x1d, 0x5e, 0xdd, // VDIVPD YMM11, YMM12, YMM13
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vdivpd_ymm12_ymm13_ymm14() {
    // VDIVPD YMM12, YMM13, YMM14
    let code = [
        0xc4, 0x41, 0x15, 0x5e, 0xe6, // VDIVPD YMM12, YMM13, YMM14
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vdivpd_ymm13_ymm14_ymm15() {
    // VDIVPD YMM13, YMM14, YMM15
    let code = [
        0xc4, 0x41, 0x0d, 0x5e, 0xef, // VDIVPD YMM13, YMM14, YMM15
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vdivpd_ymm14_ymm15_ymm8() {
    // VDIVPD YMM14, YMM15, YMM8
    let code = [
        0xc4, 0x41, 0x05, 0x5e, 0xf0, // VDIVPD YMM14, YMM15, YMM8
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vdivpd_ymm15_ymm8_ymm9() {
    // VDIVPD YMM15, YMM8, YMM9
    let code = [
        0xc4, 0x41, 0x3d, 0x5e, 0xf9, // VDIVPD YMM15, YMM8, YMM9
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// VDIVPD Tests - Memory operands (128-bit)
// ============================================================================

#[test]
fn test_vdivpd_xmm0_xmm1_mem() {
    // VDIVPD XMM0, XMM1, [mem]
    let code = [
        0xc5, 0xf1, 0x5e, 0x05, 0x00, 0x40, 0x00, 0x00, // VDIVPD XMM0, XMM1, [rip + 0x4000]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    // Initialize memory with test data (2 double-precision floats)
    let test_data: [u8; 16] = [
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xf0, 0x3f, // 1.0
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x40, // 2.0
    ];
    mem.write_slice(&test_data, GuestAddress(ALIGNED_ADDR))
        .unwrap();

    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vdivpd_xmm8_xmm9_mem() {
    // VDIVPD XMM8, XMM9, [mem]
    let code = [
        0xc4, 0x41, 0x31, 0x5e, 0x05, 0x00, 0x40, 0x00,
        0x00, // VDIVPD XMM8, XMM9, [rip + 0x4000]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    let test_data: [u8; 16] = [
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x08, 0x40, // 3.0
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x10, 0x40, // 4.0
    ];
    mem.write_slice(&test_data, GuestAddress(ALIGNED_ADDR))
        .unwrap();

    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// VDIVPD Tests - Memory operands (256-bit)
// ============================================================================

#[test]
fn test_vdivpd_ymm0_ymm1_mem() {
    // VDIVPD YMM0, YMM1, [mem]
    let code = [
        0xc5, 0xf5, 0x5e, 0x05, 0x00, 0x40, 0x00, 0x00, // VDIVPD YMM0, YMM1, [rip + 0x4000]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    // Initialize memory with test data (4 double-precision floats)
    let test_data: [u8; 32] = [
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xf0, 0x3f, // 1.0
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x40, // 2.0
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x08, 0x40, // 3.0
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x10, 0x40, // 4.0
    ];
    mem.write_slice(&test_data, GuestAddress(ALIGNED_ADDR))
        .unwrap();

    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vdivpd_ymm8_ymm9_mem() {
    // VDIVPD YMM8, YMM9, [mem]
    let code = [
        0xc4, 0x41, 0x35, 0x5e, 0x05, 0x00, 0x40, 0x00,
        0x00, // VDIVPD YMM8, YMM9, [rip + 0x4000]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    let test_data: [u8; 32] = [
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x14, 0x40, // 5.0
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x18, 0x40, // 6.0
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x1c, 0x40, // 7.0
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x20, 0x40, // 8.0
    ];
    mem.write_slice(&test_data, GuestAddress(ALIGNED_ADDR))
        .unwrap();

    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Known-answer VALUE tests : packed float DIV (src1 / src2) using ratios of
// powers of two, so the quotient is exactly representable.
// ============================================================================

use rax::backend::emulator::x86_64::X86_64Vcpu;

fn kfd_set(vcpu: &mut X86_64Vcpu, idx: usize, lo: u128, hi: u128) {
    let mut regs = vcpu.get_regs().unwrap();
    regs.xmm[idx][0] = lo as u64;
    regs.xmm[idx][1] = (lo >> 64) as u64;
    regs.ymm_high[idx][0] = hi as u64;
    regs.ymm_high[idx][1] = (hi >> 64) as u64;
    vcpu.set_regs(&regs).unwrap();
}
fn kfd_lo(vcpu: &X86_64Vcpu, idx: usize) -> u128 {
    let r = vcpu.get_regs().unwrap();
    (r.xmm[idx][0] as u128) | ((r.xmm[idx][1] as u128) << 64)
}
fn kfd_hi(vcpu: &X86_64Vcpu, idx: usize) -> u128 {
    let r = vcpu.get_regs().unwrap();
    (r.ymm_high[idx][0] as u128) | ((r.ymm_high[idx][1] as u128) << 64)
}

fn pack_ps_d(v: [f32; 4]) -> u128 {
    let mut out = 0u128;
    for i in 0..4 {
        out |= (v[i].to_bits() as u128) << (i * 32);
    }
    out
}
fn pack_pd_d(v: [f64; 2]) -> u128 {
    (v[0].to_bits() as u128) | ((v[1].to_bits() as u128) << 64)
}

#[test]
fn test_vdivps_xmm_value() {
    let code = [0xc5, 0xf0, 0x5e, 0xc2, 0xf4]; // VDIVPS XMM0, XMM1, XMM2
    let (mut vcpu, _) = setup_vm(&code, None);
    kfd_set(&mut vcpu, 1, pack_ps_d([8.0, 1.0, 3.0, 16.0]), 0xDEAD);
    kfd_set(&mut vcpu, 2, pack_ps_d([2.0, 4.0, 0.5, -8.0]), 0xBEEF);
    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(kfd_lo(&vcpu, 0), pack_ps_d([4.0, 0.25, 6.0, -2.0]));
    assert_eq!(kfd_hi(&vcpu, 0), 0, "VEX.128 must zero upper 128 bits");
}

#[test]
fn test_vdivps_ymm_value() {
    let code = [0xc5, 0xf4, 0x5e, 0xc2, 0xf4]; // VDIVPS YMM0, YMM1, YMM2
    let (mut vcpu, _) = setup_vm(&code, None);
    kfd_set(
        &mut vcpu,
        1,
        pack_ps_d([8.0, 1.0, 3.0, 16.0]),
        pack_ps_d([64.0, 0.5, 9.0, 1.0]),
    );
    kfd_set(
        &mut vcpu,
        2,
        pack_ps_d([2.0, 4.0, 0.5, -8.0]),
        pack_ps_d([4.0, 2.0, 4.0, 1.0]),
    );
    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(kfd_lo(&vcpu, 0), pack_ps_d([4.0, 0.25, 6.0, -2.0]));
    assert_eq!(kfd_hi(&vcpu, 0), pack_ps_d([16.0, 0.25, 2.25, 1.0]));
}

#[test]
fn test_vdivpd_xmm_value() {
    let code = [0xc5, 0xf1, 0x5e, 0xc2, 0xf4]; // VDIVPD XMM0, XMM1, XMM2
    let (mut vcpu, _) = setup_vm(&code, None);
    kfd_set(&mut vcpu, 1, pack_pd_d([8.0, 3.0]), 0xDEAD);
    kfd_set(&mut vcpu, 2, pack_pd_d([2.0, 0.5]), 0xBEEF);
    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(kfd_lo(&vcpu, 0), pack_pd_d([4.0, 6.0]));
    assert_eq!(kfd_hi(&vcpu, 0), 0, "VEX.128 must zero upper 128 bits");
}

#[test]
fn test_vdivpd_ymm_value() {
    let code = [0xc5, 0xf5, 0x5e, 0xc2, 0xf4]; // VDIVPD YMM0, YMM1, YMM2
    let (mut vcpu, _) = setup_vm(&code, None);
    kfd_set(&mut vcpu, 1, pack_pd_d([8.0, 3.0]), pack_pd_d([1.0, 64.0]));
    kfd_set(&mut vcpu, 2, pack_pd_d([2.0, 0.5]), pack_pd_d([4.0, -8.0]));
    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(kfd_lo(&vcpu, 0), pack_pd_d([4.0, 6.0]));
    assert_eq!(kfd_hi(&vcpu, 0), pack_pd_d([0.25, -8.0]));
}
