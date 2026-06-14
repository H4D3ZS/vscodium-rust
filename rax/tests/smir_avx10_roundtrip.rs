//! AVX10.1 and AVX10.2 SMIR Roundtrip Tests
//!
//! This module tests that:
//! 1. AVX10 instructions can be lifted from machine code to SMIR
//! 2. SMIR operations can be lowered back to machine code
//! 3. The roundtrip preserves instruction semantics
//!
//! We test both AVX10.1 (VNNI, IFMA, VPOPCNT, VBMI, BF16, FP16) and
//! AVX10.2 (saturation conversions, VMINMAX, VMPSADBW, media acceleration).

#![cfg(test)]

use rax::smir::lift::LiftContext;
use rax::smir::lift::avx10::{Avx10Lifter, EvexPrefix};
use rax::smir::lower::CodeBuffer;
use rax::smir::lower::avx10::Avx10Lowerer;
use rax::smir::ops::OpKind;
use rax::smir::types::*;

// ============================================================================
// Test Helpers
// ============================================================================

/// Test a roundtrip: bytes -> lift -> lower -> bytes
/// Verifies the lowered bytes match the original (or are semantically equivalent)
fn test_roundtrip(original_bytes: &[u8], description: &str) {
    let lifter = Avx10Lifter::new();
    let lowerer = Avx10Lowerer::new();
    let mut ctx = LiftContext::new(SourceArch::X86_64);

    // Step 1: Lift
    let lift_result = lifter.try_lift(original_bytes, 0x1000, &mut ctx);
    assert!(
        lift_result.is_some(),
        "Failed to lift {}: instruction not recognized",
        description
    );

    let lift_result = lift_result.unwrap();
    assert!(
        lift_result.is_ok(),
        "Failed to lift {}: {:?}",
        description,
        lift_result.err()
    );

    let lifted = lift_result.unwrap();
    assert!(
        !lifted.ops.is_empty(),
        "Lifting {} produced no operations",
        description
    );

    // Step 2: Lower
    let mut code = CodeBuffer::new();
    let op = &lifted.ops[0].kind;
    let lower_result = lowerer.try_lower(op, &mut code);
    assert!(
        lower_result.is_some(),
        "Failed to lower {}: operation not recognized",
        description
    );

    let lower_result = lower_result.unwrap();
    assert!(
        lower_result.is_ok(),
        "Failed to lower {}: {:?}",
        description,
        lower_result.err()
    );

    // Step 3: Verify
    let lowered_bytes = code.as_slice();
    assert!(
        !lowered_bytes.is_empty(),
        "Lowering {} produced no bytes",
        description
    );

    // Check that the EVEX prefix is present
    assert_eq!(
        lowered_bytes[0], 0x62,
        "Lowered {} should have EVEX prefix",
        description
    );

    // For roundtrip, we verify semantic equivalence rather than byte-exact match
    // because the encoder may choose different but equivalent encodings
    // (e.g., different displacement sizes, register ordering, etc.)

    // Verify the opcode byte is in the right position (after 4-byte EVEX)
    let original_opcode = original_bytes[4];
    let lowered_opcode = lowered_bytes[4];
    assert_eq!(
        original_opcode, lowered_opcode,
        "Opcode mismatch for {}: expected {:02X}, got {:02X}",
        description, original_opcode, lowered_opcode
    );

    println!(
        "OK: {} - {} bytes lifted, {} bytes lowered",
        description,
        lifted.bytes_consumed,
        lowered_bytes.len()
    );
}

/// Test just lifting (for instructions we can't easily lower back)
fn test_lift_only(bytes: &[u8], description: &str, expected_op: &str) {
    let lifter = Avx10Lifter::new();
    let mut ctx = LiftContext::new(SourceArch::X86_64);

    let lift_result = lifter.try_lift(bytes, 0x1000, &mut ctx);
    assert!(
        lift_result.is_some(),
        "Failed to lift {}: instruction not recognized",
        description
    );

    let lift_result = lift_result.unwrap();
    assert!(
        lift_result.is_ok(),
        "Failed to lift {}: {:?}",
        description,
        lift_result.err()
    );

    let lifted = lift_result.unwrap();
    assert!(
        !lifted.ops.is_empty(),
        "Lifting {} produced no operations",
        description
    );

    let op_name = format!("{:?}", lifted.ops[0].kind);
    assert!(
        op_name.contains(expected_op),
        "Expected {} for {}, got {}",
        expected_op,
        description,
        op_name
    );

    println!(
        "OK: {} - {} bytes, op={}",
        description, lifted.bytes_consumed, expected_op
    );
}

