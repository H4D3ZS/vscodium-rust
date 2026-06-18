//! AVX10.1 BF16 (Brain Float 16) Tests
//!
//! BF16 provides conversions and operations for brain float format,
//! which is useful for machine learning inference workloads.
//!
//! Instructions covered:
//! - VCVTNEPS2BF16 - Convert packed single to BF16
//! - VCVTNE2PS2BF16 - Convert two packed singles to BF16
//! - VDPBF16PS - Dot product of BF16 pairs, accumulate to single
//!
//! EVEX encoding format:
//! - VCVTNEPS2BF16: EVEX.128/256/512.F3.0F38.W0 72 /r
//! - VCVTNE2PS2BF16: EVEX.128/256/512.F2.0F38.W0 72 /r
//! - VDPBF16PS: EVEX.128/256/512.F3.0F38.W0 52 /r

use crate::common::*;

// ============================================================================
// VCVTNEPS2BF16 Tests - Convert Single to BF16
// ============================================================================

#[test]
fn test_vcvtneps2bf16_xmm_basic() {
    // VCVTNEPS2BF16 XMM0, XMM1
    // EVEX.128.F3.0F38.W0 72 /r
    let code = [
        0x62, 0xF2, 0x7E, 0x08, 0x72, 0xC1, // VCVTNEPS2BF16 xmm0, xmm1
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vcvtneps2bf16_xmm_extended_regs() {
    // VCVTNEPS2BF16 XMM8, XMM9
    let code = [
        0x62, 0x52, 0x7E, 0x08, 0x72, 0xC1, // VCVTNEPS2BF16 xmm8, xmm9
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vcvtneps2bf16_xmm_from_ymm() {
    // VCVTNEPS2BF16 XMM0, YMM1 (256-bit source to 128-bit dest)
    let code = [
        0x62, 0xF2, 0x7E, 0x28, 0x72, 0xC1, // VCVTNEPS2BF16 xmm0, ymm1
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vcvtneps2bf16_ymm_from_zmm() {
    // VCVTNEPS2BF16 YMM0, ZMM1 (512-bit source to 256-bit dest)
    let code = [
        0x62, 0xF2, 0x7E, 0x48, 0x72, 0xC1, // VCVTNEPS2BF16 ymm0, zmm1
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vcvtneps2bf16_xmm_memory() {
    // VCVTNEPS2BF16 XMM0, [RAX] (128-bit memory source)
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x30, 0x00, 0x00, // MOV RAX, 0x3000
        0x62, 0xF2, 0x7E, 0x08, 0x72, 0x00, // VCVTNEPS2BF16 xmm0, [rax]
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vcvtneps2bf16_ymm_memory() {
    // VCVTNEPS2BF16 XMM0, [RAX] (256-bit memory source)
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x30, 0x00, 0x00, // MOV RAX, 0x3000
        0x62, 0xF2, 0x7E, 0x28, 0x72, 0x00, // VCVTNEPS2BF16 xmm0, [rax]
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vcvtneps2bf16_zmm_memory() {
    // VCVTNEPS2BF16 YMM0, [RAX] (512-bit memory source)
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x30, 0x00, 0x00, // MOV RAX, 0x3000
        0x62, 0xF2, 0x7E, 0x48, 0x72, 0x00, // VCVTNEPS2BF16 ymm0, [rax]
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vcvtneps2bf16_xmm_with_mask() {
    // VCVTNEPS2BF16 XMM0 {k1}, XMM1
    let code = [
        0x62, 0xF2, 0x7E, 0x09, 0x72, 0xC1, // VCVTNEPS2BF16 xmm0 {k1}, xmm1
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vcvtneps2bf16_ymm_with_mask_zeroing() {
    // VCVTNEPS2BF16 YMM0 {k2}{z}, ZMM1
    let code = [
        0x62, 0xF2, 0x7E, 0xCA, 0x72, 0xC1, // VCVTNEPS2BF16 ymm0 {k2}{z}, zmm1
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vcvtneps2bf16_high_regs() {
    // VCVTNEPS2BF16 YMM16, ZMM24
    let code = [
        0x62, 0x82, 0x7E, 0x48, 0x72, 0xC0, // VCVTNEPS2BF16 ymm16, zmm24
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// VCVTNE2PS2BF16 Tests - Convert Two Singles to BF16
// ============================================================================

#[test]
fn test_vcvtne2ps2bf16_xmm_basic() {
    // VCVTNE2PS2BF16 XMM0, XMM1, XMM2
    // EVEX.128.F2.0F38.W0 72 /r
    let code = [
        0x62, 0xF2, 0x77, 0x08, 0x72, 0xC2, // VCVTNE2PS2BF16 xmm0, xmm1, xmm2
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vcvtne2ps2bf16_xmm_extended_regs() {
    // VCVTNE2PS2BF16 XMM8, XMM9, XMM10
    let code = [
        0x62, 0x52, 0x37, 0x08, 0x72, 0xC2, // VCVTNE2PS2BF16 xmm8, xmm9, xmm10
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vcvtne2ps2bf16_ymm_basic() {
    // VCVTNE2PS2BF16 YMM0, YMM1, YMM2
    let code = [
        0x62, 0xF2, 0x77, 0x28, 0x72, 0xC2, // VCVTNE2PS2BF16 ymm0, ymm1, ymm2
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vcvtne2ps2bf16_zmm_basic() {
    // VCVTNE2PS2BF16 ZMM0, ZMM1, ZMM2
    let code = [
        0x62, 0xF2, 0x77, 0x48, 0x72, 0xC2, // VCVTNE2PS2BF16 zmm0, zmm1, zmm2
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vcvtne2ps2bf16_zmm_high_regs() {
    // VCVTNE2PS2BF16 ZMM24, ZMM25, ZMM26
    let code = [
        0x62, 0x92, 0x37, 0x48, 0x72, 0xC2, // VCVTNE2PS2BF16 zmm24, zmm25, zmm26
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vcvtne2ps2bf16_xmm_memory() {
    // VCVTNE2PS2BF16 XMM0, XMM1, [RAX]
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x30, 0x00, 0x00, // MOV RAX, 0x3000
        0x62, 0xF2, 0x77, 0x08, 0x72, 0x00, // VCVTNE2PS2BF16 xmm0, xmm1, [rax]
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vcvtne2ps2bf16_zmm_memory() {
    // VCVTNE2PS2BF16 ZMM0, ZMM1, [RAX]
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x30, 0x00, 0x00, // MOV RAX, 0x3000
        0x62, 0xF2, 0x77, 0x48, 0x72, 0x00, // VCVTNE2PS2BF16 zmm0, zmm1, [rax]
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vcvtne2ps2bf16_xmm_with_mask() {
    // VCVTNE2PS2BF16 XMM0 {k1}, XMM1, XMM2
    let code = [
        0x62, 0xF2, 0x77, 0x09, 0x72, 0xC2, // VCVTNE2PS2BF16 xmm0 {k1}, xmm1, xmm2
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vcvtne2ps2bf16_zmm_with_mask_zeroing() {
    // VCVTNE2PS2BF16 ZMM0 {k3}{z}, ZMM1, ZMM2
    let code = [
        0x62, 0xF2, 0x77, 0xCB, 0x72, 0xC2, // VCVTNE2PS2BF16 zmm0 {k3}{z}, zmm1, zmm2
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vcvtne2ps2bf16_zmm_broadcast() {
    // VCVTNE2PS2BF16 ZMM0, ZMM1, dword ptr [RAX]{1to16}
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x30, 0x00, 0x00, // MOV RAX, 0x3000
        0x62, 0xF2, 0x77, 0x58, 0x72, 0x00, // VCVTNE2PS2BF16 zmm0, zmm1, [rax]{1to16}
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// VDPBF16PS Tests - BF16 Dot Product
// ============================================================================

#[test]
fn test_vdpbf16ps_xmm_basic() {
    // VDPBF16PS XMM0, XMM1, XMM2
    // EVEX.128.F3.0F38.W0 52 /r
    let code = [
        0x62, 0xF2, 0x76, 0x08, 0x52, 0xC2, // VDPBF16PS xmm0, xmm1, xmm2
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vdpbf16ps_xmm_extended_regs() {
    // VDPBF16PS XMM8, XMM9, XMM10
    let code = [
        0x62, 0x52, 0x36, 0x08, 0x52, 0xC2, // VDPBF16PS xmm8, xmm9, xmm10
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vdpbf16ps_ymm_basic() {
    // VDPBF16PS YMM0, YMM1, YMM2
    let code = [
        0x62, 0xF2, 0x76, 0x28, 0x52, 0xC2, // VDPBF16PS ymm0, ymm1, ymm2
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vdpbf16ps_ymm_extended_regs() {
    // VDPBF16PS YMM16, YMM17, YMM18
    let code = [
        0x62, 0xE2, 0x76, 0x28, 0x52, 0xC2, // VDPBF16PS ymm16, ymm17, ymm18
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vdpbf16ps_zmm_basic() {
    // VDPBF16PS ZMM0, ZMM1, ZMM2
    let code = [
        0x62, 0xF2, 0x76, 0x48, 0x52, 0xC2, // VDPBF16PS zmm0, zmm1, zmm2
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vdpbf16ps_zmm_high_regs() {
    // VDPBF16PS ZMM24, ZMM25, ZMM26
    let code = [
        0x62, 0x92, 0x36, 0x48, 0x52, 0xC2, // VDPBF16PS zmm24, zmm25, zmm26
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vdpbf16ps_xmm_memory() {
    // VDPBF16PS XMM0, XMM1, [RAX]
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x30, 0x00, 0x00, // MOV RAX, 0x3000
        0x62, 0xF2, 0x76, 0x08, 0x52, 0x00, // VDPBF16PS xmm0, xmm1, [rax]
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vdpbf16ps_zmm_memory() {
    // VDPBF16PS ZMM0, ZMM1, [RAX]
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x30, 0x00, 0x00, // MOV RAX, 0x3000
        0x62, 0xF2, 0x76, 0x48, 0x52, 0x00, // VDPBF16PS zmm0, zmm1, [rax]
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vdpbf16ps_xmm_with_mask() {
    // VDPBF16PS XMM0 {k1}, XMM1, XMM2
    let code = [
        0x62, 0xF2, 0x76, 0x09, 0x52, 0xC2, // VDPBF16PS xmm0 {k1}, xmm1, xmm2
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vdpbf16ps_zmm_with_mask_zeroing() {
    // VDPBF16PS ZMM0 {k4}{z}, ZMM1, ZMM2
    let code = [
        0x62, 0xF2, 0x76, 0xCC, 0x52, 0xC2, // VDPBF16PS zmm0 {k4}{z}, zmm1, zmm2
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vdpbf16ps_zmm_broadcast() {
    // VDPBF16PS ZMM0, ZMM1, dword ptr [RAX]{1to16}
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x30, 0x00, 0x00, // MOV RAX, 0x3000
        0x62, 0xF2, 0x76, 0x58, 0x52, 0x00, // VDPBF16PS zmm0, zmm1, [rax]{1to16}
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// Combined BF16 Operations Tests
// ============================================================================

#[test]
fn test_bf16_conversion_chain() {
    // Convert F32 to BF16 and then dot product
    let code = [
        0x62, 0xF2, 0x7E, 0x48, 0x72, 0xC1, // VCVTNEPS2BF16 ymm0, zmm1
        0x62, 0xF2, 0x76, 0x48, 0x52, 0xD2, // VDPBF16PS zmm2, zmm1, zmm2
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_bf16_ne2_and_dpbf16() {
    // Convert two sources and dot product
    let code = [
        0x62, 0xF2, 0x77, 0x48, 0x72, 0xC2, // VCVTNE2PS2BF16 zmm0, zmm1, zmm2
        0x62, 0xF2, 0x76, 0x48, 0x52, 0xD8, // VDPBF16PS zmm3, zmm1, zmm0
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_bf16_dotprod_accumulate() {
    // Multiple dot product accumulations
    let code = [
        0x62, 0xF2, 0x76, 0x48, 0x52, 0xC1, // VDPBF16PS zmm0, zmm1, zmm1
        0x62, 0xF2, 0x7E, 0x48, 0x52, 0xC2, // VDPBF16PS zmm0, zmm0, zmm2
        0x62, 0xF2, 0x7E, 0x48, 0x52, 0xC3, // VDPBF16PS zmm0, zmm0, zmm3
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_bf16_all_sizes() {
    // Test BF16 operations on all sizes
    let code = [
        0x62, 0xF2, 0x76, 0x08, 0x52, 0xC1, // VDPBF16PS xmm0, xmm1, xmm1
        0x62, 0xF2, 0x6E, 0x28, 0x52, 0xD2, // VDPBF16PS ymm2, ymm2, ymm2
        0x62, 0xF2, 0x66, 0x48, 0x52, 0xE3, // VDPBF16PS zmm4, zmm3, zmm3
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}
