//! Tests for WAIT and FWAIT instructions.
//!
//! WAIT/FWAIT - Wait for FPU
//!
//! Causes the processor to check for and handle pending, unmasked, floating-point
//! exceptions before proceeding. FWAIT is an alternate mnemonic for WAIT.
//!
//! This instruction is useful for synchronizing exceptions in critical sections of code.
//!
//! Opcode: 9B
//!
//! Flags affected: None (C0-C3 undefined)
//!
//! Reference: docs/wait:fwait.txt

use crate::common::{run_until_hlt, setup_vm};

// ============================================================================
// WAIT/FWAIT (opcode 9B) - Wait for FPU
// ============================================================================

#[test]
fn test_wait_basic() {
    // Basic WAIT execution
    let code = vec![
        0x9b, // WAIT
        0xf4, // HLT
    ];

    let (mut vcpu, _mem) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
    // If we reach here, WAIT executed without exception
}

#[test]
fn test_fwait_basic() {
    // FWAIT is the same as WAIT (same opcode)
    let code = vec![
        0x9b, // FWAIT
        0xf4, // HLT
    ];

    let (mut vcpu, _mem) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_wait_no_fpu_exceptions() {
    // WAIT with no pending FPU exceptions
    let code = vec![
        0x9b, // WAIT
        0x9b, // WAIT (multiple)
        0x9b, // WAIT
        0xf4, // HLT
    ];

    let (mut vcpu, _mem) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_wait_before_fpu_instruction() {
    // WAIT before FPU instruction (common pattern)
    let code = vec![
        0x9b, // WAIT
        0xd9, 0xe8, // FLD1 (load +1.0)
        0xf4, // HLT
    ];

    let (mut vcpu, _mem) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_wait_after_fpu_instruction() {
    // WAIT after FPU instruction (synchronization pattern)
    let code = vec![
        0xd9, 0xe8, // FLD1
        0x9b, // WAIT
        0xf4, // HLT
    ];

    let (mut vcpu, _mem) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_wait_between_fpu_instructions() {
    // WAIT between FPU instructions
    let code = vec![
        0xd9, 0xe8, // FLD1
        0x9b, // WAIT
        0xd9, 0xe8, // FLD1 (again)
        0x9b, // WAIT
        0xf4, // HLT
    ];

    let (mut vcpu, _mem) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_wait_with_integer_instructions() {
    // WAIT mixed with integer instructions
    let code = vec![
        0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1
        0x9b, // WAIT
        0x48, 0xc7, 0xc1, 0x02, 0x00, 0x00, 0x00, // MOV RCX, 2
        0x9b, // WAIT
        0xf4, // HLT
    ];

    let (mut vcpu, _mem) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 1, "RAX should be 1");
    assert_eq!(regs.rcx, 2, "RCX should be 2");
}

#[test]
fn test_multiple_consecutive_waits() {
    // Multiple consecutive WAIT instructions
    let code = vec![
        0x9b, // WAIT
        0x9b, // WAIT
        0x9b, // WAIT
        0x9b, // WAIT
        0x9b, // WAIT
        0xf4, // HLT
    ];

    let (mut vcpu, _mem) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_wait_sequence_pattern() {
    // Pattern of WAIT and NOP
    let code = vec![
        0x9b, // WAIT
        0x90, // NOP
        0x9b, // WAIT
        0x90, // NOP
        0x9b, // WAIT
        0x90, // NOP
        0xf4, // HLT
    ];

    let (mut vcpu, _mem) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_wait_preserves_registers() {
    // Verify WAIT doesn't modify registers
    let code = vec![
        0x48, 0xc7, 0xc0, 0xAA, 0x00, 0x00, 0x00, // MOV RAX, 0xAA
        0x48, 0xc7, 0xc3, 0xBB, 0x00, 0x00, 0x00, // MOV RBX, 0xBB
        0x48, 0xc7, 0xc1, 0xCC, 0x00, 0x00, 0x00, // MOV RCX, 0xCC
        0x9b, // WAIT
        0xf4, // HLT
    ];

    let (mut vcpu, _mem) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0xAA, "RAX preserved");
    assert_eq!(regs.rbx, 0xBB, "RBX preserved");
    assert_eq!(regs.rcx, 0xCC, "RCX preserved");
}

#[test]
fn test_wait_with_memory_operations() {
    // WAIT with memory operations
    let code = vec![
        0x48, 0xc7, 0xc0, 0x42, 0x00, 0x00, 0x00, // MOV RAX, 0x42
        0x48, 0xa3, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV [0x2000], RAX
        0x9b, // WAIT
        0x48, 0xa1, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV RAX, [0x2000]
        0x9b, // WAIT
        0xf4, // HLT
    ];

    let (mut vcpu, _mem) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x42, "Value preserved through memory");
}

#[test]
fn test_wait_in_loop_pattern() {
    // WAIT in a simple loop-like pattern
    let code = vec![
        0x48, 0xc7, 0xc0, 0x03, 0x00, 0x00, 0x00, // MOV RAX, 3
        // loop:
        0x9b, // WAIT
        0x48, 0xff, 0xc8, // DEC RAX
        0x75, 0xfb, // JNZ loop (-5 bytes)
        0xf4, // HLT
    ];

    let (mut vcpu, _mem) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0, "Loop completed");
}

#[test]
fn test_wait_with_stack_operations() {
    // WAIT with stack operations
    let code = vec![
        0x48, 0xc7, 0xc0, 0x55, 0x00, 0x00, 0x00, // MOV RAX, 0x55
        0x50, // PUSH RAX
        0x9b, // WAIT
        0x58, // POP RAX
        0x9b, // WAIT
        0xf4, // HLT
    ];

    let (mut vcpu, _mem) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x55, "Stack operations work with WAIT");
}

#[test]
fn test_wait_rip_advancement() {
    // Verify RIP advances correctly after WAIT
    let code = vec![
        0x9b, // WAIT
        0x90, // NOP
        0x9b, // WAIT
        0xf4, // HLT
    ];

    let (mut vcpu, _mem) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
    // If we reach HLT, RIP advanced correctly through all instructions
}

#[test]
fn test_wait_followed_by_conditional_jump() {
    // WAIT before conditional jump
    let code = vec![
        0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1
        0x48, 0x85, 0xc0, // TEST RAX, RAX
        0x9b, // WAIT
        0x75, 0x05, // JNZ +5
        0x48, 0xc7, 0xc0, 0x00, 0x00, 0x00, 0x00, // MOV RAX, 0 (skipped)
        0xf4, // HLT
    ];

    let (mut vcpu, _mem) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 1, "Jump taken correctly after WAIT");
}

#[test]
fn test_wait_with_flags() {
    // Verify WAIT doesn't affect flags
    let code = vec![
        0xb8, 0xff, 0xff, 0xff, 0x7f, // MOV EAX, 0x7FFFFFFF
        0x83, 0xc0, 0x01, // ADD EAX, 1 (sets OF)
        0x9b, // WAIT (shouldn't affect flags)
        0x70, 0x05, // JO +5 (jump if overflow)
        0xb8, 0x00, 0x00, 0x00, 0x00, // MOV EAX, 0 (executed if no overflow)
        0xf4, // HLT
    ];

    let (mut vcpu, _mem) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x80000000, "Flags preserved after WAIT");
}

