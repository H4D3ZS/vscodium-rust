use crate::common::*;
use vm_memory::{Bytes, GuestAddress};

// PSRAW/PSRAD - Shift Packed Data Right Arithmetic (SSE2)
//
// Performs arithmetic right shift on packed integers in XMM registers.
// Empty high-order bits are filled with the sign bit (sign extension).
// If shift count > element size in bits, result is all sign bits.
//
// PSRAW: Shift 8 packed word integers (16-bit each) right with sign extension
// PSRAD: Shift 4 packed doubleword integers (32-bit each) right with sign extension
//
// Note: PSRAQ (quadword arithmetic shift) was added in AVX512, not available in SSE2
//
// Opcodes (SSE2 - 128-bit XMM):
// 66 0F E1 /r      PSRAW xmm1, xmm2/m128   - Shift words right by count in xmm2/m128
// 66 0F 71 /4 ib   PSRAW xmm1, imm8        - Shift words right by immediate
// 66 0F E2 /r      PSRAD xmm1, xmm2/m128   - Shift dwords right by count in xmm2/m128
// 66 0F 72 /4 ib   PSRAD xmm1, imm8        - Shift dwords right by immediate

const ALIGNED_ADDR: u64 = 0x3000;
const ALIGNED_ADDR2: u64 = 0x3100;

// ============================================================================
// PSRAW Tests - Shift 8x Word Right Arithmetic
// ============================================================================

