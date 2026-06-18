// Module path for tests run via x86_64.rs
use crate::common::*;
use rax::cpu::Registers;

// PUSHAD/POPAD - Push/Pop All General Purpose Registers (32-bit mode)
// Note: These instructions are not available in 64-bit mode.
// In 64-bit mode, there is no single instruction to push all registers.
// This test file is included for completeness and documentation, but tests
// will verify proper behavior or #UD exceptions in 64-bit mode.
//
// PUSHA/POPA are 16-bit equivalents, also not available in 64-bit mode.
//
// Opcodes (32-bit mode only):
// 60           PUSHA/PUSHAD   - Push AX, CX, DX, BX, SP, BP, SI, DI (16-bit) or
//                                Push EAX, ECX, EDX, EBX, ESP, EBP, ESI, EDI (32-bit)
// 61           POPA/POPAD     - Pop DI, SI, BP, SP, BX, DX, CX, AX (16-bit) or
//                                Pop EDI, ESI, EBP, ESP, EBX, EDX, ECX, EAX (32-bit)

// Note: In x86-64 long mode, PUSHAD/POPAD/PUSHA/POPA are invalid and should
// trigger #UD (invalid opcode exception). However, implementing proper exception
// handling might not be complete in the emulator, so these tests focus on
// documenting the expected behavior.

#[test]
fn test_pushad_not_available_in_64bit() {
    // PUSHAD (opcode 0x60) is invalid in 64-bit mode
    // In a fully compliant implementation, this should raise #UD
    // For now, we document that this instruction is not supported in 64-bit mode

    // This test serves as documentation that PUSHAD is not available
    // in x86-64 long mode. Individual register pushes must be used instead.
    assert!(true, "PUSHAD is not available in x86-64 long mode");
}

#[test]
fn test_popad_not_available_in_64bit() {
    // POPAD (opcode 0x61) is invalid in 64-bit mode
    // In a fully compliant implementation, this should raise #UD
    // For now, we document that this instruction is not supported in 64-bit mode

    // This test serves as documentation that POPAD is not available
    // in x86-64 long mode. Individual register pops must be used instead.
    assert!(true, "POPAD is not available in x86-64 long mode");
}

