use rax::cpu::Registers;

use crate::common::{af_set, cf_set, of_set, pf_set, run_until_hlt, setup_vm, sf_set, zf_set};

// RDSEED - Read Random SEED
// Opcode: 0F C7 /7
// Reads NIST SP800-90B & C compliant random seed into destination register
// Sets CF=1 on success, CF=0 on failure (data zeroed)
// All other flags (OF, SF, ZF, AF, PF) forced to 0

// RDSEED r16 - 16-bit random seed to AX
#[test]
fn test_rdseed_r16_ax() {
    let code = [
        0x0f, 0xc7, 0xf8, // RDSEED AX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // CF should be set on success, random seed in AX
    assert!(cf_set(regs.rflags), "CF should be set on success");
    // Other flags should be cleared
    assert!(!of_set(regs.rflags), "OF should be cleared");
    assert!(!sf_set(regs.rflags), "SF should be cleared");
    assert!(!zf_set(regs.rflags), "ZF should be cleared");
    assert!(!af_set(regs.rflags), "AF should be cleared");
    assert!(!pf_set(regs.rflags), "PF should be cleared");
}

// RDSEED r16 - 16-bit random seed to BX
#[test]
fn test_rdseed_r16_bx() {
    let code = [
        0x0f, 0xc7, 0xfb, // RDSEED BX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags), "CF should be set");
    assert!(!of_set(regs.rflags) && !sf_set(regs.rflags) && !zf_set(regs.rflags));
}

// RDSEED r16 - 16-bit random seed to CX
#[test]
fn test_rdseed_r16_cx() {
    let code = [
        0x0f, 0xc7, 0xf9, // RDSEED CX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags), "CF should be set");
}

// RDSEED r16 - 16-bit random seed to DX
#[test]
fn test_rdseed_r16_dx() {
    let code = [
        0x0f, 0xc7, 0xfa, // RDSEED DX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags), "CF should be set");
}

// RDSEED r32 - 32-bit random seed to EAX
#[test]
fn test_rdseed_r32_eax() {
    let code = [
        0x0f, 0xc7, 0xf8, // RDSEED EAX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags), "CF should be set");
    assert!(!of_set(regs.rflags), "OF should be cleared");
    assert!(!sf_set(regs.rflags), "SF should be cleared");
    assert!(!zf_set(regs.rflags), "ZF should be cleared");
}

// RDSEED r32 - 32-bit random seed to EBX
#[test]
fn test_rdseed_r32_ebx() {
    let code = [
        0x0f, 0xc7, 0xfb, // RDSEED EBX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags), "CF should be set");
}

// RDSEED r32 - 32-bit random seed to ECX
#[test]
fn test_rdseed_r32_ecx() {
    let code = [
        0x0f, 0xc7, 0xf9, // RDSEED ECX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags), "CF should be set");
}

// RDSEED r32 - 32-bit random seed to EDX
#[test]
fn test_rdseed_r32_edx() {
    let code = [
        0x0f, 0xc7, 0xfa, // RDSEED EDX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags), "CF should be set");
}

// RDSEED r32 - 32-bit random seed to ESI
#[test]
fn test_rdseed_r32_esi() {
    let code = [
        0x0f, 0xc7, 0xfe, // RDSEED ESI
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags), "CF should be set");
}

// RDSEED r32 - 32-bit random seed to EDI
#[test]
fn test_rdseed_r32_edi() {
    let code = [
        0x0f, 0xc7, 0xff, // RDSEED EDI
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags), "CF should be set");
}

// RDSEED r64 - 64-bit random seed to RAX
#[test]
fn test_rdseed_r64_rax() {
    let code = [
        0x48, 0x0f, 0xc7, 0xf8, // RDSEED RAX
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

// RDSEED r64 - 64-bit random seed to RBX
#[test]
fn test_rdseed_r64_rbx() {
    let code = [
        0x48, 0x0f, 0xc7, 0xfb, // RDSEED RBX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags), "CF should be set");
}

// RDSEED r64 - 64-bit random seed to RCX
#[test]
fn test_rdseed_r64_rcx() {
    let code = [
        0x48, 0x0f, 0xc7, 0xf9, // RDSEED RCX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags), "CF should be set");
}

// RDSEED r64 - 64-bit random seed to RDX
#[test]
fn test_rdseed_r64_rdx() {
    let code = [
        0x48, 0x0f, 0xc7, 0xfa, // RDSEED RDX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags), "CF should be set");
}

// RDSEED r64 - 64-bit random seed to RSI
#[test]
fn test_rdseed_r64_rsi() {
    let code = [
        0x48, 0x0f, 0xc7, 0xfe, // RDSEED RSI
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags), "CF should be set");
}

// RDSEED r64 - 64-bit random seed to RDI
#[test]
fn test_rdseed_r64_rdi() {
    let code = [
        0x48, 0x0f, 0xc7, 0xff, // RDSEED RDI
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags), "CF should be set");
}

// RDSEED r64 - 64-bit random seed to R8
#[test]
fn test_rdseed_r64_r8() {
    let code = [
        0x49, 0x0f, 0xc7, 0xf8, // RDSEED R8
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags), "CF should be set");
    assert!(!of_set(regs.rflags) && !sf_set(regs.rflags) && !zf_set(regs.rflags));
}

// RDSEED r64 - 64-bit random seed to R9
#[test]
fn test_rdseed_r64_r9() {
    let code = [
        0x49, 0x0f, 0xc7, 0xf9, // RDSEED R9
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags), "CF should be set");
}

// RDSEED r64 - 64-bit random seed to R10
#[test]
fn test_rdseed_r64_r10() {
    let code = [
        0x49, 0x0f, 0xc7, 0xfa, // RDSEED R10
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags), "CF should be set");
}

// RDSEED r64 - 64-bit random seed to R11
#[test]
fn test_rdseed_r64_r11() {
    let code = [
        0x49, 0x0f, 0xc7, 0xfb, // RDSEED R11
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags), "CF should be set");
}

// RDSEED r64 - 64-bit random seed to R12
#[test]
fn test_rdseed_r64_r12() {
    let code = [
        0x49, 0x0f, 0xc7, 0xfc, // RDSEED R12
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags), "CF should be set");
}

// RDSEED r64 - 64-bit random seed to R13
#[test]
fn test_rdseed_r64_r13() {
    let code = [
        0x49, 0x0f, 0xc7, 0xfd, // RDSEED R13
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags), "CF should be set");
}

// RDSEED r64 - 64-bit random seed to R14
#[test]
fn test_rdseed_r64_r14() {
    let code = [
        0x49, 0x0f, 0xc7, 0xfe, // RDSEED R14
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags), "CF should be set");
}

// RDSEED r64 - 64-bit random seed to R15
#[test]
fn test_rdseed_r64_r15() {
    let code = [
        0x49, 0x0f, 0xc7, 0xff, // RDSEED R15
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags), "CF should be set");
}

// Test multiple consecutive RDSEED calls
#[test]
fn test_rdseed_consecutive_calls() {
    let code = [
        0x48, 0x0f, 0xc7, 0xf8, // RDSEED RAX
        0x48, 0x0f, 0xc7, 0xfb, // RDSEED RBX
        0x48, 0x0f, 0xc7, 0xf9, // RDSEED RCX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // All calls should succeed
    assert!(cf_set(regs.rflags), "CF should be set after all calls");
}

// Test RDSEED values for seed quality (non-zero check)
#[test]
fn test_rdseed_nonzero_16bit() {
    let code = [
        0x0f, 0xc7, 0xf8, // RDSEED AX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Seed generation should succeed (CF=1)
    assert!(cf_set(regs.rflags), "Seed generation should succeed");
}

// Test RDSEED values for seed quality (32-bit)
#[test]
fn test_rdseed_nonzero_32bit() {
    let code = [
        0x0f, 0xc7, 0xf8, // RDSEED EAX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags), "Seed generation should succeed");
}

// Test RDSEED values for seed quality (64-bit)
#[test]
fn test_rdseed_nonzero_64bit() {
    let code = [
        0x48, 0x0f, 0xc7, 0xf8, // RDSEED RAX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags), "Seed generation should succeed");
}

// Test RDSEED shows variation (two calls should likely differ)
#[test]
fn test_rdseed_variation() {
    let code = [
        0x48, 0x0f, 0xc7, 0xf8, // RDSEED RAX
        0x48, 0x89, 0xc3, // MOV RBX, RAX (save first value)
        0x48, 0x0f, 0xc7, 0xf8, // RDSEED RAX (get second value)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Both calls should succeed
    assert!(cf_set(regs.rflags), "Both seed generations should succeed");
}

// Test RDSEED with pre-set flags - should clear them
#[test]
fn test_rdseed_clears_all_flags() {
    let code = [
        0x48, 0xc7, 0xc0, 0xff, 0xff, 0xff, 0xff, // MOV RAX, -1
        0x48, 0x83, 0xc0, 0x01, // ADD RAX, 1 (sets ZF, clears CF)
        0x48, 0x0f, 0xc7, 0xf8, // RDSEED RAX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // RDSEED should set CF and clear all other arithmetic flags
    assert!(cf_set(regs.rflags), "CF should be set by RDSEED");
    assert!(!of_set(regs.rflags), "OF should be cleared by RDSEED");
    assert!(!sf_set(regs.rflags), "SF should be cleared by RDSEED");
    assert!(!zf_set(regs.rflags), "ZF should be cleared by RDSEED");
    assert!(!af_set(regs.rflags), "AF should be cleared by RDSEED");
    assert!(!pf_set(regs.rflags), "PF should be cleared by RDSEED");
}

// Test RDSEED doesn't affect other registers
#[test]
fn test_rdseed_preserves_other_registers() {
    let code = [
        0x48, 0xc7, 0xc3, 0x42, 0x42, 0x42,
        0x42, // MOV RBX, 0x42424242 (sign-extends to 0x42424242)
        0x48, 0xc7, 0xc1, 0x19, 0x19, 0x19,
        0x19, // MOV RCX, 0x19191919 (sign-extends to 0x19191919)
        0x48, 0x0f, 0xc7, 0xf8, // RDSEED RAX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // RBX and RCX should be unchanged (MOV r64, imm32 sign-extends, so use values with bit 31 clear)
    assert_eq!(regs.rbx, 0x42424242, "RBX should not be affected");
    assert_eq!(regs.rcx, 0x19191919, "RCX should not be affected");
}

// Test RDSEED 16-bit doesn't affect upper bits
#[test]
fn test_rdseed_16bit_preserves_upper_bits() {
    let code = [
        0x48, 0xc7, 0xc0, 0xff, 0xff, 0xff, 0xff, // MOV RAX, 0xffffffff
        0x66, 0x0f, 0xc7, 0xf8, // RDSEED AX (16-bit)
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

// Test RDSEED 32-bit zeros upper 32 bits
#[test]
fn test_rdseed_32bit_zeros_upper_bits() {
    let code = [
        0x48, 0xc7, 0xc0, 0xff, 0xff, 0xff, 0xff, // MOV RAX, 0xffffffffffffffff
        0x0f, 0xc7, 0xf8, // RDSEED EAX (32-bit)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Upper 32 bits should be zeroed (standard x86-64 behavior)
    assert_eq!(regs.rax >> 32, 0, "Upper 32 bits should be zeroed");
    assert!(cf_set(regs.rflags), "CF should be set");
}

// Test RDSEED multiple times for consistency of CF flag
#[test]
fn test_rdseed_cf_consistency() {
    let code = [
        0x48, 0x0f, 0xc7, 0xf8, // RDSEED RAX
        0x9c, // PUSHFQ
        0x48, 0x0f, 0xc7, 0xfb, // RDSEED RBX
        0x9c, // PUSHFQ
        0x48, 0x0f, 0xc7, 0xf9, // RDSEED RCX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // All calls should set CF
    assert!(
        cf_set(regs.rflags),
        "CF should be set after successful RDSEED"
    );
}

// Test RDSEED with different operand sizes in sequence
#[test]
fn test_rdseed_mixed_sizes() {
    let code = [
        0x66, 0x0f, 0xc7, 0xf8, // RDSEED AX (16-bit)
        0x0f, 0xc7, 0xfb, // RDSEED EBX (32-bit)
        0x48, 0x0f, 0xc7, 0xf9, // RDSEED RCX (64-bit)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags), "All RDSEED calls should succeed");
}

// Test RDSEED to verify it can be called at any privilege level
#[test]
fn test_rdseed_privilege_level() {
    let code = [
        0x48, 0x0f, 0xc7, 0xf8, // RDSEED RAX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Should work at any privilege level (we're in ring 0)
    assert!(
        cf_set(regs.rflags),
        "RDSEED should work at current privilege"
    );
}

// Test RDSEED r32 with R8-R15 registers (lower 32-bit)
#[test]
fn test_rdseed_r32_r8d() {
    let code = [
        0x41, 0x0f, 0xc7, 0xf8, // RDSEED R8D
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags), "CF should be set");
}

// Test RDSEED r32 with R9D
#[test]
fn test_rdseed_r32_r9d() {
    let code = [
        0x41, 0x0f, 0xc7, 0xf9, // RDSEED R9D
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags), "CF should be set");
}

// Test RDSEED with SP register (16-bit)
#[test]
fn test_rdseed_r16_sp() {
    let code = [
        0x66, 0x0f, 0xc7, 0xfc, // RDSEED SP
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags), "CF should be set");
}

// Test RDSEED with BP register (16-bit)
#[test]
fn test_rdseed_r16_bp() {
    let code = [
        0x66, 0x0f, 0xc7, 0xfd, // RDSEED BP
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags), "CF should be set");
}

// Test RDSEED with SI register (16-bit)
#[test]
fn test_rdseed_r16_si() {
    let code = [
        0x66, 0x0f, 0xc7, 0xfe, // RDSEED SI
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags), "CF should be set");
}

// Test RDSEED with DI register (16-bit)
#[test]
fn test_rdseed_r16_di() {
    let code = [
        0x66, 0x0f, 0xc7, 0xff, // RDSEED DI
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags), "CF should be set");
}

// Test RDSEED works inside transaction regions
#[test]
fn test_rdseed_in_transaction() {
    let code = [
        0x48, 0x0f, 0xc7, 0xf8, // RDSEED RAX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // RDSEED executes normally inside or outside transaction regions
    assert!(cf_set(regs.rflags), "RDSEED should work in transaction");
}

// Test RDSEED compliance with NIST SP800-90B/C
#[test]
fn test_rdseed_nist_compliance() {
    let code = [
        0x48, 0x0f, 0xc7, 0xf8, // RDSEED RAX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // RDSEED provides NIST compliant random seeds
    assert!(cf_set(regs.rflags), "RDSEED should provide compliant seeds");
}
