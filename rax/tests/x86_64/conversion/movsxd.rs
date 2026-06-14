use rax::cpu::Registers;

use crate::common::{run_until_hlt, setup_vm};

// MOVSXD — Move With Sign-Extension (Doubleword to Quadword)
// Opcode: 63 /r
// This instruction requires REX.W prefix to convert 32-bit to 64-bit
// Sign-extends a 32-bit signed integer to 64-bit

// ============================================================================
// MOVSXD r64, r/m32 - Basic Conversions
// ============================================================================

#[test]
fn test_movsxd_rax_ebx_zero() {
    let code = [0x48, 0x63, 0xc3, 0xf4]; // MOVSXD RAX, EBX
    let mut regs = Registers::default();
    regs.rbx = 0x00000000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x0000000000000000, "RAX should be 0");
}

#[test]
fn test_movsxd_rax_ebx_positive_one() {
    let code = [0x48, 0x63, 0xc3, 0xf4]; // MOVSXD RAX, EBX
    let mut regs = Registers::default();
    regs.rbx = 0x00000001;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x0000000000000001, "RAX should be 1");
}

#[test]
fn test_movsxd_rax_ebx_negative_one() {
    let code = [0x48, 0x63, 0xc3, 0xf4]; // MOVSXD RAX, EBX
    let mut regs = Registers::default();
    regs.rbx = 0xFFFFFFFF; // -1 in 32-bit signed
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax, 0xFFFFFFFFFFFFFFFF,
        "RAX should be -1 sign-extended to 64-bit"
    );
}

#[test]
fn test_movsxd_rax_ebx_max_positive() {
    let code = [0x48, 0x63, 0xc3, 0xf4]; // MOVSXD RAX, EBX
    let mut regs = Registers::default();
    regs.rbx = 0x7FFFFFFF; // 2147483647 (max 32-bit signed)
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax, 0x000000007FFFFFFF,
        "RAX should be max positive 32-bit signed"
    );
}

#[test]
fn test_movsxd_rax_ebx_min_negative() {
    let code = [0x48, 0x63, 0xc3, 0xf4]; // MOVSXD RAX, EBX
    let mut regs = Registers::default();
    regs.rbx = 0x80000000; // -2147483648 (min 32-bit signed)
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax, 0xFFFFFFFF80000000,
        "RAX should be min negative 32-bit signed"
    );
}

#[test]
fn test_movsxd_rcx_edx_positive() {
    let code = [0x48, 0x63, 0xca, 0xf4]; // MOVSXD RCX, EDX
    let mut regs = Registers::default();
    regs.rdx = 0x12345678;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rcx, 0x0000000012345678,
        "RCX should be sign-extended from EDX"
    );
}

#[test]
fn test_movsxd_rcx_edx_negative() {
    let code = [0x48, 0x63, 0xca, 0xf4]; // MOVSXD RCX, EDX
    let mut regs = Registers::default();
    regs.rdx = 0x87654321; // Negative
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rcx, 0xFFFFFFFF87654321,
        "RCX should be sign-extended from EDX"
    );
}

#[test]
fn test_movsxd_rdi_esi() {
    let code = [0x48, 0x63, 0xfe, 0xf4]; // MOVSXD RDI, ESI
    let mut regs = Registers::default();
    regs.rsi = 0xDEADBEEF; // Negative
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rdi, 0xFFFFFFFFDEADBEEF,
        "RDI should be sign-extended from ESI"
    );
}

#[test]
fn test_movsxd_rbx_eax() {
    let code = [0x48, 0x63, 0xd8, 0xf4]; // MOVSXD RBX, EAX
    let mut regs = Registers::default();
    regs.rax = 0x11223344;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rbx, 0x0000000011223344,
        "RBX should be sign-extended from EAX"
    );
}

// ============================================================================
// MOVSXD with Extended Registers (R8-R15)
// ============================================================================

#[test]
fn test_movsxd_r8_ebx() {
    // REX.W + REX.R = 0x4C (R8 as dest), 0x4D would incorrectly set REX.B
    let code = [0x4c, 0x63, 0xc3, 0xf4]; // MOVSXD R8, EBX
    let mut regs = Registers::default();
    regs.rbx = 0x7FFFFFFF; // Positive
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.r8, 0x000000007FFFFFFF,
        "R8 should be sign-extended from EBX"
    );
}

#[test]
fn test_movsxd_r9_ecx() {
    // REX.W + REX.R = 0x4C
    let code = [0x4c, 0x63, 0xc9, 0xf4]; // MOVSXD R9, ECX
    let mut regs = Registers::default();
    regs.rcx = 0x80000000; // Negative
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.r9, 0xFFFFFFFF80000000,
        "R9 should be sign-extended from ECX"
    );
}

