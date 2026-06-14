// Module path for tests run via x86_64.rs
use crate::common::*;
use rax::cpu::Registers;

// POP - Pop from Stack (Comprehensive Extended Tests)
// Reads operand from stack at RSP and increments RSP by operand size.
// Tests cover all sizes (16/32/64-bit), all register types, and memory operands.
//
// Opcodes:
// 58+rd        POP r64            - Pop top of stack into r64
// 66 58+rw     POP r16            - Pop top of stack into r16
// 8F /0        POP r/m64          - Pop top of stack into r/m64

#[test]
fn test_pop_rax() {
    // POP RAX - Basic 64-bit register pop
    let code = [
        0x58, // POP RAX
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_at_u64(&mem, STACK_ADDR - 8, 0x123456789ABCDEF0);
    let mut regs = vcpu.get_regs().unwrap();
    regs.rsp = STACK_ADDR - 8;
    vcpu.set_regs(&regs).unwrap();
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x123456789ABCDEF0);
    assert_eq!(regs.rsp, STACK_ADDR);
}

#[test]
fn test_pop_rbx() {
    // POP RBX
    let code = [
        0x5b, // POP RBX
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_at_u64(&mem, STACK_ADDR - 8, 0xDEADBEEFCAFEBABE);
    let mut regs = vcpu.get_regs().unwrap();
    regs.rsp = STACK_ADDR - 8;
    vcpu.set_regs(&regs).unwrap();
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx, 0xDEADBEEFCAFEBABE);
    assert_eq!(regs.rsp, STACK_ADDR);
}

#[test]
fn test_pop_rcx() {
    // POP RCX
    let code = [
        0x59, // POP RCX
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_at_u64(&mem, STACK_ADDR - 8, 0x1122334455667788);
    let mut regs = vcpu.get_regs().unwrap();
    regs.rsp = STACK_ADDR - 8;
    vcpu.set_regs(&regs).unwrap();
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 0x1122334455667788);
    assert_eq!(regs.rsp, STACK_ADDR);
}

#[test]
fn test_pop_rdx() {
    // POP RDX
    let code = [
        0x5a, // POP RDX
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_at_u64(&mem, STACK_ADDR - 8, 0xAABBCCDDEEFF0011);
    let mut regs = vcpu.get_regs().unwrap();
    regs.rsp = STACK_ADDR - 8;
    vcpu.set_regs(&regs).unwrap();
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rdx, 0xAABBCCDDEEFF0011);
    assert_eq!(regs.rsp, STACK_ADDR);
}

#[test]
fn test_pop_rsi() {
    // POP RSI
    let code = [
        0x5e, // POP RSI
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_at_u64(&mem, STACK_ADDR - 8, 0xFEDCBA9876543210);
    let mut regs = vcpu.get_regs().unwrap();
    regs.rsp = STACK_ADDR - 8;
    vcpu.set_regs(&regs).unwrap();
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rsi, 0xFEDCBA9876543210);
    assert_eq!(regs.rsp, STACK_ADDR);
}

#[test]
fn test_pop_rdi() {
    // POP RDI
    let code = [
        0x5f, // POP RDI
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_at_u64(&mem, STACK_ADDR - 8, 0x0123456789ABCDEF);
    let mut regs = vcpu.get_regs().unwrap();
    regs.rsp = STACK_ADDR - 8;
    vcpu.set_regs(&regs).unwrap();
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rdi, 0x0123456789ABCDEF);
    assert_eq!(regs.rsp, STACK_ADDR);
}

#[test]
fn test_pop_rbp() {
    // POP RBP - Common in function epilogues
    let code = [
        0x5d, // POP RBP
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_at_u64(&mem, STACK_ADDR - 8, 0x1000000000000000);
    let mut regs = vcpu.get_regs().unwrap();
    regs.rsp = STACK_ADDR - 8;
    vcpu.set_regs(&regs).unwrap();
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbp, 0x1000000000000000);
    assert_eq!(regs.rsp, STACK_ADDR);
}

#[test]
fn test_pop_r8() {
    // POP R8 - Extended register
    let code = [
        0x41, 0x58, // POP R8
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_at_u64(&mem, STACK_ADDR - 8, 0x8888888888888888);
    let mut regs = vcpu.get_regs().unwrap();
    regs.rsp = STACK_ADDR - 8;
    vcpu.set_regs(&regs).unwrap();
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.r8, 0x8888888888888888);
    assert_eq!(regs.rsp, STACK_ADDR);
}

#[test]
fn test_pop_r15() {
    // POP R15 - Extended register
    let code = [
        0x41, 0x5f, // POP R15
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_at_u64(&mem, STACK_ADDR - 8, 0xFFFFFFFFFFFFFFFF);
    let mut regs = vcpu.get_regs().unwrap();
    regs.rsp = STACK_ADDR - 8;
    vcpu.set_regs(&regs).unwrap();
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.r15, 0xFFFFFFFFFFFFFFFF);
    assert_eq!(regs.rsp, STACK_ADDR);
}

