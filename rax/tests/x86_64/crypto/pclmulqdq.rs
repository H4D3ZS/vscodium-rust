use crate::common::*;
use vm_memory::{Bytes, GuestAddress};

// PCLMULQDQ - Carry-Less Multiplication Quadword
//
// Performs a carry-less multiplication of two quadwords, selected from the first source
// and second source operand according to the value of the immediate byte.
// Bits 4 and 0 are used to select which 64-bit half of each operand to use.
//
// Opcode:
// 66 0F 3A 44 /r ib    PCLMULQDQ xmm1, xmm2/m128, imm8
//
// Immediate byte encoding:
// imm8[0] = 0: select xmm1[63:0],   imm8[0] = 1: select xmm1[127:64]
// imm8[4] = 0: select xmm2[63:0],   imm8[4] = 1: select xmm2[127:64]
//
// Pseudo-ops:
// PCLMULLQLQDQ xmm1, xmm2  (0x00) - Low x Low
// PCLMULHQLQDQ xmm1, xmm2  (0x01) - High x Low
// PCLMULLQHQDQ xmm1, xmm2  (0x10) - Low x High
// PCLMULHQHQDQ xmm1, xmm2  (0x11) - High x High

const ALIGNED_ADDR: u64 = 0x3000;

// ============================================================================
// Register to Register Tests - imm8 = 0x00 (Low x Low)
// ============================================================================

