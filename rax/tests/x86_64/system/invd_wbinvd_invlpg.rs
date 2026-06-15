//! Tests for cache and TLB invalidation instructions.
//!
//! INVD - Invalidate Internal Caches
//! WBINVD - Write Back and Invalidate Cache
//! INVLPG - Invalidate TLB Entries
//!
//! These are privileged instructions that manage processor caches and TLBs.
//!
//! Opcodes:
//!   0F 08    - INVD
//!   0F 09    - WBINVD
//!   0F 01 /7 - INVLPG m
//!
//! Flags affected: None
//!
//! Reference: docs/invd.txt, docs/wbinvd.txt, docs/invlpg.txt

use crate::common::*;
use rax::cpu::Registers;

// ============================================================================
// INVD Tests - Invalidate Internal Caches
// ============================================================================

#[test]
fn test_invd_basic() {
    // INVD - Flush internal caches
    // 0F 08 = INVD
    let code = [0x0F, 0x08, 0xF4];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    // INVD should complete without error
    // All registers should be preserved
    assert_eq!(regs.rax, 0);
}

#[test]
fn test_invd_preserves_registers() {
    // INVD should not modify any registers
    let code = [
        0x48, 0xC7, 0xC0, 0x11, 0x11, 0x11, 0x11, // MOV RAX, 0x11111111
        0x48, 0xC7, 0xC3, 0x22, 0x22, 0x22, 0x22, // MOV RBX, 0x22222222
        0x48, 0xC7, 0xC1, 0x33, 0x33, 0x33, 0x33, // MOV RCX, 0x33333333
        0x48, 0xC7, 0xC2, 0x44, 0x44, 0x44, 0x44, // MOV RDX, 0x44444444
        0x0F, 0x08, // INVD
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x11111111);
    assert_eq!(regs.rbx, 0x22222222);
    assert_eq!(regs.rcx, 0x33333333);
    assert_eq!(regs.rdx, 0x44444444);
}

