//! Tests for AVX-512 Bit Manipulation Instructions.
//!
//! This module covers AVX-512 advanced bit manipulation operations including
//! alignment, rotation, and ternary logic.
//!
//! Instructions covered:
//! - VALIGND - Align doubleword vectors
//! - VALIGNQ - Align quadword vectors
//! - VPROLD/VPROLVD - Rotate left dwords (immediate and variable)
//! - VPROLQ/VPROLVQ - Rotate left qwords (immediate and variable)
//! - VPRORD/VPRORVD - Rotate right dwords (immediate and variable)
//! - VPRORQ/VPRORVQ - Rotate right qwords (immediate and variable)
//! - VPTERNLOGD - Ternary logic operation on dwords
//! - VPTERNLOGQ - Ternary logic operation on qwords
//!
//! These instructions are part of AVX512F and AVX512VL extensions.
//!
//! References: Intel SDM Vol. 2, AVX-512 instruction documentation

use crate::common::*;
use rax::cpu::Registers;

// ============================================================================
// VALIGND Tests - Align Doubleword Vectors
// ============================================================================

#[test]
fn test_valignd_xmm_basic() {
    // VALIGND - Align and shift doubleword vectors (XMM)
    // EVEX.128.66.0F3A.W0 03 /r ib
    let code = [
        0x62, 0xF3, 0x7D, 0x08, 0x03, 0xC1, 0x01, // VALIGND xmm0, xmm0, xmm1, 1
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_valignd_ymm_basic() {
    // VALIGND - Align doubleword vectors (YMM)
    // EVEX.256.66.0F3A.W0 03 /r ib
    let code = [
        0x62, 0xF3, 0x7D, 0x28, 0x03, 0xC2, 0x02, // VALIGND ymm0, ymm0, ymm2, 2
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_valignd_zmm_basic() {
    // VALIGND - Align doubleword vectors (ZMM)
    // EVEX.512.66.0F3A.W0 03 /r ib
    let code = [
        0x62, 0xF3, 0x7D, 0x48, 0x03, 0xC3, 0x04, // VALIGND zmm0, zmm0, zmm3, 4
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_valignd_zmm_memory() {
    // VALIGND from memory
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x10, 0x00, 0x00, // MOV RAX, 0x1000
        0x62, 0xF3, 0x7D, 0x48, 0x03, 0x00, 0x03, // VALIGND zmm0, zmm0, [rax], 3
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_valignd_zmm_zero_shift() {
    // Test with zero shift amount
    let code = [
        0x62, 0xF3, 0x7D, 0x48, 0x03, 0xC1, 0x00, // VALIGND zmm0, zmm0, zmm1, 0
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_valignd_zmm_max_shift() {
    // Test with maximum shift amount for ZMM (15)
    let code = [
        0x62, 0xF3, 0x7D, 0x48, 0x03, 0xC1, 0x0F, // VALIGND zmm0, zmm0, zmm1, 15
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// VALIGNQ Tests - Align Quadword Vectors
// ============================================================================

#[test]
fn test_valignq_xmm_basic() {
    // VALIGNQ - Align quadword vectors (XMM)
    // EVEX.128.66.0F3A.W1 03 /r ib
    let code = [
        0x62, 0xF3, 0xFD, 0x08, 0x03, 0xC1, 0x01, // VALIGNQ xmm0, xmm0, xmm1, 1
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_valignq_ymm_basic() {
    // VALIGNQ - Align quadword vectors (YMM)
    // EVEX.256.66.0F3A.W1 03 /r ib
    let code = [
        0x62, 0xF3, 0xFD, 0x28, 0x03, 0xC2, 0x02, // VALIGNQ ymm0, ymm0, ymm2, 2
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_valignq_zmm_basic() {
    // VALIGNQ - Align quadword vectors (ZMM)
    // EVEX.512.66.0F3A.W1 03 /r ib
    let code = [
        0x62, 0xF3, 0xFD, 0x48, 0x03, 0xC3, 0x04, // VALIGNQ zmm0, zmm0, zmm3, 4
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_valignq_zmm_memory() {
    // VALIGNQ from memory
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x20, 0x00, 0x00, // MOV RAX, 0x2000
        0x62, 0xF3, 0xFD, 0x48, 0x03, 0x00, 0x05, // VALIGNQ zmm0, zmm0, [rax], 5
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_valignq_zmm_max_shift() {
    // Test with maximum shift amount for ZMM (7)
    let code = [
        0x62, 0xF3, 0xFD, 0x48, 0x03, 0xC1, 0x07, // VALIGNQ zmm0, zmm0, zmm1, 7
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// VPROLD Tests - Rotate Left Dword (Immediate)
// ============================================================================

#[test]
fn test_vprold_xmm_basic() {
    // VPROLD - Rotate left dwords by immediate (XMM)
    // EVEX.128.66.0F.W0 72 /1 ib
    let code = [
        0x62, 0xF1, 0x7D, 0x08, 0x72, 0xC8, 0x08, // VPROLD xmm1, xmm0, 8
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vprold_ymm_basic() {
    // VPROLD - Rotate left dwords (YMM)
    let code = [
        0x62, 0xF1, 0x7D, 0x28, 0x72, 0xC8, 0x10, // VPROLD ymm1, ymm0, 16
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vprold_zmm_basic() {
    // VPROLD - Rotate left dwords (ZMM)
    let code = [
        0x62, 0xF1, 0x7D, 0x48, 0x72, 0xC8, 0x04, // VPROLD zmm1, zmm0, 4
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vprold_zmm_memory() {
    // VPROLD from memory
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x30, 0x00, 0x00, // MOV RAX, 0x3000
        0x62, 0xF1, 0x7D, 0x48, 0x72, 0x08, 0x0C, // VPROLD zmm1, [rax], 12
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vprold_full_rotation() {
    // Test full 32-bit rotation
    let code = [
        0x62, 0xF1, 0x7D, 0x48, 0x72, 0xC8, 0x20, // VPROLD zmm1, zmm0, 32
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// VPROLQ Tests - Rotate Left Qword (Immediate)
// ============================================================================

#[test]
fn test_vprolq_xmm_basic() {
    // VPROLQ - Rotate left qwords by immediate (XMM)
    // EVEX.128.66.0F.W1 72 /1 ib
    let code = [
        0x62, 0xF1, 0xFD, 0x08, 0x72, 0xC8, 0x08, // VPROLQ xmm1, xmm0, 8
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vprolq_ymm_basic() {
    // VPROLQ - Rotate left qwords (YMM)
    let code = [
        0x62, 0xF1, 0xFD, 0x28, 0x72, 0xC8, 0x10, // VPROLQ ymm1, ymm0, 16
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vprolq_zmm_basic() {
    // VPROLQ - Rotate left qwords (ZMM)
    let code = [
        0x62, 0xF1, 0xFD, 0x48, 0x72, 0xC8, 0x20, // VPROLQ zmm1, zmm0, 32
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// VPROLVD Tests - Variable Rotate Left Dword
// ============================================================================

#[test]
fn test_vprolvd_xmm_basic() {
    // VPROLVD - Variable rotate left dwords (XMM)
    // EVEX.128.66.0F38.W0 15 /r
    let code = [
        0x62, 0xF2, 0x7D, 0x08, 0x15, 0xC1, // VPROLVD xmm0, xmm0, xmm1
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vprolvd_ymm_basic() {
    // VPROLVD - Variable rotate left dwords (YMM)
    let code = [
        0x62, 0xF2, 0x7D, 0x28, 0x15, 0xC2, // VPROLVD ymm0, ymm0, ymm2
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vprolvd_zmm_basic() {
    // VPROLVD - Variable rotate left dwords (ZMM)
    let code = [
        0x62, 0xF2, 0x7D, 0x48, 0x15, 0xC3, // VPROLVD zmm0, zmm0, zmm3
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vprolvd_zmm_memory() {
    // VPROLVD from memory
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x40, 0x00, 0x00, // MOV RAX, 0x4000
        0x62, 0xF2, 0x7D, 0x48, 0x15, 0x00, // VPROLVD zmm0, zmm0, [rax]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// VPROLVQ Tests - Variable Rotate Left Qword
// ============================================================================

#[test]
fn test_vprolvq_xmm_basic() {
    // VPROLVQ - Variable rotate left qwords (XMM)
    // EVEX.128.66.0F38.W1 15 /r
    let code = [
        0x62, 0xF2, 0xFD, 0x08, 0x15, 0xC1, // VPROLVQ xmm0, xmm0, xmm1
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vprolvq_ymm_basic() {
    // VPROLVQ - Variable rotate left qwords (YMM)
    let code = [
        0x62, 0xF2, 0xFD, 0x28, 0x15, 0xC2, // VPROLVQ ymm0, ymm0, ymm2
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vprolvq_zmm_basic() {
    // VPROLVQ - Variable rotate left qwords (ZMM)
    let code = [
        0x62, 0xF2, 0xFD, 0x48, 0x15, 0xC3, // VPROLVQ zmm0, zmm0, zmm3
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// VPRORD Tests - Rotate Right Dword (Immediate)
// ============================================================================

#[test]
fn test_vprord_xmm_basic() {
    // VPRORD - Rotate right dwords by immediate (XMM)
    // EVEX.128.66.0F.W0 72 /0 ib
    let code = [
        0x62, 0xF1, 0x7D, 0x08, 0x72, 0xC0, 0x08, // VPRORD xmm0, xmm0, 8
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vprord_ymm_basic() {
    // VPRORD - Rotate right dwords (YMM)
    let code = [
        0x62, 0xF1, 0x7D, 0x28, 0x72, 0xC0, 0x10, // VPRORD ymm0, ymm0, 16
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vprord_zmm_basic() {
    // VPRORD - Rotate right dwords (ZMM)
    let code = [
        0x62, 0xF1, 0x7D, 0x48, 0x72, 0xC0, 0x04, // VPRORD zmm0, zmm0, 4
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// VPRORQ Tests - Rotate Right Qword (Immediate)
// ============================================================================

#[test]
fn test_vprorq_xmm_basic() {
    // VPRORQ - Rotate right qwords by immediate (XMM)
    // EVEX.128.66.0F.W1 72 /0 ib
    let code = [
        0x62, 0xF1, 0xFD, 0x08, 0x72, 0xC0, 0x08, // VPRORQ xmm0, xmm0, 8
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vprorq_ymm_basic() {
    // VPRORQ - Rotate right qwords (YMM)
    let code = [
        0x62, 0xF1, 0xFD, 0x28, 0x72, 0xC0, 0x10, // VPRORQ ymm0, ymm0, 16
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vprorq_zmm_basic() {
    // VPRORQ - Rotate right qwords (ZMM)
    let code = [
        0x62, 0xF1, 0xFD, 0x48, 0x72, 0xC0, 0x20, // VPRORQ zmm0, zmm0, 32
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// VPRORVD Tests - Variable Rotate Right Dword
// ============================================================================

#[test]
fn test_vprorvd_xmm_basic() {
    // VPRORVD - Variable rotate right dwords (XMM)
    // EVEX.128.66.0F38.W0 14 /r
    let code = [
        0x62, 0xF2, 0x7D, 0x08, 0x14, 0xC1, // VPRORVD xmm0, xmm0, xmm1
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vprorvd_ymm_basic() {
    // VPRORVD - Variable rotate right dwords (YMM)
    let code = [
        0x62, 0xF2, 0x7D, 0x28, 0x14, 0xC2, // VPRORVD ymm0, ymm0, ymm2
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vprorvd_zmm_basic() {
    // VPRORVD - Variable rotate right dwords (ZMM)
    let code = [
        0x62, 0xF2, 0x7D, 0x48, 0x14, 0xC3, // VPRORVD zmm0, zmm0, zmm3
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// VPRORVQ Tests - Variable Rotate Right Qword
// ============================================================================

#[test]
fn test_vprorvq_xmm_basic() {
    // VPRORVQ - Variable rotate right qwords (XMM)
    // EVEX.128.66.0F38.W1 14 /r
    let code = [
        0x62, 0xF2, 0xFD, 0x08, 0x14, 0xC1, // VPRORVQ xmm0, xmm0, xmm1
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vprorvq_ymm_basic() {
    // VPRORVQ - Variable rotate right qwords (YMM)
    let code = [
        0x62, 0xF2, 0xFD, 0x28, 0x14, 0xC2, // VPRORVQ ymm0, ymm0, ymm2
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vprorvq_zmm_basic() {
    // VPRORVQ - Variable rotate right qwords (ZMM)
    let code = [
        0x62, 0xF2, 0xFD, 0x48, 0x14, 0xC3, // VPRORVQ zmm0, zmm0, zmm3
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// VPTERNLOGD Tests - Ternary Logic Dword
// ============================================================================

#[test]
fn test_vpternlogd_xmm_basic() {
    // VPTERNLOGD - Ternary logic operation on dwords (XMM)
    // EVEX.128.66.0F3A.W0 25 /r ib
    let code = [
        0x62, 0xF3, 0x6D, 0x08, 0x25, 0xC2, 0xF0, // VPTERNLOGD xmm0, xmm2, xmm2, 0xF0
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpternlogd_ymm_basic() {
    // VPTERNLOGD - Ternary logic (YMM)
    let code = [
        0x62, 0xF3, 0x6D, 0x28, 0x25, 0xC3, 0xAA, // VPTERNLOGD ymm0, ymm2, ymm3, 0xAA
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpternlogd_zmm_basic() {
    // VPTERNLOGD - Ternary logic (ZMM)
    let code = [
        0x62, 0xF3, 0x6D, 0x48, 0x25, 0xC1, 0x96, // VPTERNLOGD zmm0, zmm2, zmm1, 0x96 (XOR)
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpternlogd_zmm_memory() {
    // VPTERNLOGD from memory
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x50, 0x00, 0x00, // MOV RAX, 0x5000
        0x62, 0xF3, 0x6D, 0x48, 0x25, 0x00, 0xC0, // VPTERNLOGD zmm0, zmm2, [rax], 0xC0
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpternlogd_logic_operations() {
    // Test various logic operations
    let code = [
        // AND: (A & B & C) - imm8 = 0x80
        0x62, 0xF3, 0x6D, 0x48, 0x25, 0xC1, 0x80, // VPTERNLOGD zmm0, zmm2, zmm1, 0x80
        // OR: (A | B | C) - imm8 = 0xFE
        0x62, 0xF3, 0x65, 0x48, 0x25, 0xCA, 0xFE, // VPTERNLOGD zmm1, zmm3, zmm2, 0xFE
        // XOR: A ^ B ^ C - imm8 = 0x96
        0x62, 0xF3, 0x5D, 0x48, 0x25, 0xD3, 0x96, // VPTERNLOGD zmm2, zmm4, zmm3, 0x96
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// VPTERNLOGQ Tests - Ternary Logic Qword
// ============================================================================

#[test]
fn test_vpternlogq_xmm_basic() {
    // VPTERNLOGQ - Ternary logic operation on qwords (XMM)
    // EVEX.128.66.0F3A.W1 25 /r ib
    let code = [
        0x62, 0xF3, 0xED, 0x08, 0x25, 0xC2, 0xF0, // VPTERNLOGQ xmm0, xmm2, xmm2, 0xF0
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpternlogq_ymm_basic() {
    // VPTERNLOGQ - Ternary logic (YMM)
    let code = [
        0x62, 0xF3, 0xED, 0x28, 0x25, 0xC3, 0xAA, // VPTERNLOGQ ymm0, ymm2, ymm3, 0xAA
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpternlogq_zmm_basic() {
    // VPTERNLOGQ - Ternary logic (ZMM)
    let code = [
        0x62, 0xF3, 0xED, 0x48, 0x25, 0xC1, 0x96, // VPTERNLOGQ zmm0, zmm2, zmm1, 0x96
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpternlogq_zmm_memory() {
    // VPTERNLOGQ from memory
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x60, 0x00, 0x00, // MOV RAX, 0x6000
        0x62, 0xF3, 0xED, 0x48, 0x25, 0x00, 0xC0, // VPTERNLOGQ zmm0, zmm2, [rax], 0xC0
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// Combined Tests
// ============================================================================

#[test]
fn test_rotate_left_right_roundtrip() {
    // Rotate left then right should restore original
    let code = [
        0x62, 0xF1, 0x7D, 0x48, 0x72, 0xC8, 0x08, // VPROLD zmm1, zmm0, 8
        0x62, 0xF1, 0x75, 0x48, 0x72, 0xC1, 0x08, // VPRORD zmm0, zmm1, 8
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_align_and_rotate_combo() {
    // Combine alignment and rotation
    let code = [
        0x62, 0xF3, 0x7D, 0x48, 0x03, 0xC1, 0x02, // VALIGND zmm0, zmm0, zmm1, 2
        0x62, 0xF1, 0x7D, 0x48, 0x72, 0xC8, 0x04, // VPROLD zmm1, zmm0, 4
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_ternlog_complex_expression() {
    // Use ternary logic for complex boolean expression
    let code = [
        // (A & B) | (~A & C) - imm8 = 0xD8
        0x62, 0xF3, 0x6D, 0x48, 0x25, 0xC3, 0xD8, // VPTERNLOGD zmm0, zmm2, zmm3, 0xD8
        // Majority function: (A & B) | (B & C) | (A & C) - imm8 = 0xE8
        0x62, 0xF3, 0x65, 0x48, 0x25, 0xCA, 0xE8, // VPTERNLOGD zmm1, zmm3, zmm2, 0xE8
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_variable_rotation_pattern() {
    // Variable rotation with different amounts per element
    let code = [
        0x62, 0xF2, 0x7D, 0x48, 0x15, 0xC1, // VPROLVD zmm0, zmm0, zmm1
        0x62, 0xF2, 0x7D, 0x48, 0x14, 0xD2, // VPRORVD zmm2, zmm2, zmm2
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// Broadened EVEX coverage: value-asserting tests for integer arithmetic,
// logical ops, packed shifts, broadcasts and masked moves. These exercise
// exact element results, k-mask merge/zero semantics and VL upper-zeroing.
//
// Values are round-tripped through memory using VMOVDQU32 (load/store), the
// same technique used by the VADDPS ZMM value tests.
// ============================================================================
mod evex_broadened {
    use crate::common::*;
    use vm_memory::{Bytes, GuestAddress};

    const A_SRC1: u64 = 0x4000; // -> zmm1
    const A_SRC2: u64 = 0x4100; // -> zmm2
    const A_RES: u64 = 0x4200; // <- zmm0
    const A_POISON: u64 = 0x4300; // -> zmm0 preload / zmm14 poison

    // mov rax,A_SRC1 ; rbx,A_SRC2 ; rcx,A_RES ; rsi,A_POISON
    fn mov_addrs(out: &mut Vec<u8>) {
        out.extend_from_slice(&[0x48, 0xc7, 0xc0]);
        out.extend_from_slice(&(A_SRC1 as u32).to_le_bytes());
        out.extend_from_slice(&[0x48, 0xc7, 0xc3]);
        out.extend_from_slice(&(A_SRC2 as u32).to_le_bytes());
        out.extend_from_slice(&[0x48, 0xc7, 0xc1]);
        out.extend_from_slice(&(A_RES as u32).to_le_bytes());
        out.extend_from_slice(&[0x48, 0xc7, 0xc6]);
        out.extend_from_slice(&(A_POISON as u32).to_le_bytes());
    }

    // VMOVDQU32 zmm1,[rax] / zmm2,[rbx]
    const LD_Z1_RAX: [u8; 6] = [0x62, 0xf1, 0x7e, 0x48, 0x6f, 0x08];
    const LD_Z2_RBX: [u8; 6] = [0x62, 0xf1, 0x7e, 0x48, 0x6f, 0x13];
    // VMOVDQU32 [rcx],zmm0
    const ST_RCX_Z0: [u8; 6] = [0x62, 0xf1, 0x7e, 0x48, 0x7f, 0x01];

    fn run_d(code: Vec<u8>, src1: [u32; 16], src2: [u32; 16]) -> [u32; 16] {
        let (mut vcpu, mem) = setup_vm(&code, None);
        let mut b1 = [0u8; 64];
        let mut b2 = [0u8; 64];
        for i in 0..16 {
            b1[i * 4..i * 4 + 4].copy_from_slice(&src1[i].to_le_bytes());
            b2[i * 4..i * 4 + 4].copy_from_slice(&src2[i].to_le_bytes());
        }
        mem.write_slice(&b1, GuestAddress(A_SRC1)).unwrap();
        mem.write_slice(&b2, GuestAddress(A_SRC2)).unwrap();
        run_until_hlt(&mut vcpu).unwrap();
        let mut res = [0u8; 64];
        mem.read_slice(&mut res, GuestAddress(A_RES)).unwrap();
        let mut out = [0u32; 16];
        for i in 0..16 {
            out[i] =
                u32::from_le_bytes([res[i * 4], res[i * 4 + 1], res[i * 4 + 2], res[i * 4 + 3]]);
        }
        out
    }

    #[test]
    fn test_vpaddd_exact() {
        let mut code = Vec::new();
        mov_addrs(&mut code);
        code.extend_from_slice(&LD_Z1_RAX);
        code.extend_from_slice(&LD_Z2_RBX);
        code.extend_from_slice(&[0x62, 0xf1, 0x75, 0x48, 0xfe, 0xc2]); // vpaddd zmm0,zmm1,zmm2
        code.extend_from_slice(&ST_RCX_Z0);
        code.push(0xf4);
        let mut s1 = [0u32; 16];
        let mut s2 = [0u32; 16];
        for i in 0..16 {
            s1[i] = (i as u32).wrapping_mul(7).wrapping_add(3);
            s2[i] = 0xFFFF_FFF0u32.wrapping_add(i as u32); // forces wrap on some lanes
        }
        let out = run_d(code, s1, s2);
        for i in 0..16 {
            assert_eq!(out[i], s1[i].wrapping_add(s2[i]), "vpaddd lane {}", i);
        }
    }

    #[test]
    fn test_vpsubd_exact() {
        let mut code = Vec::new();
        mov_addrs(&mut code);
        code.extend_from_slice(&LD_Z1_RAX);
        code.extend_from_slice(&LD_Z2_RBX);
        code.extend_from_slice(&[0x62, 0xf1, 0x75, 0x48, 0xfa, 0xc2]); // vpsubd zmm0,zmm1,zmm2
        code.extend_from_slice(&ST_RCX_Z0);
        code.push(0xf4);
        let mut s1 = [0u32; 16];
        let mut s2 = [0u32; 16];
        for i in 0..16 {
            s1[i] = i as u32;
            s2[i] = (i as u32).wrapping_mul(13).wrapping_add(1); // some lanes underflow -> wrap
        }
        let out = run_d(code, s1, s2);
        for i in 0..16 {
            assert_eq!(out[i], s1[i].wrapping_sub(s2[i]), "vpsubd lane {}", i);
        }
    }

    #[test]
    fn test_vpmulld_exact() {
        let mut code = Vec::new();
        mov_addrs(&mut code);
        code.extend_from_slice(&LD_Z1_RAX);
        code.extend_from_slice(&LD_Z2_RBX);
        code.extend_from_slice(&[0x62, 0xf2, 0x75, 0x48, 0x40, 0xc2]); // vpmulld zmm0,zmm1,zmm2
        code.extend_from_slice(&ST_RCX_Z0);
        code.push(0xf4);
        let mut s1 = [0u32; 16];
        let mut s2 = [0u32; 16];
        for i in 0..16 {
            s1[i] = (i as u32).wrapping_sub(5); // includes "negative" values (wrap)
            s2[i] = (i as u32).wrapping_add(100000);
        }
        let out = run_d(code, s1, s2);
        for i in 0..16 {
            let exp = (s1[i] as i32).wrapping_mul(s2[i] as i32) as u32;
            assert_eq!(out[i], exp, "vpmulld lane {}", i);
        }
    }

    #[test]
    fn test_vpandd_vpord_vpxord_vpandnd_exact() {
        for (opcode, name) in [(0xdbu8, "and"), (0xeb, "or"), (0xef, "xor"), (0xdf, "andn")] {
            let mut code = Vec::new();
            mov_addrs(&mut code);
            code.extend_from_slice(&LD_Z1_RAX);
            code.extend_from_slice(&LD_Z2_RBX);
            code.extend_from_slice(&[0x62, 0xf1, 0x75, 0x48, opcode, 0xc2]);
            code.extend_from_slice(&ST_RCX_Z0);
            code.push(0xf4);
            let mut s1 = [0u32; 16];
            let mut s2 = [0u32; 16];
            for i in 0..16 {
                s1[i] = 0xA5A5_0000u32 | (i as u32);
                s2[i] = 0x0F0F_F0F0u32.wrapping_add((i as u32) << 8);
            }
            let out = run_d(code, s1, s2);
            for i in 0..16 {
                let exp = match name {
                    "and" => s1[i] & s2[i],
                    "or" => s1[i] | s2[i],
                    "xor" => s1[i] ^ s2[i],
                    _ => (!s1[i]) & s2[i], // andn: NOT(src1) AND src2
                };
                assert_eq!(out[i], exp, "vp{}d lane {}", name, i);
            }
        }
    }

    #[test]
    fn test_vpaddd_merge_masking() {
        // k1 = 0x00F0 -> lanes 4,5,6,7 active; others keep prior dest.
        let mut code = Vec::new();
        mov_addrs(&mut code);
        code.extend_from_slice(&[0xb8, 0xf0, 0x00, 0x00, 0x00]); // mov eax, 0xF0
        code.extend_from_slice(&[0xc5, 0xf8, 0x92, 0xc8]); // kmovw k1, eax
        // preload zmm0 (dest) from poison
        code.extend_from_slice(&[0x62, 0xf1, 0x7e, 0x48, 0x6f, 0x06]); // vmovdqu32 zmm0,[rsi]
        // reload rax (clobbered by 32-bit mov)
        code.extend_from_slice(&[0x48, 0xc7, 0xc0]);
        code.extend_from_slice(&(A_SRC1 as u32).to_le_bytes());
        code.extend_from_slice(&LD_Z1_RAX);
        code.extend_from_slice(&LD_Z2_RBX);
        code.extend_from_slice(&[0x62, 0xf1, 0x75, 0x49, 0xfe, 0xc2]); // vpaddd zmm0{k1},zmm1,zmm2
        code.extend_from_slice(&ST_RCX_Z0);
        code.push(0xf4);

        let (mut vcpu, mem) = setup_vm(&code, None);
        let mut b1 = [0u8; 64];
        let mut b2 = [0u8; 64];
        let mut poison = [0u8; 64];
        for i in 0..16 {
            b1[i * 4..i * 4 + 4].copy_from_slice(&(10u32 + i as u32).to_le_bytes());
            b2[i * 4..i * 4 + 4].copy_from_slice(&(100u32).to_le_bytes());
            poison[i * 4..i * 4 + 4].copy_from_slice(&(0xDEAD_0000u32 | i as u32).to_le_bytes());
        }
        mem.write_slice(&b1, GuestAddress(A_SRC1)).unwrap();
        mem.write_slice(&b2, GuestAddress(A_SRC2)).unwrap();
        mem.write_slice(&poison, GuestAddress(A_POISON)).unwrap();
        run_until_hlt(&mut vcpu).unwrap();
        let mut res = [0u8; 64];
        mem.read_slice(&mut res, GuestAddress(A_RES)).unwrap();
        for i in 0..16 {
            let got =
                u32::from_le_bytes([res[i * 4], res[i * 4 + 1], res[i * 4 + 2], res[i * 4 + 3]]);
            let active = (0xF0u64 >> i) & 1 != 0;
            let exp = if active {
                (10u32 + i as u32) + 100
            } else {
                0xDEAD_0000u32 | i as u32
            };
            assert_eq!(got, exp, "merge lane {} active={}", i, active);
        }
    }

    #[test]
    fn test_vpaddd_zeroing_masking() {
        // k1 = 0x00F0, {z} -> inactive lanes become 0.
        let mut code = Vec::new();
        mov_addrs(&mut code);
        code.extend_from_slice(&[0xb8, 0xf0, 0x00, 0x00, 0x00]); // mov eax, 0xF0
        code.extend_from_slice(&[0xc5, 0xf8, 0x92, 0xc8]); // kmovw k1, eax
        code.extend_from_slice(&[0x62, 0xf1, 0x7e, 0x48, 0x6f, 0x06]); // vmovdqu32 zmm0,[rsi]
        code.extend_from_slice(&[0x48, 0xc7, 0xc0]);
        code.extend_from_slice(&(A_SRC1 as u32).to_le_bytes());
        code.extend_from_slice(&LD_Z1_RAX);
        code.extend_from_slice(&LD_Z2_RBX);
        code.extend_from_slice(&[0x62, 0xf1, 0x75, 0xc9, 0xfe, 0xc2]); // vpaddd zmm0{k1}{z},zmm1,zmm2
        code.extend_from_slice(&ST_RCX_Z0);
        code.push(0xf4);

        let (mut vcpu, mem) = setup_vm(&code, None);
        let mut b1 = [0u8; 64];
        let mut b2 = [0u8; 64];
        let mut poison = [0u8; 64];
        for i in 0..16 {
            b1[i * 4..i * 4 + 4].copy_from_slice(&(10u32 + i as u32).to_le_bytes());
            b2[i * 4..i * 4 + 4].copy_from_slice(&(100u32).to_le_bytes());
            poison[i * 4..i * 4 + 4].copy_from_slice(&0xDEAD_BEEFu32.to_le_bytes());
        }
        mem.write_slice(&b1, GuestAddress(A_SRC1)).unwrap();
        mem.write_slice(&b2, GuestAddress(A_SRC2)).unwrap();
        mem.write_slice(&poison, GuestAddress(A_POISON)).unwrap();
        run_until_hlt(&mut vcpu).unwrap();
        let mut res = [0u8; 64];
        mem.read_slice(&mut res, GuestAddress(A_RES)).unwrap();
        for i in 0..16 {
            let got =
                u32::from_le_bytes([res[i * 4], res[i * 4 + 1], res[i * 4 + 2], res[i * 4 + 3]]);
            let active = (0xF0u64 >> i) & 1 != 0;
            let exp = if active { (10u32 + i as u32) + 100 } else { 0 };
            assert_eq!(got, exp, "zero lane {} active={}", i, active);
        }
    }

    #[test]
    fn test_vpaddd_ymm_vl_upper_zeroed() {
        // 256-bit VPADDD on zmm0's low half; high 256 bits (lanes 8..16) must be 0.
        let mut code = Vec::new();
        mov_addrs(&mut code);
        // First fill the whole zmm0 with poison so we can detect upper-zeroing.
        code.extend_from_slice(&[0x62, 0xf1, 0x7e, 0x48, 0x6f, 0x06]); // vmovdqu32 zmm0,[rsi]
        code.extend_from_slice(&[0x48, 0xc7, 0xc0]);
        code.extend_from_slice(&(A_SRC1 as u32).to_le_bytes());
        // load ymm1, ymm2 (256-bit) then 256-bit add into ymm0
        code.extend_from_slice(&[0x62, 0xf1, 0x7e, 0x28, 0x6f, 0x08]); // vmovdqu32 ymm1,[rax]
        code.extend_from_slice(&[0x62, 0xf1, 0x7e, 0x28, 0x6f, 0x13]); // vmovdqu32 ymm2,[rbx]
        code.extend_from_slice(&[0x62, 0xf1, 0x75, 0x28, 0xfe, 0xc2]); // vpaddd ymm0,ymm1,ymm2
        code.extend_from_slice(&ST_RCX_Z0); // store full zmm0
        code.push(0xf4);

        let (mut vcpu, mem) = setup_vm(&code, None);
        let mut b1 = [0u8; 64];
        let mut b2 = [0u8; 64];
        let poison = [0xAAu8; 64];
        for i in 0..16 {
            b1[i * 4..i * 4 + 4].copy_from_slice(&(i as u32).to_le_bytes());
            b2[i * 4..i * 4 + 4].copy_from_slice(&(1000u32).to_le_bytes());
        }
        mem.write_slice(&b1, GuestAddress(A_SRC1)).unwrap();
        mem.write_slice(&b2, GuestAddress(A_SRC2)).unwrap();
        mem.write_slice(&poison, GuestAddress(A_POISON)).unwrap();
        run_until_hlt(&mut vcpu).unwrap();
        let mut res = [0u8; 64];
        mem.read_slice(&mut res, GuestAddress(A_RES)).unwrap();
        for i in 0..8 {
            let got =
                u32::from_le_bytes([res[i * 4], res[i * 4 + 1], res[i * 4 + 2], res[i * 4 + 3]]);
            assert_eq!(got, (i as u32) + 1000, "ymm lane {}", i);
        }
        for i in 8..16 {
            let got =
                u32::from_le_bytes([res[i * 4], res[i * 4 + 1], res[i * 4 + 2], res[i * 4 + 3]]);
            assert_eq!(got, 0, "upper lane {} must be zeroed (VL=256)", i);
        }
    }

    #[test]
    fn test_vpslld_vpsrld_vpsrad_imm_exact() {
        // VPSLLD/VPSRLD/VPSRAD with imm8=3, dest=vvvv (zmm0), src=zmm1.
        for (bytes, kind) in [
            ([0x62u8, 0xf1, 0x7d, 0x48, 0x72, 0xf1, 0x03], "sll"),
            ([0x62, 0xf1, 0x7d, 0x48, 0x72, 0xd1, 0x03], "srl"),
            ([0x62, 0xf1, 0x7d, 0x48, 0x72, 0xe1, 0x03], "sra"),
        ] {
            let mut code = Vec::new();
            mov_addrs(&mut code);
            code.extend_from_slice(&LD_Z1_RAX); // zmm1 = src
            code.extend_from_slice(&bytes);
            code.extend_from_slice(&ST_RCX_Z0);
            code.push(0xf4);
            let (mut vcpu, mem) = setup_vm(&code, None);
            let mut b1 = [0u8; 64];
            let mut s1 = [0u32; 16];
            for i in 0..16 {
                // include a value with the top bit set to distinguish SRL vs SRA
                s1[i] = if i % 2 == 0 {
                    0x8000_0000u32 | (i as u32)
                } else {
                    (i as u32) * 0x101
                };
                b1[i * 4..i * 4 + 4].copy_from_slice(&s1[i].to_le_bytes());
            }
            mem.write_slice(&b1, GuestAddress(A_SRC1)).unwrap();
            run_until_hlt(&mut vcpu).unwrap();
            let mut res = [0u8; 64];
            mem.read_slice(&mut res, GuestAddress(A_RES)).unwrap();
            for i in 0..16 {
                let got = u32::from_le_bytes([
                    res[i * 4],
                    res[i * 4 + 1],
                    res[i * 4 + 2],
                    res[i * 4 + 3],
                ]);
                let exp = match kind {
                    "sll" => s1[i] << 3,
                    "srl" => s1[i] >> 3,
                    _ => ((s1[i] as i32) >> 3) as u32,
                };
                assert_eq!(got, exp, "vp{}d lane {}", kind, i);
            }
        }
    }

    #[test]
    fn test_vpslld_var_xmm_count() {
        // VPSLLD zmm0, zmm1, xmm2 ; count taken from low 64 bits of xmm2.
        let mut code = Vec::new();
        mov_addrs(&mut code);
        code.extend_from_slice(&LD_Z1_RAX); // zmm1 = src
        code.extend_from_slice(&LD_Z2_RBX); // zmm2 = count vector
        code.extend_from_slice(&[0x62, 0xf1, 0x75, 0x48, 0xf2, 0xc2]); // vpslld zmm0,zmm1,xmm2
        code.extend_from_slice(&ST_RCX_Z0);
        code.push(0xf4);
        let (mut vcpu, mem) = setup_vm(&code, None);
        let mut b1 = [0u8; 64];
        let mut b2 = [0u8; 64];
        let mut s1 = [0u32; 16];
        for i in 0..16 {
            s1[i] = (i as u32) + 1;
            b1[i * 4..i * 4 + 4].copy_from_slice(&s1[i].to_le_bytes());
        }
        // count = 5 in the low qword of xmm2
        b2[0..8].copy_from_slice(&5u64.to_le_bytes());
        mem.write_slice(&b1, GuestAddress(A_SRC1)).unwrap();
        mem.write_slice(&b2, GuestAddress(A_SRC2)).unwrap();
        run_until_hlt(&mut vcpu).unwrap();
        let mut res = [0u8; 64];
        mem.read_slice(&mut res, GuestAddress(A_RES)).unwrap();
        for i in 0..16 {
            let got =
                u32::from_le_bytes([res[i * 4], res[i * 4 + 1], res[i * 4 + 2], res[i * 4 + 3]]);
            assert_eq!(got, s1[i] << 5, "vpslld var lane {}", i);
        }
    }

    #[test]
    fn test_vpbroadcastd_exact() {
        // VPBROADCASTD zmm0, xmm1 ; xmm1 low dword broadcast to all 16 lanes.
        let mut code = Vec::new();
        mov_addrs(&mut code);
        code.extend_from_slice(&LD_Z1_RAX); // zmm1 source (only low dword used)
        code.extend_from_slice(&[0x62, 0xf2, 0x7d, 0x48, 0x58, 0xc1]); // vpbroadcastd zmm0,xmm1
        code.extend_from_slice(&ST_RCX_Z0);
        code.push(0xf4);
        let (mut vcpu, mem) = setup_vm(&code, None);
        let mut b1 = [0u8; 64];
        b1[0..4].copy_from_slice(&0xCAFEBABEu32.to_le_bytes());
        // poison the rest so we know only lane 0 is used as the source
        for i in 1..16 {
            b1[i * 4..i * 4 + 4].copy_from_slice(&(i as u32).to_le_bytes());
        }
        mem.write_slice(&b1, GuestAddress(A_SRC1)).unwrap();
        run_until_hlt(&mut vcpu).unwrap();
        let mut res = [0u8; 64];
        mem.read_slice(&mut res, GuestAddress(A_RES)).unwrap();
        for i in 0..16 {
            let got =
                u32::from_le_bytes([res[i * 4], res[i * 4 + 1], res[i * 4 + 2], res[i * 4 + 3]]);
            assert_eq!(got, 0xCAFEBABEu32, "broadcast lane {}", i);
        }
    }

    #[test]
    fn test_vmovdqu8_merge_masking() {
        // VMOVDQU8 zmm0{k1}, zmm1 with byte-granular mask; inactive bytes keep dest.
        let mut code = Vec::new();
        mov_addrs(&mut code);
        // k1 = 0x0F (low 4 bytes active)
        code.extend_from_slice(&[0xb8, 0x0f, 0x00, 0x00, 0x00]); // mov eax, 0x0F
        code.extend_from_slice(&[0xc5, 0xf8, 0x92, 0xc8]); // kmovw k1, eax
        code.extend_from_slice(&[0x62, 0xf1, 0x7e, 0x48, 0x6f, 0x06]); // vmovdqu32 zmm0,[rsi] (dest preload)
        code.extend_from_slice(&[0x48, 0xc7, 0xc0]);
        code.extend_from_slice(&(A_SRC1 as u32).to_le_bytes());
        code.extend_from_slice(&LD_Z1_RAX); // zmm1 = src
        code.extend_from_slice(&[0x62, 0xf1, 0x7f, 0x49, 0x6f, 0xc1]); // vmovdqu8 zmm0{k1},zmm1
        code.extend_from_slice(&ST_RCX_Z0);
        code.push(0xf4);
        let (mut vcpu, mem) = setup_vm(&code, None);
        let mut src = [0u8; 64];
        let mut poison = [0u8; 64];
        for i in 0..64 {
            src[i] = i as u8;
            poison[i] = 0xEE;
        }
        mem.write_slice(&src, GuestAddress(A_SRC1)).unwrap();
        mem.write_slice(&poison, GuestAddress(A_POISON)).unwrap();
        run_until_hlt(&mut vcpu).unwrap();
        let mut res = [0u8; 64];
        mem.read_slice(&mut res, GuestAddress(A_RES)).unwrap();
        for i in 0..64 {
            let active = i < 4; // k1=0x0F -> bytes 0..4
            let exp = if active { i as u8 } else { 0xEE };
            assert_eq!(res[i], exp, "vmovdqu8 byte {} active={}", i, active);
        }
    }

    #[test]
    fn test_vpaddb_exact_512() {
        // 64 byte-lanes; verify byte-granular wraparound add at VL=512.
        let mut code = Vec::new();
        mov_addrs(&mut code);
        code.extend_from_slice(&LD_Z1_RAX);
        code.extend_from_slice(&LD_Z2_RBX);
        code.extend_from_slice(&[0x62, 0xf1, 0x75, 0x48, 0xfc, 0xc2]); // vpaddb zmm0,zmm1,zmm2
        code.extend_from_slice(&ST_RCX_Z0);
        code.push(0xf4);
        let (mut vcpu, mem) = setup_vm(&code, None);
        let mut b1 = [0u8; 64];
        let mut b2 = [0u8; 64];
        for i in 0..64 {
            b1[i] = (i as u8).wrapping_mul(5);
            b2[i] = 0xF0u8.wrapping_add(i as u8); // forces 8-bit wrap on many lanes
        }
        mem.write_slice(&b1, GuestAddress(A_SRC1)).unwrap();
        mem.write_slice(&b2, GuestAddress(A_SRC2)).unwrap();
        run_until_hlt(&mut vcpu).unwrap();
        let mut res = [0u8; 64];
        mem.read_slice(&mut res, GuestAddress(A_RES)).unwrap();
        for i in 0..64 {
            assert_eq!(res[i], b1[i].wrapping_add(b2[i]), "vpaddb byte {}", i);
        }
    }

    #[test]
    fn test_vpmullw_and_vpaddq_exact() {
        // VPMULLW: 32 signed-word low-product lanes.
        let mut code = Vec::new();
        mov_addrs(&mut code);
        code.extend_from_slice(&LD_Z1_RAX);
        code.extend_from_slice(&LD_Z2_RBX);
        code.extend_from_slice(&[0x62, 0xf1, 0x75, 0x48, 0xd5, 0xc2]); // vpmullw zmm0,zmm1,zmm2
        code.extend_from_slice(&ST_RCX_Z0);
        code.push(0xf4);
        let (mut vcpu, mem) = setup_vm(&code, None);
        let mut b1 = [0u8; 64];
        let mut b2 = [0u8; 64];
        let mut w1 = [0i16; 32];
        let mut w2 = [0i16; 32];
        for i in 0..32 {
            w1[i] = (i as i16) - 16; // includes negatives
            w2[i] = (i as i16) * 300 + 1;
            b1[i * 2..i * 2 + 2].copy_from_slice(&w1[i].to_le_bytes());
            b2[i * 2..i * 2 + 2].copy_from_slice(&w2[i].to_le_bytes());
        }
        mem.write_slice(&b1, GuestAddress(A_SRC1)).unwrap();
        mem.write_slice(&b2, GuestAddress(A_SRC2)).unwrap();
        run_until_hlt(&mut vcpu).unwrap();
        let mut res = [0u8; 64];
        mem.read_slice(&mut res, GuestAddress(A_RES)).unwrap();
        for i in 0..32 {
            let got = i16::from_le_bytes([res[i * 2], res[i * 2 + 1]]);
            let exp = (w1[i] as i32).wrapping_mul(w2[i] as i32) as i16;
            assert_eq!(got, exp, "vpmullw word {}", i);
        }

        // VPADDQ: 8 qword lanes with wraparound.
        let mut code = Vec::new();
        mov_addrs(&mut code);
        code.extend_from_slice(&LD_Z1_RAX);
        code.extend_from_slice(&LD_Z2_RBX);
        code.extend_from_slice(&[0x62, 0xf1, 0xf5, 0x48, 0xd4, 0xc2]); // vpaddq zmm0,zmm1,zmm2
        code.extend_from_slice(&ST_RCX_Z0);
        code.push(0xf4);
        let (mut vcpu, mem) = setup_vm(&code, None);
        let mut b1 = [0u8; 64];
        let mut b2 = [0u8; 64];
        let mut q1 = [0u64; 8];
        let mut q2 = [0u64; 8];
        for i in 0..8 {
            q1[i] = 0xFFFF_FFFF_FFFF_FF00u64 + i as u64;
            q2[i] = 0x100u64 + (i as u64) * 7;
            b1[i * 8..i * 8 + 8].copy_from_slice(&q1[i].to_le_bytes());
            b2[i * 8..i * 8 + 8].copy_from_slice(&q2[i].to_le_bytes());
        }
        mem.write_slice(&b1, GuestAddress(A_SRC1)).unwrap();
        mem.write_slice(&b2, GuestAddress(A_SRC2)).unwrap();
        run_until_hlt(&mut vcpu).unwrap();
        let mut res = [0u8; 64];
        mem.read_slice(&mut res, GuestAddress(A_RES)).unwrap();
        for i in 0..8 {
            let got = u64::from_le_bytes([
                res[i * 8],
                res[i * 8 + 1],
                res[i * 8 + 2],
                res[i * 8 + 3],
                res[i * 8 + 4],
                res[i * 8 + 5],
                res[i * 8 + 6],
                res[i * 8 + 7],
            ]);
            assert_eq!(got, q1[i].wrapping_add(q2[i]), "vpaddq qword {}", i);
        }
    }
}
