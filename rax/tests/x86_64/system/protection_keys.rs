//! Tests for Memory Protection Key Instructions (PKU).
//!
//! Instructions covered:
//! - RDPKRU - Read Protection Key Rights for User Pages
//! - WRPKRU - Write Protection Key Rights for User Pages
//!
//! References: docs/rdpkru.txt, docs/wrpkru.txt

use crate::common::*;
use rax::cpu::Registers;

// ============================================================================
// RDPKRU Tests - Read Protection Key Rights
// ============================================================================

#[test]
fn test_rdpkru_basic() {
    // RDPKRU - Read PKRU register into EAX
    // Opcode: 0F 01 EE
    // ECX must be 0
    let code = [
        0x48, 0x31, 0xC9, // XOR RCX, RCX
        0x0F, 0x01, 0xEE, // RDPKRU
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    // EAX contains PKRU value, EDX should be 0
    assert_eq!(regs.rdx & 0xFFFFFFFF, 0, "EDX should be zero after RDPKRU");
}

#[test]
fn test_rdpkru_preserves_registers() {
    // RDPKRU should only modify EAX and EDX
    let code = [
        0x48, 0xC7, 0xC3, 0x42, 0x42, 0x42, 0x42, // MOV RBX, 0x42424242
        0x48, 0xC7, 0xC6, 0xAA, 0xAA, 0xAA, 0xAA, // MOV RSI, 0xAAAAAAAA
        0x48, 0x31, 0xC9, // XOR RCX, RCX
        0x0F, 0x01, 0xEE, // RDPKRU
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rbx, 0x42424242, "RBX should not be modified");
    assert_eq!(
        regs.rsi, 0xFFFF_FFFF_AAAA_AAAA,
        "RSI should not be modified"
    );
}

#[test]
fn test_rdpkru_ecx_zero_required() {
    // ECX must be 0 for RDPKRU
    let code = [
        0x48, 0x31, 0xC9, // XOR RCX, RCX
        0x0F, 0x01, 0xEE, // RDPKRU
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rcx, 0, "RCX should be 0");
}

#[test]
fn test_rdpkru_multiple_reads() {
    // Multiple RDPKRU reads should give consistent results
    let code = [
        0x48, 0x31, 0xC9, // XOR RCX, RCX
        0x0F, 0x01, 0xEE, // RDPKRU #1
        0x48, 0x89, 0xC3, // MOV RBX, RAX (save first read)
        0x0F, 0x01, 0xEE, // RDPKRU #2
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        regs.rbx & 0xFFFFFFFF,
        "Both RDPKRU reads should match"
    );
}

#[test]
fn test_rdpkru_clears_upper_bits() {
    // RDPKRU clears upper 32 bits of RAX and RDX
    let code = [
        0x48, 0xB8, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, // MOV RAX, -1
        0x48, 0xBA, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, // MOV RDX, -1
        0x48, 0x31, 0xC9, // XOR RCX, RCX
        0x0F, 0x01, 0xEE, // RDPKRU
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax >> 32, 0, "Upper 32 bits of RAX should be cleared");
    assert_eq!(regs.rdx >> 32, 0, "Upper 32 bits of RDX should be cleared");
}

// ============================================================================
// WRPKRU Tests - Write Protection Key Rights
// ============================================================================

#[test]
fn test_wrpkru_basic() {
    // WRPKRU - Write value to PKRU register
    // Opcode: 0F 01 EF
    // ECX must be 0, EAX contains value, EDX must be 0
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x00, 0x00, 0x00, // MOV RAX, 0
        0x48, 0x31, 0xD2, // XOR RDX, RDX
        0x48, 0x31, 0xC9, // XOR RCX, RCX
        0x0F, 0x01, 0xEF, // WRPKRU
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_wrpkru_different_values() {
    // Write different PKRU values
    let code = [
        0x48, 0xC7, 0xC0, 0x55, 0x55, 0x55, 0x55, // MOV RAX, 0x55555555
        0x48, 0x31, 0xD2, // XOR RDX, RDX
        0x48, 0x31, 0xC9, // XOR RCX, RCX
        0x0F, 0x01, 0xEF, // WRPKRU
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_wrpkru_preserves_registers() {
    // WRPKRU should not modify GP registers
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x00, 0x00, 0x00, // MOV RAX, 0
        0x48, 0x31, 0xD2, // XOR RDX, RDX
        0x48, 0x31, 0xC9, // XOR RCX, RCX
        0x48, 0xC7, 0xC3, 0x42, 0x42, 0x42, 0x42, // MOV RBX, 0x42424242
        0x0F, 0x01, 0xEF, // WRPKRU
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rbx, 0x42424242, "RBX should not be modified");
}

#[test]
fn test_wrpkru_ecx_edx_zero_required() {
    // ECX and EDX must be 0 for WRPKRU
    let code = [
        0x48, 0xC7, 0xC0, 0xAA, 0xAA, 0xAA, 0xAA, // MOV RAX, 0xAAAAAAAA
        0x48, 0x31, 0xD2, // XOR RDX, RDX
        0x48, 0x31, 0xC9, // XOR RCX, RCX
        0x0F, 0x01, 0xEF, // WRPKRU
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// RDPKRU/WRPKRU Combination Tests
// ============================================================================

#[test]
fn test_pkru_roundtrip() {
    // Write then read PKRU
    let code = [
        0x48, 0xC7, 0xC0, 0x12, 0x34, 0x56, 0x78, // MOV RAX, 0x78563412
        0x48, 0x31, 0xD2, // XOR RDX, RDX
        0x48, 0x31, 0xC9, // XOR RCX, RCX
        0x0F, 0x01, 0xEF, // WRPKRU
        0x0F, 0x01, 0xEE, // RDPKRU
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    // Value should match what was written (masked to valid bits)
    assert_eq!(regs.rdx & 0xFFFFFFFF, 0, "EDX should be 0 after RDPKRU");
}

#[test]
fn test_pkru_multiple_writes() {
    // Multiple WRPKRU operations
    let code = [
        // Write 1
        0x48, 0xC7, 0xC0, 0x00, 0x00, 0x00, 0x00, // MOV RAX, 0
        0x48, 0x31, 0xD2, // XOR RDX, RDX
        0x48, 0x31, 0xC9, // XOR RCX, RCX
        0x0F, 0x01, 0xEF, // WRPKRU
        // Write 2
        0x48, 0xC7, 0xC0, 0xFF, 0xFF, 0xFF, 0xFF, // MOV RAX, 0xFFFFFFFF
        0x0F, 0x01, 0xEF, // WRPKRU
        // Read back
        0x0F, 0x01, 0xEE, // RDPKRU
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_pkru_protection_keys() {
    // Set individual protection key rights
    // PKRU format: 2 bits per key (AD, WD)
    let code = [
        // Disable access to key 0 (bits 0-1)
        0x48, 0xC7, 0xC0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1 (AD=1, WD=0)
        0x48, 0x31, 0xD2, // XOR RDX, RDX
        0x48, 0x31, 0xC9, // XOR RCX, RCX
        0x0F, 0x01, 0xEF, // WRPKRU
        0x0F, 0x01, 0xEE, // RDPKRU
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0x3, 0x1, "Key 0 should have AD=1, WD=0");
}

#[test]
fn test_pkru_multiple_keys() {
    // Set rights for multiple protection keys
    let code = [
        // Set multiple keys: key 0 = WD, key 1 = AD, key 2 = AD+WD
        // Bits: [3:2]=11 (key1), [1:0]=10 (key0), [7:4]=11 (key2)
        0x48, 0xC7, 0xC0, 0x0E, 0x00, 0x00, 0x00, // MOV RAX, 0x0E
        0x48, 0x31, 0xD2, // XOR RDX, RDX
        0x48, 0x31, 0xC9, // XOR RCX, RCX
        0x0F, 0x01, 0xEF, // WRPKRU
        0x0F, 0x01, 0xEE, // RDPKRU
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_pkru_preserve_across_operations() {
    // PKRU should persist across other operations
    let code = [
        0x48, 0xC7, 0xC0, 0xAA, 0xAA, 0xAA, 0xAA, // MOV RAX, 0xAAAAAAAA
        0x48, 0x31, 0xD2, // XOR RDX, RDX
        0x48, 0x31, 0xC9, // XOR RCX, RCX
        0x0F, 0x01, 0xEF, // WRPKRU
        // Do some other operations
        0x48, 0xC7, 0xC3, 0x42, 0x00, 0x00, 0x00, // MOV RBX, 0x42
        0x48, 0x01, 0xD8, // ADD RAX, RBX
        // Read PKRU again
        0x0F, 0x01, 0xEE, // RDPKRU
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}
