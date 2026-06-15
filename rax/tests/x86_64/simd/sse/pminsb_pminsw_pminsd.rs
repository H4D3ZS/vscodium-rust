use crate::common::*;
use vm_memory::{Bytes, GuestAddress};

// PMINSB/PMINSW/PMINSD - Minimum of Packed Signed Integers
//
// Performs SIMD compare of packed signed integers and returns minimum values.
//
// PMINSB: Minimum of 16 packed signed byte integers (SSE4.1)
// PMINSW: Minimum of 8 packed signed word integers (SSE2/SSE)
// PMINSD: Minimum of 4 packed signed dword integers (SSE4.1)
//
// Opcodes:
// 66 0F 38 38 /r      PMINSB xmm1, xmm2/m128   - Min of packed signed bytes
// 66 0F EA /r         PMINSW xmm1, xmm2/m128   - Min of packed signed words
// 66 0F 38 39 /r      PMINSD xmm1, xmm2/m128   - Min of packed signed dwords

const ALIGNED_ADDR: u64 = 0x3000;
const ALIGNED_ADDR2: u64 = 0x3100;

// ============================================================================
// PMINSB Tests - 16x Signed Byte Minimum
// ============================================================================

#[test]
fn test_pminsb_all_zeros() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, // MOVDQA XMM0, [RAX]
        0x66, 0x0f, 0x6f, 0x0b, // MOVDQA XMM1, [RBX]
        0x66, 0x0f, 0x38, 0x38, 0xc1, // PMINSB XMM0, XMM1
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
fn test_pminsb_positive_negative() {
    // Test positive vs negative values
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x0b, 0x66, 0x0f, 0x38, 0x38, 0xc1, 0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(
        &[
            0x7F, 0x7F, 0x7F, 0x7F, 0x7F, 0x7F, 0x7F, 0x7F, 0x7F, 0x7F, 0x7F, 0x7F, 0x7F, 0x7F,
            0x7F, 0x7F,
        ],
        GuestAddress(ALIGNED_ADDR),
    )
    .unwrap(); // +127
    mem.write_slice(
        &[
            0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80,
            0x80, 0x80,
        ],
        GuestAddress(ALIGNED_ADDR2),
    )
    .unwrap(); // -128
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pminsb_mixed_values() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x0b, 0x66, 0x0f, 0x38, 0x38, 0xc1, 0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data1 = [
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F,
        0x10,
    ];
    let data2 = [
        0x10, 0x0F, 0x0E, 0x0D, 0x0C, 0x0B, 0x0A, 0x09, 0x08, 0x07, 0x06, 0x05, 0x04, 0x03, 0x02,
        0x01,
    ];
    mem.write_slice(&data1, GuestAddress(ALIGNED_ADDR)).unwrap();
    mem.write_slice(&data2, GuestAddress(ALIGNED_ADDR2))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pminsb_negative_values() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x0b, 0x66, 0x0f, 0x38, 0x38, 0xc1, 0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    // -1 vs -2 (0xFF vs 0xFE in signed byte)
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
            0xFE, 0xFE, 0xFE, 0xFE, 0xFE, 0xFE, 0xFE, 0xFE, 0xFE, 0xFE, 0xFE, 0xFE, 0xFE, 0xFE,
            0xFE, 0xFE,
        ],
        GuestAddress(ALIGNED_ADDR2),
    )
    .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pminsb_extremes() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x0b, 0x66, 0x0f, 0x38, 0x38, 0xc1, 0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(
        &[
            0x7F, 0x7F, 0x7F, 0x7F, 0x7F, 0x7F, 0x7F, 0x7F, 0x7F, 0x7F, 0x7F, 0x7F, 0x7F, 0x7F,
            0x7F, 0x7F,
        ],
        GuestAddress(ALIGNED_ADDR),
    )
    .unwrap(); // Max positive (127)
    mem.write_slice(
        &[
            0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01,
            0x01, 0x01,
        ],
        GuestAddress(ALIGNED_ADDR2),
    )
    .unwrap(); // 1
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pminsb_xmm2_xmm3() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x10, // MOVDQA XMM2, [RAX]
        0x66, 0x0f, 0x6f, 0x1b, // MOVDQA XMM3, [RBX]
        0x66, 0x0f, 0x38, 0x38, 0xd3, // PMINSB XMM2, XMM3
        0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(
        &[
            0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55,
            0x55, 0x55,
        ],
        GuestAddress(ALIGNED_ADDR),
    )
    .unwrap();
    mem.write_slice(
        &[
            0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA,
            0xAA, 0xAA,
        ],
        GuestAddress(ALIGNED_ADDR2),
    )
    .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pminsb_from_memory() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, // MOVDQA XMM0, [RAX]
        0x66, 0x0f, 0x38, 0x38, 0x03, // PMINSB XMM0, [RBX]
        0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(
        &[
            0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11,
            0x11, 0x11,
        ],
        GuestAddress(ALIGNED_ADDR),
    )
    .unwrap();
    mem.write_slice(
        &[
            0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22,
            0x22, 0x22,
        ],
        GuestAddress(ALIGNED_ADDR2),
    )
    .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pminsb_alternating_sign() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x0b, 0x66, 0x0f, 0x38, 0x38, 0xc1, 0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    // Alternating positive and negative
    let data1 = [
        0x01, 0xFF, 0x01, 0xFF, 0x01, 0xFF, 0x01, 0xFF, 0x01, 0xFF, 0x01, 0xFF, 0x01, 0xFF, 0x01,
        0xFF,
    ];
    let data2 = [
        0xFF, 0x01, 0xFF, 0x01, 0xFF, 0x01, 0xFF, 0x01, 0xFF, 0x01, 0xFF, 0x01, 0xFF, 0x01, 0xFF,
        0x01,
    ];
    mem.write_slice(&data1, GuestAddress(ALIGNED_ADDR)).unwrap();
    mem.write_slice(&data2, GuestAddress(ALIGNED_ADDR2))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// PMINSW Tests - 8x Signed Word Minimum
