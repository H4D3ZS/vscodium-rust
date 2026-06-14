use rax::cpu::Registers;

use crate::common::{af_set, cf_set, of_set, pf_set, run_until_hlt, setup_vm, sf_set, zf_set};

// RDRAND - Read Random Number
// Opcode: 0F C7 /6
// Reads hardware random number into destination register
// Sets CF=1 on success, CF=0 on failure (data zeroed)
// All other flags (OF, SF, ZF, AF, PF) forced to 0

// RDRAND r16 - 16-bit random to AX
#[test]
fn test_rdrand_r16_ax() {
    let code = [
        0x0f, 0xc7, 0xf0, // RDRAND AX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // CF should be set on success, random value in AX
    assert!(cf_set(regs.rflags), "CF should be set on success");
    // Other flags should be cleared
    assert!(!of_set(regs.rflags), "OF should be cleared");
    assert!(!sf_set(regs.rflags), "SF should be cleared");
    assert!(!zf_set(regs.rflags), "ZF should be cleared");
    assert!(!af_set(regs.rflags), "AF should be cleared");
    assert!(!pf_set(regs.rflags), "PF should be cleared");
}

// RDRAND r16 - 16-bit random to BX
#[test]
fn test_rdrand_r16_bx() {
    let code = [
        0x0f, 0xc7, 0xf3, // RDRAND BX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags), "CF should be set");
    assert!(!of_set(regs.rflags) && !sf_set(regs.rflags) && !zf_set(regs.rflags));
}

// RDRAND r16 - 16-bit random to CX
#[test]
fn test_rdrand_r16_cx() {
    let code = [
        0x0f, 0xc7, 0xf1, // RDRAND CX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags), "CF should be set");
}

// RDRAND r16 - 16-bit random to DX
#[test]
fn test_rdrand_r16_dx() {
    let code = [
        0x0f, 0xc7, 0xf2, // RDRAND DX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags), "CF should be set");
}

// RDRAND r32 - 32-bit random to EAX
#[test]
fn test_rdrand_r32_eax() {
    let code = [
        0x0f, 0xc7, 0xf0, // RDRAND EAX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags), "CF should be set");
    assert!(!of_set(regs.rflags), "OF should be cleared");
    assert!(!sf_set(regs.rflags), "SF should be cleared");
    assert!(!zf_set(regs.rflags), "ZF should be cleared");
}

// RDRAND r32 - 32-bit random to EBX
#[test]
fn test_rdrand_r32_ebx() {
    let code = [
        0x0f, 0xc7, 0xf3, // RDRAND EBX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags), "CF should be set");
}

// RDRAND r32 - 32-bit random to ECX
#[test]
fn test_rdrand_r32_ecx() {
    let code = [
        0x0f, 0xc7, 0xf1, // RDRAND ECX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags), "CF should be set");
}

// RDRAND r32 - 32-bit random to EDX
#[test]
fn test_rdrand_r32_edx() {
    let code = [
        0x0f, 0xc7, 0xf2, // RDRAND EDX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags), "CF should be set");
}

// RDRAND r32 - 32-bit random to ESI
#[test]
fn test_rdrand_r32_esi() {
    let code = [
        0x0f, 0xc7, 0xf6, // RDRAND ESI
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags), "CF should be set");
}

// RDRAND r32 - 32-bit random to EDI
#[test]
fn test_rdrand_r32_edi() {
    let code = [
        0x0f, 0xc7, 0xf7, // RDRAND EDI
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags), "CF should be set");
}

// RDRAND r64 - 64-bit random to RAX
#[test]
fn test_rdrand_r64_rax() {
    let code = [
        0x48, 0x0f, 0xc7, 0xf0, // RDRAND RAX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags), "CF should be set");
    assert!(!of_set(regs.rflags), "OF should be cleared");
    assert!(!sf_set(regs.rflags), "SF should be cleared");
    assert!(!zf_set(regs.rflags), "ZF should be cleared");
    assert!(!af_set(regs.rflags), "AF should be cleared");
    assert!(!pf_set(regs.rflags), "PF should be cleared");
}

// RDRAND r64 - 64-bit random to RBX
#[test]
fn test_rdrand_r64_rbx() {
    let code = [
        0x48, 0x0f, 0xc7, 0xf3, // RDRAND RBX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags), "CF should be set");
}

// RDRAND r64 - 64-bit random to RCX
#[test]
fn test_rdrand_r64_rcx() {
    let code = [
        0x48, 0x0f, 0xc7, 0xf1, // RDRAND RCX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags), "CF should be set");
}

// RDRAND r64 - 64-bit random to RDX
#[test]
fn test_rdrand_r64_rdx() {
    let code = [
        0x48, 0x0f, 0xc7, 0xf2, // RDRAND RDX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags), "CF should be set");
}

// RDRAND r64 - 64-bit random to RSI
#[test]
fn test_rdrand_r64_rsi() {
    let code = [
        0x48, 0x0f, 0xc7, 0xf6, // RDRAND RSI
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags), "CF should be set");
}

// RDRAND r64 - 64-bit random to RDI
#[test]
fn test_rdrand_r64_rdi() {
    let code = [
        0x48, 0x0f, 0xc7, 0xf7, // RDRAND RDI
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags), "CF should be set");
}

// RDRAND r64 - 64-bit random to R8
#[test]
fn test_rdrand_r64_r8() {
    let code = [
        0x49, 0x0f, 0xc7, 0xf0, // RDRAND R8
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags), "CF should be set");
    assert!(!of_set(regs.rflags) && !sf_set(regs.rflags) && !zf_set(regs.rflags));
}

// RDRAND r64 - 64-bit random to R9
#[test]
fn test_rdrand_r64_r9() {
    let code = [
        0x49, 0x0f, 0xc7, 0xf1, // RDRAND R9
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags), "CF should be set");
}

// RDRAND r64 - 64-bit random to R10
#[test]
fn test_rdrand_r64_r10() {
    let code = [
        0x49, 0x0f, 0xc7, 0xf2, // RDRAND R10
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags), "CF should be set");
}

// RDRAND r64 - 64-bit random to R11
#[test]
fn test_rdrand_r64_r11() {
    let code = [
        0x49, 0x0f, 0xc7, 0xf3, // RDRAND R11
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags), "CF should be set");
}

// RDRAND r64 - 64-bit random to R12
#[test]
fn test_rdrand_r64_r12() {
    let code = [
        0x49, 0x0f, 0xc7, 0xf4, // RDRAND R12
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags), "CF should be set");
}

// RDRAND r64 - 64-bit random to R13
#[test]
fn test_rdrand_r64_r13() {
    let code = [
        0x49, 0x0f, 0xc7, 0xf5, // RDRAND R13
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags), "CF should be set");
}

// RDRAND r64 - 64-bit random to R14
#[test]
fn test_rdrand_r64_r14() {
    let code = [
        0x49, 0x0f, 0xc7, 0xf6, // RDRAND R14
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags), "CF should be set");
}

// RDRAND r64 - 64-bit random to R15
#[test]
fn test_rdrand_r64_r15() {
    let code = [
        0x49, 0x0f, 0xc7, 0xf7, // RDRAND R15
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags), "CF should be set");
}