#[test]
fn test_movsxd_r10_edx() {
    // REX.W + REX.R = 0x4C (R10 as dest), ModRM 0xD2 = reg=2, r/m=2 (EDX as source)
    // 0x4D would set REX.B which extends r/m to R10D
    let code = [0x4c, 0x63, 0xd2, 0xf4]; // MOVSXD R10, EDX
    let mut regs = Registers::default();
    regs.rdx = 0xFFFFFFFF; // -1 in i32
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.r10, 0xFFFFFFFFFFFFFFFF, "R10 should be -1");
}

#[test]
fn test_movsxd_r11_esi() {
    // REX.W + REX.R = 0x4C (R11 as dest), not 0x4D which sets REX.B
    let code = [0x4c, 0x63, 0xde, 0xf4]; // MOVSXD R11, ESI
    let mut regs = Registers::default();
    regs.rsi = 0x42424242;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.r11, 0x0000000042424242,
        "R11 should be sign-extended from ESI"
    );
}

#[test]
fn test_movsxd_r12_edi() {
    // REX.W + REX.R = 0x4C
    let code = [0x4c, 0x63, 0xe7, 0xf4]; // MOVSXD R12, EDI
    let mut regs = Registers::default();
    regs.rdi = 0xF0F0F0F0; // Negative
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.r12, 0xFFFFFFFFF0F0F0F0,
        "R12 should be sign-extended from EDI"
    );
}

#[test]
fn test_movsxd_r13_eax() {
    // REX.W + REX.R = 0x4C
    let code = [0x4c, 0x63, 0xe8, 0xf4]; // MOVSXD R13, EAX
    let mut regs = Registers::default();
    regs.rax = 0x00000000; // Zero
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.r13, 0x0000000000000000, "R13 should be 0");
}

#[test]
fn test_movsxd_r14_ebx() {
    // REX.W + REX.R = 0x4C
    let code = [0x4c, 0x63, 0xf3, 0xf4]; // MOVSXD R14, EBX
    let mut regs = Registers::default();
    regs.rbx = 0x00000001; // Positive
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.r14, 0x0000000000000001, "R14 should be 1");
}

#[test]
fn test_movsxd_r15_ecx() {
    // REX.W + REX.R = 0x4C
    let code = [0x4c, 0x63, 0xf9, 0xf4]; // MOVSXD R15, ECX
    let mut regs = Registers::default();
    regs.rcx = 0x7FFFFFFF; // Max positive
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.r15, 0x000000007FFFFFFF,
        "R15 should be max positive 32-bit signed"
    );
}

// ============================================================================
// MOVSXD from Extended Registers (R8-R15) to RAX-RDI
// ============================================================================

#[test]
fn test_movsxd_rax_r8d() {
    let code = [0x49, 0x63, 0xc0, 0xf4]; // MOVSXD RAX, R8D
    let mut regs = Registers::default();
    regs.r8 = 0x80000000; // Negative
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax, 0xFFFFFFFF80000000,
        "RAX should be sign-extended from R8D"
    );
}

#[test]
fn test_movsxd_rcx_r9d() {
    let code = [0x49, 0x63, 0xc9, 0xf4]; // MOVSXD RCX, R9D
    let mut regs = Registers::default();
    regs.r9 = 0x7FFFFFFF; // Positive
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rcx, 0x000000007FFFFFFF,
        "RCX should be sign-extended from R9D"
    );
}

#[test]
fn test_movsxd_rdx_r10d() {
    let code = [0x49, 0x63, 0xd2, 0xf4]; // MOVSXD RDX, R10D
    let mut regs = Registers::default();
    regs.r10 = 0xFFFFFFFF; // -1
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rdx, 0xFFFFFFFFFFFFFFFF, "RDX should be -1");
}

#[test]
fn test_movsxd_rsi_r11d() {
    let code = [0x49, 0x63, 0xf3, 0xf4]; // MOVSXD RSI, R11D
    let mut regs = Registers::default();
    regs.r11 = 0x11223344; // Positive
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rsi, 0x0000000011223344,
        "RSI should be sign-extended from R11D"
    );
}

#[test]
fn test_movsxd_rdi_r12d() {
    let code = [0x49, 0x63, 0xfc, 0xf4]; // MOVSXD RDI, R12D
    let mut regs = Registers::default();
    regs.r12 = 0xDEADBEEF; // Negative
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rdi, 0xFFFFFFFFDEADBEEF,
        "RDI should be sign-extended from R12D"
    );
}

