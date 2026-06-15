// Module path for tests run via x86_64.rs
use crate::common::*;
use rax::cpu::Registers;

// NOP - No Operation
// Performs no operation, used for padding and alignment.
// Does not modify any registers, memory, or flags.
// Various forms exist for alignment (1-9 bytes).
//
// Opcodes:
// 90                  NOP                        - 1-byte NOP
// 0F 1F /0            NOP r/m32                  - Multi-byte NOP (2 bytes minimum)
// 66 0F 1F /0 /0      NOP r/m16                  - 3-byte NOP with operand size prefix
// 0F 1F 00            NOP dword [rax]            - 4-byte NOP
// And so on for 5-9 byte NOPs

#[test]
fn test_nop_basic_90() {
    // Basic 1-byte NOP
    let code = [
        0x90, // NOP
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x12345678;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 0x12345678, "RAX should be unchanged");
}

#[test]
fn test_nop_does_not_modify_rbx() {
    // NOP doesn't modify RBX
    let code = [
        0x90, // NOP
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xAABBCCDD;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx & 0xFFFFFFFF, 0xAABBCCDD, "RBX should be unchanged");
}

#[test]
fn test_nop_does_not_modify_rcx() {
    // NOP doesn't modify RCX
    let code = [
        0x90, // NOP
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rcx = 0x11111111;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx & 0xFFFFFFFF, 0x11111111, "RCX should be unchanged");
}

#[test]
fn test_nop_does_not_modify_flags() {
    // NOP doesn't modify any flags
    let code = [
        0x90, // NOP
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rflags = 0x2; // Only reserved bit 1 set
    let initial_flags = regs.rflags;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rflags, initial_flags, "Flags should be unchanged");
}

#[test]
fn test_nop_multiple_nops() {
    // Multiple sequential NOPs
    let code = [
        0x90, // NOP
        0x90, // NOP
        0x90, // NOP
        0x90, // NOP
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x42;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax, 0x42,
        "RAX should be unchanged after multiple NOPs"
    );
}

#[test]
fn test_nop_with_other_registers() {
    // NOP with multiple registers containing values
    let code = [
        0x90, // NOP
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x11111111;
    regs.rbx = 0x22222222;
    regs.rcx = 0x33333333;
    regs.rdx = 0x44444444;
    regs.rsi = 0x55555555;
    regs.rdi = 0x66666666;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 0x11111111, "RAX should be unchanged");
    assert_eq!(regs.rbx & 0xFFFFFFFF, 0x22222222, "RBX should be unchanged");
    assert_eq!(regs.rcx & 0xFFFFFFFF, 0x33333333, "RCX should be unchanged");
    assert_eq!(regs.rdx & 0xFFFFFFFF, 0x44444444, "RDX should be unchanged");
    assert_eq!(regs.rsi & 0xFFFFFFFF, 0x55555555, "RSI should be unchanged");
    assert_eq!(regs.rdi & 0xFFFFFFFF, 0x66666666, "RDI should be unchanged");
}

#[test]
fn test_nop_does_not_affect_64bit_registers() {
    // NOP doesn't affect 64-bit register values
    let code = [
        0x90, // NOP
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x123456789ABCDEF0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x123456789ABCDEF0, "RAX should be unchanged");
}

#[test]
fn test_nop_preserves_extended_registers() {
    // NOP doesn't affect extended registers
    let code = [
        0x90, // NOP
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.r8 = 0x0011223344556677;
    regs.r15 = 0xFEDCBA9876543210;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.r8, 0x0011223344556677, "R8 should be unchanged");
    assert_eq!(regs.r15, 0xFEDCBA9876543210, "R15 should be unchanged");
}

#[test]
fn test_nop_two_byte_0f_1f_00() {
    // 2-byte NOP: 0F 1F /0
    let code = [
        0x0f, 0x1f, 0x00, // NOP r/m32 (with ModR/M byte indicating r32 reg 0)
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x12345678;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x12345678,
        "RAX should be unchanged after 2-byte NOP"
    );
}

#[test]
fn test_nop_two_byte_multiple() {
    // Multiple 2-byte NOPs
    let code = [
        0x0f, 0x1f, 0x00, // 2-byte NOP
        0x0f, 0x1f, 0x00, // 2-byte NOP
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0xDEADBEEF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0xDEADBEEF,
        "RAX should be unchanged after multiple 2-byte NOPs"
    );
}

