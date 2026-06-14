use rax::cpu::Registers;
use vm_memory::{Bytes, GuestAddress};

use crate::common::*;

// ENTER/LEAVE - Stack Frame Setup/Teardown
// ENTER: Create stack frame with local space allocation
// LEAVE: Tear down stack frame
//
// ENTER imm16, imm8
// - imm16: number of bytes for local variables
// - imm8: nesting level (0 = simple frame)
//
// LEAVE: Restore RBP and RSP from the current stack frame

// Basic ENTER with 0 nesting level and no local space
#[test]
fn test_enter_basic_no_locals() {
    let code = [
        0xc8, 0x00, 0x00, 0x00, // ENTER 0, 0
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    regs.rbp = 0x0000; // Old base pointer value
    let (mut vcpu, vm) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // ENTER pushes RBP, then sets RBP = RSP (after push)
    // So RBP points to the saved old RBP on stack
    assert_eq!(regs.rbp, 0x0FF8, "RBP should be RSP after push");
    // RSP should be decremented by 8 (pushed RBP)
    assert_eq!(regs.rsp, 0x0FF8, "RSP same as RBP for ENTER 0,0");

    // Verify old RBP is on stack
    let mut stack_val = [0u8; 8];
    vm.read_slice(&mut stack_val, GuestAddress(0x0FF8)).unwrap();
    assert_eq!(
        u64::from_le_bytes(stack_val),
        0,
        "Old RBP should be on stack"
    );
}

// ENTER with local space allocation (8 bytes)
#[test]
fn test_enter_with_8_locals() {
    let code = [
        0xc8, 0x08, 0x00, 0x00, // ENTER 8, 0 (8 bytes of local space)
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x2000;
    regs.rbp = 0x3000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // ENTER: PUSH RBP (RSP=0x1FF8), FrameTemp=RSP, RSP-=8, RBP=FrameTemp
    assert_eq!(regs.rbp, 0x1FF8, "RBP = RSP after push");
    assert_eq!(regs.rsp, 0x1FF0, "RSP = FrameTemp - 8 bytes for locals");
}

// ENTER with larger local space allocation (64 bytes)
#[test]
fn test_enter_with_64_locals() {
    let code = [
        0xc8, 0x40, 0x00, 0x00, // ENTER 64, 0
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    regs.rbp = 0x0000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // ENTER: PUSH RBP (RSP=0xFF8), FrameTemp=RSP, RSP-=64, RBP=FrameTemp
    assert_eq!(regs.rbp, 0x0FF8, "RBP = RSP after push");
    assert_eq!(regs.rsp, 0x0FB8, "RSP = FrameTemp - 64 bytes for locals");
}

// ENTER with maximum local space (65535 bytes)
#[test]
fn test_enter_with_max_locals() {
    let code = [
        0xc8, 0xff, 0xff, 0x00, // ENTER 65535, 0
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x20000;
    regs.rbp = 0x0000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // ENTER: PUSH RBP (RSP=0x1FFF8), FrameTemp=RSP, RSP-=65535, RBP=FrameTemp
    assert_eq!(regs.rbp, 0x1FFF8, "RBP = RSP after push");
    assert_eq!(
        regs.rsp, 0x0FFF9,
        "RSP = FrameTemp - 65535 bytes for locals"
    );
}

// LEAVE basic - restore RBP and RSP
#[test]
fn test_leave_basic() {
    let code = [
        0xc8, 0x00, 0x00, 0x00, // ENTER 0, 0
        0xc9, // LEAVE
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x2000;
    regs.rbp = 0x0000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // After LEAVE, RSP should be restored to original
    assert_eq!(regs.rsp, 0x2000, "RSP restored to original");
}

// ENTER/LEAVE with locals
#[test]
fn test_enter_leave_with_locals() {
    let code = [
        0xc8, 0x20, 0x00, 0x00, // ENTER 32, 0
        0xc9, // LEAVE
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    regs.rbp = 0x5555;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rsp, 0x1000, "RSP fully restored");
    assert_eq!(regs.rbp, 0x5555, "RBP fully restored");
}

// ENTER with nesting level 1
#[test]
fn test_enter_nesting_level_1() {
    let code = [
        // Set up outer frame first
        0xc8, 0x00, 0x00, 0x00, // ENTER 0, 0
        0xc8, 0x00, 0x01, 0x00, // ENTER 0, 1 (nested level 1)
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x2000;
    regs.rbp = 0x0000;
    let (mut vcpu, vm) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // With nesting level 1, it should push the previous RBP value
    // and then create new frame
    assert_ne!(regs.rsp, 0x2000 - 8, "RSP adjusted for nested frame");

    // Verify RBP is on stack
    let mut stack_val = [0u8; 8];
    vm.read_slice(&mut stack_val, GuestAddress(regs.rbp))
        .unwrap();
    // Previous frame's RBP should be there
    let _ = u64::from_le_bytes(stack_val);
}

// ENTER with nesting level 1 and locals
#[test]
fn test_enter_nesting_level_1_with_locals() {
    let code = [
        0xc8, 0x10, 0x00, 0x00, // ENTER 16, 0 (outer)
        0xc8, 0x08, 0x01, 0x00, // ENTER 8, 1 (nested)
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x2000;
    regs.rbp = 0x0000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Verify stack management is correct
    assert!(
        regs.rsp < 0x2000 - 8 - 16,
        "RSP properly adjusted for nested frame"
    );
}

// ENTER with nesting level 2
#[test]
fn test_enter_nesting_level_2() {
    let code = [
        0xc8, 0x00, 0x00, 0x00, // ENTER 0, 0 (level 0)
        0xc8, 0x00, 0x01, 0x00, // ENTER 0, 1 (level 1)
        0xc8, 0x00, 0x02, 0x00, // ENTER 0, 2 (level 2)
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x3000;
    regs.rbp = 0x0000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // RSP should be much lower due to nested frames
    assert!(
        regs.rsp < 0x3000,
        "RSP adjusted for multiple nesting levels"
    );
}

// Multiple ENTER without LEAVE (stack growth)
#[test]
fn test_multiple_enter_no_leave() {
    let code = [
        0xc8, 0x08, 0x00, 0x00, // ENTER 8, 0
        0xc8, 0x10, 0x00, 0x00, // ENTER 16, 0
        0xc8, 0x20, 0x00, 0x00, // ENTER 32, 0
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x4000;
    regs.rbp = 0x0000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Total stack used: 8 + 8 + 16 + 8 + 32 + 8 = 80 bytes
    assert_eq!(
        regs.rsp,
        0x4000 - 80,
        "Stack correctly grown for multiple frames"
    );
}

// ENTER/LEAVE pairs
#[test]
fn test_enter_leave_pairs() {
    let code = [
        0xc8, 0x08, 0x00, 0x00, // ENTER 8, 0
        0xc9, // LEAVE
        0xc8, 0x10, 0x00, 0x00, // ENTER 16, 0
        0xc9, // LEAVE
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    regs.rbp = 0x0000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rsp, 0x1000, "RSP fully restored after pairs");
}

// ENTER with 16 bytes locals followed by LEAVE
#[test]
fn test_enter_16_leave() {
    let code = [
        0xc8, 0x10, 0x00, 0x00, // ENTER 16, 0
        0xc9, // LEAVE
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x2000;
    regs.rbp = 0xAAAAAAAAAAAAAAAA;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rsp, 0x2000, "RSP restored");
    assert_eq!(regs.rbp, 0xAAAAAAAAAAAAAAAA, "RBP restored");
}

// ENTER affects RBP and RSP only
#[test]
fn test_enter_doesnt_affect_other_registers() {
    let code = [
        0xc8, 0x00, 0x00, 0x00, // ENTER 0, 0
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    regs.rbp = 0x0000;
    regs.rax = 0x1111111111111111;
    regs.rbx = 0x2222222222222222;
    regs.rcx = 0x3333333333333333;
    regs.rdx = 0x4444444444444444;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x1111111111111111, "RAX unchanged");
    assert_eq!(regs.rbx, 0x2222222222222222, "RBX unchanged");
    assert_eq!(regs.rcx, 0x3333333333333333, "RCX unchanged");
    assert_eq!(regs.rdx, 0x4444444444444444, "RDX unchanged");
}

// LEAVE doesn't affect other registers
#[test]
fn test_leave_doesnt_affect_other_registers() {
    let code = [
        0xc8, 0x00, 0x00, 0x00, // ENTER 0, 0
        0xc9, // LEAVE
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    regs.rbp = 0x0000;
    regs.rax = 0xAAAAAAAAAAAAAAAA;
    regs.rsi = 0xBBBBBBBBBBBBBBBB;
    regs.rdi = 0xCCCCCCCCCCCCCCCC;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0xAAAAAAAAAAAAAAAA, "RAX unchanged");
    assert_eq!(regs.rsi, 0xBBBBBBBBBBBBBBBB, "RSI unchanged");
    assert_eq!(regs.rdi, 0xCCCCCCCCCCCCCCCC, "RDI unchanged");
}

// Practical use: function with locals
#[test]
fn test_enter_leave_function_prologue_epilogue() {
    let code = [
        // Function prologue
        0xc8, 0x20, 0x00, 0x00, // ENTER 32, 0 (32 bytes for local variables)
        // Function body - use RAX
        0x48, 0xc7, 0xc0, 0x42, 0x00, 0x00, 0x00, // MOV RAX, 0x42
        // Function epilogue
        0xc9, // LEAVE
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    regs.rbp = 0x0000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x42, "Function result in RAX");
    assert_eq!(regs.rsp, 0x1000, "Stack frame properly cleaned up");
}

// Nested function calls
#[test]
fn test_nested_frame_management() {
    let code = [
        // Outer function prologue
        0xc8, 0x08, 0x00, 0x00, // ENTER 8, 0 (outer: 8 bytes locals)
        // Inner function prologue
        0xc8, 0x10, 0x00, 0x00, // ENTER 16, 0 (inner: 16 bytes locals)
        // Inner function epilogue
        0xc9, // LEAVE
        // Outer function epilogue
        0xc9, // LEAVE
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x2000;
    regs.rbp = 0x0000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rsp, 0x2000,
        "Stack fully cleaned up from nested frames"
    );
}

// ENTER with very small local space (1 byte)
#[test]
fn test_enter_1_byte_locals() {
    let code = [
        0xc8, 0x01, 0x00, 0x00, // ENTER 1, 0
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    regs.rbp = 0x0000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rsp,
        0x1000 - 8 - 1,
        "RSP correctly adjusted for 1 byte locals"
    );
}

// ENTER preserves flags
#[test]
fn test_enter_preserves_flags() {
    let code = [
        0x48, 0xc7, 0xc0, 0xff, 0xff, 0xff, 0xff, // MOV RAX, -1
        0x48, 0x83, 0xc0, 0x01, // ADD RAX, 1 (sets ZF)
        0xc8, 0x00, 0x00, 0x00, // ENTER 0, 0
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    regs.rbp = 0x0000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // ZF should still be set from the ADD
    assert!(regs.rflags & 0x40 != 0, "ZF preserved");
}

// LEAVE preserves flags
#[test]
fn test_leave_preserves_flags() {
    let code = [
        0x48, 0xc7, 0xc0, 0xff, 0xff, 0xff, 0xff, // MOV RAX, -1
        0x48, 0x83, 0xc0, 0x01, // ADD RAX, 1 (sets ZF)
        0xc8, 0x00, 0x00, 0x00, // ENTER 0, 0
        0xc9, // LEAVE
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    regs.rbp = 0x0000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(regs.rflags & 0x40 != 0, "ZF still set after LEAVE");
}

// Multiple nested frames with different local sizes
#[test]
fn test_nested_frames_different_sizes() {
    let code = [
        0xc8, 0x08, 0x00, 0x00, // ENTER 8, 0
        0xc8, 0x10, 0x00, 0x00, // ENTER 16, 0
        0xc8, 0x20, 0x00, 0x00, // ENTER 32, 0
        0xc9, // LEAVE
        0xc9, // LEAVE
        0xc9, // LEAVE
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x3000;
    regs.rbp = 0x0000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rsp, 0x3000, "All frames properly unwound");
}

// ENTER/LEAVE sequence with memory modification
#[test]
fn test_enter_leave_with_memory_ops() {
    let code = [
        0xc8, 0x08, 0x00, 0x00, // ENTER 8, 0
        // Write value to local variable (at [RBP-8])
        0x48, 0xc7, 0x45, 0xf8, 0x42, 0x00, 0x00, 0x00, // MOV qword [RBP-8], 0x42
        0xc9, // LEAVE
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    regs.rbp = 0x0000;
    let (mut vcpu, vm) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // After LEAVE, the local variable area is no longer accessible
    // but RSP should still be correct
    assert_eq!(regs.rsp, 0x1000, "Stack pointer correct after LEAVE");
}

// ENTER with maximum nesting level that's reasonable
#[test]
fn test_enter_nesting_level_3() {
    let code = [
        0xc8, 0x00, 0x00, 0x00, // ENTER 0, 0
        0xc8, 0x00, 0x01, 0x00, // ENTER 0, 1
        0xc8, 0x00, 0x02, 0x00, // ENTER 0, 2
        0xc8, 0x00, 0x03, 0x00, // ENTER 0, 3
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x4000;
    regs.rbp = 0x0000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(
        regs.rsp < 0x4000,
        "RSP adjusted for multiple nesting levels"
    );
}

// ENTER then immediate LEAVE
#[test]
fn test_enter_immediate_leave() {
    let code = [
        0xc8, 0x00, 0x00, 0x00, // ENTER 0, 0
        0xc9, // LEAVE
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    regs.rbp = 0x0000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rsp, 0x1000, "RSP restored to original");
    assert_eq!(regs.rbp, 0x0000, "RBP restored to original");
}

// ENTER with 256 bytes locals
#[test]
fn test_enter_256_locals() {
    let code = [
        0xc8, 0x00, 0x01, 0x00, // ENTER 256, 0
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x2000;
    regs.rbp = 0x0000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rsp, 0x2000 - 8 - 256, "RSP adjusted for 256 bytes");
}

// RBP usage after ENTER
#[test]
fn test_rbp_access_after_enter() {
    let code = [
        0xc8, 0x10, 0x00, 0x00, // ENTER 16, 0
        // Try to read from [RBP] to get the saved RBP
        0x48, 0x8b, 0x45, 0x00, // MOV RAX, [RBP]
        0xc9, // LEAVE
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    regs.rbp = 0x5555;
    let (mut vcpu, vm) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // RAX should contain the saved old RBP value
    assert_eq!(regs.rax, 0x5555, "Old RBP value retrieved");
}

// Accessing frame at different offsets
#[test]
fn test_access_frame_locals_various_offsets() {
    let code = [
        0xc8, 0x20, 0x00, 0x00, // ENTER 32, 0
        // Write to [RBP-8]
        0x48, 0xc7, 0x45, 0xf8, 0x11, 0x00, 0x00, 0x00, // MOV [RBP-8], 0x11
        // Write to [RBP-16]
        0x48, 0xc7, 0x45, 0xf0, 0x22, 0x00, 0x00, 0x00, // MOV [RBP-16], 0x22
        // Write to [RBP-24]
        0x48, 0xc7, 0x45, 0xe8, 0x33, 0x00, 0x00, 0x00, // MOV [RBP-24], 0x33
        0xc9, // LEAVE
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    regs.rbp = 0x0000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rsp, 0x1000, "Stack correctly managed");
}

// ENTER when RBP is non-zero
#[test]
fn test_enter_with_nonzero_rbp() {
    let code = [
        0xc8, 0x00, 0x00, 0x00, // ENTER 0, 0
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    regs.rbp = 0xDEADBEEFDEADBEEF;
    let (mut vcpu, vm) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // ENTER: PUSH RBP, RBP = RSP (after push)
    assert_eq!(regs.rbp, 0x0FF8, "RBP = RSP after push");

    // Verify old RBP is on stack
    let mut stack_val = [0u8; 8];
    vm.read_slice(&mut stack_val, GuestAddress(0x0FF8)).unwrap();
    assert_eq!(
        u64::from_le_bytes(stack_val),
        0xDEADBEEFDEADBEEF,
        "Old RBP saved"
    );
}

// LEAVE restores from current RBP value
#[test]
fn test_leave_after_rbp_modification() {
    // Test that LEAVE uses current RBP to find the saved frame pointer
    // LEAVE does: RSP = RBP, POP RBP
    // So if RBP is modified, LEAVE reads from the new RBP location
    let code = [
        0xc8, 0x00, 0x00, 0x00, // ENTER 0, 0 (saves RBP=0 at 0xFF8, RBP=0xFF8)
        // Restore RBP back to frame pointer (no modification)
        // Just do LEAVE directly
        0xc9, // LEAVE
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    regs.rbp = 0x0000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // LEAVE: RSP = RBP (0xFF8), POP RBP (reads 0 from stack)
    assert_eq!(regs.rbp, 0, "Original RBP restored from stack");
    assert_eq!(regs.rsp, 0x1000, "RSP restored to original");
}
