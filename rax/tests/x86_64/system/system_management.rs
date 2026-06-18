//! Tests for System Management Instructions.
//!
//! Instructions covered:
//! - PCONFIG - Platform Configuration
//! - WBNOINVD - Write Back and Do Not Invalidate Cache
//! - INVPCID - Invalidate Process-Context Identifier
//!
//! References: docs/pconfig.txt, docs/wbnoinvd.txt, docs/invpcid.txt

use crate::common::*;
use rax::cpu::Registers;

// ============================================================================
// PCONFIG Tests - Platform Configuration
// ============================================================================

#[test]
fn test_pconfig_basic() {
    // PCONFIG - Platform configuration instruction
    // Opcode: 0F 01 C5
    // EAX contains leaf function
    let code = [
        0x48, 0x31, 0xC0, // XOR RAX, RAX (leaf 0)
        0x0F, 0x01, 0xC5, // PCONFIG
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_pconfig_different_leaves() {
    // Test different PCONFIG leaf functions
    let code = [
        0x48, 0xC7, 0xC0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1
        0x0F, 0x01, 0xC5, // PCONFIG
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_pconfig_with_parameters() {
    // PCONFIG with parameters in other registers
    let code = [
        0x48, 0x31, 0xC0, // XOR RAX, RAX
        0x48, 0xC7, 0xC3, 0x00, 0x10, 0x00, 0x00, // MOV RBX, 0x1000
        0x48, 0xC7, 0xC1, 0x00, 0x20, 0x00, 0x00, // MOV RCX, 0x2000
        0x0F, 0x01, 0xC5, // PCONFIG
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_pconfig_no_operands() {
    // PCONFIG takes no explicit operands (uses registers)
    let code = [
        0x48, 0x31, 0xC0, // XOR RAX, RAX
        0x0F, 0x01, 0xC5, // PCONFIG
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_pconfig_multiple() {
    // Multiple PCONFIG calls
    let code = [
        0x48, 0x31, 0xC0, // XOR RAX, RAX
        0x0F, 0x01, 0xC5, // PCONFIG
        0x48, 0xC7, 0xC0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1
        0x0F, 0x01, 0xC5, // PCONFIG
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// WBNOINVD Tests - Write Back No Invalidate
// ============================================================================

#[test]
fn test_wbnoinvd_basic() {
    // WBNOINVD - Write back and do not invalidate cache
    // Opcode: F3 0F 09
    let code = [
        0xF3, 0x0F, 0x09, // WBNOINVD
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_wbnoinvd_no_operands() {
    // WBNOINVD takes no operands
    let code = [0xF3, 0x0F, 0x09, 0xF4];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_wbnoinvd_preserves_registers() {
    // WBNOINVD should not modify any registers
    let code = [
        0x48, 0xC7, 0xC0, 0x11, 0x11, 0x11, 0x11, // MOV RAX, 0x11111111
        0x48, 0xC7, 0xC3, 0x22, 0x22, 0x22, 0x22, // MOV RBX, 0x22222222
        0x48, 0xC7, 0xC1, 0x33, 0x33, 0x33, 0x33, // MOV RCX, 0x33333333
        0xF3, 0x0F, 0x09, // WBNOINVD
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x11111111, "RAX should not be modified");
    assert_eq!(regs.rbx, 0x22222222, "RBX should not be modified");
    assert_eq!(regs.rcx, 0x33333333, "RCX should not be modified");
}

#[test]
fn test_wbnoinvd_multiple() {
    // Multiple WBNOINVD operations
    let code = [
        0xF3, 0x0F, 0x09, // WBNOINVD
        0xF3, 0x0F, 0x09, // WBNOINVD
        0xF3, 0x0F, 0x09, // WBNOINVD
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_wbnoinvd_preserves_flags() {
    // WBNOINVD should not modify flags
    let code = [
        0x48, 0xC7, 0xC0, 0xFF, 0xFF, 0xFF, 0xFF, // MOV RAX, -1
        0x48, 0x83, 0xC0, 0x01, // ADD RAX, 1 (sets ZF)
        0xF3, 0x0F, 0x09, // WBNOINVD
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_ne!(regs.rflags & 0x40, 0, "ZF should still be set");
}

// ============================================================================
// INVPCID Tests - Invalidate Process-Context Identifier
// ============================================================================

#[test]
fn test_invpcid_individual_address() {
    // INVPCID - Invalidate TLB entries for PCID
    // Opcode: 66 0F 38 82
    // Type 0: Individual-address invalidation
    let code = [
        0x48, 0x31, 0xC0, // XOR RAX, RAX (type 0)
        0x48, 0xC7, 0xC3, 0x00, 0x10, 0x00, 0x00, // MOV RBX, 0x1000 (descriptor)
        0x66, 0x0F, 0x38, 0x82, 0x03, // INVPCID rax, [rbx]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_invpcid_single_context() {
    // Type 1: Single-context invalidation
    let code = [
        0x48, 0xC7, 0xC1, 0x01, 0x00, 0x00, 0x00, // MOV RCX, 1 (type 1)
        0x48, 0xC7, 0xC2, 0x00, 0x20, 0x00, 0x00, // MOV RDX, 0x2000 (descriptor)
        0x66, 0x0F, 0x38, 0x82, 0x0A, // INVPCID rcx, [rdx]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_invpcid_all_contexts() {
    // Type 2: All-contexts invalidation
    let code = [
        0x48, 0xC7, 0xC0, 0x02, 0x00, 0x00, 0x00, // MOV RAX, 2 (type 2)
        0x48, 0xC7, 0xC3, 0x00, 0x30, 0x00, 0x00, // MOV RBX, 0x3000
        0x66, 0x0F, 0x38, 0x82, 0x03, // INVPCID rax, [rbx]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_invpcid_all_contexts_including_globals() {
    // Type 3: All-contexts including globals
    let code = [
        0x48, 0xC7, 0xC1, 0x03, 0x00, 0x00, 0x00, // MOV RCX, 3 (type 3)
        0x48, 0xC7, 0xC2, 0x00, 0x40, 0x00, 0x00, // MOV RDX, 0x4000
        0x66, 0x0F, 0x38, 0x82, 0x0A, // INVPCID rcx, [rdx]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_invpcid_different_types() {
    // Test all 4 INVPCID types
    let code = [
        0x48, 0xC7, 0xC3, 0x00, 0x10, 0x00, 0x00, // MOV RBX, 0x1000
        // Type 0
        0x48, 0x31, 0xC0, // XOR RAX, RAX
        0x66, 0x0F, 0x38, 0x82, 0x03, // INVPCID rax, [rbx]
        // Type 1
        0x48, 0xC7, 0xC0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1
        0x66, 0x0F, 0x38, 0x82, 0x03, // INVPCID rax, [rbx]
        // Type 2
        0x48, 0xC7, 0xC0, 0x02, 0x00, 0x00, 0x00, // MOV RAX, 2
        0x66, 0x0F, 0x38, 0x82, 0x03, // INVPCID rax, [rbx]
        // Type 3
        0x48, 0xC7, 0xC0, 0x03, 0x00, 0x00, 0x00, // MOV RAX, 3
        0x66, 0x0F, 0x38, 0x82, 0x03, // INVPCID rax, [rbx]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_invpcid_with_displacement() {
    // INVPCID with memory displacement
    let code = [
        0x48, 0x31, 0xC0, // XOR RAX, RAX
        0x48, 0xC7, 0xC1, 0x00, 0x10, 0x00, 0x00, // MOV RCX, 0x1000
        0x66, 0x0F, 0x38, 0x82, 0x81, 0x00, 0x04, 0x00, 0x00, // INVPCID rax, [rcx+0x400]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_invpcid_multiple_invalidations() {
    // Multiple INVPCID calls
    let code = [
        0x48, 0xC7, 0xC3, 0x00, 0x10, 0x00, 0x00, // MOV RBX, 0x1000
        0x48, 0x31, 0xC0, // XOR RAX, RAX
        0x66, 0x0F, 0x38, 0x82, 0x03, // INVPCID rax, [rbx]
        0x48, 0xC7, 0xC0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1
        0x66, 0x0F, 0x38, 0x82, 0x03, // INVPCID rax, [rbx]
        0x48, 0xC7, 0xC0, 0x02, 0x00, 0x00, 0x00, // MOV RAX, 2
        0x66, 0x0F, 0x38, 0x82, 0x03, // INVPCID rax, [rbx]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// Combined System Management Tests
// ============================================================================

#[test]
fn test_cache_invalidation_sequence() {
    // Sequence of cache and TLB operations
    let code = [
        0xF3, 0x0F, 0x09, // WBNOINVD (writeback caches)
        0x48, 0xC7, 0xC0, 0x02, 0x00, 0x00, 0x00, // MOV RAX, 2
        0x48, 0xC7, 0xC3, 0x00, 0x10, 0x00, 0x00, // MOV RBX, 0x1000
        0x66, 0x0F, 0x38, 0x82, 0x03, // INVPCID rax, [rbx] (invalidate TLB)
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_platform_config_with_cache_ops() {
    // Platform configuration followed by cache operations
    let code = [
        0x48, 0x31, 0xC0, // XOR RAX, RAX
        0x0F, 0x01, 0xC5, // PCONFIG
        0xF3, 0x0F, 0x09, // WBNOINVD
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_tlb_shootdown_sequence() {
    // TLB shootdown-like sequence
    let code = [
        // Invalidate specific address
        0x48, 0x31, 0xC0, // XOR RAX, RAX (type 0)
        0x48, 0xC7, 0xC3, 0x00, 0x10, 0x00, 0x00, // MOV RBX, 0x1000
        0x66, 0x0F, 0x38, 0x82, 0x03, // INVPCID rax, [rbx]
        // Invalidate single context
        0x48, 0xC7, 0xC0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1
        0x66, 0x0F, 0x38, 0x82, 0x03, // INVPCID rax, [rbx]
        // Writeback caches
        0xF3, 0x0F, 0x09, // WBNOINVD
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}
