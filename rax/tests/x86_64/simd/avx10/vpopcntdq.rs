//! AVX10.1 VPOPCNTDQ (Population Count Dword/Qword) Tests
//!
//! VPOPCNTDQ provides vector population count (bit counting) for dwords and qwords.
//! Useful for hamming distance, set cardinality, and other bit manipulation.
//!
//! Instructions covered:
//! - VPOPCNTD - Population count of dwords
//! - VPOPCNTQ - Population count of qwords
//!
//! EVEX encoding format:
//! - VPOPCNTD: EVEX.128/256/512.66.0F38.W0 55 /r
//! - VPOPCNTQ: EVEX.128/256/512.66.0F38.W1 55 /r

use crate::common::*;

// ============================================================================
// VPOPCNTD Tests - Population Count Dwords
// ============================================================================

#[test]
fn test_vpopcntd_xmm_basic() {
    // VPOPCNTD XMM0, XMM1
    // EVEX.128.66.0F38.W0 55 /r
    let code = [
        0x62, 0xF2, 0x7D, 0x08, 0x55, 0xC1, // VPOPCNTD xmm0, xmm1
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpopcntd_xmm_extended_regs() {
    // VPOPCNTD XMM8, XMM9
    let code = [
        0x62, 0x52, 0x7D, 0x08, 0x55, 0xC1, // VPOPCNTD xmm8, xmm9
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpopcntd_ymm_basic() {
    // VPOPCNTD YMM0, YMM1
    let code = [
        0x62, 0xF2, 0x7D, 0x28, 0x55, 0xC1, // VPOPCNTD ymm0, ymm1
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpopcntd_ymm_extended_regs() {
    // VPOPCNTD YMM16, YMM17
    let code = [
        0x62, 0xE2, 0x7D, 0x28, 0x55, 0xC1, // VPOPCNTD ymm16, ymm17
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpopcntd_zmm_basic() {
    // VPOPCNTD ZMM0, ZMM1
    let code = [
        0x62, 0xF2, 0x7D, 0x48, 0x55, 0xC1, // VPOPCNTD zmm0, zmm1
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpopcntd_zmm_high_regs() {
    // VPOPCNTD ZMM24, ZMM25
    let code = [
        0x62, 0x92, 0x7D, 0x48, 0x55, 0xC1, // VPOPCNTD zmm24, zmm25
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpopcntd_xmm_memory() {
    // VPOPCNTD XMM0, [RAX]
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x30, 0x00, 0x00, // MOV RAX, 0x3000
        0x62, 0xF2, 0x7D, 0x08, 0x55, 0x00, // VPOPCNTD xmm0, [rax]
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpopcntd_ymm_memory() {
    // VPOPCNTD YMM0, [RAX]
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x30, 0x00, 0x00, // MOV RAX, 0x3000
        0x62, 0xF2, 0x7D, 0x28, 0x55, 0x00, // VPOPCNTD ymm0, [rax]
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpopcntd_zmm_memory() {
    // VPOPCNTD ZMM0, [RAX]
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x30, 0x00, 0x00, // MOV RAX, 0x3000
        0x62, 0xF2, 0x7D, 0x48, 0x55, 0x00, // VPOPCNTD zmm0, [rax]
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpopcntd_xmm_with_mask() {
    // VPOPCNTD XMM0 {k1}, XMM1
    let code = [
        0x62, 0xF2, 0x7D, 0x09, 0x55, 0xC1, // VPOPCNTD xmm0 {k1}, xmm1
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpopcntd_ymm_with_mask_zeroing() {
    // VPOPCNTD YMM0 {k2}{z}, YMM1
    let code = [
        0x62, 0xF2, 0x7D, 0xAA, 0x55, 0xC1, // VPOPCNTD ymm0 {k2}{z}, ymm1
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpopcntd_zmm_broadcast() {
    // VPOPCNTD ZMM0, dword ptr [RAX]{1to16}
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x30, 0x00, 0x00, // MOV RAX, 0x3000
        0x62, 0xF2, 0x7D, 0x58, 0x55, 0x00, // VPOPCNTD zmm0, [rax]{1to16}
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// VPOPCNTQ Tests - Population Count Qwords
// ============================================================================

#[test]
fn test_vpopcntq_xmm_basic() {
    // VPOPCNTQ XMM0, XMM1
    // EVEX.128.66.0F38.W1 55 /r
    let code = [
        0x62, 0xF2, 0xFD, 0x08, 0x55, 0xC1, // VPOPCNTQ xmm0, xmm1
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpopcntq_xmm_extended_regs() {
    // VPOPCNTQ XMM12, XMM13
    let code = [
        0x62, 0x52, 0xFD, 0x08, 0x55, 0xE5, // VPOPCNTQ xmm12, xmm13
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpopcntq_ymm_basic() {
    // VPOPCNTQ YMM0, YMM1
    let code = [
        0x62, 0xF2, 0xFD, 0x28, 0x55, 0xC1, // VPOPCNTQ ymm0, ymm1
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpopcntq_ymm_extended_regs() {
    // VPOPCNTQ YMM20, YMM21
    let code = [
        0x62, 0xE2, 0xFD, 0x28, 0x55, 0xE5, // VPOPCNTQ ymm20, ymm21
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpopcntq_zmm_basic() {
    // VPOPCNTQ ZMM0, ZMM1
    let code = [
        0x62, 0xF2, 0xFD, 0x48, 0x55, 0xC1, // VPOPCNTQ zmm0, zmm1
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpopcntq_zmm_high_regs() {
    // VPOPCNTQ ZMM28, ZMM29
    let code = [
        0x62, 0x92, 0xFD, 0x48, 0x55, 0xE5, // VPOPCNTQ zmm28, zmm29
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpopcntq_xmm_memory() {
    // VPOPCNTQ XMM0, [RAX]
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x30, 0x00, 0x00, // MOV RAX, 0x3000
        0x62, 0xF2, 0xFD, 0x08, 0x55, 0x00, // VPOPCNTQ xmm0, [rax]
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpopcntq_ymm_memory() {
    // VPOPCNTQ YMM0, [RAX]
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x30, 0x00, 0x00, // MOV RAX, 0x3000
        0x62, 0xF2, 0xFD, 0x28, 0x55, 0x00, // VPOPCNTQ ymm0, [rax]
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpopcntq_zmm_memory() {
    // VPOPCNTQ ZMM0, [RAX]
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x30, 0x00, 0x00, // MOV RAX, 0x3000
        0x62, 0xF2, 0xFD, 0x48, 0x55, 0x00, // VPOPCNTQ zmm0, [rax]
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpopcntq_xmm_with_mask() {
    // VPOPCNTQ XMM0 {k3}, XMM1
    let code = [
        0x62, 0xF2, 0xFD, 0x0B, 0x55, 0xC1, // VPOPCNTQ xmm0 {k3}, xmm1
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpopcntq_zmm_with_mask_zeroing() {
    // VPOPCNTQ ZMM0 {k4}{z}, ZMM1
    let code = [
        0x62, 0xF2, 0xFD, 0xCC, 0x55, 0xC1, // VPOPCNTQ zmm0 {k4}{z}, zmm1
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpopcntq_zmm_broadcast() {
    // VPOPCNTQ ZMM0, qword ptr [RAX]{1to8}
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x30, 0x00, 0x00, // MOV RAX, 0x3000
        0x62, 0xF2, 0xFD, 0x58, 0x55, 0x00, // VPOPCNTQ zmm0, [rax]{1to8}
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// Combined VPOPCNT Operations Tests
// ============================================================================

#[test]
fn test_vpopcnt_all_sizes_dword() {
    // Test VPOPCNTD on all vector sizes
    let code = [
        0x62, 0xF2, 0x7D, 0x08, 0x55, 0xC1, // VPOPCNTD xmm0, xmm1
        0x62, 0xF2, 0x7D, 0x28, 0x55, 0xD2, // VPOPCNTD ymm2, ymm2
        0x62, 0xF2, 0x7D, 0x48, 0x55, 0xE3, // VPOPCNTD zmm4, zmm3
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpopcnt_all_sizes_qword() {
    // Test VPOPCNTQ on all vector sizes
    let code = [
        0x62, 0xF2, 0xFD, 0x08, 0x55, 0xC1, // VPOPCNTQ xmm0, xmm1
        0x62, 0xF2, 0xFD, 0x28, 0x55, 0xD2, // VPOPCNTQ ymm2, ymm2
        0x62, 0xF2, 0xFD, 0x48, 0x55, 0xE3, // VPOPCNTQ zmm4, zmm3
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpopcnt_chain() {
    // Chain popcnt operations for hamming weight computation
    let code = [
        0x62, 0xF2, 0x7D, 0x48, 0x55, 0xC1, // VPOPCNTD zmm0, zmm1
        0x62, 0xF2, 0xFD, 0x48, 0x55, 0xD2, // VPOPCNTQ zmm2, zmm2
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpopcnt_self() {
    // Popcnt of itself
    let code = [
        0x62, 0xF2, 0x7D, 0x48, 0x55, 0xC0, // VPOPCNTD zmm0, zmm0
        0x62, 0xF2, 0xFD, 0x48, 0x55, 0xC9, // VPOPCNTQ zmm1, zmm1
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}
