use crate::common::*;
use vm_memory::{Bytes, GuestAddress};

// PADDUSB/PADDUSW - Add Packed Unsigned Integers with Unsigned Saturation (SSE2)
//
// Performs SIMD add of packed unsigned integers from source and destination operands.
// Stores packed integer results in destination. Overflow handled with unsigned saturation.
//
// PADDUSB: Add 16 packed unsigned byte integers (8-bit each) with saturation to 0xFF
// PADDUSW: Add 8 packed unsigned word integers (16-bit each) with saturation to 0xFFFF
//
// Saturation behavior:
// - PADDUSB: Result > 0xFF saturates to 0xFF
// - PADDUSW: Result > 0xFFFF saturates to 0xFFFF
//
// Opcodes (SSE2 - 128-bit XMM):
// 66 0F DC /r      PADDUSB xmm1, xmm2/m128   - Add packed unsigned bytes with saturation
// 66 0F DD /r      PADDUSW xmm1, xmm2/m128   - Add packed unsigned words with saturation

const ALIGNED_ADDR: u64 = 0x3000;
const ALIGNED_ADDR2: u64 = 0x3100;

// ============================================================================
// PADDUSB Tests - 16x Unsigned Byte Addition with Saturation
// ============================================================================

#[test]
fn test_paddusb_all_zeros() {
    // PADDUSB XMM0, XMM1 with all zeros
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, // MOVDQA XMM0, [RAX]
        0x66, 0x0f, 0x6f, 0x0b, // MOVDQA XMM1, [RBX]
        0x66, 0x0f, 0xdc, 0xc1, // PADDUSB XMM0, XMM1
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
fn test_paddusb_small_values() {
    // PADDUSB XMM0, XMM1 with small values (no saturation)
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x0b, 0x66, 0x0f, 0xdc, 0xc1, 0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(
        &[
            0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10,
            0x10, 0x10,
        ],
        GuestAddress(ALIGNED_ADDR),
    )
    .unwrap();
    mem.write_slice(
        &[
            0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20,
            0x20, 0x20,
        ],
        GuestAddress(ALIGNED_ADDR2),
    )
    .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_paddusb_saturate_max() {
    // Test saturation to maximum (255 = 0xFF)
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x0b, 0x66, 0x0f, 0xdc, 0xc1, 0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    // 255 + 1 should saturate to 255
    mem.write_slice(
        &[
            0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
            0xFF, 0xFF,
        ],
        GuestAddress(ALIGNED_ADDR),
    )
    .unwrap();
    mem.write_slice(
        &[
            0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01,
            0x01, 0x01,
        ],
        GuestAddress(ALIGNED_ADDR2),
    )
    .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_paddusb_saturate_large_overflow() {
    // Test saturation with large overflow
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x0b, 0x66, 0x0f, 0xdc, 0xc1, 0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    // 200 + 200 = 400, should saturate to 255
    mem.write_slice(
        &[
            0xC8, 0xC8, 0xC8, 0xC8, 0xC8, 0xC8, 0xC8, 0xC8, 0xC8, 0xC8, 0xC8, 0xC8, 0xC8, 0xC8,
            0xC8, 0xC8,
        ],
        GuestAddress(ALIGNED_ADDR),
    )
    .unwrap();
    mem.write_slice(
        &[
            0xC8, 0xC8, 0xC8, 0xC8, 0xC8, 0xC8, 0xC8, 0xC8, 0xC8, 0xC8, 0xC8, 0xC8, 0xC8, 0xC8,
            0xC8, 0xC8,
        ],
        GuestAddress(ALIGNED_ADDR2),
    )
    .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_paddusb_all_ones() {
    // Test with all 0xFF values
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x0b, 0x66, 0x0f, 0xdc, 0xc1, 0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(
        &[
            0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
            0xFF, 0xFF,
        ],
        GuestAddress(ALIGNED_ADDR),
    )
    .unwrap();
    mem.write_slice(
        &[
            0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
            0xFF, 0xFF,
        ],
        GuestAddress(ALIGNED_ADDR2),
    )
    .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_paddusb_various_saturation_cases() {
    // Test different saturation cases in same register
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x0b, 0x66, 0x0f, 0xdc, 0xc1, 0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let src1 = [
        0xFF, 0x00, 0xC8, 0x80, 0x00, 0x01, 0xFE, 0x7F, 0xAA, 0x55, 0xFF, 0xFF, 0x80, 0x90, 0xA0,
        0x10,
    ];
    let src2 = [
        0x01, 0x00, 0xC8, 0x80, 0x00, 0xFF, 0x01, 0x80, 0x56, 0xAA, 0x01, 0x10, 0x90, 0x30, 0xE0,
        0x70,
    ];
    mem.write_slice(&src1, GuestAddress(ALIGNED_ADDR)).unwrap();
    mem.write_slice(&src2, GuestAddress(ALIGNED_ADDR2)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_paddusb_boundary_values() {
    // Test boundary values near saturation threshold
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x0b, 0x66, 0x0f, 0xdc, 0xc1, 0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let src1 = [
        0xFE, 0xFF, 0xFD, 0xFC, 0x80, 0x7F, 0xC0, 0xBF, 0x01, 0x02, 0xFE, 0xFD, 0xAA, 0x55, 0x00,
        0xFF,
    ];
    let src2 = [
        0x01, 0x01, 0x02, 0x03, 0x7F, 0x80, 0x3F, 0x40, 0xFE, 0xFD, 0x01, 0x02, 0x55, 0xAA, 0xFF,
        0x00,
    ];
    mem.write_slice(&src1, GuestAddress(ALIGNED_ADDR)).unwrap();
    mem.write_slice(&src2, GuestAddress(ALIGNED_ADDR2)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_paddusb_memory_operand() {
    // Test PADDUSB with memory operand
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, // MOVDQA XMM0, [RAX]
        0x66, 0x0f, 0xdc, 0x03, // PADDUSB XMM0, [RBX]
        0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(
        &[
            0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
            0xFF, 0xFF,
        ],
        GuestAddress(ALIGNED_ADDR),
    )
    .unwrap();
    mem.write_slice(
        &[
            0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01,
            0x01, 0x01,
        ],
        GuestAddress(ALIGNED_ADDR2),
    )
    .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_paddusb_xmm_self() {
    // Test PADDUSB with same register as source and destination
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, // MOVDQA XMM0, [RAX]
        0x66, 0x0f, 0xdc, 0xc0, // PADDUSB XMM0, XMM0
        0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(
        &[
            0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80,
            0x80, 0x80,
        ],
        GuestAddress(ALIGNED_ADDR),
    )
    .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_paddusb_alternating_pattern() {
    // Test with alternating pattern
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x0b, 0x66, 0x0f, 0xdc, 0xc1, 0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let src1 = [
        0xFF, 0x00, 0xFF, 0x00, 0xFF, 0x00, 0xFF, 0x00, 0xFF, 0x00, 0xFF, 0x00, 0xFF, 0x00, 0xFF,
        0x00,
    ];
    let src2 = [
        0xFF, 0x00, 0xFF, 0x00, 0xFF, 0x00, 0xFF, 0x00, 0xFF, 0x00, 0xFF, 0x00, 0xFF, 0x00, 0xFF,
        0x00,
    ];
    mem.write_slice(&src1, GuestAddress(ALIGNED_ADDR)).unwrap();
    mem.write_slice(&src2, GuestAddress(ALIGNED_ADDR2)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_paddusb_incremental_values() {
    // Test with incremental values
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x0b, 0x66, 0x0f, 0xdc, 0xc1, 0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let src1 = [
        0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E,
        0x0F,
    ];
    let src2 = [
        0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B, 0x1C, 0x1D, 0x1E,
        0x1F,
    ];
    mem.write_slice(&src1, GuestAddress(ALIGNED_ADDR)).unwrap();
    mem.write_slice(&src2, GuestAddress(ALIGNED_ADDR2)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_paddusb_different_registers() {
    // Test with different XMM registers
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x10, // MOVDQA XMM2, [RAX]
        0x66, 0x0f, 0x6f, 0x1b, // MOVDQA XMM3, [RBX]
        0x66, 0x0f, 0xdc, 0xd3, // PADDUSB XMM2, XMM3
        0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(
        &[
            0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
            0xFF, 0xFF,
        ],
        GuestAddress(ALIGNED_ADDR),
    )
    .unwrap();
    mem.write_slice(
        &[
            0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01,
            0x01, 0x01,
        ],
        GuestAddress(ALIGNED_ADDR2),
    )
    .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_paddusb_sequential_operations() {
    // Test multiple sequential PADDUSB operations
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, // MOVDQA XMM0, [RAX]
        0x66, 0x0f, 0x6f, 0x0b, // MOVDQA XMM1, [RBX]
        0x66, 0x0f, 0xdc, 0xc1, // PADDUSB XMM0, XMM1
        0x66, 0x0f, 0xdc, 0xc1, // PADDUSB XMM0, XMM1
        0x66, 0x0f, 0xdc, 0xc1, // PADDUSB XMM0, XMM1
        0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(
        &[
            0x50, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50,
            0x50, 0x50,
        ],
        GuestAddress(ALIGNED_ADDR),
    )
    .unwrap();
    mem.write_slice(
        &[
            0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30,
            0x30, 0x30,
        ],
        GuestAddress(ALIGNED_ADDR2),
    )
    .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_paddusb_near_saturation() {
    // Test values very close to saturation limit
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x0b, 0x66, 0x0f, 0xdc, 0xc1, 0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let src1 = [
        0xFE, 0xFD, 0xFC, 0xFB, 0xFA, 0xF9, 0xF8, 0xF7, 0xF6, 0xF5, 0xF4, 0xF3, 0xF2, 0xF1, 0xF0,
        0xEF,
    ];
    let src2 = [
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F,
        0x10,
    ];
    mem.write_slice(&src1, GuestAddress(ALIGNED_ADDR)).unwrap();
    mem.write_slice(&src2, GuestAddress(ALIGNED_ADDR2)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_paddusb_halfway_values() {
    // Test with values around 128 (halfway point)
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x0b, 0x66, 0x0f, 0xdc, 0xc1, 0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(
        &[
            0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80,
            0x80, 0x80,
        ],
        GuestAddress(ALIGNED_ADDR),
    )
    .unwrap();
    mem.write_slice(
        &[
            0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80,
            0x80, 0x80,
        ],
        GuestAddress(ALIGNED_ADDR2),
    )
    .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_paddusb_mixed_small_large() {
    // Test mix of small values and values that will saturate
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x0b, 0x66, 0x0f, 0xdc, 0xc1, 0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let src1 = [
        0x01, 0xFF, 0x10, 0xF0, 0x20, 0xE0, 0x30, 0xD0, 0x40, 0xC0, 0x50, 0xB0, 0x60, 0xA0, 0x70,
        0x90,
    ];
    let src2 = [
        0x01, 0xFF, 0x10, 0xF0, 0x20, 0xE0, 0x30, 0xD0, 0x40, 0xC0, 0x50, 0xB0, 0x60, 0xA0, 0x70,
        0x90,
    ];
    mem.write_slice(&src1, GuestAddress(ALIGNED_ADDR)).unwrap();
    mem.write_slice(&src2, GuestAddress(ALIGNED_ADDR2)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_paddusb_powers_of_two() {
    // Test with powers of two
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x0b, 0x66, 0x0f, 0xdc, 0xc1, 0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let src1 = [
        0x01, 0x02, 0x04, 0x08, 0x10, 0x20, 0x40, 0x80, 0x01, 0x02, 0x04, 0x08, 0x10, 0x20, 0x40,
        0x80,
    ];
    let src2 = [
        0x01, 0x02, 0x04, 0x08, 0x10, 0x20, 0x40, 0x80, 0x01, 0x02, 0x04, 0x08, 0x10, 0x20, 0x40,
        0x80,
    ];
    mem.write_slice(&src1, GuestAddress(ALIGNED_ADDR)).unwrap();
    mem.write_slice(&src2, GuestAddress(ALIGNED_ADDR2)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// PADDUSW Tests - 8x Unsigned Word Addition with Saturation
// ============================================================================

#[test]
fn test_paddusw_all_zeros() {
    // PADDUSW XMM0, XMM1 with all zeros
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, // MOVDQA XMM0, [RAX]
        0x66, 0x0f, 0x6f, 0x0b, // MOVDQA XMM1, [RBX]
        0x66, 0x0f, 0xdd, 0xc1, // PADDUSW XMM0, XMM1
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
fn test_paddusw_small_values() {
    // PADDUSW XMM0, XMM1 with small values (no saturation)
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x0b, 0x66, 0x0f, 0xdd, 0xc1, 0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let src1 = [
        0x00, 0x10, 0x00, 0x20, 0x00, 0x30, 0x00, 0x40, 0x00, 0x50, 0x00, 0x60, 0x00, 0x70, 0x00,
        0x01,
    ];
    let src2 = [
        0x00, 0x10, 0x00, 0x20, 0x00, 0x30, 0x00, 0x40, 0x00, 0x50, 0x00, 0x60, 0x00, 0x70, 0x00,
        0x01,
    ];
    mem.write_slice(&src1, GuestAddress(ALIGNED_ADDR)).unwrap();
    mem.write_slice(&src2, GuestAddress(ALIGNED_ADDR2)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_paddusw_saturate_max() {
    // Test saturation to maximum (65535 = 0xFFFF)
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x0b, 0x66, 0x0f, 0xdd, 0xc1, 0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    // 65535 + 1 should saturate to 65535
    let src1 = [
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF,
    ];
    let src2 = [
        0x01, 0x00, 0x01, 0x00, 0x01, 0x00, 0x01, 0x00, 0x01, 0x00, 0x01, 0x00, 0x01, 0x00, 0x01,
        0x00,
    ];
    mem.write_slice(&src1, GuestAddress(ALIGNED_ADDR)).unwrap();
    mem.write_slice(&src2, GuestAddress(ALIGNED_ADDR2)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_paddusw_saturate_large_overflow() {
    // Test saturation with large overflow
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x0b, 0x66, 0x0f, 0xdd, 0xc1, 0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    // 50000 + 50000 = 100000, should saturate to 65535
    let src1 = [
        0x50, 0xC3, 0x50, 0xC3, 0x50, 0xC3, 0x50, 0xC3, 0x50, 0xC3, 0x50, 0xC3, 0x50, 0xC3, 0x50,
        0xC3,
    ];
    let src2 = [
        0x50, 0xC3, 0x50, 0xC3, 0x50, 0xC3, 0x50, 0xC3, 0x50, 0xC3, 0x50, 0xC3, 0x50, 0xC3, 0x50,
        0xC3,
    ];
    mem.write_slice(&src1, GuestAddress(ALIGNED_ADDR)).unwrap();
    mem.write_slice(&src2, GuestAddress(ALIGNED_ADDR2)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_paddusw_all_ones() {
    // Test with all 0xFFFF values
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x0b, 0x66, 0x0f, 0xdd, 0xc1, 0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(
        &[
            0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
            0xFF, 0xFF,
        ],
        GuestAddress(ALIGNED_ADDR),
    )
    .unwrap();
    mem.write_slice(
        &[
            0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
            0xFF, 0xFF,
        ],
        GuestAddress(ALIGNED_ADDR2),
    )
    .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_paddusw_various_saturation_cases() {
    // Test different saturation cases in same register
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x0b, 0x66, 0x0f, 0xdd, 0xc1, 0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let src1 = [
        0xFF, 0xFF, 0x00, 0x00, 0x00, 0x80, 0x00, 0x40, 0x00, 0x00, 0x01, 0x00, 0xFF, 0xFF, 0xFE,
        0xFF,
    ];
    let src2 = [
        0x01, 0x00, 0x00, 0x00, 0x00, 0x80, 0x00, 0x40, 0x00, 0x00, 0xFF, 0xFF, 0x01, 0x00, 0x01,
        0x00,
    ];
    mem.write_slice(&src1, GuestAddress(ALIGNED_ADDR)).unwrap();
    mem.write_slice(&src2, GuestAddress(ALIGNED_ADDR2)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_paddusw_memory_operand() {
    // Test PADDUSW with memory operand
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, // MOVDQA XMM0, [RAX]
        0x66, 0x0f, 0xdd, 0x03, // PADDUSW XMM0, [RBX]
        0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let src1 = [
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF,
    ];
    let src2 = [
        0x01, 0x00, 0x01, 0x00, 0x01, 0x00, 0x01, 0x00, 0x01, 0x00, 0x01, 0x00, 0x01, 0x00, 0x01,
        0x00,
    ];
    mem.write_slice(&src1, GuestAddress(ALIGNED_ADDR)).unwrap();
    mem.write_slice(&src2, GuestAddress(ALIGNED_ADDR2)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_paddusw_xmm_self() {
    // Test PADDUSW with same register as source and destination
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, // MOVDQA XMM0, [RAX]
        0x66, 0x0f, 0xdd, 0xc0, // PADDUSW XMM0, XMM0
        0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let src = [
        0x00, 0x80, 0x00, 0x80, 0x00, 0x80, 0x00, 0x80, 0x00, 0x80, 0x00, 0x80, 0x00, 0x80, 0x00,
        0x80,
    ];
    mem.write_slice(&src, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_paddusw_alternating_pattern() {
    // Test with alternating pattern
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x0b, 0x66, 0x0f, 0xdd, 0xc1, 0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let src1 = [
        0xFF, 0xFF, 0x00, 0x00, 0xFF, 0xFF, 0x00, 0x00, 0xFF, 0xFF, 0x00, 0x00, 0xFF, 0xFF, 0x00,
        0x00,
    ];
    let src2 = [
        0xFF, 0xFF, 0x00, 0x00, 0xFF, 0xFF, 0x00, 0x00, 0xFF, 0xFF, 0x00, 0x00, 0xFF, 0xFF, 0x00,
        0x00,
    ];
    mem.write_slice(&src1, GuestAddress(ALIGNED_ADDR)).unwrap();
    mem.write_slice(&src2, GuestAddress(ALIGNED_ADDR2)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_paddusw_incremental_values() {
    // Test with incremental values
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x0b, 0x66, 0x0f, 0xdd, 0xc1, 0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let src1 = [
        0x00, 0x01, 0x00, 0x02, 0x00, 0x03, 0x00, 0x04, 0x00, 0x05, 0x00, 0x06, 0x00, 0x07, 0x00,
        0x08,
    ];
    let src2 = [
        0x00, 0x10, 0x00, 0x11, 0x00, 0x12, 0x00, 0x13, 0x00, 0x14, 0x00, 0x15, 0x00, 0x16, 0x00,
        0x17,
    ];
    mem.write_slice(&src1, GuestAddress(ALIGNED_ADDR)).unwrap();
    mem.write_slice(&src2, GuestAddress(ALIGNED_ADDR2)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_paddusw_different_registers() {
    // Test with different XMM registers
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x38, // MOVDQA XMM7, [RAX]
        0x66, 0x0f, 0x6f, 0x03, // MOVDQA XMM0, [RBX]
        0x66, 0x0f, 0xdd, 0xf8, // PADDUSW XMM7, XMM0
        0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let src1 = [
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF,
    ];
    let src2 = [
        0x01, 0x00, 0x01, 0x00, 0x01, 0x00, 0x01, 0x00, 0x01, 0x00, 0x01, 0x00, 0x01, 0x00, 0x01,
        0x00,
    ];
    mem.write_slice(&src1, GuestAddress(ALIGNED_ADDR)).unwrap();
    mem.write_slice(&src2, GuestAddress(ALIGNED_ADDR2)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_paddusw_sequential_operations() {
    // Test multiple sequential PADDUSW operations
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, // MOVDQA XMM0, [RAX]
        0x66, 0x0f, 0x6f, 0x0b, // MOVDQA XMM1, [RBX]
        0x66, 0x0f, 0xdd, 0xc1, // PADDUSW XMM0, XMM1
        0x66, 0x0f, 0xdd, 0xc1, // PADDUSW XMM0, XMM1
        0x66, 0x0f, 0xdd, 0xc1, // PADDUSW XMM0, XMM1
        0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let src1 = [
        0x00, 0x20, 0x00, 0x20, 0x00, 0x20, 0x00, 0x20, 0x00, 0x20, 0x00, 0x20, 0x00, 0x20, 0x00,
        0x20,
    ];
    let src2 = [
        0x00, 0x10, 0x00, 0x10, 0x00, 0x10, 0x00, 0x10, 0x00, 0x10, 0x00, 0x10, 0x00, 0x10, 0x00,
        0x10,
    ];
    mem.write_slice(&src1, GuestAddress(ALIGNED_ADDR)).unwrap();
    mem.write_slice(&src2, GuestAddress(ALIGNED_ADDR2)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_paddusw_near_saturation() {
    // Test values very close to saturation limit
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x0b, 0x66, 0x0f, 0xdd, 0xc1, 0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let src1 = [
        0xFE, 0xFF, 0xFD, 0xFF, 0xFC, 0xFF, 0xFB, 0xFF, 0xFA, 0xFF, 0xF9, 0xFF, 0xF8, 0xFF, 0xF7,
        0xFF,
    ];
    let src2 = [
        0x01, 0x00, 0x02, 0x00, 0x03, 0x00, 0x04, 0x00, 0x05, 0x00, 0x06, 0x00, 0x07, 0x00, 0x08,
        0x00,
    ];
    mem.write_slice(&src1, GuestAddress(ALIGNED_ADDR)).unwrap();
    mem.write_slice(&src2, GuestAddress(ALIGNED_ADDR2)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_paddusw_halfway_values() {
    // Test with values around 32768 (halfway point)
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x0b, 0x66, 0x0f, 0xdd, 0xc1, 0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let src = [
        0x00, 0x80, 0x00, 0x80, 0x00, 0x80, 0x00, 0x80, 0x00, 0x80, 0x00, 0x80, 0x00, 0x80, 0x00,
        0x80,
    ];
    mem.write_slice(&src, GuestAddress(ALIGNED_ADDR)).unwrap();
    mem.write_slice(&src, GuestAddress(ALIGNED_ADDR2)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_paddusw_boundary_values() {
    // Test boundary values near saturation threshold
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x0b, 0x66, 0x0f, 0xdd, 0xc1, 0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let src1 = [
        0xFE, 0xFF, 0x00, 0x00, 0x00, 0x80, 0x00, 0x40, 0x01, 0x00, 0xFF, 0xFF, 0x00, 0x00, 0xFF,
        0x00,
    ];
    let src2 = [
        0x01, 0x00, 0x00, 0x00, 0x00, 0x7F, 0x00, 0x40, 0xFE, 0xFF, 0x01, 0x00, 0x00, 0x00, 0x01,
        0x00,
    ];
    mem.write_slice(&src1, GuestAddress(ALIGNED_ADDR)).unwrap();
    mem.write_slice(&src2, GuestAddress(ALIGNED_ADDR2)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_paddusw_mixed_small_large() {
    // Test mix of small values and values that will saturate
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x0b, 0x66, 0x0f, 0xdd, 0xc1, 0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let src1 = [
        0x00, 0x01, 0xFF, 0xFF, 0x00, 0x10, 0x00, 0xF0, 0x00, 0x20, 0x00, 0xE0, 0x00, 0x30, 0x00,
        0xD0,
    ];
    let src2 = [
        0x00, 0x01, 0xFF, 0xFF, 0x00, 0x10, 0x00, 0xF0, 0x00, 0x20, 0x00, 0xE0, 0x00, 0x30, 0x00,
        0xD0,
    ];
    mem.write_slice(&src1, GuestAddress(ALIGNED_ADDR)).unwrap();
    mem.write_slice(&src2, GuestAddress(ALIGNED_ADDR2)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_paddusw_powers_of_two() {
    // Test with powers of two
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x0b, 0x66, 0x0f, 0xdd, 0xc1, 0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let src1 = [
        0x01, 0x00, 0x02, 0x00, 0x04, 0x00, 0x08, 0x00, 0x10, 0x00, 0x20, 0x00, 0x40, 0x00, 0x80,
        0x00,
    ];
    let src2 = [
        0x01, 0x00, 0x02, 0x00, 0x04, 0x00, 0x08, 0x00, 0x10, 0x00, 0x20, 0x00, 0x40, 0x00, 0x80,
        0x00,
    ];
    mem.write_slice(&src1, GuestAddress(ALIGNED_ADDR)).unwrap();
    mem.write_slice(&src2, GuestAddress(ALIGNED_ADDR2)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Known-answer value tests (register-to-register via set_xmm/get_xmm)
//
// Unsigned saturating add. Each lane clamps to [0,255] (byte) / [0,65535]
// (word). Computed by hand from x86 PADDUSB/PADDUSW semantics.
// ============================================================================

#[test]
fn kat_paddusb_value() {
    // PADDUSB XMM0, XMM1 (66 0F DC C1)
    let code = [0x66, 0x0f, 0xdc, 0xc1, 0xf4];
    let (mut vcpu, mem) = setup_vm(&code, None);
    set_xmm(&mem, &mut vcpu, 0, 0x7f7f80800102fe007f01ff80fe7f0100);
    set_xmm(&mem, &mut vcpu, 1, 0x010180800102020101ff01800201ff00);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        get_xmm(&regs, 0),
        0x8080ffff0204ff0180ffffffff80ff00,
        "PADDUSB got {:032x}",
        get_xmm(&regs, 0)
    );
}

#[test]
fn kat_paddusw_value() {
    // PADDUSW XMM0, XMM1 (66 0F DD C1)
    let code = [0x66, 0x0f, 0xdd, 0xc1, 0xf4];
    let (mut vcpu, mem) = setup_vm(&code, None);
    set_xmm(&mem, &mut vcpu, 0, 0x7fff80000001fffe7fff8000ffff0001);
    set_xmm(&mem, &mut vcpu, 1, 0x0001800000027fff0001ffff00018000);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        get_xmm(&regs, 0),
        0x8000ffff0003ffff8000ffffffff8001,
        "PADDUSW got {:032x}",
        get_xmm(&regs, 0)
    );
}

#[test]
fn kat_paddusb_saturation_all() {
    // 0xFF + 0x01 saturates to 0xFF (unsigned) in every byte lane.
    let code = [0x66, 0x0f, 0xdc, 0xc1, 0xf4];
    let (mut vcpu, mem) = setup_vm(&code, None);
    set_xmm(&mem, &mut vcpu, 0, 0xffffffffffffffffffffffffffffffff);
    set_xmm(&mem, &mut vcpu, 1, 0x01010101010101010101010101010101);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(get_xmm(&regs, 0), 0xffffffffffffffffffffffffffffffff);
}

#[test]
fn kat_paddusw_no_saturation() {
    // Below the cap, PADDUSW is ordinary addition.
    let code = [0x66, 0x0f, 0xdd, 0xc1, 0xf4];
    let (mut vcpu, mem) = setup_vm(&code, None);
    set_xmm(&mem, &mut vcpu, 0, 0x00010002000300040005000600070008);
    set_xmm(&mem, &mut vcpu, 1, 0x00100020003000400050006000700080);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(get_xmm(&regs, 0), 0x00110022003300440055006600770088);
}
