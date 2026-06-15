use rax::cpu::Registers;

use crate::common::{run_until_hlt, setup_vm};

// CMOVG/CMOVNLE - Conditional Move if Greater/Not Less or Equal
// Moves source to destination if ZF=0 AND SF=OF (signed comparison >)

// Basic CMOVG when condition is true (positive > positive, smaller)
#[test]
fn test_cmovg_eax_ebx_positive_greater() {
    let code = [
        0x48, 0xc7, 0xc0, 0x0a, 0x00, 0x00, 0x00, // MOV RAX, 10
        0x48, 0xc7, 0xc1, 0x05, 0x00, 0x00, 0x00, // MOV RCX, 5
        0x48, 0x39, 0xc8, // CMP RAX, RCX (10 > 5, signed)
        0x0f, 0x4f, 0xc3, // CMOVG EAX, EBX
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x11111111;
    regs.rbx = 0x22222222;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x22222222,
        "EAX should be moved when 10 > 5"
    );
}

// CMOVG when values are equal (should not move)
#[test]
fn test_cmovg_eax_ebx_equal() {
    let code = [
        0x48, 0xc7, 0xc0, 0x05, 0x00, 0x00, 0x00, // MOV RAX, 5
        0x48, 0xc7, 0xc1, 0x05, 0x00, 0x00, 0x00, // MOV RCX, 5
        0x48, 0x39, 0xc8, // CMP RAX, RCX (5 == 5)
        0x0f, 0x4f, 0xc3, // CMOVG EAX, EBX
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x11111111;
    regs.rbx = 0x22222222;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x00000005,
        "EAX should not be moved when equal"
    );
}

// CMOVG when less (should not move)
#[test]
fn test_cmovg_eax_ebx_less() {
    let code = [
        0x48, 0xc7, 0xc0, 0x05, 0x00, 0x00, 0x00, // MOV RAX, 5
        0x48, 0xc7, 0xc1, 0x0a, 0x00, 0x00, 0x00, // MOV RCX, 10
        0x48, 0x39, 0xc8, // CMP RAX, RCX (5 < 10)
        0x0f, 0x4f, 0xc3, // CMOVG EAX, EBX
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x11111111;
    regs.rbx = 0x22222222;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x00000005,
        "EAX should not be moved when 5 < 10"
    );
}

// CMOVNLE (same as CMOVG)
#[test]
fn test_cmovnle_edx_ecx() {
    let code = [
        0x48, 0xc7, 0xc0, 0x14, 0x00, 0x00, 0x00, // MOV RAX, 20
        0x48, 0xc7, 0xc1, 0x0a, 0x00, 0x00, 0x00, // MOV RCX, 10
        0x48, 0x39, 0xc8, // CMP RAX, RCX (20 > 10)
        0x0f, 0x4f, 0xd1, // CMOVNLE EDX, ECX
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rdx = 0x33333333;
    regs.rcx = 0x44444444;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rdx & 0xFFFFFFFF,
        0x0000000a,
        "EDX should be moved when not less or equal"
    );
}

// Test signed comparison: positive > negative
#[test]
fn test_cmovg_positive_vs_negative() {
    let code = [
        0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1
        0x48, 0xc7, 0xc1, 0xff, 0xff, 0xff, 0xff, // MOV RCX, -1 (0xFFFFFFFF)
        0x48, 0x39, 0xc8, // CMP RAX, RCX (1 > -1, signed)
        0x0f, 0x4f, 0xc3, // CMOVG EAX, EBX
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x11111111;
    regs.rbx = 0x22222222;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x22222222,
        "EAX should be moved (1 > -1 signed)"
    );
}

// Test signed comparison: negative > negative (less negative)
#[test]
fn test_cmovg_negative_vs_negative() {
    let code = [
        0x48, 0xc7, 0xc0, 0xff, 0xff, 0xff, 0xff, // MOV RAX, -1
        0x48, 0xc7, 0xc1, 0xfe, 0xff, 0xff, 0xff, // MOV RCX, -2
        0x48, 0x39, 0xc8, // CMP RAX, RCX (-1 > -2, signed)
        0x0f, 0x4f, 0xc3, // CMOVG EAX, EBX
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x11111111;
    regs.rbx = 0x22222222;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x22222222,
        "EAX should be moved (-1 > -2 signed)"
    );
}

// Test 16-bit operand
#[test]
fn test_cmovg_ax_bx() {
    let code = [
        0x48, 0xc7, 0xc0, 0x10, 0x00, 0x00, 0x00, // MOV RAX, 16
        0x48, 0xc7, 0xc1, 0x08, 0x00, 0x00, 0x00, // MOV RCX, 8
        0x48, 0x39, 0xc8, // CMP RAX, RCX
        0x66, 0x0f, 0x4f, 0xc3, // CMOVG AX, BX
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x1111;
    regs.rbx = 0x2222;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFFFF, 0x2222, "AX should be moved");
}

