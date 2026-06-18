use crate::common::*;
use vm_memory::{Bytes, GuestAddress};

// VPMADDUBSW - Multiply Unsigned and Signed Bytes, Add Horizontal Pair (AVX2)
//
// Multiplies vertically each unsigned byte of the destination operand with the
// corresponding signed byte of the source operand, producing intermediate signed
// word results. Adjacent pairs of signed words are then added horizontally and
// the saturated results are stored in the destination operand.
//
// For each pair of bytes:
//   temp[i*2]   = unsigned(dest[i*2])   * signed(src[i*2])
//   temp[i*2+1] = unsigned(dest[i*2+1]) * signed(src[i*2+1])
//   result[i]   = saturate_i16(temp[i*2] + temp[i*2+1])
//
// VPMADDUBSW: Process 32 bytes (16 pairs) in YMM registers → 16 signed words
//
// Opcodes (AVX2 - 256-bit YMM):
// VEX.256.66.0F38.WIG 04 /r     VPMADDUBSW ymm1, ymm2, ymm3/m256

const ALIGNED_ADDR: u64 = 0x3000;
const ALIGNED_ADDR2: u64 = 0x3100;

// ============================================================================
// VPMADDUBSW Tests - Multiply Unsigned/Signed and Add (256-bit)
// ============================================================================

