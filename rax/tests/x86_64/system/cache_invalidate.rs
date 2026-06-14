//! Tests for Cache Invalidation and Writeback Instructions.
//!
//! This module covers system-level cache management instructions including
//! invalidation, write-back, and TLB management.
//!
//! Instructions covered:
//! - INVD - Invalidate Internal Caches
//! - WBINVD - Write Back and Invalidate Cache
//! - WBNOINVD - Write Back and Do Not Invalidate Cache
//! - INVLPG - Invalidate TLB Entry
//! - INVPCID - Invalidate Process-Context Identifier
//!
//! These are privileged instructions that manage cache and TLB state.
//!
//! References: Intel SDM Vol. 2, Cache and TLB management instructions

use crate::common::*;
use rax::cpu::Registers;

// ============================================================================
// INVD Tests - Invalidate Internal Caches
// ============================================================================

#[test]
fn test_invd_basic() {
    // INVD - Invalidate internal caches
    // 0F 08
    let code = [
        0x0F, 0x08, // INVD
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_invd_after_memory_operations() {
    // Test INVD after memory writes
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x10, 0x00, 0x00, // MOV RAX, 0x1000
        0x48, 0xC7, 0x00, 0x42, 0x00, 0x00, 0x00, // MOV QWORD PTR [RAX], 0x42
        0x0F, 0x08, // INVD
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_invd_multiple_sequential() {
    // Multiple sequential INVD instructions
    let code = [
        0x0F, 0x08, // INVD
        0x0F, 0x08, // INVD
        0x0F, 0x08, // INVD
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// WBINVD Tests - Write Back and Invalidate Cache
// ============================================================================

#[test]
fn test_wbinvd_basic() {
    // WBINVD - Write back and invalidate cache
    // 0F 09
    let code = [
        0x0F, 0x09, // WBINVD
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_wbinvd_after_memory_writes() {
    // Test WBINVD after memory modifications
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x20, 0x00, 0x00, // MOV RAX, 0x2000
        0x48, 0xC7, 0x00, 0xDE, 0xAD, 0xBE, 0xEF, // MOV QWORD PTR [RAX], 0xEFBEADDE
        0x48, 0xC7, 0x40, 0x08, 0xCA, 0xFE, 0xBA, 0xBE, // MOV QWORD PTR [RAX+8], 0xBEBAFECA
        0x0F, 0x09, // WBINVD
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_wbinvd_verify_writeback() {
    // Verify data is written back before invalidation
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x30, 0x00, 0x00, // MOV RAX, 0x3000
        0x48, 0xC7, 0x00, 0x11, 0x22, 0x33, 0x44, // MOV QWORD PTR [RAX], 0x44332211
        0x0F, 0x09, // WBINVD
        0x48, 0x8B, 0x08, // MOV RCX, [RAX]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_wbinvd_multiple_cache_lines() {
    // Test with multiple cache lines
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x40, 0x00, 0x00, // MOV RAX, 0x4000
        0xB9, 0x10, 0x00, 0x00, 0x00, // MOV ECX, 16
        // Loop: Write to multiple cache lines
        0x48, 0x89, 0x08, // MOV [RAX], RCX
        0x48, 0x83, 0xC0, 0x40, // ADD RAX, 64 (cache line size)
        0xFF, 0xC9, // DEC ECX
        0x75, 0xF6, // JNZ loop
        0x0F, 0x09, // WBINVD
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
    // WBNOINVD - Write back without invalidating cache
    // F3 0F 09
    let code = [
        0xF3, 0x0F, 0x09, // WBNOINVD
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_wbnoinvd_after_writes() {
    // Test WBNOINVD after memory modifications
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x50, 0x00, 0x00, // MOV RAX, 0x5000
        0x48, 0xC7, 0x00, 0x55, 0x55, 0x55, 0x55, // MOV QWORD PTR [RAX], 0x55555555
        0xF3, 0x0F, 0x09, // WBNOINVD
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_wbnoinvd_cache_remains_valid() {
    // Verify cache remains valid after WBNOINVD
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x60, 0x00, 0x00, // MOV RAX, 0x6000
        0x48, 0xC7, 0x00, 0xAA, 0xAA, 0xAA, 0xAA, // MOV QWORD PTR [RAX], 0xAAAAAAAA
        0xF3, 0x0F, 0x09, // WBNOINVD
        // Cache should still be valid, read should be fast
        0x48, 0x8B, 0x08, // MOV RCX, [RAX]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_wbnoinvd_vs_wbinvd() {
    // Compare WBNOINVD vs WBINVD behavior
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x70, 0x00, 0x00, // MOV RAX, 0x7000
        0x48, 0xC7, 0x00, 0x12, 0x34, 0x56, 0x78, // MOV QWORD PTR [RAX], 0x78563412
        0xF3, 0x0F, 0x09, // WBNOINVD (cache stays valid)
        0x48, 0x8B, 0x08, // MOV RCX, [RAX]
        0x48, 0xC7, 0x00, 0x87, 0x65, 0x43, 0x21, // MOV QWORD PTR [RAX], 0x21436587
        0x0F, 0x09, // WBINVD (cache invalidated)
        0x48, 0x8B, 0x10, // MOV RDX, [RAX]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// INVLPG Tests - Invalidate TLB Entry
// ============================================================================

#[test]
fn test_invlpg_basic() {
    // INVLPG - Invalidate TLB entry
    // 0F 01 /7
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x10, 0x00, 0x00, // MOV RAX, 0x1000
        0x0F, 0x01, 0x38, // INVLPG [RAX]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_invlpg_multiple_pages() {
    // Invalidate TLB entries for multiple pages
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x10, 0x00, 0x00, // MOV RAX, 0x1000
        0x0F, 0x01, 0x38, // INVLPG [RAX]
        0x48, 0x05, 0x00, 0x10, 0x00, 0x00, // ADD RAX, 0x1000
        0x0F, 0x01, 0x38, // INVLPG [RAX]
        0x48, 0x05, 0x00, 0x10, 0x00, 0x00, // ADD RAX, 0x1000
        0x0F, 0x01, 0x38, // INVLPG [RAX]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_invlpg_with_memory_access() {
    // Test INVLPG with memory access before and after
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x20, 0x00, 0x00, // MOV RAX, 0x2000
        0x48, 0xC7, 0x00, 0xAB, 0xCD, 0xEF, 0x01, // MOV QWORD PTR [RAX], 0x01EFCDAB
        0x48, 0x8B, 0x08, // MOV RCX, [RAX] (load into TLB)
        0x0F, 0x01, 0x38, // INVLPG [RAX] (invalidate TLB entry)
        0x48, 0x8B, 0x10, // MOV RDX, [RAX] (reload TLB entry)
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_invlpg_different_addressing() {
    // Test INVLPG with different addressing modes
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x30, 0x00, 0x00, // MOV RAX, 0x3000
        0x48, 0xC7, 0xC3, 0x00, 0x10, 0x00, 0x00, // MOV RBX, 0x1000
        0x0F, 0x01, 0x38, // INVLPG [RAX]
        0x0F, 0x01, 0x3C, 0x18, // INVLPG [RAX+RBX]
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
    // INVPCID - Invalidate PCID
    // 66 0F 38 82 /r
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x40, 0x00, 0x00, // MOV RAX, 0x4000
        0x48, 0xC7, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV QWORD PTR [RAX], 0 (PCID=0)
        0x48, 0xC7, 0x40, 0x08, 0x00, 0x10, 0x00,
        0x00, // MOV QWORD PTR [RAX+8], 0x1000 (address)
        0x31, 0xC9, // XOR ECX, ECX (type 0: individual address)
        0x66, 0x0F, 0x38, 0x82, 0x08, // INVPCID ECX, [RAX]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_invpcid_single_context() {
    // INVPCID - Invalidate single-context
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x50, 0x00, 0x00, // MOV RAX, 0x5000
        0x48, 0xC7, 0x00, 0x01, 0x00, 0x00, 0x00, // MOV QWORD PTR [RAX], 1 (PCID=1)
        0x48, 0xC7, 0x40, 0x08, 0x00, 0x00, 0x00, 0x00, // MOV QWORD PTR [RAX+8], 0
        0xB9, 0x01, 0x00, 0x00, 0x00, // MOV ECX, 1 (type 1: single context)
        0x66, 0x0F, 0x38, 0x82, 0x08, // INVPCID ECX, [RAX]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_invpcid_all_contexts() {
    // INVPCID - Invalidate all contexts
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x60, 0x00, 0x00, // MOV RAX, 0x6000
        0x48, 0xC7, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV QWORD PTR [RAX], 0
        0x48, 0xC7, 0x40, 0x08, 0x00, 0x00, 0x00, 0x00, // MOV QWORD PTR [RAX+8], 0
        0xB9, 0x02, 0x00, 0x00, 0x00, // MOV ECX, 2 (type 2: all contexts)
        0x66, 0x0F, 0x38, 0x82, 0x08, // INVPCID ECX, [RAX]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_invpcid_all_including_global() {
    // INVPCID - Invalidate all contexts including globals
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x70, 0x00, 0x00, // MOV RAX, 0x7000
        0x48, 0xC7, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV QWORD PTR [RAX], 0
        0x48, 0xC7, 0x40, 0x08, 0x00, 0x00, 0x00, 0x00, // MOV QWORD PTR [RAX+8], 0
        0xB9, 0x03, 0x00, 0x00, 0x00, // MOV ECX, 3 (type 3: all + global)
        0x66, 0x0F, 0x38, 0x82, 0x08, // INVPCID ECX, [RAX]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// Combined Tests
// ============================================================================

#[test]
fn test_cache_coherence_sequence() {
    // Test cache coherence operations in sequence
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x10, 0x00, 0x00, // MOV RAX, 0x1000
        0x48, 0xC7, 0x00, 0x42, 0x42, 0x42, 0x42, // MOV QWORD PTR [RAX], 0x42424242
        0x0F, 0x09, // WBINVD
        0x48, 0x8B, 0x08, // MOV RCX, [RAX]
        0x0F, 0x01, 0x38, // INVLPG [RAX]
        0x48, 0x8B, 0x10, // MOV RDX, [RAX]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_invd_vs_wbinvd() {
    // Compare INVD vs WBINVD behavior
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x20, 0x00, 0x00, // MOV RAX, 0x2000
        0x48, 0xC7, 0x00, 0x11, 0x11, 0x11, 0x11, // MOV QWORD PTR [RAX], 0x11111111
        0x0F, 0x08, // INVD (no writeback)
        0x48, 0xC7, 0x00, 0x22, 0x22, 0x22, 0x22, // MOV QWORD PTR [RAX], 0x22222222
        0x0F, 0x09, // WBINVD (writeback + invalidate)
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_wbnoinvd_performance() {
    // Test WBNOINVD for performance scenarios
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x30, 0x00, 0x00, // MOV RAX, 0x3000
        0xB9, 0x08, 0x00, 0x00, 0x00, // MOV ECX, 8
        // Write to cache
        0x48, 0x89, 0x08, // MOV [RAX], RCX
        0x48, 0x83, 0xC0, 0x08, // ADD RAX, 8
        0xFF, 0xC9, // DEC ECX
        0x75, 0xF6, // JNZ loop
        // Writeback without invalidating (faster)
        0xF3, 0x0F, 0x09, // WBNOINVD
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_tlb_and_cache_management() {
    // Combined TLB and cache management
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x40, 0x00, 0x00, // MOV RAX, 0x4000
        0x48, 0xC7, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, // MOV QWORD PTR [RAX], 0xFFFFFFFF
        0x0F, 0x01, 0x38, // INVLPG [RAX] (invalidate TLB)
        0x0F, 0x09, // WBINVD (writeback and invalidate cache)
        0x48, 0x8B, 0x08, // MOV RCX, [RAX] (reload both TLB and cache)
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}
