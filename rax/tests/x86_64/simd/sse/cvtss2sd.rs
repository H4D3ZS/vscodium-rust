use crate::common::*;

// CVTSS2SD - Convert Scalar Single Precision Floating-Point Value to Scalar Double Precision
//
// Converts a scalar single precision floating-point value in the source operand to a
// scalar double precision floating-point value in the destination operand.
// Upper bits of the destination are preserved from the first source operand.
//
// Opcode:
// F3 0F 5A /r    CVTSS2SD xmm1, xmm2/m32
//
// The conversion increases precision from 32-bit to 64-bit floating-point format.
// The upper 64 bits of the destination register are copied from the corresponding bits
// in the first source operand (for VEX/EVEX versions).

const DATA_ADDR: u64 = 0x3000;

// ============================================================================
// Basic Register to Register Tests
// ============================================================================

#[test]
fn test_cvtss2sd_xmm0_to_xmm1() {
    // CVTSS2SD XMM1, XMM0
    let code = [
        0xf3, 0x0f, 0x5a, 0xc8, // CVTSS2SD XMM1, XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvtss2sd_xmm2_to_xmm3() {
    // CVTSS2SD XMM3, XMM2
    let code = [
        0xf3, 0x0f, 0x5a, 0xda, // CVTSS2SD XMM3, XMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvtss2sd_xmm4_to_xmm5() {
    // CVTSS2SD XMM5, XMM4
    let code = [
        0xf3, 0x0f, 0x5a, 0xec, // CVTSS2SD XMM5, XMM4
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvtss2sd_xmm6_to_xmm7() {
    // CVTSS2SD XMM7, XMM6
    let code = [
        0xf3, 0x0f, 0x5a, 0xfe, // CVTSS2SD XMM7, XMM6
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvtss2sd_xmm8_to_xmm9() {
    // CVTSS2SD XMM9, XMM8
    let code = [
        0xf3, 0x45, 0x0f, 0x5a, 0xc8, // CVTSS2SD XMM9, XMM8
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvtss2sd_xmm10_to_xmm11() {
    // CVTSS2SD XMM11, XMM10
    let code = [
        0xf3, 0x45, 0x0f, 0x5a, 0xda, // CVTSS2SD XMM11, XMM10
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvtss2sd_xmm14_to_xmm15() {
    // CVTSS2SD XMM15, XMM14
    let code = [
        0xf3, 0x45, 0x0f, 0x5a, 0xfe, // CVTSS2SD XMM15, XMM14
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvtss2sd_xmm0_to_xmm15() {
    // CVTSS2SD XMM15, XMM0
    let code = [
        0xf3, 0x44, 0x0f, 0x5a, 0xf8, // CVTSS2SD XMM15, XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvtss2sd_xmm15_to_xmm0() {
    // CVTSS2SD XMM0, XMM15
    let code = [
        0xf3, 0x41, 0x0f, 0x5a, 0xc7, // CVTSS2SD XMM0, XMM15
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Memory to Register Tests
// ============================================================================

#[test]
fn test_cvtss2sd_mem_to_xmm0() {
    // CVTSS2SD XMM0, [RAX]
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xf3, 0x0f, 0x5a, 0x00, // CVTSS2SD XMM0, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);

    let f1: f32 = 1.0;
    mem.write_slice(&f1.to_le_bytes(), vm_memory::GuestAddress(DATA_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvtss2sd_mem_to_xmm1() {
    // CVTSS2SD XMM1, [RBX]
    let code = [
        0x48, 0xbb, // MOV RBX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xf3, 0x0f, 0x5a, 0x0b, // CVTSS2SD XMM1, [RBX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);

    let f1: f32 = -3.14159;
    mem.write_slice(&f1.to_le_bytes(), vm_memory::GuestAddress(DATA_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvtss2sd_mem_to_xmm7() {
    // CVTSS2SD XMM7, [RCX]
    let code = [
        0x48, 0xb9, // MOV RCX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xf3, 0x0f, 0x5a, 0x39, // CVTSS2SD XMM7, [RCX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);

    let f1: f32 = 42.5;
    mem.write_slice(&f1.to_le_bytes(), vm_memory::GuestAddress(DATA_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvtss2sd_mem_to_xmm8() {
    // CVTSS2SD XMM8, [RDX]
    let code = [
        0x48, 0xba, // MOV RDX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xf3, 0x44, 0x0f, 0x5a, 0x02, // CVTSS2SD XMM8, [RDX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);

    let f1: f32 = -99.999;
    mem.write_slice(&f1.to_le_bytes(), vm_memory::GuestAddress(DATA_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvtss2sd_mem_to_xmm15() {
    // CVTSS2SD XMM15, [RSI]
    let code = [
        0x48, 0xbe, // MOV RSI, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xf3, 0x44, 0x0f, 0x5a, 0x3e, // CVTSS2SD XMM15, [RSI]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);

    let f1: f32 = 0.125;
    mem.write_slice(&f1.to_le_bytes(), vm_memory::GuestAddress(DATA_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Special Float Values - Zeros
// ============================================================================

#[test]
fn test_cvtss2sd_positive_zero() {
    // Test conversion of +0.0f
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xf3, 0x0f, 0x5a, 0x00, // CVTSS2SD XMM0, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);

    let f1: f32 = 0.0;
    mem.write_slice(&f1.to_le_bytes(), vm_memory::GuestAddress(DATA_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvtss2sd_negative_zero() {
    // Test conversion of -0.0f
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xf3, 0x0f, 0x5a, 0x00, // CVTSS2SD XMM0, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);

    let f1: f32 = -0.0;
    mem.write_slice(&f1.to_le_bytes(), vm_memory::GuestAddress(DATA_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Special Float Values - Infinity
// ============================================================================

#[test]
fn test_cvtss2sd_positive_infinity() {
    // Test conversion of +Inf
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xf3, 0x0f, 0x5a, 0x00, // CVTSS2SD XMM0, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);

    let f1: f32 = f32::INFINITY;
    mem.write_slice(&f1.to_le_bytes(), vm_memory::GuestAddress(DATA_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvtss2sd_negative_infinity() {
    // Test conversion of -Inf
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xf3, 0x0f, 0x5a, 0x00, // CVTSS2SD XMM0, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);

    let f1: f32 = f32::NEG_INFINITY;
    mem.write_slice(&f1.to_le_bytes(), vm_memory::GuestAddress(DATA_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Special Float Values - NaN
// ============================================================================

#[test]
fn test_cvtss2sd_quiet_nan() {
    // Test conversion of quiet NaN
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xf3, 0x0f, 0x5a, 0x00, // CVTSS2SD XMM0, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);

    let f1: f32 = f32::NAN;
    mem.write_slice(&f1.to_le_bytes(), vm_memory::GuestAddress(DATA_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvtss2sd_signaling_nan() {
    // Test conversion of signaling NaN
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xf3, 0x0f, 0x5a, 0x00, // CVTSS2SD XMM0, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);

    // Signaling NaN bit pattern
    let snan: u32 = 0x7F800001;
    mem.write_slice(&snan.to_le_bytes(), vm_memory::GuestAddress(DATA_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Special Float Values - Denormals
// ============================================================================

#[test]
fn test_cvtss2sd_denormal_positive() {
    // Test conversion of positive denormal
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xf3, 0x0f, 0x5a, 0x00, // CVTSS2SD XMM0, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);

    // Smallest positive denormal
    let denorm: u32 = 0x00000001;
    mem.write_slice(&denorm.to_le_bytes(), vm_memory::GuestAddress(DATA_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvtss2sd_denormal_negative() {
    // Test conversion of negative denormal
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xf3, 0x0f, 0x5a, 0x00, // CVTSS2SD XMM0, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);

    // Smallest negative denormal
    let denorm: u32 = 0x80000001;
    mem.write_slice(&denorm.to_le_bytes(), vm_memory::GuestAddress(DATA_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvtss2sd_denormal_largest() {
    // Test conversion of largest denormal
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xf3, 0x0f, 0x5a, 0x00, // CVTSS2SD XMM0, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);

    // Largest positive denormal
    let denorm: u32 = 0x007FFFFF;
    mem.write_slice(&denorm.to_le_bytes(), vm_memory::GuestAddress(DATA_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Normal Float Value Tests
// ============================================================================

#[test]
fn test_cvtss2sd_small_values() {
    // Test conversion of small normal values
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xf3, 0x0f, 0x5a, 0x00, // CVTSS2SD XMM0, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);

    let f1: f32 = 1.0e-30;
    mem.write_slice(&f1.to_le_bytes(), vm_memory::GuestAddress(DATA_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvtss2sd_large_values() {
    // Test conversion of large normal values
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xf3, 0x0f, 0x5a, 0x00, // CVTSS2SD XMM0, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);

    let f1: f32 = 1.0e30;
    mem.write_slice(&f1.to_le_bytes(), vm_memory::GuestAddress(DATA_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvtss2sd_fractional_values() {
    // Test conversion of fractional values
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xf3, 0x0f, 0x5a, 0x00, // CVTSS2SD XMM0, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);

    let f1: f32 = 0.333333;
    mem.write_slice(&f1.to_le_bytes(), vm_memory::GuestAddress(DATA_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvtss2sd_powers_of_two() {
    // Test conversion of powers of two (exact conversion)
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xf3, 0x0f, 0x5a, 0x00, // CVTSS2SD XMM0, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);

    let f1: f32 = 128.0;
    mem.write_slice(&f1.to_le_bytes(), vm_memory::GuestAddress(DATA_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvtss2sd_negative_values() {
    // Test conversion of negative values
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xf3, 0x0f, 0x5a, 0x00, // CVTSS2SD XMM0, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);

    let f1: f32 = -42.5;
    mem.write_slice(&f1.to_le_bytes(), vm_memory::GuestAddress(DATA_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Addressing Mode Tests
// ============================================================================

#[test]
fn test_cvtss2sd_with_displacement() {
    // CVTSS2SD XMM0, [RAX + 16]
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&(DATA_ADDR - 16).to_le_bytes());
    full_code.extend_from_slice(&[
        0xf3, 0x0f, 0x5a, 0x40, 0x10, // CVTSS2SD XMM0, [RAX + 16]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);

    let f1: f32 = 7.5;
    mem.write_slice(&f1.to_le_bytes(), vm_memory::GuestAddress(DATA_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvtss2sd_with_negative_displacement() {
    // CVTSS2SD XMM1, [RBX - 8]
    let code = [
        0x48, 0xbb, // MOV RBX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&(DATA_ADDR + 8).to_le_bytes());
    full_code.extend_from_slice(&[
        0xf3, 0x0f, 0x5a, 0x4b, 0xf8, // CVTSS2SD XMM1, [RBX - 8]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);

    let f1: f32 = -99.99;
    mem.write_slice(&f1.to_le_bytes(), vm_memory::GuestAddress(DATA_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvtss2sd_rip_relative() {
    // CVTSS2SD XMM0, [RIP + disp]
    let code = [
        0xf3, 0x0f, 0x5a, 0x05, 0x00, 0x00, 0x00, 0x00, // CVTSS2SD XMM0, [RIP + 0]
        0xf4, // HLT
    ];

    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Upper Bits Preservation Tests
// ============================================================================

#[test]
fn test_cvtss2sd_upper_bits_preserved_reg() {
    // For legacy SSE, upper bits are unmodified
    // This tests register-to-register preserving upper bits
    let code = [
        0xf3, 0x0f, 0x5a, 0xc8, // CVTSS2SD XMM1, XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvtss2sd_upper_bits_preserved_mem() {
    // For legacy SSE, upper bits are unmodified
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xf3, 0x0f, 0x5a, 0x00, // CVTSS2SD XMM0, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);

    let f1: f32 = 123.456;
    mem.write_slice(&f1.to_le_bytes(), vm_memory::GuestAddress(DATA_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Edge Case Tests
// ============================================================================

#[test]
fn test_cvtss2sd_max_float32() {
    // Test conversion of maximum float32 value
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xf3, 0x0f, 0x5a, 0x00, // CVTSS2SD XMM0, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);

    let f1: f32 = f32::MAX;
    mem.write_slice(&f1.to_le_bytes(), vm_memory::GuestAddress(DATA_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvtss2sd_min_positive_float32() {
    // Test conversion of minimum positive normalized float32
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xf3, 0x0f, 0x5a, 0x00, // CVTSS2SD XMM0, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);

    let f1: f32 = f32::MIN_POSITIVE;
    mem.write_slice(&f1.to_le_bytes(), vm_memory::GuestAddress(DATA_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvtss2sd_one() {
    // Test conversion of 1.0
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xf3, 0x0f, 0x5a, 0x00, // CVTSS2SD XMM0, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);

    let f1: f32 = 1.0;
    mem.write_slice(&f1.to_le_bytes(), vm_memory::GuestAddress(DATA_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvtss2sd_minus_one() {
    // Test conversion of -1.0
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xf3, 0x0f, 0x5a, 0x00, // CVTSS2SD XMM0, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);

    let f1: f32 = -1.0;
    mem.write_slice(&f1.to_le_bytes(), vm_memory::GuestAddress(DATA_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvtss2sd_pi() {
    // Test conversion of pi
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xf3, 0x0f, 0x5a, 0x00, // CVTSS2SD XMM0, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);

    let f1: f32 = std::f32::consts::PI;
    mem.write_slice(&f1.to_le_bytes(), vm_memory::GuestAddress(DATA_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvtss2sd_multiple_conversions() {
    // Multiple CVTSS2SD operations in sequence
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xf3, 0x0f, 0x5a, 0x00, // CVTSS2SD XMM0, [RAX]
        0xf3, 0x0f, 0x5a, 0x08, // CVTSS2SD XMM1, [RAX]
        0xf3, 0x0f, 0x5a, 0x10, // CVTSS2SD XMM2, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);

    let f1: f32 = 3.14159;
    mem.write_slice(&f1.to_le_bytes(), vm_memory::GuestAddress(DATA_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}
