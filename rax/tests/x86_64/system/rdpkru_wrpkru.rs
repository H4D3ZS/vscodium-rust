//! Tests for the RDPKRU and WRPKRU instructions.
//!
//! RDPKRU - Read Protection Key Rights for User Pages
//! WRPKRU - Write Data to User Page Key Register
//!
//! These instructions read and write the Protection Key Rights for User Pages
//! (PKRU) register, which controls memory protection keys. Protection keys
//! provide page-level protection independent of traditional paging protections.
//!
//! RDPKRU:
//!   - Opcode: 0F 01 EE
//!   - Reads PKRU into EAX, clears EDX
//!   - Requires ECX = 0 or causes #GP
//!   - Requires CR4.PKE = 1 (OSPKE feature)
//!
//! WRPKRU:
//!   - Opcode: 0F 01 EF
//!   - Writes EAX into PKRU
//!   - Requires ECX = 0 and EDX = 0 or causes #GP
//!   - Requires CR4.PKE = 1 (OSPKE feature)
//!   - Never executes speculatively
//!
//! Reference: docs/rdpkru.txt, docs/wrpkru.txt

use crate::common::*;
use rax::cpu::Registers;

// ============================================================================
// WRPKRU - Write PKRU Tests
// ============================================================================

#[test]
fn test_wrpkru_basic() {
    // WRPKRU - Write value from EAX to PKRU
    // Requires ECX = 0, EDX = 0
    let code = [
        0x48, 0xC7, 0xC0, 0x55, 0x55, 0x55, 0x55, // MOV RAX, 0x55555555
        0x48, 0x31, 0xC9, // XOR RCX, RCX (ECX = 0)
        0x48, 0x31, 0xD2, // XOR RDX, RDX (EDX = 0)
        0x0F, 0x01, 0xEF, // WRPKRU
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    // PKRU should be set to 0x55555555
    // RAX, RCX, RDX should be unchanged
    assert_eq!(regs.rax, 0x55555555);
    assert_eq!(regs.rcx, 0);
    assert_eq!(regs.rdx, 0);
}

#[test]
fn test_wrpkru_various_values() {
    // Test writing various PKRU values
    let test_values = [
        0x00000000u32,
        0xFFFFFFFFu32,
        0x55555555u32,
        0xAAAAAAAAu32,
        0x12345678u32,
    ];

    for &value in &test_values {
        let code = [
            0x48,
            0xC7,
            0xC0,
            (value & 0xFF) as u8,
            ((value >> 8) & 0xFF) as u8,
            ((value >> 16) & 0xFF) as u8,
            ((value >> 24) & 0xFF) as u8, // MOV RAX, value
            0x48,
            0x31,
            0xC9, // XOR RCX, RCX
            0x48,
            0x31,
            0xD2, // XOR RDX, RDX
            0x0F,
            0x01,
            0xEF, // WRPKRU
            0xF4, // HLT
        ];
        let (mut vcpu, _) = setup_vm(&code, None);

        let _regs = run_until_hlt(&mut vcpu).unwrap();
        // PKRU should be set to value
    }
}

#[test]
fn test_wrpkru_ignores_upper_32_bits() {
    // WRPKRU should ignore upper 32 bits of RAX
    let code = [
        0x48, 0xB8, 0x78, 0x56, 0x34, 0x12, 0xFF, 0xFF, 0xFF,
        0xFF, // MOV RAX, 0xFFFFFFFF12345678
        0x48, 0x31, 0xC9, // XOR RCX, RCX
        0x48, 0x31, 0xD2, // XOR RDX, RDX
        0x0F, 0x01, 0xEF, // WRPKRU
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let _regs = run_until_hlt(&mut vcpu).unwrap();

    // PKRU should be set to 0x12345678 (lower 32 bits only)
}

#[test]
fn test_wrpkru_preserves_registers() {
    // WRPKRU should not modify RAX, RCX, RDX (beyond what they're set to)
    let code = [
        0x48, 0xC7, 0xC0, 0x11, 0x11, 0x11, 0x11, // MOV RAX, 0x11111111
        0x48, 0x31, 0xC9, // XOR RCX, RCX
        0x48, 0x31, 0xD2, // XOR RDX, RDX
        0x0F, 0x01, 0xEF, // WRPKRU
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x11111111, "RAX should be unchanged");
    assert_eq!(regs.rcx, 0, "RCX should be 0");
    assert_eq!(regs.rdx, 0, "RDX should be 0");
}

#[test]
fn test_wrpkru_preserves_flags() {
    // WRPKRU should not modify flags
    let code = [
        // Set some flags
        0x48, 0xC7, 0xC0, 0xFF, 0xFF, 0xFF, 0xFF, // MOV RAX, -1
        0x48, 0x83, 0xC0, 0x01, // ADD RAX, 1 (sets ZF)
        // Save flags
        0x9C, // PUSHFQ
        0x5B, // POP RBX
        // Write PKRU
        0x48, 0xC7, 0xC0, 0xAA, 0xAA, 0xAA, 0xAA, // MOV RAX, 0xAAAAAAAA
        0xB9, 0x00, 0x00, 0x00, 0x00, // MOV ECX, 0
        0xBA, 0x00, 0x00, 0x00, 0x00, // MOV EDX, 0
        0x0F, 0x01, 0xEF, // WRPKRU
        // Check flags
        0x9C, // PUSHFQ
        0x5E, // POP RSI
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rbx & 0xCD5,
        regs.rsi & 0xCD5,
        "WRPKRU should preserve flags"
    );
}

#[test]
fn test_wrpkru_sequential_writes() {
    // Multiple sequential WRPKRU operations
    let code = [
        // Write 0x11111111
        0x48, 0xC7, 0xC0, 0x11, 0x11, 0x11, 0x11, // MOV RAX, 0x11111111
        0x48, 0x31, 0xC9, // XOR RCX, RCX
        0x48, 0x31, 0xD2, // XOR RDX, RDX
        0x0F, 0x01, 0xEF, // WRPKRU
        // Write 0x22222222
        0x48, 0xC7, 0xC0, 0x22, 0x22, 0x22, 0x22, // MOV RAX, 0x22222222
        0x0F, 0x01, 0xEF, // WRPKRU (RCX, RDX still 0)
        // Write 0x33333333
        0x48, 0xC7, 0xC0, 0x33, 0x33, 0x33, 0x33, // MOV RAX, 0x33333333
        0x0F, 0x01, 0xEF, // WRPKRU
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let _regs = run_until_hlt(&mut vcpu).unwrap();

    // Final PKRU value should be 0x33333333
}

// ============================================================================
// RDPKRU - Read PKRU Tests
// ============================================================================

#[test]
fn test_rdpkru_basic() {
    // RDPKRU - Read PKRU into EAX, clear EDX
    // Requires ECX = 0
    let code = [
        0x48, 0x31, 0xC9, // XOR RCX, RCX (ECX = 0)
        0x0F, 0x01, 0xEE, // RDPKRU
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    // EDX should be cleared (upper 32 bits of RDX also cleared)
    assert_eq!(regs.rdx, 0, "RDX should be cleared by RDPKRU");
    assert_eq!(regs.rcx, 0, "RCX should remain 0");
}

#[test]
fn test_rdpkru_clears_upper_bits() {
    // RDPKRU should clear upper 32 bits of RAX and RDX
    let code = [
        // Set RAX and RDX to all 1s
        0x48, 0xC7, 0xC0, 0xFF, 0xFF, 0xFF, 0xFF, // MOV RAX, -1
        0x48, 0xC7, 0xC2, 0xFF, 0xFF, 0xFF, 0xFF, // MOV RDX, -1
        // Read PKRU
        0xB9, 0x00, 0x00, 0x00, 0x00, // MOV ECX, 0
        0x0F, 0x01, 0xEE, // RDPKRU
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Upper 32 bits should be cleared
    assert_eq!(regs.rax >> 32, 0, "Upper 32 bits of RAX should be cleared");
    assert_eq!(regs.rdx, 0, "RDX should be completely cleared");
}

#[test]
fn test_rdpkru_preserves_flags() {
    // RDPKRU should not modify flags
    let code = [
        // Set some flags
        0x48, 0xC7, 0xC0, 0xFF, 0xFF, 0xFF, 0xFF, // MOV RAX, -1
        0x48, 0x83, 0xC0, 0x01, // ADD RAX, 1 (sets ZF)
        // Save flags
        0x9C, // PUSHFQ
        0x5B, // POP RBX
        // Read PKRU
        0xB9, 0x00, 0x00, 0x00, 0x00, // MOV ECX, 0
        0x0F, 0x01, 0xEE, // RDPKRU
        // Check flags
        0x9C, // PUSHFQ
        0x5E, // POP RSI
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rbx & 0xCD5,
        regs.rsi & 0xCD5,
        "RDPKRU should preserve flags"
    );
}

#[test]
fn test_rdpkru_multiple_reads() {
    // Multiple RDPKRU operations should return same value
    let code = [
        0x48, 0x31, 0xC9, // XOR RCX, RCX
        0x0F, 0x01, 0xEE, // RDPKRU
        0x48, 0x89, 0xC3, // MOV RBX, RAX (save first read)
        0x0F, 0x01, 0xEE, // RDPKRU (read again)
        0x49, 0x89, 0xC0, // MOV R8, RAX (save second read)
        0x0F, 0x01, 0xEE, // RDPKRU (read third time)
        0x49, 0x89, 0xC1, // MOV R9, RAX (save third read)
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rbx, regs.r8,
        "Multiple RDPKRUs should return same value"
    );
    assert_eq!(
        regs.r8, regs.r9,
        "Multiple RDPKRUs should return same value"
    );
}

// ============================================================================
// Combined RDPKRU/WRPKRU Tests
// ============================================================================

#[test]
fn test_write_then_read_pkru() {
    // Write a value with WRPKRU, then read it back with RDPKRU
    let code = [
        // Write PKRU
        0x48, 0xC7, 0xC0, 0x12, 0x34, 0x56, 0x78, // MOV RAX, 0x78563412
        0x48, 0x31, 0xC9, // XOR RCX, RCX
        0x48, 0x31, 0xD2, // XOR RDX, RDX
        0x0F, 0x01, 0xEF, // WRPKRU
        // Read it back
        0x48, 0x31, 0xC0, // XOR RAX, RAX (clear RAX)
        0x0F, 0x01, 0xEE, // RDPKRU
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Should read back the value we wrote
    assert_eq!(
        regs.rax, 0x78563412,
        "RDPKRU should read back written value"
    );
    assert_eq!(regs.rdx, 0, "RDX should be cleared");
}

#[test]
fn test_pkru_write_read_cycle() {
    // Test write-read cycles with various values
    let test_values = [0x00000000u32, 0x55555555u32, 0xAAAAAAAAu32, 0xFFFFFFFFu32];

    for &value in &test_values {
        let code = [
            // Write
            0x48,
            0xC7,
            0xC0,
            (value & 0xFF) as u8,
            ((value >> 8) & 0xFF) as u8,
            ((value >> 16) & 0xFF) as u8,
            ((value >> 24) & 0xFF) as u8, // MOV RAX, value
            0x48,
            0x31,
            0xC9, // XOR RCX, RCX
            0x48,
            0x31,
            0xD2, // XOR RDX, RDX
            0x0F,
            0x01,
            0xEF, // WRPKRU
            // Read
            0x48,
            0x31,
            0xC0, // XOR RAX, RAX
            0x0F,
            0x01,
            0xEE, // RDPKRU
            0xF4, // HLT
        ];
        let (mut vcpu, _) = setup_vm(&code, None);

        let regs = run_until_hlt(&mut vcpu).unwrap();

        assert_eq!(
            regs.rax, value as u64,
            "RDPKRU should read back 0x{:08X}",
            value
        );
    }
}

#[test]
fn test_pkru_alternating_write_read() {
    // Alternate writes and reads
    let code = [
        // Write 0x11111111
        0x48, 0xC7, 0xC0, 0x11, 0x11, 0x11, 0x11, // MOV RAX, 0x11111111
        0x48, 0x31, 0xC9, // XOR RCX, RCX
        0x48, 0x31, 0xD2, // XOR RDX, RDX
        0x0F, 0x01, 0xEF, // WRPKRU
        // Read it
        0x48, 0x31, 0xC0, // XOR RAX, RAX
        0x0F, 0x01, 0xEE, // RDPKRU
        0x48, 0x89, 0xC3, // MOV RBX, RAX (save)
        // Write 0x22222222
        0x48, 0xC7, 0xC0, 0x22, 0x22, 0x22, 0x22, // MOV RAX, 0x22222222
        0x0F, 0x01, 0xEF, // WRPKRU
        // Read it
        0x48, 0x31, 0xC0, // XOR RAX, RAX
        0x0F, 0x01, 0xEE, // RDPKRU
        0x49, 0x89, 0xC0, // MOV R8, RAX (save)
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx, 0x11111111, "First read should be 0x11111111");
    assert_eq!(regs.r8, 0x22222222, "Second read should be 0x22222222");
}

#[test]
fn test_pkru_persistence_across_operations() {
    // PKRU value should persist across other operations
    let code = [
        // Write PKRU
        0x48, 0xC7, 0xC0, 0xAB, 0xCD, 0xEF, 0x01, // MOV RAX, 0x01EFCDAB
        0x48, 0x31, 0xC9, // XOR RCX, RCX
        0x48, 0x31, 0xD2, // XOR RDX, RDX
        0x0F, 0x01, 0xEF, // WRPKRU
        // Do some other operations
        0x48, 0xC7, 0xC0, 0x42, 0x00, 0x00, 0x00, // MOV RAX, 0x42
        0x48, 0xFF, 0xC0, // INC RAX
        0x48, 0x01, 0xC3, // ADD RBX, RAX
        // Read PKRU - should still be 0x01EFCDAB
        0x48, 0x31, 0xC0, // XOR RAX, RAX
        0x0F, 0x01, 0xEE, // RDPKRU
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax, 0x01EFCDAB,
        "PKRU should persist across operations"
    );
}

#[test]
fn test_pkru_zero_state() {
    // Test PKRU in zero state (default/reset)
    let code = [
        // Write 0 to PKRU
        0x48, 0x31, 0xC0, // XOR RAX, RAX
        0x48, 0x31, 0xC9, // XOR RCX, RCX
        0x48, 0x31, 0xD2, // XOR RDX, RDX
        0x0F, 0x01, 0xEF, // WRPKRU
        // Read it back
        0x0F, 0x01, 0xEE, // RDPKRU
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0, "PKRU should be 0 after writing 0");
    assert_eq!(regs.rdx, 0, "RDX should be 0");
}

#[test]
fn test_pkru_all_bits_set() {
    // Test PKRU with all bits set
    let code = [
        // Write all 1s to PKRU
        0x48, 0xC7, 0xC0, 0xFF, 0xFF, 0xFF, 0xFF, // MOV RAX, -1
        0x48, 0x31, 0xC9, // XOR RCX, RCX
        0x48, 0x31, 0xD2, // XOR RDX, RDX
        0x0F, 0x01, 0xEF, // WRPKRU
        // Read it back
        0x48, 0x31, 0xC0, // XOR RAX, RAX
        0x0F, 0x01, 0xEE, // RDPKRU
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0xFFFFFFFF, "PKRU should be all 1s");
}

#[test]
fn test_wrpkru_rdpkru_stress() {
    // Stress test with rapid write/read cycles
    let mut code = vec![];

    // Setup
    code.extend_from_slice(&[0x48, 0x31, 0xC9]); // XOR RCX, RCX
    code.extend_from_slice(&[0x48, 0x31, 0xD2]); // XOR RDX, RDX

    // Perform 10 write/read cycles with different values
    for i in 0..10 {
        let value = (i as u32) * 0x1111_1111;

        // Write
        code.extend_from_slice(&[0x48, 0xC7, 0xC0]);
        code.push((value & 0xFF) as u8);
        code.push(((value >> 8) & 0xFF) as u8);
        code.push(((value >> 16) & 0xFF) as u8);
        code.push(((value >> 24) & 0xFF) as u8);
        code.extend_from_slice(&[0x0F, 0x01, 0xEF]); // WRPKRU

        // Read
        code.extend_from_slice(&[0x48, 0x31, 0xC0]); // XOR RAX, RAX
        code.extend_from_slice(&[0x0F, 0x01, 0xEE]); // RDPKRU
    }

    code.push(0xF4); // HLT

    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Last value should be 9 * 0x11111111 = 0x9999999
    let expected = ((9 * 0x11111111u64) & 0xFFFFFFFF) as u64;
    assert_eq!(
        regs.rax, expected,
        "Final PKRU value should match last write"
    );
}

#[test]
fn test_pkru_bit_patterns() {
    // Test specific bit patterns in PKRU
    let patterns = [
        0x00000001u32, // Single bit
        0x00000003u32, // Two bits
        0x00000005u32, // Alternating in low nibble
        0x0000000Fu32, // Low nibble
        0x000000FFu32, // Low byte
        0x0000FFFFu32, // Low word
        0x00FF00FFu32, // Alternating bytes
        0xFF00FF00u32, // Alternating bytes (inverse)
    ];

    for &pattern in &patterns {
        let code = [
            0x48,
            0xC7,
            0xC0,
            (pattern & 0xFF) as u8,
            ((pattern >> 8) & 0xFF) as u8,
            ((pattern >> 16) & 0xFF) as u8,
            ((pattern >> 24) & 0xFF) as u8, // MOV RAX, pattern
            0x48,
            0x31,
            0xC9, // XOR RCX, RCX
            0x48,
            0x31,
            0xD2, // XOR RDX, RDX
            0x0F,
            0x01,
            0xEF, // WRPKRU
            0x48,
            0x31,
            0xC0, // XOR RAX, RAX
            0x0F,
            0x01,
            0xEE, // RDPKRU
            0xF4, // HLT
        ];
        let (mut vcpu, _) = setup_vm(&code, None);

        let regs = run_until_hlt(&mut vcpu).unwrap();

        assert_eq!(
            regs.rax, pattern as u64,
            "PKRU should correctly store/retrieve pattern 0x{:08X}",
            pattern
        );
    }
}
