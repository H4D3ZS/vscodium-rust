use rax::cpu::Registers;

use crate::common::{run_until_hlt, setup_vm};

// CMOVAE/CMOVNB/CMOVNC - Conditional Move if Above or Equal/Not Below/Not Carry
// Moves source to destination if CF=0 (unsigned comparison >= or no carry)

// Basic CMOVAE when CF=0
#[test]
fn test_cmovae_eax_ebx_cf_clear() {
    let code = [
        0x48, 0xc7, 0xc0, 0x05, 0x00, 0x00, 0x00, // MOV RAX, 5
        0x48, 0xc7, 0xc1, 0x03, 0x00, 0x00, 0x00, // MOV RCX, 3
        0x48, 0x39, 0xc8, // CMP RAX, RCX (5 >= 3, CF=0)
        0x0f, 0x43, 0xc3, // CMOVAE EAX, EBX
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
        "EAX should be moved when 5 >= 3"
    );
}

// CMOVAE when values are equal (CF=0, ZF=1)
#[test]
fn test_cmovae_eax_ebx_equal() {
    let code = [
        0x48, 0xc7, 0xc0, 0x05, 0x00, 0x00, 0x00, // MOV RAX, 5
        0x48, 0xc7, 0xc1, 0x05, 0x00, 0x00, 0x00, // MOV RCX, 5
        0x48, 0x39, 0xc8, // CMP RAX, RCX (5 == 5, CF=0, ZF=1)
        0x0f, 0x43, 0xc3, // CMOVAE EAX, EBX
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
        "EAX should be moved when 5 == 5"
    );
}

// CMOVAE when CF=1 (below, should not move)
#[test]
fn test_cmovae_eax_ebx_cf_set() {
    let code = [
        0x48, 0xc7, 0xc0, 0x03, 0x00, 0x00, 0x00, // MOV RAX, 3
        0x48, 0xc7, 0xc1, 0x05, 0x00, 0x00, 0x00, // MOV RCX, 5
        0x48, 0x39, 0xc8, // CMP RAX, RCX (3 < 5, CF=1)
        0x0f, 0x43, 0xc3, // CMOVAE EAX, EBX
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x11111111;
    regs.rbx = 0x22222222;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x00000003,
        "EAX should not be moved when 3 < 5"
    );
}

// CMOVNB (same as CMOVAE)
#[test]
fn test_cmovnb_edx_ecx() {
    let code = [
        0x48, 0xc7, 0xc0, 0x0a, 0x00, 0x00, 0x00, // MOV RAX, 10
        0x48, 0xc7, 0xc1, 0x05, 0x00, 0x00, 0x00, // MOV RCX, 5
        0x48, 0x39, 0xc8, // CMP RAX, RCX (10 >= 5)
        0x0f, 0x43, 0xd1, // CMOVNB EDX, ECX
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rdx = 0x33333333;
    regs.rcx = 0x44444444;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rdx & 0xFFFFFFFF,
        0x00000005,
        "EDX should be moved when not below"
    );
}

// CMOVNC (same as CMOVAE) - "not carry"
#[test]
fn test_cmovnc_edx_ecx() {
    let code = [
        0xf8, // CLC (clear carry flag)
        0x0f, 0x43, 0xd1, // CMOVNC EDX, ECX
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rdx = 0x33333333;
    regs.rcx = 0x44444444;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rdx & 0xFFFFFFFF,
        0x44444444,
        "EDX should be moved when CF=0"
    );
}

#[test]
fn test_cmovnc_no_move_when_cf_set() {
    let code = [
        0xf9, // STC (set carry flag)
        0x0f, 0x43, 0xd1, // CMOVNC EDX, ECX
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rdx = 0x33333333;
    regs.rcx = 0x44444444;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rdx & 0xFFFFFFFF,
        0x33333333,
        "EDX should not be moved when CF=1"
    );
}

