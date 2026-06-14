use rax::cpu::Registers;

use crate::common::{run_until_hlt, setup_vm};

// CMOVE/CMOVZ - Conditional Move if Equal/Zero
// Moves source to destination if ZF=1

// Basic CMOVE when ZF is set
#[test]
fn test_cmove_eax_ebx_zf_set() {
    let code = [
        0x48, 0xc7, 0xc0, 0xff, 0xff, 0xff, 0xff, // MOV RAX, -1
        0x48, 0x83, 0xc0, 0x01, // ADD RAX, 1 (sets ZF)
        0x0f, 0x44, 0xc3, // CMOVE EAX, EBX
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
        "EAX should be moved from EBX when ZF=1"
    );
}

// CMOVE when ZF is clear (should not move)
#[test]
fn test_cmove_eax_ebx_zf_clear() {
    let code = [
        0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1
        0x48, 0x83, 0xc0, 0x01, // ADD RAX, 1 (clears ZF, result=2)
        0x0f, 0x44, 0xc3, // CMOVE EAX, EBX
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x11111111;
    regs.rbx = 0x22222222;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x00000002,
        "EAX should not be moved when ZF=0"
    );
}

// CMOVZ (same as CMOVE) when ZF is set
#[test]
fn test_cmovz_edx_ecx_zf_set() {
    let code = [
        0x48, 0x31, 0xc0, // XOR RAX, RAX (sets ZF)
        0x0f, 0x44, 0xd1, // CMOVZ EDX, ECX
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
        "EDX should be moved from ECX when ZF=1"
    );
}

// Test 16-bit operand
#[test]
fn test_cmove_ax_bx_zf_set() {
    let code = [
        0x48, 0x31, 0xc0, // XOR RAX, RAX (sets ZF)
        0x66, 0x0f, 0x44, 0xc3, // CMOVE AX, BX
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
        "AX should be moved from BX when ZF=1"
    );
}

// Test 64-bit operand
#[test]
fn test_cmove_rax_rbx_zf_set() {
    let code = [
        0x48, 0x31, 0xc9, // XOR RCX, RCX (sets ZF)
        0x48, 0x0f, 0x44, 0xc3, // CMOVE RAX, RBX
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x1111111111111111;
    regs.rbx = 0x2222222222222222;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax, 0x2222222222222222,
        "RAX should be moved from RBX when ZF=1"
    );
}

// Test with extended registers
#[test]
fn test_cmove_r8d_r9d_zf_set() {
    let code = [
        0x48, 0x31, 0xc0, // XOR RAX, RAX (sets ZF)
        0x45, 0x0f, 0x44, 0xc1, // CMOVE R8D, R9D
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
        "R8D should be moved from R9D when ZF=1"
    );
}

#[test]
fn test_cmove_rax_r10_zf_set() {
    let code = [
        0x48, 0x31, 0xc0, // XOR RAX, RAX (sets ZF)
        0x4c, 0x89, 0xd0, // MOV RAX, R10
        0x48, 0x31, 0xc0, // XOR RAX, RAX (sets ZF again)
        0x49, 0x0f, 0x44, 0xc2, // CMOVE RAX, R10
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x1111111111111111;
    regs.r10 = 0x3333333333333333;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax, 0x3333333333333333,
        "RAX should be moved from R10 when ZF=1"
    );
}

