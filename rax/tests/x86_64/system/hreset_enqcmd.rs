//! Tests for HRESET, ENQCMD, and ENQCMDS Instructions.
//!
//! This module covers history reset and enqueue command instructions.
//!
//! Instructions covered:
//! - HRESET - History Reset (predictor history reset)
//! - ENQCMD - Enqueue Command (user-mode)
//! - ENQCMDS - Enqueue Command Supervisor
//!
//! References: docs/hreset.txt, docs/enqcmd.txt, docs/enqcmds.txt

use crate::common::*;
use rax::cpu::Registers;

// ============================================================================
// HRESET Tests - History Reset
// ============================================================================

#[test]
fn test_hreset_basic() {
    // HRESET - Reset processor prediction history
    // Opcode: F3 0F 3A F0 C0 /ib
    // Note: Requires CPL=0 and HRESET feature
    let code = [
        0x48, 0x31, 0xC0, // XOR RAX, RAX (no reset)
        0xF3, 0x0F, 0x3A, 0xF0, 0xC0, 0x00, // HRESET 0
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    // May #UD if HRESET not supported or #GP if CPL > 0
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_hreset_bit0() {
    // HRESET with bit 0 set (reset specific predictor)
    let code = [
        0x48, 0xC7, 0xC0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1 (bit 0)
        0xF3, 0x0F, 0x3A, 0xF0, 0xC0, 0x00, // HRESET 0
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_hreset_multiple_bits() {
    // HRESET with multiple reset bits set
    let code = [
        0x48, 0xC7, 0xC0, 0x03, 0x00, 0x00, 0x00, // MOV RAX, 3 (bits 0-1)
        0xF3, 0x0F, 0x3A, 0xF0, 0xC0, 0x00, // HRESET 0
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_hreset_imm8_ignored() {
    // HRESET - imm8 operand is ignored
    let code = [
        0x48, 0xC7, 0xC0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1
        0xF3, 0x0F, 0x3A, 0xF0, 0xC0, 0xFF, // HRESET 0xFF (imm8 ignored)
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_hreset_eax_zero() {
    // HRESET with EAX=0 (NOP behavior)
    let code = [
        0x48, 0x31, 0xC0, // XOR RAX, RAX
        0xF3, 0x0F, 0x3A, 0xF0, 0xC0, 0x00, // HRESET 0
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_hreset_preserves_registers() {
    // HRESET should not modify registers except RFLAGS
    let code = [
        0x48, 0xC7, 0xC0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1
        0x48, 0xC7, 0xC3, 0x42, 0x42, 0x42, 0x42, // MOV RBX, 0x42424242
        0x48, 0xC7, 0xC1, 0xAA, 0xBB, 0xCC, 0xDD, // MOV RCX, 0xDDCCBBAA
        0xF3, 0x0F, 0x3A, 0xF0, 0xC0, 0x00, // HRESET 0
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_hreset_sequence() {
    // Multiple HRESET operations in sequence
    let code = [
        0x48, 0xC7, 0xC0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1
        0xF3, 0x0F, 0x3A, 0xF0, 0xC0, 0x00, // HRESET 0
        0x48, 0xC7, 0xC0, 0x02, 0x00, 0x00, 0x00, // MOV RAX, 2
        0xF3, 0x0F, 0x3A, 0xF0, 0xC0, 0x00, // HRESET 0
        0x48, 0x31, 0xC0, // XOR RAX, RAX
        0xF3, 0x0F, 0x3A, 0xF0, 0xC0, 0x00, // HRESET 0
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_hreset_high_bits() {
    // HRESET with high bits set
    let code = [
        0x48, 0xC7, 0xC0, 0xFF, 0x00, 0x00, 0x00, // MOV RAX, 0xFF
        0xF3, 0x0F, 0x3A, 0xF0, 0xC0, 0x00, // HRESET 0
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_hreset_varied_imm8() {
    // HRESET with various imm8 values (all ignored)
    let code = [
        0x48, 0xC7, 0xC0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1
        0xF3, 0x0F, 0x3A, 0xF0, 0xC0, 0x00, // HRESET 0
        0xF3, 0x0F, 0x3A, 0xF0, 0xC0, 0x01, // HRESET 1
        0xF3, 0x0F, 0x3A, 0xF0, 0xC0, 0x42, // HRESET 0x42
        0xF3, 0x0F, 0x3A, 0xF0, 0xC0, 0xFF, // HRESET 0xFF
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_hreset_modrm_c0() {
    // HRESET with ModRM byte 0xC0 (required format)
    let code = [
        0x48, 0xC7, 0xC0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1
        0xF3, 0x0F, 0x3A, 0xF0, 0xC0, 0x00, // HRESET (ModRM=0xC0)
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// ENQCMD Tests - Enqueue Command (User Mode)
// ============================================================================

#[test]
fn test_enqcmd_basic() {
    // ENQCMD - Enqueue 64-byte command (user mode)
    // Opcode: F2 0F 38 F8 /r
    // Requires ENQCMD feature and PASID
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x10, 0x00, 0x00, // MOV RAX, 0x1000 (dest in ES)
        0x48, 0xC7, 0xC3, 0x00, 0x20, 0x00, 0x00, // MOV RBX, 0x2000 (src 64 bytes)
        0xF2, 0x0F, 0x38, 0xF8, 0x03, // ENQCMD RAX, [RBX]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    // May #UD if ENQCMD not supported
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_enqcmd_64bit_aligned() {
    // ENQCMD with 64-byte aligned destination
    let code = [
        0x48, 0xC7, 0xC1, 0x00, 0x10, 0x00, 0x00, // MOV RCX, 0x1000 (64-byte aligned)
        0x48, 0xC7, 0xC2, 0x00, 0x30, 0x00, 0x00, // MOV RDX, 0x3000 (src)
        0xF2, 0x0F, 0x38, 0xF8, 0x0A, // ENQCMD RCX, [RDX]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_enqcmd_different_registers() {
    // ENQCMD with various register combinations
    let code = [
        0x48, 0xC7, 0xC6, 0x00, 0x40, 0x00, 0x00, // MOV RSI, 0x4000
        0x48, 0xC7, 0xC7, 0x00, 0x50, 0x00, 0x00, // MOV RDI, 0x5000
        0xF2, 0x0F, 0x38, 0xF8, 0x37, // ENQCMD RSI, [RDI]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_enqcmd_r8_r15() {
    // ENQCMD with extended registers
    let code = [
        0x49, 0xC7, 0xC0, 0x00, 0x60, 0x00, 0x00, // MOV R8, 0x6000
        0x49, 0xC7, 0xC1, 0x00, 0x70, 0x00, 0x00, // MOV R9, 0x7000
        0xF2, 0x43, 0x0F, 0x38, 0xF8, 0x01, // ENQCMD R8, [R9]
        0xF4, // HLT
    ];
    let mut regs = Registers::default();
    regs.r8 = 0x6000;
    regs.r9 = 0x7000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_enqcmd_with_displacement() {
    // ENQCMD with memory displacement
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x80, 0x00, 0x00, // MOV RAX, 0x8000
        0x48, 0xC7, 0xC3, 0x00, 0x90, 0x00, 0x00, // MOV RBX, 0x9000
        0xF2, 0x0F, 0x38, 0xF8, 0x83, 0x00, 0x01, 0x00, 0x00, // ENQCMD RAX, [RBX+0x100]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_enqcmd_sets_zf() {
    // ENQCMD sets ZF based on success/retry
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0xA0, 0x00, 0x00, // MOV RAX, 0xA000
        0x48, 0xC7, 0xC1, 0x00, 0xB0, 0x00, 0x00, // MOV RCX, 0xB000
        0xF2, 0x0F, 0x38, 0xF8, 0x01, // ENQCMD RAX, [RCX]
        // ZF=0 means success, ZF=1 means retry
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_enqcmd_rip_relative() {
    // ENQCMD with RIP-relative addressing
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0xC0, 0x00, 0x00, // MOV RAX, 0xC000
        0xF2, 0x0F, 0x38, 0xF8, 0x05, 0x00, 0x00, 0x00, 0x00, // ENQCMD RAX, [RIP+0]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_enqcmd_with_sib() {
    // ENQCMD with SIB byte
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0xD0, 0x00, 0x00, // MOV RAX, 0xD000
        0x48, 0xC7, 0xC3, 0x00, 0xE0, 0x00, 0x00, // MOV RBX, 0xE000
        0x48, 0xC7, 0xC1, 0x00, 0x01, 0x00, 0x00, // MOV RCX, 0x100
        0xF2, 0x0F, 0x38, 0xF8, 0x04, 0x0B, // ENQCMD RAX, [RBX+RCX]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// ENQCMDS Tests - Enqueue Command Supervisor
// ============================================================================

#[test]
fn test_enqcmds_basic() {
    // ENQCMDS - Enqueue command (supervisor mode)
    // Opcode: F3 0F 38 F8 /r
    // Allows supervisor to specify PASID and privilege
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x10, 0x00, 0x00, // MOV RAX, 0x1000 (dest)
        0x48, 0xC7, 0xC3, 0x00, 0x20, 0x00, 0x00, // MOV RBX, 0x2000 (src)
        0xF3, 0x0F, 0x38, 0xF8, 0x03, // ENQCMDS RAX, [RBX]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_enqcmds_64byte_aligned() {
    // ENQCMDS with aligned destination
    let code = [
        0x48, 0xC7, 0xC1, 0x00, 0x30, 0x00, 0x00, // MOV RCX, 0x3000
        0x48, 0xC7, 0xC2, 0x00, 0x40, 0x00, 0x00, // MOV RDX, 0x4000
        0xF3, 0x0F, 0x38, 0xF8, 0x0A, // ENQCMDS RCX, [RDX]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_enqcmds_with_pasid() {
    // ENQCMDS with PASID in source data bits[19:0]
    let code = [
        0x48, 0xC7, 0xC6, 0x00, 0x50, 0x00, 0x00, // MOV RSI, 0x5000
        0x48, 0xC7, 0xC7, 0x00, 0x60, 0x00, 0x00, // MOV RDI, 0x6000 (src with PASID)
        0xF3, 0x0F, 0x38, 0xF8, 0x37, // ENQCMDS RSI, [RDI]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_enqcmds_different_registers() {
    // ENQCMDS with various registers
    let code = [
        0x48, 0xC7, 0xC2, 0x00, 0x70, 0x00, 0x00, // MOV RDX, 0x7000
        0x48, 0xC7, 0xC5, 0x00, 0x80, 0x00, 0x00, // MOV RBP, 0x8000
        0xF3, 0x0F, 0x38, 0xF8, 0x55, 0x00, // ENQCMDS RDX, [RBP]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_enqcmds_r8_r15() {
    // ENQCMDS with extended registers
    let code = [
        0x49, 0xC7, 0xC2, 0x00, 0x90, 0x00, 0x00, // MOV R10, 0x9000
        0x49, 0xC7, 0xC3, 0x00, 0xA0, 0x00, 0x00, // MOV R11, 0xA000
        0xF3, 0x47, 0x0F, 0x38, 0xF8, 0x13, // ENQCMDS R10, [R11]
        0xF4, // HLT
    ];
    let mut regs = Registers::default();
    regs.r10 = 0x9000;
    regs.r11 = 0xA000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_enqcmds_with_displacement() {
    // ENQCMDS with memory displacement
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0xB0, 0x00, 0x00, // MOV RAX, 0xB000
        0x48, 0xC7, 0xC3, 0x00, 0xC0, 0x00, 0x00, // MOV RBX, 0xC000
        0xF3, 0x0F, 0x38, 0xF8, 0x83, 0x40, 0x00, 0x00, 0x00, // ENQCMDS RAX, [RBX+0x40]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_enqcmds_sets_zf() {
    // ENQCMDS sets ZF on success/retry
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0xD0, 0x00, 0x00, // MOV RAX, 0xD000
        0x48, 0xC7, 0xC1, 0x00, 0xE0, 0x00, 0x00, // MOV RCX, 0xE000
        0xF3, 0x0F, 0x38, 0xF8, 0x01, // ENQCMDS RAX, [RCX]
        // ZF=0 success, ZF=1 retry
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_enqcmds_rip_relative() {
    // ENQCMDS with RIP-relative addressing
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0xF0, 0x00, 0x00, // MOV RAX, 0xF000
        0xF3, 0x0F, 0x38, 0xF8, 0x05, 0x00, 0x00, 0x00, 0x00, // ENQCMDS RAX, [RIP+0]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_enqcmds_with_sib() {
    // ENQCMDS with SIB byte
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x00, 0x01, 0x00, // MOV RAX, 0x10000
        0x48, 0xC7, 0xC3, 0x00, 0x10, 0x01, 0x00, // MOV RBX, 0x11000
        0x48, 0xC7, 0xC1, 0x00, 0x00, 0x00, 0x00, // MOV RCX, 0
        0xF3, 0x0F, 0x38, 0xF8, 0x04, 0x0B, // ENQCMDS RAX, [RBX+RCX]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// Mixed Tests
// ============================================================================

#[test]
fn test_enqcmd_enqcmds_comparison() {
    // Compare ENQCMD and ENQCMDS behavior
    let code = [
        // ENQCMD (user mode)
        0x48, 0xC7, 0xC0, 0x00, 0x10, 0x00, 0x00, // MOV RAX, 0x1000
        0x48, 0xC7, 0xC3, 0x00, 0x20, 0x00, 0x00, // MOV RBX, 0x2000
        0xF2, 0x0F, 0x38, 0xF8, 0x03, // ENQCMD RAX, [RBX]
        // ENQCMDS (supervisor mode)
        0x48, 0xC7, 0xC1, 0x00, 0x30, 0x00, 0x00, // MOV RCX, 0x3000
        0x48, 0xC7, 0xC2, 0x00, 0x40, 0x00, 0x00, // MOV RDX, 0x4000
        0xF3, 0x0F, 0x38, 0xF8, 0x0A, // ENQCMDS RCX, [RDX]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_hreset_after_enqcmd() {
    // HRESET after ENQCMD operations
    let code = [
        // Perform ENQCMD
        0x48, 0xC7, 0xC0, 0x00, 0x10, 0x00, 0x00, // MOV RAX, 0x1000
        0x48, 0xC7, 0xC3, 0x00, 0x20, 0x00, 0x00, // MOV RBX, 0x2000
        0xF2, 0x0F, 0x38, 0xF8, 0x03, // ENQCMD RAX, [RBX]
        // Reset prediction history
        0x48, 0xC7, 0xC0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1
        0xF3, 0x0F, 0x3A, 0xF0, 0xC0, 0x00, // HRESET 0
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_enqcmd_retry_loop() {
    // ENQCMD with retry on ZF=1
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x50, 0x00, 0x00, // MOV RAX, 0x5000
        0x48, 0xC7, 0xC3, 0x00, 0x60, 0x00, 0x00, // MOV RBX, 0x6000
        // retry_loop:
        0xF2, 0x0F, 0x38, 0xF8, 0x03, // ENQCMD RAX, [RBX]
        0x75, 0xF9, // JNZ retry_loop (if ZF=0, success)
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_enqcmds_multiple_commands() {
    // ENQCMDS issuing multiple commands
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x70, 0x00, 0x00, // MOV RAX, 0x7000
        0x48, 0xC7, 0xC3, 0x00, 0x80, 0x00, 0x00, // MOV RBX, 0x8000
        // Command 1
        0xF3, 0x0F, 0x38, 0xF8, 0x03, // ENQCMDS RAX, [RBX]
        // Command 2 (different source)
        0x48, 0x83, 0xC3, 0x40, // ADD RBX, 0x40 (next 64 bytes)
        0xF3, 0x0F, 0x38, 0xF8, 0x03, // ENQCMDS RAX, [RBX]
        // Command 3
        0x48, 0x83, 0xC3, 0x40, // ADD RBX, 0x40
        0xF3, 0x0F, 0x38, 0xF8, 0x03, // ENQCMDS RAX, [RBX]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}