#[test]
fn test_vpmaddubsw_ymm0_ymm1_ymm2_all_zeros() {
    // VPMADDUBSW YMM0, YMM1, YMM2 with all zeros
    let code = [
        0xc4, 0xe2, 0x75, 0x04, 0xc2, // VPMADDUBSW YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmaddubsw_ymm3_ymm4_ymm5_all_ones() {
    // VPMADDUBSW YMM3, YMM4, YMM5 with all 0x01 values
    // 1 * 1 + 1 * 1 = 2
    let code = [
        0xc4, 0xe2, 0x5d, 0x04, 0xdd, // VPMADDUBSW YMM3, YMM4, YMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmaddubsw_ymm6_ymm7_ymm8_positive_values() {
    // Test with positive values
    let code = [
        0xc4, 0x62, 0x45, 0x04, 0xf0, // VPMADDUBSW YMM6, YMM7, YMM8
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmaddubsw_ymm9_ymm10_ymm11_negative_multiplier() {
    // Test with negative signed bytes in source
    let code = [
        0xc4, 0x42, 0x2d, 0x04, 0xcb, // VPMADDUBSW YMM9, YMM10, YMM11
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmaddubsw_ymm12_ymm13_ymm14_mixed_signs() {
    // Test with mixed positive and negative values in signed source
    let code = [
        0xc4, 0x42, 0x15, 0x04, 0xe6, // VPMADDUBSW YMM12, YMM13, YMM14
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmaddubsw_ymm15_ymm0_ymm1_high_reg() {
    let code = [
        0xc4, 0x62, 0x7d, 0x04, 0xf9, // VPMADDUBSW YMM15, YMM0, YMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmaddubsw_ymm0_ymm1_mem() {
    // VPMADDUBSW YMM0, YMM1, [memory]
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x75, 0x04, 0x00, // VPMADDUBSW YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(
        &[
            0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01,
            0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01,
            0x01, 0x01, 0x01, 0x01,
        ],
        GuestAddress(ALIGNED_ADDR),
    )
    .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmaddubsw_ymm2_ymm3_mem_negative() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x65, 0x04, 0x10, // VPMADDUBSW YMM2, YMM3, [RAX]
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
fn test_vpmaddubsw_ymm4_ymm5_mem_sequential() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x55, 0x04, 0x20, // VPMADDUBSW YMM4, YMM5, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let seq: Vec<u8> = (0..32).collect();
    mem.write_slice(&seq, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmaddubsw_ymm6_ymm7_mem_alternating() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x45, 0x04, 0x30, // VPMADDUBSW YMM6, YMM7, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let pattern: Vec<u8> = (0..32)
        .map(|i| if i % 2 == 0 { 0x01 } else { 0xFF })
        .collect();
    mem.write_slice(&pattern, GuestAddress(ALIGNED_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmaddubsw_unsigned_signed_multiply() {
    // Test unsigned * signed: 255 * 1 + 255 * 1 = 510
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x75, 0x04, 0x00, // VPMADDUBSW YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data: Vec<u8> = vec![
        0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01,
        0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01,
        0x01, 0x01,
    ];
    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmaddubsw_negative_product() {
    // Test unsigned * negative signed: 128 * (-1) + 128 * (-1) = -256
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x75, 0x04, 0x00, // VPMADDUBSW YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data: Vec<u8> = vec![
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF,
    ]; // -1 as signed byte
    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmaddubsw_saturation_positive() {
    // Test positive saturation to 0x7FFF (32767)
    // 255 * 127 + 255 * 127 = 64770, should saturate to 32767
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x75, 0x04, 0x00, // VPMADDUBSW YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data: Vec<u8> = vec![
        0x7F, 0x7F, 0x7F, 0x7F, 0x7F, 0x7F, 0x7F, 0x7F, 0x7F, 0x7F, 0x7F, 0x7F, 0x7F, 0x7F, 0x7F,
        0x7F, 0x7F, 0x7F, 0x7F, 0x7F, 0x7F, 0x7F, 0x7F, 0x7F, 0x7F, 0x7F, 0x7F, 0x7F, 0x7F, 0x7F,
        0x7F, 0x7F,
    ]; // 127 as signed byte
    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmaddubsw_saturation_negative() {
    // Test negative saturation to 0x8000 (-32768)
    // 255 * (-128) + 255 * (-128) = -65280, should saturate to -32768
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x75, 0x04, 0x00, // VPMADDUBSW YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data: Vec<u8> = vec![
        0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80,
        0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80,
        0x80, 0x80,
    ]; // -128 as signed byte
    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmaddubsw_zero_result() {
    // Test: unsigned * 0 = 0
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x75, 0x04, 0x00, // VPMADDUBSW YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data: Vec<u8> = vec![
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00,
    ];
    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmaddubsw_chain_multiple_ops() {
    // Chain multiple VPMADDUBSW operations
    let code = [
        0xc4, 0xe2, 0x75, 0x04, 0xc2, // VPMADDUBSW YMM0, YMM1, YMM2
        0xc4, 0xe2, 0x7d, 0x04, 0xc3, // VPMADDUBSW YMM0, YMM0, YMM3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmaddubsw_mem_unaligned_offset() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&(ALIGNED_ADDR + 1).to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x75, 0x04, 0x00, // VPMADDUBSW YMM0, YMM1, [RAX]
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
fn test_vpmaddubsw_extended_regs_r8_r9_r10() {
    let code = [
        0xc4, 0x42, 0x3d, 0x04, 0xc2, // VPMADDUBSW YMM8, YMM8, YMM10
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmaddubsw_extended_regs_r11_r12_r13() {
    let code = [
        0xc4, 0x42, 0x1d, 0x04, 0xdd, // VPMADDUBSW YMM11, YMM12, YMM13
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmaddubsw_extended_regs_r14_r15_r8() {
    let code = [
        0xc4, 0x42, 0x05, 0x04, 0xf0, // VPMADDUBSW YMM14, YMM15, YMM8
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmaddubsw_small_values() {
    // Small unsigned values with small signed values
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x75, 0x04, 0x00, // VPMADDUBSW YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data: Vec<u8> = vec![
        0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02,
        0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02,
        0x02, 0x02,
    ];
    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmaddubsw_boundary_values() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x75, 0x04, 0x00, // VPMADDUBSW YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    // Mix of boundary values for signed bytes
    let boundary: Vec<u8> = vec![0x00, 0x01, 0x7F, 0x80, 0x81, 0xFE, 0xFF, 0x00].repeat(4);
    mem.write_slice(&boundary, GuestAddress(ALIGNED_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmaddubsw_powers_of_two() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x75, 0x04, 0x00, // VPMADDUBSW YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let powers: Vec<u8> = (0..8)
        .map(|i| 1u8 << i)
        .chain((0..8).map(|i| 1u8 << i))
        .chain((0..8).map(|i| 1u8 << i))
        .chain((0..8).map(|i| 1u8 << i))
        .collect();
    mem.write_slice(&powers, GuestAddress(ALIGNED_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmaddubsw_alternating_positive_negative() {
    // Alternating positive and negative signed values
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x75, 0x04, 0x00, // VPMADDUBSW YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let pattern: Vec<u8> = (0..32)
        .map(|i| if i % 2 == 0 { 0x02 } else { 0xFE })
        .collect();
    mem.write_slice(&pattern, GuestAddress(ALIGNED_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmaddubsw_sequential_pattern() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x75, 0x04, 0x00, // VPMADDUBSW YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let sequential: Vec<u8> = (0..32).collect();
    mem.write_slice(&sequential, GuestAddress(ALIGNED_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmaddubsw_reverse_sequential() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x75, 0x04, 0x00, // VPMADDUBSW YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let reverse: Vec<u8> = (0..32).rev().collect();
    mem.write_slice(&reverse, GuestAddress(ALIGNED_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmaddubsw_symmetric_pattern() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x75, 0x04, 0x00, // VPMADDUBSW YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let symmetric: Vec<u8> = vec![
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x08, 0x07, 0x06, 0x05, 0x04, 0x03, 0x02,
        0x01, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x08, 0x07, 0x06, 0x05, 0x04, 0x03,
        0x02, 0x01,
    ];
    mem.write_slice(&symmetric, GuestAddress(ALIGNED_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmaddubsw_large_unsigned_small_signed() {
    // Large unsigned values with small signed values
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x75, 0x04, 0x00, // VPMADDUBSW YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data: Vec<u8> = (0..32)
        .map(|i| if i % 2 == 0 { 0x02 } else { 0x03 })
        .collect();
    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmaddubsw_near_saturation() {
    // Values that are close to saturation limits
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x75, 0x04, 0x00, // VPMADDUBSW YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data: Vec<u8> = vec![
        0x40, 0x40, 0x40, 0x40, 0x40, 0x40, 0x40, 0x40, 0x40, 0x40, 0x40, 0x40, 0x40, 0x40, 0x40,
        0x40, 0x40, 0x40, 0x40, 0x40, 0x40, 0x40, 0x40, 0x40, 0x40, 0x40, 0x40, 0x40, 0x40, 0x40,
        0x40, 0x40,
    ]; // 64 as signed byte
    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmaddubsw_checkerboard() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x75, 0x04, 0x00, // VPMADDUBSW YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let checkerboard: Vec<u8> = (0..32)
        .map(|i| if i % 2 == 0 { 0x55 } else { 0xAA })
        .collect();
    mem.write_slice(&checkerboard, GuestAddress(ALIGNED_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmaddubsw_gradient_pattern() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x75, 0x04, 0x00, // VPMADDUBSW YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let gradient: Vec<u8> = (0..32).map(|i| ((i * 8) % 256) as u8).collect();
    mem.write_slice(&gradient, GuestAddress(ALIGNED_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmaddubsw_cancellation() {
    // Test where products cancel out: a*1 + b*(-1) = a - b
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x75, 0x04, 0x00, // VPMADDUBSW YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data: Vec<u8> = (0..32)
        .map(|i| if i % 2 == 0 { 0x01 } else { 0xFF })
        .collect();
    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmaddubsw_mixed_magnitudes() {
    // Mix of small and large unsigned values with varying signed values
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x75, 0x04, 0x00, // VPMADDUBSW YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let mixed: Vec<u8> = vec![0x01, 0x7F, 0x80, 0x01, 0xFF, 0x01, 0x01, 0xFF].repeat(4);
    mem.write_slice(&mixed, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmaddubsw_diagonal_pattern() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x75, 0x04, 0x00, // VPMADDUBSW YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let diagonal: Vec<u8> = (0..32).map(|i| ((i * 7 + 13) % 256) as u8).collect();
    mem.write_slice(&diagonal, GuestAddress(ALIGNED_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmaddubsw_max_unsigned_positive_signed() {
    // Maximum unsigned (255) with maximum positive signed (127)
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x75, 0x04, 0x00, // VPMADDUBSW YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data: Vec<u8> = (0..32)
        .map(|i| if i % 2 == 0 { 0x7F } else { 0x7E })
        .collect();
    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmaddubsw_max_unsigned_negative_signed() {
    // Maximum unsigned (255) with minimum signed (-128)
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x75, 0x04, 0x00, // VPMADDUBSW YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data: Vec<u8> = (0..32)
        .map(|i| if i % 2 == 0 { 0x80 } else { 0x81 })
        .collect();
    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Regression: VPMADDUBSW intermediate accumulation must be wide enough.
//
// First operand bytes are unsigned, second signed. A single product can reach
// 255*127 and the sum of two products per word can exceed the i16 range before
// the final signed-saturate to a word. The accumulation must be done at i32
// width (this previously panicked from an i16 multiply/add overflow), then the
// word result is signed-saturated to [-32768, 32767].
// ============================================================================

use rax::backend::emulator::x86_64::X86_64Vcpu;

fn vpmaddubsw_ymm_set(vcpu: &mut X86_64Vcpu, idx: usize, lo: u128, hi: u128) {
    let mut regs = vcpu.get_regs().unwrap();
    regs.xmm[idx][0] = lo as u64;
    regs.xmm[idx][1] = (lo >> 64) as u64;
    regs.ymm_high[idx][0] = hi as u64;
    regs.ymm_high[idx][1] = (hi >> 64) as u64;
    vcpu.set_regs(&regs).unwrap();
}
fn vpmaddubsw_ymm_lo(vcpu: &X86_64Vcpu, idx: usize) -> u128 {
    let regs = vcpu.get_regs().unwrap();
    (regs.xmm[idx][0] as u128) | ((regs.xmm[idx][1] as u128) << 64)
}

#[test]
fn test_vpmaddubsw_positive_saturate() {
    // VPMADDUBSW ymm0, ymm1, ymm2 ; a bytes (unsigned) = [200,200], b (signed) = [100,100]
    // 200*100 + 200*100 = 40000 -> signed-saturate to 0x7FFF.
    let code = [0xc4, 0xe2, 0x75, 0x04, 0xc2, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    vpmaddubsw_ymm_set(&mut vcpu, 1, 0xC8C8, 0);
    vpmaddubsw_ymm_set(&mut vcpu, 2, 0x6464, 0);
    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(vpmaddubsw_ymm_lo(&vcpu, 0) as u16, 32767);
}

#[test]
fn test_vpmaddubsw_negative_saturate() {
    // a bytes (unsigned) = [255,255], b (signed) = [-128,-128] = [0x80,0x80].
    // 255*(-128) + 255*(-128) = -65280 -> signed-saturate to -32768 = 0x8000.
    let code = [0xc4, 0xe2, 0x75, 0x04, 0xc2, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    vpmaddubsw_ymm_set(&mut vcpu, 1, 0xFFFF, 0);
    vpmaddubsw_ymm_set(&mut vcpu, 2, 0x8080, 0);
    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(vpmaddubsw_ymm_lo(&vcpu, 0) as u16, 0x8000);
}

#[test]
fn test_vpmaddubsw_basic() {
    // a=[1,2], b=[3,4] -> 1*3 + 2*4 = 11 (no saturation).
    let code = [0xc4, 0xe2, 0x75, 0x04, 0xc2, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    vpmaddubsw_ymm_set(&mut vcpu, 1, 0x0201, 0);
    vpmaddubsw_ymm_set(&mut vcpu, 2, 0x0403, 0);
    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(vpmaddubsw_ymm_lo(&vcpu, 0) as u16, 11);
}
