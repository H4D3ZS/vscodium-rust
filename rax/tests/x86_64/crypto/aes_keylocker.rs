//! Tests for AES Key Locker Instructions.
//!
//! This module covers AES Key Locker instructions for encrypted key management.
//!
//! Instructions covered:
//! - LOADIWKEY - Load internal wrapping key
//! - ENCODEKEY128 - Encode 128-bit AES key into handle
//! - ENCODEKEY256 - Encode 256-bit AES key into handle
//! - AESDEC128KL - AES decrypt with 128-bit key locker
//! - AESDEC256KL - AES decrypt with 256-bit key locker
//! - AESENC128KL - AES encrypt with 128-bit key locker
//! - AESENC256KL - AES encrypt with 256-bit key locker
//! - AESDECWIDE128KL - Wide AES decrypt with 128-bit key locker
//! - AESDECWIDE256KL - Wide AES decrypt with 256-bit key locker
//! - AESENCWIDE128KL - Wide AES encrypt with 128-bit key locker
//! - AESENCWIDE256KL - Wide AES encrypt with 256-bit key locker
//!
//! References: docs/loadiwkey.txt, docs/encodekey128.txt, docs/encodekey256.txt,
//!            docs/aesdec128kl.txt, docs/aesdec256kl.txt, docs/aesenc128kl.txt,
//!            docs/aesenc256kl.txt, docs/aesdecwide128kl.txt, docs/aesdecwide256kl.txt,
//!            docs/aesencwide128kl.txt, docs/aesencwide256kl.txt

use crate::common::*;
use rax::cpu::Registers;

// ============================================================================
// LOADIWKEY Tests - Load Internal Wrapping Key
// ============================================================================