#[test]
fn test_psraw_imm8_zero_shift() {
    // Shift by 0 should leave data unchanged
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, // MOVDQA XMM0, [RAX]
        0x66, 0x0f, 0x71, 0xe0, 0x00, // PSRAW XMM0, 0
        0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data = [
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F,
        0x10,
    ];
    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_psraw_imm8_one_bit_positive() {
    // Shift positive words right by 1 bit
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x71, 0xe0, 0x01, // PSRAW XMM0, 1
        0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    // Positive words: 0x0002, 0x0004, 0x0006, 0x0008
    let data = [
        0x02, 0x00, 0x04, 0x00, 0x06, 0x00, 0x08, 0x00, 0x0A, 0x00, 0x0C, 0x00, 0x0E, 0x00, 0x10,
        0x00,
    ];
    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_psraw_imm8_one_bit_negative() {
    // Shift negative words right by 1 bit with sign extension
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x71, 0xe0, 0x01, // PSRAW XMM0, 1
        0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    // Negative words: 0xFFFE, 0xFFFC, 0xFFFA, 0xFFF8 (-2, -4, -6, -8)
    let data = [
        0xFE, 0xFF, 0xFC, 0xFF, 0xFA, 0xFF, 0xF8, 0xFF, 0xF6, 0xFF, 0xF4, 0xFF, 0xF2, 0xFF, 0xF0,
        0xFF,
    ];
    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_psraw_imm8_seven_bits_positive() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x71, 0xe0, 0x07, // PSRAW XMM0, 7
        0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    // Words: 0x0080, 0x0100, 0x0180, 0x0200
    let data = [
        0x80, 0x00, 0x00, 0x01, 0x80, 0x01, 0x00, 0x02, 0x80, 0x02, 0x00, 0x03, 0x80, 0x03, 0x00,
        0x04,
    ];
    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_psraw_imm8_seven_bits_negative() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x71, 0xe0, 0x07, // PSRAW XMM0, 7
        0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    // Negative words: 0xFF80, 0xFF00, 0xFE80, 0xFE00
    let data = [
        0x80, 0xFF, 0x00, 0xFF, 0x80, 0xFE, 0x00, 0xFE, 0x80, 0xFD, 0x00, 0xFD, 0x80, 0xFC, 0x00,
        0xFC,
    ];
    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_psraw_imm8_eight_bits_positive() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x71, 0xe0, 0x08, // PSRAW XMM0, 8
        0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    // Positive words with value in high byte
    let data = [
        0x00, 0x01, 0x00, 0x02, 0x00, 0x03, 0x00, 0x04, 0x00, 0x05, 0x00, 0x06, 0x00, 0x07, 0x00,
        0x08,
    ];
    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_psraw_imm8_eight_bits_negative() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x71, 0xe0, 0x08, // PSRAW XMM0, 8
        0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    // Negative words: 0xFF00, 0xFE00, etc.
    let data = [
        0x00, 0xFF, 0x00, 0xFE, 0x00, 0xFD, 0x00, 0xFC, 0x00, 0xFB, 0x00, 0xFA, 0x00, 0xF9, 0x00,
        0xF8,
    ];
    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_psraw_imm8_fifteen_bits_positive() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x71, 0xe0, 0x0F, // PSRAW XMM0, 15
        0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    // Positive words (sign bit = 0): should become 0x0000
    let data = [
        0xFF, 0x7F, 0xFF, 0x7F, 0xFF, 0x7F, 0xFF, 0x7F, 0xFF, 0x7F, 0xFF, 0x7F, 0xFF, 0x7F, 0xFF,
        0x7F,
    ];
    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_psraw_imm8_fifteen_bits_negative() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x71, 0xe0, 0x0F, // PSRAW XMM0, 15
        0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    // Negative words (sign bit = 1): should become 0xFFFF
    let data = [
        0x00, 0x80, 0x01, 0x80, 0x00, 0x80, 0x01, 0x80, 0x00, 0x80, 0x01, 0x80, 0x00, 0x80, 0x01,
        0x80,
    ];
    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_psraw_imm8_sixteen_bits_sign_fill_positive() {
    // Shift by 16 or more fills with sign bit (all 0s for positive)
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x71, 0xe0, 0x10, // PSRAW XMM0, 16
        0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    // Positive words
    let data = [
        0xFF, 0x7F, 0xFE, 0x7F, 0xFD, 0x7F, 0xFC, 0x7F, 0xFB, 0x7F, 0xFA, 0x7F, 0xF9, 0x7F, 0xF8,
        0x7F,
    ];
    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_psraw_imm8_sixteen_bits_sign_fill_negative() {
    // Shift by 16 or more fills with sign bit (all 1s for negative)
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x71, 0xe0, 0x10, // PSRAW XMM0, 16
        0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    // Negative words
    let data = [
        0x00, 0x80, 0x01, 0x80, 0x02, 0x80, 0x03, 0x80, 0x04, 0x80, 0x05, 0x80, 0x06, 0x80, 0x07,
        0x80,
    ];
    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_psraw_imm8_mixed_signs() {
    // Test mixed positive and negative values
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x71, 0xe0, 0x01, // PSRAW XMM0, 1
        0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    // Alternating positive and negative words
    let data = [
        0x04, 0x00, 0xFC, 0xFF, 0x08, 0x00, 0xF8, 0xFF, 0x0C, 0x00, 0xF4, 0xFF, 0x10, 0x00, 0xF0,
        0xFF,
    ];
    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_psraw_xmm_count_zero() {
    // Shift count in XMM register = 0
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, // MOVDQA XMM0, [RAX]
        0x66, 0x0f, 0x6f, 0x0b, // MOVDQA XMM1, [RBX]
        0x66, 0x0f, 0xe1, 0xc1, // PSRAW XMM0, XMM1
        0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data = [
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F,
        0x10,
    ];
    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR)).unwrap();
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
fn test_psraw_xmm_count_four_negative() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x0b, 0x66, 0x0f, 0xe1, 0xc1, 0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    // Negative words: 0xFFF0, 0xFFE0, 0xFFD0, 0xFFC0
    let data = [
        0xF0, 0xFF, 0xE0, 0xFF, 0xD0, 0xFF, 0xC0, 0xFF, 0xB0, 0xFF, 0xA0, 0xFF, 0x90, 0xFF, 0x80,
        0xFF,
    ];
    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR)).unwrap();
    let count = [
        0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00,
    ];
    mem.write_slice(&count, GuestAddress(ALIGNED_ADDR2))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_psraw_xmm_from_memory() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0xe1, 0x03, // PSRAW XMM0, [RBX]
        0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    // Mixed positive and negative
    let data = [
        0x00, 0x0F, 0x00, 0xF0, 0x00, 0x0F, 0x00, 0xF0, 0x00, 0x0F, 0x00, 0xF0, 0x00, 0x0F, 0x00,
        0xF0,
    ];
    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR)).unwrap();
    let count = [
        0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00,
    ];
    mem.write_slice(&count, GuestAddress(ALIGNED_ADDR2))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// PSRAD Tests - Shift 4x Dword Right Arithmetic
// ============================================================================

#[test]
fn test_psrad_imm8_zero_shift() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x72, 0xe0, 0x00, // PSRAD XMM0, 0
        0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data = [
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F,
        0x10,
    ];
    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_psrad_imm8_one_bit_positive() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x72, 0xe0, 0x01, // PSRAD XMM0, 1
        0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    // Positive dwords: 0x00000002, 0x00000004, 0x00000006, 0x00000008
    let data = [
        0x02, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0x06, 0x00, 0x00, 0x00, 0x08, 0x00, 0x00,
        0x00,
    ];
    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_psrad_imm8_one_bit_negative() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x72, 0xe0, 0x01, // PSRAD XMM0, 1
        0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    // Negative dwords: 0xFFFFFFFE, 0xFFFFFFFC, 0xFFFFFFFA, 0xFFFFFFF8 (-2, -4, -6, -8)
    let data = [
        0xFE, 0xFF, 0xFF, 0xFF, 0xFC, 0xFF, 0xFF, 0xFF, 0xFA, 0xFF, 0xFF, 0xFF, 0xF8, 0xFF, 0xFF,
        0xFF,
    ];
    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_psrad_imm8_seven_bits_positive() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x72, 0xe0, 0x07, // PSRAD XMM0, 7
        0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    // Dwords: 0x00000080, 0x00000100, 0x00000180, 0x00000200
    let data = [
        0x80, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x80, 0x01, 0x00, 0x00, 0x00, 0x02, 0x00,
        0x00,
    ];
    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_psrad_imm8_seven_bits_negative() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x72, 0xe0, 0x07, // PSRAD XMM0, 7
        0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    // Negative dwords
    let data = [
        0x80, 0xFF, 0xFF, 0xFF, 0x00, 0xFF, 0xFF, 0xFF, 0x80, 0xFE, 0xFF, 0xFF, 0x00, 0xFE, 0xFF,
        0xFF,
    ];
    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_psrad_imm8_eight_bits_positive() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x72, 0xe0, 0x08, // PSRAD XMM0, 8
        0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data = [
        0x00, 0xFF, 0x00, 0x00, 0x00, 0xFE, 0x00, 0x00, 0x00, 0xFD, 0x00, 0x00, 0x00, 0xFC, 0x00,
        0x00,
    ];
    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_psrad_imm8_eight_bits_negative() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x72, 0xe0, 0x08, // PSRAD XMM0, 8
        0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    // Negative dwords
    let data = [
        0x00, 0xFF, 0xFF, 0xFF, 0x00, 0xFE, 0xFF, 0xFF, 0x00, 0xFD, 0xFF, 0xFF, 0x00, 0xFC, 0xFF,
        0xFF,
    ];
    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_psrad_imm8_sixteen_bits_positive() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x72, 0xe0, 0x10, // PSRAD XMM0, 16
        0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data = [
        0x00, 0x00, 0xFF, 0x00, 0x00, 0x00, 0xFE, 0x00, 0x00, 0x00, 0xFD, 0x00, 0x00, 0x00, 0xFC,
        0x00,
    ];
    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_psrad_imm8_sixteen_bits_negative() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x72, 0xe0, 0x10, // PSRAD XMM0, 16
        0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data = [
        0x00, 0x00, 0xFF, 0xFF, 0x00, 0x00, 0xFE, 0xFF, 0x00, 0x00, 0xFD, 0xFF, 0x00, 0x00, 0xFC,
        0xFF,
    ];
    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_psrad_imm8_thirtyone_bits_positive() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x72, 0xe0, 0x1F, // PSRAD XMM0, 31
        0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    // Positive dwords (sign bit = 0): should become 0x00000000
    let data = [
        0xFF, 0xFF, 0xFF, 0x7F, 0xFF, 0xFF, 0xFF, 0x7F, 0xFF, 0xFF, 0xFF, 0x7F, 0xFF, 0xFF, 0xFF,
        0x7F,
    ];
    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_psrad_imm8_thirtyone_bits_negative() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x72, 0xe0, 0x1F, // PSRAD XMM0, 31
        0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    // Negative dwords (sign bit = 1): should become 0xFFFFFFFF
    let data = [
        0x00, 0x00, 0x00, 0x80, 0x01, 0x00, 0x00, 0x80, 0x02, 0x00, 0x00, 0x80, 0x03, 0x00, 0x00,
        0x80,
    ];
    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_psrad_imm8_thirtytwo_bits_sign_fill_positive() {
    // Shift by 32 or more fills with sign bit (all 0s for positive)
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x72, 0xe0, 0x20, // PSRAD XMM0, 32
        0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    // Positive dwords
    let data = [
        0xFF, 0xFF, 0xFF, 0x7F, 0xFE, 0xFF, 0xFF, 0x7F, 0xFD, 0xFF, 0xFF, 0x7F, 0xFC, 0xFF, 0xFF,
        0x7F,
    ];
    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_psrad_imm8_thirtytwo_bits_sign_fill_negative() {
    // Shift by 32 or more fills with sign bit (all 1s for negative)
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x72, 0xe0, 0x20, // PSRAD XMM0, 32
        0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    // Negative dwords
    let data = [
        0x00, 0x00, 0x00, 0x80, 0x01, 0x00, 0x00, 0x80, 0x02, 0x00, 0x00, 0x80, 0x03, 0x00, 0x00,
        0x80,
    ];
    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_psrad_imm8_mixed_signs() {
    // Test mixed positive and negative values
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x72, 0xe0, 0x01, // PSRAD XMM0, 1
        0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    // Alternating positive and negative dwords
    let data = [
        0x08, 0x00, 0x00, 0x00, 0xF8, 0xFF, 0xFF, 0xFF, 0x10, 0x00, 0x00, 0x00, 0xF0, 0xFF, 0xFF,
        0xFF,
    ];
    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_psrad_xmm_count_zero() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x0b, 0x66, 0x0f, 0xe2,
        0xc1, // PSRAD XMM0, XMM1
        0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data = [
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F,
        0x10,
    ];
    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR)).unwrap();
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
fn test_psrad_xmm_count_eight_negative() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x0b, 0x66, 0x0f, 0xe2, 0xc1, 0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    // Negative dwords
    let data = [
        0x00, 0xFF, 0xFF, 0xFF, 0x00, 0xEE, 0xFF, 0xFF, 0x00, 0xDD, 0xFF, 0xFF, 0x00, 0xCC, 0xFF,
        0xFF,
    ];
    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR)).unwrap();
    let count = [
        0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00,
    ];
    mem.write_slice(&count, GuestAddress(ALIGNED_ADDR2))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_psrad_xmm_from_memory() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0xe2, 0x03, // PSRAD XMM0, [RBX]
        0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    // Mixed positive and negative
    let data = [
        0x00, 0x00, 0x0F, 0x00, 0x00, 0x00, 0xF0, 0xFF, 0x00, 0x00, 0x0F, 0x00, 0x00, 0x00, 0xF0,
        0xFF,
    ];
    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR)).unwrap();
    let count = [
        0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00,
    ];
    mem.write_slice(&count, GuestAddress(ALIGNED_ADDR2))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Register Variant Tests
