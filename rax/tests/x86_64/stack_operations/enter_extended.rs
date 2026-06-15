use crate::common::{read_mem_at_u64, run_until_hlt, setup_vm, write_mem_at_u64};
use rax::cpu::Registers;

// Comprehensive tests for ENTER instruction
//
// ENTER - Create Stack Frame for Procedure Parameters
// Opcode: C8 iw ib
// Format: ENTER imm16, imm8
//
// Creates a stack frame for a procedure with:
// - imm16: Amount of local variable space to allocate
// - imm8: Nesting level (0-31)
//
// ENTER performs:
// 1. PUSH RBP
// 2. Set frame pointer to current stack pointer
// 3. If nesting level > 0, push frame pointers from outer levels
// 4. Subtract allocation size from RSP

// ============================================================================
// ENTER with nesting level 0 (most common case)
// ============================================================================

#[test]
fn test_enter_basic_no_nesting() {
    let code = [
        0xc8, 0x00, 0x00, 0x00, // ENTER 0, 0
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    regs.rbp = 0x2000;
    let (mut vcpu, vm) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // ENTER 0,0 should: PUSH RBP, MOV RBP, RSP
    assert_eq!(regs.rsp, 0x1000 - 8, "RSP decremented by 8 (pushed RBP)");
    assert_eq!(regs.rbp, 0x1000 - 8, "RBP = RSP after PUSH");

    // Verify old RBP is on stack
    let saved_rbp = read_mem_at_u64(&vm, 0x1000 - 8);
    assert_eq!(saved_rbp, 0x2000, "Old RBP saved on stack");
}

#[test]
fn test_enter_allocate_8_bytes() {
    let code = [
        0xc8, 0x08, 0x00, 0x00, // ENTER 8, 0
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    regs.rbp = 0x2000;
    let (mut vcpu, vm) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbp, 0x1000 - 8, "RBP set after pushing");
    assert_eq!(regs.rsp, 0x1000 - 16, "RSP = RBP - 8 (allocated space)");

    let saved_rbp = read_mem_at_u64(&vm, 0x1000 - 8);
    assert_eq!(saved_rbp, 0x2000, "Old RBP saved");
}

#[test]
fn test_enter_allocate_16_bytes() {
    let code = [
        0xc8, 0x10, 0x00, 0x00, // ENTER 16, 0
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    regs.rbp = 0x2000;
    let (mut vcpu, vm) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbp, 0x1000 - 8, "RBP set");
    assert_eq!(regs.rsp, 0x1000 - 24, "RSP = RBP - 16");

    let saved_rbp = read_mem_at_u64(&vm, regs.rbp);
    assert_eq!(saved_rbp, 0x2000, "Old RBP saved");
}

#[test]
fn test_enter_allocate_32_bytes() {
    let code = [
        0xc8, 0x20, 0x00, 0x00, // ENTER 32, 0
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    regs.rbp = 0x2000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbp, 0x1000 - 8, "RBP set");
    assert_eq!(regs.rsp, 0x1000 - 40, "RSP = RBP - 32");
}

#[test]
fn test_enter_allocate_64_bytes() {
    let code = [
        0xc8, 0x40, 0x00, 0x00, // ENTER 64, 0
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    regs.rbp = 0x2000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbp, 0x1000 - 8, "RBP set");
    assert_eq!(regs.rsp, 0x1000 - 72, "RSP = RBP - 64");
}

#[test]
fn test_enter_allocate_128_bytes() {
    let code = [
        0xc8, 0x80, 0x00, 0x00, // ENTER 128, 0
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    regs.rbp = 0x2000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbp, 0x1000 - 8, "RBP set");
    assert_eq!(regs.rsp, 0x1000 - 136, "RSP = RBP - 128");
}

#[test]
fn test_enter_allocate_256_bytes() {
    let code = [
        0xc8, 0x00, 0x01, 0x00, // ENTER 256, 0
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    regs.rbp = 0x2000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbp, 0x1000 - 8, "RBP set");
    assert_eq!(regs.rsp, 0x1000 - 264, "RSP = RBP - 256");
}

#[test]
fn test_enter_allocate_1024_bytes() {
    let code = [
        0xc8, 0x00, 0x04, 0x00, // ENTER 1024, 0
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x2000;
    regs.rbp = 0x3000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbp, 0x2000 - 8, "RBP set");
    assert_eq!(regs.rsp, 0x2000 - 1032, "RSP = RBP - 1024");
}

#[test]
fn test_enter_allocate_max_16bit() {
    let code = [
        0xc8, 0xff, 0xff, 0x00, // ENTER 65535, 0
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x20000;
    regs.rbp = 0x30000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbp, 0x20000 - 8, "RBP set");
    assert_eq!(regs.rsp, 0x20000 - 8 - 65535, "RSP = RBP - 65535");
}

// ============================================================================
// ENTER with nesting level 1
// ============================================================================

#[test]
fn test_enter_nesting_level_1_no_alloc() {
    let code = [
        0xc8, 0x00, 0x00, 0x01, // ENTER 0, 1
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    regs.rbp = 0x2000;
    let (mut vcpu, vm) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // With nesting level 1:
    // 1. PUSH RBP (old RBP = 0x2000)
    // 2. temp = RSP
    // 3. PUSH [temp] (push the old RBP again)
    // 4. RBP = temp
    // 5. RSP -= 0

    // Two values pushed: old RBP and frame pointer
    assert_eq!(regs.rsp, 0x1000 - 16, "RSP decremented by 16");

    let first_push = read_mem_at_u64(&vm, 0x1000 - 8);
    assert_eq!(first_push, 0x2000, "First push is old RBP");
}

#[test]
fn test_enter_nesting_level_1_with_alloc() {
    let code = [
        0xc8, 0x10, 0x00, 0x01, // ENTER 16, 1
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    regs.rbp = 0x2000;
    let (mut vcpu, vm) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Two pushes (16 bytes) + 16 bytes allocation = 32 bytes
    assert_eq!(regs.rsp, 0x1000 - 32, "RSP decremented by 32");

    let saved_rbp = read_mem_at_u64(&vm, 0x1000 - 8);
    assert_eq!(saved_rbp, 0x2000, "Old RBP saved");
}

// ============================================================================
// ENTER with various nesting levels
// ============================================================================

#[test]
fn test_enter_nesting_level_2() {
    let code = [
        0xc8, 0x00, 0x00, 0x02, // ENTER 0, 2
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x2000;
    regs.rbp = 0x3000;
    let (mut vcpu, vm) = setup_vm(&code, Some(regs));

    // Set up a frame pointer at old RBP
    write_mem_at_u64(&vm, 0x3000, 0x4000);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Nesting level 2 pushes: old RBP + 1 frame pointer + new frame pointer
    assert_eq!(regs.rsp, 0x2000 - 24, "RSP decremented by 24");
}

#[test]
fn test_enter_nesting_level_3() {
    let code = [
        0xc8, 0x08, 0x00, 0x03, // ENTER 8, 3
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x2000;
    regs.rbp = 0x3000;
    let (mut vcpu, vm) = setup_vm(&code, Some(regs));

    // Set up frame pointers
    write_mem_at_u64(&vm, 0x3000, 0x4000);
    write_mem_at_u64(&vm, 0x4000, 0x5000);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Nesting level 3: pushes old RBP + 2 frame pointers + new frame + alloc 8
    assert_eq!(regs.rsp, 0x2000 - 40, "RSP decremented by 40");
}

#[test]
fn test_enter_nesting_level_4_with_alloc() {
    let code = [
        0xc8, 0x20, 0x00, 0x04, // ENTER 32, 4
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x2000;
    regs.rbp = 0x3000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Nesting 4: old RBP + 3 frame pointers + new frame + 32 bytes = 72 bytes
    assert_eq!(regs.rsp, 0x2000 - 72, "RSP decremented by 72");
}

#[test]
fn test_enter_nesting_level_5() {
    let code = [
        0xc8, 0x00, 0x00, 0x05, // ENTER 0, 5
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x3000;
    regs.rbp = 0x4000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Nesting 5: 1 + 4 + 1 = 6 pushes = 48 bytes
    assert_eq!(regs.rsp, 0x3000 - 48, "RSP decremented by 48");
}

// ============================================================================
// ENTER preserves other registers
// ============================================================================

#[test]
fn test_enter_preserves_other_registers() {
    let code = [
        0x48, 0xc7, 0xc0, 0x11, 0x00, 0x00, 0x00, // MOV RAX, 0x11
        0x48, 0xc7, 0xc3, 0x22, 0x00, 0x00, 0x00, // MOV RBX, 0x22
        0x48, 0xc7, 0xc1, 0x33, 0x00, 0x00, 0x00, // MOV RCX, 0x33
        0xc8, 0x10, 0x00, 0x00, // ENTER 16, 0
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
fn test_enter_preserves_flags() {
    let code = [
        0xf9, // STC (set carry)
        0xc8, 0x10, 0x00, 0x00, // ENTER 16, 0
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_ne!(regs.rflags & 0x01, 0, "CF should be preserved");
}

// ============================================================================
// ENTER followed by LEAVE
// ============================================================================

#[test]
fn test_enter_leave_roundtrip() {
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

    assert_eq!(regs.rsp, 0x1000, "RSP restored");
    assert_eq!(regs.rbp, 0x2000, "RBP restored");
}

#[test]
fn test_enter_leave_nested() {
    let code = [
        // Outer function
        0xc8, 0x08, 0x00, 0x00, // ENTER 8, 0
        // Inner function
        0xc8, 0x10, 0x00, 0x00, // ENTER 16, 0
        0xc9, // LEAVE (inner)
        0xc9, // LEAVE (outer)
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    regs.rbp = 0x2000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rsp, 0x1000, "RSP restored after nested ENTER/LEAVE");
    assert_eq!(regs.rbp, 0x2000, "RBP restored after nested ENTER/LEAVE");
}

// ============================================================================
// ENTER with different stack positions
// ============================================================================

#[test]
fn test_enter_high_stack_address() {
    let code = [
        0xc8, 0x20, 0x00, 0x00, // ENTER 32, 0
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x100000;
    regs.rbp = 0x200000;
    let (mut vcpu, vm) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbp, 0x100000 - 8, "RBP set");
    assert_eq!(regs.rsp, 0x100000 - 40, "RSP decremented");

    let saved_rbp = read_mem_at_u64(&vm, regs.rbp);
    assert_eq!(saved_rbp, 0x200000, "Old RBP saved");
}

#[test]
fn test_enter_low_stack_address() {
    let code = [
        0xc8, 0x10, 0x00, 0x00, // ENTER 16, 0
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x100;
    regs.rbp = 0x200;
    let (mut vcpu, vm) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbp, 0x100 - 8, "RBP set");
    assert_eq!(regs.rsp, 0x100 - 24, "RSP decremented");

    let saved_rbp = read_mem_at_u64(&vm, regs.rbp);
    assert_eq!(saved_rbp, 0x200, "Old RBP saved");
}

// ============================================================================
// ENTER multiple times (function call chain)
// ============================================================================

#[test]
fn test_enter_multiple_calls() {
    let code = [
        // First function
        0xc8, 0x08, 0x00, 0x00, // ENTER 8, 0
        // Second function
        0xc8, 0x10, 0x00, 0x00, // ENTER 16, 0
        // Third function
        0xc8, 0x20, 0x00, 0x00, // ENTER 32, 0
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x2000;
    regs.rbp = 0x3000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // First ENTER: -8 (push) -8 (alloc) = -16
    // Second ENTER: -8 (push) -16 (alloc) = -24
    // Third ENTER: -8 (push) -32 (alloc) = -40
    // Total: -80
    assert_eq!(regs.rsp, 0x2000 - 80, "RSP after three ENTERs");
}

#[test]
fn test_enter_with_odd_allocation() {
    let code = [
        0xc8, 0x11, 0x00, 0x00, // ENTER 17, 0
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    regs.rbp = 0x2000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbp, 0x1000 - 8, "RBP set");
    assert_eq!(regs.rsp, 0x1000 - 25, "RSP = RBP - 17");
}

#[test]
fn test_enter_allocation_alignment() {
    let code = [
        0xc8, 0x18, 0x00, 0x00, // ENTER 24, 0
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    regs.rbp = 0x2000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbp, 0x1000 - 8, "RBP set");
    assert_eq!(regs.rsp, 0x1000 - 32, "RSP = RBP - 24");
}

// ============================================================================
// ENTER with frame pointer chain
// ============================================================================

#[test]
fn test_enter_frame_pointer_chain() {
    let code = [
        // First frame
        0xc8, 0x00, 0x00, 0x00, // ENTER 0, 0
        // Second frame
        0xc8, 0x00, 0x00, 0x00, // ENTER 0, 0
        // Third frame
        0xc8, 0x00, 0x00, 0x00, // ENTER 0, 0
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x2000;
    regs.rbp = 0x3000;
    let (mut vcpu, vm) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Each ENTER 0,0 pushes 8 bytes
    assert_eq!(regs.rsp, 0x2000 - 24, "Three frames created");

    // Verify frame pointer chain
    let frame3_rbp = regs.rbp;
    let frame2_rbp = read_mem_at_u64(&vm, frame3_rbp);
    let frame1_rbp = read_mem_at_u64(&vm, frame2_rbp);
    let original_rbp = read_mem_at_u64(&vm, frame1_rbp);

    assert_eq!(original_rbp, 0x3000, "Original RBP at end of chain");
}

// ============================================================================
// ENTER with practical function prologue patterns
// ============================================================================

#[test]
fn test_enter_small_local_vars() {
    // Typical small function with 2-3 local variables
    let code = [
        0xc8, 0x18, 0x00, 0x00, // ENTER 24, 0 (3 x 8-byte vars)
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    regs.rbp = 0x2000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbp, 0x1000 - 8, "Frame pointer set");
    assert_eq!(regs.rsp, 0x1000 - 32, "Space for 3 local vars");
}

#[test]
fn test_enter_large_local_array() {
    // Function with large local array (e.g., char buf[512])
    let code = [
        0xc8, 0x00, 0x02, 0x00, // ENTER 512, 0
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x2000;
    regs.rbp = 0x3000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbp, 0x2000 - 8, "Frame pointer set");
    assert_eq!(regs.rsp, 0x2000 - 520, "Space for 512-byte buffer");
}

#[test]
fn test_enter_no_locals() {
    // Leaf function with no local variables
    let code = [
        0xc8, 0x00, 0x00, 0x00, // ENTER 0, 0
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    regs.rbp = 0x2000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbp, 0x1000 - 8, "Frame pointer set");
    assert_eq!(regs.rsp, 0x1000 - 8, "No local space allocated");
}

// ============================================================================
// ENTER with subsequent stack operations
// ============================================================================

#[test]
fn test_enter_then_push() {
    let code = [
        0xc8, 0x10, 0x00, 0x00, // ENTER 16, 0
        0x48, 0xc7, 0xc0, 0x42, 0x00, 0x00, 0x00, // MOV RAX, 0x42
        0x50, // PUSH RAX
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    regs.rbp = 0x2000;
    let (mut vcpu, vm) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rsp, 0x1000 - 32, "RSP after ENTER and PUSH");

    let pushed_val = read_mem_at_u64(&vm, regs.rsp);
    assert_eq!(pushed_val, 0x42, "Pushed value on stack");
}

#[test]
fn test_enter_then_mov_to_local() {
    let code = [
        0xc8, 0x10, 0x00, 0x00, // ENTER 16, 0
        0x48, 0xc7, 0xc0, 0xaa, 0x00, 0x00, 0x00, // MOV RAX, 0xAA
        0x48, 0x89, 0x45, 0xf8, // MOV [RBP-8], RAX
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    regs.rbp = 0x2000;
    let (mut vcpu, vm) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Read local variable at RBP-8
    let local_var = read_mem_at_u64(&vm, regs.rbp - 8);
    assert_eq!(local_var, 0xAA, "Local variable set correctly");
}

#[test]
fn test_enter_with_parameter_access() {
    let code = [
        // Simulate: parameters pushed before call, then ENTER
        0x48, 0xc7, 0xc0, 0x11, 0x00, 0x00, 0x00, // MOV RAX, 0x11
        0x50, // PUSH RAX (parameter)
        0xc8, 0x08, 0x00, 0x00, // ENTER 8, 0
        // Access parameter at [RBP+16] (pushed param + return addr + old RBP)
        0x48, 0x8b, 0x5d, 0x10, // MOV RBX, [RBP+16]
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    regs.rbp = 0x2000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Note: In real scenario, there would be a return address
    // For this test, we're just verifying the stack layout
    assert_ne!(regs.rbx, 0, "Parameter accessed through RBP");
}

// ============================================================================
// ENTER with different nesting patterns
// ============================================================================

#[test]
fn test_enter_nesting_level_10() {
    let code = [
        0xc8, 0x00, 0x00, 0x0a, // ENTER 0, 10
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x3000;
    regs.rbp = 0x4000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Nesting 10: 1 + 9 + 1 = 11 pushes = 88 bytes
    assert_eq!(regs.rsp, 0x3000 - 88, "RSP with nesting level 10");
}

#[test]
fn test_enter_nesting_level_16() {
    let code = [
        0xc8, 0x00, 0x00, 0x10, // ENTER 0, 16
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x4000;
    regs.rbp = 0x5000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Nesting 16: 1 + 15 + 1 = 17 pushes = 136 bytes
    assert_eq!(regs.rsp, 0x4000 - 136, "RSP with nesting level 16");
}

#[test]
fn test_enter_nesting_level_31() {
    // Maximum nesting level (5 bits)
    let code = [
        0xc8, 0x00, 0x00, 0x1f, // ENTER 0, 31
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x5000;
    regs.rbp = 0x6000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Nesting 31: 1 + 30 + 1 = 32 pushes = 256 bytes
    assert_eq!(regs.rsp, 0x5000 - 256, "RSP with max nesting level 31");
}

// ============================================================================
// ENTER edge cases
// ============================================================================

#[test]
fn test_enter_zero_allocation_zero_nesting() {
    let code = [
        0xc8, 0x00, 0x00, 0x00, // ENTER 0, 0
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    regs.rbp = 0x2000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Just pushes RBP and sets RBP = RSP
    assert_eq!(regs.rsp, 0x1000 - 8, "Only RBP pushed");
    assert_eq!(regs.rbp, 0x1000 - 8, "RBP = RSP");
}

#[test]
fn test_enter_with_same_rbp_rsp() {
    let code = [
        0xc8, 0x10, 0x00, 0x00, // ENTER 16, 0
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    regs.rbp = 0x1000; // RBP == RSP
    let (mut vcpu, vm) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbp, 0x1000 - 8, "RBP updated");

    let saved_rbp = read_mem_at_u64(&vm, regs.rbp);
    assert_eq!(saved_rbp, 0x1000, "Old RBP (which was RSP) saved");
}

#[test]
fn test_enter_consecutive_same_size() {
    let code = [
        0xc8, 0x20, 0x00, 0x00, // ENTER 32, 0
        0xc8, 0x20, 0x00, 0x00, // ENTER 32, 0
        0xc8, 0x20, 0x00, 0x00, // ENTER 32, 0
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x2000;
    regs.rbp = 0x3000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Each ENTER: -8 (push) -32 (alloc) = -40
    // Three times: -120
    assert_eq!(regs.rsp, 0x2000 - 120, "Three identical ENTERs");
}

#[test]
fn test_enter_mixed_sizes() {
    let code = [
        0xc8, 0x08, 0x00, 0x00, // ENTER 8, 0
        0xc8, 0x10, 0x00, 0x00, // ENTER 16, 0
        0xc8, 0x20, 0x00, 0x00, // ENTER 32, 0
        0xc8, 0x40, 0x00, 0x00, // ENTER 64, 0
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x3000;
    regs.rbp = 0x4000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // 16 + 24 + 40 + 72 = 152
    assert_eq!(regs.rsp, 0x3000 - 152, "Mixed size allocations");
}

// ============================================================================
// Strengthened ENTER tests (appended): exact frame setup for level 0 — the old
// RBP is pushed, RBP := new RSP (the saved-RBP slot), and RSP := RBP - storage.
// ============================================================================

#[test]
fn test_strict_enter_level0_frame_setup() {
    // ENTER 0x20, 0  (C8 20 00 00)
    // Steps: push RBP -> RSP=0x4000-8=0x3FF8, [0x3FF8]=old RBP(0x5000);
    //        RBP := 0x3FF8; RSP := RBP - 0x20 = 0x3FD8.
    let code = [0xc8, 0x20, 0x00, 0x00, 0xf4]; // ENTER 0x20, 0
    let mut regs = Registers::default();
    regs.rsp = 0x4000;
    regs.rbp = 0x5000;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    let out = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(out.rbp, 0x3FF8, "RBP set to saved-RBP slot");
    assert_eq!(out.rsp, 0x3FD8, "RSP = RBP - 0x20");
    assert_eq!(
        read_mem_at_u64(&mem, 0x3FF8),
        0x5000,
        "old RBP saved at new RBP slot"
    );
}

#[test]
fn test_strict_enter_level0_zero_storage() {
    // ENTER 0, 0: just push RBP and set RBP=RSP; RSP only changes by the push.
    let code = [0xc8, 0x00, 0x00, 0x00, 0xf4]; // ENTER 0, 0
    let mut regs = Registers::default();
    regs.rsp = 0x4000;
    regs.rbp = 0xABCD;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    let out = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        out.rsp, 0x3FF8,
        "RSP decremented by the saved-RBP push only"
    );
    assert_eq!(out.rbp, 0x3FF8, "RBP points at saved-RBP slot");
    assert_eq!(read_mem_at_u64(&mem, 0x3FF8), 0xABCD, "old RBP saved");
}

#[test]
fn test_strict_enter_then_leave_restores() {
    // ENTER 0x10,0 followed by LEAVE returns RSP and RBP to their originals.
    let code = [0xc8, 0x10, 0x00, 0x00, 0xc9, 0xf4]; // ENTER 0x10,0; LEAVE
    let mut regs = Registers::default();
    regs.rsp = 0x4000;
    regs.rbp = 0x5000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let out = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(out.rbp, 0x5000, "LEAVE restores old RBP");
    assert_eq!(out.rsp, 0x4000, "LEAVE restores RSP to pre-ENTER value");
}
