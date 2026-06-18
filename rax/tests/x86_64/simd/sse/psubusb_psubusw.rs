use crate::common::*;
use vm_memory::{Bytes, GuestAddress};

// PSUBUSB/PSUBUSW - Subtract Packed Unsigned Integers with Unsigned Saturation (SSE2)
//
// Performs SIMD subtract of packed unsigned integers from source and destination operands.
// Stores packed integer results in destination. Underflow handled with unsigned saturation to 0.
//
// PSUBUSB: Subtract 16 packed unsigned byte integers (8-bit each) with saturation to 0
// PSUBUSW: Subtract 8 packed unsigned word integers (16-bit each) with saturation to 0
//
// Saturation behavior:
// - PSUBUSB: Result < 0 saturates to 0
// - PSUBUSW: Result < 0 saturates to 0
//
// Opcodes (SSE2 - 128-bit XMM):
// 66 0F D8 /r      PSUBUSB xmm1, xmm2/m128   - Subtract packed unsigned bytes with saturation
// 66 0F D9 /r      PSUBUSW xmm1, xmm2/m128   - Subtract packed unsigned words with saturation

const ALIGNED_ADDR: u64 = 0x3000;
const ALIGNED_ADDR2: u64 = 0x3100;

// ============================================================================
// PSUBUSB Tests - 16x Unsigned Byte Subtraction with Saturation
// ============================================================================

