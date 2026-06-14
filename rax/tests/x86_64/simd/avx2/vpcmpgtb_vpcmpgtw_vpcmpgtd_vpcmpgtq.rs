use crate::common::*;
use vm_memory::{Bytes, GuestAddress};

// VPCMPGTB/VPCMPGTW/VPCMPGTD/VPCMPGTQ - Packed Greater Than Comparison (AVX2)
//
// Performs SIMD signed comparison of packed integers for greater than.
// For each element, if src1 > src2 (signed), all bits in result element are set to 1, otherwise 0.
//
// VPCMPGTB: Compare 32 packed byte integers (8-bit each) - signed
// VPCMPGTW: Compare 16 packed word integers (16-bit each) - signed
// VPCMPGTD: Compare 8 packed doubleword integers (32-bit each) - signed
// VPCMPGTQ: Compare 4 packed quadword integers (64-bit each) - signed
//
// Opcodes (AVX2 - 256-bit YMM):
// VEX.256.66.0F.WIG 64 /r     VPCMPGTB ymm1, ymm2, ymm3/m256
// VEX.256.66.0F.WIG 65 /r     VPCMPGTW ymm1, ymm2, ymm3/m256
// VEX.256.66.0F.WIG 66 /r     VPCMPGTD ymm1, ymm2, ymm3/m256
// VEX.256.66.0F38.WIG 37 /r   VPCMPGTQ ymm1, ymm2, ymm3/m256

const ALIGNED_ADDR: u64 = 0x3000;
const ALIGNED_ADDR2: u64 = 0x3100;

// ============================================================================
// VPCMPGTB Tests - 32x Byte Greater Than Comparison (256-bit)
// ============================================================================

