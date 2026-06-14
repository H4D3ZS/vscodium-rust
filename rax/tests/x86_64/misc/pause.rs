// Module path for tests run via x86_64.rs
use crate::common::*;
use rax::cpu::Registers;

// PAUSE - Spin Loop Hint
// Improves the performance of spin-wait loops by providing a hint to the processor.
// The PAUSE instruction provides a hint to the processor that the code sequence is a
// spin-wait loop, which helps avoid memory order violations and reduces power consumption.
//
// Opcode: F3 90
//
// The PAUSE instruction does not change the architectural state of the processor
// (performs essentially a delaying no-op operation). It does not modify any registers,
// memory, or flags.

#[test]
fn test_pause_basic() {
    // Basic PAUSE instruction
    let code = [
        0xf3, 0x90, // PAUSE
        0xf4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // PAUSE should complete without error
    assert_eq!(regs.rax, 0, "RAX should be unchanged (default 0)");
}

#[test]
fn test_pause_preserves_rax() {
    // PAUSE should not modify RAX
    let code = [
        0xf3, 0x90, // PAUSE
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x1234567890ABCDEF;

    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let final_regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(final_regs.rax, 0x1234567890ABCDEF, "RAX unchanged");
}

#[test]
fn test_pause_preserves_all_registers() {
    // PAUSE should not modify any registers
    let code = [
        0xf3, 0x90, // PAUSE
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x1111111111111111;
    regs.rbx = 0x2222222222222222;
    regs.rcx = 0x3333333333333333;
    regs.rdx = 0x4444444444444444;
    regs.rsi = 0x5555555555555555;
    regs.rdi = 0x6666666666666666;
    regs.rbp = 0x7777777777777777;
    regs.r8 = 0x8888888888888888;
    regs.r9 = 0x9999999999999999;
    regs.r10 = 0xAAAAAAAAAAAAAAAA;
    regs.r11 = 0xBBBBBBBBBBBBBBBB;
    regs.r12 = 0xCCCCCCCCCCCCCCCC;
    regs.r13 = 0xDDDDDDDDDDDDDDDD;
    regs.r14 = 0xEEEEEEEEEEEEEEEE;
    regs.r15 = 0xFFFFFFFFFFFFFFFF;

    let (mut vcpu, _) = setup_vm(&code, Some(regs.clone()));
    let final_regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(final_regs.rax, regs.rax, "RAX unchanged");
    assert_eq!(final_regs.rbx, regs.rbx, "RBX unchanged");
    assert_eq!(final_regs.rcx, regs.rcx, "RCX unchanged");
    assert_eq!(final_regs.rdx, regs.rdx, "RDX unchanged");
    assert_eq!(final_regs.rsi, regs.rsi, "RSI unchanged");
    assert_eq!(final_regs.rdi, regs.rdi, "RDI unchanged");
    assert_eq!(final_regs.rbp, regs.rbp, "RBP unchanged");
    assert_eq!(final_regs.r8, regs.r8, "R8 unchanged");
    assert_eq!(final_regs.r9, regs.r9, "R9 unchanged");
    assert_eq!(final_regs.r10, regs.r10, "R10 unchanged");
    assert_eq!(final_regs.r11, regs.r11, "R11 unchanged");
    assert_eq!(final_regs.r12, regs.r12, "R12 unchanged");
    assert_eq!(final_regs.r13, regs.r13, "R13 unchanged");
    assert_eq!(final_regs.r14, regs.r14, "R14 unchanged");
    assert_eq!(final_regs.r15, regs.r15, "R15 unchanged");
}

#[test]
fn test_pause_preserves_flags() {
    // PAUSE should not modify any flags
    let code = [
        0xf3, 0x90, // PAUSE
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rflags = 0x2 | 0x1 | (1 << 2) | (1 << 4) | (1 << 6) | (1 << 7) | (1 << 11);
    let initial_flags = regs.rflags;

    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let final_regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(final_regs.rflags, initial_flags, "Flags unchanged");
}

#[test]
fn test_pause_multiple_consecutive() {
    // Multiple consecutive PAUSE instructions
    let code = [
        0xf3, 0x90, // PAUSE
        0xf3, 0x90, // PAUSE
        0xf3, 0x90, // PAUSE
        0xf3, 0x90, // PAUSE
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x42;

    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let final_regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(final_regs.rax, 0x42, "RAX unchanged after multiple PAUSE");
}

#[test]
fn test_pause_in_spin_wait_pattern() {
    // PAUSE in a typical spin-wait pattern (simulated)
    // This simulates: while (flag == 0) { PAUSE; }
    let code = [
        0xb8, 0x00, 0x00, 0x00, 0x00, // MOV EAX, 0
        0xf3, 0x90, // PAUSE
        0x83, 0xc0, 0x01, // ADD EAX, 1 (exit condition)
        0xf4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 1, "Loop should exit with EAX = 1");
}

#[test]
fn test_pause_does_not_affect_memory() {
    // PAUSE should not access or modify memory
    let code = [
        0xf3, 0x90, // PAUSE
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    // Write a value to memory before PAUSE
    write_mem_u64(&mem, 0x123456789ABCDEF0);

    let _regs = run_until_hlt(&mut vcpu).unwrap();

    // Verify memory unchanged
    let mem_val = read_mem_u64(&mem);
    assert_eq!(mem_val, 0x123456789ABCDEF0, "Memory should be unchanged");
}

#[test]
fn test_pause_after_arithmetic() {
    // PAUSE after arithmetic operation preserves flags
    let code = [
        0xb8, 0xff, 0xff, 0xff, 0xff, // MOV EAX, 0xFFFFFFFF
        0x83, 0xc0, 0x01, // ADD EAX, 1 (sets CF, ZF)
        0xf3, 0x90, // PAUSE
        0xf4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 0, "EAX should wrap to 0");
    assert!(cf_set(regs.rflags), "CF should remain set after PAUSE");
    assert!(zf_set(regs.rflags), "ZF should remain set after PAUSE");
}

#[test]
fn test_pause_between_instructions() {
    // PAUSE between two MOV instructions
    let code = [
        0xb8, 0x11, 0x00, 0x00, 0x00, // MOV EAX, 0x11
        0xf3, 0x90, // PAUSE
        0xbb, 0x22, 0x00, 0x00, 0x00, // MOV EBX, 0x22
        0xf4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 0x11, "EAX should be 0x11");
    assert_eq!(regs.rbx & 0xFFFFFFFF, 0x22, "EBX should be 0x22");
}

#[test]
fn test_pause_preserves_stack_pointer() {
    // PAUSE should not modify stack pointer
    let code = [
        0xf3, 0x90, // PAUSE
        0xf4,
    ];
    let mut regs = Registers::default();
    let initial_rsp = STACK_ADDR;
    regs.rsp = initial_rsp;

    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let final_regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(final_regs.rsp, initial_rsp, "RSP unchanged");
}

#[test]
fn test_pause_with_zero_registers() {
    // PAUSE with all registers at 0
    let code = [
        0xf3, 0x90, // PAUSE
        0xf4,
    ];
    let regs = Registers::default();

    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let final_regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(final_regs.rax, 0, "RAX should be 0");
    assert_eq!(final_regs.rbx, 0, "RBX should be 0");
    assert_eq!(final_regs.rcx, 0, "RCX should be 0");
}

#[test]
fn test_pause_with_max_register_values() {
    // PAUSE with maximum register values
    let code = [
        0xf3, 0x90, // PAUSE
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0xFFFFFFFFFFFFFFFF;
    regs.rbx = 0xFFFFFFFFFFFFFFFF;

    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let final_regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(final_regs.rax, 0xFFFFFFFFFFFFFFFF, "RAX unchanged");
    assert_eq!(final_regs.rbx, 0xFFFFFFFFFFFFFFFF, "RBX unchanged");
}

#[test]
fn test_pause_after_conditional_jump() {
    // PAUSE after a conditional jump
    let code = [
        0xb8, 0x01, 0x00, 0x00, 0x00, // MOV EAX, 1
        0x85, 0xc0, // TEST EAX, EAX
        0x75, 0x05, // JNZ +5 (skip next instruction)
        0xb8, 0xff, 0x00, 0x00, 0x00, // MOV EAX, 0xFF (skipped)
        0xf3, 0x90, // PAUSE
        0xf4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 1, "EAX should still be 1");
}

#[test]
fn test_pause_before_loop_condition_check() {
    // PAUSE before checking loop condition (typical usage)
    let code = [
        0xb9, 0x05, 0x00, 0x00, 0x00, // MOV ECX, 5 (loop counter)
        0xf3, 0x90, // PAUSE
        0x83, 0xe9, 0x01, // SUB ECX, 1
        0xf4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx & 0xFFFFFFFF, 4, "ECX should be decremented to 4");
}

#[test]
fn test_pause_with_carry_flag_set() {
    // PAUSE with carry flag set
    let code = [
        0xf9, // STC (set carry flag)
        0xf3, 0x90, // PAUSE
        0xf4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags), "CF should remain set");
}

#[test]
fn test_pause_with_zero_flag_set() {
    // PAUSE with zero flag set
    let code = [
        0x31, 0xc0, // XOR EAX, EAX (sets ZF)
        0xf3, 0x90, // PAUSE
        0xf4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(zf_set(regs.rflags), "ZF should remain set");
    assert_eq!(regs.rax, 0, "RAX should be 0");
}

#[test]
fn test_pause_with_sign_flag_set() {
    // PAUSE with sign flag set
    let code = [
        0xb8, 0x00, 0x00, 0x00, 0x80, // MOV EAX, 0x80000000 (negative in 32-bit)
        0x85, 0xc0, // TEST EAX, EAX (sets SF)
        0xf3, 0x90, // PAUSE
        0xf4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(sf_set(regs.rflags), "SF should remain set");
}

#[test]
fn test_pause_with_overflow_flag_set() {
    // PAUSE with overflow flag set
    let code = [
        0xb8, 0xff, 0xff, 0xff, 0x7f, // MOV EAX, 0x7FFFFFFF
        0x83, 0xc0, 0x01, // ADD EAX, 1 (sets OF)
        0xf3, 0x90, // PAUSE
        0xf4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(of_set(regs.rflags), "OF should remain set");
}

#[test]
fn test_pause_after_stack_push() {
    // PAUSE after stack push
    let code = [
        0xb8, 0x42, 0x00, 0x00, 0x00, // MOV EAX, 0x42
        0x50, // PUSH RAX
        0xf3, 0x90, // PAUSE
        0x58, // POP RAX
        0xf4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x42, "RAX should be restored");
}

#[test]
fn test_pause_in_mixed_instruction_sequence() {
    // PAUSE mixed with various instructions
    let code = [
        0xb8, 0x01, 0x00, 0x00, 0x00, // MOV EAX, 1
        0xbb, 0x02, 0x00, 0x00, 0x00, // MOV EBX, 2
        0xf3, 0x90, // PAUSE
        0x01, 0xd8, // ADD EAX, EBX
        0xf3, 0x90, // PAUSE
        0xf4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 3, "EAX should be 3");
}

#[test]
fn test_pause_does_not_cause_exceptions() {
    // PAUSE should execute without causing exceptions
    let code = [
        0xf3, 0x90, // PAUSE
        0xf4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    // Should complete without error
    let result = run_until_hlt(&mut vcpu);
    assert!(result.is_ok(), "PAUSE should not cause exceptions");
}

#[test]
fn test_pause_preserves_direction_flag() {
    // PAUSE with direction flag set
    let code = [
        0xfd, // STD (set direction flag)
        0xf3, 0x90, // PAUSE
        0xf4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(df_set(regs.rflags), "DF should remain set");
}

#[test]
fn test_pause_as_nop_equivalent() {
    // PAUSE acts as a NOP (no operation) in terms of architectural state
    let code = [
        0xb8, 0x55, 0x00, 0x00, 0x00, // MOV EAX, 0x55
        0xf3, 0x90, // PAUSE
        0xbb, 0xAA, 0x00, 0x00, 0x00, // MOV EBX, 0xAA
        0xf4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 0x55, "EAX should be 0x55");
    assert_eq!(regs.rbx & 0xFFFFFFFF, 0xAA, "EBX should be 0xAA");
}

#[test]
fn test_pause_before_memory_read() {
    // PAUSE before memory read (typical in synchronization)
    let code = [
        0xf3, 0x90, // PAUSE
        0x48, 0xa1, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV RAX, [0x2000]
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    // Set up memory value
    write_mem_u64(&mem, 0xDEADBEEFCAFEBABE);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax, 0xDEADBEEFCAFEBABE,
        "RAX should contain memory value"
    );
}

#[test]
fn test_pause_intensive_loop() {
    // Multiple PAUSE instructions simulating intensive spin-wait
    let code = [
        0xf3, 0x90, // PAUSE
        0xf3, 0x90, // PAUSE
        0xf3, 0x90, // PAUSE
        0xf3, 0x90, // PAUSE
        0xf3, 0x90, // PAUSE
        0xf3, 0x90, // PAUSE
        0xf3, 0x90, // PAUSE
        0xf3, 0x90, // PAUSE
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x123456789ABCDEF0;

    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let final_regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        final_regs.rax, 0x123456789ABCDEF0,
        "RAX unchanged after many PAUSE"
    );
}

#[test]
fn test_pause_with_base_pointer() {
    // PAUSE should preserve base pointer
    let code = [
        0xf3, 0x90, // PAUSE
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbp = 0x7000;

    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let final_regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(final_regs.rbp, 0x7000, "RBP unchanged");
}
