use crate::common::{run_until_hlt, setup_vm};
use rax::cpu::Registers;

// ENDBR32 / ENDBR64 - Terminate Indirect Branch (CET Indirect Branch Tracking)
// Opcodes:
//   F3 0F 1E FB - ENDBR32 (32-bit and compatibility mode)
//   F3 0F 1E FA - ENDBR64 (64-bit mode)
// On processors without CET, these act as multi-byte NOPs
// On processors with CET enabled, they terminate indirect branches
// Do not modify registers, flags, or memory

// Basic ENDBR64 test
#[test]
fn test_endbr64_basic() {
    let code = [
        0xf3, 0x0f, 0x1e, 0xfa, // ENDBR64
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Should complete without error (acts as NOP on non-CET)
    let _ = regs;
}

// Basic ENDBR32 test
#[test]
fn test_endbr32_basic() {
    let code = [
        0xf3, 0x0f, 0x1e, 0xfb, // ENDBR32
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Should complete without error (acts as NOP on non-CET)
    let _ = regs;
}

// Test ENDBR64 doesn't modify RAX
#[test]
fn test_endbr64_preserves_rax() {
    let code = [
        0x48, 0xc7, 0xc0, 0x42, 0x42, 0x42, 0x42, // MOV RAX, 0x42424242
        0xf3, 0x0f, 0x1e, 0xfa, // ENDBR64
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x42424242, "RAX should be unchanged");
}

// Test ENDBR32 doesn't modify RBX
#[test]
fn test_endbr32_preserves_rbx() {
    let code = [
        0x48, 0xc7, 0xc3, 0x11, 0x22, 0x33, 0x44, // MOV RBX, 0x44332211
        0xf3, 0x0f, 0x1e, 0xfb, // ENDBR32
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx, 0x44332211, "RBX should be unchanged");
}

// Test ENDBR64 doesn't modify flags
#[test]
fn test_endbr64_preserves_flags() {
    let code = [
        0x48, 0xc7, 0xc0, 0xff, 0xff, 0xff, 0xff, // MOV RAX, -1
        0x48, 0x83, 0xc0, 0x01, // ADD RAX, 1 (sets ZF)
        0xf3, 0x0f, 0x1e, 0xfa, // ENDBR64
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // ZF should still be set from the ADD
    assert!(regs.rflags & 0x40 != 0, "ZF should be preserved");
}

// Test ENDBR32 doesn't modify flags
#[test]
fn test_endbr32_preserves_flags() {
    let code = [
        0x48, 0xc7, 0xc0, 0xff, 0xff, 0xff, 0xff, // MOV RAX, -1
        0x48, 0x83, 0xc0, 0x01, // ADD RAX, 1 (sets ZF)
        0xf3, 0x0f, 0x1e, 0xfb, // ENDBR32
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // ZF should still be set from the ADD
    assert!(regs.rflags & 0x40 != 0, "ZF should be preserved");
}

// Test ENDBR64 preserves all general-purpose registers
#[test]
fn test_endbr64_preserves_all_gprs() {
    let code = [
        0x48, 0xc7, 0xc0, 0x01, 0x01, 0x01, 0x01, // MOV RAX, 0x01010101
        0x48, 0xc7, 0xc3, 0x02, 0x02, 0x02, 0x02, // MOV RBX, 0x02020202
        0x48, 0xc7, 0xc1, 0x03, 0x03, 0x03, 0x03, // MOV RCX, 0x03030303
        0x48, 0xc7, 0xc2, 0x04, 0x04, 0x04, 0x04, // MOV RDX, 0x04040404
        0xf3, 0x0f, 0x1e, 0xfa, // ENDBR64
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x01010101, "RAX should be unchanged");
    assert_eq!(regs.rbx, 0x02020202, "RBX should be unchanged");
    assert_eq!(regs.rcx, 0x03030303, "RCX should be unchanged");
    assert_eq!(regs.rdx, 0x04040404, "RDX should be unchanged");
}

// Test ENDBR32 preserves all general-purpose registers
#[test]
fn test_endbr32_preserves_all_gprs() {
    let code = [
        0x48, 0xc7, 0xc0, 0x01, 0x01, 0x01, 0x01, // MOV RAX, 0x01010101
        0x48, 0xc7, 0xc3, 0x02, 0x02, 0x02, 0x02, // MOV RBX, 0x02020202
        0x48, 0xc7, 0xc1, 0x03, 0x03, 0x03, 0x03, // MOV RCX, 0x03030303
        0x48, 0xc7, 0xc2, 0x04, 0x04, 0x04, 0x04, // MOV RDX, 0x04040404
        0xf3, 0x0f, 0x1e, 0xfb, // ENDBR32
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x01010101, "RAX should be unchanged");
    assert_eq!(regs.rbx, 0x02020202, "RBX should be unchanged");
    assert_eq!(regs.rcx, 0x03030303, "RCX should be unchanged");
    assert_eq!(regs.rdx, 0x04040404, "RDX should be unchanged");
}

// Test multiple sequential ENDBR64 calls
#[test]
fn test_endbr64_sequential_calls() {
    let code = [
        0xf3, 0x0f, 0x1e, 0xfa, // ENDBR64
        0xf3, 0x0f, 0x1e, 0xfa, // ENDBR64
        0xf3, 0x0f, 0x1e, 0xfa, // ENDBR64
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Should complete without error
    let _ = regs;
}

// Test multiple sequential ENDBR32 calls
#[test]
fn test_endbr32_sequential_calls() {
    let code = [
        0xf3, 0x0f, 0x1e, 0xfb, // ENDBR32
        0xf3, 0x0f, 0x1e, 0xfb, // ENDBR32
        0xf3, 0x0f, 0x1e, 0xfb, // ENDBR32
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Should complete without error
    let _ = regs;
}

// Test ENDBR64 after indirect call target
#[test]
fn test_endbr64_after_call() {
    let code = [
        0xe8, 0x05, 0x00, 0x00, 0x00, // CALL +5 (to target)
        0xf4, // HLT
        // target:
        0xf3, 0x0f, 0x1e, 0xfa, // ENDBR64 (indirect branch target)
        0xc3, // RET
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Should complete without error
    let _ = regs;
}

// Test ENDBR32 after indirect call target
#[test]
fn test_endbr32_after_call() {
    let code = [
        0xe8, 0x05, 0x00, 0x00, 0x00, // CALL +5 (to target)
        0xf4, // HLT
        // target:
        0xf3, 0x0f, 0x1e, 0xfb, // ENDBR32 (indirect branch target)
        0xc3, // RET
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Should complete without error
    let _ = regs;
}

// Test ENDBR64 with all flags set
#[test]
fn test_endbr64_with_all_flags_set() {
    let code = [
        0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1
        0x48, 0x83, 0xe8, 0x02, // SUB RAX, 2 (sets CF, SF)
        0xf3, 0x0f, 0x1e, 0xfa, // ENDBR64
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Flags from SUB should be preserved
    assert!(regs.rflags & 0x01 != 0, "CF should be preserved");
    assert!(regs.rflags & 0x80 != 0, "SF should be preserved");
}

// Test ENDBR32 with all flags set
#[test]
fn test_endbr32_with_all_flags_set() {
    let code = [
        0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1
        0x48, 0x83, 0xe8, 0x02, // SUB RAX, 2 (sets CF, SF)
        0xf3, 0x0f, 0x1e, 0xfb, // ENDBR32
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Flags from SUB should be preserved
    assert!(regs.rflags & 0x01 != 0, "CF should be preserved");
    assert!(regs.rflags & 0x80 != 0, "SF should be preserved");
}

// Test ENDBR64 preserves R8-R15
#[test]
fn test_endbr64_preserves_extended_registers() {
    let code = [
        0x49, 0xc7, 0xc0, 0x11, 0x11, 0x11, 0x11, // MOV R8, 0x11111111
        0x49, 0xc7, 0xc7, 0xff, 0xff, 0xff, 0xff, // MOV R15, 0xffffffff
        0xf3, 0x0f, 0x1e, 0xfa, // ENDBR64
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.r8, 0x11111111, "R8 should be preserved");
    assert_eq!(regs.r15, 0xffffffffffffffff, "R15 should be preserved");
}

// Test ENDBR32 preserves R8-R15
#[test]
fn test_endbr32_preserves_extended_registers() {
    let code = [
        0x49, 0xc7, 0xc0, 0x11, 0x11, 0x11, 0x11, // MOV R8, 0x11111111
        0x49, 0xc7, 0xc7, 0xff, 0xff, 0xff, 0xff, // MOV R15, 0xffffffff
        0xf3, 0x0f, 0x1e, 0xfb, // ENDBR32
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.r8, 0x11111111, "R8 should be preserved");
    assert_eq!(regs.r15, 0xffffffffffffffff, "R15 should be preserved");
}

// Test ENDBR64 preserves stack pointer
#[test]
fn test_endbr64_preserves_stack_pointer() {
    let code = [
        0xf3, 0x0f, 0x1e, 0xfa, // ENDBR64
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x8000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rsp, 0x8000, "RSP should be unchanged");
}

// Test ENDBR32 preserves stack pointer
#[test]
fn test_endbr32_preserves_stack_pointer() {
    let code = [
        0xf3, 0x0f, 0x1e, 0xfb, // ENDBR32
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x8000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rsp, 0x8000, "RSP should be unchanged");
}

// Test ENDBR64 between MOV instructions
#[test]
fn test_endbr64_between_movs() {
    let code = [
        0xb8, 0x42, 0x00, 0x00, 0x00, // MOV EAX, 0x42
        0xf3, 0x0f, 0x1e, 0xfa, // ENDBR64
        0xbb, 0x99, 0x00, 0x00, 0x00, // MOV EBX, 0x99
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Both MOVs should execute correctly
    assert_eq!(regs.rax & 0xFFFFFFFF, 0x42, "First MOV executed");
    assert_eq!(regs.rbx & 0xFFFFFFFF, 0x99, "Second MOV executed");
}

// Test ENDBR32 between MOV instructions
#[test]
fn test_endbr32_between_movs() {
    let code = [
        0xb8, 0x42, 0x00, 0x00, 0x00, // MOV EAX, 0x42
        0xf3, 0x0f, 0x1e, 0xfb, // ENDBR32
        0xbb, 0x99, 0x00, 0x00, 0x00, // MOV EBX, 0x99
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Both MOVs should execute correctly
    assert_eq!(regs.rax & 0xFFFFFFFF, 0x42, "First MOV executed");
    assert_eq!(regs.rbx & 0xFFFFFFFF, 0x99, "Second MOV executed");
}

// Test ENDBR64 with conditional jumps
#[test]
fn test_endbr64_with_conditional_jump() {
    let code = [
        0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1
        0x48, 0x85, 0xc0, // TEST RAX, RAX
        0x74, 0x04, // JZ skip (should not jump)
        0xf3, 0x0f, 0x1e, 0xfa, // ENDBR64
        // skip:
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Should complete successfully
    let _ = regs;
}

// Test ENDBR32 with conditional jumps
#[test]
fn test_endbr32_with_conditional_jump() {
    let code = [
        0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1
        0x48, 0x85, 0xc0, // TEST RAX, RAX
        0x74, 0x04, // JZ skip (should not jump)
        0xf3, 0x0f, 0x1e, 0xfb, // ENDBR32
        // skip:
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Should complete successfully
    let _ = regs;
}

// Test ENDBR64 acts as NOP-like instruction
#[test]
fn test_endbr64_nop_behavior() {
    let code = [
        0x48, 0xc7, 0xc0, 0x42, 0x00, 0x00, 0x00, // MOV RAX, 0x42
        0xf3, 0x0f, 0x1e, 0xfa, // ENDBR64 (acts as NOP on non-CET)
        0x48, 0x83, 0xc0, 0x01, // ADD RAX, 1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // RAX should be 0x43
    assert_eq!(regs.rax, 0x43, "ENDBR64 should not affect execution");
}

// Test ENDBR32 acts as NOP-like instruction
#[test]
fn test_endbr32_nop_behavior() {
    let code = [
        0x48, 0xc7, 0xc0, 0x42, 0x00, 0x00, 0x00, // MOV RAX, 0x42
        0xf3, 0x0f, 0x1e, 0xfb, // ENDBR32 (acts as NOP on non-CET)
        0x48, 0x83, 0xc0, 0x01, // ADD RAX, 1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // RAX should be 0x43
    assert_eq!(regs.rax, 0x43, "ENDBR32 should not affect execution");
}

// Test ENDBR64 preserves base pointer
#[test]
fn test_endbr64_preserves_base_pointer() {
    let code = [
        0x48, 0xc7, 0xc5, 0x00, 0x70, 0x00, 0x00, // MOV RBP, 0x7000
        0xf3, 0x0f, 0x1e, 0xfa, // ENDBR64
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbp, 0x7000, "RBP should be preserved");
}

// Test ENDBR32 preserves base pointer
#[test]
fn test_endbr32_preserves_base_pointer() {
    let code = [
        0x48, 0xc7, 0xc5, 0x00, 0x70, 0x00, 0x00, // MOV RBP, 0x7000
        0xf3, 0x0f, 0x1e, 0xfb, // ENDBR32
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbp, 0x7000, "RBP should be preserved");
}

// Test alternating ENDBR64 and ENDBR32
#[test]
fn test_endbr64_endbr32_alternating() {
    let code = [
        0xf3, 0x0f, 0x1e, 0xfa, // ENDBR64
        0xf3, 0x0f, 0x1e, 0xfb, // ENDBR32
        0xf3, 0x0f, 0x1e, 0xfa, // ENDBR64
        0xf3, 0x0f, 0x1e, 0xfb, // ENDBR32
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Should complete without error
    let _ = regs;
}

// Test ENDBR64 doesn't cause exceptions
#[test]
fn test_endbr64_no_exception() {
    let code = [
        0xf3, 0x0f, 0x1e, 0xfa, // ENDBR64
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    // Should complete without panicking or returning an error
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// Test ENDBR32 doesn't cause exceptions
#[test]
fn test_endbr32_no_exception() {
    let code = [
        0xf3, 0x0f, 0x1e, 0xfb, // ENDBR32
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    // Should complete without panicking or returning an error
    let _ = run_until_hlt(&mut vcpu).unwrap();
}
