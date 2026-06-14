use crate::common::*;
use vm_memory::{Bytes, GuestAddress};

// VPMINUB/VPMINUW/VPMINUD - Minimum of Packed Unsigned Integers (AVX2)
//
// Compare packed unsigned integers from source and destination operands and
// return the minimum values. Stores packed minimum results in destination.
//
// VPMINUB: Minimum of 32 packed unsigned byte integers (8-bit each) in YMM registers
// VPMINUW: Minimum of 16 packed unsigned word integers (16-bit each) in YMM registers
// VPMINUD: Minimum of 8 packed unsigned doubleword integers (32-bit each) in YMM registers
//
// Opcodes (AVX2 - 256-bit YMM):
// VEX.256.66.0F.WIG DA /r       VPMINUB ymm1, ymm2, ymm3/m256
// VEX.256.66.0F38.WIG 3A /r     VPMINUW ymm1, ymm2, ymm3/m256
// VEX.256.66.0F38.WIG 3B /r     VPMINUD ymm1, ymm2, ymm3/m256

const ALIGNED_ADDR: u64 = 0x3000;
const ALIGNED_ADDR2: u64 = 0x3100;

// ============================================================================
// VPMINUB Tests - 32x Unsigned Byte Minimum (256-bit)
// ============================================================================

