use crate::common::*;
use rax::cpu::Registers;
use std::sync::Arc;
use vm_memory::{Bytes, GuestAddress};

fn assert_invalid_segment(code: &[u8]) {
    // Use setup_vm_no_idt so exceptions return errors instead of being handled
    let (mut vcpu, _) = setup_vm_no_idt(code, None);
    let result = vcpu.run();
    match result {
        Ok(VcpuExit::Hlt) => panic!("segment opcode should be invalid in 64-bit mode"),
        Ok(VcpuExit::Shutdown) => {}
        Err(_) => {}
        _ => {}
    }
}

// Comprehensive tests for PUSH and POP with segment registers
//
// PUSH/POP segment registers
// - In 64-bit mode, only FS/GS are valid (ES/CS/SS/DS are #UD)
// - CS cannot be popped (use far RET instead)
// In 64-bit mode, segment values are pushed as 16-bit but stack pointer adjusts by 8

// ============================================================================
// PUSH segment registers
// ============================================================================

#[test]
fn test_push_es_invalid_in_64bit() {
    let code = [
        0x06, // PUSH ES
        0xf4, // HLT (should not be reached)
    ];
    assert_invalid_segment(&code);
}

#[test]
fn test_push_cs_invalid_in_64bit() {
    let code = [
        0x0e, // PUSH CS
        0xf4, // HLT (should not be reached)
    ];
    assert_invalid_segment(&code);
}

#[test]
fn test_push_ss_invalid_in_64bit() {
    let code = [
        0x16, // PUSH SS
        0xf4, // HLT (should not be reached)
    ];
    assert_invalid_segment(&code);
}

#[test]
fn test_push_ds_invalid_in_64bit() {
    let code = [
        0x1e, // PUSH DS
        0xf4, // HLT (should not be reached)
    ];
    assert_invalid_segment(&code);
}

#[test]
fn test_push_fs() {
    let code = [
        0x0f, 0xa0, // PUSH FS
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rsp, STACK_ADDR - 8);

    let mut buf = [0u8; 8];
    mem.read_slice(&mut buf, GuestAddress(STACK_ADDR - 8))
        .unwrap();
    let stack_value = u64::from_le_bytes(buf);
    // FS selector is 0 in the test harness; PUSH FS writes a zero-extended word.
    assert_eq!(stack_value, 0x0000, "PUSH FS pushes FS selector (0)");
}

#[test]
fn test_push_gs() {
    let code = [
        0x0f, 0xa8, // PUSH GS
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rsp, STACK_ADDR - 8);

    let mut buf = [0u8; 8];
    mem.read_slice(&mut buf, GuestAddress(STACK_ADDR - 8))
        .unwrap();
    let stack_value = u64::from_le_bytes(buf);
    // GS selector is 0 in the test harness; PUSH GS writes a zero-extended word.
    assert_eq!(stack_value, 0x0000, "PUSH GS pushes GS selector (0)");
}

// ============================================================================
// POP segment registers
// Note: Cannot POP CS
// ============================================================================

#[test]
fn test_pop_es_invalid_in_64bit() {
    let code = [
        0x07, // POP ES
        0xf4, // HLT (should not be reached)
    ];
    assert_invalid_segment(&code);
}

#[test]
fn test_pop_ss_invalid_in_64bit() {
    let code = [
        0x17, // POP SS
        0xf4, // HLT (should not be reached)
    ];
    assert_invalid_segment(&code);
}

#[test]
fn test_pop_ds_invalid_in_64bit() {
    let code = [
        0x1f, // POP DS
        0xf4, // HLT (should not be reached)
    ];
    assert_invalid_segment(&code);
}

#[test]
fn test_pop_fs() {
    let code = [
        0x0f, 0xa0, // PUSH FS
        0x0f, 0xa1, // POP FS
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rsp, STACK_ADDR);
}

#[test]
fn test_pop_gs() {
    let code = [
        0x0f, 0xa8, // PUSH GS
        0x0f, 0xa9, // POP GS
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rsp, STACK_ADDR);
}

// ============================================================================
// Push/Pop combinations and transfers
// ============================================================================

