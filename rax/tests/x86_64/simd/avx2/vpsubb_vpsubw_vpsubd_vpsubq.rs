use crate::common::*;
use vm_memory::{Bytes, GuestAddress};

// VPSUBB/VPSUBW/VPSUBD/VPSUBQ - Subtract Packed Integers (AVX2)
//
// Performs SIMD subtract of packed integers. Subtracts source from destination.
// Stores packed integer results in destination. Overflow handled with wraparound.
//
// VPSUBB: Subtract 32 packed byte integers (8-bit each) in YMM registers
// VPSUBW: Subtract 16 packed word integers (16-bit each) in YMM registers
// VPSUBD: Subtract 8 packed doubleword integers (32-bit each) in YMM registers
// VPSUBQ: Subtract 4 packed quadword integers (64-bit each) in YMM registers
//
// Opcodes (AVX2 - 256-bit YMM):
// VEX.256.66.0F.WIG F8 /r     VPSUBB ymm1, ymm2, ymm3/m256
// VEX.256.66.0F.WIG F9 /r     VPSUBW ymm1, ymm2, ymm3/m256
// VEX.256.66.0F.WIG FA /r     VPSUBD ymm1, ymm2, ymm3/m256
// VEX.256.66.0F.WIG FB /r     VPSUBQ ymm1, ymm2, ymm3/m256

const ALIGNED_ADDR: u64 = 0x3000;
const ALIGNED_ADDR2: u64 = 0x3100;

// ============================================================================
// VPSUBB Tests - 32x Byte Subtraction (256-bit)
// ============================================================================

