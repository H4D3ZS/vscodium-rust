//! Tests for AVX-512 Specialized Instructions.
//!
//! This module covers specialized AVX-512 operations including sum-of-absolute-differences,
//! leading zero count, and double-precision shift operations.
//!
//! Instructions covered:
//! - VDBPSADBW - Double Block Packed Sum-Absolute-Differences on unsigned bytes
//! - VPLZCNTD - Count leading zero bits in dwords
//! - VPLZCNTQ - Count leading zero bits in qwords
//! - VPSHLDW - Concatenate and shift packed words left
//! - VPSHLDD - Concatenate and shift packed dwords left
//! - VPSHLDQ - Concatenate and shift packed qwords left
//! - VPSHLDVW - Variable shift packed words left
//! - VPSHLDVD - Variable shift packed dwords left
//! - VPSHLDVQ - Variable shift packed qwords left
//! - VPSHRDW - Concatenate and shift packed words right
//! - VPSHRDD - Concatenate and shift packed dwords right
//! - VPSHRDQ - Concatenate and shift packed qwords right
//! - VPSHRDVW - Variable shift packed words right
//! - VPSHRDVD - Variable shift packed dwords right
//! - VPSHRDVQ - Variable shift packed qwords right
//!
//! These instructions are part of AVX512BW, AVX512CD, and AVX512VBMI2 extensions.
//!
//! References: Intel SDM Vol. 2, AVX-512 specialized instruction documentation

use crate::common::*;
use rax::cpu::Registers;

// ============================================================================
// VDBPSADBW Tests - Double Block Packed SAD
// ============================================================================

