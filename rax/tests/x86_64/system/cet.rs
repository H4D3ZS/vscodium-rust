//! Tests for Intel CET (Control-flow Enforcement Technology) Instructions.
//!
//! This module covers CET instructions for shadow stack and indirect branch tracking.
//!
//! Instructions covered:
//! - CLRSSBSY - Clear Shadow Stack Busy Flag
//! - INCSSPD - Increment Shadow Stack Pointer by 4 bytes
//! - INCSSPQ - Increment Shadow Stack Pointer by 8 bytes
//! - RSTORSSP - Restore Saved Shadow Stack Pointer
//! - SAVEPREVSSP - Save Previous Shadow Stack Pointer
//! - SETSSBSY - Set Shadow Stack Busy Flag
//! - WRUSSD - Write to User Shadow Stack (32-bit)
//! - WRUSSQ - Write to User Shadow Stack (64-bit)
//! - CLAC - Clear AC Flag (Supervisor Mode Access Prevention)
//! - STAC - Set AC Flag (Supervisor Mode Access Prevention)
//!
//! References: docs/clrssbsy.txt, docs/incsspd:incsspq.txt, docs/rstorssp.txt,
//!            docs/saveprevssp.txt, docs/setssbsy.txt, docs/wrussd:wrussq.txt,
//!            docs/clac.txt, docs/stac.txt

use crate::common::*;
use rax::cpu::Registers;

// ============================================================================
// INCSSPD/INCSSPQ Tests - Increment Shadow Stack Pointer
// ============================================================================

