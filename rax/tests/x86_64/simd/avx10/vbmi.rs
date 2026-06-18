//! AVX10.1 VBMI (Vector Byte Manipulation Instructions) Tests
//!
//! VBMI provides byte-level permutation and manipulation operations.
//! These are useful for string processing, compression, and cryptography.
//!
//! Instructions covered:
//! - VPERMB - Permute packed bytes
//! - VPERMI2B - Full permute of bytes from two tables
//! - VPERMT2B - Full permute with table as dest
//! - VPCOMPRESSB - Compress packed bytes
//! - VPEXPANDB - Expand packed bytes
//! - VPSHLDVW/D/Q - Variable packed shift left and merge
//! - VPSHRDVW/D/Q - Variable packed shift right and merge
//!
//! EVEX encoding format:
//! - VPERMB: EVEX.128/256/512.66.0F38.W0 8D /r
//! - VPERMI2B: EVEX.128/256/512.66.0F38.W0 75 /r
//! - VPERMT2B: EVEX.128/256/512.66.0F38.W0 7D /r
//! - VPCOMPRESSB: EVEX.128/256/512.66.0F38.W0 63 /r
//! - VPEXPANDB: EVEX.128/256/512.66.0F38.W0 62 /r

use crate::common::*;

// ============================================================================
// VPERMB Tests - Permute Packed Bytes
// ============================================================================