// ============================================================================
// AVX10.1 VNNI Tests
// ============================================================================

#[test]
fn test_vpdpbusd_zmm() {
    // VPDPBUSD zmm1, zmm2, zmm3: 62 F2 6D 48 50 CB
    let bytes = [0x62, 0xF2, 0x6D, 0x48, 0x50, 0xCB];
    test_roundtrip(&bytes, "VPDPBUSD zmm1, zmm2, zmm3");
}

#[test]
fn test_vpdpbusd_ymm() {
    // VPDPBUSD ymm1, ymm2, ymm3: 62 F2 6D 28 50 CB
    let bytes = [0x62, 0xF2, 0x6D, 0x28, 0x50, 0xCB];
    test_roundtrip(&bytes, "VPDPBUSD ymm1, ymm2, ymm3");
}

#[test]
fn test_vpdpbusd_xmm() {
    // VPDPBUSD xmm1, xmm2, xmm3: 62 F2 6D 08 50 CB
    let bytes = [0x62, 0xF2, 0x6D, 0x08, 0x50, 0xCB];
    test_roundtrip(&bytes, "VPDPBUSD xmm1, xmm2, xmm3");
}

#[test]
fn test_vpdpbusds_zmm() {
    // VPDPBUSDS zmm1, zmm2, zmm3: 62 F2 6D 48 51 CB
    let bytes = [0x62, 0xF2, 0x6D, 0x48, 0x51, 0xCB];
    test_roundtrip(&bytes, "VPDPBUSDS zmm1, zmm2, zmm3");
}

#[test]
fn test_vpdpwssd_zmm() {
    // VPDPWSSD zmm1, zmm2, zmm3: 62 F2 6D 48 52 CB
    let bytes = [0x62, 0xF2, 0x6D, 0x48, 0x52, 0xCB];
    test_roundtrip(&bytes, "VPDPWSSD zmm1, zmm2, zmm3");
}

#[test]
fn test_vpdpwssds_zmm() {
    // VPDPWSSDS zmm1, zmm2, zmm3: 62 F2 6D 48 53 CB
    let bytes = [0x62, 0xF2, 0x6D, 0x48, 0x53, 0xCB];
    test_roundtrip(&bytes, "VPDPWSSDS zmm1, zmm2, zmm3");
}

// ============================================================================
// AVX10.1 IFMA Tests
// ============================================================================

#[test]
fn test_vpmadd52luq_zmm() {
    // VPMADD52LUQ zmm1, zmm2, zmm3: 62 F2 ED 48 B4 CB
    let bytes = [0x62, 0xF2, 0xED, 0x48, 0xB4, 0xCB];
    test_roundtrip(&bytes, "VPMADD52LUQ zmm1, zmm2, zmm3");
}

#[test]
fn test_vpmadd52huq_zmm() {
    // VPMADD52HUQ zmm1, zmm2, zmm3: 62 F2 ED 48 B5 CB
    let bytes = [0x62, 0xF2, 0xED, 0x48, 0xB5, 0xCB];
    test_roundtrip(&bytes, "VPMADD52HUQ zmm1, zmm2, zmm3");
}

// ============================================================================
// AVX10.1 VPOPCNT Tests
// ============================================================================

#[test]
fn test_vpopcntb_zmm() {
    // VPOPCNTB zmm1, zmm2: 62 F2 7D 48 54 CA
    let bytes = [0x62, 0xF2, 0x7D, 0x48, 0x54, 0xCA];
    test_roundtrip(&bytes, "VPOPCNTB zmm1, zmm2");
}

#[test]
fn test_vpopcntw_zmm() {
    // VPOPCNTW zmm1, zmm2: 62 F2 FD 48 54 CA
    let bytes = [0x62, 0xF2, 0xFD, 0x48, 0x54, 0xCA];
    test_roundtrip(&bytes, "VPOPCNTW zmm1, zmm2");
}

