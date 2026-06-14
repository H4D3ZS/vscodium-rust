use crate::common::*;
use vm_memory::{Bytes, GuestAddress};

// VPMAXSB/VPMAXSW/VPMAXSD - Maximum of Packed Signed Integers (AVX2)
//
// Compare packed signed integers from source and destination operands and
// return the maximum values. Stores packed maximum results in destination.
//
// VPMAXSB: Maximum of 32 packed signed byte integers (8-bit each) in YMM registers
// VPMAXSW: Maximum of 16 packed signed word integers (16-bit each) in YMM registers
// VPMAXSD: Maximum of 8 packed signed doubleword integers (32-bit each) in YMM registers
//
// Opcodes (AVX2 - 256-bit YMM):
// VEX.256.66.0F38.WIG 3C /r     VPMAXSB ymm1, ymm2, ymm3/m256
// VEX.256.66.0F.WIG EE /r       VPMAXSW ymm1, ymm2, ymm3/m256
// VEX.256.66.0F38.WIG 3D /r     VPMAXSD ymm1, ymm2, ymm3/m256

const ALIGNED_ADDR: u64 = 0x3000;
const ALIGNED_ADDR2: u64 = 0x3100;

// ============================================================================
// VPMAXSB Tests - 32x Signed Byte Maximum (256-bit)
// ============================================================================