#[test]
fn test_loadiwkey_basic() {
    // LOADIWKEY - Load internal wrapping key
    // Opcode: F3 0F 38 DC /r
    // Note: Requires CPL=0 and KL feature
    let code = [
        // Setup XMM0 with integrity key
        0x66, 0x0F, 0xEF, 0xC0, // PXOR XMM0, XMM0
        0x48, 0x31, 0xC0, // XOR RAX, RAX (KeySource=0, NoBackup=0)
        0x66, 0x0F, 0xEF, 0xC9, // PXOR XMM1, XMM1 (encryption key low)
        0x66, 0x0F, 0xEF, 0xD2, // PXOR XMM2, XMM2 (encryption key high)
        0xF3, 0x0F, 0x38, 0xDC, 0xCA, // LOADIWKEY XMM1, XMM2
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    // May #UD if KL not supported or #GP if CPL > 0
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_loadiwkey_keysource_0() {
    // LOADIWKEY with KeySource=0 (direct key)
    let code = [
        0x66, 0x0F, 0xEF, 0xC0, // PXOR XMM0, XMM0
        0x48, 0x31, 0xC0, // XOR RAX, RAX (KeySource=0)
        0x66, 0x0F, 0x6F, 0x0D, 0x00, 0x00, 0x00, 0x00, // MOVDQA XMM1, [data]
        0x66, 0x0F, 0x6F, 0x15, 0x00, 0x00, 0x00, 0x00, // MOVDQA XMM2, [data]
        0xF3, 0x0F, 0x38, 0xDC, 0xCA, // LOADIWKEY XMM1, XMM2
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_loadiwkey_keysource_1() {
    // LOADIWKEY with KeySource=1 (random XOR)
    let code = [
        0x66, 0x0F, 0xEF, 0xC0, // PXOR XMM0, XMM0
        0x48, 0xC7, 0xC0, 0x02, 0x00, 0x00, 0x00, // MOV RAX, 2 (KeySource=1)
        0x66, 0x0F, 0xEF, 0xC9, // PXOR XMM1, XMM1
        0x66, 0x0F, 0xEF, 0xD2, // PXOR XMM2, XMM2
        0xF3, 0x0F, 0x38, 0xDC, 0xCA, // LOADIWKEY XMM1, XMM2
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_loadiwkey_nobackup() {
    // LOADIWKEY with NoBackup flag set
    let code = [
        0x66, 0x0F, 0xEF, 0xC0, // PXOR XMM0, XMM0
        0x48, 0xC7, 0xC0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1 (NoBackup=1)
        0x66, 0x0F, 0xEF, 0xC9, // PXOR XMM1, XMM1
        0x66, 0x0F, 0xEF, 0xD2, // PXOR XMM2, XMM2
        0xF3, 0x0F, 0x38, 0xDC, 0xCA, // LOADIWKEY XMM1, XMM2
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_loadiwkey_different_registers() {
    // LOADIWKEY with different XMM registers
    let code = [
        0x66, 0x0F, 0xEF, 0xC0, // PXOR XMM0, XMM0
        0x48, 0x31, 0xC0, // XOR RAX, RAX
        0x66, 0x0F, 0xEF, 0xDB, // PXOR XMM3, XMM3
        0x66, 0x0F, 0xEF, 0xE4, // PXOR XMM4, XMM4
        0xF3, 0x0F, 0x38, 0xDC, 0xDC, // LOADIWKEY XMM3, XMM4
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_loadiwkey_xmm_high_regs() {
    // LOADIWKEY with XMM8-XMM15
    let code = [
        0x66, 0x0F, 0xEF, 0xC0, // PXOR XMM0, XMM0
        0x48, 0x31, 0xC0, // XOR RAX, RAX
        0x66, 0x45, 0x0F, 0xEF, 0xC0, // PXOR XMM8, XMM8
        0x66, 0x45, 0x0F, 0xEF, 0xC9, // PXOR XMM9, XMM9
        0xF3, 0x45, 0x0F, 0x38, 0xDC, 0xC1, // LOADIWKEY XMM8, XMM9
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// ENCODEKEY128 Tests - Encode 128-bit AES Key
// ============================================================================

#[test]
fn test_encodekey128_basic() {
    // ENCODEKEY128 - Encode 128-bit AES key into handle
    // Opcode: F3 0F 38 FA /r
    let code = [
        0x48, 0x31, 0xC0, // XOR RAX, RAX (KeySource=0)
        0x66, 0x0F, 0xEF, 0xC9, // PXOR XMM1, XMM1 (key)
        0x48, 0xC7, 0xC3, 0x00, 0x10, 0x00, 0x00, // MOV RBX, 0x1000 (handle output)
        0xF3, 0x0F, 0x38, 0xFA, 0x0B, // ENCODEKEY128 XMM1, [RBX]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_encodekey128_with_key() {
    // ENCODEKEY128 with actual key data
    let code = [
        0x48, 0x31, 0xC0, // XOR RAX, RAX
        // Load key into XMM2
        0x66, 0x0F, 0x6F, 0x15, 0x00, 0x00, 0x00, 0x00, // MOVDQA XMM2, [data]
        0x48, 0xC7, 0xC1, 0x00, 0x20, 0x00, 0x00, // MOV RCX, 0x2000
        0xF3, 0x0F, 0x38, 0xFA, 0x11, // ENCODEKEY128 XMM2, [RCX]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_encodekey128_different_registers() {
    // ENCODEKEY128 with various XMM registers
    let code = [
        0x48, 0x31, 0xC0, // XOR RAX, RAX
        0x66, 0x0F, 0xEF, 0xDB, // PXOR XMM3, XMM3
        0x48, 0xC7, 0xC2, 0x00, 0x30, 0x00, 0x00, // MOV RDX, 0x3000
        0xF3, 0x0F, 0x38, 0xFA, 0x1A, // ENCODEKEY128 XMM3, [RDX]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_encodekey128_with_displacement() {
    // ENCODEKEY128 with memory displacement
    let code = [
        0x48, 0x31, 0xC0, // XOR RAX, RAX
        0x66, 0x0F, 0xEF, 0xC0, // PXOR XMM0, XMM0
        0x48, 0xC7, 0xC3, 0x00, 0x10, 0x00, 0x00, // MOV RBX, 0x1000
        0xF3, 0x0F, 0x38, 0xFA, 0x83, 0x00, 0x01, 0x00,
        0x00, // ENCODEKEY128 XMM0, [RBX+0x100]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// ENCODEKEY256 Tests - Encode 256-bit AES Key
// ============================================================================

#[test]
fn test_encodekey256_basic() {
    // ENCODEKEY256 - Encode 256-bit AES key into handle
    // Opcode: F3 0F 38 FB /r
    let code = [
        0x48, 0x31, 0xC0, // XOR RAX, RAX
        0x66, 0x0F, 0xEF, 0xC9, // PXOR XMM1, XMM1 (key high)
        0x66, 0x0F, 0xEF, 0xD2, // PXOR XMM2, XMM2 (key low, implicit)
        0x48, 0xC7, 0xC3, 0x00, 0x40, 0x00, 0x00, // MOV RBX, 0x4000
        0xF3, 0x0F, 0x38, 0xFB, 0x0B, // ENCODEKEY256 XMM1, [RBX]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_encodekey256_full_key() {
    // ENCODEKEY256 with full 256-bit key
    let code = [
        0x48, 0x31, 0xC0, // XOR RAX, RAX
        0x66, 0x0F, 0x6F, 0x1D, 0x00, 0x00, 0x00, 0x00, // MOVDQA XMM3, [data] (high)
        0x66, 0x0F, 0x6F, 0x25, 0x00, 0x00, 0x00, 0x00, // MOVDQA XMM4, [data] (low)
        0x48, 0xC7, 0xC1, 0x00, 0x50, 0x00, 0x00, // MOV RCX, 0x5000
        0xF3, 0x0F, 0x38, 0xFB, 0x19, // ENCODEKEY256 XMM3, [RCX]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_encodekey256_xmm_high_regs() {
    // ENCODEKEY256 with XMM8-XMM15
    let code = [
        0x48, 0x31, 0xC0, // XOR RAX, RAX
        0x66, 0x45, 0x0F, 0xEF, 0xED, // PXOR XMM13, XMM13
        0x49, 0xC7, 0xC0, 0x00, 0x60, 0x00, 0x00, // MOV R8, 0x6000
        0xF3, 0x47, 0x0F, 0x38, 0xFB, 0x28, // ENCODEKEY256 XMM13, [R8]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// AESENC128KL Tests - AES Encrypt with 128-bit Key Locker
// ============================================================================

#[test]
fn test_aesenc128kl_basic() {
    // AESENC128KL - AES encrypt with 128-bit key locker
    // Opcode: F3 0F 38 DC /r
    let code = [
        0x66, 0x0F, 0xEF, 0xC0, // PXOR XMM0, XMM0 (data)
        0x48, 0xC7, 0xC3, 0x00, 0x10, 0x00, 0x00, // MOV RBX, 0x1000 (handle)
        0xF3, 0x0F, 0x38, 0xDC, 0x03, // AESENC128KL XMM0, [RBX]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_aesenc128kl_data_encryption() {
    // AESENC128KL encrypting actual data
    let code = [
        0x66, 0x0F, 0x6F, 0x05, 0x00, 0x00, 0x00, 0x00, // MOVDQA XMM0, [plaintext]
        0x48, 0xC7, 0xC1, 0x00, 0x20, 0x00, 0x00, // MOV RCX, 0x2000 (handle)
        0xF3, 0x0F, 0x38, 0xDC, 0x01, // AESENC128KL XMM0, [RCX]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_aesenc128kl_different_xmm() {
    // AESENC128KL with various XMM registers
    let code = [
        0x66, 0x0F, 0xEF, 0xED, // PXOR XMM5, XMM5
        0x48, 0xC7, 0xC2, 0x00, 0x30, 0x00, 0x00, // MOV RDX, 0x3000
        0xF3, 0x0F, 0x38, 0xDC, 0x2A, // AESENC128KL XMM5, [RDX]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// AESDEC128KL Tests - AES Decrypt with 128-bit Key Locker
// ============================================================================

#[test]
fn test_aesdec128kl_basic() {
    // AESDEC128KL - AES decrypt with 128-bit key locker
    // Opcode: F3 0F 38 DD /r
    let code = [
        0x66, 0x0F, 0xEF, 0xC0, // PXOR XMM0, XMM0 (ciphertext)
        0x48, 0xC7, 0xC3, 0x00, 0x40, 0x00, 0x00, // MOV RBX, 0x4000 (handle)
        0xF3, 0x0F, 0x38, 0xDD, 0x03, // AESDEC128KL XMM0, [RBX]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_aesdec128kl_data_decryption() {
    // AESDEC128KL decrypting ciphertext
    let code = [
        0x66, 0x0F, 0x6F, 0x0D, 0x00, 0x00, 0x00, 0x00, // MOVDQA XMM1, [ciphertext]
        0x48, 0xC7, 0xC1, 0x00, 0x50, 0x00, 0x00, // MOV RCX, 0x5000 (handle)
        0xF3, 0x0F, 0x38, 0xDD, 0x09, // AESDEC128KL XMM1, [RCX]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_aesdec128kl_xmm_high_regs() {
    // AESDEC128KL with XMM8-XMM15
    let code = [
        0x66, 0x45, 0x0F, 0xEF, 0xF6, // PXOR XMM14, XMM14
        0x49, 0xC7, 0xC1, 0x00, 0x60, 0x00, 0x00, // MOV R9, 0x6000
        0xF3, 0x47, 0x0F, 0x38, 0xDD, 0x31, // AESDEC128KL XMM14, [R9]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// AESENC256KL Tests - AES Encrypt with 256-bit Key Locker
// ============================================================================

#[test]
fn test_aesenc256kl_basic() {
    // AESENC256KL - AES encrypt with 256-bit key locker
    // Opcode: F3 0F 38 DE /r
    let code = [
        0x66, 0x0F, 0xEF, 0xC0, // PXOR XMM0, XMM0
        0x48, 0xC7, 0xC3, 0x00, 0x70, 0x00, 0x00, // MOV RBX, 0x7000 (handle)
        0xF3, 0x0F, 0x38, 0xDE, 0x03, // AESENC256KL XMM0, [RBX]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_aesenc256kl_with_data() {
    // AESENC256KL encrypting data
    let code = [
        0x66, 0x0F, 0x6F, 0x15, 0x00, 0x00, 0x00, 0x00, // MOVDQA XMM2, [plaintext]
        0x48, 0xC7, 0xC1, 0x00, 0x80, 0x00, 0x00, // MOV RCX, 0x8000
        0xF3, 0x0F, 0x38, 0xDE, 0x11, // AESENC256KL XMM2, [RCX]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// AESDEC256KL Tests - AES Decrypt with 256-bit Key Locker
// ============================================================================

#[test]
fn test_aesdec256kl_basic() {
    // AESDEC256KL - AES decrypt with 256-bit key locker
    // Opcode: F3 0F 38 DF /r
    let code = [
        0x66, 0x0F, 0xEF, 0xC0, // PXOR XMM0, XMM0
        0x48, 0xC7, 0xC3, 0x00, 0x90, 0x00, 0x00, // MOV RBX, 0x9000 (handle)
        0xF3, 0x0F, 0x38, 0xDF, 0x03, // AESDEC256KL XMM0, [RBX]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_aesdec256kl_with_data() {
    // AESDEC256KL decrypting ciphertext
    let code = [
        0x66, 0x0F, 0x6F, 0x1D, 0x00, 0x00, 0x00, 0x00, // MOVDQA XMM3, [ciphertext]
        0x48, 0xC7, 0xC2, 0x00, 0xA0, 0x00, 0x00, // MOV RDX, 0xA000
        0xF3, 0x0F, 0x38, 0xDF, 0x1A, // AESDEC256KL XMM3, [RDX]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// AESENCWIDE128KL Tests - Wide AES Encrypt with 128-bit Key Locker
// ============================================================================

#[test]
fn test_aesencwide128kl_basic() {
    // AESENCWIDE128KL - Wide AES encrypt (8 blocks)
    // Opcode: F3 0F 38 D8 /r
    // Encrypts XMM0-XMM7 in parallel
    let code = [
        0x66, 0x0F, 0xEF, 0xC0, // PXOR XMM0, XMM0
        0x66, 0x0F, 0xEF, 0xC9, // PXOR XMM1, XMM1
        0x66, 0x0F, 0xEF, 0xD2, // PXOR XMM2, XMM2
        0x66, 0x0F, 0xEF, 0xDB, // PXOR XMM3, XMM3
        0x66, 0x0F, 0xEF, 0xE4, // PXOR XMM4, XMM4
        0x66, 0x0F, 0xEF, 0xED, // PXOR XMM5, XMM5
        0x66, 0x0F, 0xEF, 0xF6, // PXOR XMM6, XMM6
        0x66, 0x0F, 0xEF, 0xFF, // PXOR XMM7, XMM7
        0x48, 0xC7, 0xC3, 0x00, 0xB0, 0x00, 0x00, // MOV RBX, 0xB000 (handle)
        0xF3, 0x0F, 0x38, 0xD8, 0x03, // AESENCWIDE128KL [RBX]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_aesencwide128kl_multiple_blocks() {
    // AESENCWIDE128KL with actual data in all XMM registers
    let code = [
        // Load data into XMM0-XMM7
        0x66, 0x0F, 0x6F, 0x05, 0x00, 0x00, 0x00, 0x00, // MOVDQA XMM0, [block0]
        0x66, 0x0F, 0x6F, 0x0D, 0x00, 0x00, 0x00, 0x00, // MOVDQA XMM1, [block1]
        0x66, 0x0F, 0x6F, 0x15, 0x00, 0x00, 0x00, 0x00, // MOVDQA XMM2, [block2]
        0x66, 0x0F, 0x6F, 0x1D, 0x00, 0x00, 0x00, 0x00, // MOVDQA XMM3, [block3]
        0x66, 0x0F, 0x6F, 0x25, 0x00, 0x00, 0x00, 0x00, // MOVDQA XMM4, [block4]
        0x66, 0x0F, 0x6F, 0x2D, 0x00, 0x00, 0x00, 0x00, // MOVDQA XMM5, [block5]
        0x66, 0x0F, 0x6F, 0x35, 0x00, 0x00, 0x00, 0x00, // MOVDQA XMM6, [block6]
        0x66, 0x0F, 0x6F, 0x3D, 0x00, 0x00, 0x00, 0x00, // MOVDQA XMM7, [block7]
        0x48, 0xC7, 0xC1, 0x00, 0xC0, 0x00, 0x00, // MOV RCX, 0xC000
        0xF3, 0x0F, 0x38, 0xD8, 0x01, // AESENCWIDE128KL [RCX]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// AESDECWIDE128KL Tests - Wide AES Decrypt with 128-bit Key Locker
// ============================================================================

#[test]
fn test_aesdecwide128kl_basic() {
    // AESDECWIDE128KL - Wide AES decrypt (8 blocks)
    // Opcode: F3 0F 38 D8 /r (with different ModRM)
    let code = [
        0x66, 0x0F, 0xEF, 0xC0, // PXOR XMM0, XMM0
        0x66, 0x0F, 0xEF, 0xC9, // PXOR XMM1, XMM1
        0x66, 0x0F, 0xEF, 0xD2, // PXOR XMM2, XMM2
        0x66, 0x0F, 0xEF, 0xDB, // PXOR XMM3, XMM3
        0x66, 0x0F, 0xEF, 0xE4, // PXOR XMM4, XMM4
        0x66, 0x0F, 0xEF, 0xED, // PXOR XMM5, XMM5
        0x66, 0x0F, 0xEF, 0xF6, // PXOR XMM6, XMM6
        0x66, 0x0F, 0xEF, 0xFF, // PXOR XMM7, XMM7
        0x48, 0xC7, 0xC3, 0x00, 0xD0, 0x00, 0x00, // MOV RBX, 0xD000 (handle)
        0xF3, 0x0F, 0x38, 0xD8, 0x0B, // AESDECWIDE128KL [RBX]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// AESENCWIDE256KL Tests - Wide AES Encrypt with 256-bit Key Locker
// ============================================================================

#[test]
fn test_aesencwide256kl_basic() {
    // AESENCWIDE256KL - Wide AES encrypt with 256-bit key
    // Opcode: F3 0F 38 D8 /r
    let code = [
        0x66, 0x0F, 0xEF, 0xC0, // PXOR XMM0, XMM0
        0x66, 0x0F, 0xEF, 0xC9, // PXOR XMM1, XMM1
        0x66, 0x0F, 0xEF, 0xD2, // PXOR XMM2, XMM2
        0x66, 0x0F, 0xEF, 0xDB, // PXOR XMM3, XMM3
        0x66, 0x0F, 0xEF, 0xE4, // PXOR XMM4, XMM4
        0x66, 0x0F, 0xEF, 0xED, // PXOR XMM5, XMM5
        0x66, 0x0F, 0xEF, 0xF6, // PXOR XMM6, XMM6
        0x66, 0x0F, 0xEF, 0xFF, // PXOR XMM7, XMM7
        0x48, 0xC7, 0xC2, 0x00, 0xE0, 0x00, 0x00, // MOV RDX, 0xE000 (handle)
        0xF3, 0x0F, 0x38, 0xD8, 0x12, // AESENCWIDE256KL [RDX]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// AESDECWIDE256KL Tests - Wide AES Decrypt with 256-bit Key Locker
// ============================================================================

#[test]
fn test_aesdecwide256kl_basic() {
    // AESDECWIDE256KL - Wide AES decrypt with 256-bit key
    // Opcode: F3 0F 38 D8 /r
    let code = [
        0x66, 0x0F, 0xEF, 0xC0, // PXOR XMM0, XMM0
        0x66, 0x0F, 0xEF, 0xC9, // PXOR XMM1, XMM1
        0x66, 0x0F, 0xEF, 0xD2, // PXOR XMM2, XMM2
        0x66, 0x0F, 0xEF, 0xDB, // PXOR XMM3, XMM3
        0x66, 0x0F, 0xEF, 0xE4, // PXOR XMM4, XMM4
        0x66, 0x0F, 0xEF, 0xED, // PXOR XMM5, XMM5
        0x66, 0x0F, 0xEF, 0xF6, // PXOR XMM6, XMM6
        0x66, 0x0F, 0xEF, 0xFF, // PXOR XMM7, XMM7
        0x48, 0xC7, 0xC1, 0x00, 0xF0, 0x00, 0x00, // MOV RCX, 0xF000 (handle)
        0xF3, 0x0F, 0x38, 0xD8, 0x19, // AESDECWIDE256KL [RCX]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// Combined Tests
// ============================================================================

#[test]
fn test_kl_full_workflow() {
    // Full Key Locker workflow: LOADIWKEY -> ENCODEKEY128 -> AESENC128KL
    let code = [
        // Step 1: Load internal wrapping key
        0x66, 0x0F, 0xEF, 0xC0, // PXOR XMM0, XMM0
        0x48, 0x31, 0xC0, // XOR RAX, RAX
        0x66, 0x0F, 0xEF, 0xC9, // PXOR XMM1, XMM1
        0x66, 0x0F, 0xEF, 0xD2, // PXOR XMM2, XMM2
        0xF3, 0x0F, 0x38, 0xDC, 0xCA, // LOADIWKEY XMM1, XMM2
        // Step 2: Encode key into handle
        0x48, 0x31, 0xC0, // XOR RAX, RAX
        0x66, 0x0F, 0xEF, 0xDB, // PXOR XMM3, XMM3
        0x48, 0xC7, 0xC3, 0x00, 0x10, 0x00, 0x00, // MOV RBX, 0x1000
        0xF3, 0x0F, 0x38, 0xFA, 0x1B, // ENCODEKEY128 XMM3, [RBX]
        // Step 3: Encrypt data
        0x66, 0x0F, 0xEF, 0xE4, // PXOR XMM4, XMM4
        0xF3, 0x0F, 0x38, 0xDC, 0x23, // AESENC128KL XMM4, [RBX]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_kl_256bit_workflow() {
    // Key Locker with 256-bit keys
    let code = [
        // Load wrapping key
        0x66, 0x0F, 0xEF, 0xC0, // PXOR XMM0, XMM0
        0x48, 0x31, 0xC0, // XOR RAX, RAX
        0x66, 0x0F, 0xEF, 0xC9, // PXOR XMM1, XMM1
        0x66, 0x0F, 0xEF, 0xD2, // PXOR XMM2, XMM2
        0xF3, 0x0F, 0x38, 0xDC, 0xCA, // LOADIWKEY XMM1, XMM2
        // Encode 256-bit key
        0x48, 0x31, 0xC0, // XOR RAX, RAX
        0x66, 0x0F, 0xEF, 0xDB, // PXOR XMM3, XMM3
        0x48, 0xC7, 0xC1, 0x00, 0x20, 0x00, 0x00, // MOV RCX, 0x2000
        0xF3, 0x0F, 0x38, 0xFB, 0x19, // ENCODEKEY256 XMM3, [RCX]
        // Encrypt with 256-bit key
        0x66, 0x0F, 0xEF, 0xE4, // PXOR XMM4, XMM4
        0xF3, 0x0F, 0x38, 0xDE, 0x21, // AESENC256KL XMM4, [RCX]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}
