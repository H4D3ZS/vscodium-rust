//! AVX10.1 VNNI (Vector Neural Network Instructions) Tests
//!
//! VNNI provides optimized integer dot-product operations for neural network inference.
//! These instructions are critical for efficient INT8/INT16 quantized inference.
//!
//! Instructions covered:
//! - VPDPBUSD - Multiply and add unsigned/signed bytes to dword
//! - VPDPBUSDS - Same with saturation
//! - VPDPWSSD - Multiply and add signed words to dword
//! - VPDPWSSDS - Same with saturation
//!
//! In AVX10, these are available on XMM/YMM/ZMM with consistent encoding.
//!
//! EVEX encoding format:
//! - VPDPBUSD: EVEX.128/256/512.66.0F38.W0 50 /r
//! - VPDPBUSDS: EVEX.128/256/512.66.0F38.W0 51 /r
//! - VPDPWSSD: EVEX.128/256/512.66.0F38.W0 52 /r
//! - VPDPWSSDS: EVEX.128/256/512.66.0F38.W0 53 /r

use crate::common::*;

// ============================================================================
// VPDPBUSD Tests - Multiply Unsigned/Signed Bytes to Dword
// ============================================================================

#[test]
fn test_vpdpbusd_xmm_basic() {
    // VPDPBUSD XMM0, XMM1, XMM2
    // EVEX.128.66.0F38.W0 50 /r
    let code = [
        0x62, 0xF2, 0x75, 0x08, 0x50, 0xC2, // VPDPBUSD xmm0, xmm1, xmm2
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpdpbusd_xmm_extended_regs() {
    // VPDPBUSD XMM8, XMM9, XMM10
    let code = [
        0x62, 0x52, 0x35, 0x08, 0x50, 0xC2, // VPDPBUSD xmm8, xmm9, xmm10
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpdpbusd_ymm_basic() {
    // VPDPBUSD YMM0, YMM1, YMM2
    // EVEX.256.66.0F38.W0 50 /r
    let code = [
        0x62, 0xF2, 0x75, 0x28, 0x50, 0xC2, // VPDPBUSD ymm0, ymm1, ymm2
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpdpbusd_ymm_extended_regs() {
    // VPDPBUSD YMM16, YMM17, YMM18
    let code = [
        0x62, 0xE2, 0x75, 0x28, 0x50, 0xC2, // VPDPBUSD ymm16, ymm17, ymm18
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpdpbusd_zmm_basic() {
    // VPDPBUSD ZMM0, ZMM1, ZMM2
    // EVEX.512.66.0F38.W0 50 /r
    let code = [
        0x62, 0xF2, 0x75, 0x48, 0x50, 0xC2, // VPDPBUSD zmm0, zmm1, zmm2
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpdpbusd_zmm_high_regs() {
    // VPDPBUSD ZMM24, ZMM25, ZMM26
    let code = [
        0x62, 0x92, 0x35, 0x48, 0x50, 0xC2, // VPDPBUSD zmm24, zmm25, zmm26
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpdpbusd_xmm_memory() {
    // VPDPBUSD XMM0, XMM1, [RAX]
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x30, 0x00, 0x00, // MOV RAX, 0x3000
        0x62, 0xF2, 0x75, 0x08, 0x50, 0x00, // VPDPBUSD xmm0, xmm1, [rax]
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpdpbusd_ymm_memory() {
    // VPDPBUSD YMM0, YMM1, [RAX]
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x30, 0x00, 0x00, // MOV RAX, 0x3000
        0x62, 0xF2, 0x75, 0x28, 0x50, 0x00, // VPDPBUSD ymm0, ymm1, [rax]
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpdpbusd_zmm_memory() {
    // VPDPBUSD ZMM0, ZMM1, [RAX]
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x30, 0x00, 0x00, // MOV RAX, 0x3000
        0x62, 0xF2, 0x75, 0x48, 0x50, 0x00, // VPDPBUSD zmm0, zmm1, [rax]
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpdpbusd_xmm_with_mask_k1() {
    // VPDPBUSD XMM0 {k1}, XMM1, XMM2
    let code = [
        0x62, 0xF2, 0x75, 0x09, 0x50, 0xC2, // VPDPBUSD xmm0 {k1}, xmm1, xmm2
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpdpbusd_ymm_with_mask_zeroing() {
    // VPDPBUSD YMM0 {k2}{z}, YMM1, YMM2
    let code = [
        0x62, 0xF2, 0x75, 0xAA, 0x50, 0xC2, // VPDPBUSD ymm0 {k2}{z}, ymm1, ymm2
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpdpbusd_zmm_broadcast() {
    // VPDPBUSD ZMM0, ZMM1, dword ptr [RAX]{1to16}
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x30, 0x00, 0x00, // MOV RAX, 0x3000
        0x62, 0xF2, 0x75, 0x58, 0x50, 0x00, // VPDPBUSD zmm0, zmm1, [rax]{1to16}
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// VPDPBUSDS Tests - With Saturation
// ============================================================================

#[test]
fn test_vpdpbusds_xmm_basic() {
    // VPDPBUSDS XMM0, XMM1, XMM2
    // EVEX.128.66.0F38.W0 51 /r
    let code = [
        0x62, 0xF2, 0x75, 0x08, 0x51, 0xC2, // VPDPBUSDS xmm0, xmm1, xmm2
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpdpbusds_ymm_basic() {
    // VPDPBUSDS YMM0, YMM1, YMM2
    let code = [
        0x62, 0xF2, 0x75, 0x28, 0x51, 0xC2, // VPDPBUSDS ymm0, ymm1, ymm2
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpdpbusds_zmm_basic() {
    // VPDPBUSDS ZMM0, ZMM1, ZMM2
    let code = [
        0x62, 0xF2, 0x75, 0x48, 0x51, 0xC2, // VPDPBUSDS zmm0, zmm1, zmm2
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpdpbusds_xmm_memory() {
    // VPDPBUSDS XMM0, XMM1, [RAX]
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x30, 0x00, 0x00, // MOV RAX, 0x3000
        0x62, 0xF2, 0x75, 0x08, 0x51, 0x00, // VPDPBUSDS xmm0, xmm1, [rax]
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpdpbusds_zmm_with_mask() {
    // VPDPBUSDS ZMM0 {k3}, ZMM1, ZMM2
    let code = [
        0x62, 0xF2, 0x75, 0x4B, 0x51, 0xC2, // VPDPBUSDS zmm0 {k3}, zmm1, zmm2
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpdpbusds_extended_regs() {
    // VPDPBUSDS ZMM30, ZMM29, ZMM28
    let code = [
        0x62, 0x92, 0x15, 0x48, 0x51, 0xF4, // VPDPBUSDS zmm30, zmm29, zmm28
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// VPDPWSSD Tests - Multiply Signed Words to Dword
// ============================================================================

#[test]
fn test_vpdpwssd_xmm_basic() {
    // VPDPWSSD XMM0, XMM1, XMM2
    // EVEX.128.66.0F38.W0 52 /r
    let code = [
        0x62, 0xF2, 0x75, 0x08, 0x52, 0xC2, // VPDPWSSD xmm0, xmm1, xmm2
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpdpwssd_ymm_basic() {
    // VPDPWSSD YMM0, YMM1, YMM2
    let code = [
        0x62, 0xF2, 0x75, 0x28, 0x52, 0xC2, // VPDPWSSD ymm0, ymm1, ymm2
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpdpwssd_zmm_basic() {
    // VPDPWSSD ZMM0, ZMM1, ZMM2
    let code = [
        0x62, 0xF2, 0x75, 0x48, 0x52, 0xC2, // VPDPWSSD zmm0, zmm1, zmm2
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpdpwssd_xmm_memory() {
    // VPDPWSSD XMM0, XMM1, [RAX]
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x30, 0x00, 0x00, // MOV RAX, 0x3000
        0x62, 0xF2, 0x75, 0x08, 0x52, 0x00, // VPDPWSSD xmm0, xmm1, [rax]
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpdpwssd_ymm_extended_regs() {
    // VPDPWSSD YMM12, YMM13, YMM14
    let code = [
        0x62, 0x52, 0x15, 0x28, 0x52, 0xE6, // VPDPWSSD ymm12, ymm13, ymm14
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpdpwssd_zmm_with_mask_zeroing() {
    // VPDPWSSD ZMM0 {k4}{z}, ZMM1, ZMM2
    let code = [
        0x62, 0xF2, 0x75, 0xCC, 0x52, 0xC2, // VPDPWSSD zmm0 {k4}{z}, zmm1, zmm2
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpdpwssd_zmm_broadcast() {
    // VPDPWSSD ZMM0, ZMM1, dword ptr [RAX]{1to16}
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x30, 0x00, 0x00, // MOV RAX, 0x3000
        0x62, 0xF2, 0x75, 0x58, 0x52, 0x00, // VPDPWSSD zmm0, zmm1, [rax]{1to16}
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// VPDPWSSDS Tests - With Saturation
// ============================================================================

#[test]
fn test_vpdpwssds_xmm_basic() {
    // VPDPWSSDS XMM0, XMM1, XMM2
    // EVEX.128.66.0F38.W0 53 /r
    let code = [
        0x62, 0xF2, 0x75, 0x08, 0x53, 0xC2, // VPDPWSSDS xmm0, xmm1, xmm2
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpdpwssds_ymm_basic() {
    // VPDPWSSDS YMM0, YMM1, YMM2
    let code = [
        0x62, 0xF2, 0x75, 0x28, 0x53, 0xC2, // VPDPWSSDS ymm0, ymm1, ymm2
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpdpwssds_zmm_basic() {
    // VPDPWSSDS ZMM0, ZMM1, ZMM2
    let code = [
        0x62, 0xF2, 0x75, 0x48, 0x53, 0xC2, // VPDPWSSDS zmm0, zmm1, zmm2
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpdpwssds_xmm_memory() {
    // VPDPWSSDS XMM0, XMM1, [RAX]
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x30, 0x00, 0x00, // MOV RAX, 0x3000
        0x62, 0xF2, 0x75, 0x08, 0x53, 0x00, // VPDPWSSDS xmm0, xmm1, [rax]
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpdpwssds_zmm_with_mask() {
    // VPDPWSSDS ZMM0 {k5}, ZMM1, ZMM2
    let code = [
        0x62, 0xF2, 0x75, 0x4D, 0x53, 0xC2, // VPDPWSSDS zmm0 {k5}, zmm1, zmm2
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpdpwssds_high_regs() {
    // VPDPWSSDS ZMM31, ZMM30, ZMM29
    let code = [
        0x62, 0x92, 0x0D, 0x48, 0x53, 0xFD, // VPDPWSSDS zmm31, zmm30, zmm29
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// Combined VNNI Operations Tests
// ============================================================================

#[test]
fn test_vnni_chain_xmm() {
    // Chain multiple VNNI ops on XMM
    let code = [
        0x62, 0xF2, 0x75, 0x08, 0x50, 0xC2, // VPDPBUSD xmm0, xmm1, xmm2
        0x62, 0xF2, 0x75, 0x08, 0x52, 0xC3, // VPDPWSSD xmm0, xmm1, xmm3
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vnni_chain_zmm() {
    // Chain multiple VNNI ops on ZMM
    let code = [
        0x62, 0xF2, 0x75, 0x48, 0x50, 0xC2, // VPDPBUSD zmm0, zmm1, zmm2
        0x62, 0xF2, 0x75, 0x48, 0x51, 0xC3, // VPDPBUSDS zmm0, zmm1, zmm3
        0x62, 0xF2, 0x75, 0x48, 0x52, 0xC4, // VPDPWSSD zmm0, zmm1, zmm4
        0x62, 0xF2, 0x75, 0x48, 0x53, 0xC5, // VPDPWSSDS zmm0, zmm1, zmm5
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vnni_all_vector_sizes() {
    // Test same instruction on all vector sizes
    let code = [
        0x62, 0xF2, 0x75, 0x08, 0x50, 0xC2, // VPDPBUSD xmm0, xmm1, xmm2
        0x62, 0xF2, 0x6D, 0x28, 0x50, 0xD3, // VPDPBUSD ymm2, ymm2, ymm3
        0x62, 0xF2, 0x65, 0x48, 0x50, 0xE4, // VPDPBUSD zmm4, zmm3, zmm4
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vnni_self_accumulate() {
    // Accumulate into self
    let code = [
        0x62, 0xF2, 0x7D, 0x48, 0x50, 0xC1, // VPDPBUSD zmm0, zmm0, zmm1
        0x62, 0xF2, 0x7D, 0x48, 0x50, 0xC1, // VPDPBUSD zmm0, zmm0, zmm1
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}
