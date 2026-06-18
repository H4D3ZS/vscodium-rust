use crate::common::*;
use vm_memory::{Bytes, GuestAddress};

// MINSS - Return Minimum Scalar Single Precision Floating-Point Value
// MINSD - Return Minimum Scalar Double Precision Floating-Point Value
//
// MINSS compares the low single-precision (32-bit) floating-point values
// and returns the minimum to the destination.
//
// MINSD compares the low double-precision (64-bit) floating-point values
// and returns the minimum to the destination.
//
// Special cases:
// - If values are both 0.0s (either sign), return second operand
// - If second operand is SNaN, forward SNaN unchanged to destination
// - If only one value is NaN, return second operand
//
// Opcodes:
// F3 0F 5D /r             MINSS xmm1, xmm2/m32   - Return minimum scalar single
// F2 0F 5D /r             MINSD xmm1, xmm2/m64   - Return minimum scalar double

const ALIGNED_ADDR: u64 = 0x3000;
const ALIGNED_ADDR2: u64 = 0x3100;

fn float_bits(val: f32) -> [u8; 4] {
    val.to_le_bytes()
}

fn double_bits(val: f64) -> [u8; 8] {
    val.to_le_bytes()
}

// ============================================================================
// MINSS Tests - Scalar Single Precision Minimum
// ============================================================================