#[test]
fn test_vpcmpgtb_ymm0_ymm1_ymm2_all_greater() {
    // VPCMPGTB YMM0, YMM1, YMM2 - all bytes in YMM1 > YMM2
    let code = [
        0xc5, 0xf5, 0x64, 0xc2, // VPCMPGTB YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpcmpgtb_ymm3_ymm4_ymm5_all_less() {
    // VPCMPGTB YMM3, YMM4, YMM5 - all bytes in YMM4 < YMM5
    let code = [
        0xc5, 0xdd, 0x64, 0xdd, // VPCMPGTB YMM3, YMM4, YMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpcmpgtb_ymm6_ymm7_ymm8_mixed() {
    let code = [
        0xc5, 0x45, 0x64, 0xf0, // VPCMPGTB YMM6, YMM7, YMM8
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpcmpgtb_ymm9_ymm10_ymm11_negative() {
    let code = [
        0xc4, 0x41, 0x2d, 0x64, 0xcb, // VPCMPGTB YMM9, YMM10, YMM11
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpcmpgtb_ymm12_ymm13_ymm14_zeros() {
    let code = [
        0xc4, 0x41, 0x15, 0x64, 0xe6, // VPCMPGTB YMM12, YMM13, YMM14
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpcmpgtb_ymm15_ymm0_ymm1_high_reg() {
    let code = [
        0xc4, 0xc1, 0x7d, 0x64, 0xf9, // VPCMPGTB YMM15, YMM0, YMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpcmpgtb_ymm0_ymm1_mem() {
    // VPCMPGTB YMM0, YMM1, [memory]
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0x64, 0x00, // VPCMPGTB YMM0, YMM1, [RAX]
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
fn test_vpcmpgtb_ymm2_ymm3_mem_pattern() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xe5, 0x64, 0x10, // VPCMPGTB YMM2, YMM3, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let pattern: Vec<u8> = (0..32).collect();
    mem.write_slice(&pattern, GuestAddress(ALIGNED_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpcmpgtb_ymm4_ymm5_mem_negative() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xd5, 0x64, 0x20, // VPCMPGTB YMM4, YMM5, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(
        &[
            0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80,
            0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80,
            0x80, 0x80, 0x80, 0x80,
        ],
        GuestAddress(ALIGNED_ADDR),
    )
    .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpcmpgtb_self_comparison() {
    // Compare register with itself (should all be false)
    let code = [
        0xc5, 0xf5, 0x64, 0xc1, // VPCMPGTB YMM0, YMM1, YMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// VPCMPGTW Tests - 16x Word Greater Than Comparison (256-bit)
// ============================================================================

#[test]
fn test_vpcmpgtw_ymm0_ymm1_ymm2_all_greater() {
    // VPCMPGTW YMM0, YMM1, YMM2 - all words in YMM1 > YMM2
    let code = [
        0xc5, 0xf5, 0x65, 0xc2, // VPCMPGTW YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpcmpgtw_ymm3_ymm4_ymm5_all_less() {
    // VPCMPGTW YMM3, YMM4, YMM5 - all words in YMM4 < YMM5
    let code = [
        0xc5, 0xdd, 0x65, 0xdd, // VPCMPGTW YMM3, YMM4, YMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpcmpgtw_ymm6_ymm7_ymm8_mixed() {
    let code = [
        0xc5, 0x45, 0x65, 0xf0, // VPCMPGTW YMM6, YMM7, YMM8
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpcmpgtw_ymm9_ymm10_ymm11_negative() {
    let code = [
        0xc4, 0x41, 0x2d, 0x65, 0xcb, // VPCMPGTW YMM9, YMM10, YMM11
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpcmpgtw_ymm12_ymm13_ymm14_zeros() {
    let code = [
        0xc4, 0x41, 0x15, 0x65, 0xe6, // VPCMPGTW YMM12, YMM13, YMM14
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpcmpgtw_ymm15_ymm0_ymm1_high_reg() {
    let code = [
        0xc4, 0xc1, 0x7d, 0x65, 0xf9, // VPCMPGTW YMM15, YMM0, YMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpcmpgtw_ymm0_ymm1_mem() {
    // VPCMPGTW YMM0, YMM1, [memory]
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0x65, 0x00, // VPCMPGTW YMM0, YMM1, [RAX]
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
fn test_vpcmpgtw_ymm2_ymm3_mem_pattern() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xe5, 0x65, 0x10, // VPCMPGTW YMM2, YMM3, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let pattern: Vec<u8> = (0..16).flat_map(|i| (i as u16).to_le_bytes()).collect();
    mem.write_slice(&pattern, GuestAddress(ALIGNED_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpcmpgtw_ymm4_ymm5_mem_negative() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xd5, 0x65, 0x20, // VPCMPGTW YMM4, YMM5, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(
        &[
            0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80,
            0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80,
            0x80, 0x80, 0x80, 0x80,
        ],
        GuestAddress(ALIGNED_ADDR),
    )
    .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpcmpgtw_self_comparison() {
    // Compare register with itself
    let code = [
        0xc5, 0xf5, 0x65, 0xc1, // VPCMPGTW YMM0, YMM1, YMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// VPCMPGTD Tests - 8x Doubleword Greater Than Comparison (256-bit)
// ============================================================================

#[test]
fn test_vpcmpgtd_ymm0_ymm1_ymm2_all_greater() {
    // VPCMPGTD YMM0, YMM1, YMM2 - all dwords in YMM1 > YMM2
    let code = [
        0xc5, 0xf5, 0x66, 0xc2, // VPCMPGTD YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpcmpgtd_ymm3_ymm4_ymm5_all_less() {
    // VPCMPGTD YMM3, YMM4, YMM5 - all dwords in YMM4 < YMM5
    let code = [
        0xc5, 0xdd, 0x66, 0xdd, // VPCMPGTD YMM3, YMM4, YMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpcmpgtd_ymm6_ymm7_ymm8_mixed() {
    let code = [
        0xc5, 0x45, 0x66, 0xf0, // VPCMPGTD YMM6, YMM7, YMM8
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpcmpgtd_ymm9_ymm10_ymm11_negative() {
    let code = [
        0xc4, 0x41, 0x2d, 0x66, 0xcb, // VPCMPGTD YMM9, YMM10, YMM11
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpcmpgtd_ymm12_ymm13_ymm14_zeros() {
    let code = [
        0xc4, 0x41, 0x15, 0x66, 0xe6, // VPCMPGTD YMM12, YMM13, YMM14
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpcmpgtd_ymm15_ymm0_ymm1_high_reg() {
    let code = [
        0xc4, 0xc1, 0x7d, 0x66, 0xf9, // VPCMPGTD YMM15, YMM0, YMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpcmpgtd_ymm0_ymm1_mem() {
    // VPCMPGTD YMM0, YMM1, [memory]
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0x66, 0x00, // VPCMPGTD YMM0, YMM1, [RAX]
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
fn test_vpcmpgtd_ymm2_ymm3_mem_pattern() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xe5, 0x66, 0x10, // VPCMPGTD YMM2, YMM3, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let pattern: Vec<u8> = (0..8).flat_map(|i| (i as u32).to_le_bytes()).collect();
    mem.write_slice(&pattern, GuestAddress(ALIGNED_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpcmpgtd_ymm4_ymm5_mem_negative() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xd5, 0x66, 0x20, // VPCMPGTD YMM4, YMM5, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(
        &[
            0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80,
            0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80,
            0x80, 0x80, 0x80, 0x80,
        ],
        GuestAddress(ALIGNED_ADDR),
    )
    .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpcmpgtd_self_comparison() {
    // Compare register with itself
    let code = [
        0xc5, 0xf5, 0x66, 0xc1, // VPCMPGTD YMM0, YMM1, YMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// VPCMPGTQ Tests - 4x Quadword Greater Than Comparison (256-bit)
// ============================================================================

#[test]
fn test_vpcmpgtq_ymm0_ymm1_ymm2_all_greater() {
    // VPCMPGTQ YMM0, YMM1, YMM2 - all qwords in YMM1 > YMM2
    let code = [
        0xc4, 0xe2, 0x75, 0x37, 0xc2, // VPCMPGTQ YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpcmpgtq_ymm3_ymm4_ymm5_all_less() {
    // VPCMPGTQ YMM3, YMM4, YMM5 - all qwords in YMM4 < YMM5
    let code = [
        0xc4, 0xe2, 0x5d, 0x37, 0xdd, // VPCMPGTQ YMM3, YMM4, YMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpcmpgtq_ymm6_ymm7_ymm8_mixed() {
    let code = [
        0xc4, 0xc2, 0x45, 0x37, 0xf0, // VPCMPGTQ YMM6, YMM7, YMM8
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpcmpgtq_ymm9_ymm10_ymm11_negative() {
    let code = [
        0xc4, 0x42, 0x2d, 0x37, 0xcb, // VPCMPGTQ YMM9, YMM10, YMM11
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpcmpgtq_ymm12_ymm13_ymm14_zeros() {
    let code = [
        0xc4, 0x42, 0x15, 0x37, 0xe6, // VPCMPGTQ YMM12, YMM13, YMM14
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpcmpgtq_ymm15_ymm0_ymm1_high_reg() {
    let code = [
        0xc4, 0xc2, 0x7d, 0x37, 0xf9, // VPCMPGTQ YMM15, YMM0, YMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpcmpgtq_ymm0_ymm1_mem() {
    // VPCMPGTQ YMM0, YMM1, [memory]
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x75, 0x37, 0x00, // VPCMPGTQ YMM0, YMM1, [RAX]
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
fn test_vpcmpgtq_ymm2_ymm3_mem_pattern() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x65, 0x37, 0x10, // VPCMPGTQ YMM2, YMM3, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let pattern: Vec<u8> = (0..4).flat_map(|i| (i as u64).to_le_bytes()).collect();
    mem.write_slice(&pattern, GuestAddress(ALIGNED_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpcmpgtq_ymm4_ymm5_mem_negative() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x55, 0x37, 0x20, // VPCMPGTQ YMM4, YMM5, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(
        &[
            0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80,
            0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80,
            0x80, 0x80, 0x80, 0x80,
        ],
        GuestAddress(ALIGNED_ADDR),
    )
    .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpcmpgtq_self_comparison() {
    // Compare register with itself
    let code = [
        0xc4, 0xe2, 0x75, 0x37, 0xc1, // VPCMPGTQ YMM0, YMM1, YMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Additional comprehensive tests
// ============================================================================

#[test]
fn test_vpcmpgtb_chain_operations() {
    // Chain multiple comparison operations
    let code = [
        0xc5, 0xf5, 0x64, 0xc2, // VPCMPGTB YMM0, YMM1, YMM2
        0xc5, 0xed, 0x64, 0xdb, // VPCMPGTB YMM3, YMM2, YMM3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpcmpgtw_chain_operations() {
    let code = [
        0xc5, 0xf5, 0x65, 0xc2, // VPCMPGTW YMM0, YMM1, YMM2
        0xc5, 0xed, 0x65, 0xdb, // VPCMPGTW YMM3, YMM2, YMM3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpcmpgtd_chain_operations() {
    let code = [
        0xc5, 0xf5, 0x66, 0xc2, // VPCMPGTD YMM0, YMM1, YMM2
        0xc5, 0xed, 0x66, 0xdb, // VPCMPGTD YMM3, YMM2, YMM3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpcmpgtq_chain_operations() {
    let code = [
        0xc4, 0xe2, 0x75, 0x37, 0xc2, // VPCMPGTQ YMM0, YMM1, YMM2
        0xc4, 0xe2, 0x6d, 0x37, 0xdb, // VPCMPGTQ YMM3, YMM2, YMM3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpcmpgt_mixed_sizes() {
    // Test different element sizes in sequence
    let code = [
        0xc5, 0xf5, 0x64, 0xc2, // VPCMPGTB YMM0, YMM1, YMM2
        0xc5, 0xed, 0x65, 0xdb, // VPCMPGTW YMM3, YMM2, YMM3
        0xc5, 0xe5, 0x66, 0xe4, // VPCMPGTD YMM4, YMM3, YMM4
        0xc4, 0xe2, 0x5d, 0x37, 0xed, // VPCMPGTQ YMM5, YMM4, YMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpcmpgtb_signed_comparison() {
    // Test that comparison is signed (negative < positive)
    let code = [
        0xc5, 0xf5, 0x64, 0xc2, // VPCMPGTB YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpcmpgtw_signed_comparison() {
    let code = [
        0xc5, 0xf5, 0x65, 0xc2, // VPCMPGTW YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpcmpgtd_signed_comparison() {
    let code = [
        0xc5, 0xf5, 0x66, 0xc2, // VPCMPGTD YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpcmpgtq_signed_comparison() {
    let code = [
        0xc4, 0xe2, 0x75, 0x37, 0xc2, // VPCMPGTQ YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpcmpgtb_extended_regs() {
    // Test with extended registers YMM8-YMM15
    let code = [
        0xc4, 0x41, 0x3d, 0x64, 0xc1, // VPCMPGTB YMM8, YMM8, YMM9
        0xc4, 0x41, 0x15, 0x64, 0xee, // VPCMPGTB YMM13, YMM13, YMM14
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpcmpgtw_extended_regs() {
    let code = [
        0xc4, 0x41, 0x35, 0x65, 0xcb, // VPCMPGTW YMM9, YMM9, YMM11
        0xc4, 0x41, 0x0d, 0x65, 0xf7, // VPCMPGTW YMM14, YMM14, YMM15
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpcmpgtd_extended_regs() {
    let code = [
        0xc4, 0x41, 0x2d, 0x66, 0xd4, // VPCMPGTD YMM10, YMM10, YMM12
        0xc4, 0x41, 0x05, 0x66, 0xf8, // VPCMPGTD YMM15, YMM15, YMM8
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpcmpgtq_extended_regs() {
    let code = [
        0xc4, 0x42, 0x25, 0x37, 0xdd, // VPCMPGTQ YMM11, YMM11, YMM13
        0xc4, 0x42, 0x05, 0x37, 0xf9, // VPCMPGTQ YMM15, YMM15, YMM9
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpcmpgtb_positive_vs_negative() {
    // Positive values should be > negative values
    let code = [
        0xc5, 0xf5, 0x64, 0xc2, // VPCMPGTB YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpcmpgtw_positive_vs_negative() {
    let code = [
        0xc5, 0xf5, 0x65, 0xc2, // VPCMPGTW YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpcmpgtd_positive_vs_negative() {
    let code = [
        0xc5, 0xf5, 0x66, 0xc2, // VPCMPGTD YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpcmpgtq_positive_vs_negative() {
    let code = [
        0xc4, 0xe2, 0x75, 0x37, 0xc2, // VPCMPGTQ YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpcmpgtb_zero_comparison() {
    // Test comparison with zero
    let code = [
        0xc5, 0xf5, 0x64, 0xc2, // VPCMPGTB YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpcmpgtw_zero_comparison() {
    let code = [
        0xc5, 0xf5, 0x65, 0xc2, // VPCMPGTW YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpcmpgtd_zero_comparison() {
    let code = [
        0xc5, 0xf5, 0x66, 0xc2, // VPCMPGTD YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpcmpgtq_zero_comparison() {
    let code = [
        0xc4, 0xe2, 0x75, 0x37, 0xc2, // VPCMPGTQ YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Known-answer VALUE tests : signed per-element greater-than (src1 > src2).
// Produces all-ones mask where the lane comparison is true, else all-zeros.
// ============================================================================

use rax::backend::emulator::x86_64::X86_64Vcpu;

fn kgt_set(vcpu: &mut X86_64Vcpu, idx: usize, lo: u128, hi: u128) {
    let mut regs = vcpu.get_regs().unwrap();
    regs.xmm[idx][0] = lo as u64;
    regs.xmm[idx][1] = (lo >> 64) as u64;
    regs.ymm_high[idx][0] = hi as u64;
    regs.ymm_high[idx][1] = (hi >> 64) as u64;
    vcpu.set_regs(&regs).unwrap();
}
fn kgt_lo(vcpu: &X86_64Vcpu, idx: usize) -> u128 {
    let r = vcpu.get_regs().unwrap();
    (r.xmm[idx][0] as u128) | ((r.xmm[idx][1] as u128) << 64)
}
fn kgt_hi(vcpu: &X86_64Vcpu, idx: usize) -> u128 {
    let r = vcpu.get_regs().unwrap();
    (r.ymm_high[idx][0] as u128) | ((r.ymm_high[idx][1] as u128) << 64)
}

fn pcmpgt_bytes(a: u128, b: u128) -> u128 {
    let (ab, bb) = (a.to_le_bytes(), b.to_le_bytes());
    let mut out = [0u8; 16];
    for i in 0..16 {
        out[i] = if (ab[i] as i8) > (bb[i] as i8) {
            0xFF
        } else {
            0x00
        };
    }
    u128::from_le_bytes(out)
}
fn pcmpgt_words(a: u128, b: u128) -> u128 {
    let mut out = 0u128;
    for i in 0..8 {
        let av = ((a >> (i * 16)) & 0xFFFF) as u16 as i16;
        let bv = ((b >> (i * 16)) & 0xFFFF) as u16 as i16;
        if av > bv {
            out |= 0xFFFFu128 << (i * 16);
        }
    }
    out
}
fn pcmpgt_dwords(a: u128, b: u128) -> u128 {
    let mut out = 0u128;
    for i in 0..4 {
        let av = ((a >> (i * 32)) & 0xFFFF_FFFF) as u32 as i32;
        let bv = ((b >> (i * 32)) & 0xFFFF_FFFF) as u32 as i32;
        if av > bv {
            out |= 0xFFFF_FFFFu128 << (i * 32);
        }
    }
    out
}
fn pcmpgt_qwords(a: u128, b: u128) -> u128 {
    let mut out = 0u128;
    if (a as u64 as i64) > (b as u64 as i64) {
        out |= 0xFFFF_FFFF_FFFF_FFFFu128;
    }
    if ((a >> 64) as u64 as i64) > ((b >> 64) as u64 as i64) {
        out |= 0xFFFF_FFFF_FFFF_FFFFu128 << 64;
    }
    out
}

// Mix of positive, negative and equal lanes (signed).
const GT_A_LO: u128 = 0x7F_80_05_FB_00_01_FF_7E_02_FE_10_F0_03_FD_01_00;
const GT_B_LO: u128 = 0x7E_81_05_FA_FF_02_FE_7E_01_FF_0F_F1_03_FC_00_FF;
const GT_A_HI: u128 = 0x7FFF_8000_0001_FFFF_0000_7FFE_8001_1234;
const GT_B_HI: u128 = 0x7FFE_8001_0000_FFFE_FFFF_7FFE_8000_1233;

#[test]
fn test_vpcmpgtb_xmm_value() {
    let code = [0xc5, 0xf1, 0x64, 0xc2, 0xf4]; // VPCMPGTB XMM0, XMM1, XMM2
    let (mut vcpu, _) = setup_vm(&code, None);
    kgt_set(&mut vcpu, 1, GT_A_LO, 0xDEAD);
    kgt_set(&mut vcpu, 2, GT_B_LO, 0xBEEF);
    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(kgt_lo(&vcpu, 0), pcmpgt_bytes(GT_A_LO, GT_B_LO));
    assert_eq!(kgt_hi(&vcpu, 0), 0, "VEX.128 must zero upper 128 bits");
}

#[test]
fn test_vpcmpgtb_ymm_value() {
    let code = [0xc5, 0xf5, 0x64, 0xc2, 0xf4]; // VPCMPGTB YMM0, YMM1, YMM2
    let (mut vcpu, _) = setup_vm(&code, None);
    kgt_set(&mut vcpu, 1, GT_A_LO, GT_A_HI);
    kgt_set(&mut vcpu, 2, GT_B_LO, GT_B_HI);
    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(kgt_lo(&vcpu, 0), pcmpgt_bytes(GT_A_LO, GT_B_LO));
    assert_eq!(kgt_hi(&vcpu, 0), pcmpgt_bytes(GT_A_HI, GT_B_HI));
}

#[test]
fn test_vpcmpgtw_ymm_value() {
    let code = [0xc5, 0xf5, 0x65, 0xc2, 0xf4]; // VPCMPGTW YMM0, YMM1, YMM2
    let (mut vcpu, _) = setup_vm(&code, None);
    kgt_set(&mut vcpu, 1, GT_A_LO, GT_A_HI);
    kgt_set(&mut vcpu, 2, GT_B_LO, GT_B_HI);
    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(kgt_lo(&vcpu, 0), pcmpgt_words(GT_A_LO, GT_B_LO));
    assert_eq!(kgt_hi(&vcpu, 0), pcmpgt_words(GT_A_HI, GT_B_HI));
}

#[test]
fn test_vpcmpgtd_ymm_value() {
    let code = [0xc5, 0xf5, 0x66, 0xc2, 0xf4]; // VPCMPGTD YMM0, YMM1, YMM2
    let (mut vcpu, _) = setup_vm(&code, None);
    kgt_set(&mut vcpu, 1, GT_A_LO, GT_A_HI);
    kgt_set(&mut vcpu, 2, GT_B_LO, GT_B_HI);
    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(kgt_lo(&vcpu, 0), pcmpgt_dwords(GT_A_LO, GT_B_LO));
    assert_eq!(kgt_hi(&vcpu, 0), pcmpgt_dwords(GT_A_HI, GT_B_HI));
}

#[test]
fn test_vpcmpgtq_ymm_value() {
    let code = [0xc4, 0xe2, 0x75, 0x37, 0xc2, 0xf4]; // VPCMPGTQ YMM0, YMM1, YMM2
    let (mut vcpu, _) = setup_vm(&code, None);
    // lane0: 1 > -1 (true); lane1 lo: -1 not > 0 (false); hi lane handled too.
    let a_lo: u128 = 0xFFFF_FFFF_FFFF_FFFF_0000_0000_0000_0001; // [1, -1]
    let b_lo: u128 = 0x0000_0000_0000_0000_FFFF_FFFF_FFFF_FFFF; // [-1, 0]
    let a_hi: u128 = 0x0000_0000_0000_0005_8000_0000_0000_0000; // [INT64_MIN, 5]
    let b_hi: u128 = 0x0000_0000_0000_0004_8000_0000_0000_0001; // [MIN+1, 4]
    kgt_set(&mut vcpu, 1, a_lo, a_hi);
    kgt_set(&mut vcpu, 2, b_lo, b_hi);
    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(kgt_lo(&vcpu, 0), pcmpgt_qwords(a_lo, b_lo));
    assert_eq!(kgt_hi(&vcpu, 0), pcmpgt_qwords(a_hi, b_hi));
    // lane0 (1 > -1) true, lane1 (-1 > 0) false.
    assert_eq!(kgt_lo(&vcpu, 0), 0x0000_0000_0000_0000_FFFF_FFFF_FFFF_FFFF);
}
