//! AVX10.2 Copy Sign and Zero-Based Instructions Tests
//!
//! AVX10.2 introduces new instructions for sign manipulation and
//! zero-based move operations.
//!
//! Instructions covered:
//! - VMOVD/VMOVW with zero extension variants
//! - VCVTBIASPH2BF8/BF8S - Convert biased half to BF8
//! - VCVTNE2PH2BF8/BF8S - Convert packed half to BF8
//! - VCVTNEPH2BF8/BF8S - Convert non-exceptional half to BF8
//! - VCVTPH2BF8/BF8S - Convert half to BF8

use crate::common::*;

// ============================================================================
// VMOVD Zero-Extension Tests (New AVX10.2 encodings)
// ============================================================================

#[test]
fn test_vmovd_zero_extend_xmm_gpr() {
    // VMOVD XMM0, EAX (with zero extension to upper lanes)
    // Standard EVEX encoding
    let code = [
        0x62, 0xF1, 0x7D, 0x08, 0x6E, 0xC0, // VMOVD xmm0, eax
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vmovd_zero_extend_gpr_xmm() {
    // VMOVD EAX, XMM0
    let code = [
        0x62, 0xF1, 0x7D, 0x08, 0x7E, 0xC0, // VMOVD eax, xmm0
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vmovd_zero_extend_xmm_memory() {
    // VMOVD XMM0, [RAX]
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x30, 0x00, 0x00, // MOV RAX, 0x3000
        0x62, 0xF1, 0x7D, 0x08, 0x6E, 0x00, // VMOVD xmm0, [rax]
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vmovd_zero_extend_extended_regs() {
    // VMOVD XMM8, R8D
    let code = [
        0x62, 0x51, 0x7D, 0x08, 0x6E, 0xC0, // VMOVD xmm8, r8d
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// VMOVW Tests (New AVX10.2 word move)
// ============================================================================

#[test]
fn test_vmovw_xmm_gpr() {
    // VMOVW XMM0, AX (move word with zero extension)
    // EVEX.128.66.MAP5.W0 6E /r
    let code = [
        0x62, 0xF5, 0x7D, 0x08, 0x6E, 0xC0, // VMOVW xmm0, eax (lower word)
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vmovw_gpr_xmm() {
    // VMOVW AX, XMM0
    let code = [
        0x62, 0xF5, 0x7D, 0x08, 0x7E, 0xC0, // VMOVW eax, xmm0 (lower word)
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vmovw_xmm_memory() {
    // VMOVW XMM0, [RAX]
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x30, 0x00, 0x00, // MOV RAX, 0x3000
        0x62, 0xF5, 0x7D, 0x08, 0x6E, 0x00, // VMOVW xmm0, [rax]
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vmovw_memory_xmm() {
    // VMOVW [RAX], XMM0
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x30, 0x00, 0x00, // MOV RAX, 0x3000
        0x62, 0xF5, 0x7D, 0x08, 0x7E, 0x00, // VMOVW [rax], xmm0
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// VCVTBIASPH2BF8 Tests - Convert Biased Half to BF8
// ============================================================================

#[test]
fn test_vcvtbiasph2bf8_xmm_basic() {
    // VCVTBIASPH2BF8 XMM0, XMM1, XMM2
    // EVEX.128.NP.MAP5.W0 74 /r
    let code = [
        0x62, 0xF5, 0x74, 0x08, 0x74, 0xC2, // VCVTBIASPH2BF8 xmm0, xmm1, xmm2
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vcvtbiasph2bf8_ymm_basic() {
    // VCVTBIASPH2BF8 XMM0, YMM1, YMM2
    let code = [
        0x62, 0xF5, 0x74, 0x28, 0x74, 0xC2, // VCVTBIASPH2BF8 xmm0, ymm1, ymm2
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vcvtbiasph2bf8_zmm_basic() {
    // VCVTBIASPH2BF8 YMM0, ZMM1, ZMM2
    let code = [
        0x62, 0xF5, 0x74, 0x48, 0x74, 0xC2, // VCVTBIASPH2BF8 ymm0, zmm1, zmm2
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vcvtbiasph2bf8_xmm_memory() {
    // VCVTBIASPH2BF8 XMM0, XMM1, [RAX]
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x30, 0x00, 0x00, // MOV RAX, 0x3000
        0x62, 0xF5, 0x74, 0x08, 0x74, 0x00, // VCVTBIASPH2BF8 xmm0, xmm1, [rax]
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// VCVTBIASPH2BF8S Tests - With Saturation
// ============================================================================

#[test]
fn test_vcvtbiasph2bf8s_xmm_basic() {
    // VCVTBIASPH2BF8S XMM0, XMM1, XMM2
    // EVEX.128.66.MAP5.W0 74 /r
    let code = [
        0x62, 0xF5, 0x75, 0x08, 0x74, 0xC2, // VCVTBIASPH2BF8S xmm0, xmm1, xmm2
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vcvtbiasph2bf8s_zmm_basic() {
    // VCVTBIASPH2BF8S YMM0, ZMM1, ZMM2
    let code = [
        0x62, 0xF5, 0x75, 0x48, 0x74, 0xC2, // VCVTBIASPH2BF8S ymm0, zmm1, zmm2
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// VCVTNE2PH2BF8 Tests - Convert Two Packed Half to BF8
// ============================================================================

#[test]
fn test_vcvtne2ph2bf8_xmm_basic() {
    // VCVTNE2PH2BF8 XMM0, XMM1, XMM2
    // EVEX.128.F2.MAP5.W0 74 /r
    let code = [
        0x62, 0xF5, 0x77, 0x08, 0x74, 0xC2, // VCVTNE2PH2BF8 xmm0, xmm1, xmm2
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vcvtne2ph2bf8_ymm_basic() {
    // VCVTNE2PH2BF8 YMM0, YMM1, YMM2
    let code = [
        0x62, 0xF5, 0x77, 0x28, 0x74, 0xC2, // VCVTNE2PH2BF8 ymm0, ymm1, ymm2
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vcvtne2ph2bf8_zmm_basic() {
    // VCVTNE2PH2BF8 ZMM0, ZMM1, ZMM2
    let code = [
        0x62, 0xF5, 0x77, 0x48, 0x74, 0xC2, // VCVTNE2PH2BF8 zmm0, zmm1, zmm2
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// VCVTNE2PH2BF8S Tests - With Saturation
// ============================================================================

#[test]
fn test_vcvtne2ph2bf8s_xmm_basic() {
    // VCVTNE2PH2BF8S XMM0, XMM1, XMM2
    // EVEX.128.F2.MAP6.W0 74 /r
    let code = [
        0x62, 0xF5, 0x57, 0x08, 0x74, 0xC2, // VCVTNE2PH2BF8S xmm0, xmm1, xmm2
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vcvtne2ph2bf8s_zmm_basic() {
    // VCVTNE2PH2BF8S ZMM0, ZMM1, ZMM2
    let code = [
        0x62, 0xF5, 0x57, 0x48, 0x74, 0xC2, // VCVTNE2PH2BF8S zmm0, zmm1, zmm2
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// VCVTNEPH2BF8 Tests - Convert Non-Exceptional Half to BF8
// ============================================================================

#[test]
fn test_vcvtneph2bf8_xmm_basic() {
    // VCVTNEPH2BF8 XMM0, XMM1
    // EVEX.128.F3.MAP5.W0 74 /r
    let code = [
        0x62, 0xF5, 0x7E, 0x08, 0x74, 0xC1, // VCVTNEPH2BF8 xmm0, xmm1
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vcvtneph2bf8_ymm_basic() {
    // VCVTNEPH2BF8 XMM0, YMM1
    let code = [
        0x62, 0xF5, 0x7E, 0x28, 0x74, 0xC1, // VCVTNEPH2BF8 xmm0, ymm1
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vcvtneph2bf8_zmm_basic() {
    // VCVTNEPH2BF8 YMM0, ZMM1
    let code = [
        0x62, 0xF5, 0x7E, 0x48, 0x74, 0xC1, // VCVTNEPH2BF8 ymm0, zmm1
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// VCVTNEPH2BF8S Tests - With Saturation
// ============================================================================

#[test]
fn test_vcvtneph2bf8s_xmm_basic() {
    // VCVTNEPH2BF8S XMM0, XMM1
    // EVEX.128.F3.MAP6.W0 74 /r
    let code = [
        0x62, 0xF5, 0x5E, 0x08, 0x74, 0xC1, // VCVTNEPH2BF8S xmm0, xmm1
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vcvtneph2bf8s_zmm_basic() {
    // VCVTNEPH2BF8S YMM0, ZMM1
    let code = [
        0x62, 0xF5, 0x5E, 0x48, 0x74, 0xC1, // VCVTNEPH2BF8S ymm0, zmm1
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// Combined Copy Sign Tests
// ============================================================================

#[test]
fn test_copy_sign_conversion_chain() {
    // Chain BF8 conversion operations
    let code = [
        0x62, 0xF5, 0x74, 0x48, 0x74, 0xC2, // VCVTBIASPH2BF8 ymm0, zmm1, zmm2
        0x62, 0xF5, 0x77, 0x48, 0x74, 0xD3, // VCVTNE2PH2BF8 zmm2, zmm1, zmm3
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_mov_word_dword_combo() {
    // Test word and dword move combinations
    let code = [
        0x62, 0xF1, 0x7D, 0x08, 0x6E, 0xC0, // VMOVD xmm0, eax
        0x62, 0xF5, 0x7D, 0x08, 0x6E, 0xC9, // VMOVW xmm1, ecx
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}