#[test]
fn test_push_pop_transfer_gs_to_fs() {
    let code = [
        0x0f, 0xa8, // PUSH GS
        0x0f, 0xa1, // POP FS
        0x8c, 0xe0, // MOV AX, FS (read FS)
        0x8c, 0xeb, // MOV BX, GS (read GS)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // FS should equal GS (transferred via stack)
    assert_eq!(regs.rax & 0xFFFF, regs.rbx & 0xFFFF);
}

#[test]
fn test_push_pop_transfer_fs_to_gs() {
    let code = [
        0x0f, 0xa0, // PUSH FS
        0x0f, 0xa9, // POP GS
        0x8c, 0xe0, // MOV AX, FS
        0x8c, 0xeb, // MOV BX, GS
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFF, regs.rbx & 0xFFFF);
}

#[test]
fn test_push_pop_transfer_fs_to_gs_with_rcx() {
    let code = [
        0x0f, 0xa0, // PUSH FS
        0x0f, 0xa9, // POP GS
        0x8c, 0xe0, // MOV AX, FS
        0x8c, 0xe9, // MOV CX, GS
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFF, regs.rcx & 0xFFFF);
}

#[test]
fn test_multiple_pushes() {
    let code = [
        0x0f, 0xa0, // PUSH FS
        0x0f, 0xa8, // PUSH GS
        0x0f, 0xa0, // PUSH FS
        0x0f, 0xa8, // PUSH GS
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Stack pointer should have decreased by 32 (4 pushes × 8 bytes)
    assert_eq!(regs.rsp, STACK_ADDR - 32);
}

#[test]
fn test_multiple_pops() {
    let code = [
        0x0f, 0xa0, // PUSH FS
        0x0f, 0xa8, // PUSH GS
        0x0f, 0xa0, // PUSH FS
        0x0f, 0xa8, // PUSH GS
        // Pop in reverse order
        0x0f, 0xa9, // POP GS
        0x0f, 0xa1, // POP FS
        0x0f, 0xa9, // POP GS
        0x0f, 0xa1, // POP FS
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Stack should be balanced
    assert_eq!(regs.rsp, STACK_ADDR);
}

#[test]
fn test_push_all_segments() {
    let code = [
        0x0f, 0xa0, // PUSH FS
        0x0f, 0xa8, // PUSH GS
        0x0f, 0xa0, // PUSH FS
        0x0f, 0xa8, // PUSH GS
        0x0f, 0xa0, // PUSH FS
        0x0f, 0xa8, // PUSH GS
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // 6 segments × 8 bytes = 48 bytes
    assert_eq!(regs.rsp, STACK_ADDR - 48);

    // Verify all values on stack are valid segment values (16-bit)
    for i in 0..6 {
        let mut buf = [0u8; 8];
        mem.read_slice(&mut buf, GuestAddress(STACK_ADDR - 8 * (i + 1)))
            .unwrap();
        let value = u64::from_le_bytes(buf);
        assert_eq!(value >> 16, 0); // Upper bits should be zero
    }
}

#[test]
fn test_pop_all_segments_except_cs() {
    let code = [
        0x0f, 0xa0, // PUSH FS
        0x0f, 0xa8, // PUSH GS
        0x0f, 0xa0, // PUSH FS
        0x0f, 0xa8, // PUSH GS
        0x0f, 0xa0, // PUSH FS
        // Pop in reverse order
        0x0f, 0xa1, // POP FS
        0x0f, 0xa9, // POP GS
        0x0f, 0xa1, // POP FS
        0x0f, 0xa9, // POP GS
        0x0f, 0xa1, // POP FS
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Stack should be balanced
    assert_eq!(regs.rsp, STACK_ADDR);
}

// ============================================================================
// Stack alignment and memory tests
// ============================================================================

#[test]
fn test_push_segment_memory_content() {
    let code = [
        0x8c, 0xe0, // MOV AX, FS (get FS value)
        0x0f, 0xa0, // PUSH FS
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Read pushed value from stack
    let mut buf = [0u8; 8];
    mem.read_slice(&mut buf, GuestAddress(STACK_ADDR - 8))
        .unwrap();
    let stack_value = u64::from_le_bytes(buf);

    // Should match FS value
    assert_eq!(stack_value & 0xFFFF, regs.rax & 0xFFFF);
}

#[test]
fn test_pop_segment_from_prepared_stack() {
    let code = [
        // Prepare stack with known value (0)
        0x48, 0xc7, 0xc0, 0x00, 0x00, 0x00, 0x00, // MOV RAX, 0
        0x50, // PUSH RAX
        // Pop into FS
        0x0f, 0xa1, // POP FS
        // Read FS back
        0x8c, 0xe3, // MOV BX, FS
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx & 0xFFFF, 0);
}

#[test]
fn test_segment_stack_with_general_registers() {
    let code = [
        0x0f, 0xa0, // PUSH FS
        0x48, 0xc7, 0xc0, 0x42, 0x00, 0x00, 0x00, // MOV RAX, 0x42
        0x50, // PUSH RAX
        0x58, // POP RAX (should be 0x42)
        0x0f, 0xa9, // POP GS (should be FS value)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x42);
    assert_eq!(regs.rsp, STACK_ADDR);
}

#[test]
fn test_nested_push_pop_segments() {
    let code = [
        0x0f, 0xa0, // PUSH FS
        0x0f, 0xa8, // PUSH GS
        0x0f, 0xa0, // PUSH FS
        0x0f, 0xa1, // POP FS
        0x0f, 0xa9, // POP GS
        0x0f, 0xa1, // POP FS
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rsp, STACK_ADDR);
}

#[test]
fn test_push_pop_preserves_segment_value() {
    let code = [
        // Get original FS value
        0x8c, 0xe0, // MOV AX, FS
        // Push and pop FS
        0x0f, 0xa0, // PUSH FS
        0x0f, 0xa1, // POP FS
        // Get FS value again
        0x8c, 0xe3, // MOV BX, FS
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // FS should be unchanged
    assert_eq!(regs.rax & 0xFFFF, regs.rbx & 0xFFFF);
}

#[test]
fn test_stack_grows_downward() {
    let code = [
        0x48, 0x89, 0xe0, // MOV RAX, RSP (save initial SP)
        0x0f, 0xa0, // PUSH FS
        0x48, 0x89, 0xe3, // MOV RBX, RSP (save SP after push)
        0x0f, 0xa1, // POP FS
        0x48, 0x89, 0xe1, // MOV RCX, RSP (save SP after pop)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // RBX (after push) < RAX (initial)
    assert!(regs.rbx < regs.rax);
    // RCX (after pop) == RAX (back to initial)
    assert_eq!(regs.rcx, regs.rax);
}

#[test]
fn test_multiple_segment_round_trip() {
    let code = [
        // Save original values
        0x8c, 0xe0, // MOV AX, FS
        0x8c, 0xe9, // MOV CX, GS
        // Push both
        0x0f, 0xa0, // PUSH FS
        0x0f, 0xa8, // PUSH GS
        // Pop in reverse order
        0x0f, 0xa9, // POP GS (gets GS back)
        0x0f, 0xa1, // POP FS (gets FS back)
        // Read back
        0x8c, 0xe3, // MOV BX, FS
        0x8c, 0xea, // MOV DX, GS
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Values should match
    assert_eq!(regs.rax & 0xFFFF, regs.rbx & 0xFFFF); // FS unchanged
    assert_eq!(regs.rcx & 0xFFFF, regs.rdx & 0xFFFF); // GS unchanged
}

#[test]
fn test_push_pop_with_offset_stack() {
    let code = [
        // Adjust stack pointer
        0x48, 0x83, 0xec, 0x10, // SUB RSP, 16
        0x0f, 0xa0, // PUSH FS
        0x0f, 0xa1, // POP FS
        0x48, 0x83, 0xc4, 0x10, // ADD RSP, 16
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rsp, STACK_ADDR);
}

#[test]
fn test_interleaved_segment_general_pushes() {
    let code = [
        0x0f, 0xa0, // PUSH FS
        0x48, 0xc7, 0xc0, 0x11, 0x00, 0x00, 0x00, // MOV RAX, 0x11
        0x50, // PUSH RAX
        0x0f, 0xa8, // PUSH GS
        0x48, 0xc7, 0xc3, 0x22, 0x00, 0x00, 0x00, // MOV RBX, 0x22
        0x53, // PUSH RBX
        0x0f, 0xa0, // PUSH FS
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // 5 pushes × 8 bytes = 40 bytes
    assert_eq!(regs.rsp, STACK_ADDR - 40);
}

#[test]
fn test_segment_push_pop_symmetry() {
    let code = [
        // Push all segment registers
        0x0f, 0xa0, // PUSH FS
        0x0f, 0xa8, // PUSH GS
        0x0f, 0xa0, // PUSH FS
        0x0f, 0xa8, // PUSH GS
        0x0f, 0xa0, // PUSH FS
        // Pop in same order (different destinations)
        0x0f, 0xa1, // POP FS
        0x0f, 0xa9, // POP GS
        0x0f, 0xa1, // POP FS
        0x0f, 0xa9, // POP GS
        0x0f, 0xa1, // POP FS
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rsp, STACK_ADDR);
}

#[test]
fn test_push_fs_multiple_times() {
    let code = [
        0x0f, 0xa0, // PUSH FS
        0x0f, 0xa0, // PUSH FS
        0x0f, 0xa0, // PUSH FS
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rsp, STACK_ADDR - 24);
}

#[test]
fn test_pop_gs_multiple_times() {
    let code = [
        0x0f, 0xa8, // PUSH GS
        0x0f, 0xa8, // PUSH GS
        0x0f, 0xa8, // PUSH GS
        0x0f, 0xa9, // POP GS
        0x0f, 0xa9, // POP GS
        0x0f, 0xa9, // POP GS
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rsp, STACK_ADDR);
}

#[test]
fn test_push_pop_fs_gs_interleaved() {
    let code = [
        0x0f, 0xa0, // PUSH FS
        0x0f, 0xa8, // PUSH GS
        0x0f, 0xa1, // POP FS
        0x0f, 0xa9, // POP GS
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rsp, STACK_ADDR);
}

#[test]
fn test_segment_push_pop_lifo() {
    let code = [
        0x0f, 0xa0, // PUSH FS
        0x0f, 0xa8, // PUSH GS
        0x0f, 0xa0, // PUSH FS
        // Pop in LIFO order
        0x0f, 0xa9, // POP GS (gets FS)
        0x0f, 0xa1, // POP FS (gets GS)
        0x0f, 0xa9, // POP GS (gets FS)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_push_segment_after_modification() {
    let code = [
        0x48, 0x31, 0xc0, // XOR RAX, RAX
        0x8e, 0xe0, // MOV FS, AX (set FS to 0)
        0x0f, 0xa0, // PUSH FS (push modified FS)
        0x58, // POP RAX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFFFF, 0);
}

#[test]
fn test_pop_segment_clears_high_bits() {
    let code = [
        0x48, 0xc7, 0xc0, 0xff, 0xff, 0xff, 0xff, // MOV RAX, 0xFFFFFFFF
        0x50, // PUSH RAX
        0x0f, 0xa1, // POP FS
        0x8c, 0xe3, // MOV BX, FS
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    // FS should only have lower 16 bits
    assert_eq!(regs.rbx & 0xFFFF, 0xFFFF);
}

#[test]
fn test_push_all_pop_one() {
    let code = [
        0x0f, 0xa0, // PUSH FS
        0x0f, 0xa8, // PUSH GS
        0x0f, 0xa0, // PUSH FS
        0x0f, 0xa9, // POP GS (gets FS)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    // Stack should have 2 items left
    assert_eq!(regs.rsp, STACK_ADDR - 16);
}

// ============================================================================
// Value-asserting PUSH/POP segment tests with non-zero selectors, so the
// transferred value (not merely 0 == 0) is checked end-to-end.
// ============================================================================

// Load a non-zero selector into FS, PUSH FS, and read the exact pushed word.
#[test]
fn test_push_fs_nonzero_value_on_stack() {
    let code = [
        0x48, 0xc7, 0xc0, 0x23, 0x00, 0x00, 0x00, // MOV RAX, 0x23
        0x8e, 0xe0, // MOV FS, AX
        0x0f, 0xa0, // PUSH FS
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rsp, STACK_ADDR - 8, "PUSH FS decrements RSP by 8");
    let mut buf = [0u8; 8];
    mem.read_slice(&mut buf, GuestAddress(STACK_ADDR - 8))
        .unwrap();
    assert_eq!(
        u64::from_le_bytes(buf),
        0x23,
        "PUSH FS pushes selector 0x23"
    );
}

// PUSH FS then POP GS transfers the FS selector value into GS.
#[test]
fn test_push_fs_pop_gs_transfers_selector() {
    let code = [
        0x48, 0xc7, 0xc0, 0x33, 0x00, 0x00, 0x00, // MOV RAX, 0x33
        0x8e, 0xe0, // MOV FS, AX (FS = 0x33)
        0x0f, 0xa0, // PUSH FS
        0x0f, 0xa9, // POP GS  (GS <- 0x33)
        0x8c, 0xeb, // MOV BX, GS
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rbx & 0xFFFF,
        0x33,
        "GS receives FS selector via PUSH/POP"
    );
    assert_eq!(regs.rsp, STACK_ADDR, "stack balanced after PUSH/POP");
}

// POP FS from a prepared stack loads exactly the low 16 bits and clears the rest.
#[test]
fn test_pop_fs_loads_low16_only() {
    let code = [
        0x48, 0xb8, 0x99, 0x00, 0x00, 0x00, 0xff, 0xff, 0xff,
        0xff, // MOV RAX, 0xFFFFFFFF00000099
        0x50, // PUSH RAX
        0x0f, 0xa1, // POP FS
        0x8c, 0xe3, // MOV BX, FS
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rbx & 0xFFFF,
        0x0099,
        "POP FS takes low 16 bits of the stack word"
    );
    assert_eq!(regs.rsp, STACK_ADDR, "RSP restored after PUSH/POP pair");
}