// ============================================================================

#[test]
fn test_psraw_xmm2_xmm3() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x10, // MOVDQA XMM2, [RAX]
        0x66, 0x0f, 0x71, 0xe2, 0x04, // PSRAW XMM2, 4
        0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    // Mixed signs
    let data = [
        0xF0, 0x00, 0xF0, 0xFF, 0xE0, 0x00, 0xE0, 0xFF, 0xD0, 0x00, 0xD0, 0xFF, 0xC0, 0x00, 0xC0,
        0xFF,
    ];
    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_psrad_xmm4_xmm5() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x20, // MOVDQA XMM4, [RAX]
        0x66, 0x0f, 0x72, 0xe4, 0x08, // PSRAD XMM4, 8
        0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    // Mixed signs
    let data = [
        0x00, 0xFF, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0x00, 0xEE, 0x00, 0x00, 0x00, 0xEE, 0xFF,
        0xFF,
    ];
    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Extended Register Tests (XMM8-XMM15)
// ============================================================================

#[test]
fn test_psraw_xmm8_xmm9() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x44, 0x0f, 0x6f, 0x00, // MOVDQA XMM8, [RAX]
        0x66, 0x41, 0x0f, 0x71, 0xe0, 0x01, // PSRAW XMM8, 1
        0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data = [
        0x04, 0x00, 0xFC, 0xFF, 0x08, 0x00, 0xF8, 0xFF, 0x0C, 0x00, 0xF4, 0xFF, 0x10, 0x00, 0xF0,
        0xFF,
    ];
    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_psrad_xmm10_xmm11() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x44, 0x0f, 0x6f, 0x10, // MOVDQA XMM10, [RAX]
        0x66, 0x41, 0x0f, 0x72, 0xe2, 0x04, // PSRAD XMM10, 4
        0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data = [
        0xF0, 0x00, 0x00, 0x00, 0xF0, 0xFF, 0xFF, 0xFF, 0xE0, 0x00, 0x00, 0x00, 0xE0, 0xFF, 0xFF,
        0xFF,
    ];
    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Known-answer value tests (immediate shift counts via set_xmm/get_xmm)
