// Module path for tests run via x86_64.rs
use crate::common::*;
use rax::cpu::Registers;
use rax::cpu::VcpuExit;

// HLT - Halt
// Stops instruction execution and places the processor in a HALT state.
// An enabled interrupt (including NMI and SMI), a debug exception, the BINIT# signal,
// the INIT# signal, or the RESET# signal will resume execution.
//
// Opcode: F4
//
// In our emulator context, HLT causes the VM to exit with VcpuExit::Hlt.
// This is the standard way to terminate execution in tests.

#[test]
fn test_hlt_basic() {
    // Basic HLT instruction stops execution
    let code = [
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    // Run should return VcpuExit::Hlt
    let exit_reason = vcpu.run().unwrap();
    assert!(
        matches!(exit_reason, VcpuExit::Hlt),
        "HLT should cause Hlt exit"
    );
}

#[test]
fn test_hlt_after_nop() {
    // HLT after NOP
    let code = [
        0x90, // NOP
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Verify execution completed
    assert!(regs.rip > CODE_ADDR, "RIP should have advanced past code");
}

#[test]
fn test_hlt_after_mov() {
    // HLT after MOV instruction
    let code = [
        0xb8, 0x42, 0x00, 0x00, 0x00, // MOV EAX, 0x42
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 0x42, "MOV should execute before HLT");
}

#[test]
fn test_hlt_preserves_registers() {
    // HLT should not modify any registers
    let code = [
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x1111111111111111;
    regs.rbx = 0x2222222222222222;
    regs.rcx = 0x3333333333333333;
    regs.rdx = 0x4444444444444444;
    regs.rsi = 0x5555555555555555;
    regs.rdi = 0x6666666666666666;

    let (mut vcpu, _) = setup_vm(&code, Some(regs.clone()));
    let final_regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(final_regs.rax, regs.rax, "RAX unchanged");
    assert_eq!(final_regs.rbx, regs.rbx, "RBX unchanged");
    assert_eq!(final_regs.rcx, regs.rcx, "RCX unchanged");
    assert_eq!(final_regs.rdx, regs.rdx, "RDX unchanged");
    assert_eq!(final_regs.rsi, regs.rsi, "RSI unchanged");
    assert_eq!(final_regs.rdi, regs.rdi, "RDI unchanged");
}

#[test]
fn test_hlt_preserves_flags() {
    // HLT should not modify flags
    let code = [
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rflags = 0x2 | 0x1 | (1 << 6) | (1 << 7) | (1 << 11); // CF, ZF, SF, OF set
    let initial_flags = regs.rflags;

    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let final_regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        final_regs.rflags, initial_flags,
        "Flags should be unchanged"
    );
}

#[test]
fn test_hlt_with_all_flags_set() {
    // HLT with various flags set
    let code = [
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    // Set CF, PF, AF, ZF, SF, OF
    regs.rflags = 0x2 | 0x1 | (1 << 2) | (1 << 4) | (1 << 6) | (1 << 7) | (1 << 11);
    let initial_flags = regs.rflags;

    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let final_regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(final_regs.rflags, initial_flags, "All flags preserved");
}

#[test]
fn test_hlt_after_arithmetic() {
    // HLT after arithmetic that sets flags
    let code = [
        0xb8, 0xff, 0xff, 0xff, 0xff, // MOV EAX, 0xFFFFFFFF
        0x83, 0xc0, 0x01, // ADD EAX, 1 (sets CF, ZF, etc.)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 0, "ADD should wrap to 0");
    assert!(cf_set(regs.rflags), "CF should be set from overflow");
    assert!(zf_set(regs.rflags), "ZF should be set (result is 0)");
}

#[test]
fn test_hlt_after_multiple_instructions() {
    // HLT after multiple instructions
    let code = [
        0xb8, 0x01, 0x00, 0x00, 0x00, // MOV EAX, 1
        0xbb, 0x02, 0x00, 0x00, 0x00, // MOV EBX, 2
        0x01, 0xd8, // ADD EAX, EBX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 3, "EAX should be 3");
    assert_eq!(regs.rbx & 0xFFFFFFFF, 2, "EBX should be 2");
}

#[test]
fn test_hlt_with_zero_registers() {
    // HLT with all registers at 0
    let code = [
        0xf4, // HLT
    ];
    let regs = Registers::default();
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let final_regs = run_until_hlt(&mut vcpu).unwrap();

    // Default values should be preserved
    assert_eq!(final_regs.rax, 0, "RAX should be 0");
    assert_eq!(final_regs.rbx, 0, "RBX should be 0");
}

#[test]
fn test_hlt_with_max_register_values() {
    // HLT with maximum register values
    let code = [
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0xFFFFFFFFFFFFFFFF;
    regs.rbx = 0xFFFFFFFFFFFFFFFF;
    regs.rcx = 0xFFFFFFFFFFFFFFFF;

    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let final_regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(final_regs.rax, 0xFFFFFFFFFFFFFFFF, "RAX unchanged");
    assert_eq!(final_regs.rbx, 0xFFFFFFFFFFFFFFFF, "RBX unchanged");
    assert_eq!(final_regs.rcx, 0xFFFFFFFFFFFFFFFF, "RCX unchanged");
}

#[test]
fn test_hlt_after_stack_operations() {
    // HLT after stack push/pop operations
    let code = [
        0x50, // PUSH RAX
        0x58, // POP RAX
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x42;
    regs.rsp = STACK_ADDR; // Must set RSP when passing custom regs
    let initial_rsp = STACK_ADDR;

    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let final_regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(final_regs.rax, 0x42, "RAX should be restored");
    assert_eq!(final_regs.rsp, initial_rsp, "RSP should be restored");
}

#[test]
fn test_hlt_in_sequence() {
    // First HLT should stop execution, second should never execute
    let code = [
        0xb8, 0x42, 0x00, 0x00, 0x00, // MOV EAX, 0x42
        0xf4, // HLT
        0xb8, 0xff, 0x00, 0x00, 0x00, // MOV EAX, 0xFF (should not execute)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 0x42, "Only first MOV should execute");
}

#[test]
fn test_hlt_preserves_stack_pointer() {
    // HLT should not modify stack pointer
    let code = [
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    let initial_rsp = STACK_ADDR;
    regs.rsp = initial_rsp;

    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let final_regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(final_regs.rsp, initial_rsp, "RSP unchanged");
}

#[test]
fn test_hlt_after_conditional_jump() {
    // HLT after a conditional jump
    let code = [
        0xb8, 0x01, 0x00, 0x00, 0x00, // MOV EAX, 1
        0x85, 0xc0, // TEST EAX, EAX (sets ZF=0)
        0x75, 0x05, // JNZ +5 (skip next instruction)
        0xb8, 0xff, 0x00, 0x00, 0x00, // MOV EAX, 0xFF (skipped)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        1,
        "EAX should still be 1 (jump taken)"
    );
}

#[test]
fn test_hlt_after_unconditional_jump() {
    // HLT after unconditional jump
    let code = [
        0xeb, 0x05, // JMP +5
        0xb8, 0xff, 0x00, 0x00, 0x00, // MOV EAX, 0xFF (skipped)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0, "EAX should be 0 (MOV skipped)");
}

#[test]
fn test_hlt_with_memory_operations() {
    // HLT after memory write
    let code = [
        0x48, 0xb8, 0x42, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV RAX, 0x42
        0x48, 0xa3, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV [0x2000], RAX
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x42, "RAX should be 0x42");
    let mem_val = read_mem_u64(&mem);
    assert_eq!(mem_val, 0x42, "Memory should contain 0x42");
}

#[test]
fn test_hlt_preserves_extended_registers() {
    // HLT should preserve R8-R15
    let code = [
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
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
fn test_hlt_after_64bit_operations() {
    // HLT after 64-bit operations
    let code = [
        0x48, 0xb8, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, // MOV RAX, 0xFFFFFFFFFFFFFFFF
        0x48, 0xff, 0xc0, // INC RAX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0, "RAX should wrap to 0");
}

#[test]
fn test_hlt_after_string_operations() {
    // HLT after string operation setup
    let code = [
        0x48, 0xbf, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV RDI, 0x2000
        0x48, 0xbe, 0x00, 0x30, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV RSI, 0x3000
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rdi, 0x2000, "RDI should be 0x2000");
    assert_eq!(regs.rsi, 0x3000, "RSI should be 0x3000");
}

#[test]
fn test_hlt_immediate_after_code_start() {
    // HLT as first instruction
    let code = [
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Should halt immediately without error
    assert!(regs.rip >= CODE_ADDR, "RIP should be at or past code start");
}

#[test]
fn test_hlt_with_carry_flag() {
    // HLT with carry flag set
    let code = [
        0xf9, // STC (set carry flag)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags), "CF should remain set");
}

#[test]
fn test_hlt_with_direction_flag() {
    // HLT with direction flag set
    let code = [
        0xfd, // STD (set direction flag)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(df_set(regs.rflags), "DF should remain set");
}

#[test]
fn test_hlt_after_xor_zeroing() {
    // HLT after XOR register with itself (common zeroing pattern)
    let code = [
        0x48, 0x31, 0xc0, // XOR RAX, RAX
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0xFFFFFFFFFFFFFFFF;

    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let final_regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(final_regs.rax, 0, "RAX should be zeroed");
    assert!(zf_set(final_regs.rflags), "ZF should be set");
}

#[test]
fn test_hlt_after_bit_operations() {
    // HLT after bit manipulation
    let code = [
        0xb8, 0x55, 0x00, 0x00, 0x00, // MOV EAX, 0x55
        0x48, 0x0f, 0xba, 0xe8, 0x03, // BTS RAX, 3 (set bit 3)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFF,
        0x5D,
        "Bit 3 should be set (0x55 | 0x08 = 0x5D)"
    );
}

#[test]
fn test_hlt_preserves_base_pointer() {
    // HLT should preserve base pointer
    let code = [
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rbp = 0x7000;

    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let final_regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(final_regs.rbp, 0x7000, "RBP unchanged");
}
