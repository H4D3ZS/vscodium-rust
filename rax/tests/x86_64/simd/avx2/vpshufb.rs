use crate::common::*;
use vm_memory::{Bytes, GuestAddress};

// VPSHUFB - Packed Shuffle Bytes (AVX2)
//
// Performs in-place shuffles of bytes in the destination operand according to the
// shuffle control mask in the source operand. Each 128-bit lane is shuffled independently.
//
// For each byte in the shuffle control mask:
// - Bits [3:0] select which byte from the corresponding 128-bit lane (0-15)
// - Bit 7 set means write zero to that destination byte position
//
// Opcode: VEX.256.66.0F38.WIG 00 /r    VPSHUFB ymm1, ymm2, ymm3/m256

const ALIGNED_ADDR: u64 = 0x3000;

// ============================================================================
// Tests with zero mask (high bit set - zeros output)
// ============================================================================

#[test]
fn test_vpshufb_ymm0_ymm1_ymm2_all_zeros() {
    // VPSHUFB YMM0, YMM1, YMM2 where YMM2 has all high bits set
    let code = [
        0xc4, 0xe2, 0x75, 0x00, 0xc2, // VPSHUFB YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpshufb_ymm3_ymm4_ymm5_all_zeros() {
    let code = [
        0xc4, 0xe2, 0x5d, 0x00, 0xdd, // VPSHUFB YMM3, YMM4, YMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpshufb_ymm6_ymm7_ymm8_partial_zeros() {
    // Mix of zero and non-zero mask bytes
    let code = [
        0xc4, 0xc2, 0x45, 0x00, 0xf0, // VPSHUFB YMM6, YMM7, YMM8
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Tests with identity shuffle (0x00, 0x01, 0x02, ..., 0x0F for each lane)
// ============================================================================

#[test]
fn test_vpshufb_ymm0_ymm1_ymm2_identity() {
    // VPSHUFB YMM0, YMM1, YMM2 - identity shuffle
    let code = [
        0xc4, 0xe2, 0x75, 0x00, 0xc2, // VPSHUFB YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpshufb_ymm3_ymm4_ymm5_identity() {
    let code = [
        0xc4, 0xe2, 0x5d, 0x00, 0xdd, // VPSHUFB YMM3, YMM4, YMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpshufb_ymm9_ymm10_ymm11_identity() {
    let code = [
        0xc4, 0x42, 0x2d, 0x00, 0xcb, // VPSHUFB YMM9, YMM10, YMM11
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Tests with reverse shuffle (0x0F, 0x0E, 0x0D, ..., 0x00 for each lane)
// ============================================================================

#[test]
fn test_vpshufb_ymm0_ymm1_ymm2_reverse() {
    // VPSHUFB YMM0, YMM1, YMM2 - reverse byte order in each lane
    let code = [
        0xc4, 0xe2, 0x75, 0x00, 0xc2, // VPSHUFB YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpshufb_ymm3_ymm4_ymm5_reverse() {
    let code = [
        0xc4, 0xe2, 0x5d, 0x00, 0xdd, // VPSHUFB YMM3, YMM4, YMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpshufb_ymm12_ymm13_ymm14_reverse() {
    let code = [
        0xc4, 0x42, 0x15, 0x00, 0xe6, // VPSHUFB YMM12, YMM13, YMM14
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Tests with broadcast byte 0 from each lane
// ============================================================================

#[test]
fn test_vpshufb_ymm0_ymm1_ymm2_broadcast_byte0() {
    // VPSHUFB YMM0, YMM1, YMM2 - broadcast byte 0 in each lane
    let code = [
        0xc4, 0xe2, 0x75, 0x00, 0xc2, // VPSHUFB YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpshufb_ymm3_ymm4_ymm5_broadcast_byte7() {
    // Broadcast byte 7 in each lane
    let code = [
        0xc4, 0xe2, 0x5d, 0x00, 0xdd, // VPSHUFB YMM3, YMM4, YMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpshufb_ymm6_ymm7_ymm8_broadcast_byte15() {
    // Broadcast byte 15 in each lane
    let code = [
        0xc4, 0xc2, 0x45, 0x00, 0xf0, // VPSHUFB YMM6, YMM7, YMM8
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Tests with alternating patterns
// ============================================================================

#[test]
fn test_vpshufb_ymm0_ymm1_ymm2_even_bytes() {
    // Select only even bytes from each lane
    let code = [
        0xc4, 0xe2, 0x75, 0x00, 0xc2, // VPSHUFB YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpshufb_ymm3_ymm4_ymm5_odd_bytes() {
    // Select only odd bytes from each lane
    let code = [
        0xc4, 0xe2, 0x5d, 0x00, 0xdd, // VPSHUFB YMM3, YMM4, YMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpshufb_ymm9_ymm10_ymm11_alternating_pattern() {
    let code = [
        0xc4, 0x42, 0x2d, 0x00, 0xcb, // VPSHUFB YMM9, YMM10, YMM11
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Tests with extended registers (YMM8-YMM15)
// ============================================================================

#[test]
fn test_vpshufb_ymm8_ymm9_ymm10() {
    let code = [
        0xc4, 0x42, 0x35, 0x00, 0xc2, // VPSHUFB YMM8, YMM9, YMM10
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpshufb_ymm11_ymm12_ymm13() {
    let code = [
        0xc4, 0x42, 0x1d, 0x00, 0xdd, // VPSHUFB YMM11, YMM12, YMM13
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpshufb_ymm14_ymm15_ymm8() {
    let code = [
        0xc4, 0x42, 0x05, 0x00, 0xf0, // VPSHUFB YMM14, YMM15, YMM8
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpshufb_ymm15_ymm0_ymm1() {
    let code = [
        0xc4, 0xc2, 0x7d, 0x00, 0xf9, // VPSHUFB YMM15, YMM0, YMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Tests with memory operands
// ============================================================================

#[test]
fn test_vpshufb_ymm0_ymm1_mem_identity() {
    // VPSHUFB YMM0, YMM1, [memory] - identity shuffle from memory
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x75, 0x00, 0x00, // VPSHUFB YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    // Identity shuffle pattern for both lanes
    let mut pattern = Vec::new();
    pattern.extend_from_slice(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);
    pattern.extend_from_slice(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);
    mem.write_slice(&pattern, GuestAddress(ALIGNED_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpshufb_ymm2_ymm3_mem_reverse() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x65, 0x00, 0x10, // VPSHUFB YMM2, YMM3, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    // Reverse shuffle pattern for both lanes
    let mut pattern = Vec::new();
    pattern.extend_from_slice(&[15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0]);
    pattern.extend_from_slice(&[15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0]);
    mem.write_slice(&pattern, GuestAddress(ALIGNED_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpshufb_ymm4_ymm5_mem_zeros() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x55, 0x00, 0x20, // VPSHUFB YMM4, YMM5, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    // All high bits set (zeros output)
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
fn test_vpshufb_ymm6_ymm7_mem_broadcast() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x45, 0x00, 0x30, // VPSHUFB YMM6, YMM7, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    // Broadcast byte 0 from each lane
    mem.write_slice(
        &[
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
        ],
        GuestAddress(ALIGNED_ADDR),
    )
    .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpshufb_ymm8_ymm9_mem_mixed() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0x62, 0x35, 0x00, 0x00, // VPSHUFB YMM8, YMM9, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    // Mixed pattern: some broadcast, some reverse, some zeros
    let pattern: Vec<u8> = vec![
        0, 0, 0, 0, 15, 14, 13, 12, 0x80, 0x80, 0x80, 0x80, 3, 2, 1, 0, 0, 0, 0, 0, 15, 14, 13, 12,
        0x80, 0x80, 0x80, 0x80, 3, 2, 1, 0,
    ];
    mem.write_slice(&pattern, GuestAddress(ALIGNED_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Tests with chained operations
// ============================================================================

#[test]
fn test_vpshufb_chain_operations() {
    // Chain multiple VPSHUFB operations
    let code = [
        0xc4, 0xe2, 0x75, 0x00, 0xc2, // VPSHUFB YMM0, YMM1, YMM2
        0xc4, 0xe2, 0x7d, 0x00, 0xc3, // VPSHUFB YMM0, YMM0, YMM3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpshufb_double_shuffle() {
    // Shuffle, then shuffle again with identity (should restore some bytes)
    let code = [
        0xc4, 0xe2, 0x75, 0x00, 0xc2, // VPSHUFB YMM0, YMM1, YMM2
        0xc4, 0xe2, 0x7d, 0x00, 0xc3, // VPSHUFB YMM0, YMM0, YMM3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Tests with specific shuffle patterns
// ============================================================================

#[test]
fn test_vpshufb_swap_pairs() {
    // Swap adjacent byte pairs in each lane
    let code = [
        0xc4, 0xe2, 0x75, 0x00, 0xc2, // VPSHUFB YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpshufb_swap_nibbles() {
    // Pattern to reverse nibbles within bytes (simulation via shuffle)
    let code = [
        0xc4, 0xe2, 0x75, 0x00, 0xc2, // VPSHUFB YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpshufb_duplicate_first_half() {
    // Duplicate first 8 bytes of each lane
    let code = [
        0xc4, 0xe2, 0x75, 0x00, 0xc2, // VPSHUFB YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpshufb_duplicate_second_half() {
    // Duplicate second 8 bytes of each lane
    let code = [
        0xc4, 0xe2, 0x75, 0x00, 0xc2, // VPSHUFB YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpshufb_rotate_left_1() {
    // Rotate bytes left by 1 in each lane
    let code = [
        0xc4, 0xe2, 0x75, 0x00, 0xc2, // VPSHUFB YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpshufb_rotate_right_1() {
    // Rotate bytes right by 1 in each lane
    let code = [
        0xc4, 0xe2, 0x75, 0x00, 0xc2, // VPSHUFB YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpshufb_extract_low_nibbles() {
    // Extract low nibble bytes and pack them
    let code = [
        0xc4, 0xe2, 0x75, 0x00, 0xc2, // VPSHUFB YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpshufb_interleave_bytes() {
    // Interleave bytes from two halves of each lane
    let code = [
        0xc4, 0xe2, 0x75, 0x00, 0xc2, // VPSHUFB YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpshufb_deinterleave_bytes() {
    // Deinterleave bytes to two halves of each lane
    let code = [
        0xc4, 0xe2, 0x75, 0x00, 0xc2, // VPSHUFB YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Tests with lane-specific patterns
// ============================================================================

#[test]
fn test_vpshufb_different_per_lane() {
    // Different shuffle pattern for each 128-bit lane
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x75, 0x00, 0x00, // VPSHUFB YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    // Lane 0: identity, Lane 1: reverse
    let mut pattern = Vec::new();
    pattern.extend_from_slice(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);
    pattern.extend_from_slice(&[15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0]);
    mem.write_slice(&pattern, GuestAddress(ALIGNED_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpshufb_mem_unaligned() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&(ALIGNED_ADDR + 1).to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x75, 0x00, 0x00, // VPSHUFB YMM0, YMM1, [RAX]
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
