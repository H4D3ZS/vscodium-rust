//! AVX10.2 VMINMAX Instructions Tests
//!
//! AVX10.2 introduces new min/max instructions with comparison predicates
//! for better control over min/max semantics (particularly for NaN handling).
//!
//! Instructions covered:
//! - VMINMAXPS - Min/max packed single with predicate
//! - VMINMAXPD - Min/max packed double with predicate
//! - VMINMAXPH - Min/max packed half precision
//! - VMINMAXSS - Min/max scalar single with predicate
//! - VMINMAXSD - Min/max scalar double with predicate
//! - VMINMAXSH - Min/max scalar half precision
//!
//! The immediate byte controls the operation:
//! - Bits [3:0]: Control min/max selection and NaN handling
//! - Bit 4: Sign control for -0/+0 handling

use crate::common::*;

// ============================================================================
// VMINMAXPS Tests - Packed Single Min/Max with Predicate
// ============================================================================

#[test]
fn test_vminmaxps_xmm_basic() {
    // VMINMAXPS XMM0, XMM1, XMM2, imm8
    // EVEX.128.66.0F3A.W0 52 /r ib
    let code = [
        0x62, 0xF3, 0x75, 0x08, 0x52, 0xC2, 0x00, // VMINMAXPS xmm0, xmm1, xmm2, 0
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vminmaxps_xmm_min_mode() {
    // VMINMAXPS XMM0, XMM1, XMM2, 0x00 (min mode)
    let code = [
        0x62, 0xF3, 0x75, 0x08, 0x52, 0xC2, 0x00, // VMINMAXPS xmm0, xmm1, xmm2, 0
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vminmaxps_xmm_max_mode() {
    // VMINMAXPS XMM0, XMM1, XMM2, 0x01 (max mode)
    let code = [
        0x62, 0xF3, 0x75, 0x08, 0x52, 0xC2, 0x01, // VMINMAXPS xmm0, xmm1, xmm2, 1
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vminmaxps_xmm_extended_regs() {
    // VMINMAXPS XMM8, XMM9, XMM10, 0x02
    let code = [
        0x62, 0x53, 0x35, 0x08, 0x52, 0xC2, 0x02, // VMINMAXPS xmm8, xmm9, xmm10, 2
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vminmaxps_ymm_basic() {
    // VMINMAXPS YMM0, YMM1, YMM2, imm8
    let code = [
        0x62, 0xF3, 0x75, 0x28, 0x52, 0xC2, 0x03, // VMINMAXPS ymm0, ymm1, ymm2, 3
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vminmaxps_zmm_basic() {
    // VMINMAXPS ZMM0, ZMM1, ZMM2, imm8
    let code = [
        0x62, 0xF3, 0x75, 0x48, 0x52, 0xC2, 0x04, // VMINMAXPS zmm0, zmm1, zmm2, 4
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vminmaxps_zmm_high_regs() {
    // VMINMAXPS ZMM24, ZMM25, ZMM26, imm8
    let code = [
        0x62, 0x93, 0x35, 0x48, 0x52, 0xC2, 0x05, // VMINMAXPS zmm24, zmm25, zmm26, 5
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vminmaxps_xmm_memory() {
    // VMINMAXPS XMM0, XMM1, [RAX], imm8
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x30, 0x00, 0x00, // MOV RAX, 0x3000
        0x62, 0xF3, 0x75, 0x08, 0x52, 0x00, 0x06, // VMINMAXPS xmm0, xmm1, [rax], 6
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vminmaxps_zmm_with_mask() {
    // VMINMAXPS ZMM0 {k1}, ZMM1, ZMM2, imm8
    let code = [
        0x62, 0xF3, 0x75, 0x49, 0x52, 0xC2, 0x07, // VMINMAXPS zmm0 {k1}, zmm1, zmm2, 7
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vminmaxps_zmm_with_mask_zeroing() {
    // VMINMAXPS ZMM0 {k2}{z}, ZMM1, ZMM2, imm8
    let code = [
        0x62, 0xF3, 0x75, 0xCA, 0x52, 0xC2, 0x08, // VMINMAXPS zmm0 {k2}{z}, zmm1, zmm2, 8
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vminmaxps_zmm_embedded_rounding() {
    // VMINMAXPS ZMM0, ZMM1, ZMM2, imm8, {sae}
    let code = [
        0x62, 0xF3, 0x75, 0x18, 0x52, 0xC2, 0x09, // VMINMAXPS zmm0, zmm1, zmm2, 9, {sae}
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// VMINMAXPD Tests - Packed Double Min/Max with Predicate
// ============================================================================

#[test]
fn test_vminmaxpd_xmm_basic() {
    // VMINMAXPD XMM0, XMM1, XMM2, imm8
    // EVEX.128.66.0F3A.W1 52 /r ib
    let code = [
        0x62, 0xF3, 0xF5, 0x08, 0x52, 0xC2, 0x00, // VMINMAXPD xmm0, xmm1, xmm2, 0
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vminmaxpd_ymm_basic() {
    // VMINMAXPD YMM0, YMM1, YMM2, imm8
    let code = [
        0x62, 0xF3, 0xF5, 0x28, 0x52, 0xC2, 0x01, // VMINMAXPD ymm0, ymm1, ymm2, 1
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vminmaxpd_zmm_basic() {
    // VMINMAXPD ZMM0, ZMM1, ZMM2, imm8
    let code = [
        0x62, 0xF3, 0xF5, 0x48, 0x52, 0xC2, 0x02, // VMINMAXPD zmm0, zmm1, zmm2, 2
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vminmaxpd_xmm_memory() {
    // VMINMAXPD XMM0, XMM1, [RAX], imm8
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x30, 0x00, 0x00, // MOV RAX, 0x3000
        0x62, 0xF3, 0xF5, 0x08, 0x52, 0x00, 0x03, // VMINMAXPD xmm0, xmm1, [rax], 3
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vminmaxpd_zmm_broadcast() {
    // VMINMAXPD ZMM0, ZMM1, qword ptr [RAX]{1to8}, imm8
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x30, 0x00, 0x00, // MOV RAX, 0x3000
        0x62, 0xF3, 0xF5, 0x58, 0x52, 0x00, 0x04, // VMINMAXPD zmm0, zmm1, [rax]{1to8}, 4
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// VMINMAXPH Tests - Packed Half Precision Min/Max
// ============================================================================

#[test]
fn test_vminmaxph_xmm_basic() {
    // VMINMAXPH XMM0, XMM1, XMM2, imm8
    // EVEX.128.NP.0F3A.W0 52 /r ib
    let code = [
        0x62, 0xF3, 0x7C, 0x08, 0x52, 0xC2, 0x00, // VMINMAXPH xmm0, xmm1, xmm2, 0
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vminmaxph_ymm_basic() {
    // VMINMAXPH YMM0, YMM1, YMM2, imm8
    let code = [
        0x62, 0xF3, 0x7C, 0x28, 0x52, 0xC2, 0x01, // VMINMAXPH ymm0, ymm1, ymm2, 1
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vminmaxph_zmm_basic() {
    // VMINMAXPH ZMM0, ZMM1, ZMM2, imm8
    let code = [
        0x62, 0xF3, 0x7C, 0x48, 0x52, 0xC2, 0x02, // VMINMAXPH zmm0, zmm1, zmm2, 2
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// VMINMAXSS/SD/SH Tests - Scalar Min/Max
// ============================================================================

#[test]
fn test_vminmaxss_xmm_basic() {
    // VMINMAXSS XMM0, XMM1, XMM2, imm8
    // EVEX.LIG.66.0F3A.W0 53 /r ib
    let code = [
        0x62, 0xF3, 0x75, 0x08, 0x53, 0xC2, 0x00, // VMINMAXSS xmm0, xmm1, xmm2, 0
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vminmaxss_xmm_extended_regs() {
    // VMINMAXSS XMM8, XMM9, XMM10, imm8
    let code = [
        0x62, 0x53, 0x35, 0x08, 0x53, 0xC2, 0x01, // VMINMAXSS xmm8, xmm9, xmm10, 1
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vminmaxss_xmm_memory() {
    // VMINMAXSS XMM0, XMM1, [RAX], imm8
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x30, 0x00, 0x00, // MOV RAX, 0x3000
        0x62, 0xF3, 0x75, 0x08, 0x53, 0x00, 0x02, // VMINMAXSS xmm0, xmm1, [rax], 2
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vminmaxss_xmm_sae() {
    // VMINMAXSS XMM0, XMM1, XMM2, imm8, {sae}
    let code = [
        0x62, 0xF3, 0x75, 0x18, 0x53, 0xC2, 0x03, // VMINMAXSS xmm0, xmm1, xmm2, 3, {sae}
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vminmaxsd_xmm_basic() {
    // VMINMAXSD XMM0, XMM1, XMM2, imm8
    // EVEX.LIG.66.0F3A.W1 53 /r ib
    let code = [
        0x62, 0xF3, 0xF5, 0x08, 0x53, 0xC2, 0x00, // VMINMAXSD xmm0, xmm1, xmm2, 0
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vminmaxsd_xmm_memory() {
    // VMINMAXSD XMM0, XMM1, [RAX], imm8
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x30, 0x00, 0x00, // MOV RAX, 0x3000
        0x62, 0xF3, 0xF5, 0x08, 0x53, 0x00, 0x01, // VMINMAXSD xmm0, xmm1, [rax], 1
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vminmaxsh_xmm_basic() {
    // VMINMAXSH XMM0, XMM1, XMM2, imm8
    // EVEX.LIG.NP.0F3A.W0 53 /r ib
    let code = [
        0x62, 0xF3, 0x7C, 0x08, 0x53, 0xC2, 0x00, // VMINMAXSH xmm0, xmm1, xmm2, 0
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vminmaxsh_xmm_with_mask() {
    // VMINMAXSH XMM0 {k1}, XMM1, XMM2, imm8
    let code = [
        0x62, 0xF3, 0x7C, 0x09, 0x53, 0xC2, 0x01, // VMINMAXSH xmm0 {k1}, xmm1, xmm2, 1
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// Combined VMINMAX Tests
// ============================================================================

#[test]
fn test_vminmax_all_modes() {
    // Test various immediate values representing different min/max modes
    let code = [
        0x62, 0xF3, 0x75, 0x48, 0x52, 0xC2, 0x00, // VMINMAXPS zmm0, zmm1, zmm2, 0 (min)
        0x62, 0xF3, 0x75, 0x48, 0x52, 0xD3, 0x01, // VMINMAXPS zmm2, zmm1, zmm3, 1 (max)
        0x62, 0xF3, 0x75, 0x48, 0x52, 0xE4, 0x04, // VMINMAXPS zmm4, zmm1, zmm4, 4
        0x62, 0xF3, 0x75, 0x48, 0x52, 0xED, 0x05, // VMINMAXPS zmm5, zmm1, zmm5, 5
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vminmax_chain() {
    // Chain min and max operations
    let code = [
        0x62, 0xF3, 0x75, 0x48, 0x52, 0xC2, 0x00, // VMINMAXPS zmm0, zmm1, zmm2, 0 (min)
        0x62, 0xF3, 0x7D, 0x48, 0x52, 0xC3, 0x01, // VMINMAXPS zmm0, zmm0, zmm3, 1 (max)
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vminmax_scalar_chain() {
    // Chain scalar min/max operations
    let code = [
        0x62, 0xF3, 0x75, 0x08, 0x53, 0xC2, 0x00, // VMINMAXSS xmm0, xmm1, xmm2, 0
        0x62, 0xF3, 0xF5, 0x08, 0x53, 0xD3, 0x01, // VMINMAXSD xmm2, xmm1, xmm3, 1
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}
