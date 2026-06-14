use rax::cpu::Registers;

use crate::common::{run_until_hlt, setup_vm};

// CMOVBE/CMOVNA - Conditional Move if Below or Equal/Not Above
// Moves source to destination if CF=1 OR ZF=1 (unsigned comparison <=)

// Basic CMOVBE when CF=1 (below)
#[test]
fn test_cmovbe_eax_ebx_cf_set() {
    let code = [
        0x48, 0xc7, 0xc0, 0x03, 0x00, 0x00, 0x00, // MOV RAX, 3
        0x48, 0xc7, 0xc1, 0x05, 0x00, 0x00, 0x00, // MOV RCX, 5
        0x48, 0x39, 0xc8, // CMP RAX, RCX (3 < 5, CF=1)
        0x0f, 0x46, 0xc3, // CMOVBE EAX, EBX
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
        "EAX should be moved when 3 <= 5"
    );
}

// CMOVBE when ZF=1 (equal)
#[test]
fn test_cmovbe_eax_ebx_zf_set() {
    let code = [
        0x48, 0xc7, 0xc0, 0x05, 0x00, 0x00, 0x00, // MOV RAX, 5
        0x48, 0xc7, 0xc1, 0x05, 0x00, 0x00, 0x00, // MOV RCX, 5
        0x48, 0x39, 0xc8, // CMP RAX, RCX (5 == 5, ZF=1, CF=0)
        0x0f, 0x46, 0xc3, // CMOVBE EAX, EBX
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
        "EAX should be moved when 5 <= 5"
    );
}

// CMOVBE when CF=0 and ZF=0 (above, should not move)
#[test]
fn test_cmovbe_eax_ebx_above() {
    let code = [
        0x48, 0xc7, 0xc0, 0x05, 0x00, 0x00, 0x00, // MOV RAX, 5
        0x48, 0xc7, 0xc1, 0x03, 0x00, 0x00, 0x00, // MOV RCX, 3
        0x48, 0x39, 0xc8, // CMP RAX, RCX (5 > 3, CF=0, ZF=0)
        0x0f, 0x46, 0xc3, // CMOVBE EAX, EBX
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
        "EAX should not be moved when 5 > 3"
    );
}

// CMOVNA (same as CMOVBE)
#[test]
fn test_cmovna_edx_ecx_below() {
    let code = [
        0x48, 0xc7, 0xc0, 0x02, 0x00, 0x00, 0x00, // MOV RAX, 2
        0x48, 0xc7, 0xc1, 0x0a, 0x00, 0x00, 0x00, // MOV RCX, 10
        0x48, 0x39, 0xc8, // CMP RAX, RCX (2 <= 10)
        0x0f, 0x46, 0xd1, // CMOVNA EDX, ECX
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
        "EDX should be moved when not above"
    );
}

#[test]
fn test_cmovna_edx_ecx_equal() {
    let code = [
        0x48, 0xc7, 0xc0, 0x0a, 0x00, 0x00, 0x00, // MOV RAX, 10
        0x48, 0xc7, 0xc1, 0x0a, 0x00, 0x00, 0x00, // MOV RCX, 10
        0x48, 0x39, 0xc8, // CMP RAX, RCX (10 == 10)
        0x0f, 0x46, 0xd1, // CMOVNA EDX, ECX
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
        "EDX should be moved when not above"
    );
}