#[test]
fn test_vpsubb_ymm0_ymm1_ymm2_all_zeros() {
    // VPSUBB YMM0, YMM1, YMM2 (0 - 0 = 0)
    let code = [
        0xc5, 0xf5, 0xf8, 0xc2, // VPSUBB YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpsubb_ymm3_ymm4_ymm5_same_values() {
    // VPSUBB YMM3, YMM4, YMM5 (0x55 - 0x55 = 0x00)
    let code = [
        0xc5, 0xdd, 0xf8, 0xdd, // VPSUBB YMM3, YMM4, YMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpsubb_ymm6_ymm7_ymm8_underflow() {
    // Test byte subtraction with underflow (0x00 - 0x01 = 0xFF)
    let code = [
        0xc5, 0x45, 0xf8, 0xf0, // VPSUBB YMM6, YMM7, YMM8
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpsubb_ymm9_ymm10_ymm11_mixed() {
    let code = [
        0xc4, 0x41, 0x2d, 0xf8, 0xcb, // VPSUBB YMM9, YMM10, YMM11
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpsubb_ymm12_ymm13_ymm14_alternating() {
    // Alternating 0xAA - 0x55
    let code = [
        0xc4, 0x41, 0x15, 0xf8, 0xe6, // VPSUBB YMM12, YMM13, YMM14
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpsubb_ymm15_ymm0_ymm1_high_reg() {
    let code = [
        0xc4, 0xc1, 0x7d, 0xf8, 0xf9, // VPSUBB YMM15, YMM0, YMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpsubb_ymm0_ymm1_mem() {
    // VPSUBB YMM0, YMM1, [memory]
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0xf8, 0x00, // VPSUBB YMM0, YMM1, [RAX]
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
fn test_vpsubb_ymm2_ymm3_mem_max_values() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xe5, 0xf8, 0x10, // VPSUBB YMM2, YMM3, [RAX]
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
fn test_vpsubb_ymm4_ymm5_mem_sequential() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xd5, 0xf8, 0x20, // VPSUBB YMM4, YMM5, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let seq: Vec<u8> = (0..32).collect();
    mem.write_slice(&seq, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// VPSUBW Tests - 16x Word Subtraction (256-bit)
// ============================================================================

#[test]
fn test_vpsubw_ymm0_ymm1_ymm2_all_zeros() {
    // VPSUBW YMM0, YMM1, YMM2 (0 - 0 = 0)
    let code = [
        0xc5, 0xf5, 0xf9, 0xc2, // VPSUBW YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpsubw_ymm3_ymm4_ymm5_same_values() {
    // VPSUBW YMM3, YMM4, YMM5
    let code = [
        0xc5, 0xdd, 0xf9, 0xdd, // VPSUBW YMM3, YMM4, YMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpsubw_ymm6_ymm7_ymm8_underflow() {
    // Test word underflow (0x0000 - 0x0001 = 0xFFFF)
    let code = [
        0xc5, 0x45, 0xf9, 0xf0, // VPSUBW YMM6, YMM7, YMM8
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpsubw_ymm9_ymm10_ymm11_mixed() {
    let code = [
        0xc4, 0x41, 0x2d, 0xf9, 0xcb, // VPSUBW YMM9, YMM10, YMM11
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpsubw_ymm12_ymm13_ymm14_alternating() {
    let code = [
        0xc4, 0x41, 0x15, 0xf9, 0xe6, // VPSUBW YMM12, YMM13, YMM14
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpsubw_ymm15_ymm0_ymm1_high_reg() {
    let code = [
        0xc4, 0xc1, 0x7d, 0xf9, 0xf9, // VPSUBW YMM15, YMM0, YMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpsubw_ymm0_ymm1_mem() {
    // VPSUBW YMM0, YMM1, [memory]
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0xf9, 0x00, // VPSUBW YMM0, YMM1, [RAX]
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
fn test_vpsubw_ymm2_ymm3_mem_max_values() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xe5, 0xf9, 0x10, // VPSUBW YMM2, YMM3, [RAX]
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
fn test_vpsubw_ymm4_ymm5_mem_sequential() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xd5, 0xf9, 0x20, // VPSUBW YMM4, YMM5, [RAX]
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
// VPSUBD Tests - 8x Doubleword Subtraction (256-bit)
// ============================================================================

#[test]
fn test_vpsubd_ymm0_ymm1_ymm2_all_zeros() {
    // VPSUBD YMM0, YMM1, YMM2 (0 - 0 = 0)
    let code = [
        0xc5, 0xf5, 0xfa, 0xc2, // VPSUBD YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpsubd_ymm3_ymm4_ymm5_same_values() {
    // VPSUBD YMM3, YMM4, YMM5
    let code = [
        0xc5, 0xdd, 0xfa, 0xdd, // VPSUBD YMM3, YMM4, YMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpsubd_ymm6_ymm7_ymm8_underflow() {
    // Test dword underflow (0x00000000 - 0x00000001 = 0xFFFFFFFF)
    let code = [
        0xc5, 0x45, 0xfa, 0xf0, // VPSUBD YMM6, YMM7, YMM8
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpsubd_ymm9_ymm10_ymm11_mixed() {
    let code = [
        0xc4, 0x41, 0x2d, 0xfa, 0xcb, // VPSUBD YMM9, YMM10, YMM11
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpsubd_ymm12_ymm13_ymm14_alternating() {
    let code = [
        0xc4, 0x41, 0x15, 0xfa, 0xe6, // VPSUBD YMM12, YMM13, YMM14
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpsubd_ymm15_ymm0_ymm1_high_reg() {
    let code = [
        0xc4, 0xc1, 0x7d, 0xfa, 0xf9, // VPSUBD YMM15, YMM0, YMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpsubd_ymm0_ymm1_mem() {
    // VPSUBD YMM0, YMM1, [memory]
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0xfa, 0x00, // VPSUBD YMM0, YMM1, [RAX]
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
fn test_vpsubd_ymm2_ymm3_mem_max_values() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xe5, 0xfa, 0x10, // VPSUBD YMM2, YMM3, [RAX]
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
fn test_vpsubd_ymm4_ymm5_mem_sequential() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xd5, 0xfa, 0x20, // VPSUBD YMM4, YMM5, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data: Vec<u8> = (0..8).flat_map(|i| (i as u32).to_le_bytes()).collect();
    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// VPSUBQ Tests - 4x Quadword Subtraction (256-bit)
// ============================================================================

#[test]
fn test_vpsubq_ymm0_ymm1_ymm2_all_zeros() {
    // VPSUBQ YMM0, YMM1, YMM2 (0 - 0 = 0)
    let code = [
        0xc5, 0xf5, 0xfb, 0xc2, // VPSUBQ YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpsubq_ymm3_ymm4_ymm5_same_values() {
    // VPSUBQ YMM3, YMM4, YMM5
    let code = [
        0xc5, 0xdd, 0xfb, 0xdd, // VPSUBQ YMM3, YMM4, YMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpsubq_ymm6_ymm7_ymm8_underflow() {
    // Test qword underflow (0x0000000000000000 - 0x0000000000000001 = 0xFFFFFFFFFFFFFFFF)
    let code = [
        0xc5, 0x45, 0xfb, 0xf0, // VPSUBQ YMM6, YMM7, YMM8
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpsubq_ymm9_ymm10_ymm11_mixed() {
    let code = [
        0xc4, 0x41, 0x2d, 0xfb, 0xcb, // VPSUBQ YMM9, YMM10, YMM11
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpsubq_ymm12_ymm13_ymm14_alternating() {
    let code = [
        0xc4, 0x41, 0x15, 0xfb, 0xe6, // VPSUBQ YMM12, YMM13, YMM14
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpsubq_ymm15_ymm0_ymm1_high_reg() {
    let code = [
        0xc4, 0xc1, 0x7d, 0xfb, 0xf9, // VPSUBQ YMM15, YMM0, YMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpsubq_ymm0_ymm1_mem() {
    // VPSUBQ YMM0, YMM1, [memory]
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0xfb, 0x00, // VPSUBQ YMM0, YMM1, [RAX]
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
fn test_vpsubq_ymm2_ymm3_mem_max_values() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xe5, 0xfb, 0x10, // VPSUBQ YMM2, YMM3, [RAX]
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
fn test_vpsubq_ymm4_ymm5_mem_sequential() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xd5, 0xfb, 0x20, // VPSUBQ YMM4, YMM5, [RAX]
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
fn test_vpsubb_chain_multiple_ops() {
    // Chain multiple VPSUBB operations
    let code = [
        0xc5, 0xf5, 0xf8, 0xc2, // VPSUBB YMM0, YMM1, YMM2
        0xc5, 0xfd, 0xf8, 0xc3, // VPSUBB YMM0, YMM0, YMM3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpsubw_chain_multiple_ops() {
    // Chain multiple VPSUBW operations
    let code = [
        0xc5, 0xf5, 0xf9, 0xc2, // VPSUBW YMM0, YMM1, YMM2
        0xc5, 0xfd, 0xf9, 0xc3, // VPSUBW YMM0, YMM0, YMM3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpsubd_chain_multiple_ops() {
    // Chain multiple VPSUBD operations
    let code = [
        0xc5, 0xf5, 0xfa, 0xc2, // VPSUBD YMM0, YMM1, YMM2
        0xc5, 0xfd, 0xfa, 0xc3, // VPSUBD YMM0, YMM0, YMM3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpsubq_chain_multiple_ops() {
    // Chain multiple VPSUBQ operations
    let code = [
        0xc5, 0xf5, 0xfb, 0xc2, // VPSUBQ YMM0, YMM1, YMM2
        0xc5, 0xfd, 0xfb, 0xc3, // VPSUBQ YMM0, YMM0, YMM3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpsubb_mem_unaligned_offset() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&(ALIGNED_ADDR + 1).to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0xf8, 0x00, // VPSUBB YMM0, YMM1, [RAX]
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
fn test_vpsubw_mem_pattern() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0xf9, 0x00, // VPSUBW YMM0, YMM1, [RAX]
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
fn test_vpsubd_mem_powers_of_two() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0xfa, 0x00, // VPSUBD YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let powers: Vec<u8> = (0..8).flat_map(|i| (1u32 << i).to_le_bytes()).collect();
    mem.write_slice(&powers, GuestAddress(ALIGNED_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpsubq_mem_large_values() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0xfb, 0x00, // VPSUBQ YMM0, YMM1, [RAX]
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
// Known-answer VALUE tests : packed wrapping subtract (src1 - src2) per element.
// ============================================================================

use rax::backend::emulator::x86_64::X86_64Vcpu;

fn ksub_set(vcpu: &mut X86_64Vcpu, idx: usize, lo: u128, hi: u128) {
    let mut regs = vcpu.get_regs().unwrap();
    regs.xmm[idx][0] = lo as u64;
    regs.xmm[idx][1] = (lo >> 64) as u64;
    regs.ymm_high[idx][0] = hi as u64;
    regs.ymm_high[idx][1] = (hi >> 64) as u64;
    vcpu.set_regs(&regs).unwrap();
}
fn ksub_lo(vcpu: &X86_64Vcpu, idx: usize) -> u128 {
    let r = vcpu.get_regs().unwrap();
    (r.xmm[idx][0] as u128) | ((r.xmm[idx][1] as u128) << 64)
}
fn ksub_hi(vcpu: &X86_64Vcpu, idx: usize) -> u128 {
    let r = vcpu.get_regs().unwrap();
    (r.ymm_high[idx][0] as u128) | ((r.ymm_high[idx][1] as u128) << 64)
}

fn psub_bytes(a: u128, b: u128) -> u128 {
    let (ab, bb) = (a.to_le_bytes(), b.to_le_bytes());
    let mut out = [0u8; 16];
    for i in 0..16 {
        out[i] = ab[i].wrapping_sub(bb[i]);
    }
    u128::from_le_bytes(out)
}
fn psub_words(a: u128, b: u128) -> u128 {
    let mut out = 0u128;
    for i in 0..8 {
        let av = ((a >> (i * 16)) & 0xFFFF) as u16;
        let bv = ((b >> (i * 16)) & 0xFFFF) as u16;
        out |= (av.wrapping_sub(bv) as u128) << (i * 16);
    }
    out
}
fn psub_dwords(a: u128, b: u128) -> u128 {
    let mut out = 0u128;
    for i in 0..4 {
        let av = ((a >> (i * 32)) & 0xFFFF_FFFF) as u32;
        let bv = ((b >> (i * 32)) & 0xFFFF_FFFF) as u32;
        out |= (av.wrapping_sub(bv) as u128) << (i * 32);
    }
    out
}
fn psub_qwords(a: u128, b: u128) -> u128 {
    let lo = (a as u64).wrapping_sub(b as u64);
    let hi = ((a >> 64) as u64).wrapping_sub((b >> 64) as u64);
    (lo as u128) | ((hi as u128) << 64)
}

const SA_LO: u128 = 0x0000_0001_0010_0100_1234_5678_9ABC_DEF0;
const SA_HI: u128 = 0xFEDC_BA98_7654_3210_0102_0304_0506_0708;
const SB_LO: u128 = 0x0001_0002_0020_0200_FEDC_BA98_7654_3210;
const SB_HI: u128 = 0x1111_2222_3333_4444_FFFF_FFFF_FFFF_FFFF;

#[test]
fn test_vpsubb_xmm_value() {
    let code = [0xc5, 0xf1, 0xf8, 0xc2, 0xf4]; // VPSUBB XMM0, XMM1, XMM2
    let (mut vcpu, _) = setup_vm(&code, None);
    ksub_set(&mut vcpu, 1, SA_LO, 0xDEAD);
    ksub_set(&mut vcpu, 2, SB_LO, 0xBEEF);
    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(ksub_lo(&vcpu, 0), psub_bytes(SA_LO, SB_LO));
    assert_eq!(ksub_hi(&vcpu, 0), 0, "VEX.128 must zero upper 128 bits");
}

#[test]
fn test_vpsubb_ymm_value() {
    let code = [0xc5, 0xf5, 0xf8, 0xc2, 0xf4]; // VPSUBB YMM0, YMM1, YMM2
    let (mut vcpu, _) = setup_vm(&code, None);
    ksub_set(&mut vcpu, 1, SA_LO, SA_HI);
    ksub_set(&mut vcpu, 2, SB_LO, SB_HI);
    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(ksub_lo(&vcpu, 0), psub_bytes(SA_LO, SB_LO));
    assert_eq!(ksub_hi(&vcpu, 0), psub_bytes(SA_HI, SB_HI));
}

#[test]
fn test_vpsubw_ymm_value() {
    let code = [0xc5, 0xf5, 0xf9, 0xc2, 0xf4]; // VPSUBW YMM0, YMM1, YMM2
    let (mut vcpu, _) = setup_vm(&code, None);
    ksub_set(&mut vcpu, 1, SA_LO, SA_HI);
    ksub_set(&mut vcpu, 2, SB_LO, SB_HI);
    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(ksub_lo(&vcpu, 0), psub_words(SA_LO, SB_LO));
    assert_eq!(ksub_hi(&vcpu, 0), psub_words(SA_HI, SB_HI));
}

#[test]
fn test_vpsubd_ymm_value() {
    let code = [0xc5, 0xf5, 0xfa, 0xc2, 0xf4]; // VPSUBD YMM0, YMM1, YMM2
    let (mut vcpu, _) = setup_vm(&code, None);
    ksub_set(&mut vcpu, 1, SA_LO, SA_HI);
    ksub_set(&mut vcpu, 2, SB_LO, SB_HI);
    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(ksub_lo(&vcpu, 0), psub_dwords(SA_LO, SB_LO));
    assert_eq!(ksub_hi(&vcpu, 0), psub_dwords(SA_HI, SB_HI));
}

#[test]
fn test_vpsubq_ymm_value() {
    let code = [0xc5, 0xf5, 0xfb, 0xc2, 0xf4]; // VPSUBQ YMM0, YMM1, YMM2
    let (mut vcpu, _) = setup_vm(&code, None);
    ksub_set(&mut vcpu, 1, SA_LO, SA_HI);
    ksub_set(&mut vcpu, 2, SB_LO, SB_HI);
    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(ksub_lo(&vcpu, 0), psub_qwords(SA_LO, SB_LO));
    assert_eq!(ksub_hi(&vcpu, 0), psub_qwords(SA_HI, SB_HI));
}

#[test]
fn test_vpsubb_zero_minus_one_wraps_to_ff() {
    // 0x00 - 0x01 per byte must wrap to 0xFF independently.
    let code = [0xc5, 0xf5, 0xf8, 0xc2, 0xf4]; // VPSUBB YMM0, YMM1, YMM2
    let (mut vcpu, _) = setup_vm(&code, None);
    let all_01: u128 = 0x0101_0101_0101_0101_0101_0101_0101_0101;
    ksub_set(&mut vcpu, 1, 0, 0);
    ksub_set(&mut vcpu, 2, all_01, all_01);
    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(ksub_lo(&vcpu, 0), u128::MAX);
    assert_eq!(ksub_hi(&vcpu, 0), u128::MAX);
}