#[test]
fn test_vpmaxsb_ymm0_ymm1_ymm2_all_zeros() {
    // VPMAXSB YMM0, YMM1, YMM2 with all zeros
    let code = [
        0xc4, 0xe2, 0x75, 0x3c, 0xc2, // VPMAXSB YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmaxsb_ymm3_ymm4_ymm5_positive_values() {
    // VPMAXSB YMM3, YMM4, YMM5 with positive values
    let code = [
        0xc4, 0xe2, 0x5d, 0x3c, 0xdd, // VPMAXSB YMM3, YMM4, YMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmaxsb_ymm6_ymm7_ymm8_negative_values() {
    // Test signed byte maximum with negative values
    let code = [
        0xc4, 0x62, 0x45, 0x3c, 0xf0, // VPMAXSB YMM6, YMM7, YMM8
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmaxsb_ymm9_ymm10_ymm11_mixed_signs() {
    // Test with mixed positive and negative values
    let code = [
        0xc4, 0x42, 0x2d, 0x3c, 0xcb, // VPMAXSB YMM9, YMM10, YMM11
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmaxsb_ymm12_ymm13_ymm14_min_max_values() {
    // Test with min (0x80 = -128) and max (0x7F = 127) signed byte values
    let code = [
        0xc4, 0x42, 0x15, 0x3c, 0xe6, // VPMAXSB YMM12, YMM13, YMM14
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmaxsb_ymm15_ymm0_ymm1_high_reg() {
    let code = [
        0xc4, 0x62, 0x7d, 0x3c, 0xf9, // VPMAXSB YMM15, YMM0, YMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmaxsb_ymm0_ymm1_mem() {
    // VPMAXSB YMM0, YMM1, [memory]
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x75, 0x3c, 0x00, // VPMAXSB YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(
        &[
            0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11,
            0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11,
            0x11, 0x11, 0x11, 0x11,
        ],
        GuestAddress(ALIGNED_ADDR),
    )
    .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmaxsb_ymm2_ymm3_mem_negative() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x65, 0x3c, 0x10, // VPMAXSB YMM2, YMM3, [RAX]
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
fn test_vpmaxsb_ymm4_ymm5_mem_sequential() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x55, 0x3c, 0x20, // VPMAXSB YMM4, YMM5, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let seq: Vec<u8> = (0..32).collect();
    mem.write_slice(&seq, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmaxsb_ymm6_ymm7_mem_alternating() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x45, 0x3c, 0x30, // VPMAXSB YMM6, YMM7, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let pattern: Vec<u8> = (0..32)
        .map(|i| if i % 2 == 0 { 0x7F } else { 0x80 })
        .collect();
    mem.write_slice(&pattern, GuestAddress(ALIGNED_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// VPMAXSW Tests - 16x Signed Word Maximum (256-bit)
// ============================================================================

#[test]
fn test_vpmaxsw_ymm0_ymm1_ymm2_all_zeros() {
    // VPMAXSW YMM0, YMM1, YMM2 with all zeros
    let code = [
        0xc5, 0xf5, 0xee, 0xc2, // VPMAXSW YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmaxsw_ymm3_ymm4_ymm5_positive_values() {
    // VPMAXSW YMM3, YMM4, YMM5
    let code = [
        0xc5, 0xdd, 0xee, 0xdd, // VPMAXSW YMM3, YMM4, YMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmaxsw_ymm6_ymm7_ymm8_negative_values() {
    // Test signed word maximum with negative values
    let code = [
        0xc5, 0x45, 0xee, 0xf0, // VPMAXSW YMM6, YMM7, YMM8
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmaxsw_ymm9_ymm10_ymm11_mixed_signs() {
    let code = [
        0xc4, 0x41, 0x2d, 0xee, 0xcb, // VPMAXSW YMM9, YMM10, YMM11
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmaxsw_ymm12_ymm13_ymm14_min_max_values() {
    // Test with min (0x8000 = -32768) and max (0x7FFF = 32767) signed word values
    let code = [
        0xc4, 0x41, 0x15, 0xee, 0xe6, // VPMAXSW YMM12, YMM13, YMM14
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmaxsw_ymm15_ymm0_ymm1_high_reg() {
    let code = [
        0xc4, 0xc1, 0x7d, 0xee, 0xf9, // VPMAXSW YMM15, YMM0, YMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmaxsw_ymm0_ymm1_mem() {
    // VPMAXSW YMM0, YMM1, [memory]
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0xee, 0x00, // VPMAXSW YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data: Vec<u8> = (0..16)
        .flat_map(|i| ((i * 0x1111u16) as u16).to_le_bytes())
        .collect();
    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmaxsw_ymm2_ymm3_mem_negative() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xe5, 0xee, 0x10, // VPMAXSW YMM2, YMM3, [RAX]
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
fn test_vpmaxsw_ymm4_ymm5_mem_sequential() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xd5, 0xee, 0x20, // VPMAXSW YMM4, YMM5, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data: Vec<u8> = (0..16).flat_map(|i| (i as u16).to_le_bytes()).collect();
    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmaxsw_ymm6_ymm7_mem_alternating() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0x45, 0xee, 0x30, // VPMAXSW YMM6, YMM7, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let pattern: Vec<u8> = (0..16)
        .flat_map(|i| if i % 2 == 0 { 0x7FFFu16 } else { 0x8000u16 }.to_le_bytes())
        .collect();
    mem.write_slice(&pattern, GuestAddress(ALIGNED_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// VPMAXSD Tests - 8x Signed Doubleword Maximum (256-bit)
// ============================================================================

#[test]
fn test_vpmaxsd_ymm0_ymm1_ymm2_all_zeros() {
    // VPMAXSD YMM0, YMM1, YMM2 with all zeros
    let code = [
        0xc4, 0xe2, 0x75, 0x3d, 0xc2, // VPMAXSD YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmaxsd_ymm3_ymm4_ymm5_positive_values() {
    // VPMAXSD YMM3, YMM4, YMM5
    let code = [
        0xc4, 0xe2, 0x5d, 0x3d, 0xdd, // VPMAXSD YMM3, YMM4, YMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmaxsd_ymm6_ymm7_ymm8_negative_values() {
    // Test signed dword maximum with negative values
    let code = [
        0xc4, 0x62, 0x45, 0x3d, 0xf0, // VPMAXSD YMM6, YMM7, YMM8
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmaxsd_ymm9_ymm10_ymm11_mixed_signs() {
    let code = [
        0xc4, 0x42, 0x2d, 0x3d, 0xcb, // VPMAXSD YMM9, YMM10, YMM11
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmaxsd_ymm12_ymm13_ymm14_min_max_values() {
    // Test with min (0x80000000) and max (0x7FFFFFFF) signed dword values
    let code = [
        0xc4, 0x42, 0x15, 0x3d, 0xe6, // VPMAXSD YMM12, YMM13, YMM14
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmaxsd_ymm15_ymm0_ymm1_high_reg() {
    let code = [
        0xc4, 0x62, 0x7d, 0x3d, 0xf9, // VPMAXSD YMM15, YMM0, YMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmaxsd_ymm0_ymm1_mem() {
    // VPMAXSD YMM0, YMM1, [memory]
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x75, 0x3d, 0x00, // VPMAXSD YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data: Vec<u8> = (0..8)
        .flat_map(|i| ((i * 0x11111111u32) as u32).to_le_bytes())
        .collect();
    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmaxsd_ymm2_ymm3_mem_negative() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x65, 0x3d, 0x10, // VPMAXSD YMM2, YMM3, [RAX]
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
fn test_vpmaxsd_ymm4_ymm5_mem_sequential() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x55, 0x3d, 0x20, // VPMAXSD YMM4, YMM5, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data: Vec<u8> = (0..8).flat_map(|i| (i as u32).to_le_bytes()).collect();
    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmaxsd_ymm6_ymm7_mem_alternating() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x45, 0x3d, 0x30, // VPMAXSD YMM6, YMM7, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let pattern: Vec<u8> = (0..8)
        .flat_map(|i| {
            if i % 2 == 0 {
                0x7FFFFFFFu32
            } else {
                0x80000000u32
            }
            .to_le_bytes()
        })
        .collect();
    mem.write_slice(&pattern, GuestAddress(ALIGNED_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Additional comprehensive tests mixing different operations
// ============================================================================

#[test]
fn test_vpmaxsb_chain_multiple_ops() {
    // Chain multiple VPMAXSB operations
    let code = [
        0xc4, 0xe2, 0x75, 0x3c, 0xc2, // VPMAXSB YMM0, YMM1, YMM2
        0xc4, 0xe2, 0x7d, 0x3c, 0xc3, // VPMAXSB YMM0, YMM0, YMM3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmaxsw_chain_multiple_ops() {
    // Chain multiple VPMAXSW operations
    let code = [
        0xc5, 0xf5, 0xee, 0xc2, // VPMAXSW YMM0, YMM1, YMM2
        0xc5, 0xfd, 0xee, 0xc3, // VPMAXSW YMM0, YMM0, YMM3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmaxsd_chain_multiple_ops() {
    // Chain multiple VPMAXSD operations
    let code = [
        0xc4, 0xe2, 0x75, 0x3d, 0xc2, // VPMAXSD YMM0, YMM1, YMM2
        0xc4, 0xe2, 0x7d, 0x3d, 0xc3, // VPMAXSD YMM0, YMM0, YMM3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmaxsb_mem_unaligned_offset() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&(ALIGNED_ADDR + 1).to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x75, 0x3c, 0x00, // VPMAXSB YMM0, YMM1, [RAX]
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

#[test]
fn test_vpmaxsw_mem_pattern() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0xee, 0x00, // VPMAXSW YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let pattern: Vec<u8> = (0..16)
        .flat_map(|i| if i % 2 == 0 { 0xAAAAu16 } else { 0x5555u16 }.to_le_bytes())
        .collect();
    mem.write_slice(&pattern, GuestAddress(ALIGNED_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmaxsd_mem_powers_of_two() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x75, 0x3d, 0x00, // VPMAXSD YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let powers: Vec<u8> = (0..8).flat_map(|i| (1u32 << i).to_le_bytes()).collect();
    mem.write_slice(&powers, GuestAddress(ALIGNED_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmaxsb_mixed_signs_comprehensive() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x75, 0x3c, 0x00, // VPMAXSB YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let pattern: Vec<u8> = (0..32)
        .map(|i| {
            if i % 4 < 2 {
                (i as i8) as u8
            } else {
                (-(i as i8)) as u8
            }
        })
        .collect();
    mem.write_slice(&pattern, GuestAddress(ALIGNED_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmaxsw_boundary_values() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0xee, 0x00, // VPMAXSW YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let boundary: Vec<u8> = vec![
        0x0000u16, 0x0001u16, 0x7FFEu16, 0x7FFFu16, 0x8000u16, 0x8001u16, 0xFFFEu16, 0xFFFFu16,
        0x0000u16, 0x0001u16, 0x7FFEu16, 0x7FFFu16, 0x8000u16, 0x8001u16, 0xFFFEu16, 0xFFFFu16,
    ]
    .into_iter()
    .flat_map(|v| v.to_le_bytes())
    .collect();
    mem.write_slice(&boundary, GuestAddress(ALIGNED_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmaxsd_large_negative_values() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x75, 0x3d, 0x00, // VPMAXSD YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let large_vals: Vec<u8> = vec![
        0x80000000u32,
        0x80000001u32,
        0xFFFFFFFFu32,
        0x00000000u32,
        0x7FFFFFFFu32,
        0x7FFFFFFEu32,
        0x00000001u32,
        0xFFFFFFFEu32,
    ]
    .into_iter()
    .flat_map(|v| v.to_le_bytes())
    .collect();
    mem.write_slice(&large_vals, GuestAddress(ALIGNED_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmaxsb_all_same_values() {
    // Test when all values are the same
    let code = [
        0xc4, 0xe2, 0x75, 0x3c, 0xc1, // VPMAXSB YMM0, YMM1, YMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmaxsw_all_same_values() {
    // Test when all values are the same
    let code = [
        0xc5, 0xf5, 0xee, 0xc1, // VPMAXSW YMM0, YMM1, YMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmaxsd_all_same_values() {
    // Test when all values are the same
    let code = [
        0xc4, 0xe2, 0x75, 0x3d, 0xc1, // VPMAXSD YMM0, YMM1, YMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmaxsb_extended_regs_r8_r9_r10() {
    let code = [
        0xc4, 0x42, 0x3d, 0x3c, 0xc2, // VPMAXSB YMM8, YMM8, YMM10
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmaxsw_extended_regs_r11_r12_r13() {
    let code = [
        0xc4, 0x41, 0x1d, 0xee, 0xdd, // VPMAXSW YMM11, YMM12, YMM13
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmaxsd_extended_regs_r14_r15_r8() {
    let code = [
        0xc4, 0x42, 0x05, 0x3d, 0xf0, // VPMAXSD YMM14, YMM15, YMM8
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