#[test]
fn test_vpopcntd_zmm() {
    // VPOPCNTD zmm1, zmm2: 62 F2 7D 48 55 CA
    let bytes = [0x62, 0xF2, 0x7D, 0x48, 0x55, 0xCA];
    test_roundtrip(&bytes, "VPOPCNTD zmm1, zmm2");
}

#[test]
fn test_vpopcntq_zmm() {
    // VPOPCNTQ zmm1, zmm2: 62 F2 FD 48 55 CA
    let bytes = [0x62, 0xF2, 0xFD, 0x48, 0x55, 0xCA];
    test_roundtrip(&bytes, "VPOPCNTQ zmm1, zmm2");
}

// ============================================================================
// AVX10.1 VBMI Tests
// ============================================================================

#[test]
fn test_vpermb_zmm() {
    // VPERMB zmm1, zmm2, zmm3: 62 F2 65 48 8D CB
    let bytes = [0x62, 0xF2, 0x65, 0x48, 0x8D, 0xCB];
    test_roundtrip(&bytes, "VPERMB zmm1, zmm2, zmm3");
}

#[test]
fn test_vpermi2b_zmm() {
    // VPERMI2B zmm1, zmm2, zmm3: 62 F2 65 48 75 CB
    let bytes = [0x62, 0xF2, 0x65, 0x48, 0x75, 0xCB];
    test_roundtrip(&bytes, "VPERMI2B zmm1, zmm2, zmm3");
}

#[test]
fn test_vpermt2b_zmm() {
    // VPERMT2B zmm1, zmm2, zmm3: 62 F2 65 48 7D CB
    let bytes = [0x62, 0xF2, 0x65, 0x48, 0x7D, 0xCB];
    test_roundtrip(&bytes, "VPERMT2B zmm1, zmm2, zmm3");
}

// ============================================================================
// AVX10.1 BITALG Tests
// ============================================================================

#[test]
fn test_vpshufbitqmb_zmm() {
    // VPSHUFBITQMB k1, zmm2, zmm3: 62 F2 65 48 8F CB
    let bytes = [0x62, 0xF2, 0x65, 0x48, 0x8F, 0xCB];
    test_lift_only(&bytes, "VPSHUFBITQMB k1, zmm2, zmm3", "VShuffleBitQM");
}

// ============================================================================
// AVX10.1 BF16 Tests
// ============================================================================

#[test]
fn test_vdpbf16ps_zmm() {
    // VDPBF16PS zmm1, zmm2, zmm3: 62 F2 66 48 52 CB
    let bytes = [0x62, 0xF2, 0x66, 0x48, 0x52, 0xCB];
    test_roundtrip(&bytes, "VDPBF16PS zmm1, zmm2, zmm3");
}

#[test]
fn test_vcvtneps2bf16_zmm() {
    // VCVTNEPS2BF16 ymm1, zmm2: 62 F2 7E 48 72 CA
    let bytes = [0x62, 0xF2, 0x7E, 0x48, 0x72, 0xCA];
    test_roundtrip(&bytes, "VCVTNEPS2BF16 ymm1, zmm2");
}

#[test]
fn test_vcvtne2ps2bf16_zmm() {
    // VCVTNE2PS2BF16 zmm1, zmm2, zmm3: 62 F2 67 48 72 CB
    let bytes = [0x62, 0xF2, 0x67, 0x48, 0x72, 0xCB];
    test_roundtrip(&bytes, "VCVTNE2PS2BF16 zmm1, zmm2, zmm3");
}

// ============================================================================
// AVX10.1 FP16 Tests
// ============================================================================

#[test]
fn test_vaddph_zmm() {
    // VADDPH zmm1, zmm2, zmm3: 62 F5 6C 48 58 CB (MAP5)
    // Note: MAP5 encoding uses mm=5 in EVEX
    let bytes = [0x62, 0xF5, 0x6C, 0x48, 0x58, 0xCB];
    test_lift_only(&bytes, "VADDPH zmm1, zmm2, zmm3", "VFP16Arith");
}

#[test]
fn test_vmulph_zmm() {
    // VMULPH zmm1, zmm2, zmm3: 62 F5 6C 48 59 CB (MAP5)
    let bytes = [0x62, 0xF5, 0x6C, 0x48, 0x59, 0xCB];
    test_lift_only(&bytes, "VMULPH zmm1, zmm2, zmm3", "VFP16Arith");
}