#[test]
fn test_nop_sequences_for_alignment() {
    // NOPs for instruction alignment (4-byte alignment)
    let code = [
        0x90, // 1-byte NOP
        0x90, // 1-byte NOP
        0x90, // 1-byte NOP
        0x90, // 1-byte NOP (total 4 bytes)
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0xFFFFFFFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax, 0xFFFFFFFF,
        "RAX should be unchanged after alignment NOPs"
    );
}

#[test]
fn test_nop_with_zero_flags() {
    // NOP with zero flags
    let code = [
        0x90, // NOP
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rflags = 0x2; // Only reserved bit 1
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rflags, 0x2, "Flags should remain as 0x2");
}

#[test]
fn test_nop_with_various_flags() {
    // NOP with various flags set
    let code = [
        0x90, // NOP
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rflags = 0x2 | (1 << 6) | (1 << 7) | (1 << 11); // ZF, PF, OF set
    let initial_flags = regs.rflags;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rflags, initial_flags, "Flags should not be modified");
}

#[test]
fn test_nop_preserves_all_registers() {
    // NOP preserves all 16 general-purpose registers
    let code = [
        0x90, // NOP
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
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x1111111111111111, "RAX unchanged");
    assert_eq!(regs.rbx, 0x2222222222222222, "RBX unchanged");
    assert_eq!(regs.rcx, 0x3333333333333333, "RCX unchanged");
    assert_eq!(regs.rdx, 0x4444444444444444, "RDX unchanged");
    assert_eq!(regs.rsi, 0x5555555555555555, "RSI unchanged");
    assert_eq!(regs.rdi, 0x6666666666666666, "RDI unchanged");
    assert_eq!(regs.rbp, 0x7777777777777777, "RBP unchanged");
    assert_eq!(regs.r8, 0x8888888888888888, "R8 unchanged");
    assert_eq!(regs.r9, 0x9999999999999999, "R9 unchanged");
    assert_eq!(regs.r10, 0xAAAAAAAAAAAAAAAA, "R10 unchanged");
    assert_eq!(regs.r11, 0xBBBBBBBBBBBBBBBB, "R11 unchanged");
    assert_eq!(regs.r12, 0xCCCCCCCCCCCCCCCC, "R12 unchanged");
    assert_eq!(regs.r13, 0xDDDDDDDDDDDDDDDD, "R13 unchanged");
    assert_eq!(regs.r14, 0xEEEEEEEEEEEEEEEE, "R14 unchanged");
    assert_eq!(regs.r15, 0xFFFFFFFFFFFFFFFF, "R15 unchanged");
}

#[test]
fn test_nop_can_be_used_for_alignment_2bytes() {
    // 2 NOPs = 2 bytes for alignment
    let code = [
        0x90, // NOP (1 byte)
        0x90, // NOP (1 byte) - total 2 bytes
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x42;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x42, "RAX unchanged");
}

#[test]
fn test_nop_can_be_used_for_alignment_3bytes() {
    // 3 NOPs = 3 bytes for alignment
    let code = [
        0x90, // NOP
        0x90, // NOP
        0x90, // NOP - total 3 bytes
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x42;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x42, "RAX unchanged");
}

#[test]
fn test_nop_does_not_affect_stack() {
    // NOP doesn't affect stack pointer
    let code = [
        0x90, // NOP
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x8000;
    let initial_rsp = regs.rsp;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rsp, initial_rsp,
        "RSP should be unchanged (stack setup preserved)"
    );
}

#[test]
fn test_nop_before_halt() {
    // NOP followed by HLT
    let code = [
        0x90, // NOP
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x123456789ABCDEF0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x123456789ABCDEF0, "RAX unchanged before HLT");
}

#[test]
fn test_nop_between_instructions() {
    // NOP between two MOV instructions
    let code = [
        0xb8, 0x42, 0x00, 0x00, 0x00, // MOV EAX, 0x42
        0x90, // NOP
        0x89, 0xc3, // MOV EBX, EAX
        0xf4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 0x42, "EAX should be 0x42");
    assert_eq!(
        regs.rbx & 0xFFFFFFFF,
        0x42,
        "EBX should be 0x42 (MOV succeeded)"
    );
}

#[test]
fn test_nop_with_immediate_register_values() {
    // NOP with immediate values in registers
    let code = [
        0x90, // NOP
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x12345678;
    regs.rbx = 0x87654321;
    regs.rcx = 0xDEADBEEF;
    regs.rdx = 0xCAFEBABE;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 0x12345678, "RAX unchanged");
    assert_eq!(regs.rbx & 0xFFFFFFFF, 0x87654321, "RBX unchanged");
    assert_eq!(regs.rcx & 0xFFFFFFFF, 0xDEADBEEF, "RCX unchanged");
    assert_eq!(regs.rdx & 0xFFFFFFFF, 0xCAFEBABE, "RDX unchanged");
}

