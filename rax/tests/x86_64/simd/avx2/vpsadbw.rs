use crate::common::*;
use vm_memory::{Bytes, GuestAddress};

// VPSADBW - Compute Sum of Absolute Differences of Packed Unsigned Bytes (AVX2)
//
// Computes the absolute value of the difference of 8 unsigned byte integers from
// the source and destination operands. These 8 differences are then summed to produce
// an unsigned word integer result that is stored in the destination operand.
// The operation is performed separately for the lower and upper 8 bytes in each 128-bit lane.
//
// For each 128-bit lane:
//   - Low 8 bytes: abs_diff summed to qword 0
//   - High 8 bytes: abs_diff summed to qword 1
//
// VPSADBW: Process 256-bit YMM registers (two 128-bit lanes)
//
// Opcodes (AVX2 - 256-bit YMM):
// VEX.256.66.0F.WIG F6 /r       VPSADBW ymm1, ymm2, ymm3/m256

const ALIGNED_ADDR: u64 = 0x3000;
const ALIGNED_ADDR2: u64 = 0x3100;

// ============================================================================
// VPSADBW Tests - Sum of Absolute Differences (256-bit)
// ============================================================================

#[test]
fn test_vpsadbw_ymm0_ymm1_ymm2_all_zeros() {
    // VPSADBW YMM0, YMM1, YMM2 with all zeros
    let code = [
        0xc5, 0xf5, 0xf6, 0xc2, // VPSADBW YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpsadbw_ymm3_ymm4_ymm5_all_ones() {
    // VPSADBW YMM3, YMM4, YMM5 with all 0x01 values
    let code = [
        0xc5, 0xdd, 0xf6, 0xdd, // VPSADBW YMM3, YMM4, YMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpsadbw_ymm6_ymm7_ymm8_identical() {
    // Test when both operands are identical (SAD should be zero)
    let code = [
        0xc5, 0x45, 0xf6, 0xf0, // VPSADBW YMM6, YMM7, YMM8
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpsadbw_ymm9_ymm10_ymm11_mixed_values() {
    // Test with mixed values
    let code = [
        0xc4, 0x41, 0x2d, 0xf6, 0xcb, // VPSADBW YMM9, YMM10, YMM11
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpsadbw_ymm12_ymm13_ymm14_alternating() {
    // Test with alternating values
    let code = [
        0xc4, 0x41, 0x15, 0xf6, 0xe6, // VPSADBW YMM12, YMM13, YMM14
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpsadbw_ymm15_ymm0_ymm1_high_reg() {
    let code = [
        0xc4, 0xc1, 0x7d, 0xf6, 0xf9, // VPSADBW YMM15, YMM0, YMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpsadbw_ymm0_ymm1_mem() {
    // VPSADBW YMM0, YMM1, [memory]
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0xf6, 0x00, // VPSADBW YMM0, YMM1, [RAX]
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
fn test_vpsadbw_ymm2_ymm3_mem_max() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xe5, 0xf6, 0x10, // VPSADBW YMM2, YMM3, [RAX]
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
fn test_vpsadbw_ymm4_ymm5_mem_sequential() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xd5, 0xf6, 0x20, // VPSADBW YMM4, YMM5, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let seq: Vec<u8> = (0..32).collect();
    mem.write_slice(&seq, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpsadbw_ymm6_ymm7_mem_alternating() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0x45, 0xf6, 0x30, // VPSADBW YMM6, YMM7, [RAX]
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

#[test]
fn test_vpsadbw_absolute_difference() {
    // Test that differences are absolute (no sign)
    // e.g., abs(0x00 - 0xFF) = 0xFF, abs(0xFF - 0x00) = 0xFF
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0xf6, 0x00, // VPSADBW YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data: Vec<u8> = vec![
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF,
    ];
    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpsadbw_zero_difference() {
    // When src1 == src2, all differences should be zero
    let code = [
        0xc5, 0xf5, 0xf6, 0xc1, // VPSADBW YMM0, YMM1, YMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpsadbw_max_difference() {
    // Maximum difference: 0xFF - 0x00 = 0xFF per byte
    // Sum of 8 bytes: 8 * 0xFF = 0x7F8
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0xf6, 0x00, // VPSADBW YMM0, YMM1, [RAX]
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
fn test_vpsadbw_lane_independence() {
    // Test that the two 128-bit lanes are processed independently
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0xf6, 0x00, // VPSADBW YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    // First 16 bytes: 0x01, Second 16 bytes: 0x02
    let mut data = vec![
        0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01,
        0x01,
    ];
    data.extend(vec![
        0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02,
        0x02,
    ]);
    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpsadbw_qword_grouping() {
    // Within each 128-bit lane, there are two qwords
    // Each qword gets the SAD of 8 bytes
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0xf6, 0x00, // VPSADBW YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    // Pattern: first 8 bytes = 0x00, next 8 bytes = 0xFF, repeat
    let mut data = vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    data.extend(vec![0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);
    data.extend(vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
    data.extend(vec![0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);
    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpsadbw_chain_multiple_ops() {
    // Chain multiple VPSADBW operations
    let code = [
        0xc5, 0xf5, 0xf6, 0xc2, // VPSADBW YMM0, YMM1, YMM2
        0xc5, 0xfd, 0xf6, 0xc3, // VPSADBW YMM0, YMM0, YMM3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpsadbw_mem_unaligned_offset() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&(ALIGNED_ADDR + 1).to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0xf6, 0x00, // VPSADBW YMM0, YMM1, [RAX]
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
fn test_vpsadbw_extended_regs_r8_r9_r10() {
    let code = [
        0xc4, 0x41, 0x3d, 0xf6, 0xc2, // VPSADBW YMM8, YMM8, YMM10
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpsadbw_extended_regs_r11_r12_r13() {
    let code = [
        0xc4, 0x41, 0x1d, 0xf6, 0xdd, // VPSADBW YMM11, YMM12, YMM13
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpsadbw_extended_regs_r14_r15_r8() {
    let code = [
        0xc4, 0x41, 0x05, 0xf6, 0xf0, // VPSADBW YMM14, YMM15, YMM8
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpsadbw_pattern_1() {
    // Test specific pattern: incremental differences
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0xf6, 0x00, // VPSADBW YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data: Vec<u8> = (0..32u8).map(|i| i * 2).collect();
    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpsadbw_pattern_2() {
    // Test specific pattern: each byte differs by 1
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0xf6, 0x00, // VPSADBW YMM0, YMM1, [RAX]
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
fn test_vpsadbw_pattern_3() {
    // Test with alternating high/low differences
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0xf6, 0x00, // VPSADBW YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let pattern: Vec<u8> = (0..32)
        .map(|i| if i % 2 == 0 { 0x80 } else { 0x7F })
        .collect();
    mem.write_slice(&pattern, GuestAddress(ALIGNED_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpsadbw_boundary_values() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0xf6, 0x00, // VPSADBW YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let boundary: Vec<u8> = vec![0x00, 0x01, 0x7F, 0x80, 0x81, 0xFE, 0xFF, 0x00].repeat(4);
    mem.write_slice(&boundary, GuestAddress(ALIGNED_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpsadbw_reverse_sequential() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0xf6, 0x00, // VPSADBW YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let pattern: Vec<u8> = (0..32).rev().collect();
    mem.write_slice(&pattern, GuestAddress(ALIGNED_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpsadbw_small_differences() {
    // Test small differences (should accumulate correctly)
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0xf6, 0x00, // VPSADBW YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data: Vec<u8> = (0..32).map(|_| 0x02).collect();
    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpsadbw_mixed_differences() {
    // Test with varying differences per byte
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0xf6, 0x00, // VPSADBW YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data: Vec<u8> = vec![0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08].repeat(4);
    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpsadbw_powers_of_two() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0xf6, 0x00, // VPSADBW YMM0, YMM1, [RAX]
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
fn test_vpsadbw_gradient_pattern() {
    // Gradient pattern: 0, 16, 32, 48, ...
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0xf6, 0x00, // VPSADBW YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let gradient: Vec<u8> = (0..32).map(|i| ((i * 16) % 256) as u8).collect();
    mem.write_slice(&gradient, GuestAddress(ALIGNED_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpsadbw_checkerboard_pattern() {
    // Checkerboard: alternating between two values
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0xf6, 0x00, // VPSADBW YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let checkerboard: Vec<u8> = (0..32)
        .map(|i| if i % 2 == 0 { 0xAA } else { 0x55 })
        .collect();
    mem.write_slice(&checkerboard, GuestAddress(ALIGNED_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpsadbw_diagonal_pattern() {
    // Diagonal pattern with increasing values
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0xf6, 0x00, // VPSADBW YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let diagonal: Vec<u8> = (0..32).map(|i| ((i * 7 + 13) % 256) as u8).collect();
    mem.write_slice(&diagonal, GuestAddress(ALIGNED_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpsadbw_symmetric_pattern() {
    // Symmetric pattern: same values mirrored
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0xf6, 0x00, // VPSADBW YMM0, YMM1, [RAX]
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
fn test_vpsadbw_all_max_diff() {
    // All bytes have maximum difference (0xFF vs 0x00)
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0xf6, 0x00, // VPSADBW YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let all_max: Vec<u8> = vec![
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00,
    ];
    mem.write_slice(&all_max, GuestAddress(ALIGNED_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpsadbw_partial_match() {
    // Some bytes match, some don't
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0xf6, 0x00, // VPSADBW YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let partial: Vec<u8> = (0..32)
        .map(|i| if i % 4 == 0 { 0x00 } else { 0x10 })
        .collect();
    mem.write_slice(&partial, GuestAddress(ALIGNED_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpsadbw_fibonacci_like() {
    // Fibonacci-like pattern
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0xf6, 0x00, // VPSADBW YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let mut fib = vec![0u8, 1u8];
    for i in 2..32 {
        fib.push(fib[i - 1].wrapping_add(fib[i - 2]));
    }
    mem.write_slice(&fib, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}