// ============================================================================
// MOVSXD Between Extended Registers
// ============================================================================

#[test]
fn test_movsxd_r8_r9d() {
    let code = [0x4d, 0x63, 0xc1, 0xf4]; // MOVSXD R8, R9D
    let mut regs = Registers::default();
    regs.r9 = 0x80000000; // Negative
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.r8, 0xFFFFFFFF80000000,
        "R8 should be sign-extended from R9D"
    );
}

#[test]
fn test_movsxd_r10_r11d() {
    let code = [0x4d, 0x63, 0xd3, 0xf4]; // MOVSXD R10, R11D
    let mut regs = Registers::default();
    regs.r11 = 0x7FFFFFFF; // Positive
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.r10, 0x000000007FFFFFFF,
        "R10 should be sign-extended from R11D"
    );
}

#[test]
fn test_movsxd_r12_r13d() {
    let code = [0x4d, 0x63, 0xe5, 0xf4]; // MOVSXD R12, R13D
    let mut regs = Registers::default();
    regs.r13 = 0xFFFFFFFF; // -1
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.r12, 0xFFFFFFFFFFFFFFFF, "R12 should be -1");
}

#[test]
fn test_movsxd_r14_r15d() {
    let code = [0x4d, 0x63, 0xf7, 0xf4]; // MOVSXD R14, R15D
    let mut regs = Registers::default();
    regs.r15 = 0x00000001; // Positive 1
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.r14, 0x0000000000000001, "R14 should be 1");
}

// ============================================================================
// Boundary Cases
// ============================================================================

#[test]
fn test_movsxd_boundary_0x7FFFFFFE() {
    let code = [0x48, 0x63, 0xc3, 0xf4]; // MOVSXD RAX, EBX
    let mut regs = Registers::default();
    regs.rbx = 0x7FFFFFFE; // 2147483646
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x000000007FFFFFFE, "RAX should be 0x7FFFFFFE");
}

#[test]
fn test_movsxd_boundary_0x80000001() {
    let code = [0x48, 0x63, 0xc3, 0xf4]; // MOVSXD RAX, EBX
    let mut regs = Registers::default();
    regs.rbx = 0x80000001; // -2147483647
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0xFFFFFFFF80000001, "RAX should be -2147483647");
}

#[test]
fn test_movsxd_boundary_positive_zero() {
    let code = [0x48, 0x63, 0xc3, 0xf4]; // MOVSXD RAX, EBX
    let mut regs = Registers::default();
    regs.rbx = 0x00000000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x0000000000000000, "RAX should be 0");
}

#[test]
fn test_movsxd_boundary_negative_zero() {
    let code = [0x48, 0x63, 0xc3, 0xf4]; // MOVSXD RAX, EBX
    let mut regs = Registers::default();
    regs.rbx = 0x00000000; // Zero (no sign bit)
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x0000000000000000, "RAX should be 0");
}

// ============================================================================
// Sign Extension Verification
// ============================================================================

#[test]
fn test_movsxd_sign_extension_pattern_0x00000001() {
    let code = [0x48, 0x63, 0xc3, 0xf4]; // MOVSXD RAX, EBX
    let mut regs = Registers::default();
    regs.rbx = 0x00000001; // Sign bit is 0
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    // Upper 32 bits should be all 0s
    assert_eq!(regs.rax >> 32, 0x00000000, "Upper 32 bits should be 0");
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x00000001,
        "Lower 32 bits should be 1"
    );
}

#[test]
fn test_movsxd_sign_extension_pattern_0xFFFFFFFF() {
    let code = [0x48, 0x63, 0xc3, 0xf4]; // MOVSXD RAX, EBX
    let mut regs = Registers::default();
    regs.rbx = 0xFFFFFFFF; // Sign bit is 1 (all 1s for negative)
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    // Upper 32 bits should be all 1s
    assert_eq!(regs.rax >> 32, 0xFFFFFFFF, "Upper 32 bits should be F");
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0xFFFFFFFF,
        "Lower 32 bits should be F"
    );
}

#[test]
fn test_movsxd_sign_extension_pattern_0x80000000() {
    let code = [0x48, 0x63, 0xc3, 0xf4]; // MOVSXD RAX, EBX
    let mut regs = Registers::default();
    regs.rbx = 0x80000000; // Sign bit is 1 (min negative)
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    // Upper 32 bits should be all 1s
    assert_eq!(regs.rax >> 32, 0xFFFFFFFF, "Upper 32 bits should be F");
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x80000000,
        "Lower 32 bits should be 80000000"
    );
}

