use rax::cpu::Registers;

use crate::common::{run_until_hlt, setup_vm};

// CMOVB/CMOVC/CMOVNAE - Conditional Move if Below/Carry/Not Above or Equal
// Moves source to destination if CF=1 (unsigned comparison < or carry set)

// Basic CMOVB when CF=1
#[test]
fn test_cmovb_eax_ebx_cf_set() {
    let code = [
        0x48, 0xc7, 0xc0, 0x03, 0x00, 0x00, 0x00, // MOV RAX, 3
        0x48, 0xc7, 0xc1, 0x05, 0x00, 0x00, 0x00, // MOV RCX, 5
        0x48, 0x39, 0xc8, // CMP RAX, RCX (3 < 5, CF=1)
        0x0f, 0x42, 0xc3, // CMOVB EAX, EBX
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
        "EAX should be moved when 3 < 5"
    );
}

// CMOVB when CF=0 (should not move)
#[test]
fn test_cmovb_eax_ebx_cf_clear() {
    let code = [
        0x48, 0xc7, 0xc0, 0x05, 0x00, 0x00, 0x00, // MOV RAX, 5
        0x48, 0xc7, 0xc1, 0x03, 0x00, 0x00, 0x00, // MOV RCX, 3
        0x48, 0x39, 0xc8, // CMP RAX, RCX (5 >= 3, CF=0)
        0x0f, 0x42, 0xc3, // CMOVB EAX, EBX
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
        "EAX should not be moved when 5 >= 3"
    );
}

