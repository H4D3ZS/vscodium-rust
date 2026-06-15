use crate::common::{read_mem_at_u64, run_until_hlt, setup_vm, write_mem_at_u64};
use rax::cpu::Registers;

// Comprehensive tests for LEAVE instruction
//
// LEAVE - High Level Procedure Exit
// Opcode: C9
//
// LEAVE performs:
// 1. MOV RSP, RBP (restore stack pointer to frame pointer)
// 2. POP RBP (restore old frame pointer)
//
// This instruction is the complement of ENTER for function epilogues

// ============================================================================
// Basic LEAVE functionality
// ============================================================================

#[test]
fn test_leave_basic() {
    let code = [
        0xc9, // LEAVE
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    // Use addresses that don't conflict with CODE_ADDR (0x1000)
    regs.rsp = 0x2FF8; // Stack pointer below frame pointer
    regs.rbp = 0x3000; // Frame pointer
    let (mut vcpu, vm) = setup_vm(&code, Some(regs));

    // Set up saved RBP value on stack at RBP address
    write_mem_at_u64(&vm, 0x3000, 0x4000);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rsp, 0x3008, "RSP = old RBP + 8");
    assert_eq!(regs.rbp, 0x4000, "RBP restored from stack");
}

#[test]
fn test_leave_after_enter() {
    let code = [
        0xc8, 0x10, 0x00, 0x00, // ENTER 16, 0
        0xc9, // LEAVE
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    regs.rbp = 0x2000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rsp, 0x1000, "RSP restored to original");
    assert_eq!(regs.rbp, 0x2000, "RBP restored to original");
}

#[test]
fn test_leave_with_local_variables() {
    let code = [
        0xc8, 0x20, 0x00, 0x00, // ENTER 32, 0 (allocate space for locals)
        // Simulate using local variables
        0x48, 0xc7, 0x45, 0xf8, 0xaa, 0x00, 0x00, 0x00, // MOV QWORD [RBP-8], 0xAA
        0xc9, // LEAVE
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    regs.rbp = 0x2000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rsp, 0x1000, "RSP restored despite local var usage");
    assert_eq!(regs.rbp, 0x2000, "RBP restored");
}

#[test]
fn test_leave_small_frame() {
    let code = [
        0xc8, 0x08, 0x00, 0x00, // ENTER 8, 0
        0xc9, // LEAVE
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    regs.rbp = 0x2000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rsp, 0x1000, "RSP restored");
    assert_eq!(regs.rbp, 0x2000, "RBP restored");
}

#[test]
fn test_leave_large_frame() {
    let code = [
        0xc8, 0x00, 0x04, 0x00, // ENTER 1024, 0
        0xc9, // LEAVE
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x2000;
    regs.rbp = 0x3000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rsp, 0x2000, "RSP restored after large allocation");
    assert_eq!(regs.rbp, 0x3000, "RBP restored");
}

// ============================================================================
// LEAVE with nested function calls
// ============================================================================

#[test]
fn test_leave_nested_functions() {
    let code = [
        // Outer function
        0xc8, 0x10, 0x00, 0x00, // ENTER 16, 0
        // Inner function
        0xc8, 0x08, 0x00, 0x00, // ENTER 8, 0
        0xc9, // LEAVE (inner)
        0xc9, // LEAVE (outer)
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    regs.rbp = 0x2000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rsp, 0x1000, "RSP fully restored after nested calls");
    assert_eq!(regs.rbp, 0x2000, "RBP fully restored after nested calls");
}

#[test]
fn test_leave_triple_nested() {
    let code = [
        0xc8, 0x20, 0x00, 0x00, // ENTER 32, 0 (func1)
        0xc8, 0x10, 0x00, 0x00, // ENTER 16, 0 (func2)
        0xc8, 0x08, 0x00, 0x00, // ENTER 8, 0 (func3)
        0xc9, // LEAVE (func3)
        0xc9, // LEAVE (func2)
        0xc9, // LEAVE (func1)
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x2000;
    regs.rbp = 0x3000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rsp, 0x2000, "RSP restored through 3 levels");
    assert_eq!(regs.rbp, 0x3000, "RBP restored through 3 levels");
}