#[test]
fn test_pop_all_extended_regs() {
    // Pop all extended registers R8-R15
    let code = [
        0x41, 0x5f, // POP R15
        0x41, 0x5e, // POP R14
        0x41, 0x5d, // POP R13
        0x41, 0x5c, // POP R12
        0x41, 0x5b, // POP R11
        0x41, 0x5a, // POP R10
        0x41, 0x59, // POP R9
        0x41, 0x58, // POP R8
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    // Set up stack with values (in reverse order since POP is LIFO)
    write_mem_at_u64(&mem, STACK_ADDR - 64, 15);
    write_mem_at_u64(&mem, STACK_ADDR - 56, 14);
    write_mem_at_u64(&mem, STACK_ADDR - 48, 13);
    write_mem_at_u64(&mem, STACK_ADDR - 40, 12);
    write_mem_at_u64(&mem, STACK_ADDR - 32, 11);
    write_mem_at_u64(&mem, STACK_ADDR - 24, 10);
    write_mem_at_u64(&mem, STACK_ADDR - 16, 9);
    write_mem_at_u64(&mem, STACK_ADDR - 8, 8);

    let mut regs = vcpu.get_regs().unwrap();
    regs.rsp = STACK_ADDR - 64;
    vcpu.set_regs(&regs).unwrap();
    let regs = run_until_hlt(&mut vcpu).unwrap();

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
fn test_pop_word() {
    // POP r16 - 16-bit register pop
    let code = [
        0x66, 0x58, // POP AX
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_at_u16(&mem, STACK_ADDR - 2, 0x1234);
    let mut regs = vcpu.get_regs().unwrap();
    regs.rsp = STACK_ADDR - 2;
    regs.rax = 0xFFFFFFFFFFFFFFFF; // Will be partially overwritten
    vcpu.set_regs(&regs).unwrap();
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFF, 0x1234);
    assert_eq!(regs.rsp, STACK_ADDR);
}

#[test]
fn test_pop_to_memory() {
    // POP to memory location
    let code = [
        0x8f, 0x03, // POP QWORD PTR [RBX]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = DATA_ADDR;
    regs.rsp = STACK_ADDR - 8;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    write_mem_at_u64(&mem, STACK_ADDR - 8, 0x123456789ABCDEF0);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rsp, STACK_ADDR);
    let value = read_mem_u64(&mem);
    assert_eq!(value, 0x123456789ABCDEF0);
}

#[test]
fn test_pop_multiple_registers() {
    // Multiple POP operations
    let code = [
        0x59, // POP RCX
        0x5b, // POP RBX
        0x58, // POP RAX
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    // Set up stack
    write_mem_at_u64(&mem, STACK_ADDR - 24, 0x3333333333333333);
    write_mem_at_u64(&mem, STACK_ADDR - 16, 0x2222222222222222);
    write_mem_at_u64(&mem, STACK_ADDR - 8, 0x1111111111111111);

    let mut regs = vcpu.get_regs().unwrap();
    regs.rsp = STACK_ADDR - 24;
    vcpu.set_regs(&regs).unwrap();
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 0x3333333333333333);
    assert_eq!(regs.rbx, 0x2222222222222222);
    assert_eq!(regs.rax, 0x1111111111111111);
    assert_eq!(regs.rsp, STACK_ADDR);
}

#[test]
fn test_push_pop_round_trip() {
    // PUSH then POP should restore original value
    let code = [
        0x50, // PUSH RAX
        0x58, // POP RAX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x123456789ABCDEF0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x123456789ABCDEF0);
    assert_eq!(regs.rsp, STACK_ADDR); // Stack pointer restored
}

#[test]
fn test_pop_increments_rsp_correctly() {
    // Verify RSP is incremented by 8 for each POP
    let code = [
        0x58, // POP RAX
        0x58, // POP RAX
        0x58, // POP RAX
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_at_u64(&mem, STACK_ADDR - 24, 0x1111);
    write_mem_at_u64(&mem, STACK_ADDR - 16, 0x2222);
    write_mem_at_u64(&mem, STACK_ADDR - 8, 0x3333);

    let mut regs = vcpu.get_regs().unwrap();
    regs.rsp = STACK_ADDR - 24;
    vcpu.set_regs(&regs).unwrap();
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rsp, STACK_ADDR);
}

#[test]
fn test_pop_all_gprs() {
    // Pop all general purpose registers
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
    write_mem_at_u64(&mem, STACK_ADDR - 56, 7);
    write_mem_at_u64(&mem, STACK_ADDR - 48, 6);
    write_mem_at_u64(&mem, STACK_ADDR - 40, 5);
    write_mem_at_u64(&mem, STACK_ADDR - 32, 4);
    write_mem_at_u64(&mem, STACK_ADDR - 24, 3);
    write_mem_at_u64(&mem, STACK_ADDR - 16, 2);
    write_mem_at_u64(&mem, STACK_ADDR - 8, 1);

    let mut regs = vcpu.get_regs().unwrap();
    regs.rsp = STACK_ADDR - 56;
    vcpu.set_regs(&regs).unwrap();
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 1);
    assert_eq!(regs.rbx, 2);
    assert_eq!(regs.rcx, 3);
    assert_eq!(regs.rdx, 4);
    assert_eq!(regs.rsi, 5);
    assert_eq!(regs.rdi, 6);
    assert_eq!(regs.rbp, 7);
    assert_eq!(regs.rsp, STACK_ADDR);
}

#[test]
fn test_pop_function_epilogue_pattern() {
    // Common function epilogue: mov rsp, rbp; pop rbp
    let code = [
        0x48, 0x89, 0xec, // MOV RSP, RBP
        0x5d, // POP RBP
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbp = STACK_ADDR - 8; // Frame pointer
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    write_mem_at_u64(&mem, STACK_ADDR - 8, 0x1000); // Saved RBP
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbp, 0x1000); // Restored RBP
    assert_eq!(regs.rsp, STACK_ADDR); // Stack cleaned up
}

#[test]
fn test_pop_zero_value() {
    // POP zero value
    let code = [
        0x58, // POP RAX
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_at_u64(&mem, STACK_ADDR - 8, 0);
    let mut regs = vcpu.get_regs().unwrap();
    regs.rsp = STACK_ADDR - 8;
    regs.rax = 0xFFFFFFFFFFFFFFFF;
    vcpu.set_regs(&regs).unwrap();
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0);
}

#[test]
fn test_pop_does_not_affect_flags() {
    // POP should not modify flags
    let code = [
        0x58, // POP RAX
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_at_u64(&mem, STACK_ADDR - 8, 0x123456789ABCDEF0);
    let mut regs = vcpu.get_regs().unwrap();
    regs.rsp = STACK_ADDR - 8;
    regs.rflags = 0x246; // Some flags set
    let initial_flags = regs.rflags;
    vcpu.set_regs(&regs).unwrap();
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rflags, initial_flags);
}

#[test]
fn test_pop_word_multiple() {
    // Multiple 16-bit pops
    let code = [
        0x66, 0x59, // POP CX
        0x66, 0x5b, // POP BX
        0x66, 0x58, // POP AX
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, STACK_ADDR - 6, 0x3333);
    write_mem_at_u16(&mem, STACK_ADDR - 4, 0x2222);
    write_mem_at_u16(&mem, STACK_ADDR - 2, 0x1111);

    let mut regs = vcpu.get_regs().unwrap();
    regs.rsp = STACK_ADDR - 6;
    vcpu.set_regs(&regs).unwrap();
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx & 0xFFFF, 0x3333);
    assert_eq!(regs.rbx & 0xFFFF, 0x2222);
    assert_eq!(regs.rax & 0xFFFF, 0x1111);
    assert_eq!(regs.rsp, STACK_ADDR);
}

#[test]
fn test_pop_preserves_upper_bits_word() {
    // 16-bit POP preserves upper 48 bits
    let code = [
        0x66, 0x58, // POP AX
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_at_u16(&mem, STACK_ADDR - 2, 0x1234);
    let mut regs = vcpu.get_regs().unwrap();
    regs.rsp = STACK_ADDR - 2;
    regs.rax = 0xFFFFFFFFFFFFFFFF;
    vcpu.set_regs(&regs).unwrap();
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFF, 0x1234);
    assert_eq!(regs.rax & 0xFFFFFFFFFFFF0000, 0xFFFFFFFFFFFF0000);
}

#[test]
fn test_pop_r12_special_encoding() {
    // R12 requires SIB byte in many contexts
    let code = [
        0x41, 0x5c, // POP R12
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_at_u64(&mem, STACK_ADDR - 8, 0xBBBBBBBBBBBBBBBB);
    let mut regs = vcpu.get_regs().unwrap();
    regs.rsp = STACK_ADDR - 8;
    vcpu.set_regs(&regs).unwrap();
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.r12, 0xBBBBBBBBBBBBBBBB);
}

#[test]
fn test_pop_r13_special_encoding() {
    // R13 requires displacement in many contexts
    let code = [
        0x41, 0x5d, // POP R13
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_at_u64(&mem, STACK_ADDR - 8, 0xCCCCCCCCCCCCCCCC);
    let mut regs = vcpu.get_regs().unwrap();
    regs.rsp = STACK_ADDR - 8;
    vcpu.set_regs(&regs).unwrap();
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.r13, 0xCCCCCCCCCCCCCCCC);
}

#[test]
fn test_push_pop_sequence() {
    // Full PUSH/POP sequence for register preservation
    let code = [
        0x50, // PUSH RAX
        0x53, // PUSH RBX
        0x51, // PUSH RCX
        // ... do work ...
        0x59, // POP RCX
        0x5b, // POP RBX
        0x58, // POP RAX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x1111111111111111;
    regs.rbx = 0x2222222222222222;
    regs.rcx = 0x3333333333333333;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x1111111111111111);
    assert_eq!(regs.rbx, 0x2222222222222222);
    assert_eq!(regs.rcx, 0x3333333333333333);
    assert_eq!(regs.rsp, STACK_ADDR); // Stack balanced
}

#[test]
fn test_pop_maximum_value() {
    // POP maximum 64-bit value
    let code = [
        0x58, // POP RAX
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_at_u64(&mem, STACK_ADDR - 8, 0xFFFFFFFFFFFFFFFF);
    let mut regs = vcpu.get_regs().unwrap();
    regs.rsp = STACK_ADDR - 8;
    vcpu.set_regs(&regs).unwrap();
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0xFFFFFFFFFFFFFFFF);
}

#[test]
fn test_pop_to_memory_with_displacement() {
    // POP to memory with displacement
    let code = [
        0x8f, 0x43, 0x10, // POP QWORD PTR [RBX+16]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = DATA_ADDR;
    regs.rsp = STACK_ADDR - 8;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    write_mem_at_u64(&mem, STACK_ADDR - 8, 0xDEADBEEFCAFEBABE);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let value = read_mem_at_u64(&mem, DATA_ADDR + 16);
    assert_eq!(value, 0xDEADBEEFCAFEBABE);
}

#[test]
fn test_pop_alternating_sizes() {
    // Mix of 16-bit and 64-bit pops
    let code = [
        0x59, // POP RCX (64-bit)
        0x66, 0x5b, // POP BX (16-bit)
        0x58, // POP RAX (64-bit)
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u64(&mem, STACK_ADDR - 18, 0x3333333333333333);
    write_mem_at_u16(&mem, STACK_ADDR - 10, 0x2222);
    write_mem_at_u64(&mem, STACK_ADDR - 8, 0x1111111111111111);

    let mut regs = vcpu.get_regs().unwrap();
    regs.rsp = STACK_ADDR - 18;
    vcpu.set_regs(&regs).unwrap();
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 0x3333333333333333);
    assert_eq!(regs.rbx & 0xFFFF, 0x2222);
    assert_eq!(regs.rax, 0x1111111111111111);
}

#[test]
fn test_pop_stack_cleanup() {
    // Simulate cleaning up function arguments from stack
    let code = [
        0x58, // POP RAX
        0x58, // POP RAX
        0x58, // POP RAX
        0x58, // POP RAX
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    // Simulated arguments on stack
    write_mem_at_u64(&mem, STACK_ADDR - 32, 0x1);
    write_mem_at_u64(&mem, STACK_ADDR - 24, 0x2);
    write_mem_at_u64(&mem, STACK_ADDR - 16, 0x3);
    write_mem_at_u64(&mem, STACK_ADDR - 8, 0x4);

    let mut regs = vcpu.get_regs().unwrap();
    regs.rsp = STACK_ADDR - 32;
    vcpu.set_regs(&regs).unwrap();
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rsp, STACK_ADDR); // Stack cleaned
}

#[test]
fn test_pop_sequential_values() {
    // POP sequential values to verify order
    let code = [
        0x58, // POP RAX
        0x5b, // POP RBX
        0x59, // POP RCX
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u64(&mem, STACK_ADDR - 24, 1);
    write_mem_at_u64(&mem, STACK_ADDR - 16, 2);
    write_mem_at_u64(&mem, STACK_ADDR - 8, 3);

    let mut regs = vcpu.get_regs().unwrap();
    regs.rsp = STACK_ADDR - 24;
    vcpu.set_regs(&regs).unwrap();
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 1);
    assert_eq!(regs.rbx, 2);
    assert_eq!(regs.rcx, 3);
}

#[test]
fn test_pop_overwrites_previous_value() {
    // POP completely overwrites destination register
    let code = [
        0x58, // POP RAX
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_at_u64(&mem, STACK_ADDR - 8, 0x1234567890ABCDEF);
    let mut regs = vcpu.get_regs().unwrap();
    regs.rsp = STACK_ADDR - 8;
    regs.rax = 0xFFFFFFFFFFFFFFFF; // Will be replaced
    vcpu.set_regs(&regs).unwrap();
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x1234567890ABCDEF);
}
