use crate::common::*;
use vm_memory::{Bytes, GuestAddress};

// VPCMPEQB/VPCMPEQW/VPCMPEQD/VPCMPEQQ - Compare Packed Integers for Equality (AVX2)
//
// Performs SIMD comparison of packed integers for equality.
// For each element, if equal, all bits in result element are set to 1, otherwise 0.
//
// VPCMPEQB: Compare 32 packed byte integers (8-bit each)
// VPCMPEQW: Compare 16 packed word integers (16-bit each)
// VPCMPEQD: Compare 8 packed doubleword integers (32-bit each)
// VPCMPEQQ: Compare 4 packed quadword integers (64-bit each)
//
// Opcodes (AVX2 - 256-bit YMM):
// VEX.256.66.0F.WIG 74 /r     VPCMPEQB ymm1, ymm2, ymm3/m256
// VEX.256.66.0F.WIG 75 /r     VPCMPEQW ymm1, ymm2, ymm3/m256
// VEX.256.66.0F.WIG 76 /r     VPCMPEQD ymm1, ymm2, ymm3/m256
// VEX.256.66.0F38.WIG 29 /r   VPCMPEQQ ymm1, ymm2, ymm3/m256

const ALIGNED_ADDR: u64 = 0x3000;
const ALIGNED_ADDR2: u64 = 0x3100;

// ============================================================================
// VPCMPEQB Tests - 32x Byte Equality Comparison (256-bit)
// ============================================================================