#[test]
fn test_nop_no_memory_access() {
    // NOP doesn't access memory
    let code = [
        0x90, // NOP
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();

    // Just verify memory at DATA_ADDR wasn't touched
    let mem_value = read_mem_u32(&mem);
    assert_eq!(mem_value, 0, "Memory should not be accessed by NOP");
}

#[test]
fn test_nop_after_flags_modification() {
    // NOP after flag modification doesn't undo it
    let code = [
        0x83, 0xc0, 0x01, // ADD EAX, 1 (sets flags)
        0x90, // NOP
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 1, "EAX should be 1 after ADD");
    // Flags should have been set by ADD, not modified by NOP
    assert_eq!(zf_set(regs.rflags), false, "ZF should be clear");
}

#[test]
fn test_multiple_consecutive_90_nops() {
    // Multiple consecutive 0x90 NOPs (common alignment pattern)
    let code = [
        0x90, // NOP
        0x90, // NOP
        0x90, // NOP
        0x90, // NOP
        0x90, // NOP
        0x90, // NOP
        0x90, // NOP
        0x90, // NOP
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0xFFFFFFFFFFFFFFFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0xFFFFFFFFFFFFFFFF, "RAX unchanged after 8 NOPs");
}

#[test]
fn test_nop_preserves_rip_increment() {
    // NOP properly increments RIP for subsequent instructions
    let code = [
        0xb8, 0x11, 0x00, 0x00, 0x00, // MOV EAX, 0x11
        0x90, // NOP
        0xbb, 0x22, 0x00, 0x00, 0x00, // MOV EBX, 0x22
        0xf4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 0x11, "First MOV executed");
    assert_eq!(regs.rbx & 0xFFFFFFFF, 0x22, "Second MOV executed after NOP");
}

#[test]
fn test_nop_zero_value() {
    // NOP with zero register value
    let code = [
        0x90, // NOP
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0, "RAX should remain zero");
}

#[test]
fn test_nop_all_ones_value() {
    // NOP with all ones in 64-bit register
    let code = [
        0x90, // NOP
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0xFFFFFFFFFFFFFFFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0xFFFFFFFFFFFFFFFF, "RAX should have all ones");
}

#[test]
fn test_nop_sign_extended_value() {
    // NOP with sign-extended value
    let code = [
        0x90, // NOP
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0xFFFFFFFF80000000u64; // Sign-extended negative 32-bit value
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0xFFFFFFFF80000000u64, "RAX should be unchanged");
}

#[test]
fn test_nop_typical_alignment_before_code() {
    // Typical NOP padding before actual code
    let code = [
        0x90, // NOP (alignment)
        0x90, // NOP (alignment)
        0x90, // NOP (alignment)
        0xb8, 0x42, 0x00, 0x00, 0x00, // MOV EAX, 0x42 (actual code)
        0xf4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x42,
        "MOV executed correctly after NOP padding"
    );
}

#[test]
fn test_nop_does_not_cause_prefetch_issues() {
    // NOP doesn't cause instruction prefetch issues
    let code = [
        0x90, // NOP
        0x90, // NOP
        0x89, 0xd8, // MOV EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x12345678;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x12345678,
        "MOV correctly executed after NOPs"
    );
}

#[test]
fn test_nop_with_carry_flag_set() {
    // NOP with carry flag set
    let code = [
        0x90, // NOP
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rflags = 0x2 | 1; // CF set
    let initial_flags = regs.rflags;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(cf_set(regs.rflags), true, "CF should remain set");
    assert_eq!(regs.rflags, initial_flags, "Flags unchanged");
}

#[test]
fn test_nop_with_sign_flag_set() {
    // NOP with sign flag set
    let code = [
        0x90, // NOP
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rflags = 0x2 | (1 << 7); // SF set
    let initial_flags = regs.rflags;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(sf_set(regs.rflags), true, "SF should remain set");
    assert_eq!(regs.rflags, initial_flags, "Flags unchanged");
}

#[test]
fn test_nop_does_not_fault() {
    // NOP should execute without faulting
    let code = [
        0x90, // NOP
        0xf4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    // This should complete without panicking or returning an error
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_nop_pattern_for_code_caves() {
    // NOP pattern commonly used in code caves
    let code = [
        0x90, 0x90, 0x90, 0x90, // 4-byte NOP padding
        0xb8, 0xFF, 0xFF, 0xFF, 0xFF, // MOV EAX, 0xFFFFFFFF
        0xf4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0xFFFFFFFF,
        "Code after NOP padding executed correctly"
    );
}
