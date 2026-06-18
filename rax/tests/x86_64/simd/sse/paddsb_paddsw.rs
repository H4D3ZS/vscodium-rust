use crate::common::*;
use vm_memory::{Bytes, GuestAddress};

// PADDSB/PADDSW - Add Packed Signed Integers with Signed Saturation (SSE2)
//
// Performs SIMD add of packed signed integers from source and destination operands.
// Stores packed integer results in destination. Overflow handled with signed saturation.
//
// PADDSB: Add 16 packed signed byte integers (8-bit each) with saturation to INT8_MIN/MAX
// PADDSW: Add 8 packed signed word integers (16-bit each) with saturation to INT16_MIN/MAX
//
// Saturation behavior:
// - PADDSB: Result > 0x7F saturates to 0x7F, result < 0x80 saturates to 0x80
// - PADDSW: Result > 0x7FFF saturates to 0x7FFF, result < 0x8000 saturates to 0x8000
//
// Opcodes (SSE2 - 128-bit XMM):
// 66 0F EC /r      PADDSB xmm1, xmm2/m128   - Add packed signed bytes with saturation
// 66 0F ED /r      PADDSW xmm1, xmm2/m128   - Add packed signed words with saturation

const ALIGNED_ADDR: u64 = 0x3000;
const ALIGNED_ADDR2: u64 = 0x3100;

// ============================================================================
// PADDSB Tests - 16x Signed Byte Addition with Saturation
// ============================================================================