// ============================================================================
// Source Register Unchanged
// ============================================================================

#[test]
fn test_movsxd_source_unchanged() {
    let code = [0x48, 0x63, 0xc3, 0xf4]; // MOVSXD RAX, EBX
    let mut regs = Registers::default();
    regs.rbx = 0xDEADBEEF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rbx, 0xDEADBEEF, "RBX should be unchanged");
}

#[test]
fn test_movsxd_multiple_consecutive() {
    let code = [
        0x48, 0x63, 0xc3, // MOVSXD RAX, EBX
        0x48, 0x63, 0xca, // MOVSXD RCX, EDX
        0x48, 0x63, 0xd1, // MOVSXD RDX, ECX (uses new RCX value)
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x7FFFFFFF; // Positive
    regs.rdx = 0xFFFFFFFF; // -1
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x000000007FFFFFFF, "RAX should have EBX value");
    assert_eq!(
        regs.rcx, 0xFFFFFFFFFFFFFFFF,
        "RCX should have EDX value (-1)"
    );
}

// ============================================================================
// Practical Use Cases
// ============================================================================

#[test]
fn test_movsxd_practical_i32_to_i64_conversion() {
    let code = [0x48, 0x63, 0xc0, 0xf4]; // MOVSXD RAX, EAX
    let mut regs = Registers::default();
    regs.rax = 0xDEADBEEF_DEADBEEF; // Set garbage in upper half
    regs.rax = 0xDEADBEEF80000001; // -2147483647 in lower 32 bits
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    // Should sign-extend to full 64-bit
    assert_eq!(regs.rax, 0xFFFFFFFF80000001, "RAX should be sign-extended");
}

#[test]
fn test_movsxd_array_index_conversion() {
    // Simulates converting an array index from 32-bit to 64-bit
    let code = [
        0x48, 0x63, 0xcb, // MOVSXD RCX, EBX (convert index to 64-bit)
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x00000100; // Index 256
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rcx, 0x0000000000000100, "RCX should be 256 as 64-bit");
}

#[test]
fn test_movsxd_signed_offset_conversion() {
    // Simulates converting a signed offset from 32-bit to 64-bit
    let code = [
        0x48, 0x63, 0xc8, // MOVSXD RCX, EAX (convert offset to 64-bit)
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0xFFFFFFC8; // -56 as 32-bit signed
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rcx, 0xFFFFFFFFFFFFFFC8,
        "RCX should be -56 as 64-bit signed"
    );
}

#[test]
fn test_movsxd_upper_bits_preserved_in_source() {
    // When source is 32-bit operand, only lower 32 bits matter
    let code = [0x48, 0x63, 0xc3, 0xf4]; // MOVSXD RAX, EBX
    let mut regs = Registers::default();
    regs.rbx = 0xCAFEBABE_DEADBEEF; // Upper 32 bits are ignored, only EBX (0xDEADBEEF) used
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    // Should sign-extend from 0xDEADBEEF
    assert_eq!(
        regs.rax, 0xFFFFFFFFDEADBEEF,
        "Should use lower 32 bits only"
    );
}

#[test]
fn test_movsxd_chained_conversions() {
    let code = [
        0x48, 0x63, 0xc3, // MOVSXD RAX, EBX
        0x48, 0x63, 0xc8, // MOVSXD RCX, EAX (now using RAX value)
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x80000000; // Negative
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    // Both should have same value
    assert_eq!(regs.rax, 0xFFFFFFFF80000000, "RAX should be -2147483648");
    assert_eq!(regs.rcx, 0xFFFFFFFF80000000, "RCX should be same as RAX");
}

#[test]
fn test_movsxd_across_all_registers() {
    // Test with multiple registers
    let code = [
        0x48, 0x63, 0xc3, // MOVSXD RAX, EBX
        0x48, 0x63, 0xca, // MOVSXD RCX, EDX
        0x48, 0x63, 0xd1, // MOVSXD RDX, ECX
        0x48, 0x63, 0xfe, // MOVSXD RDI, ESI
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x00000001; // Positive
    regs.rdx = 0x80000000; // Negative
    regs.rsi = 0xFFFFFFFF; // -1
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x0000000000000001, "RAX should be 1");
    assert_eq!(regs.rcx, 0xFFFFFFFF80000000, "RCX should be -2147483648");
    assert_eq!(regs.rdi, 0xFFFFFFFFFFFFFFFF, "RDI should be -1");
}
