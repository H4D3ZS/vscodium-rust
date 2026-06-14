use crate::common::*;
use rax::cpu::Registers;
use vm_memory::{Bytes, GuestAddress};

// VSHUFPS - Shuffle Packed Single-Precision Floating-Point Values
// VSHUFPD - Shuffle Packed Double-Precision Floating-Point Values
//
// VSHUFPS shuffles four single-precision FP values from two source operands
// using an 8-bit immediate control byte. Each 2-bit field selects which element
// to copy to the destination.
// VSHUFPD shuffles two or four double-precision FP values from two source operands
// using immediate control bits.
//
// Opcodes:
// VEX.128.0F.WIG C6 /r ib       VSHUFPS xmm1, xmm2, xmm3/m128, imm8
// VEX.256.0F.WIG C6 /r ib       VSHUFPS ymm1, ymm2, ymm3/m256, imm8
// VEX.128.66.0F.WIG C6 /r ib    VSHUFPD xmm1, xmm2, xmm3/m128, imm8
// VEX.256.66.0F.WIG C6 /r ib    VSHUFPD ymm1, ymm2, ymm3/m256, imm8

const ALIGNED_ADDR: u64 = 0x3000; // 32-byte aligned address for testing

// ============================================================================
// VSHUFPS 128-bit Tests - Shuffle Single-Precision (XMM)
// ============================================================================

