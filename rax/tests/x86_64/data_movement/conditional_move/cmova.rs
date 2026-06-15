use rax::cpu::Registers;

use crate::common::{run_until_hlt, setup_vm};

// CMOVA/CMOVNBE - Conditional Move if Above/Not Below or Equal
// Moves source to destination if CF=0 and ZF=0 (unsigned comparison)

// Basic CMOVA when CF=0 and ZF=0
#[test]
fn test_cmova_eax_ebx_cf_clear_zf_clear() {
    let code = [
        0x48, 0xc7, 0xc0, 0x05, 0x00, 0x00, 0x00, // MOV RAX, 5
        0x48, 0xc7, 0xc1, 0x03, 0x00, 0x00, 0x00, // MOV RCX, 3
        0x48, 0x39, 0xc8, // CMP RAX, RCX (5 > 3, CF=0, ZF=0)
        0x0f, 0x47, 0xc3, // CMOVA EAX, EBX
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
        "EAX should be moved when 5 > 3"
    );
}

// CMOVA when CF=1 (below, should not move)
#[test]
fn test_cmova_eax_ebx_cf_set() {
    let code = [
        0x48, 0xc7, 0xc0, 0x03, 0x00, 0x00, 0x00, // MOV RAX, 3
        0x48, 0xc7, 0xc1, 0x05, 0x00, 0x00, 0x00, // MOV RCX, 5
        0x48, 0x39, 0xc8, // CMP RAX, RCX (3 < 5, CF=1)
        0x0f, 0x47, 0xc3, // CMOVA EAX, EBX
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

// CMOVA when ZF=1 (equal, should not move)
#[test]
fn test_cmova_eax_ebx_zf_set() {
    let code = [
        0x48, 0xc7, 0xc0, 0x05, 0x00, 0x00, 0x00, // MOV RAX, 5
        0x48, 0xc7, 0xc1, 0x05, 0x00, 0x00, 0x00, // MOV RCX, 5
        0x48, 0x39, 0xc8, // CMP RAX, RCX (5 == 5, ZF=1)
        0x0f, 0x47, 0xc3, // CMOVA EAX, EBX
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
        "EAX should not be moved when 5 == 5"
    );
}

// CMOVNBE (same as CMOVA)
#[test]
fn test_cmovnbe_edx_ecx() {
    let code = [
        0x48, 0xc7, 0xc0, 0x0a, 0x00, 0x00, 0x00, // MOV RAX, 10
        0x48, 0xc7, 0xc1, 0x05, 0x00, 0x00, 0x00, // MOV RCX, 5
        0x48, 0x39, 0xc8, // CMP RAX, RCX (10 > 5)
        0x0f, 0x47, 0xd1, // CMOVNBE EDX, ECX
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
        "EDX should be moved when not below or equal"
    );
}

// Test 16-bit operand
#[test]
fn test_cmova_ax_bx() {
    let code = [
        0x48, 0xc7, 0xc0, 0x10, 0x00, 0x00, 0x00, // MOV RAX, 16
        0x48, 0xc7, 0xc1, 0x08, 0x00, 0x00, 0x00, // MOV RCX, 8
        0x48, 0x39, 0xc8, // CMP RAX, RCX
        0x66, 0x0f, 0x47, 0xc3, // CMOVA AX, BX
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
fn test_cmova_rax_rbx() {
    let code = [
        0x48, 0xc7, 0xc2, 0x64, 0x00, 0x00, 0x00, // MOV RDX, 100
        0x48, 0xc7, 0xc1, 0x32, 0x00, 0x00, 0x00, // MOV RCX, 50
        0x48, 0x39, 0xca, // CMP RDX, RCX
        0x48, 0x0f, 0x47, 0xc3, // CMOVA RAX, RBX
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
fn test_cmova_r8d_r9d() {
    let code = [
        0x48, 0xc7, 0xc0, 0xff, 0x00, 0x00, 0x00, // MOV RAX, 255
        0x48, 0xc7, 0xc1, 0x01, 0x00, 0x00, 0x00, // MOV RCX, 1
        0x48, 0x39, 0xc8, // CMP RAX, RCX
        0x45, 0x0f, 0x47, 0xc1, // CMOVA R8D, R9D
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
fn test_cmova_unsigned_comparison() {
    let code = [
        0x48, 0xc7, 0xc0, 0xff, 0xff, 0xff, 0xff, // MOV RAX, 0xFFFFFFFF
        0x48, 0xc7, 0xc1, 0x01, 0x00, 0x00, 0x00, // MOV RCX, 1
        0x48, 0x39, 0xc8, // CMP RAX, RCX (unsigned: 0xFFFFFFFF > 1)
        0x0f, 0x47, 0xc3, // CMOVA EAX, EBX
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
        "EAX should be moved (unsigned 0xFFFFFFFF > 1)"
    );
}

// Test with 0 comparison
#[test]
fn test_cmova_greater_than_zero() {
    let code = [
        0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1
        0x48, 0xc7, 0xc1, 0x00, 0x00, 0x00, 0x00, // MOV RCX, 0
        0x48, 0x39, 0xc8, // CMP RAX, RCX (1 > 0)
        0x0f, 0x47, 0xc3, // CMOVA EAX, EBX
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

// Test maximum unsigned values
#[test]
fn test_cmova_max_unsigned() {
    let code = [
        0x48, 0xc7, 0xc0, 0xff, 0xff, 0xff, 0xff, // MOV RAX, 0xFFFFFFFF
        0x48, 0xc7, 0xc1, 0xfe, 0xff, 0xff, 0xff, // MOV RCX, 0xFFFFFFFE
        0x48, 0x39, 0xc8, // CMP RAX, RCX (0xFFFFFFFF > 0xFFFFFFFE)
        0x0f, 0x47, 0xc3, // CMOVA EAX, EBX
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x11111111;
    regs.rbx = 0x22222222;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFFFFFFFF, 0x22222222, "EAX should be moved");
}

// Test that flags are preserved
#[test]
fn test_cmova_preserves_flags() {
    let code = [
        0x48, 0xc7, 0xc0, 0x05, 0x00, 0x00, 0x00, // MOV RAX, 5
        0x48, 0xc7, 0xc1, 0x03, 0x00, 0x00, 0x00, // MOV RCX, 3
        0x48, 0x39, 0xc8, // CMP RAX, RCX
        0x0f, 0x47, 0xc3, // CMOVA EAX, EBX
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x11111111;
    regs.rbx = 0x22222222;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert!(regs.rflags & 0x01 == 0, "CF should be 0");
    assert!(regs.rflags & 0x40 == 0, "ZF should be 0");
}

// Test 32-bit operation zeros upper bits
#[test]
fn test_cmova_zeros_upper_32() {
    let code = [
        0x48, 0xc7, 0xc2, 0x0a, 0x00, 0x00, 0x00, // MOV RDX, 10
        0x48, 0xc7, 0xc1, 0x05, 0x00, 0x00, 0x00, // MOV RCX, 5
        0x48, 0x39, 0xca, // CMP RDX, RCX
        0x0f, 0x47, 0xc3, // CMOVA EAX, EBX
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

// Test practical use case: max of two unsigned values
// For max(a,b): if a < b, move b to a (use CMOVB)
#[test]
fn test_cmova_practical_max() {
    let code = [
        0x48, 0x39, 0xd8, // CMP RAX, RBX
        0x48, 0x0f, 0x42, 0xc3, // CMOVB RAX, RBX (if RAX < RBX, move RBX to RAX)
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 100;
    regs.rbx = 50;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    // 100 > 50, so no move (RAX stays 100)
    assert_eq!(regs.rax, 100, "RAX should remain 100 (max of 100 and 50)");
}

#[test]
fn test_cmova_practical_max_swap() {
    let code = [
        0x48, 0x39, 0xd8, // CMP RAX, RBX
        0x48, 0x0f, 0x42, 0xc3, // CMOVB RAX, RBX (if RAX < RBX, move RBX to RAX)
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 50;
    regs.rbx = 100;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    // 50 < 100, so move RBX to RAX
    assert_eq!(regs.rax, 100, "RAX should be 100 (max of 50 and 100)");
}

// Test edge case: comparing with itself
#[test]
fn test_cmova_self_comparison() {
    let code = [
        0x48, 0x39, 0xc0, // CMP RAX, RAX (always equal, ZF=1)
        0x0f, 0x47, 0xc3, // CMOVA EAX, EBX
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
        "EAX should not change (equal comparison)"
    );
}

// Test different register combinations
#[test]
fn test_cmova_esi_edi() {
    let code = [
        0x48, 0xc7, 0xc0, 0x14, 0x00, 0x00, 0x00, // MOV RAX, 20
        0x48, 0xc7, 0xc1, 0x0a, 0x00, 0x00, 0x00, // MOV RCX, 10
        0x48, 0x39, 0xc8, // CMP RAX, RCX
        0x0f, 0x47, 0xf7, // CMOVA ESI, EDI
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsi = 0x55555555;
    regs.rdi = 0x66666666;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rsi & 0xFFFFFFFF, 0x66666666, "ESI should be moved");
}

// Test boundary: just above
#[test]
fn test_cmova_boundary_just_above() {
    let code = [
        0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1
        0x48, 0xc7, 0xc1, 0x00, 0x00, 0x00, 0x00, // MOV RCX, 0
        0x48, 0x39, 0xc8, // CMP RAX, RCX (1 > 0 by 1)
        0x0f, 0x47, 0xc3, // CMOVA EAX, EBX
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x11111111;
    regs.rbx = 0x22222222;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFFFFFFFF, 0x22222222, "EAX should be moved");
}

// Test with SUB instruction setting flags
#[test]
fn test_cmova_after_sub() {
    let code = [
        0x48, 0xc7, 0xc0, 0x0a, 0x00, 0x00, 0x00, // MOV RAX, 10
        0x48, 0xc7, 0xc1, 0x05, 0x00, 0x00, 0x00, // MOV RCX, 5
        0x48, 0x29, 0xc8, // SUB RAX, RCX (10 - 5 = 5, no borrow, CF=0, ZF=0)
        0x0f, 0x47, 0xd3, // CMOVA EDX, EBX
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
fn test_cmova_after_sub_borrow() {
    let code = [
        0x48, 0xc7, 0xc0, 0x05, 0x00, 0x00, 0x00, // MOV RAX, 5
        0x48, 0xc7, 0xc1, 0x0a, 0x00, 0x00, 0x00, // MOV RCX, 10
        0x48, 0x29, 0xc8, // SUB RAX, RCX (5 - 10, borrow, CF=1)
        0x0f, 0x47, 0xd3, // CMOVA EDX, EBX
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
