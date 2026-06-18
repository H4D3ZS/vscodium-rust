use crate::common::*;
use rax::cpu::Registers;
use vm_memory::{Bytes, GuestAddress};

// VPERM2F128 - Permute 128-bit Floating-Point Fields
//
// VPERM2F128 permutes 128-bit floating-point fields from two 256-bit source
// operands using an 8-bit immediate control byte. The instruction selects
// which 128-bit lane from the two sources to place in each 128-bit lane
// of the destination.
//
// Immediate control byte format (bits [3:0] for lower lane, [7:4] for upper):
//   [1:0] - Select source: 00=src1[127:0], 01=src1[255:128], 10=src2[127:0], 11=src2[255:128]
//   [3]   - Zero flag: 0=use selected source, 1=zero the lane
//
// Opcodes:
// VEX.256.66.0F3A.W0 06 /r ib    VPERM2F128 ymm1, ymm2, ymm3/m256, imm8

const ALIGNED_ADDR: u64 = 0x3000; // 32-byte aligned address for testing

// ============================================================================
// VPERM2F128 Tests - All meaningful immediate patterns
// ============================================================================

#[test]
fn test_vperm2f128_ymm0_ymm1_ymm2_imm0x00() {
    // VPERM2F128 YMM0, YMM1, YMM2, 0x00 - [1[127:0], 1[127:0]]
    let code = [
        0xc4, 0xe3, 0x75, 0x06, 0xc2, 0x00, // VPERM2F128 YMM0, YMM1, YMM2, 0x00
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vperm2f128_ymm0_ymm1_ymm2_imm0x01() {
    // VPERM2F128 YMM0, YMM1, YMM2, 0x01 - [1[255:128], 1[127:0]]
    let code = [
        0xc4, 0xe3, 0x75, 0x06, 0xc2, 0x01, // VPERM2F128 YMM0, YMM1, YMM2, 0x01
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vperm2f128_ymm0_ymm1_ymm2_imm0x02() {
    // VPERM2F128 YMM0, YMM1, YMM2, 0x02 - [2[127:0], 1[127:0]]
    let code = [
        0xc4, 0xe3, 0x75, 0x06, 0xc2, 0x02, // VPERM2F128 YMM0, YMM1, YMM2, 0x02
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vperm2f128_ymm0_ymm1_ymm2_imm0x03() {
    // VPERM2F128 YMM0, YMM1, YMM2, 0x03 - [2[255:128], 1[127:0]]
    let code = [
        0xc4, 0xe3, 0x75, 0x06, 0xc2, 0x03, // VPERM2F128 YMM0, YMM1, YMM2, 0x03
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vperm2f128_ymm0_ymm1_ymm2_imm0x10() {
    // VPERM2F128 YMM0, YMM1, YMM2, 0x10 - [1[127:0], 1[255:128]]
    let code = [
        0xc4, 0xe3, 0x75, 0x06, 0xc2, 0x10, // VPERM2F128 YMM0, YMM1, YMM2, 0x10
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vperm2f128_ymm0_ymm1_ymm2_imm0x11() {
    // VPERM2F128 YMM0, YMM1, YMM2, 0x11 - [1[255:128], 1[255:128]]
    let code = [
        0xc4, 0xe3, 0x75, 0x06, 0xc2, 0x11, // VPERM2F128 YMM0, YMM1, YMM2, 0x11
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vperm2f128_ymm0_ymm1_ymm2_imm0x12() {
    // VPERM2F128 YMM0, YMM1, YMM2, 0x12 - [2[127:0], 1[255:128]]
    let code = [
        0xc4, 0xe3, 0x75, 0x06, 0xc2, 0x12, // VPERM2F128 YMM0, YMM1, YMM2, 0x12
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vperm2f128_ymm0_ymm1_ymm2_imm0x13() {
    // VPERM2F128 YMM0, YMM1, YMM2, 0x13 - [2[255:128], 1[255:128]]
    let code = [
        0xc4, 0xe3, 0x75, 0x06, 0xc2, 0x13, // VPERM2F128 YMM0, YMM1, YMM2, 0x13
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vperm2f128_ymm0_ymm1_ymm2_imm0x20() {
    // VPERM2F128 YMM0, YMM1, YMM2, 0x20 - [1[127:0], 2[127:0]]
    let code = [
        0xc4, 0xe3, 0x75, 0x06, 0xc2, 0x20, // VPERM2F128 YMM0, YMM1, YMM2, 0x20
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vperm2f128_ymm0_ymm1_ymm2_imm0x21() {
    // VPERM2F128 YMM0, YMM1, YMM2, 0x21 - [1[255:128], 2[127:0]]
    let code = [
        0xc4, 0xe3, 0x75, 0x06, 0xc2, 0x21, // VPERM2F128 YMM0, YMM1, YMM2, 0x21
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vperm2f128_ymm0_ymm1_ymm2_imm0x22() {
    // VPERM2F128 YMM0, YMM1, YMM2, 0x22 - [2[127:0], 2[127:0]]
    let code = [
        0xc4, 0xe3, 0x75, 0x06, 0xc2, 0x22, // VPERM2F128 YMM0, YMM1, YMM2, 0x22
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vperm2f128_ymm0_ymm1_ymm2_imm0x23() {
    // VPERM2F128 YMM0, YMM1, YMM2, 0x23 - [2[255:128], 2[127:0]]
    let code = [
        0xc4, 0xe3, 0x75, 0x06, 0xc2, 0x23, // VPERM2F128 YMM0, YMM1, YMM2, 0x23
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vperm2f128_ymm0_ymm1_ymm2_imm0x30() {
    // VPERM2F128 YMM0, YMM1, YMM2, 0x30 - [1[127:0], 2[255:128]]
    let code = [
        0xc4, 0xe3, 0x75, 0x06, 0xc2, 0x30, // VPERM2F128 YMM0, YMM1, YMM2, 0x30
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vperm2f128_ymm0_ymm1_ymm2_imm0x31() {
    // VPERM2F128 YMM0, YMM1, YMM2, 0x31 - [1[255:128], 2[255:128]]
    let code = [
        0xc4, 0xe3, 0x75, 0x06, 0xc2, 0x31, // VPERM2F128 YMM0, YMM1, YMM2, 0x31
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vperm2f128_ymm0_ymm1_ymm2_imm0x32() {
    // VPERM2F128 YMM0, YMM1, YMM2, 0x32 - [2[127:0], 2[255:128]]
    let code = [
        0xc4, 0xe3, 0x75, 0x06, 0xc2, 0x32, // VPERM2F128 YMM0, YMM1, YMM2, 0x32
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vperm2f128_ymm0_ymm1_ymm2_imm0x33() {
    // VPERM2F128 YMM0, YMM1, YMM2, 0x33 - [2[255:128], 2[255:128]]
    let code = [
        0xc4, 0xe3, 0x75, 0x06, 0xc2, 0x33, // VPERM2F128 YMM0, YMM1, YMM2, 0x33
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// VPERM2F128 Tests with zero flags
// ============================================================================

#[test]
fn test_vperm2f128_ymm0_ymm1_ymm2_imm0x08() {
    // VPERM2F128 YMM0, YMM1, YMM2, 0x08 - [zero, 1[127:0]]
    let code = [
        0xc4, 0xe3, 0x75, 0x06, 0xc2, 0x08, // VPERM2F128 YMM0, YMM1, YMM2, 0x08
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vperm2f128_ymm0_ymm1_ymm2_imm0x80() {
    // VPERM2F128 YMM0, YMM1, YMM2, 0x80 - [1[127:0], zero]
    let code = [
        0xc4, 0xe3, 0x75, 0x06, 0xc2, 0x80, // VPERM2F128 YMM0, YMM1, YMM2, 0x80
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vperm2f128_ymm0_ymm1_ymm2_imm0x88() {
    // VPERM2F128 YMM0, YMM1, YMM2, 0x88 - [zero, zero]
    let code = [
        0xc4, 0xe3, 0x75, 0x06, 0xc2, 0x88, // VPERM2F128 YMM0, YMM1, YMM2, 0x88
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vperm2f128_ymm0_ymm1_ymm2_imm0x18() {
    // VPERM2F128 YMM0, YMM1, YMM2, 0x18 - [zero, 1[255:128]]
    let code = [
        0xc4, 0xe3, 0x75, 0x06, 0xc2, 0x18, // VPERM2F128 YMM0, YMM1, YMM2, 0x18
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vperm2f128_ymm0_ymm1_ymm2_imm0x81() {
    // VPERM2F128 YMM0, YMM1, YMM2, 0x81 - [1[255:128], zero]
    let code = [
        0xc4, 0xe3, 0x75, 0x06, 0xc2, 0x81, // VPERM2F128 YMM0, YMM1, YMM2, 0x81
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vperm2f128_ymm0_ymm1_ymm2_imm0x28() {
    // VPERM2F128 YMM0, YMM1, YMM2, 0x28 - [zero, 2[127:0]]
    let code = [
        0xc4, 0xe3, 0x75, 0x06, 0xc2, 0x28, // VPERM2F128 YMM0, YMM1, YMM2, 0x28
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vperm2f128_ymm0_ymm1_ymm2_imm0x82() {
    // VPERM2F128 YMM0, YMM1, YMM2, 0x82 - [2[127:0], zero]
    let code = [
        0xc4, 0xe3, 0x75, 0x06, 0xc2, 0x82, // VPERM2F128 YMM0, YMM1, YMM2, 0x82
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vperm2f128_ymm0_ymm1_ymm2_imm0x38() {
    // VPERM2F128 YMM0, YMM1, YMM2, 0x38 - [zero, 2[255:128]]
    let code = [
        0xc4, 0xe3, 0x75, 0x06, 0xc2, 0x38, // VPERM2F128 YMM0, YMM1, YMM2, 0x38
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vperm2f128_ymm0_ymm1_ymm2_imm0x83() {
    // VPERM2F128 YMM0, YMM1, YMM2, 0x83 - [2[255:128], zero]
    let code = [
        0xc4, 0xe3, 0x75, 0x06, 0xc2, 0x83, // VPERM2F128 YMM0, YMM1, YMM2, 0x83
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// VPERM2F128 Tests - Different register combinations
// ============================================================================

#[test]
fn test_vperm2f128_ymm1_ymm2_ymm3_imm0x20() {
    // VPERM2F128 YMM1, YMM2, YMM3, 0x20
    let code = [
        0xc4, 0xe3, 0x6d, 0x06, 0xcb, 0x20, // VPERM2F128 YMM1, YMM2, YMM3, 0x20
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vperm2f128_ymm2_ymm3_ymm4_imm0x31() {
    // VPERM2F128 YMM2, YMM3, YMM4, 0x31
    let code = [
        0xc4, 0xe3, 0x65, 0x06, 0xd4, 0x31, // VPERM2F128 YMM2, YMM3, YMM4, 0x31
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vperm2f128_ymm3_ymm4_ymm5_imm0x12() {
    // VPERM2F128 YMM3, YMM4, YMM5, 0x12
    let code = [
        0xc4, 0xe3, 0x5d, 0x06, 0xdd, 0x12, // VPERM2F128 YMM3, YMM4, YMM5, 0x12
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vperm2f128_ymm4_ymm5_ymm6_imm0x03() {
    // VPERM2F128 YMM4, YMM5, YMM6, 0x03
    let code = [
        0xc4, 0xe3, 0x55, 0x06, 0xe6, 0x03, // VPERM2F128 YMM4, YMM5, YMM6, 0x03
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vperm2f128_ymm5_ymm6_ymm7_imm0x21() {
    // VPERM2F128 YMM5, YMM6, YMM7, 0x21
    let code = [
        0xc4, 0xe3, 0x4d, 0x06, 0xef, 0x21, // VPERM2F128 YMM5, YMM6, YMM7, 0x21
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vperm2f128_ymm6_ymm7_ymm0_imm0x30() {
    // VPERM2F128 YMM6, YMM7, YMM0, 0x30
    let code = [
        0xc4, 0xe3, 0x45, 0x06, 0xf0, 0x30, // VPERM2F128 YMM6, YMM7, YMM0, 0x30
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vperm2f128_ymm7_ymm0_ymm1_imm0x01() {
    // VPERM2F128 YMM7, YMM0, YMM1, 0x01
    let code = [
        0xc4, 0xe3, 0x7d, 0x06, 0xf9, 0x01, // VPERM2F128 YMM7, YMM0, YMM1, 0x01
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// VPERM2F128 Tests - Extended registers
// ============================================================================

#[test]
fn test_vperm2f128_ymm8_ymm9_ymm10_imm0x20() {
    // VPERM2F128 YMM8, YMM9, YMM10, 0x20
    let code = [
        0xc4, 0xc3, 0x35, 0x06, 0xc2, 0x20, // VPERM2F128 YMM8, YMM9, YMM10, 0x20
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vperm2f128_ymm9_ymm10_ymm11_imm0x31() {
    // VPERM2F128 YMM9, YMM10, YMM11, 0x31
    let code = [
        0xc4, 0xc3, 0x2d, 0x06, 0xcb, 0x31, // VPERM2F128 YMM9, YMM10, YMM11, 0x31
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vperm2f128_ymm10_ymm11_ymm12_imm0x12() {
    // VPERM2F128 YMM10, YMM11, YMM12, 0x12
    let code = [
        0xc4, 0xc3, 0x25, 0x06, 0xd4, 0x12, // VPERM2F128 YMM10, YMM11, YMM12, 0x12
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vperm2f128_ymm11_ymm12_ymm13_imm0x03() {
    // VPERM2F128 YMM11, YMM12, YMM13, 0x03
    let code = [
        0xc4, 0xc3, 0x1d, 0x06, 0xdd, 0x03, // VPERM2F128 YMM11, YMM12, YMM13, 0x03
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vperm2f128_ymm12_ymm13_ymm14_imm0x21() {
    // VPERM2F128 YMM12, YMM13, YMM14, 0x21
    let code = [
        0xc4, 0xc3, 0x15, 0x06, 0xe6, 0x21, // VPERM2F128 YMM12, YMM13, YMM14, 0x21
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vperm2f128_ymm13_ymm14_ymm15_imm0x30() {
    // VPERM2F128 YMM13, YMM14, YMM15, 0x30
    let code = [
        0xc4, 0xc3, 0x0d, 0x06, 0xef, 0x30, // VPERM2F128 YMM13, YMM14, YMM15, 0x30
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vperm2f128_ymm14_ymm15_ymm0_imm0x01() {
    // VPERM2F128 YMM14, YMM15, YMM0, 0x01
    let code = [
        0xc4, 0xe3, 0x05, 0x06, 0xf0, 0x01, // VPERM2F128 YMM14, YMM15, YMM0, 0x01
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vperm2f128_ymm15_ymm0_ymm1_imm0x10() {
    // VPERM2F128 YMM15, YMM0, YMM1, 0x10
    let code = [
        0xc4, 0xe3, 0x7d, 0x06, 0xf9, 0x10, // VPERM2F128 YMM15, YMM0, YMM1, 0x10
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// VPERM2F128 Memory Tests
// ============================================================================

#[test]
fn test_vperm2f128_ymm0_ymm1_mem256_imm0x20() {
    // VPERM2F128 YMM0, YMM1, [mem256], 0x20
    let code = [
        0xc4, 0xe3, 0x75, 0x06, 0x05, 0x00, 0x40, 0x00, 0x00,
        0x20, // VPERM2F128 YMM0, YMM1, [rip + 0x4000], 0x20
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    // Initialize memory with test data
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
fn test_vperm2f128_ymm1_ymm2_mem256_imm0x31() {
    // VPERM2F128 YMM1, YMM2, [mem256], 0x31
    let code = [
        0xc4, 0xe3, 0x6d, 0x06, 0x0d, 0x00, 0x40, 0x00, 0x00,
        0x31, // VPERM2F128 YMM1, YMM2, [rip + 0x4000], 0x31
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    let test_data: [u8; 32] = [
        0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa,
        0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa,
        0xaa, 0xaa,
    ];
    mem.write_slice(&test_data, GuestAddress(ALIGNED_ADDR))
        .unwrap();

    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vperm2f128_ymm8_ymm9_mem256_imm0x12() {
    // VPERM2F128 YMM8, YMM9, [mem256], 0x12
    let code = [
        0xc4, 0xc3, 0x35, 0x06, 0x05, 0x00, 0x40, 0x00, 0x00,
        0x12, // VPERM2F128 YMM8, YMM9, [rip + 0x4000], 0x12
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    let test_data: [u8; 32] = [
        0xbb, 0xbb, 0xbb, 0xbb, 0xbb, 0xbb, 0xbb, 0xbb, 0xbb, 0xbb, 0xbb, 0xbb, 0xbb, 0xbb, 0xbb,
        0xbb, 0xbb, 0xbb, 0xbb, 0xbb, 0xbb, 0xbb, 0xbb, 0xbb, 0xbb, 0xbb, 0xbb, 0xbb, 0xbb, 0xbb,
        0xbb, 0xbb,
    ];
    mem.write_slice(&test_data, GuestAddress(ALIGNED_ADDR))
        .unwrap();

    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vperm2f128_ymm15_ymm14_mem256_imm0x01() {
    // VPERM2F128 YMM15, YMM14, [mem256], 0x01
    let code = [
        0xc4, 0xe3, 0x0d, 0x06, 0x3d, 0x00, 0x40, 0x00, 0x00,
        0x01, // VPERM2F128 YMM15, YMM14, [rip + 0x4000], 0x01
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    let test_data: [u8; 32] = [
        0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc,
        0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc,
        0xcc, 0xcc,
    ];
    mem.write_slice(&test_data, GuestAddress(ALIGNED_ADDR))
        .unwrap();

    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vperm2f128_ymm0_ymm1_mem256_imm0x88() {
    // VPERM2F128 YMM0, YMM1, [mem256], 0x88 - both lanes zeroed
    let code = [
        0xc4, 0xe3, 0x75, 0x06, 0x05, 0x00, 0x40, 0x00, 0x00,
        0x88, // VPERM2F128 YMM0, YMM1, [rip + 0x4000], 0x88
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    let test_data: [u8; 32] = [
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0xff,
    ];
    mem.write_slice(&test_data, GuestAddress(ALIGNED_ADDR))
        .unwrap();

    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Known-answer VALUE tests : VPERM2F128 selects a 128-bit lane for each output
// lane. imm[1:0] -> dst low lane, imm[5:4] -> dst high lane (0=s1.lo,1=s1.hi,
// 2=s2.lo,3=s2.hi). imm bit 3 zeroes low lane, imm bit 7 zeroes high lane.
// ============================================================================

use rax::backend::emulator::x86_64::X86_64Vcpu;

fn kp2_set(vcpu: &mut X86_64Vcpu, idx: usize, lo: u128, hi: u128) {
    let mut regs = vcpu.get_regs().unwrap();
    regs.xmm[idx][0] = lo as u64;
    regs.xmm[idx][1] = (lo >> 64) as u64;
    regs.ymm_high[idx][0] = hi as u64;
    regs.ymm_high[idx][1] = (hi >> 64) as u64;
    vcpu.set_regs(&regs).unwrap();
}
fn kp2_lo(vcpu: &X86_64Vcpu, idx: usize) -> u128 {
    let r = vcpu.get_regs().unwrap();
    (r.xmm[idx][0] as u128) | ((r.xmm[idx][1] as u128) << 64)
}
fn kp2_hi(vcpu: &X86_64Vcpu, idx: usize) -> u128 {
    let r = vcpu.get_regs().unwrap();
    (r.ymm_high[idx][0] as u128) | ((r.ymm_high[idx][1] as u128) << 64)
}

fn perm2_lane(sel: u8, s1_lo: u128, s1_hi: u128, s2_lo: u128, s2_hi: u128) -> u128 {
    match sel & 3 {
        0 => s1_lo,
        1 => s1_hi,
        2 => s2_lo,
        _ => s2_hi,
    }
}
fn perm2(imm: u8, s1_lo: u128, s1_hi: u128, s2_lo: u128, s2_hi: u128) -> (u128, u128) {
    let mut lo = perm2_lane(imm, s1_lo, s1_hi, s2_lo, s2_hi);
    let mut hi = perm2_lane(imm >> 4, s1_lo, s1_hi, s2_lo, s2_hi);
    if (imm >> 3) & 1 == 1 {
        lo = 0;
    }
    if (imm >> 7) & 1 == 1 {
        hi = 0;
    }
    (lo, hi)
}

const P1_LO: u128 = 0x1111_1111_1111_1111_1111_1111_1111_1111;
const P1_HI: u128 = 0x2222_2222_2222_2222_2222_2222_2222_2222;
const P2_LO: u128 = 0x3333_3333_3333_3333_3333_3333_3333_3333;
const P2_HI: u128 = 0x4444_4444_4444_4444_4444_4444_4444_4444;

fn run_perm2(imm: u8) -> (u128, u128) {
    let code = [0xc4, 0xe3, 0x75, 0x06, 0xc2, imm, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    kp2_set(&mut vcpu, 1, P1_LO, P1_HI);
    kp2_set(&mut vcpu, 2, P2_LO, P2_HI);
    run_until_hlt(&mut vcpu).unwrap();
    (kp2_lo(&vcpu, 0), kp2_hi(&vcpu, 0))
}

#[test]
fn test_vperm2f128_swap_lanes() {
    // imm 0x01: dst.lo = src1.hi, dst.hi = src1.lo.
    let (lo, hi) = run_perm2(0x01);
    assert_eq!((lo, hi), perm2(0x01, P1_LO, P1_HI, P2_LO, P2_HI));
    assert_eq!(lo, P1_HI);
    assert_eq!(hi, P1_LO);
}

#[test]
fn test_vperm2f128_cross_src() {
    // imm 0x20: dst.lo = sel(imm[1:0]=0)=src1.lo ; dst.hi = sel(imm[5:4]=2)=src2.lo.
    let (lo, hi) = run_perm2(0x20);
    assert_eq!((lo, hi), perm2(0x20, P1_LO, P1_HI, P2_LO, P2_HI));
    assert_eq!(lo, P1_LO);
    assert_eq!(hi, P2_LO);
}

#[test]
fn test_vperm2f128_high_from_src2_hi() {
    // imm 0x31: dst.lo = src1.hi, dst.hi = src2.hi.
    let (lo, hi) = run_perm2(0x31);
    assert_eq!((lo, hi), perm2(0x31, P1_LO, P1_HI, P2_LO, P2_HI));
    assert_eq!(lo, P1_HI);
    assert_eq!(hi, P2_HI);
}

#[test]
fn test_vperm2f128_zero_low_lane() {
    // imm 0x08: bit 3 set -> low lane forced to zero; high from src1.lo.
    let (lo, hi) = run_perm2(0x08);
    assert_eq!((lo, hi), perm2(0x08, P1_LO, P1_HI, P2_LO, P2_HI));
    assert_eq!(lo, 0);
    assert_eq!(hi, P1_LO);
}

#[test]
fn test_vperm2f128_zero_high_lane() {
    // imm 0x82: bit 7 set -> high lane zeroed; low from src2.lo.
    let (lo, hi) = run_perm2(0x82);
    assert_eq!((lo, hi), perm2(0x82, P1_LO, P1_HI, P2_LO, P2_HI));
    assert_eq!(lo, P2_LO);
    assert_eq!(hi, 0);
}
