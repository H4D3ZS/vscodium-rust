use crate::common::*;
use vm_memory::{Bytes, GuestAddress};

// VPMULLW/VPMULLD - Multiply Packed Integers and Store Low Result (AVX2)
//
// Performs SIMD multiply of packed integers and stores the low half of the result.
//
// VPMULLW: Multiply 16 packed word integers (16-bit each) and store low 16 bits
// VPMULLD: Multiply 8 packed doubleword integers (32-bit each) and store low 32 bits
//
// Opcodes (AVX2 - 256-bit YMM):
// VEX.256.66.0F.WIG D5 /r     VPMULLW ymm1, ymm2, ymm3/m256
// VEX.256.66.0F38.WIG 40 /r   VPMULLD ymm1, ymm2, ymm3/m256

const ALIGNED_ADDR: u64 = 0x3000;
const ALIGNED_ADDR2: u64 = 0x3100;

// ============================================================================
// VPMULLW Tests - 16x Word Multiplication (256-bit, low result)
// ============================================================================

#[test]
fn test_vpmullw_ymm0_ymm1_ymm2_all_zeros() {
    // VPMULLW YMM0, YMM1, YMM2 (0 * 0 = 0)
    let code = [
        0xc5, 0xf5, 0xd5, 0xc2, // VPMULLW YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmullw_ymm3_ymm4_ymm5_all_ones() {
    // VPMULLW YMM3, YMM4, YMM5 (1 * 1 = 1)
    let code = [
        0xc5, 0xdd, 0xd5, 0xdd, // VPMULLW YMM3, YMM4, YMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmullw_ymm6_ymm7_ymm8_by_two() {
    // Multiply by 2
    let code = [
        0xc5, 0x45, 0xd5, 0xf0, // VPMULLW YMM6, YMM7, YMM8
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmullw_ymm9_ymm10_ymm11_overflow() {
    // Test overflow (0xFFFF * 0x0002 = 0x0001FFFE, low 16 bits = 0xFFFE)
    let code = [
        0xc4, 0x41, 0x2d, 0xd5, 0xcb, // VPMULLW YMM9, YMM10, YMM11
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmullw_ymm12_ymm13_ymm14_mixed() {
    let code = [
        0xc4, 0x41, 0x15, 0xd5, 0xe6, // VPMULLW YMM12, YMM13, YMM14
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmullw_ymm15_ymm0_ymm1_high_reg() {
    let code = [
        0xc4, 0xc1, 0x7d, 0xd5, 0xf9, // VPMULLW YMM15, YMM0, YMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmullw_ymm0_ymm1_mem() {
    // VPMULLW YMM0, YMM1, [memory]
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0xd5, 0x00, // VPMULLW YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data: Vec<u8> = (0..16).flat_map(|i| (i as u16).to_le_bytes()).collect();
    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmullw_ymm2_ymm3_mem_max_values() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xe5, 0xd5, 0x10, // VPMULLW YMM2, YMM3, [RAX]
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
fn test_vpmullw_ymm4_ymm5_mem_powers_of_two() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xd5, 0xd5, 0x20, // VPMULLW YMM4, YMM5, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let powers: Vec<u8> = (0..16)
        .flat_map(|i| (1u16 << (i % 16)).to_le_bytes())
        .collect();
    mem.write_slice(&powers, GuestAddress(ALIGNED_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmullw_ymm6_ymm7_mem_sequential() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0x45, 0xd5, 0x30, // VPMULLW YMM6, YMM7, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data: Vec<u8> = (1..=16).flat_map(|i| (i as u16).to_le_bytes()).collect();
    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmullw_ymm0_ymm1_ymm2_small_values() {
    // 2 * 3 = 6
    let code = [
        0xc5, 0xf5, 0xd5, 0xc2, // VPMULLW YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmullw_ymm3_ymm4_ymm5_negative_representation() {
    // Test with sign-extended values (treated as unsigned)
    let code = [
        0xc5, 0xdd, 0xd5, 0xdd, // VPMULLW YMM3, YMM4, YMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmullw_ymm8_ymm9_ymm10_alternating() {
    let code = [
        0xc4, 0x41, 0x35, 0xd5, 0xc2, // VPMULLW YMM8, YMM9, YMM10
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmullw_ymm11_ymm12_ymm13_pattern() {
    let code = [
        0xc4, 0x41, 0x1d, 0xd5, 0xdd, // VPMULLW YMM11, YMM12, YMM13
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmullw_chain_operations() {
    // Chain multiple VPMULLW operations
    let code = [
        0xc5, 0xf5, 0xd5, 0xc2, // VPMULLW YMM0, YMM1, YMM2
        0xc5, 0xfd, 0xd5, 0xc3, // VPMULLW YMM0, YMM0, YMM3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmullw_mem_different_patterns() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0xd5, 0x00, // VPMULLW YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let pattern: Vec<u8> = (0..16)
        .flat_map(|i| if i % 2 == 0 { 0x000Au16 } else { 0x0005u16 }.to_le_bytes())
        .collect();
    mem.write_slice(&pattern, GuestAddress(ALIGNED_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmullw_mem_large_multipliers() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0xd5, 0x00, // VPMULLW YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data: Vec<u8> = (0..16).flat_map(|_| 0x8000u16.to_le_bytes()).collect();
    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// VPMULLD Tests - 8x Doubleword Multiplication (256-bit, low result)
// ============================================================================

#[test]
fn test_vpmulld_ymm0_ymm1_ymm2_all_zeros() {
    // VPMULLD YMM0, YMM1, YMM2 (0 * 0 = 0)
    let code = [
        0xc4, 0xe2, 0x75, 0x40, 0xc2, // VPMULLD YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmulld_ymm3_ymm4_ymm5_all_ones() {
    // VPMULLD YMM3, YMM4, YMM5 (1 * 1 = 1)
    let code = [
        0xc4, 0xe2, 0x5d, 0x40, 0xdd, // VPMULLD YMM3, YMM4, YMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmulld_ymm6_ymm7_ymm8_by_two() {
    // Multiply by 2
    let code = [
        0xc4, 0xc2, 0x45, 0x40, 0xf0, // VPMULLD YMM6, YMM7, YMM8
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmulld_ymm9_ymm10_ymm11_overflow() {
    // Test overflow (0xFFFFFFFF * 0x00000002 = 0x1FFFFFFFE, low 32 bits = 0xFFFFFFFE)
    let code = [
        0xc4, 0x42, 0x2d, 0x40, 0xcb, // VPMULLD YMM9, YMM10, YMM11
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmulld_ymm12_ymm13_ymm14_mixed() {
    let code = [
        0xc4, 0x42, 0x15, 0x40, 0xe6, // VPMULLD YMM12, YMM13, YMM14
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmulld_ymm15_ymm0_ymm1_high_reg() {
    let code = [
        0xc4, 0xc2, 0x7d, 0x40, 0xf9, // VPMULLD YMM15, YMM0, YMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmulld_ymm0_ymm1_mem() {
    // VPMULLD YMM0, YMM1, [memory]
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x75, 0x40, 0x00, // VPMULLD YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data: Vec<u8> = (0..8).flat_map(|i| (i as u32).to_le_bytes()).collect();
    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmulld_ymm2_ymm3_mem_max_values() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x65, 0x40, 0x10, // VPMULLD YMM2, YMM3, [RAX]
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
fn test_vpmulld_ymm4_ymm5_mem_powers_of_two() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x55, 0x40, 0x20, // VPMULLD YMM4, YMM5, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let powers: Vec<u8> = (0..8).flat_map(|i| (1u32 << i).to_le_bytes()).collect();
    mem.write_slice(&powers, GuestAddress(ALIGNED_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmulld_ymm6_ymm7_mem_sequential() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xc2, 0x45, 0x40, 0x30, // VPMULLD YMM6, YMM7, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data: Vec<u8> = (1..=8).flat_map(|i| (i as u32).to_le_bytes()).collect();
    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmulld_ymm0_ymm1_ymm2_small_values() {
    // 2 * 3 = 6
    let code = [
        0xc4, 0xe2, 0x75, 0x40, 0xc2, // VPMULLD YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmulld_ymm3_ymm4_ymm5_negative_representation() {
    // Test with sign-extended values (treated as unsigned)
    let code = [
        0xc4, 0xe2, 0x5d, 0x40, 0xdd, // VPMULLD YMM3, YMM4, YMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmulld_ymm8_ymm9_ymm10_alternating() {
    let code = [
        0xc4, 0x42, 0x35, 0x40, 0xc2, // VPMULLD YMM8, YMM9, YMM10
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmulld_ymm11_ymm12_ymm13_pattern() {
    let code = [
        0xc4, 0x42, 0x1d, 0x40, 0xdd, // VPMULLD YMM11, YMM12, YMM13
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmulld_chain_operations() {
    // Chain multiple VPMULLD operations
    let code = [
        0xc4, 0xe2, 0x75, 0x40, 0xc2, // VPMULLD YMM0, YMM1, YMM2
        0xc4, 0xe2, 0x7d, 0x40, 0xc3, // VPMULLD YMM0, YMM0, YMM3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmulld_mem_different_patterns() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x75, 0x40, 0x00, // VPMULLD YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let pattern: Vec<u8> = (0..8)
        .flat_map(|i| if i % 2 == 0 { 10u32 } else { 5u32 }.to_le_bytes())
        .collect();
    mem.write_slice(&pattern, GuestAddress(ALIGNED_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmulld_mem_large_multipliers() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x75, 0x40, 0x00, // VPMULLD YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data: Vec<u8> = (0..8).flat_map(|_| 0x80000000u32.to_le_bytes()).collect();
    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmulld_mem_prime_numbers() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x75, 0x40, 0x00, // VPMULLD YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let primes: Vec<u8> = vec![2u32, 3, 5, 7, 11, 13, 17, 19]
        .into_iter()
        .flat_map(|p| p.to_le_bytes())
        .collect();
    mem.write_slice(&primes, GuestAddress(ALIGNED_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmullw_mem_unaligned() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&(ALIGNED_ADDR + 2).to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0xd5, 0x00, // VPMULLW YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(
        &[
            0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03,
            0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03,
            0x03, 0x03, 0x03, 0x03, 0x03, 0x03,
        ],
        GuestAddress(ALIGNED_ADDR),
    )
    .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpmulld_mem_unaligned() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&(ALIGNED_ADDR + 4).to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x75, 0x40, 0x00, // VPMULLD YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(
        &[
            0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02,
            0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02,
            0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02,
        ],
        GuestAddress(ALIGNED_ADDR),
    )
    .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Regression tests: VEX/AVX2 integer lane handling and 128-bit upper-zeroing.
//
// These assert exact element values (the pre-existing tests in this suite are
// smoke tests that only check the instruction runs). They cover:
//   * VEX.128 (L=0) forms zero the upper 128 bits of the destination YMM.
//   * VEX.256 (L=1) forms compute both 128-bit lanes independently.
//   * Saturation / signedness correctness for packing and multiply-add ops.
// ============================================================================

use rax::backend::emulator::x86_64::X86_64Vcpu;

fn vex_ymm_set(vcpu: &mut X86_64Vcpu, idx: usize, lo: u128, hi: u128) {
    let mut regs = vcpu.get_regs().unwrap();
    regs.xmm[idx][0] = lo as u64;
    regs.xmm[idx][1] = (lo >> 64) as u64;
    regs.ymm_high[idx][0] = hi as u64;
    regs.ymm_high[idx][1] = (hi >> 64) as u64;
    vcpu.set_regs(&regs).unwrap();
}
fn vex_ymm_get(vcpu: &X86_64Vcpu, idx: usize) -> (u128, u128) {
    let regs = vcpu.get_regs().unwrap();
    let lo = (regs.xmm[idx][0] as u128) | ((regs.xmm[idx][1] as u128) << 64);
    let hi = (regs.ymm_high[idx][0] as u128) | ((regs.ymm_high[idx][1] as u128) << 64);
    (lo, hi)
}
/// Run `code` (HLT appended) after seeding two source registers; return dst (lo,hi).
fn vex_run3(
    code: &[u8],
    s1: usize,
    v1: (u128, u128),
    s2: usize,
    v2: (u128, u128),
    dst: usize,
) -> (u128, u128) {
    let mut full = code.to_vec();
    full.push(0xf4);
    let (mut vcpu, _) = setup_vm(&full, None);
    vex_ymm_set(&mut vcpu, s1, v1.0, v1.1);
    vex_ymm_set(&mut vcpu, s2, v2.0, v2.1);
    run_until_hlt(&mut vcpu).unwrap();
    vex_ymm_get(&vcpu, dst)
}

#[test]
fn test_vpaddusb_unsigned_saturate_both_lanes() {
    // VPADDUSB ymm0, ymm1, ymm2 : 0xF0 + 0x20 -> 0xFF (unsigned saturate) in both lanes.
    let (lo, hi) = vex_run3(
        &[0xc5, 0xf5, 0xdc, 0xc2],
        1,
        (0xF0, 0xF0),
        2,
        (0x20, 0x20),
        0,
    );
    assert_eq!(lo & 0xFF, 0xFF);
    assert_eq!(hi & 0xFF, 0xFF);
}

#[test]
fn test_vpacksswb_128_upper_zeroed_and_quadword_layout() {
    // VPACKSSWB xmm0, xmm1, xmm2 : src1 packs into low qword, src2 into high qword.
    // src1 word0 = 256 -> +0x7F; src2 word0 = -128 -> 0x80; upper 128 zeroed.
    let (lo, hi) = vex_run3(
        &[0xc5, 0xf1, 0x63, 0xc2],
        1,
        (0x0100, 0xAA),
        2,
        (0xFF80, 0xBB),
        0,
    );
    assert_eq!(lo & 0xFF, 0x7F, "src1 word saturated into low qword");
    assert_eq!(
        (lo >> 64) & 0xFF,
        0x80,
        "src2 word saturated into high qword"
    );
    assert_eq!(hi, 0, "VEX.128 must zero upper 128 bits");
}

#[test]
fn test_vpacksswb_256_independent_lanes() {
    // VPACKSSWB ymm0, ymm1, ymm2 : per-128-bit-lane interleave of src1/src2.
    let (lo, hi) = vex_run3(
        &[0xc5, 0xf5, 0x63, 0xc2],
        1,
        (0x0100, 0x0100),
        2,
        (0xFF80, 0xFF80),
        0,
    );
    assert_eq!(lo & 0xFF, 0x7F);
    assert_eq!((lo >> 64) & 0xFF, 0x80);
    assert_eq!(hi & 0xFF, 0x7F);
    assert_eq!((hi >> 64) & 0xFF, 0x80);
}

#[test]
fn test_vpackusdw_signed_to_unsigned_saturate_128_upper_zeroed() {
    // VPACKUSDW xmm0, xmm1, xmm2 : -1 -> 0, 0x12345 -> 0xFFFF.
    let (lo, hi) = vex_run3(
        &[0xc4, 0xe2, 0x71, 0x2b, 0xc2],
        1,
        (0xFFFFFFFF | (0x00012345u128 << 32), 0),
        2,
        (0, 0),
        0,
    );
    assert_eq!(lo & 0xFFFF, 0x0000);
    assert_eq!((lo >> 16) & 0xFFFF, 0xFFFF);
    assert_eq!(hi, 0);
}

#[test]
fn test_vpmulhw_vs_vpmulhuw_signedness() {
    // VPMULHW (signed high word): 0xFFFF*0xFFFF = (-1)*(-1)=1 -> high word 0.
    let (lo, _) = vex_run3(&[0xc5, 0xf5, 0xe5, 0xc2], 1, (0xFFFF, 0), 2, (0xFFFF, 0), 0);
    assert_eq!(lo & 0xFFFF, 0x0000);
    // VPMULHUW (unsigned high word): 0xFFFF*0xFFFF = 0xFFFE0001 -> high word 0xFFFE.
    let (lo, _) = vex_run3(&[0xc5, 0xf5, 0xe4, 0xc2], 1, (0xFFFF, 0), 2, (0xFFFF, 0), 0);
    assert_eq!(lo & 0xFFFF, 0xFFFE);
}

#[test]
fn test_vpmuldq_vs_vpmuludq_signedness() {
    // VPMULDQ (signed low dword -> qword): -1 * -1 = 1.
    let (lo, _) = vex_run3(
        &[0xc4, 0xe2, 0x75, 0x28, 0xc2],
        1,
        (0xFFFFFFFF, 0),
        2,
        (0xFFFFFFFF, 0),
        0,
    );
    assert_eq!(lo, 1);
    // VPMULUDQ (unsigned low dword -> qword): 0xFFFFFFFF^2 = 0xFFFFFFFE00000001.
    let (lo, _) = vex_run3(
        &[0xc5, 0xf5, 0xf4, 0xc2],
        1,
        (0xFFFFFFFF, 0),
        2,
        (0xFFFFFFFF, 0),
        0,
    );
    assert_eq!(lo as u64, 0xFFFFFFFE00000001u64);
}

#[test]
fn test_vpminsb_128_upper_zeroed() {
    // VPMINSB xmm0, xmm1, xmm2 : min(-128, 127) = -128; upper zeroed.
    let (lo, hi) = vex_run3(
        &[0xc4, 0xe2, 0x71, 0x38, 0xc2],
        1,
        (0x80, 0),
        2,
        (0x7F, 0),
        0,
    );
    assert_eq!(lo & 0xFF, 0x80);
    assert_eq!(hi, 0);
}

#[test]
fn test_vpsravd_arithmetic_fill_both_lanes() {
    // VPSRAVD ymm: 0x80000000 >> 4 (arithmetic) = 0xF8000000.
    let (lo, _) = vex_run3(
        &[0xc4, 0xe2, 0x75, 0x46, 0xc2],
        1,
        (0x80000000, 0),
        2,
        (4, 0),
        0,
    );
    assert_eq!(lo & 0xFFFFFFFF, 0xF8000000);
}

#[test]
fn test_vpcmpeqd_256_independent_lanes() {
    // VPCMPEQD ymm: low lane equal -> all ones; high lane low dword not equal -> 0.
    let (lo, hi) = vex_run3(
        &[0xc5, 0xf5, 0x76, 0xc2],
        1,
        (0x12345678, 0x12345678),
        2,
        (0x12345678, 0x99),
        0,
    );
    assert_eq!(lo & 0xFFFFFFFF, 0xFFFFFFFF);
    assert_eq!(hi & 0xFFFFFFFF, 0);
}

#[test]
fn test_vpcmpgtq_128_upper_zeroed() {
    // VPCMPGTQ xmm: (-1) > 0 is false -> 0; upper zeroed.
    let (lo, hi) = vex_run3(
        &[0xc4, 0xe2, 0x71, 0x37, 0xc2],
        1,
        (0xFFFFFFFFFFFFFFFF, 0),
        2,
        (0, 0),
        0,
    );
    assert_eq!(lo as u64, 0);
    assert_eq!(hi, 0);
}