#[test]
fn test_minss_xmm0_xmm1() {
    let code = [
        0x0f, 0x28, 0xc1, // MOVAPS XMM0, XMM1
        0xf3, 0x0f, 0x5d, 0xc1, // MINSS XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_minss_first_smaller() {
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
        0xf3, 0x0f, 0x5d, 0xc1, // MINSS XMM0, XMM1
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(&float_bits(1.0), GuestAddress(ALIGNED_ADDR))
        .unwrap();
    mem.write_slice(&float_bits(3.0), GuestAddress(ALIGNED_ADDR2))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_minss_second_smaller() {
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
        0xf3, 0x0f, 0x5d, 0xc1, // MINSS XMM0, XMM1
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(&float_bits(5.0), GuestAddress(ALIGNED_ADDR))
        .unwrap();
    mem.write_slice(&float_bits(2.0), GuestAddress(ALIGNED_ADDR2))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_minss_equal_values() {
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xf3, 0x0f, 0x10, 0x00, // MOVSS XMM0, [RAX]
        0xf3, 0x0f, 0x5d, 0xc0, // MINSS XMM0, XMM0
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(&float_bits(4.5), GuestAddress(ALIGNED_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_minss_negative_values() {
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
        0xf3, 0x0f, 0x5d, 0xc1, // MINSS XMM0, XMM1
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
fn test_minss_positive_and_negative() {
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
        0xf3, 0x0f, 0x5d, 0xc1, // MINSS XMM0, XMM1
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(&float_bits(5.0), GuestAddress(ALIGNED_ADDR))
        .unwrap();
    mem.write_slice(&float_bits(-3.0), GuestAddress(ALIGNED_ADDR2))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_minss_both_zero() {
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xf3, 0x0f, 0x10, 0x00, // MOVSS XMM0, [RAX]
        0xf3, 0x0f, 0x5d, 0xc0, // MINSS XMM0, XMM0
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(&float_bits(0.0), GuestAddress(ALIGNED_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_minss_positive_and_negative_zero() {
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
        0xf3, 0x0f, 0x5d, 0xc1, // MINSS XMM0, XMM1
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
fn test_minss_infinity() {
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
        0xf3, 0x0f, 0x5d, 0xc1, // MINSS XMM0, XMM1
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
fn test_minss_negative_infinity() {
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
        0xf3, 0x0f, 0x5d, 0xc1, // MINSS XMM0, XMM1
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
fn test_minss_very_large_numbers() {
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
        0xf3, 0x0f, 0x5d, 0xc1, // MINSS XMM0, XMM1
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
fn test_minss_very_small_numbers() {
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
        0xf3, 0x0f, 0x5d, 0xc1, // MINSS XMM0, XMM1
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(&float_bits(1e-10), GuestAddress(ALIGNED_ADDR))
        .unwrap();
    mem.write_slice(&float_bits(1e-20), GuestAddress(ALIGNED_ADDR2))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_minss_denormal_values() {
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xf3, 0x0f, 0x10, 0x00, // MOVSS XMM0, [RAX]
        0xf3, 0x0f, 0x5d, 0xc0, // MINSS XMM0, XMM0
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let denormal = f32::from_bits(1);
    mem.write_slice(&float_bits(denormal), GuestAddress(ALIGNED_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// MINSD Tests - Scalar Double Precision Minimum
// ============================================================================

#[test]
fn test_minsd_xmm0_xmm1() {
    let code = [
        0x0f, 0x28, 0xc1, // MOVAPS XMM0, XMM1
        0xf2, 0x0f, 0x5d, 0xc1, // MINSD XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_minsd_first_smaller() {
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
        0xf2, 0x0f, 0x5d, 0xc1, // MINSD XMM0, XMM1
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(&double_bits(1.0), GuestAddress(ALIGNED_ADDR))
        .unwrap();
    mem.write_slice(&double_bits(3.0), GuestAddress(ALIGNED_ADDR2))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_minsd_second_smaller() {
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
        0xf2, 0x0f, 0x5d, 0xc1, // MINSD XMM0, XMM1
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(&double_bits(5.0), GuestAddress(ALIGNED_ADDR))
        .unwrap();
    mem.write_slice(&double_bits(2.0), GuestAddress(ALIGNED_ADDR2))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_minsd_equal_values() {
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xf2, 0x0f, 0x10, 0x00, // MOVSD XMM0, [RAX]
        0xf2, 0x0f, 0x5d, 0xc0, // MINSD XMM0, XMM0
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(&double_bits(4.5), GuestAddress(ALIGNED_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_minsd_negative_values() {
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
        0xf2, 0x0f, 0x5d, 0xc1, // MINSD XMM0, XMM1
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
fn test_minsd_positive_and_negative() {
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
        0xf2, 0x0f, 0x5d, 0xc1, // MINSD XMM0, XMM1
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(&double_bits(5.0), GuestAddress(ALIGNED_ADDR))
        .unwrap();
    mem.write_slice(&double_bits(-3.0), GuestAddress(ALIGNED_ADDR2))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_minsd_both_zero() {
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xf2, 0x0f, 0x10, 0x00, // MOVSD XMM0, [RAX]
        0xf2, 0x0f, 0x5d, 0xc0, // MINSD XMM0, XMM0
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(&double_bits(0.0), GuestAddress(ALIGNED_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_minsd_positive_and_negative_zero() {
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
        0xf2, 0x0f, 0x5d, 0xc1, // MINSD XMM0, XMM1
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
fn test_minsd_infinity() {
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
        0xf2, 0x0f, 0x5d, 0xc1, // MINSD XMM0, XMM1
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
fn test_minsd_negative_infinity() {
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
        0xf2, 0x0f, 0x5d, 0xc1, // MINSD XMM0, XMM1
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
fn test_minsd_very_large_numbers() {
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
        0xf2, 0x0f, 0x5d, 0xc1, // MINSD XMM0, XMM1
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
fn test_minsd_very_small_numbers() {
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
        0xf2, 0x0f, 0x5d, 0xc1, // MINSD XMM0, XMM1
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(&double_bits(1e-100), GuestAddress(ALIGNED_ADDR))
        .unwrap();
    mem.write_slice(&double_bits(1e-200), GuestAddress(ALIGNED_ADDR2))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_minsd_high_precision_pi() {
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xf2, 0x0f, 0x10, 0x00, // MOVSD XMM0, [RAX]
        0xf2, 0x0f, 0x5d, 0xc0, // MINSD XMM0, XMM0
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(&double_bits(3.14159265358979), GuestAddress(ALIGNED_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_minsd_denormal_values() {
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xf2, 0x0f, 0x10, 0x00, // MOVSD XMM0, [RAX]
        0xf2, 0x0f, 0x5d, 0xc0, // MINSD XMM0, XMM0
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let denormal = f64::from_bits(1);
    mem.write_slice(&double_bits(denormal), GuestAddress(ALIGNED_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_minsd_both_infinities() {
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
        0xf2, 0x0f, 0x5d, 0xc1, // MINSD XMM0, XMM1
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
fn test_minsd_mixed_infinity_values() {
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
        0xf2, 0x0f, 0x5d, 0xc1, // MINSD XMM0, XMM1
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(&double_bits(f64::INFINITY), GuestAddress(ALIGNED_ADDR))
        .unwrap();
    mem.write_slice(&double_bits(f64::NEG_INFINITY), GuestAddress(ALIGNED_ADDR2))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}