// Test that flags are preserved (CMOV doesn't modify flags)
#[test]
fn test_cmove_preserves_flags() {
    let code = [
        0x48, 0xc7, 0xc0, 0xff, 0xff, 0xff, 0xff, // MOV RAX, -1 (0xFFFFFFFFFFFFFFFF)
        0x48, 0x83, 0xc0, 0x01, // ADD RAX, 1 (sets ZF=1, CF=1 due to wrap-around)
        0x0f, 0x44, 0xc3, // CMOVE EAX, EBX (ZF=1, so move happens)
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x11111111;
    regs.rbx = 0x22222222;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    // CMOV doesn't modify flags
    assert!(regs.rflags & 0x40 != 0, "ZF should still be set");
    assert!(
        regs.rflags & 0x01 != 0,
        "CF should be set (wraparound from -1+1)"
    );
}

// Test different register combinations
#[test]
fn test_cmove_esi_edi_zf_set() {
    let code = [
        0x48, 0x31, 0xc0, // XOR RAX, RAX (sets ZF)
        0x0f, 0x44, 0xf7, // CMOVE ESI, EDI
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
        "ESI should be moved from EDI when ZF=1"
    );
}

// Test moving zero value
#[test]
fn test_cmove_with_zero_source() {
    let code = [
        0x48, 0x31, 0xc0, // XOR RAX, RAX (sets ZF)
        0x0f, 0x44, 0xc3, // CMOVE EAX, EBX
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
fn test_cmove_with_all_ones_source() {
    let code = [
        0x48, 0x31, 0xc0, // XOR RAX, RAX (sets ZF)
        0x0f, 0x44, 0xc3, // CMOVE EAX, EBX
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
fn test_cmove_eax_ebx_zeros_upper_32() {
    let code = [
        0x48, 0x31, 0xc0, // XOR RAX, RAX (sets ZF)
        0x0f, 0x44, 0xc3, // CMOVE EAX, EBX
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
fn test_cmove_chain() {
    let code = [
        0x48, 0x31, 0xc0, // XOR RAX, RAX (sets ZF, RAX=0)
        0x0f, 0x44, 0xc3, // CMOVE EAX, EBX (ZF=1, so move: EAX = EBX = 0)
        0x48, 0x83, 0xc0, 0x01, // ADD RAX, 1 (RAX=1, clears ZF)
        0x0f, 0x44, 0xd1, // CMOVE EDX, ECX (ZF=0, no move)
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x11111111;
    regs.rbx = 0x00000000; // Source for first CMOVE is 0
    regs.rdx = 0x33333333;
    regs.rcx = 0x44444444;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x00000001,
        "EAX should be 0+1 (was moved then incremented)"
    );
    assert_eq!(regs.rdx & 0xFFFFFFFF, 0x33333333, "EDX should not change");
}

// Test practical use case: conditional assignment
#[test]
fn test_cmove_practical_conditional_assignment() {
    // if (x == 0) result = value;
    let code = [
        0x48, 0x85, 0xc0, // TEST RAX, RAX (sets ZF if RAX==0)
        0x0f, 0x44, 0xd3, // CMOVE EDX, EBX
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
        0x22222222,
        "result should be updated"
    );
}

#[test]
fn test_cmove_practical_conditional_assignment_no_move() {
    // if (x == 0) result = value;
    let code = [
        0x48, 0x85, 0xc0, // TEST RAX, RAX (sets ZF if RAX==0)
        0x0f, 0x44, 0xd3, // CMOVE EDX, EBX
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
        0x11111111,
        "result should stay the same"
    );
}

// Test with all registers to verify encoding
#[test]
fn test_cmove_all_gp_registers() {
    // Test moving between different register pairs
    let code = [
        0x48, 0x31, 0xc0, // XOR RAX, RAX (sets ZF)
        0x0f, 0x44, 0xfb, // CMOVE EDI, EBX
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
fn test_cmove_same_register() {
    // XOR RAX, RAX sets RAX to 0 and sets ZF=1
    // CMOVE EAX, EAX: move EAX to EAX (EAX is 0 after XOR)
    // The 32-bit operation zeros upper bits
    let code = [
        0x48, 0x31, 0xc0, // XOR RAX, RAX (sets ZF, RAX=0)
        0x0f, 0x44, 0xc0, // CMOVE EAX, EAX (EAX=0 stays 0)
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x12345678;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax, 0x0000000000000000,
        "RAX should be 0 (XOR then CMOVE same reg)"
    );
}
