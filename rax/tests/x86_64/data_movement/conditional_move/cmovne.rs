use rax::cpu::Registers;

use crate::common::{run_until_hlt, setup_vm};

// CMOVNE/CMOVNZ - Conditional Move if Not Equal/Not Zero
// Moves source to destination if ZF=0

// Basic CMOVNE when ZF is clear
#[test]
fn test_cmovne_eax_ebx_zf_clear() {
    let code = [
        0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1
        0x48, 0x83, 0xc0, 0x01, // ADD RAX, 1 (clears ZF, result=2)
        0x0f, 0x45, 0xc3, // CMOVNE EAX, EBX
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
        "EAX should be moved from EBX when ZF=0"
    );
}

// CMOVNE when ZF is set (should not move)
#[test]
fn test_cmovne_eax_ebx_zf_set() {
    let code = [
        0x48, 0xc7, 0xc0, 0xff, 0xff, 0xff, 0xff, // MOV RAX, -1
        0x48, 0x83, 0xc0, 0x01, // ADD RAX, 1 (sets ZF)
        0x0f, 0x45, 0xc3, // CMOVNE EAX, EBX
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
        "EAX should not be moved when ZF=1"
    );
}

// CMOVNZ (same as CMOVNE) when ZF is clear
#[test]
fn test_cmovnz_edx_ecx_zf_clear() {
    let code = [
        0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1
        0x48, 0x85, 0xc0, // TEST RAX, RAX (clears ZF since RAX != 0)
        0x0f, 0x45, 0xd1, // CMOVNZ EDX, ECX
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
        "EDX should be moved from ECX when ZF=0"
    );
}

// Test 16-bit operand
#[test]
fn test_cmovne_ax_bx_zf_clear() {
    let code = [
        0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1
        0x48, 0x85, 0xc0, // TEST RAX, RAX (clears ZF)
        0x66, 0x0f, 0x45, 0xc3, // CMOVNE AX, BX
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x1111;
    regs.rbx = 0x2222;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax & 0xFFFF,
        0x2222,
        "AX should be moved from BX when ZF=0"
    );
}

// Test 64-bit operand
#[test]
fn test_cmovne_rax_rbx_zf_clear() {
    let code = [
        0x48, 0xc7, 0xc1, 0x01, 0x00, 0x00, 0x00, // MOV RCX, 1
        0x48, 0x85, 0xc9, // TEST RCX, RCX (clears ZF)
        0x48, 0x0f, 0x45, 0xc3, // CMOVNE RAX, RBX
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x1111111111111111;
    regs.rbx = 0x2222222222222222;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax, 0x2222222222222222,
        "RAX should be moved from RBX when ZF=0"
    );
}

// Test with extended registers
#[test]
fn test_cmovne_r8d_r9d_zf_clear() {
    let code = [
        0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1
        0x48, 0x85, 0xc0, // TEST RAX, RAX (clears ZF)
        0x45, 0x0f, 0x45, 0xc1, // CMOVNE R8D, R9D
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.r8 = 0x11111111;
    regs.r9 = 0x22222222;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.r8 & 0xFFFFFFFF,
        0x22222222,
        "R8D should be moved from R9D when ZF=0"
    );
}

#[test]
fn test_cmovne_rax_r10_zf_clear() {
    let code = [
        0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1
        0x48, 0x85, 0xc0, // TEST RAX, RAX (clears ZF)
        0x49, 0x0f, 0x45, 0xc2, // CMOVNE RAX, R10
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x1111111111111111;
    regs.r10 = 0x3333333333333333;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax, 0x3333333333333333,
        "RAX should be moved from R10 when ZF=0"
    );
}

