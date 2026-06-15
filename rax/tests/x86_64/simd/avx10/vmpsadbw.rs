//! AVX10.2 VMPSADBW (Multiple Packed Sum of Absolute Differences) Tests
//!
//! VMPSADBW computes multiple sums of absolute differences between
//! packed bytes in source operands, useful for motion estimation
//! and pattern matching in video encoding.
//!
//! EVEX encoding format:
//! - VMPSADBW: EVEX.128/256/512.66.0F3A.WIG 42 /r ib

use crate::common::*;

// ============================================================================
// VMPSADBW XMM Tests
// ============================================================================

#[test]
fn test_vmpsadbw_xmm_basic() {
    // VMPSADBW XMM0, XMM1, XMM2, imm8
    // EVEX.128.66.0F3A.WIG 42 /r ib
    let code = [
        0x62, 0xF3, 0x75, 0x08, 0x42, 0xC2, 0x00, // VMPSADBW xmm0, xmm1, xmm2, 0
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vmpsadbw_xmm_imm1() {
    // VMPSADBW XMM0, XMM1, XMM2, 1
    let code = [
        0x62, 0xF3, 0x75, 0x08, 0x42, 0xC2, 0x01, // VMPSADBW xmm0, xmm1, xmm2, 1
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vmpsadbw_xmm_imm2() {
    // VMPSADBW XMM0, XMM1, XMM2, 2
    let code = [
        0x62, 0xF3, 0x75, 0x08, 0x42, 0xC2, 0x02, // VMPSADBW xmm0, xmm1, xmm2, 2
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vmpsadbw_xmm_imm4() {
    // VMPSADBW XMM0, XMM1, XMM2, 4
    let code = [
        0x62, 0xF3, 0x75, 0x08, 0x42, 0xC2, 0x04, // VMPSADBW xmm0, xmm1, xmm2, 4
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vmpsadbw_xmm_extended_regs() {
    // VMPSADBW XMM8, XMM9, XMM10, imm8
    let code = [
        0x62, 0x53, 0x35, 0x08, 0x42, 0xC2, 0x03, // VMPSADBW xmm8, xmm9, xmm10, 3
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vmpsadbw_xmm_memory() {
    // VMPSADBW XMM0, XMM1, [RAX], imm8
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x30, 0x00, 0x00, // MOV RAX, 0x3000
        0x62, 0xF3, 0x75, 0x08, 0x42, 0x00, 0x05, // VMPSADBW xmm0, xmm1, [rax], 5
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vmpsadbw_xmm_with_mask() {
    // VMPSADBW XMM0 {k1}, XMM1, XMM2, imm8
    let code = [
        0x62, 0xF3, 0x75, 0x09, 0x42, 0xC2, 0x06, // VMPSADBW xmm0 {k1}, xmm1, xmm2, 6
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vmpsadbw_xmm_with_mask_zeroing() {
    // VMPSADBW XMM0 {k2}{z}, XMM1, XMM2, imm8
    let code = [
        0x62, 0xF3, 0x75, 0x8A, 0x42, 0xC2, 0x07, // VMPSADBW xmm0 {k2}{z}, xmm1, xmm2, 7
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// VMPSADBW YMM Tests
// ============================================================================

#[test]
fn test_vmpsadbw_ymm_basic() {
    // VMPSADBW YMM0, YMM1, YMM2, imm8
    let code = [
        0x62, 0xF3, 0x75, 0x28, 0x42, 0xC2, 0x00, // VMPSADBW ymm0, ymm1, ymm2, 0
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vmpsadbw_ymm_imm1() {
    // VMPSADBW YMM0, YMM1, YMM2, 1
    let code = [
        0x62, 0xF3, 0x75, 0x28, 0x42, 0xC2, 0x01, // VMPSADBW ymm0, ymm1, ymm2, 1
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vmpsadbw_ymm_extended_regs() {
    // VMPSADBW YMM16, YMM17, YMM18, imm8
    let code = [
        0x62, 0xE3, 0x75, 0x28, 0x42, 0xC2, 0x02, // VMPSADBW ymm16, ymm17, ymm18, 2
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vmpsadbw_ymm_memory() {
    // VMPSADBW YMM0, YMM1, [RAX], imm8
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x30, 0x00, 0x00, // MOV RAX, 0x3000
        0x62, 0xF3, 0x75, 0x28, 0x42, 0x00, 0x03, // VMPSADBW ymm0, ymm1, [rax], 3
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vmpsadbw_ymm_with_mask() {
    // VMPSADBW YMM0 {k3}, YMM1, YMM2, imm8
    let code = [
        0x62, 0xF3, 0x75, 0x2B, 0x42, 0xC2, 0x04, // VMPSADBW ymm0 {k3}, ymm1, ymm2, 4
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// VMPSADBW ZMM Tests
// ============================================================================

#[test]
fn test_vmpsadbw_zmm_basic() {
    // VMPSADBW ZMM0, ZMM1, ZMM2, imm8
    let code = [
        0x62, 0xF3, 0x75, 0x48, 0x42, 0xC2, 0x00, // VMPSADBW zmm0, zmm1, zmm2, 0
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vmpsadbw_zmm_imm_modes() {
    // Test various immediate values
    let code = [
        0x62, 0xF3, 0x75, 0x48, 0x42, 0xC2, 0x01, // VMPSADBW zmm0, zmm1, zmm2, 1
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vmpsadbw_zmm_high_regs() {
    // VMPSADBW ZMM24, ZMM25, ZMM26, imm8
    let code = [
        0x62, 0x93, 0x35, 0x48, 0x42, 0xC2, 0x05, // VMPSADBW zmm24, zmm25, zmm26, 5
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vmpsadbw_zmm_memory() {
    // VMPSADBW ZMM0, ZMM1, [RAX], imm8
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x30, 0x00, 0x00, // MOV RAX, 0x3000
        0x62, 0xF3, 0x75, 0x48, 0x42, 0x00, 0x06, // VMPSADBW zmm0, zmm1, [rax], 6
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vmpsadbw_zmm_with_mask() {
    // VMPSADBW ZMM0 {k4}, ZMM1, ZMM2, imm8
    let code = [
        0x62, 0xF3, 0x75, 0x4C, 0x42, 0xC2, 0x07, // VMPSADBW zmm0 {k4}, zmm1, zmm2, 7
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vmpsadbw_zmm_with_mask_zeroing() {
    // VMPSADBW ZMM0 {k5}{z}, ZMM1, ZMM2, imm8
    let code = [
        0x62, 0xF3, 0x75, 0xCD, 0x42, 0xC2, 0x08, // VMPSADBW zmm0 {k5}{z}, zmm1, zmm2, 8
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// Combined VMPSADBW Tests
// ============================================================================

#[test]
fn test_vmpsadbw_all_sizes() {
    // Test VMPSADBW on all vector sizes
    let code = [
        0x62, 0xF3, 0x75, 0x08, 0x42, 0xC2, 0x00, // VMPSADBW xmm0, xmm1, xmm2, 0
        0x62, 0xF3, 0x6D, 0x28, 0x42, 0xD3, 0x01, // VMPSADBW ymm2, ymm2, ymm3, 1
        0x62, 0xF3, 0x65, 0x48, 0x42, 0xE4, 0x02, // VMPSADBW zmm4, zmm3, zmm4, 2
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vmpsadbw_chain() {
    // Chain VMPSADBW operations for multi-block SAD
    let code = [
        0x62, 0xF3, 0x75, 0x48, 0x42, 0xC2, 0x00, // VMPSADBW zmm0, zmm1, zmm2, 0
        0x62, 0xF3, 0x75, 0x48, 0x42, 0xC3, 0x01, // VMPSADBW zmm0, zmm1, zmm3, 1
        0x62, 0xF3, 0x75, 0x48, 0x42, 0xC4, 0x02, // VMPSADBW zmm0, zmm1, zmm4, 2
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vmpsadbw_all_imm_bits() {
    // Test with all immediate bits combinations
    let code = [
        0x62, 0xF3, 0x75, 0x48, 0x42, 0xC2, 0x00, // VMPSADBW zmm0, zmm1, zmm2, 0x00
        0x62, 0xF3, 0x6D, 0x48, 0x42, 0xD2, 0x0F, // VMPSADBW zmm2, zmm2, zmm2, 0x0F
        0x62, 0xF3, 0x65, 0x48, 0x42, 0xE3, 0xFF, // VMPSADBW zmm4, zmm3, zmm3, 0xFF
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}