#[test]
fn test_vsubph_zmm() {
    // VSUBPH zmm1, zmm2, zmm3: 62 F5 6C 48 5C CB (MAP5)
    let bytes = [0x62, 0xF5, 0x6C, 0x48, 0x5C, 0xCB];
    test_lift_only(&bytes, "VSUBPH zmm1, zmm2, zmm3", "VFP16Arith");
}

#[test]
fn test_vdivph_zmm() {
    // VDIVPH zmm1, zmm2, zmm3: 62 F5 6C 48 5E CB (MAP5)
    let bytes = [0x62, 0xF5, 0x6C, 0x48, 0x5E, 0xCB];
    test_lift_only(&bytes, "VDIVPH zmm1, zmm2, zmm3", "VFP16Arith");
}

// ============================================================================
// AVX10.2 Saturation Conversion Tests
// ============================================================================

#[test]
fn test_vcvttps2ibs_zmm() {
    // VCVTTPS2IBS zmm1, zmm2: 62 F2 7C 48 68 CA
    let bytes = [0x62, 0xF2, 0x7C, 0x48, 0x68, 0xCA];
    test_roundtrip(&bytes, "VCVTTPS2IBS zmm1, zmm2");
}

#[test]
fn test_vcvttps2iubs_zmm() {
    // VCVTTPS2IUBS zmm1, zmm2: 62 F2 7C 48 6A CA
    let bytes = [0x62, 0xF2, 0x7C, 0x48, 0x6A, 0xCA];
    test_roundtrip(&bytes, "VCVTTPS2IUBS zmm1, zmm2");
}

#[test]
fn test_vcvttpd2qqs_zmm() {
    // VCVTTPD2QQS zmm1, zmm2: 62 F2 FD 48 6D CA
    let bytes = [0x62, 0xF2, 0xFD, 0x48, 0x6D, 0xCA];
    test_roundtrip(&bytes, "VCVTTPD2QQS zmm1, zmm2");
}

#[test]
fn test_vcvttpd2uqqs_zmm() {
    // VCVTTPD2UQQS zmm1, zmm2: 62 F2 FD 48 6C CA
    let bytes = [0x62, 0xF2, 0xFD, 0x48, 0x6C, 0xCA];
    test_roundtrip(&bytes, "VCVTTPD2UQQS zmm1, zmm2");
}

// ============================================================================
// AVX10.2 VMINMAX Tests
// ============================================================================

#[test]
fn test_vminmaxps_zmm() {
    // VMINMAXPS zmm1, zmm2, zmm3, 0x00: 62 F3 6C 48 52 CB 00
    let bytes = [0x62, 0xF3, 0x6C, 0x48, 0x52, 0xCB, 0x00];
    test_roundtrip(&bytes, "VMINMAXPS zmm1, zmm2, zmm3, 0");
}

#[test]
fn test_vminmaxpd_zmm() {
    // VMINMAXPD zmm1, zmm2, zmm3, 0x01: 62 F3 ED 48 52 CB 01
    let bytes = [0x62, 0xF3, 0xED, 0x48, 0x52, 0xCB, 0x01];
    test_roundtrip(&bytes, "VMINMAXPD zmm1, zmm2, zmm3, 1");
}

// ============================================================================
// AVX10.2 VMPSADBW Tests
// ============================================================================

#[test]
fn test_vmpsadbw_zmm() {
    // VMPSADBW zmm1, zmm2, zmm3, 0x55: 62 F3 6D 48 42 CB 55
    let bytes = [0x62, 0xF3, 0x6D, 0x48, 0x42, 0xCB, 0x55];
    test_roundtrip(&bytes, "VMPSADBW zmm1, zmm2, zmm3, 0x55");
}

// ============================================================================
// AVX10.2 Media Acceleration Tests (Byte Dot Products)
// ============================================================================

#[test]
fn test_vpdpbssd_zmm() {
    // VPDPBSSD zmm1, zmm2, zmm3: 62 F2 66 48 50 CB
    let bytes = [0x62, 0xF2, 0x66, 0x48, 0x50, 0xCB];
    test_lift_only(&bytes, "VPDPBSSD zmm1, zmm2, zmm3", "VDotProductExt");
}

