use crate::common::*;
use vm_memory::{Bytes, GuestAddress};

// MAXSS - Return Maximum Scalar Single Precision Floating-Point Value
// MAXSD - Return Maximum Scalar Double Precision Floating-Point Value
//
// MAXSS compares the low single-precision (32-bit) floating-point values
// and returns the maximum to the destination.
//
// MAXSD compares the low double-precision (64-bit) floating-point values
// and returns the maximum to the destination.
//
// Special cases:
// - If values are both 0.0s (either sign), return second operand
// - If second operand is SNaN, forward SNaN unchanged to destination
// - If only one value is NaN, return second operand
//
// Opcodes:
// F3 0F 5F /r             MAXSS xmm1, xmm2/m32   - Return maximum scalar single
// F2 0F 5F /r             MAXSD xmm1, xmm2/m64   - Return maximum scalar double

const ALIGNED_ADDR: u64 = 0x3000;
const ALIGNED_ADDR2: u64 = 0x3100;

fn float_bits(val: f32) -> [u8; 4] {
    val.to_le_bytes()
}

fn double_bits(val: f64) -> [u8; 8] {
    val.to_le_bytes()
}

// ============================================================================
// MAXSS Tests - Scalar Single Precision Maximum
// ============================================================================

#[test]
fn test_maxss_xmm0_xmm1() {
    let code = [
        0x0f, 0x28, 0xc1, // MOVAPS XMM0, XMM1
        0xf3, 0x0f, 0x5f, 0xc1, // MAXSS XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_maxss_first_larger() {
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x48, 0xbb, // MOV RBX, imm64
    ]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0xf3, 0x0f, 0x10, 0x00, // MOVSS XMM0, [RAX]
        0xf3, 0x0f, 0x10, 0x0b, // MOVSS XMM1, [RBX]
        0xf3, 0x0f, 0x5f, 0xc1, // MAXSS XMM0, XMM1
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(&float_bits(5.0), GuestAddress(ALIGNED_ADDR))
        .unwrap();
    mem.write_slice(&float_bits(3.0), GuestAddress(ALIGNED_ADDR2))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_maxss_second_larger() {
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x48, 0xbb, // MOV RBX, imm64
    ]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0xf3, 0x0f, 0x10, 0x00, // MOVSS XMM0, [RAX]
        0xf3, 0x0f, 0x10, 0x0b, // MOVSS XMM1, [RBX]
        0xf3, 0x0f, 0x5f, 0xc1, // MAXSS XMM0, XMM1
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(&float_bits(1.0), GuestAddress(ALIGNED_ADDR))
        .unwrap();
    mem.write_slice(&float_bits(2.0), GuestAddress(ALIGNED_ADDR2))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_maxss_equal_values() {
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xf3, 0x0f, 0x10, 0x00, // MOVSS XMM0, [RAX]
        0xf3, 0x0f, 0x5f, 0xc0, // MAXSS XMM0, XMM0
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(&float_bits(4.5), GuestAddress(ALIGNED_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_maxss_negative_values() {
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x48, 0xbb, // MOV RBX, imm64
    ]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0xf3, 0x0f, 0x10, 0x00, // MOVSS XMM0, [RAX]
        0xf3, 0x0f, 0x10, 0x0b, // MOVSS XMM1, [RBX]
        0xf3, 0x0f, 0x5f, 0xc1, // MAXSS XMM0, XMM1
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(&float_bits(-1.0), GuestAddress(ALIGNED_ADDR))
        .unwrap();
    mem.write_slice(&float_bits(-5.0), GuestAddress(ALIGNED_ADDR2))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_maxss_positive_and_negative() {
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x48, 0xbb, // MOV RBX, imm64
    ]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0xf3, 0x0f, 0x10, 0x00, // MOVSS XMM0, [RAX]
        0xf3, 0x0f, 0x10, 0x0b, // MOVSS XMM1, [RBX]
        0xf3, 0x0f, 0x5f, 0xc1, // MAXSS XMM0, XMM1
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(&float_bits(-5.0), GuestAddress(ALIGNED_ADDR))
        .unwrap();
    mem.write_slice(&float_bits(3.0), GuestAddress(ALIGNED_ADDR2))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_maxss_both_zero() {
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xf3, 0x0f, 0x10, 0x00, // MOVSS XMM0, [RAX]
        0xf3, 0x0f, 0x5f, 0xc0, // MAXSS XMM0, XMM0
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(&float_bits(0.0), GuestAddress(ALIGNED_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_maxss_positive_and_negative_zero() {
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x48, 0xbb, // MOV RBX, imm64
    ]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0xf3, 0x0f, 0x10, 0x00, // MOVSS XMM0, [RAX]
        0xf3, 0x0f, 0x10, 0x0b, // MOVSS XMM1, [RBX]
        0xf3, 0x0f, 0x5f, 0xc1, // MAXSS XMM0, XMM1
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(&float_bits(0.0), GuestAddress(ALIGNED_ADDR))
        .unwrap();
    mem.write_slice(&float_bits(-0.0), GuestAddress(ALIGNED_ADDR2))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_maxss_infinity() {
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x48, 0xbb, // MOV RBX, imm64
    ]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0xf3, 0x0f, 0x10, 0x00, // MOVSS XMM0, [RAX]
        0xf3, 0x0f, 0x10, 0x0b, // MOVSS XMM1, [RBX]
        0xf3, 0x0f, 0x5f, 0xc1, // MAXSS XMM0, XMM1
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(&float_bits(1.0), GuestAddress(ALIGNED_ADDR))
        .unwrap();
    mem.write_slice(&float_bits(f32::INFINITY), GuestAddress(ALIGNED_ADDR2))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_maxss_negative_infinity() {
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x48, 0xbb, // MOV RBX, imm64
    ]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0xf3, 0x0f, 0x10, 0x00, // MOVSS XMM0, [RAX]
        0xf3, 0x0f, 0x10, 0x0b, // MOVSS XMM1, [RBX]
        0xf3, 0x0f, 0x5f, 0xc1, // MAXSS XMM0, XMM1
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(&float_bits(1.0), GuestAddress(ALIGNED_ADDR))
        .unwrap();
    mem.write_slice(&float_bits(f32::NEG_INFINITY), GuestAddress(ALIGNED_ADDR2))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_maxss_very_large_numbers() {
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x48, 0xbb, // MOV RBX, imm64
    ]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0xf3, 0x0f, 0x10, 0x00, // MOVSS XMM0, [RAX]
        0xf3, 0x0f, 0x10, 0x0b, // MOVSS XMM1, [RBX]
        0xf3, 0x0f, 0x5f, 0xc1, // MAXSS XMM0, XMM1
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(&float_bits(1e30), GuestAddress(ALIGNED_ADDR))
        .unwrap();
    mem.write_slice(&float_bits(1e29), GuestAddress(ALIGNED_ADDR2))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_maxss_very_small_numbers() {
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x48, 0xbb, // MOV RBX, imm64
    ]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0xf3, 0x0f, 0x10, 0x00, // MOVSS XMM0, [RAX]
        0xf3, 0x0f, 0x10, 0x0b, // MOVSS XMM1, [RBX]
        0xf3, 0x0f, 0x5f, 0xc1, // MAXSS XMM0, XMM1
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(&float_bits(1e-20), GuestAddress(ALIGNED_ADDR))
        .unwrap();
    mem.write_slice(&float_bits(1e-10), GuestAddress(ALIGNED_ADDR2))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_maxss_denormal_values() {
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xf3, 0x0f, 0x10, 0x00, // MOVSS XMM0, [RAX]
        0xf3, 0x0f, 0x5f, 0xc0, // MAXSS XMM0, XMM0
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let denormal = f32::from_bits(1);
    mem.write_slice(&float_bits(denormal), GuestAddress(ALIGNED_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_maxss_both_infinity() {
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x48, 0xbb, // MOV RBX, imm64
    ]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0xf3, 0x0f, 0x10, 0x00, // MOVSS XMM0, [RAX]
        0xf3, 0x0f, 0x10, 0x0b, // MOVSS XMM1, [RBX]
        0xf3, 0x0f, 0x5f, 0xc1, // MAXSS XMM0, XMM1
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(&float_bits(f32::INFINITY), GuestAddress(ALIGNED_ADDR))
        .unwrap();
    mem.write_slice(&float_bits(f32::INFINITY), GuestAddress(ALIGNED_ADDR2))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_maxss_fractional_values() {
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x48, 0xbb, // MOV RBX, imm64
    ]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0xf3, 0x0f, 0x10, 0x00, // MOVSS XMM0, [RAX]
        0xf3, 0x0f, 0x10, 0x0b, // MOVSS XMM1, [RBX]
        0xf3, 0x0f, 0x5f, 0xc1, // MAXSS XMM0, XMM1
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(&float_bits(0.333333), GuestAddress(ALIGNED_ADDR))
        .unwrap();
    mem.write_slice(&float_bits(0.5), GuestAddress(ALIGNED_ADDR2))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// MAXSD Tests - Scalar Double Precision Maximum
// ============================================================================

#[test]
fn test_maxsd_xmm0_xmm1() {
    let code = [
        0x0f, 0x28, 0xc1, // MOVAPS XMM0, XMM1
        0xf2, 0x0f, 0x5f, 0xc1, // MAXSD XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_maxsd_first_larger() {
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x48, 0xbb, // MOV RBX, imm64
    ]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0xf2, 0x0f, 0x10, 0x00, // MOVSD XMM0, [RAX]
        0xf2, 0x0f, 0x10, 0x0b, // MOVSD XMM1, [RBX]
        0xf2, 0x0f, 0x5f, 0xc1, // MAXSD XMM0, XMM1
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(&double_bits(5.0), GuestAddress(ALIGNED_ADDR))
        .unwrap();
    mem.write_slice(&double_bits(3.0), GuestAddress(ALIGNED_ADDR2))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_maxsd_second_larger() {
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x48, 0xbb, // MOV RBX, imm64
    ]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0xf2, 0x0f, 0x10, 0x00, // MOVSD XMM0, [RAX]
        0xf2, 0x0f, 0x10, 0x0b, // MOVSD XMM1, [RBX]
        0xf2, 0x0f, 0x5f, 0xc1, // MAXSD XMM0, XMM1
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(&double_bits(1.0), GuestAddress(ALIGNED_ADDR))
        .unwrap();
    mem.write_slice(&double_bits(2.0), GuestAddress(ALIGNED_ADDR2))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_maxsd_equal_values() {
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xf2, 0x0f, 0x10, 0x00, // MOVSD XMM0, [RAX]
        0xf2, 0x0f, 0x5f, 0xc0, // MAXSD XMM0, XMM0
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(&double_bits(4.5), GuestAddress(ALIGNED_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_maxsd_negative_values() {
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x48, 0xbb, // MOV RBX, imm64
    ]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0xf2, 0x0f, 0x10, 0x00, // MOVSD XMM0, [RAX]
        0xf2, 0x0f, 0x10, 0x0b, // MOVSD XMM1, [RBX]
        0xf2, 0x0f, 0x5f, 0xc1, // MAXSD XMM0, XMM1
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(&double_bits(-1.0), GuestAddress(ALIGNED_ADDR))
        .unwrap();
    mem.write_slice(&double_bits(-5.0), GuestAddress(ALIGNED_ADDR2))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_maxsd_positive_and_negative() {
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x48, 0xbb, // MOV RBX, imm64
    ]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0xf2, 0x0f, 0x10, 0x00, // MOVSD XMM0, [RAX]
        0xf2, 0x0f, 0x10, 0x0b, // MOVSD XMM1, [RBX]
        0xf2, 0x0f, 0x5f, 0xc1, // MAXSD XMM0, XMM1
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(&double_bits(-5.0), GuestAddress(ALIGNED_ADDR))
        .unwrap();
    mem.write_slice(&double_bits(3.0), GuestAddress(ALIGNED_ADDR2))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_maxsd_both_zero() {
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xf2, 0x0f, 0x10, 0x00, // MOVSD XMM0, [RAX]
        0xf2, 0x0f, 0x5f, 0xc0, // MAXSD XMM0, XMM0
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(&double_bits(0.0), GuestAddress(ALIGNED_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_maxsd_positive_and_negative_zero() {
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x48, 0xbb, // MOV RBX, imm64
    ]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0xf2, 0x0f, 0x10, 0x00, // MOVSD XMM0, [RAX]
        0xf2, 0x0f, 0x10, 0x0b, // MOVSD XMM1, [RBX]
        0xf2, 0x0f, 0x5f, 0xc1, // MAXSD XMM0, XMM1
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(&double_bits(0.0), GuestAddress(ALIGNED_ADDR))
        .unwrap();
    mem.write_slice(&double_bits(-0.0), GuestAddress(ALIGNED_ADDR2))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_maxsd_infinity() {
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x48, 0xbb, // MOV RBX, imm64
    ]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0xf2, 0x0f, 0x10, 0x00, // MOVSD XMM0, [RAX]
        0xf2, 0x0f, 0x10, 0x0b, // MOVSD XMM1, [RBX]
        0xf2, 0x0f, 0x5f, 0xc1, // MAXSD XMM0, XMM1
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(&double_bits(1.0), GuestAddress(ALIGNED_ADDR))
        .unwrap();
    mem.write_slice(&double_bits(f64::INFINITY), GuestAddress(ALIGNED_ADDR2))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_maxsd_negative_infinity() {
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x48, 0xbb, // MOV RBX, imm64
    ]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0xf2, 0x0f, 0x10, 0x00, // MOVSD XMM0, [RAX]
        0xf2, 0x0f, 0x10, 0x0b, // MOVSD XMM1, [RBX]
        0xf2, 0x0f, 0x5f, 0xc1, // MAXSD XMM0, XMM1
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(&double_bits(1.0), GuestAddress(ALIGNED_ADDR))
        .unwrap();
    mem.write_slice(&double_bits(f64::NEG_INFINITY), GuestAddress(ALIGNED_ADDR2))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_maxsd_very_large_numbers() {
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x48, 0xbb, // MOV RBX, imm64
    ]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0xf2, 0x0f, 0x10, 0x00, // MOVSD XMM0, [RAX]
        0xf2, 0x0f, 0x10, 0x0b, // MOVSD XMM1, [RBX]
        0xf2, 0x0f, 0x5f, 0xc1, // MAXSD XMM0, XMM1
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(&double_bits(1e100), GuestAddress(ALIGNED_ADDR))
        .unwrap();
    mem.write_slice(&double_bits(1e99), GuestAddress(ALIGNED_ADDR2))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_maxsd_very_small_numbers() {
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x48, 0xbb, // MOV RBX, imm64
    ]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0xf2, 0x0f, 0x10, 0x00, // MOVSD XMM0, [RAX]
        0xf2, 0x0f, 0x10, 0x0b, // MOVSD XMM1, [RBX]
        0xf2, 0x0f, 0x5f, 0xc1, // MAXSD XMM0, XMM1
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(&double_bits(1e-200), GuestAddress(ALIGNED_ADDR))
        .unwrap();
    mem.write_slice(&double_bits(1e-100), GuestAddress(ALIGNED_ADDR2))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_maxsd_high_precision_e() {
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xf2, 0x0f, 0x10, 0x00, // MOVSD XMM0, [RAX]
        0xf2, 0x0f, 0x5f, 0xc0, // MAXSD XMM0, XMM0
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(&double_bits(2.71828182845904), GuestAddress(ALIGNED_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_maxsd_denormal_values() {
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xf2, 0x0f, 0x10, 0x00, // MOVSD XMM0, [RAX]
        0xf2, 0x0f, 0x5f, 0xc0, // MAXSD XMM0, XMM0
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let denormal = f64::from_bits(1);
    mem.write_slice(&double_bits(denormal), GuestAddress(ALIGNED_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_maxsd_both_infinity() {
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x48, 0xbb, // MOV RBX, imm64
    ]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0xf2, 0x0f, 0x10, 0x00, // MOVSD XMM0, [RAX]
        0xf2, 0x0f, 0x10, 0x0b, // MOVSD XMM1, [RBX]
        0xf2, 0x0f, 0x5f, 0xc1, // MAXSD XMM0, XMM1
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(&double_bits(f64::INFINITY), GuestAddress(ALIGNED_ADDR))
        .unwrap();
    mem.write_slice(&double_bits(f64::INFINITY), GuestAddress(ALIGNED_ADDR2))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_maxsd_mixed_infinity_values() {
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x48, 0xbb, // MOV RBX, imm64
    ]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0xf2, 0x0f, 0x10, 0x00, // MOVSD XMM0, [RAX]
        0xf2, 0x0f, 0x10, 0x0b, // MOVSD XMM1, [RBX]
        0xf2, 0x0f, 0x5f, 0xc1, // MAXSD XMM0, XMM1
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(&double_bits(f64::INFINITY), GuestAddress(ALIGNED_ADDR))
        .unwrap();
    mem.write_slice(&double_bits(f64::NEG_INFINITY), GuestAddress(ALIGNED_ADDR2))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_maxsd_fractional_values() {
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x48, 0xbb, // MOV RBX, imm64
    ]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0xf2, 0x0f, 0x10, 0x00, // MOVSD XMM0, [RAX]
        0xf2, 0x0f, 0x10, 0x0b, // MOVSD XMM1, [RBX]
        0xf2, 0x0f, 0x5f, 0xc1, // MAXSD XMM0, XMM1
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(&double_bits(0.3333333333), GuestAddress(ALIGNED_ADDR))
        .unwrap();
    mem.write_slice(&double_bits(0.5), GuestAddress(ALIGNED_ADDR2))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}
