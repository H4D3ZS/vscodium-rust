//! AVX10.1 BITALG (Bit Algorithms) Tests
//!
//! BITALG provides additional bit manipulation operations for bytes and words.
//! Useful for data processing, compression, and specialized algorithms.
//!
//! Instructions covered:
//! - VPSHUFBITQMB - Shuffle bits in qwords using byte indices to mask
//! - VPOPCNTB - Population count of bytes
//! - VPOPCNTW - Population count of words
//!
//! EVEX encoding format:
//! - VPSHUFBITQMB: EVEX.128/256/512.66.0F38.W0 8F /r
//! - VPOPCNTB: EVEX.128/256/512.66.0F38.W0 54 /r
//! - VPOPCNTW: EVEX.128/256/512.66.0F38.W1 54 /r

use crate::common::*;

// ============================================================================
// VPSHUFBITQMB Tests - Shuffle Bits to Mask
// ============================================================================

#[test]
fn test_vpshufbitqmb_xmm_basic() {
    // VPSHUFBITQMB K0, XMM1, XMM2
    // EVEX.128.66.0F38.W0 8F /r
    let code = [
        0x62, 0xF2, 0x75, 0x08, 0x8F, 0xC2, // VPSHUFBITQMB k0, xmm1, xmm2
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpshufbitqmb_xmm_k1() {
    // VPSHUFBITQMB K1, XMM1, XMM2
    let code = [
        0x62, 0xF2, 0x75, 0x08, 0x8F, 0xCA, // VPSHUFBITQMB k1, xmm1, xmm2
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpshufbitqmb_ymm_basic() {
    // VPSHUFBITQMB K0, YMM1, YMM2
    let code = [
        0x62, 0xF2, 0x75, 0x28, 0x8F, 0xC2, // VPSHUFBITQMB k0, ymm1, ymm2
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpshufbitqmb_zmm_basic() {
    // VPSHUFBITQMB K0, ZMM1, ZMM2
    let code = [
        0x62, 0xF2, 0x75, 0x48, 0x8F, 0xC2, // VPSHUFBITQMB k0, zmm1, zmm2
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpshufbitqmb_zmm_high_regs() {
    // VPSHUFBITQMB K2, ZMM16, ZMM17
    let code = [
        0x62, 0xE2, 0x7D, 0x48, 0x8F, 0xD1, // VPSHUFBITQMB k2, zmm16, zmm17
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpshufbitqmb_xmm_memory() {
    // VPSHUFBITQMB K0, XMM1, [RAX]
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x30, 0x00, 0x00, // MOV RAX, 0x3000
        0x62, 0xF2, 0x75, 0x08, 0x8F, 0x00, // VPSHUFBITQMB k0, xmm1, [rax]
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpshufbitqmb_zmm_memory() {
    // VPSHUFBITQMB K0, ZMM1, [RAX]
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x30, 0x00, 0x00, // MOV RAX, 0x3000
        0x62, 0xF2, 0x75, 0x48, 0x8F, 0x00, // VPSHUFBITQMB k0, zmm1, [rax]
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpshufbitqmb_xmm_with_writemask() {
    // VPSHUFBITQMB K0 {K1}, XMM1, XMM2
    let code = [
        0x62, 0xF2, 0x75, 0x09, 0x8F, 0xC2, // VPSHUFBITQMB k0 {k1}, xmm1, xmm2
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// VPOPCNTB Tests - Population Count Bytes
// ============================================================================

#[test]
fn test_vpopcntb_xmm_basic() {
    // VPOPCNTB XMM0, XMM1
    // EVEX.128.66.0F38.W0 54 /r
    let code = [
        0x62, 0xF2, 0x7D, 0x08, 0x54, 0xC1, // VPOPCNTB xmm0, xmm1
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpopcntb_xmm_extended_regs() {
    // VPOPCNTB XMM8, XMM9
    let code = [
        0x62, 0x52, 0x7D, 0x08, 0x54, 0xC1, // VPOPCNTB xmm8, xmm9
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpopcntb_ymm_basic() {
    // VPOPCNTB YMM0, YMM1
    let code = [
        0x62, 0xF2, 0x7D, 0x28, 0x54, 0xC1, // VPOPCNTB ymm0, ymm1
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpopcntb_zmm_basic() {
    // VPOPCNTB ZMM0, ZMM1
    let code = [
        0x62, 0xF2, 0x7D, 0x48, 0x54, 0xC1, // VPOPCNTB zmm0, zmm1
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpopcntb_zmm_high_regs() {
    // VPOPCNTB ZMM24, ZMM25
    let code = [
        0x62, 0x92, 0x7D, 0x48, 0x54, 0xC1, // VPOPCNTB zmm24, zmm25
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpopcntb_xmm_memory() {
    // VPOPCNTB XMM0, [RAX]
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x30, 0x00, 0x00, // MOV RAX, 0x3000
        0x62, 0xF2, 0x7D, 0x08, 0x54, 0x00, // VPOPCNTB xmm0, [rax]
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpopcntb_zmm_memory() {
    // VPOPCNTB ZMM0, [RAX]
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x30, 0x00, 0x00, // MOV RAX, 0x3000
        0x62, 0xF2, 0x7D, 0x48, 0x54, 0x00, // VPOPCNTB zmm0, [rax]
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpopcntb_xmm_with_mask() {
    // VPOPCNTB XMM0 {k1}, XMM1
    let code = [
        0x62, 0xF2, 0x7D, 0x09, 0x54, 0xC1, // VPOPCNTB xmm0 {k1}, xmm1
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpopcntb_zmm_with_mask_zeroing() {
    // VPOPCNTB ZMM0 {k2}{z}, ZMM1
    let code = [
        0x62, 0xF2, 0x7D, 0xCA, 0x54, 0xC1, // VPOPCNTB zmm0 {k2}{z}, zmm1
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// VPOPCNTW Tests - Population Count Words
// ============================================================================

#[test]
fn test_vpopcntw_xmm_basic() {
    // VPOPCNTW XMM0, XMM1
    // EVEX.128.66.0F38.W1 54 /r
    let code = [
        0x62, 0xF2, 0xFD, 0x08, 0x54, 0xC1, // VPOPCNTW xmm0, xmm1
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpopcntw_xmm_extended_regs() {
    // VPOPCNTW XMM12, XMM13
    let code = [
        0x62, 0x52, 0xFD, 0x08, 0x54, 0xE5, // VPOPCNTW xmm12, xmm13
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpopcntw_ymm_basic() {
    // VPOPCNTW YMM0, YMM1
    let code = [
        0x62, 0xF2, 0xFD, 0x28, 0x54, 0xC1, // VPOPCNTW ymm0, ymm1
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpopcntw_zmm_basic() {
    // VPOPCNTW ZMM0, ZMM1
    let code = [
        0x62, 0xF2, 0xFD, 0x48, 0x54, 0xC1, // VPOPCNTW zmm0, zmm1
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpopcntw_zmm_high_regs() {
    // VPOPCNTW ZMM28, ZMM29
    let code = [
        0x62, 0x92, 0xFD, 0x48, 0x54, 0xE5, // VPOPCNTW zmm28, zmm29
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpopcntw_xmm_memory() {
    // VPOPCNTW XMM0, [RAX]
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x30, 0x00, 0x00, // MOV RAX, 0x3000
        0x62, 0xF2, 0xFD, 0x08, 0x54, 0x00, // VPOPCNTW xmm0, [rax]
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpopcntw_zmm_memory() {
    // VPOPCNTW ZMM0, [RAX]
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x30, 0x00, 0x00, // MOV RAX, 0x3000
        0x62, 0xF2, 0xFD, 0x48, 0x54, 0x00, // VPOPCNTW zmm0, [rax]
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpopcntw_ymm_with_mask() {
    // VPOPCNTW YMM0 {k3}, YMM1
    let code = [
        0x62, 0xF2, 0xFD, 0x2B, 0x54, 0xC1, // VPOPCNTW ymm0 {k3}, ymm1
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpopcntw_zmm_with_mask_zeroing() {
    // VPOPCNTW ZMM0 {k4}{z}, ZMM1
    let code = [
        0x62, 0xF2, 0xFD, 0xCC, 0x54, 0xC1, // VPOPCNTW zmm0 {k4}{z}, zmm1
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// Combined BITALG Operations Tests
// ============================================================================

#[test]
fn test_bitalg_all_popcnt_sizes() {
    // Test all popcnt variants on XMM
    let code = [
        0x62, 0xF2, 0x7D, 0x08, 0x54, 0xC1, // VPOPCNTB xmm0, xmm1
        0x62, 0xF2, 0xFD, 0x08, 0x54, 0xD2, // VPOPCNTW xmm2, xmm2
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_bitalg_popcnt_chain_zmm() {
    // Chain popcnt ops on ZMM
    let code = [
        0x62, 0xF2, 0x7D, 0x48, 0x54, 0xC1, // VPOPCNTB zmm0, zmm1
        0x62, 0xF2, 0xFD, 0x48, 0x54, 0xD2, // VPOPCNTW zmm2, zmm2
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_bitalg_shufbit_and_popcnt() {
    // Combine shufbit with popcnt
    let code = [
        0x62, 0xF2, 0x75, 0x48, 0x8F, 0xC2, // VPSHUFBITQMB k0, zmm1, zmm2
        0x62, 0xF2, 0x7D, 0x48, 0x54, 0xC1, // VPOPCNTB zmm0, zmm1
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_bitalg_self_popcnt() {
    // Popcnt of self for all sizes
    let code = [
        0x62, 0xF2, 0x7D, 0x48, 0x54, 0xC0, // VPOPCNTB zmm0, zmm0
        0x62, 0xF2, 0xFD, 0x48, 0x54, 0xC9, // VPOPCNTW zmm1, zmm1
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}