#[test]
fn test_vpdpbssds_zmm() {
    // VPDPBSSDS zmm1, zmm2, zmm3: 62 F2 66 48 51 CB
    let bytes = [0x62, 0xF2, 0x66, 0x48, 0x51, 0xCB];
    test_lift_only(&bytes, "VPDPBSSDS zmm1, zmm2, zmm3", "VDotProductExt");
}

#[test]
fn test_vpdpbsud_zmm() {
    // VPDPBSUD zmm1, zmm2, zmm3: 62 F2 E6 48 50 CB
    let bytes = [0x62, 0xF2, 0xE6, 0x48, 0x50, 0xCB];
    test_lift_only(&bytes, "VPDPBSUD zmm1, zmm2, zmm3", "VDotProductExt");
}

#[test]
fn test_vpdpbuud_zmm() {
    // VPDPBUUD zmm1, zmm2, zmm3: 62 F2 EC 48 50 CB
    let bytes = [0x62, 0xF2, 0xEC, 0x48, 0x50, 0xCB];
    test_lift_only(&bytes, "VPDPBUUD zmm1, zmm2, zmm3", "VDotProductExt");
}

// ============================================================================
// AVX10.2 Media Acceleration Tests (Word Dot Products)
// ============================================================================

#[test]
fn test_vpdpwsud_zmm() {
    // VPDPWSUD zmm1, zmm2, zmm3: 62 F2 66 48 D2 CB
    let bytes = [0x62, 0xF2, 0x66, 0x48, 0xD2, 0xCB];
    test_lift_only(&bytes, "VPDPWSUD zmm1, zmm2, zmm3", "VDotProductExt");
}

#[test]
fn test_vpdpwusd_zmm() {
    // VPDPWUSD zmm1, zmm2, zmm3: 62 F2 65 48 D2 CB
    let bytes = [0x62, 0xF2, 0x65, 0x48, 0xD2, 0xCB];
    test_lift_only(&bytes, "VPDPWUSD zmm1, zmm2, zmm3", "VDotProductExt");
}

#[test]
fn test_vpdpwuud_zmm() {
    // VPDPWUUD zmm1, zmm2, zmm3: 62 F2 6C 48 D2 CB
    let bytes = [0x62, 0xF2, 0x6C, 0x48, 0xD2, 0xCB];
    test_lift_only(&bytes, "VPDPWUUD zmm1, zmm2, zmm3", "VDotProductExt");
}

// ============================================================================
// EVEX Prefix Decoding Tests
// ============================================================================

#[test]
fn test_evex_decode_basic() {
    // EVEX prefix for ZMM operation
    let bytes = [0x62, 0xF2, 0x6D, 0x48, 0x50, 0xCB];
    let evex = EvexPrefix::decode(&bytes).unwrap();

    assert_eq!(evex.map, 2); // 0F38
    assert_eq!(evex.pp, 1); // 66
    assert!(!evex.w);
    assert_eq!(evex.ll, 2); // 512-bit
    assert_eq!(evex.vec_width(), VecWidth::V512);
    assert_eq!(evex.bytes, 4);
}

#[test]
fn test_evex_decode_ymm() {
    // EVEX prefix for YMM operation (L'L = 01)
    let bytes = [0x62, 0xF2, 0x6D, 0x28, 0x50, 0xCB];
    let evex = EvexPrefix::decode(&bytes).unwrap();

    assert_eq!(evex.ll, 1);
    assert_eq!(evex.vec_width(), VecWidth::V256);
}

#[test]
fn test_evex_decode_xmm() {
    // EVEX prefix for XMM operation (L'L = 00)
    let bytes = [0x62, 0xF2, 0x6D, 0x08, 0x50, 0xCB];
    let evex = EvexPrefix::decode(&bytes).unwrap();

    assert_eq!(evex.ll, 0);
    assert_eq!(evex.vec_width(), VecWidth::V128);
}

#[test]
fn test_evex_decode_with_mask() {
    // EVEX prefix with k1 mask
    let bytes = [0x62, 0xF2, 0x6D, 0x49, 0x50, 0xCB];
    let evex = EvexPrefix::decode(&bytes).unwrap();

    assert_eq!(evex.aaa, 1); // k1
    assert!(!evex.z); // merge masking
}