#[test]
fn test_vpcmpeqb_ymm0_ymm1_ymm2_all_equal() {
    // VPCMPEQB YMM0, YMM1, YMM2 - all bytes equal
    let code = [
        0xc5, 0xf5, 0x74, 0xc2, // VPCMPEQB YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpcmpeqb_ymm3_ymm4_ymm5_all_different() {
    // VPCMPEQB YMM3, YMM4, YMM5 - all bytes different
    let code = [
        0xc5, 0xdd, 0x74, 0xdd, // VPCMPEQB YMM3, YMM4, YMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpcmpeqb_ymm6_ymm7_ymm8_mixed() {
    let code = [
        0xc5, 0x45, 0x74, 0xf0, // VPCMPEQB YMM6, YMM7, YMM8
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpcmpeqb_ymm9_ymm10_ymm11_alternating() {
    let code = [
        0xc4, 0x41, 0x2d, 0x74, 0xcb, // VPCMPEQB YMM9, YMM10, YMM11
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpcmpeqb_ymm12_ymm13_ymm14_zeros() {
    let code = [
        0xc4, 0x41, 0x15, 0x74, 0xe6, // VPCMPEQB YMM12, YMM13, YMM14
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpcmpeqb_ymm15_ymm0_ymm1_high_reg() {
    let code = [
        0xc4, 0xc1, 0x7d, 0x74, 0xf9, // VPCMPEQB YMM15, YMM0, YMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpcmpeqb_ymm0_ymm1_mem() {
    // VPCMPEQB YMM0, YMM1, [memory]
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0x74, 0x00, // VPCMPEQB YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(
        &[
            0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42,
            0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42,
            0x42, 0x42, 0x42, 0x42,
        ],
        GuestAddress(ALIGNED_ADDR),
    )
    .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpcmpeqb_ymm2_ymm3_mem_pattern() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xe5, 0x74, 0x10, // VPCMPEQB YMM2, YMM3, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let pattern: Vec<u8> = (0..32).collect();
    mem.write_slice(&pattern, GuestAddress(ALIGNED_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpcmpeqb_ymm4_ymm5_mem_all_same() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xd5, 0x74, 0x20, // VPCMPEQB YMM4, YMM5, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(
        &[
            0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
            0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
            0xFF, 0xFF, 0xFF, 0xFF,
        ],
        GuestAddress(ALIGNED_ADDR),
    )
    .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpcmpeqb_self_comparison() {
    // Compare register with itself (should all be equal)
    let code = [
        0xc5, 0xf5, 0x74, 0xc1, // VPCMPEQB YMM0, YMM1, YMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// VPCMPEQW Tests - 16x Word Equality Comparison (256-bit)
// ============================================================================

#[test]
fn test_vpcmpeqw_ymm0_ymm1_ymm2_all_equal() {
    // VPCMPEQW YMM0, YMM1, YMM2 - all words equal
    let code = [
        0xc5, 0xf5, 0x75, 0xc2, // VPCMPEQW YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpcmpeqw_ymm3_ymm4_ymm5_all_different() {
    // VPCMPEQW YMM3, YMM4, YMM5 - all words different
    let code = [
        0xc5, 0xdd, 0x75, 0xdd, // VPCMPEQW YMM3, YMM4, YMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpcmpeqw_ymm6_ymm7_ymm8_mixed() {
    let code = [
        0xc5, 0x45, 0x75, 0xf0, // VPCMPEQW YMM6, YMM7, YMM8
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpcmpeqw_ymm9_ymm10_ymm11_alternating() {
    let code = [
        0xc4, 0x41, 0x2d, 0x75, 0xcb, // VPCMPEQW YMM9, YMM10, YMM11
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpcmpeqw_ymm12_ymm13_ymm14_zeros() {
    let code = [
        0xc4, 0x41, 0x15, 0x75, 0xe6, // VPCMPEQW YMM12, YMM13, YMM14
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpcmpeqw_ymm15_ymm0_ymm1_high_reg() {
    let code = [
        0xc4, 0xc1, 0x7d, 0x75, 0xf9, // VPCMPEQW YMM15, YMM0, YMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpcmpeqw_ymm0_ymm1_mem() {
    // VPCMPEQW YMM0, YMM1, [memory]
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0x75, 0x00, // VPCMPEQW YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data: Vec<u8> = (0..16)
        .flat_map(|i| (i as u16 * 0x1111).to_le_bytes())
        .collect();
    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpcmpeqw_ymm2_ymm3_mem_pattern() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xe5, 0x75, 0x10, // VPCMPEQW YMM2, YMM3, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let pattern: Vec<u8> = (0..16).flat_map(|i| (i as u16).to_le_bytes()).collect();
    mem.write_slice(&pattern, GuestAddress(ALIGNED_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpcmpeqw_ymm4_ymm5_mem_all_same() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xd5, 0x75, 0x20, // VPCMPEQW YMM4, YMM5, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(
        &[
            0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
            0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
            0xFF, 0xFF, 0xFF, 0xFF,
        ],
        GuestAddress(ALIGNED_ADDR),
    )
    .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpcmpeqw_self_comparison() {
    // Compare register with itself
    let code = [
        0xc5, 0xf5, 0x75, 0xc1, // VPCMPEQW YMM0, YMM1, YMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// VPCMPEQD Tests - 8x Doubleword Equality Comparison (256-bit)
// ============================================================================

#[test]
fn test_vpcmpeqd_ymm0_ymm1_ymm2_all_equal() {
    // VPCMPEQD YMM0, YMM1, YMM2 - all dwords equal
    let code = [
        0xc5, 0xf5, 0x76, 0xc2, // VPCMPEQD YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpcmpeqd_ymm3_ymm4_ymm5_all_different() {
    // VPCMPEQD YMM3, YMM4, YMM5 - all dwords different
    let code = [
        0xc5, 0xdd, 0x76, 0xdd, // VPCMPEQD YMM3, YMM4, YMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpcmpeqd_ymm6_ymm7_ymm8_mixed() {
    let code = [
        0xc5, 0x45, 0x76, 0xf0, // VPCMPEQD YMM6, YMM7, YMM8
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpcmpeqd_ymm9_ymm10_ymm11_alternating() {
    let code = [
        0xc4, 0x41, 0x2d, 0x76, 0xcb, // VPCMPEQD YMM9, YMM10, YMM11
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpcmpeqd_ymm12_ymm13_ymm14_zeros() {
    let code = [
        0xc4, 0x41, 0x15, 0x76, 0xe6, // VPCMPEQD YMM12, YMM13, YMM14
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpcmpeqd_ymm15_ymm0_ymm1_high_reg() {
    let code = [
        0xc4, 0xc1, 0x7d, 0x76, 0xf9, // VPCMPEQD YMM15, YMM0, YMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpcmpeqd_ymm0_ymm1_mem() {
    // VPCMPEQD YMM0, YMM1, [memory]
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0x76, 0x00, // VPCMPEQD YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data: Vec<u8> = (0..8)
        .flat_map(|i| (i as u32 * 0x11111111).to_le_bytes())
        .collect();
    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpcmpeqd_ymm2_ymm3_mem_pattern() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xe5, 0x76, 0x10, // VPCMPEQD YMM2, YMM3, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let pattern: Vec<u8> = (0..8).flat_map(|i| (i as u32).to_le_bytes()).collect();
    mem.write_slice(&pattern, GuestAddress(ALIGNED_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpcmpeqd_ymm4_ymm5_mem_all_same() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xd5, 0x76, 0x20, // VPCMPEQD YMM4, YMM5, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(
        &[
            0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
            0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
            0xFF, 0xFF, 0xFF, 0xFF,
        ],
        GuestAddress(ALIGNED_ADDR),
    )
    .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpcmpeqd_self_comparison() {
    // Compare register with itself
    let code = [
        0xc5, 0xf5, 0x76, 0xc1, // VPCMPEQD YMM0, YMM1, YMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// VPCMPEQQ Tests - 4x Quadword Equality Comparison (256-bit)
// ============================================================================

#[test]
fn test_vpcmpeqq_ymm0_ymm1_ymm2_all_equal() {
    // VPCMPEQQ YMM0, YMM1, YMM2 - all qwords equal
    let code = [
        0xc4, 0xe2, 0x75, 0x29, 0xc2, // VPCMPEQQ YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpcmpeqq_ymm3_ymm4_ymm5_all_different() {
    // VPCMPEQQ YMM3, YMM4, YMM5 - all qwords different
    let code = [
        0xc4, 0xe2, 0x5d, 0x29, 0xdd, // VPCMPEQQ YMM3, YMM4, YMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpcmpeqq_ymm6_ymm7_ymm8_mixed() {
    let code = [
        0xc4, 0xc2, 0x45, 0x29, 0xf0, // VPCMPEQQ YMM6, YMM7, YMM8
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpcmpeqq_ymm9_ymm10_ymm11_alternating() {
    let code = [
        0xc4, 0x42, 0x2d, 0x29, 0xcb, // VPCMPEQQ YMM9, YMM10, YMM11
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpcmpeqq_ymm12_ymm13_ymm14_zeros() {
    let code = [
        0xc4, 0x42, 0x15, 0x29, 0xe6, // VPCMPEQQ YMM12, YMM13, YMM14
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpcmpeqq_ymm15_ymm0_ymm1_high_reg() {
    let code = [
        0xc4, 0xc2, 0x7d, 0x29, 0xf9, // VPCMPEQQ YMM15, YMM0, YMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpcmpeqq_ymm0_ymm1_mem() {
    // VPCMPEQQ YMM0, YMM1, [memory]
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x75, 0x29, 0x00, // VPCMPEQQ YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data: Vec<u8> = (0..4)
        .flat_map(|i| (i as u64 * 0x1111111111111111).to_le_bytes())
        .collect();
    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpcmpeqq_ymm2_ymm3_mem_pattern() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x65, 0x29, 0x10, // VPCMPEQQ YMM2, YMM3, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let pattern: Vec<u8> = (0..4).flat_map(|i| (i as u64).to_le_bytes()).collect();
    mem.write_slice(&pattern, GuestAddress(ALIGNED_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpcmpeqq_ymm4_ymm5_mem_all_same() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x55, 0x29, 0x20, // VPCMPEQQ YMM4, YMM5, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(
        &[
            0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
            0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
            0xFF, 0xFF, 0xFF, 0xFF,
        ],
        GuestAddress(ALIGNED_ADDR),
    )
    .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpcmpeqq_self_comparison() {
    // Compare register with itself
    let code = [
        0xc4, 0xe2, 0x75, 0x29, 0xc1, // VPCMPEQQ YMM0, YMM1, YMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Additional comprehensive tests
// ============================================================================

#[test]
fn test_vpcmpeqb_chain_operations() {
    // Chain multiple comparison operations
    let code = [
        0xc5, 0xf5, 0x74, 0xc2, // VPCMPEQB YMM0, YMM1, YMM2
        0xc5, 0xed, 0x74, 0xdb, // VPCMPEQB YMM3, YMM2, YMM3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpcmpeqw_chain_operations() {
    let code = [
        0xc5, 0xf5, 0x75, 0xc2, // VPCMPEQW YMM0, YMM1, YMM2
        0xc5, 0xed, 0x75, 0xdb, // VPCMPEQW YMM3, YMM2, YMM3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpcmpeqd_chain_operations() {
    let code = [
        0xc5, 0xf5, 0x76, 0xc2, // VPCMPEQD YMM0, YMM1, YMM2
        0xc5, 0xed, 0x76, 0xdb, // VPCMPEQD YMM3, YMM2, YMM3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpcmpeqq_chain_operations() {
    let code = [
        0xc4, 0xe2, 0x75, 0x29, 0xc2, // VPCMPEQQ YMM0, YMM1, YMM2
        0xc4, 0xe2, 0x6d, 0x29, 0xdb, // VPCMPEQQ YMM3, YMM2, YMM3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpcmpeq_mixed_sizes() {
    // Test different element sizes in sequence
    let code = [
        0xc5, 0xf5, 0x74, 0xc2, // VPCMPEQB YMM0, YMM1, YMM2
        0xc5, 0xed, 0x75, 0xdb, // VPCMPEQW YMM3, YMM2, YMM3
        0xc5, 0xe5, 0x76, 0xe4, // VPCMPEQD YMM4, YMM3, YMM4
        0xc4, 0xe2, 0x5d, 0x29, 0xed, // VPCMPEQQ YMM5, YMM4, YMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpcmpeqb_mem_unaligned() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&(ALIGNED_ADDR + 1).to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0x74, 0x00, // VPCMPEQB YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(
        &[
            0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42,
            0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42,
            0x42, 0x42, 0x42, 0x42, 0x42,
        ],
        GuestAddress(ALIGNED_ADDR),
    )
    .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Known-answer VALUE tests : per-element equality producing all-ones / all-zeros.
// ============================================================================

use rax::backend::emulator::x86_64::X86_64Vcpu;

fn keq_set(vcpu: &mut X86_64Vcpu, idx: usize, lo: u128, hi: u128) {
    let mut regs = vcpu.get_regs().unwrap();
    regs.xmm[idx][0] = lo as u64;
    regs.xmm[idx][1] = (lo >> 64) as u64;
    regs.ymm_high[idx][0] = hi as u64;
    regs.ymm_high[idx][1] = (hi >> 64) as u64;
    vcpu.set_regs(&regs).unwrap();
}
fn keq_lo(vcpu: &X86_64Vcpu, idx: usize) -> u128 {
    let r = vcpu.get_regs().unwrap();
    (r.xmm[idx][0] as u128) | ((r.xmm[idx][1] as u128) << 64)
}
fn keq_hi(vcpu: &X86_64Vcpu, idx: usize) -> u128 {
    let r = vcpu.get_regs().unwrap();
    (r.ymm_high[idx][0] as u128) | ((r.ymm_high[idx][1] as u128) << 64)
}

fn pcmpeq_bytes(a: u128, b: u128) -> u128 {
    let (ab, bb) = (a.to_le_bytes(), b.to_le_bytes());
    let mut out = [0u8; 16];
    for i in 0..16 {
        out[i] = if ab[i] == bb[i] { 0xFF } else { 0x00 };
    }
    u128::from_le_bytes(out)
}
fn pcmpeq_words(a: u128, b: u128) -> u128 {
    let mut out = 0u128;
    for i in 0..8 {
        let av = (a >> (i * 16)) & 0xFFFF;
        let bv = (b >> (i * 16)) & 0xFFFF;
        if av == bv {
            out |= 0xFFFFu128 << (i * 16);
        }
    }
    out
}
fn pcmpeq_dwords(a: u128, b: u128) -> u128 {
    let mut out = 0u128;
    for i in 0..4 {
        let av = (a >> (i * 32)) & 0xFFFF_FFFF;
        let bv = (b >> (i * 32)) & 0xFFFF_FFFF;
        if av == bv {
            out |= 0xFFFF_FFFFu128 << (i * 32);
        }
    }
    out
}
fn pcmpeq_qwords(a: u128, b: u128) -> u128 {
    let mut out = 0u128;
    if (a as u64) == (b as u64) {
        out |= 0xFFFF_FFFF_FFFF_FFFFu128;
    }
    if ((a >> 64) as u64) == ((b >> 64) as u64) {
        out |= 0xFFFF_FFFF_FFFF_FFFFu128 << 64;
    }
    out
}

// First arg matches second in some lanes, differs in others.
const EQ_A_LO: u128 = 0x11_22_33_44_55_66_77_88_99_AA_BB_CC_DD_EE_FF_00;
const EQ_B_LO: u128 = 0x11_FF_33_00_55_66_77_FF_99_AA_00_CC_DD_EE_00_00;
const EQ_A_HI: u128 = 0x0102_0304_0506_0708_090A_0B0C_0D0E_0F10;
const EQ_B_HI: u128 = 0x0102_FFFF_0506_0708_090A_0000_0D0E_0F10;

#[test]
fn test_vpcmpeqb_xmm_value() {
    let code = [0xc5, 0xf1, 0x74, 0xc2, 0xf4]; // VPCMPEQB XMM0, XMM1, XMM2
    let (mut vcpu, _) = setup_vm(&code, None);
    keq_set(&mut vcpu, 1, EQ_A_LO, 0xDEAD);
    keq_set(&mut vcpu, 2, EQ_B_LO, 0xBEEF);
    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(keq_lo(&vcpu, 0), pcmpeq_bytes(EQ_A_LO, EQ_B_LO));
    assert_eq!(keq_hi(&vcpu, 0), 0, "VEX.128 must zero upper 128 bits");
}

#[test]
fn test_vpcmpeqb_ymm_value() {
    let code = [0xc5, 0xf5, 0x74, 0xc2, 0xf4]; // VPCMPEQB YMM0, YMM1, YMM2
    let (mut vcpu, _) = setup_vm(&code, None);
    keq_set(&mut vcpu, 1, EQ_A_LO, EQ_A_HI);
    keq_set(&mut vcpu, 2, EQ_B_LO, EQ_B_HI);
    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(keq_lo(&vcpu, 0), pcmpeq_bytes(EQ_A_LO, EQ_B_LO));
    assert_eq!(keq_hi(&vcpu, 0), pcmpeq_bytes(EQ_A_HI, EQ_B_HI));
}

#[test]
fn test_vpcmpeqw_ymm_value() {
    let code = [0xc5, 0xf5, 0x75, 0xc2, 0xf4]; // VPCMPEQW YMM0, YMM1, YMM2
    let (mut vcpu, _) = setup_vm(&code, None);
    keq_set(&mut vcpu, 1, EQ_A_LO, EQ_A_HI);
    keq_set(&mut vcpu, 2, EQ_B_LO, EQ_B_HI);
    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(keq_lo(&vcpu, 0), pcmpeq_words(EQ_A_LO, EQ_B_LO));
    assert_eq!(keq_hi(&vcpu, 0), pcmpeq_words(EQ_A_HI, EQ_B_HI));
}

#[test]
fn test_vpcmpeqd_ymm_value() {
    let code = [0xc5, 0xf5, 0x76, 0xc2, 0xf4]; // VPCMPEQD YMM0, YMM1, YMM2
    let (mut vcpu, _) = setup_vm(&code, None);
    keq_set(&mut vcpu, 1, EQ_A_LO, EQ_A_HI);
    keq_set(&mut vcpu, 2, EQ_B_LO, EQ_B_HI);
    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(keq_lo(&vcpu, 0), pcmpeq_dwords(EQ_A_LO, EQ_B_LO));
    assert_eq!(keq_hi(&vcpu, 0), pcmpeq_dwords(EQ_A_HI, EQ_B_HI));
}

#[test]
fn test_vpcmpeqq_ymm_value() {
    let code = [0xc4, 0xe2, 0x75, 0x29, 0xc2, 0xf4]; // VPCMPEQQ YMM0, YMM1, YMM2
    let (mut vcpu, _) = setup_vm(&code, None);
    // lane0 lo equal, lane1 lo differ; hi lane both equal.
    let a_lo: u128 = 0xDEAD_BEEF_CAFE_BABE_0011_2233_4455_6677;
    let b_lo: u128 = 0xDEAD_BEEF_CAFE_0000_0011_2233_4455_6677;
    let a_hi: u128 = 0x1122_3344_5566_7788_99AA_BBCC_DDEE_FF00;
    let b_hi: u128 = a_hi;
    keq_set(&mut vcpu, 1, a_lo, a_hi);
    keq_set(&mut vcpu, 2, b_lo, b_hi);
    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(keq_lo(&vcpu, 0), pcmpeq_qwords(a_lo, b_lo));
    assert_eq!(keq_hi(&vcpu, 0), pcmpeq_qwords(a_hi, b_hi));
    // Sanity: low qword equal -> all ones; high qword of low lane differs -> zero.
    assert_eq!(keq_lo(&vcpu, 0), 0x0000_0000_0000_0000_FFFF_FFFF_FFFF_FFFF);
    assert_eq!(keq_hi(&vcpu, 0), u128::MAX);
}