//
// Arithmetic right shift: each lane shifted independently with sign extension
// (the lane's MSB is replicated into the vacated high bits). Computed by hand.
// ============================================================================

#[test]
fn kat_psraw_imm4_value() {
    // PSRAW XMM0, 4 (66 0F 71 /4 ib => E0 04)
    let code = [0x66, 0x0f, 0x71, 0xe0, 0x04, 0xf4];
    let (mut vcpu, mem) = setup_vm(&code, None);
    set_xmm(&mem, &mut vcpu, 0, 0x8001400220031004f008700c600d500e);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        get_xmm(&regs, 0),
        0xf800040002000100ff00070006000500,
        "PSRAW got {:032x}",
        get_xmm(&regs, 0)
    );
}

#[test]
fn kat_psrad_imm4_value() {
    // PSRAD XMM0, 4 (66 0F 72 /4 ib => E0 04)
    let code = [0x66, 0x0f, 0x72, 0xe0, 0x04, 0xf4];
    let (mut vcpu, mem) = setup_vm(&code, None);
    set_xmm(&mem, &mut vcpu, 0, 0x800000017fffffff00000010ffffffff);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        get_xmm(&regs, 0),
        0xf800000007ffffff00000001ffffffff,
        "PSRAD got {:032x}",
        get_xmm(&regs, 0)
    );
}

#[test]
fn kat_psraw_negative_fills_ones() {
    // A negative word (MSB=1) shifted right by 15 becomes all-ones (0xFFFF);
    // a positive word becomes 0x0000.
    let code = [0x66, 0x0f, 0x71, 0xe0, 0x0f, 0xf4]; // PSRAW XMM0, 15
    let (mut vcpu, mem) = setup_vm(&code, None);
    set_xmm(&mem, &mut vcpu, 0, 0x8000000080000000_7fff00007fff0000);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(get_xmm(&regs, 0), 0xffff0000ffff0000_0000000000000000);
}
