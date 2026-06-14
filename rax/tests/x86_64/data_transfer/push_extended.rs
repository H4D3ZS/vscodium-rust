// Module path for tests run via x86_64.rs
use crate::common::*;
use rax::cpu::Registers;

// PUSH - Push onto Stack (Comprehensive Extended Tests)
// Decrements RSP by operand size and writes operand to new stack location.
// Tests cover all sizes (16/32/64-bit), all register types, memory operands,
// and immediate values.
//
// Opcodes:
// 50+rd        PUSH r64           - Push r64
// 66 50+rw     PUSH r16           - Push r16
// FF /6        PUSH r/m64         - Push r/m64
// 6A ib        PUSH imm8          - Push sign-extended imm8
// 68 id        PUSH imm32         - Push sign-extended imm32

#[test]
fn test_push_rax() {
    // PUSH RAX - Basic 64-bit register push
    let code = [
        0x50, // PUSH RAX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x123456789ABCDEF0;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rsp, STACK_ADDR - 8);
    let value = read_mem_at_u64(&mem, STACK_ADDR - 8);
    assert_eq!(value, 0x123456789ABCDEF0);
}

#[test]
fn test_push_rbx() {
    // PUSH RBX
    let code = [
        0x53, // PUSH RBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xDEADBEEFCAFEBABE;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rsp, STACK_ADDR - 8);
    let value = read_mem_at_u64(&mem, STACK_ADDR - 8);
    assert_eq!(value, 0xDEADBEEFCAFEBABE);
}

#[test]
fn test_push_rcx() {
    // PUSH RCX
    let code = [
        0x51, // PUSH RCX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rcx = 0x1122334455667788;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rsp, STACK_ADDR - 8);
    let value = read_mem_at_u64(&mem, STACK_ADDR - 8);
    assert_eq!(value, 0x1122334455667788);
}

#[test]
fn test_push_rdx() {
    // PUSH RDX
    let code = [
        0x52, // PUSH RDX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rdx = 0xAABBCCDDEEFF0011;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rsp, STACK_ADDR - 8);
    let value = read_mem_at_u64(&mem, STACK_ADDR - 8);
    assert_eq!(value, 0xAABBCCDDEEFF0011);
}

#[test]
fn test_push_rsi() {
    // PUSH RSI
    let code = [
        0x56, // PUSH RSI
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rsi = 0xFEDCBA9876543210;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rsp, STACK_ADDR - 8);
    let value = read_mem_at_u64(&mem, STACK_ADDR - 8);
    assert_eq!(value, 0xFEDCBA9876543210);
}

#[test]
fn test_push_rdi() {
    // PUSH RDI
    let code = [
        0x57, // PUSH RDI
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rdi = 0x0123456789ABCDEF;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rsp, STACK_ADDR - 8);
    let value = read_mem_at_u64(&mem, STACK_ADDR - 8);
    assert_eq!(value, 0x0123456789ABCDEF);
}

#[test]
fn test_push_rbp() {
    // PUSH RBP - Common in function prologues
    let code = [
        0x55, // PUSH RBP
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbp = 0x1000000000000000;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rsp, STACK_ADDR - 8);
    let value = read_mem_at_u64(&mem, STACK_ADDR - 8);
    assert_eq!(value, 0x1000000000000000);
}

#[test]
fn test_push_r8() {
    // PUSH R8 - Extended register
    let code = [
        0x41, 0x50, // PUSH R8
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.r8 = 0x8888888888888888;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rsp, STACK_ADDR - 8);
    let value = read_mem_at_u64(&mem, STACK_ADDR - 8);
    assert_eq!(value, 0x8888888888888888);
}

#[test]
fn test_push_r15() {
    // PUSH R15 - Extended register
    let code = [
        0x41, 0x57, // PUSH R15
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.r15 = 0xFFFFFFFFFFFFFFFF;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rsp, STACK_ADDR - 8);
    let value = read_mem_at_u64(&mem, STACK_ADDR - 8);
    assert_eq!(value, 0xFFFFFFFFFFFFFFFF);
}

#[test]
fn test_push_all_extended_regs() {
    // Push all extended registers R8-R15
    let code = [
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
    regs.r8 = 8;
    regs.r9 = 9;
    regs.r10 = 10;
    regs.r11 = 11;
    regs.r12 = 12;
    regs.r13 = 13;
    regs.r14 = 14;
    regs.r15 = 15;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rsp, STACK_ADDR - 64); // 8 pushes * 8 bytes

    // Verify values (pushed in reverse order on stack)
    assert_eq!(read_mem_at_u64(&mem, STACK_ADDR - 8), 8);
    assert_eq!(read_mem_at_u64(&mem, STACK_ADDR - 16), 9);
    assert_eq!(read_mem_at_u64(&mem, STACK_ADDR - 24), 10);
    assert_eq!(read_mem_at_u64(&mem, STACK_ADDR - 32), 11);
    assert_eq!(read_mem_at_u64(&mem, STACK_ADDR - 40), 12);
    assert_eq!(read_mem_at_u64(&mem, STACK_ADDR - 48), 13);
    assert_eq!(read_mem_at_u64(&mem, STACK_ADDR - 56), 14);
    assert_eq!(read_mem_at_u64(&mem, STACK_ADDR - 64), 15);
}

#[test]
fn test_push_imm8() {
    // PUSH imm8 - Sign-extended to 64 bits
    let code = [
        0x6a, 0x42, // PUSH 0x42
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rsp, STACK_ADDR - 8);
    let value = read_mem_at_u64(&mem, STACK_ADDR - 8);
    assert_eq!(value, 0x0000000000000042);
}

#[test]
fn test_push_imm8_negative() {
    // PUSH imm8 - Negative value (sign-extended)
    let code = [
        0x6a, 0xff, // PUSH -1 (0xFF sign-extended)
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rsp, STACK_ADDR - 8);
    let value = read_mem_at_u64(&mem, STACK_ADDR - 8);
    assert_eq!(value, 0xFFFFFFFFFFFFFFFF); // Sign-extended
}

#[test]
fn test_push_imm32() {
    // PUSH imm32 - Sign-extended to 64 bits
    let code = [
        0x68, 0x78, 0x56, 0x34, 0x12, // PUSH 0x12345678
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rsp, STACK_ADDR - 8);
    let value = read_mem_at_u64(&mem, STACK_ADDR - 8);
    assert_eq!(value, 0x0000000012345678);
}

#[test]
fn test_push_imm32_negative() {
    // PUSH imm32 - Negative value (sign-extended)
    let code = [
        0x68, 0xff, 0xff, 0xff, 0xff, // PUSH -1 (0xFFFFFFFF sign-extended)
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rsp, STACK_ADDR - 8);
    let value = read_mem_at_u64(&mem, STACK_ADDR - 8);
    assert_eq!(value, 0xFFFFFFFFFFFFFFFF);
}

#[test]
fn test_push_word() {
    // PUSH r16 - 16-bit register push
    let code = [
        0x66, 0x50, // PUSH AX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x1234;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rsp, STACK_ADDR - 2); // 16-bit push
    let value = read_mem_at_u16(&mem, STACK_ADDR - 2);
    assert_eq!(value, 0x1234);
}

#[test]
fn test_push_memory_qword() {
    // PUSH [RBX] - Push from memory
    let code = [
        0xff, 0x33, // PUSH QWORD PTR [RBX]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = DATA_ADDR;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    write_mem_u64(&mem, 0x123456789ABCDEF0);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rsp, STACK_ADDR - 8);
    let value = read_mem_at_u64(&mem, STACK_ADDR - 8);
    assert_eq!(value, 0x123456789ABCDEF0);
}

#[test]
fn test_push_memory_with_displacement() {
    // PUSH [RBX+16]
    let code = [
        0xff, 0x73, 0x10, // PUSH QWORD PTR [RBX+16]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = DATA_ADDR;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    write_mem_at_u64(&mem, DATA_ADDR + 16, 0xDEADBEEFCAFEBABE);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rsp, STACK_ADDR - 8);
    let value = read_mem_at_u64(&mem, STACK_ADDR - 8);
    assert_eq!(value, 0xDEADBEEFCAFEBABE);
}

#[test]
fn test_push_multiple_registers() {
    // Multiple PUSH operations
    let code = [
        0x50, // PUSH RAX
        0x53, // PUSH RBX
        0x51, // PUSH RCX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x1111111111111111;
    regs.rbx = 0x2222222222222222;
    regs.rcx = 0x3333333333333333;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rsp, STACK_ADDR - 24); // 3 pushes * 8 bytes

    // Check stack (last in, first out order)
    assert_eq!(read_mem_at_u64(&mem, STACK_ADDR - 8), 0x1111111111111111);
    assert_eq!(read_mem_at_u64(&mem, STACK_ADDR - 16), 0x2222222222222222);
    assert_eq!(read_mem_at_u64(&mem, STACK_ADDR - 24), 0x3333333333333333);
}

#[test]
fn test_push_zero() {
    // PUSH 0
    let code = [
        0x6a, 0x00, // PUSH 0
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rsp, STACK_ADDR - 8);
    let value = read_mem_at_u64(&mem, STACK_ADDR - 8);
    assert_eq!(value, 0);
}

#[test]
fn test_push_preserves_register() {
    // PUSH should not modify the source register
    let code = [
        0x50, // PUSH RAX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x123456789ABCDEF0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x123456789ABCDEF0); // Should be unchanged
}

#[test]
fn test_push_decrements_rsp_correctly() {
    // Verify RSP is decremented by 8 for each PUSH
    let code = [
        0x50, // PUSH RAX
        0x50, // PUSH RAX
        0x50, // PUSH RAX
        0xf4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rsp, STACK_ADDR - 24); // 3 * 8 bytes
}

#[test]
fn test_push_all_gprs() {
    // Push all general purpose registers
    let code = [
        0x50, // PUSH RAX
        0x53, // PUSH RBX
        0x51, // PUSH RCX
        0x52, // PUSH RDX
        0x56, // PUSH RSI
        0x57, // PUSH RDI
        0x55, // PUSH RBP
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 1;
    regs.rbx = 2;
    regs.rcx = 3;
    regs.rdx = 4;
    regs.rsi = 5;
    regs.rdi = 6;
    regs.rbp = 7;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rsp, STACK_ADDR - 56); // 7 pushes * 8 bytes

    assert_eq!(read_mem_at_u64(&mem, STACK_ADDR - 8), 1);
    assert_eq!(read_mem_at_u64(&mem, STACK_ADDR - 16), 2);
    assert_eq!(read_mem_at_u64(&mem, STACK_ADDR - 24), 3);
    assert_eq!(read_mem_at_u64(&mem, STACK_ADDR - 32), 4);
    assert_eq!(read_mem_at_u64(&mem, STACK_ADDR - 40), 5);
    assert_eq!(read_mem_at_u64(&mem, STACK_ADDR - 48), 6);
    assert_eq!(read_mem_at_u64(&mem, STACK_ADDR - 56), 7);
}

#[test]
fn test_push_function_prologue_pattern() {
    // Common function prologue: push rbp; mov rbp, rsp
    let code = [
        0x55, // PUSH RBP
        0x48, 0x89, 0xe5, // MOV RBP, RSP
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbp = 0x1000;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbp, STACK_ADDR - 8); // RBP now points to saved RBP
    let saved_rbp = read_mem_at_u64(&mem, STACK_ADDR - 8);
    assert_eq!(saved_rbp, 0x1000); // Old RBP saved on stack
}

#[test]
fn test_push_immediate_values() {
    // Various immediate values
    let code = [
        0x6a, 0x01, // PUSH 1
        0x6a, 0x7f, // PUSH 127
        0x68, 0x00, 0x01, 0x00, 0x00, // PUSH 256
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rsp, STACK_ADDR - 24); // 3 pushes

    assert_eq!(read_mem_at_u64(&mem, STACK_ADDR - 8), 1);
    assert_eq!(read_mem_at_u64(&mem, STACK_ADDR - 16), 127);
    assert_eq!(read_mem_at_u64(&mem, STACK_ADDR - 24), 256);
}

#[test]
fn test_push_max_values() {
    // Push maximum values
    let code = [
        0x6a, 0x7f, // PUSH 127 (max positive imm8)
        0x68, 0xff, 0xff, 0xff, 0x7f, // PUSH 0x7FFFFFFF (max positive imm32)
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(read_mem_at_u64(&mem, STACK_ADDR - 8), 127);
    assert_eq!(read_mem_at_u64(&mem, STACK_ADDR - 16), 0x000000007FFFFFFF);
}

#[test]
fn test_push_sib_addressing() {
    // PUSH with SIB byte addressing
    let code = [
        0xff, 0x34, 0x0b, // PUSH QWORD PTR [RBX+RCX]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = DATA_ADDR;
    regs.rcx = 16;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    write_mem_at_u64(&mem, DATA_ADDR + 16, 0x1122334455667788);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let value = read_mem_at_u64(&mem, STACK_ADDR - 8);
    assert_eq!(value, 0x1122334455667788);
}

#[test]
fn test_push_does_not_affect_flags() {
    // PUSH should not modify flags
    let code = [
        0x50, // PUSH RAX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x123456789ABCDEF0;
    regs.rflags = 0x246; // Some flags set
    let initial_flags = regs.rflags;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rflags, initial_flags);
}

#[test]
fn test_push_stack_alignment() {
    // Verify stack maintains proper alignment
    let code = [
        0x50, // PUSH RAX
        0x53, // PUSH RBX
        0xf4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Stack should be aligned to 8 bytes after pushes
    assert_eq!(regs.rsp % 8, 0);
}

#[test]
fn test_push_negative_imm8_values() {
    // Push negative immediate values (sign-extended)
    let test_cases = vec![
        (0x80u8, 0xFFFFFFFFFFFFFF80u64),  // -128
        (0xFF_u8, 0xFFFFFFFFFFFFFFFFu64), // -1
        (0xFE_u8, 0xFFFFFFFFFFFFFFFEu64), // -2
    ];

    for (imm, expected) in test_cases {
        let code = [
            0x6a, imm, // PUSH imm8
            0xf4,
        ];
        let (mut vcpu, mem) = setup_vm(&code, None);
        let regs = run_until_hlt(&mut vcpu).unwrap();
        let value = read_mem_at_u64(&mem, STACK_ADDR - 8);
        assert_eq!(value, expected, "PUSH {:#x} should be {:#x}", imm, expected);
    }
}

#[test]
fn test_push_word_multiple() {
    // Multiple 16-bit pushes
    let code = [
        0x66, 0x50, // PUSH AX
        0x66, 0x53, // PUSH BX
        0x66, 0x51, // PUSH CX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x1111;
    regs.rbx = 0x2222;
    regs.rcx = 0x3333;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rsp, STACK_ADDR - 6); // 3 * 2 bytes

    assert_eq!(read_mem_at_u16(&mem, STACK_ADDR - 2), 0x1111);
    assert_eq!(read_mem_at_u16(&mem, STACK_ADDR - 4), 0x2222);
    assert_eq!(read_mem_at_u16(&mem, STACK_ADDR - 6), 0x3333);
}

#[test]
fn test_push_r12_special_encoding() {
    // R12 requires SIB byte in many contexts
    let code = [
        0x41, 0x54, // PUSH R12
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.r12 = 0xBBBBBBBBBBBBBBBB;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rsp, STACK_ADDR - 8);
    let value = read_mem_at_u64(&mem, STACK_ADDR - 8);
    assert_eq!(value, 0xBBBBBBBBBBBBBBBB);
}

#[test]
fn test_push_r13_special_encoding() {
    // R13 requires displacement in many contexts
    let code = [
        0x41, 0x55, // PUSH R13
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.r13 = 0xCCCCCCCCCCCCCCCC;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rsp, STACK_ADDR - 8);
    let value = read_mem_at_u64(&mem, STACK_ADDR - 8);
    assert_eq!(value, 0xCCCCCCCCCCCCCCCC);
}

#[test]
fn test_push_alternating_sizes() {
    // Mix of 16-bit and 64-bit pushes
    let code = [
        0x50, // PUSH RAX (64-bit)
        0x66, 0x53, // PUSH BX (16-bit)
        0x51, // PUSH RCX (64-bit)
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x1111111111111111;
    regs.rbx = 0x2222;
    regs.rcx = 0x3333333333333333;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rsp, STACK_ADDR - 18); // 8 + 2 + 8

    assert_eq!(read_mem_at_u64(&mem, STACK_ADDR - 8), 0x1111111111111111);
    assert_eq!(read_mem_at_u16(&mem, STACK_ADDR - 10), 0x2222);
    assert_eq!(read_mem_at_u64(&mem, STACK_ADDR - 18), 0x3333333333333333);
}

#[test]
fn test_push_memory_indirect() {
    // PUSH from memory with indirect addressing
    let code = [
        0xff, 0x33, // PUSH QWORD PTR [RBX]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = DATA_ADDR;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    write_mem_u64(&mem, 0xDEADBEEFCAFEBABE);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let value = read_mem_at_u64(&mem, STACK_ADDR - 8);
    assert_eq!(value, 0xDEADBEEFCAFEBABE);
}

#[test]
fn test_push_large_immediate() {
    // PUSH with large immediate value
    let code = [
        0x68, 0xff, 0xff, 0xff, 0x7f, // PUSH 0x7FFFFFFF
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let value = read_mem_at_u64(&mem, STACK_ADDR - 8);
    assert_eq!(value, 0x000000007FFFFFFF);
}

#[test]
fn test_push_preserves_memory_below_stack() {
    // PUSH should not corrupt memory above the new RSP
    let code = [
        0x50, // PUSH RAX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x1111111111111111;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    // Write a value above where the push will occur
    write_mem_at_u64(&mem, STACK_ADDR, 0x9999999999999999);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Verify the value above stack is unchanged
    let above = read_mem_at_u64(&mem, STACK_ADDR);
    assert_eq!(above, 0x9999999999999999);

    // Verify the pushed value
    let pushed = read_mem_at_u64(&mem, STACK_ADDR - 8);
    assert_eq!(pushed, 0x1111111111111111);
}

#[test]
fn test_push_sequence_for_variadic_args() {
    // Simulate pushing arguments for a variadic function
    let code = [
        0x6a, 0x03, // PUSH 3 (arg count)
        0x68, 0x0a, 0x00, 0x00, 0x00, // PUSH 10 (arg 3)
        0x68, 0x14, 0x00, 0x00, 0x00, // PUSH 20 (arg 2)
        0x68, 0x1e, 0x00, 0x00, 0x00, // PUSH 30 (arg 1)
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rsp, STACK_ADDR - 32);

    assert_eq!(read_mem_at_u64(&mem, STACK_ADDR - 8), 3);
    assert_eq!(read_mem_at_u64(&mem, STACK_ADDR - 16), 10);
    assert_eq!(read_mem_at_u64(&mem, STACK_ADDR - 24), 20);
    assert_eq!(read_mem_at_u64(&mem, STACK_ADDR - 32), 30);
}

#[test]
fn test_push_with_base_plus_index() {
    // PUSH with base + index addressing
    let code = [
        0xff, 0x34, 0x8b, // PUSH QWORD PTR [RBX+RCX*4]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = DATA_ADDR;
    regs.rcx = 4;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    write_mem_at_u64(&mem, DATA_ADDR + 16, 0xAABBCCDDEEFF0011);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let value = read_mem_at_u64(&mem, STACK_ADDR - 8);
    assert_eq!(value, 0xAABBCCDDEEFF0011);
}
