//! Tests for the CLAC and STAC instructions.
//!
//! CLAC - Clear AC Flag in EFLAGS Register
//! STAC - Set AC Flag in EFLAGS Register
//!
//! These instructions manipulate the Alignment Check (AC) flag in the EFLAGS
//! register. When combined with the SMAP (Supervisor Mode Access Prevention)
//! feature, they control access to user-mode pages from supervisor mode.
//!
//! CLAC: Clears AC flag (bit 18 in RFLAGS)
//!   - Opcode: 0F 01 CA
//!   - Disables alignment checking
//!   - With SMAP: disallows supervisor access to user pages
//!
//! STAC: Sets AC flag (bit 18 in RFLAGS)
//!   - Opcode: 0F 01 CB
//!   - Enables alignment checking
//!   - With SMAP: allows supervisor access to user pages
//!
//! Both instructions require CPL=0 (ring 0) and SMAP support.
//!
//! Reference: docs/clac.txt, docs/stac.txt

use crate::common::*;
use rax::cpu::Registers;

// AC flag is bit 18 of RFLAGS
const AC_FLAG: u64 = 1 << 18;

// ============================================================================
// STAC - Set AC Flag Tests
// ============================================================================

#[test]
fn test_stac_sets_ac_flag() {
    // STAC should set bit 18 (AC) in RFLAGS
    let code = [
        // Clear all flags first
        0x48, 0x31, 0xC0, // XOR RAX, RAX
        0x50, // PUSH RAX
        0x9D, // POPFQ
        // Execute STAC
        0x0F, 0x01, 0xCB, // STAC
        // Read RFLAGS
        0x9C, // PUSHFQ
        0x58, // POP RAX
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    // AC flag (bit 18) should be set
    assert_ne!(regs.rax & AC_FLAG, 0, "STAC should set AC flag");
}

#[test]
fn test_stac_only_affects_ac_flag() {
    // STAC should only set AC, leaving other flags unchanged
    let code = [
        // Set some flags (ZF, CF, SF)
        0x48, 0xC7, 0xC0, 0xFF, 0xFF, 0xFF, 0xFF, // MOV RAX, -1
        0x48, 0x83, 0xC0, 0x01, // ADD RAX, 1 (sets ZF)
        0x48, 0xF7, 0xD8, // NEG RAX (sets flags)
        // Save flags before STAC
        0x9C, // PUSHFQ
        0x5B, // POP RBX
        // Execute STAC
        0x0F, 0x01, 0xCB, // STAC
        // Read flags after STAC
        0x9C, // PUSHFQ
        0x58, // POP RAX
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    // AC should be newly set
    assert_ne!(regs.rax & AC_FLAG, 0, "AC should be set after STAC");
    assert_eq!(regs.rbx & AC_FLAG, 0, "AC should not be set before STAC");

    // Other flags should be unchanged (mask out AC bit and reserved bits)
    let mask = !(AC_FLAG | 0xFFC00000); // Ignore AC and reserved bits
    assert_eq!(
        regs.rax & mask,
        regs.rbx & mask,
        "Other flags should be unchanged"
    );
}

#[test]
fn test_stac_when_ac_already_set() {
    // STAC when AC is already set should be a no-op
    let code = [
        // Manually set AC flag
        0x48, 0xB8, 0x00, 0x04, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV RAX, AC_FLAG
        0x50, // PUSH RAX
        0x9D, // POPFQ
        // Execute STAC (should be no-op)
        0x0F, 0x01, 0xCB, // STAC
        // Read RFLAGS
        0x9C, // PUSHFQ
        0x58, // POP RAX
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    // AC should still be set
    assert_ne!(regs.rax & AC_FLAG, 0, "AC flag should remain set");
}

#[test]
fn test_stac_multiple_times() {
    // Multiple STAC executions
    let code = [
        0x0F, 0x01, 0xCB, // STAC
        0x0F, 0x01, 0xCB, // STAC
        0x0F, 0x01, 0xCB, // STAC
        0x9C, // PUSHFQ
        0x58, // POP RAX
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_ne!(
        regs.rax & AC_FLAG,
        0,
        "AC should be set after multiple STAC"
    );
}

// ============================================================================
// CLAC - Clear AC Flag Tests
// ============================================================================

#[test]
fn test_clac_clears_ac_flag() {
    // CLAC should clear bit 18 (AC) in RFLAGS
    let code = [
        // First set AC with STAC
        0x0F, 0x01, 0xCB, // STAC
        // Verify it's set
        0x9C, // PUSHFQ
        0x5B, // POP RBX
        // Clear it with CLAC
        0x0F, 0x01, 0xCA, // CLAC
        // Read RFLAGS
        0x9C, // PUSHFQ
        0x58, // POP RAX
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    // AC should have been set, then cleared
    assert_ne!(regs.rbx & AC_FLAG, 0, "AC should be set after STAC");
    assert_eq!(regs.rax & AC_FLAG, 0, "AC should be cleared after CLAC");
}

#[test]
fn test_clac_only_affects_ac_flag() {
    // CLAC should only clear AC, leaving other flags unchanged
    let code = [
        // Set some flags and AC
        0x48, 0xC7, 0xC0, 0xFF, 0xFF, 0xFF, 0xFF, // MOV RAX, -1
        0x48, 0x83, 0xC0, 0x01, // ADD RAX, 1 (sets ZF)
        0x0F, 0x01, 0xCB, // STAC
        // Save flags before CLAC
        0x9C, // PUSHFQ
        0x5B, // POP RBX
        // Execute CLAC
        0x0F, 0x01, 0xCA, // CLAC
        // Read flags after CLAC
        0x9C, // PUSHFQ
        0x58, // POP RAX
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    // AC should be cleared
    assert_ne!(regs.rbx & AC_FLAG, 0, "AC should be set before CLAC");
    assert_eq!(regs.rax & AC_FLAG, 0, "AC should be cleared after CLAC");

    // Other flags should be unchanged
    let mask = !(AC_FLAG | 0xFFC00000);
    assert_eq!(
        regs.rax & mask,
        regs.rbx & mask,
        "Other flags should be unchanged"
    );
}

#[test]
fn test_clac_when_ac_already_clear() {
    // CLAC when AC is already clear should be a no-op
    let code = [
        // Clear all flags
        0x48, 0x31, 0xC0, // XOR RAX, RAX
        0x50, // PUSH RAX
        0x9D, // POPFQ
        // Execute CLAC (should be no-op)
        0x0F, 0x01, 0xCA, // CLAC
        // Read RFLAGS
        0x9C, // PUSHFQ
        0x58, // POP RAX
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    // AC should remain clear
    assert_eq!(regs.rax & AC_FLAG, 0, "AC flag should remain clear");
}

#[test]
fn test_clac_multiple_times() {
    // Multiple CLAC executions
    let code = [
        0x0F, 0x01, 0xCB, // STAC (set it first)
        0x0F, 0x01, 0xCA, // CLAC
        0x0F, 0x01, 0xCA, // CLAC
        0x0F, 0x01, 0xCA, // CLAC
        0x9C, // PUSHFQ
        0x58, // POP RAX
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & AC_FLAG,
        0,
        "AC should be clear after multiple CLAC"
    );
}

// ============================================================================
// STAC/CLAC Combination Tests
// ============================================================================

#[test]
fn test_stac_clac_toggle() {
    // Toggle AC flag multiple times
    let code = [
        0x0F, 0x01, 0xCB, // STAC
        0x9C, // PUSHFQ
        0x58, // POP RAX (should have AC set)
        0x0F, 0x01, 0xCA, // CLAC
        0x9C, // PUSHFQ
        0x5B, // POP RBX (should have AC clear)
        0x0F, 0x01, 0xCB, // STAC
        0x9C, // PUSHFQ
        0x59, // POP RCX (should have AC set)
        0x0F, 0x01, 0xCA, // CLAC
        0x9C, // PUSHFQ
        0x5A, // POP RDX (should have AC clear)
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_ne!(regs.rax & AC_FLAG, 0, "1st: AC should be set after STAC");
    assert_eq!(regs.rbx & AC_FLAG, 0, "2nd: AC should be clear after CLAC");
    assert_ne!(regs.rcx & AC_FLAG, 0, "3rd: AC should be set after STAC");
    assert_eq!(regs.rdx & AC_FLAG, 0, "4th: AC should be clear after CLAC");
}

#[test]
fn test_stac_clac_with_other_instructions() {
    // Interleave STAC/CLAC with other instructions
    let code = [
        0x48, 0xC7, 0xC0, 0x42, 0x00, 0x00, 0x00, // MOV RAX, 0x42
        0x0F, 0x01, 0xCB, // STAC
        0x48, 0xFF, 0xC0, // INC RAX
        0x0F, 0x01, 0xCA, // CLAC
        0x48, 0xFF, 0xC0, // INC RAX
        0x9C, // PUSHFQ
        0x5B, // POP RBX
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x44, "RAX should be incremented twice");
    assert_eq!(regs.rbx & AC_FLAG, 0, "AC should be clear at end");
}

#[test]
fn test_clac_stac_sequence() {
    // Test the sequence: clear, set, clear, set
    let code = [
        0x0F, 0x01, 0xCA, // CLAC
        0x0F, 0x01, 0xCB, // STAC
        0x0F, 0x01, 0xCA, // CLAC
        0x0F, 0x01, 0xCB, // STAC
        0x9C, // PUSHFQ
        0x58, // POP RAX
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_ne!(regs.rax & AC_FLAG, 0, "AC should be set after final STAC");
}

// ============================================================================
// Edge Cases and Register Preservation
// ============================================================================

#[test]
fn test_stac_preserves_all_registers() {
    // STAC should not modify any general-purpose registers
    let code = [
        // Set all registers to known values
        0x48, 0xC7, 0xC0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1
        0x48, 0xC7, 0xC3, 0x02, 0x00, 0x00, 0x00, // MOV RBX, 2
        0x48, 0xC7, 0xC1, 0x03, 0x00, 0x00, 0x00, // MOV RCX, 3
        0x48, 0xC7, 0xC2, 0x04, 0x00, 0x00, 0x00, // MOV RDX, 4
        0x48, 0xC7, 0xC6, 0x05, 0x00, 0x00, 0x00, // MOV RSI, 5
        0x48, 0xC7, 0xC7, 0x06, 0x00, 0x00, 0x00, // MOV RDI, 6
        // Execute STAC
        0x0F, 0x01, 0xCB, // STAC
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 1, "RAX should be preserved");
    assert_eq!(regs.rbx, 2, "RBX should be preserved");
    assert_eq!(regs.rcx, 3, "RCX should be preserved");
    assert_eq!(regs.rdx, 4, "RDX should be preserved");
    assert_eq!(regs.rsi, 5, "RSI should be preserved");
    assert_eq!(regs.rdi, 6, "RDI should be preserved");
}

#[test]
fn test_clac_preserves_all_registers() {
    // CLAC should not modify any general-purpose registers
    let code = [
        // Set all registers to known values
        0x48, 0xC7, 0xC0, 0x11, 0x00, 0x00, 0x00, // MOV RAX, 0x11
        0x48, 0xC7, 0xC3, 0x22, 0x00, 0x00, 0x00, // MOV RBX, 0x22
        0x48, 0xC7, 0xC1, 0x33, 0x00, 0x00, 0x00, // MOV RCX, 0x33
        0x48, 0xC7, 0xC2, 0x44, 0x00, 0x00, 0x00, // MOV RDX, 0x44
        0x48, 0xC7, 0xC6, 0x55, 0x00, 0x00, 0x00, // MOV RSI, 0x55
        0x48, 0xC7, 0xC7, 0x66, 0x00, 0x00, 0x00, // MOV RDI, 0x66
        // Execute CLAC
        0x0F, 0x01, 0xCA, // CLAC
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x11, "RAX should be preserved");
    assert_eq!(regs.rbx, 0x22, "RBX should be preserved");
    assert_eq!(regs.rcx, 0x33, "RCX should be preserved");
    assert_eq!(regs.rdx, 0x44, "RDX should be preserved");
    assert_eq!(regs.rsi, 0x55, "RSI should be preserved");
    assert_eq!(regs.rdi, 0x66, "RDI should be preserved");
}

#[test]
fn test_stac_clac_in_sequence_with_arithmetic() {
    // Mix STAC/CLAC with arithmetic operations
    let code = [
        0x48, 0xC7, 0xC0, 0x10, 0x00, 0x00, 0x00, // MOV RAX, 16
        0x0F, 0x01, 0xCB, // STAC
        0x48, 0x83, 0xC0, 0x10, // ADD RAX, 16
        0x0F, 0x01, 0xCA, // CLAC
        0x48, 0x83, 0xC0, 0x10, // ADD RAX, 16
        0x0F, 0x01, 0xCB, // STAC
        0x48, 0x83, 0xC0, 0x10, // ADD RAX, 16
        0x9C, // PUSHFQ
        0x5B, // POP RBX
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 64, "RAX should be 16*4 = 64");
    assert_ne!(regs.rbx & AC_FLAG, 0, "AC should be set at end");
}

#[test]
fn test_ac_flag_bit_position() {
    // Verify AC flag is at bit 18
    let code = [
        0x0F, 0x01, 0xCB, // STAC
        0x9C, // PUSHFQ
        0x58, // POP RAX
        0x0F, 0x01, 0xCA, // CLAC
        0x9C, // PUSHFQ
        0x5B, // POP RBX
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Difference should be exactly bit 18
    let diff = regs.rax ^ regs.rbx;
    assert_eq!(
        diff, AC_FLAG,
        "Only bit 18 should differ between STAC and CLAC states"
    );
}

#[test]
fn test_stac_clac_rapid_toggle() {
    // Rapidly toggle AC flag 10 times
    let mut code = vec![];

    // Toggle 10 times
    for i in 0..10 {
        if i % 2 == 0 {
            code.extend_from_slice(&[0x0F, 0x01, 0xCB]); // STAC
        } else {
            code.extend_from_slice(&[0x0F, 0x01, 0xCA]); // CLAC
        }
    }

    code.extend_from_slice(&[0x9C]); // PUSHFQ
    code.extend_from_slice(&[0x58]); // POP RAX
    code.push(0xF4); // HLT

    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // After 10 toggles (starting with STAC), should be clear (even count)
    assert_eq!(
        regs.rax & AC_FLAG,
        0,
        "AC should be clear after even number of toggles"
    );
}

#[test]
fn test_stac_with_all_flags_set() {
    // Set all possible flags, then STAC
    let code = [
        // Set many flags via operations
        0x48, 0xC7, 0xC0, 0xFF, 0xFF, 0xFF, 0xFF, // MOV RAX, -1
        0x48, 0x83, 0xC0, 0x01, // ADD RAX, 1 (CF=1, ZF=1, PF=1)
        0x48, 0xF7, 0xD8, // NEG RAX (many flags)
        // Manually set more flags including TF, IF, DF, etc.
        0x9C, // PUSHFQ
        0x58, // POP RAX
        0x48, 0x81, 0xC8, 0x00, 0x05, 0x00, 0x00, // OR RAX, 0x500 (DF, IF)
        0x50, // PUSH RAX
        0x9D, // POPFQ
        // Now STAC
        0x0F, 0x01, 0xCB, // STAC
        0x9C, // PUSHFQ
        0x5B, // POP RBX
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    // AC should be set in addition to other flags
    assert_ne!(regs.rbx & AC_FLAG, 0, "AC should be set");
}