#[test]
fn test_invd_preserves_flags() {
    // INVD should not modify flags
    let code = [
        0x48, 0xC7, 0xC0, 0xFF, 0xFF, 0xFF, 0xFF, // MOV RAX, -1
        0x48, 0x83, 0xC0, 0x01, // ADD RAX, 1 (sets ZF)
        0x0F, 0x08, // INVD
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(regs.rflags & 0x40 != 0, "ZF should be preserved");
}

#[test]
fn test_invd_multiple() {
    // Multiple INVD instructions
    let code = [
        0x0F, 0x08, // INVD #1
        0x0F, 0x08, // INVD #2
        0x0F, 0x08, // INVD #3
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let _regs = run_until_hlt(&mut vcpu).unwrap();
    // All INVD operations should complete
}

#[test]
fn test_invd_with_memory_operations() {
    // INVD after memory writes
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x50, 0x00, 0x00, // MOV RAX, 0x5000
        0xC7, 0x00, 0x42, 0x42, 0x42, 0x42, // MOV DWORD PTR [RAX], 0x42424242
        0x0F, 0x08, // INVD
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// WBINVD Tests - Write Back and Invalidate Cache
// ============================================================================

#[test]
fn test_wbinvd_basic() {
    // WBINVD - Write back and flush caches
    // 0F 09 = WBINVD
    let code = [0x0F, 0x09, 0xF4];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    // WBINVD should complete without error
    assert_eq!(regs.rax, 0);
}

#[test]
fn test_wbinvd_preserves_registers() {
    // WBINVD should not modify any registers
    let code = [
        0x48, 0xC7, 0xC0, 0xAA, 0xAA, 0xAA, 0xAA, // MOV RAX, 0xAAAAAAAA
        0x48, 0xC7, 0xC3, 0xBB, 0xBB, 0xBB, 0xBB, // MOV RBX, 0xBBBBBBBB
        0x48, 0xC7, 0xC1, 0xCC, 0xCC, 0xCC, 0xCC, // MOV RCX, 0xCCCCCCCC
        0x48, 0xC7, 0xC2, 0xDD, 0xDD, 0xDD, 0xDD, // MOV RDX, 0xDDDDDDDD
        0x0F, 0x09, // WBINVD
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    // MOV r64, imm32 sign-extends when bit 31 is set
    assert_eq!(regs.rax, 0xFFFFFFFFAAAAAAAAu64);
    assert_eq!(regs.rbx, 0xFFFFFFFFBBBBBBBBu64);
    assert_eq!(regs.rcx, 0xFFFFFFFFCCCCCCCCu64);
    assert_eq!(regs.rdx, 0xFFFFFFFFDDDDDDDDu64);
}

#[test]
fn test_wbinvd_preserves_flags() {
    // WBINVD should not modify flags
    let code = [
        0x48, 0xC7, 0xC0, 0xFF, 0xFF, 0xFF, 0xFF, // MOV RAX, -1
        0x48, 0x83, 0xC0, 0x01, // ADD RAX, 1 (sets ZF)
        0x0F, 0x09, // WBINVD
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(regs.rflags & 0x40 != 0, "ZF should be preserved");
}

#[test]
fn test_wbinvd_is_serializing() {
    // WBINVD is a serializing instruction
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x50, 0x00, 0x00, // MOV RAX, 0x5000
        0xC7, 0x00, 0x11, 0x22, 0x33, 0x44, // MOV DWORD PTR [RAX], 0x44332211
        0x0F, 0x09, // WBINVD (serializes)
        0x8B, 0x08, // MOV ECX, [RAX]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx & 0xFFFFFFFF, 0x44332211);
}

#[test]
fn test_wbinvd_multiple() {
    // Multiple WBINVD instructions
    let code = [
        0x0F, 0x09, // WBINVD #1
        0x0F, 0x09, // WBINVD #2
        0x0F, 0x09, // WBINVD #3
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_wbinvd_with_writes() {
    // WBINVD after multiple memory writes
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x50, 0x00, 0x00, // MOV RAX, 0x5000
        0xC7, 0x00, 0x01, 0x00, 0x00, 0x00, // MOV DWORD PTR [RAX], 1
        0xC7, 0x40, 0x04, 0x02, 0x00, 0x00, 0x00, // MOV DWORD PTR [RAX+4], 2
        0xC7, 0x40, 0x08, 0x03, 0x00, 0x00, 0x00, // MOV DWORD PTR [RAX+8], 3
        0x0F, 0x09, // WBINVD
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// INVD vs WBINVD Comparison
// ============================================================================

#[test]
fn test_invd_then_wbinvd() {
    // INVD followed by WBINVD
    let code = [
        0x0F, 0x08, // INVD
        0x0F, 0x09, // WBINVD
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_wbinvd_then_invd() {
    // WBINVD followed by INVD
    let code = [
        0x0F, 0x09, // WBINVD
        0x0F, 0x08, // INVD
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// INVLPG Tests - Invalidate TLB Entries
// ============================================================================

#[test]
fn test_invlpg_basic() {
    // INVLPG - Invalidate TLB entry for a page
    // 0F 01 /7 = INVLPG m
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x40, 0x00, 0x00, // MOV RAX, 0x4000
        0x0F, 0x01, 0x38, // INVLPG [RAX]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_invlpg_preserves_registers() {
    // INVLPG should not modify registers
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x40, 0x00, 0x00, // MOV RAX, 0x4000
        0x48, 0xC7, 0xC3, 0x42, 0x42, 0x42, 0x42, // MOV RBX, 0x42424242
        0x48, 0xC7, 0xC1, 0x99, 0x99, 0x99, 0x99, // MOV RCX, 0x99999999
        0x0F, 0x01, 0x38, // INVLPG [RAX]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x4000);
    assert_eq!(regs.rbx, 0x42424242);
    // MOV r64, imm32 sign-extends when bit 31 is set
    assert_eq!(regs.rcx, 0xFFFFFFFF99999999u64);
}

#[test]
fn test_invlpg_preserves_flags() {
    // INVLPG should not modify flags
    let code = [
        0x48, 0xC7, 0xC0, 0xFF, 0xFF, 0xFF, 0xFF, // MOV RAX, -1
        0x48, 0x83, 0xC0, 0x01, // ADD RAX, 1 (sets ZF)
        0x48, 0xC7, 0xC3, 0x00, 0x50, 0x00, 0x00, // MOV RBX, 0x5000
        0x0F, 0x01, 0x3B, // INVLPG [RBX]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(regs.rflags & 0x40 != 0, "ZF should be preserved");
}

#[test]
fn test_invlpg_different_addresses() {
    // Invalidate TLB entries for different pages
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x40, 0x00, 0x00, // MOV RAX, 0x4000
        0x0F, 0x01, 0x38, // INVLPG [RAX]
        0x48, 0xC7, 0xC0, 0x00, 0x50, 0x00, 0x00, // MOV RAX, 0x5000
        0x0F, 0x01, 0x38, // INVLPG [RAX]
        0x48, 0xC7, 0xC0, 0x00, 0x60, 0x00, 0x00, // MOV RAX, 0x6000
        0x0F, 0x01, 0x38, // INVLPG [RAX]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_invlpg_page_boundaries() {
    // Test INVLPG at page boundaries (4KB pages)
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x10, 0x00, 0x00, // MOV RAX, 0x1000
        0x0F, 0x01, 0x38, // INVLPG [RAX]
        0x48, 0xC7, 0xC0, 0x00, 0x20, 0x00, 0x00, // MOV RAX, 0x2000
        0x0F, 0x01, 0x38, // INVLPG [RAX]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_invlpg_with_offset() {
    // INVLPG with base+offset addressing
    let code = [
        0x48, 0xC7, 0xC3, 0x00, 0x40, 0x00, 0x00, // MOV RBX, 0x4000
        0x0F, 0x01, 0x7B, 0x10, // INVLPG [RBX+0x10]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx, 0x4000);
}

#[test]
fn test_invlpg_loop() {
    // Invalidate multiple pages in a loop
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x10, 0x00, 0x00, // MOV RAX, 0x1000
        0x48, 0xC7, 0xC1, 0x05, 0x00, 0x00, 0x00, // MOV RCX, 5
        // loop:
        0x0F, 0x01, 0x38, // INVLPG [RAX]
        0x48, 0x05, 0x00, 0x10, 0x00, 0x00, // ADD RAX, 0x1000 (next page)
        0x48, 0xFF, 0xC9, // DEC RCX
        0x75, 0xF3, // JNZ loop
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 0);
    assert_eq!(regs.rax, 0x6000); // 0x1000 + 5*0x1000
}

#[test]
fn test_invlpg_after_memory_access() {
    // Access memory, then invalidate TLB
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x50, 0x00, 0x00, // MOV RAX, 0x5000
        0xC7, 0x00, 0x42, 0x42, 0x42, 0x42, // MOV DWORD PTR [RAX], 0x42424242
        0x8B, 0x08, // MOV ECX, [RAX]
        0x0F, 0x01, 0x38, // INVLPG [RAX]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx & 0xFFFFFFFF, 0x42424242);
}

// ============================================================================
// Combined Cache/TLB Operations
// ============================================================================

#[test]
fn test_invlpg_then_wbinvd() {
    // INVLPG followed by WBINVD
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x40, 0x00, 0x00, // MOV RAX, 0x4000
        0x0F, 0x01, 0x38, // INVLPG [RAX]
        0x0F, 0x09, // WBINVD
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_wbinvd_then_invlpg() {
    // WBINVD followed by INVLPG
    let code = [
        0x0F, 0x09, // WBINVD
        0x48, 0xC7, 0xC0, 0x00, 0x40, 0x00, 0x00, // MOV RAX, 0x4000
        0x0F, 0x01, 0x38, // INVLPG [RAX]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cache_tlb_sequence() {
    // Comprehensive cache and TLB management sequence
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x50, 0x00, 0x00, // MOV RAX, 0x5000
        0xC7, 0x00, 0x11, 0x22, 0x33, 0x44, // MOV DWORD PTR [RAX], 0x44332211
        0x0F, 0x01, 0x38, // INVLPG [RAX]
        0x0F, 0x09, // WBINVD
        0x8B, 0x08, // MOV ECX, [RAX]
        0x0F, 0x08, // INVD
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx & 0xFFFFFFFF, 0x44332211);
}

#[test]
fn test_invlpg_multiple_same_page() {
    // Multiple INVLPG to same page
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x40, 0x00, 0x00, // MOV RAX, 0x4000
        0x0F, 0x01, 0x38, // INVLPG [RAX]
        0x0F, 0x01, 0x38, // INVLPG [RAX] again
        0x0F, 0x01, 0x38, // INVLPG [RAX] again
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_all_cache_instructions() {
    // Use all three cache/TLB instructions
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x40, 0x00, 0x00, // MOV RAX, 0x4000
        0x0F, 0x08, // INVD
        0x0F, 0x01, 0x38, // INVLPG [RAX]
        0x0F, 0x09, // WBINVD
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let _regs = run_until_hlt(&mut vcpu).unwrap();
}
