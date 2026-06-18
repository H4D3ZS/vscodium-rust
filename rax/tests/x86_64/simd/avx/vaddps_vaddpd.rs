use crate::common::*;
use rax::cpu::Registers;
use vm_memory::{Bytes, GuestAddress};

// VADDPS - Add Packed Single-Precision Floating-Point Values
// VADDPD - Add Packed Double-Precision Floating-Point Values
//
// VADDPS performs element-wise addition of packed single-precision floating-point values.
// VADDPD performs element-wise addition of packed double-precision floating-point values.
//
// Opcodes:
// VEX.128.0F.WIG 58 /r    VADDPS xmm1, xmm2, xmm3/m128   - Add packed single from xmm3/mem to xmm2
// VEX.256.0F.WIG 58 /r    VADDPS ymm1, ymm2, ymm3/m256   - Add packed single from ymm3/mem to ymm2
// VEX.128.66.0F.WIG 58 /r VADDPD xmm1, xmm2, xmm3/m128   - Add packed double from xmm3/mem to xmm2
// VEX.256.66.0F.WIG 58 /r VADDPD ymm1, ymm2, ymm3/m256   - Add packed double from ymm3/mem to ymm2

const ALIGNED_ADDR: u64 = 0x3000; // 32-byte aligned address for testing

// ============================================================================
// VADDPS Tests - 128-bit XMM registers (4x float32)
// ============================================================================