#[test]
fn test_vdbpsadbw_xmm_basic() {
    // VDBPSADBW - Compute packed SAD on unsigned bytes (XMM)
    // EVEX.128.66.0F3A.W0 42 /r ib
    let code = [
        0x62, 0xF3, 0x6D, 0x08, 0x42, 0xC1, 0x00, // VDBPSADBW xmm0, xmm2, xmm1, 0
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vdbpsadbw_ymm_basic() {
    // VDBPSADBW - Compute packed SAD (YMM)
    // EVEX.256.66.0F3A.W0 42 /r ib
    let code = [
        0x62, 0xF3, 0x6D, 0x28, 0x42, 0xC2, 0xAA, // VDBPSADBW ymm0, ymm2, ymm2, 0xAA
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vdbpsadbw_zmm_basic() {
    // VDBPSADBW - Compute packed SAD (ZMM)
    // EVEX.512.66.0F3A.W0 42 /r ib
    let code = [
        0x62, 0xF3, 0x6D, 0x48, 0x42, 0xC3, 0x55, // VDBPSADBW zmm0, zmm2, zmm3, 0x55
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vdbpsadbw_zmm_memory() {
    // VDBPSADBW from memory
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x10, 0x00, 0x00, // MOV RAX, 0x1000
        0x62, 0xF3, 0x6D, 0x48, 0x42, 0x00, 0xFF, // VDBPSADBW zmm0, zmm2, [rax], 0xFF
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vdbpsadbw_shuffle_control() {
    // Test different shuffle control values
    let code = [
        // imm8 = 0xE4 (11 10 01 00)
        0x62, 0xF3, 0x6D, 0x48, 0x42, 0xC1, 0xE4, // VDBPSADBW zmm0, zmm2, zmm1, 0xE4
        // imm8 = 0x1B (00 01 10 11)
        0x62, 0xF3, 0x65, 0x48, 0x42, 0xCA, 0x1B, // VDBPSADBW zmm1, zmm3, zmm2, 0x1B
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vdbpsadbw_identity_shuffle() {
    // Test with identity shuffle (0xE4 = 11 10 01 00)
    let code = [
        0x62, 0xF3, 0x6D, 0x48, 0x42, 0xC1, 0xE4, // VDBPSADBW zmm0, zmm2, zmm1, 0xE4
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// VPLZCNTD Tests - Count Leading Zeros Dword
// ============================================================================

#[test]
fn test_vplzcntd_xmm_basic() {
    // VPLZCNTD - Count leading zero bits in dwords (XMM)
    // EVEX.128.66.0F38.W0 44 /r
    let code = [
        0x62, 0xF2, 0x7D, 0x08, 0x44, 0xC1, // VPLZCNTD xmm0, xmm1
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vplzcntd_ymm_basic() {
    // VPLZCNTD - Count leading zeros (YMM)
    let code = [
        0x62, 0xF2, 0x7D, 0x28, 0x44, 0xC2, // VPLZCNTD ymm0, ymm2
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vplzcntd_zmm_basic() {
    // VPLZCNTD - Count leading zeros (ZMM)
    let code = [
        0x62, 0xF2, 0x7D, 0x48, 0x44, 0xC3, // VPLZCNTD zmm0, zmm3
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vplzcntd_zmm_memory() {
    // VPLZCNTD from memory
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x20, 0x00, 0x00, // MOV RAX, 0x2000
        0x62, 0xF2, 0x7D, 0x48, 0x44, 0x00, // VPLZCNTD zmm0, [rax]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vplzcntd_broadcast() {
    // VPLZCNTD with broadcast
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x30, 0x00, 0x00, // MOV RAX, 0x3000
        0x62, 0xF2, 0x7D, 0x58, 0x44, 0x00, // VPLZCNTD zmm0, dword ptr [rax]{1to16}
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// VPLZCNTQ Tests - Count Leading Zeros Qword
// ============================================================================

#[test]
fn test_vplzcntq_xmm_basic() {
    // VPLZCNTQ - Count leading zero bits in qwords (XMM)
    // EVEX.128.66.0F38.W1 44 /r
    let code = [
        0x62, 0xF2, 0xFD, 0x08, 0x44, 0xC1, // VPLZCNTQ xmm0, xmm1
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vplzcntq_ymm_basic() {
    // VPLZCNTQ - Count leading zeros (YMM)
    let code = [
        0x62, 0xF2, 0xFD, 0x28, 0x44, 0xC2, // VPLZCNTQ ymm0, ymm2
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vplzcntq_zmm_basic() {
    // VPLZCNTQ - Count leading zeros (ZMM)
    let code = [
        0x62, 0xF2, 0xFD, 0x48, 0x44, 0xC3, // VPLZCNTQ zmm0, zmm3
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vplzcntq_zmm_memory() {
    // VPLZCNTQ from memory
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x40, 0x00, 0x00, // MOV RAX, 0x4000
        0x62, 0xF2, 0xFD, 0x48, 0x44, 0x00, // VPLZCNTQ zmm0, [rax]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// VPSHLDW Tests - Shift Left Words
// ============================================================================

#[test]
fn test_vpshldw_xmm_basic() {
    // VPSHLDW - Concatenate and shift left words (XMM)
    // EVEX.128.66.0F3A.W1 70 /r ib
    let code = [
        0x62, 0xF3, 0xED, 0x08, 0x70, 0xC1, 0x08, // VPSHLDW xmm0, xmm2, xmm1, 8
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpshldw_ymm_basic() {
    // VPSHLDW - Shift left words (YMM)
    let code = [
        0x62, 0xF3, 0xED, 0x28, 0x70, 0xC2, 0x04, // VPSHLDW ymm0, ymm2, ymm2, 4
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpshldw_zmm_basic() {
    // VPSHLDW - Shift left words (ZMM)
    let code = [
        0x62, 0xF3, 0xED, 0x48, 0x70, 0xC3, 0x0C, // VPSHLDW zmm0, zmm2, zmm3, 12
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// VPSHLDD Tests - Shift Left Dwords
// ============================================================================

#[test]
fn test_vpshldd_xmm_basic() {
    // VPSHLDD - Concatenate and shift left dwords (XMM)
    // EVEX.128.66.0F3A.W0 71 /r ib
    let code = [
        0x62, 0xF3, 0x6D, 0x08, 0x71, 0xC1, 0x08, // VPSHLDD xmm0, xmm2, xmm1, 8
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpshldd_ymm_basic() {
    // VPSHLDD - Shift left dwords (YMM)
    let code = [
        0x62, 0xF3, 0x6D, 0x28, 0x71, 0xC2, 0x10, // VPSHLDD ymm0, ymm2, ymm2, 16
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpshldd_zmm_basic() {
    // VPSHLDD - Shift left dwords (ZMM)
    let code = [
        0x62, 0xF3, 0x6D, 0x48, 0x71, 0xC3, 0x04, // VPSHLDD zmm0, zmm2, zmm3, 4
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpshldd_zmm_memory() {
    // VPSHLDD from memory
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x50, 0x00, 0x00, // MOV RAX, 0x5000
        0x62, 0xF3, 0x6D, 0x48, 0x71, 0x00, 0x10, // VPSHLDD zmm0, zmm2, [rax], 16
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// VPSHLDQ Tests - Shift Left Qwords
// ============================================================================

#[test]
fn test_vpshldq_xmm_basic() {
    // VPSHLDQ - Concatenate and shift left qwords (XMM)
    // EVEX.128.66.0F3A.W1 71 /r ib
    let code = [
        0x62, 0xF3, 0xED, 0x08, 0x71, 0xC1, 0x10, // VPSHLDQ xmm0, xmm2, xmm1, 16
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpshldq_ymm_basic() {
    // VPSHLDQ - Shift left qwords (YMM)
    let code = [
        0x62, 0xF3, 0xED, 0x28, 0x71, 0xC2, 0x20, // VPSHLDQ ymm0, ymm2, ymm2, 32
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpshldq_zmm_basic() {
    // VPSHLDQ - Shift left qwords (ZMM)
    let code = [
        0x62, 0xF3, 0xED, 0x48, 0x71, 0xC3, 0x08, // VPSHLDQ zmm0, zmm2, zmm3, 8
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// VPSHLDVW Tests - Variable Shift Left Words
// ============================================================================

#[test]
fn test_vpshldvw_xmm_basic() {
    // VPSHLDVW - Variable shift left words (XMM)
    // EVEX.128.66.0F38.W1 70 /r
    let code = [
        0x62, 0xF2, 0xED, 0x08, 0x70, 0xC1, // VPSHLDVW xmm0, xmm2, xmm1
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpshldvw_ymm_basic() {
    // VPSHLDVW - Variable shift left words (YMM)
    let code = [
        0x62, 0xF2, 0xED, 0x28, 0x70, 0xC2, // VPSHLDVW ymm0, ymm2, ymm2
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpshldvw_zmm_basic() {
    // VPSHLDVW - Variable shift left words (ZMM)
    let code = [
        0x62, 0xF2, 0xED, 0x48, 0x70, 0xC3, // VPSHLDVW zmm0, zmm2, zmm3
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// VPSHLDVD Tests - Variable Shift Left Dwords
// ============================================================================

#[test]
fn test_vpshldvd_xmm_basic() {
    // VPSHLDVD - Variable shift left dwords (XMM)
    // EVEX.128.66.0F38.W0 71 /r
    let code = [
        0x62, 0xF2, 0x6D, 0x08, 0x71, 0xC1, // VPSHLDVD xmm0, xmm2, xmm1
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpshldvd_ymm_basic() {
    // VPSHLDVD - Variable shift left dwords (YMM)
    let code = [
        0x62, 0xF2, 0x6D, 0x28, 0x71, 0xC2, // VPSHLDVD ymm0, ymm2, ymm2
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpshldvd_zmm_basic() {
    // VPSHLDVD - Variable shift left dwords (ZMM)
    let code = [
        0x62, 0xF2, 0x6D, 0x48, 0x71, 0xC3, // VPSHLDVD zmm0, zmm2, zmm3
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// VPSHLDVQ Tests - Variable Shift Left Qwords
// ============================================================================

#[test]
fn test_vpshldvq_xmm_basic() {
    // VPSHLDVQ - Variable shift left qwords (XMM)
    // EVEX.128.66.0F38.W1 71 /r
    let code = [
        0x62, 0xF2, 0xED, 0x08, 0x71, 0xC1, // VPSHLDVQ xmm0, xmm2, xmm1
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpshldvq_ymm_basic() {
    // VPSHLDVQ - Variable shift left qwords (YMM)
    let code = [
        0x62, 0xF2, 0xED, 0x28, 0x71, 0xC2, // VPSHLDVQ ymm0, ymm2, ymm2
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpshldvq_zmm_basic() {
    // VPSHLDVQ - Variable shift left qwords (ZMM)
    let code = [
        0x62, 0xF2, 0xED, 0x48, 0x71, 0xC3, // VPSHLDVQ zmm0, zmm2, zmm3
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// VPSHRDW Tests - Shift Right Words
// ============================================================================

#[test]
fn test_vpshrdw_xmm_basic() {
    // VPSHRDW - Concatenate and shift right words (XMM)
    // EVEX.128.66.0F3A.W1 72 /r ib
    let code = [
        0x62, 0xF3, 0xED, 0x08, 0x72, 0xC1, 0x08, // VPSHRDW xmm0, xmm2, xmm1, 8
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpshrdw_ymm_basic() {
    // VPSHRDW - Shift right words (YMM)
    let code = [
        0x62, 0xF3, 0xED, 0x28, 0x72, 0xC2, 0x04, // VPSHRDW ymm0, ymm2, ymm2, 4
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpshrdw_zmm_basic() {
    // VPSHRDW - Shift right words (ZMM)
    let code = [
        0x62, 0xF3, 0xED, 0x48, 0x72, 0xC3, 0x0C, // VPSHRDW zmm0, zmm2, zmm3, 12
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// VPSHRDD Tests - Shift Right Dwords
// ============================================================================

#[test]
fn test_vpshrdd_xmm_basic() {
    // VPSHRDD - Concatenate and shift right dwords (XMM)
    // EVEX.128.66.0F3A.W0 73 /r ib
    let code = [
        0x62, 0xF3, 0x6D, 0x08, 0x73, 0xC1, 0x08, // VPSHRDD xmm0, xmm2, xmm1, 8
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpshrdd_ymm_basic() {
    // VPSHRDD - Shift right dwords (YMM)
    let code = [
        0x62, 0xF3, 0x6D, 0x28, 0x73, 0xC2, 0x10, // VPSHRDD ymm0, ymm2, ymm2, 16
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpshrdd_zmm_basic() {
    // VPSHRDD - Shift right dwords (ZMM)
    let code = [
        0x62, 0xF3, 0x6D, 0x48, 0x73, 0xC3, 0x04, // VPSHRDD zmm0, zmm2, zmm3, 4
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// VPSHRDQ Tests - Shift Right Qwords
// ============================================================================

#[test]
fn test_vpshrdq_xmm_basic() {
    // VPSHRDQ - Concatenate and shift right qwords (XMM)
    // EVEX.128.66.0F3A.W1 73 /r ib
    let code = [
        0x62, 0xF3, 0xED, 0x08, 0x73, 0xC1, 0x10, // VPSHRDQ xmm0, xmm2, xmm1, 16
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpshrdq_ymm_basic() {
    // VPSHRDQ - Shift right qwords (YMM)
    let code = [
        0x62, 0xF3, 0xED, 0x28, 0x73, 0xC2, 0x20, // VPSHRDQ ymm0, ymm2, ymm2, 32
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpshrdq_zmm_basic() {
    // VPSHRDQ - Shift right qwords (ZMM)
    let code = [
        0x62, 0xF3, 0xED, 0x48, 0x73, 0xC3, 0x08, // VPSHRDQ zmm0, zmm2, zmm3, 8
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// VPSHRDVW Tests - Variable Shift Right Words
// ============================================================================

#[test]
fn test_vpshrdvw_zmm_basic() {
    // VPSHRDVW - Variable shift right words (ZMM)
    // EVEX.512.66.0F38.W1 72 /r
    let code = [
        0x62, 0xF2, 0xED, 0x48, 0x72, 0xC3, // VPSHRDVW zmm0, zmm2, zmm3
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// VPSHRDVD Tests - Variable Shift Right Dwords
// ============================================================================

#[test]
fn test_vpshrdvd_zmm_basic() {
    // VPSHRDVD - Variable shift right dwords (ZMM)
    // EVEX.512.66.0F38.W0 73 /r
    let code = [
        0x62, 0xF2, 0x6D, 0x48, 0x73, 0xC3, // VPSHRDVD zmm0, zmm2, zmm3
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// VPSHRDVQ Tests - Variable Shift Right Qwords
// ============================================================================

#[test]
fn test_vpshrdvq_zmm_basic() {
    // VPSHRDVQ - Variable shift right qwords (ZMM)
    // EVEX.512.66.0F38.W1 73 /r
    let code = [
        0x62, 0xF2, 0xED, 0x48, 0x73, 0xC3, // VPSHRDVQ zmm0, zmm2, zmm3
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// Combined Tests
// ============================================================================

#[test]
fn test_shift_left_right_roundtrip() {
    // Shift left then right should restore original
    let code = [
        0x62, 0xF3, 0x6D, 0x48, 0x71, 0xC1, 0x10, // VPSHLDD zmm0, zmm2, zmm1, 16
        0x62, 0xF3, 0x75, 0x48, 0x73, 0xC8, 0x10, // VPSHRDD zmm1, zmm0, zmm0, 16
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_lzcnt_sad_combo() {
    // Combine leading zero count with SAD operation
    let code = [
        0x62, 0xF2, 0x7D, 0x48, 0x44, 0xC1, // VPLZCNTD zmm0, zmm1
        0x62, 0xF3, 0x7D, 0x48, 0x42, 0xD2, 0xE4, // VDBPSADBW zmm2, zmm0, zmm2, 0xE4
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// Broadened EVEX coverage: value-asserting tests for the integer compare-into-
// mask instructions (VPCMPEQ*/VPCMPGT* fixed forms and the VPCMP[U]* imm8 forms).
// The k-destination is read directly from the returned register state.
// ============================================================================
mod evex_cmp_mask {
    use crate::common::*;
    use vm_memory::{Bytes, GuestAddress};

    const C_SRC1: u64 = 0x4800; // -> zmm1
    const C_SRC2: u64 = 0x4900; // -> zmm2

    fn mov_addrs(out: &mut Vec<u8>) {
        out.extend_from_slice(&[0x48, 0xc7, 0xc0]); // mov rax, C_SRC1
        out.extend_from_slice(&(C_SRC1 as u32).to_le_bytes());
        out.extend_from_slice(&[0x48, 0xc7, 0xc3]); // mov rbx, C_SRC2
        out.extend_from_slice(&(C_SRC2 as u32).to_le_bytes());
    }

    const LD_Z1_RAX: [u8; 6] = [0x62, 0xf1, 0x7e, 0x48, 0x6f, 0x08]; // vmovdqu32 zmm1,[rax]
    const LD_Z2_RBX: [u8; 6] = [0x62, 0xf1, 0x7e, 0x48, 0x6f, 0x13]; // vmovdqu32 zmm2,[rbx]

    fn run_cmp(extra: &[u8], s1: [u32; 16], s2: [u32; 16]) -> [u64; 8] {
        let mut code = Vec::new();
        mov_addrs(&mut code);
        code.extend_from_slice(&LD_Z1_RAX);
        code.extend_from_slice(&LD_Z2_RBX);
        code.extend_from_slice(extra);
        code.push(0xf4);
        let (mut vcpu, mem) = setup_vm(&code, None);
        let mut b1 = [0u8; 64];
        let mut b2 = [0u8; 64];
        for i in 0..16 {
            b1[i * 4..i * 4 + 4].copy_from_slice(&s1[i].to_le_bytes());
            b2[i * 4..i * 4 + 4].copy_from_slice(&s2[i].to_le_bytes());
        }
        mem.write_slice(&b1, GuestAddress(C_SRC1)).unwrap();
        mem.write_slice(&b2, GuestAddress(C_SRC2)).unwrap();
        let regs = run_until_hlt(&mut vcpu).unwrap();
        regs.k
    }

    #[test]
    fn test_vpcmpeqd_mask() {
        // VPCMPEQD k1, zmm1, zmm2 -> k1 bit i set iff lane i equal.
        let mut s1 = [0u32; 16];
        let mut s2 = [0u32; 16];
        let mut expected = 0u64;
        for i in 0..16 {
            s1[i] = i as u32;
            // make even lanes equal, odd lanes differ
            s2[i] = if i % 2 == 0 { i as u32 } else { 0xFFFF_FFFF };
            if s1[i] == s2[i] {
                expected |= 1u64 << i;
            }
        }
        let k = run_cmp(&[0x62, 0xf1, 0x75, 0x48, 0x76, 0xca], s1, s2); // vpcmpeqd k1,zmm1,zmm2
        assert_eq!(k[1], expected, "vpcmpeqd mask");
    }

    #[test]
    fn test_vpcmpgtd_mask_signed() {
        // VPCMPGTD k1, zmm1, zmm2 -> signed greater-than.
        let mut s1 = [0u32; 16];
        let mut s2 = [0u32; 16];
        let mut expected = 0u64;
        for i in 0..16 {
            // s1 alternates large-positive and "negative" (top bit set)
            s1[i] = if i % 2 == 0 {
                1000 + i as u32
            } else {
                0x8000_0000u32 | i as u32
            };
            s2[i] = 500;
            if (s1[i] as i32) > (s2[i] as i32) {
                expected |= 1u64 << i;
            }
        }
        let k = run_cmp(&[0x62, 0xf1, 0x75, 0x48, 0x66, 0xca], s1, s2); // vpcmpgtd k1,zmm1,zmm2
        assert_eq!(k[1], expected, "vpcmpgtd signed mask");
    }

    #[test]
    fn test_vpcmpd_imm_lt() {
        // VPCMPD k1, zmm1, zmm2, 1 (LT, signed).
        let mut s1 = [0u32; 16];
        let mut s2 = [0u32; 16];
        let mut expected = 0u64;
        for i in 0..16 {
            s1[i] = (i as i32 - 8) as u32; // -8..7
            s2[i] = 0;
            if (s1[i] as i32) < (s2[i] as i32) {
                expected |= 1u64 << i;
            }
        }
        // 62 f3 75 48 1f ca 01  vpcmpltd (predicate 1) k1,zmm1,zmm2
        let k = run_cmp(&[0x62, 0xf3, 0x75, 0x48, 0x1f, 0xca, 0x01], s1, s2);
        assert_eq!(k[1], expected, "vpcmpd imm=1 (LT) mask");
    }

    #[test]
    fn test_vpcmpud_imm_nle() {
        // VPCMPUD k1, zmm1, zmm2, 6 (NLE == GT, unsigned).
        let mut s1 = [0u32; 16];
        let mut s2 = [0u32; 16];
        let mut expected = 0u64;
        for i in 0..16 {
            // values that order differently signed vs unsigned (top-bit-set are "large" unsigned)
            s1[i] = if i % 2 == 0 {
                0x8000_0000u32 + i as u32
            } else {
                i as u32
            };
            s2[i] = 10;
            if s1[i] > s2[i] {
                // unsigned compare
                expected |= 1u64 << i;
            }
        }
        // 62 f3 75 48 1e ca 06  vpcmpud (predicate 6 = NLE) k1,zmm1,zmm2
        let k = run_cmp(&[0x62, 0xf3, 0x75, 0x48, 0x1e, 0xca, 0x06], s1, s2);
        assert_eq!(k[1], expected, "vpcmpud imm=6 (NLE/GT) mask");
    }

    #[test]
    fn test_vpcmpd_masked_writemask() {
        // VPCMPD k1{k2}, ... : only lanes selected by k2 participate; others 0.
        // Use predicate 0 (EQ) and set k2 = 0x00FF so only low 8 lanes are written.
        let mut code = Vec::new();
        mov_addrs(&mut code);
        code.extend_from_slice(&LD_Z1_RAX);
        code.extend_from_slice(&LD_Z2_RBX);
        // mov eax, 0xFF ; kmovw k2, eax
        code.extend_from_slice(&[0xb8, 0xff, 0x00, 0x00, 0x00]);
        code.extend_from_slice(&[0xc5, 0xf8, 0x92, 0xd0]); // kmovw k2, eax
        // vpcmpd k1{k2}, zmm1, zmm2, 0   (EQ)   -> aaa=010 (k2)
        code.extend_from_slice(&[0x62, 0xf3, 0x75, 0x4a, 0x1f, 0xca, 0x00]);
        code.push(0xf4);
        let (mut vcpu, mem) = setup_vm(&code, None);
        let mut b1 = [0u8; 64];
        let mut b2 = [0u8; 64];
        let mut expected = 0u64;
        for i in 0..16 {
            let v = i as u32;
            b1[i * 4..i * 4 + 4].copy_from_slice(&v.to_le_bytes());
            b2[i * 4..i * 4 + 4].copy_from_slice(&v.to_le_bytes()); // all equal
            if i < 8 {
                expected |= 1u64 << i; // only lanes selected by k2 are written
            }
        }
        mem.write_slice(&b1, GuestAddress(C_SRC1)).unwrap();
        mem.write_slice(&b2, GuestAddress(C_SRC2)).unwrap();
        let regs = run_until_hlt(&mut vcpu).unwrap();
        assert_eq!(regs.k[1], expected, "vpcmpd writemask-limited result");
    }

    #[test]
    fn test_vpcmpeqb_mask_64_lanes() {
        // VPCMPEQB k1, zmm1, zmm2 -> 64-bit mask, byte granularity at VL=512.
        let mut code = Vec::new();
        mov_addrs(&mut code);
        code.extend_from_slice(&LD_Z1_RAX);
        code.extend_from_slice(&LD_Z2_RBX);
        code.extend_from_slice(&[0x62, 0xf1, 0x75, 0x48, 0x74, 0xca]); // vpcmpeqb k1,zmm1,zmm2
        code.push(0xf4);
        let (mut vcpu, mem) = setup_vm(&code, None);
        let mut b1 = [0u8; 64];
        let mut b2 = [0u8; 64];
        let mut expected = 0u64;
        for i in 0..64 {
            b1[i] = i as u8;
            // every third byte equal
            b2[i] = if i % 3 == 0 {
                i as u8
            } else {
                (i as u8).wrapping_add(1)
            };
            if b1[i] == b2[i] {
                expected |= 1u64 << i;
            }
        }
        mem.write_slice(&b1, GuestAddress(C_SRC1)).unwrap();
        mem.write_slice(&b2, GuestAddress(C_SRC2)).unwrap();
        let regs = run_until_hlt(&mut vcpu).unwrap();
        assert_eq!(regs.k[1], expected, "vpcmpeqb 64-lane mask");
    }
}