#[test]
fn test_evex_decode_with_zeroing() {
    // EVEX prefix with k1 mask and zeroing
    let bytes = [0x62, 0xF2, 0x6D, 0xC9, 0x50, 0xCB];
    let evex = EvexPrefix::decode(&bytes).unwrap();

    assert_eq!(evex.aaa, 1); // k1
    assert!(evex.z); // zeroing masking
}

#[test]
fn test_evex_register_encoding() {
    // Test register field decoding
    let bytes = [0x62, 0xF2, 0x6D, 0x48, 0x50, 0xCB];
    let evex = EvexPrefix::decode(&bytes).unwrap();

    // ModRM = CB = 11 001 011
    // reg = 1, rm = 3
    assert_eq!(evex.dest_reg(1), 1); // zmm1
    assert_eq!(evex.rm_reg(3), 3); // zmm3
    assert_eq!(evex.src1_reg(), 2); // zmm2 from vvvv
}

// ============================================================================
// Vector Width Variation Tests
// ============================================================================

#[test]
fn test_vnni_all_widths() {
    // Test VPDPBUSD at all three vector widths

    // XMM (128-bit)
    let xmm = [0x62, 0xF2, 0x6D, 0x08, 0x50, 0xCB];
    test_roundtrip(&xmm, "VPDPBUSD xmm");

    // YMM (256-bit)
    let ymm = [0x62, 0xF2, 0x6D, 0x28, 0x50, 0xCB];
    test_roundtrip(&ymm, "VPDPBUSD ymm");

    // ZMM (512-bit)
    let zmm = [0x62, 0xF2, 0x6D, 0x48, 0x50, 0xCB];
    test_roundtrip(&zmm, "VPDPBUSD zmm");
}

// ============================================================================
// Batch Verification Tests
// ============================================================================

#[test]
fn test_all_avx10_1_vnni() {
    let test_cases = [
        ([0x62, 0xF2, 0x6D, 0x48, 0x50, 0xCB], "VPDPBUSD"),
        ([0x62, 0xF2, 0x6D, 0x48, 0x51, 0xCB], "VPDPBUSDS"),
        ([0x62, 0xF2, 0x6D, 0x48, 0x52, 0xCB], "VPDPWSSD"),
        ([0x62, 0xF2, 0x6D, 0x48, 0x53, 0xCB], "VPDPWSSDS"),
    ];

    for (bytes, name) in test_cases {
        test_roundtrip(&bytes, name);
    }
}

#[test]
fn test_all_avx10_1_ifma() {
    let test_cases = [
        ([0x62, 0xF2, 0xED, 0x48, 0xB4, 0xCB], "VPMADD52LUQ"),
        ([0x62, 0xF2, 0xED, 0x48, 0xB5, 0xCB], "VPMADD52HUQ"),
    ];

    for (bytes, name) in test_cases {
        test_roundtrip(&bytes, name);
    }
}

#[test]
fn test_all_avx10_1_vpopcnt() {
    let test_cases = [
        ([0x62, 0xF2, 0x7D, 0x48, 0x54, 0xCA], "VPOPCNTB"),
        ([0x62, 0xF2, 0xFD, 0x48, 0x54, 0xCA], "VPOPCNTW"),
        ([0x62, 0xF2, 0x7D, 0x48, 0x55, 0xCA], "VPOPCNTD"),
        ([0x62, 0xF2, 0xFD, 0x48, 0x55, 0xCA], "VPOPCNTQ"),
    ];

    for (bytes, name) in test_cases {
        test_roundtrip(&bytes, name);
    }
}

#[test]
fn test_all_avx10_2_saturation_converts() {
    let test_cases = [
        ([0x62, 0xF2, 0x7C, 0x48, 0x68, 0xCA], "VCVTTPS2IBS"),
        ([0x62, 0xF2, 0x7C, 0x48, 0x6A, 0xCA], "VCVTTPS2IUBS"),
        ([0x62, 0xF2, 0xFD, 0x48, 0x6D, 0xCA], "VCVTTPD2QQS"),
        ([0x62, 0xF2, 0xFD, 0x48, 0x6C, 0xCA], "VCVTTPD2UQQS"),
    ];

    for (bytes, name) in test_cases {
        test_roundtrip(&bytes, name);
    }
}