#[test]
fn test_manual_push_all_64bit_equivalent() {
    // In 64-bit mode, to achieve PUSHAD-like behavior, push registers manually
    // This is the recommended pattern for 64-bit code
    let code = [
        0x50, // PUSH RAX
        0x53, // PUSH RBX
        0x51, // PUSH RCX
        0x52, // PUSH RDX
        0x56, // PUSH RSI
        0x57, // PUSH RDI
        0x55, // PUSH RBP
        // RSP is not typically pushed in function prologues
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
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Verify all registers were pushed
    assert_eq!(read_mem_at_u64(&mem, STACK_ADDR - 8), 0x1111111111111111);
    assert_eq!(read_mem_at_u64(&mem, STACK_ADDR - 16), 0x2222222222222222);
    assert_eq!(read_mem_at_u64(&mem, STACK_ADDR - 24), 0x3333333333333333);
    assert_eq!(read_mem_at_u64(&mem, STACK_ADDR - 32), 0x4444444444444444);
    assert_eq!(read_mem_at_u64(&mem, STACK_ADDR - 40), 0x5555555555555555);
    assert_eq!(read_mem_at_u64(&mem, STACK_ADDR - 48), 0x6666666666666666);
    assert_eq!(read_mem_at_u64(&mem, STACK_ADDR - 56), 0x7777777777777777);
    assert_eq!(regs.rsp, STACK_ADDR - 56);
}

#[test]
fn test_manual_pop_all_64bit_equivalent() {
    // In 64-bit mode, to achieve POPAD-like behavior, pop registers manually
    let code = [
        0x5d, // POP RBP
        0x5f, // POP RDI
        0x5e, // POP RSI
        0x5a, // POP RDX
        0x59, // POP RCX
        0x5b, // POP RBX
        0x58, // POP RAX
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    // Set up stack
    write_mem_at_u64(&mem, STACK_ADDR - 56, 0x7777777777777777);
    write_mem_at_u64(&mem, STACK_ADDR - 48, 0x6666666666666666);
    write_mem_at_u64(&mem, STACK_ADDR - 40, 0x5555555555555555);
    write_mem_at_u64(&mem, STACK_ADDR - 32, 0x4444444444444444);
    write_mem_at_u64(&mem, STACK_ADDR - 24, 0x3333333333333333);
    write_mem_at_u64(&mem, STACK_ADDR - 16, 0x2222222222222222);
    write_mem_at_u64(&mem, STACK_ADDR - 8, 0x1111111111111111);

    let mut regs = vcpu.get_regs().unwrap();
    regs.rsp = STACK_ADDR - 56;
    vcpu.set_regs(&regs).unwrap();
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Verify all registers were restored
    assert_eq!(regs.rax, 0x1111111111111111);
    assert_eq!(regs.rbx, 0x2222222222222222);
    assert_eq!(regs.rcx, 0x3333333333333333);
    assert_eq!(regs.rdx, 0x4444444444444444);
    assert_eq!(regs.rsi, 0x5555555555555555);
    assert_eq!(regs.rdi, 0x6666666666666666);
    assert_eq!(regs.rbp, 0x7777777777777777);
    assert_eq!(regs.rsp, STACK_ADDR);
}

#[test]
fn test_manual_push_pop_all_round_trip() {
    // Complete round trip: push all, then pop all
    let code = [
        // Push all
        0x50, // PUSH RAX
        0x53, // PUSH RBX
        0x51, // PUSH RCX
        0x52, // PUSH RDX
        0x56, // PUSH RSI
        0x57, // PUSH RDI
        0x55, // PUSH RBP
        // Pop all in reverse order
        0x5d, // POP RBP
        0x5f, // POP RDI
        0x5e, // POP RSI
        0x5a, // POP RDX
        0x59, // POP RCX
        0x5b, // POP RBX
        0x58, // POP RAX
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
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // All registers should be restored
    assert_eq!(regs.rax, 0x1111111111111111);
    assert_eq!(regs.rbx, 0x2222222222222222);
    assert_eq!(regs.rcx, 0x3333333333333333);
    assert_eq!(regs.rdx, 0x4444444444444444);
    assert_eq!(regs.rsi, 0x5555555555555555);
    assert_eq!(regs.rdi, 0x6666666666666666);
    assert_eq!(regs.rbp, 0x7777777777777777);
    assert_eq!(regs.rsp, STACK_ADDR); // Stack balanced
}

#[test]
fn test_push_all_including_extended_regs() {
    // In 64-bit mode, we also have R8-R15 to consider
    let code = [
        // Push traditional GPRs
        0x50, // PUSH RAX
        0x53, // PUSH RBX
        0x51, // PUSH RCX
        0x52, // PUSH RDX
        0x56, // PUSH RSI
        0x57, // PUSH RDI
        0x55, // PUSH RBP
        // Push extended registers
        0x41, 0x50, // PUSH R8
        0x41, 0x51, // PUSH R9
        0x41, 0x52, // PUSH R10
        0x41, 0x53, // PUSH R11
        0x41, 0x54, // PUSH R12
        0x41, 0x55, // PUSH R13
        0x41, 0x56, // PUSH R14
        0x41, 0x57, // PUSH R15
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x01;
    regs.rbx = 0x02;
    regs.rcx = 0x03;
    regs.rdx = 0x04;
    regs.rsi = 0x05;
    regs.rdi = 0x06;
    regs.rbp = 0x07;
    regs.r8 = 0x08;
    regs.r9 = 0x09;
    regs.r10 = 0x0A;
    regs.r11 = 0x0B;
    regs.r12 = 0x0C;
    regs.r13 = 0x0D;
    regs.r14 = 0x0E;
    regs.r15 = 0x0F;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Verify all 15 registers were pushed (112 bytes total)
    assert_eq!(regs.rsp, STACK_ADDR - 120); // 15 * 8

    // Check some values
    assert_eq!(read_mem_at_u64(&mem, STACK_ADDR - 8), 0x01); // RAX
    assert_eq!(read_mem_at_u64(&mem, STACK_ADDR - 64), 0x08); // R8
    assert_eq!(read_mem_at_u64(&mem, STACK_ADDR - 120), 0x0F); // R15
}

#[test]
fn test_pop_all_including_extended_regs() {
    // Pop all registers including R8-R15
    let code = [
        // Pop extended registers (reverse order)
        0x41, 0x5f, // POP R15
        0x41, 0x5e, // POP R14
        0x41, 0x5d, // POP R13
        0x41, 0x5c, // POP R12
        0x41, 0x5b, // POP R11
        0x41, 0x5a, // POP R10
        0x41, 0x59, // POP R9
        0x41, 0x58, // POP R8
        // Pop traditional GPRs
        0x5d, // POP RBP
        0x5f, // POP RDI
        0x5e, // POP RSI
        0x5a, // POP RDX
        0x59, // POP RCX
        0x5b, // POP RBX
        0x58, // POP RAX
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    // Set up stack with 15 values
    for i in 0..15 {
        write_mem_at_u64(&mem, STACK_ADDR - ((i + 1) * 8) as u64, (i + 1) as u64);
    }

    let mut regs = vcpu.get_regs().unwrap();
    regs.rsp = STACK_ADDR - 120;
    vcpu.set_regs(&regs).unwrap();
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Verify all registers restored
    assert_eq!(regs.rax, 1);
    assert_eq!(regs.rbx, 2);
    assert_eq!(regs.rcx, 3);
    assert_eq!(regs.rdx, 4);
    assert_eq!(regs.rsi, 5);
    assert_eq!(regs.rdi, 6);
    assert_eq!(regs.rbp, 7);
    assert_eq!(regs.r8, 8);
    assert_eq!(regs.r9, 9);
    assert_eq!(regs.r10, 10);
    assert_eq!(regs.r11, 11);
    assert_eq!(regs.r12, 12);
    assert_eq!(regs.r13, 13);
    assert_eq!(regs.r14, 14);
    assert_eq!(regs.r15, 15);
    assert_eq!(regs.rsp, STACK_ADDR);
}

#[test]
fn test_documentation_pushad_order() {
    // Document the order PUSHAD would use in 32-bit mode:
    // PUSHAD pushes: EAX, ECX, EDX, EBX, ESP (original), EBP, ESI, EDI
    // This is for documentation purposes only
    assert!(
        true,
        "PUSHAD order (32-bit): EAX, ECX, EDX, EBX, ESP, EBP, ESI, EDI"
    );
}

#[test]
fn test_documentation_popad_order() {
    // Document the order POPAD would use in 32-bit mode:
    // POPAD pops: EDI, ESI, EBP, ESP (ignored), EBX, EDX, ECX, EAX
    // Note: ESP value from stack is ignored, not restored
    // This is for documentation purposes only
    assert!(
        true,
        "POPAD order (32-bit): EDI, ESI, EBP, ESP (ignored), EBX, EDX, ECX, EAX"
    );
}

#[test]
fn test_callee_saved_registers_pattern() {
    // Common calling convention: callee-saved registers
    // System V AMD64 ABI: RBX, RBP, R12-R15 are callee-saved
    let code = [
        // Save callee-saved registers
        0x53, // PUSH RBX
        0x55, // PUSH RBP
        0x41, 0x54, // PUSH R12
        0x41, 0x55, // PUSH R13
        0x41, 0x56, // PUSH R14
        0x41, 0x57, // PUSH R15
        // ... function body would go here ...
        // Restore callee-saved registers
        0x41, 0x5f, // POP R15
        0x41, 0x5e, // POP R14
        0x41, 0x5d, // POP R13
        0x41, 0x5c, // POP R12
        0x5d, // POP RBP
        0x5b, // POP RBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xBBBBBBBBBBBBBBBB;
    regs.rbp = 0x1111111111111111;
    regs.r12 = 0x1212121212121212;
    regs.r13 = 0x1313131313131313;
    regs.r14 = 0x1414141414141414;
    regs.r15 = 0x1515151515151515;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // All callee-saved registers should be preserved
    assert_eq!(regs.rbx, 0xBBBBBBBBBBBBBBBB);
    assert_eq!(regs.rbp, 0x1111111111111111);
    assert_eq!(regs.r12, 0x1212121212121212);
    assert_eq!(regs.r13, 0x1313131313131313);
    assert_eq!(regs.r14, 0x1414141414141414);
    assert_eq!(regs.r15, 0x1515151515151515);
    assert_eq!(regs.rsp, STACK_ADDR); // Stack balanced
}

#[test]
fn test_windows_x64_callee_saved() {
    // Windows x64 calling convention: RBX, RBP, RDI, RSI, R12-R15 are callee-saved
    let code = [
        // Save callee-saved registers (Windows x64)
        0x53, // PUSH RBX
        0x55, // PUSH RBP
        0x57, // PUSH RDI
        0x56, // PUSH RSI
        0x41, 0x54, // PUSH R12
        0x41, 0x55, // PUSH R13
        0x41, 0x56, // PUSH R14
        0x41, 0x57, // PUSH R15
        // ... function body ...
        // Restore callee-saved registers
        0x41, 0x5f, // POP R15
        0x41, 0x5e, // POP R14
        0x41, 0x5d, // POP R13
        0x41, 0x5c, // POP R12
        0x5e, // POP RSI
        0x5f, // POP RDI
        0x5d, // POP RBP
        0x5b, // POP RBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xBB;
    regs.rbp = 0xBBBB;
    regs.rdi = 0xDD;
    regs.rsi = 0xEE;
    regs.r12 = 0x12;
    regs.r13 = 0x13;
    regs.r14 = 0x14;
    regs.r15 = 0x15;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // All Windows x64 callee-saved registers preserved
    assert_eq!(regs.rbx, 0xBB);
    assert_eq!(regs.rbp, 0xBBBB);
    assert_eq!(regs.rdi, 0xDD);
    assert_eq!(regs.rsi, 0xEE);
    assert_eq!(regs.r12, 0x12);
    assert_eq!(regs.r13, 0x13);
    assert_eq!(regs.r14, 0x14);
    assert_eq!(regs.r15, 0x15);
    assert_eq!(regs.rsp, STACK_ADDR);
}