#[test]
fn test_wait_no_side_effects() {
    // Verify WAIT has no observable side effects beyond synchronization
    let code = vec![
        0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1
        0x48, 0xc7, 0xc3, 0x02, 0x00, 0x00, 0x00, // MOV RBX, 2
        0x9b, // WAIT
        0x48, 0x01, 0xd8, // ADD RAX, RBX
        0x9b, // WAIT
        0xf4, // HLT
    ];

    let (mut vcpu, _mem) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 3, "Computation correct with WAIT");
}

#[test]
fn test_wait_alignment_independent() {
    // WAIT at different alignments
    let code = vec![
        0x90, // NOP (alignment)
        0x9b, // WAIT
        0x90, // NOP
        0x90, // NOP
        0x9b, // WAIT
        0xf4, // HLT
    ];

    let (mut vcpu, _mem) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_wait_with_arithmetic() {
    // WAIT interspersed with arithmetic
    let code = vec![
        0x48, 0xc7, 0xc0, 0x0A, 0x00, 0x00, 0x00, // MOV RAX, 10
        0x9b, // WAIT
        0x48, 0x83, 0xc0, 0x05, // ADD RAX, 5
        0x9b, // WAIT
        0x48, 0x83, 0xe8, 0x03, // SUB RAX, 3
        0x9b, // WAIT
        0xf4, // HLT
    ];

    let (mut vcpu, _mem) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 12, "Arithmetic with WAIT");
}