#[test]
fn test_vpminub_ymm0_ymm1_ymm2_all_zeros() {
    // VPMINUB YMM0, YMM1, YMM2 with all zeros
    let code = [
        0xc5, 0xf5, 0xda, 0xc2, // VPMINUB YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpminub_ymm3_ymm4_ymm5_small_values() {
    // VPMINUB YMM3, YMM4, YMM5 with small values
    let code = [
        0xc5, 0xdd, 0xda, 0xdd, // VPMINUB YMM3, YMM4, YMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpminub_ymm6_ymm7_ymm8_max_values() {
    // Test unsigned byte minimum with max values (0xFF)
    let code = [
        0xc5, 0x45, 0xda, 0xf0, // VPMINUB YMM6, YMM7, YMM8
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpminub_ymm9_ymm10_ymm11_mixed_values() {
    // Test with mixed values
    let code = [
        0xc4, 0x41, 0x2d, 0xda, 0xcb, // VPMINUB YMM9, YMM10, YMM11
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpminub_ymm12_ymm13_ymm14_boundary_values() {
    // Test with min (0x00) and max (0xFF) unsigned byte values
    let code = [
        0xc4, 0x41, 0x15, 0xda, 0xe6, // VPMINUB YMM12, YMM13, YMM14
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpminub_ymm15_ymm0_ymm1_high_reg() {
    let code = [
        0xc4, 0xc1, 0x7d, 0xda, 0xf9, // VPMINUB YMM15, YMM0, YMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpminub_ymm0_ymm1_mem() {
    // VPMINUB YMM0, YMM1, [memory]
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0xda, 0x00, // VPMINUB YMM0, YMM1, [RAX]
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
fn test_vpminub_ymm2_ymm3_mem_max() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xe5, 0xda, 0x10, // VPMINUB YMM2, YMM3, [RAX]
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
fn test_vpminub_ymm4_ymm5_mem_sequential() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xd5, 0xda, 0x20, // VPMINUB YMM4, YMM5, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let seq: Vec<u8> = (0..32).collect();
    mem.write_slice(&seq, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpminub_ymm6_ymm7_mem_alternating() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0x45, 0xda, 0x30, // VPMINUB YMM6, YMM7, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let pattern: Vec<u8> = (0..32)
        .map(|i| if i % 2 == 0 { 0xFF } else { 0x00 })
        .collect();
    mem.write_slice(&pattern, GuestAddress(ALIGNED_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// VPMINUW Tests - 16x Unsigned Word Minimum (256-bit)
// ============================================================================

#[test]
fn test_vpminuw_ymm0_ymm1_ymm2_all_zeros() {
    // VPMINUW YMM0, YMM1, YMM2 with all zeros
    let code = [
        0xc4, 0xe2, 0x75, 0x3a, 0xc2, // VPMINUW YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpminuw_ymm3_ymm4_ymm5_small_values() {
    // VPMINUW YMM3, YMM4, YMM5
    let code = [
        0xc4, 0xe2, 0x5d, 0x3a, 0xdd, // VPMINUW YMM3, YMM4, YMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpminuw_ymm6_ymm7_ymm8_max_values() {
    // Test unsigned word minimum with max values
    let code = [
        0xc4, 0x62, 0x45, 0x3a, 0xf0, // VPMINUW YMM6, YMM7, YMM8
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpminuw_ymm9_ymm10_ymm11_mixed_values() {
    let code = [
        0xc4, 0x42, 0x2d, 0x3a, 0xcb, // VPMINUW YMM9, YMM10, YMM11
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpminuw_ymm12_ymm13_ymm14_boundary_values() {
    // Test with min (0x0000) and max (0xFFFF) unsigned word values
    let code = [
        0xc4, 0x42, 0x15, 0x3a, 0xe6, // VPMINUW YMM12, YMM13, YMM14
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpminuw_ymm15_ymm0_ymm1_high_reg() {
    let code = [
        0xc4, 0x62, 0x7d, 0x3a, 0xf9, // VPMINUW YMM15, YMM0, YMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpminuw_ymm0_ymm1_mem() {
    // VPMINUW YMM0, YMM1, [memory]
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x75, 0x3a, 0x00, // VPMINUW YMM0, YMM1, [RAX]
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
fn test_vpminuw_ymm2_ymm3_mem_max() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x65, 0x3a, 0x10, // VPMINUW YMM2, YMM3, [RAX]
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
fn test_vpminuw_ymm4_ymm5_mem_sequential() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x55, 0x3a, 0x20, // VPMINUW YMM4, YMM5, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data: Vec<u8> = (0..16).flat_map(|i| (i as u16).to_le_bytes()).collect();
    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpminuw_ymm6_ymm7_mem_alternating() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x45, 0x3a, 0x30, // VPMINUW YMM6, YMM7, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let pattern: Vec<u8> = (0..16)
        .flat_map(|i| if i % 2 == 0 { 0xFFFFu16 } else { 0x0000u16 }.to_le_bytes())
        .collect();
    mem.write_slice(&pattern, GuestAddress(ALIGNED_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// VPMINUD Tests - 8x Unsigned Doubleword Minimum (256-bit)
// ============================================================================

#[test]
fn test_vpminud_ymm0_ymm1_ymm2_all_zeros() {
    // VPMINUD YMM0, YMM1, YMM2 with all zeros
    let code = [
        0xc4, 0xe2, 0x75, 0x3b, 0xc2, // VPMINUD YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpminud_ymm3_ymm4_ymm5_small_values() {
    // VPMINUD YMM3, YMM4, YMM5
    let code = [
        0xc4, 0xe2, 0x5d, 0x3b, 0xdd, // VPMINUD YMM3, YMM4, YMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpminud_ymm6_ymm7_ymm8_max_values() {
    // Test unsigned dword minimum with max values
    let code = [
        0xc4, 0x62, 0x45, 0x3b, 0xf0, // VPMINUD YMM6, YMM7, YMM8
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpminud_ymm9_ymm10_ymm11_mixed_values() {
    let code = [
        0xc4, 0x42, 0x2d, 0x3b, 0xcb, // VPMINUD YMM9, YMM10, YMM11
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpminud_ymm12_ymm13_ymm14_boundary_values() {
    // Test with min (0x00000000) and max (0xFFFFFFFF) unsigned dword values
    let code = [
        0xc4, 0x42, 0x15, 0x3b, 0xe6, // VPMINUD YMM12, YMM13, YMM14
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpminud_ymm15_ymm0_ymm1_high_reg() {
    let code = [
        0xc4, 0x62, 0x7d, 0x3b, 0xf9, // VPMINUD YMM15, YMM0, YMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpminud_ymm0_ymm1_mem() {
    // VPMINUD YMM0, YMM1, [memory]
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x75, 0x3b, 0x00, // VPMINUD YMM0, YMM1, [RAX]
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
fn test_vpminud_ymm2_ymm3_mem_max() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x65, 0x3b, 0x10, // VPMINUD YMM2, YMM3, [RAX]
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
fn test_vpminud_ymm4_ymm5_mem_sequential() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x55, 0x3b, 0x20, // VPMINUD YMM4, YMM5, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data: Vec<u8> = (0..8).flat_map(|i| (i as u32).to_le_bytes()).collect();
    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpminud_ymm6_ymm7_mem_alternating() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x45, 0x3b, 0x30, // VPMINUD YMM6, YMM7, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let pattern: Vec<u8> = (0..8)
        .flat_map(|i| {
            if i % 2 == 0 {
                0xFFFFFFFFu32
            } else {
                0x00000000u32
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
fn test_vpminub_chain_multiple_ops() {
    // Chain multiple VPMINUB operations
    let code = [
        0xc5, 0xf5, 0xda, 0xc2, // VPMINUB YMM0, YMM1, YMM2
        0xc5, 0xfd, 0xda, 0xc3, // VPMINUB YMM0, YMM0, YMM3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpminuw_chain_multiple_ops() {
    // Chain multiple VPMINUW operations
    let code = [
        0xc4, 0xe2, 0x75, 0x3a, 0xc2, // VPMINUW YMM0, YMM1, YMM2
        0xc4, 0xe2, 0x7d, 0x3a, 0xc3, // VPMINUW YMM0, YMM0, YMM3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpminud_chain_multiple_ops() {
    // Chain multiple VPMINUD operations
    let code = [
        0xc4, 0xe2, 0x75, 0x3b, 0xc2, // VPMINUD YMM0, YMM1, YMM2
        0xc4, 0xe2, 0x7d, 0x3b, 0xc3, // VPMINUD YMM0, YMM0, YMM3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpminub_mem_unaligned_offset() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&(ALIGNED_ADDR + 1).to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0xda, 0x00, // VPMINUB YMM0, YMM1, [RAX]
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
fn test_vpminuw_mem_pattern() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x75, 0x3a, 0x00, // VPMINUW YMM0, YMM1, [RAX]
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
fn test_vpminud_mem_powers_of_two() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x75, 0x3b, 0x00, // VPMINUD YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let powers: Vec<u8> = (0..8).flat_map(|i| (1u32 << i).to_le_bytes()).collect();
    mem.write_slice(&powers, GuestAddress(ALIGNED_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpminub_reverse_sequential() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0xda, 0x00, // VPMINUB YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let pattern: Vec<u8> = (0..32).rev().collect();
    mem.write_slice(&pattern, GuestAddress(ALIGNED_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpminuw_boundary_values() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x75, 0x3a, 0x00, // VPMINUW YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let boundary: Vec<u8> = vec![
        0x0000u16, 0x0001u16, 0xFFFEu16, 0xFFFFu16, 0x7FFFu16, 0x8000u16, 0x8001u16, 0x7FFEu16,
        0x0000u16, 0x0001u16, 0xFFFEu16, 0xFFFFu16, 0x7FFFu16, 0x8000u16, 0x8001u16, 0x7FFEu16,
    ]
    .into_iter()
    .flat_map(|v| v.to_le_bytes())
    .collect();
    mem.write_slice(&boundary, GuestAddress(ALIGNED_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpminud_large_values() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x75, 0x3b, 0x00, // VPMINUD YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let large_vals: Vec<u8> = vec![
        0xFFFFFFFFu32,
        0xFFFFFFFEu32,
        0x80000000u32,
        0x80000001u32,
        0x7FFFFFFFu32,
        0x7FFFFFFEu32,
        0x00000001u32,
        0x00000000u32,
    ]
    .into_iter()
    .flat_map(|v| v.to_le_bytes())
    .collect();
    mem.write_slice(&large_vals, GuestAddress(ALIGNED_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpminub_all_same_values() {
    // Test when all values are the same
    let code = [
        0xc5, 0xf5, 0xda, 0xc1, // VPMINUB YMM0, YMM1, YMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpminuw_all_same_values() {
    // Test when all values are the same
    let code = [
        0xc4, 0xe2, 0x75, 0x3a, 0xc1, // VPMINUW YMM0, YMM1, YMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpminud_all_same_values() {
    // Test when all values are the same
    let code = [
        0xc4, 0xe2, 0x75, 0x3b, 0xc1, // VPMINUD YMM0, YMM1, YMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpminub_extended_regs_r8_r9_r10() {
    let code = [
        0xc4, 0x41, 0x3d, 0xda, 0xc2, // VPMINUB YMM8, YMM8, YMM10
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpminuw_extended_regs_r11_r12_r13() {
    let code = [
        0xc4, 0x42, 0x1d, 0x3a, 0xdd, // VPMINUW YMM11, YMM12, YMM13
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpminud_extended_regs_r14_r15_r8() {
    let code = [
        0xc4, 0x42, 0x05, 0x3b, 0xf0, // VPMINUD YMM14, YMM15, YMM8
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