// Test 16-bit operand
#[test]
fn test_cmovbe_ax_bx() {
    let code = [
        0x48, 0xc7, 0xc0, 0x10, 0x00, 0x00, 0x00, // MOV RAX, 16
        0x48, 0xc7, 0xc1, 0x10, 0x00, 0x00, 0x00, // MOV RCX, 16
        0x48, 0x39, 0xc8, // CMP RAX, RCX
        0x66, 0x0f, 0x46, 0xc3, // CMOVBE AX, BX
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
fn test_cmovbe_rax_rbx() {
    let code = [
        0x48, 0xc7, 0xc2, 0x32, 0x00, 0x00, 0x00, // MOV RDX, 50
        0x48, 0xc7, 0xc1, 0x64, 0x00, 0x00, 0x00, // MOV RCX, 100
        0x48, 0x39, 0xca, // CMP RDX, RCX
        0x48, 0x0f, 0x46, 0xc3, // CMOVBE RAX, RBX
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
fn test_cmovbe_r8d_r9d() {
    let code = [
        0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1
        0x48, 0xc7, 0xc1, 0xff, 0x00, 0x00, 0x00, // MOV RCX, 255
        0x48, 0x39, 0xc8, // CMP RAX, RCX
        0x45, 0x0f, 0x46, 0xc1, // CMOVBE R8D, R9D
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
fn test_cmovbe_unsigned_comparison() {
    let code = [
        0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1
        0x48, 0xc7, 0xc1, 0xff, 0xff, 0xff, 0xff, // MOV RCX, 0xFFFFFFFF
        0x48, 0x39, 0xc8, // CMP RAX, RCX (unsigned: 1 <= 0xFFFFFFFF)
        0x0f, 0x46, 0xc3, // CMOVBE EAX, EBX
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x11111111;
    regs.rbx = 0x22222222;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFFFFFFFF, 0x22222222, "EAX should be moved");
}

// Test with 0 comparison
#[test]
fn test_cmovbe_zero_comparison() {
    let code = [
        0x48, 0xc7, 0xc0, 0x00, 0x00, 0x00, 0x00, // MOV RAX, 0
        0x48, 0xc7, 0xc1, 0x00, 0x00, 0x00, 0x00, // MOV RCX, 0
        0x48, 0x39, 0xc8, // CMP RAX, RCX (0 <= 0)
        0x0f, 0x46, 0xc3, // CMOVBE EAX, EBX
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
        "EAX should be moved when 0 <= 0"
    );
}

// Test that flags are preserved
#[test]
fn test_cmovbe_preserves_flags() {
    let code = [
        0x48, 0xc7, 0xc0, 0x03, 0x00, 0x00, 0x00, // MOV RAX, 3
        0x48, 0xc7, 0xc1, 0x05, 0x00, 0x00, 0x00, // MOV RCX, 5
        0x48, 0x39, 0xc8, // CMP RAX, RCX
        0x0f, 0x46, 0xc3, // CMOVBE EAX, EBX
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
fn test_cmovbe_zeros_upper_32() {
    let code = [
        0x48, 0xc7, 0xc2, 0x05, 0x00, 0x00, 0x00, // MOV RDX, 5
        0x48, 0xc7, 0xc1, 0x0a, 0x00, 0x00, 0x00, // MOV RCX, 10
        0x48, 0x39, 0xca, // CMP RDX, RCX
        0x0f, 0x46, 0xc3, // CMOVBE EAX, EBX
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

// Test practical use case: clamping to max value
#[test]
fn test_cmovbe_practical_clamp() {
    // if (value <= max) result = value; else result = max
    let code = [
        0x48, 0x39, 0xd8, // CMP RAX, RBX (compare value with max)
        0x48, 0x0f, 0x47, 0xc3, // CMOVA RAX, RBX (if above max, clamp to max)
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 50; // value
    regs.rbx = 100; // max
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 50, "Value should remain 50 (within limit)");
}

// Test with subtraction
#[test]
fn test_cmovbe_after_sub_borrow() {
    let code = [
        0x48, 0xc7, 0xc0, 0x05, 0x00, 0x00, 0x00, // MOV RAX, 5
        0x48, 0xc7, 0xc1, 0x0a, 0x00, 0x00, 0x00, // MOV RCX, 10
        0x48, 0x29, 0xc8, // SUB RAX, RCX (5 - 10, borrow, CF=1)
        0x0f, 0x46, 0xd3, // CMOVBE EDX, EBX
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
fn test_cmovbe_after_sub_equal() {
    let code = [
        0x48, 0xc7, 0xc0, 0x0a, 0x00, 0x00, 0x00, // MOV RAX, 10
        0x48, 0xc7, 0xc1, 0x0a, 0x00, 0x00, 0x00, // MOV RCX, 10
        0x48, 0x29, 0xc8, // SUB RAX, RCX (10 - 10 = 0, ZF=1)
        0x0f, 0x46, 0xd3, // CMOVBE EDX, EBX
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
        "EDX should be moved (ZF=1)"
    );
}

// Test edge case: comparing with itself
#[test]
fn test_cmovbe_self_comparison() {
    let code = [
        0x48, 0x39, 0xc0, // CMP RAX, RAX (always equal, ZF=1, CF=0)
        0x0f, 0x46, 0xc3, // CMOVBE EAX, EBX
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
        "EAX should be moved (equal is <=)"
    );
}

// Test different register combinations
#[test]
fn test_cmovbe_esi_edi() {
    let code = [
        0x48, 0xc7, 0xc0, 0x0a, 0x00, 0x00, 0x00, // MOV RAX, 10
        0x48, 0xc7, 0xc1, 0x14, 0x00, 0x00, 0x00, // MOV RCX, 20
        0x48, 0x39, 0xc8, // CMP RAX, RCX
        0x0f, 0x46, 0xf7, // CMOVBE ESI, EDI
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsi = 0x55555555;
    regs.rdi = 0x66666666;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rsi & 0xFFFFFFFF, 0x66666666, "ESI should be moved");
}

// Test boundary: just at limit
#[test]
fn test_cmovbe_boundary_at_limit() {
    let code = [
        0x48, 0xc7, 0xc0, 0xff, 0x00, 0x00, 0x00, // MOV RAX, 255
        0x48, 0xc7, 0xc1, 0xff, 0x00, 0x00, 0x00, // MOV RCX, 255
        0x48, 0x39, 0xc8, // CMP RAX, RCX (255 <= 255)
        0x0f, 0x46, 0xc3, // CMOVBE EAX, EBX
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x11111111;
    regs.rbx = 0x22222222;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFFFFFFFF, 0x22222222, "EAX should be moved");
}

// Test boundary: just above limit
#[test]
fn test_cmovbe_boundary_above_limit() {
    let code = [
        0x48, 0xc7, 0xc0, 0x00, 0x01, 0x00, 0x00, // MOV RAX, 256
        0x48, 0xc7, 0xc1, 0xff, 0x00, 0x00, 0x00, // MOV RCX, 255
        0x48, 0x39, 0xc8, // CMP RAX, RCX (256 > 255)
        0x0f, 0x46, 0xc3, // CMOVBE EAX, EBX
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x11111111;
    regs.rbx = 0x22222222;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFFFFFFFF, 0x00000100, "EAX should not be moved");
}

// Test maximum unsigned values
#[test]
fn test_cmovbe_max_unsigned() {
    let code = [
        0x48, 0xc7, 0xc0, 0xff, 0xff, 0xff, 0xff, // MOV RAX, 0xFFFFFFFF
        0x48, 0xc7, 0xc1, 0xff, 0xff, 0xff, 0xff, // MOV RCX, 0xFFFFFFFF
        0x48, 0x39, 0xc8, // CMP RAX, RCX (0xFFFFFFFF <= 0xFFFFFFFF)
        0x0f, 0x46, 0xc3, // CMOVBE EAX, EBX
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x11111111;
    regs.rbx = 0x22222222;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFFFFFFFF, 0x22222222, "EAX should be moved");
}

// Test chaining operations
#[test]
fn test_cmovbe_chain() {
    let code = [
        0x48, 0xc7, 0xc0, 0x03, 0x00, 0x00, 0x00, // MOV RAX, 3
        0x48, 0xc7, 0xc1, 0x05, 0x00, 0x00, 0x00, // MOV RCX, 5
        0x48, 0x39, 0xc8, // CMP RAX, RCX (3 <= 5)
        0x0f, 0x46, 0xc3, // CMOVBE EAX, EBX (should move)
        0x48, 0xc7, 0xc2, 0x0a, 0x00, 0x00, 0x00, // MOV RDX, 10
        0x48, 0xc7, 0xc1, 0x05, 0x00, 0x00, 0x00, // MOV RCX, 5
        0x48, 0x39, 0xca, // CMP RDX, RCX (10 > 5)
        0x0f, 0x46, 0xf7, // CMOVBE ESI, EDI (should not move)
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

// Test practical use case: bounds checking
#[test]
fn test_cmovbe_practical_bounds_check() {
    // if (index <= max_index) valid = true
    let code = [
        0x48, 0x39, 0xd8, // CMP RAX, RBX (compare index with max)
        0x48, 0x0f, 0x46, 0xd1, // CMOVBE RDX, RCX (if within bounds, set valid flag)
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 10; // index
    regs.rbx = 100; // max_index
    regs.rdx = 0; // valid flag (initially false)
    regs.rcx = 1; // true value
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rdx, 1, "Valid flag should be set");
}

#[test]
fn test_cmovbe_practical_bounds_check_out_of_bounds() {
    let code = [
        0x48, 0x39, 0xd8, // CMP RAX, RBX
        0x48, 0x0f, 0x46, 0xd1, // CMOVBE RDX, RCX
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 150; // index (out of bounds)
    regs.rbx = 100; // max_index
    regs.rdx = 0; // valid flag
    regs.rcx = 1; // true value
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rdx, 0, "Valid flag should remain false");
}