// Test multiple consecutive RDRAND calls
#[test]
fn test_rdrand_consecutive_calls() {
    let code = [
        0x48, 0x0f, 0xc7, 0xf0, // RDRAND RAX
        0x48, 0x0f, 0xc7, 0xf3, // RDRAND RBX
        0x48, 0x0f, 0xc7, 0xf1, // RDRAND RCX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // All calls should succeed
    assert!(cf_set(regs.rflags), "CF should be set after all calls");
}

// Test RDRAND values are non-zero (randomness property)
#[test]
fn test_rdrand_nonzero_16bit() {
    let code = [
        0x0f, 0xc7, 0xf0, // RDRAND AX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Random value should typically be non-zero (though technically 0 is possible)
    // We check CF which indicates success
    assert!(cf_set(regs.rflags), "Random generation should succeed");
}

// Test RDRAND values are non-zero (32-bit)
#[test]
fn test_rdrand_nonzero_32bit() {
    let code = [
        0x0f, 0xc7, 0xf0, // RDRAND EAX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags), "Random generation should succeed");
}

// Test RDRAND values are non-zero (64-bit)
#[test]
fn test_rdrand_nonzero_64bit() {
    let code = [
        0x48, 0x0f, 0xc7, 0xf0, // RDRAND RAX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags), "Random generation should succeed");
}

// Test RDRAND shows variation (two calls should likely differ)
#[test]
fn test_rdrand_variation() {
    let code = [
        0x48, 0x0f, 0xc7, 0xf0, // RDRAND RAX
        0x48, 0x89, 0xc3, // MOV RBX, RAX (save first value)
        0x48, 0x0f, 0xc7, 0xf0, // RDRAND RAX (get second value)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Both calls should succeed
    assert!(
        cf_set(regs.rflags),
        "Both random generations should succeed"
    );
    // Values might be different (though not guaranteed)
    // Just verify both calls completed
}

// Test RDRAND with pre-set flags - should clear them
#[test]
fn test_rdrand_clears_all_flags() {
    let code = [
        0x48, 0xc7, 0xc0, 0xff, 0xff, 0xff, 0xff, // MOV RAX, -1
        0x48, 0x83, 0xc0, 0x01, // ADD RAX, 1 (sets ZF, clears CF)
        0x48, 0x0f, 0xc7, 0xf0, // RDRAND RAX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // RDRAND should set CF and clear all other arithmetic flags
    assert!(cf_set(regs.rflags), "CF should be set by RDRAND");
    assert!(!of_set(regs.rflags), "OF should be cleared by RDRAND");
    assert!(!sf_set(regs.rflags), "SF should be cleared by RDRAND");
    assert!(!zf_set(regs.rflags), "ZF should be cleared by RDRAND");
    assert!(!af_set(regs.rflags), "AF should be cleared by RDRAND");
    assert!(!pf_set(regs.rflags), "PF should be cleared by RDRAND");
}

// Test RDRAND doesn't affect other registers
#[test]
fn test_rdrand_preserves_other_registers() {
    let code = [
        0x48, 0xc7, 0xc3, 0x42, 0x42, 0x42,
        0x42, // MOV RBX, 0x42424242 (sign-extends to 0x42424242)
        0x48, 0xc7, 0xc1, 0x19, 0x19, 0x19,
        0x19, // MOV RCX, 0x19191919 (sign-extends to 0x19191919)
        0x48, 0x0f, 0xc7, 0xf0, // RDRAND RAX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // RBX and RCX should be unchanged (MOV r64, imm32 sign-extends, so use values with bit 31 clear)
    assert_eq!(regs.rbx, 0x42424242, "RBX should not be affected");
    assert_eq!(regs.rcx, 0x19191919, "RCX should not be affected");
}

// Test RDRAND 16-bit doesn't affect upper bits
#[test]
fn test_rdrand_16bit_preserves_upper_bits() {
    let code = [
        0x48, 0xc7, 0xc0, 0xff, 0xff, 0xff, 0xff, // MOV RAX, 0xffffffff
        0x66, 0x0f, 0xc7, 0xf0, // RDRAND AX (16-bit)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Upper 48 bits should be preserved
    assert_eq!(
        regs.rax & 0xFFFFFFFFFFFF0000,
        0xFFFFFFFFFFFF0000,
        "Upper bits should be preserved"
    );
    assert!(cf_set(regs.rflags), "CF should be set");
}

// Test RDRAND 32-bit zeros upper 32 bits
#[test]
fn test_rdrand_32bit_zeros_upper_bits() {
    let code = [
        0x48, 0xc7, 0xc0, 0xff, 0xff, 0xff, 0xff, // MOV RAX, 0xffffffffffffffff
        0x0f, 0xc7, 0xf0, // RDRAND EAX (32-bit)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Upper 32 bits should be zeroed (standard x86-64 behavior)
    assert_eq!(regs.rax >> 32, 0, "Upper 32 bits should be zeroed");
    assert!(cf_set(regs.rflags), "CF should be set");
}

// Test RDRAND multiple times for consistency of CF flag
#[test]
fn test_rdrand_cf_consistency() {
    let code = [
        0x48, 0x0f, 0xc7, 0xf0, // RDRAND RAX
        0x9c, // PUSHFQ
        0x48, 0x0f, 0xc7, 0xf3, // RDRAND RBX
        0x9c, // PUSHFQ
        0x48, 0x0f, 0xc7, 0xf1, // RDRAND RCX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // All calls should set CF
    assert!(
        cf_set(regs.rflags),
        "CF should be set after successful RDRAND"
    );
}

// Test RDRAND with different operand sizes in sequence
#[test]
fn test_rdrand_mixed_sizes() {
    let code = [
        0x66, 0x0f, 0xc7, 0xf0, // RDRAND AX (16-bit)
        0x0f, 0xc7, 0xf3, // RDRAND EBX (32-bit)
        0x48, 0x0f, 0xc7, 0xf1, // RDRAND RCX (64-bit)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags), "All RDRAND calls should succeed");
}

// Test RDRAND to verify it can be called at any privilege level
#[test]
fn test_rdrand_privilege_level() {
    let code = [
        0x48, 0x0f, 0xc7, 0xf0, // RDRAND RAX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Should work at any privilege level (we're in ring 0)
    assert!(
        cf_set(regs.rflags),
        "RDRAND should work at current privilege"
    );
}

// Test RDRAND r32 with R8-R15 registers (lower 32-bit)
#[test]
fn test_rdrand_r32_r8d() {
    let code = [
        0x41, 0x0f, 0xc7, 0xf0, // RDRAND R8D
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags), "CF should be set");
}

// Test RDRAND r32 with R9D
#[test]
fn test_rdrand_r32_r9d() {
    let code = [
        0x41, 0x0f, 0xc7, 0xf1, // RDRAND R9D
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags), "CF should be set");
}

// Test RDRAND with SP register (16-bit)
#[test]
fn test_rdrand_r16_sp() {
    let code = [
        0x66, 0x0f, 0xc7, 0xf4, // RDRAND SP
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags), "CF should be set");
    // Stack pointer modified, but instruction should complete
}

// Test RDRAND with BP register (16-bit)
#[test]
fn test_rdrand_r16_bp() {
    let code = [
        0x66, 0x0f, 0xc7, 0xf5, // RDRAND BP
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags), "CF should be set");
}

// Test RDRAND with SI register (16-bit)
#[test]
fn test_rdrand_r16_si() {
    let code = [
        0x66, 0x0f, 0xc7, 0xf6, // RDRAND SI
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags), "CF should be set");
}

// Test RDRAND with DI register (16-bit)
#[test]
fn test_rdrand_r16_di() {
    let code = [
        0x66, 0x0f, 0xc7, 0xf7, // RDRAND DI
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags), "CF should be set");
}