// Test that flags are preserved
#[test]
fn test_cmovne_preserves_flags() {
    let code = [
        0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1
        0x48, 0x83, 0xc0, 0x01, // ADD RAX, 1 (clears ZF, sets other flags)
        0x0f, 0x45, 0xc3, // CMOVNE EAX, EBX
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x11111111;
    regs.rbx = 0x22222222;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert!(regs.rflags & 0x40 == 0, "ZF should still be clear");
}

// Test different register combinations
#[test]
fn test_cmovne_esi_edi_zf_clear() {
    let code = [
        0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1
        0x48, 0x85, 0xc0, // TEST RAX, RAX (clears ZF)
        0x0f, 0x45, 0xf7, // CMOVNE ESI, EDI
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsi = 0x55555555;
    regs.rdi = 0x66666666;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rsi & 0xFFFFFFFF,
        0x66666666,
        "ESI should be moved from EDI when ZF=0"
    );
}

// Test moving zero value
#[test]
fn test_cmovne_with_zero_source() {
    let code = [
        0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1
        0x48, 0x85, 0xc0, // TEST RAX, RAX (clears ZF)
        0x0f, 0x45, 0xc3, // CMOVNE EAX, EBX
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x11111111;
    regs.rbx = 0x00000000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFFFFFFFF, 0x00000000, "EAX should be set to 0");
}

// Test moving all 1s
#[test]
fn test_cmovne_with_all_ones_source() {
    let code = [
        0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1
        0x48, 0x85, 0xc0, // TEST RAX, RAX (clears ZF)
        0x0f, 0x45, 0xc3, // CMOVNE EAX, EBX
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x00000000;
    regs.rbx = 0xFFFFFFFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFFFFFFFF, 0xFFFFFFFF, "EAX should be all 1s");
}

// Test that 32-bit operation zeros upper 32 bits
#[test]
fn test_cmovne_eax_ebx_zeros_upper_32() {
    let code = [
        0x48, 0xc7, 0xc1, 0x01, 0x00, 0x00, 0x00, // MOV RCX, 1
        0x48, 0x85, 0xc9, // TEST RCX, RCX (clears ZF)
        0x0f, 0x45, 0xc3, // CMOVNE EAX, EBX
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

// Test chaining multiple conditional moves
#[test]
fn test_cmovne_chain() {
    let code = [
        0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1
        0x48, 0x85, 0xc0, // TEST RAX, RAX (clears ZF)
        0x0f, 0x45, 0xc3, // CMOVNE EAX, EBX (should move)
        0x48, 0x31, 0xc0, // XOR RAX, RAX (sets ZF)
        0x0f, 0x45, 0xd1, // CMOVNE EDX, ECX (should not move)
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x11111111;
    regs.rbx = 0x22222222;
    regs.rdx = 0x33333333;
    regs.rcx = 0x44444444;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFFFFFFFF, 0x00000000, "RAX should be 0 (XOR)");
    assert_eq!(regs.rdx & 0xFFFFFFFF, 0x33333333, "EDX should not change");
}

// Test practical use case: conditional assignment
#[test]
fn test_cmovne_practical_conditional_assignment() {
    // if (x != 0) result = value;
    let code = [
        0x48, 0x85, 0xc0, // TEST RAX, RAX (sets ZF if RAX==0)
        0x0f, 0x45, 0xd3, // CMOVNE EDX, EBX
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x00000001; // x != 0
    regs.rdx = 0x11111111; // result (old value)
    regs.rbx = 0x22222222; // value (new value)
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rdx & 0xFFFFFFFF,
        0x22222222,
        "result should be updated"
    );
}

#[test]
fn test_cmovne_practical_conditional_assignment_no_move() {
    // if (x != 0) result = value;
    let code = [
        0x48, 0x85, 0xc0, // TEST RAX, RAX (sets ZF if RAX==0)
        0x0f, 0x45, 0xd3, // CMOVNE EDX, EBX
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x00000000; // x == 0
    regs.rdx = 0x11111111; // result (old value)
    regs.rbx = 0x22222222; // value (new value)
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rdx & 0xFFFFFFFF,
        0x11111111,
        "result should stay the same"
    );
}

// Test with comparison instruction
#[test]
fn test_cmovne_after_cmp_not_equal() {
    let code = [
        0x48, 0x39, 0xd8, // CMP RAX, RBX
        0x0f, 0x45, 0xd1, // CMOVNE EDX, ECX
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x11111111;
    regs.rbx = 0x22222222; // Different values, ZF=0
    regs.rdx = 0x33333333;
    regs.rcx = 0x44444444;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rdx & 0xFFFFFFFF,
        0x44444444,
        "EDX should be moved when RAX != RBX"
    );
}

#[test]
fn test_cmovne_after_cmp_equal() {
    let code = [
        0x48, 0x39, 0xd8, // CMP RAX, RBX
        0x0f, 0x45, 0xd1, // CMOVNE EDX, ECX
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x11111111;
    regs.rbx = 0x11111111; // Same values, ZF=1
    regs.rdx = 0x33333333;
    regs.rcx = 0x44444444;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rdx & 0xFFFFFFFF,
        0x33333333,
        "EDX should not be moved when RAX == RBX"
    );
}

// Test with all registers to verify encoding
#[test]
fn test_cmovne_all_gp_registers() {
    let code = [
        0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1
        0x48, 0x85, 0xc0, // TEST RAX, RAX (clears ZF)
        0x0f, 0x45, 0xfb, // CMOVNE EDI, EBX
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rdi = 0xAAAAAAAA;
    regs.rbx = 0xBBBBBBBB;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rdi & 0xFFFFFFFF,
        0xBBBBBBBB,
        "EDI should be moved from EBX"
    );
}

// Test edge case: source and destination are the same register
#[test]
fn test_cmovne_same_register() {
    let code = [
        0x48, 0xc7, 0xc1, 0x01, 0x00, 0x00, 0x00, // MOV RCX, 1
        0x48, 0x85, 0xc9, // TEST RCX, RCX (clears ZF)
        0x0f, 0x45, 0xc0, // CMOVNE EAX, EAX
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x12345678;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax, 0x0000000012345678,
        "RAX upper bits should be zeroed even for same-reg move"
    );
}

// Test alternating CMOVE/CMOVNE usage
#[test]
fn test_cmovne_alternating_with_cmove() {
    let code = [
        0x48, 0x31, 0xc0, // XOR RAX, RAX (sets ZF)
        0x0f, 0x44, 0xc3, // CMOVE EAX, EBX (should move)
        0x48, 0xc7, 0xc1, 0x01, 0x00, 0x00, 0x00, // MOV RCX, 1
        0x48, 0x85, 0xc9, // TEST RCX, RCX (clears ZF)
        0x0f, 0x45, 0xd6, // CMOVNE EDX, ESI (should move)
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x11111111;
    regs.rbx = 0x22222222;
    regs.rdx = 0x33333333;
    regs.rsi = 0x44444444;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x22222222,
        "EAX should be moved by CMOVE"
    );
    assert_eq!(
        regs.rdx & 0xFFFFFFFF,
        0x44444444,
        "EDX should be moved by CMOVNE"
    );
}