#[test]
fn test_vpermb_xmm_basic() {
    // VPERMB XMM0, XMM1, XMM2
    // EVEX.128.66.0F38.W0 8D /r
    let code = [
        0x62, 0xF2, 0x75, 0x08, 0x8D, 0xC2, // VPERMB xmm0, xmm1, xmm2
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpermb_xmm_extended_regs() {
    // VPERMB XMM8, XMM9, XMM10
    let code = [
        0x62, 0x52, 0x35, 0x08, 0x8D, 0xC2, // VPERMB xmm8, xmm9, xmm10
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpermb_ymm_basic() {
    // VPERMB YMM0, YMM1, YMM2
    let code = [
        0x62, 0xF2, 0x75, 0x28, 0x8D, 0xC2, // VPERMB ymm0, ymm1, ymm2
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpermb_zmm_basic() {
    // VPERMB ZMM0, ZMM1, ZMM2
    let code = [
        0x62, 0xF2, 0x75, 0x48, 0x8D, 0xC2, // VPERMB zmm0, zmm1, zmm2
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpermb_zmm_high_regs() {
    // VPERMB ZMM24, ZMM25, ZMM26
    let code = [
        0x62, 0x92, 0x35, 0x48, 0x8D, 0xC2, // VPERMB zmm24, zmm25, zmm26
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpermb_xmm_memory() {
    // VPERMB XMM0, XMM1, [RAX]
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x30, 0x00, 0x00, // MOV RAX, 0x3000
        0x62, 0xF2, 0x75, 0x08, 0x8D, 0x00, // VPERMB xmm0, xmm1, [rax]
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpermb_zmm_with_mask() {
    // VPERMB ZMM0 {k1}, ZMM1, ZMM2
    let code = [
        0x62, 0xF2, 0x75, 0x49, 0x8D, 0xC2, // VPERMB zmm0 {k1}, zmm1, zmm2
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// VPERMI2B Tests - Full Permute from Two Tables (Index in dest)
// ============================================================================

#[test]
fn test_vpermi2b_xmm_basic() {
    // VPERMI2B XMM0, XMM1, XMM2
    // EVEX.128.66.0F38.W0 75 /r
    let code = [
        0x62, 0xF2, 0x75, 0x08, 0x75, 0xC2, // VPERMI2B xmm0, xmm1, xmm2
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpermi2b_ymm_basic() {
    // VPERMI2B YMM0, YMM1, YMM2
    let code = [
        0x62, 0xF2, 0x75, 0x28, 0x75, 0xC2, // VPERMI2B ymm0, ymm1, ymm2
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpermi2b_zmm_basic() {
    // VPERMI2B ZMM0, ZMM1, ZMM2
    let code = [
        0x62, 0xF2, 0x75, 0x48, 0x75, 0xC2, // VPERMI2B zmm0, zmm1, zmm2
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpermi2b_xmm_memory() {
    // VPERMI2B XMM0, XMM1, [RAX]
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x30, 0x00, 0x00, // MOV RAX, 0x3000
        0x62, 0xF2, 0x75, 0x08, 0x75, 0x00, // VPERMI2B xmm0, xmm1, [rax]
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpermi2b_zmm_with_mask_zeroing() {
    // VPERMI2B ZMM0 {k2}{z}, ZMM1, ZMM2
    let code = [
        0x62, 0xF2, 0x75, 0xCA, 0x75, 0xC2, // VPERMI2B zmm0 {k2}{z}, zmm1, zmm2
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// VPERMT2B Tests - Full Permute with Table as Dest
// ============================================================================

#[test]
fn test_vpermt2b_xmm_basic() {
    // VPERMT2B XMM0, XMM1, XMM2
    // EVEX.128.66.0F38.W0 7D /r
    let code = [
        0x62, 0xF2, 0x75, 0x08, 0x7D, 0xC2, // VPERMT2B xmm0, xmm1, xmm2
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpermt2b_ymm_basic() {
    // VPERMT2B YMM0, YMM1, YMM2
    let code = [
        0x62, 0xF2, 0x75, 0x28, 0x7D, 0xC2, // VPERMT2B ymm0, ymm1, ymm2
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpermt2b_zmm_basic() {
    // VPERMT2B ZMM0, ZMM1, ZMM2
    let code = [
        0x62, 0xF2, 0x75, 0x48, 0x7D, 0xC2, // VPERMT2B zmm0, zmm1, zmm2
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpermt2b_zmm_memory() {
    // VPERMT2B ZMM0, ZMM1, [RAX]
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x30, 0x00, 0x00, // MOV RAX, 0x3000
        0x62, 0xF2, 0x75, 0x48, 0x7D, 0x00, // VPERMT2B zmm0, zmm1, [rax]
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// VPCOMPRESSB Tests - Compress Packed Bytes
// ============================================================================

#[test]
fn test_vpcompressb_xmm_basic() {
    // VPCOMPRESSB XMM0 {k1}, XMM1
    // EVEX.128.66.0F38.W0 63 /r
    let code = [
        0x62, 0xF2, 0x7D, 0x09, 0x63, 0xC1, // VPCOMPRESSB xmm0 {k1}, xmm1
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpcompressb_ymm_basic() {
    // VPCOMPRESSB YMM0 {k1}, YMM1
    let code = [
        0x62, 0xF2, 0x7D, 0x29, 0x63, 0xC1, // VPCOMPRESSB ymm0 {k1}, ymm1
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpcompressb_zmm_basic() {
    // VPCOMPRESSB ZMM0 {k1}, ZMM1
    let code = [
        0x62, 0xF2, 0x7D, 0x49, 0x63, 0xC1, // VPCOMPRESSB zmm0 {k1}, zmm1
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpcompressb_mem_basic() {
    // VPCOMPRESSB [RAX] {k1}, XMM1
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x30, 0x00, 0x00, // MOV RAX, 0x3000
        0x62, 0xF2, 0x7D, 0x09, 0x63, 0x08, // VPCOMPRESSB [rax] {k1}, xmm1
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// VPEXPANDB Tests - Expand Packed Bytes
// ============================================================================

#[test]
fn test_vpexpandb_xmm_basic() {
    // VPEXPANDB XMM0 {k1}, XMM1
    // EVEX.128.66.0F38.W0 62 /r
    let code = [
        0x62, 0xF2, 0x7D, 0x09, 0x62, 0xC1, // VPEXPANDB xmm0 {k1}, xmm1
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpexpandb_ymm_basic() {
    // VPEXPANDB YMM0 {k1}, YMM1
    let code = [
        0x62, 0xF2, 0x7D, 0x29, 0x62, 0xC1, // VPEXPANDB ymm0 {k1}, ymm1
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpexpandb_zmm_basic() {
    // VPEXPANDB ZMM0 {k1}, ZMM1
    let code = [
        0x62, 0xF2, 0x7D, 0x49, 0x62, 0xC1, // VPEXPANDB zmm0 {k1}, zmm1
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpexpandb_mem_basic() {
    // VPEXPANDB XMM0 {k1}, [RAX]
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x30, 0x00, 0x00, // MOV RAX, 0x3000
        0x62, 0xF2, 0x7D, 0x09, 0x62, 0x00, // VPEXPANDB xmm0 {k1}, [rax]
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// VPSHLDVW Tests - Variable Shift Left and Merge Words
// ============================================================================

#[test]
fn test_vpshldvw_xmm_basic() {
    // VPSHLDVW XMM0, XMM1, XMM2
    // EVEX.128.66.0F38.W1 70 /r
    let code = [
        0x62, 0xF2, 0xF5, 0x08, 0x70, 0xC2, // VPSHLDVW xmm0, xmm1, xmm2
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpshldvw_ymm_basic() {
    // VPSHLDVW YMM0, YMM1, YMM2
    let code = [
        0x62, 0xF2, 0xF5, 0x28, 0x70, 0xC2, // VPSHLDVW ymm0, ymm1, ymm2
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpshldvw_zmm_basic() {
    // VPSHLDVW ZMM0, ZMM1, ZMM2
    let code = [
        0x62, 0xF2, 0xF5, 0x48, 0x70, 0xC2, // VPSHLDVW zmm0, zmm1, zmm2
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// VPSHLDVD Tests - Variable Shift Left and Merge Dwords
// ============================================================================

#[test]
fn test_vpshldvd_xmm_basic() {
    // VPSHLDVD XMM0, XMM1, XMM2
    // EVEX.128.66.0F38.W0 71 /r
    let code = [
        0x62, 0xF2, 0x75, 0x08, 0x71, 0xC2, // VPSHLDVD xmm0, xmm1, xmm2
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpshldvd_ymm_basic() {
    // VPSHLDVD YMM0, YMM1, YMM2
    let code = [
        0x62, 0xF2, 0x75, 0x28, 0x71, 0xC2, // VPSHLDVD ymm0, ymm1, ymm2
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpshldvd_zmm_basic() {
    // VPSHLDVD ZMM0, ZMM1, ZMM2
    let code = [
        0x62, 0xF2, 0x75, 0x48, 0x71, 0xC2, // VPSHLDVD zmm0, zmm1, zmm2
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpshldvd_zmm_broadcast() {
    // VPSHLDVD ZMM0, ZMM1, dword ptr [RAX]{1to16}
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x30, 0x00, 0x00, // MOV RAX, 0x3000
        0x62, 0xF2, 0x75, 0x58, 0x71, 0x00, // VPSHLDVD zmm0, zmm1, [rax]{1to16}
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// VPSHLDVQ Tests - Variable Shift Left and Merge Qwords
// ============================================================================

#[test]
fn test_vpshldvq_xmm_basic() {
    // VPSHLDVQ XMM0, XMM1, XMM2
    // EVEX.128.66.0F38.W1 71 /r
    let code = [
        0x62, 0xF2, 0xF5, 0x08, 0x71, 0xC2, // VPSHLDVQ xmm0, xmm1, xmm2
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpshldvq_ymm_basic() {
    // VPSHLDVQ YMM0, YMM1, YMM2
    let code = [
        0x62, 0xF2, 0xF5, 0x28, 0x71, 0xC2, // VPSHLDVQ ymm0, ymm1, ymm2
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpshldvq_zmm_basic() {
    // VPSHLDVQ ZMM0, ZMM1, ZMM2
    let code = [
        0x62, 0xF2, 0xF5, 0x48, 0x71, 0xC2, // VPSHLDVQ zmm0, zmm1, zmm2
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// VPSHRDVW/D/Q Tests - Variable Shift Right and Merge
// ============================================================================

#[test]
fn test_vpshrdvw_xmm_basic() {
    // VPSHRDVW XMM0, XMM1, XMM2
    // EVEX.128.66.0F38.W1 72 /r
    let code = [
        0x62, 0xF2, 0xF5, 0x08, 0x72, 0xC2, // VPSHRDVW xmm0, xmm1, xmm2
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpshrdvw_zmm_basic() {
    // VPSHRDVW ZMM0, ZMM1, ZMM2
    let code = [
        0x62, 0xF2, 0xF5, 0x48, 0x72, 0xC2, // VPSHRDVW zmm0, zmm1, zmm2
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpshrdvd_xmm_basic() {
    // VPSHRDVD XMM0, XMM1, XMM2
    // EVEX.128.66.0F38.W0 73 /r
    let code = [
        0x62, 0xF2, 0x75, 0x08, 0x73, 0xC2, // VPSHRDVD xmm0, xmm1, xmm2
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpshrdvd_zmm_basic() {
    // VPSHRDVD ZMM0, ZMM1, ZMM2
    let code = [
        0x62, 0xF2, 0x75, 0x48, 0x73, 0xC2, // VPSHRDVD zmm0, zmm1, zmm2
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpshrdvq_xmm_basic() {
    // VPSHRDVQ XMM0, XMM1, XMM2
    // EVEX.128.66.0F38.W1 73 /r
    let code = [
        0x62, 0xF2, 0xF5, 0x08, 0x73, 0xC2, // VPSHRDVQ xmm0, xmm1, xmm2
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpshrdvq_zmm_basic() {
    // VPSHRDVQ ZMM0, ZMM1, ZMM2
    let code = [
        0x62, 0xF2, 0xF5, 0x48, 0x73, 0xC2, // VPSHRDVQ zmm0, zmm1, zmm2
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// Combined VBMI Operations Tests
// ============================================================================

#[test]
fn test_vbmi_permute_chain() {
    // Chain permute operations
    let code = [
        0x62, 0xF2, 0x75, 0x48, 0x8D, 0xC2, // VPERMB zmm0, zmm1, zmm2
        0x62, 0xF2, 0x75, 0x48, 0x75, 0xC3, // VPERMI2B zmm0, zmm1, zmm3
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vbmi_compress_expand_roundtrip() {
    // Compress and expand (conceptual roundtrip)
    let code = [
        0x62, 0xF2, 0x7D, 0x49, 0x63, 0xC1, // VPCOMPRESSB zmm0 {k1}, zmm1
        0x62, 0xF2, 0x7D, 0x49, 0x62, 0xD0, // VPEXPANDB zmm2 {k1}, zmm0
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}
