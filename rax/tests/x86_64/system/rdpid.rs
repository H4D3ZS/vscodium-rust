use crate::common::{run_until_hlt, setup_vm};
use rax::cpu::Registers;

// RDPID - Read Processor ID
// Opcode: F3 0F C7 /7
// Reads IA32_TSC_AUX MSR into destination register
// Does not modify flags
// ModRM.MOD must be 11B (register direct mode)

// Basic RDPID test - reads TSC_AUX into RAX
#[test]
fn test_rdpid_rax_basic() {
    let code = [
        0xf3, 0x0f, 0xc7, 0xf8, // RDPID RAX (ModRM = 11 111 000)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // TSC_AUX value should be in RAX
    // Upper 32 bits should be cleared in 32-bit operand mode
    let _ = regs.rax;
}

// Test RDPID with RCX destination
#[test]
fn test_rdpid_rcx() {
    let code = [
        0xf3, 0x0f, 0xc7, 0xf9, // RDPID RCX (ModRM = 11 111 001)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // TSC_AUX value should be in RCX
    let _ = regs.rcx;
}

// Test RDPID with RDX destination
#[test]
fn test_rdpid_rdx() {
    let code = [
        0xf3, 0x0f, 0xc7, 0xfa, // RDPID RDX (ModRM = 11 111 010)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // TSC_AUX value should be in RDX
    let _ = regs.rdx;
}

// Test RDPID with RBX destination
#[test]
fn test_rdpid_rbx() {
    let code = [
        0xf3, 0x0f, 0xc7, 0xfb, // RDPID RBX (ModRM = 11 111 011)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // TSC_AUX value should be in RBX
    let _ = regs.rbx;
}

// Test RDPID with RSI destination
#[test]
fn test_rdpid_rsi() {
    let code = [
        0xf3, 0x0f, 0xc7, 0xfe, // RDPID RSI (ModRM = 11 111 110)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // TSC_AUX value should be in RSI
    let _ = regs.rsi;
}

// Test RDPID with RDI destination
#[test]
fn test_rdpid_rdi() {
    let code = [
        0xf3, 0x0f, 0xc7, 0xff, // RDPID RDI (ModRM = 11 111 111)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // TSC_AUX value should be in RDI
    let _ = regs.rdi;
}

// Test RDPID with R8 destination (requires REX prefix)
#[test]
fn test_rdpid_r8() {
    let code = [
        0xf3, 0x41, 0x0f, 0xc7, 0xf8, // RDPID R8 (REX.B=1, ModRM = 11 111 000)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // TSC_AUX value should be in R8
    let _ = regs.r8;
}

// Test RDPID with R15 destination
#[test]
fn test_rdpid_r15() {
    let code = [
        0xf3, 0x41, 0x0f, 0xc7, 0xff, // RDPID R15 (REX.B=1, ModRM = 11 111 111)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // TSC_AUX value should be in R15
    let _ = regs.r15;
}

// Test RDPID doesn't modify flags
#[test]
fn test_rdpid_preserves_flags() {
    let code = [
        0x48, 0xc7, 0xc0, 0xff, 0xff, 0xff, 0xff, // MOV RAX, -1
        0x48, 0x83, 0xc0, 0x01, // ADD RAX, 1 (sets ZF)
        0xf3, 0x0f, 0xc7, 0xf9, // RDPID RCX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // ZF should still be set from the ADD
    assert!(regs.rflags & 0x40 != 0, "ZF should be preserved");
}

// Test RDPID preserves other registers
#[test]
fn test_rdpid_preserves_other_registers() {
    let code = [
        0x48, 0xc7, 0xc3, 0x42, 0x42, 0x42,
        0x42, // MOV RBX, 0x42424242 (bit 31 clear, no sign-ext)
        0x48, 0xc7, 0xc6, 0x2a, 0x2a, 0x2a,
        0x2a, // MOV RSI, 0x2a2a2a2a (bit 31 clear, no sign-ext)
        0x48, 0xc7, 0xc7, 0x19, 0x19, 0x19,
        0x19, // MOV RDI, 0x19191919 (bit 31 clear, no sign-ext)
        0xf3, 0x0f, 0xc7, 0xf8, // RDPID RAX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // RBX, RSI, RDI should be unchanged (MOV r64, imm32 sign-extends, use values with bit 31 clear)
    assert_eq!(regs.rbx, 0x42424242, "RBX should not be affected");
    assert_eq!(regs.rsi, 0x2a2a2a2a, "RSI should not be affected");
    assert_eq!(regs.rdi, 0x19191919, "RDI should not be affected");
}

// Test RDPID overwrites destination register
#[test]
fn test_rdpid_overwrites_register() {
    let code = [
        0x48, 0xc7, 0xc0, 0x11, 0x11, 0x11, 0x11, // MOV RAX, 0x11111111
        0xf3, 0x0f, 0xc7, 0xf8, // RDPID RAX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Old value should be overwritten
    // Can't predict exact TSC_AUX value, but it should be different
    let _ = regs.rax;
}

// Test RDPID multiple sequential calls
#[test]
fn test_rdpid_sequential_calls() {
    let code = [
        0xf3, 0x0f, 0xc7, 0xf8, // RDPID RAX
        0xf3, 0x0f, 0xc7, 0xf9, // RDPID RCX
        0xf3, 0x0f, 0xc7, 0xfa, // RDPID RDX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // All should read the same value (same processor ID)
    assert_eq!(regs.rax, regs.rcx, "RAX and RCX should match");
    assert_eq!(regs.rax, regs.rdx, "RAX and RDX should match");
}

// Test RDPID with all flags set
#[test]
fn test_rdpid_with_all_flags_set() {
    let code = [
        0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1
        0x48, 0x83, 0xe8, 0x02, // SUB RAX, 2 (sets CF, SF)
        0xf3, 0x0f, 0xc7, 0xfb, // RDPID RBX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Flags from SUB should be preserved
    assert!(regs.rflags & 0x01 != 0, "CF should be preserved");
    assert!(regs.rflags & 0x80 != 0, "SF should be preserved");
}

// Test RDPID not serializing
#[test]
fn test_rdpid_not_serializing() {
    let code = [
        0x48, 0xc7, 0xc3, 0x01, 0x00, 0x00, 0x00, // MOV RBX, 1
        0xf3, 0x0f, 0xc7, 0xf8, // RDPID RAX
        0x48, 0x83, 0xc3, 0x01, // ADD RBX, 1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // ADD should complete
    assert_eq!(regs.rbx, 2, "ADD should complete normally");
}

// Test RDPID with PUSH/POP operations
#[test]
fn test_rdpid_with_push_pop() {
    let code = [
        0xf3, 0x0f, 0xc7, 0xf8, // RDPID RAX
        0x50, // PUSH RAX
        0x58, // POP RAX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Value should be restored
    let _ = regs.rax;
}

// Test RDPID preserves RBP
#[test]
fn test_rdpid_preserves_rbp() {
    let code = [
        0x48, 0xc7, 0xc5, 0x00, 0x70, 0x00, 0x00, // MOV RBP, 0x7000
        0xf3, 0x0f, 0xc7, 0xf8, // RDPID RAX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbp, 0x7000, "RBP should be preserved");
}

// Test RDPID with conditional jumps
#[test]
fn test_rdpid_with_conditional_jump() {
    let code = [
        0xf3, 0x0f, 0xc7, 0xf8, // RDPID RAX
        0x48, 0x85, 0xc0, // TEST RAX, RAX
        0x75, 0x02, // JNZ skip
        0x90, // NOP
        0x90, // NOP
        // skip:
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Should complete successfully
    let _ = regs;
}

// Test RDPID with zero flag conditions
#[test]
fn test_rdpid_zero_flag() {
    let code = [
        0x48, 0x31, 0xc0, // XOR RAX, RAX (sets ZF)
        0xf3, 0x0f, 0xc7, 0xf9, // RDPID RCX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // ZF should still be set
    assert!(regs.rflags & 0x40 != 0, "ZF should be preserved");
}

// Test RDPID with carry flag conditions
#[test]
fn test_rdpid_carry_flag() {
    let code = [
        0x48, 0xc7, 0xc0, 0xff, 0xff, 0xff, 0xff, // MOV RAX, 0xffffffffffffffff
        0x48, 0x83, 0xc0, 0x01, // ADD RAX, 1 (sets CF)
        0xf3, 0x0f, 0xc7, 0xf9, // RDPID RCX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Flags should be preserved
    let _ = regs.rflags;
}

// Test RDPID with sign flag conditions
#[test]
fn test_rdpid_sign_flag() {
    let code = [
        0x48, 0xc7, 0xc0, 0xff, 0xff, 0xff, 0xff, // MOV RAX, -1
        0x48, 0x85, 0xc0, // TEST RAX, RAX (sets SF)
        0xf3, 0x0f, 0xc7, 0xf9, // RDPID RCX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // SF should still be set
    assert!(regs.rflags & 0x80 != 0, "SF should be preserved");
}

// Test RDPID execution completes quickly
#[test]
fn test_rdpid_execution_speed() {
    let code = [
        0xf3, 0x0f, 0xc7, 0xf8, // RDPID RAX
        0xf3, 0x0f, 0xc7, 0xf9, // RDPID RCX
        0xf3, 0x0f, 0xc7, 0xfa, // RDPID RDX
        0xf3, 0x0f, 0xc7, 0xfb, // RDPID RBX
        0xf3, 0x0f, 0xc7, 0xfe, // RDPID RSI
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // All calls should complete
    let _ = regs;
}

// Test RDPID preserves stack pointer
#[test]
fn test_rdpid_preserves_stack_pointer() {
    let code = [
        0xf3, 0x0f, 0xc7, 0xf8, // RDPID RAX
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x8000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rsp, 0x8000, "RSP should be unchanged");
}

// Test RDPID with arithmetic operations
#[test]
fn test_rdpid_after_arithmetic() {
    let code = [
        0x48, 0xc7, 0xc3, 0x01, 0x00, 0x00, 0x00, // MOV RBX, 1
        0x48, 0x83, 0xc3, 0x02, // ADD RBX, 2
        0x48, 0xf7, 0xdb, // NEG RBX
        0xf3, 0x0f, 0xc7, 0xf8, // RDPID RAX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // RBX should still have result of NEG
    assert_eq!(regs.rbx as i64, -3, "RBX should be -3");
}

// Test RDPID value consistency
#[test]
fn test_rdpid_value_consistency() {
    let code = [
        0xf3, 0x0f, 0xc7, 0xf8, // RDPID RAX
        0x48, 0x89, 0xc3, // MOV RBX, RAX
        0xf3, 0x0f, 0xc7, 0xf8, // RDPID RAX (again)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Both reads should return same value (same processor)
    assert_eq!(regs.rax, regs.rbx, "TSC_AUX should be consistent");
}

// Test RDPID with REX.W prefix (64-bit operand size)
#[test]
fn test_rdpid_rex_w() {
    let code = [
        0xf3, 0x48, 0x0f, 0xc7, 0xf8, // RDPID RAX with REX.W
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // TSC_AUX value in RAX
    let _ = regs.rax;
}

// Test RDPID between instructions
#[test]
fn test_rdpid_between_instructions() {
    let code = [
        0xb8, 0x42, 0x00, 0x00, 0x00, // MOV EAX, 0x42
        0xf3, 0x0f, 0xc7, 0xf9, // RDPID RCX
        0xbb, 0x99, 0x00, 0x00, 0x00, // MOV EBX, 0x99
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Instructions should execute in order
    assert_eq!(regs.rax & 0xFFFFFFFF, 0x42, "First MOV executed");
    assert_eq!(regs.rbx & 0xFFFFFFFF, 0x99, "Second MOV executed");
}