#[test]
fn test_paddsb_all_zeros() {
    // PADDSB XMM0, XMM1 with all zeros
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, // MOVDQA XMM0, [RAX]
        0x66, 0x0f, 0x6f, 0x0b, // MOVDQA XMM1, [RBX]
        0x66, 0x0f, 0xec, 0xc1, // PADDSB XMM0, XMM1
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
fn test_paddsb_positive_values() {
    // PADDSB XMM0, XMM1 with positive values (no saturation)
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x0b, 0x66, 0x0f, 0xec, 0xc1, 0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    // 0x10 + 0x20 = 0x30
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
fn test_paddsb_negative_values() {
    // PADDSB XMM0, XMM1 with negative values (no saturation)
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x0b, 0x66, 0x0f, 0xec, 0xc1, 0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    // -16 + -16 = -32 (0xF0 + 0xF0 = 0xE0)
    mem.write_slice(
        &[
            0xF0, 0xF0, 0xF0, 0xF0, 0xF0, 0xF0, 0xF0, 0xF0, 0xF0, 0xF0, 0xF0, 0xF0, 0xF0, 0xF0,
            0xF0, 0xF0,
        ],
        GuestAddress(ALIGNED_ADDR),
    )
    .unwrap();
    mem.write_slice(
        &[
            0xF0, 0xF0, 0xF0, 0xF0, 0xF0, 0xF0, 0xF0, 0xF0, 0xF0, 0xF0, 0xF0, 0xF0, 0xF0, 0xF0,
            0xF0, 0xF0,
        ],
        GuestAddress(ALIGNED_ADDR2),
    )
    .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_paddsb_saturate_positive_max() {
    // Test saturation to positive maximum (127 = 0x7F)
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x0b, 0x66, 0x0f, 0xec, 0xc1, 0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    // 127 + 1 should saturate to 127
    mem.write_slice(
        &[
            0x7F, 0x7F, 0x7F, 0x7F, 0x7F, 0x7F, 0x7F, 0x7F, 0x7F, 0x7F, 0x7F, 0x7F, 0x7F, 0x7F,
            0x7F, 0x7F,
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
fn test_paddsb_saturate_positive_max_large() {
    // Test saturation with large positive overflow
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x0b, 0x66, 0x0f, 0xec, 0xc1, 0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    // 100 + 100 = 200, should saturate to 127
    mem.write_slice(
        &[
            0x64, 0x64, 0x64, 0x64, 0x64, 0x64, 0x64, 0x64, 0x64, 0x64, 0x64, 0x64, 0x64, 0x64,
            0x64, 0x64,
        ],
        GuestAddress(ALIGNED_ADDR),
    )
    .unwrap();
    mem.write_slice(
        &[
            0x64, 0x64, 0x64, 0x64, 0x64, 0x64, 0x64, 0x64, 0x64, 0x64, 0x64, 0x64, 0x64, 0x64,
            0x64, 0x64,
        ],
        GuestAddress(ALIGNED_ADDR2),
    )
    .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_paddsb_saturate_negative_min() {
    // Test saturation to negative minimum (-128 = 0x80)
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x0b, 0x66, 0x0f, 0xec, 0xc1, 0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    // -128 + -1 should saturate to -128
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
            0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
            0xFF, 0xFF,
        ],
        GuestAddress(ALIGNED_ADDR2),
    )
    .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_paddsb_saturate_negative_min_large() {
    // Test saturation with large negative overflow
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x0b, 0x66, 0x0f, 0xec, 0xc1, 0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    // -100 + -100 = -200, should saturate to -128 (0x80)
    mem.write_slice(
        &[
            0x9C, 0x9C, 0x9C, 0x9C, 0x9C, 0x9C, 0x9C, 0x9C, 0x9C, 0x9C, 0x9C, 0x9C, 0x9C, 0x9C,
            0x9C, 0x9C,
        ],
        GuestAddress(ALIGNED_ADDR),
    )
    .unwrap();
    mem.write_slice(
        &[
            0x9C, 0x9C, 0x9C, 0x9C, 0x9C, 0x9C, 0x9C, 0x9C, 0x9C, 0x9C, 0x9C, 0x9C, 0x9C, 0x9C,
            0x9C, 0x9C,
        ],
        GuestAddress(ALIGNED_ADDR2),
    )
    .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_paddsb_mixed_positive_negative() {
    // Test mixed positive and negative values
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x0b, 0x66, 0x0f, 0xec, 0xc1, 0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    // 50 + (-30) = 20
    mem.write_slice(
        &[
            0x32, 0x32, 0x32, 0x32, 0x32, 0x32, 0x32, 0x32, 0x32, 0x32, 0x32, 0x32, 0x32, 0x32,
            0x32, 0x32,
        ],
        GuestAddress(ALIGNED_ADDR),
    )
    .unwrap();
    mem.write_slice(
        &[
            0xE2, 0xE2, 0xE2, 0xE2, 0xE2, 0xE2, 0xE2, 0xE2, 0xE2, 0xE2, 0xE2, 0xE2, 0xE2, 0xE2,
            0xE2, 0xE2,
        ],
        GuestAddress(ALIGNED_ADDR2),
    )
    .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_paddsb_various_saturation_cases() {
    // Test different saturation cases in same register
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x0b, 0x66, 0x0f, 0xec, 0xc1, 0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let src1 = [
        0x7F, 0x80, 0x64, 0x9C, 0x00, 0x01, 0xFF, 0x7E, 0x81, 0x40, 0xC0, 0x7F, 0x80, 0x50, 0xA0,
        0x10,
    ];
    let src2 = [
        0x01, 0xFF, 0x64, 0x9C, 0x00, 0xFF, 0x01, 0x01, 0xFF, 0x40, 0xC0, 0x10, 0x90, 0x30, 0xE0,
        0x70,
    ];
    mem.write_slice(&src1, GuestAddress(ALIGNED_ADDR)).unwrap();
    mem.write_slice(&src2, GuestAddress(ALIGNED_ADDR2)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_paddsb_boundary_values() {
    // Test boundary values near saturation thresholds
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x0b, 0x66, 0x0f, 0xec, 0xc1, 0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let src1 = [
        0x7E, 0x7F, 0x81, 0x80, 0x40, 0x3F, 0xC0, 0xBF, 0x7D, 0x82, 0x7C, 0x83, 0x01, 0x02, 0xFE,
        0xFD,
    ];
    let src2 = [
        0x01, 0x01, 0xFF, 0xFF, 0x3F, 0x40, 0xC0, 0xBF, 0x02, 0xFE, 0x03, 0xFD, 0x7E, 0x7D, 0x82,
        0x83,
    ];
    mem.write_slice(&src1, GuestAddress(ALIGNED_ADDR)).unwrap();
    mem.write_slice(&src2, GuestAddress(ALIGNED_ADDR2)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_paddsb_memory_operand() {
    // Test PADDSB with memory operand
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, // MOVDQA XMM0, [RAX]
        0x66, 0x0f, 0xec, 0x03, // PADDSB XMM0, [RBX]
        0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(
        &[
            0x7F, 0x7F, 0x7F, 0x7F, 0x7F, 0x7F, 0x7F, 0x7F, 0x7F, 0x7F, 0x7F, 0x7F, 0x7F, 0x7F,
            0x7F, 0x7F,
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
fn test_paddsb_xmm_self() {
    // Test PADDSB with same register as source and destination
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, // MOVDQA XMM0, [RAX]
        0x66, 0x0f, 0xec, 0xc0, // PADDSB XMM0, XMM0
        0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(
        &[
            0x40, 0x40, 0x40, 0x40, 0x40, 0x40, 0x40, 0x40, 0x40, 0x40, 0x40, 0x40, 0x40, 0x40,
            0x40, 0x40,
        ],
        GuestAddress(ALIGNED_ADDR),
    )
    .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_paddsb_alternating_pattern() {
    // Test with alternating positive/negative pattern
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x0b, 0x66, 0x0f, 0xec, 0xc1, 0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let src1 = [
        0x7F, 0x80, 0x7F, 0x80, 0x7F, 0x80, 0x7F, 0x80, 0x7F, 0x80, 0x7F, 0x80, 0x7F, 0x80, 0x7F,
        0x80,
    ];
    let src2 = [
        0x7F, 0x80, 0x7F, 0x80, 0x7F, 0x80, 0x7F, 0x80, 0x7F, 0x80, 0x7F, 0x80, 0x7F, 0x80, 0x7F,
        0x80,
    ];
    mem.write_slice(&src1, GuestAddress(ALIGNED_ADDR)).unwrap();
    mem.write_slice(&src2, GuestAddress(ALIGNED_ADDR2)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_paddsb_incremental_values() {
    // Test with incremental values
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x0b, 0x66, 0x0f, 0xec, 0xc1, 0xf4,
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
fn test_paddsb_all_ff() {
    // Test with all 0xFF bytes (-1 in signed interpretation)
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x0b, 0x66, 0x0f, 0xec, 0xc1, 0xf4,
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
fn test_paddsb_different_registers() {
    // Test with different XMM registers
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x18, // MOVDQA XMM3, [RAX]
        0x66, 0x0f, 0x6f, 0x23, // MOVDQA XMM4, [RBX]
        0x66, 0x0f, 0xec, 0xdc, // PADDSB XMM3, XMM4
        0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(
        &[
            0x7F, 0x7F, 0x7F, 0x7F, 0x7F, 0x7F, 0x7F, 0x7F, 0x7F, 0x7F, 0x7F, 0x7F, 0x7F, 0x7F,
            0x7F, 0x7F,
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
fn test_paddsb_sequential_operations() {
    // Test multiple sequential PADDSB operations
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, // MOVDQA XMM0, [RAX]
        0x66, 0x0f, 0x6f, 0x0b, // MOVDQA XMM1, [RBX]
        0x66, 0x0f, 0xec, 0xc1, // PADDSB XMM0, XMM1
        0x66, 0x0f, 0xec, 0xc1, // PADDSB XMM0, XMM1
        0x66, 0x0f, 0xec, 0xc1, // PADDSB XMM0, XMM1
        0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(
        &[
            0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20,
            0x20, 0x20,
        ],
        GuestAddress(ALIGNED_ADDR),
    )
    .unwrap();
    mem.write_slice(
        &[
            0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10,
            0x10, 0x10,
        ],
        GuestAddress(ALIGNED_ADDR2),
    )
    .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_paddsb_near_saturation_positive() {
    // Test values very close to positive saturation limit
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x0b, 0x66, 0x0f, 0xec, 0xc1, 0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let src1 = [
        0x7E, 0x7D, 0x7C, 0x7B, 0x7A, 0x79, 0x78, 0x77, 0x76, 0x75, 0x74, 0x73, 0x72, 0x71, 0x70,
        0x6F,
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
fn test_paddsb_near_saturation_negative() {
    // Test values very close to negative saturation limit
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x0b, 0x66, 0x0f, 0xec, 0xc1, 0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let src1 = [
        0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8A, 0x8B, 0x8C, 0x8D, 0x8E, 0x8F,
        0x90,
    ];
    let src2 = [
        0xFF, 0xFE, 0xFD, 0xFC, 0xFB, 0xFA, 0xF9, 0xF8, 0xF7, 0xF6, 0xF5, 0xF4, 0xF3, 0xF2, 0xF1,
        0xF0,
    ];
    mem.write_slice(&src1, GuestAddress(ALIGNED_ADDR)).unwrap();
    mem.write_slice(&src2, GuestAddress(ALIGNED_ADDR2)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// PADDSW Tests - 8x Signed Word Addition with Saturation
// ============================================================================

#[test]
fn test_paddsw_all_zeros() {
    // PADDSW XMM0, XMM1 with all zeros
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, // MOVDQA XMM0, [RAX]
        0x66, 0x0f, 0x6f, 0x0b, // MOVDQA XMM1, [RBX]
        0x66, 0x0f, 0xed, 0xc1, // PADDSW XMM0, XMM1
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
fn test_paddsw_positive_values() {
    // PADDSW XMM0, XMM1 with positive values (no saturation)
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x0b, 0x66, 0x0f, 0xed, 0xc1, 0xf4,
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
fn test_paddsw_negative_values() {
    // PADDSW XMM0, XMM1 with negative values (no saturation)
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x0b, 0x66, 0x0f, 0xed, 0xc1, 0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    // -256 + -256 = -512
    let src1 = [
        0x00, 0xFF, 0x00, 0xFF, 0x00, 0xFF, 0x00, 0xFF, 0x00, 0xFF, 0x00, 0xFF, 0x00, 0xFF, 0x00,
        0xFF,
    ];
    let src2 = [
        0x00, 0xFF, 0x00, 0xFF, 0x00, 0xFF, 0x00, 0xFF, 0x00, 0xFF, 0x00, 0xFF, 0x00, 0xFF, 0x00,
        0xFF,
    ];
    mem.write_slice(&src1, GuestAddress(ALIGNED_ADDR)).unwrap();
    mem.write_slice(&src2, GuestAddress(ALIGNED_ADDR2)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_paddsw_saturate_positive_max() {
    // Test saturation to positive maximum (32767 = 0x7FFF)
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x0b, 0x66, 0x0f, 0xed, 0xc1, 0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    // 32767 + 1 should saturate to 32767
    let src1 = [
        0xFF, 0x7F, 0xFF, 0x7F, 0xFF, 0x7F, 0xFF, 0x7F, 0xFF, 0x7F, 0xFF, 0x7F, 0xFF, 0x7F, 0xFF,
        0x7F,
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
fn test_paddsw_saturate_positive_max_large() {
    // Test saturation with large positive overflow
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x0b, 0x66, 0x0f, 0xed, 0xc1, 0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    // 20000 + 20000 = 40000, should saturate to 32767
    let src1 = [
        0x20, 0x4E, 0x20, 0x4E, 0x20, 0x4E, 0x20, 0x4E, 0x20, 0x4E, 0x20, 0x4E, 0x20, 0x4E, 0x20,
        0x4E,
    ];
    let src2 = [
        0x20, 0x4E, 0x20, 0x4E, 0x20, 0x4E, 0x20, 0x4E, 0x20, 0x4E, 0x20, 0x4E, 0x20, 0x4E, 0x20,
        0x4E,
    ];
    mem.write_slice(&src1, GuestAddress(ALIGNED_ADDR)).unwrap();
    mem.write_slice(&src2, GuestAddress(ALIGNED_ADDR2)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_paddsw_saturate_negative_min() {
    // Test saturation to negative minimum (-32768 = 0x8000)
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x0b, 0x66, 0x0f, 0xed, 0xc1, 0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    // -32768 + -1 should saturate to -32768
    let src1 = [
        0x00, 0x80, 0x00, 0x80, 0x00, 0x80, 0x00, 0x80, 0x00, 0x80, 0x00, 0x80, 0x00, 0x80, 0x00,
        0x80,
    ];
    let src2 = [
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF,
    ];
    mem.write_slice(&src1, GuestAddress(ALIGNED_ADDR)).unwrap();
    mem.write_slice(&src2, GuestAddress(ALIGNED_ADDR2)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_paddsw_saturate_negative_min_large() {
    // Test saturation with large negative overflow
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x0b, 0x66, 0x0f, 0xed, 0xc1, 0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    // -20000 + -20000 = -40000, should saturate to -32768
    let src1 = [
        0xE0, 0xB1, 0xE0, 0xB1, 0xE0, 0xB1, 0xE0, 0xB1, 0xE0, 0xB1, 0xE0, 0xB1, 0xE0, 0xB1, 0xE0,
        0xB1,
    ];
    let src2 = [
        0xE0, 0xB1, 0xE0, 0xB1, 0xE0, 0xB1, 0xE0, 0xB1, 0xE0, 0xB1, 0xE0, 0xB1, 0xE0, 0xB1, 0xE0,
        0xB1,
    ];
    mem.write_slice(&src1, GuestAddress(ALIGNED_ADDR)).unwrap();
    mem.write_slice(&src2, GuestAddress(ALIGNED_ADDR2)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_paddsw_mixed_positive_negative() {
    // Test mixed positive and negative values
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x0b, 0x66, 0x0f, 0xed, 0xc1, 0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    // 1000 + (-500) = 500
    let src1 = [
        0xE8, 0x03, 0xE8, 0x03, 0xE8, 0x03, 0xE8, 0x03, 0xE8, 0x03, 0xE8, 0x03, 0xE8, 0x03, 0xE8,
        0x03,
    ];
    let src2 = [
        0x0C, 0xFE, 0x0C, 0xFE, 0x0C, 0xFE, 0x0C, 0xFE, 0x0C, 0xFE, 0x0C, 0xFE, 0x0C, 0xFE, 0x0C,
        0xFE,
    ];
    mem.write_slice(&src1, GuestAddress(ALIGNED_ADDR)).unwrap();
    mem.write_slice(&src2, GuestAddress(ALIGNED_ADDR2)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_paddsw_various_saturation_cases() {
    // Test different saturation cases in same register
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x0b, 0x66, 0x0f, 0xed, 0xc1, 0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let src1 = [
        0xFF, 0x7F, 0x00, 0x80, 0x00, 0x40, 0x00, 0xC0, 0x00, 0x00, 0x01, 0x00, 0xFF, 0xFF, 0xFE,
        0x7F,
    ];
    let src2 = [
        0x01, 0x00, 0xFF, 0xFF, 0x00, 0x40, 0x00, 0xC0, 0x00, 0x00, 0xFF, 0xFF, 0x01, 0x00, 0x01,
        0x00,
    ];
    mem.write_slice(&src1, GuestAddress(ALIGNED_ADDR)).unwrap();
    mem.write_slice(&src2, GuestAddress(ALIGNED_ADDR2)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_paddsw_memory_operand() {
    // Test PADDSW with memory operand
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, // MOVDQA XMM0, [RAX]
        0x66, 0x0f, 0xed, 0x03, // PADDSW XMM0, [RBX]
        0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let src1 = [
        0xFF, 0x7F, 0xFF, 0x7F, 0xFF, 0x7F, 0xFF, 0x7F, 0xFF, 0x7F, 0xFF, 0x7F, 0xFF, 0x7F, 0xFF,
        0x7F,
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
fn test_paddsw_xmm_self() {
    // Test PADDSW with same register as source and destination
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, // MOVDQA XMM0, [RAX]
        0x66, 0x0f, 0xed, 0xc0, // PADDSW XMM0, XMM0
        0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let src = [
        0x00, 0x40, 0x00, 0x40, 0x00, 0x40, 0x00, 0x40, 0x00, 0x40, 0x00, 0x40, 0x00, 0x40, 0x00,
        0x40,
    ];
    mem.write_slice(&src, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_paddsw_alternating_pattern() {
    // Test with alternating positive/negative pattern
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x0b, 0x66, 0x0f, 0xed, 0xc1, 0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let src1 = [
        0xFF, 0x7F, 0x00, 0x80, 0xFF, 0x7F, 0x00, 0x80, 0xFF, 0x7F, 0x00, 0x80, 0xFF, 0x7F, 0x00,
        0x80,
    ];
    let src2 = [
        0xFF, 0x7F, 0x00, 0x80, 0xFF, 0x7F, 0x00, 0x80, 0xFF, 0x7F, 0x00, 0x80, 0xFF, 0x7F, 0x00,
        0x80,
    ];
    mem.write_slice(&src1, GuestAddress(ALIGNED_ADDR)).unwrap();
    mem.write_slice(&src2, GuestAddress(ALIGNED_ADDR2)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_paddsw_incremental_values() {
    // Test with incremental values
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x0b, 0x66, 0x0f, 0xed, 0xc1, 0xf4,
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
fn test_paddsw_all_ffff() {
    // Test with all 0xFFFF words (-1 in signed interpretation)
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x0b, 0x66, 0x0f, 0xed, 0xc1, 0xf4,
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
fn test_paddsw_different_registers() {
    // Test with different XMM registers
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x28, // MOVDQA XMM5, [RAX]
        0x66, 0x0f, 0x6f, 0x33, // MOVDQA XMM6, [RBX]
        0x66, 0x0f, 0xed, 0xee, // PADDSW XMM5, XMM6
        0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let src1 = [
        0xFF, 0x7F, 0xFF, 0x7F, 0xFF, 0x7F, 0xFF, 0x7F, 0xFF, 0x7F, 0xFF, 0x7F, 0xFF, 0x7F, 0xFF,
        0x7F,
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
fn test_paddsw_sequential_operations() {
    // Test multiple sequential PADDSW operations
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, // MOVDQA XMM0, [RAX]
        0x66, 0x0f, 0x6f, 0x0b, // MOVDQA XMM1, [RBX]
        0x66, 0x0f, 0xed, 0xc1, // PADDSW XMM0, XMM1
        0x66, 0x0f, 0xed, 0xc1, // PADDSW XMM0, XMM1
        0x66, 0x0f, 0xed, 0xc1, // PADDSW XMM0, XMM1
        0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let src1 = [
        0x00, 0x10, 0x00, 0x10, 0x00, 0x10, 0x00, 0x10, 0x00, 0x10, 0x00, 0x10, 0x00, 0x10, 0x00,
        0x10,
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
fn test_paddsw_near_saturation_positive() {
    // Test values very close to positive saturation limit
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x0b, 0x66, 0x0f, 0xed, 0xc1, 0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let src1 = [
        0xFE, 0x7F, 0xFD, 0x7F, 0xFC, 0x7F, 0xFB, 0x7F, 0xFA, 0x7F, 0xF9, 0x7F, 0xF8, 0x7F, 0xF7,
        0x7F,
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
fn test_paddsw_near_saturation_negative() {
    // Test values very close to negative saturation limit
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x0b, 0x66, 0x0f, 0xed, 0xc1, 0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let src1 = [
        0x01, 0x80, 0x02, 0x80, 0x03, 0x80, 0x04, 0x80, 0x05, 0x80, 0x06, 0x80, 0x07, 0x80, 0x08,
        0x80,
    ];
    let src2 = [
        0xFF, 0xFF, 0xFE, 0xFF, 0xFD, 0xFF, 0xFC, 0xFF, 0xFB, 0xFF, 0xFA, 0xFF, 0xF9, 0xFF, 0xF8,
        0xFF,
    ];
    mem.write_slice(&src1, GuestAddress(ALIGNED_ADDR)).unwrap();
    mem.write_slice(&src2, GuestAddress(ALIGNED_ADDR2)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_paddsw_boundary_values() {
    // Test boundary values near saturation thresholds
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x0b, 0x66, 0x0f, 0xed, 0xc1, 0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let src1 = [
        0xFE, 0x7F, 0x00, 0x80, 0x00, 0x40, 0x00, 0xC0, 0x01, 0x00, 0xFF, 0xFF, 0x00, 0x00, 0xFF,
        0x00,
    ];
    let src2 = [
        0x01, 0x00, 0xFF, 0xFF, 0x00, 0x3F, 0x00, 0xC0, 0x7E, 0x7F, 0x01, 0x80, 0x00, 0x00, 0x01,
        0x00,
    ];
    mem.write_slice(&src1, GuestAddress(ALIGNED_ADDR)).unwrap();
    mem.write_slice(&src2, GuestAddress(ALIGNED_ADDR2)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Known-answer value tests (register-to-register via set_xmm/get_xmm)
//
// Signed saturating add. Each lane clamps to [-128,127] (byte) / [-32768,32767]
// (word). Vectors chosen to hit positive and negative saturation. Computed by
// hand from x86 PADDSB/PADDSW semantics.
// ============================================================================

#[test]
fn kat_paddsb_value() {
    // PADDSB XMM0, XMM1 (66 0F EC C1)
    let code = [0x66, 0x0f, 0xec, 0xc1, 0xf4];
    let (mut vcpu, mem) = setup_vm(&code, None);
    set_xmm(&mem, &mut vcpu, 0, 0x7f7f80800102fe007f01ff80fe7f0100);
    set_xmm(&mem, &mut vcpu, 1, 0x010180800102020101ff01800201ff00);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        get_xmm(&regs, 0),
        0x7f7f8080020400017f000080007f0000,
        "PADDSB got {:032x}",
        get_xmm(&regs, 0)
    );
}

#[test]
fn kat_paddsw_value() {
    // PADDSW XMM0, XMM1 (66 0F ED C1)
    let code = [0x66, 0x0f, 0xed, 0xc1, 0xf4];
    let (mut vcpu, mem) = setup_vm(&code, None);
    set_xmm(&mem, &mut vcpu, 0, 0x7fff80000001fffe7fff8000ffff0001);
    set_xmm(&mem, &mut vcpu, 1, 0x0001800000027fff0001ffff00018000);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        get_xmm(&regs, 0),
        0x7fff800000037ffd7fff800000008001,
        "PADDSW got {:032x}",
        get_xmm(&regs, 0)
    );
}

#[test]
fn kat_paddsb_positive_saturation() {
    // 0x7F + 0x7F = 254 saturates to +127 (0x7F) in every lane.
    let code = [0x66, 0x0f, 0xec, 0xc1, 0xf4];
    let (mut vcpu, mem) = setup_vm(&code, None);
    set_xmm(&mem, &mut vcpu, 0, 0x7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f);
    set_xmm(&mem, &mut vcpu, 1, 0x7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(get_xmm(&regs, 0), 0x7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f);
}

#[test]
fn kat_paddsw_negative_saturation() {
    // 0x8000 + 0x8000 = -65536 saturates to -32768 (0x8000) in each word.
    let code = [0x66, 0x0f, 0xed, 0xc1, 0xf4];
    let (mut vcpu, mem) = setup_vm(&code, None);
    set_xmm(&mem, &mut vcpu, 0, 0x80008000800080008000800080008000);
    set_xmm(&mem, &mut vcpu, 1, 0x80008000800080008000800080008000);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(get_xmm(&regs, 0), 0x80008000800080008000800080008000);
}