// Test 16-bit operand
#[test]
fn test_cmovae_ax_bx() {
    let code = [
        0x48, 0xc7, 0xc0, 0x10, 0x00, 0x00, 0x00, // MOV RAX, 16
        0x48, 0xc7, 0xc1, 0x08, 0x00, 0x00, 0x00, // MOV RCX, 8
        0x48, 0x39, 0xc8, // CMP RAX, RCX
        0x66, 0x0f, 0x43, 0xc3, // CMOVAE AX, BX
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
fn test_cmovae_rax_rbx() {
    let code = [
        0x48, 0xc7, 0xc2, 0x64, 0x00, 0x00, 0x00, // MOV RDX, 100
        0x48, 0xc7, 0xc1, 0x32, 0x00, 0x00, 0x00, // MOV RCX, 50
        0x48, 0x39, 0xca, // CMP RDX, RCX
        0x48, 0x0f, 0x43, 0xc3, // CMOVAE RAX, RBX
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
fn test_cmovae_r8d_r9d() {
    let code = [
        0x48, 0xc7, 0xc0, 0xff, 0x00, 0x00, 0x00, // MOV RAX, 255
        0x48, 0xc7, 0xc1, 0x01, 0x00, 0x00, 0x00, // MOV RCX, 1
        0x48, 0x39, 0xc8, // CMP RAX, RCX
        0x45, 0x0f, 0x43, 0xc1, // CMOVAE R8D, R9D
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.r8 = 0x11111111;
    regs.r9 = 0x22222222;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.r8 & 0xFFFFFFFF, 0x22222222, "R8D should be moved");
}

// Test unsigned comparison semantics
#[test]
fn test_cmovae_unsigned_comparison() {
    let code = [
        0x48, 0xc7, 0xc0, 0xff, 0xff, 0xff, 0xff, // MOV RAX, 0xFFFFFFFF
        0x48, 0xc7, 0xc1, 0x01, 0x00, 0x00, 0x00, // MOV RCX, 1
        0x48, 0x39, 0xc8, // CMP RAX, RCX (unsigned: 0xFFFFFFFF >= 1)
        0x0f, 0x43, 0xc3, // CMOVAE EAX, EBX
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
        "EAX should be moved (unsigned 0xFFFFFFFF >= 1)"
    );
}

// Test with 0 comparison
#[test]
fn test_cmovae_greater_or_equal_zero() {
    let code = [
        0x48, 0xc7, 0xc0, 0x00, 0x00, 0x00, 0x00, // MOV RAX, 0
        0x48, 0xc7, 0xc1, 0x00, 0x00, 0x00, 0x00, // MOV RCX, 0
        0x48, 0x39, 0xc8, // CMP RAX, RCX (0 >= 0)
        0x0f, 0x43, 0xc3, // CMOVAE EAX, EBX
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
        "EAX should be moved when 0 >= 0"
    );
}

// Test that flags are preserved
#[test]
fn test_cmovae_preserves_flags() {
    let code = [
        0x48, 0xc7, 0xc0, 0x05, 0x00, 0x00, 0x00, // MOV RAX, 5
        0x48, 0xc7, 0xc1, 0x05, 0x00, 0x00, 0x00, // MOV RCX, 5
        0x48, 0x39, 0xc8, // CMP RAX, RCX
        0x0f, 0x43, 0xc3, // CMOVAE EAX, EBX
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x11111111;
    regs.rbx = 0x22222222;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert!(regs.rflags & 0x01 == 0, "CF should be 0");
    assert!(regs.rflags & 0x40 != 0, "ZF should be 1");
}

// Test 32-bit operation zeros upper bits
#[test]
fn test_cmovae_zeros_upper_32() {
    let code = [
        0x48, 0xc7, 0xc2, 0x0a, 0x00, 0x00, 0x00, // MOV RDX, 10
        0x48, 0xc7, 0xc1, 0x05, 0x00, 0x00, 0x00, // MOV RCX, 5
        0x48, 0x39, 0xca, // CMP RDX, RCX
        0x0f, 0x43, 0xc3, // CMOVAE EAX, EBX
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

// Test practical use case: check if no overflow in addition
#[test]
fn test_cmovae_practical_no_overflow() {
    // Check if addition would overflow using ADC with 0
    let code = [
        0x48, 0xc7, 0xc0, 0x0a, 0x00, 0x00, 0x00, // MOV RAX, 10
        0x48, 0xc7, 0xc1, 0x05, 0x00, 0x00, 0x00, // MOV RCX, 5
        0x48, 0x01, 0xc8, // ADD RAX, RCX (no overflow, CF=0)
        0x0f, 0x43, 0xd3, // CMOVAE EDX, EBX
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rdx = 0x11111111;
    regs.rbx = 0x22222222;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rdx & 0xFFFFFFFF,
        0x22222222,
        "EDX should be moved (no carry)"
    );
}

// Test with subtraction that doesn't borrow
#[test]
fn test_cmovae_after_sub_no_borrow() {
    let code = [
        0x48, 0xc7, 0xc0, 0x0a, 0x00, 0x00, 0x00, // MOV RAX, 10
        0x48, 0xc7, 0xc1, 0x05, 0x00, 0x00, 0x00, // MOV RCX, 5
        0x48, 0x29, 0xc8, // SUB RAX, RCX (10 - 5, no borrow, CF=0)
        0x0f, 0x43, 0xd3, // CMOVAE EDX, EBX
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rdx = 0x33333333;
    regs.rbx = 0x44444444;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rdx & 0xFFFFFFFF, 0x44444444, "EDX should be moved");
}

#[test]
fn test_cmovae_after_sub_with_borrow() {
    let code = [
        0x48, 0xc7, 0xc0, 0x05, 0x00, 0x00, 0x00, // MOV RAX, 5
        0x48, 0xc7, 0xc1, 0x0a, 0x00, 0x00, 0x00, // MOV RCX, 10
        0x48, 0x29, 0xc8, // SUB RAX, RCX (5 - 10, borrow, CF=1)
        0x0f, 0x43, 0xd3, // CMOVAE EDX, EBX
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rdx = 0x33333333;
    regs.rbx = 0x44444444;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rdx & 0xFFFFFFFF,
        0x33333333,
        "EDX should not be moved (CF=1)"
    );
}

// Test edge case: comparing with itself
#[test]
fn test_cmovae_self_comparison() {
    let code = [
        0x48, 0x39, 0xc0, // CMP RAX, RAX (always equal, CF=0)
        0x0f, 0x43, 0xc3, // CMOVAE EAX, EBX
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
        "EAX should be moved (equal is >=)"
    );
}

// Test different register combinations
#[test]
fn test_cmovae_esi_edi() {
    let code = [
        0x48, 0xc7, 0xc0, 0x14, 0x00, 0x00, 0x00, // MOV RAX, 20
        0x48, 0xc7, 0xc1, 0x14, 0x00, 0x00, 0x00, // MOV RCX, 20
        0x48, 0x39, 0xc8, // CMP RAX, RCX (20 >= 20)
        0x0f, 0x43, 0xf7, // CMOVAE ESI, EDI
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsi = 0x55555555;
    regs.rdi = 0x66666666;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rsi & 0xFFFFFFFF, 0x66666666, "ESI should be moved");
}

// Test maximum unsigned values
#[test]
fn test_cmovae_max_unsigned() {
    let code = [
        0x48, 0xc7, 0xc0, 0xff, 0xff, 0xff, 0xff, // MOV RAX, 0xFFFFFFFF
        0x48, 0xc7, 0xc1, 0xff, 0xff, 0xff, 0xff, // MOV RCX, 0xFFFFFFFF
        0x48, 0x39, 0xc8, // CMP RAX, RCX (0xFFFFFFFF >= 0xFFFFFFFF)
        0x0f, 0x43, 0xc3, // CMOVAE EAX, EBX
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x11111111;
    regs.rbx = 0x22222222;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFFFFFFFF, 0x22222222, "EAX should be moved");
}

// Test practical use case: saturating subtraction
#[test]
fn test_cmovae_practical_saturating_sub() {
    // result = (a >= b) ? (a - b) : 0
    let code = [
        0x48, 0x89, 0xc2, // MOV RDX, RAX (save original)
        0x48, 0x39, 0xd8, // CMP RAX, RBX
        0x48, 0x29, 0xd8, // SUB RAX, RBX
        0x48, 0x31, 0xc9, // XOR RCX, RCX (RCX = 0)
        0x48, 0x0f, 0x43, 0xc1, // CMOVAE RAX, RCX (if no borrow, keep result, else use 0)
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 10;
    regs.rbx = 5;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0, "Should be 0 (CMOVAE moves 0 when CF=0)");
}

// Test with TEST instruction
#[test]
fn test_cmovae_after_test() {
    let code = [
        0x48, 0xc7, 0xc0, 0xff, 0x00, 0x00, 0x00, // MOV RAX, 0xFF
        0x48, 0x85, 0xc0, // TEST RAX, RAX (doesn't affect CF, CF=0)
        0x0f, 0x43, 0xc3, // CMOVAE EAX, EBX
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
        "EAX should be moved (TEST clears CF)"
    );
}

// Test chaining operations
#[test]
fn test_cmovae_chain() {
    let code = [
        0xf8, // CLC (CF=0)
        0x0f, 0x43, 0xc3, // CMOVAE EAX, EBX (should move)
        0xf9, // STC (CF=1)
        0x0f, 0x43, 0xd1, // CMOVAE EDX, ECX (should not move)
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x11111111;
    regs.rbx = 0x22222222;
    regs.rdx = 0x33333333;
    regs.rcx = 0x44444444;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFFFFFFFF, 0x22222222, "EAX should be moved");
    assert_eq!(regs.rdx & 0xFFFFFFFF, 0x33333333, "EDX should not be moved");
}
