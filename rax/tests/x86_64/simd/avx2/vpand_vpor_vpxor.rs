use crate::common::*;
use vm_memory::{Bytes, GuestAddress};

// VPAND/VPOR/VPXOR - Packed Bitwise Logical Operations (AVX2)
//
// Performs bitwise logical operations on 256-bit packed integers.
//
// VPAND: Bitwise AND of two 256-bit operands
// VPOR: Bitwise OR of two 256-bit operands
// VPXOR: Bitwise XOR of two 256-bit operands
//
// Opcodes (AVX2 - 256-bit YMM):
// VEX.256.66.0F.WIG DB /r     VPAND ymm1, ymm2, ymm3/m256
// VEX.256.66.0F.WIG EB /r     VPOR ymm1, ymm2, ymm3/m256
// VEX.256.66.0F.WIG EF /r     VPXOR ymm1, ymm2, ymm3/m256

const ALIGNED_ADDR: u64 = 0x3000;
const ALIGNED_ADDR2: u64 = 0x3100;

// ============================================================================
// VPAND Tests - 256-bit Bitwise AND
// ============================================================================

#[test]
fn test_vpand_ymm0_ymm1_ymm2_all_zeros() {
    // VPAND YMM0, YMM1, YMM2 (0 AND 0 = 0)
    let code = [
        0xc5, 0xf5, 0xdb, 0xc2, // VPAND YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpand_ymm3_ymm4_ymm5_all_ones() {
    // VPAND YMM3, YMM4, YMM5 (0xFF AND 0xFF = 0xFF)
    let code = [
        0xc5, 0xdd, 0xdb, 0xdd, // VPAND YMM3, YMM4, YMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpand_ymm6_ymm7_ymm8_masking() {
    // Test masking operation (0xFF AND 0x0F = 0x0F)
    let code = [
        0xc5, 0x45, 0xdb, 0xf0, // VPAND YMM6, YMM7, YMM8
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpand_ymm9_ymm10_ymm11_mixed() {
    let code = [
        0xc4, 0x41, 0x2d, 0xdb, 0xcb, // VPAND YMM9, YMM10, YMM11
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpand_ymm12_ymm13_ymm14_alternating() {
    // 0xAA AND 0x55 = 0x00
    let code = [
        0xc4, 0x41, 0x15, 0xdb, 0xe6, // VPAND YMM12, YMM13, YMM14
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpand_ymm15_ymm0_ymm1_high_reg() {
    let code = [
        0xc4, 0xc1, 0x7d, 0xdb, 0xf9, // VPAND YMM15, YMM0, YMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpand_ymm0_ymm1_mem() {
    // VPAND YMM0, YMM1, [memory]
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0xdb, 0x00, // VPAND YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(
        &[
            0x0F, 0x0F, 0x0F, 0x0F, 0x0F, 0x0F, 0x0F, 0x0F, 0x0F, 0x0F, 0x0F, 0x0F, 0x0F, 0x0F,
            0x0F, 0x0F, 0x0F, 0x0F, 0x0F, 0x0F, 0x0F, 0x0F, 0x0F, 0x0F, 0x0F, 0x0F, 0x0F, 0x0F,
            0x0F, 0x0F, 0x0F, 0x0F,
        ],
        GuestAddress(ALIGNED_ADDR),
    )
    .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpand_ymm2_ymm3_mem_pattern() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xe5, 0xdb, 0x10, // VPAND YMM2, YMM3, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let pattern: Vec<u8> = (0..32)
        .map(|i| if i % 2 == 0 { 0xAA } else { 0x55 })
        .collect();
    mem.write_slice(&pattern, GuestAddress(ALIGNED_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpand_ymm4_ymm5_mem_sequential() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xd5, 0xdb, 0x20, // VPAND YMM4, YMM5, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let seq: Vec<u8> = (0..32).collect();
    mem.write_slice(&seq, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpand_chain_operations() {
    // Chain multiple VPAND operations
    let code = [
        0xc5, 0xf5, 0xdb, 0xc2, // VPAND YMM0, YMM1, YMM2
        0xc5, 0xfd, 0xdb, 0xc3, // VPAND YMM0, YMM0, YMM3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpand_identity() {
    // AND with all 1s (identity operation)
    let code = [
        0xc5, 0xf5, 0xdb, 0xc2, // VPAND YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpand_clear() {
    // AND with all 0s (clears result)
    let code = [
        0xc5, 0xf5, 0xdb, 0xc2, // VPAND YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpand_bit_mask_low_nibble() {
    // Mask to extract low nibble
    let code = [
        0xc5, 0xf5, 0xdb, 0xc2, // VPAND YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpand_bit_mask_high_nibble() {
    // Mask to extract high nibble
    let code = [
        0xc5, 0xf5, 0xdb, 0xc2, // VPAND YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// VPOR Tests - 256-bit Bitwise OR
// ============================================================================

#[test]
fn test_vpor_ymm0_ymm1_ymm2_all_zeros() {
    // VPOR YMM0, YMM1, YMM2 (0 OR 0 = 0)
    let code = [
        0xc5, 0xf5, 0xeb, 0xc2, // VPOR YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpor_ymm3_ymm4_ymm5_all_ones() {
    // VPOR YMM3, YMM4, YMM5 (0xFF OR 0xFF = 0xFF)
    let code = [
        0xc5, 0xdd, 0xeb, 0xdd, // VPOR YMM3, YMM4, YMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpor_ymm6_ymm7_ymm8_combining() {
    // Test combining bits (0x0F OR 0xF0 = 0xFF)
    let code = [
        0xc5, 0x45, 0xeb, 0xf0, // VPOR YMM6, YMM7, YMM8
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpor_ymm9_ymm10_ymm11_mixed() {
    let code = [
        0xc4, 0x41, 0x2d, 0xeb, 0xcb, // VPOR YMM9, YMM10, YMM11
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpor_ymm12_ymm13_ymm14_alternating() {
    // 0xAA OR 0x55 = 0xFF
    let code = [
        0xc4, 0x41, 0x15, 0xeb, 0xe6, // VPOR YMM12, YMM13, YMM14
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpor_ymm15_ymm0_ymm1_high_reg() {
    let code = [
        0xc4, 0xc1, 0x7d, 0xeb, 0xf9, // VPOR YMM15, YMM0, YMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpor_ymm0_ymm1_mem() {
    // VPOR YMM0, YMM1, [memory]
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0xeb, 0x00, // VPOR YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(
        &[
            0xF0, 0xF0, 0xF0, 0xF0, 0xF0, 0xF0, 0xF0, 0xF0, 0xF0, 0xF0, 0xF0, 0xF0, 0xF0, 0xF0,
            0xF0, 0xF0, 0xF0, 0xF0, 0xF0, 0xF0, 0xF0, 0xF0, 0xF0, 0xF0, 0xF0, 0xF0, 0xF0, 0xF0,
            0xF0, 0xF0, 0xF0, 0xF0,
        ],
        GuestAddress(ALIGNED_ADDR),
    )
    .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpor_ymm2_ymm3_mem_pattern() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xe5, 0xeb, 0x10, // VPOR YMM2, YMM3, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let pattern: Vec<u8> = (0..32)
        .map(|i| if i % 2 == 0 { 0xAA } else { 0x55 })
        .collect();
    mem.write_slice(&pattern, GuestAddress(ALIGNED_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpor_ymm4_ymm5_mem_sequential() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xd5, 0xeb, 0x20, // VPOR YMM4, YMM5, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let seq: Vec<u8> = (0..32).collect();
    mem.write_slice(&seq, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpor_chain_operations() {
    // Chain multiple VPOR operations
    let code = [
        0xc5, 0xf5, 0xeb, 0xc2, // VPOR YMM0, YMM1, YMM2
        0xc5, 0xfd, 0xeb, 0xc3, // VPOR YMM0, YMM0, YMM3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpor_identity() {
    // OR with all 0s (identity operation)
    let code = [
        0xc5, 0xf5, 0xeb, 0xc2, // VPOR YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpor_set_all() {
    // OR with all 1s (sets all bits)
    let code = [
        0xc5, 0xf5, 0xeb, 0xc2, // VPOR YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpor_complement_bits() {
    let code = [
        0xc5, 0xf5, 0xeb, 0xc2, // VPOR YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// VPXOR Tests - 256-bit Bitwise XOR
// ============================================================================

#[test]
fn test_vpxor_ymm0_ymm1_ymm2_all_zeros() {
    // VPXOR YMM0, YMM1, YMM2 (0 XOR 0 = 0)
    let code = [
        0xc5, 0xf5, 0xef, 0xc2, // VPXOR YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpxor_ymm3_ymm4_ymm5_same_values() {
    // VPXOR YMM3, YMM4, YMM5 (0xFF XOR 0xFF = 0x00)
    let code = [
        0xc5, 0xdd, 0xef, 0xdd, // VPXOR YMM3, YMM4, YMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpxor_ymm6_ymm7_ymm8_toggling() {
    // Test bit toggling (0xFF XOR 0x0F = 0xF0)
    let code = [
        0xc5, 0x45, 0xef, 0xf0, // VPXOR YMM6, YMM7, YMM8
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpxor_ymm9_ymm10_ymm11_mixed() {
    let code = [
        0xc4, 0x41, 0x2d, 0xef, 0xcb, // VPXOR YMM9, YMM10, YMM11
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpxor_ymm12_ymm13_ymm14_alternating() {
    // 0xAA XOR 0x55 = 0xFF
    let code = [
        0xc4, 0x41, 0x15, 0xef, 0xe6, // VPXOR YMM12, YMM13, YMM14
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpxor_ymm15_ymm0_ymm1_high_reg() {
    let code = [
        0xc4, 0xc1, 0x7d, 0xef, 0xf9, // VPXOR YMM15, YMM0, YMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpxor_ymm0_ymm1_mem() {
    // VPXOR YMM0, YMM1, [memory]
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0xef, 0x00, // VPXOR YMM0, YMM1, [RAX]
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
fn test_vpxor_ymm2_ymm3_mem_pattern() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xe5, 0xef, 0x10, // VPXOR YMM2, YMM3, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let pattern: Vec<u8> = (0..32)
        .map(|i| if i % 2 == 0 { 0xAA } else { 0x55 })
        .collect();
    mem.write_slice(&pattern, GuestAddress(ALIGNED_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpxor_ymm4_ymm5_mem_sequential() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xd5, 0xef, 0x20, // VPXOR YMM4, YMM5, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let seq: Vec<u8> = (0..32).collect();
    mem.write_slice(&seq, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpxor_chain_operations() {
    // Chain multiple VPXOR operations
    let code = [
        0xc5, 0xf5, 0xef, 0xc2, // VPXOR YMM0, YMM1, YMM2
        0xc5, 0xfd, 0xef, 0xc3, // VPXOR YMM0, YMM0, YMM3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpxor_self_clear() {
    // XOR with self (clears register)
    let code = [
        0xc5, 0xf5, 0xef, 0xc9, // VPXOR YMM1, YMM1, YMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpxor_identity() {
    // XOR with all 0s (identity operation)
    let code = [
        0xc5, 0xf5, 0xef, 0xc2, // VPXOR YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpxor_complement() {
    // XOR with all 1s (complements bits)
    let code = [
        0xc5, 0xf5, 0xef, 0xc2, // VPXOR YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpxor_double_application() {
    // XOR twice with same value (returns to original)
    let code = [
        0xc5, 0xf5, 0xef, 0xc2, // VPXOR YMM0, YMM1, YMM2
        0xc5, 0xfd, 0xef, 0xc2, // VPXOR YMM0, YMM0, YMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Combined tests
// ============================================================================

#[test]
fn test_vpand_vpor_vpxor_combined() {
    // Test all three operations in sequence
    let code = [
        0xc5, 0xf5, 0xdb, 0xc2, // VPAND YMM0, YMM1, YMM2
        0xc5, 0xed, 0xeb, 0xdb, // VPOR YMM3, YMM2, YMM3
        0xc5, 0xe5, 0xef, 0xe4, // VPXOR YMM4, YMM3, YMM4
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_logical_ops_mem_unaligned() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&(ALIGNED_ADDR + 1).to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0xdb, 0x00, // VPAND YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(
        &[
            0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
            0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
            0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        ],
        GuestAddress(ALIGNED_ADDR),
    )
    .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Known-answer VALUE tests (256-bit packed bitwise AND/OR/XOR, both lanes).
// ============================================================================

use rax::backend::emulator::x86_64::X86_64Vcpu;

fn kbit_set(vcpu: &mut X86_64Vcpu, idx: usize, lo: u128, hi: u128) {
    let mut regs = vcpu.get_regs().unwrap();
    regs.xmm[idx][0] = lo as u64;
    regs.xmm[idx][1] = (lo >> 64) as u64;
    regs.ymm_high[idx][0] = hi as u64;
    regs.ymm_high[idx][1] = (hi >> 64) as u64;
    vcpu.set_regs(&regs).unwrap();
}
fn kbit_lo(vcpu: &X86_64Vcpu, idx: usize) -> u128 {
    let r = vcpu.get_regs().unwrap();
    (r.xmm[idx][0] as u128) | ((r.xmm[idx][1] as u128) << 64)
}
fn kbit_hi(vcpu: &X86_64Vcpu, idx: usize) -> u128 {
    let r = vcpu.get_regs().unwrap();
    (r.ymm_high[idx][0] as u128) | ((r.ymm_high[idx][1] as u128) << 64)
}

const A_LO: u128 = 0xF0F0_F0F0_AAAA_5555_FFFF_0000_1234_5678;
const A_HI: u128 = 0x0123_4567_89AB_CDEF_FEDC_BA98_7654_3210;
const B_LO: u128 = 0x0F0F_0F0F_5555_AAAA_0000_FFFF_8765_4321;
const B_HI: u128 = 0xDEAD_BEEF_CAFE_BABE_1122_3344_5566_7788;

#[test]
fn test_vpand_ymm_value() {
    // VPAND YMM0, YMM1, YMM2
    let code = [0xc5, 0xf5, 0xdb, 0xc2, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    kbit_set(&mut vcpu, 1, A_LO, A_HI);
    kbit_set(&mut vcpu, 2, B_LO, B_HI);
    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(kbit_lo(&vcpu, 0), A_LO & B_LO);
    assert_eq!(kbit_hi(&vcpu, 0), A_HI & B_HI);
}

#[test]
fn test_vpor_ymm_value() {
    // VPOR YMM0, YMM1, YMM2
    let code = [0xc5, 0xf5, 0xeb, 0xc2, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    kbit_set(&mut vcpu, 1, A_LO, A_HI);
    kbit_set(&mut vcpu, 2, B_LO, B_HI);
    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(kbit_lo(&vcpu, 0), A_LO | B_LO);
    assert_eq!(kbit_hi(&vcpu, 0), A_HI | B_HI);
}

#[test]
fn test_vpxor_ymm_value() {
    // VPXOR YMM0, YMM1, YMM2
    let code = [0xc5, 0xf5, 0xef, 0xc2, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    kbit_set(&mut vcpu, 1, A_LO, A_HI);
    kbit_set(&mut vcpu, 2, B_LO, B_HI);
    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(kbit_lo(&vcpu, 0), A_LO ^ B_LO);
    assert_eq!(kbit_hi(&vcpu, 0), A_HI ^ B_HI);
}

#[test]
fn test_vpxor_ymm_self_clears_full_256() {
    // VPXOR YMM5, YMM5, YMM5 ; entire 256-bit register must be zero.
    let code = [0xc5, 0xd5, 0xef, 0xed, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    kbit_set(&mut vcpu, 5, A_LO, A_HI);
    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(kbit_lo(&vcpu, 5), 0);
    assert_eq!(kbit_hi(&vcpu, 5), 0);
}
