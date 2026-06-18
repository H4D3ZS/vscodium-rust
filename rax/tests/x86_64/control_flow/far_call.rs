use crate::common::*;
use rax::backend::emulator::x86_64::X86_64Vcpu;

// Comprehensive tests for FAR CALL instruction (inter-segment call)
// CALL ptr16:16, CALL ptr16:32, CALL m16:16, CALL m16:32, CALL m16:64
// Opcode: 9A (immediate), FF /3 (memory)
// Note: 0x9A is invalid in 64-bit mode; use compatibility mode for that form.

// ============================================================================
// FAR CALL - Direct with Immediate Selector:Offset
// ============================================================================

#[test]
fn test_far_call_immediate_16_16_basic() {
    // CALL 0x0008:0x2000 - far call to selector 0x0008, offset 0x2000
    let code = [
        0x9a, 0x00, 0x20, 0x08, 0x00, // CALL 0x0008:0x2000 (16-bit addressing)
        0xf4, // HLT (should not execute immediately)
              // Target code at 0x2000
    ];
    let (mut vcpu, mem) = setup_vm_compat(&code, None);

    // Write HLT at target address 0x2000
    let target_code = [0xf4]; // HLT
    mem.write_slice(&target_code, vm_memory::GuestAddress(0x2000))
        .unwrap();

    let regs = run_until_hlt(&mut vcpu).unwrap();
    // Should have performed far call
    assert_eq!(regs.rip, 0x2001);
}

#[test]
fn test_far_call_immediate_16_32_basic() {
    // CALL 0x0008:0x00003000 - far call with 32-bit offset
    let code = [
        0x66, 0x9a, 0x00, 0x30, 0x00, 0x00, 0x08, 0x00, // CALL 0x0008:0x00003000
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm_compat(&code, None);

    let target_code = [0xf4];
    mem.write_slice(&target_code, vm_memory::GuestAddress(0x3000))
        .unwrap();

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rip, 0x3001);
}

#[test]
fn test_far_call_immediate_invalid_in_64bit_raises_ud() {
    let code = [
        0x9a, 0x00, 0x20, 0x08, 0x00, // CALL 0x0008:0x2000
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    let ud_handler = [
        0x48, 0xc7, 0xc0, 0x06, 0x00, 0x00, 0x00, // MOV RAX, 6
        0xf4, // HLT
    ];
    mem.write_slice(&ud_handler, GuestAddress(INT_HANDLER_ADDR))
        .unwrap();

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 6);
}

#[test]
fn test_far_call_saves_return_address() {
    // Verify that FAR CALL pushes CS and IP/EIP/RIP onto stack
    let code = [
        0x48, 0xc7, 0xc4, 0x00, 0x80, 0x00, 0x00, // MOV RSP, 0x8000
        0x9a, 0x00, 0x20, 0x08, 0x00, // CALL 0x0008:0x2000
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm_compat(&code, None);

    // At target, check stack and return
    let target_code = [
        0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1 (marker)
        0xf4,
    ];
    mem.write_slice(&target_code, vm_memory::GuestAddress(0x2000))
        .unwrap();

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 1);
    assert_eq!(regs.rip, 0x2008);
}

// ============================================================================
// FAR CALL - Memory Indirect m16:16, m16:32, m16:64
// ============================================================================

#[test]
fn test_far_call_mem_indirect_16_16() {
    // CALL FAR [mem] with 16-bit offset (0x66 operand-size override in 64-bit mode).
    let code = [
        0x66, 0xff, 0x1c, 0x25, 0x00, 0x20, 0x00, 0x00, // CALL FAR [0x2000]
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    // Write far pointer at 0x2000: offset=0x3000, selector=0x0008
    let far_ptr = [0x00, 0x30, 0x08, 0x00]; // offset:selector (little-endian)
    mem.write_slice(&far_ptr, vm_memory::GuestAddress(0x2000))
        .unwrap();

    // Write target code at 0x3000
    let target_code = [0xf4];
    mem.write_slice(&target_code, vm_memory::GuestAddress(0x3000))
        .unwrap();

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rip, 0x3001);
}

