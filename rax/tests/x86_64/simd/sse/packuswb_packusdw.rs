use crate::common::*;
use vm_memory::{Bytes, GuestAddress};

// PACKUSWB/PACKUSDW - Pack with Unsigned Saturation (SSE2/SSE4.1)
//
// Converts packed signed integers from source and destination operands
// into packed unsigned integers of smaller data type using unsigned saturation.
// Negative values saturate to 0, values beyond max saturate to max.
//
// PACKUSWB: Converts 8 signed word integers (16-bit) from dest and 8 from src
//           into 16 unsigned byte integers (8-bit) with unsigned saturation
//           Range: 0 to 255 (0xFF)
//           Negative values -> 0, values > 255 -> 255
//
// PACKUSDW: Converts 4 signed dword integers (32-bit) from dest and 4 from src
//           into 8 unsigned word integers (16-bit) with unsigned saturation
//           Range: 0 to 65535 (0xFFFF)
//           Negative values -> 0, values > 65535 -> 65535
//
// Opcodes:
// 66 0F 67 /r          PACKUSWB xmm1, xmm2/m128   - Pack words to unsigned bytes (SSE2)
// 66 0F 38 2B /r       PACKUSDW xmm1, xmm2/m128   - Pack dwords to unsigned words (SSE4.1)

const ALIGNED_ADDR: u64 = 0x3000;
const ALIGNED_ADDR2: u64 = 0x3100;

// ============================================================================
// PACKUSWB Tests - Pack Words to Unsigned Bytes
// ============================================================================

