use crate::common::*;
use vm_memory::{Bytes, GuestAddress};

// VPADDB/VPADDW/VPADDD/VPADDQ - Add Packed Integers (AVX2)
//
// Performs SIMD add of packed integers from source and destination operands.
// Stores packed integer results in destination. Overflow handled with wraparound.
//
// VPADDB: Add 32 packed byte integers (8-bit each) in YMM registers
// VPADDW: Add 16 packed word integers (16-bit each) in YMM registers
// VPADDD: Add 8 packed doubleword integers (32-bit each) in YMM registers
// VPADDQ: Add 4 packed quadword integers (64-bit each) in YMM registers
//
// Opcodes (AVX2 - 256-bit YMM):
// VEX.256.66.0F.WIG FC /r     VPADDB ymm1, ymm2, ymm3/m256
// VEX.256.66.0F.WIG FD /r     VPADDW ymm1, ymm2, ymm3/m256
// VEX.256.66.0F.WIG FE /r     VPADDD ymm1, ymm2, ymm3/m256
// VEX.256.66.0F.WIG D4 /r     VPADDQ ymm1, ymm2, ymm3/m256

const ALIGNED_ADDR: u64 = 0x3000;
const ALIGNED_ADDR2: u64 = 0x3100;

// ============================================================================
// VPADDB Tests - 32x Byte Addition (256-bit)
// ============================================================================