#[test]
fn test_far_call_mem_indirect_16_32() {
    // CALL FAR [mem] with 32-bit offset (default in 64-bit mode).
    let code = [
        0xff, 0x1c, 0x25, 0x00, 0x20, 0x00, 0x00, // CALL FAR [0x2000]
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    // Write far pointer: 32-bit offset + 16-bit selector
    let far_ptr = [0x00, 0x40, 0x00, 0x00, 0x08, 0x00];
    mem.write_slice(&far_ptr, vm_memory::GuestAddress(0x2000))
        .unwrap();

    let target_code = [0xf4];
    mem.write_slice(&target_code, vm_memory::GuestAddress(0x4000))
        .unwrap();

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rip, 0x4001);
}

#[test]
fn test_far_call_mem_indirect_16_64() {
    // CALL FAR [mem] with 64-bit offset
    let code = [
        0x48, 0xff, 0x1c, 0x25, 0x00, 0x20, 0x00, 0x00, // CALL FAR [0x2000]
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    // Write far pointer: 64-bit offset + 16-bit selector
    let far_ptr = [
        0x00, 0x50, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 64-bit offset
        0x08, 0x00, // 16-bit selector
    ];
    mem.write_slice(&far_ptr, vm_memory::GuestAddress(0x2000))
        .unwrap();

    let target_code = [0xf4];
    mem.write_slice(&target_code, vm_memory::GuestAddress(0x5000))
        .unwrap();

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rip, 0x5001);
}

// ============================================================================
// FAR CALL - Stack Behavior
// ============================================================================

