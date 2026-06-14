//! AVX10.1 YMM Embedded Rounding Tests
//!
//! AVX10 extends embedded rounding control (formerly ZMM-only in AVX-512)
//! to YMM vector instructions. This allows specifying rounding mode in
//! the instruction encoding rather than modifying MXCSR.
//!
//! Rounding modes (in EVEX.L'L bits when b=1):
//! - 00 = Round to nearest (even) - {rn-sae}
//! - 01 = Round down (toward -inf) - {rd-sae}
//! - 10 = Round up (toward +inf) - {ru-sae}
//! - 11 = Round toward zero (truncate) - {rz-sae}
//!
//! This tests the new AVX10.1 YMM embedded rounding capability.
//! Previously only ZMM instructions could use embedded rounding.

use crate::common::*;

// ============================================================================
// VADDPS YMM with Embedded Rounding Tests
// ============================================================================

#[test]
fn test_vaddps_ymm_rn_sae() {
    // VADDPS YMM0, YMM1, YMM2, {rn-sae}
    // Round to nearest (even)
    let code = [
        0x62, 0xF1, 0x74, 0x18, 0x58, 0xC2, // VADDPS ymm0, ymm1, ymm2, {rn-sae}
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vaddps_ymm_rd_sae() {
    // VADDPS YMM0, YMM1, YMM2, {rd-sae}
    // Round down (toward -inf)
    let code = [
        0x62, 0xF1, 0x74, 0x38, 0x58, 0xC2, // VADDPS ymm0, ymm1, ymm2, {rd-sae}
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vaddps_ymm_ru_sae() {
    // VADDPS YMM0, YMM1, YMM2, {ru-sae}
    // Round up (toward +inf)
    let code = [
        0x62, 0xF1, 0x74, 0x58, 0x58, 0xC2, // VADDPS ymm0, ymm1, ymm2, {ru-sae}
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vaddps_ymm_rz_sae() {
    // VADDPS YMM0, YMM1, YMM2, {rz-sae}
    // Round toward zero (truncate)
    let code = [
        0x62, 0xF1, 0x74, 0x78, 0x58, 0xC2, // VADDPS ymm0, ymm1, ymm2, {rz-sae}
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vaddps_ymm_extended_regs_rn() {
    // VADDPS YMM16, YMM17, YMM18, {rn-sae}
    let code = [
        0x62, 0xE1, 0x74, 0x18, 0x58, 0xC2, // VADDPS ymm16, ymm17, ymm18, {rn-sae}
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// VADDPD YMM with Embedded Rounding Tests
// ============================================================================

#[test]
fn test_vaddpd_ymm_rn_sae() {
    // VADDPD YMM0, YMM1, YMM2, {rn-sae}
    let code = [
        0x62, 0xF1, 0xF5, 0x18, 0x58, 0xC2, // VADDPD ymm0, ymm1, ymm2, {rn-sae}
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vaddpd_ymm_rd_sae() {
    // VADDPD YMM0, YMM1, YMM2, {rd-sae}
    let code = [
        0x62, 0xF1, 0xF5, 0x38, 0x58, 0xC2, // VADDPD ymm0, ymm1, ymm2, {rd-sae}
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vaddpd_ymm_ru_sae() {
    // VADDPD YMM0, YMM1, YMM2, {ru-sae}
    let code = [
        0x62, 0xF1, 0xF5, 0x58, 0x58, 0xC2, // VADDPD ymm0, ymm1, ymm2, {ru-sae}
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vaddpd_ymm_rz_sae() {
    // VADDPD YMM0, YMM1, YMM2, {rz-sae}
    let code = [
        0x62, 0xF1, 0xF5, 0x78, 0x58, 0xC2, // VADDPD ymm0, ymm1, ymm2, {rz-sae}
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// VMULPS YMM with Embedded Rounding Tests
// ============================================================================

#[test]
fn test_vmulps_ymm_rn_sae() {
    // VMULPS YMM0, YMM1, YMM2, {rn-sae}
    let code = [
        0x62, 0xF1, 0x74, 0x18, 0x59, 0xC2, // VMULPS ymm0, ymm1, ymm2, {rn-sae}
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vmulps_ymm_rd_sae() {
    // VMULPS YMM0, YMM1, YMM2, {rd-sae}
    let code = [
        0x62, 0xF1, 0x74, 0x38, 0x59, 0xC2, // VMULPS ymm0, ymm1, ymm2, {rd-sae}
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vmulps_ymm_ru_sae() {
    // VMULPS YMM0, YMM1, YMM2, {ru-sae}
    let code = [
        0x62, 0xF1, 0x74, 0x58, 0x59, 0xC2, // VMULPS ymm0, ymm1, ymm2, {ru-sae}
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vmulps_ymm_rz_sae() {
    // VMULPS YMM0, YMM1, YMM2, {rz-sae}
    let code = [
        0x62, 0xF1, 0x74, 0x78, 0x59, 0xC2, // VMULPS ymm0, ymm1, ymm2, {rz-sae}
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// VMULPD YMM with Embedded Rounding Tests
// ============================================================================

#[test]
fn test_vmulpd_ymm_rn_sae() {
    // VMULPD YMM0, YMM1, YMM2, {rn-sae}
    let code = [
        0x62, 0xF1, 0xF5, 0x18, 0x59, 0xC2, // VMULPD ymm0, ymm1, ymm2, {rn-sae}
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vmulpd_ymm_rz_sae() {
    // VMULPD YMM0, YMM1, YMM2, {rz-sae}
    let code = [
        0x62, 0xF1, 0xF5, 0x78, 0x59, 0xC2, // VMULPD ymm0, ymm1, ymm2, {rz-sae}
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// VDIVPS/VDIVPD YMM with Embedded Rounding Tests
// ============================================================================

#[test]
fn test_vdivps_ymm_rn_sae() {
    // VDIVPS YMM0, YMM1, YMM2, {rn-sae}
    let code = [
        0x62, 0xF1, 0x74, 0x18, 0x5E, 0xC2, // VDIVPS ymm0, ymm1, ymm2, {rn-sae}
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vdivps_ymm_rz_sae() {
    // VDIVPS YMM0, YMM1, YMM2, {rz-sae}
    let code = [
        0x62, 0xF1, 0x74, 0x78, 0x5E, 0xC2, // VDIVPS ymm0, ymm1, ymm2, {rz-sae}
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vdivpd_ymm_rn_sae() {
    // VDIVPD YMM0, YMM1, YMM2, {rn-sae}
    let code = [
        0x62, 0xF1, 0xF5, 0x18, 0x5E, 0xC2, // VDIVPD ymm0, ymm1, ymm2, {rn-sae}
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vdivpd_ymm_rd_sae() {
    // VDIVPD YMM0, YMM1, YMM2, {rd-sae}
    let code = [
        0x62, 0xF1, 0xF5, 0x38, 0x5E, 0xC2, // VDIVPD ymm0, ymm1, ymm2, {rd-sae}
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// VSQRTPS/VSQRTPD YMM with Embedded Rounding Tests
// ============================================================================

#[test]
fn test_vsqrtps_ymm_rn_sae() {
    // VSQRTPS YMM0, YMM1, {rn-sae}
    let code = [
        0x62, 0xF1, 0x7C, 0x18, 0x51, 0xC1, // VSQRTPS ymm0, ymm1, {rn-sae}
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vsqrtps_ymm_rz_sae() {
    // VSQRTPS YMM0, YMM1, {rz-sae}
    let code = [
        0x62, 0xF1, 0x7C, 0x78, 0x51, 0xC1, // VSQRTPS ymm0, ymm1, {rz-sae}
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vsqrtpd_ymm_rn_sae() {
    // VSQRTPD YMM0, YMM1, {rn-sae}
    let code = [
        0x62, 0xF1, 0xFD, 0x18, 0x51, 0xC1, // VSQRTPD ymm0, ymm1, {rn-sae}
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vsqrtpd_ymm_ru_sae() {
    // VSQRTPD YMM0, YMM1, {ru-sae}
    let code = [
        0x62, 0xF1, 0xFD, 0x58, 0x51, 0xC1, // VSQRTPD ymm0, ymm1, {ru-sae}
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// VCVTPS2PD/VCVTPD2PS YMM with Embedded Rounding Tests
// ============================================================================

#[test]
fn test_vcvtps2pd_ymm_sae() {
    // VCVTPS2PD YMM0, XMM1, {sae}
    let code = [
        0x62, 0xF1, 0x7C, 0x18, 0x5A, 0xC1, // VCVTPS2PD ymm0, xmm1, {sae}
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vcvtpd2ps_ymm_rn_sae() {
    // VCVTPD2PS XMM0, YMM1, {rn-sae}
    let code = [
        0x62, 0xF1, 0xFD, 0x18, 0x5A, 0xC1, // VCVTPD2PS xmm0, ymm1, {rn-sae}
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vcvtpd2ps_ymm_rz_sae() {
    // VCVTPD2PS XMM0, YMM1, {rz-sae}
    let code = [
        0x62, 0xF1, 0xFD, 0x78, 0x5A, 0xC1, // VCVTPD2PS xmm0, ymm1, {rz-sae}
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// VFMADD YMM with Embedded Rounding Tests
// ============================================================================

#[test]
fn test_vfmadd132ps_ymm_rn_sae() {
    // VFMADD132PS YMM0, YMM1, YMM2, {rn-sae}
    let code = [
        0x62, 0xF2, 0x75, 0x18, 0x98, 0xC2, // VFMADD132PS ymm0, ymm1, ymm2, {rn-sae}
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vfmadd132ps_ymm_rz_sae() {
    // VFMADD132PS YMM0, YMM1, YMM2, {rz-sae}
    let code = [
        0x62, 0xF2, 0x75, 0x78, 0x98, 0xC2, // VFMADD132PS ymm0, ymm1, ymm2, {rz-sae}
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vfmadd213pd_ymm_rn_sae() {
    // VFMADD213PD YMM0, YMM1, YMM2, {rn-sae}
    let code = [
        0x62, 0xF2, 0xF5, 0x18, 0xA8, 0xC2, // VFMADD213PD ymm0, ymm1, ymm2, {rn-sae}
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vfmadd231ps_ymm_rd_sae() {
    // VFMADD231PS YMM0, YMM1, YMM2, {rd-sae}
    let code = [
        0x62, 0xF2, 0x75, 0x38, 0xB8, 0xC2, // VFMADD231PS ymm0, ymm1, ymm2, {rd-sae}
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// Combined YMM Embedded Rounding Tests
// ============================================================================

#[test]
fn test_ymm_er_all_rounding_modes_chain() {
    // Test chain of operations with different rounding modes
    let code = [
        0x62, 0xF1, 0x74, 0x18, 0x58, 0xC1, // VADDPS ymm0, ymm1, ymm1, {rn-sae}
        0x62, 0xF1, 0x74, 0x38, 0x59, 0xD2, // VMULPS ymm2, ymm1, ymm2, {rd-sae}
        0x62, 0xF1, 0x74, 0x58, 0x5E, 0xDB, // VDIVPS ymm3, ymm1, ymm3, {ru-sae}
        0x62, 0xF1, 0x7C, 0x78, 0x51, 0xE4, // VSQRTPS ymm4, ymm4, {rz-sae}
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_ymm_er_with_extended_regs() {
    // Test embedded rounding with extended YMM registers
    let code = [
        0x62, 0xE1, 0x74, 0x18, 0x58, 0xC1, // VADDPS ymm16, ymm17, ymm17, {rn-sae}
        0x62, 0xE1, 0xF5, 0x78, 0x59, 0xD2, // VMULPD ymm18, ymm17, ymm18, {rz-sae}
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_ymm_er_with_mask() {
    // Test embedded rounding with masking
    let code = [
        0x62, 0xF1, 0x74, 0x19, 0x58, 0xC2, // VADDPS ymm0 {k1}, ymm1, ymm2, {rn-sae}
        0x62, 0xF1, 0x74, 0xB9, 0x59, 0xC3, // VMULPS ymm0 {k1}{z}, ymm1, ymm3, {rd-sae}
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}
