//! Tests for memory fence instructions.
//!
//! LFENCE - Load Fence
//! MFENCE - Memory Fence
//! SFENCE - Store Fence
//!
//! These instructions provide memory ordering and serialization guarantees.
//!
//! Opcodes:
//!   0F AE E8 - LFENCE
//!   0F AE F0 - MFENCE
//!   0F AE F8 - SFENCE
//!
//! Flags affected: None
//!
//! Reference: docs/lfence.txt, docs/mfence.txt, docs/sfence.txt

use crate::common::*;
use rax::cpu::Registers;

// ============================================================================
// LFENCE Tests - Load Fence
// ============================================================================

#[test]
fn test_lfence_basic() {
    // LFENCE - Load fence (serializes loads)
    // 0F AE E8 = LFENCE
    let code = [0x0F, 0xAE, 0xE8, 0xF4];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    // LFENCE should complete without error
    assert_eq!(regs.rax, 0);
}

#[test]
fn test_lfence_preserves_registers() {
    // LFENCE should not modify any registers
    let code = [
        0x48, 0xC7, 0xC0, 0x11, 0x11, 0x11, 0x11, // MOV RAX, 0x11111111
        0x48, 0xC7, 0xC3, 0x22, 0x22, 0x22, 0x22, // MOV RBX, 0x22222222
        0x48, 0xC7, 0xC1, 0x33, 0x33, 0x33, 0x33, // MOV RCX, 0x33333333
        0x48, 0xC7, 0xC2, 0x44, 0x44, 0x44, 0x44, // MOV RDX, 0x44444444
        0x0F, 0xAE, 0xE8, // LFENCE
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
fn test_lfence_preserves_flags() {
    // LFENCE should not modify flags
    let code = [
        0x48, 0xC7, 0xC0, 0xFF, 0xFF, 0xFF, 0xFF, // MOV RAX, -1
        0x48, 0x83, 0xC0, 0x01, // ADD RAX, 1 (sets ZF)
        0x0F, 0xAE, 0xE8, // LFENCE
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(regs.rflags & 0x40 != 0, "ZF should be preserved");
}

#[test]
fn test_lfence_after_load() {
    // LFENCE after memory load
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x50, 0x00, 0x00, // MOV RAX, 0x5000
        0xC7, 0x00, 0x42, 0x42, 0x42, 0x42, // MOV DWORD PTR [RAX], 0x42424242
        0x8B, 0x08, // MOV ECX, [RAX]
        0x0F, 0xAE, 0xE8, // LFENCE (serializes load)
        0x8B, 0x10, // MOV EDX, [RAX]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx & 0xFFFFFFFF, 0x42424242);
    assert_eq!(regs.rdx & 0xFFFFFFFF, 0x42424242);
}

#[test]
fn test_lfence_multiple() {
    // Multiple LFENCE instructions
    let code = [
        0x0F, 0xAE, 0xE8, // LFENCE #1
        0x0F, 0xAE, 0xE8, // LFENCE #2
        0x0F, 0xAE, 0xE8, // LFENCE #3
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_lfence_with_loop() {
    // LFENCE in a loop
    // MOV at 0x1000, 7 bytes. Loop starts at 0x1007.
    // LFENCE at 0x1007, 3 bytes. DEC at 0x100A, 3 bytes. JNZ at 0x100D, 2 bytes.
    // After JNZ, RIP = 0x100F. To jump to 0x1007: offset = 0x1007 - 0x100F = -8 = 0xF8
    let code = [
        0x48, 0xC7, 0xC1, 0x03, 0x00, 0x00, 0x00, // MOV RCX, 3
        // loop:
        0x0F, 0xAE, 0xE8, // LFENCE
        0x48, 0xFF, 0xC9, // DEC RCX
        0x75, 0xF8, // JNZ loop (-8)
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 0);
}

// ============================================================================
// MFENCE Tests - Memory Fence
// ============================================================================

#[test]
fn test_mfence_basic() {
    // MFENCE - Memory fence (serializes loads and stores)
    // 0F AE F0 = MFENCE
    let code = [0x0F, 0xAE, 0xF0, 0xF4];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    // MFENCE should complete without error
    assert_eq!(regs.rax, 0);
}

#[test]
fn test_mfence_preserves_registers() {
    // MFENCE should not modify any registers
    // Note: MOV r64, imm32 sign-extends, so 0xAAAAAAAA becomes 0xFFFFFFFFAAAAAAAA
    let code = [
        0x48, 0xC7, 0xC0, 0xAA, 0xAA, 0xAA, 0xAA, // MOV RAX, 0xAAAAAAAA (sign-ext)
        0x48, 0xC7, 0xC3, 0xBB, 0xBB, 0xBB, 0xBB, // MOV RBX, 0xBBBBBBBB (sign-ext)
        0x48, 0xC7, 0xC1, 0xCC, 0xCC, 0xCC, 0xCC, // MOV RCX, 0xCCCCCCCC (sign-ext)
        0x48, 0xC7, 0xC2, 0xDD, 0xDD, 0xDD, 0xDD, // MOV RDX, 0xDDDDDDDD (sign-ext)
        0x0F, 0xAE, 0xF0, // MFENCE
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    // These all have bit 31 set, so they are sign-extended to 64 bits
    assert_eq!(regs.rax, 0xFFFFFFFFAAAAAAAAu64);
    assert_eq!(regs.rbx, 0xFFFFFFFFBBBBBBBBu64);
    assert_eq!(regs.rcx, 0xFFFFFFFFCCCCCCCCu64);
    assert_eq!(regs.rdx, 0xFFFFFFFFDDDDDDDDu64);
}

#[test]
fn test_mfence_preserves_flags() {
    // MFENCE should not modify flags
    let code = [
        0x48, 0xC7, 0xC0, 0xFF, 0xFF, 0xFF, 0xFF, // MOV RAX, -1
        0x48, 0x83, 0xC0, 0x01, // ADD RAX, 1 (sets ZF)
        0x0F, 0xAE, 0xF0, // MFENCE
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(regs.rflags & 0x40 != 0, "ZF should be preserved");
}

#[test]
fn test_mfence_between_store_load() {
    // MFENCE between store and load
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x50, 0x00, 0x00, // MOV RAX, 0x5000
        0xC7, 0x00, 0x11, 0x22, 0x33, 0x44, // MOV DWORD PTR [RAX], 0x44332211
        0x0F, 0xAE, 0xF0, // MFENCE (serializes)
        0x8B, 0x08, // MOV ECX, [RAX]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx & 0xFFFFFFFF, 0x44332211);
}

#[test]
fn test_mfence_multiple() {
    // Multiple MFENCE instructions
    let code = [
        0x0F, 0xAE, 0xF0, // MFENCE #1
        0x0F, 0xAE, 0xF0, // MFENCE #2
        0x0F, 0xAE, 0xF0, // MFENCE #3
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_mfence_with_multiple_stores() {
    // MFENCE after multiple stores
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x50, 0x00, 0x00, // MOV RAX, 0x5000
        0xC7, 0x00, 0x01, 0x00, 0x00, 0x00, // MOV DWORD PTR [RAX], 1
        0xC7, 0x40, 0x04, 0x02, 0x00, 0x00, 0x00, // MOV DWORD PTR [RAX+4], 2
        0xC7, 0x40, 0x08, 0x03, 0x00, 0x00, 0x00, // MOV DWORD PTR [RAX+8], 3
        0x0F, 0xAE, 0xF0, // MFENCE
        0x8B, 0x08, // MOV ECX, [RAX]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx & 0xFFFFFFFF, 1);
}

// ============================================================================
// SFENCE Tests - Store Fence
// ============================================================================

#[test]
fn test_sfence_basic() {
    // SFENCE - Store fence (serializes stores)
    // 0F AE F8 = SFENCE
    let code = [0x0F, 0xAE, 0xF8, 0xF4];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    // SFENCE should complete without error
    assert_eq!(regs.rax, 0);
}

#[test]
fn test_sfence_preserves_registers() {
    // SFENCE should not modify any registers
    // Note: MOV r64, imm32 sign-extends. Only 0x88888888 has bit 31 set.
    let code = [
        0x48, 0xC7, 0xC0, 0x55, 0x55, 0x55, 0x55, // MOV RAX, 0x55555555 (no sign-ext)
        0x48, 0xC7, 0xC3, 0x66, 0x66, 0x66, 0x66, // MOV RBX, 0x66666666 (no sign-ext)
        0x48, 0xC7, 0xC1, 0x77, 0x77, 0x77, 0x77, // MOV RCX, 0x77777777 (no sign-ext)
        0x48, 0xC7, 0xC2, 0x88, 0x88, 0x88, 0x88, // MOV RDX, 0x88888888 (sign-ext)
        0x0F, 0xAE, 0xF8, // SFENCE
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x55555555);
    assert_eq!(regs.rbx, 0x66666666);
    assert_eq!(regs.rcx, 0x77777777);
    assert_eq!(regs.rdx, 0xFFFFFFFF88888888u64); // sign-extended
}

#[test]
fn test_sfence_preserves_flags() {
    // SFENCE should not modify flags
    let code = [
        0x48, 0xC7, 0xC0, 0xFF, 0xFF, 0xFF, 0xFF, // MOV RAX, -1
        0x48, 0x83, 0xC0, 0x01, // ADD RAX, 1 (sets ZF)
        0x0F, 0xAE, 0xF8, // SFENCE
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(regs.rflags & 0x40 != 0, "ZF should be preserved");
}

#[test]
fn test_sfence_after_store() {
    // SFENCE after memory store
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x50, 0x00, 0x00, // MOV RAX, 0x5000
        0xC7, 0x00, 0x99, 0x99, 0x99, 0x99, // MOV DWORD PTR [RAX], 0x99999999
        0x0F, 0xAE, 0xF8, // SFENCE (ensures store completes)
        0x8B, 0x08, // MOV ECX, [RAX]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx & 0xFFFFFFFF, 0x99999999);
}

#[test]
fn test_sfence_multiple() {
    // Multiple SFENCE instructions
    let code = [
        0x0F, 0xAE, 0xF8, // SFENCE #1
        0x0F, 0xAE, 0xF8, // SFENCE #2
        0x0F, 0xAE, 0xF8, // SFENCE #3
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sfence_between_stores() {
    // SFENCE between stores
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x50, 0x00, 0x00, // MOV RAX, 0x5000
        0xC7, 0x00, 0x01, 0x00, 0x00, 0x00, // MOV DWORD PTR [RAX], 1
        0x0F, 0xAE, 0xF8, // SFENCE
        0xC7, 0x40, 0x04, 0x02, 0x00, 0x00, 0x00, // MOV DWORD PTR [RAX+4], 2
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Fence Combinations
// ============================================================================

#[test]
fn test_all_fences_sequence() {
    // Test all three fences in sequence
    let code = [
        0x0F, 0xAE, 0xE8, // LFENCE
        0x0F, 0xAE, 0xF0, // MFENCE
        0x0F, 0xAE, 0xF8, // SFENCE
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_lfence_mfence_combination() {
    // LFENCE followed by MFENCE
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x50, 0x00, 0x00, // MOV RAX, 0x5000
        0x8B, 0x08, // MOV ECX, [RAX]
        0x0F, 0xAE, 0xE8, // LFENCE
        0xC7, 0x00, 0x42, 0x42, 0x42, 0x42, // MOV DWORD PTR [RAX], 0x42424242
        0x0F, 0xAE, 0xF0, // MFENCE
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sfence_lfence_combination() {
    // SFENCE followed by LFENCE
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x50, 0x00, 0x00, // MOV RAX, 0x5000
        0xC7, 0x00, 0x11, 0x11, 0x11, 0x11, // MOV DWORD PTR [RAX], 0x11111111
        0x0F, 0xAE, 0xF8, // SFENCE
        0x8B, 0x08, // MOV ECX, [RAX]
        0x0F, 0xAE, 0xE8, // LFENCE
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx & 0xFFFFFFFF, 0x11111111);
}

#[test]
fn test_mfence_replaces_both() {
    // MFENCE provides both LFENCE and SFENCE guarantees
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x50, 0x00, 0x00, // MOV RAX, 0x5000
        0xC7, 0x00, 0x22, 0x22, 0x22, 0x22, // MOV DWORD PTR [RAX], 0x22222222
        0x0F, 0xAE, 0xF0, // MFENCE (replaces SFENCE+LFENCE)
        0x8B, 0x08, // MOV ECX, [RAX]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx & 0xFFFFFFFF, 0x22222222);
}

#[test]
fn test_fence_pattern_loads_stores() {
    // Complex pattern with loads, stores, and fences
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x50, 0x00, 0x00, // MOV RAX, 0x5000
        0xC7, 0x00, 0x01, 0x00, 0x00, 0x00, // MOV DWORD PTR [RAX], 1
        0x0F, 0xAE, 0xF8, // SFENCE
        0x8B, 0x08, // MOV ECX, [RAX]
        0x0F, 0xAE, 0xE8, // LFENCE
        0xC7, 0x40, 0x04, 0x02, 0x00, 0x00, 0x00, // MOV DWORD PTR [RAX+4], 2
        0x0F, 0xAE, 0xF0, // MFENCE
        0x8B, 0x50, 0x04, // MOV EDX, [RAX+4]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx & 0xFFFFFFFF, 1);
    assert_eq!(regs.rdx & 0xFFFFFFFF, 2);
}

#[test]
fn test_fence_in_loop() {
    // Fences in a loop with memory operations
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x50, 0x00, 0x00, // MOV RAX, 0x5000
        0x48, 0xC7, 0xC1, 0x03, 0x00, 0x00, 0x00, // MOV RCX, 3
        0x48, 0x31, 0xDB, // XOR RBX, RBX
        // loop:
        0x48, 0xFF, 0xC3, // INC RBX
        0x48, 0x89, 0x18, // MOV [RAX], RBX
        0x0F, 0xAE, 0xF8, // SFENCE
        0x48, 0x8B, 0x10, // MOV RDX, [RAX]
        0x0F, 0xAE, 0xE8, // LFENCE
        0x48, 0xFF, 0xC9, // DEC RCX
        0x75, 0xED, // JNZ loop
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 0);
    assert_eq!(regs.rbx, 3);
    assert_eq!(regs.rdx, 3);
}

#[test]
fn test_double_mfence() {
    // Back-to-back MFENCE instructions
    let code = [
        0x0F, 0xAE, 0xF0, // MFENCE
        0x0F, 0xAE, 0xF0, // MFENCE
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_all_fences_repeated() {
    // All three fences repeated multiple times
    let code = [
        0x0F, 0xAE, 0xE8, // LFENCE
        0x0F, 0xAE, 0xF0, // MFENCE
        0x0F, 0xAE, 0xF8, // SFENCE
        0x0F, 0xAE, 0xE8, // LFENCE
        0x0F, 0xAE, 0xF0, // MFENCE
        0x0F, 0xAE, 0xF8, // SFENCE
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let _regs = run_until_hlt(&mut vcpu).unwrap();
}