#[test]
fn test_packuswb_all_zeros() {
    // PACKUSWB XMM0, XMM1 with all zeros
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, // MOVDQA XMM0, [RAX]
        0x66, 0x0f, 0x6f, 0x0b, // MOVDQA XMM1, [RBX]
        0x66, 0x0f, 0x67, 0xc1, // PACKUSWB XMM0, XMM1
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(
        &[
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00,
        ],
        GuestAddress(ALIGNED_ADDR),
    )
    .unwrap();
    mem.write_slice(
        &[
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00,
        ],
        GuestAddress(ALIGNED_ADDR2),
    )
    .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_packuswb_positive_values() {
    // Test packing positive values within range
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x0b, 0x66, 0x0f, 0x67, 0xc1, 0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    // 8 words: 1, 2, 3, 4, 5, 6, 7, 8
    let data1 = [
        0x01, 0x00, 0x02, 0x00, 0x03, 0x00, 0x04, 0x00, 0x05, 0x00, 0x06, 0x00, 0x07, 0x00, 0x08,
        0x00,
    ];
    // 8 words: 9, 10, 11, 12, 13, 14, 15, 16
    let data2 = [
        0x09, 0x00, 0x0A, 0x00, 0x0B, 0x00, 0x0C, 0x00, 0x0D, 0x00, 0x0E, 0x00, 0x0F, 0x00, 0x10,
        0x00,
    ];
    mem.write_slice(&data1, GuestAddress(ALIGNED_ADDR)).unwrap();
    mem.write_slice(&data2, GuestAddress(ALIGNED_ADDR2))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_packuswb_negative_saturate_to_zero() {
    // Test that negative values saturate to 0
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x0b, 0x66, 0x0f, 0x67, 0xc1, 0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    // 8 words: -1, -2, -3, -4, -5, -6, -7, -8
    let data1 = [
        0xFF, 0xFF, 0xFE, 0xFF, 0xFD, 0xFF, 0xFC, 0xFF, 0xFB, 0xFF, 0xFA, 0xFF, 0xF9, 0xFF, 0xF8,
        0xFF,
    ];
    // 8 words: -9, -10, -100, -1000, -10000, -20000, -30000, -32768
    let data2 = [
        0xF7, 0xFF, 0xF6, 0xFF, 0x9C, 0xFF, 0x18, 0xFC, 0xF0, 0xD8, 0xE0, 0xB1, 0xD0, 0x8A, 0x00,
        0x80,
    ];
    mem.write_slice(&data1, GuestAddress(ALIGNED_ADDR)).unwrap();
    mem.write_slice(&data2, GuestAddress(ALIGNED_ADDR2))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_packuswb_saturate_positive_max() {
    // Test saturation of positive values > 255 to 0xFF
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x0b, 0x66, 0x0f, 0x67, 0xc1, 0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    // Words: 256, 300, 1000, 32767, ...
    let data1 = [
        0x00, 0x01, 0x2C, 0x01, 0xE8, 0x03, 0xFF, 0x7F, 0x00, 0x10, 0xFF, 0x0F, 0x00, 0x20, 0x00,
        0x40,
    ];
    let data2 = [
        0xFF, 0x7F, 0xFF, 0x7F, 0xFF, 0x7F, 0xFF, 0x7F, 0xFF, 0x7F, 0xFF, 0x7F, 0xFF, 0x7F, 0xFF,
        0x7F,
    ];
    mem.write_slice(&data1, GuestAddress(ALIGNED_ADDR)).unwrap();
    mem.write_slice(&data2, GuestAddress(ALIGNED_ADDR2))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_packuswb_boundary_values() {
    // Test exact boundary values: 0 and 255
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x0b, 0x66, 0x0f, 0x67, 0xc1, 0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    // Words: 0, 255, 0, 255, ...
    let data1 = [
        0x00, 0x00, 0xFF, 0x00, 0x00, 0x00, 0xFF, 0x00, 0x00, 0x00, 0xFF, 0x00, 0x00, 0x00, 0xFF,
        0x00,
    ];
    let data2 = [
        0x00, 0x00, 0xFF, 0x00, 0x00, 0x00, 0xFF, 0x00, 0x00, 0x00, 0xFF, 0x00, 0x00, 0x00, 0xFF,
        0x00,
    ];
    mem.write_slice(&data1, GuestAddress(ALIGNED_ADDR)).unwrap();
    mem.write_slice(&data2, GuestAddress(ALIGNED_ADDR2))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_packuswb_mixed_saturation() {
    // Test mix of normal values and saturated values
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x0b, 0x66, 0x0f, 0x67, 0xc1, 0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    // Mix: 10, 200, 50, 300, 100, 400, 150, 500
    let data1 = [
        0x0A, 0x00, 0xC8, 0x00, 0x32, 0x00, 0x2C, 0x01, 0x64, 0x00, 0x90, 0x01, 0x96, 0x00, 0xF4,
        0x01,
    ];
    // Mix: -10, 255, -50, 256, 0, 1000, -1, -1000
    let data2 = [
        0xF6, 0xFF, 0xFF, 0x00, 0xCE, 0xFF, 0x00, 0x01, 0x00, 0x00, 0xE8, 0x03, 0xFF, 0xFF, 0x18,
        0xFC,
    ];
    mem.write_slice(&data1, GuestAddress(ALIGNED_ADDR)).unwrap();
    mem.write_slice(&data2, GuestAddress(ALIGNED_ADDR2))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_packuswb_xmm2_xmm3() {
    // Test with different register operands
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x10, // MOVDQA XMM2, [RAX]
        0x66, 0x0f, 0x6f, 0x1b, // MOVDQA XMM3, [RBX]
        0x66, 0x0f, 0x67, 0xd3, // PACKUSWB XMM2, XMM3
        0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data1 = [
        0x10, 0x00, 0x20, 0x00, 0x30, 0x00, 0x40, 0x00, 0x50, 0x00, 0x60, 0x00, 0x70, 0x00, 0x80,
        0x00,
    ];
    let data2 = [
        0x90, 0x00, 0xA0, 0x00, 0xB0, 0x00, 0xC0, 0x00, 0xD0, 0x00, 0xE0, 0x00, 0xF0, 0x00, 0xFF,
        0x00,
    ];
    mem.write_slice(&data1, GuestAddress(ALIGNED_ADDR)).unwrap();
    mem.write_slice(&data2, GuestAddress(ALIGNED_ADDR2))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_packuswb_from_memory() {
    // PACKUSWB XMM0, [mem]
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, // MOVDQA XMM0, [RAX]
        0x66, 0x0f, 0x67, 0x03, // PACKUSWB XMM0, [RBX]
        0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data1 = [
        0x01, 0x00, 0x02, 0x00, 0x03, 0x00, 0x04, 0x00, 0x05, 0x00, 0x06, 0x00, 0x07, 0x00, 0x08,
        0x00,
    ];
    let data2 = [
        0x09, 0x00, 0x0A, 0x00, 0x0B, 0x00, 0x0C, 0x00, 0x0D, 0x00, 0x0E, 0x00, 0x0F, 0x00, 0x10,
        0x00,
    ];
    mem.write_slice(&data1, GuestAddress(ALIGNED_ADDR)).unwrap();
    mem.write_slice(&data2, GuestAddress(ALIGNED_ADDR2))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_packuswb_xmm7_xmm6() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x38, // MOVDQA XMM7, [RAX]
        0x66, 0x0f, 0x6f, 0x33, // MOVDQA XMM6, [RBX]
        0x66, 0x0f, 0x67, 0xfe, // PACKUSWB XMM7, XMM6
        0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data1 = [
        0x00, 0x00, 0x01, 0x00, 0x02, 0x00, 0x03, 0x00, 0x04, 0x00, 0x05, 0x00, 0x06, 0x00, 0x07,
        0x00,
    ];
    let data2 = [
        0x08, 0x00, 0x09, 0x00, 0x0A, 0x00, 0x0B, 0x00, 0x0C, 0x00, 0x0D, 0x00, 0x0E, 0x00, 0x0F,
        0x00,
    ];
    mem.write_slice(&data1, GuestAddress(ALIGNED_ADDR)).unwrap();
    mem.write_slice(&data2, GuestAddress(ALIGNED_ADDR2))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// PACKUSDW Tests - Pack Dwords to Unsigned Words
// ============================================================================

#[test]
fn test_packusdw_all_zeros() {
    // PACKUSDW XMM0, XMM1 with all zeros
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x0b, 0x66, 0x0f, 0x38, 0x2b,
        0xc1, // PACKUSDW XMM0, XMM1
        0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(
        &[
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00,
        ],
        GuestAddress(ALIGNED_ADDR),
    )
    .unwrap();
    mem.write_slice(
        &[
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00,
        ],
        GuestAddress(ALIGNED_ADDR2),
    )
    .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_packusdw_positive_values() {
    // Test packing positive values within range
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x0b, 0x66, 0x0f, 0x38, 0x2b, 0xc1, 0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    // 4 dwords: 1, 2, 3, 4
    let data1 = [
        0x01, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00,
        0x00,
    ];
    // 4 dwords: 5, 6, 7, 8
    let data2 = [
        0x05, 0x00, 0x00, 0x00, 0x06, 0x00, 0x00, 0x00, 0x07, 0x00, 0x00, 0x00, 0x08, 0x00, 0x00,
        0x00,
    ];
    mem.write_slice(&data1, GuestAddress(ALIGNED_ADDR)).unwrap();
    mem.write_slice(&data2, GuestAddress(ALIGNED_ADDR2))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_packusdw_negative_saturate_to_zero() {
    // Test that negative values saturate to 0
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x0b, 0x66, 0x0f, 0x38, 0x2b, 0xc1, 0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    // 4 dwords: -1, -2, -3, -4
    let data1 = [
        0xFF, 0xFF, 0xFF, 0xFF, 0xFE, 0xFF, 0xFF, 0xFF, 0xFD, 0xFF, 0xFF, 0xFF, 0xFC, 0xFF, 0xFF,
        0xFF,
    ];
    // 4 dwords: -100, -1000, -100000, -2147483648
    let data2 = [
        0x9C, 0xFF, 0xFF, 0xFF, 0x18, 0xFC, 0xFF, 0xFF, 0x60, 0x79, 0xFE, 0xFF, 0x00, 0x00, 0x00,
        0x80,
    ];
    mem.write_slice(&data1, GuestAddress(ALIGNED_ADDR)).unwrap();
    mem.write_slice(&data2, GuestAddress(ALIGNED_ADDR2))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_packusdw_saturate_positive_max() {
    // Test saturation of positive values > 65535 to 0xFFFF
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x0b, 0x66, 0x0f, 0x38, 0x2b, 0xc1, 0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    // Dwords: 65536, 70000, 100000, 2147483647
    let data1 = [
        0x00, 0x00, 0x01, 0x00, 0x70, 0x11, 0x01, 0x00, 0xA0, 0x86, 0x01, 0x00, 0xFF, 0xFF, 0xFF,
        0x7F,
    ];
    let data2 = [
        0xFF, 0xFF, 0xFF, 0x7F, 0xFF, 0xFF, 0xFF, 0x7F, 0xFF, 0xFF, 0xFF, 0x7F, 0xFF, 0xFF, 0xFF,
        0x7F,
    ];
    mem.write_slice(&data1, GuestAddress(ALIGNED_ADDR)).unwrap();
    mem.write_slice(&data2, GuestAddress(ALIGNED_ADDR2))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_packusdw_boundary_values() {
    // Test exact boundary values: 0 and 65535
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x0b, 0x66, 0x0f, 0x38, 0x2b, 0xc1, 0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    // Dwords: 0, 65535, 0, 65535
    let data1 = [
        0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0x00,
        0x00,
    ];
    let data2 = [
        0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0x00,
        0x00,
    ];
    mem.write_slice(&data1, GuestAddress(ALIGNED_ADDR)).unwrap();
    mem.write_slice(&data2, GuestAddress(ALIGNED_ADDR2))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_packusdw_mixed_saturation() {
    // Test mix of normal values and saturated values
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x0b, 0x66, 0x0f, 0x38, 0x2b, 0xc1, 0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    // Mix: 100, 70000, 5000, 100000
    let data1 = [
        0x64, 0x00, 0x00, 0x00, 0x70, 0x11, 0x01, 0x00, 0x88, 0x13, 0x00, 0x00, 0xA0, 0x86, 0x01,
        0x00,
    ];
    // Mix: -100, 65535, -5000, 65536
    let data2 = [
        0x9C, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0x78, 0xEC, 0xFF, 0xFF, 0x00, 0x00, 0x01,
        0x00,
    ];
    mem.write_slice(&data1, GuestAddress(ALIGNED_ADDR)).unwrap();
    mem.write_slice(&data2, GuestAddress(ALIGNED_ADDR2))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_packusdw_xmm4_xmm5() {
    // Test with different register operands
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x20, // MOVDQA XMM4, [RAX]
        0x66, 0x0f, 0x6f, 0x2b, // MOVDQA XMM5, [RBX]
        0x66, 0x0f, 0x38, 0x2b, 0xe5, // PACKUSDW XMM4, XMM5
        0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data1 = [
        0x00, 0x01, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00, 0x04, 0x00,
        0x00,
    ];
    let data2 = [
        0x00, 0x05, 0x00, 0x00, 0x00, 0x06, 0x00, 0x00, 0x00, 0x07, 0x00, 0x00, 0x00, 0x08, 0x00,
        0x00,
    ];
    mem.write_slice(&data1, GuestAddress(ALIGNED_ADDR)).unwrap();
    mem.write_slice(&data2, GuestAddress(ALIGNED_ADDR2))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_packusdw_from_memory() {
    // PACKUSDW XMM0, [mem]
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, // MOVDQA XMM0, [RAX]
        0x66, 0x0f, 0x38, 0x2b, 0x03, // PACKUSDW XMM0, [RBX]
        0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data1 = [
        0x0A, 0x00, 0x00, 0x00, 0x14, 0x00, 0x00, 0x00, 0x1E, 0x00, 0x00, 0x00, 0x28, 0x00, 0x00,
        0x00,
    ];
    let data2 = [
        0x32, 0x00, 0x00, 0x00, 0x3C, 0x00, 0x00, 0x00, 0x46, 0x00, 0x00, 0x00, 0x50, 0x00, 0x00,
        0x00,
    ];
    mem.write_slice(&data1, GuestAddress(ALIGNED_ADDR)).unwrap();
    mem.write_slice(&data2, GuestAddress(ALIGNED_ADDR2))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_packusdw_xmm1_xmm2() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x08, // MOVDQA XMM1, [RAX]
        0x66, 0x0f, 0x6f, 0x13, // MOVDQA XMM2, [RBX]
        0x66, 0x0f, 0x38, 0x2b, 0xca, // PACKUSDW XMM1, XMM2
        0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data1 = [
        0xFF, 0x00, 0x00, 0x00, 0xFE, 0x00, 0x00, 0x00, 0xFD, 0x00, 0x00, 0x00, 0xFC, 0x00, 0x00,
        0x00,
    ];
    let data2 = [
        0xFB, 0x00, 0x00, 0x00, 0xFA, 0x00, 0x00, 0x00, 0xF9, 0x00, 0x00, 0x00, 0xF8, 0x00, 0x00,
        0x00,
    ];
    mem.write_slice(&data1, GuestAddress(ALIGNED_ADDR)).unwrap();
    mem.write_slice(&data2, GuestAddress(ALIGNED_ADDR2))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Extended Register Tests (XMM8-XMM15)
// ============================================================================

#[test]
fn test_packuswb_xmm8_xmm9() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x44, 0x0f, 0x6f, 0x00, // MOVDQA XMM8, [RAX]
        0x66, 0x44, 0x0f, 0x6f, 0x0b, // MOVDQA XMM9, [RBX]
        0x66, 0x45, 0x0f, 0x67, 0xc1, // PACKUSWB XMM8, XMM9
        0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data1 = [
        0x01, 0x00, 0x02, 0x00, 0x03, 0x00, 0x04, 0x00, 0x05, 0x00, 0x06, 0x00, 0x07, 0x00, 0x08,
        0x00,
    ];
    let data2 = [
        0x09, 0x00, 0x0A, 0x00, 0x0B, 0x00, 0x0C, 0x00, 0x0D, 0x00, 0x0E, 0x00, 0x0F, 0x00, 0x10,
        0x00,
    ];
    mem.write_slice(&data1, GuestAddress(ALIGNED_ADDR)).unwrap();
    mem.write_slice(&data2, GuestAddress(ALIGNED_ADDR2))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_packusdw_xmm10_xmm11() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x44, 0x0f, 0x6f, 0x10, // MOVDQA XMM10, [RAX]
        0x66, 0x44, 0x0f, 0x6f, 0x1b, // MOVDQA XMM11, [RBX]
        0x66, 0x45, 0x0f, 0x38, 0x2b, 0xd3, // PACKUSDW XMM10, XMM11
        0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data1 = [
        0x00, 0x01, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00, 0x04, 0x00,
        0x00,
    ];
    let data2 = [
        0x00, 0x05, 0x00, 0x00, 0x00, 0x06, 0x00, 0x00, 0x00, 0x07, 0x00, 0x00, 0x00, 0x08, 0x00,
        0x00,
    ];
    mem.write_slice(&data1, GuestAddress(ALIGNED_ADDR)).unwrap();
    mem.write_slice(&data2, GuestAddress(ALIGNED_ADDR2))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_packuswb_xmm12_xmm13() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x44, 0x0f, 0x6f, 0x20, // MOVDQA XMM12, [RAX]
        0x66, 0x44, 0x0f, 0x6f, 0x2b, // MOVDQA XMM13, [RBX]
        0x66, 0x45, 0x0f, 0x67, 0xe5, // PACKUSWB XMM12, XMM13
        0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data1 = [
        0x00, 0x00, 0xFF, 0x00, 0x80, 0x00, 0x00, 0x01, 0x7F, 0x00, 0x01, 0x00, 0xFE, 0xFF, 0x00,
        0x00,
    ];
    let data2 = [
        0x50, 0x00, 0xA0, 0x00, 0x64, 0x00, 0xC8, 0x00, 0x32, 0x00, 0x96, 0x00, 0x14, 0x00, 0xFA,
        0x00,
    ];
    mem.write_slice(&data1, GuestAddress(ALIGNED_ADDR)).unwrap();
    mem.write_slice(&data2, GuestAddress(ALIGNED_ADDR2))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_packusdw_xmm14_xmm15() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x44, 0x0f, 0x6f, 0x30, // MOVDQA XMM14, [RAX]
        0x66, 0x44, 0x0f, 0x6f, 0x3b, // MOVDQA XMM15, [RBX]
        0x66, 0x45, 0x0f, 0x38, 0x2b, 0xf7, // PACKUSDW XMM14, XMM15
        0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data1 = [
        0xFF, 0xFF, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x9C, 0xFF, 0xFF,
        0xFF,
    ];
    let data2 = [
        0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0x7F, 0x10, 0x27, 0x00, 0x00, 0xF0, 0xD8, 0xFF,
        0xFF,
    ];
    mem.write_slice(&data1, GuestAddress(ALIGNED_ADDR)).unwrap();
    mem.write_slice(&data2, GuestAddress(ALIGNED_ADDR2))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Sequence and Combination Tests
// ============================================================================

#[test]
fn test_packuswb_sequence() {
    // Test sequence of multiple PACKUSWB instructions
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, // MOVDQA XMM0, [RAX]
        0x66, 0x0f, 0x6f, 0x0b, // MOVDQA XMM1, [RBX]
        0x66, 0x0f, 0x6f, 0x10, // MOVDQA XMM2, [RAX]
        0x66, 0x0f, 0x67, 0xc1, // PACKUSWB XMM0, XMM1
        0x66, 0x0f, 0x67, 0xd1, // PACKUSWB XMM2, XMM1
        0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data1 = [
        0x01, 0x00, 0x02, 0x00, 0x03, 0x00, 0x04, 0x00, 0x05, 0x00, 0x06, 0x00, 0x07, 0x00, 0x08,
        0x00,
    ];
    let data2 = [
        0x09, 0x00, 0x0A, 0x00, 0x0B, 0x00, 0x0C, 0x00, 0x0D, 0x00, 0x0E, 0x00, 0x0F, 0x00, 0x10,
        0x00,
    ];
    mem.write_slice(&data1, GuestAddress(ALIGNED_ADDR)).unwrap();
    mem.write_slice(&data2, GuestAddress(ALIGNED_ADDR2))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_packusdw_sequence() {
    // Test sequence of multiple PACKUSDW instructions
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, // MOVDQA XMM0, [RAX]
        0x66, 0x0f, 0x6f, 0x0b, // MOVDQA XMM1, [RBX]
        0x66, 0x0f, 0x6f, 0x10, // MOVDQA XMM2, [RAX]
        0x66, 0x0f, 0x38, 0x2b, 0xc1, // PACKUSDW XMM0, XMM1
        0x66, 0x0f, 0x38, 0x2b, 0xd1, // PACKUSDW XMM2, XMM1
        0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data1 = [
        0x01, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00,
        0x00,
    ];
    let data2 = [
        0x05, 0x00, 0x00, 0x00, 0x06, 0x00, 0x00, 0x00, 0x07, 0x00, 0x00, 0x00, 0x08, 0x00, 0x00,
        0x00,
    ];
    mem.write_slice(&data1, GuestAddress(ALIGNED_ADDR)).unwrap();
    mem.write_slice(&data2, GuestAddress(ALIGNED_ADDR2))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_packusdw_then_packuswb() {
    // Test PACKUSDW followed by PACKUSWB (dword -> word -> byte)
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, // MOVDQA XMM0, [RAX]
        0x66, 0x0f, 0x6f, 0x0b, // MOVDQA XMM1, [RBX]
        0x66, 0x0f, 0x6f, 0x10, // MOVDQA XMM2, [RAX]
        0x66, 0x0f, 0x38, 0x2b, 0xc1, // PACKUSDW XMM0, XMM1
        0x66, 0x0f, 0x38, 0x2b, 0xd1, // PACKUSDW XMM2, XMM1
        0x66, 0x0f, 0x67, 0xc2, // PACKUSWB XMM0, XMM2
        0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data1 = [
        0x01, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00,
        0x00,
    ];
    let data2 = [
        0x05, 0x00, 0x00, 0x00, 0x06, 0x00, 0x00, 0x00, 0x07, 0x00, 0x00, 0x00, 0x08, 0x00, 0x00,
        0x00,
    ];
    mem.write_slice(&data1, GuestAddress(ALIGNED_ADDR)).unwrap();
    mem.write_slice(&data2, GuestAddress(ALIGNED_ADDR2))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_packuswb_all_saturate_positive() {
    // All values saturate to 0xFF
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x0b, 0x66, 0x0f, 0x67, 0xc1, 0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    // All words = 1000 (should saturate to 255)
    let data = [
        0xE8, 0x03, 0xE8, 0x03, 0xE8, 0x03, 0xE8, 0x03, 0xE8, 0x03, 0xE8, 0x03, 0xE8, 0x03, 0xE8,
        0x03,
    ];
    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR)).unwrap();
    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR2)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_packuswb_all_saturate_negative() {
    // All negative values saturate to 0
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x0b, 0x66, 0x0f, 0x67, 0xc1, 0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    // All words = -1000 (should saturate to 0)
    let data = [
        0x18, 0xFC, 0x18, 0xFC, 0x18, 0xFC, 0x18, 0xFC, 0x18, 0xFC, 0x18, 0xFC, 0x18, 0xFC, 0x18,
        0xFC,
    ];
    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR)).unwrap();
    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR2)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_packusdw_all_saturate_positive() {
    // All values saturate to 0xFFFF
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x0b, 0x66, 0x0f, 0x38, 0x2b, 0xc1, 0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    // All dwords = 100000 (should saturate to 65535)
    let data = [
        0xA0, 0x86, 0x01, 0x00, 0xA0, 0x86, 0x01, 0x00, 0xA0, 0x86, 0x01, 0x00, 0xA0, 0x86, 0x01,
        0x00,
    ];
    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR)).unwrap();
    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR2)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_packusdw_all_saturate_negative() {
    // All negative values saturate to 0
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x0b, 0x66, 0x0f, 0x38, 0x2b, 0xc1, 0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    // All dwords = -100000 (should saturate to 0)
    let data = [
        0x60, 0x79, 0xFE, 0xFF, 0x60, 0x79, 0xFE, 0xFF, 0x60, 0x79, 0xFE, 0xFF, 0x60, 0x79, 0xFE,
        0xFF,
    ];
    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR)).unwrap();
    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR2)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_packuswb_alternating_saturation() {
    // Alternating between saturation and normal values
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x0b, 0x66, 0x0f, 0x67, 0xc1, 0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    // Alternating: 10, 300, 20, 400, 30, 500, 40, 600
    let data1 = [
        0x0A, 0x00, 0x2C, 0x01, 0x14, 0x00, 0x90, 0x01, 0x1E, 0x00, 0xF4, 0x01, 0x28, 0x00, 0x58,
        0x02,
    ];
    // Alternating: -10, 255, -20, 256, -30, 0, -40, 1
    let data2 = [
        0xF6, 0xFF, 0xFF, 0x00, 0xEC, 0xFF, 0x00, 0x01, 0xE2, 0xFF, 0x00, 0x00, 0xD8, 0xFF, 0x01,
        0x00,
    ];
    mem.write_slice(&data1, GuestAddress(ALIGNED_ADDR)).unwrap();
    mem.write_slice(&data2, GuestAddress(ALIGNED_ADDR2))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_packusdw_alternating_saturation() {
    // Alternating between saturation and normal values
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x0b, 0x66, 0x0f, 0x38, 0x2b, 0xc1, 0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    // Alternating: 100, 70000, 1000, 100000
    let data1 = [
        0x64, 0x00, 0x00, 0x00, 0x70, 0x11, 0x01, 0x00, 0xE8, 0x03, 0x00, 0x00, 0xA0, 0x86, 0x01,
        0x00,
    ];
    // Alternating: -100, 65535, -1000, 65536
    let data2 = [
        0x9C, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0x18, 0xFC, 0xFF, 0xFF, 0x00, 0x00, 0x01,
        0x00,
    ];
    mem.write_slice(&data1, GuestAddress(ALIGNED_ADDR)).unwrap();
    mem.write_slice(&data2, GuestAddress(ALIGNED_ADDR2))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_packuswb_edge_cases() {
    // Test edge cases around boundaries
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x0b, 0x66, 0x0f, 0x67, 0xc1, 0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    // Words: 254, 255, 256, 257, -1, -2, 0, 1
    let data1 = [
        0xFE, 0x00, 0xFF, 0x00, 0x00, 0x01, 0x01, 0x01, 0xFF, 0xFF, 0xFE, 0xFF, 0x00, 0x00, 0x01,
        0x00,
    ];
    let data2 = [
        0x7F, 0x00, 0x80, 0x00, 0x81, 0x00, 0x00, 0x80, 0xFF, 0x7F, 0x01, 0x80, 0x00, 0x00, 0xFF,
        0xFF,
    ];
    mem.write_slice(&data1, GuestAddress(ALIGNED_ADDR)).unwrap();
    mem.write_slice(&data2, GuestAddress(ALIGNED_ADDR2))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_packusdw_edge_cases() {
    // Test edge cases around boundaries
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x0b, 0x66, 0x0f, 0x38, 0x2b, 0xc1, 0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    // Dwords: 65534, 65535, 65536, 65537
    let data1 = [
        0xFE, 0xFF, 0x00, 0x00, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x01, 0x00, 0x01,
        0x00,
    ];
    // Dwords: -1, -2, 0, 1
    let data2 = [
        0xFF, 0xFF, 0xFF, 0xFF, 0xFE, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00,
        0x00,
    ];
    mem.write_slice(&data1, GuestAddress(ALIGNED_ADDR)).unwrap();
    mem.write_slice(&data2, GuestAddress(ALIGNED_ADDR2))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Known-answer value tests (register-to-register via set_xmm/get_xmm)
//
// PACKUSWB takes 8 signed words from DST + 8 from SRC and saturates each to an
// UNSIGNED byte [0,255] (negative -> 0). DST fills the low 8 bytes, SRC high 8.
// Computed by hand.
//   DST = XMM0 = 0x7FFF8000010000FF00010002FFFEFF00
//   SRC = XMM1 = 0x0102030405060708A1A2B3B4C5D6E7F8
// ============================================================================

#[test]
fn kat_packuswb_value() {
    // PACKUSWB XMM0, XMM1 (66 0F 67 C1)
    let code = [0x66, 0x0f, 0x67, 0xc1, 0xf4];
    let (mut vcpu, mem) = setup_vm(&code, None);
    set_xmm(&mem, &mut vcpu, 0, 0x7FFF8000010000FF00010002FFFEFF00);
    set_xmm(&mem, &mut vcpu, 1, 0x0102030405060708A1A2B3B4C5D6E7F8);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        get_xmm(&regs, 0),
        0xffffffff00000000ff00ffff01020000,
        "PACKUSWB got {:032x}",
        get_xmm(&regs, 0)
    );
}

#[test]
fn kat_packuswb_negative_to_zero() {
    // Every negative source word clamps to 0; 0x7FFF clamps to 0xFF.
    let code = [0x66, 0x0f, 0x67, 0xc1, 0xf4];
    let (mut vcpu, mem) = setup_vm(&code, None);
    set_xmm(&mem, &mut vcpu, 0, 0x8000800080008000_7fff7fff7fff7fff);
    set_xmm(&mem, &mut vcpu, 1, 0x00ff00ff00ff00ff_0001000200030004);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    // DST low words 0x7FFF*4 -> 0xFF; DST high words 0x8000*4 -> 0x00.
    // SRC low words 0x0004,0x0003,0x0002,0x0001 -> bytes 0x04,0x03,0x02,0x01
    // (word 0 = least-significant -> lowest byte); SRC high words 0x00FF*4 -> 0xFF.
    assert_eq!(get_xmm(&regs, 0), 0xffffffff0102030400000000ffffffff);
}