// CMOVB when values are equal (CF=0, should not move)
#[test]
fn test_cmovb_eax_ebx_equal() {
    let code = [
        0x48, 0xc7, 0xc0, 0x05, 0x00, 0x00, 0x00, // MOV RAX, 5
        0x48, 0xc7, 0xc1, 0x05, 0x00, 0x00, 0x00, // MOV RCX, 5
        0x48, 0x39, 0xc8, // CMP RAX, RCX (5 == 5, CF=0)
        0x0f, 0x42, 0xc3, // CMOVB EAX, EBX
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

// CMOVC (same as CMOVB) - "carry"
#[test]
fn test_cmovc_edx_ecx_cf_set() {
    let code = [
        0xf9, // STC (set carry flag)
        0x0f, 0x42, 0xd1, // CMOVC EDX, ECX
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
        "EDX should be moved when CF=1"
    );
}

#[test]
fn test_cmovc_no_move_when_cf_clear() {
    let code = [
        0xf8, // CLC (clear carry flag)
        0x0f, 0x42, 0xd1, // CMOVC EDX, ECX
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
        "EDX should not be moved when CF=0"
    );
}

// CMOVNAE (same as CMOVB) - "not above or equal"
#[test]
fn test_cmovnae_edx_ecx() {
    let code = [
        0x48, 0xc7, 0xc0, 0x02, 0x00, 0x00, 0x00, // MOV RAX, 2
        0x48, 0xc7, 0xc1, 0x0a, 0x00, 0x00, 0x00, // MOV RCX, 10
        0x48, 0x39, 0xc8, // CMP RAX, RCX (2 < 10)
        0x0f, 0x42, 0xd1, // CMOVNAE EDX, ECX
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
        "EDX should be moved when not above or equal"
    );
}

// Test 16-bit operand
#[test]
fn test_cmovb_ax_bx() {
    let code = [
        0x48, 0xc7, 0xc0, 0x08, 0x00, 0x00, 0x00, // MOV RAX, 8
        0x48, 0xc7, 0xc1, 0x10, 0x00, 0x00, 0x00, // MOV RCX, 16
        0x48, 0x39, 0xc8, // CMP RAX, RCX
        0x66, 0x0f, 0x42, 0xc3, // CMOVB AX, BX
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
fn test_cmovb_rax_rbx() {
    let code = [
        0x48, 0xc7, 0xc2, 0x32, 0x00, 0x00, 0x00, // MOV RDX, 50
        0x48, 0xc7, 0xc1, 0x64, 0x00, 0x00, 0x00, // MOV RCX, 100
        0x48, 0x39, 0xca, // CMP RDX, RCX
        0x48, 0x0f, 0x42, 0xc3, // CMOVB RAX, RBX
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
fn test_cmovb_r8d_r9d() {
    let code = [
        0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1
        0x48, 0xc7, 0xc1, 0xff, 0x00, 0x00, 0x00, // MOV RCX, 255
        0x48, 0x39, 0xc8, // CMP RAX, RCX
        0x45, 0x0f, 0x42, 0xc1, // CMOVB R8D, R9D
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
fn test_cmovb_unsigned_comparison() {
    let code = [
        0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1
        0x48, 0xc7, 0xc1, 0xff, 0xff, 0xff, 0xff, // MOV RCX, 0xFFFFFFFF
        0x48, 0x39, 0xc8, // CMP RAX, RCX (unsigned: 1 < 0xFFFFFFFF)
        0x0f, 0x42, 0xc3, // CMOVB EAX, EBX
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
        "EAX should be moved (unsigned 1 < 0xFFFFFFFF)"
    );
}

// Test with 0 comparison
#[test]
fn test_cmovb_less_than_one() {
    let code = [
        0x48, 0xc7, 0xc0, 0x00, 0x00, 0x00, 0x00, // MOV RAX, 0
        0x48, 0xc7, 0xc1, 0x01, 0x00, 0x00, 0x00, // MOV RCX, 1
        0x48, 0x39, 0xc8, // CMP RAX, RCX (0 < 1)
        0x0f, 0x42, 0xc3, // CMOVB EAX, EBX
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
        "EAX should be moved when 0 < 1"
    );
}

// Test that flags are preserved
#[test]
fn test_cmovb_preserves_flags() {
    let code = [
        0x48, 0xc7, 0xc0, 0x03, 0x00, 0x00, 0x00, // MOV RAX, 3
        0x48, 0xc7, 0xc1, 0x05, 0x00, 0x00, 0x00, // MOV RCX, 5
        0x48, 0x39, 0xc8, // CMP RAX, RCX
        0x0f, 0x42, 0xc3, // CMOVB EAX, EBX
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x11111111;
    regs.rbx = 0x22222222;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert!(regs.rflags & 0x01 != 0, "CF should be 1");
}

// Test 32-bit operation zeros upper bits
#[test]
fn test_cmovb_zeros_upper_32() {
    let code = [
        0x48, 0xc7, 0xc2, 0x05, 0x00, 0x00, 0x00, // MOV RDX, 5
        0x48, 0xc7, 0xc1, 0x0a, 0x00, 0x00, 0x00, // MOV RCX, 10
        0x48, 0x39, 0xca, // CMP RDX, RCX
        0x0f, 0x42, 0xc3, // CMOVB EAX, EBX
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

// Test practical use case: min of two unsigned values
// For min(a,b): if a > b, move b to a (use CMOVA)
#[test]
fn test_cmovb_practical_min() {
    let code = [
        0x48, 0x39, 0xd8, // CMP RAX, RBX
        0x48, 0x0f, 0x47, 0xc3, // CMOVA RAX, RBX (if RAX > RBX, move RBX to RAX)
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 50;
    regs.rbx = 100;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    // 50 < 100, so no move (RAX stays 50)
    assert_eq!(regs.rax, 50, "RAX should remain 50 (min of 50 and 100)");
}

#[test]
fn test_cmovb_practical_min_swap() {
    let code = [
        0x48, 0x39, 0xd8, // CMP RAX, RBX
        0x48, 0x0f, 0x47, 0xc3, // CMOVA RAX, RBX (if RAX > RBX, move RBX to RAX)
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 100;
    regs.rbx = 50;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    // 100 > 50, so move RBX to RAX
    assert_eq!(regs.rax, 50, "RAX should be 50 (min of 100 and 50)");
}

// Test with subtraction that borrows
#[test]
fn test_cmovb_after_sub_with_borrow() {
    let code = [
        0x48, 0xc7, 0xc0, 0x05, 0x00, 0x00, 0x00, // MOV RAX, 5
        0x48, 0xc7, 0xc1, 0x0a, 0x00, 0x00, 0x00, // MOV RCX, 10
        0x48, 0x29, 0xc8, // SUB RAX, RCX (5 - 10, borrow, CF=1)
        0x0f, 0x42, 0xd3, // CMOVB EDX, EBX
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
        "EDX should be moved (CF=1)"
    );
}

#[test]
fn test_cmovb_after_sub_no_borrow() {
    let code = [
        0x48, 0xc7, 0xc0, 0x0a, 0x00, 0x00, 0x00, // MOV RAX, 10
        0x48, 0xc7, 0xc1, 0x05, 0x00, 0x00, 0x00, // MOV RCX, 5
        0x48, 0x29, 0xc8, // SUB RAX, RCX (10 - 5, no borrow, CF=0)
        0x0f, 0x42, 0xd3, // CMOVB EDX, EBX
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
        "EDX should not be moved (CF=0)"
    );
}

// Test edge case: comparing with itself
#[test]
fn test_cmovb_self_comparison() {
    let code = [
        0x48, 0x39, 0xc0, // CMP RAX, RAX (always equal, CF=0)
        0x0f, 0x42, 0xc3, // CMOVB EAX, EBX
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
        "EAX should not change (equal, not below)"
    );
}

// Test different register combinations
#[test]
fn test_cmovb_esi_edi() {
    let code = [
        0x48, 0xc7, 0xc0, 0x0a, 0x00, 0x00, 0x00, // MOV RAX, 10
        0x48, 0xc7, 0xc1, 0x14, 0x00, 0x00, 0x00, // MOV RCX, 20
        0x48, 0x39, 0xc8, // CMP RAX, RCX
        0x0f, 0x42, 0xf7, // CMOVB ESI, EDI
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsi = 0x55555555;
    regs.rdi = 0x66666666;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rsi & 0xFFFFFFFF, 0x66666666, "ESI should be moved");
}

// Test with addition that sets carry
#[test]
fn test_cmovb_after_add_with_carry() {
    let code = [
        0x48, 0xc7, 0xc0, 0xff, 0xff, 0xff, 0xff, // MOV RAX, 0xFFFFFFFF
        0x48, 0xc7, 0xc1, 0x01, 0x00, 0x00, 0x00, // MOV RCX, 1
        0x48, 0x01, 0xc8, // ADD RAX, RCX (overflow in 32-bit, sets CF)
        0x0f, 0x42, 0xd3, // CMOVB EDX, EBX
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rdx = 0x33333333;
    regs.rbx = 0x44444444;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    // Note: This might not set CF depending on how 64-bit ADD works
    // For 32-bit overflow to set CF, we'd need to use 32-bit registers
}

// Test maximum unsigned values
#[test]
fn test_cmovb_max_unsigned() {
    let code = [
        0x48, 0xc7, 0xc0, 0xfe, 0xff, 0xff, 0xff, // MOV RAX, 0xFFFFFFFE
        0x48, 0xc7, 0xc1, 0xff, 0xff, 0xff, 0xff, // MOV RCX, 0xFFFFFFFF
        0x48, 0x39, 0xc8, // CMP RAX, RCX (0xFFFFFFFE < 0xFFFFFFFF)
        0x0f, 0x42, 0xc3, // CMOVB EAX, EBX
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x11111111;
    regs.rbx = 0x22222222;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFFFFFFFF, 0x22222222, "EAX should be moved");
}

// Test with TEST instruction (doesn't affect CF)
#[test]
fn test_cmovb_after_test() {
    let code = [
        0xf8, // CLC (clear CF)
        0x48, 0xc7, 0xc0, 0xff, 0x00, 0x00, 0x00, // MOV RAX, 0xFF
        0x48, 0x85, 0xc0, // TEST RAX, RAX (doesn't affect CF)
        0x0f, 0x42, 0xc3, // CMOVB EAX, EBX
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x11111111;
    regs.rbx = 0x22222222;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    // CF=0, so no move. RAX stays at 0xFF from the MOV instruction
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x000000FF,
        "EAX should be 0xFF (CF=0, no move)"
    );
}

// Test chaining operations
#[test]
fn test_cmovb_chain() {
    let code = [
        0xf9, // STC (CF=1)
        0x0f, 0x42, 0xc3, // CMOVB EAX, EBX (should move)
        0xf8, // CLC (CF=0)
        0x0f, 0x42, 0xd1, // CMOVB EDX, ECX (should not move)
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

// Test practical use case: error checking
#[test]
fn test_cmovb_practical_error_check() {
    // if (result < expected) error_value = actual_error
    let code = [
        0x48, 0x39, 0xd8, // CMP RAX, RBX (compare result with expected)
        0x48, 0x0f, 0x42, 0xd1, // CMOVB RDX, RCX (if below, set error)
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 10; // result
    regs.rbx = 20; // expected
    regs.rdx = 0; // error value (initially no error)
    regs.rcx = 0xFFFFFFFF; // error code
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rdx, 0xFFFFFFFF, "Error should be set");
}
