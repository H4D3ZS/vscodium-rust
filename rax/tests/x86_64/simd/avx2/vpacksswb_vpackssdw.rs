use crate::common::*;
use vm_memory::{Bytes, GuestAddress};

// VPACKSSWB/VPACKSSDW - Pack with Signed Saturation (AVX2)
//
// Converts packed integers from larger to smaller data types with signed saturation.
// Values are clamped to the representable range of the target type.
//
// VPACKSSWB: Pack 16-bit signed words to 8-bit signed bytes with saturation
//            32 words (16 from each source) -> 32 bytes
//            Saturation range: -128 to 127
//
// VPACKSSDW: Pack 32-bit signed doublewords to 16-bit signed words with saturation
//            16 dwords (8 from each source) -> 16 words
//            Saturation range: -32768 to 32767
//
// Opcodes (AVX2 - 256-bit YMM):
// VEX.256.66.0F.WIG 63 /r     VPACKSSWB ymm1, ymm2, ymm3/m256
// VEX.256.66.0F.WIG 6B /r     VPACKSSDW ymm1, ymm2, ymm3/m256

const ALIGNED_ADDR: u64 = 0x3000;
const ALIGNED_ADDR2: u64 = 0x3100;

// ============================================================================
// VPACKSSWB Tests - Pack Signed Words to Signed Bytes (256-bit)
// ============================================================================

