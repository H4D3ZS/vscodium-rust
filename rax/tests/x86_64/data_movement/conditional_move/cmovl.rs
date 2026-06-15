use rax::cpu::Registers;

use crate::common::{run_until_hlt, setup_vm};

// CMOVL/CMOVNGE - Conditional Move if Less/Not Greater or Equal
// Moves source to destination if SF!=OF (signed comparison <)

// Basic CMOVL when condition is true (less than)
#[test]
fn test_cmovl_eax_ebx_less() {
    let code = [
        0x48, 0xc7, 0xc0, 0x05, 0x00, 0x00, 0x00, // MOV RAX, 5
        0x48, 0xc7, 0xc1, 0x0a, 0x00, 0x00, 0x00, // MOV RCX, 10
        0x48, 0x39, 0xc8, // CMP RAX, RCX (5 < 10, signed)
        0x0f, 0x4c, 0xc3, // CMOVL EAX, EBX
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
        "EAX should be moved when 5 < 10"
    );
}

// CMOVL when values are equal (should not move)
#[test]
fn test_cmovl_eax_ebx_equal() {
    let code = [
        0x48, 0xc7, 0xc0, 0x05, 0x00, 0x00, 0x00, // MOV RAX, 5
        0x48, 0xc7, 0xc1, 0x05, 0x00, 0x00, 0x00, // MOV RCX, 5
        0x48, 0x39, 0xc8, // CMP RAX, RCX (5 == 5)
        0x0f, 0x4c, 0xc3, // CMOVL EAX, EBX
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

// CMOVL when greater (should not move)
#[test]
fn test_cmovl_eax_ebx_greater() {
    let code = [
        0x48, 0xc7, 0xc0, 0x0a, 0x00, 0x00, 0x00, // MOV RAX, 10
        0x48, 0xc7, 0xc1, 0x05, 0x00, 0x00, 0x00, // MOV RCX, 5
        0x48, 0x39, 0xc8, // CMP RAX, RCX (10 > 5)
        0x0f, 0x4c, 0xc3, // CMOVL EAX, EBX
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x11111111;
    regs.rbx = 0x22222222;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x0000000a,
        "EAX should not be moved when 10 > 5"
    );
}

// CMOVNGE (same as CMOVL)
#[test]
fn test_cmovnge_edx_ecx() {
    let code = [
        0x48, 0xc7, 0xc0, 0x0a, 0x00, 0x00, 0x00, // MOV RAX, 10
        0x48, 0xc7, 0xc1, 0x14, 0x00, 0x00, 0x00, // MOV RCX, 20
        0x48, 0x39, 0xc8, // CMP RAX, RCX (10 < 20)
        0x0f, 0x4c, 0xd1, // CMOVNGE EDX, ECX
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rdx = 0x33333333;
    regs.rcx = 0x44444444;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rdx & 0xFFFFFFFF,
        0x00000014,
        "EDX should be moved when not greater or equal"
    );
}

// Test signed comparison: negative < positive
#[test]
fn test_cmovl_negative_vs_positive() {
    let code = [
        0x48, 0xc7, 0xc0, 0xff, 0xff, 0xff, 0xff, // MOV RAX, -1
        0x48, 0xc7, 0xc1, 0x01, 0x00, 0x00, 0x00, // MOV RCX, 1
        0x48, 0x39, 0xc8, // CMP RAX, RCX (-1 < 1, signed)
        0x0f, 0x4c, 0xc3, // CMOVL EAX, EBX
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
        "EAX should be moved (-1 < 1 signed)"
    );
}

// Test signed comparison: negative < negative (more negative)
#[test]
fn test_cmovl_negative_vs_negative() {
    let code = [
        0x48, 0xc7, 0xc0, 0xfe, 0xff, 0xff, 0xff, // MOV RAX, -2
        0x48, 0xc7, 0xc1, 0xff, 0xff, 0xff, 0xff, // MOV RCX, -1
        0x48, 0x39, 0xc8, // CMP RAX, RCX (-2 < -1, signed)
        0x0f, 0x4c, 0xc3, // CMOVL EAX, EBX
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
        "EAX should be moved (-2 < -1 signed)"
    );
}

// Test 16-bit operand
#[test]
fn test_cmovl_ax_bx() {
    let code = [
        0x48, 0xc7, 0xc0, 0x08, 0x00, 0x00, 0x00, // MOV RAX, 8
        0x48, 0xc7, 0xc1, 0x10, 0x00, 0x00, 0x00, // MOV RCX, 16
        0x48, 0x39, 0xc8, // CMP RAX, RCX
        0x66, 0x0f, 0x4c, 0xc3, // CMOVL AX, BX
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
fn test_cmovl_rax_rbx() {
    let code = [
        0x48, 0xc7, 0xc2, 0x32, 0x00, 0x00, 0x00, // MOV RDX, 50
        0x48, 0xc7, 0xc1, 0x64, 0x00, 0x00, 0x00, // MOV RCX, 100
        0x48, 0x39, 0xca, // CMP RDX, RCX
        0x48, 0x0f, 0x4c, 0xc3, // CMOVL RAX, RBX
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
fn test_cmovl_r8d_r9d() {
    let code = [
        0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1
        0x48, 0xc7, 0xc1, 0xff, 0x00, 0x00, 0x00, // MOV RCX, 255
        0x48, 0x39, 0xc8, // CMP RAX, RCX
        0x45, 0x0f, 0x4c, 0xc1, // CMOVL R8D, R9D
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
fn test_cmovl_less_than_zero() {
    let code = [
        0x48, 0xc7, 0xc0, 0xff, 0xff, 0xff, 0xff, // MOV RAX, -1
        0x48, 0xc7, 0xc1, 0x00, 0x00, 0x00, 0x00, // MOV RCX, 0
        0x48, 0x39, 0xc8, // CMP RAX, RCX (-1 < 0)
        0x0f, 0x4c, 0xc3, // CMOVL EAX, EBX
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
        "EAX should be moved when -1 < 0"
    );
}

#[test]
fn test_cmovl_positive_vs_zero() {
    let code = [
        0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1
        0x48, 0xc7, 0xc1, 0x00, 0x00, 0x00, 0x00, // MOV RCX, 0
        0x48, 0x39, 0xc8, // CMP RAX, RCX (1 > 0)
        0x0f, 0x4c, 0xc3, // CMOVL EAX, EBX
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x11111111;
    regs.rbx = 0x22222222;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x00000001,
        "EAX should not be moved (1 >= 0)"
    );
}

// Test that flags are preserved
#[test]
fn test_cmovl_preserves_flags() {
    let code = [
        0x48, 0xc7, 0xc0, 0x05, 0x00, 0x00, 0x00, // MOV RAX, 5
        0x48, 0xc7, 0xc1, 0x0a, 0x00, 0x00, 0x00, // MOV RCX, 10
        0x48, 0x39, 0xc8, // CMP RAX, RCX
        0x0f, 0x4c, 0xc3, // CMOVL EAX, EBX
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x11111111;
    regs.rbx = 0x22222222;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    // Flags should be preserved from CMP
}

// Test 32-bit operation zeros upper bits
#[test]
fn test_cmovl_zeros_upper_32() {
    let code = [
        0x48, 0xc7, 0xc2, 0x05, 0x00, 0x00, 0x00, // MOV RDX, 5
        0x48, 0xc7, 0xc1, 0x0a, 0x00, 0x00, 0x00, // MOV RCX, 10
        0x48, 0x39, 0xca, // CMP RDX, RCX
        0x0f, 0x4c, 0xc3, // CMOVL EAX, EBX
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

// Test practical use case: min of two signed values
#[test]
fn test_cmovl_practical_signed_min() {
    // min = (a > b) ? b : a - use CMOVG to move b to a if a > b
    let code = [
        0x48, 0x39, 0xd8, // CMP RAX, RBX
        0x48, 0x0f, 0x4f, 0xc3, // CMOVG RAX, RBX (move RBX to RAX if RAX > RBX)
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 50;
    regs.rbx = 100;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    // 50 < 100, so RAX stays 50 (no move happens)
    assert_eq!(regs.rax, 50, "RAX should remain 50 (min of 50 and 100)");
}

#[test]
fn test_cmovl_practical_signed_min_with_negative() {
    // min = (a > b) ? b : a - use CMOVG to move b to a if a > b
    let code = [
        0x48, 0x39, 0xd8, // CMP RAX, RBX
        0x48, 0x0f, 0x4f, 0xc3, // CMOVG RAX, RBX (move RBX to RAX if RAX > RBX)
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0xFFFFFFFFFFFFFFF5; // -11
    regs.rbx = 0xFFFFFFFFFFFFFFF0; // -16
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    // -11 > -16, so RAX gets RBX (-16)
    assert_eq!(
        regs.rax, 0xFFFFFFFFFFFFFFF0,
        "RAX should be -16 (min of -11 and -16)"
    );
}

// Test edge case: comparing with itself
#[test]
fn test_cmovl_self_comparison() {
    let code = [
        0x48, 0x39, 0xc0, // CMP RAX, RAX (always equal)
        0x0f, 0x4c, 0xc3, // CMOVL EAX, EBX
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
        "EAX should not change (not less, equal)"
    );
}

// Test different register combinations
#[test]
fn test_cmovl_esi_edi() {
    let code = [
        0x48, 0xc7, 0xc0, 0x0a, 0x00, 0x00, 0x00, // MOV RAX, 10
        0x48, 0xc7, 0xc1, 0x14, 0x00, 0x00, 0x00, // MOV RCX, 20
        0x48, 0x39, 0xc8, // CMP RAX, RCX
        0x0f, 0x4c, 0xf7, // CMOVL ESI, EDI
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
fn test_cmovl_signed_vs_unsigned_semantics() {
    // 0x80000000 is large unsigned, but MIN_INT signed (most negative)
    let code = [
        0x48, 0xc7, 0xc0, 0x00, 0x00, 0x00, 0x80, // MOV RAX, 0x80000000 (MIN_INT)
        0x48, 0xc7, 0xc1, 0x01, 0x00, 0x00, 0x00, // MOV RCX, 1
        0x48, 0x39, 0xc8, // CMP RAX, RCX (signed: MIN_INT < 1)
        0x0f, 0x4c, 0xc3, // CMOVL EAX, EBX
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
        "EAX should move (signed comparison)"
    );
}

// Test minimum signed values
#[test]
fn test_cmovl_min_signed_negative() {
    let code = [
        0x48, 0xc7, 0xc0, 0x00, 0x00, 0x00, 0x80, // MOV RAX, 0x80000000 (MIN_INT)
        0x48, 0xc7, 0xc1, 0x01, 0x00, 0x00, 0x80, // MOV RCX, 0x80000001
        0x48, 0x39, 0xc8, // CMP RAX, RCX (MIN_INT < MIN_INT+1)
        0x0f, 0x4c, 0xc3, // CMOVL EAX, EBX
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
fn test_cmovl_after_sub_negative_result() {
    let code = [
        0x48, 0xc7, 0xc0, 0x05, 0x00, 0x00, 0x00, // MOV RAX, 5
        0x48, 0xc7, 0xc1, 0x0a, 0x00, 0x00, 0x00, // MOV RCX, 10
        0x48, 0x29, 0xc8, // SUB RAX, RCX (5 - 10 = -5, negative result)
        0x0f, 0x4c, 0xd3, // CMOVL EDX, EBX
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
        "EDX should be moved (result < 0)"
    );
}

// Test chaining operations
#[test]
fn test_cmovl_chain() {
    let code = [
        0x48, 0xc7, 0xc0, 0x05, 0x00, 0x00, 0x00, // MOV RAX, 5
        0x48, 0xc7, 0xc1, 0x0a, 0x00, 0x00, 0x00, // MOV RCX, 10
        0x48, 0x39, 0xc8, // CMP RAX, RCX (5 < 10)
        0x0f, 0x4c, 0xc3, // CMOVL EAX, EBX (should move)
        0x48, 0xc7, 0xc2, 0x0a, 0x00, 0x00, 0x00, // MOV RDX, 10
        0x48, 0xc7, 0xc1, 0x05, 0x00, 0x00, 0x00, // MOV RCX, 5
        0x48, 0x39, 0xca, // CMP RDX, RCX (10 > 5)
        0x0f, 0x4c, 0xf7, // CMOVL ESI, EDI (should not move)
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

// Test boundary values
#[test]
fn test_cmovl_boundary_max_vs_min() {
    let code = [
        0x48, 0xc7, 0xc0, 0x00, 0x00, 0x00, 0x80, // MOV RAX, 0x80000000 (MIN_INT)
        0x48, 0xc7, 0xc1, 0xff, 0xff, 0xff, 0x7f, // MOV RCX, 0x7FFFFFFF (MAX_INT)
        0x48, 0x39, 0xc8, // CMP RAX, RCX (MIN_INT < MAX_INT)
        0x0f, 0x4c, 0xc3, // CMOVL EAX, EBX
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x11111111;
    regs.rbx = 0x22222222;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFFFFFFFF, 0x22222222, "EAX should be moved");
}