#[test]
fn test_vaddps_xmm0_xmm1_xmm2() {
    // VADDPS XMM0, XMM1, XMM2
    let code = [
        0xc5, 0xf0, 0x58, 0xc2, // VADDPS XMM0, XMM1, XMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vaddps_xmm1_xmm2_xmm3() {
    // VADDPS XMM1, XMM2, XMM3
    let code = [
        0xc5, 0xe8, 0x58, 0xcb, // VADDPS XMM1, XMM2, XMM3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vaddps_xmm2_xmm3_xmm4() {
    // VADDPS XMM2, XMM3, XMM4
    let code = [
        0xc5, 0xe0, 0x58, 0xd4, // VADDPS XMM2, XMM3, XMM4
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vaddps_xmm3_xmm4_xmm5() {
    // VADDPS XMM3, XMM4, XMM5
    let code = [
        0xc5, 0xd8, 0x58, 0xdd, // VADDPS XMM3, XMM4, XMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vaddps_xmm4_xmm5_xmm6() {
    // VADDPS XMM4, XMM5, XMM6
    let code = [
        0xc5, 0xd0, 0x58, 0xe6, // VADDPS XMM4, XMM5, XMM6
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vaddps_xmm5_xmm6_xmm7() {
    // VADDPS XMM5, XMM6, XMM7
    let code = [
        0xc5, 0xc8, 0x58, 0xef, // VADDPS XMM5, XMM6, XMM7
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vaddps_xmm6_xmm7_xmm0() {
    // VADDPS XMM6, XMM7, XMM0
    let code = [
        0xc5, 0xc0, 0x58, 0xf0, // VADDPS XMM6, XMM7, XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vaddps_xmm7_xmm0_xmm1() {
    // VADDPS XMM7, XMM0, XMM1
    let code = [
        0xc5, 0xf8, 0x58, 0xf9, // VADDPS XMM7, XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// VADDPS Tests - Extended XMM registers (XMM8-XMM15)
// ============================================================================

#[test]
fn test_vaddps_xmm8_xmm9_xmm10() {
    // VADDPS XMM8, XMM9, XMM10
    let code = [
        0xc4, 0x41, 0x30, 0x58, 0xc2, // VADDPS XMM8, XMM9, XMM10
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vaddps_xmm9_xmm10_xmm11() {
    // VADDPS XMM9, XMM10, XMM11
    let code = [
        0xc4, 0x41, 0x28, 0x58, 0xcb, // VADDPS XMM9, XMM10, XMM11
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vaddps_xmm10_xmm11_xmm12() {
    // VADDPS XMM10, XMM11, XMM12
    let code = [
        0xc4, 0x41, 0x20, 0x58, 0xd4, // VADDPS XMM10, XMM11, XMM12
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vaddps_xmm11_xmm12_xmm13() {
    // VADDPS XMM11, XMM12, XMM13
    let code = [
        0xc4, 0x41, 0x18, 0x58, 0xdd, // VADDPS XMM11, XMM12, XMM13
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vaddps_xmm12_xmm13_xmm14() {
    // VADDPS XMM12, XMM13, XMM14
    let code = [
        0xc4, 0x41, 0x10, 0x58, 0xe6, // VADDPS XMM12, XMM13, XMM14
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vaddps_xmm13_xmm14_xmm15() {
    // VADDPS XMM13, XMM14, XMM15
    let code = [
        0xc4, 0x41, 0x08, 0x58, 0xef, // VADDPS XMM13, XMM14, XMM15
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vaddps_xmm14_xmm15_xmm8() {
    // VADDPS XMM14, XMM15, XMM8
    let code = [
        0xc4, 0x41, 0x00, 0x58, 0xf0, // VADDPS XMM14, XMM15, XMM8
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vaddps_xmm15_xmm8_xmm9() {
    // VADDPS XMM15, XMM8, XMM9
    let code = [
        0xc4, 0x41, 0x38, 0x58, 0xf9, // VADDPS XMM15, XMM8, XMM9
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// VADDPS Tests - Cross-domain (mixing low and high XMM registers)
// ============================================================================

#[test]
fn test_vaddps_xmm0_xmm8_xmm15() {
    // VADDPS XMM0, XMM8, XMM15
    let code = [
        0xc4, 0xc1, 0x38, 0x58, 0xc7, // VADDPS XMM0, XMM8, XMM15
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vaddps_xmm8_xmm0_xmm7() {
    // VADDPS XMM8, XMM0, XMM7
    let code = [
        0xc4, 0xc1, 0x78, 0x58, 0xc7, // VADDPS XMM8, XMM0, XMM7
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vaddps_xmm7_xmm8_xmm0() {
    // VADDPS XMM7, XMM8, XMM0
    let code = [
        0xc4, 0xc1, 0x38, 0x58, 0xf8, // VADDPS XMM7, XMM8, XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// VADDPS Tests - 256-bit YMM registers (8x float32)
// ============================================================================

#[test]
fn test_vaddps_ymm0_ymm1_ymm2() {
    // VADDPS YMM0, YMM1, YMM2
    let code = [
        0xc5, 0xf4, 0x58, 0xc2, // VADDPS YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vaddps_ymm1_ymm2_ymm3() {
    // VADDPS YMM1, YMM2, YMM3
    let code = [
        0xc5, 0xec, 0x58, 0xcb, // VADDPS YMM1, YMM2, YMM3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vaddps_ymm2_ymm3_ymm4() {
    // VADDPS YMM2, YMM3, YMM4
    let code = [
        0xc5, 0xe4, 0x58, 0xd4, // VADDPS YMM2, YMM3, YMM4
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vaddps_ymm3_ymm4_ymm5() {
    // VADDPS YMM3, YMM4, YMM5
    let code = [
        0xc5, 0xdc, 0x58, 0xdd, // VADDPS YMM3, YMM4, YMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vaddps_ymm4_ymm5_ymm6() {
    // VADDPS YMM4, YMM5, YMM6
    let code = [
        0xc5, 0xd4, 0x58, 0xe6, // VADDPS YMM4, YMM5, YMM6
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vaddps_ymm5_ymm6_ymm7() {
    // VADDPS YMM5, YMM6, YMM7
    let code = [
        0xc5, 0xcc, 0x58, 0xef, // VADDPS YMM5, YMM6, YMM7
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vaddps_ymm6_ymm7_ymm0() {
    // VADDPS YMM6, YMM7, YMM0
    let code = [
        0xc5, 0xc4, 0x58, 0xf0, // VADDPS YMM6, YMM7, YMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vaddps_ymm7_ymm0_ymm1() {
    // VADDPS YMM7, YMM0, YMM1
    let code = [
        0xc5, 0xfc, 0x58, 0xf9, // VADDPS YMM7, YMM0, YMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// VADDPS Tests - Extended YMM registers (YMM8-YMM15)
// ============================================================================

#[test]
fn test_vaddps_ymm8_ymm9_ymm10() {
    // VADDPS YMM8, YMM9, YMM10
    let code = [
        0xc4, 0x41, 0x34, 0x58, 0xc2, // VADDPS YMM8, YMM9, YMM10
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vaddps_ymm9_ymm10_ymm11() {
    // VADDPS YMM9, YMM10, YMM11
    let code = [
        0xc4, 0x41, 0x2c, 0x58, 0xcb, // VADDPS YMM9, YMM10, YMM11
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vaddps_ymm10_ymm11_ymm12() {
    // VADDPS YMM10, YMM11, YMM12
    let code = [
        0xc4, 0x41, 0x24, 0x58, 0xd4, // VADDPS YMM10, YMM11, YMM12
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vaddps_ymm11_ymm12_ymm13() {
    // VADDPS YMM11, YMM12, YMM13
    let code = [
        0xc4, 0x41, 0x1c, 0x58, 0xdd, // VADDPS YMM11, YMM12, YMM13
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vaddps_ymm12_ymm13_ymm14() {
    // VADDPS YMM12, YMM13, YMM14
    let code = [
        0xc4, 0x41, 0x14, 0x58, 0xe6, // VADDPS YMM12, YMM13, YMM14
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vaddps_ymm13_ymm14_ymm15() {
    // VADDPS YMM13, YMM14, YMM15
    let code = [
        0xc4, 0x41, 0x0c, 0x58, 0xef, // VADDPS YMM13, YMM14, YMM15
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vaddps_ymm14_ymm15_ymm8() {
    // VADDPS YMM14, YMM15, YMM8
    let code = [
        0xc4, 0x41, 0x04, 0x58, 0xf0, // VADDPS YMM14, YMM15, YMM8
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vaddps_ymm15_ymm8_ymm9() {
    // VADDPS YMM15, YMM8, YMM9
    let code = [
        0xc4, 0x41, 0x3c, 0x58, 0xf9, // VADDPS YMM15, YMM8, YMM9
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// VADDPS Tests - Cross-domain YMM registers
// ============================================================================

#[test]
fn test_vaddps_ymm0_ymm8_ymm15() {
    // VADDPS YMM0, YMM8, YMM15
    let code = [
        0xc4, 0xc1, 0x3c, 0x58, 0xc7, // VADDPS YMM0, YMM8, YMM15
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vaddps_ymm8_ymm0_ymm7() {
    // VADDPS YMM8, YMM0, YMM7
    let code = [
        0xc4, 0xc1, 0x7c, 0x58, 0xc7, // VADDPS YMM8, YMM0, YMM7
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vaddps_ymm15_ymm0_ymm1() {
    // VADDPS YMM15, YMM0, YMM1
    let code = [
        0xc4, 0xc1, 0x7c, 0x58, 0xf9, // VADDPS YMM15, YMM0, YMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// VADDPS Tests - Memory operands (128-bit)
// ============================================================================

#[test]
fn test_vaddps_xmm0_xmm1_mem() {
    // VADDPS XMM0, XMM1, [mem]
    let code = [
        0xc5, 0xf0, 0x58, 0x05, 0x00, 0x40, 0x00, 0x00, // VADDPS XMM0, XMM1, [rip + 0x4000]
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
fn test_vaddps_xmm8_xmm9_mem() {
    // VADDPS XMM8, XMM9, [mem]
    let code = [
        0xc4, 0x41, 0x30, 0x58, 0x05, 0x00, 0x40, 0x00,
        0x00, // VADDPS XMM8, XMM9, [rip + 0x4000]
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
// VADDPS Tests - Memory operands (256-bit)
// ============================================================================

#[test]
fn test_vaddps_ymm0_ymm1_mem() {
    // VADDPS YMM0, YMM1, [mem]
    let code = [
        0xc5, 0xf4, 0x58, 0x05, 0x00, 0x40, 0x00, 0x00, // VADDPS YMM0, YMM1, [rip + 0x4000]
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
fn test_vaddps_ymm8_ymm9_mem() {
    // VADDPS YMM8, YMM9, [mem]
    let code = [
        0xc4, 0x41, 0x34, 0x58, 0x05, 0x00, 0x40, 0x00,
        0x00, // VADDPS YMM8, YMM9, [rip + 0x4000]
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
// VADDPD Tests - 128-bit XMM registers (2x float64)
// ============================================================================

#[test]
fn test_vaddpd_xmm0_xmm1_xmm2() {
    // VADDPD XMM0, XMM1, XMM2
    let code = [
        0xc5, 0xf1, 0x58, 0xc2, // VADDPD XMM0, XMM1, XMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vaddpd_xmm1_xmm2_xmm3() {
    // VADDPD XMM1, XMM2, XMM3
    let code = [
        0xc5, 0xe9, 0x58, 0xcb, // VADDPD XMM1, XMM2, XMM3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vaddpd_xmm2_xmm3_xmm4() {
    // VADDPD XMM2, XMM3, XMM4
    let code = [
        0xc5, 0xe1, 0x58, 0xd4, // VADDPD XMM2, XMM3, XMM4
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vaddpd_xmm3_xmm4_xmm5() {
    // VADDPD XMM3, XMM4, XMM5
    let code = [
        0xc5, 0xd9, 0x58, 0xdd, // VADDPD XMM3, XMM4, XMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vaddpd_xmm4_xmm5_xmm6() {
    // VADDPD XMM4, XMM5, XMM6
    let code = [
        0xc5, 0xd1, 0x58, 0xe6, // VADDPD XMM4, XMM5, XMM6
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vaddpd_xmm5_xmm6_xmm7() {
    // VADDPD XMM5, XMM6, XMM7
    let code = [
        0xc5, 0xc9, 0x58, 0xef, // VADDPD XMM5, XMM6, XMM7
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vaddpd_xmm6_xmm7_xmm0() {
    // VADDPD XMM6, XMM7, XMM0
    let code = [
        0xc5, 0xc1, 0x58, 0xf0, // VADDPD XMM6, XMM7, XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vaddpd_xmm7_xmm0_xmm1() {
    // VADDPD XMM7, XMM0, XMM1
    let code = [
        0xc5, 0xf9, 0x58, 0xf9, // VADDPD XMM7, XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// VADDPD Tests - Extended XMM registers (XMM8-XMM15)
// ============================================================================

#[test]
fn test_vaddpd_xmm8_xmm9_xmm10() {
    // VADDPD XMM8, XMM9, XMM10
    let code = [
        0xc4, 0x41, 0x31, 0x58, 0xc2, // VADDPD XMM8, XMM9, XMM10
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vaddpd_xmm9_xmm10_xmm11() {
    // VADDPD XMM9, XMM10, XMM11
    let code = [
        0xc4, 0x41, 0x29, 0x58, 0xcb, // VADDPD XMM9, XMM10, XMM11
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vaddpd_xmm10_xmm11_xmm12() {
    // VADDPD XMM10, XMM11, XMM12
    let code = [
        0xc4, 0x41, 0x21, 0x58, 0xd4, // VADDPD XMM10, XMM11, XMM12
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vaddpd_xmm11_xmm12_xmm13() {
    // VADDPD XMM11, XMM12, XMM13
    let code = [
        0xc4, 0x41, 0x19, 0x58, 0xdd, // VADDPD XMM11, XMM12, XMM13
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vaddpd_xmm12_xmm13_xmm14() {
    // VADDPD XMM12, XMM13, XMM14
    let code = [
        0xc4, 0x41, 0x11, 0x58, 0xe6, // VADDPD XMM12, XMM13, XMM14
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vaddpd_xmm13_xmm14_xmm15() {
    // VADDPD XMM13, XMM14, XMM15
    let code = [
        0xc4, 0x41, 0x09, 0x58, 0xef, // VADDPD XMM13, XMM14, XMM15
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vaddpd_xmm14_xmm15_xmm8() {
    // VADDPD XMM14, XMM15, XMM8
    let code = [
        0xc4, 0x41, 0x01, 0x58, 0xf0, // VADDPD XMM14, XMM15, XMM8
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vaddpd_xmm15_xmm8_xmm9() {
    // VADDPD XMM15, XMM8, XMM9
    let code = [
        0xc4, 0x41, 0x39, 0x58, 0xf9, // VADDPD XMM15, XMM8, XMM9
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// VADDPD Tests - 256-bit YMM registers (4x float64)
// ============================================================================

#[test]
fn test_vaddpd_ymm0_ymm1_ymm2() {
    // VADDPD YMM0, YMM1, YMM2
    let code = [
        0xc5, 0xf5, 0x58, 0xc2, // VADDPD YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vaddpd_ymm1_ymm2_ymm3() {
    // VADDPD YMM1, YMM2, YMM3
    let code = [
        0xc5, 0xed, 0x58, 0xcb, // VADDPD YMM1, YMM2, YMM3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vaddpd_ymm2_ymm3_ymm4() {
    // VADDPD YMM2, YMM3, YMM4
    let code = [
        0xc5, 0xe5, 0x58, 0xd4, // VADDPD YMM2, YMM3, YMM4
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vaddpd_ymm3_ymm4_ymm5() {
    // VADDPD YMM3, YMM4, YMM5
    let code = [
        0xc5, 0xdd, 0x58, 0xdd, // VADDPD YMM3, YMM4, YMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vaddpd_ymm4_ymm5_ymm6() {
    // VADDPD YMM4, YMM5, YMM6
    let code = [
        0xc5, 0xd5, 0x58, 0xe6, // VADDPD YMM4, YMM5, YMM6
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vaddpd_ymm5_ymm6_ymm7() {
    // VADDPD YMM5, YMM6, YMM7
    let code = [
        0xc5, 0xcd, 0x58, 0xef, // VADDPD YMM5, YMM6, YMM7
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vaddpd_ymm6_ymm7_ymm0() {
    // VADDPD YMM6, YMM7, YMM0
    let code = [
        0xc5, 0xc5, 0x58, 0xf0, // VADDPD YMM6, YMM7, YMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vaddpd_ymm7_ymm0_ymm1() {
    // VADDPD YMM7, YMM0, YMM1
    let code = [
        0xc5, 0xfd, 0x58, 0xf9, // VADDPD YMM7, YMM0, YMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// VADDPD Tests - Extended YMM registers (YMM8-YMM15)
// ============================================================================

#[test]
fn test_vaddpd_ymm8_ymm9_ymm10() {
    // VADDPD YMM8, YMM9, YMM10
    let code = [
        0xc4, 0x41, 0x35, 0x58, 0xc2, // VADDPD YMM8, YMM9, YMM10
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vaddpd_ymm9_ymm10_ymm11() {
    // VADDPD YMM9, YMM10, YMM11
    let code = [
        0xc4, 0x41, 0x2d, 0x58, 0xcb, // VADDPD YMM9, YMM10, YMM11
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vaddpd_ymm10_ymm11_ymm12() {
    // VADDPD YMM10, YMM11, YMM12
    let code = [
        0xc4, 0x41, 0x25, 0x58, 0xd4, // VADDPD YMM10, YMM11, YMM12
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vaddpd_ymm11_ymm12_ymm13() {
    // VADDPD YMM11, YMM12, YMM13
    let code = [
        0xc4, 0x41, 0x1d, 0x58, 0xdd, // VADDPD YMM11, YMM12, YMM13
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vaddpd_ymm12_ymm13_ymm14() {
    // VADDPD YMM12, YMM13, YMM14
    let code = [
        0xc4, 0x41, 0x15, 0x58, 0xe6, // VADDPD YMM12, YMM13, YMM14
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vaddpd_ymm13_ymm14_ymm15() {
    // VADDPD YMM13, YMM14, YMM15
    let code = [
        0xc4, 0x41, 0x0d, 0x58, 0xef, // VADDPD YMM13, YMM14, YMM15
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vaddpd_ymm14_ymm15_ymm8() {
    // VADDPD YMM14, YMM15, YMM8
    let code = [
        0xc4, 0x41, 0x05, 0x58, 0xf0, // VADDPD YMM14, YMM15, YMM8
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vaddpd_ymm15_ymm8_ymm9() {
    // VADDPD YMM15, YMM8, YMM9
    let code = [
        0xc4, 0x41, 0x3d, 0x58, 0xf9, // VADDPD YMM15, YMM8, YMM9
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// VADDPD Tests - Memory operands (128-bit)
// ============================================================================

#[test]
fn test_vaddpd_xmm0_xmm1_mem() {
    // VADDPD XMM0, XMM1, [mem]
    let code = [
        0xc5, 0xf1, 0x58, 0x05, 0x00, 0x40, 0x00, 0x00, // VADDPD XMM0, XMM1, [rip + 0x4000]
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
fn test_vaddpd_xmm8_xmm9_mem() {
    // VADDPD XMM8, XMM9, [mem]
    let code = [
        0xc4, 0x41, 0x31, 0x58, 0x05, 0x00, 0x40, 0x00,
        0x00, // VADDPD XMM8, XMM9, [rip + 0x4000]
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
// VADDPD Tests - Memory operands (256-bit)
// ============================================================================

#[test]
fn test_vaddpd_ymm0_ymm1_mem() {
    // VADDPD YMM0, YMM1, [mem]
    let code = [
        0xc5, 0xf5, 0x58, 0x05, 0x00, 0x40, 0x00, 0x00, // VADDPD YMM0, YMM1, [rip + 0x4000]
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
fn test_vaddpd_ymm8_ymm9_mem() {
    // VADDPD YMM8, YMM9, [mem]
    let code = [
        0xc4, 0x41, 0x35, 0x58, 0x05, 0x00, 0x40, 0x00,
        0x00, // VADDPD YMM8, YMM9, [rip + 0x4000]
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
// Known-answer VALUE tests : packed float ADD using exactly representable
// operands (powers of two), so results are bit-exact with no rounding.
// ============================================================================

use rax::backend::emulator::x86_64::X86_64Vcpu;

fn kfa_set(vcpu: &mut X86_64Vcpu, idx: usize, lo: u128, hi: u128) {
    let mut regs = vcpu.get_regs().unwrap();
    regs.xmm[idx][0] = lo as u64;
    regs.xmm[idx][1] = (lo >> 64) as u64;
    regs.ymm_high[idx][0] = hi as u64;
    regs.ymm_high[idx][1] = (hi >> 64) as u64;
    vcpu.set_regs(&regs).unwrap();
}
fn kfa_lo(vcpu: &X86_64Vcpu, idx: usize) -> u128 {
    let r = vcpu.get_regs().unwrap();
    (r.xmm[idx][0] as u128) | ((r.xmm[idx][1] as u128) << 64)
}
fn kfa_hi(vcpu: &X86_64Vcpu, idx: usize) -> u128 {
    let r = vcpu.get_regs().unwrap();
    (r.ymm_high[idx][0] as u128) | ((r.ymm_high[idx][1] as u128) << 64)
}

fn pack_ps(v: [f32; 4]) -> u128 {
    let mut out = 0u128;
    for i in 0..4 {
        out |= (v[i].to_bits() as u128) << (i * 32);
    }
    out
}
fn pack_pd(v: [f64; 2]) -> u128 {
    (v[0].to_bits() as u128) | ((v[1].to_bits() as u128) << 64)
}

#[test]
fn test_vaddps_xmm_value() {
    // VADDPS XMM0, XMM1, XMM2 ; 4x f32, upper 128 zeroed.
    let code = [0xc5, 0xf0, 0x58, 0xc2, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    let a = [1.0f32, 2.0, 4.0, 0.5];
    let b = [0.25f32, 8.0, -2.0, 0.5];
    kfa_set(&mut vcpu, 1, pack_ps(a), 0xDEAD);
    kfa_set(&mut vcpu, 2, pack_ps(b), 0xBEEF);
    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(kfa_lo(&vcpu, 0), pack_ps([1.25, 10.0, 2.0, 1.0]));
    assert_eq!(kfa_hi(&vcpu, 0), 0, "VEX.128 must zero upper 128 bits");
}

#[test]
fn test_vaddps_ymm_value() {
    // VADDPS YMM0, YMM1, YMM2 ; 8x f32.
    let code = [0xc5, 0xf4, 0x58, 0xc2, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    let a_lo = [1.0f32, 2.0, 4.0, 0.5];
    let b_lo = [0.25f32, 8.0, -2.0, 0.5];
    let a_hi = [16.0f32, 0.125, -1.0, 32.0];
    let b_hi = [16.0f32, 0.125, 1.0, -32.0];
    kfa_set(&mut vcpu, 1, pack_ps(a_lo), pack_ps(a_hi));
    kfa_set(&mut vcpu, 2, pack_ps(b_lo), pack_ps(b_hi));
    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(kfa_lo(&vcpu, 0), pack_ps([1.25, 10.0, 2.0, 1.0]));
    assert_eq!(kfa_hi(&vcpu, 0), pack_ps([32.0, 0.25, 0.0, 0.0]));
}

#[test]
fn test_vaddpd_xmm_value() {
    // VADDPD XMM0, XMM1, XMM2 ; 2x f64, upper 128 zeroed.
    let code = [0xc5, 0xf1, 0x58, 0xc2, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    kfa_set(&mut vcpu, 1, pack_pd([1.0, 4.0]), 0xDEAD);
    kfa_set(&mut vcpu, 2, pack_pd([0.5, -2.0]), 0xBEEF);
    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(kfa_lo(&vcpu, 0), pack_pd([1.5, 2.0]));
    assert_eq!(kfa_hi(&vcpu, 0), 0, "VEX.128 must zero upper 128 bits");
}

#[test]
fn test_vaddpd_ymm_value() {
    // VADDPD YMM0, YMM1, YMM2 ; 4x f64.
    let code = [0xc5, 0xf5, 0x58, 0xc2, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    kfa_set(&mut vcpu, 1, pack_pd([1.0, 4.0]), pack_pd([8.0, 0.25]));
    kfa_set(&mut vcpu, 2, pack_pd([0.5, -2.0]), pack_pd([8.0, 0.75]));
    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(kfa_lo(&vcpu, 0), pack_pd([1.5, 2.0]));
    assert_eq!(kfa_hi(&vcpu, 0), pack_pd([16.0, 1.0]));
}