// ============================================================================

#[test]
fn test_pminsw_all_zeros() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x0b, 0x66, 0x0f, 0xea,
        0xc1, // PMINSW XMM0, XMM1
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
fn test_pminsw_positive_negative() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x0b, 0x66, 0x0f, 0xea, 0xc1, 0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    // +32767 vs -32768
    mem.write_slice(
        &[
            0xFF, 0x7F, 0xFF, 0x7F, 0xFF, 0x7F, 0xFF, 0x7F, 0xFF, 0x7F, 0xFF, 0x7F, 0xFF, 0x7F,
            0xFF, 0x7F,
        ],
        GuestAddress(ALIGNED_ADDR),
    )
    .unwrap();
    mem.write_slice(
        &[
            0x00, 0x80, 0x00, 0x80, 0x00, 0x80, 0x00, 0x80, 0x00, 0x80, 0x00, 0x80, 0x00, 0x80,
            0x00, 0x80,
        ],
        GuestAddress(ALIGNED_ADDR2),
    )
    .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pminsw_mixed_values() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x0b, 0x66, 0x0f, 0xea, 0xc1, 0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data1 = [
        0x00, 0x01, 0x00, 0x02, 0x00, 0x03, 0x00, 0x04, 0x00, 0x05, 0x00, 0x06, 0x00, 0x07, 0x00,
        0x08,
    ];
    let data2 = [
        0x00, 0x08, 0x00, 0x07, 0x00, 0x06, 0x00, 0x05, 0x00, 0x04, 0x00, 0x03, 0x00, 0x02, 0x00,
        0x01,
    ];
    mem.write_slice(&data1, GuestAddress(ALIGNED_ADDR)).unwrap();
    mem.write_slice(&data2, GuestAddress(ALIGNED_ADDR2))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pminsw_negative_values() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x0b, 0x66, 0x0f, 0xea, 0xc1, 0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    // -1 vs -2
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
            0xFE, 0xFF, 0xFE, 0xFF, 0xFE, 0xFF, 0xFE, 0xFF, 0xFE, 0xFF, 0xFE, 0xFF, 0xFE, 0xFF,
            0xFE, 0xFF,
        ],
        GuestAddress(ALIGNED_ADDR2),
    )
    .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pminsw_xmm4_xmm5() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x20, // MOVDQA XMM4, [RAX]
        0x66, 0x0f, 0x6f, 0x2b, // MOVDQA XMM5, [RBX]
        0x66, 0x0f, 0xea, 0xe5, // PMINSW XMM4, XMM5
        0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(
        &[
            0x11, 0x11, 0x22, 0x22, 0x33, 0x33, 0x44, 0x44, 0x55, 0x55, 0x66, 0x66, 0x77, 0x77,
            0x88, 0x88,
        ],
        GuestAddress(ALIGNED_ADDR),
    )
    .unwrap();
    mem.write_slice(
        &[
            0xEE, 0xEE, 0xDD, 0xDD, 0xCC, 0xCC, 0xBB, 0xBB, 0xAA, 0xAA, 0x99, 0x99, 0x88, 0x88,
            0x77, 0x77,
        ],
        GuestAddress(ALIGNED_ADDR2),
    )
    .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pminsw_from_memory() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0xea, 0x03, // PMINSW XMM0, [RBX]
        0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(
        &[
            0x00, 0x10, 0x00, 0x20, 0x00, 0x30, 0x00, 0x40, 0x00, 0x50, 0x00, 0x60, 0x00, 0x70,
            0x00, 0x80,
        ],
        GuestAddress(ALIGNED_ADDR),
    )
    .unwrap();
    mem.write_slice(
        &[
            0xFF, 0x0F, 0xFF, 0x1F, 0xFF, 0x2F, 0xFF, 0x3F, 0xFF, 0x4F, 0xFF, 0x5F, 0xFF, 0x6F,
            0xFF, 0x7F,
        ],
        GuestAddress(ALIGNED_ADDR2),
    )
    .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pminsw_extremes() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x0b, 0x66, 0x0f, 0xea, 0xc1, 0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(
        &[
            0x00, 0x80, 0x00, 0x80, 0x00, 0x80, 0x00, 0x80, 0x00, 0x80, 0x00, 0x80, 0x00, 0x80,
            0x00, 0x80,
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

// ============================================================================
// PMINSD Tests - 4x Signed Dword Minimum
// ============================================================================

#[test]
fn test_pminsd_all_zeros() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x0b, 0x66, 0x0f, 0x38, 0x39,
        0xc1, // PMINSD XMM0, XMM1
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
fn test_pminsd_positive_negative() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x0b, 0x66, 0x0f, 0x38, 0x39, 0xc1, 0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    // +2147483647 vs -2147483648
    mem.write_slice(
        &[
            0xFF, 0xFF, 0xFF, 0x7F, 0xFF, 0xFF, 0xFF, 0x7F, 0xFF, 0xFF, 0xFF, 0x7F, 0xFF, 0xFF,
            0xFF, 0x7F,
        ],
        GuestAddress(ALIGNED_ADDR),
    )
    .unwrap();
    mem.write_slice(
        &[
            0x00, 0x00, 0x00, 0x80, 0x00, 0x00, 0x00, 0x80, 0x00, 0x00, 0x00, 0x80, 0x00, 0x00,
            0x00, 0x80,
        ],
        GuestAddress(ALIGNED_ADDR2),
    )
    .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pminsd_mixed_values() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x0b, 0x66, 0x0f, 0x38, 0x39, 0xc1, 0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data1 = [
        0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00,
        0x04,
    ];
    let data2 = [
        0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00,
        0x01,
    ];
    mem.write_slice(&data1, GuestAddress(ALIGNED_ADDR)).unwrap();
    mem.write_slice(&data2, GuestAddress(ALIGNED_ADDR2))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pminsd_negative_values() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x0b, 0x66, 0x0f, 0x38, 0x39, 0xc1, 0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    // -1 vs -2
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
            0xFE, 0xFF, 0xFF, 0xFF, 0xFE, 0xFF, 0xFF, 0xFF, 0xFE, 0xFF, 0xFF, 0xFF, 0xFE, 0xFF,
            0xFF, 0xFF,
        ],
        GuestAddress(ALIGNED_ADDR2),
    )
    .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pminsd_xmm1_xmm2() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x08, // MOVDQA XMM1, [RAX]
        0x66, 0x0f, 0x6f, 0x13, // MOVDQA XMM2, [RBX]
        0x66, 0x0f, 0x38, 0x39, 0xca, // PMINSD XMM1, XMM2
        0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(
        &[
            0x11, 0x11, 0x11, 0x11, 0x22, 0x22, 0x22, 0x22, 0x33, 0x33, 0x33, 0x33, 0x44, 0x44,
            0x44, 0x44,
        ],
        GuestAddress(ALIGNED_ADDR),
    )
    .unwrap();
    mem.write_slice(
        &[
            0xEE, 0xEE, 0xEE, 0xEE, 0xDD, 0xDD, 0xDD, 0xDD, 0xCC, 0xCC, 0xCC, 0xCC, 0xBB, 0xBB,
            0xBB, 0xBB,
        ],
        GuestAddress(ALIGNED_ADDR2),
    )
    .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pminsd_from_memory() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x38, 0x39, 0x03, // PMINSD XMM0, [RBX]
        0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(
        &[
            0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00,
            0x04, 0x00,
        ],
        GuestAddress(ALIGNED_ADDR),
    )
    .unwrap();
    mem.write_slice(
        &[
            0xFF, 0xFF, 0x00, 0x00, 0xFF, 0xFF, 0x01, 0x00, 0xFF, 0xFF, 0x02, 0x00, 0xFF, 0xFF,
            0x03, 0x00,
        ],
        GuestAddress(ALIGNED_ADDR2),
    )
    .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pminsd_large_values() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x0b, 0x66, 0x0f, 0x38, 0x39, 0xc1, 0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data1 = [
        0x00, 0x00, 0x00, 0x40, 0x00, 0x00, 0x00, 0x40, 0x00, 0x00, 0x00, 0x40, 0x00, 0x00, 0x00,
        0x40,
    ];
    let data2 = [
        0x00, 0x00, 0x00, 0xC0, 0x00, 0x00, 0x00, 0xC0, 0x00, 0x00, 0x00, 0xC0, 0x00, 0x00, 0x00,
        0xC0,
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
fn test_pminsb_xmm8_xmm9() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x44, 0x0f, 0x6f, 0x00, // MOVDQA XMM8, [RAX]
        0x66, 0x44, 0x0f, 0x6f, 0x0b, // MOVDQA XMM9, [RBX]
        0x66, 0x45, 0x0f, 0x38, 0x38, 0xc1, // PMINSB XMM8, XMM9
        0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(
        &[
            0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01,
            0x01, 0x01,
        ],
        GuestAddress(ALIGNED_ADDR),
    )
    .unwrap();
    mem.write_slice(
        &[
            0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02,
            0x02, 0x02,
        ],
        GuestAddress(ALIGNED_ADDR2),
    )
    .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pminsw_xmm10_xmm11() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x44, 0x0f, 0x6f, 0x10, // MOVDQA XMM10, [RAX]
        0x66, 0x44, 0x0f, 0x6f, 0x1b, // MOVDQA XMM11, [RBX]
        0x66, 0x45, 0x0f, 0xea, 0xd3, // PMINSW XMM10, XMM11
        0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(
        &[
            0x01, 0x00, 0x02, 0x00, 0x03, 0x00, 0x04, 0x00, 0x05, 0x00, 0x06, 0x00, 0x07, 0x00,
            0x08, 0x00,
        ],
        GuestAddress(ALIGNED_ADDR),
    )
    .unwrap();
    mem.write_slice(
        &[
            0xFF, 0xFF, 0xFE, 0xFF, 0xFD, 0xFF, 0xFC, 0xFF, 0xFB, 0xFF, 0xFA, 0xFF, 0xF9, 0xFF,
            0xF8, 0xFF,
        ],
        GuestAddress(ALIGNED_ADDR2),
    )
    .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pminsd_xmm12_xmm13() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x44, 0x0f, 0x6f, 0x20, // MOVDQA XMM12, [RAX]
        0x66, 0x44, 0x0f, 0x6f, 0x2b, // MOVDQA XMM13, [RBX]
        0x66, 0x45, 0x0f, 0x38, 0x39, 0xe5, // PMINSD XMM12, XMM13
        0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(
        &[
            0x01, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00, 0x04, 0x00,
            0x00, 0x00,
        ],
        GuestAddress(ALIGNED_ADDR),
    )
    .unwrap();
    mem.write_slice(
        &[
            0xFF, 0xFF, 0xFF, 0xFF, 0xFE, 0xFF, 0xFF, 0xFF, 0xFD, 0xFF, 0xFF, 0xFF, 0xFC, 0xFF,
            0xFF, 0xFF,
        ],
        GuestAddress(ALIGNED_ADDR2),
    )
    .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Combined/Sequence Tests
// ============================================================================

#[test]
fn test_pminsb_pminsw_sequence() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, // MOVDQA XMM0, [RAX]
        0x66, 0x0f, 0x6f, 0x08, // MOVDQA XMM1, [RAX]
        0x66, 0x0f, 0x38, 0x38, 0xc1, // PMINSB XMM0, XMM1
        0x66, 0x0f, 0xea, 0xc1, // PMINSW XMM0, XMM1
        0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(
        &[
            0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01,
            0x01, 0x01,
        ],
        GuestAddress(ALIGNED_ADDR),
    )
    .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pminsd_sequence() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, // MOVDQA XMM0, [RAX]
        0x66, 0x0f, 0x6f, 0x08, // MOVDQA XMM1, [RAX]
        0x66, 0x0f, 0x38, 0x39, 0xc1, // PMINSD XMM0, XMM1
        0x66, 0x0f, 0x38, 0x39, 0xc1, // PMINSD XMM0, XMM1
        0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(
        &[
            0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00,
            0x00, 0x00,
        ],
        GuestAddress(ALIGNED_ADDR),
    )
    .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_all_pmins_sequence() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, // MOVDQA XMM0, [RAX]
        0x66, 0x0f, 0x6f, 0x08, // MOVDQA XMM1, [RAX]
        0x66, 0x0f, 0x38, 0x38, 0xc1, // PMINSB XMM0, XMM1
        0x66, 0x0f, 0xea, 0xc1, // PMINSW XMM0, XMM1
        0x66, 0x0f, 0x38, 0x39, 0xc1, // PMINSD XMM0, XMM1
        0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(
        &[
            0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01,
            0x01, 0x01,
        ],
        GuestAddress(ALIGNED_ADDR),
    )
    .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pminsb_same_register() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, // MOVDQA XMM0, [RAX]
        0x66, 0x0f, 0x38, 0x38, 0xc0, // PMINSB XMM0, XMM0
        0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(
        &[
            0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42,
            0x42, 0x42,
        ],
        GuestAddress(ALIGNED_ADDR),
    )
    .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pminsw_same_register() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, // MOVDQA XMM0, [RAX]
        0x66, 0x0f, 0xea, 0xc0, // PMINSW XMM0, XMM0
        0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(
        &[
            0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42,
            0x42, 0x42,
        ],
        GuestAddress(ALIGNED_ADDR),
    )
    .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pminsd_same_register() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, // MOVDQA XMM0, [RAX]
        0x66, 0x0f, 0x38, 0x39, 0xc0, // PMINSD XMM0, XMM0
        0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(
        &[
            0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42,
            0x42, 0x42,
        ],
        GuestAddress(ALIGNED_ADDR),
    )
    .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}
