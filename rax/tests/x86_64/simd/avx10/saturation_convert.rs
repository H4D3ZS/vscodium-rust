//! AVX10.2 Saturation Conversion Instructions Tests
//!
//! AVX10.2 introduces new conversion instructions with saturation,
//! providing better control over overflow handling during type conversion.
//!
//! Instructions covered:
//! - VCVTTPS2IBS - Convert packed single to signed byte with truncation and saturation
//! - VCVTTPS2IUBS - Convert packed single to unsigned byte with truncation and saturation
//! - VCVTTPD2QQS - Convert packed double to signed quadword with truncation and saturation
//! - VCVTTPD2UQQS - Convert packed double to unsigned quadword with truncation and saturation
//! - VCVTTPS2QQS - Convert packed single to signed quadword with truncation and saturation
//! - VCVTTPS2UQQS - Convert packed single to unsigned quadword with truncation and saturation
//! - VCVTTNEBF162IBS - Convert packed BF16 to signed byte with truncation and saturation
//! - VCVTTNEBF162IUBS - Convert packed BF16 to unsigned byte with truncation and saturation

use crate::common::*;

// ============================================================================
// VCVTTPS2IBS Tests - Convert Single to Signed Byte with Saturation
// ============================================================================

#[test]
fn test_vcvttps2ibs_xmm_basic() {
    // VCVTTPS2IBS XMM0, XMM1
    // EVEX.128.NP.MAP5.W0 68 /r
    let code = [
        0x62, 0xF5, 0x7D, 0x08, 0x68, 0xC1, // VCVTTPS2IBS xmm0, xmm1
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vcvttps2ibs_xmm_extended_regs() {
    // VCVTTPS2IBS XMM8, XMM9
    let code = [
        0x62, 0x55, 0x7D, 0x08, 0x68, 0xC1, // VCVTTPS2IBS xmm8, xmm9
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vcvttps2ibs_ymm_basic() {
    // VCVTTPS2IBS YMM0, YMM1
    let code = [
        0x62, 0xF5, 0x7D, 0x28, 0x68, 0xC1, // VCVTTPS2IBS ymm0, ymm1
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vcvttps2ibs_zmm_basic() {
    // VCVTTPS2IBS ZMM0, ZMM1
    let code = [
        0x62, 0xF5, 0x7D, 0x48, 0x68, 0xC1, // VCVTTPS2IBS zmm0, zmm1
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vcvttps2ibs_xmm_memory() {
    // VCVTTPS2IBS XMM0, [RAX]
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x30, 0x00, 0x00, // MOV RAX, 0x3000
        0x62, 0xF5, 0x7D, 0x08, 0x68, 0x00, // VCVTTPS2IBS xmm0, [rax]
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vcvttps2ibs_zmm_with_mask() {
    // VCVTTPS2IBS ZMM0 {k1}, ZMM1
    let code = [
        0x62, 0xF5, 0x7D, 0x49, 0x68, 0xC1, // VCVTTPS2IBS zmm0 {k1}, zmm1
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// VCVTTPS2IUBS Tests - Convert Single to Unsigned Byte with Saturation
// ============================================================================

#[test]
fn test_vcvttps2iubs_xmm_basic() {
    // VCVTTPS2IUBS XMM0, XMM1
    // EVEX.128.66.MAP5.W0 68 /r
    let code = [
        0x62, 0xF5, 0x7D, 0x08, 0x6A, 0xC1, // VCVTTPS2IUBS xmm0, xmm1
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vcvttps2iubs_ymm_basic() {
    // VCVTTPS2IUBS YMM0, YMM1
    let code = [
        0x62, 0xF5, 0x7D, 0x28, 0x6A, 0xC1, // VCVTTPS2IUBS ymm0, ymm1
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vcvttps2iubs_zmm_basic() {
    // VCVTTPS2IUBS ZMM0, ZMM1
    let code = [
        0x62, 0xF5, 0x7D, 0x48, 0x6A, 0xC1, // VCVTTPS2IUBS zmm0, zmm1
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vcvttps2iubs_xmm_memory() {
    // VCVTTPS2IUBS XMM0, [RAX]
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x30, 0x00, 0x00, // MOV RAX, 0x3000
        0x62, 0xF5, 0x7D, 0x08, 0x6A, 0x00, // VCVTTPS2IUBS xmm0, [rax]
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// VCVTTPD2QQS Tests - Convert Double to Signed Qword with Saturation
// ============================================================================

#[test]
fn test_vcvttpd2qqs_xmm_basic() {
    // VCVTTPD2QQS XMM0, XMM1
    // EVEX.128.66.0F.W1 6D /r
    let code = [
        0x62, 0xF5, 0xFD, 0x08, 0x6D, 0xC1, // VCVTTPD2QQS xmm0, xmm1
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vcvttpd2qqs_ymm_basic() {
    // VCVTTPD2QQS YMM0, YMM1
    let code = [
        0x62, 0xF5, 0xFD, 0x28, 0x6D, 0xC1, // VCVTTPD2QQS ymm0, ymm1
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vcvttpd2qqs_zmm_basic() {
    // VCVTTPD2QQS ZMM0, ZMM1
    let code = [
        0x62, 0xF5, 0xFD, 0x48, 0x6D, 0xC1, // VCVTTPD2QQS zmm0, zmm1
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vcvttpd2qqs_xmm_memory() {
    // VCVTTPD2QQS XMM0, [RAX]
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x30, 0x00, 0x00, // MOV RAX, 0x3000
        0x62, 0xF5, 0xFD, 0x08, 0x6D, 0x00, // VCVTTPD2QQS xmm0, [rax]
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vcvttpd2qqs_zmm_broadcast() {
    // VCVTTPD2QQS ZMM0, qword ptr [RAX]{1to8}
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x30, 0x00, 0x00, // MOV RAX, 0x3000
        0x62, 0xF5, 0xFD, 0x58, 0x6D, 0x00, // VCVTTPD2QQS zmm0, [rax]{1to8}
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// VCVTTPD2UQQS Tests - Convert Double to Unsigned Qword with Saturation
// ============================================================================

#[test]
fn test_vcvttpd2uqqs_xmm_basic() {
    // VCVTTPD2UQQS XMM0, XMM1
    // EVEX.128.66.0F.W1 6C /r
    let code = [
        0x62, 0xF5, 0xFD, 0x08, 0x6C, 0xC1, // VCVTTPD2UQQS xmm0, xmm1
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vcvttpd2uqqs_ymm_basic() {
    // VCVTTPD2UQQS YMM0, YMM1
    let code = [
        0x62, 0xF5, 0xFD, 0x28, 0x6C, 0xC1, // VCVTTPD2UQQS ymm0, ymm1
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vcvttpd2uqqs_zmm_basic() {
    // VCVTTPD2UQQS ZMM0, ZMM1
    let code = [
        0x62, 0xF5, 0xFD, 0x48, 0x6C, 0xC1, // VCVTTPD2UQQS zmm0, zmm1
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// VCVTTPS2QQS Tests - Convert Single to Signed Qword with Saturation
// ============================================================================

#[test]
fn test_vcvttps2qqs_xmm_basic() {
    // VCVTTPS2QQS XMM0, XMM1
    // EVEX.128.66.0F.W0 6D /r
    let code = [
        0x62, 0xF5, 0x7D, 0x08, 0x6D, 0xC1, // VCVTTPS2QQS xmm0, xmm1
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vcvttps2qqs_ymm_basic() {
    // VCVTTPS2QQS YMM0, XMM1 (xmm source for ymm dest)
    let code = [
        0x62, 0xF5, 0x7D, 0x28, 0x6D, 0xC1, // VCVTTPS2QQS ymm0, xmm1
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vcvttps2qqs_zmm_basic() {
    // VCVTTPS2QQS ZMM0, YMM1 (ymm source for zmm dest)
    let code = [
        0x62, 0xF5, 0x7D, 0x48, 0x6D, 0xC1, // VCVTTPS2QQS zmm0, ymm1
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// VCVTTPS2UQQS Tests - Convert Single to Unsigned Qword with Saturation
// ============================================================================

#[test]
fn test_vcvttps2uqqs_xmm_basic() {
    // VCVTTPS2UQQS XMM0, XMM1
    // EVEX.128.66.0F.W0 6C /r
    let code = [
        0x62, 0xF5, 0x7D, 0x08, 0x6C, 0xC1, // VCVTTPS2UQQS xmm0, xmm1
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vcvttps2uqqs_ymm_basic() {
    // VCVTTPS2UQQS YMM0, XMM1
    let code = [
        0x62, 0xF5, 0x7D, 0x28, 0x6C, 0xC1, // VCVTTPS2UQQS ymm0, xmm1
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vcvttps2uqqs_zmm_basic() {
    // VCVTTPS2UQQS ZMM0, YMM1
    let code = [
        0x62, 0xF5, 0x7D, 0x48, 0x6C, 0xC1, // VCVTTPS2UQQS zmm0, ymm1
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// VCVTTNEBF162IBS Tests - Convert BF16 to Signed Byte with Saturation
// ============================================================================

#[test]
fn test_vcvttnebf162ibs_xmm_basic() {
    // VCVTTNEBF162IBS XMM0, XMM1
    // EVEX.128.F2.MAP5.W0 68 /r
    let code = [
        0x62, 0xF5, 0x7F, 0x08, 0x68, 0xC1, // VCVTTNEBF162IBS xmm0, xmm1
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vcvttnebf162ibs_ymm_basic() {
    // VCVTTNEBF162IBS YMM0, YMM1
    let code = [
        0x62, 0xF5, 0x7F, 0x28, 0x68, 0xC1, // VCVTTNEBF162IBS ymm0, ymm1
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vcvttnebf162ibs_zmm_basic() {
    // VCVTTNEBF162IBS ZMM0, ZMM1
    let code = [
        0x62, 0xF5, 0x7F, 0x48, 0x68, 0xC1, // VCVTTNEBF162IBS zmm0, zmm1
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// VCVTTNEBF162IUBS Tests - Convert BF16 to Unsigned Byte with Saturation
// ============================================================================

#[test]
fn test_vcvttnebf162iubs_xmm_basic() {
    // VCVTTNEBF162IUBS XMM0, XMM1
    // EVEX.128.F3.MAP5.W0 68 /r
    let code = [
        0x62, 0xF5, 0x7E, 0x08, 0x68, 0xC1, // VCVTTNEBF162IUBS xmm0, xmm1
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vcvttnebf162iubs_ymm_basic() {
    // VCVTTNEBF162IUBS YMM0, YMM1
    let code = [
        0x62, 0xF5, 0x7E, 0x28, 0x68, 0xC1, // VCVTTNEBF162IUBS ymm0, ymm1
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vcvttnebf162iubs_zmm_basic() {
    // VCVTTNEBF162IUBS ZMM0, ZMM1
    let code = [
        0x62, 0xF5, 0x7E, 0x48, 0x68, 0xC1, // VCVTTNEBF162IUBS zmm0, zmm1
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// Combined Saturation Conversion Tests
// ============================================================================

#[test]
fn test_saturation_convert_chain() {
    // Chain different saturation conversions
    let code = [
        0x62, 0xF5, 0x7D, 0x48, 0x68, 0xC1, // VCVTTPS2IBS zmm0, zmm1
        0x62, 0xF5, 0xFD, 0x48, 0x6D, 0xD2, // VCVTTPD2QQS zmm2, zmm2
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_saturation_convert_all_sizes() {
    // Test saturation conversion on all vector sizes
    let code = [
        0x62, 0xF5, 0x7D, 0x08, 0x68, 0xC1, // VCVTTPS2IBS xmm0, xmm1
        0x62, 0xF5, 0x7D, 0x28, 0x68, 0xD2, // VCVTTPS2IBS ymm2, ymm2
        0x62, 0xF5, 0x7D, 0x48, 0x68, 0xE3, // VCVTTPS2IBS zmm4, zmm3
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}