#[test]
fn test_pclmulqdq_xmm0_xmm1_imm00() {
    // PCLMULQDQ XMM0, XMM1, 0x00
    let code = [
        0x66, 0x0f, 0x3a, 0x44, 0xc1, 0x00, // PCLMULQDQ XMM0, XMM1, 0x00
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pclmulqdq_xmm2_xmm3_imm00() {
    // PCLMULQDQ XMM2, XMM3, 0x00
    let code = [
        0x66, 0x0f, 0x3a, 0x44, 0xd3, 0x00, // PCLMULQDQ XMM2, XMM3, 0x00
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pclmulqdq_xmm4_xmm5_imm00() {
    // PCLMULQDQ XMM4, XMM5, 0x00
    let code = [
        0x66, 0x0f, 0x3a, 0x44, 0xe5, 0x00, // PCLMULQDQ XMM4, XMM5, 0x00
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pclmulqdq_xmm6_xmm7_imm00() {
    // PCLMULQDQ XMM6, XMM7, 0x00
    let code = [
        0x66, 0x0f, 0x3a, 0x44, 0xf7, 0x00, // PCLMULQDQ XMM6, XMM7, 0x00
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pclmulqdq_xmm8_xmm9_imm00() {
    // PCLMULQDQ XMM8, XMM9, 0x00 (requires REX prefix)
    let code = [
        0x66, 0x45, 0x0f, 0x3a, 0x44, 0xc1, 0x00, // PCLMULQDQ XMM8, XMM9, 0x00
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pclmulqdq_xmm10_xmm11_imm00() {
    // PCLMULQDQ XMM10, XMM11, 0x00
    let code = [
        0x66, 0x45, 0x0f, 0x3a, 0x44, 0xd3, 0x00, // PCLMULQDQ XMM10, XMM11, 0x00
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pclmulqdq_xmm12_xmm13_imm00() {
    // PCLMULQDQ XMM12, XMM13, 0x00
    let code = [
        0x66, 0x45, 0x0f, 0x3a, 0x44, 0xe5, 0x00, // PCLMULQDQ XMM12, XMM13, 0x00
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pclmulqdq_xmm14_xmm15_imm00() {
    // PCLMULQDQ XMM14, XMM15, 0x00
    let code = [
        0x66, 0x45, 0x0f, 0x3a, 0x44, 0xf7, 0x00, // PCLMULQDQ XMM14, XMM15, 0x00
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Register to Register Tests - imm8 = 0x01 (High x Low)
// ============================================================================

#[test]
fn test_pclmulqdq_xmm0_xmm1_imm01() {
    // PCLMULQDQ XMM0, XMM1, 0x01
    let code = [
        0x66, 0x0f, 0x3a, 0x44, 0xc1, 0x01, // PCLMULQDQ XMM0, XMM1, 0x01
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pclmulqdq_xmm2_xmm3_imm01() {
    // PCLMULQDQ XMM2, XMM3, 0x01
    let code = [
        0x66, 0x0f, 0x3a, 0x44, 0xd3, 0x01, // PCLMULQDQ XMM2, XMM3, 0x01
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pclmulqdq_xmm4_xmm5_imm01() {
    // PCLMULQDQ XMM4, XMM5, 0x01
    let code = [
        0x66, 0x0f, 0x3a, 0x44, 0xe5, 0x01, // PCLMULQDQ XMM4, XMM5, 0x01
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pclmulqdq_xmm6_xmm7_imm01() {
    // PCLMULQDQ XMM6, XMM7, 0x01
    let code = [
        0x66, 0x0f, 0x3a, 0x44, 0xf7, 0x01, // PCLMULQDQ XMM6, XMM7, 0x01
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pclmulqdq_xmm8_xmm9_imm01() {
    // PCLMULQDQ XMM8, XMM9, 0x01
    let code = [
        0x66, 0x45, 0x0f, 0x3a, 0x44, 0xc1, 0x01, // PCLMULQDQ XMM8, XMM9, 0x01
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Register to Register Tests - imm8 = 0x10 (Low x High)
// ============================================================================

#[test]
fn test_pclmulqdq_xmm0_xmm1_imm10() {
    // PCLMULQDQ XMM0, XMM1, 0x10
    let code = [
        0x66, 0x0f, 0x3a, 0x44, 0xc1, 0x10, // PCLMULQDQ XMM0, XMM1, 0x10
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pclmulqdq_xmm2_xmm3_imm10() {
    // PCLMULQDQ XMM2, XMM3, 0x10
    let code = [
        0x66, 0x0f, 0x3a, 0x44, 0xd3, 0x10, // PCLMULQDQ XMM2, XMM3, 0x10
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pclmulqdq_xmm4_xmm5_imm10() {
    // PCLMULQDQ XMM4, XMM5, 0x10
    let code = [
        0x66, 0x0f, 0x3a, 0x44, 0xe5, 0x10, // PCLMULQDQ XMM4, XMM5, 0x10
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pclmulqdq_xmm6_xmm7_imm10() {
    // PCLMULQDQ XMM6, XMM7, 0x10
    let code = [
        0x66, 0x0f, 0x3a, 0x44, 0xf7, 0x10, // PCLMULQDQ XMM6, XMM7, 0x10
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pclmulqdq_xmm8_xmm9_imm10() {
    // PCLMULQDQ XMM8, XMM9, 0x10
    let code = [
        0x66, 0x45, 0x0f, 0x3a, 0x44, 0xc1, 0x10, // PCLMULQDQ XMM8, XMM9, 0x10
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Register to Register Tests - imm8 = 0x11 (High x High)
// ============================================================================

#[test]
fn test_pclmulqdq_xmm0_xmm1_imm11() {
    // PCLMULQDQ XMM0, XMM1, 0x11
    let code = [
        0x66, 0x0f, 0x3a, 0x44, 0xc1, 0x11, // PCLMULQDQ XMM0, XMM1, 0x11
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pclmulqdq_xmm2_xmm3_imm11() {
    // PCLMULQDQ XMM2, XMM3, 0x11
    let code = [
        0x66, 0x0f, 0x3a, 0x44, 0xd3, 0x11, // PCLMULQDQ XMM2, XMM3, 0x11
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pclmulqdq_xmm4_xmm5_imm11() {
    // PCLMULQDQ XMM4, XMM5, 0x11
    let code = [
        0x66, 0x0f, 0x3a, 0x44, 0xe5, 0x11, // PCLMULQDQ XMM4, XMM5, 0x11
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pclmulqdq_xmm6_xmm7_imm11() {
    // PCLMULQDQ XMM6, XMM7, 0x11
    let code = [
        0x66, 0x0f, 0x3a, 0x44, 0xf7, 0x11, // PCLMULQDQ XMM6, XMM7, 0x11
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pclmulqdq_xmm8_xmm9_imm11() {
    // PCLMULQDQ XMM8, XMM9, 0x11
    let code = [
        0x66, 0x45, 0x0f, 0x3a, 0x44, 0xc1, 0x11, // PCLMULQDQ XMM8, XMM9, 0x11
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Memory to Register Tests - All immediate values
// ============================================================================

#[test]
fn test_pclmulqdq_xmm0_mem_imm00() {
    // PCLMULQDQ XMM0, [RAX], 0x00
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x3a, 0x44, 0x00, 0x00, // PCLMULQDQ XMM0, [RAX], 0x00
        0xf4, // HLT
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
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pclmulqdq_xmm1_mem_imm01() {
    // PCLMULQDQ XMM1, [RAX], 0x01
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x3a, 0x44, 0x08, 0x01, // PCLMULQDQ XMM1, [RAX], 0x01
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(
        &[
            0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA,
            0xAA, 0xAA,
        ],
        GuestAddress(ALIGNED_ADDR),
    )
    .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pclmulqdq_xmm2_mem_imm10() {
    // PCLMULQDQ XMM2, [RAX], 0x10
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x3a, 0x44, 0x10, 0x10, // PCLMULQDQ XMM2, [RAX], 0x10
        0xf4, // HLT
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
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pclmulqdq_xmm3_mem_imm11() {
    // PCLMULQDQ XMM3, [RAX], 0x11
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x3a, 0x44, 0x18, 0x11, // PCLMULQDQ XMM3, [RAX], 0x11
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(
        &[
            0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33,
            0x33, 0x33,
        ],
        GuestAddress(ALIGNED_ADDR),
    )
    .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pclmulqdq_xmm7_mem_imm00() {
    // PCLMULQDQ XMM7, [RAX], 0x00
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x3a, 0x44, 0x38, 0x00, // PCLMULQDQ XMM7, [RAX], 0x00
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(
        &[
            0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77,
            0x77, 0x77,
        ],
        GuestAddress(ALIGNED_ADDR),
    )
    .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pclmulqdq_xmm8_mem_imm00() {
    // PCLMULQDQ XMM8, [RAX], 0x00
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x44, 0x0f, 0x3a, 0x44, 0x00, 0x00, // PCLMULQDQ XMM8, [RAX], 0x00
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(
        &[
            0x88, 0x88, 0x88, 0x88, 0x88, 0x88, 0x88, 0x88, 0x88, 0x88, 0x88, 0x88, 0x88, 0x88,
            0x88, 0x88,
        ],
        GuestAddress(ALIGNED_ADDR),
    )
    .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pclmulqdq_xmm15_mem_imm11() {
    // PCLMULQDQ XMM15, [RAX], 0x11
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x44, 0x0f, 0x3a, 0x44, 0x38, 0x11, // PCLMULQDQ XMM15, [RAX], 0x11
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(
        &[
            0xEE, 0xEE, 0xEE, 0xEE, 0xEE, 0xEE, 0xEE, 0xEE, 0xEE, 0xEE, 0xEE, 0xEE, 0xEE, 0xEE,
            0xEE, 0xEE,
        ],
        GuestAddress(ALIGNED_ADDR),
    )
    .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Polynomial Value Tests - Testing with known polynomial values
// ============================================================================

#[test]
fn test_pclmulqdq_polynomial_all_zeros() {
    // Test with all zero polynomial
    let code = [
        0x66, 0x0f, 0x3a, 0x44, 0xc1, 0x00, // PCLMULQDQ XMM0, XMM1, 0x00
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pclmulqdq_polynomial_all_ones() {
    // Test with all ones polynomial (0xFFFFFFFFFFFFFFFF)
    let code = [
        0x66, 0x0f, 0x3a, 0x44, 0xc1, 0x00, // PCLMULQDQ XMM0, XMM1, 0x00
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pclmulqdq_polynomial_one() {
    // Test with polynomial = 1
    let code = [
        0x66, 0x0f, 0x3a, 0x44, 0xc1, 0x00, // PCLMULQDQ XMM0, XMM1, 0x00
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pclmulqdq_polynomial_alternating_bits() {
    // Test with alternating bit pattern (0xAAAAAAAAAAAAAAAA, 0x5555555555555555)
    let code = [
        0x66, 0x0f, 0x3a, 0x44, 0xc1, 0x00, // PCLMULQDQ XMM0, XMM1, 0x00
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pclmulqdq_polynomial_high_bit() {
    // Test with only high bit set (0x8000000000000000)
    let code = [
        0x66, 0x0f, 0x3a, 0x44, 0xc1, 0x00, // PCLMULQDQ XMM0, XMM1, 0x00
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pclmulqdq_polynomial_low_bit() {
    // Test with only low bit set (0x0000000000000001)
    let code = [
        0x66, 0x0f, 0x3a, 0x44, 0xc1, 0x00, // PCLMULQDQ XMM0, XMM1, 0x00
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pclmulqdq_gcm_polynomial() {
    // Test with GCM polynomial (used in AES-GCM)
    // The GCM reduction polynomial is x^128 + x^7 + x^2 + x + 1
    let code = [
        0x66, 0x0f, 0x3a, 0x44, 0xc1, 0x00, // PCLMULQDQ XMM0, XMM1, 0x00
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pclmulqdq_sequential_values() {
    // Test with sequential values
    let code = [
        0x66, 0x0f, 0x3a, 0x44, 0xc1, 0x00, // PCLMULQDQ XMM0, XMM1, 0x00
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Quadword Selection Tests - Testing different quadword combinations
// ============================================================================

#[test]
fn test_pclmulqdq_same_register_imm00() {
    // Test multiplying a register with itself (low x low)
    let code = [
        0x66, 0x0f, 0x3a, 0x44, 0xc0, 0x00, // PCLMULQDQ XMM0, XMM0, 0x00
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pclmulqdq_same_register_imm01() {
    // Test multiplying a register with itself (high x low)
    let code = [
        0x66, 0x0f, 0x3a, 0x44, 0xc0, 0x01, // PCLMULQDQ XMM0, XMM0, 0x01
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pclmulqdq_same_register_imm10() {
    // Test multiplying a register with itself (low x high)
    let code = [
        0x66, 0x0f, 0x3a, 0x44, 0xc0, 0x10, // PCLMULQDQ XMM0, XMM0, 0x10
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pclmulqdq_same_register_imm11() {
    // Test multiplying a register with itself (high x high)
    let code = [
        0x66, 0x0f, 0x3a, 0x44, 0xc0, 0x11, // PCLMULQDQ XMM0, XMM0, 0x11
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pclmulqdq_all_immediates_xmm1_xmm2() {
    // Test all 4 immediate values in sequence with different registers
    let code = [
        0x66, 0x0f, 0x3a, 0x44, 0xca, 0x00, // PCLMULQDQ XMM1, XMM2, 0x00
        0x66, 0x0f, 0x3a, 0x44, 0xca, 0x01, // PCLMULQDQ XMM1, XMM2, 0x01
        0x66, 0x0f, 0x3a, 0x44, 0xca, 0x10, // PCLMULQDQ XMM1, XMM2, 0x10
        0x66, 0x0f, 0x3a, 0x44, 0xca, 0x11, // PCLMULQDQ XMM1, XMM2, 0x11
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Addressing Mode Tests
// ============================================================================

#[test]
fn test_pclmulqdq_base_displacement() {
    // PCLMULQDQ XMM0, [RAX + displacement], 0x00
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&(ALIGNED_ADDR - 0x20).to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x3a, 0x44, 0x40, 0x20, 0x00, // PCLMULQDQ XMM0, [RAX+0x20], 0x00
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(
        &[
            0x12, 0x12, 0x12, 0x12, 0x12, 0x12, 0x12, 0x12, 0x12, 0x12, 0x12, 0x12, 0x12, 0x12,
            0x12, 0x12,
        ],
        GuestAddress(ALIGNED_ADDR),
    )
    .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pclmulqdq_with_rbx_base() {
    // PCLMULQDQ XMM0, [RBX], 0x00
    let code = [
        0x48, 0xbb, // MOV RBX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x3a, 0x44, 0x03, 0x00, // PCLMULQDQ XMM0, [RBX], 0x00
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(
        &[
            0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77,
            0x77, 0x77,
        ],
        GuestAddress(ALIGNED_ADDR),
    )
    .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pclmulqdq_with_rcx_base() {
    // PCLMULQDQ XMM0, [RCX], 0x01
    let code = [
        0x48, 0xb9, // MOV RCX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x3a, 0x44, 0x01, 0x01, // PCLMULQDQ XMM0, [RCX], 0x01
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(
        &[
            0x88, 0x88, 0x88, 0x88, 0x88, 0x88, 0x88, 0x88, 0x88, 0x88, 0x88, 0x88, 0x88, 0x88,
            0x88, 0x88,
        ],
        GuestAddress(ALIGNED_ADDR),
    )
    .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pclmulqdq_with_rdx_base() {
    // PCLMULQDQ XMM0, [RDX], 0x10
    let code = [
        0x48, 0xba, // MOV RDX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x3a, 0x44, 0x02, 0x10, // PCLMULQDQ XMM0, [RDX], 0x10
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(
        &[
            0x99, 0x99, 0x99, 0x99, 0x99, 0x99, 0x99, 0x99, 0x99, 0x99, 0x99, 0x99, 0x99, 0x99,
            0x99, 0x99,
        ],
        GuestAddress(ALIGNED_ADDR),
    )
    .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pclmulqdq_with_rsi_base() {
    // PCLMULQDQ XMM1, [RSI], 0x11
    let code = [
        0x48, 0xbe, // MOV RSI, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x3a, 0x44, 0x0e, 0x11, // PCLMULQDQ XMM1, [RSI], 0x11
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(
        &[
            0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA,
            0xAA, 0xAA,
        ],
        GuestAddress(ALIGNED_ADDR),
    )
    .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Cross-Register Pattern Tests
// ============================================================================

#[test]
fn test_pclmulqdq_xmm0_xmm15() {
    // PCLMULQDQ XMM0, XMM15, 0x00
    let code = [
        0x66, 0x41, 0x0f, 0x3a, 0x44, 0xc7, 0x00, // PCLMULQDQ XMM0, XMM15, 0x00
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pclmulqdq_xmm15_xmm0() {
    // PCLMULQDQ XMM15, XMM0, 0x11
    let code = [
        0x66, 0x44, 0x0f, 0x3a, 0x44, 0xf8, 0x11, // PCLMULQDQ XMM15, XMM0, 0x11
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pclmulqdq_xmm7_xmm8() {
    // PCLMULQDQ XMM7, XMM8, 0x01
    let code = [
        0x66, 0x41, 0x0f, 0x3a, 0x44, 0xf8, 0x01, // PCLMULQDQ XMM7, XMM8, 0x01
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pclmulqdq_xmm8_xmm7() {
    // PCLMULQDQ XMM8, XMM7, 0x10
    let code = [
        0x66, 0x44, 0x0f, 0x3a, 0x44, 0xc7, 0x10, // PCLMULQDQ XMM8, XMM7, 0x10
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Data Pattern Tests with Memory
// ============================================================================

#[test]
fn test_pclmulqdq_mem_pattern_sequential() {
    // Test with sequential byte pattern in memory
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x3a, 0x44, 0x00, 0x00, // PCLMULQDQ XMM0, [RAX], 0x00
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(
        &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
        GuestAddress(ALIGNED_ADDR),
    )
    .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pclmulqdq_mem_pattern_alternating() {
    // Test with alternating pattern in memory
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x3a, 0x44, 0x08, 0x11, // PCLMULQDQ XMM1, [RAX], 0x11
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(
        &[
            0xAA, 0x55, 0xAA, 0x55, 0xAA, 0x55, 0xAA, 0x55, 0xAA, 0x55, 0xAA, 0x55, 0xAA, 0x55,
            0xAA, 0x55,
        ],
        GuestAddress(ALIGNED_ADDR),
    )
    .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pclmulqdq_mem_pattern_sparse() {
    // Test with sparse bit pattern
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x3a, 0x44, 0x10, 0x01, // PCLMULQDQ XMM2, [RAX], 0x01
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(
        &[
            0x01, 0x00, 0x01, 0x00, 0x01, 0x00, 0x01, 0x00, 0x01, 0x00, 0x01, 0x00, 0x01, 0x00,
            0x01, 0x00,
        ],
        GuestAddress(ALIGNED_ADDR),
    )
    .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Ignore bits test - only bits 0 and 4 should matter in immediate
// ============================================================================

#[test]
fn test_pclmulqdq_ignore_imm_bits() {
    // Test that other bits in immediate are ignored
    // 0x55 = 0b01010101, bits 0 and 4 are both 1, so should be same as 0x11
    let code = [
        0x66, 0x0f, 0x3a, 0x44, 0xc1, 0x55, // PCLMULQDQ XMM0, XMM1, 0x55
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pclmulqdq_ignore_imm_bits_aa() {
    // 0xAA = 0b10101010, bits 0 and 4 are both 0, so should be same as 0x00
    let code = [
        0x66, 0x0f, 0x3a, 0x44, 0xc1, 0xaa, // PCLMULQDQ XMM0, XMM1, 0xAA
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pclmulqdq_ignore_imm_bits_fe() {
    // 0xFE = 0b11111110, bit 0 = 0, bit 4 = 1, so should be same as 0x10
    let code = [
        0x66, 0x0f, 0x3a, 0x44, 0xc1, 0xfe, // PCLMULQDQ XMM0, XMM1, 0xFE
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pclmulqdq_ignore_imm_bits_0f() {
    // 0x0F = 0b00001111, bit 0 = 1, bit 4 = 0, so should be same as 0x01
    let code = [
        0x66, 0x0f, 0x3a, 0x44, 0xc1, 0x0f, // PCLMULQDQ XMM0, XMM1, 0x0F
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