// Test 64-bit operand
#[test]
fn test_cmovg_rax_rbx() {
    let code = [
        0x48, 0xc7, 0xc2, 0x64, 0x00, 0x00, 0x00, // MOV RDX, 100
        0x48, 0xc7, 0xc1, 0x32, 0x00, 0x00, 0x00, // MOV RCX, 50
        0x48, 0x39, 0xca, // CMP RDX, RCX
        0x48, 0x0f, 0x4f, 0xc3, // CMOVG RAX, RBX
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x1111111111111111;
    regs.rbx = 0x2222222222222222;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x2222222222222222, "RAX should be moved");
}

// Test with extended registers
#[test]
fn test_cmovg_r8d_r9d() {
    let code = [
        0x48, 0xc7, 0xc0, 0xff, 0x00, 0x00, 0x00, // MOV RAX, 255
        0x48, 0xc7, 0xc1, 0x01, 0x00, 0x00, 0x00, // MOV RCX, 1
        0x48, 0x39, 0xc8, // CMP RAX, RCX
        0x45, 0x0f, 0x4f, 0xc1, // CMOVG R8D, R9D
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.r8 = 0x11111111;
    regs.r9 = 0x22222222;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.r8 & 0xFFFFFFFF, 0x22222222, "R8D should be moved");
}

// Test with zero
#[test]
fn test_cmovg_greater_than_zero() {
    let code = [
        0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1
        0x48, 0xc7, 0xc1, 0x00, 0x00, 0x00, 0x00, // MOV RCX, 0
        0x48, 0x39, 0xc8, // CMP RAX, RCX (1 > 0)
        0x0f, 0x4f, 0xc3, // CMOVG EAX, EBX
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x11111111;
    regs.rbx = 0x22222222;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x22222222,
        "EAX should be moved when 1 > 0"
    );
}

#[test]
fn test_cmovg_negative_vs_zero() {
    let code = [
        0x48, 0xc7, 0xc0, 0xff, 0xff, 0xff, 0xff, // MOV RAX, -1
        0x48, 0xc7, 0xc1, 0x00, 0x00, 0x00, 0x00, // MOV RCX, 0
        0x48, 0x39, 0xc8, // CMP RAX, RCX (-1 < 0, signed)
        0x0f, 0x4f, 0xc3, // CMOVG EAX, EBX
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x11111111;
    regs.rbx = 0x22222222;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0xffffffff,
        "EAX should not be moved (-1 < 0 signed)"
    );
}