#[test]
fn test_incsspd_basic() {
    // INCSSPD - Increment shadow stack pointer by (reg * 4) bytes
    // Opcode: F3 0F AE /5
    let code = [
        0x48, 0xC7, 0xC0, 0x04, 0x00, 0x00, 0x00, // MOV RAX, 4 (increment by 16 bytes)
        0xF3, 0x0F, 0xAE, 0xE8, // INCSSPD eax
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_incsspq_basic() {
    // INCSSPQ - Increment shadow stack pointer by (reg * 8) bytes
    // Opcode: F3 REX.W 0F AE /5
    let code = [
        0x48, 0xC7, 0xC1, 0x04, 0x00, 0x00, 0x00, // MOV RCX, 4 (increment by 32 bytes)
        0xF3, 0x48, 0x0F, 0xAE, 0xE9, // INCSSPQ rcx
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_incsspd_different_counts() {
    // Test different increment counts
    let code = [
        0x48, 0xC7, 0xC2, 0x01, 0x00, 0x00, 0x00, // MOV RDX, 1
        0xF3, 0x0F, 0xAE, 0xEA, // INCSSPD edx (4 bytes)
        0x48, 0xC7, 0xC3, 0x10, 0x00, 0x00, 0x00, // MOV RBX, 16
        0xF3, 0x0F, 0xAE, 0xEB, // INCSSPD ebx (64 bytes)
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_incsspq_different_registers() {
    // Test INCSSPQ with different registers
    let code = [
        0x48, 0xC7, 0xC0, 0x02, 0x00, 0x00, 0x00, // MOV RAX, 2
        0xF3, 0x48, 0x0F, 0xAE, 0xE8, // INCSSPQ rax
        0x48, 0xC7, 0xC6, 0x08, 0x00, 0x00, 0x00, // MOV RSI, 8
        0xF3, 0x48, 0x0F, 0xAE, 0xEE, // INCSSPQ rsi
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_incsspd_zero() {
    // Increment by zero (no-op)
    let code = [
        0x48, 0x31, 0xC0, // XOR RAX, RAX
        0xF3, 0x0F, 0xAE, 0xE8, // INCSSPD eax
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_incsspq_large_count() {
    // Large increment count
    let code = [
        0x48, 0xC7, 0xC7, 0xFF, 0x00, 0x00, 0x00, // MOV RDI, 255
        0xF3, 0x48, 0x0F, 0xAE, 0xEF, // INCSSPQ rdi (2040 bytes)
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_incsspd_sequential() {
    // Multiple INCSSPD operations
    let code = [
        0x48, 0xC7, 0xC0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1
        0xF3, 0x0F, 0xAE, 0xE8, // INCSSPD eax
        0xF3, 0x0F, 0xAE, 0xE8, // INCSSPD eax
        0xF3, 0x0F, 0xAE, 0xE8, // INCSSPD eax
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_incsspd_preserves_register() {
    // INCSSPD should not modify the register
    let code = [
        0x48, 0xC7, 0xC0, 0x42, 0x00, 0x00, 0x00, // MOV RAX, 0x42
        0xF3, 0x0F, 0xAE, 0xE8, // INCSSPD eax
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFFFFFFFF, 0x42, "EAX should not be modified");
}

// ============================================================================
// RSTORSSP Tests - Restore Saved Shadow Stack Pointer
// ============================================================================

#[test]
fn test_rstorssp_basic() {
    // RSTORSSP - Restore shadow stack pointer
    // Opcode: F3 0F 01 /5
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x00, 0x30, 0x00, // MOV RAX, 0x300000
        0xF3, 0x0F, 0x01, 0x28, // RSTORSSP [rax]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_rstorssp_different_addresses() {
    // Restore from different memory locations
    let code = [
        0x48, 0xC7, 0xC3, 0x00, 0x00, 0x40, 0x00, // MOV RBX, 0x400000
        0xF3, 0x0F, 0x01, 0x2B, // RSTORSSP [rbx]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_rstorssp_with_displacement() {
    // RSTORSSP with memory displacement
    let code = [
        0x48, 0xC7, 0xC1, 0x00, 0x00, 0x30, 0x00, // MOV RCX, 0x300000
        0xF3, 0x0F, 0x01, 0xA9, 0x00, 0x10, 0x00, 0x00, // RSTORSSP [rcx+0x1000]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_rstorssp_multiple() {
    // Multiple restore operations
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x00, 0x30, 0x00, // MOV RAX, 0x300000
        0xF3, 0x0F, 0x01, 0x28, // RSTORSSP [rax]
        0x48, 0xC7, 0xC0, 0x00, 0x00, 0x40, 0x00, // MOV RAX, 0x400000
        0xF3, 0x0F, 0x01, 0x28, // RSTORSSP [rax]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// SAVEPREVSSP Tests - Save Previous Shadow Stack Pointer
// ============================================================================

#[test]
fn test_saveprevssp_basic() {
    // SAVEPREVSSP - Save previous shadow stack pointer
    // Opcode: F3 0F 01 EA
    let code = [
        0xF3, 0x0F, 0x01, 0xEA, // SAVEPREVSSP
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_saveprevssp_no_operands() {
    // SAVEPREVSSP takes no operands
    let code = [0xF3, 0x0F, 0x01, 0xEA, 0xF4];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_saveprevssp_preserves_registers() {
    // SAVEPREVSSP should not modify GP registers
    let code = [
        0x48, 0xC7, 0xC0, 0x11, 0x11, 0x11, 0x11, // MOV RAX, 0x11111111
        0x48, 0xC7, 0xC3, 0x22, 0x22, 0x22, 0x22, // MOV RBX, 0x22222222
        0xF3, 0x0F, 0x01, 0xEA, // SAVEPREVSSP
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x11111111, "RAX should not be modified");
    assert_eq!(regs.rbx, 0x22222222, "RBX should not be modified");
}

#[test]
fn test_saveprevssp_multiple() {
    // Multiple SAVEPREVSSP operations
    let code = [
        0xF3, 0x0F, 0x01, 0xEA, // SAVEPREVSSP
        0xF3, 0x0F, 0x01, 0xEA, // SAVEPREVSSP
        0xF3, 0x0F, 0x01, 0xEA, // SAVEPREVSSP
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// CLRSSBSY Tests - Clear Shadow Stack Busy Flag
// ============================================================================

#[test]
fn test_clrssbsy_basic() {
    // CLRSSBSY - Clear shadow stack busy flag
    // Opcode: F3 0F AE /6
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x00, 0x50, 0x00, // MOV RAX, 0x500000
        0xF3, 0x0F, 0xAE, 0x30, // CLRSSBSY [rax]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_clrssbsy_different_addresses() {
    // Clear busy flag at different addresses
    let code = [
        0x48, 0xC7, 0xC3, 0x00, 0x00, 0x60, 0x00, // MOV RBX, 0x600000
        0xF3, 0x0F, 0xAE, 0x33, // CLRSSBSY [rbx]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_clrssbsy_with_displacement() {
    // CLRSSBSY with memory displacement
    let code = [
        0x48, 0xC7, 0xC1, 0x00, 0x00, 0x50, 0x00, // MOV RCX, 0x500000
        0xF3, 0x0F, 0xAE, 0xB1, 0x00, 0x08, 0x00, 0x00, // CLRSSBSY [rcx+0x800]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_clrssbsy_multiple() {
    // Multiple CLRSSBSY operations
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x00, 0x50, 0x00, // MOV RAX, 0x500000
        0xF3, 0x0F, 0xAE, 0x30, // CLRSSBSY [rax]
        0x48, 0xC7, 0xC0, 0x00, 0x00, 0x60, 0x00, // MOV RAX, 0x600000
        0xF3, 0x0F, 0xAE, 0x30, // CLRSSBSY [rax]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// SETSSBSY Tests - Set Shadow Stack Busy Flag
// ============================================================================

#[test]
fn test_setssbsy_basic() {
    // SETSSBSY - Mark shadow stack as busy
    // Opcode: F3 0F 01 E8
    let code = [
        0xF3, 0x0F, 0x01, 0xE8, // SETSSBSY
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_setssbsy_no_operands() {
    // SETSSBSY takes no operands
    let code = [0xF3, 0x0F, 0x01, 0xE8, 0xF4];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_setssbsy_preserves_registers() {
    // SETSSBSY should not modify GP registers
    let code = [
        0x48, 0xC7, 0xC0, 0x33, 0x33, 0x33, 0x33, // MOV RAX, 0x33333333
        0x48, 0xC7, 0xC1, 0x44, 0x44, 0x44, 0x44, // MOV RCX, 0x44444444
        0xF3, 0x0F, 0x01, 0xE8, // SETSSBSY
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x33333333, "RAX should not be modified");
    assert_eq!(regs.rcx, 0x44444444, "RCX should not be modified");
}

#[test]
fn test_setssbsy_multiple() {
    // Multiple SETSSBSY operations
    let code = [
        0xF3, 0x0F, 0x01, 0xE8, // SETSSBSY
        0xF3, 0x0F, 0x01, 0xE8, // SETSSBSY
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// WRUSSD/WRUSSQ Tests - Write to User Shadow Stack
// ============================================================================

#[test]
fn test_wrussd_basic() {
    // WRUSSD - Write 32-bit value to user shadow stack
    // Opcode: 66 0F 38 F5
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x00, 0x70, 0x00, // MOV RAX, 0x700000
        0x48, 0xC7, 0xC3, 0x42, 0x42, 0x42, 0x42, // MOV RBX, 0x42424242
        0x66, 0x0F, 0x38, 0xF5, 0x18, // WRUSSD [rax], ebx
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_wrussq_basic() {
    // WRUSSQ - Write 64-bit value to user shadow stack
    // Opcode: 66 REX.W 0F 38 F5
    let code = [
        0x48, 0xC7, 0xC1, 0x00, 0x00, 0x80, 0x00, // MOV RCX, 0x800000
        0x48, 0xC7, 0xC2, 0x88, 0x77, 0x66, 0x55, // MOV RDX, 0x55667788
        0x66, 0x48, 0x0F, 0x38, 0xF5, 0x11, // WRUSSQ [rcx], rdx
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_wrussd_different_values() {
    // Write different values
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x00, 0x70, 0x00, // MOV RAX, 0x700000
        0x48, 0xC7, 0xC3, 0x11, 0x11, 0x11, 0x11, // MOV RBX, 0x11111111
        0x66, 0x0F, 0x38, 0xF5, 0x18, // WRUSSD [rax], ebx
        0x48, 0xC7, 0xC3, 0x22, 0x22, 0x22, 0x22, // MOV RBX, 0x22222222
        0x66, 0x0F, 0x38, 0xF5, 0x58, 0x08, // WRUSSD [rax+8], ebx
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_wrussq_different_registers() {
    // Use different registers
    let code = [
        0x48, 0xC7, 0xC6, 0x00, 0x00, 0x80, 0x00, // MOV RSI, 0x800000
        0x48, 0xC7, 0xC7, 0xAA, 0xBB, 0xCC, 0xDD, // MOV RDI, 0xDDCCBBAA
        0x66, 0x48, 0x0F, 0x38, 0xF5, 0x3E, // WRUSSQ [rsi], rdi
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_wrussd_with_displacement() {
    // WRUSSD with displacement
    let code = [
        0x48, 0xC7, 0xC2, 0x00, 0x00, 0x70, 0x00, // MOV RDX, 0x700000
        0x48, 0xC7, 0xC0, 0xFF, 0xFF, 0xFF, 0x00, // MOV RAX, 0x00FFFFFF
        0x66, 0x0F, 0x38, 0xF5, 0x82, 0x00, 0x10, 0x00, 0x00, // WRUSSD [rdx+0x1000], eax
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_wrussd_zero_value() {
    // Write zero
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x00, 0x70, 0x00, // MOV RAX, 0x700000
        0x48, 0x31, 0xDB, // XOR RBX, RBX
        0x66, 0x0F, 0x38, 0xF5, 0x18, // WRUSSD [rax], ebx
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_wrussq_multiple_writes() {
    // Multiple 64-bit writes
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x00, 0x80, 0x00, // MOV RAX, 0x800000
        0x48, 0xC7, 0xC3, 0x01, 0x00, 0x00, 0x00, // MOV RBX, 1
        0x66, 0x48, 0x0F, 0x38, 0xF5, 0x18, // WRUSSQ [rax], rbx
        0x48, 0xC7, 0xC3, 0x02, 0x00, 0x00, 0x00, // MOV RBX, 2
        0x66, 0x48, 0x0F, 0x38, 0xF5, 0x58, 0x08, // WRUSSQ [rax+8], rbx
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// CLAC/STAC Tests - Supervisor Mode Access Prevention
// ============================================================================

#[test]
fn test_clac_basic() {
    // CLAC - Clear AC flag in RFLAGS
    // Opcode: 0F 01 CA
    let code = [
        0x0F, 0x01, 0xCA, // CLAC
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    // AC flag (bit 18) should be cleared
    assert_eq!(regs.rflags & (1 << 18), 0, "AC flag should be cleared");
}

#[test]
fn test_stac_basic() {
    // STAC - Set AC flag in RFLAGS
    // Opcode: 0F 01 CB
    let code = [
        0x0F, 0x01, 0xCB, // STAC
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    // AC flag (bit 18) should be set
    assert_eq!(regs.rflags & (1 << 18), 1 << 18, "AC flag should be set");
}

#[test]
fn test_clac_stac_sequence() {
    // Set and clear AC flag
    let code = [
        0x0F, 0x01, 0xCB, // STAC
        0x0F, 0x01, 0xCA, // CLAC
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rflags & (1 << 18), 0, "AC flag should be cleared");
}

#[test]
fn test_clac_no_operands() {
    // CLAC takes no operands
    let code = [0x0F, 0x01, 0xCA, 0xF4];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_stac_no_operands() {
    // STAC takes no operands
    let code = [0x0F, 0x01, 0xCB, 0xF4];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_clac_preserves_registers() {
    // CLAC should not modify GP registers
    let code = [
        0x48, 0xC7, 0xC0, 0xAA, 0xAA, 0xAA, 0xAA, // MOV RAX, 0xAAAAAAAA
        0x48, 0xC7, 0xC3, 0xBB, 0xBB, 0xBB, 0xBB, // MOV RBX, 0xBBBBBBBB
        0x0F, 0x01, 0xCA, // CLAC
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    // MOV r64, imm32 sign-extends, so 0xAAAAAAAA becomes 0xFFFFFFFFAAAAAAAA
    assert_eq!(
        regs.rax, 0xFFFFFFFFAAAAAAAAu64,
        "RAX should not be modified"
    );
    assert_eq!(
        regs.rbx, 0xFFFFFFFFBBBBBBBBu64,
        "RBX should not be modified"
    );
}

#[test]
fn test_stac_preserves_registers() {
    // STAC should not modify GP registers
    let code = [
        0x48, 0xC7, 0xC1, 0xCC, 0xCC, 0xCC, 0xCC, // MOV RCX, 0xCCCCCCCC
        0x48, 0xC7, 0xC2, 0xDD, 0xDD, 0xDD, 0xDD, // MOV RDX, 0xDDDDDDDD
        0x0F, 0x01, 0xCB, // STAC
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    // MOV r64, imm32 sign-extends, so 0xCCCCCCCC becomes 0xFFFFFFFFCCCCCCCC
    assert_eq!(
        regs.rcx, 0xFFFFFFFFCCCCCCCCu64,
        "RCX should not be modified"
    );
    assert_eq!(
        regs.rdx, 0xFFFFFFFFDDDDDDDDu64,
        "RDX should not be modified"
    );
}

#[test]
fn test_clac_preserves_other_flags() {
    // CLAC should only affect AC flag
    let code = [
        0x48, 0xC7, 0xC0, 0xFF, 0xFF, 0xFF, 0xFF, // MOV RAX, -1
        0x48, 0x83, 0xC0, 0x01, // ADD RAX, 1 (sets ZF)
        0x0F, 0x01, 0xCA, // CLAC
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rflags & 0x40, 0x40, "ZF should be preserved");
}

#[test]
fn test_stac_multiple() {
    // Multiple STAC operations (idempotent)
    let code = [
        0x0F, 0x01, 0xCB, // STAC
        0x0F, 0x01, 0xCB, // STAC
        0x0F, 0x01, 0xCB, // STAC
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rflags & (1 << 18),
        1 << 18,
        "AC flag should remain set"
    );
}

#[test]
fn test_clac_multiple() {
    // Multiple CLAC operations (idempotent)
    let code = [
        0x0F, 0x01, 0xCA, // CLAC
        0x0F, 0x01, 0xCA, // CLAC
        0x0F, 0x01, 0xCA, // CLAC
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rflags & (1 << 18), 0, "AC flag should remain cleared");
}

// ============================================================================
// Combined CET Operation Tests
// ============================================================================

#[test]
fn test_cet_shadow_stack_setup() {
    // Setup shadow stack operations
    let code = [
        0xF3, 0x0F, 0x01, 0xE8, // SETSSBSY
        0x48, 0xC7, 0xC0, 0x04, 0x00, 0x00, 0x00, // MOV RAX, 4
        0xF3, 0x48, 0x0F, 0xAE, 0xE8, // INCSSPQ rax
        0xF3, 0x0F, 0x01, 0xEA, // SAVEPREVSSP
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_cet_shadow_stack_save_restore() {
    // Save and restore shadow stack
    let code = [
        0xF3, 0x0F, 0x01, 0xEA, // SAVEPREVSSP
        0x48, 0xC7, 0xC0, 0x00, 0x00, 0x30, 0x00, // MOV RAX, 0x300000
        0xF3, 0x0F, 0x01, 0x28, // RSTORSSP [rax]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_cet_busy_flag_operations() {
    // Set and clear busy flag
    let code = [
        0xF3, 0x0F, 0x01, 0xE8, // SETSSBSY
        0x48, 0xC7, 0xC0, 0x00, 0x00, 0x50, 0x00, // MOV RAX, 0x500000
        0xF3, 0x0F, 0xAE, 0x30, // CLRSSBSY [rax]
        0xF3, 0x0F, 0x01, 0xE8, // SETSSBSY
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_cet_write_operations() {
    // Write to shadow stack
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x00, 0x70, 0x00, // MOV RAX, 0x700000
        0x48, 0xC7, 0xC3, 0x01, 0x02, 0x03, 0x04, // MOV RBX, 0x04030201
        0x66, 0x0F, 0x38, 0xF5, 0x18, // WRUSSD [rax], ebx
        0x48, 0xC7, 0xC1, 0x00, 0x00, 0x80, 0x00, // MOV RCX, 0x800000
        0x48, 0xC7, 0xC2, 0x05, 0x06, 0x07, 0x08, // MOV RDX, 0x08070605
        0x66, 0x48, 0x0F, 0x38, 0xF5, 0x11, // WRUSSQ [rcx], rdx
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_cet_smap_control() {
    // SMAP control with CLAC/STAC
    let code = [
        0x0F, 0x01, 0xCB, // STAC (enable user access)
        // Access user memory here...
        0x0F, 0x01, 0xCA, // CLAC (disable user access)
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rflags & (1 << 18), 0, "AC flag should be cleared");
}

#[test]
fn test_cet_complete_workflow() {
    // Complete CET workflow
    let code = [
        // Initialize shadow stack
        0xF3, 0x0F, 0x01, 0xE8, // SETSSBSY
        // Write initial values
        0x48, 0xC7, 0xC0, 0x00, 0x00, 0x70, 0x00, // MOV RAX, 0x700000
        0x48, 0xC7, 0xC3, 0x42, 0x00, 0x00, 0x00, // MOV RBX, 0x42
        0x66, 0x48, 0x0F, 0x38, 0xF5, 0x18, // WRUSSQ [rax], rbx
        // Increment pointer
        0x48, 0xC7, 0xC1, 0x02, 0x00, 0x00, 0x00, // MOV RCX, 2
        0xF3, 0x48, 0x0F, 0xAE, 0xE9, // INCSSPQ rcx
        // Save state
        0xF3, 0x0F, 0x01, 0xEA, // SAVEPREVSSP
        // Restore state
        0x48, 0xC7, 0xC2, 0x00, 0x00, 0x30, 0x00, // MOV RDX, 0x300000
        0xF3, 0x0F, 0x01, 0x2A, // RSTORSSP [rdx]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}
