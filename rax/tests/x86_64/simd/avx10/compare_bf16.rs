//! AVX10.2 BF16 Comparison Instructions Tests
//!
//! AVX10.2 introduces BF16 scalar comparison instructions for
//! comparing brain float values and setting EFLAGS.
//!
//! Instructions covered:
//! - VCOMSBF16 - Compare scalar BF16 and set EFLAGS (ordered)
//! - VUCOMSBF16 - Compare scalar BF16 and set EFLAGS (unordered)

use crate::common::*;

// ============================================================================
// VCOMSBF16 Tests - Ordered Scalar BF16 Compare
// ============================================================================

#[test]
fn test_vcomsbf16_xmm_basic() {
    // VCOMSBF16 XMM0, XMM1
    // EVEX.LIG.NP.MAP5.W0 2F /r
    let code = [
        0x62, 0xF5, 0x7D, 0x08, 0x2F, 0xC1, // VCOMSBF16 xmm0, xmm1
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vcomsbf16_xmm_extended_regs() {
    // VCOMSBF16 XMM8, XMM9
    let code = [
        0x62, 0x55, 0x7D, 0x08, 0x2F, 0xC1, // VCOMSBF16 xmm8, xmm9
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vcomsbf16_xmm_high_regs() {
    // VCOMSBF16 XMM16, XMM17
    let code = [
        0x62, 0xE5, 0x7D, 0x08, 0x2F, 0xC1, // VCOMSBF16 xmm16, xmm17
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vcomsbf16_xmm_memory() {
    // VCOMSBF16 XMM0, [RAX]
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x30, 0x00, 0x00, // MOV RAX, 0x3000
        0x62, 0xF5, 0x7D, 0x08, 0x2F, 0x00, // VCOMSBF16 xmm0, [rax]
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vcomsbf16_xmm_sae() {
    // VCOMSBF16 XMM0, XMM1, {sae}
    let code = [
        0x62, 0xF5, 0x7D, 0x18, 0x2F, 0xC1, // VCOMSBF16 xmm0, xmm1, {sae}
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vcomsbf16_same_register() {
    // VCOMSBF16 XMM0, XMM0 (compare with self)
    let code = [
        0x62, 0xF5, 0x7D, 0x08, 0x2F, 0xC0, // VCOMSBF16 xmm0, xmm0
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vcomsbf16_cross_reg_range() {
    // VCOMSBF16 XMM7, XMM24
    let code = [
        0x62, 0xD5, 0x7D, 0x08, 0x2F, 0xF8, // VCOMSBF16 xmm7, xmm24
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// VUCOMSBF16 Tests - Unordered Scalar BF16 Compare
// ============================================================================

#[test]
fn test_vucomsbf16_xmm_basic() {
    // VUCOMSBF16 XMM0, XMM1
    // EVEX.LIG.NP.MAP5.W0 2E /r
    let code = [
        0x62, 0xF5, 0x7D, 0x08, 0x2E, 0xC1, // VUCOMSBF16 xmm0, xmm1
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vucomsbf16_xmm_extended_regs() {
    // VUCOMSBF16 XMM8, XMM9
    let code = [
        0x62, 0x55, 0x7D, 0x08, 0x2E, 0xC1, // VUCOMSBF16 xmm8, xmm9
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vucomsbf16_xmm_high_regs() {
    // VUCOMSBF16 XMM20, XMM21
    let code = [
        0x62, 0xE5, 0x7D, 0x08, 0x2E, 0xE5, // VUCOMSBF16 xmm20, xmm21
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vucomsbf16_xmm_memory() {
    // VUCOMSBF16 XMM0, [RAX]
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x30, 0x00, 0x00, // MOV RAX, 0x3000
        0x62, 0xF5, 0x7D, 0x08, 0x2E, 0x00, // VUCOMSBF16 xmm0, [rax]
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vucomsbf16_xmm_sae() {
    // VUCOMSBF16 XMM0, XMM1, {sae}
    let code = [
        0x62, 0xF5, 0x7D, 0x18, 0x2E, 0xC1, // VUCOMSBF16 xmm0, xmm1, {sae}
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vucomsbf16_same_register() {
    // VUCOMSBF16 XMM0, XMM0 (compare with self)
    let code = [
        0x62, 0xF5, 0x7D, 0x08, 0x2E, 0xC0, // VUCOMSBF16 xmm0, xmm0
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vucomsbf16_cross_reg_range() {
    // VUCOMSBF16 XMM15, XMM31
    let code = [
        0x62, 0x95, 0x7D, 0x08, 0x2E, 0xFF, // VUCOMSBF16 xmm15, xmm31
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// Combined BF16 Compare Tests
// ============================================================================

#[test]
fn test_bf16_compare_ordered_vs_unordered() {
    // Compare both ordered and unordered
    let code = [
        0x62, 0xF5, 0x7D, 0x08, 0x2F, 0xC1, // VCOMSBF16 xmm0, xmm1
        0x62, 0xF5, 0x7D, 0x08, 0x2E, 0xC2, // VUCOMSBF16 xmm0, xmm2
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_bf16_compare_chain() {
    // Chain multiple comparisons
    let code = [
        0x62, 0xF5, 0x7D, 0x08, 0x2F, 0xC1, // VCOMSBF16 xmm0, xmm1
        0x62, 0xF5, 0x7D, 0x08, 0x2F, 0xCA, // VCOMSBF16 xmm1, xmm2
        0x62, 0xF5, 0x7D, 0x08, 0x2F, 0xD3, // VCOMSBF16 xmm2, xmm3
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_bf16_compare_with_sae() {
    // Test both comparison types with SAE
    let code = [
        0x62, 0xF5, 0x7D, 0x18, 0x2F, 0xC1, // VCOMSBF16 xmm0, xmm1, {sae}
        0x62, 0xF5, 0x7D, 0x18, 0x2E, 0xC2, // VUCOMSBF16 xmm0, xmm2, {sae}
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_bf16_compare_memory_variations() {
    // Test memory operand variations
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x30, 0x00, 0x00, // MOV RAX, 0x3000
        0x48, 0xC7, 0xC3, 0x10, 0x30, 0x00, 0x00, // MOV RBX, 0x3010
        0x62, 0xF5, 0x7D, 0x08, 0x2F, 0x00, // VCOMSBF16 xmm0, [rax]
        0x62, 0xF5, 0x7D, 0x08, 0x2E, 0x03, // VUCOMSBF16 xmm0, [rbx]
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}