// Test that flags are preserved
#[test]
fn test_cmovg_preserves_flags() {
    let code = [
        0x48, 0xc7, 0xc0, 0x0a, 0x00, 0x00, 0x00, // MOV RAX, 10
        0x48, 0xc7, 0xc1, 0x05, 0x00, 0x00, 0x00, // MOV RCX, 5
        0x48, 0x39, 0xc8, // CMP RAX, RCX
        0x0f, 0x4f, 0xc3, // CMOVG EAX, EBX
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x11111111;
    regs.rbx = 0x22222222;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    // Flags should be preserved from CMP
    assert!(regs.rflags & 0x40 == 0, "ZF should be 0");
}

// Test 32-bit operation zeros upper bits
#[test]
fn test_cmovg_zeros_upper_32() {
    let code = [
        0x48, 0xc7, 0xc2, 0x0a, 0x00, 0x00, 0x00, // MOV RDX, 10
        0x48, 0xc7, 0xc1, 0x05, 0x00, 0x00, 0x00, // MOV RCX, 5
        0x48, 0x39, 0xca, // CMP RDX, RCX
        0x0f, 0x4f, 0xc3, // CMOVG EAX, EBX
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0xDEADBEEFDEADBEEF;
    regs.rbx = 0x12345678;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax, 0x0000000012345678,
        "Upper 32 bits should be zeroed"
    );
}

// Test practical use case: max of two signed values
#[test]
fn test_cmovg_practical_signed_max() {
    // max = (a < b) ? b : a - use CMOVL to move b to a if a < b
    let code = [
        0x48, 0x39, 0xd8, // CMP RAX, RBX
        0x48, 0x0f, 0x4c, 0xc3, // CMOVL RAX, RBX (move RBX to RAX if RAX < RBX)
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 100;
    regs.rbx = 50;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 100, "RAX should remain 100 (max of 100 and 50)");
}

#[test]
fn test_cmovg_practical_signed_max_with_negative() {
    // max = (a < b) ? b : a - use CMOVL to move b to a if a < b
    let code = [
        0x48, 0x39, 0xd8, // CMP RAX, RBX
        0x48, 0x0f, 0x4c, 0xc3, // CMOVL RAX, RBX (move RBX to RAX if RAX < RBX)
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0xFFFFFFFFFFFFFFF0; // -16
    regs.rbx = 0xFFFFFFFFFFFFFFF5; // -11
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    // -16 < -11, so RAX gets RBX (-11)
    assert_eq!(
        regs.rax, 0xFFFFFFFFFFFFFFF5,
        "RAX should be -11 (max of -16 and -11)"
    );
}

// Test edge case: comparing with itself
#[test]
fn test_cmovg_self_comparison() {
    let code = [
        0x48, 0x39, 0xc0, // CMP RAX, RAX (always equal)
        0x0f, 0x4f, 0xc3, // CMOVG EAX, EBX
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x11111111;
    regs.rbx = 0x22222222;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x11111111,
        "EAX should not change (not greater, equal)"
    );
}

// Test different register combinations
#[test]
fn test_cmovg_esi_edi() {
    let code = [
        0x48, 0xc7, 0xc0, 0x14, 0x00, 0x00, 0x00, // MOV RAX, 20
        0x48, 0xc7, 0xc1, 0x0a, 0x00, 0x00, 0x00, // MOV RCX, 10
        0x48, 0x39, 0xc8, // CMP RAX, RCX
        0x0f, 0x4f, 0xf7, // CMOVG ESI, EDI
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsi = 0x55555555;
    regs.rdi = 0x66666666;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rsi & 0xFFFFFFFF, 0x66666666, "ESI should be moved");
}

// Test signed vs unsigned difference
#[test]
fn test_cmovg_signed_vs_unsigned_semantics() {
    // 0x80000000 is large unsigned, but negative signed
    let code = [
        0x48, 0xc7, 0xc0, 0x00, 0x00, 0x00, 0x80, // MOV RAX, 0x80000000 (MIN_INT signed)
        0x48, 0xc7, 0xc1, 0x01, 0x00, 0x00, 0x00, // MOV RCX, 1
        0x48, 0x39, 0xc8, // CMP RAX, RCX (signed: 0x80000000 < 1)
        0x0f, 0x4f, 0xc3, // CMOVG EAX, EBX
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x11111111;
    regs.rbx = 0x22222222;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x80000000,
        "EAX should not move (signed comparison)"
    );
}

// Test maximum signed values
#[test]
fn test_cmovg_max_signed_positive() {
    let code = [
        0x48, 0xc7, 0xc0, 0xff, 0xff, 0xff, 0x7f, // MOV RAX, 0x7FFFFFFF (MAX_INT)
        0x48, 0xc7, 0xc1, 0xfe, 0xff, 0xff, 0x7f, // MOV RCX, 0x7FFFFFFE
        0x48, 0x39, 0xc8, // CMP RAX, RCX (0x7FFFFFFF > 0x7FFFFFFE)
        0x0f, 0x4f, 0xc3, // CMOVG EAX, EBX
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x11111111;
    regs.rbx = 0x22222222;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFFFFFFFF, 0x22222222, "EAX should be moved");
}

// Test with SUB instruction
#[test]
fn test_cmovg_after_sub() {
    let code = [
        0x48, 0xc7, 0xc0, 0x0a, 0x00, 0x00, 0x00, // MOV RAX, 10
        0x48, 0xc7, 0xc1, 0x05, 0x00, 0x00, 0x00, // MOV RCX, 5
        0x48, 0x29, 0xc8, // SUB RAX, RCX (10 - 5 = 5, positive result)
        0x0f, 0x4f, 0xd3, // CMOVG EDX, EBX
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rdx = 0x33333333;
    regs.rbx = 0x44444444;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rdx & 0xFFFFFFFF,
        0x44444444,
        "EDX should be moved (result > 0)"
    );
}

// Test chaining operations
#[test]
fn test_cmovg_chain() {
    let code = [
        0x48, 0xc7, 0xc0, 0x0a, 0x00, 0x00, 0x00, // MOV RAX, 10
        0x48, 0xc7, 0xc1, 0x05, 0x00, 0x00, 0x00, // MOV RCX, 5
        0x48, 0x39, 0xc8, // CMP RAX, RCX (10 > 5)
        0x0f, 0x4f, 0xc3, // CMOVG EAX, EBX (should move)
        0x48, 0xc7, 0xc2, 0x05, 0x00, 0x00, 0x00, // MOV RDX, 5
        0x48, 0xc7, 0xc1, 0x0a, 0x00, 0x00, 0x00, // MOV RCX, 10
        0x48, 0x39, 0xca, // CMP RDX, RCX (5 < 10)
        0x0f, 0x4f, 0xf7, // CMOVG ESI, EDI (should not move)
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x11111111;
    regs.rbx = 0x22222222;
    regs.rsi = 0x33333333;
    regs.rdi = 0x44444444;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFFFFFFFF, 0x22222222, "EAX should be moved");
    assert_eq!(regs.rsi & 0xFFFFFFFF, 0x33333333, "ESI should not be moved");
}