#[test]
fn test_leave_deep_nesting() {
    let code = [
        0xc8, 0x08, 0x00, 0x00, // ENTER 8, 0
        0xc8, 0x08, 0x00, 0x00, // ENTER 8, 0
        0xc8, 0x08, 0x00, 0x00, // ENTER 8, 0
        0xc8, 0x08, 0x00, 0x00, // ENTER 8, 0
        0xc8, 0x08, 0x00, 0x00, // ENTER 8, 0
        0xc9, // LEAVE
        0xc9, // LEAVE
        0xc9, // LEAVE
        0xc9, // LEAVE
        0xc9, // LEAVE
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x3000;
    regs.rbp = 0x4000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rsp, 0x3000, "RSP restored through deep nesting");
    assert_eq!(regs.rbp, 0x4000, "RBP restored through deep nesting");
}

// ============================================================================
// LEAVE preserves other registers
// ============================================================================

#[test]
fn test_leave_preserves_registers() {
    let code = [
        0x48, 0xc7, 0xc0, 0x11, 0x00, 0x00, 0x00, // MOV RAX, 0x11
        0x48, 0xc7, 0xc3, 0x22, 0x00, 0x00, 0x00, // MOV RBX, 0x22
        0x48, 0xc7, 0xc1, 0x33, 0x00, 0x00, 0x00, // MOV RCX, 0x33
        0xc8, 0x10, 0x00, 0x00, // ENTER 16, 0
        0xc9, // LEAVE
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x11, "RAX unchanged");
    assert_eq!(regs.rbx, 0x22, "RBX unchanged");
    assert_eq!(regs.rcx, 0x33, "RCX unchanged");
}

