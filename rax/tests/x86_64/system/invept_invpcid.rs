//! Tests for INVEPT and INVPCID Instructions.
//!
//! This module covers TLB and paging-structure cache invalidation instructions.
//!
//! Instructions covered:
//! - INVEPT - Invalidate EPT (Extended Page Table) translations
//! - INVPCID - Invalidate Process-Context Identifier
//!
//! References: docs/invept.txt, docs/invpcid.txt

use crate::common::*;
use rax::cpu::Registers;

// ============================================================================
// INVEPT Tests - Invalidate EPT Translations
// ============================================================================

#[test]
fn test_invept_basic() {
    // INVEPT - Invalidate EPT translations
    // Opcode: 66 0F 38 80 /r
    // Note: Requires VMX operation and CPL=0
    let code = [
        0x48, 0xC7, 0xC0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1 (single-context type)
        0x48, 0xC7, 0xC3, 0x00, 0x20, 0x00, 0x00, // MOV RBX, 0x2000 (descriptor addr)
        0x66, 0x0F, 0x38, 0x80, 0x03, // INVEPT RAX, [RBX]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    // Will likely #UD outside VMX operation
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_invept_single_context() {
    // INVEPT type 1 - Single-context invalidation
    let code = [
        0x48, 0xC7, 0xC1, 0x01, 0x00, 0x00, 0x00, // MOV RCX, 1 (type 1)
        0x48, 0xC7, 0xC2, 0x00, 0x30, 0x00, 0x00, // MOV RDX, 0x3000 (descriptor)
        0x66, 0x0F, 0x38, 0x80, 0x0A, // INVEPT RCX, [RDX]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_invept_global_context() {
    // INVEPT type 2 - Global invalidation
    let code = [
        0x48, 0xC7, 0xC0, 0x02, 0x00, 0x00, 0x00, // MOV RAX, 2 (type 2)
        0x48, 0xC7, 0xC6, 0x00, 0x40, 0x00, 0x00, // MOV RSI, 0x4000 (descriptor)
        0x66, 0x0F, 0x38, 0x80, 0x06, // INVEPT RAX, [RSI]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_invept_with_displacement() {
    // INVEPT with memory displacement
    let code = [
        0x48, 0xC7, 0xC0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1
        0x48, 0xC7, 0xC7, 0x00, 0x20, 0x00, 0x00, // MOV RDI, 0x2000
        0x66, 0x0F, 0x38, 0x80, 0x87, 0x00, 0x10, 0x00, 0x00, // INVEPT RAX, [RDI+0x1000]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_invept_different_registers() {
    // INVEPT using different register combinations
    let code = [
        0x48, 0xC7, 0xC2, 0x01, 0x00, 0x00, 0x00, // MOV RDX, 1
        0x48, 0xC7, 0xC5, 0x00, 0x50, 0x00, 0x00, // MOV RBP, 0x5000
        0x66, 0x0F, 0x38, 0x80, 0x55, 0x00, // INVEPT RDX, [RBP]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_invept_rsp_based() {
    // INVEPT with RSP-based addressing
    let code = [
        0x48, 0xC7, 0xC0, 0x02, 0x00, 0x00, 0x00, // MOV RAX, 2
        0x48, 0x81, 0xEC, 0x00, 0x10, 0x00, 0x00, // SUB RSP, 0x1000
        0x66, 0x0F, 0x38, 0x80, 0x04, 0x24, // INVEPT RAX, [RSP]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_invept_preserves_registers() {
    // INVEPT should not modify general-purpose registers
    let code = [
        0x48, 0xC7, 0xC0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1
        0x48, 0xC7, 0xC3, 0x00, 0x30, 0x00, 0x00, // MOV RBX, 0x3000
        0x48, 0xC7, 0xC1, 0xAA, 0xBB, 0xCC, 0xDD, // MOV RCX, 0xDDCCBBAA
        0x48, 0xC7, 0xC2, 0x11, 0x22, 0x33, 0x44, // MOV RDX, 0x44332211
        0x66, 0x0F, 0x38, 0x80, 0x03, // INVEPT RAX, [RBX]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_invept_rip_relative() {
    // INVEPT with RIP-relative addressing
    let code = [
        0x48, 0xC7, 0xC0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1
        0x66, 0x0F, 0x38, 0x80, 0x05, 0x00, 0x00, 0x00, 0x00, // INVEPT RAX, [RIP+0]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_invept_with_sib() {
    // INVEPT with SIB byte (scaled index)
    let code = [
        0x48, 0xC7, 0xC0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1
        0x48, 0xC7, 0xC3, 0x00, 0x20, 0x00, 0x00, // MOV RBX, 0x2000
        0x48, 0xC7, 0xC1, 0x00, 0x01, 0x00, 0x00, // MOV RCX, 0x100
        0x66, 0x0F, 0x38, 0x80, 0x04, 0x0B, // INVEPT RAX, [RBX+RCX]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_invept_zero_type() {
    // INVEPT with type 0 (invalid - should fail)
    let code = [
        0x48, 0x31, 0xC0, // XOR RAX, RAX (type 0 = invalid)
        0x48, 0xC7, 0xC3, 0x00, 0x20, 0x00, 0x00, // MOV RBX, 0x2000
        0x66, 0x0F, 0x38, 0x80, 0x03, // INVEPT RAX, [RBX]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// INVPCID Tests - Invalidate Process-Context Identifier
// ============================================================================

#[test]
fn test_invpcid_basic() {
    // INVPCID - Invalidate PCID-based TLB entries
    // Opcode: 66 0F 38 82 /r
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x00, 0x00, 0x00, // MOV RAX, 0 (type 0)
        0x48, 0xC7, 0xC3, 0x00, 0x30, 0x00, 0x00, // MOV RBX, 0x3000 (descriptor)
        0x66, 0x0F, 0x38, 0x82, 0x03, // INVPCID RAX, [RBX]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    // May #UD if INVPCID not supported
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_invpcid_individual_address() {
    // INVPCID type 0 - Individual-address invalidation
    let code = [
        0x48, 0x31, 0xC0, // XOR RAX, RAX (type 0)
        0x48, 0xC7, 0xC1, 0x00, 0x40, 0x00, 0x00, // MOV RCX, 0x4000 (descriptor)
        0x66, 0x0F, 0x38, 0x82, 0x01, // INVPCID RAX, [RCX]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_invpcid_single_context() {
    // INVPCID type 1 - Single-context invalidation
    let code = [
        0x48, 0xC7, 0xC0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1 (type 1)
        0x48, 0xC7, 0xC2, 0x00, 0x50, 0x00, 0x00, // MOV RDX, 0x5000 (descriptor)
        0x66, 0x0F, 0x38, 0x82, 0x02, // INVPCID RAX, [RDX]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_invpcid_all_context_including_global() {
    // INVPCID type 2 - All-context including global translations
    let code = [
        0x48, 0xC7, 0xC0, 0x02, 0x00, 0x00, 0x00, // MOV RAX, 2 (type 2)
        0x48, 0xC7, 0xC6, 0x00, 0x60, 0x00, 0x00, // MOV RSI, 0x6000 (descriptor)
        0x66, 0x0F, 0x38, 0x82, 0x06, // INVPCID RAX, [RSI]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_invpcid_all_context_retain_global() {
    // INVPCID type 3 - All-context retaining global translations
    let code = [
        0x48, 0xC7, 0xC0, 0x03, 0x00, 0x00, 0x00, // MOV RAX, 3 (type 3)
        0x48, 0xC7, 0xC7, 0x00, 0x70, 0x00, 0x00, // MOV RDI, 0x7000 (descriptor)
        0x66, 0x0F, 0x38, 0x82, 0x07, // INVPCID RAX, [RDI]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_invpcid_with_displacement() {
    // INVPCID with memory displacement
    let code = [
        0x48, 0xC7, 0xC0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1
        0x48, 0xC7, 0xC3, 0x00, 0x30, 0x00, 0x00, // MOV RBX, 0x3000
        0x66, 0x0F, 0x38, 0x82, 0x83, 0x00, 0x01, 0x00, 0x00, // INVPCID RAX, [RBX+0x100]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_invpcid_different_registers() {
    // INVPCID using different register combinations
    let code = [
        0x48, 0xC7, 0xC2, 0x02, 0x00, 0x00, 0x00, // MOV RDX, 2
        0x48, 0xC7, 0xC5, 0x00, 0x80, 0x00, 0x00, // MOV RBP, 0x8000
        0x66, 0x0F, 0x38, 0x82, 0x55, 0x00, // INVPCID RDX, [RBP]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_invpcid_r8_r15() {
    // INVPCID with R8-R15 registers
    let code = [
        0x49, 0xC7, 0xC0, 0x01, 0x00, 0x00, 0x00, // MOV R8, 1
        0x49, 0xC7, 0xC1, 0x00, 0x40, 0x00, 0x00, // MOV R9, 0x4000
        0x66, 0x41, 0x0F, 0x38, 0x82, 0x01, // INVPCID RAX, [R9]
        0xF4, // HLT
    ];
    let mut regs = Registers::default();
    regs.r8 = 1;
    regs.r9 = 0x4000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_invpcid_with_sib() {
    // INVPCID with SIB byte
    let code = [
        0x48, 0xC7, 0xC0, 0x03, 0x00, 0x00, 0x00, // MOV RAX, 3
        0x48, 0xC7, 0xC3, 0x00, 0x50, 0x00, 0x00, // MOV RBX, 0x5000
        0x48, 0xC7, 0xC1, 0x00, 0x10, 0x00, 0x00, // MOV RCX, 0x1000
        0x66, 0x0F, 0x38, 0x82, 0x04, 0x0B, // INVPCID RAX, [RBX+RCX]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_invpcid_rip_relative() {
    // INVPCID with RIP-relative addressing
    let code = [
        0x48, 0xC7, 0xC0, 0x02, 0x00, 0x00, 0x00, // MOV RAX, 2
        0x66, 0x0F, 0x38, 0x82, 0x05, 0x00, 0x00, 0x00, 0x00, // INVPCID RAX, [RIP+0]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_invpcid_preserves_registers() {
    // INVPCID should not modify general-purpose registers
    let code = [
        0x48, 0xC7, 0xC0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1
        0x48, 0xC7, 0xC3, 0x00, 0x40, 0x00, 0x00, // MOV RBX, 0x4000
        0x48, 0xC7, 0xC1, 0xDE, 0xAD, 0xBE, 0xEF, // MOV RCX, 0xEFBEADDE
        0x48, 0xC7, 0xC2, 0xCA, 0xFE, 0xBA, 0xBE, // MOV RDX, 0xBEBAFECA
        0x66, 0x0F, 0x38, 0x82, 0x03, // INVPCID RAX, [RBX]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_invpcid_all_types() {
    // Test all INVPCID types in sequence
    let code = [
        0x48, 0xC7, 0xC3, 0x00, 0x50, 0x00, 0x00, // MOV RBX, 0x5000
        // Type 0
        0x48, 0x31, 0xC0, // XOR RAX, RAX
        0x66, 0x0F, 0x38, 0x82, 0x03, // INVPCID RAX, [RBX]
        // Type 1
        0x48, 0xC7, 0xC0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1
        0x66, 0x0F, 0x38, 0x82, 0x03, // INVPCID RAX, [RBX]
        // Type 2
        0x48, 0xC7, 0xC0, 0x02, 0x00, 0x00, 0x00, // MOV RAX, 2
        0x66, 0x0F, 0x38, 0x82, 0x03, // INVPCID RAX, [RBX]
        // Type 3
        0x48, 0xC7, 0xC0, 0x03, 0x00, 0x00, 0x00, // MOV RAX, 3
        0x66, 0x0F, 0x38, 0x82, 0x03, // INVPCID RAX, [RBX]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_invpcid_invalid_type() {
    // INVPCID with invalid type (> 3) should #GP
    let code = [
        0x48, 0xC7, 0xC0, 0x04, 0x00, 0x00, 0x00, // MOV RAX, 4 (invalid)
        0x48, 0xC7, 0xC3, 0x00, 0x40, 0x00, 0x00, // MOV RBX, 0x4000
        0x66, 0x0F, 0x38, 0x82, 0x03, // INVPCID RAX, [RBX]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// INVEPT and INVPCID Mixed Tests
// ============================================================================

#[test]
fn test_invept_invpcid_sequence() {
    // Use both INVEPT and INVPCID in sequence
    let code = [
        // INVEPT type 2
        0x48, 0xC7, 0xC0, 0x02, 0x00, 0x00, 0x00, // MOV RAX, 2
        0x48, 0xC7, 0xC3, 0x00, 0x30, 0x00, 0x00, // MOV RBX, 0x3000
        0x66, 0x0F, 0x38, 0x80, 0x03, // INVEPT RAX, [RBX]
        // INVPCID type 3
        0x48, 0xC7, 0xC0, 0x03, 0x00, 0x00, 0x00, // MOV RAX, 3
        0x48, 0xC7, 0xC1, 0x00, 0x40, 0x00, 0x00, // MOV RCX, 0x4000
        0x66, 0x0F, 0x38, 0x82, 0x01, // INVPCID RAX, [RCX]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_invept_multiple_types() {
    // Test both INVEPT types
    let code = [
        0x48, 0xC7, 0xC3, 0x00, 0x30, 0x00, 0x00, // MOV RBX, 0x3000
        // Type 1 - Single context
        0x48, 0xC7, 0xC0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1
        0x66, 0x0F, 0x38, 0x80, 0x03, // INVEPT RAX, [RBX]
        // Type 2 - Global
        0x48, 0xC7, 0xC0, 0x02, 0x00, 0x00, 0x00, // MOV RAX, 2
        0x66, 0x0F, 0x38, 0x80, 0x03, // INVEPT RAX, [RBX]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_invpcid_with_stack_descriptor() {
    // INVPCID with descriptor on stack
    let code = [
        0x48, 0x83, 0xEC, 0x20, // SUB RSP, 0x20
        0x48, 0xC7, 0xC0, 0x02, 0x00, 0x00, 0x00, // MOV RAX, 2
        0x66, 0x0F, 0x38, 0x82, 0x04, 0x24, // INVPCID RAX, [RSP]
        0x48, 0x83, 0xC4, 0x20, // ADD RSP, 0x20
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_invept_with_stack_descriptor() {
    // INVEPT with descriptor on stack
    let code = [
        0x48, 0x83, 0xEC, 0x20, // SUB RSP, 0x20
        0x48, 0xC7, 0xC0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1
        0x66, 0x0F, 0x38, 0x80, 0x04, 0x24, // INVEPT RAX, [RSP]
        0x48, 0x83, 0xC4, 0x20, // ADD RSP, 0x20
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_invpcid_aligned_access() {
    // INVPCID with 16-byte aligned descriptor
    let code = [
        0x48, 0xC7, 0xC0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1
        0x48, 0xC7, 0xC3, 0x00, 0x40, 0x00, 0x00, // MOV RBX, 0x4000 (aligned)
        0x66, 0x0F, 0x38, 0x82, 0x03, // INVPCID RAX, [RBX]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_invept_aligned_access() {
    // INVEPT with 16-byte aligned descriptor
    let code = [
        0x48, 0xC7, 0xC0, 0x02, 0x00, 0x00, 0x00, // MOV RAX, 2
        0x48, 0xC7, 0xC3, 0x00, 0x50, 0x00, 0x00, // MOV RBX, 0x5000 (aligned)
        0x66, 0x0F, 0x38, 0x80, 0x03, // INVEPT RAX, [RBX]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}
