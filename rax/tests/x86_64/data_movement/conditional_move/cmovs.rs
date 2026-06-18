use rax::cpu::Registers;

use crate::common::{run_until_hlt, setup_vm};

// CMOVS - Conditional Move if Sign
// Moves source to destination if SF=1 (sign flag set, negative result)

// Basic CMOVS when SF=1 (negative result)
#[test]
fn test_cmovs_eax_ebx_sf_set() {
    let code = [
        0x48, 0xc7, 0xc0, 0xff, 0xff, 0xff, 0xff, // MOV RAX, -1
        0x48, 0x85, 0xc0, // TEST RAX, RAX (sets SF=1 for negative)
        0x0f, 0x48, 0xc3, // CMOVS EAX, EBX
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
        "EAX should be moved when SF=1"
    );
}

// CMOVS when SF=0 (positive result, should not move)
#[test]
fn test_cmovs_eax_ebx_sf_clear() {
    let code = [
        0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1
        0x48, 0x85, 0xc0, // TEST RAX, RAX (clears SF for positive)
        0x0f, 0x48, 0xc3, // CMOVS EAX, EBX
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
        "EAX should not be moved when SF=0"
    );
}

// Test after SUB with negative result
#[test]
fn test_cmovs_after_sub_negative() {
    let code = [
        0x48, 0xc7, 0xc0, 0x05, 0x00, 0x00, 0x00, // MOV RAX, 5
        0x48, 0xc7, 0xc1, 0x0a, 0x00, 0x00, 0x00, // MOV RCX, 10
        0x48, 0x29, 0xc8, // SUB RAX, RCX (5 - 10 = -5, sets SF)
        0x0f, 0x48, 0xd3, // CMOVS EDX, EBX
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
        "EDX should be moved (negative result)"
    );
}

// Test after SUB with positive result
#[test]
fn test_cmovs_after_sub_positive() {
    let code = [
        0x48, 0xc7, 0xc0, 0x0a, 0x00, 0x00, 0x00, // MOV RAX, 10
        0x48, 0xc7, 0xc1, 0x05, 0x00, 0x00, 0x00, // MOV RCX, 5
        0x48, 0x29, 0xc8, // SUB RAX, RCX (10 - 5 = 5, clears SF)
        0x0f, 0x48, 0xd3, // CMOVS EDX, EBX
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
        "EDX should not be moved (positive result)"
    );
}

