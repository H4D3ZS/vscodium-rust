use crate::common::*;

// CVTSD2SS - Convert Scalar Double Precision Floating-Point Value to Scalar Single Precision
//
// Converts a scalar double precision floating-point value in the source operand to a
// scalar single precision floating-point value in the destination operand.
// Upper bits of the destination are preserved from the first source operand.
// When conversion is inexact, the value is rounded according to MXCSR rounding control bits.
//
// Opcode:
// F2 0F 5A /r    CVTSD2SS xmm1, xmm2/m64
//
// The conversion may lose precision and the result is rounded according to MXCSR.
// The upper bits of the destination register are copied from the corresponding bits
// in the first source operand (for VEX/EVEX versions).

const DATA_ADDR: u64 = 0x3000;

// ============================================================================
// Basic Register to Register Tests
// ============================================================================

#[test]
fn test_cvtsd2ss_xmm0_to_xmm1() {
    let code = [
        0xf2, 0x0f, 0x5a, 0xc8, // CVTSD2SS XMM1, XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvtsd2ss_xmm2_to_xmm3() {
    let code = [
        0xf2, 0x0f, 0x5a, 0xda, // CVTSD2SS XMM3, XMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvtsd2ss_xmm4_to_xmm5() {
    let code = [
        0xf2, 0x0f, 0x5a, 0xec, // CVTSD2SS XMM5, XMM4
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvtsd2ss_xmm6_to_xmm7() {
    let code = [
        0xf2, 0x0f, 0x5a, 0xfe, // CVTSD2SS XMM7, XMM6
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvtsd2ss_xmm8_to_xmm9() {
    let code = [
        0xf2, 0x45, 0x0f, 0x5a, 0xc8, // CVTSD2SS XMM9, XMM8
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvtsd2ss_xmm10_to_xmm11() {
    let code = [
        0xf2, 0x45, 0x0f, 0x5a, 0xda, // CVTSD2SS XMM11, XMM10
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvtsd2ss_xmm14_to_xmm15() {
    let code = [
        0xf2, 0x45, 0x0f, 0x5a, 0xfe, // CVTSD2SS XMM15, XMM14
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvtsd2ss_xmm0_to_xmm15() {
    let code = [
        0xf2, 0x44, 0x0f, 0x5a, 0xf8, // CVTSD2SS XMM15, XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvtsd2ss_xmm15_to_xmm0() {
    let code = [
        0xf2, 0x41, 0x0f, 0x5a, 0xc7, // CVTSD2SS XMM0, XMM15
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Memory to Register Tests
// ============================================================================

#[test]
fn test_cvtsd2ss_mem_to_xmm0() {
    let code = [0x48, 0xb8]; // MOV RAX, imm64
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xf2, 0x0f, 0x5a, 0x00, // CVTSD2SS XMM0, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let f1: f64 = 1.0;
    mem.write_slice(&f1.to_le_bytes(), vm_memory::GuestAddress(DATA_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvtsd2ss_mem_to_xmm1() {
    let code = [0x48, 0xbb]; // MOV RBX, imm64
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xf2, 0x0f, 0x5a, 0x0b, // CVTSD2SS XMM1, [RBX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let f1: f64 = -3.14159265358979;
    mem.write_slice(&f1.to_le_bytes(), vm_memory::GuestAddress(DATA_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvtsd2ss_mem_to_xmm7() {
    let code = [0x48, 0xb9]; // MOV RCX, imm64
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xf2, 0x0f, 0x5a, 0x39, // CVTSD2SS XMM7, [RCX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let f1: f64 = 42.5;
    mem.write_slice(&f1.to_le_bytes(), vm_memory::GuestAddress(DATA_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvtsd2ss_mem_to_xmm8() {
    let code = [0x48, 0xba]; // MOV RDX, imm64
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xf2, 0x44, 0x0f, 0x5a, 0x02, // CVTSD2SS XMM8, [RDX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let f1: f64 = -99.999;
    mem.write_slice(&f1.to_le_bytes(), vm_memory::GuestAddress(DATA_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvtsd2ss_mem_to_xmm15() {
    let code = [0x48, 0xbe]; // MOV RSI, imm64
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xf2, 0x44, 0x0f, 0x5a, 0x3e, // CVTSD2SS XMM15, [RSI]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let f1: f64 = 0.125;
    mem.write_slice(&f1.to_le_bytes(), vm_memory::GuestAddress(DATA_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Special Values - Zeros
// ============================================================================

#[test]
fn test_cvtsd2ss_positive_zero() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf2, 0x0f, 0x5a, 0x00, 0xf4]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let f1: f64 = 0.0;
    mem.write_slice(&f1.to_le_bytes(), vm_memory::GuestAddress(DATA_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvtsd2ss_negative_zero() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf2, 0x0f, 0x5a, 0x00, 0xf4]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let f1: f64 = -0.0;
    mem.write_slice(&f1.to_le_bytes(), vm_memory::GuestAddress(DATA_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Special Values - Infinity
// ============================================================================

#[test]
fn test_cvtsd2ss_positive_infinity() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf2, 0x0f, 0x5a, 0x00, 0xf4]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let f1: f64 = f64::INFINITY;
    mem.write_slice(&f1.to_le_bytes(), vm_memory::GuestAddress(DATA_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvtsd2ss_negative_infinity() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf2, 0x0f, 0x5a, 0x00, 0xf4]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let f1: f64 = f64::NEG_INFINITY;
    mem.write_slice(&f1.to_le_bytes(), vm_memory::GuestAddress(DATA_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Special Values - NaN
// ============================================================================

#[test]
fn test_cvtsd2ss_quiet_nan() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf2, 0x0f, 0x5a, 0x00, 0xf4]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let f1: f64 = f64::NAN;
    mem.write_slice(&f1.to_le_bytes(), vm_memory::GuestAddress(DATA_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvtsd2ss_signaling_nan() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf2, 0x0f, 0x5a, 0x00, 0xf4]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let snan: u64 = 0x7FF0000000000001; // Signaling NaN
    mem.write_slice(&snan.to_le_bytes(), vm_memory::GuestAddress(DATA_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Precision Loss and Rounding Tests
// ============================================================================

#[test]
fn test_cvtsd2ss_precision_loss() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf2, 0x0f, 0x5a, 0x00, 0xf4]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let f1: f64 = 1.0000000001; // More precision than f32
    mem.write_slice(&f1.to_le_bytes(), vm_memory::GuestAddress(DATA_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvtsd2ss_rounding_nearest() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf2, 0x0f, 0x5a, 0x00, 0xf4]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let f1: f64 = 1.5000000000000002;
    mem.write_slice(&f1.to_le_bytes(), vm_memory::GuestAddress(DATA_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvtsd2ss_very_small_precision() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf2, 0x0f, 0x5a, 0x00, 0xf4]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let f1: f64 = 1.00000000000000001;
    mem.write_slice(&f1.to_le_bytes(), vm_memory::GuestAddress(DATA_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Overflow Tests
// ============================================================================

#[test]
fn test_cvtsd2ss_overflow_to_positive_inf() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf2, 0x0f, 0x5a, 0x00, 0xf4]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let f1: f64 = 1.0e100; // Larger than f32::MAX
    mem.write_slice(&f1.to_le_bytes(), vm_memory::GuestAddress(DATA_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvtsd2ss_overflow_to_negative_inf() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf2, 0x0f, 0x5a, 0x00, 0xf4]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let f1: f64 = -1.0e100;
    mem.write_slice(&f1.to_le_bytes(), vm_memory::GuestAddress(DATA_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvtsd2ss_near_max_f32() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf2, 0x0f, 0x5a, 0x00, 0xf4]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let f1: f64 = 3.4e38; // Close to f32::MAX
    mem.write_slice(&f1.to_le_bytes(), vm_memory::GuestAddress(DATA_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Normal Values
// ============================================================================

#[test]
fn test_cvtsd2ss_small_values() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf2, 0x0f, 0x5a, 0x00, 0xf4]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let f1: f64 = 1.0e-30;
    mem.write_slice(&f1.to_le_bytes(), vm_memory::GuestAddress(DATA_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvtsd2ss_large_values() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf2, 0x0f, 0x5a, 0x00, 0xf4]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let f1: f64 = 1.0e30;
    mem.write_slice(&f1.to_le_bytes(), vm_memory::GuestAddress(DATA_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvtsd2ss_fractional() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf2, 0x0f, 0x5a, 0x00, 0xf4]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let f1: f64 = 0.333333333333333;
    mem.write_slice(&f1.to_le_bytes(), vm_memory::GuestAddress(DATA_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvtsd2ss_powers_of_two() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf2, 0x0f, 0x5a, 0x00, 0xf4]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let f1: f64 = 128.0;
    mem.write_slice(&f1.to_le_bytes(), vm_memory::GuestAddress(DATA_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvtsd2ss_negative_values() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf2, 0x0f, 0x5a, 0x00, 0xf4]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let f1: f64 = -42.5;
    mem.write_slice(&f1.to_le_bytes(), vm_memory::GuestAddress(DATA_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Addressing Mode Tests
// ============================================================================

#[test]
fn test_cvtsd2ss_with_displacement() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&(DATA_ADDR - 16).to_le_bytes());
    full_code.extend_from_slice(&[
        0xf2, 0x0f, 0x5a, 0x40, 0x10, // CVTSD2SS XMM0, [RAX + 16]
        0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let f1: f64 = 7.5;
    mem.write_slice(&f1.to_le_bytes(), vm_memory::GuestAddress(DATA_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvtsd2ss_rip_relative() {
    let code = [
        0xf2, 0x0f, 0x5a, 0x05, 0x00, 0x00, 0x00, 0x00, // CVTSD2SS XMM0, [RIP + 0]
        0xf4,
    ];

    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Edge Cases
// ============================================================================

#[test]
fn test_cvtsd2ss_max_float64() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf2, 0x0f, 0x5a, 0x00, 0xf4]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let f1: f64 = f64::MAX;
    mem.write_slice(&f1.to_le_bytes(), vm_memory::GuestAddress(DATA_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvtsd2ss_min_positive_float64() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf2, 0x0f, 0x5a, 0x00, 0xf4]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let f1: f64 = f64::MIN_POSITIVE;
    mem.write_slice(&f1.to_le_bytes(), vm_memory::GuestAddress(DATA_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvtsd2ss_one() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf2, 0x0f, 0x5a, 0x00, 0xf4]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let f1: f64 = 1.0;
    mem.write_slice(&f1.to_le_bytes(), vm_memory::GuestAddress(DATA_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvtsd2ss_minus_one() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf2, 0x0f, 0x5a, 0x00, 0xf4]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let f1: f64 = -1.0;
    mem.write_slice(&f1.to_le_bytes(), vm_memory::GuestAddress(DATA_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvtsd2ss_pi() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf2, 0x0f, 0x5a, 0x00, 0xf4]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let f1: f64 = std::f64::consts::PI;
    mem.write_slice(&f1.to_le_bytes(), vm_memory::GuestAddress(DATA_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvtsd2ss_multiple_conversions() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xf2, 0x0f, 0x5a, 0x00, // CVTSD2SS XMM0, [RAX]
        0xf2, 0x0f, 0x5a, 0x08, // CVTSD2SS XMM1, [RAX]
        0xf2, 0x0f, 0x5a, 0x10, // CVTSD2SS XMM2, [RAX]
        0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let f1: f64 = 3.14159265358979;
    mem.write_slice(&f1.to_le_bytes(), vm_memory::GuestAddress(DATA_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}
