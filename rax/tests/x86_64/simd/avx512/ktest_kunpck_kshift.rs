//! Tests for AVX-512 KTEST, KUNPCK, and KSHIFT Mask Operations.
//!
//! This module covers AVX-512 mask test, unpack, and shift operations.
//!
//! Instructions covered:
//! - KTESTW/B/Q/D - Test mask registers and set flags
//! - KORTESTW/B/Q/D - Test mask registers with OR and set flags
//! - KUNPCKBW/WD/DQ - Unpack and interleave mask registers
//! - KSHIFTLW/B/Q/D - Shift mask left
//! - KSHIFTRW/B/Q/D - Shift mask right
//!
//! These instructions are part of AVX512F, AVX512DQ, and AVX512BW extensions.
//!
//! References: Intel SDM Vol. 2, KTEST, KUNPCK, and KSHIFT instruction documentation

use crate::common::*;
use rax::cpu::Registers;

// ============================================================================
// KTESTW Tests - 16-bit Mask Test
// ============================================================================

#[test]
fn test_ktestw_basic() {
    // KTESTW - Test two 16-bit masks and set flags
    // VEX.L1.0F.W0 99 /r
    let code = [
        0xB8, 0xFF, 0xFF, 0x00, 0x00, // MOV EAX, 0xFFFF
        0xC5, 0xF8, 0x92, 0xC0, // KMOVW k0, eax
        0xC5, 0xF8, 0x92, 0xC8, // KMOVW k1, eax
        0xC4, 0xE1, 0xF8, 0x99, 0xC1, // KTESTW k0, k1
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_ktestw_zero_mask() {
    // Test with zero mask (should set ZF)
    let code = [
        0x31, 0xC0, // XOR EAX, EAX
        0xC5, 0xF8, 0x92, 0xC0, // KMOVW k0, eax
        0xB8, 0xFF, 0xFF, 0x00, 0x00, // MOV EAX, 0xFFFF
        0xC5, 0xF8, 0x92, 0xC8, // KMOVW k1, eax
        0xC4, 0xE1, 0xF8, 0x99, 0xC1, // KTESTW k0, k1
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_ktestw_partial_overlap() {
    // Test with partial overlap
    let code = [
        0xB8, 0xFF, 0x00, 0x00, 0x00, // MOV EAX, 0x00FF
        0xC5, 0xF8, 0x92, 0xC0, // KMOVW k0, eax
        0xB8, 0x00, 0xFF, 0x00, 0x00, // MOV EAX, 0xFF00
        0xC5, 0xF8, 0x92, 0xC8, // KMOVW k1, eax
        0xC4, 0xE1, 0xF8, 0x99, 0xC1, // KTESTW k0, k1
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// KTESTB Tests - 8-bit Mask Test
// ============================================================================

#[test]
fn test_ktestb_basic() {
    // KTESTB - Test two 8-bit masks
    // VEX.L1.66.0F.W0 99 /r
    let code = [
        0xB8, 0xFF, 0x00, 0x00, 0x00, // MOV EAX, 0xFF
        0xC5, 0xF9, 0x92, 0xC0, // KMOVB k0, eax
        0xC5, 0xF9, 0x92, 0xC8, // KMOVB k1, eax
        0xC5, 0xF9, 0x99, 0xC1, // KTESTB k0, k1
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_ktestb_no_overlap() {
    // Test with no overlap
    let code = [
        0xB8, 0x0F, 0x00, 0x00, 0x00, // MOV EAX, 0x0F
        0xC5, 0xF9, 0x92, 0xC0, // KMOVB k0, eax
        0xB8, 0xF0, 0x00, 0x00, 0x00, // MOV EAX, 0xF0
        0xC5, 0xF9, 0x92, 0xC8, // KMOVB k1, eax
        0xC5, 0xF9, 0x99, 0xC1, // KTESTB k0, k1
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// KTESTQ Tests - 64-bit Mask Test
// ============================================================================

#[test]
fn test_ktestq_basic() {
    // KTESTQ - Test two 64-bit masks
    // VEX.L1.0F.W1 99 /r
    let code = [
        0x48, 0xC7, 0xC0, 0xFF, 0xFF, 0xFF, 0xFF, // MOV RAX, -1
        0xC4, 0xE1, 0xF8, 0x92, 0xC0, // KMOVQ k0, rax
        0xC4, 0xE1, 0xF8, 0x92, 0xC8, // KMOVQ k1, rax
        0xC4, 0xE1, 0xF8, 0x99, 0xC1, // KTESTQ k0, k1
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// KTESTD Tests - 32-bit Mask Test
// ============================================================================

#[test]
fn test_ktestd_basic() {
    // KTESTD - Test two 32-bit masks
    // VEX.L1.66.0F.W1 99 /r
    let code = [
        0xB8, 0xFF, 0xFF, 0xFF, 0xFF, // MOV EAX, 0xFFFFFFFF
        0xC5, 0xFB, 0x92, 0xC0, // KMOVD k0, eax
        0xC5, 0xFB, 0x92, 0xC8, // KMOVD k1, eax
        0xC5, 0xFB, 0x99, 0xC1, // KTESTD k0, k1
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// KORTESTW Tests - 16-bit Mask OR Test
// ============================================================================

#[test]
fn test_kortestw_basic() {
    // KORTESTW - OR test two 16-bit masks and set flags
    // VEX.L1.0F.W0 98 /r
    let code = [
        0xB8, 0xFF, 0x00, 0x00, 0x00, // MOV EAX, 0x00FF
        0xC5, 0xF8, 0x92, 0xC0, // KMOVW k0, eax
        0xB8, 0x00, 0xFF, 0x00, 0x00, // MOV EAX, 0xFF00
        0xC5, 0xF8, 0x92, 0xC8, // KMOVW k1, eax
        0xC4, 0xE1, 0xF8, 0x98, 0xC1, // KORTESTW k0, k1
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_kortestw_all_zeros() {
    // Test with all zeros (should set ZF and CF)
    let code = [
        0x31, 0xC0, // XOR EAX, EAX
        0xC5, 0xF8, 0x92, 0xC0, // KMOVW k0, eax
        0xC5, 0xF8, 0x92, 0xC8, // KMOVW k1, eax
        0xC4, 0xE1, 0xF8, 0x98, 0xC1, // KORTESTW k0, k1
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// KORTESTB Tests - 8-bit Mask OR Test
// ============================================================================

#[test]
fn test_kortestb_basic() {
    // KORTESTB - OR test two 8-bit masks
    // VEX.L1.66.0F.W0 98 /r
    let code = [
        0xB8, 0x0F, 0x00, 0x00, 0x00, // MOV EAX, 0x0F
        0xC5, 0xF9, 0x92, 0xC0, // KMOVB k0, eax
        0xB8, 0xF0, 0x00, 0x00, 0x00, // MOV EAX, 0xF0
        0xC5, 0xF9, 0x92, 0xC8, // KMOVB k1, eax
        0xC5, 0xF9, 0x98, 0xC1, // KORTESTB k0, k1
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// KUNPCKBW Tests - Unpack Byte to Word
// ============================================================================

#[test]
fn test_kunpckbw_basic() {
    // KUNPCKBW - Unpack and interleave low bytes
    // VEX.L1.66.0F.W0 4B /r
    let code = [
        0xB8, 0x0F, 0x00, 0x00, 0x00, // MOV EAX, 0x0F
        0xC5, 0xF9, 0x92, 0xC0, // KMOVB k0, eax
        0xB8, 0xF0, 0x00, 0x00, 0x00, // MOV EAX, 0xF0
        0xC5, 0xF9, 0x92, 0xC8, // KMOVB k1, eax
        0xC5, 0xF5, 0x4B, 0xD0, // KUNPCKBW k2, k0, k1
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_kunpckbw_pattern() {
    // Test with specific patterns
    let code = [
        0xB8, 0xAA, 0x00, 0x00, 0x00, // MOV EAX, 0xAA
        0xC5, 0xF9, 0x92, 0xC0, // KMOVB k0, eax
        0xB8, 0x55, 0x00, 0x00, 0x00, // MOV EAX, 0x55
        0xC5, 0xF9, 0x92, 0xC8, // KMOVB k1, eax
        0xC5, 0xF5, 0x4B, 0xD0, // KUNPCKBW k2, k0, k1
        0xC5, 0xF8, 0x93, 0xC2, // KMOVW eax, k2
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// KUNPCKWD Tests - Unpack Word to Dword
// ============================================================================

#[test]
fn test_kunpckwd_basic() {
    // KUNPCKWD - Unpack and interleave low words
    // VEX.L1.0F.W0 4B /r
    let code = [
        0xB8, 0xFF, 0x00, 0x00, 0x00, // MOV EAX, 0x00FF
        0xC5, 0xF8, 0x92, 0xC0, // KMOVW k0, eax
        0xB8, 0x00, 0xFF, 0x00, 0x00, // MOV EAX, 0xFF00
        0xC5, 0xF8, 0x92, 0xC8, // KMOVW k1, eax
        0xC4, 0xE1, 0xF4, 0x4B, 0xD0, // KUNPCKWD k2, k0, k1
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// KUNPCKDQ Tests - Unpack Dword to Qword
// ============================================================================

#[test]
fn test_kunpckdq_basic() {
    // KUNPCKDQ - Unpack and interleave low dwords
    // VEX.L1.0F.W1 4B /r
    let code = [
        0xB8, 0xFF, 0xFF, 0x00, 0x00, // MOV EAX, 0xFFFF
        0xC5, 0xFB, 0x92, 0xC0, // KMOVD k0, eax
        0xB8, 0x00, 0x00, 0xFF, 0xFF, // MOV EAX, 0xFFFF0000
        0xC5, 0xFB, 0x92, 0xC8, // KMOVD k1, eax
        0xC4, 0xE1, 0xF4, 0x4B, 0xD0, // KUNPCKDQ k2, k0, k1
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// KSHIFTLW Tests - Shift Left 16-bit Mask
// ============================================================================

#[test]
fn test_kshiftlw_basic() {
    // KSHIFTLW - Shift left 16-bit mask
    // VEX.L1.66.0F3A.W1 32 /r ib
    let code = [
        0xB8, 0xFF, 0x00, 0x00, 0x00, // MOV EAX, 0x00FF
        0xC5, 0xF8, 0x92, 0xC0, // KMOVW k0, eax
        0xC4, 0xE3, 0xF9, 0x32, 0xC8, 0x04, // KSHIFTLW k1, k0, 4
        0xC5, 0xF8, 0x93, 0xC1, // KMOVW eax, k1
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_kshiftlw_by_8() {
    // Shift left by 8 bits
    let code = [
        0xB8, 0xFF, 0x00, 0x00, 0x00, // MOV EAX, 0x00FF
        0xC5, 0xF8, 0x92, 0xC0, // KMOVW k0, eax
        0xC4, 0xE3, 0xF9, 0x32, 0xC8, 0x08, // KSHIFTLW k1, k0, 8
        0xC5, 0xF8, 0x93, 0xC1, // KMOVW eax, k1
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_kshiftlw_overflow() {
    // Shift left beyond register size
    let code = [
        0xB8, 0xFF, 0xFF, 0x00, 0x00, // MOV EAX, 0xFFFF
        0xC5, 0xF8, 0x92, 0xC0, // KMOVW k0, eax
        0xC4, 0xE3, 0xF9, 0x32, 0xC8, 0x10, // KSHIFTLW k1, k0, 16
        0xC5, 0xF8, 0x93, 0xC1, // KMOVW eax, k1
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// KSHIFTLB Tests - Shift Left 8-bit Mask
// ============================================================================

#[test]
fn test_kshiftlb_basic() {
    // KSHIFTLB - Shift left 8-bit mask
    // VEX.L1.66.0F3A.W0 32 /r ib
    let code = [
        0xB8, 0x0F, 0x00, 0x00, 0x00, // MOV EAX, 0x0F
        0xC5, 0xF9, 0x92, 0xC0, // KMOVB k0, eax
        0xC4, 0xE3, 0x79, 0x32, 0xC8, 0x04, // KSHIFTLB k1, k0, 4
        0xC5, 0xF9, 0x93, 0xC1, // KMOVB eax, k1
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// KSHIFTRW Tests - Shift Right 16-bit Mask
// ============================================================================

#[test]
fn test_kshiftrw_basic() {
    // KSHIFTRW - Shift right 16-bit mask
    // VEX.L1.66.0F3A.W1 30 /r ib
    let code = [
        0xB8, 0x00, 0xFF, 0x00, 0x00, // MOV EAX, 0xFF00
        0xC5, 0xF8, 0x92, 0xC0, // KMOVW k0, eax
        0xC4, 0xE3, 0xF9, 0x30, 0xC8, 0x04, // KSHIFTRW k1, k0, 4
        0xC5, 0xF8, 0x93, 0xC1, // KMOVW eax, k1
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_kshiftrw_by_8() {
    // Shift right by 8 bits
    let code = [
        0xB8, 0x00, 0xFF, 0x00, 0x00, // MOV EAX, 0xFF00
        0xC5, 0xF8, 0x92, 0xC0, // KMOVW k0, eax
        0xC4, 0xE3, 0xF9, 0x30, 0xC8, 0x08, // KSHIFTRW k1, k0, 8
        0xC5, 0xF8, 0x93, 0xC1, // KMOVW eax, k1
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// KSHIFTRB Tests - Shift Right 8-bit Mask
// ============================================================================

#[test]
fn test_kshiftrb_basic() {
    // KSHIFTRB - Shift right 8-bit mask
    // VEX.L1.66.0F3A.W0 30 /r ib
    let code = [
        0xB8, 0xF0, 0x00, 0x00, 0x00, // MOV EAX, 0xF0
        0xC5, 0xF9, 0x92, 0xC0, // KMOVB k0, eax
        0xC4, 0xE3, 0x79, 0x30, 0xC8, 0x04, // KSHIFTRB k1, k0, 4
        0xC5, 0xF9, 0x93, 0xC1, // KMOVB eax, k1
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// KSHIFTLD Tests - Shift Left 32-bit Mask
// ============================================================================

#[test]
fn test_kshiftld_basic() {
    // KSHIFTLD - Shift left 32-bit mask
    // VEX.L1.66.0F3A.W0 33 /r ib
    let code = [
        0xB8, 0xFF, 0xFF, 0x00, 0x00, // MOV EAX, 0xFFFF
        0xC5, 0xFB, 0x92, 0xC0, // KMOVD k0, eax
        0xC4, 0xE3, 0x79, 0x33, 0xC8, 0x08, // KSHIFTLD k1, k0, 8
        0xC5, 0xFB, 0x93, 0xC1, // KMOVD eax, k1
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// KSHIFTRD Tests - Shift Right 32-bit Mask
// ============================================================================

#[test]
fn test_kshiftrd_basic() {
    // KSHIFTRD - Shift right 32-bit mask
    // VEX.L1.66.0F3A.W0 31 /r ib
    let code = [
        0xB8, 0x00, 0x00, 0xFF, 0xFF, // MOV EAX, 0xFFFF0000
        0xC5, 0xFB, 0x92, 0xC0, // KMOVD k0, eax
        0xC4, 0xE3, 0x79, 0x31, 0xC8, 0x08, // KSHIFTRD k1, k0, 8
        0xC5, 0xFB, 0x93, 0xC1, // KMOVD eax, k1
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// KSHIFTLQ Tests - Shift Left 64-bit Mask
// ============================================================================

#[test]
fn test_kshiftlq_basic() {
    // KSHIFTLQ - Shift left 64-bit mask
    // VEX.L1.66.0F3A.W1 33 /r ib
    let code = [
        0x48, 0xB8, 0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0x00, // MOV RAX, 0xFFFFFFFF
        0xC4, 0xE1, 0xF8, 0x92, 0xC0, // KMOVQ k0, rax
        0xC4, 0xE3, 0xF9, 0x33, 0xC8, 0x10, // KSHIFTLQ k1, k0, 16
        0xC4, 0xE1, 0xF8, 0x93, 0xC1, // KMOVQ rax, k1
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// KSHIFTRQ Tests - Shift Right 64-bit Mask
// ============================================================================

#[test]
fn test_kshiftrq_basic() {
    // KSHIFTRQ - Shift right 64-bit mask
    // VEX.L1.66.0F3A.W1 31 /r ib
    let code = [
        0x48, 0xB8, 0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF,
        0xFF, // MOV RAX, 0xFFFFFFFF00000000
        0xC4, 0xE1, 0xF8, 0x92, 0xC0, // KMOVQ k0, rax
        0xC4, 0xE3, 0xF9, 0x31, 0xC8, 0x10, // KSHIFTRQ k1, k0, 16
        0xC4, 0xE1, 0xF8, 0x93, 0xC1, // KMOVQ rax, k1
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// Combined Tests
// ============================================================================

#[test]
fn test_shift_and_test_combo() {
    // Combine shift and test operations
    let code = [
        0xB8, 0xFF, 0x00, 0x00, 0x00, // MOV EAX, 0x00FF
        0xC5, 0xF8, 0x92, 0xC0, // KMOVW k0, eax
        0xC4, 0xE3, 0xF9, 0x32, 0xC8, 0x08, // KSHIFTLW k1, k0, 8
        0xC4, 0xE1, 0xF8, 0x99, 0xC1, // KTESTW k0, k1
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_unpack_and_shift() {
    // Combine unpack and shift operations
    let code = [
        0xB8, 0x0F, 0x00, 0x00, 0x00, // MOV EAX, 0x0F
        0xC5, 0xF9, 0x92, 0xC0, // KMOVB k0, eax
        0xB8, 0xF0, 0x00, 0x00, 0x00, // MOV EAX, 0xF0
        0xC5, 0xF9, 0x92, 0xC8, // KMOVB k1, eax
        0xC5, 0xF5, 0x4B, 0xD0, // KUNPCKBW k2, k0, k1
        0xC4, 0xE3, 0xF9, 0x32, 0xDA, 0x04, // KSHIFTLW k3, k2, 4
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_bidirectional_shifts() {
    // Test shifting left then right
    let code = [
        0xB8, 0xFF, 0x0F, 0x00, 0x00, // MOV EAX, 0x0FFF
        0xC5, 0xF8, 0x92, 0xC0, // KMOVW k0, eax
        0xC4, 0xE3, 0xF9, 0x32, 0xC8, 0x04, // KSHIFTLW k1, k0, 4
        0xC4, 0xE3, 0xF9, 0x30, 0xD1, 0x04, // KSHIFTRW k2, k1, 4
        0xC5, 0xF8, 0x93, 0xC2, // KMOVW eax, k2
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}