#[test]
fn test_psubusb_all_zeros() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, // MOVDQA XMM0, [RAX]
        0x66, 0x0f, 0x6f, 0x0b, // MOVDQA XMM1, [RBX]
        0x66, 0x0f, 0xd8, 0xc1, // PSUBUSB XMM0, XMM1
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
fn test_psubusb_no_saturation() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x0b, 0x66, 0x0f, 0xd8, 0xc1, 0xf4,
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
fn test_psubusb_saturate_to_zero() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x0b, 0x66, 0x0f, 0xd8, 0xc1, 0xf4,
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
fn test_psubusb_exact_zero() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x0b, 0x66, 0x0f, 0xd8, 0xc1, 0xf4,
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
            0x50, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50,
            0x50, 0x50,
        ],
        GuestAddress(ALIGNED_ADDR2),
    )
    .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_psubusb_max_values() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x0b, 0x66, 0x0f, 0xd8, 0xc1, 0xf4,
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
fn test_psubusb_various_saturation_cases() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x0b, 0x66, 0x0f, 0xd8, 0xc1, 0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let src1 = [
        0xFF, 0x00, 0xC8, 0x80, 0x00, 0x01, 0xFE, 0x7F, 0xAA, 0x55, 0xFF, 0xFF, 0x80, 0x90, 0xA0,
        0x10,
    ];
    let src2 = [
        0x01, 0x00, 0xC8, 0x81, 0x01, 0xFF, 0x01, 0x80, 0x56, 0xAB, 0x01, 0x10, 0x90, 0x30, 0xE0,
        0x70,
    ];
    mem.write_slice(&src1, GuestAddress(ALIGNED_ADDR)).unwrap();
    mem.write_slice(&src2, GuestAddress(ALIGNED_ADDR2)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_psubusb_boundary_values() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x0b, 0x66, 0x0f, 0xd8, 0xc1, 0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let src1 = [
        0x01, 0x00, 0x02, 0x01, 0x80, 0x7F, 0xC0, 0xBF, 0x01, 0x02, 0xFE, 0xFD, 0xAA, 0x55, 0x00,
        0xFF,
    ];
    let src2 = [
        0x01, 0x01, 0x02, 0x02, 0x7F, 0x80, 0x3F, 0x40, 0x02, 0x03, 0xFF, 0xFE, 0x55, 0xAA, 0xFF,
        0x00,
    ];
    mem.write_slice(&src1, GuestAddress(ALIGNED_ADDR)).unwrap();
    mem.write_slice(&src2, GuestAddress(ALIGNED_ADDR2)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_psubusb_memory_operand() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0xd8, 0x03, 0xf4]);

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
fn test_psubusb_xmm_self() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0xd8, 0xc0, 0xf4]);

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
fn test_psubusb_alternating_pattern() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x0b, 0x66, 0x0f, 0xd8, 0xc1, 0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let src1 = [
        0xFF, 0x00, 0xFF, 0x00, 0xFF, 0x00, 0xFF, 0x00, 0xFF, 0x00, 0xFF, 0x00, 0xFF, 0x00, 0xFF,
        0x00,
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
fn test_psubusb_incremental_values() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x0b, 0x66, 0x0f, 0xd8, 0xc1, 0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let src1 = [
        0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2A, 0x2B, 0x2C, 0x2D, 0x2E,
        0x2F,
    ];
    let src2 = [
        0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E,
        0x0F,
    ];
    mem.write_slice(&src1, GuestAddress(ALIGNED_ADDR)).unwrap();
    mem.write_slice(&src2, GuestAddress(ALIGNED_ADDR2)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_psubusb_different_registers() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x10, 0x66, 0x0f, 0x6f, 0x1b, 0x66, 0x0f, 0xd8, 0xd3, 0xf4,
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
fn test_psubusb_sequential_operations() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x0b, 0x66, 0x0f, 0xd8, 0xc1, 0x66, 0x0f, 0xd8,
        0xc1, 0x66, 0x0f, 0xd8, 0xc1, 0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(
        &[
            0xA0, 0xA0, 0xA0, 0xA0, 0xA0, 0xA0, 0xA0, 0xA0, 0xA0, 0xA0, 0xA0, 0xA0, 0xA0, 0xA0,
            0xA0, 0xA0,
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
fn test_psubusb_near_zero() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x0b, 0x66, 0x0f, 0xd8, 0xc1, 0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let src1 = [
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F,
        0x10,
    ];
    let src2 = [
        0x01, 0x02, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x10,
        0x11,
    ];
    mem.write_slice(&src1, GuestAddress(ALIGNED_ADDR)).unwrap();
    mem.write_slice(&src2, GuestAddress(ALIGNED_ADDR2)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_psubusb_halfway_values() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x0b, 0x66, 0x0f, 0xd8, 0xc1, 0xf4,
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
            0x40, 0x40, 0x40, 0x40, 0x40, 0x40, 0x40, 0x40, 0x40, 0x40, 0x40, 0x40, 0x40, 0x40,
            0x40, 0x40,
        ],
        GuestAddress(ALIGNED_ADDR2),
    )
    .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_psubusb_mixed_small_large() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x0b, 0x66, 0x0f, 0xd8, 0xc1, 0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let src1 = [
        0x01, 0xFF, 0x10, 0xF0, 0x20, 0xE0, 0x30, 0xD0, 0x40, 0xC0, 0x50, 0xB0, 0x60, 0xA0, 0x70,
        0x90,
    ];
    let src2 = [
        0x02, 0x00, 0x20, 0x10, 0x30, 0x20, 0x40, 0x30, 0x50, 0x40, 0x60, 0x50, 0x70, 0x60, 0x80,
        0x70,
    ];
    mem.write_slice(&src1, GuestAddress(ALIGNED_ADDR)).unwrap();
    mem.write_slice(&src2, GuestAddress(ALIGNED_ADDR2)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_psubusb_powers_of_two() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x0b, 0x66, 0x0f, 0xd8, 0xc1, 0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let src1 = [
        0x80, 0x40, 0x20, 0x10, 0x08, 0x04, 0x02, 0x01, 0x80, 0x40, 0x20, 0x10, 0x08, 0x04, 0x02,
        0x01,
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
// PSUBUSW Tests - 8x Unsigned Word Subtraction with Saturation
// ============================================================================

#[test]
fn test_psubusw_all_zeros() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x0b, 0x66, 0x0f, 0xd9, 0xc1, 0xf4,
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
fn test_psubusw_no_saturation() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x0b, 0x66, 0x0f, 0xd9, 0xc1, 0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let src1 = [
        0x00, 0x50, 0x00, 0x50, 0x00, 0x50, 0x00, 0x50, 0x00, 0x50, 0x00, 0x50, 0x00, 0x50, 0x00,
        0x50,
    ];
    let src2 = [
        0x00, 0x30, 0x00, 0x30, 0x00, 0x30, 0x00, 0x30, 0x00, 0x30, 0x00, 0x30, 0x00, 0x30, 0x00,
        0x30,
    ];
    mem.write_slice(&src1, GuestAddress(ALIGNED_ADDR)).unwrap();
    mem.write_slice(&src2, GuestAddress(ALIGNED_ADDR2)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_psubusw_saturate_to_zero() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x0b, 0x66, 0x0f, 0xd9, 0xc1, 0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let src1 = [
        0x00, 0x10, 0x00, 0x10, 0x00, 0x10, 0x00, 0x10, 0x00, 0x10, 0x00, 0x10, 0x00, 0x10, 0x00,
        0x10,
    ];
    let src2 = [
        0x00, 0x20, 0x00, 0x20, 0x00, 0x20, 0x00, 0x20, 0x00, 0x20, 0x00, 0x20, 0x00, 0x20, 0x00,
        0x20,
    ];
    mem.write_slice(&src1, GuestAddress(ALIGNED_ADDR)).unwrap();
    mem.write_slice(&src2, GuestAddress(ALIGNED_ADDR2)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_psubusw_exact_zero() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x0b, 0x66, 0x0f, 0xd9, 0xc1, 0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let src = [
        0x00, 0x50, 0x00, 0x50, 0x00, 0x50, 0x00, 0x50, 0x00, 0x50, 0x00, 0x50, 0x00, 0x50, 0x00,
        0x50,
    ];
    mem.write_slice(&src, GuestAddress(ALIGNED_ADDR)).unwrap();
    mem.write_slice(&src, GuestAddress(ALIGNED_ADDR2)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_psubusw_max_values() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x0b, 0x66, 0x0f, 0xd9, 0xc1, 0xf4,
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
fn test_psubusw_various_saturation_cases() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x0b, 0x66, 0x0f, 0xd9, 0xc1, 0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let src1 = [
        0xFF, 0xFF, 0x00, 0x00, 0x00, 0x80, 0x00, 0x40, 0x00, 0x00, 0x01, 0x00, 0xFF, 0xFF, 0xFE,
        0xFF,
    ];
    let src2 = [
        0x01, 0x00, 0x01, 0x00, 0x01, 0x80, 0xFF, 0x40, 0x00, 0x00, 0x02, 0x00, 0x01, 0x00, 0xFF,
        0xFF,
    ];
    mem.write_slice(&src1, GuestAddress(ALIGNED_ADDR)).unwrap();
    mem.write_slice(&src2, GuestAddress(ALIGNED_ADDR2)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_psubusw_memory_operand() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0xd9, 0x03, 0xf4]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let src1 = [
        0x00, 0x10, 0x00, 0x10, 0x00, 0x10, 0x00, 0x10, 0x00, 0x10, 0x00, 0x10, 0x00, 0x10, 0x00,
        0x10,
    ];
    let src2 = [
        0x00, 0x20, 0x00, 0x20, 0x00, 0x20, 0x00, 0x20, 0x00, 0x20, 0x00, 0x20, 0x00, 0x20, 0x00,
        0x20,
    ];
    mem.write_slice(&src1, GuestAddress(ALIGNED_ADDR)).unwrap();
    mem.write_slice(&src2, GuestAddress(ALIGNED_ADDR2)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_psubusw_xmm_self() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0xd9, 0xc0, 0xf4]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let src = [
        0x00, 0x80, 0x00, 0x80, 0x00, 0x80, 0x00, 0x80, 0x00, 0x80, 0x00, 0x80, 0x00, 0x80, 0x00,
        0x80,
    ];
    mem.write_slice(&src, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_psubusw_alternating_pattern() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x0b, 0x66, 0x0f, 0xd9, 0xc1, 0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let src1 = [
        0xFF, 0xFF, 0x00, 0x00, 0xFF, 0xFF, 0x00, 0x00, 0xFF, 0xFF, 0x00, 0x00, 0xFF, 0xFF, 0x00,
        0x00,
    ];
    let src2 = [
        0x00, 0x00, 0xFF, 0xFF, 0x00, 0x00, 0xFF, 0xFF, 0x00, 0x00, 0xFF, 0xFF, 0x00, 0x00, 0xFF,
        0xFF,
    ];
    mem.write_slice(&src1, GuestAddress(ALIGNED_ADDR)).unwrap();
    mem.write_slice(&src2, GuestAddress(ALIGNED_ADDR2)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_psubusw_incremental_values() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x0b, 0x66, 0x0f, 0xd9, 0xc1, 0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let src1 = [
        0x00, 0x20, 0x00, 0x21, 0x00, 0x22, 0x00, 0x23, 0x00, 0x24, 0x00, 0x25, 0x00, 0x26, 0x00,
        0x27,
    ];
    let src2 = [
        0x00, 0x00, 0x00, 0x01, 0x00, 0x02, 0x00, 0x03, 0x00, 0x04, 0x00, 0x05, 0x00, 0x06, 0x00,
        0x07,
    ];
    mem.write_slice(&src1, GuestAddress(ALIGNED_ADDR)).unwrap();
    mem.write_slice(&src2, GuestAddress(ALIGNED_ADDR2)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_psubusw_different_registers() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x38, 0x66, 0x0f, 0x6f, 0x03, 0x66, 0x0f, 0xd9, 0xf8, 0xf4,
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
fn test_psubusw_sequential_operations() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x0b, 0x66, 0x0f, 0xd9, 0xc1, 0x66, 0x0f, 0xd9,
        0xc1, 0x66, 0x0f, 0xd9, 0xc1, 0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let src1 = [
        0x00, 0xA0, 0x00, 0xA0, 0x00, 0xA0, 0x00, 0xA0, 0x00, 0xA0, 0x00, 0xA0, 0x00, 0xA0, 0x00,
        0xA0,
    ];
    let src2 = [
        0x00, 0x30, 0x00, 0x30, 0x00, 0x30, 0x00, 0x30, 0x00, 0x30, 0x00, 0x30, 0x00, 0x30, 0x00,
        0x30,
    ];
    mem.write_slice(&src1, GuestAddress(ALIGNED_ADDR)).unwrap();
    mem.write_slice(&src2, GuestAddress(ALIGNED_ADDR2)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_psubusw_near_zero() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x0b, 0x66, 0x0f, 0xd9, 0xc1, 0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let src1 = [
        0x01, 0x00, 0x02, 0x00, 0x03, 0x00, 0x04, 0x00, 0x05, 0x00, 0x06, 0x00, 0x07, 0x00, 0x08,
        0x00,
    ];
    let src2 = [
        0x01, 0x00, 0x03, 0x00, 0x04, 0x00, 0x05, 0x00, 0x06, 0x00, 0x07, 0x00, 0x08, 0x00, 0x09,
        0x00,
    ];
    mem.write_slice(&src1, GuestAddress(ALIGNED_ADDR)).unwrap();
    mem.write_slice(&src2, GuestAddress(ALIGNED_ADDR2)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_psubusw_halfway_values() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x0b, 0x66, 0x0f, 0xd9, 0xc1, 0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let src1 = [
        0x00, 0x80, 0x00, 0x80, 0x00, 0x80, 0x00, 0x80, 0x00, 0x80, 0x00, 0x80, 0x00, 0x80, 0x00,
        0x80,
    ];
    let src2 = [
        0x00, 0x40, 0x00, 0x40, 0x00, 0x40, 0x00, 0x40, 0x00, 0x40, 0x00, 0x40, 0x00, 0x40, 0x00,
        0x40,
    ];
    mem.write_slice(&src1, GuestAddress(ALIGNED_ADDR)).unwrap();
    mem.write_slice(&src2, GuestAddress(ALIGNED_ADDR2)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_psubusw_boundary_values() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x0b, 0x66, 0x0f, 0xd9, 0xc1, 0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let src1 = [
        0x01, 0x00, 0x00, 0x00, 0x00, 0x80, 0x00, 0x40, 0x01, 0x00, 0xFF, 0xFF, 0x00, 0x00, 0xFF,
        0x00,
    ];
    let src2 = [
        0x01, 0x00, 0x01, 0x00, 0x01, 0x80, 0xFF, 0x40, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x01,
    ];
    mem.write_slice(&src1, GuestAddress(ALIGNED_ADDR)).unwrap();
    mem.write_slice(&src2, GuestAddress(ALIGNED_ADDR2)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_psubusw_mixed_small_large() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x0b, 0x66, 0x0f, 0xd9, 0xc1, 0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let src1 = [
        0x00, 0x01, 0xFF, 0xFF, 0x00, 0x10, 0x00, 0xF0, 0x00, 0x20, 0x00, 0xE0, 0x00, 0x30, 0x00,
        0xD0,
    ];
    let src2 = [
        0x01, 0x01, 0x00, 0x00, 0x01, 0x10, 0xFF, 0xF0, 0x01, 0x20, 0xFF, 0xE0, 0x01, 0x30, 0xFF,
        0xD0,
    ];
    mem.write_slice(&src1, GuestAddress(ALIGNED_ADDR)).unwrap();
    mem.write_slice(&src2, GuestAddress(ALIGNED_ADDR2)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_psubusw_powers_of_two() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x0b, 0x66, 0x0f, 0xd9, 0xc1, 0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let src1 = [
        0x00, 0x80, 0x00, 0x40, 0x00, 0x20, 0x00, 0x10, 0x00, 0x08, 0x00, 0x04, 0x00, 0x02, 0x00,
        0x01,
    ];
    let src2 = [
        0x00, 0x01, 0x00, 0x02, 0x00, 0x04, 0x00, 0x08, 0x00, 0x10, 0x00, 0x20, 0x00, 0x40, 0x00,
        0x80,
    ];
    mem.write_slice(&src1, GuestAddress(ALIGNED_ADDR)).unwrap();
    mem.write_slice(&src2, GuestAddress(ALIGNED_ADDR2)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Known-answer value tests (register-to-register via set_xmm/get_xmm)
//
// Unsigned saturating subtract DST - SRC, clamping each lane at 0 (no borrow
// below zero). Computed by hand from x86 PSUBUSB/PSUBUSW semantics.
// ============================================================================

#[test]
fn kat_psubusb_value() {
    // PSUBUSB XMM0, XMM1 (66 0F D8 C1)
    let code = [0x66, 0x0f, 0xd8, 0xc1, 0xf4];
    let (mut vcpu, mem) = setup_vm(&code, None);
    set_xmm(&mem, &mut vcpu, 0, 0x7f7f80800102fe007f01ff80fe7f0100);
    set_xmm(&mem, &mut vcpu, 1, 0x010180800102020101ff01800201ff00);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        get_xmm(&regs, 0),
        0x7e7e00000000fc007e00fe00fc7e0000,
        "PSUBUSB got {:032x}",
        get_xmm(&regs, 0)
    );
}

#[test]
fn kat_psubusw_value() {
    // PSUBUSW XMM0, XMM1 (66 0F D9 C1)
    let code = [0x66, 0x0f, 0xd9, 0xc1, 0xf4];
    let (mut vcpu, mem) = setup_vm(&code, None);
    set_xmm(&mem, &mut vcpu, 0, 0x7fff80000001fffe7fff8000ffff0001);
    set_xmm(&mem, &mut vcpu, 1, 0x0001800000027fff0001ffff00018000);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        get_xmm(&regs, 0),
        0x7ffe000000007fff7ffe0000fffe0000,
        "PSUBUSW got {:032x}",
        get_xmm(&regs, 0)
    );
}

#[test]
fn kat_psubusb_clamp_to_zero() {
    // Any lane where SRC > DST clamps to 0; 0x00 - anything = 0x00.
    let code = [0x66, 0x0f, 0xd8, 0xc1, 0xf4];
    let (mut vcpu, mem) = setup_vm(&code, None);
    set_xmm(&mem, &mut vcpu, 0, 0x00000000000000000000000000000000);
    set_xmm(&mem, &mut vcpu, 1, 0x0102030405060708090a0b0c0d0e0f10);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(get_xmm(&regs, 0), 0);
}
