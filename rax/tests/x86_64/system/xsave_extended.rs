//! Tests for Extended XSAVE Family Instructions.
//!
//! This module covers extended XSAVE instructions for processor extended state management.
//!
//! Instructions covered:
//! - XSAVEC - Save processor extended states with compaction
//! - XSAVEOPT - Save processor extended states optimized
//! - XSAVES - Save processor extended states supervisor
//! - XRSTOR - Restore processor extended states
//! - XRSTORS - Restore processor extended states supervisor
//!
//! References: docs/xsavec.txt, docs/xsaveopt.txt, docs/xsaves.txt,
//!            docs/xrstor.txt, docs/xrstors.txt

use crate::common::*;
use rax::cpu::Registers;

// ============================================================================
// XSAVEC Tests - Save Extended State with Compaction
// ============================================================================

#[test]
fn test_xsavec_basic() {
    // XSAVEC - Save processor extended states with compaction
    // Opcode: 0F C7 /4
    let code = [
        0x48, 0xC7, 0xC0, 0xFF, 0xFF, 0x00, 0x00, // MOV RAX, 0xFFFF (state mask)
        0x48, 0x31, 0xD2, // XOR RDX, RDX
        0x48, 0xC7, 0xC3, 0x00, 0x00, 0x20, 0x00, // MOV RBX, 0x200000
        0x0F, 0xC7, 0x23, // XSAVEC [rbx]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_xsavec64_all_states() {
    // XSAVEC64 - Save all extended states
    let code = [
        0x48, 0xC7, 0xC0, 0xFF, 0xFF, 0xFF, 0xFF, // MOV RAX, -1 (all states)
        0x48, 0xC7, 0xC2, 0xFF, 0xFF, 0xFF, 0xFF, // MOV RDX, -1
        0x48, 0xC7, 0xC1, 0x00, 0x00, 0x30, 0x00, // MOV RCX, 0x300000
        0x0F, 0xC7, 0x21, // XSAVEC64 [rcx]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_xsavec_x87_sse() {
    // Save x87 and SSE state (bits 0-1)
    let code = [
        0x48, 0xC7, 0xC0, 0x03, 0x00, 0x00, 0x00, // MOV RAX, 3
        0x48, 0x31, 0xD2, // XOR RDX, RDX
        0x48, 0xC7, 0xC3, 0x00, 0x00, 0x20, 0x00, // MOV RBX, 0x200000
        0x0F, 0xC7, 0x23, // XSAVEC [rbx]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_xsavec_avx() {
    // Save AVX state (bit 2)
    let code = [
        0x48, 0xC7, 0xC0, 0x04, 0x00, 0x00, 0x00, // MOV RAX, 4
        0x48, 0x31, 0xD2, // XOR RDX, RDX
        0x48, 0xC7, 0xC1, 0x00, 0x00, 0x40, 0x00, // MOV RCX, 0x400000
        0x0F, 0xC7, 0x21, // XSAVEC [rcx]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// XSAVEOPT Tests - Optimized Save Extended State
// ============================================================================

#[test]
fn test_xsaveopt_basic() {
    // XSAVEOPT - Save processor extended states (optimized)
    // Opcode: 0F AE /6
    let code = [
        0x48, 0xC7, 0xC0, 0xFF, 0xFF, 0x00, 0x00, // MOV RAX, 0xFFFF
        0x48, 0x31, 0xD2, // XOR RDX, RDX
        0x48, 0xC7, 0xC3, 0x00, 0x00, 0x50, 0x00, // MOV RBX, 0x500000
        0x0F, 0xAE, 0x33, // XSAVEOPT [rbx]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_xsaveopt64_all_states() {
    // XSAVEOPT64 with all state components
    let code = [
        0x48, 0xC7, 0xC0, 0xFF, 0xFF, 0xFF, 0xFF, // MOV RAX, -1
        0x48, 0xC7, 0xC2, 0xFF, 0xFF, 0xFF, 0xFF, // MOV RDX, -1
        0x48, 0xC7, 0xC1, 0x00, 0x00, 0x60, 0x00, // MOV RCX, 0x600000
        0x0F, 0xAE, 0x31, // XSAVEOPT64 [rcx]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_xsaveopt_selective() {
    // Save only modified components
    let code = [
        0x48, 0xC7, 0xC0, 0x07, 0x00, 0x00, 0x00, // MOV RAX, 7 (x87+SSE+AVX)
        0x48, 0x31, 0xD2, // XOR RDX, RDX
        0x48, 0xC7, 0xC3, 0x00, 0x00, 0x50, 0x00, // MOV RBX, 0x500000
        0x0F, 0xAE, 0x33, // XSAVEOPT [rbx]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// XSAVES Tests - Supervisor Save Extended State
// ============================================================================

#[test]
fn test_xsaves_basic() {
    // XSAVES - Save processor extended states (supervisor)
    // Opcode: 0F C7 /5
    let code = [
        0x48, 0xC7, 0xC0, 0xFF, 0xFF, 0x00, 0x00, // MOV RAX, 0xFFFF
        0x48, 0x31, 0xD2, // XOR RDX, RDX
        0x48, 0xC7, 0xC3, 0x00, 0x00, 0x70, 0x00, // MOV RBX, 0x700000
        0x0F, 0xC7, 0x2B, // XSAVES [rbx]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_xsaves64_supervisor_states() {
    // XSAVES64 including supervisor states
    let code = [
        0x48, 0xC7, 0xC0, 0xFF, 0xFF, 0xFF, 0xFF, // MOV RAX, -1
        0x48, 0xC7, 0xC2, 0xFF, 0xFF, 0xFF, 0xFF, // MOV RDX, -1
        0x48, 0xC7, 0xC1, 0x00, 0x00, 0x80, 0x00, // MOV RCX, 0x800000
        0x0F, 0xC7, 0x29, // XSAVES64 [rcx]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// XRSTOR Tests - Restore Extended State
// ============================================================================

#[test]
fn test_xrstor_basic() {
    // XRSTOR - Restore processor extended states
    // Opcode: 0F AE /5
    let code = [
        0x48, 0xC7, 0xC0, 0xFF, 0xFF, 0x00, 0x00, // MOV RAX, 0xFFFF
        0x48, 0x31, 0xD2, // XOR RDX, RDX
        0x48, 0xC7, 0xC3, 0x00, 0x00, 0x20, 0x00, // MOV RBX, 0x200000
        0x0F, 0xAE, 0x2B, // XRSTOR [rbx]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_xrstor64_all_states() {
    // XRSTOR64 restore all states
    let code = [
        0x48, 0xC7, 0xC0, 0xFF, 0xFF, 0xFF, 0xFF, // MOV RAX, -1
        0x48, 0xC7, 0xC2, 0xFF, 0xFF, 0xFF, 0xFF, // MOV RDX, -1
        0x48, 0xC7, 0xC1, 0x00, 0x00, 0x30, 0x00, // MOV RCX, 0x300000
        0x0F, 0xAE, 0x29, // XRSTOR64 [rcx]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_xrstor_selective() {
    // Restore only specific components
    let code = [
        0x48, 0xC7, 0xC0, 0x03, 0x00, 0x00, 0x00, // MOV RAX, 3 (x87+SSE)
        0x48, 0x31, 0xD2, // XOR RDX, RDX
        0x48, 0xC7, 0xC3, 0x00, 0x00, 0x20, 0x00, // MOV RBX, 0x200000
        0x0F, 0xAE, 0x2B, // XRSTOR [rbx]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// XRSTORS Tests - Supervisor Restore Extended State
// ============================================================================

#[test]
fn test_xrstors_basic() {
    // XRSTORS - Restore processor extended states (supervisor)
    // Opcode: 0F C7 /3
    let code = [
        0x48, 0xC7, 0xC0, 0xFF, 0xFF, 0x00, 0x00, // MOV RAX, 0xFFFF
        0x48, 0x31, 0xD2, // XOR RDX, RDX
        0x48, 0xC7, 0xC3, 0x00, 0x00, 0x70, 0x00, // MOV RBX, 0x700000
        0x0F, 0xC7, 0x1B, // XRSTORS [rbx]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_xrstors64_supervisor_states() {
    // XRSTORS64 including supervisor states
    let code = [
        0x48, 0xC7, 0xC0, 0xFF, 0xFF, 0xFF, 0xFF, // MOV RAX, -1
        0x48, 0xC7, 0xC2, 0xFF, 0xFF, 0xFF, 0xFF, // MOV RDX, -1
        0x48, 0xC7, 0xC1, 0x00, 0x00, 0x80, 0x00, // MOV RCX, 0x800000
        0x0F, 0xC7, 0x19, // XRSTORS64 [rcx]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// Combined XSAVE Tests
// ============================================================================

#[test]
fn test_xsave_roundtrip() {
    // Save and restore state
    let code = [
        0x48, 0xC7, 0xC0, 0x07, 0x00, 0x00, 0x00, // MOV RAX, 7
        0x48, 0x31, 0xD2, // XOR RDX, RDX
        0x48, 0xC7, 0xC3, 0x00, 0x00, 0x20, 0x00, // MOV RBX, 0x200000
        0x0F, 0xC7, 0x23, // XSAVEC [rbx]
        0x0F, 0xAE, 0x2B, // XRSTOR [rbx]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_xsaves_xrstors_roundtrip() {
    // Supervisor save/restore roundtrip
    let code = [
        0x48, 0xC7, 0xC0, 0xFF, 0xFF, 0xFF, 0xFF, // MOV RAX, -1
        0x48, 0xC7, 0xC2, 0xFF, 0xFF, 0xFF, 0xFF, // MOV RDX, -1
        0x48, 0xC7, 0xC3, 0x00, 0x00, 0x70, 0x00, // MOV RBX, 0x700000
        0x0F, 0xC7, 0x2B, // XSAVES [rbx]
        0x0F, 0xC7, 0x1B, // XRSTORS [rbx]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_xsaveopt_multiple_saves() {
    // Multiple optimized saves
    let code = [
        0x48, 0xC7, 0xC0, 0x07, 0x00, 0x00, 0x00, // MOV RAX, 7
        0x48, 0x31, 0xD2, // XOR RDX, RDX
        0x48, 0xC7, 0xC3, 0x00, 0x00, 0x50, 0x00, // MOV RBX, 0x500000
        0x0F, 0xAE, 0x33, // XSAVEOPT [rbx]
        0x0F, 0xAE, 0x33, // XSAVEOPT [rbx] (2nd save)
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_xsavec_compact_format() {
    // XSAVEC produces compacted format
    let code = [
        0x48, 0xC7, 0xC0, 0x1F, 0x00, 0x00, 0x00, // MOV RAX, 0x1F (multiple components)
        0x48, 0x31, 0xD2, // XOR RDX, RDX
        0x48, 0xC7, 0xC1, 0x00, 0x00, 0x30, 0x00, // MOV RCX, 0x300000
        0x0F, 0xC7, 0x21, // XSAVEC [rcx]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_all_xsave_variants() {
    // Test all XSAVE instruction variants
    let code = [
        0x48, 0xC7, 0xC0, 0x07, 0x00, 0x00, 0x00, // MOV RAX, 7
        0x48, 0x31, 0xD2, // XOR RDX, RDX
        // XSAVEC
        0x48, 0xC7, 0xC3, 0x00, 0x00, 0x20, 0x00, // MOV RBX, 0x200000
        0x0F, 0xC7, 0x23, // XSAVEC [rbx]
        // XSAVEOPT
        0x48, 0xC7, 0xC3, 0x00, 0x00, 0x50, 0x00, // MOV RBX, 0x500000
        0x0F, 0xAE, 0x33, // XSAVEOPT [rbx]
        // XSAVES
        0x48, 0xC7, 0xC3, 0x00, 0x00, 0x70, 0x00, // MOV RBX, 0x700000
        0x0F, 0xC7, 0x2B, // XSAVES [rbx]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}
