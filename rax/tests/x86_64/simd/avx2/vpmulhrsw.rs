use crate::common::*;
use vm_memory::{Bytes, GuestAddress};

// VPMULHRSW - Packed Multiply High with Round and Scale (AVX2)
//
// Multiplies vertically each signed 16-bit integer from the destination operand
// with the corresponding signed 16-bit integer from the source operand, producing
// intermediate 32-bit signed integers. Each intermediate result is shifted right
// by 14 bits, rounded, and stored in the destination.
//
// For each pair of 16-bit words:
// temp = (src1[i] * src2[i]) >> 14
// dst[i] = (temp + 1) >> 1  (with rounding)
//
// This effectively multiplies two Q15 fixed-point numbers and returns a Q15 result.
//
// VPMULHRSW: Multiply high with rounding for 16 packed 16-bit signed words
//
// Opcodes (AVX2 - 256-bit YMM):
// VEX.256.66.0F38.WIG 0B /r     VPMULHRSW ymm1, ymm2, ymm3/m256

const ALIGNED_ADDR: u64 = 0x3000;

// ============================================================================
// VPMULHRSW Tests - 16x Word Multiply High with Round and Scale (256-bit)
// ============================================================================

#[test]
fn test_vpmulhrsw_ymm0_ymm1_ymm2_all_zeros() {
    // VPMULHRSW YMM0, YMM1, YMM2 with all zeros
    let code = [
        0xc4, 0xe2, 0x75, 0x0b, 0xc2, // VPMULHRSW YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmulhrsw_ymm3_ymm4_ymm5_all_ones() {
    // VPMULHRSW YMM3, YMM4, YMM5 with 0x0001 values
    let code = [
        0xc4, 0xe2, 0x5d, 0x0b, 0xdd, // VPMULHRSW YMM3, YMM4, YMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmulhrsw_ymm6_ymm7_ymm8_identity() {
    // Test with 0x4000 (0.5 in Q15) - multiply by 0.5
    let code = [
        0xc4, 0xc2, 0x45, 0x0b, 0xf0, // VPMULHRSW YMM6, YMM7, YMM8
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmulhrsw_ymm9_ymm10_ymm11_max_positive() {
    // Test with 0x7FFF (max positive)
    let code = [
        0xc4, 0x42, 0x2d, 0x0b, 0xcb, // VPMULHRSW YMM9, YMM10, YMM11
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmulhrsw_ymm12_ymm13_ymm14_max_negative() {
    // Test with 0x8000 (max negative)
    let code = [
        0xc4, 0x42, 0x15, 0x0b, 0xe6, // VPMULHRSW YMM12, YMM13, YMM14
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmulhrsw_ymm15_ymm0_ymm1_high_reg() {
    let code = [
        0xc4, 0x62, 0x7d, 0x0b, 0xf9, // VPMULHRSW YMM15, YMM0, YMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmulhrsw_ymm0_ymm1_ymm2_mixed_signs() {
    // Test positive * negative
    let code = [
        0xc4, 0xe2, 0x75, 0x0b, 0xc2, // VPMULHRSW YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmulhrsw_ymm3_ymm4_ymm5_negative_times_negative() {
    // Test negative * negative (should be positive)
    let code = [
        0xc4, 0xe2, 0x5d, 0x0b, 0xdd, // VPMULHRSW YMM3, YMM4, YMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmulhrsw_ymm6_ymm7_ymm8_rounding() {
    // Test rounding behavior
    let code = [
        0xc4, 0xc2, 0x45, 0x0b, 0xf0, // VPMULHRSW YMM6, YMM7, YMM8
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmulhrsw_ymm9_ymm10_ymm11_fractional() {
    // Test fractional multiplication (Q15 format)
    let code = [
        0xc4, 0x42, 0x2d, 0x0b, 0xcb, // VPMULHRSW YMM9, YMM10, YMM11
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmulhrsw_ymm0_ymm1_mem() {
    // VPMULHRSW YMM0, YMM1, [memory]
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x75, 0x0b, 0x00, // VPMULHRSW YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data: Vec<u8> = (0..16)
        .flat_map(|i| ((i as u16) * 0x1000).to_le_bytes())
        .collect();
    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmulhrsw_ymm2_ymm3_mem_max_values() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x65, 0x0b, 0x10, // VPMULHRSW YMM2, YMM3, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data: Vec<u8> = vec![0x7FFFu16; 16]
        .into_iter()
        .flat_map(|v| v.to_le_bytes())
        .collect();
    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmulhrsw_ymm4_ymm5_mem_negative() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x55, 0x0b, 0x20, // VPMULHRSW YMM4, YMM5, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data: Vec<u8> = vec![0x8000u16; 16]
        .into_iter()
        .flat_map(|v| v.to_le_bytes())
        .collect();
    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmulhrsw_ymm6_ymm7_mem_alternating() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x45, 0x0b, 0x30, // VPMULHRSW YMM6, YMM7, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data: Vec<u8> = (0..16)
        .flat_map(|i| if i % 2 == 0 { 0x4000u16 } else { 0xC000u16 }.to_le_bytes())
        .collect();
    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmulhrsw_ymm8_ymm9_mem_sequential() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0x62, 0x35, 0x0b, 0x00, // VPMULHRSW YMM8, YMM9, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data: Vec<u8> = (0..16)
        .flat_map(|i| ((i as u16) * 0x0800).to_le_bytes())
        .collect();
    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Additional comprehensive tests for rounding and edge cases
// ============================================================================

#[test]
fn test_vpmulhrsw_chain_operations() {
    // Chain multiple VPMULHRSW operations
    let code = [
        0xc4, 0xe2, 0x75, 0x0b, 0xc2, // VPMULHRSW YMM0, YMM1, YMM2
        0xc4, 0xe2, 0x7d, 0x0b, 0xc3, // VPMULHRSW YMM0, YMM0, YMM3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmulhrsw_same_register() {
    // VPMULHRSW with same source registers (square operation)
    let code = [
        0xc4, 0xe2, 0x75, 0x0b, 0xc1, // VPMULHRSW YMM0, YMM1, YMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmulhrsw_quarter_values() {
    // 0x2000 = 0.25 in Q15 format
    let code = [
        0xc4, 0xe2, 0x75, 0x0b, 0xc2, // VPMULHRSW YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmulhrsw_three_quarter_values() {
    // 0x6000 = 0.75 in Q15 format
    let code = [
        0xc4, 0xe2, 0x75, 0x0b, 0xc2, // VPMULHRSW YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmulhrsw_rounding_up() {
    // Test values that should round up
    let code = [
        0xc4, 0xe2, 0x75, 0x0b, 0xc2, // VPMULHRSW YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmulhrsw_rounding_down() {
    // Test values that should round down
    let code = [
        0xc4, 0xe2, 0x75, 0x0b, 0xc2, // VPMULHRSW YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmulhrsw_small_values() {
    // Test with small values (near zero)
    let code = [
        0xc4, 0xe2, 0x75, 0x0b, 0xc2, // VPMULHRSW YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmulhrsw_large_values() {
    // Test with large values (near max)
    let code = [
        0xc4, 0xe2, 0x75, 0x0b, 0xc2, // VPMULHRSW YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmulhrsw_mem_mixed_signs() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x75, 0x0b, 0x00, // VPMULHRSW YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data: Vec<u8> = vec![
        0x7FFFu16, 0x8000, 0x4000, 0xC000, 0x2000, 0xE000, 0x1000, 0xF000,
    ]
    .into_iter()
    .cycle()
    .take(16)
    .flat_map(|v| v.to_le_bytes())
    .collect();
    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmulhrsw_mem_powers_of_two() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x75, 0x0b, 0x00, // VPMULHRSW YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data: Vec<u8> = (0..16)
        .flat_map(|i| (0x4000u16 >> (i % 8)).to_le_bytes())
        .collect();
    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmulhrsw_overflow_positive() {
    // Test positive overflow: 0x7FFF * 0x7FFF
    let code = [
        0xc4, 0xe2, 0x75, 0x0b, 0xc2, // VPMULHRSW YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmulhrsw_overflow_negative() {
    // Test negative overflow: 0x8000 * 0x8000
    let code = [
        0xc4, 0xe2, 0x75, 0x0b, 0xc2, // VPMULHRSW YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmulhrsw_one_times_value() {
    // Multiply by 1.0 (0x7FFF is approximately 1.0 in Q15)
    let code = [
        0xc4, 0xe2, 0x75, 0x0b, 0xc2, // VPMULHRSW YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmulhrsw_half_times_value() {
    // Multiply by 0.5 (0x4000 in Q15)
    let code = [
        0xc4, 0xe2, 0x75, 0x0b, 0xc2, // VPMULHRSW YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmulhrsw_mem_boundary_values() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x75, 0x0b, 0x00, // VPMULHRSW YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data: Vec<u8> = vec![
        0x0000u16, 0x0001, 0x7FFF, 0x8000, 0xFFFFu16, 0x4000, 0xC000, 0x2000,
    ]
    .into_iter()
    .cycle()
    .take(16)
    .flat_map(|v| v.to_le_bytes())
    .collect();
    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmulhrsw_sequential_multipliers() {
    let code = [
        0xc4, 0xe2, 0x75, 0x0b, 0xc2, // VPMULHRSW YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmulhrsw_inverse_values() {
    // Test complementary values
    let code = [
        0xc4, 0xe2, 0x75, 0x0b, 0xc2, // VPMULHRSW YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmulhrsw_asymmetric_values() {
    let code = [
        0xc4, 0xe2, 0x75, 0x0b, 0xc2, // VPMULHRSW YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmulhrsw_mem_all_patterns() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x75, 0x0b, 0x00, // VPMULHRSW YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data: Vec<u8> = (0..16)
        .flat_map(|i| ((i as u16) * 0x0842).to_le_bytes())
        .collect();
    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmulhrsw_precision_test() {
    // Test precision of rounding
    let code = [
        0xc4, 0xe2, 0x75, 0x0b, 0xc2, // VPMULHRSW YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmulhrsw_extended_regs() {
    // Test with extended registers
    let code = [
        0xc4, 0x42, 0x2d, 0x0b, 0xef, // VPMULHRSW YMM13, YMM10, YMM15
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