#[test]
fn test_vshufps_xmm0_xmm1_xmm2_imm0x00() {
    // VSHUFPS XMM0, XMM1, XMM2, 0x00 - [1[0], 1[0], 2[0], 2[0]]
    let code = [
        0xc5, 0xf0, 0xc6, 0xc2, 0x00, // VSHUFPS XMM0, XMM1, XMM2, 0x00
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vshufps_xmm0_xmm1_xmm2_imm0xff() {
    // VSHUFPS XMM0, XMM1, XMM2, 0xFF - [1[3], 1[3], 2[3], 2[3]]
    let code = [
        0xc5, 0xf0, 0xc6, 0xc2, 0xff, // VSHUFPS XMM0, XMM1, XMM2, 0xFF
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vshufps_xmm0_xmm1_xmm2_imm0xe4() {
    // VSHUFPS XMM0, XMM1, XMM2, 0xE4 - [1[0], 1[1], 2[2], 2[3]]
    let code = [
        0xc5, 0xf0, 0xc6, 0xc2, 0xe4, // VSHUFPS XMM0, XMM1, XMM2, 0xE4
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vshufps_xmm1_xmm2_xmm3_imm0x1b() {
    // VSHUFPS XMM1, XMM2, XMM3, 0x1B - [2[3], 2[2], 3[1], 3[0]]
    let code = [
        0xc5, 0xe8, 0xc6, 0xcb, 0x1b, // VSHUFPS XMM1, XMM2, XMM3, 0x1B
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vshufps_xmm2_xmm3_xmm4_imm0x4e() {
    // VSHUFPS XMM2, XMM3, XMM4, 0x4E - [3[2], 3[1], 4[0], 4[1]]
    let code = [
        0xc5, 0xe0, 0xc6, 0xd4, 0x4e, // VSHUFPS XMM2, XMM3, XMM4, 0x4E
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vshufps_xmm3_xmm4_xmm5_imm0xb1() {
    // VSHUFPS XMM3, XMM4, XMM5, 0xB1 - [4[1], 4[0], 5[3], 5[2]]
    let code = [
        0xc5, 0xd8, 0xc6, 0xdd, 0xb1, // VSHUFPS XMM3, XMM4, XMM5, 0xB1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vshufps_xmm4_xmm5_xmm6_imm0x72() {
    // VSHUFPS XMM4, XMM5, XMM6, 0x72
    let code = [
        0xc5, 0xd0, 0xc6, 0xe6, 0x72, // VSHUFPS XMM4, XMM5, XMM6, 0x72
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vshufps_xmm5_xmm6_xmm7_imm0x39() {
    // VSHUFPS XMM5, XMM6, XMM7, 0x39
    let code = [
        0xc5, 0xc8, 0xc6, 0xef, 0x39, // VSHUFPS XMM5, XMM6, XMM7, 0x39
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vshufps_xmm6_xmm7_xmm0_imm0xd8() {
    // VSHUFPS XMM6, XMM7, XMM0, 0xD8
    let code = [
        0xc5, 0xc0, 0xc6, 0xf0, 0xd8, // VSHUFPS XMM6, XMM7, XMM0, 0xD8
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vshufps_xmm7_xmm0_xmm1_imm0x27() {
    // VSHUFPS XMM7, XMM0, XMM1, 0x27
    let code = [
        0xc5, 0xf8, 0xc6, 0xf9, 0x27, // VSHUFPS XMM7, XMM0, XMM1, 0x27
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vshufps_xmm8_xmm9_xmm10_imm0x88() {
    // VSHUFPS XMM8, XMM9, XMM10, 0x88
    let code = [
        0xc4, 0xc1, 0x30, 0xc6, 0xc2, 0x88, // VSHUFPS XMM8, XMM9, XMM10, 0x88
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vshufps_xmm12_xmm13_xmm14_imm0xdd() {
    // VSHUFPS XMM12, XMM13, XMM14, 0xDD
    let code = [
        0xc4, 0xc1, 0x10, 0xc6, 0xe6, 0xdd, // VSHUFPS XMM12, XMM13, XMM14, 0xDD
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// VSHUFPS 256-bit Tests - Shuffle Single-Precision (YMM)
// ============================================================================

#[test]
fn test_vshufps_ymm0_ymm1_ymm2_imm0x00() {
    // VSHUFPS YMM0, YMM1, YMM2, 0x00
    let code = [
        0xc5, 0xf4, 0xc6, 0xc2, 0x00, // VSHUFPS YMM0, YMM1, YMM2, 0x00
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vshufps_ymm0_ymm1_ymm2_imm0xff() {
    // VSHUFPS YMM0, YMM1, YMM2, 0xFF
    let code = [
        0xc5, 0xf4, 0xc6, 0xc2, 0xff, // VSHUFPS YMM0, YMM1, YMM2, 0xFF
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vshufps_ymm1_ymm2_ymm3_imm0xe4() {
    // VSHUFPS YMM1, YMM2, YMM3, 0xE4
    let code = [
        0xc5, 0xec, 0xc6, 0xcb, 0xe4, // VSHUFPS YMM1, YMM2, YMM3, 0xE4
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vshufps_ymm2_ymm3_ymm4_imm0x1b() {
    // VSHUFPS YMM2, YMM3, YMM4, 0x1B
    let code = [
        0xc5, 0xe4, 0xc6, 0xd4, 0x1b, // VSHUFPS YMM2, YMM3, YMM4, 0x1B
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vshufps_ymm3_ymm4_ymm5_imm0x4e() {
    // VSHUFPS YMM3, YMM4, YMM5, 0x4E
    let code = [
        0xc5, 0xdc, 0xc6, 0xdd, 0x4e, // VSHUFPS YMM3, YMM4, YMM5, 0x4E
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vshufps_ymm4_ymm5_ymm6_imm0xb1() {
    // VSHUFPS YMM4, YMM5, YMM6, 0xB1
    let code = [
        0xc5, 0xd4, 0xc6, 0xe6, 0xb1, // VSHUFPS YMM4, YMM5, YMM6, 0xB1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vshufps_ymm5_ymm6_ymm7_imm0x72() {
    // VSHUFPS YMM5, YMM6, YMM7, 0x72
    let code = [
        0xc5, 0xcc, 0xc6, 0xef, 0x72, // VSHUFPS YMM5, YMM6, YMM7, 0x72
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vshufps_ymm6_ymm7_ymm0_imm0x39() {
    // VSHUFPS YMM6, YMM7, YMM0, 0x39
    let code = [
        0xc5, 0xc4, 0xc6, 0xf0, 0x39, // VSHUFPS YMM6, YMM7, YMM0, 0x39
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vshufps_ymm7_ymm0_ymm1_imm0xd8() {
    // VSHUFPS YMM7, YMM0, YMM1, 0xD8
    let code = [
        0xc5, 0xfc, 0xc6, 0xf9, 0xd8, // VSHUFPS YMM7, YMM0, YMM1, 0xD8
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vshufps_ymm8_ymm9_ymm10_imm0x27() {
    // VSHUFPS YMM8, YMM9, YMM10, 0x27
    let code = [
        0xc4, 0xc1, 0x34, 0xc6, 0xc2, 0x27, // VSHUFPS YMM8, YMM9, YMM10, 0x27
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vshufps_ymm12_ymm13_ymm14_imm0x88() {
    // VSHUFPS YMM12, YMM13, YMM14, 0x88
    let code = [
        0xc4, 0xc1, 0x14, 0xc6, 0xe6, 0x88, // VSHUFPS YMM12, YMM13, YMM14, 0x88
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// VSHUFPS Memory Tests
// ============================================================================

#[test]
fn test_vshufps_xmm0_xmm1_mem128_imm0xe4() {
    // VSHUFPS XMM0, XMM1, [mem128], 0xE4
    let code = [
        0xc5, 0xf0, 0xc6, 0x05, 0x00, 0x40, 0x00, 0x00,
        0xe4, // VSHUFPS XMM0, XMM1, [rip + 0x4000], 0xE4
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    // Initialize memory with test data (four floats)
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
fn test_vshufps_ymm0_ymm1_mem256_imm0x1b() {
    // VSHUFPS YMM0, YMM1, [mem256], 0x1B
    let code = [
        0xc5, 0xf4, 0xc6, 0x05, 0x00, 0x40, 0x00, 0x00,
        0x1b, // VSHUFPS YMM0, YMM1, [rip + 0x4000], 0x1B
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    // Initialize memory with test data (eight floats)
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

// ============================================================================
// VSHUFPD 128-bit Tests - Shuffle Double-Precision (XMM)
// ============================================================================

#[test]
fn test_vshufpd_xmm0_xmm1_xmm2_imm0x0() {
    // VSHUFPD XMM0, XMM1, XMM2, 0x0 - [1[0], 2[0]]
    let code = [
        0xc5, 0xf1, 0xc6, 0xc2, 0x00, // VSHUFPD XMM0, XMM1, XMM2, 0x0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vshufpd_xmm0_xmm1_xmm2_imm0x1() {
    // VSHUFPD XMM0, XMM1, XMM2, 0x1 - [1[1], 2[0]]
    let code = [
        0xc5, 0xf1, 0xc6, 0xc2, 0x01, // VSHUFPD XMM0, XMM1, XMM2, 0x1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vshufpd_xmm0_xmm1_xmm2_imm0x2() {
    // VSHUFPD XMM0, XMM1, XMM2, 0x2 - [1[0], 2[1]]
    let code = [
        0xc5, 0xf1, 0xc6, 0xc2, 0x02, // VSHUFPD XMM0, XMM1, XMM2, 0x2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vshufpd_xmm0_xmm1_xmm2_imm0x3() {
    // VSHUFPD XMM0, XMM1, XMM2, 0x3 - [1[1], 2[1]]
    let code = [
        0xc5, 0xf1, 0xc6, 0xc2, 0x03, // VSHUFPD XMM0, XMM1, XMM2, 0x3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vshufpd_xmm1_xmm2_xmm3_imm0x0() {
    // VSHUFPD XMM1, XMM2, XMM3, 0x0
    let code = [
        0xc5, 0xe9, 0xc6, 0xcb, 0x00, // VSHUFPD XMM1, XMM2, XMM3, 0x0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vshufpd_xmm2_xmm3_xmm4_imm0x1() {
    // VSHUFPD XMM2, XMM3, XMM4, 0x1
    let code = [
        0xc5, 0xe1, 0xc6, 0xd4, 0x01, // VSHUFPD XMM2, XMM3, XMM4, 0x1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vshufpd_xmm3_xmm4_xmm5_imm0x2() {
    // VSHUFPD XMM3, XMM4, XMM5, 0x2
    let code = [
        0xc5, 0xd9, 0xc6, 0xdd, 0x02, // VSHUFPD XMM3, XMM4, XMM5, 0x2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vshufpd_xmm4_xmm5_xmm6_imm0x3() {
    // VSHUFPD XMM4, XMM5, XMM6, 0x3
    let code = [
        0xc5, 0xd1, 0xc6, 0xe6, 0x03, // VSHUFPD XMM4, XMM5, XMM6, 0x3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vshufpd_xmm8_xmm9_xmm10_imm0x1() {
    // VSHUFPD XMM8, XMM9, XMM10, 0x1
    let code = [
        0xc4, 0xc1, 0x31, 0xc6, 0xc2, 0x01, // VSHUFPD XMM8, XMM9, XMM10, 0x1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vshufpd_xmm12_xmm13_xmm14_imm0x2() {
    // VSHUFPD XMM12, XMM13, XMM14, 0x2
    let code = [
        0xc4, 0xc1, 0x11, 0xc6, 0xe6, 0x02, // VSHUFPD XMM12, XMM13, XMM14, 0x2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// VSHUFPD 256-bit Tests - Shuffle Double-Precision (YMM)
// ============================================================================

#[test]
fn test_vshufpd_ymm0_ymm1_ymm2_imm0x0() {
    // VSHUFPD YMM0, YMM1, YMM2, 0x0 - [1[0], 2[0], 1[2], 2[2]]
    let code = [
        0xc5, 0xf5, 0xc6, 0xc2, 0x00, // VSHUFPD YMM0, YMM1, YMM2, 0x0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vshufpd_ymm0_ymm1_ymm2_imm0x5() {
    // VSHUFPD YMM0, YMM1, YMM2, 0x5 - [1[1], 2[0], 1[3], 2[2]]
    let code = [
        0xc5, 0xf5, 0xc6, 0xc2, 0x05, // VSHUFPD YMM0, YMM1, YMM2, 0x5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vshufpd_ymm0_ymm1_ymm2_imm0xa() {
    // VSHUFPD YMM0, YMM1, YMM2, 0xA - [1[0], 2[1], 1[2], 2[3]]
    let code = [
        0xc5, 0xf5, 0xc6, 0xc2, 0x0a, // VSHUFPD YMM0, YMM1, YMM2, 0xA
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vshufpd_ymm0_ymm1_ymm2_imm0xf() {
    // VSHUFPD YMM0, YMM1, YMM2, 0xF - [1[1], 2[1], 1[3], 2[3]]
    let code = [
        0xc5, 0xf5, 0xc6, 0xc2, 0x0f, // VSHUFPD YMM0, YMM1, YMM2, 0xF
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vshufpd_ymm1_ymm2_ymm3_imm0x6() {
    // VSHUFPD YMM1, YMM2, YMM3, 0x6
    let code = [
        0xc5, 0xed, 0xc6, 0xcb, 0x06, // VSHUFPD YMM1, YMM2, YMM3, 0x6
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vshufpd_ymm2_ymm3_ymm4_imm0x9() {
    // VSHUFPD YMM2, YMM3, YMM4, 0x9
    let code = [
        0xc5, 0xe5, 0xc6, 0xd4, 0x09, // VSHUFPD YMM2, YMM3, YMM4, 0x9
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vshufpd_ymm3_ymm4_ymm5_imm0xc() {
    // VSHUFPD YMM3, YMM4, YMM5, 0xC
    let code = [
        0xc5, 0xdd, 0xc6, 0xdd, 0x0c, // VSHUFPD YMM3, YMM4, YMM5, 0xC
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vshufpd_ymm4_ymm5_ymm6_imm0x3() {
    // VSHUFPD YMM4, YMM5, YMM6, 0x3
    let code = [
        0xc5, 0xd5, 0xc6, 0xe6, 0x03, // VSHUFPD YMM4, YMM5, YMM6, 0x3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vshufpd_ymm8_ymm9_ymm10_imm0x7() {
    // VSHUFPD YMM8, YMM9, YMM10, 0x7
    let code = [
        0xc4, 0xc1, 0x35, 0xc6, 0xc2, 0x07, // VSHUFPD YMM8, YMM9, YMM10, 0x7
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vshufpd_ymm12_ymm13_ymm14_imm0xd() {
    // VSHUFPD YMM12, YMM13, YMM14, 0xD
    let code = [
        0xc4, 0xc1, 0x15, 0xc6, 0xe6, 0x0d, // VSHUFPD YMM12, YMM13, YMM14, 0xD
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// VSHUFPD Memory Tests
// ============================================================================

#[test]
fn test_vshufpd_xmm0_xmm1_mem128_imm0x3() {
    // VSHUFPD XMM0, XMM1, [mem128], 0x3
    let code = [
        0xc5, 0xf1, 0xc6, 0x05, 0x00, 0x40, 0x00, 0x00,
        0x03, // VSHUFPD XMM0, XMM1, [rip + 0x4000], 0x3
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    // Initialize memory with test data (two doubles)
    let test_data: [u8; 16] = [
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xf0, 0x3f, // 1.0
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x40, // 2.0
    ];
    mem.write_slice(&test_data, GuestAddress(ALIGNED_ADDR))
        .unwrap();

    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vshufpd_ymm0_ymm1_mem256_imm0xa() {
    // VSHUFPD YMM0, YMM1, [mem256], 0xA
    let code = [
        0xc5, 0xf5, 0xc6, 0x05, 0x00, 0x40, 0x00, 0x00,
        0x0a, // VSHUFPD YMM0, YMM1, [rip + 0x4000], 0xA
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    // Initialize memory with test data (four doubles)
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

// ============================================================================
// Known-answer VALUE tests : VSHUFPS / VSHUFPD selecting elements by imm8.
//   VSHUFPS (per 128-bit lane): out[0]=s1[imm[1:0]], out[1]=s1[imm[3:2]],
//                               out[2]=s2[imm[5:4]], out[3]=s2[imm[7:6]] (dwords).
//   VSHUFPD (per 128-bit lane): out[0]=s1[imm bit], out[1]=s2[imm bit] (qwords).
// ============================================================================

use rax::backend::emulator::x86_64::X86_64Vcpu;

fn kshf_set(vcpu: &mut X86_64Vcpu, idx: usize, lo: u128, hi: u128) {
    let mut regs = vcpu.get_regs().unwrap();
    regs.xmm[idx][0] = lo as u64;
    regs.xmm[idx][1] = (lo >> 64) as u64;
    regs.ymm_high[idx][0] = hi as u64;
    regs.ymm_high[idx][1] = (hi >> 64) as u64;
    vcpu.set_regs(&regs).unwrap();
}
fn kshf_lo(vcpu: &X86_64Vcpu, idx: usize) -> u128 {
    let r = vcpu.get_regs().unwrap();
    (r.xmm[idx][0] as u128) | ((r.xmm[idx][1] as u128) << 64)
}
fn kshf_hi(vcpu: &X86_64Vcpu, idx: usize) -> u128 {
    let r = vcpu.get_regs().unwrap();
    (r.ymm_high[idx][0] as u128) | ((r.ymm_high[idx][1] as u128) << 64)
}

fn shufps_lane(s1: u128, s2: u128, imm: u8) -> u128 {
    let dw = |v: u128, n: u32| ((v >> (n * 32)) & 0xFFFF_FFFF) as u128;
    let s0 = ((imm >> 0) & 3) as u32;
    let s1sel = ((imm >> 2) & 3) as u32;
    let s2sel = ((imm >> 4) & 3) as u32;
    let s3 = ((imm >> 6) & 3) as u32;
    dw(s1, s0) | (dw(s1, s1sel) << 32) | (dw(s2, s2sel) << 64) | (dw(s2, s3) << 96)
}
fn shufpd_lane(s1: u128, s2: u128, b0: u32, b1: u32) -> u128 {
    let qw = |v: u128, n: u32| ((v >> (n * 64)) & 0xFFFF_FFFF_FFFF_FFFF) as u128;
    qw(s1, b0) | (qw(s2, b1) << 64)
}

const F1_LO: u128 = 0x3333_3333_2222_2222_1111_1111_0000_0000;
const F2_LO: u128 = 0x7777_7777_6666_6666_5555_5555_4444_4444;
const F1_HI: u128 = 0xBBBB_BBBB_AAAA_AAAA_9999_9999_8888_8888;
const F2_HI: u128 = 0xFFFF_FFFF_EEEE_EEEE_DDDD_DDDD_CCCC_CCCC;

#[test]
fn test_vshufps_xmm_value() {
    // VSHUFPS XMM0, XMM1, XMM2, 0xE4 ; upper 128 zeroed.
    let code = [0xc5, 0xf0, 0xc6, 0xc2, 0xe4, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    kshf_set(&mut vcpu, 1, F1_LO, 0xDEAD);
    kshf_set(&mut vcpu, 2, F2_LO, 0xBEEF);
    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(kshf_lo(&vcpu, 0), shufps_lane(F1_LO, F2_LO, 0xe4));
    assert_eq!(kshf_hi(&vcpu, 0), 0, "VEX.128 must zero upper 128 bits");
}

#[test]
fn test_vshufps_ymm_value() {
    // VSHUFPS YMM0, YMM1, YMM2, 0x1B ; imm applies per 128-bit lane.
    let code = [0xc5, 0xf4, 0xc6, 0xc2, 0x1b, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    kshf_set(&mut vcpu, 1, F1_LO, F1_HI);
    kshf_set(&mut vcpu, 2, F2_LO, F2_HI);
    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(kshf_lo(&vcpu, 0), shufps_lane(F1_LO, F2_LO, 0x1b));
    assert_eq!(kshf_hi(&vcpu, 0), shufps_lane(F1_HI, F2_HI, 0x1b));
}

#[test]
fn test_vshufpd_xmm_value() {
    // VSHUFPD XMM0, XMM1, XMM2, 0x1 ; out=[s1[1], s2[0]]; upper 128 zeroed.
    let code = [0xc5, 0xf1, 0xc6, 0xc2, 0x01, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    kshf_set(&mut vcpu, 1, F1_LO, 0xDEAD);
    kshf_set(&mut vcpu, 2, F2_LO, 0xBEEF);
    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(kshf_lo(&vcpu, 0), shufpd_lane(F1_LO, F2_LO, 1, 0));
    assert_eq!(kshf_hi(&vcpu, 0), 0, "VEX.128 must zero upper 128 bits");
}

#[test]
fn test_vshufpd_ymm_value() {
    // VSHUFPD YMM0, YMM1, YMM2, 0b1001 ; lane0 imm bits {0,1}=1,0 ; lane1 bits {2,3}=0,1.
    let imm: u8 = 0b1001;
    let code = [0xc5, 0xf5, 0xc6, 0xc2, imm, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    kshf_set(&mut vcpu, 1, F1_LO, F1_HI);
    kshf_set(&mut vcpu, 2, F2_LO, F2_HI);
    run_until_hlt(&mut vcpu).unwrap();
    let b0 = (imm & 1) as u32;
    let b1 = ((imm >> 1) & 1) as u32;
    let b2 = ((imm >> 2) & 1) as u32;
    let b3 = ((imm >> 3) & 1) as u32;
    assert_eq!(kshf_lo(&vcpu, 0), shufpd_lane(F1_LO, F2_LO, b0, b1));
    assert_eq!(kshf_hi(&vcpu, 0), shufpd_lane(F1_HI, F2_HI, b2, b3));
}