#[test]
fn test_leave_preserves_flags() {
    let code = [
        0xf9, // STC (set carry)
        0xc8, 0x08, 0x00, 0x00, // ENTER 8, 0
        0xc9, // LEAVE
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_ne!(regs.rflags & 0x01, 0, "CF should be preserved");
}

// ============================================================================
// LEAVE with manual stack frame setup
// ============================================================================

#[test]
fn test_leave_manual_frame() {
    let code = [
        // Manual ENTER equivalent
        0x55, // PUSH RBP
        0x48, 0x89, 0xe5, // MOV RBP, RSP
        0x48, 0x83, 0xec, 0x10, // SUB RSP, 16
        // LEAVE should undo all of this
        0xc9, // LEAVE
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    regs.rbp = 0x2000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rsp, 0x1000, "RSP restored");
    assert_eq!(regs.rbp, 0x2000, "RBP restored");
}

#[test]
fn test_leave_with_push_pop_in_function() {
    let code = [
        0xc8, 0x10, 0x00, 0x00, // ENTER 16, 0
        0x50, // PUSH RAX
        0x53, // PUSH RBX
        // Function body would be here
        0x5b, // POP RBX
        0x58, // POP RAX
        0xc9, // LEAVE
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    regs.rbp = 0x2000;
    regs.rax = 0xAAAA;
    regs.rbx = 0xBBBB;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rsp, 0x1000, "RSP restored");
    assert_eq!(regs.rbp, 0x2000, "RBP restored");
    assert_eq!(regs.rax, 0xAAAA, "RAX restored via PUSH/POP");
    assert_eq!(regs.rbx, 0xBBBB, "RBX restored via PUSH/POP");
}

// ============================================================================
// LEAVE without corresponding ENTER
// ============================================================================

#[test]
fn test_leave_standalone() {
    let code = [
        0xc9, // LEAVE (without prior ENTER)
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    // Use addresses that don't conflict with CODE_ADDR (0x1000)
    regs.rsp = 0x2FF8;
    regs.rbp = 0x3000;
    let (mut vcpu, vm) = setup_vm(&code, Some(regs));

    // Set up expected saved RBP
    write_mem_at_u64(&vm, 0x3000, 0x4000);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rsp, 0x3008, "RSP = RBP + 8");
    assert_eq!(regs.rbp, 0x4000, "RBP from stack");
}

#[test]
fn test_leave_with_modified_stack() {
    let code = [
        0xc8, 0x10, 0x00, 0x00, // ENTER 16, 0
        // Manually adjust RSP
        0x48, 0x83, 0xec, 0x08, // SUB RSP, 8
        0xc9, // LEAVE (should still work correctly)
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    regs.rbp = 0x2000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // LEAVE uses RBP, not RSP, so RSP modification doesn't matter
    assert_eq!(regs.rsp, 0x1000, "RSP restored via RBP");
    assert_eq!(regs.rbp, 0x2000, "RBP restored");
}

// ============================================================================
// LEAVE at different stack positions
// ============================================================================

#[test]
fn test_leave_high_stack_address() {
    let code = [
        0xc8, 0x20, 0x00, 0x00, // ENTER 32, 0
        0xc9, // LEAVE
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x100000;
    regs.rbp = 0x200000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rsp, 0x100000, "RSP restored at high address");
    assert_eq!(regs.rbp, 0x200000, "RBP restored");
}

#[test]
fn test_leave_low_stack_address() {
    let code = [
        0xc8, 0x10, 0x00, 0x00, // ENTER 16, 0
        0xc9, // LEAVE
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x100;
    regs.rbp = 0x200;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rsp, 0x100, "RSP restored at low address");
    assert_eq!(regs.rbp, 0x200, "RBP restored");
}

// ============================================================================
// LEAVE with zero-sized frames
// ============================================================================

#[test]
fn test_leave_zero_sized_frame() {
    let code = [
        0xc8, 0x00, 0x00, 0x00, // ENTER 0, 0
        0xc9, // LEAVE
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    regs.rbp = 0x2000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rsp, 0x1000, "RSP restored");
    assert_eq!(regs.rbp, 0x2000, "RBP restored");
}

// ============================================================================
// LEAVE in typical function epilogue patterns
// ============================================================================

#[test]
fn test_leave_typical_epilogue() {
    let code = [
        // Prologue
        0xc8, 0x18, 0x00, 0x00, // ENTER 24, 0
        // Function body
        0x48, 0xc7, 0xc0, 0x42, 0x00, 0x00, 0x00, // MOV RAX, 0x42 (return value)
        // Epilogue
        0xc9, // LEAVE
        0xf4, // HLT (would be RET)
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    regs.rbp = 0x2000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x42, "Return value set");
    assert_eq!(regs.rsp, 0x1000, "Stack cleaned up");
    assert_eq!(regs.rbp, 0x2000, "Frame pointer restored");
}

#[test]
fn test_leave_with_saved_registers() {
    let code = [
        // Prologue
        0xc8, 0x10, 0x00, 0x00, // ENTER 16, 0
        0x50, // PUSH RAX (callee-save)
        0x53, // PUSH RBX (callee-save)
        // Function body
        0x48, 0xc7, 0xc0, 0x11, 0x00, 0x00, 0x00, // MOV RAX, 0x11
        0x48, 0xc7, 0xc3, 0x22, 0x00, 0x00, 0x00, // MOV RBX, 0x22
        // Epilogue
        0x5b, // POP RBX
        0x58, // POP RAX
        0xc9, // LEAVE
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    regs.rbp = 0x2000;
    regs.rax = 0xAAAA;
    regs.rbx = 0xBBBB;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0xAAAA, "RAX restored");
    assert_eq!(regs.rbx, 0xBBBB, "RBX restored");
    assert_eq!(regs.rsp, 0x1000, "Stack balanced");
}

// ============================================================================
// LEAVE multiple times in sequence
// ============================================================================

#[test]
fn test_leave_sequence() {
    let code = [
        0xc8, 0x10, 0x00, 0x00, // ENTER 16, 0
        0xc9, // LEAVE
        0xc8, 0x10, 0x00, 0x00, // ENTER 16, 0 again
        0xc9, // LEAVE again
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    regs.rbp = 0x2000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rsp, 0x1000, "RSP restored after sequence");
    assert_eq!(regs.rbp, 0x2000, "RBP restored after sequence");
}

// ============================================================================
// LEAVE with frame pointer chain verification
// ============================================================================

#[test]
fn test_leave_frame_chain() {
    let code = [
        // Create frame chain
        0xc8, 0x00, 0x00, 0x00, // ENTER 0, 0 (frame 1)
        0xc8, 0x00, 0x00, 0x00, // ENTER 0, 0 (frame 2)
        0xc8, 0x00, 0x00, 0x00, // ENTER 0, 0 (frame 3)
        // Unwind chain
        0xc9, // LEAVE (frame 3)
        0xc9, // LEAVE (frame 2)
        0xc9, // LEAVE (frame 1)
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x2000;
    regs.rbp = 0x3000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rsp, 0x2000, "Full chain unwound");
    assert_eq!(regs.rbp, 0x3000, "Original frame pointer restored");
}

#[test]
fn test_leave_with_varying_frames() {
    let code = [
        0xc8, 0x40, 0x00, 0x00, // ENTER 64, 0
        0xc8, 0x20, 0x00, 0x00, // ENTER 32, 0
        0xc8, 0x10, 0x00, 0x00, // ENTER 16, 0
        0xc9, // LEAVE
        0xc9, // LEAVE
        0xc9, // LEAVE
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x2000;
    regs.rbp = 0x3000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rsp, 0x2000, "All frames unwound");
    assert_eq!(regs.rbp, 0x3000, "Frame pointer fully restored");
}

// ============================================================================
// LEAVE edge cases
// ============================================================================

#[test]
fn test_leave_rbp_equals_rsp() {
    let code = [
        0xc9, // LEAVE
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    // Use addresses that don't conflict with CODE_ADDR (0x1000)
    regs.rsp = 0x3000;
    regs.rbp = 0x3000; // RBP == RSP
    let (mut vcpu, vm) = setup_vm(&code, Some(regs));

    write_mem_at_u64(&vm, 0x3000, 0x4000);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rsp, 0x3008, "RSP incremented");
    assert_eq!(regs.rbp, 0x4000, "RBP loaded from stack");
}

#[test]
fn test_leave_with_zero_saved_rbp() {
    let code = [
        0xc8, 0x10, 0x00, 0x00, // ENTER 16, 0
        0xc9, // LEAVE
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    regs.rbp = 0x0000; // RBP is zero
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbp, 0x0000, "Zero RBP restored");
}

#[test]
fn test_leave_stack_grows_correctly() {
    let code = [
        0xc8, 0x00, 0x01, 0x00, // ENTER 256, 0
        0xc9, // LEAVE
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x2000;
    regs.rbp = 0x3000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rsp, 0x2000, "Large frame properly unwound");
}

// ============================================================================
// LEAVE with interleaved operations
// ============================================================================

#[test]
fn test_leave_after_stack_arithmetic() {
    let code = [
        0xc8, 0x20, 0x00, 0x00, // ENTER 32, 0
        // Do some stack operations
        0x48, 0x83, 0xec, 0x10, // SUB RSP, 16 (allocate more)
        0x48, 0x83, 0xc4, 0x10, // ADD RSP, 16 (deallocate)
        0xc9, // LEAVE
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    regs.rbp = 0x2000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rsp, 0x1000, "LEAVE restores correctly despite SUB/ADD");
}

#[test]
fn test_leave_after_many_pushes() {
    let code = [
        0xc8, 0x10, 0x00, 0x00, // ENTER 16, 0
        0x50, 0x50, 0x50, 0x50, // PUSH RAX x4
        0x58, 0x58, 0x58, 0x58, // POP RAX x4
        0xc9, // LEAVE
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    regs.rbp = 0x2000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rsp, 0x1000, "Stack balanced after PUSH/POP");
}

// ============================================================================
// LEAVE in real-world scenarios
// ============================================================================

#[test]
fn test_leave_recursive_function_simulation() {
    let code = [
        // Recursion level 1
        0xc8, 0x08, 0x00, 0x00, // ENTER 8, 0
        // Recursion level 2
        0xc8, 0x08, 0x00, 0x00, // ENTER 8, 0
        // Recursion level 3
        0xc8, 0x08, 0x00, 0x00, // ENTER 8, 0
        // Unwind
        0xc9, // LEAVE (level 3)
        0xc9, // LEAVE (level 2)
        0xc9, // LEAVE (level 1)
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x3000;
    regs.rbp = 0x4000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rsp, 0x3000, "Recursion fully unwound");
    assert_eq!(regs.rbp, 0x4000, "Base frame restored");
}

#[test]
fn test_leave_with_mixed_operations() {
    let code = [
        0xc8, 0x20, 0x00, 0x00, // ENTER 32, 0
        // Save registers
        0x50, // PUSH RAX
        0x53, // PUSH RBX
        // Do work
        0x48, 0xc7, 0xc0, 0x99, 0x00, 0x00, 0x00, // MOV RAX, 0x99
        // Restore registers
        0x5b, // POP RBX
        0x58, // POP RAX
        // Clean up frame
        0xc9, // LEAVE
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    regs.rbp = 0x2000;
    regs.rax = 0x1111;
    regs.rbx = 0x2222;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x1111, "RAX restored by POP");
    assert_eq!(regs.rbx, 0x2222, "RBX restored by POP");
    assert_eq!(regs.rsp, 0x1000, "Stack cleaned by LEAVE");
}

#[test]
fn test_leave_exception_handler_pattern() {
    let code = [
        0xc8, 0x10, 0x00, 0x00, // ENTER 16, 0 (exception handler frame)
        // Handler code
        0x48, 0xc7, 0x45, 0xf8, 0xff, 0x00, 0x00, 0x00, // MOV [RBP-8], 0xFF
        // Exit handler
        0xc9, // LEAVE
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    regs.rbp = 0x2000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rsp, 0x1000, "Handler frame unwound");
}

#[test]
fn test_leave_tail_call_preparation() {
    let code = [
        0xc8, 0x08, 0x00, 0x00, // ENTER 8, 0
        // Prepare for tail call
        0xc9, // LEAVE (clean up current frame before tail call)
        // Would JMP to tail call target here
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    regs.rbp = 0x2000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rsp, 0x1000, "Frame cleaned for tail call");
    assert_eq!(regs.rbp, 0x2000, "RBP restored for tail call");
}

#[test]
fn test_leave_coroutine_switch_pattern() {
    let code = [
        0xc8, 0x00, 0x00, 0x00, // ENTER 0, 0
        // Save context
        0x50, 0x53, 0x51, 0x52, // PUSH RAX, RBX, RCX, RDX
        // Restore context
        0x5a, 0x59, 0x5b, 0x58, // POP RDX, RCX, RBX, RAX
        0xc9, // LEAVE
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x2000;
    regs.rbp = 0x3000;
    regs.rax = 0xAA;
    regs.rbx = 0xBB;
    regs.rcx = 0xCC;
    regs.rdx = 0xDD;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rsp, 0x2000, "Context switch cleaned up");
    assert_eq!(regs.rax, 0xAA, "Registers preserved");
}

// ============================================================================
// Strengthened LEAVE tests (appended): exact semantics RSP:=RBP; POP RBP. The
// new RBP comes from [old RBP] and RSP ends at old RBP + 8.
// ============================================================================

#[test]
fn test_strict_leave_exact_semantics() {
    // RBP = 0x3FF8; [0x3FF8] = 0x5000 (caller's RBP). After LEAVE:
    //   RSP := RBP = 0x3FF8; RBP := [0x3FF8] = 0x5000; RSP := 0x3FF8 + 8 = 0x4000.
    let code = [0xc9, 0xf4]; // LEAVE
    let mut regs = Registers::default();
    regs.rsp = 0x3000; // arbitrary; LEAVE overwrites RSP from RBP first
    regs.rbp = 0x3FF8;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    write_mem_at_u64(&mem, 0x3FF8, 0x5000);
    let out = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(out.rbp, 0x5000, "RBP loaded from [old RBP]");
    assert_eq!(out.rsp, 0x4000, "RSP = old RBP + 8");
}

#[test]
fn test_strict_leave_with_local_storage() {
    // Simulate a frame with locals: RSP below RBP. LEAVE discards locals.
    // RBP=0x4000; [0x4000]=0xCAFE (caller RBP). RSP starts at 0x3FC0 (locals).
    let code = [0xc9, 0xf4]; // LEAVE
    let mut regs = Registers::default();
    regs.rsp = 0x3FC0;
    regs.rbp = 0x4000;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    write_mem_at_u64(&mem, 0x4000, 0xCAFE);
    let out = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(out.rbp, 0xCAFE, "caller RBP restored");
    assert_eq!(out.rsp, 0x4008, "locals discarded, RSP = RBP + 8");
}
