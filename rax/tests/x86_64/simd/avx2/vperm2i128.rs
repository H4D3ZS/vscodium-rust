use crate::common::*;
use vm_memory::{Bytes, GuestAddress};

// VPERM2I128 - Permute 128-bit Integer Lanes (AVX2)
//
// Permutes 128-bit lanes from two source operands and stores to destination.
// The immediate byte controls which lanes are selected:
//
// Bits [1:0] select the source for the lower 128-bit lane:
//   00: Lower lane of first source (ymm2 bits [127:0])
//   01: Upper lane of first source (ymm2 bits [255:128])
//   10: Lower lane of second source (ymm3 bits [127:0])
//   11: Upper lane of second source (ymm3 bits [255:128])
//
// Bits [5:4] select the source for the upper 128-bit lane (same encoding)
//
// Bit 3: Zero lower lane if set
// Bit 7: Zero upper lane if set
//
// Opcode: VEX.256.66.0F3A.W0 46 /r ib    VPERM2I128 ymm1, ymm2, ymm3/m256, imm8

const ALIGNED_ADDR: u64 = 0x3000;

// ============================================================================
// Tests with identity permutation (0x20 = low[2], high[0])
// ============================================================================

#[test]
fn test_vperm2i128_ymm0_ymm1_ymm2_identity() {
    // VPERM2I128 YMM0, YMM1, YMM2, 0x20 (ymm2[127:0], ymm1[127:0])
    let code = [
        0xc4, 0xe3, 0x75, 0x46, 0xc2, 0x20, // VPERM2I128 YMM0, YMM1, YMM2, 0x20
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vperm2i128_ymm3_ymm4_ymm5_identity() {
    let code = [
        0xc4, 0xe3, 0x5d, 0x46, 0xdd, 0x20, // VPERM2I128 YMM3, YMM4, YMM5, 0x20
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Tests selecting both lanes from first source (0x00)
// ============================================================================

#[test]
fn test_vperm2i128_ymm0_ymm1_ymm2_both_src1() {
    // VPERM2I128 YMM0, YMM1, YMM2, 0x00 (ymm1[127:0], ymm1[127:0])
    let code = [
        0xc4, 0xe3, 0x75, 0x46, 0xc2, 0x00, // VPERM2I128 YMM0, YMM1, YMM2, 0x00
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vperm2i128_ymm0_ymm1_ymm2_upper_from_src1() {
    // VPERM2I128 YMM0, YMM1, YMM2, 0x11 (ymm1[255:128], ymm1[255:128])
    let code = [
        0xc4, 0xe3, 0x75, 0x46, 0xc2, 0x11, // VPERM2I128 YMM0, YMM1, YMM2, 0x11
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vperm2i128_ymm0_ymm1_ymm2_swap_src1() {
    // VPERM2I128 YMM0, YMM1, YMM2, 0x01 (ymm1[255:128], ymm1[127:0])
    let code = [
        0xc4, 0xe3, 0x75, 0x46, 0xc2, 0x01, // VPERM2I128 YMM0, YMM1, YMM2, 0x01
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Tests selecting both lanes from second source (0x22, 0x33)
// ============================================================================

#[test]
fn test_vperm2i128_ymm0_ymm1_ymm2_both_src2_lower() {
    // VPERM2I128 YMM0, YMM1, YMM2, 0x22 (ymm2[127:0], ymm2[127:0])
    let code = [
        0xc4, 0xe3, 0x75, 0x46, 0xc2, 0x22, // VPERM2I128 YMM0, YMM1, YMM2, 0x22
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vperm2i128_ymm0_ymm1_ymm2_both_src2_upper() {
    // VPERM2I128 YMM0, YMM1, YMM2, 0x33 (ymm2[255:128], ymm2[255:128])
    let code = [
        0xc4, 0xe3, 0x75, 0x46, 0xc2, 0x33, // VPERM2I128 YMM0, YMM1, YMM2, 0x33
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vperm2i128_ymm0_ymm1_ymm2_swap_src2() {
    // VPERM2I128 YMM0, YMM1, YMM2, 0x23 (ymm2[255:128], ymm2[127:0])
    let code = [
        0xc4, 0xe3, 0x75, 0x46, 0xc2, 0x23, // VPERM2I128 YMM0, YMM1, YMM2, 0x23
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Tests with mixed sources
// ============================================================================

#[test]
fn test_vperm2i128_ymm0_ymm1_ymm2_mix_0x02() {
    // VPERM2I128 YMM0, YMM1, YMM2, 0x02 (ymm1[127:0], ymm2[127:0])
    let code = [
        0xc4, 0xe3, 0x75, 0x46, 0xc2, 0x02, // VPERM2I128 YMM0, YMM1, YMM2, 0x02
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vperm2i128_ymm0_ymm1_ymm2_mix_0x13() {
    // VPERM2I128 YMM0, YMM1, YMM2, 0x13 (ymm2[255:128], ymm1[255:128])
    let code = [
        0xc4, 0xe3, 0x75, 0x46, 0xc2, 0x13, // VPERM2I128 YMM0, YMM1, YMM2, 0x13
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vperm2i128_ymm0_ymm1_ymm2_mix_0x30() {
    // VPERM2I128 YMM0, YMM1, YMM2, 0x30 (ymm1[127:0], ymm2[255:128])
    let code = [
        0xc4, 0xe3, 0x75, 0x46, 0xc2, 0x30, // VPERM2I128 YMM0, YMM1, YMM2, 0x30
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vperm2i128_ymm0_ymm1_ymm2_mix_0x21() {
    // VPERM2I128 YMM0, YMM1, YMM2, 0x21 (ymm1[255:128], ymm2[127:0])
    let code = [
        0xc4, 0xe3, 0x75, 0x46, 0xc2, 0x21, // VPERM2I128 YMM0, YMM1, YMM2, 0x21
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vperm2i128_ymm0_ymm1_ymm2_mix_0x12() {
    // VPERM2I128 YMM0, YMM1, YMM2, 0x12 (ymm2[127:0], ymm1[255:128])
    let code = [
        0xc4, 0xe3, 0x75, 0x46, 0xc2, 0x12, // VPERM2I128 YMM0, YMM1, YMM2, 0x12
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vperm2i128_ymm0_ymm1_ymm2_mix_0x31() {
    // VPERM2I128 YMM0, YMM1, YMM2, 0x31 (ymm1[255:128], ymm2[255:128])
    let code = [
        0xc4, 0xe3, 0x75, 0x46, 0xc2, 0x31, // VPERM2I128 YMM0, YMM1, YMM2, 0x31
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vperm2i128_ymm0_ymm1_ymm2_mix_0x03() {
    // VPERM2I128 YMM0, YMM1, YMM2, 0x03 (ymm1[127:0], ymm2[255:128])
    let code = [
        0xc4, 0xe3, 0x75, 0x46, 0xc2, 0x03, // VPERM2I128 YMM0, YMM1, YMM2, 0x03
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Tests with zero flags (bit 3 and bit 7)
// ============================================================================

#[test]
fn test_vperm2i128_ymm0_ymm1_ymm2_zero_lower() {
    // VPERM2I128 YMM0, YMM1, YMM2, 0x08 (zero lower lane)
    let code = [
        0xc4, 0xe3, 0x75, 0x46, 0xc2, 0x08, // VPERM2I128 YMM0, YMM1, YMM2, 0x08
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vperm2i128_ymm0_ymm1_ymm2_zero_upper() {
    // VPERM2I128 YMM0, YMM1, YMM2, 0x80 (zero upper lane)
    let code = [
        0xc4, 0xe3, 0x75, 0x46, 0xc2, 0x80, // VPERM2I128 YMM0, YMM1, YMM2, 0x80
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vperm2i128_ymm0_ymm1_ymm2_zero_both() {
    // VPERM2I128 YMM0, YMM1, YMM2, 0x88 (zero both lanes)
    let code = [
        0xc4, 0xe3, 0x75, 0x46, 0xc2, 0x88, // VPERM2I128 YMM0, YMM1, YMM2, 0x88
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vperm2i128_ymm0_ymm1_ymm2_zero_lower_select_upper() {
    // VPERM2I128 YMM0, YMM1, YMM2, 0x28 (ymm2[127:0], zero)
    let code = [
        0xc4, 0xe3, 0x75, 0x46, 0xc2, 0x28, // VPERM2I128 YMM0, YMM1, YMM2, 0x28
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vperm2i128_ymm0_ymm1_ymm2_zero_upper_select_lower() {
    // VPERM2I128 YMM0, YMM1, YMM2, 0x82 (zero, ymm2[127:0])
    let code = [
        0xc4, 0xe3, 0x75, 0x46, 0xc2, 0x82, // VPERM2I128 YMM0, YMM1, YMM2, 0x82
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Tests with extended registers
// ============================================================================

#[test]
fn test_vperm2i128_ymm8_ymm9_ymm10_swap() {
    let code = [
        0xc4, 0x43, 0x35, 0x46, 0xc2, 0x01, // VPERM2I128 YMM8, YMM9, YMM10, 0x01
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vperm2i128_ymm11_ymm12_ymm13_mix() {
    let code = [
        0xc4, 0x43, 0x1d, 0x46, 0xdd, 0x30, // VPERM2I128 YMM11, YMM12, YMM13, 0x30
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vperm2i128_ymm14_ymm15_ymm0_zero() {
    let code = [
        0xc4, 0x63, 0x05, 0x46, 0xf0, 0x88, // VPERM2I128 YMM14, YMM15, YMM0, 0x88
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vperm2i128_ymm0_ymm1_ymm15_mix() {
    let code = [
        0xc4, 0xc3, 0x75, 0x46, 0xc7, 0x21, // VPERM2I128 YMM0, YMM1, YMM15, 0x21
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vperm2i128_ymm15_ymm8_ymm9_swap() {
    let code = [
        0xc4, 0x43, 0x3d, 0x46, 0xf9, 0x01, // VPERM2I128 YMM15, YMM8, YMM9, 0x01
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Tests with memory operands
// ============================================================================

#[test]
fn test_vperm2i128_ymm0_ymm1_mem_swap() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe3, 0x75, 0x46, 0x00, 0x01, // VPERM2I128 YMM0, YMM1, [RAX], 0x01
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data: Vec<u8> = (0..32).collect();
    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vperm2i128_ymm2_ymm3_mem_mix() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe3, 0x65, 0x46, 0x10, 0x30, // VPERM2I128 YMM2, YMM3, [RAX], 0x30
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data: Vec<u8> = vec![
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF,
    ];
    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vperm2i128_ymm4_ymm5_mem_zero() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe3, 0x55, 0x46, 0x20, 0x88, // VPERM2I128 YMM4, YMM5, [RAX], 0x88
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data: Vec<u8> = vec![
        0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA,
        0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA,
        0xAA, 0xAA,
    ];
    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vperm2i128_ymm6_ymm7_mem_identity() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe3, 0x45, 0x46, 0x30, 0x20, // VPERM2I128 YMM6, YMM7, [RAX], 0x20
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data: Vec<u8> = vec![
        0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55,
        0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55,
        0x55, 0x55,
    ];
    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vperm2i128_ymm8_ymm9_mem_mixed() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0x63, 0x35, 0x46, 0x00, 0x13, // VPERM2I128 YMM8, YMM9, [RAX], 0x13
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data: Vec<u8> = (0..32).map(|i| i * 2).collect();
    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Chained operations
// ============================================================================

#[test]
fn test_vperm2i128_chain_operations() {
    let code = [
        0xc4, 0xe3, 0x75, 0x46, 0xc2, 0x01, // VPERM2I128 YMM0, YMM1, YMM2, 0x01
        0xc4, 0xe3, 0x7d, 0x46, 0xc3, 0x01, // VPERM2I128 YMM0, YMM0, YMM3, 0x01
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vperm2i128_same_register() {
    let code = [
        0xc4, 0xe3, 0x75, 0x46, 0xc1, 0x01, // VPERM2I128 YMM0, YMM1, YMM1, 0x01
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Additional patterns
// ============================================================================

#[test]
fn test_vperm2i128_ymm0_ymm1_ymm2_0x10() {
    let code = [
        0xc4, 0xe3, 0x75, 0x46, 0xc2, 0x10, // VPERM2I128 YMM0, YMM1, YMM2, 0x10
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vperm2i128_ymm0_ymm1_ymm2_0x32() {
    let code = [
        0xc4, 0xe3, 0x75, 0x46, 0xc2, 0x32, // VPERM2I128 YMM0, YMM1, YMM2, 0x32
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vperm2i128_mem_unaligned() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&(ALIGNED_ADDR + 1).to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe3, 0x75, 0x46, 0x00, 0x20, // VPERM2I128 YMM0, YMM1, [RAX], 0x20
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(
        &[
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00,
        ],
        GuestAddress(ALIGNED_ADDR),
    )
    .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vperm2i128_all_extended_regs() {
    let code = [
        0xc4, 0x43, 0x05, 0x46, 0xff, 0x01, // VPERM2I128 YMM15, YMM15, YMM15, 0x01
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vperm2i128_ymm10_ymm11_ymm12_complex() {
    let code = [
        0xc4, 0x43, 0x25, 0x46, 0xd4, 0x12, // VPERM2I128 YMM10, YMM11, YMM12, 0x12
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
