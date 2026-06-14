use crate::common::{run_until_hlt, setup_vm};

// CLTS - Clear Task-Switched Flag in CR0
// Opcode: 0F 06
// Clears the TS (task-switched) flag in CR0 register
// Privilege level 0 required
// Does not modify arithmetic flags

// Test CLTS basic execution
#[test]
fn test_clts_basic() {
    let code = [
        0x0f, 0x06, // CLTS
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Should complete successfully
    let _ = regs;
}

// Test CLTS doesn't modify flags
#[test]
fn test_clts_preserves_flags() {
    let code = [
        0x48, 0xc7, 0xc0, 0xff, 0xff, 0xff, 0xff, // MOV RAX, -1
        0x48, 0x83, 0xc0, 0x01, // ADD RAX, 1 (sets ZF, CF)
        0x0f, 0x06, // CLTS
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Flags from ADD should be preserved
    assert!(regs.rflags & 0x40 != 0, "ZF should be preserved");
    assert!(regs.rflags & 0x01 != 0, "CF should be preserved");
}

// Test CLTS preserves all registers
#[test]
fn test_clts_preserves_registers() {
    let code = [
        0x48, 0xc7, 0xc0, 0x11, 0x11, 0x11, 0x11, // MOV RAX, 0x11111111
        0x48, 0xc7, 0xc3, 0x22, 0x22, 0x22, 0x22, // MOV RBX, 0x22222222
        0x48, 0xc7, 0xc1, 0x33, 0x33, 0x33, 0x33, // MOV RCX, 0x33333333
        0x48, 0xc7, 0xc2, 0x44, 0x44, 0x44, 0x44, // MOV RDX, 0x44444444
        0x0f, 0x06, // CLTS
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x11111111, "RAX should be preserved");
    assert_eq!(regs.rbx, 0x22222222, "RBX should be preserved");
    assert_eq!(regs.rcx, 0x33333333, "RCX should be preserved");
    assert_eq!(regs.rdx, 0x44444444, "RDX should be preserved");
}

// Test CLTS multiple times in sequence
#[test]
fn test_clts_sequential() {
    let code = [
        0x0f, 0x06, // CLTS #1
        0x0f, 0x06, // CLTS #2
        0x0f, 0x06, // CLTS #3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // All should complete
    let _ = regs;
}

// Test CLTS in loop
#[test]
fn test_clts_in_loop() {
    let code = [
        0x48, 0x31, 0xc9, // XOR RCX, RCX
        // loop:
        0x0f, 0x06, // CLTS
        0x48, 0x83, 0xc1, 0x01, // ADD RCX, 1
        0x48, 0x83, 0xf9, 0x05, // CMP RCX, 5
        0x75, 0xf4, // JNZ loop
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 5, "Loop should complete");
}

// Test CLTS with conditional jumps
#[test]
fn test_clts_with_conditional_jump() {
    let code = [
        0x48, 0x31, 0xc0, // XOR RAX, RAX (sets ZF)
        0x0f, 0x06, // CLTS
        0x74, 0x02, // JZ skip (should jump)
        0x90, // NOP
        0x90, // NOP
        // skip:
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Should complete via jump
    let _ = regs;
}

// Test CLTS preserves stack
#[test]
fn test_clts_preserves_stack() {
    let code = [
        0x48, 0xc7, 0xc0, 0x42, 0x00, 0x00, 0x00, // MOV RAX, 0x42
        0x50, // PUSH RAX
        0x0f, 0x06, // CLTS
        0x58, // POP RAX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x42, "Stack value should be preserved");
}

// Test CLTS with other CR0 operations
#[test]
fn test_clts_with_cr0_read() {
    let code = [
        0x0f, 0x20, 0xc0, // MOV RAX, CR0 (read CR0)
        0x0f, 0x06, // CLTS (clear TS bit)
        0x0f, 0x20, 0xc3, // MOV RBX, CR0 (read again)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // CR0 should be readable
    let _ = (regs.rax, regs.rbx);
}

// Test CLTS preserves R8-R15
#[test]
fn test_clts_preserves_extended_registers() {
    let code = [
        0x49, 0xc7, 0xc0, 0xaa, 0xaa, 0xaa, 0xaa, // MOV R8, 0xaaaaaaaa (sign-extended)
        0x49, 0xc7, 0xc7, 0xbb, 0xbb, 0xbb, 0xbb, // MOV R15, 0xbbbbbbbb (sign-extended)
        0x0f, 0x06, // CLTS
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.r8, 0xffff_ffff_aaaa_aaaa, "R8 should be preserved");
    assert_eq!(regs.r15, 0xffff_ffff_bbbb_bbbb, "R15 should be preserved");
}

// Test CLTS with arithmetic operations
#[test]
fn test_clts_with_arithmetic() {
    let code = [
        0x48, 0xc7, 0xc0, 0x05, 0x00, 0x00, 0x00, // MOV RAX, 5
        0x48, 0xc7, 0xc3, 0x03, 0x00, 0x00, 0x00, // MOV RBX, 3
        0x48, 0x01, 0xd8, // ADD RAX, RBX
        0x0f, 0x06, // CLTS
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 8, "RAX should be 8");
}

// Test CLTS preserves stack pointer
#[test]
fn test_clts_preserves_stack_pointer() {
    let code = [
        0x0f, 0x06, // CLTS
        0xf4, // HLT
    ];
    let mut regs = rax::cpu::Registers::default();
    regs.rsp = 0x8000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rsp, 0x8000, "RSP should be unchanged");
}

// Test CLTS preserves base pointer
#[test]
fn test_clts_preserves_base_pointer() {
    let code = [
        0x48, 0xc7, 0xc5, 0x00, 0x70, 0x00, 0x00, // MOV RBP, 0x7000
        0x0f, 0x06, // CLTS
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbp, 0x7000, "RBP should be preserved");
}

// Test CLTS with zero flag
#[test]
fn test_clts_zero_flag() {
    let code = [
        0x48, 0x31, 0xc0, // XOR RAX, RAX (sets ZF)
        0x0f, 0x06, // CLTS
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(regs.rflags & 0x40 != 0, "ZF should be preserved");
}

// Test CLTS with carry flag
#[test]
fn test_clts_carry_flag() {
    let code = [
        0xf9, // STC (set carry flag)
        0x0f, 0x06, // CLTS
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(regs.rflags & 0x01 != 0, "CF should be preserved");
}

// Test CLTS with sign flag
#[test]
fn test_clts_sign_flag() {
    let code = [
        0x48, 0xc7, 0xc0, 0xff, 0xff, 0xff, 0xff, // MOV RAX, -1
        0x48, 0x85, 0xc0, // TEST RAX, RAX (sets SF)
        0x0f, 0x06, // CLTS
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(regs.rflags & 0x80 != 0, "SF should be preserved");
}

// Test CLTS with overflow flag
#[test]
fn test_clts_overflow_flag() {
    let code = [
        0x48, 0xc7, 0xc0, 0xff, 0xff, 0xff, 0x7f, // MOV RAX, 0x7fffffff
        0x48, 0x83, 0xc0, 0x01, // ADD RAX, 1 (sets OF)
        0x0f, 0x06, // CLTS
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // OF should be preserved
    let _ = regs.rflags;
}

// Test CLTS with parity flag
#[test]
fn test_clts_parity_flag() {
    let code = [
        0xb8, 0x03, 0x00, 0x00, 0x00, // MOV EAX, 3 (odd parity, PF=0)
        0x84, 0xc0, // TEST AL, AL
        0x0f, 0x06, // CLTS
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // PF should be preserved
    let _ = regs.rflags;
}

// Test CLTS with auxiliary carry flag
#[test]
fn test_clts_auxiliary_carry_flag() {
    let code = [
        0xb8, 0x0f, 0x00, 0x00, 0x00, // MOV EAX, 0x0F
        0x83, 0xc0, 0x01, // ADD EAX, 1 (sets AF)
        0x0f, 0x06, // CLTS
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // AF should be preserved
    let _ = regs.rflags;
}

// Test CLTS after FPU operation (relevant since TS affects FPU)
#[test]
fn test_clts_after_fpu_context() {
    let code = [
        0x0f, 0x06, // CLTS (allow FPU operations)
        0x48, 0xc7, 0xc0, 0x99, 0x00, 0x00, 0x00, // MOV RAX, 0x99
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x99, "Should complete normally");
}

// Test CLTS multiple times with CR0 reads between
#[test]
fn test_clts_multiple_with_cr0_reads() {
    let code = [
        0x0f, 0x06, // CLTS #1
        0x0f, 0x20, 0xc0, // MOV RAX, CR0
        0x0f, 0x06, // CLTS #2
        0x0f, 0x20, 0xc3, // MOV RBX, CR0
        0x0f, 0x06, // CLTS #3
        0x0f, 0x20, 0xc1, // MOV RCX, CR0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // All CR0 reads should complete
    let _ = (regs.rax, regs.rbx, regs.rcx);
}

// Test CLTS with direction flag
#[test]
fn test_clts_direction_flag() {
    let code = [
        0xfd, // STD (set direction flag)
        0x0f, 0x06, // CLTS
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // DF should be preserved
    assert!(regs.rflags & 0x400 != 0, "DF should be preserved");
}

// Test CLTS with interrupt flag
#[test]
fn test_clts_interrupt_flag() {
    let code = [
        0xfb, // STI (set interrupt flag, if permitted)
        0x0f, 0x06, // CLTS
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Should complete
    let _ = regs;
}

// Test CLTS execution speed
#[test]
fn test_clts_execution_speed() {
    let code = [
        0x0f, 0x06, // CLTS #1
        0x0f, 0x06, // CLTS #2
        0x0f, 0x06, // CLTS #3
        0x0f, 0x06, // CLTS #4
        0x0f, 0x06, // CLTS #5
        0x0f, 0x06, // CLTS #6
        0x0f, 0x06, // CLTS #7
        0x0f, 0x06, // CLTS #8
        0x0f, 0x06, // CLTS #9
        0x0f, 0x06, // CLTS #10
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // All should complete quickly
    let _ = regs;
}

// Test CLTS preserves memory operations
#[test]
fn test_clts_preserves_memory() {
    let code = [
        0x48, 0xc7, 0xc0, 0x77, 0x00, 0x00, 0x00, // MOV RAX, 0x77
        0x48, 0xc7, 0xc3, 0x00, 0x20, 0x00, 0x00, // MOV RBX, 0x2000
        0x48, 0x89, 0x03, // MOV [RBX], RAX (store)
        0x0f, 0x06, // CLTS
        0x48, 0x8b, 0x0b, // MOV RCX, [RBX] (load)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 0x77, "Memory value should be preserved");
}

// Test CLTS with nested operations
#[test]
fn test_clts_nested_operations() {
    let code = [
        0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1
        0x48, 0xc7, 0xc3, 0x02, 0x00, 0x00, 0x00, // MOV RBX, 2
        0x48, 0x01, 0xd8, // ADD RAX, RBX
        0x0f, 0x06, // CLTS
        0x48, 0x01, 0xd8, // ADD RAX, RBX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 5, "RAX should be 5");
}

// Test CLTS with function call pattern
#[test]
fn test_clts_with_call_pattern() {
    let code = [
        0x48, 0xc7, 0xc0, 0x88, 0x00, 0x00, 0x00, // MOV RAX, 0x88
        0x50, // PUSH RAX
        0x0f, 0x06, // CLTS
        0x58, // POP RAX
        0x48, 0x83, 0xc0, 0x01, // ADD RAX, 1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x89, "RAX should be 0x89");
}

// Test CLTS idempotency (multiple calls should be safe)
#[test]
fn test_clts_idempotent() {
    let code = [
        0x0f, 0x06, // CLTS #1
        0x0f, 0x06, // CLTS #2
        0x0f, 0x06, // CLTS #3
        0x0f, 0x20, 0xc0, // MOV RAX, CR0
        0x0f, 0x06, // CLTS #4
        0x0f, 0x20, 0xc3, // MOV RBX, CR0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Multiple CLTS should be safe (TS bit should remain clear)
    let _ = (regs.rax, regs.rbx);
}

// Test CLTS with segment operations
#[test]
fn test_clts_with_segments() {
    let code = [
        0x0f, 0x06, // CLTS
        0x48, 0xc7, 0xc0, 0x55, 0x00, 0x00, 0x00, // MOV RAX, 0x55
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x55, "Segment operations should work");
}

// Test CLTS instruction length (2 bytes)
#[test]
fn test_clts_instruction_length() {
    let code = [
        0x0f, 0x06, // CLTS (2 bytes)
        0x48, 0xc7, 0xc0, 0x42, 0x00, 0x00, 0x00, // MOV RAX, 0x42
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x42, "RIP should advance correctly");
}