#[test]
fn test_vpacksswb_ymm0_ymm1_ymm2() {
    // VPACKSSWB YMM0, YMM1, YMM2
    let code = [
        0xc5, 0xf5, 0x63, 0xc2, // VPACKSSWB YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpacksswb_ymm3_ymm4_ymm5() {
    let code = [
        0xc5, 0xdd, 0x63, 0xdd, // VPACKSSWB YMM3, YMM4, YMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpacksswb_ymm6_ymm7_ymm8() {
    let code = [
        0xc5, 0x45, 0x63, 0xf0, // VPACKSSWB YMM6, YMM7, YMM8
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpacksswb_ymm9_ymm10_ymm11() {
    let code = [
        0xc4, 0x41, 0x2d, 0x63, 0xcb, // VPACKSSWB YMM9, YMM10, YMM11
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpacksswb_ymm12_ymm13_ymm14() {
    let code = [
        0xc4, 0x41, 0x15, 0x63, 0xe6, // VPACKSSWB YMM12, YMM13, YMM14
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpacksswb_ymm15_ymm0_ymm1() {
    let code = [
        0xc4, 0xc1, 0x7d, 0x63, 0xf9, // VPACKSSWB YMM15, YMM0, YMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpacksswb_ymm0_ymm1_mem() {
    // VPACKSSWB YMM0, YMM1, [memory]
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0x63, 0x00, // VPACKSSWB YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    // Initialize with small values that won't saturate
    let data: Vec<u8> = (0..16).flat_map(|i| (i as i16).to_le_bytes()).collect();
    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpacksswb_positive_saturation() {
    // Test positive saturation (values > 127 should become 127)
    let code = [
        0xc5, 0xf5, 0x63, 0xc2, // VPACKSSWB YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpacksswb_negative_saturation() {
    // Test negative saturation (values < -128 should become -128)
    let code = [
        0xc5, 0xf5, 0x63, 0xc2, // VPACKSSWB YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpacksswb_no_saturation() {
    // Test values within range (no saturation)
    let code = [
        0xc5, 0xf5, 0x63, 0xc2, // VPACKSSWB YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpacksswb_boundary_values() {
    // Test boundary values (127, 128, -128, -129)
    let code = [
        0xc5, 0xf5, 0x63, 0xc2, // VPACKSSWB YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpacksswb_chain_operations() {
    // Chain multiple pack operations
    let code = [
        0xc5, 0xf5, 0x63, 0xc2, // VPACKSSWB YMM0, YMM1, YMM2
        0xc5, 0xed, 0x63, 0xdb, // VPACKSSWB YMM3, YMM2, YMM3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpacksswb_ymm2_ymm3_mem() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xe5, 0x63, 0x10, // VPACKSSWB YMM2, YMM3, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let pattern: Vec<u8> = (0..16)
        .flat_map(|i| ((i * 8) as i16).to_le_bytes())
        .collect();
    mem.write_slice(&pattern, GuestAddress(ALIGNED_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpacksswb_max_positive() {
    // Test maximum positive values (32767)
    let code = [
        0xc5, 0xf5, 0x63, 0xc2, // VPACKSSWB YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpacksswb_max_negative() {
    // Test maximum negative values (-32768)
    let code = [
        0xc5, 0xf5, 0x63, 0xc2, // VPACKSSWB YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// VPACKSSDW Tests - Pack Signed Dwords to Signed Words (256-bit)
// ============================================================================

#[test]
fn test_vpackssdw_ymm0_ymm1_ymm2() {
    // VPACKSSDW YMM0, YMM1, YMM2
    let code = [
        0xc5, 0xf5, 0x6b, 0xc2, // VPACKSSDW YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpackssdw_ymm3_ymm4_ymm5() {
    let code = [
        0xc5, 0xdd, 0x6b, 0xdd, // VPACKSSDW YMM3, YMM4, YMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpackssdw_ymm6_ymm7_ymm8() {
    let code = [
        0xc5, 0x45, 0x6b, 0xf0, // VPACKSSDW YMM6, YMM7, YMM8
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpackssdw_ymm9_ymm10_ymm11() {
    let code = [
        0xc4, 0x41, 0x2d, 0x6b, 0xcb, // VPACKSSDW YMM9, YMM10, YMM11
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpackssdw_ymm12_ymm13_ymm14() {
    let code = [
        0xc4, 0x41, 0x15, 0x6b, 0xe6, // VPACKSSDW YMM12, YMM13, YMM14
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpackssdw_ymm15_ymm0_ymm1() {
    let code = [
        0xc4, 0xc1, 0x7d, 0x6b, 0xf9, // VPACKSSDW YMM15, YMM0, YMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpackssdw_ymm0_ymm1_mem() {
    // VPACKSSDW YMM0, YMM1, [memory]
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0x6b, 0x00, // VPACKSSDW YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data: Vec<u8> = (0..8)
        .flat_map(|i| (i as i32 * 100).to_le_bytes())
        .collect();
    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpackssdw_positive_saturation() {
    // Test positive saturation (values > 32767 should become 32767)
    let code = [
        0xc5, 0xf5, 0x6b, 0xc2, // VPACKSSDW YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpackssdw_negative_saturation() {
    // Test negative saturation (values < -32768 should become -32768)
    let code = [
        0xc5, 0xf5, 0x6b, 0xc2, // VPACKSSDW YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpackssdw_no_saturation() {
    // Test values within range (no saturation)
    let code = [
        0xc5, 0xf5, 0x6b, 0xc2, // VPACKSSDW YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpackssdw_boundary_values() {
    // Test boundary values (32767, 32768, -32768, -32769)
    let code = [
        0xc5, 0xf5, 0x6b, 0xc2, // VPACKSSDW YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpackssdw_chain_operations() {
    // Chain multiple pack operations
    let code = [
        0xc5, 0xf5, 0x6b, 0xc2, // VPACKSSDW YMM0, YMM1, YMM2
        0xc5, 0xed, 0x6b, 0xdb, // VPACKSSDW YMM3, YMM2, YMM3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpackssdw_ymm2_ymm3_mem() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xe5, 0x6b, 0x10, // VPACKSSDW YMM2, YMM3, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let pattern: Vec<u8> = (0..8)
        .flat_map(|i| (i as i32 * 1000).to_le_bytes())
        .collect();
    mem.write_slice(&pattern, GuestAddress(ALIGNED_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpackssdw_max_positive() {
    // Test maximum positive values (2147483647)
    let code = [
        0xc5, 0xf5, 0x6b, 0xc2, // VPACKSSDW YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpackssdw_max_negative() {
    // Test maximum negative values (-2147483648)
    let code = [
        0xc5, 0xf5, 0x6b, 0xc2, // VPACKSSDW YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Additional comprehensive tests
// ============================================================================

#[test]
fn test_vpacksswb_extended_regs() {
    // Test with extended registers YMM8-YMM15
    let code = [
        0xc4, 0x41, 0x3d, 0x63, 0xc1, // VPACKSSWB YMM8, YMM8, YMM9
        0xc4, 0x41, 0x15, 0x63, 0xee, // VPACKSSWB YMM13, YMM13, YMM14
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpackssdw_extended_regs() {
    let code = [
        0xc4, 0x41, 0x35, 0x6b, 0xcb, // VPACKSSDW YMM9, YMM9, YMM11
        0xc4, 0x41, 0x0d, 0x6b, 0xf7, // VPACKSSDW YMM14, YMM14, YMM15
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpacksswb_all_zeros() {
    // Pack all zeros
    let code = [
        0xc5, 0xf5, 0x63, 0xc2, // VPACKSSWB YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpackssdw_all_zeros() {
    let code = [
        0xc5, 0xf5, 0x6b, 0xc2, // VPACKSSDW YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpacksswb_alternating_signs() {
    // Pack alternating positive and negative values
    let code = [
        0xc5, 0xf5, 0x63, 0xc2, // VPACKSSWB YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpackssdw_alternating_signs() {
    let code = [
        0xc5, 0xf5, 0x6b, 0xc2, // VPACKSSDW YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpacksswb_mixed_saturation() {
    // Mix of saturating and non-saturating values
    let code = [
        0xc5, 0xf5, 0x63, 0xc2, // VPACKSSWB YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpackssdw_mixed_saturation() {
    let code = [
        0xc5, 0xf5, 0x6b, 0xc2, // VPACKSSDW YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpacksswb_sequential() {
    // Sequential packing operations
    let code = [
        0xc5, 0xf5, 0x63, 0xc2, // VPACKSSWB YMM0, YMM1, YMM2
        0xc5, 0xed, 0x63, 0xdb, // VPACKSSWB YMM3, YMM2, YMM3
        0xc5, 0xe5, 0x63, 0xe4, // VPACKSSWB YMM4, YMM3, YMM4
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpackssdw_sequential() {
    let code = [
        0xc5, 0xf5, 0x6b, 0xc2, // VPACKSSDW YMM0, YMM1, YMM2
        0xc5, 0xed, 0x6b, 0xdb, // VPACKSSDW YMM3, YMM2, YMM3
        0xc5, 0xe5, 0x6b, 0xe4, // VPACKSSDW YMM4, YMM3, YMM4
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpacksswb_same_source() {
    // Pack register with itself
    let code = [
        0xc5, 0xf5, 0x63, 0xc1, // VPACKSSWB YMM0, YMM1, YMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpackssdw_same_source() {
    let code = [
        0xc5, 0xf5, 0x6b, 0xc1, // VPACKSSDW YMM0, YMM1, YMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpacksswb_ymm4_ymm5_mem() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xd5, 0x63, 0x20, // VPACKSSWB YMM4, YMM5, [RAX]
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
fn test_vpackssdw_ymm4_ymm5_mem() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xd5, 0x6b, 0x20, // VPACKSSDW YMM4, YMM5, [RAX]
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
fn test_vpacksswb_small_values() {
    // Small values that fit in target range
    let code = [
        0xc5, 0xf5, 0x63, 0xc2, // VPACKSSWB YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vpackssdw_small_values() {
    let code = [
        0xc5, 0xf5, 0x6b, 0xc2, // VPACKSSDW YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
