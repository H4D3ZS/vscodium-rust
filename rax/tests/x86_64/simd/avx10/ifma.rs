//! AVX10.1 IFMA (Integer Fused Multiply-Add) Tests
//!
//! IFMA provides 52-bit integer multiply-add operations, useful for
//! cryptographic computations like RSA and elliptic curve operations.
//!
//! Instructions covered:
//! - VPMADD52LUQ - Packed multiply of unsigned 52-bit integers, add low 52-bit to qword
//! - VPMADD52HUQ - Packed multiply of unsigned 52-bit integers, add high 52-bit to qword
//!
//! In AVX10, these are available on XMM/YMM/ZMM with consistent encoding.
//!
//! EVEX encoding format:
//! - VPMADD52LUQ: EVEX.128/256/512.66.0F38.W1 B4 /r
//! - VPMADD52HUQ: EVEX.128/256/512.66.0F38.W1 B5 /r

use crate::common::*;

// ============================================================================
// VPMADD52LUQ Tests - Multiply Add Low 52-bit
// ============================================================================

#[test]
fn test_vpmadd52luq_xmm_basic() {
    // VPMADD52LUQ XMM0, XMM1, XMM2
    // EVEX.128.66.0F38.W1 B4 /r
    let code = [
        0x62, 0xF2, 0xF5, 0x08, 0xB4, 0xC2, // VPMADD52LUQ xmm0, xmm1, xmm2
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpmadd52luq_xmm_extended_regs() {
    // VPMADD52LUQ XMM8, XMM9, XMM10
    let code = [
        0x62, 0x52, 0xB5, 0x08, 0xB4, 0xC2, // VPMADD52LUQ xmm8, xmm9, xmm10
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpmadd52luq_ymm_basic() {
    // VPMADD52LUQ YMM0, YMM1, YMM2
    // EVEX.256.66.0F38.W1 B4 /r
    let code = [
        0x62, 0xF2, 0xF5, 0x28, 0xB4, 0xC2, // VPMADD52LUQ ymm0, ymm1, ymm2
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpmadd52luq_ymm_extended_regs() {
    // VPMADD52LUQ YMM16, YMM17, YMM18
    let code = [
        0x62, 0xE2, 0xF5, 0x28, 0xB4, 0xC2, // VPMADD52LUQ ymm16, ymm17, ymm18
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpmadd52luq_zmm_basic() {
    // VPMADD52LUQ ZMM0, ZMM1, ZMM2
    // EVEX.512.66.0F38.W1 B4 /r
    let code = [
        0x62, 0xF2, 0xF5, 0x48, 0xB4, 0xC2, // VPMADD52LUQ zmm0, zmm1, zmm2
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpmadd52luq_zmm_high_regs() {
    // VPMADD52LUQ ZMM24, ZMM25, ZMM26
    let code = [
        0x62, 0x92, 0xB5, 0x48, 0xB4, 0xC2, // VPMADD52LUQ zmm24, zmm25, zmm26
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpmadd52luq_xmm_memory() {
    // VPMADD52LUQ XMM0, XMM1, [RAX]
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x30, 0x00, 0x00, // MOV RAX, 0x3000
        0x62, 0xF2, 0xF5, 0x08, 0xB4, 0x00, // VPMADD52LUQ xmm0, xmm1, [rax]
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpmadd52luq_ymm_memory() {
    // VPMADD52LUQ YMM0, YMM1, [RAX]
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x30, 0x00, 0x00, // MOV RAX, 0x3000
        0x62, 0xF2, 0xF5, 0x28, 0xB4, 0x00, // VPMADD52LUQ ymm0, ymm1, [rax]
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpmadd52luq_zmm_memory() {
    // VPMADD52LUQ ZMM0, ZMM1, [RAX]
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x30, 0x00, 0x00, // MOV RAX, 0x3000
        0x62, 0xF2, 0xF5, 0x48, 0xB4, 0x00, // VPMADD52LUQ zmm0, zmm1, [rax]
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpmadd52luq_xmm_with_mask_k1() {
    // VPMADD52LUQ XMM0 {k1}, XMM1, XMM2
    let code = [
        0x62, 0xF2, 0xF5, 0x09, 0xB4, 0xC2, // VPMADD52LUQ xmm0 {k1}, xmm1, xmm2
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpmadd52luq_ymm_with_mask_zeroing() {
    // VPMADD52LUQ YMM0 {k2}{z}, YMM1, YMM2
    let code = [
        0x62, 0xF2, 0xF5, 0xAA, 0xB4, 0xC2, // VPMADD52LUQ ymm0 {k2}{z}, ymm1, ymm2
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpmadd52luq_zmm_broadcast() {
    // VPMADD52LUQ ZMM0, ZMM1, qword ptr [RAX]{1to8}
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x30, 0x00, 0x00, // MOV RAX, 0x3000
        0x62, 0xF2, 0xF5, 0x58, 0xB4, 0x00, // VPMADD52LUQ zmm0, zmm1, [rax]{1to8}
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// VPMADD52HUQ Tests - Multiply Add High 52-bit
// ============================================================================

#[test]
fn test_vpmadd52huq_xmm_basic() {
    // VPMADD52HUQ XMM0, XMM1, XMM2
    // EVEX.128.66.0F38.W1 B5 /r
    let code = [
        0x62, 0xF2, 0xF5, 0x08, 0xB5, 0xC2, // VPMADD52HUQ xmm0, xmm1, xmm2
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpmadd52huq_xmm_extended_regs() {
    // VPMADD52HUQ XMM12, XMM13, XMM14
    let code = [
        0x62, 0x52, 0x95, 0x08, 0xB5, 0xE6, // VPMADD52HUQ xmm12, xmm13, xmm14
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpmadd52huq_ymm_basic() {
    // VPMADD52HUQ YMM0, YMM1, YMM2
    let code = [
        0x62, 0xF2, 0xF5, 0x28, 0xB5, 0xC2, // VPMADD52HUQ ymm0, ymm1, ymm2
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpmadd52huq_ymm_extended_regs() {
    // VPMADD52HUQ YMM20, YMM21, YMM22
    let code = [
        0x62, 0xE2, 0xD5, 0x28, 0xB5, 0xE6, // VPMADD52HUQ ymm20, ymm21, ymm22
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpmadd52huq_zmm_basic() {
    // VPMADD52HUQ ZMM0, ZMM1, ZMM2
    let code = [
        0x62, 0xF2, 0xF5, 0x48, 0xB5, 0xC2, // VPMADD52HUQ zmm0, zmm1, zmm2
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpmadd52huq_zmm_high_regs() {
    // VPMADD52HUQ ZMM28, ZMM29, ZMM30
    let code = [
        0x62, 0x92, 0x95, 0x48, 0xB5, 0xE6, // VPMADD52HUQ zmm28, zmm29, zmm30
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpmadd52huq_xmm_memory() {
    // VPMADD52HUQ XMM0, XMM1, [RAX]
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x30, 0x00, 0x00, // MOV RAX, 0x3000
        0x62, 0xF2, 0xF5, 0x08, 0xB5, 0x00, // VPMADD52HUQ xmm0, xmm1, [rax]
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpmadd52huq_ymm_memory() {
    // VPMADD52HUQ YMM0, YMM1, [RAX]
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x30, 0x00, 0x00, // MOV RAX, 0x3000
        0x62, 0xF2, 0xF5, 0x28, 0xB5, 0x00, // VPMADD52HUQ ymm0, ymm1, [rax]
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpmadd52huq_zmm_memory() {
    // VPMADD52HUQ ZMM0, ZMM1, [RAX]
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x30, 0x00, 0x00, // MOV RAX, 0x3000
        0x62, 0xF2, 0xF5, 0x48, 0xB5, 0x00, // VPMADD52HUQ zmm0, zmm1, [rax]
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpmadd52huq_xmm_with_mask() {
    // VPMADD52HUQ XMM0 {k3}, XMM1, XMM2
    let code = [
        0x62, 0xF2, 0xF5, 0x0B, 0xB5, 0xC2, // VPMADD52HUQ xmm0 {k3}, xmm1, xmm2
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpmadd52huq_zmm_with_mask_zeroing() {
    // VPMADD52HUQ ZMM0 {k4}{z}, ZMM1, ZMM2
    let code = [
        0x62, 0xF2, 0xF5, 0xCC, 0xB5, 0xC2, // VPMADD52HUQ zmm0 {k4}{z}, zmm1, zmm2
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpmadd52huq_zmm_broadcast() {
    // VPMADD52HUQ ZMM0, ZMM1, qword ptr [RAX]{1to8}
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x30, 0x00, 0x00, // MOV RAX, 0x3000
        0x62, 0xF2, 0xF5, 0x58, 0xB5, 0x00, // VPMADD52HUQ zmm0, zmm1, [rax]{1to8}
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// Combined IFMA Operations Tests
// ============================================================================

#[test]
fn test_ifma_low_high_chain_xmm() {
    // Use both low and high for full 104-bit result
    let code = [
        0x62, 0xF2, 0xF5, 0x08, 0xB4, 0xC2, // VPMADD52LUQ xmm0, xmm1, xmm2
        0x62, 0xF2, 0xED, 0x08, 0xB5, 0xDB, // VPMADD52HUQ xmm3, xmm2, xmm3
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_ifma_low_high_chain_zmm() {
    // Use both low and high for full 104-bit result on ZMM
    let code = [
        0x62, 0xF2, 0xF5, 0x48, 0xB4, 0xC2, // VPMADD52LUQ zmm0, zmm1, zmm2
        0x62, 0xF2, 0xED, 0x48, 0xB5, 0xDB, // VPMADD52HUQ zmm3, zmm2, zmm3
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_ifma_all_vector_sizes() {
    // Test same instruction on all vector sizes
    let code = [
        0x62, 0xF2, 0xF5, 0x08, 0xB4, 0xC2, // VPMADD52LUQ xmm0, xmm1, xmm2
        0x62, 0xF2, 0xED, 0x28, 0xB4, 0xD3, // VPMADD52LUQ ymm2, ymm2, ymm3
        0x62, 0xF2, 0xE5, 0x48, 0xB4, 0xE4, // VPMADD52LUQ zmm4, zmm3, zmm4
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_ifma_accumulate() {
    // Repeated accumulation pattern for Montgomery multiplication
    let code = [
        0x62, 0xF2, 0xF5, 0x48, 0xB4, 0xC2, // VPMADD52LUQ zmm0, zmm1, zmm2
        0x62, 0xF2, 0xF5, 0x48, 0xB4, 0xC3, // VPMADD52LUQ zmm0, zmm1, zmm3
        0x62, 0xF2, 0xF5, 0x48, 0xB4, 0xC4, // VPMADD52LUQ zmm0, zmm1, zmm4
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_ifma_self_multiply() {
    // Square operation (multiply by self)
    let code = [
        0x62, 0xF2, 0xF5, 0x48, 0xB4, 0xC1, // VPMADD52LUQ zmm0, zmm1, zmm1
        0x62, 0xF2, 0xF5, 0x48, 0xB5, 0xD1, // VPMADD52HUQ zmm2, zmm1, zmm1
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}