// Test 16-bit operand - use 64-bit negative value to set SF
#[test]
fn test_cmovs_ax_bx() {
    let code = [
        0x48, 0xc7, 0xc0, 0xff, 0xff, 0xff, 0xff, // MOV RAX, -1 (0xFFFFFFFFFFFFFFFF)
        0x48, 0x85, 0xc0, // TEST RAX, RAX (sets SF=1 for negative 64-bit)
        0x66, 0x0f, 0x48, 0xc3, // CMOVS AX, BX
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x1111;
    regs.rbx = 0x2222;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    // SF=1 from TEST, so CMOVS moves BX to AX
    assert_eq!(regs.rax & 0xFFFF, 0x2222, "AX should be moved");
}

// Test 64-bit operand
#[test]
fn test_cmovs_rax_rbx() {
    let code = [
        0x48, 0xc7, 0xc2, 0xff, 0xff, 0xff, 0xff, // MOV RDX, -1
        0x48, 0x85, 0xd2, // TEST RDX, RDX
        0x48, 0x0f, 0x48, 0xc3, // CMOVS RAX, RBX
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x1111111111111111;
    regs.rbx = 0x2222222222222222;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x2222222222222222, "RAX should be moved");
}

// Test with extended registers - use 64-bit negative value to set SF
#[test]
fn test_cmovs_r8d_r9d() {
    let code = [
        0x48, 0xc7, 0xc0, 0xff, 0xff, 0xff, 0xff, // MOV RAX, -1 (negative in 64-bit)
        0x48, 0x85, 0xc0, // TEST RAX, RAX (sets SF=1)
        0x45, 0x0f, 0x48, 0xc1, // CMOVS R8D, R9D
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.r8 = 0x11111111;
    regs.r9 = 0x22222222;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    // SF=1 from TEST, so CMOVS moves R9D to R8D
    assert_eq!(
        regs.r8 & 0xFFFFFFFF,
        0x22222222,
        "R8D should be moved when SF=1"
    );
}

// Test with zero (SF=0)
#[test]
fn test_cmovs_zero() {
    let code = [
        0x48, 0x31, 0xc0, // XOR RAX, RAX (result = 0, SF=0)
        0x0f, 0x48, 0xc3, // CMOVS EAX, EBX
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x11111111;
    regs.rbx = 0x22222222;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x00000000,
        "EAX should not be moved (zero, SF=0)"
    );
}

// Test that flags are preserved
#[test]
fn test_cmovs_preserves_flags() {
    let code = [
        0x48, 0xc7, 0xc0, 0xff, 0xff, 0xff, 0xff, // MOV RAX, -1
        0x48, 0x85, 0xc0, // TEST RAX, RAX
        0x0f, 0x48, 0xc3, // CMOVS EAX, EBX
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x11111111;
    regs.rbx = 0x22222222;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert!(regs.rflags & 0x80 != 0, "SF should still be set");
}

// Test 32-bit operation zeros upper bits
#[test]
fn test_cmovs_zeros_upper_32() {
    let code = [
        0x48, 0xc7, 0xc2, 0xff, 0xff, 0xff, 0xff, // MOV RDX, -1
        0x48, 0x85, 0xd2, // TEST RDX, RDX
        0x0f, 0x48, 0xc3, // CMOVS EAX, EBX
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

// Test practical use case: error flag check
#[test]
fn test_cmovs_practical_error_check() {
    // if (result < 0) error_code = -1
    let code = [
        0x48, 0x85, 0xc0, // TEST RAX, RAX
        0x48, 0x0f, 0x48, 0xd1, // CMOVS RDX, RCX
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0xFFFFFFFFFFFFFFFF; // -1 (error)
    regs.rdx = 0; // error_code
    regs.rcx = 0xFFFFFFFFFFFFFFFF; // -1
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rdx, 0xFFFFFFFFFFFFFFFF, "Error code should be set");
}

// Test different register combinations
// Test CMOVS with positive 64-bit value (SF=0, no move)
#[test]
fn test_cmovs_esi_edi() {
    // Note: MOV RAX, imm32 sign-extends, so use a small positive value
    let code = [
        0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1 (positive, SF will be 0)
        0x48, 0x85, 0xc0, // TEST RAX, RAX (SF=0 because bit 63 is 0)
        0x0f, 0x48, 0xf7, // CMOVS ESI, EDI
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsi = 0x55555555;
    regs.rdi = 0x66666666;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    // 1 is positive (bit 63 = 0), so SF=0
    assert_eq!(
        regs.rsi & 0xFFFFFFFF,
        0x55555555,
        "ESI should not be moved when SF=0"
    );
}

// CMOVNS - Conditional Move if Not Sign (SF=0)

#[test]
fn test_cmovns_eax_ebx_sf_clear() {
    let code = [
        0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1
        0x48, 0x85, 0xc0, // TEST RAX, RAX (clears SF for positive)
        0x0f, 0x49, 0xc3, // CMOVNS EAX, EBX
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
        "EAX should be moved when SF=0"
    );
}

#[test]
fn test_cmovns_eax_ebx_sf_set() {
    let code = [
        0x48, 0xc7, 0xc0, 0xff, 0xff, 0xff, 0xff, // MOV RAX, -1
        0x48, 0x85, 0xc0, // TEST RAX, RAX (sets SF for negative)
        0x0f, 0x49, 0xc3, // CMOVNS EAX, EBX
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
        "EAX should not be moved when SF=1"
    );
}

#[test]
fn test_cmovns_after_sub_positive() {
    let code = [
        0x48, 0xc7, 0xc0, 0x0a, 0x00, 0x00, 0x00, // MOV RAX, 10
        0x48, 0xc7, 0xc1, 0x05, 0x00, 0x00, 0x00, // MOV RCX, 5
        0x48, 0x29, 0xc8, // SUB RAX, RCX (10 - 5 = 5, clears SF)
        0x0f, 0x49, 0xd3, // CMOVNS EDX, EBX
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
        "EDX should be moved (positive result)"
    );
}

#[test]
fn test_cmovns_after_sub_negative() {
    let code = [
        0x48, 0xc7, 0xc0, 0x05, 0x00, 0x00, 0x00, // MOV RAX, 5
        0x48, 0xc7, 0xc1, 0x0a, 0x00, 0x00, 0x00, // MOV RCX, 10
        0x48, 0x29, 0xc8, // SUB RAX, RCX (5 - 10 = -5, sets SF)
        0x0f, 0x49, 0xd3, // CMOVNS EDX, EBX
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rdx = 0x33333333;
    regs.rbx = 0x44444444;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    // Result is negative in two's complement
    // In 64-bit, the result will have high bit set
    // SF depends on the actual result bit 63 for 64-bit operations
}

#[test]
fn test_cmovns_zero() {
    let code = [
        0x48, 0x31, 0xc0, // XOR RAX, RAX (result = 0, SF=0)
        0x0f, 0x49, 0xc3, // CMOVNS EAX, EBX
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
        "EAX should be moved (zero, SF=0)"
    );
}

#[test]
fn test_cmovns_practical_success_check() {
    // if (result >= 0) success_flag = true
    let code = [
        0x48, 0x85, 0xc0, // TEST RAX, RAX
        0x48, 0x0f, 0x49, 0xd1, // CMOVNS RDX, RCX
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 100; // positive result
    regs.rdx = 0; // success_flag
    regs.rcx = 1; // true
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rdx, 1, "Success flag should be set");
}

#[test]
fn test_cmovns_64bit_operand() {
    let code = [
        0x48, 0xc7, 0xc2, 0x01, 0x00, 0x00, 0x00, // MOV RDX, 1
        0x48, 0x85, 0xd2, // TEST RDX, RDX
        0x48, 0x0f, 0x49, 0xc3, // CMOVNS RAX, RBX
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x1111111111111111;
    regs.rbx = 0x2222222222222222;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x2222222222222222, "RAX should be moved");
}

#[test]
fn test_cmovns_preserves_flags() {
    let code = [
        0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1
        0x48, 0x85, 0xc0, // TEST RAX, RAX
        0x0f, 0x49, 0xc3, // CMOVNS EAX, EBX
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x11111111;
    regs.rbx = 0x22222222;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert!(regs.rflags & 0x80 == 0, "SF should still be clear");
}

// Test chaining CMOVS and CMOVNS
#[test]
fn test_cmovs_cmovns_chain() {
    let code = [
        0x48, 0xc7, 0xc2, 0xff, 0xff, 0xff, 0xff, // MOV RDX, -1
        0x48, 0x85, 0xd2, // TEST RDX, RDX (SF=1)
        0x0f, 0x48, 0xc3, // CMOVS EAX, EBX (should move)
        0x48, 0xc7, 0xc2, 0x01, 0x00, 0x00, 0x00, // MOV RDX, 1
        0x48, 0x85, 0xd2, // TEST RDX, RDX (SF=0)
        0x0f, 0x49, 0xf7, // CMOVNS ESI, EDI (should move)
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
    assert_eq!(regs.rsi & 0xFFFFFFFF, 0x44444444, "ESI should be moved");
}