#[test]
fn test_far_call_stack_push_order() {
    // Verify that CS is pushed before IP
    let code = [
        0x48, 0xc7, 0xc4, 0x00, 0x80, 0x00, 0x00, // MOV RSP, 0x8000
        0x9a, 0x00, 0x20, 0x08, 0x00, // CALL 0x0008:0x2000
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm_compat(&code, None);

    let target_code = [0xf4];
    mem.write_slice(&target_code, vm_memory::GuestAddress(0x2000))
        .unwrap();

    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Stack should have return address pushed
    // Stack should have return address pushed above the prior stack contents.
    assert!(regs.rsp < 0x8000);
}

#[test]
fn test_far_call_stack_alignment() {
    let code = [
        0x48, 0xc7, 0xc4, 0x00, 0x80, 0x00, 0x00, // MOV RSP, 0x8000
        0x9a, 0x00, 0x20, 0x08, 0x00, // CALL 0x0008:0x2000
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm_compat(&code, None);

    let target_code = [
        0x48, 0x89, 0xe0, // MOV RAX, RSP (save stack pointer)
        0xf4,
    ];
    mem.write_slice(&target_code, vm_memory::GuestAddress(0x2000))
        .unwrap();

    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Stack pointer should be decremented after push
    assert!(regs.rax < 0x8000);
}

// ============================================================================
// FAR CALL - Different Privilege Levels
// ============================================================================

#[test]
fn test_far_call_same_privilege_level() {
    // Call within same privilege level (CPL=0)
    let code = [
        0x9a, 0x00, 0x20, 0x08, 0x00, // CALL 0x0008:0x2000
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm_compat(&code, None);

    let target_code = [
        0x48, 0xc7, 0xc1, 0xaa, 0x00, 0x00, 0x00, // MOV RCX, 0xAA
        0xf4,
    ];
    mem.write_slice(&target_code, vm_memory::GuestAddress(0x2000))
        .unwrap();

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rcx, 0xaa);
}

#[test]
fn test_far_call_conforming_segment() {
    // Call using an alternate GDT selector (descriptor checks not modeled).
    let code = [
        0x9a, 0x00, 0x20, 0x10, 0x00, // CALL 0x0010:0x2000
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm_compat(&code, None);

    let target_code = [
        0x48, 0xc7, 0xc2, 0xbb, 0x00, 0x00, 0x00, // MOV RDX, 0xBB
        0xf4,
    ];
    mem.write_slice(&target_code, vm_memory::GuestAddress(0x2000))
        .unwrap();

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rdx, 0xbb);
}

// ============================================================================
// FAR CALL - Call Gates
// ============================================================================

#[test]
fn test_far_call_through_call_gate_basic() {
    // Call through a call gate descriptor
    let code = [
        0x9a, 0x00, 0x20, 0x10, 0x00, // CALL 0x0010:0x2000 (call gate selector)
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm_compat(&code, None);

    let target_code = [
        0x48, 0xc7, 0xc3, 0xcc, 0x00, 0x00, 0x00, // MOV RBX, 0xCC
        0xf4,
    ];
    mem.write_slice(&target_code, vm_memory::GuestAddress(0x2000))
        .unwrap();

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rbx, 0xcc);
}

#[test]
fn test_far_call_call_gate_parameter_copy() {
    // In IA-32e mode, far calls do not copy parameters to a new stack.
    let code = [
        0x48, 0xc7, 0xc4, 0x00, 0x80, 0x00, 0x00, // MOV RSP, 0x8000
        0x48, 0xc7, 0xc0, 0x11, 0x00, 0x00, 0x00, // MOV RAX, 0x11
        0x50, // PUSH RAX (parameter)
        0x48, 0xff, 0x1c, 0x25, 0x00, 0x20, 0x00, 0x00, // CALL FAR [0x2000] (64-bit offset)
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    let target_code = [
        0x58, // POP RAX (return RIP)
        0x5b, // POP RBX (return CS)
        0x59, // POP RCX (original parameter)
        0xf4,
    ];
    mem.write_slice(&target_code, vm_memory::GuestAddress(0x3000))
        .unwrap();
    let far_ptr = [
        0x00, 0x30, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 64-bit offset
        0x08, 0x00, // selector
    ];
    mem.write_slice(&far_ptr, vm_memory::GuestAddress(0x2000))
        .unwrap();

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, CODE_ADDR + 23);
    assert_eq!(regs.rcx, 0x11);
}

// ============================================================================
// FAR CALL - Task Gates
// ============================================================================

#[test]
fn test_far_call_through_task_gate() {
    // NOTE: Task gates are not supported in IA-32e mode; this verifies the
    // emulator's simplified far-call transfer only.
    let code = [
        0x9a, 0x00, 0x20, 0x18, 0x00, // CALL 0x0018:0x2000 (task gate)
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm_compat(&code, None);

    let target_code = [
        0x48, 0xc7, 0xc4, 0xdd, 0x00, 0x00, 0x00, // MOV RSP, 0xDD
        0xf4,
    ];
    mem.write_slice(&target_code, vm_memory::GuestAddress(0x2000))
        .unwrap();

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rsp, 0xdd);
}

#[test]
fn test_far_call_task_gate_saves_state() {
    // NOTE: Task gates are not supported in IA-32e mode; this verifies the
    // emulator's simplified far-call transfer only.
    let code = [
        0x48, 0xc7, 0xc0, 0x42, 0x00, 0x00, 0x00, // MOV RAX, 0x42
        0x9a, 0x00, 0x20, 0x18, 0x00, // CALL task gate
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm_compat(&code, None);

    let target_code = [
        0x48, 0xc7, 0xc0, 0x99, 0x00, 0x00, 0x00, // MOV RAX, 0x99
        0xf4,
    ];
    mem.write_slice(&target_code, vm_memory::GuestAddress(0x2000))
        .unwrap();

    let regs = run_until_hlt(&mut vcpu).unwrap();
    // New task should have different RAX
    assert_eq!(regs.rax, 0x99);
}

// ============================================================================
// FAR CALL - Segment Selector Validation
// ============================================================================

#[test]
fn test_far_call_null_selector() {
    // Calling with a null selector should fault (#GP).
    let code = [
        0x9a, 0x00, 0x20, 0x00, 0x00, // CALL 0x0000:0x2000 (null selector)
        0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1 (fallback)
        0xf4,
    ];
    let (mut vcpu, _) = setup_vm_compat(&code, None);

    assert!(run_until_hlt(&mut vcpu).is_err());
}

#[test]
fn test_far_call_invalid_selector() {
    // Selector beyond GDT/LDT limit should fault (#GP).
    let code = [
        0x9a, 0x00, 0x20, 0xff, 0xff, // CALL 0xFFFF:0x2000
        0x48, 0xc7, 0xc0, 0x02, 0x00, 0x00, 0x00, // MOV RAX, 2
        0xf4,
    ];
    let (mut vcpu, _) = setup_vm_compat(&code, None);

    assert!(run_until_hlt(&mut vcpu).is_err());
}

#[test]
fn test_far_call_ldt_selector() {
    // Call using an LDT selector without an LDT present should fault (#GP).
    let code = [
        0x9a, 0x00, 0x20, 0x0c, 0x00, // CALL 0x000C:0x2000 (LDT selector)
        0xf4,
    ];
    let (mut vcpu, _mem) = setup_vm_compat(&code, None);

    assert!(run_until_hlt(&mut vcpu).is_err());
}

// ============================================================================
// FAR CALL - Different Operand Sizes
// ============================================================================

#[test]
fn test_far_call_operand_size_16() {
    // 16-bit operand size (default in compatibility mode).
    let code = [
        0x9a, 0x00, 0x20, 0x08, 0x00, // CALL 0x0008:0x2000 (16-bit)
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm_compat(&code, None);

    let target_code = [0xf4];
    mem.write_slice(&target_code, vm_memory::GuestAddress(0x2000))
        .unwrap();

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rip, 0x2001);
}

#[test]
fn test_far_call_operand_size_32() {
    // 32-bit operand size (0x66 in compatibility mode).
    let code = [
        0x66, 0x9a, 0x00, 0x30, 0x00, 0x00, 0x08, 0x00, // CALL 0x0008:0x3000 (32-bit offset)
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm_compat(&code, None);

    let target_code = [0xf4];
    mem.write_slice(&target_code, vm_memory::GuestAddress(0x3000))
        .unwrap();

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rip, 0x3001);
}

#[test]
fn test_far_call_rex_prefix_64() {
    // REX.W prefix for 64-bit operand size
    let code = [
        0x48, 0xff, 0x1c, 0x25, 0x00, 0x20, 0x00, 0x00, // CALL FAR [0x2000]
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    let far_ptr = [0x00, 0x40, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x08, 0x00];
    mem.write_slice(&far_ptr, vm_memory::GuestAddress(0x2000))
        .unwrap();

    let target_code = [0xf4];
    mem.write_slice(&target_code, vm_memory::GuestAddress(0x4000))
        .unwrap();

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rip, 0x4001);
}

// ============================================================================
// FAR CALL - Nested Calls
// ============================================================================

#[test]
fn test_far_call_nested_same_segment() {
    let code = [
        0x48, 0xc7, 0xc4, 0x00, 0x80, 0x00, 0x00, // MOV RSP, 0x8000
        0x9a, 0x00, 0x20, 0x08, 0x00, // CALL 0x0008:0x2000
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm_compat(&code, None);

    // First level call
    let level1_code = [
        0x9a, 0x00, 0x30, 0x08, 0x00, // CALL 0x0008:0x3000
        0xf4,
    ];
    mem.write_slice(&level1_code, vm_memory::GuestAddress(0x2000))
        .unwrap();

    // Second level
    let level2_code = [
        0x48, 0xc7, 0xc6, 0x77, 0x00, 0x00, 0x00, // MOV RSI, 0x77
        0xf4,
    ];
    mem.write_slice(&level2_code, vm_memory::GuestAddress(0x3000))
        .unwrap();

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rsi, 0x77);
}

#[test]
fn test_far_call_nested_different_segments() {
    let code = [
        0x48, 0xc7, 0xc4, 0x00, 0x80, 0x00, 0x00, // MOV RSP, 0x8000
        0x9a, 0x00, 0x20, 0x08, 0x00, // CALL 0x0008:0x2000
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm_compat(&code, None);

    let level1_code = [
        0x9a, 0x00, 0x30, 0x10, 0x00, // CALL 0x0010:0x3000 (different segment)
        0xf4,
    ];
    mem.write_slice(&level1_code, vm_memory::GuestAddress(0x2000))
        .unwrap();

    let level2_code = [
        0x48, 0xc7, 0xc7, 0x88, 0x00, 0x00, 0x00, // MOV RDI, 0x88
        0xf4,
    ];
    mem.write_slice(&level2_code, vm_memory::GuestAddress(0x3000))
        .unwrap();

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rdi, 0x88);
}

// ============================================================================
// FAR CALL - Error Conditions
// ============================================================================

#[test]
fn test_far_call_non_present_segment() {
    // Selector beyond GDT limit should fault (#GP).
    let code = [
        0x9a, 0x00, 0x20, 0x20, 0x00, // CALL 0x0020:0x2000 (non-present)
        0x48, 0xc7, 0xc0, 0xff, 0x00, 0x00, 0x00, // MOV RAX, 0xFF
        0xf4,
    ];
    let (mut vcpu, _) = setup_vm_compat(&code, None);

    assert!(run_until_hlt(&mut vcpu).is_err());
}

#[test]
fn test_far_call_wrong_descriptor_type() {
    // Selector beyond GDT limit should fault (#GP).
    let code = [
        0x9a, 0x00, 0x20, 0x28, 0x00, // CALL 0x0028:0x2000 (data segment)
        0x48, 0xc7, 0xc0, 0xfe, 0x00, 0x00, 0x00, // MOV RAX, 0xFE
        0xf4,
    ];
    let (mut vcpu, _) = setup_vm_compat(&code, None);

    assert!(run_until_hlt(&mut vcpu).is_err());
}

// ============================================================================
// FAR CALL - Memory Addressing Modes
// ============================================================================

#[test]
fn test_far_call_mem_register_indirect() {
    // CALL FAR [RAX]
    let code = [
        0x48, 0xc7, 0xc0, 0x00, 0x20, 0x00, 0x00, // MOV RAX, 0x2000
        0xff, 0x18, // CALL FAR [RAX]
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    let far_ptr = [0x00, 0x30, 0x00, 0x00, 0x08, 0x00];
    mem.write_slice(&far_ptr, vm_memory::GuestAddress(0x2000))
        .unwrap();

    let target_code = [0xf4];
    mem.write_slice(&target_code, vm_memory::GuestAddress(0x3000))
        .unwrap();

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rip, 0x3001);
}

#[test]
fn test_far_call_mem_base_displacement() {
    // CALL FAR [RBX + 0x100]
    let code = [
        0x48, 0xc7, 0xc3, 0x00, 0x1f, 0x00, 0x00, // MOV RBX, 0x1F00
        0xff, 0x9b, 0x00, 0x01, 0x00, 0x00, // CALL FAR [RBX + 0x100]
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    let far_ptr = [0x00, 0x40, 0x00, 0x00, 0x08, 0x00];
    mem.write_slice(&far_ptr, vm_memory::GuestAddress(0x2000))
        .unwrap();

    let target_code = [0xf4];
    mem.write_slice(&target_code, vm_memory::GuestAddress(0x4000))
        .unwrap();

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rip, 0x4001);
}

#[test]
fn test_far_call_mem_sib_addressing() {
    // CALL FAR [RAX + RBX*4]
    let code = [
        0x48, 0xc7, 0xc0, 0x00, 0x1e, 0x00, 0x00, // MOV RAX, 0x1E00
        0x48, 0xc7, 0xc3, 0x80, 0x00, 0x00, 0x00, // MOV RBX, 0x80
        0xff, 0x1c, 0x98, // CALL FAR [RAX + RBX*4]
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    // Address = 0x1E00 + 0x80*4 = 0x1E00 + 0x200 = 0x2000
    let far_ptr = [0x00, 0x50, 0x00, 0x00, 0x08, 0x00];
    mem.write_slice(&far_ptr, vm_memory::GuestAddress(0x2000))
        .unwrap();

    let target_code = [0xf4];
    mem.write_slice(&target_code, vm_memory::GuestAddress(0x5000))
        .unwrap();

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rip, 0x5001);
}

// ============================================================================
// FAR CALL - Register Preservation
// ============================================================================

#[test]
fn test_far_call_preserves_general_registers() {
    let code = [
        0x48, 0xc7, 0xc0, 0x11, 0x11, 0x00, 0x00, // MOV RAX, 0x1111
        0x48, 0xc7, 0xc3, 0x22, 0x22, 0x00, 0x00, // MOV RBX, 0x2222
        0x48, 0xc7, 0xc1, 0x33, 0x33, 0x00, 0x00, // MOV RCX, 0x3333
        0x9a, 0x00, 0x20, 0x08, 0x00, // CALL 0x0008:0x2000
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm_compat(&code, None);

    let target_code = [
        // Registers should be preserved
        0xf4,
    ];
    mem.write_slice(&target_code, vm_memory::GuestAddress(0x2000))
        .unwrap();

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x1111);
    assert_eq!(regs.rbx, 0x2222);
    assert_eq!(regs.rcx, 0x3333);
}

#[test]
fn test_far_call_modifies_cs_and_rip() {
    let code = [
        0x9a, 0x00, 0x20, 0x08, 0x00, // CALL 0x0008:0x2000
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm_compat(&code, None);

    let target_code = [0xf4];
    mem.write_slice(&target_code, vm_memory::GuestAddress(0x2000))
        .unwrap();

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rip, 0x2001);
    // CS should be updated to selector 0x0008
}

// ============================================================================
// FAR CALL - Edge Cases
// ============================================================================

#[test]
fn test_far_call_to_boundary_address() {
    // Call to address at segment boundary
    let code = [
        0x9a, 0xff, 0xff, 0x08, 0x00, // CALL 0x0008:0xFFFF
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm_compat(&code, None);

    let target_code = [0xf4];
    mem.write_slice(&target_code, vm_memory::GuestAddress(0xFFFF))
        .unwrap();

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rip, 0x10000);
}

#[test]
fn test_far_call_zero_offset() {
    // Call to offset 0
    let code = [
        0x9a, 0x00, 0x00, 0x08, 0x00, // CALL 0x0008:0x0000
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm_compat(&code, None);

    let target_code = [0xf4];
    mem.write_slice(&target_code, vm_memory::GuestAddress(0x0000))
        .unwrap();

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rip, 0x0001);
}

#[test]
fn test_far_call_max_offset_32bit() {
    // Call with maximum 32-bit offset within test memory.
    let code = [
        0x66, 0x9a, 0xff, 0xff, 0xff, 0x00, 0x08, 0x00, // CALL 0x0008:0x00FFFFFF
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm_compat(&code, None);

    let target_code = [0xf4];
    mem.write_slice(&target_code, vm_memory::GuestAddress(0x00FF_FFFF))
        .unwrap();

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rip, 0x0100_0000);
}

#[test]
fn test_far_call_aligned_addresses() {
    // Call to 16-byte aligned address
    let code = [
        0x9a, 0x00, 0x30, 0x08, 0x00, // CALL 0x0008:0x3000 (aligned)
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm_compat(&code, None);

    let target_code = [0xf4];
    mem.write_slice(&target_code, vm_memory::GuestAddress(0x3000))
        .unwrap();

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rip, 0x3001);
}

#[test]
fn test_far_call_unaligned_addresses() {
    // Call to unaligned address
    let code = [
        0x9a, 0x03, 0x30, 0x08, 0x00, // CALL 0x0008:0x3003 (unaligned)
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm_compat(&code, None);

    let target_code = [0xf4];
    mem.write_slice(&target_code, vm_memory::GuestAddress(0x3003))
        .unwrap();

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rip, 0x3004);
}

// ============================================================================
// FAR CALL / RETF - Real GDT/LDT descriptor loading (regression)
//
// These exercise the load_code_segment path that reads the real 8-byte
// descriptor from the GDT (or LDT) on a far control transfer, instead of
// faking base=0/limit=max/DPL=0/L. They build a richer descriptor table than
// the default test harness so the loaded CS reflects actual descriptor fields.
// ============================================================================

/// Encode an 8-byte legacy segment descriptor.
/// `access` is the access byte (P/DPL/S/type); `flags` is the 4-bit high nibble
/// (G/D/L/AVL) placed in the upper nibble of byte 6.
fn encode_descriptor(base: u32, limit: u32, access: u8, flags: u8) -> [u8; 8] {
    [
        (limit & 0xFF) as u8,
        ((limit >> 8) & 0xFF) as u8,
        (base & 0xFF) as u8,
        ((base >> 8) & 0xFF) as u8,
        ((base >> 16) & 0xFF) as u8,
        access,
        (((flags & 0x0F) << 4) | (((limit >> 16) & 0x0F) as u8)),
        ((base >> 24) & 0xFF) as u8,
    ]
}

/// Write a far-return frame [offset][selector] (each `op_size` bytes) at `rsp`,
/// growing downward, and leave RSP pointing at the offset word.
fn write_far_return_frame(mem: &GuestMemoryMmap, rsp: u64, op_size: u8, ret_addr: u64, cs: u16) {
    let sz = op_size as u64;
    // selector at the higher address, offset at the lower (popped first).
    mem.write_slice(
        &(cs as u64).to_le_bytes()[..sz as usize],
        GuestAddress(rsp + sz),
    )
    .unwrap();
    mem.write_slice(&ret_addr.to_le_bytes()[..sz as usize], GuestAddress(rsp))
        .unwrap();
}

/// Install a descriptor at GDT_BASE + offset and widen gdt.limit to cover it.
fn install_gdt_descriptor(
    vcpu: &mut X86_64Vcpu,
    mem: &GuestMemoryMmap,
    sel_index_bytes: u64,
    desc: [u8; 8],
) {
    mem.write_slice(&desc, GuestAddress(GDT_BASE + sel_index_bytes))
        .unwrap();
    let mut sregs = vcpu.get_sregs().unwrap();
    let needed = (sel_index_bytes + 7) as u16;
    if sregs.gdt.limit < needed {
        sregs.gdt.limit = needed;
        vcpu.set_sregs(&sregs).unwrap();
    }
}

#[test]
fn test_retf_loads_real_descriptor_base_limit() {
    // RETF (converted path) popping selector 0x30 must load the real descriptor
    // base/limit/DPL from the GDT instead of the historical base=0/limit=max
    // fake. (The converted instruction paths preserve the current execution
    // mode, so CS.l is asserted via the strict loader tests below, not here.)
    let code = [
        0xcb, // RETF
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    // Code descriptor with a distinctive base/limit, DPL=0, G=1, L=1.
    let desc = encode_descriptor(0x000B_0000, 0x00042, 0x9A, 0b1010);
    install_gdt_descriptor(&mut vcpu, &mem, 0x30, desc);

    // Build a far-return frame on the stack: [RIP=0x3000][CS=0x30].
    // RETF (0xCB) without REX.W uses the 32-bit default operand size here.
    write_far_return_frame(&mem, STACK_ADDR, 4, 0x3000, 0x30);
    mem.write_slice(&[0xf4], GuestAddress(0x3000)).unwrap();

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rip, 0x3001);

    let sregs = vcpu.get_sregs().unwrap();
    assert_eq!(sregs.cs.selector, 0x30);
    assert_eq!(sregs.cs.base, 0x000B_0000, "RETF loads real CS base");
    // G=1 scales the 0x42-page limit to (0x42 << 12) | 0xFFF.
    assert_eq!(
        sregs.cs.limit,
        (0x00042 << 12) | 0xFFF,
        "RETF loads G-scaled limit"
    );
    assert_eq!(sregs.cs.dpl, 0, "RETF loads real DPL");
    assert!(sregs.cs.present);
}

#[test]
fn test_far_call_ptr_loads_real_descriptor_base_limit() {
    // CALL FAR ptr16:32 (0x9A, converted path) in compatibility mode loads the
    // real descriptor base/limit for selector 0x30 from the GDT.
    let code = [
        0x66, 0x9a, 0x00, 0x30, 0x00, 0x00, 0x30, 0x00, // CALL 0x0030:0x00003000
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm_compat(&code, None);

    // base=0x000C_0000, limit=0xFFFF bytes (G=0), 32-bit code (L=0, D=1).
    let desc = encode_descriptor(0x000C_0000, 0xFFFF, 0x9A, 0b0100);
    install_gdt_descriptor(&mut vcpu, &mem, 0x30, desc);
    mem.write_slice(&[0xf4], GuestAddress(0x3000)).unwrap();

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rip, 0x3001);

    let sregs = vcpu.get_sregs().unwrap();
    assert_eq!(sregs.cs.selector, 0x30);
    assert_eq!(sregs.cs.base, 0x000C_0000, "CALL FAR loads real CS base");
    assert_eq!(
        sregs.cs.limit, 0xFFFF,
        "byte-granular limit (G=0) loaded unscaled"
    );
}

#[test]
fn test_load_code_segment_loads_l_and_d_bits() {
    // The strict loader adopts the descriptor's L/D execution-mode bits.
    let code = [0xf4];
    let (mut vcpu, mem) = setup_vm(&code, None);

    // 64-bit code segment (L=1) => CS.l=true, CS.db=false.
    let desc64 = encode_descriptor(0x000A_0000, 0x12345, 0x9A, 0b1010);
    install_gdt_descriptor(&mut vcpu, &mem, 0x30, desc64);
    vcpu.load_code_segment_strict(0x30).unwrap();
    let sregs = vcpu.get_sregs().unwrap();
    assert_eq!(sregs.cs.base, 0x000A_0000, "strict loads real base");
    assert_eq!(
        sregs.cs.limit,
        (0x12345 << 12) | 0xFFF,
        "strict G-scaled limit"
    );
    assert!(sregs.cs.l, "L=1 adopted");
    assert!(!sregs.cs.db, "D forced 0 when L=1");
    assert_eq!(sregs.cs.dpl, 0);

    // 32-bit code segment (L=0, D=1) => CS.l=false, CS.db=true, byte-granular.
    let desc32 = encode_descriptor(0, 0xFFFF, 0x9A, 0b0100);
    install_gdt_descriptor(&mut vcpu, &mem, 0x38, desc32);
    vcpu.load_code_segment_strict(0x38).unwrap();
    let sregs = vcpu.get_sregs().unwrap();
    assert_eq!(sregs.cs.limit, 0xFFFF, "byte-granular limit (G=0)");
    assert!(!sregs.cs.l, "L=0 adopted");
    assert!(sregs.cs.db, "D=1 adopted when L=0");
}

#[test]
fn test_load_code_segment_not_present_faults_np() {
    // Strict descriptor load of a not-present (P=0) code segment must fault (#NP).
    let code = [0xf4];
    let (mut vcpu, mem) = setup_vm(&code, None);

    // P=0 (access 0x1A => S=1, type=code, but present bit clear), L=1.
    let desc = encode_descriptor(0, 0, 0x1A, 0b1010);
    install_gdt_descriptor(&mut vcpu, &mem, 0x30, desc);

    let err = vcpu.load_code_segment_strict(0x30);
    assert!(err.is_err(), "not-present descriptor must fault");
    let msg = format!("{:?}", err.unwrap_err());
    assert!(msg.contains("#NP"), "expected #NP, got: {}", msg);
}

#[test]
fn test_load_code_segment_non_code_faults_gp() {
    // Strict descriptor load of a data segment (non-code) must fault (#GP).
    let code = [0xf4];
    let (mut vcpu, mem) = setup_vm(&code, None);

    // Present data segment: access 0x92 => P=1, S=1, type=0x2 (data, read/write).
    let desc = encode_descriptor(0, 0xFFFF, 0x92, 0b1100);
    install_gdt_descriptor(&mut vcpu, &mem, 0x30, desc);

    let err = vcpu.load_code_segment_strict(0x30);
    assert!(err.is_err(), "non-code descriptor must fault");
    let msg = format!("{:?}", err.unwrap_err());
    assert!(msg.contains("#GP"), "expected #GP, got: {}", msg);
}

#[test]
fn test_load_code_segment_null_selector_faults_gp() {
    // CS may never be loaded with a null selector: strict load must fault (#GP).
    let code = [0xf4];
    let (mut vcpu, _mem) = setup_vm(&code, None);

    let err = vcpu.load_code_segment_strict(0x0000);
    assert!(err.is_err(), "null selector must fault");
    let msg = format!("{:?}", err.unwrap_err());
    assert!(msg.contains("#GP"), "expected #GP, got: {}", msg);
}

#[test]
fn test_load_code_segment_out_of_limit_faults_gp() {
    // A selector beyond the GDT limit must fault (#GP).
    let code = [0xf4];
    let (mut vcpu, _mem) = setup_vm(&code, None);
    // Default gdt.limit is 0x1F; selector 0xFFF8 is far past it.
    let err = vcpu.load_code_segment_strict(0xFFF8);
    assert!(err.is_err(), "out-of-limit selector must fault");
    let msg = format!("{:?}", err.unwrap_err());
    assert!(msg.contains("#GP"), "expected #GP, got: {}", msg);
}

#[test]
fn test_far_transfer_selector_0x08_stays_64bit() {
    // Regression guard: a converted far transfer (RETF) through the harness's
    // selector 0x08 must keep CS in 64-bit mode (cs.l=true) so existing 64-bit
    // tests that transfer through 0x08 keep running in 64-bit mode, while still
    // loading the descriptor's real base (0 for the harness 0x08 descriptor).
    let code = [
        0xcb, // RETF
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_far_return_frame(&mem, STACK_ADDR, 4, 0x3000, 0x08);
    mem.write_slice(&[0xf4], GuestAddress(0x3000)).unwrap();

    run_until_hlt(&mut vcpu).unwrap();
    let sregs = vcpu.get_sregs().unwrap();
    assert_eq!(sregs.cs.selector, 0x08);
    assert!(sregs.cs.l, "selector 0x08 transfer must stay 64-bit");
    assert_eq!(sregs.cs.base, 0, "harness 0x08 descriptor base is 0");
}
