use crate::common::{run_until_hlt, setup_vm};
use rax::cpu::Registers;

// Comprehensive tests for JCXZ, JECXZ, and JRCXZ instructions
// These instructions jump if CX/ECX/RCX is zero (don't check flags)
// Based on documentation from /Users/int/dev/rax/docs/jcc.txt
//
// JCXZ - Jump if CX is zero (16-bit, NOT available in 64-bit mode)
// JECXZ - Jump if ECX is zero (32-bit)
// JRCXZ - Jump if RCX is zero (64-bit)
//
// All use opcode E3 cb with address-size prefix determining which register

// ============================================================================
// JECXZ - Jump if ECX Zero (32-bit counter)
// Opcode: E3 cb
// ============================================================================

#[test]
fn test_jecxz_taken_zero() {
    let code = [
        0x48, 0x31, 0xc9, // XOR RCX, RCX (ECX = 0)
        0x67, 0xe3, 0x02, // JECXZ +2 (address-size prefix for 32-bit)
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_jecxz_not_taken_nonzero() {
    let code = [
        0x48, 0xc7, 0xc1, 0x01, 0x00, 0x00, 0x00, // MOV RCX, 1 (ECX = 1)
        0x67, 0xe3, 0x05, // JECXZ +5 (should not jump)
        0x48, 0xc7, 0xc0, 0x42, 0x00, 0x00, 0x00, // MOV RAX, 0x42
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x42);
}

#[test]
fn test_jecxz_taken_high_bits_set() {
    // ECX is zero even if high 32 bits of RCX are set
    let code = [
        0x48, 0xb9, 0x00, 0x00, 0x00, 0x00, 0xff, 0xff, 0xff,
        0xff, // MOV RCX, 0xFFFFFFFF00000000
        0x67, 0xe3, 0x02, // JECXZ +2 (should jump, ECX = 0)
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_jecxz_not_taken_max_value() {
    let code = [
        0x48, 0xc7, 0xc1, 0xff, 0xff, 0xff, 0xff, // MOV RCX, 0xFFFFFFFF (ECX = max)
        0x67, 0xe3, 0x05, // JECXZ +5 (should not jump)
        0x48, 0xc7, 0xc0, 0x99, 0x00, 0x00, 0x00, // MOV RAX, 0x99
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x99);
}

#[test]
fn test_jecxz_loop_countdown() {
    let code = [
        0x48, 0xc7, 0xc1, 0x05, 0x00, 0x00, 0x00, // MOV RCX, 5
        0x48, 0xc7, 0xc0, 0x00, 0x00, 0x00, 0x00, // MOV RAX, 0
        // loop start (offset 14):
        0x48, 0x83, 0xc0, 0x01, // ADD RAX, 1
        0x48, 0x83, 0xe9, 0x01, // SUB RCX, 1
        0x67, 0xe3, 0x02, // JECXZ +2 (exit when ECX = 0)
        0xeb, 0xf4, // JMP -12 (back to loop start)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 5);
}

#[test]
fn test_jecxz_backward_jump() {
    let code = [
        0x48, 0xc7, 0xc1, 0x00, 0x00, 0x00, 0x00, // MOV RCX, 0 (7 bytes, ends at 0x1007)
        0xeb, 0x01, // JMP +1 (skip over HLT to JECXZ)
        0xf4, // HLT (target at 0x1009)
        0x67, 0xe3, 0xfc, // JECXZ -4 (jump back to HLT at 0x1009)
        0xf4, 0xf4, // HLT, HLT (should not execute)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_jecxz_forward_skip() {
    // Set RAX=42, then skip MOV RAX, 0 with JECXZ when ECX=0
    let code = [
        0x48, 0xc7, 0xc0, 0x2a, 0x00, 0x00, 0x00, // MOV RAX, 42
        0x48, 0x31, 0xc9, // XOR RCX, RCX (ECX = 0)
        0x67, 0xe3, 0x07, // JECXZ +7 (skip MOV RAX, 0)
        0x48, 0xc7, 0xc0, 0x00, 0x00, 0x00, 0x00, // MOV RAX, 0 (skipped)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 42); // RAX should still be 42 (MOV RAX, 0 was skipped)
}

#[test]
fn test_jecxz_preserves_flags() {
    let code = [
        0x48, 0xc7, 0xc1, 0x00, 0x00, 0x00, 0x00, // MOV RCX, 0 (doesn't affect flags)
        0xf9, // STC (set carry flag)
        0x67, 0xe3, 0x02, // JECXZ +2 (should not affect flags)
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    // Carry flag should still be set (JECXZ doesn't affect flags)
    assert_ne!(regs.rflags & 0x1, 0);
}

// ============================================================================
// JRCXZ - Jump if RCX Zero (64-bit counter)
// Opcode: E3 cb (no address-size prefix in 64-bit mode)
// ============================================================================

#[test]
fn test_jrcxz_taken_zero() {
    let code = [
        0x48, 0x31, 0xc9, // XOR RCX, RCX (RCX = 0)
        0xe3, 0x02, // JRCXZ +2
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_jrcxz_not_taken_nonzero() {
    let code = [
        0x48, 0xc7, 0xc1, 0x01, 0x00, 0x00, 0x00, // MOV RCX, 1
        0xe3, 0x05, // JRCXZ +5 (should not jump)
        0x48, 0xc7, 0xc0, 0x55, 0x00, 0x00, 0x00, // MOV RAX, 0x55
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x55);
}

#[test]
fn test_jrcxz_not_taken_high_bits() {
    let code = [
        0x48, 0xb9, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, // MOV RCX, 0x100000000
        0xe3, 0x05, // JRCXZ +5 (should not jump, RCX != 0)
        0x48, 0xc7, 0xc0, 0x77, 0x00, 0x00, 0x00, // MOV RAX, 0x77
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x77);
}

#[test]
fn test_jrcxz_not_taken_max_value() {
    let code = [
        0x48, 0xb9, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, // MOV RCX, 0xFFFFFFFFFFFFFFFF
        0xe3, 0x05, // JRCXZ +5 (should not jump)
        0x48, 0xc7, 0xc0, 0xaa, 0x00, 0x00, 0x00, // MOV RAX, 0xaa
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0xaa);
}

#[test]
fn test_jrcxz_loop_countdown() {
    let code = [
        0x48, 0xc7, 0xc1, 0x0a, 0x00, 0x00, 0x00, // MOV RCX, 10
        0x48, 0xc7, 0xc0, 0x00, 0x00, 0x00, 0x00, // MOV RAX, 0
        // loop start (offset 14):
        0x48, 0x83, 0xc0, 0x01, // ADD RAX, 1
        0x48, 0x83, 0xe9, 0x01, // SUB RCX, 1
        0xe3, 0x02, // JRCXZ +2 (exit when RCX = 0)
        0xeb, 0xf4, // JMP -12 (back to loop start)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 10);
}

#[test]
fn test_jrcxz_backward_jump() {
    let code = [
        0x48, 0x31, 0xc9, // XOR RCX, RCX (RCX = 0)
        0xeb, 0x02, // JMP +2 (skip over target)
        0xf4, // HLT (target)
        0xe3, 0xfb, // JRCXZ -5 (jump back to HLT)
        0xf4, 0xf4, // HLT, HLT (should not execute)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_jrcxz_forward_long_skip() {
    // Layout:
    // 0x1000: XOR RCX, RCX (3 bytes)
    // 0x1003: JRCXZ +14 (2 bytes) -> target = 0x1005 + 14 = 0x1013
    // 0x1005: MOV RAX, 0 (7 bytes) - skipped
    // 0x100C: MOV RBX, 0 (7 bytes) - skipped
    // 0x1013: HLT (1 byte)
    let code = [
        0x48, 0x31, 0xc9, // XOR RCX, RCX (RCX = 0)
        0xe3, 0x0e, // JRCXZ +14
        0x48, 0xc7, 0xc0, 0x00, 0x00, 0x00, 0x00, // MOV RAX, 0 (skipped)
        0x48, 0xc7, 0xc3, 0x00, 0x00, 0x00, 0x00, // MOV RBX, 0 (skipped)
        0xf4, // HLT (target)
    ];
    let mut regs = Registers::default();
    regs.rax = 0x42; // Set initial value
    regs.rbx = 0x42; // Set initial value
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x42); // RAX should not be changed (MOV skipped)
    assert_eq!(regs.rbx, 0x42); // RBX should not be changed (MOV skipped)
}

#[test]
fn test_jrcxz_preserves_flags() {
    let code = [
        0x48, 0xc7, 0xc1, 0x00, 0x00, 0x00, 0x00, // MOV RCX, 0 (doesn't affect flags)
        0xf9, // STC (set carry flag)
        0xe3, 0x02, // JRCXZ +2 (should not affect flags)
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    // Carry flag should still be set (JRCXZ doesn't affect flags)
    assert_ne!(regs.rflags & 0x1, 0);
}

#[test]
fn test_jrcxz_vs_jecxz_different_behavior() {
    // This tests that JRCXZ checks full 64-bit RCX, not just ECX
    let code = [
        0x48, 0xb9, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00,
        0x00, // MOV RCX, 0x100000000 (ECX=0, RCX!=0)
        0xe3, 0x05, // JRCXZ +5 (should NOT jump, RCX != 0)
        0x48, 0xc7, 0xc0, 0x88, 0x00, 0x00, 0x00, // MOV RAX, 0x88
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x88); // Should execute MOV
}

// ============================================================================
// Edge cases and special scenarios
// ============================================================================

#[test]
fn test_jecxz_zero_offset() {
    let code = [
        0x48, 0x31, 0xc9, // XOR RCX, RCX (ECX = 0)
        0x67, 0xe3, 0x00, // JECXZ +0 (jump to next instruction)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_jrcxz_zero_offset() {
    let code = [
        0x48, 0x31, 0xc9, // XOR RCX, RCX (RCX = 0)
        0xe3, 0x00, // JRCXZ +0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_jecxz_in_loop_skip_on_zero() {
    // Useful pattern: skip loop if counter already zero
    let code = [
        0x48, 0xc7, 0xc1, 0x00, 0x00, 0x00, 0x00, // MOV RCX, 0
        0x48, 0xc7, 0xc0, 0x00, 0x00, 0x00, 0x00, // MOV RAX, 0
        0x67, 0xe3, 0x04, // JECXZ +4 (skip loop if ECX=0)
        // loop body (should be skipped):
        0x48, 0x83, 0xc0, 0x01, // ADD RAX, 1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0); // Loop body should be skipped
}

#[test]
fn test_jrcxz_in_loop_skip_on_zero() {
    let code = [
        0x48, 0x31, 0xc9, // XOR RCX, RCX (RCX = 0)
        0x48, 0xc7, 0xc0, 0x00, 0x00, 0x00, 0x00, // MOV RAX, 0
        0xe3, 0x04, // JRCXZ +4 (skip loop if RCX=0)
        // loop body (should be skipped):
        0x48, 0x83, 0xc0, 0x01, // ADD RAX, 1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0); // Loop body should be skipped
}

#[test]
fn test_jecxz_alternating_jumps() {
    let code = [
        0x48, 0xc7, 0xc1, 0x02, 0x00, 0x00, 0x00, // MOV RCX, 2
        0x48, 0xc7, 0xc0, 0x00, 0x00, 0x00, 0x00, // MOV RAX, 0
        // loop:
        0x48, 0x83, 0xc0, 0x01, // ADD RAX, 1
        0x48, 0x83, 0xe9, 0x01, // SUB RCX, 1
        0x67, 0xe3, 0x02, // JECXZ +2 (skip decrement on zero)
        0xeb, 0xf4, // JMP -12 (back to loop)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 2);
}

#[test]
fn test_jrcxz_with_other_registers() {
    // Verify JRCXZ only checks RCX, not other registers
    let code = [
        0x48, 0x31, 0xc0, // XOR RAX, RAX (RAX = 0)
        0x48, 0x31, 0xdb, // XOR RBX, RBX (RBX = 0)
        0x48, 0xc7, 0xc1, 0x01, 0x00, 0x00, 0x00, // MOV RCX, 1 (RCX != 0)
        0xe3, 0x05, // JRCXZ +5 (should NOT jump)
        0x48, 0xc7, 0xc0, 0xcc, 0x00, 0x00, 0x00, // MOV RAX, 0xcc
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0xcc); // Should execute MOV
}

#[test]
fn test_jecxz_boundary_values() {
    // Test JECXZ with ECX that has high bit set (0x80000064)
    // Layout:
    // 0x1000: MOV RCX, 0x80000064 (7 bytes) - ECX=0x80000064
    // 0x1007: MOV RAX, 0 (7 bytes)
    // 0x100E: ADD RAX, 1 (4 bytes) <- loop
    // 0x1012: SUB RCX, 1 (4 bytes)
    // 0x1016: JECXZ +2 (3 bytes) -> target = 0x1019 + 2 = 0x101B
    // 0x1019: JMP -13 (2 bytes) -> target = 0x101B - 13 = 0x100E
    // 0x101B: HLT (1 byte)
    let code = [
        0x48, 0xc7, 0xc1, 0x64, 0x00, 0x00, 0x00, // MOV RCX, 100
        0x48, 0xc7, 0xc0, 0x00, 0x00, 0x00, 0x00, // MOV RAX, 0
        // loop:
        0x48, 0x83, 0xc0, 0x01, // ADD RAX, 1
        0x48, 0x83, 0xe9, 0x01, // SUB RCX, 1
        0x67, 0xe3, 0x02, // JECXZ +2
        0xeb, 0xf3, // JMP -13 (continue loop)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 100);
}

#[test]
fn test_jrcxz_max_backward_offset() {
    let code = [
        0x48, 0x31, 0xc9, // XOR RCX, RCX (RCX = 0)
        0xeb, 0x02, // JMP +2 (forward to JRCXZ)
        0xf4, // HLT (target)
        0xe3, 0xfb, // JRCXZ -5 (max backward in this test)
        0xf4, 0xf4, // HLT, HLT (should not execute)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_jecxz_max_forward_offset() {
    // Layout:
    // 0x1000: XOR RCX, RCX (3 bytes)
    // 0x1003: JECXZ +127 (3 bytes) -> target = 0x1006 + 127 = 0x1085
    // 0x1006: 127 NOPs (127 bytes)
    // 0x1085: HLT (1 byte)
    let code = [
        0x48, 0x31, 0xc9, // XOR RCX, RCX (ECX = 0)
        0x67, 0xe3, 0x7f, // JECXZ +127 (max positive signed byte)
        // Add padding to reach offset 127 (127 NOPs needed)
        0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90,
        0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90,
        0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90,
        0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90,
        0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90,
        0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90,
        0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90,
        0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90,
        0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, // NOP padding (127 bytes total)
        0xf4, // HLT (at offset +127)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_jrcxz_nested_condition() {
    // Layout:
    // 0x1000: MOV RCX, 5 (7 bytes)
    // 0x1007: MOV RAX, 0 (7 bytes)
    // 0x100E: ADD RAX, 1 (4 bytes) <- outer loop
    // 0x1012: SUB RCX, 1 (4 bytes)
    // 0x1016: JRCXZ +5 (2 bytes) -> target = 0x1018 + 5 = 0x101D
    // 0x1018: CMP RAX, RCX (3 bytes)
    // 0x101B: JNE -15 (2 bytes) -> target = 0x101D - 15 = 0x100E
    // 0x101D: HLT (1 byte)
    let code = [
        0x48, 0xc7, 0xc1, 0x05, 0x00, 0x00, 0x00, // MOV RCX, 5
        0x48, 0xc7, 0xc0, 0x00, 0x00, 0x00, 0x00, // MOV RAX, 0
        // outer:
        0x48, 0x83, 0xc0, 0x01, // ADD RAX, 1
        0x48, 0x83, 0xe9, 0x01, // SUB RCX, 1
        0xe3, 0x05, // JRCXZ +5 (exit if zero)
        0x48, 0x39, 0xc8, // CMP RAX, RCX
        0x75, 0xf1, // JNE -15 (back to outer)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 5);
}

#[test]
fn test_combined_jecxz_jrcxz() {
    // Shows that with proper prefixing, same opcode tests different registers
    let code = [
        0x48, 0xb9, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00,
        0x00, // MOV RCX, 0x100000000 (ECX=0, RCX!=0)
        0x67, 0xe3, 0x02, // JECXZ +2 (should jump, ECX=0)
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xe3, 0x05, // JRCXZ +5 (should not jump, RCX!=0)
        0x48, 0xc7, 0xc0, 0xab, 0x00, 0x00, 0x00, // MOV RAX, 0xab
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0xab);
}

#[test]
fn test_jecxz_with_partial_register_write() {
    let code = [
        0x48, 0xb9, 0xff, 0xff, 0xff, 0xff, 0x01, 0x00, 0x00, 0x00, // MOV RCX, 0x1FFFFFFFF
        0xb1, 0x00, // MOV CL, 0 (clears low byte)
        0xb5, 0x00, // MOV CH, 0 (clears next byte)
        0x66, 0xc7, 0xc1, 0x00, 0x00, // MOV CX, 0 (clears low word, ECX=0)
        0x67, 0xe3, 0x02, // JECXZ +2 (should jump)
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_jrcxz_decrement_to_zero() {
    let code = [
        0x48, 0xc7, 0xc1, 0x01, 0x00, 0x00, 0x00, // MOV RCX, 1
        0x48, 0xff, 0xc9, // DEC RCX
        0xe3, 0x02, // JRCXZ +2 (should jump)
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_jecxz_increment_from_negative() {
    let code = [
        0x48, 0xc7, 0xc1, 0xff, 0xff, 0xff, 0xff, // MOV RCX, 0xFFFFFFFF (ECX=-1)
        0x48, 0xff, 0xc1, // INC RCX (ECX=0)
        0x67, 0xe3, 0x02, // JECXZ +2 (should jump)
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_jrcxz_with_neg_instruction() {
    let code = [
        0x48, 0x31, 0xc9, // XOR RCX, RCX
        0x48, 0xf7, 0xd9, // NEG RCX (RCX still 0)
        0xe3, 0x02, // JRCXZ +2 (should jump)
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_jecxz_multiple_sequential() {
    let code = [
        0x48, 0x31, 0xc9, // XOR RCX, RCX
        0x67, 0xe3, 0x01, // JECXZ +1
        0xf4, // HLT (should not execute)
        0x67, 0xe3, 0x01, // JECXZ +1
        0xf4, // HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_jrcxz_chained_jumps() {
    let code = [
        0x48, 0x31, 0xc9, // XOR RCX, RCX
        0xe3, 0x04, // JRCXZ +4
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xeb, 0x02, // JMP +2
        0xe3, 0x01, // JRCXZ +1 (chained)
        0xf4, // HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_jecxz_with_xchg() {
    let code = [
        0x48, 0x31, 0xc9, // XOR RCX, RCX (ECX=0)
        0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1
        0x48, 0x91, // XCHG RCX, RAX (ECX=1, RAX=0)
        0x67, 0xe3, 0x05, // JECXZ +5 (should not jump)
        0x48, 0xc7, 0xc3, 0x42, 0x00, 0x00, 0x00, // MOV RBX, 0x42
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rbx, 0x42);
}

#[test]
fn test_jrcxz_stress_test() {
    // Layout:
    // 0x1000: MOV RCX, 100 (7 bytes)
    // 0x1007: MOV RAX, 0 (7 bytes)
    // 0x100E: JRCXZ +8 (2 bytes) <- loop start, target = 0x1010 + 8 = 0x1018
    // 0x1010: INC RAX (3 bytes)
    // 0x1013: DEC RCX (3 bytes)
    // 0x1016: JMP -10 (2 bytes) -> target = 0x1018 - 10 = 0x100E
    // 0x1018: HLT (1 byte)
    let code = [
        0x48, 0xc7, 0xc1, 0x64, 0x00, 0x00, 0x00, // MOV RCX, 100
        0x48, 0xc7, 0xc0, 0x00, 0x00, 0x00, 0x00, // MOV RAX, 0
        // loop:
        0xe3, 0x08, // JRCXZ +8 (exit when zero)
        0x48, 0xff, 0xc0, // INC RAX
        0x48, 0xff, 0xc9, // DEC RCX
        0xeb, 0xf6, // JMP -10 (back to JRCXZ)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 100);
}