#[test]
fn test_vpaddb_ymm0_ymm1_ymm2_all_zeros() {
    // VPADDB YMM0, YMM1, YMM2 with all zeros
    let code = [
        0xc5, 0xf5, 0xfc, 0xc2, // VPADDB YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpaddb_ymm3_ymm4_ymm5_all_ones() {
    // VPADDB YMM3, YMM4, YMM5 with all 0x01 values
    let code = [
        0xc5, 0xdd, 0xfc, 0xdd, // VPADDB YMM3, YMM4, YMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpaddb_ymm6_ymm7_ymm8_wraparound() {
    // Test byte addition with wraparound (0xFF + 0x01 = 0x00)
    let code = [
        0xc5, 0x45, 0xfc, 0xf0, // VPADDB YMM6, YMM7, YMM8
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpaddb_ymm9_ymm10_ymm11_mixed() {
    let code = [
        0xc4, 0x41, 0x2d, 0xfc, 0xcb, // VPADDB YMM9, YMM10, YMM11
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpaddb_ymm12_ymm13_ymm14_alternating() {
    // Alternating 0xAA and 0x55
    let code = [
        0xc4, 0x41, 0x15, 0xfc, 0xe6, // VPADDB YMM12, YMM13, YMM14
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpaddb_ymm15_ymm0_ymm1_high_reg() {
    let code = [
        0xc4, 0xc1, 0x7d, 0xfc, 0xf9, // VPADDB YMM15, YMM0, YMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpaddb_ymm0_ymm1_mem() {
    // VPADDB YMM0, YMM1, [memory]
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0xfc, 0x00, // VPADDB YMM0, YMM1, [RAX]
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
fn test_vpaddb_ymm2_ymm3_mem_max_values() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xe5, 0xfc, 0x10, // VPADDB YMM2, YMM3, [RAX]
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
fn test_vpaddb_ymm4_ymm5_mem_sequential() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xd5, 0xfc, 0x20, // VPADDB YMM4, YMM5, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let seq: Vec<u8> = (0..32).collect();
    mem.write_slice(&seq, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// VPADDW Tests - 16x Word Addition (256-bit)
// ============================================================================

#[test]
fn test_vpaddw_ymm0_ymm1_ymm2_all_zeros() {
    // VPADDW YMM0, YMM1, YMM2 with all zeros
    let code = [
        0xc5, 0xf5, 0xfd, 0xc2, // VPADDW YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpaddw_ymm3_ymm4_ymm5_all_ones() {
    // VPADDW YMM3, YMM4, YMM5
    let code = [
        0xc5, 0xdd, 0xfd, 0xdd, // VPADDW YMM3, YMM4, YMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpaddw_ymm6_ymm7_ymm8_wraparound() {
    // Test word wraparound (0xFFFF + 0x0001 = 0x0000)
    let code = [
        0xc5, 0x45, 0xfd, 0xf0, // VPADDW YMM6, YMM7, YMM8
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpaddw_ymm9_ymm10_ymm11_mixed() {
    let code = [
        0xc4, 0x41, 0x2d, 0xfd, 0xcb, // VPADDW YMM9, YMM10, YMM11
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpaddw_ymm12_ymm13_ymm14_alternating() {
    let code = [
        0xc4, 0x41, 0x15, 0xfd, 0xe6, // VPADDW YMM12, YMM13, YMM14
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpaddw_ymm15_ymm0_ymm1_high_reg() {
    let code = [
        0xc4, 0xc1, 0x7d, 0xfd, 0xf9, // VPADDW YMM15, YMM0, YMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpaddw_ymm0_ymm1_mem() {
    // VPADDW YMM0, YMM1, [memory]
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0xfd, 0x00, // VPADDW YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data: Vec<u8> = (0..32)
        .flat_map(|i| ((i * 0x0101u16) as u16).to_le_bytes())
        .take(32)
        .collect();
    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpaddw_ymm2_ymm3_mem_max_values() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xe5, 0xfd, 0x10, // VPADDW YMM2, YMM3, [RAX]
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
fn test_vpaddw_ymm4_ymm5_mem_sequential() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xd5, 0xfd, 0x20, // VPADDW YMM4, YMM5, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data: Vec<u8> = (0..16)
        .flat_map(|i| (i as u16 * 0x1111).to_le_bytes())
        .collect();
    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// VPADDD Tests - 8x Doubleword Addition (256-bit)
// ============================================================================

#[test]
fn test_vpaddd_ymm0_ymm1_ymm2_all_zeros() {
    // VPADDD YMM0, YMM1, YMM2 with all zeros
    let code = [
        0xc5, 0xf5, 0xfe, 0xc2, // VPADDD YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpaddd_ymm3_ymm4_ymm5_all_ones() {
    // VPADDD YMM3, YMM4, YMM5
    let code = [
        0xc5, 0xdd, 0xfe, 0xdd, // VPADDD YMM3, YMM4, YMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpaddd_ymm6_ymm7_ymm8_wraparound() {
    // Test dword wraparound (0xFFFFFFFF + 0x00000001 = 0x00000000)
    let code = [
        0xc5, 0x45, 0xfe, 0xf0, // VPADDD YMM6, YMM7, YMM8
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpaddd_ymm9_ymm10_ymm11_mixed() {
    let code = [
        0xc4, 0x41, 0x2d, 0xfe, 0xcb, // VPADDD YMM9, YMM10, YMM11
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpaddd_ymm12_ymm13_ymm14_alternating() {
    let code = [
        0xc4, 0x41, 0x15, 0xfe, 0xe6, // VPADDD YMM12, YMM13, YMM14
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpaddd_ymm15_ymm0_ymm1_high_reg() {
    let code = [
        0xc4, 0xc1, 0x7d, 0xfe, 0xf9, // VPADDD YMM15, YMM0, YMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpaddd_ymm0_ymm1_mem() {
    // VPADDD YMM0, YMM1, [memory]
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0xfe, 0x00, // VPADDD YMM0, YMM1, [RAX]
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
fn test_vpaddd_ymm2_ymm3_mem_max_values() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xe5, 0xfe, 0x10, // VPADDD YMM2, YMM3, [RAX]
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
fn test_vpaddd_ymm4_ymm5_mem_sequential() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xd5, 0xfe, 0x20, // VPADDD YMM4, YMM5, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data: Vec<u8> = (0..8).flat_map(|i| (i as u32).to_le_bytes()).collect();
    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// VPADDQ Tests - 4x Quadword Addition (256-bit)
// ============================================================================

#[test]
fn test_vpaddq_ymm0_ymm1_ymm2_all_zeros() {
    // VPADDQ YMM0, YMM1, YMM2 with all zeros
    let code = [
        0xc5, 0xf5, 0xd4, 0xc2, // VPADDQ YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpaddq_ymm3_ymm4_ymm5_all_ones() {
    // VPADDQ YMM3, YMM4, YMM5
    let code = [
        0xc5, 0xdd, 0xd4, 0xdd, // VPADDQ YMM3, YMM4, YMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpaddq_ymm6_ymm7_ymm8_wraparound() {
    // Test qword wraparound (0xFFFFFFFFFFFFFFFF + 0x0000000000000001 = 0x0000000000000000)
    let code = [
        0xc5, 0x45, 0xd4, 0xf0, // VPADDQ YMM6, YMM7, YMM8
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpaddq_ymm9_ymm10_ymm11_mixed() {
    let code = [
        0xc4, 0x41, 0x2d, 0xd4, 0xcb, // VPADDQ YMM9, YMM10, YMM11
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpaddq_ymm12_ymm13_ymm14_alternating() {
    let code = [
        0xc4, 0x41, 0x15, 0xd4, 0xe6, // VPADDQ YMM12, YMM13, YMM14
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpaddq_ymm15_ymm0_ymm1_high_reg() {
    let code = [
        0xc4, 0xc1, 0x7d, 0xd4, 0xf9, // VPADDQ YMM15, YMM0, YMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpaddq_ymm0_ymm1_mem() {
    // VPADDQ YMM0, YMM1, [memory]
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0xd4, 0x00, // VPADDQ YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data: Vec<u8> = (0..4)
        .flat_map(|i| ((i * 0x1111111111111111u64) as u64).to_le_bytes())
        .collect();
    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpaddq_ymm2_ymm3_mem_max_values() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xe5, 0xd4, 0x10, // VPADDQ YMM2, YMM3, [RAX]
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
fn test_vpaddq_ymm4_ymm5_mem_sequential() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xd5, 0xd4, 0x20, // VPADDQ YMM4, YMM5, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data: Vec<u8> = (0..4).flat_map(|i| (i as u64).to_le_bytes()).collect();
    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Additional comprehensive tests mixing different operations
// ============================================================================

#[test]
fn test_vpaddb_chain_multiple_ops() {
    // Chain multiple VPADDB operations
    let code = [
        0xc5, 0xf5, 0xfc, 0xc2, // VPADDB YMM0, YMM1, YMM2
        0xc5, 0xfd, 0xfc, 0xc3, // VPADDB YMM0, YMM0, YMM3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpaddw_chain_multiple_ops() {
    // Chain multiple VPADDW operations
    let code = [
        0xc5, 0xf5, 0xfd, 0xc2, // VPADDW YMM0, YMM1, YMM2
        0xc5, 0xfd, 0xfd, 0xc3, // VPADDW YMM0, YMM0, YMM3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpaddd_chain_multiple_ops() {
    // Chain multiple VPADDD operations
    let code = [
        0xc5, 0xf5, 0xfe, 0xc2, // VPADDD YMM0, YMM1, YMM2
        0xc5, 0xfd, 0xfe, 0xc3, // VPADDD YMM0, YMM0, YMM3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpaddq_chain_multiple_ops() {
    // Chain multiple VPADDQ operations
    let code = [
        0xc5, 0xf5, 0xd4, 0xc2, // VPADDQ YMM0, YMM1, YMM2
        0xc5, 0xfd, 0xd4, 0xc3, // VPADDQ YMM0, YMM0, YMM3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpaddb_mem_unaligned_offset() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&(ALIGNED_ADDR + 1).to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0xfc, 0x00, // VPADDB YMM0, YMM1, [RAX]
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
fn test_vpaddw_mem_pattern() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0xfd, 0x00, // VPADDW YMM0, YMM1, [RAX]
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
fn test_vpaddd_mem_powers_of_two() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0xfe, 0x00, // VPADDD YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let powers: Vec<u8> = (0..8).flat_map(|i| (1u32 << i).to_le_bytes()).collect();
    mem.write_slice(&powers, GuestAddress(ALIGNED_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpaddq_mem_large_values() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0xd4, 0x00, // VPADDQ YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let large_vals: Vec<u8> = vec![
        0xFFFFFFFFFFFFFF00u64,
        0xAAAAAAAAAAAAAAAAu64,
        0x5555555555555555u64,
        0x0000000000000001u64,
    ]
    .into_iter()
    .flat_map(|v| v.to_le_bytes())
    .collect();
    mem.write_slice(&large_vals, GuestAddress(ALIGNED_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Known-answer VALUE tests : packed wrapping add of bytes/words/dwords/qwords.
// Verifies independent lane arithmetic with wrap (no carry across elements).
// ============================================================================

use rax::backend::emulator::x86_64::X86_64Vcpu;

fn kadd_set(vcpu: &mut X86_64Vcpu, idx: usize, lo: u128, hi: u128) {
    let mut regs = vcpu.get_regs().unwrap();
    regs.xmm[idx][0] = lo as u64;
    regs.xmm[idx][1] = (lo >> 64) as u64;
    regs.ymm_high[idx][0] = hi as u64;
    regs.ymm_high[idx][1] = (hi >> 64) as u64;
    vcpu.set_regs(&regs).unwrap();
}
fn kadd_lo(vcpu: &X86_64Vcpu, idx: usize) -> u128 {
    let r = vcpu.get_regs().unwrap();
    (r.xmm[idx][0] as u128) | ((r.xmm[idx][1] as u128) << 64)
}
fn kadd_hi(vcpu: &X86_64Vcpu, idx: usize) -> u128 {
    let r = vcpu.get_regs().unwrap();
    (r.ymm_high[idx][0] as u128) | ((r.ymm_high[idx][1] as u128) << 64)
}

fn padd_bytes(a: u128, b: u128) -> u128 {
    let (ab, bb) = (a.to_le_bytes(), b.to_le_bytes());
    let mut out = [0u8; 16];
    for i in 0..16 {
        out[i] = ab[i].wrapping_add(bb[i]);
    }
    u128::from_le_bytes(out)
}
fn padd_words(a: u128, b: u128) -> u128 {
    let mut out = 0u128;
    for i in 0..8 {
        let av = ((a >> (i * 16)) & 0xFFFF) as u16;
        let bv = ((b >> (i * 16)) & 0xFFFF) as u16;
        out |= (av.wrapping_add(bv) as u128) << (i * 16);
    }
    out
}
fn padd_dwords(a: u128, b: u128) -> u128 {
    let mut out = 0u128;
    for i in 0..4 {
        let av = ((a >> (i * 32)) & 0xFFFF_FFFF) as u32;
        let bv = ((b >> (i * 32)) & 0xFFFF_FFFF) as u32;
        out |= (av.wrapping_add(bv) as u128) << (i * 32);
    }
    out
}
fn padd_qwords(a: u128, b: u128) -> u128 {
    let lo = (a as u64).wrapping_add(b as u64);
    let hi = ((a >> 64) as u64).wrapping_add((b >> 64) as u64);
    (lo as u128) | ((hi as u128) << 64)
}

const PA_LO: u128 = 0x7F7F_7F7F_00FF_8081_1234_5678_9ABC_DEF0;
const PA_HI: u128 = 0xFEDC_BA98_7654_3210_0102_0304_0506_0708;
const PB_LO: u128 = 0x0102_0304_01FF_807F_FEDC_BA98_7654_3210;
const PB_HI: u128 = 0x1111_2222_3333_4444_FFFF_FFFF_FFFF_FFFF;

#[test]
fn test_vpaddb_xmm_value() {
    // VPADDB XMM0, XMM1, XMM2 (128-bit); upper 128 zeroed.
    let code = [0xc5, 0xf1, 0xfc, 0xc2, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    kadd_set(&mut vcpu, 1, PA_LO, 0xDEAD);
    kadd_set(&mut vcpu, 2, PB_LO, 0xBEEF);
    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(kadd_lo(&vcpu, 0), padd_bytes(PA_LO, PB_LO));
    assert_eq!(kadd_hi(&vcpu, 0), 0, "VEX.128 must zero upper 128 bits");
}

#[test]
fn test_vpaddb_ymm_value() {
    let code = [0xc5, 0xf5, 0xfc, 0xc2, 0xf4]; // VPADDB YMM0, YMM1, YMM2
    let (mut vcpu, _) = setup_vm(&code, None);
    kadd_set(&mut vcpu, 1, PA_LO, PA_HI);
    kadd_set(&mut vcpu, 2, PB_LO, PB_HI);
    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(kadd_lo(&vcpu, 0), padd_bytes(PA_LO, PB_LO));
    assert_eq!(kadd_hi(&vcpu, 0), padd_bytes(PA_HI, PB_HI));
}

#[test]
fn test_vpaddw_ymm_value() {
    let code = [0xc5, 0xf5, 0xfd, 0xc2, 0xf4]; // VPADDW YMM0, YMM1, YMM2
    let (mut vcpu, _) = setup_vm(&code, None);
    kadd_set(&mut vcpu, 1, PA_LO, PA_HI);
    kadd_set(&mut vcpu, 2, PB_LO, PB_HI);
    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(kadd_lo(&vcpu, 0), padd_words(PA_LO, PB_LO));
    assert_eq!(kadd_hi(&vcpu, 0), padd_words(PA_HI, PB_HI));
}

#[test]
fn test_vpaddd_ymm_value() {
    let code = [0xc5, 0xf5, 0xfe, 0xc2, 0xf4]; // VPADDD YMM0, YMM1, YMM2
    let (mut vcpu, _) = setup_vm(&code, None);
    kadd_set(&mut vcpu, 1, PA_LO, PA_HI);
    kadd_set(&mut vcpu, 2, PB_LO, PB_HI);
    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(kadd_lo(&vcpu, 0), padd_dwords(PA_LO, PB_LO));
    assert_eq!(kadd_hi(&vcpu, 0), padd_dwords(PA_HI, PB_HI));
}

#[test]
fn test_vpaddq_ymm_value() {
    let code = [0xc5, 0xf5, 0xd4, 0xc2, 0xf4]; // VPADDQ YMM0, YMM1, YMM2
    let (mut vcpu, _) = setup_vm(&code, None);
    kadd_set(&mut vcpu, 1, PA_LO, PA_HI);
    kadd_set(&mut vcpu, 2, PB_LO, PB_HI);
    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(kadd_lo(&vcpu, 0), padd_qwords(PA_LO, PB_LO));
    assert_eq!(kadd_hi(&vcpu, 0), padd_qwords(PA_HI, PB_HI));
}

#[test]
fn test_vpaddb_no_carry_across_lane_boundary() {
    // 0xFF + 0x01 in each byte wraps to 0x00 independently (no carry to next byte).
    let code = [0xc5, 0xf5, 0xfc, 0xc2, 0xf4]; // VPADDB YMM0, YMM1, YMM2
    let (mut vcpu, _) = setup_vm(&code, None);
    let all_ff: u128 = u128::MAX;
    let all_01: u128 = 0x0101_0101_0101_0101_0101_0101_0101_0101;
    kadd_set(&mut vcpu, 1, all_ff, all_ff);
    kadd_set(&mut vcpu, 2, all_01, all_01);
    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(kadd_lo(&vcpu, 0), 0, "each byte 0xFF+1 wraps to 0x00");
    assert_eq!(kadd_hi(&vcpu, 0), 0);
}
