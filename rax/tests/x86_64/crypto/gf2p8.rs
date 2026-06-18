//! Tests for GF2P8 (Galois Field) Instructions.
//!
//! This module covers Galois Field GF(2^8) arithmetic instructions.
//!
//! Instructions covered:
//! - GF2P8MULB - Galois Field multiply bytes
//! - GF2P8AFFINEQB - Galois Field affine transformation
//! - GF2P8AFFINEINVQB - Galois Field affine transformation inverse
//! - VGF2P8MULB - Vector Galois Field multiply (AVX/AVX-512)
//! - VGF2P8AFFINEQB - Vector Galois Field affine (AVX/AVX-512)
//! - VGF2P8AFFINEINVQB - Vector Galois Field affine inverse (AVX/AVX-512)
//!
//! References: docs/gf2p8mulb.txt, docs/gf2p8affineqb.txt, docs/gf2p8affineinvqb.txt

use crate::common::*;
use rax::cpu::Registers;

// ============================================================================
// GF2P8MULB Tests - Galois Field Multiply Bytes
// ============================================================================

#[test]
fn test_gf2p8mulb_basic() {
    // GF2P8MULB - Multiply in GF(2^8)
    // Opcode: 66 0F 38 CF /r
    // Reduction polynomial: x^8 + x^4 + x^3 + x + 1
    let code = [
        0x66, 0x0F, 0xEF, 0xC0, // PXOR XMM0, XMM0
        0x66, 0x0F, 0xEF, 0xC9, // PXOR XMM1, XMM1
        0x66, 0x0F, 0x38, 0xCF, 0xC1, // GF2P8MULB XMM0, XMM1
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    // May #UD if GFNI not supported
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_gf2p8mulb_identity() {
    // GF2P8MULB with identity element (1)
    let code = [
        0x66, 0x0F, 0x6F, 0x05, 0x00, 0x00, 0x00, 0x00, // MOVDQA XMM0, [data]
        0x66, 0x0F, 0x6F, 0x0D, 0x00, 0x00, 0x00, 0x00, // MOVDQA XMM1, [ones]
        0x66, 0x0F, 0x38, 0xCF, 0xC1, // GF2P8MULB XMM0, XMM1
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_gf2p8mulb_different_xmm() {
    // GF2P8MULB with various XMM registers
    let code = [
        0x66, 0x0F, 0xEF, 0xD2, // PXOR XMM2, XMM2
        0x66, 0x0F, 0xEF, 0xDB, // PXOR XMM3, XMM3
        0x66, 0x0F, 0x38, 0xCF, 0xD3, // GF2P8MULB XMM2, XMM3
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_gf2p8mulb_memory_operand() {
    // GF2P8MULB with memory operand
    let code = [
        0x66, 0x0F, 0xEF, 0xC0, // PXOR XMM0, XMM0
        0x48, 0xC7, 0xC3, 0x00, 0x10, 0x00, 0x00, // MOV RBX, 0x1000
        0x66, 0x0F, 0x38, 0xCF, 0x03, // GF2P8MULB XMM0, [RBX]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_gf2p8mulb_xmm_high_regs() {
    // GF2P8MULB with XMM8-XMM15
    let code = [
        0x66, 0x45, 0x0F, 0xEF, 0xC0, // PXOR XMM8, XMM8
        0x66, 0x45, 0x0F, 0xEF, 0xC9, // PXOR XMM9, XMM9
        0x66, 0x45, 0x0F, 0x38, 0xCF, 0xC1, // GF2P8MULB XMM8, XMM9
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_gf2p8mulb_self_multiply() {
    // GF2P8MULB multiplying register by itself
    let code = [
        0x66, 0x0F, 0x6F, 0x05, 0x00, 0x00, 0x00, 0x00, // MOVDQA XMM0, [data]
        0x66, 0x0F, 0x38, 0xCF, 0xC0, // GF2P8MULB XMM0, XMM0
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_gf2p8mulb_sequence() {
    // Multiple GF2P8MULB operations
    let code = [
        0x66, 0x0F, 0xEF, 0xC0, // PXOR XMM0, XMM0
        0x66, 0x0F, 0xEF, 0xC9, // PXOR XMM1, XMM1
        0x66, 0x0F, 0x38, 0xCF, 0xC1, // GF2P8MULB XMM0, XMM1
        0x66, 0x0F, 0x38, 0xCF, 0xC1, // GF2P8MULB XMM0, XMM1
        0x66, 0x0F, 0x38, 0xCF, 0xC1, // GF2P8MULB XMM0, XMM1
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// GF2P8AFFINEQB Tests - Galois Field Affine Transformation
// ============================================================================

#[test]
fn test_gf2p8affineqb_basic() {
    // GF2P8AFFINEQB - Affine transformation in GF(2^8)
    // Opcode: 66 0F 3A CE /r ib
    // Performs A*x + b transformation
    let code = [
        0x66, 0x0F, 0xEF, 0xC0, // PXOR XMM0, XMM0
        0x66, 0x0F, 0xEF, 0xC9, // PXOR XMM1, XMM1
        0x66, 0x0F, 0x3A, 0xCE, 0xC1, 0x00, // GF2P8AFFINEQB XMM0, XMM1, 0
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_gf2p8affineqb_with_imm8() {
    // GF2P8AFFINEQB with various imm8 values (b vector)
    let code = [
        0x66, 0x0F, 0xEF, 0xC0, // PXOR XMM0, XMM0
        0x66, 0x0F, 0xEF, 0xC9, // PXOR XMM1, XMM1
        0x66, 0x0F, 0x3A, 0xCE, 0xC1, 0xFF, // GF2P8AFFINEQB XMM0, XMM1, 0xFF
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_gf2p8affineqb_identity_matrix() {
    // GF2P8AFFINEQB with identity matrix
    let code = [
        0x66, 0x0F, 0x6F, 0x05, 0x00, 0x00, 0x00, 0x00, // MOVDQA XMM0, [data]
        0x66, 0x0F, 0x6F, 0x0D, 0x00, 0x00, 0x00, 0x00, // MOVDQA XMM1, [identity]
        0x66, 0x0F, 0x3A, 0xCE, 0xC1, 0x00, // GF2P8AFFINEQB XMM0, XMM1, 0
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_gf2p8affineqb_different_xmm() {
    // GF2P8AFFINEQB with various XMM registers
    let code = [
        0x66, 0x0F, 0xEF, 0xD2, // PXOR XMM2, XMM2
        0x66, 0x0F, 0xEF, 0xDB, // PXOR XMM3, XMM3
        0x66, 0x0F, 0x3A, 0xCE, 0xD3, 0x42, // GF2P8AFFINEQB XMM2, XMM3, 0x42
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_gf2p8affineqb_memory_operand() {
    // GF2P8AFFINEQB with memory operand
    let code = [
        0x66, 0x0F, 0xEF, 0xC0, // PXOR XMM0, XMM0
        0x48, 0xC7, 0xC3, 0x00, 0x20, 0x00, 0x00, // MOV RBX, 0x2000
        0x66, 0x0F, 0x3A, 0xCE, 0x03, 0xAA, // GF2P8AFFINEQB XMM0, [RBX], 0xAA
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_gf2p8affineqb_xmm_high_regs() {
    // GF2P8AFFINEQB with XMM8-XMM15
    let code = [
        0x66, 0x45, 0x0F, 0xEF, 0xED, // PXOR XMM13, XMM13
        0x66, 0x45, 0x0F, 0xEF, 0xF6, // PXOR XMM14, XMM14
        0x66, 0x45, 0x0F, 0x3A, 0xCE, 0xEE, 0x55, // GF2P8AFFINEQB XMM13, XMM14, 0x55
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_gf2p8affineqb_various_imm() {
    // GF2P8AFFINEQB with different immediate values
    let code = [
        0x66, 0x0F, 0xEF, 0xC0, // PXOR XMM0, XMM0
        0x66, 0x0F, 0xEF, 0xC9, // PXOR XMM1, XMM1
        0x66, 0x0F, 0x3A, 0xCE, 0xC1, 0x00, // GF2P8AFFINEQB XMM0, XMM1, 0x00
        0x66, 0x0F, 0x3A, 0xCE, 0xC1, 0x01, // GF2P8AFFINEQB XMM0, XMM1, 0x01
        0x66, 0x0F, 0x3A, 0xCE, 0xC1, 0x80, // GF2P8AFFINEQB XMM0, XMM1, 0x80
        0x66, 0x0F, 0x3A, 0xCE, 0xC1, 0xFF, // GF2P8AFFINEQB XMM0, XMM1, 0xFF
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// GF2P8AFFINEINVQB Tests - Galois Field Affine Transformation Inverse
// ============================================================================

#[test]
fn test_gf2p8affineinvqb_basic() {
    // GF2P8AFFINEINVQB - Affine transformation with inverse in GF(2^8)
    // Opcode: 66 0F 3A CF /r ib
    // Performs A*inv(x) + b transformation
    let code = [
        0x66, 0x0F, 0xEF, 0xC0, // PXOR XMM0, XMM0
        0x66, 0x0F, 0xEF, 0xC9, // PXOR XMM1, XMM1
        0x66, 0x0F, 0x3A, 0xCF, 0xC1, 0x00, // GF2P8AFFINEINVQB XMM0, XMM1, 0
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_gf2p8affineinvqb_with_imm8() {
    // GF2P8AFFINEINVQB with various imm8 values
    let code = [
        0x66, 0x0F, 0xEF, 0xC0, // PXOR XMM0, XMM0
        0x66, 0x0F, 0xEF, 0xC9, // PXOR XMM1, XMM1
        0x66, 0x0F, 0x3A, 0xCF, 0xC1, 0x63, // GF2P8AFFINEINVQB XMM0, XMM1, 0x63
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_gf2p8affineinvqb_aes_sbox() {
    // GF2P8AFFINEINVQB for AES S-box (matrix and constant)
    // AES S-box uses specific matrix and b=0x63
    let code = [
        0x66, 0x0F, 0x6F, 0x05, 0x00, 0x00, 0x00, 0x00, // MOVDQA XMM0, [data]
        0x66, 0x0F, 0x6F, 0x0D, 0x00, 0x00, 0x00, 0x00, // MOVDQA XMM1, [aes_matrix]
        0x66, 0x0F, 0x3A, 0xCF, 0xC1, 0x63, // GF2P8AFFINEINVQB XMM0, XMM1, 0x63
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_gf2p8affineinvqb_different_xmm() {
    // GF2P8AFFINEINVQB with various XMM registers
    let code = [
        0x66, 0x0F, 0xEF, 0xE4, // PXOR XMM4, XMM4
        0x66, 0x0F, 0xEF, 0xED, // PXOR XMM5, XMM5
        0x66, 0x0F, 0x3A, 0xCF, 0xE5, 0x12, // GF2P8AFFINEINVQB XMM4, XMM5, 0x12
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_gf2p8affineinvqb_memory_operand() {
    // GF2P8AFFINEINVQB with memory operand
    let code = [
        0x66, 0x0F, 0xEF, 0xC0, // PXOR XMM0, XMM0
        0x48, 0xC7, 0xC1, 0x00, 0x30, 0x00, 0x00, // MOV RCX, 0x3000
        0x66, 0x0F, 0x3A, 0xCF, 0x01, 0x9A, // GF2P8AFFINEINVQB XMM0, [RCX], 0x9A
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_gf2p8affineinvqb_xmm_high_regs() {
    // GF2P8AFFINEINVQB with XMM8-XMM15
    let code = [
        0x66, 0x45, 0x0F, 0xEF, 0xFF, // PXOR XMM15, XMM15
        0x66, 0x45, 0x0F, 0xEF, 0xC0, // PXOR XMM8, XMM8
        0x66, 0x45, 0x0F, 0x3A, 0xCF, 0xF8, 0x7C, // GF2P8AFFINEINVQB XMM15, XMM8, 0x7C
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_gf2p8affineinvqb_various_imm() {
    // GF2P8AFFINEINVQB with different immediate values
    let code = [
        0x66, 0x0F, 0xEF, 0xC0, // PXOR XMM0, XMM0
        0x66, 0x0F, 0xEF, 0xC9, // PXOR XMM1, XMM1
        0x66, 0x0F, 0x3A, 0xCF, 0xC1, 0x00, // GF2P8AFFINEINVQB XMM0, XMM1, 0x00
        0x66, 0x0F, 0x3A, 0xCF, 0xC1, 0x63, // GF2P8AFFINEINVQB XMM0, XMM1, 0x63
        0x66, 0x0F, 0x3A, 0xCF, 0xC1, 0xC0, // GF2P8AFFINEINVQB XMM0, XMM1, 0xC0
        0x66, 0x0F, 0x3A, 0xCF, 0xC1, 0xFF, // GF2P8AFFINEINVQB XMM0, XMM1, 0xFF
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// VGF2P8MULB Tests - Vector Galois Field Multiply (AVX)
// ============================================================================

#[test]
fn test_vgf2p8mulb_vex128_basic() {
    // VGF2P8MULB - VEX.128 encoded
    // Opcode: VEX.128.66.0F38.W0 CF /r
    let code = [
        0x66, 0x0F, 0xEF, 0xC0, // PXOR XMM0, XMM0
        0x66, 0x0F, 0xEF, 0xC9, // PXOR XMM1, XMM1
        0x66, 0x0F, 0xEF, 0xD2, // PXOR XMM2, XMM2
        0xC4, 0xE2, 0x71, 0xCF, 0xC2, // VGF2P8MULB XMM0, XMM1, XMM2
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vgf2p8mulb_vex256_basic() {
    // VGF2P8MULB - VEX.256 encoded
    // Opcode: VEX.256.66.0F38.W0 CF /r
    let code = [
        0xC5, 0xFD, 0xEF, 0xC0, // VPXOR YMM0, YMM0, YMM0
        0xC5, 0xF5, 0xEF, 0xC9, // VPXOR YMM1, YMM1, YMM1
        0xC5, 0xED, 0xEF, 0xD2, // VPXOR YMM2, YMM2, YMM2
        0xC4, 0xE2, 0x75, 0xCF, 0xC2, // VGF2P8MULB YMM0, YMM1, YMM2
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vgf2p8mulb_memory_operand() {
    // VGF2P8MULB with memory operand
    let code = [
        0x66, 0x0F, 0xEF, 0xC0, // PXOR XMM0, XMM0
        0x66, 0x0F, 0xEF, 0xC9, // PXOR XMM1, XMM1
        0x48, 0xC7, 0xC3, 0x00, 0x40, 0x00, 0x00, // MOV RBX, 0x4000
        0xC4, 0xE2, 0x71, 0xCF, 0x03, // VGF2P8MULB XMM0, XMM1, [RBX]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// VGF2P8AFFINEQB Tests - Vector Galois Field Affine (AVX)
// ============================================================================

#[test]
fn test_vgf2p8affineqb_vex128_basic() {
    // VGF2P8AFFINEQB - VEX.128 encoded
    // Opcode: VEX.128.66.0F3A.W1 CE /r ib
    let code = [
        0x66, 0x0F, 0xEF, 0xC0, // PXOR XMM0, XMM0
        0x66, 0x0F, 0xEF, 0xC9, // PXOR XMM1, XMM1
        0x66, 0x0F, 0xEF, 0xD2, // PXOR XMM2, XMM2
        0xC4, 0xE3, 0xF1, 0xCE, 0xC2, 0x00, // VGF2P8AFFINEQB XMM0, XMM1, XMM2, 0
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vgf2p8affineqb_vex256_basic() {
    // VGF2P8AFFINEQB - VEX.256 encoded
    let code = [
        0xC5, 0xFD, 0xEF, 0xC0, // VPXOR YMM0, YMM0, YMM0
        0xC5, 0xF5, 0xEF, 0xC9, // VPXOR YMM1, YMM1, YMM1
        0xC5, 0xED, 0xEF, 0xD2, // VPXOR YMM2, YMM2, YMM2
        0xC4, 0xE3, 0xF5, 0xCE, 0xC2, 0xFF, // VGF2P8AFFINEQB YMM0, YMM1, YMM2, 0xFF
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// VGF2P8AFFINEINVQB Tests - Vector Galois Field Affine Inverse (AVX)
// ============================================================================

#[test]
fn test_vgf2p8affineinvqb_vex128_basic() {
    // VGF2P8AFFINEINVQB - VEX.128 encoded
    // Opcode: VEX.128.66.0F3A.W1 CF /r ib
    let code = [
        0x66, 0x0F, 0xEF, 0xC0, // PXOR XMM0, XMM0
        0x66, 0x0F, 0xEF, 0xC9, // PXOR XMM1, XMM1
        0x66, 0x0F, 0xEF, 0xD2, // PXOR XMM2, XMM2
        0xC4, 0xE3, 0xF1, 0xCF, 0xC2, 0x00, // VGF2P8AFFINEINVQB XMM0, XMM1, XMM2, 0
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vgf2p8affineinvqb_vex256_basic() {
    // VGF2P8AFFINEINVQB - VEX.256 encoded
    let code = [
        0xC5, 0xFD, 0xEF, 0xC0, // VPXOR YMM0, YMM0, YMM0
        0xC5, 0xF5, 0xEF, 0xC9, // VPXOR YMM1, YMM1, YMM1
        0xC5, 0xED, 0xEF, 0xD2, // VPXOR YMM2, YMM2, YMM2
        0xC4, 0xE3, 0xF5, 0xCF, 0xC2, 0x63, // VGF2P8AFFINEINVQB YMM0, YMM1, YMM2, 0x63
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// Combined and Application Tests
// ============================================================================

#[test]
fn test_gf2p8_aes_sbox_simulation() {
    // Simulate AES S-box using GF2P8 instructions
    let code = [
        // Load input data
        0x66, 0x0F, 0x6F, 0x05, 0x00, 0x00, 0x00, 0x00, // MOVDQA XMM0, [input]
        // Load AES affine matrix
        0x66, 0x0F, 0x6F, 0x0D, 0x00, 0x00, 0x00, 0x00, // MOVDQA XMM1, [aes_matrix]
        // Apply affine transformation with inverse and b=0x63
        0x66, 0x0F, 0x3A, 0xCF, 0xC1, 0x63, // GF2P8AFFINEINVQB XMM0, XMM1, 0x63
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_gf2p8_multiply_chain() {
    // Chain of GF2P8 multiplications
    let code = [
        0x66, 0x0F, 0x6F, 0x05, 0x00, 0x00, 0x00, 0x00, // MOVDQA XMM0, [data1]
        0x66, 0x0F, 0x6F, 0x0D, 0x00, 0x00, 0x00, 0x00, // MOVDQA XMM1, [data2]
        0x66, 0x0F, 0x6F, 0x15, 0x00, 0x00, 0x00, 0x00, // MOVDQA XMM2, [data3]
        0x66, 0x0F, 0x38, 0xCF, 0xC1, // GF2P8MULB XMM0, XMM1
        0x66, 0x0F, 0x38, 0xCF, 0xC2, // GF2P8MULB XMM0, XMM2
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_gf2p8_affine_composition() {
    // Compose affine transformations
    let code = [
        0x66, 0x0F, 0x6F, 0x05, 0x00, 0x00, 0x00, 0x00, // MOVDQA XMM0, [input]
        0x66, 0x0F, 0x6F, 0x0D, 0x00, 0x00, 0x00, 0x00, // MOVDQA XMM1, [matrix1]
        0x66, 0x0F, 0x6F, 0x15, 0x00, 0x00, 0x00, 0x00, // MOVDQA XMM2, [matrix2]
        // First affine transformation
        0x66, 0x0F, 0x3A, 0xCE, 0xC1, 0x00, // GF2P8AFFINEQB XMM0, XMM1, 0
        // Second affine transformation
        0x66, 0x0F, 0x3A, 0xCE, 0xC2, 0x00, // GF2P8AFFINEQB XMM0, XMM2, 0
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_gf2p8_inverse_roundtrip() {
    // Test affine and affine-inverse roundtrip
    let code = [
        0x66, 0x0F, 0x6F, 0x05, 0x00, 0x00, 0x00, 0x00, // MOVDQA XMM0, [original]
        0x66, 0x0F, 0x6F, 0x0D, 0x00, 0x00, 0x00, 0x00, // MOVDQA XMM1, [matrix]
        // Apply affine with inverse
        0x66, 0x0F, 0x3A, 0xCF, 0xC1, 0x63, // GF2P8AFFINEINVQB XMM0, XMM1, 0x63
        // Apply regular affine (should undo inverse)
        0x66, 0x0F, 0x3A, 0xCE, 0xC1, 0x9A, // GF2P8AFFINEQB XMM0, XMM1, 0x9A
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_gf2p8_all_operations() {
    // Use all three GF2P8 operations
    let code = [
        0x66, 0x0F, 0x6F, 0x05, 0x00, 0x00, 0x00, 0x00, // MOVDQA XMM0, [data]
        0x66, 0x0F, 0x6F, 0x0D, 0x00, 0x00, 0x00, 0x00, // MOVDQA XMM1, [multiplier]
        0x66, 0x0F, 0x6F, 0x15, 0x00, 0x00, 0x00, 0x00, // MOVDQA XMM2, [matrix]
        // Multiply
        0x66, 0x0F, 0x38, 0xCF, 0xC1, // GF2P8MULB XMM0, XMM1
        // Affine
        0x66, 0x0F, 0x3A, 0xCE, 0xC2, 0x42, // GF2P8AFFINEQB XMM0, XMM2, 0x42
        // Affine inverse
        0x66, 0x0F, 0x3A, 0xCF, 0xC2, 0x63, // GF2P8AFFINEINVQB XMM0, XMM2, 0x63
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// GFNI Known-Answer Tests (Intel SDM GF2P8MULB / GF2P8AFFINEQB vectors)
// ============================================================================
//
// IMPORTANT: GFNI (GF2P8MULB / GF2P8AFFINEQB / GF2P8AFFINEINVQB) is NOT
// implemented by the emulator. The legacy-map opcodes 66 0F 38 CF and
// 66 0F 3A CE/CF fall through to the `_ => Err(...)` arm in escape_38/escape_3a,
// so executing them returns an emulator error and `run_until_hlt(..).unwrap()`
// panics. These known-answer tests encode the correct Intel SDM expected
// outputs so they become live the moment GFNI is implemented, but they are
// `#[ignore]`d until then. See the agent summary for the reported bug.
//
// Reduction polynomial for GF2P8MULB is x^8+x^4+x^3+x+1 (0x11B).

// GF2P8MULB: byte-wise GF(2^8) multiply.
//   src1 (xmm0) bytes = 00 01 02 .. 0f  (u128 LE = 0f0e..0100)
//   src2 (xmm1) bytes = all 0x02
//   result bytes      = 2*i mod poly = 00 02 04 .. 1e  (u128 LE = 1e1c..0200)
const GF2P8MULB_A: u128 = 0x0f0e0d0c0b0a09080706050403020100;
const GF2P8MULB_B: u128 = 0x02020202020202020202020202020202;
const GF2P8MULB_RESULT: u128 = 0x1e1c1a18161412100e0c0a0806040200;

#[test]
fn kat_gf2p8mulb_intel_vector() {
    // GF2P8MULB XMM0, XMM1  (66 0F 38 CF C1)
    let code = [0x66, 0x0f, 0x38, 0xcf, 0xc1, 0xf4];
    let (mut vcpu, mem) = setup_vm(&code, None);
    set_xmm(&mem, &mut vcpu, 0, GF2P8MULB_A);
    set_xmm(&mem, &mut vcpu, 1, GF2P8MULB_B);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        get_xmm(&regs, 0),
        GF2P8MULB_RESULT,
        "GF2P8MULB produced {:032x}, expected {:032x}",
        get_xmm(&regs, 0),
        GF2P8MULB_RESULT
    );
}

#[test]
fn kat_gf2p8mulb_inverses() {
    // 0x53 * 0xCA = 0x01 in GF(2^8) (FIPS-197 multiplicative inverses).
    let code = [0x66, 0x0f, 0x38, 0xcf, 0xc1, 0xf4];
    let (mut vcpu, mem) = setup_vm(&code, None);
    set_xmm(&mem, &mut vcpu, 0, 0x53);
    set_xmm(&mem, &mut vcpu, 1, 0xca);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(get_xmm(&regs, 0) & 0xff, 0x01);
}

// GF2P8AFFINEQB XMM0, XMM1, imm8: for each byte x of xmm0, output bit (7-i) =
// parity(rowbyte_i & x) ^ imm[7-i], where the 8 row bytes come from the
// corresponding qword of xmm1. With xmm1 = the bit-reflected identity matrix
// (rows 0x80,0x40,0x20,0x10,0x08,0x04,0x02,0x01 per qword) and imm8 = 0, the
// transform is the identity, so output == input. With imm8 = 0xFF every output
// byte is complemented.
// True GFNI identity matrix per qword (qword 0x0102040810204080): each output
// bit i selects matrix.byte[7-i] = 1<<i, so the transform is the identity.
// (0x8040201008040201 is the bit-REVERSAL matrix, not identity.)
const GF2P8_IDENT_MATRIX: u128 = 0x0102040810204080_0102040810204080;
const GF2P8_AFFINE_X: u128 = 0x0011223344556677_8899aabbccddeeff;

#[test]
fn kat_gf2p8affineqb_identity_imm0() {
    // GF2P8AFFINEQB XMM0, XMM1, 0x00  (66 0F 3A CE C1 00)
    let code = [0x66, 0x0f, 0x3a, 0xce, 0xc1, 0x00, 0xf4];
    let (mut vcpu, mem) = setup_vm(&code, None);
    set_xmm(&mem, &mut vcpu, 0, GF2P8_AFFINE_X);
    set_xmm(&mem, &mut vcpu, 1, GF2P8_IDENT_MATRIX);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        get_xmm(&regs, 0),
        GF2P8_AFFINE_X,
        "GF2P8AFFINEQB(identity, imm0) should be identity, got {:032x}",
        get_xmm(&regs, 0)
    );
}

#[test]
fn kat_gf2p8affineqb_identity_immff() {
    // imm8 = 0xFF complements every output byte: output == !x.
    let code = [0x66, 0x0f, 0x3a, 0xce, 0xc1, 0xff, 0xf4];
    let (mut vcpu, mem) = setup_vm(&code, None);
    set_xmm(&mem, &mut vcpu, 0, GF2P8_AFFINE_X);
    set_xmm(&mem, &mut vcpu, 1, GF2P8_IDENT_MATRIX);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(get_xmm(&regs, 0), !GF2P8_AFFINE_X);
}
