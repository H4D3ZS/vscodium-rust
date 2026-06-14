//! AVX10.2 Media Acceleration Instructions Tests
//!
//! AVX10.2 introduces new media acceleration instructions for
//! video/image processing and DSP workloads.
//!
//! Instructions covered:
//! - VPDPBSSD - Multiply and add signed bytes to dword
//! - VPDPBSSDS - Same with saturation
//! - VPDPBSUD - Multiply signed/unsigned bytes to dword
//! - VPDPBSUDS - Same with saturation
//! - VPDPBUUD - Multiply unsigned bytes to dword
//! - VPDPBUUDS - Same with saturation
//! - VPDPWSUD - Multiply signed/unsigned words to dword
//! - VPDPWSUDS - Same with saturation
//! - VPDPWUSD - Multiply unsigned/signed words to dword
//! - VPDPWUSDS - Same with saturation
//! - VPDPWUUD - Multiply unsigned words to dword
//! - VPDPWUUDS - Same with saturation

use crate::common::*;

// ============================================================================
// VPDPBSSD Tests - Multiply Signed Bytes to Dword
// ============================================================================

#[test]
fn test_vpdpbssd_xmm_basic() {
    // VPDPBSSD XMM0, XMM1, XMM2
    // EVEX.128.F2.0F38.W0 50 /r
    let code = [
        0x62, 0xF2, 0x77, 0x08, 0x50, 0xC2, // VPDPBSSD xmm0, xmm1, xmm2
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpdpbssd_ymm_basic() {
    // VPDPBSSD YMM0, YMM1, YMM2
    let code = [
        0x62, 0xF2, 0x77, 0x28, 0x50, 0xC2, // VPDPBSSD ymm0, ymm1, ymm2
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpdpbssd_zmm_basic() {
    // VPDPBSSD ZMM0, ZMM1, ZMM2
    let code = [
        0x62, 0xF2, 0x77, 0x48, 0x50, 0xC2, // VPDPBSSD zmm0, zmm1, zmm2
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpdpbssd_xmm_memory() {
    // VPDPBSSD XMM0, XMM1, [RAX]
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x30, 0x00, 0x00, // MOV RAX, 0x3000
        0x62, 0xF2, 0x77, 0x08, 0x50, 0x00, // VPDPBSSD xmm0, xmm1, [rax]
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpdpbssd_zmm_with_mask() {
    // VPDPBSSD ZMM0 {k1}, ZMM1, ZMM2
    let code = [
        0x62, 0xF2, 0x77, 0x49, 0x50, 0xC2, // VPDPBSSD zmm0 {k1}, zmm1, zmm2
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// VPDPBSSDS Tests - With Saturation
// ============================================================================

#[test]
fn test_vpdpbssds_xmm_basic() {
    // VPDPBSSDS XMM0, XMM1, XMM2
    // EVEX.128.F2.0F38.W0 51 /r
    let code = [
        0x62, 0xF2, 0x77, 0x08, 0x51, 0xC2, // VPDPBSSDS xmm0, xmm1, xmm2
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpdpbssds_ymm_basic() {
    // VPDPBSSDS YMM0, YMM1, YMM2
    let code = [
        0x62, 0xF2, 0x77, 0x28, 0x51, 0xC2, // VPDPBSSDS ymm0, ymm1, ymm2
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpdpbssds_zmm_basic() {
    // VPDPBSSDS ZMM0, ZMM1, ZMM2
    let code = [
        0x62, 0xF2, 0x77, 0x48, 0x51, 0xC2, // VPDPBSSDS zmm0, zmm1, zmm2
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// VPDPBSUD Tests - Multiply Signed/Unsigned Bytes
// ============================================================================

#[test]
fn test_vpdpbsud_xmm_basic() {
    // VPDPBSUD XMM0, XMM1, XMM2
    // EVEX.128.F3.0F38.W0 50 /r
    let code = [
        0x62, 0xF2, 0x76, 0x08, 0x50, 0xC2, // VPDPBSUD xmm0, xmm1, xmm2
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpdpbsud_ymm_basic() {
    // VPDPBSUD YMM0, YMM1, YMM2
    let code = [
        0x62, 0xF2, 0x76, 0x28, 0x50, 0xC2, // VPDPBSUD ymm0, ymm1, ymm2
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpdpbsud_zmm_basic() {
    // VPDPBSUD ZMM0, ZMM1, ZMM2
    let code = [
        0x62, 0xF2, 0x76, 0x48, 0x50, 0xC2, // VPDPBSUD zmm0, zmm1, zmm2
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// VPDPBSUDS Tests - With Saturation
// ============================================================================

#[test]
fn test_vpdpbsuds_xmm_basic() {
    // VPDPBSUDS XMM0, XMM1, XMM2
    // EVEX.128.F3.0F38.W0 51 /r
    let code = [
        0x62, 0xF2, 0x76, 0x08, 0x51, 0xC2, // VPDPBSUDS xmm0, xmm1, xmm2
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpdpbsuds_zmm_basic() {
    // VPDPBSUDS ZMM0, ZMM1, ZMM2
    let code = [
        0x62, 0xF2, 0x76, 0x48, 0x51, 0xC2, // VPDPBSUDS zmm0, zmm1, zmm2
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// VPDPBUUD Tests - Multiply Unsigned Bytes
// ============================================================================

#[test]
fn test_vpdpbuud_xmm_basic() {
    // VPDPBUUD XMM0, XMM1, XMM2
    // EVEX.128.NP.0F38.W0 50 /r
    let code = [
        0x62, 0xF2, 0x7C, 0x08, 0x50, 0xC2, // VPDPBUUD xmm0, xmm1, xmm2
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpdpbuud_ymm_basic() {
    // VPDPBUUD YMM0, YMM1, YMM2
    let code = [
        0x62, 0xF2, 0x7C, 0x28, 0x50, 0xC2, // VPDPBUUD ymm0, ymm1, ymm2
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpdpbuud_zmm_basic() {
    // VPDPBUUD ZMM0, ZMM1, ZMM2
    let code = [
        0x62, 0xF2, 0x7C, 0x48, 0x50, 0xC2, // VPDPBUUD zmm0, zmm1, zmm2
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// VPDPBUUDS Tests - With Saturation
// ============================================================================

#[test]
fn test_vpdpbuuds_xmm_basic() {
    // VPDPBUUDS XMM0, XMM1, XMM2
    // EVEX.128.NP.0F38.W0 51 /r
    let code = [
        0x62, 0xF2, 0x7C, 0x08, 0x51, 0xC2, // VPDPBUUDS xmm0, xmm1, xmm2
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpdpbuuds_zmm_basic() {
    // VPDPBUUDS ZMM0, ZMM1, ZMM2
    let code = [
        0x62, 0xF2, 0x7C, 0x48, 0x51, 0xC2, // VPDPBUUDS zmm0, zmm1, zmm2
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// VPDPWSUD Tests - Multiply Signed/Unsigned Words
// ============================================================================

#[test]
fn test_vpdpwsud_xmm_basic() {
    // VPDPWSUD XMM0, XMM1, XMM2
    // EVEX.128.F3.0F38.W0 D2 /r
    let code = [
        0x62, 0xF2, 0x76, 0x08, 0xD2, 0xC2, // VPDPWSUD xmm0, xmm1, xmm2
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpdpwsud_ymm_basic() {
    // VPDPWSUD YMM0, YMM1, YMM2
    let code = [
        0x62, 0xF2, 0x76, 0x28, 0xD2, 0xC2, // VPDPWSUD ymm0, ymm1, ymm2
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpdpwsud_zmm_basic() {
    // VPDPWSUD ZMM0, ZMM1, ZMM2
    let code = [
        0x62, 0xF2, 0x76, 0x48, 0xD2, 0xC2, // VPDPWSUD zmm0, zmm1, zmm2
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// VPDPWSUDS Tests - With Saturation
// ============================================================================

#[test]
fn test_vpdpwsuds_xmm_basic() {
    // VPDPWSUDS XMM0, XMM1, XMM2
    // EVEX.128.F3.0F38.W0 D3 /r
    let code = [
        0x62, 0xF2, 0x76, 0x08, 0xD3, 0xC2, // VPDPWSUDS xmm0, xmm1, xmm2
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpdpwsuds_zmm_basic() {
    // VPDPWSUDS ZMM0, ZMM1, ZMM2
    let code = [
        0x62, 0xF2, 0x76, 0x48, 0xD3, 0xC2, // VPDPWSUDS zmm0, zmm1, zmm2
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// VPDPWUSD Tests - Multiply Unsigned/Signed Words
// ============================================================================

#[test]
fn test_vpdpwusd_xmm_basic() {
    // VPDPWUSD XMM0, XMM1, XMM2
    // EVEX.128.66.0F38.W0 D2 /r
    let code = [
        0x62, 0xF2, 0x75, 0x08, 0xD2, 0xC2, // VPDPWUSD xmm0, xmm1, xmm2
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpdpwusd_ymm_basic() {
    // VPDPWUSD YMM0, YMM1, YMM2
    let code = [
        0x62, 0xF2, 0x75, 0x28, 0xD2, 0xC2, // VPDPWUSD ymm0, ymm1, ymm2
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpdpwusd_zmm_basic() {
    // VPDPWUSD ZMM0, ZMM1, ZMM2
    let code = [
        0x62, 0xF2, 0x75, 0x48, 0xD2, 0xC2, // VPDPWUSD zmm0, zmm1, zmm2
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// VPDPWUSDS Tests - With Saturation
// ============================================================================

#[test]
fn test_vpdpwusds_xmm_basic() {
    // VPDPWUSDS XMM0, XMM1, XMM2
    // EVEX.128.66.0F38.W0 D3 /r
    let code = [
        0x62, 0xF2, 0x75, 0x08, 0xD3, 0xC2, // VPDPWUSDS xmm0, xmm1, xmm2
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpdpwusds_zmm_basic() {
    // VPDPWUSDS ZMM0, ZMM1, ZMM2
    let code = [
        0x62, 0xF2, 0x75, 0x48, 0xD3, 0xC2, // VPDPWUSDS zmm0, zmm1, zmm2
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// VPDPWUUD Tests - Multiply Unsigned Words
// ============================================================================

#[test]
fn test_vpdpwuud_xmm_basic() {
    // VPDPWUUD XMM0, XMM1, XMM2
    // EVEX.128.NP.0F38.W0 D2 /r
    let code = [
        0x62, 0xF2, 0x7C, 0x08, 0xD2, 0xC2, // VPDPWUUD xmm0, xmm1, xmm2
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpdpwuud_ymm_basic() {
    // VPDPWUUD YMM0, YMM1, YMM2
    let code = [
        0x62, 0xF2, 0x7C, 0x28, 0xD2, 0xC2, // VPDPWUUD ymm0, ymm1, ymm2
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpdpwuud_zmm_basic() {
    // VPDPWUUD ZMM0, ZMM1, ZMM2
    let code = [
        0x62, 0xF2, 0x7C, 0x48, 0xD2, 0xC2, // VPDPWUUD zmm0, zmm1, zmm2
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// VPDPWUUDS Tests - With Saturation
// ============================================================================

#[test]
fn test_vpdpwuuds_xmm_basic() {
    // VPDPWUUDS XMM0, XMM1, XMM2
    // EVEX.128.NP.0F38.W0 D3 /r
    let code = [
        0x62, 0xF2, 0x7C, 0x08, 0xD3, 0xC2, // VPDPWUUDS xmm0, xmm1, xmm2
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpdpwuuds_zmm_basic() {
    // VPDPWUUDS ZMM0, ZMM1, ZMM2
    let code = [
        0x62, 0xF2, 0x7C, 0x48, 0xD3, 0xC2, // VPDPWUUDS zmm0, zmm1, zmm2
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// Combined Media Acceleration Tests
// ============================================================================

#[test]
fn test_media_accel_byte_chain() {
    // Chain different byte dot products
    let code = [
        0x62, 0xF2, 0x77, 0x48, 0x50, 0xC2, // VPDPBSSD zmm0, zmm1, zmm2
        0x62, 0xF2, 0x76, 0x48, 0x50, 0xC3, // VPDPBSUD zmm0, zmm1, zmm3
        0x62, 0xF2, 0x7C, 0x48, 0x50, 0xC4, // VPDPBUUD zmm0, zmm1, zmm4
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_media_accel_word_chain() {
    // Chain different word dot products
    let code = [
        0x62, 0xF2, 0x76, 0x48, 0xD2, 0xC2, // VPDPWSUD zmm0, zmm1, zmm2
        0x62, 0xF2, 0x75, 0x48, 0xD2, 0xC3, // VPDPWUSD zmm0, zmm1, zmm3
        0x62, 0xF2, 0x7C, 0x48, 0xD2, 0xC4, // VPDPWUUD zmm0, zmm1, zmm4
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}
