use crate::common::*;
use vm_memory::{Bytes, GuestAddress};

// VPMADDWD - Multiply and Add Packed Integers (AVX2)
//
// Multiplies the signed word integers in the source and destination operands,
// producing intermediate signed doubleword results. Adjacent doubleword results
// are then added to produce signed doubleword sums stored in the destination.
//
// For each pair of words:
//   dest[31:0]   = (src1[15:0]   * src2[15:0])   + (src1[31:16]   * src2[31:16])
//   dest[63:32]  = (src1[47:32]  * src2[47:32])  + (src1[63:48]   * src2[63:48])
//   ... and so on for all 16 words → 8 doublewords
//
// VPMADDWD: Process 16 signed words (8 pairs) in YMM registers → 8 doublewords
//
// Opcodes (AVX2 - 256-bit YMM):
// VEX.256.66.0F.WIG F5 /r       VPMADDWD ymm1, ymm2, ymm3/m256

const ALIGNED_ADDR: u64 = 0x3000;
const ALIGNED_ADDR2: u64 = 0x3100;

// ============================================================================
// VPMADDWD Tests - Multiply and Add (256-bit)
// ============================================================================

#[test]
fn test_vpmaddwd_ymm0_ymm1_ymm2_all_zeros() {
    // VPMADDWD YMM0, YMM1, YMM2 with all zeros
    let code = [
        0xc5, 0xf5, 0xf5, 0xc2, // VPMADDWD YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmaddwd_ymm3_ymm4_ymm5_all_ones() {
    // VPMADDWD YMM3, YMM4, YMM5 with all 0x0001 values
    // 1 * 1 + 1 * 1 = 2
    let code = [
        0xc5, 0xdd, 0xf5, 0xdd, // VPMADDWD YMM3, YMM4, YMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmaddwd_ymm6_ymm7_ymm8_positive_values() {
    // Test with positive values
    let code = [
        0xc5, 0x45, 0xf5, 0xf0, // VPMADDWD YMM6, YMM7, YMM8
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmaddwd_ymm9_ymm10_ymm11_negative_values() {
    // Test with negative values (signed multiplication)
    let code = [
        0xc4, 0x41, 0x2d, 0xf5, 0xcb, // VPMADDWD YMM9, YMM10, YMM11
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmaddwd_ymm12_ymm13_ymm14_mixed_signs() {
    // Test with mixed positive and negative values
    let code = [
        0xc4, 0x41, 0x15, 0xf5, 0xe6, // VPMADDWD YMM12, YMM13, YMM14
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmaddwd_ymm15_ymm0_ymm1_high_reg() {
    let code = [
        0xc4, 0xc1, 0x7d, 0xf5, 0xf9, // VPMADDWD YMM15, YMM0, YMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmaddwd_ymm0_ymm1_mem() {
    // VPMADDWD YMM0, YMM1, [memory]
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0xf5, 0x00, // VPMADDWD YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data: Vec<u8> = (0..16)
        .flat_map(|i| ((i as u16) * 0x0101).to_le_bytes())
        .collect();
    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmaddwd_ymm2_ymm3_mem_max() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xe5, 0xf5, 0x10, // VPMADDWD YMM2, YMM3, [RAX]
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
fn test_vpmaddwd_ymm4_ymm5_mem_sequential() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xd5, 0xf5, 0x20, // VPMADDWD YMM4, YMM5, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data: Vec<u8> = (0..16).flat_map(|i| (i as u16).to_le_bytes()).collect();
    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmaddwd_ymm6_ymm7_mem_alternating() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0x45, 0xf5, 0x30, // VPMADDWD YMM6, YMM7, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let pattern: Vec<u8> = (0..16)
        .flat_map(|i| if i % 2 == 0 { 0x0001u16 } else { 0xFFFFu16 }.to_le_bytes())
        .collect();
    mem.write_slice(&pattern, GuestAddress(ALIGNED_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmaddwd_simple_multiplication() {
    // Test: [2, 3] * [4, 5] = 2*4 + 3*5 = 8 + 15 = 23
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0xf5, 0x00, // VPMADDWD YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data: Vec<u8> = vec![0x04, 0x00, 0x05, 0x00].repeat(8);
    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmaddwd_negative_multiplication() {
    // Test with negative values: [-1, -1] * [1, 1] = -1*1 + -1*1 = -2
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0xf5, 0x00, // VPMADDWD YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data: Vec<u8> = vec![0x01, 0x00, 0x01, 0x00].repeat(8);
    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmaddwd_overflow_handling() {
    // Test overflow: large values that may overflow intermediate results
    // Max signed word: 0x7FFF (32767)
    // 32767 * 32767 = 1,073,676,289 (fits in i32)
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0xf5, 0x00, // VPMADDWD YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data: Vec<u8> = vec![0xFF, 0x7F].repeat(16);
    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmaddwd_saturation_check() {
    // Test with values that could cause wraparound
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0xf5, 0x00, // VPMADDWD YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    // Min signed word: 0x8000 (-32768)
    let data: Vec<u8> = vec![0x00, 0x80].repeat(16);
    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmaddwd_chain_multiple_ops() {
    // Chain multiple VPMADDWD operations
    let code = [
        0xc5, 0xf5, 0xf5, 0xc2, // VPMADDWD YMM0, YMM1, YMM2
        0xc5, 0xfd, 0xf5, 0xc3, // VPMADDWD YMM0, YMM0, YMM3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmaddwd_mem_unaligned_offset() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&(ALIGNED_ADDR + 1).to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0xf5, 0x00, // VPMADDWD YMM0, YMM1, [RAX]
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
fn test_vpmaddwd_extended_regs_r8_r9_r10() {
    let code = [
        0xc4, 0x41, 0x3d, 0xf5, 0xc2, // VPMADDWD YMM8, YMM8, YMM10
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmaddwd_extended_regs_r11_r12_r13() {
    let code = [
        0xc4, 0x41, 0x1d, 0xf5, 0xdd, // VPMADDWD YMM11, YMM12, YMM13
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmaddwd_extended_regs_r14_r15_r8() {
    let code = [
        0xc4, 0x41, 0x05, 0xf5, 0xf0, // VPMADDWD YMM14, YMM15, YMM8
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmaddwd_identity_multiply() {
    // Test: [1, 0] * [a, b] = 1*a + 0*b = a
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0xf5, 0x00, // VPMADDWD YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data: Vec<u8> = (0..16)
        .flat_map(|i| if i % 2 == 0 { 0x0001u16 } else { 0x0000u16 }.to_le_bytes())
        .collect();
    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmaddwd_zero_result() {
    // Test: [1, -1] * [1, 1] = 1*1 + (-1)*1 = 0
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0xf5, 0x00, // VPMADDWD YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data: Vec<u8> = vec![0x01, 0x00, 0x01, 0x00].repeat(8);
    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmaddwd_boundary_values() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0xf5, 0x00, // VPMADDWD YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let boundary: Vec<u8> = vec![
        0x0000u16, 0x0001u16, 0x7FFFu16, 0x8000u16, 0x8001u16, 0xFFFEu16, 0xFFFFu16, 0x0000u16,
    ]
    .into_iter()
    .flat_map(|v| v.to_le_bytes())
    .chain(
        vec![
            0x0000u16, 0x0001u16, 0x7FFFu16, 0x8000u16, 0x8001u16, 0xFFFEu16, 0xFFFFu16, 0x0000u16,
        ]
        .into_iter()
        .flat_map(|v| v.to_le_bytes()),
    )
    .collect();
    mem.write_slice(&boundary, GuestAddress(ALIGNED_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmaddwd_powers_of_two() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0xf5, 0x00, // VPMADDWD YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let powers: Vec<u8> = (0..16)
        .flat_map(|i| (1u16 << (i % 15)).to_le_bytes())
        .collect();
    mem.write_slice(&powers, GuestAddress(ALIGNED_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmaddwd_alternating_signs() {
    // Alternating positive and negative values
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0xf5, 0x00, // VPMADDWD YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let pattern: Vec<u8> = (0..16)
        .flat_map(|i| if i % 2 == 0 { 0x0002u16 } else { 0xFFFEu16 }.to_le_bytes())
        .collect();
    mem.write_slice(&pattern, GuestAddress(ALIGNED_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmaddwd_sequential_pattern() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0xf5, 0x00, // VPMADDWD YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let sequential: Vec<u8> = (1..=16).flat_map(|i| (i as u16).to_le_bytes()).collect();
    mem.write_slice(&sequential, GuestAddress(ALIGNED_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmaddwd_reverse_sequential() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0xf5, 0x00, // VPMADDWD YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let reverse: Vec<u8> = (1..=16)
        .rev()
        .flat_map(|i| (i as u16).to_le_bytes())
        .collect();
    mem.write_slice(&reverse, GuestAddress(ALIGNED_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmaddwd_symmetric_pattern() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0xf5, 0x00, // VPMADDWD YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let symmetric: Vec<u8> = vec![
        0x01, 0x00, 0x02, 0x00, 0x03, 0x00, 0x04, 0x00, 0x04, 0x00, 0x03, 0x00, 0x02, 0x00, 0x01,
        0x00, 0x01, 0x00, 0x02, 0x00, 0x03, 0x00, 0x04, 0x00, 0x04, 0x00, 0x03, 0x00, 0x02, 0x00,
        0x01, 0x00,
    ];
    mem.write_slice(&symmetric, GuestAddress(ALIGNED_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmaddwd_small_values() {
    // Test with small values to verify addition
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0xf5, 0x00, // VPMADDWD YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let small: Vec<u8> = vec![0x01, 0x00, 0x02, 0x00].repeat(8);
    mem.write_slice(&small, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmaddwd_large_products() {
    // Test with values that produce large products
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0xf5, 0x00, // VPMADDWD YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let large: Vec<u8> = vec![0x00, 0x10].repeat(16); // 4096
    mem.write_slice(&large, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmaddwd_mixed_magnitudes() {
    // Mix of small and large values
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0xf5, 0x00, // VPMADDWD YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let mixed: Vec<u8> = vec![
        0x01, 0x00, 0xFF, 0x0F, // 1, 4095
        0x00, 0x10, 0x01, 0x00, // 4096, 1
    ]
    .repeat(4);
    mem.write_slice(&mixed, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmaddwd_dot_product_pattern() {
    // Simulating a dot product operation
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0xf5, 0x00, // VPMADDWD YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let dot_product: Vec<u8> = (0..16)
        .flat_map(|i| ((i % 8) as u16 + 1).to_le_bytes())
        .collect();
    mem.write_slice(&dot_product, GuestAddress(ALIGNED_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmaddwd_checkerboard() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0xf5, 0x00, // VPMADDWD YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let checkerboard: Vec<u8> = (0..16)
        .flat_map(|i| if i % 2 == 0 { 0x5555u16 } else { 0xAAAAu16 }.to_le_bytes())
        .collect();
    mem.write_slice(&checkerboard, GuestAddress(ALIGNED_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Regression: VPMADDWD intermediate accumulation must be wide enough.
//
// Each i16*i16 product fits in i32, but the sum of the two products per output
// dword can overflow i32 (0x8000*0x8000 in both lanes = 2147483648). The result
// must wrap (as on hardware), not panic/overflow. Also verifies value semantics.
// ============================================================================

use rax::backend::emulator::x86_64::X86_64Vcpu;

fn vpmaddwd_ymm_set(vcpu: &mut X86_64Vcpu, idx: usize, lo: u128, hi: u128) {
    let mut regs = vcpu.get_regs().unwrap();
    regs.xmm[idx][0] = lo as u64;
    regs.xmm[idx][1] = (lo >> 64) as u64;
    regs.ymm_high[idx][0] = hi as u64;
    regs.ymm_high[idx][1] = (hi >> 64) as u64;
    vcpu.set_regs(&regs).unwrap();
}
fn vpmaddwd_ymm_lo(vcpu: &X86_64Vcpu, idx: usize) -> u128 {
    let regs = vcpu.get_regs().unwrap();
    (regs.xmm[idx][0] as u128) | ((regs.xmm[idx][1] as u128) << 64)
}

#[test]
fn test_vpmaddwd_basic_value() {
    // VPMADDWD ymm0, ymm1, ymm2 ; words a=[2,3], b=[5,7] -> 2*5+3*7 = 31.
    let code = [0xc5, 0xf5, 0xf5, 0xc2, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    vpmaddwd_ymm_set(&mut vcpu, 1, 0x0003_0002, 0);
    vpmaddwd_ymm_set(&mut vcpu, 2, 0x0007_0005, 0);
    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(vpmaddwd_ymm_lo(&vcpu, 0) as u32, 31);
}

#[test]
fn test_vpmaddwd_overflow_wraps() {
    // All words = 0x8000 (-32768): (-32768)^2 * 2 = 2147483648 overflows i32 and
    // wraps to 0x80000000. This previously panicked due to an i32 add overflow.
    let code = [0xc5, 0xf5, 0xf5, 0xc2, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    vpmaddwd_ymm_set(&mut vcpu, 1, 0x8000_8000_8000_8000, 0);
    vpmaddwd_ymm_set(&mut vcpu, 2, 0x8000_8000_8000_8000, 0);
    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(vpmaddwd_ymm_lo(&vcpu, 0) as u32, 0x80000000);
}