#[test]
fn test_wait_long_sequence() {
    // Long sequence of WAITs
    let code = vec![
        0x9b, 0x9b, 0x9b, 0x9b, 0x9b, // 5 WAITs
        0x9b, 0x9b, 0x9b, 0x9b, 0x9b, // 5 WAITs
        0x9b, 0x9b, 0x9b, 0x9b, 0x9b, // 5 WAITs
        0xf4, // HLT
    ];

    let (mut vcpu, _mem) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_wait_with_xor() {
    // WAIT with XOR operations
    let code = vec![
        0x48, 0xc7, 0xc0, 0xAA, 0x00, 0x00, 0x00, // MOV RAX, 0xAA
        0x9b, // WAIT
        0x48, 0x31, 0xc0, // XOR RAX, RAX
        0x9b, // WAIT
        0xf4, // HLT
    ];

    let (mut vcpu, _mem) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0, "XOR with WAIT");
}

#[test]
fn test_wait_with_shift() {
    // WAIT with shift operations
    let code = vec![
        0x48, 0xc7, 0xc0, 0x08, 0x00, 0x00, 0x00, // MOV RAX, 8
        0x9b, // WAIT
        0x48, 0xd1, 0xe0, // SHL RAX, 1
        0x9b, // WAIT
        0xf4, // HLT
    ];

    let (mut vcpu, _mem) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 16, "Shift with WAIT");
}

#[test]
fn test_wait_dense_pattern() {
    // Very dense WAIT pattern
    let code = vec![
        0x9b, 0x90, 0x9b, 0x90, 0x9b, 0x90, // WAIT-NOP pattern
        0x9b, 0x90, 0x9b, 0x90, 0x9b, 0x90, 0xf4,
    ];

    let (mut vcpu, _mem) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_wait_with_compare() {
    // WAIT with comparison
    let code = vec![
        0x48, 0xc7, 0xc0, 0x0A, 0x00, 0x00, 0x00, // MOV RAX, 10
        0x48, 0xc7, 0xc3, 0x0A, 0x00, 0x00, 0x00, // MOV RBX, 10
        0x9b, // WAIT
        0x48, 0x39, 0xd8, // CMP RAX, RBX
        0x9b, // WAIT
        0x74, 0x05, // JE +5
        0x48, 0xc7, 0xc0, 0xFF, 0x00, 0x00, 0x00, // MOV RAX, 0xFF (not executed)
        0xf4, // HLT
    ];

    let (mut vcpu, _mem) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 10, "Compare with WAIT");
}

#[test]
fn test_wait_isolated() {
    // Single isolated WAIT
    let code = vec![
        0x9b, // WAIT
        0xf4, // HLT
    ];

    let (mut vcpu, _mem) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_wait_register_state() {
    // Comprehensive register state test with WAIT
    let code = vec![
        0x48, 0xc7, 0xc0, 0x11, 0x00, 0x00, 0x00, // MOV RAX, 0x11
        0x48, 0xc7, 0xc3, 0x22, 0x00, 0x00, 0x00, // MOV RBX, 0x22
        0x48, 0xc7, 0xc1, 0x33, 0x00, 0x00, 0x00, // MOV RCX, 0x33
        0x48, 0xc7, 0xc2, 0x44, 0x00, 0x00, 0x00, // MOV RDX, 0x44
        0x9b, // WAIT
        0xf4, // HLT
    ];

    let (mut vcpu, _mem) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x11, "RAX preserved");
    assert_eq!(regs.rbx, 0x22, "RBX preserved");
    assert_eq!(regs.rcx, 0x33, "RCX preserved");
    assert_eq!(regs.rdx, 0x44, "RDX preserved");
}
