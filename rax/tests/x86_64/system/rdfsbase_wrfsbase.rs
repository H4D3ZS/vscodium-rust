//! Tests for the RDFSBASE, RDGSBASE, WRFSBASE, and WRGSBASE instructions.
//!
//! RDFSBASE/RDGSBASE - Read FS/GS Segment Base
//! WRFSBASE/WRGSBASE - Write FS/GS Segment Base
//!
//! These instructions read and write the base address of the FS and GS
//! segment registers. They are only available in 64-bit mode and require
//! the FSGSBASE CPU feature.
//!
//! RDFSBASE r32/r64: Read FS base into 32-bit or 64-bit register
//!   - Opcode: F3 0F AE /0 (32-bit), F3 REX.W 0F AE /0 (64-bit)
//!
//! RDGSBASE r32/r64: Read GS base into 32-bit or 64-bit register
//!   - Opcode: F3 0F AE /1 (32-bit), F3 REX.W 0F AE /1 (64-bit)
//!
//! WRFSBASE r32/r64: Write FS base from 32-bit or 64-bit register
//!   - Opcode: F3 0F AE /2 (32-bit), F3 REX.W 0F AE /2 (64-bit)
//!
//! WRGSBASE r32/r64: Write GS base from 32-bit or 64-bit register
//!   - Opcode: F3 0F AE /3 (32-bit), F3 REX.W 0F AE /3 (64-bit)
//!
//! Note: For 32-bit operations, upper 32 bits are cleared.
//!
//! Reference: docs/rdfsbase:rdgsbase.txt, docs/wrfsbase:wrgsbase.txt

use crate::common::*;
use rax::cpu::Registers;

// ============================================================================
// WRFSBASE - Write FS Base Tests
// ============================================================================

#[test]
fn test_wrfsbase_64bit() {
    // WRFSBASE with 64-bit operand
    // F3 REX.W 0F AE /2
    let code = [
        0x48, 0xB8, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 0x100000000
        0xF3, 0x48, 0x0F, 0xAE, 0xD0, // WRFSBASE RAX
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let _regs = run_until_hlt(&mut vcpu).unwrap();

    // FS base should now be 0x100000000
    // (This is a basic test - actual verification would require RDFSBASE)
}

#[test]
fn test_wrfsbase_32bit() {
    // WRFSBASE with 32-bit operand (no REX.W prefix)
    // F3 0F AE /2 - upper 32 bits of FS base should be cleared
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x10, 0x00, 0x00, // MOV RAX, 0x1000
        0xF3, 0x0F, 0xAE, 0xD0, // WRFSBASE EAX (32-bit)
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let _regs = run_until_hlt(&mut vcpu).unwrap();

    // FS base should be 0x00001000 (upper 32 bits cleared)
}

#[test]
fn test_wrfsbase_various_values() {
    // Test writing various values to FS base
    let test_values = [
        0x0000000000000000u64,
        0x0000000000001000u64,
        0x00007FFFFFFFE000u64,
        0xFFFFFFFF80000000u64,
        0xFFFFFFFFFFFFFFFFu64,
    ];

    for &value in &test_values {
        let code = [
            0x48,
            0xB8,
            (value & 0xFF) as u8,
            ((value >> 8) & 0xFF) as u8,
            ((value >> 16) & 0xFF) as u8,
            ((value >> 24) & 0xFF) as u8,
            ((value >> 32) & 0xFF) as u8,
            ((value >> 40) & 0xFF) as u8,
            ((value >> 48) & 0xFF) as u8,
            ((value >> 56) & 0xFF) as u8, // MOV RAX, value
            0xF3,
            0x48,
            0x0F,
            0xAE,
            0xD0, // WRFSBASE RAX
            0xF4, // HLT
        ];
        let (mut vcpu, _) = setup_vm(&code, None);

        let _regs = run_until_hlt(&mut vcpu).unwrap();
        // FS base should be set to value
    }
}

#[test]
fn test_wrfsbase_from_different_registers() {
    // Test writing from various registers
    let code = [
        // Write from RBX
        0x48, 0xC7, 0xC3, 0x00, 0x20, 0x00, 0x00, // MOV RBX, 0x2000
        0xF3, 0x48, 0x0F, 0xAE, 0xD3, // WRFSBASE RBX
        // Write from RCX
        0x48, 0xC7, 0xC1, 0x00, 0x30, 0x00, 0x00, // MOV RCX, 0x3000
        0xF3, 0x48, 0x0F, 0xAE, 0xD1, // WRFSBASE RCX
        // Write from RDX
        0x48, 0xC7, 0xC2, 0x00, 0x40, 0x00, 0x00, // MOV RDX, 0x4000
        0xF3, 0x48, 0x0F, 0xAE, 0xD2, // WRFSBASE RDX
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let _regs = run_until_hlt(&mut vcpu).unwrap();

    // Final FS base should be 0x4000
}

#[test]
fn test_wrfsbase_preserves_flags() {
    // WRFSBASE should not modify flags
    let code = [
        // Set some flags
        0x48, 0xC7, 0xC0, 0xFF, 0xFF, 0xFF, 0xFF, // MOV RAX, -1
        0x48, 0x83, 0xC0, 0x01, // ADD RAX, 1 (sets ZF)
        // Save flags
        0x9C, // PUSHFQ
        0x5B, // POP RBX
        // Write FS base
        0x48, 0xC7, 0xC0, 0x00, 0x10, 0x00, 0x00, // MOV RAX, 0x1000
        0xF3, 0x48, 0x0F, 0xAE, 0xD0, // WRFSBASE RAX
        // Check flags
        0x9C, // PUSHFQ
        0x59, // POP RCX
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Flags should be unchanged
    assert_eq!(
        regs.rbx & 0xCD5,
        regs.rcx & 0xCD5,
        "WRFSBASE should preserve flags"
    );
}

// ============================================================================
// WRGSBASE - Write GS Base Tests
// ============================================================================

#[test]
fn test_wrgsbase_64bit() {
    // WRGSBASE with 64-bit operand
    // F3 REX.W 0F AE /3
    let code = [
        0x48, 0xB8, 0x00, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, // MOV RAX, 0x200000000
        0xF3, 0x48, 0x0F, 0xAE, 0xD8, // WRGSBASE RAX
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let _regs = run_until_hlt(&mut vcpu).unwrap();

    // GS base should now be 0x200000000
}

#[test]
fn test_wrgsbase_32bit() {
    // WRGSBASE with 32-bit operand
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x50, 0x00, 0x00, // MOV RAX, 0x5000
        0xF3, 0x0F, 0xAE, 0xD8, // WRGSBASE EAX (32-bit)
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let _regs = run_until_hlt(&mut vcpu).unwrap();

    // GS base should be 0x00005000 (upper 32 bits cleared)
}

#[test]
fn test_wrgsbase_preserves_flags() {
    // WRGSBASE should not modify flags
    let code = [
        // Set flags
        0x48, 0xC7, 0xC0, 0xFF, 0xFF, 0xFF, 0xFF, // MOV RAX, -1
        0x48, 0x83, 0xC0, 0x01, // ADD RAX, 1 (sets ZF)
        // Save flags
        0x9C, // PUSHFQ
        0x5B, // POP RBX
        // Write GS base
        0x48, 0xC7, 0xC0, 0x00, 0x60, 0x00, 0x00, // MOV RAX, 0x6000
        0xF3, 0x48, 0x0F, 0xAE, 0xD8, // WRGSBASE RAX
        // Check flags
        0x9C, // PUSHFQ
        0x59, // POP RCX
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rbx & 0xCD5,
        regs.rcx & 0xCD5,
        "WRGSBASE should preserve flags"
    );
}

// ============================================================================
// RDFSBASE - Read FS Base Tests
// ============================================================================

#[test]
fn test_rdfsbase_64bit() {
    // RDFSBASE with 64-bit operand
    // F3 REX.W 0F AE /0
    let code = [
        // First set FS base
        0x48, 0xB8, 0x34, 0x12, 0x00, 0x00, 0x78, 0x56, 0x00, 0x00, // MOV RAX, 0x5678000 01234
        0xF3, 0x48, 0x0F, 0xAE, 0xD0, // WRFSBASE RAX
        // Now read it back
        0x48, 0x31, 0xDB, // XOR RBX, RBX (clear RBX)
        0xF3, 0x48, 0x0F, 0xAE, 0xC3, // RDFSBASE RBX
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    // RBX should contain the FS base value we wrote
    assert_eq!(
        regs.rbx, regs.rax,
        "RDFSBASE should read back written value"
    );
}

#[test]
fn test_rdfsbase_32bit_clears_upper_bits() {
    // RDFSBASE with 32-bit operand should clear upper 32 bits of destination
    let code = [
        // Set FS base to a 64-bit value
        0x48, 0xB8, 0x00, 0x00, 0x00, 0x80, 0x00, 0x00, 0x00, 0x00, // MOV RAX, 0x80000000
        0xF3, 0x48, 0x0F, 0xAE, 0xD0, // WRFSBASE RAX
        // Read with 32-bit RDFSBASE
        0x48, 0xC7, 0xC3, 0xFF, 0xFF, 0xFF, 0xFF, // MOV RBX, -1 (all bits set)
        0xF3, 0x0F, 0xAE, 0xC3, // RDFSBASE EBX (32-bit)
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Upper 32 bits of RBX should be cleared
    assert_eq!(
        regs.rbx >> 32,
        0,
        "32-bit RDFSBASE should clear upper 32 bits"
    );
    assert_eq!(
        regs.rbx & 0xFFFFFFFF,
        0x80000000,
        "Lower 32 bits should match"
    );
}

#[test]
fn test_rdfsbase_to_different_registers() {
    // Read FS base into various registers
    let code = [
        // Set FS base
        0x48, 0xC7, 0xC0, 0x00, 0x70, 0x00, 0x00, // MOV RAX, 0x7000
        0xF3, 0x48, 0x0F, 0xAE, 0xD0, // WRFSBASE RAX
        // Read into different registers
        0xF3, 0x48, 0x0F, 0xAE, 0xC3, // RDFSBASE RBX
        0xF3, 0x48, 0x0F, 0xAE, 0xC1, // RDFSBASE RCX
        0xF3, 0x48, 0x0F, 0xAE, 0xC2, // RDFSBASE RDX
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx, 0x7000, "RBX should contain FS base");
    assert_eq!(regs.rcx, 0x7000, "RCX should contain FS base");
    assert_eq!(regs.rdx, 0x7000, "RDX should contain FS base");
}

#[test]
fn test_rdfsbase_preserves_flags() {
    // RDFSBASE should not modify flags
    let code = [
        // Set FS base
        0x48, 0xC7, 0xC0, 0x00, 0x10, 0x00, 0x00, // MOV RAX, 0x1000
        0xF3, 0x48, 0x0F, 0xAE, 0xD0, // WRFSBASE RAX
        // Set some flags
        0x48, 0xC7, 0xC0, 0xFF, 0xFF, 0xFF, 0xFF, // MOV RAX, -1
        0x48, 0x83, 0xC0, 0x01, // ADD RAX, 1 (sets ZF)
        // Save flags
        0x9C, // PUSHFQ
        0x5B, // POP RBX
        // Read FS base
        0xF3, 0x48, 0x0F, 0xAE, 0xC0, // RDFSBASE RAX
        // Check flags
        0x9C, // PUSHFQ
        0x59, // POP RCX
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rbx & 0xCD5,
        regs.rcx & 0xCD5,
        "RDFSBASE should preserve flags"
    );
    assert_eq!(regs.rax, 0x1000, "RDFSBASE should read correct value");
}

// ============================================================================
// RDGSBASE - Read GS Base Tests
// ============================================================================

#[test]
fn test_rdgsbase_64bit() {
    // RDGSBASE with 64-bit operand
    // F3 REX.W 0F AE /1
    let code = [
        // Set GS base
        0x48, 0xB8, 0x00, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV RAX, 0x8000
        0xF3, 0x48, 0x0F, 0xAE, 0xD8, // WRGSBASE RAX
        // Read it back
        0x48, 0x31, 0xDB, // XOR RBX, RBX
        0xF3, 0x48, 0x0F, 0xAE, 0xCB, // RDGSBASE RBX
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx, 0x8000, "RDGSBASE should read back written value");
}

#[test]
fn test_rdgsbase_32bit_clears_upper_bits() {
    // RDGSBASE with 32-bit operand
    let code = [
        // Set GS base
        0x48, 0xB8, 0xFF, 0xFF, 0xFF, 0x7F, 0x00, 0x00, 0x00, 0x00, // MOV RAX, 0x7FFFFFFF
        0xF3, 0x48, 0x0F, 0xAE, 0xD8, // WRGSBASE RAX
        // Read with 32-bit operand
        0x48, 0xC7, 0xC3, 0xFF, 0xFF, 0xFF, 0xFF, // MOV RBX, -1
        0xF3, 0x0F, 0xAE, 0xCB, // RDGSBASE EBX (32-bit)
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rbx >> 32,
        0,
        "32-bit RDGSBASE should clear upper 32 bits"
    );
    assert_eq!(
        regs.rbx & 0xFFFFFFFF,
        0x7FFFFFFF,
        "Lower 32 bits should match"
    );
}

#[test]
fn test_rdgsbase_preserves_flags() {
    // RDGSBASE should not modify flags
    let code = [
        // Set GS base
        0x48, 0xC7, 0xC0, 0x00, 0x90, 0x00, 0x00, // MOV RAX, 0x9000
        0xF3, 0x48, 0x0F, 0xAE, 0xD8, // WRGSBASE RAX
        // Set flags
        0x48, 0xC7, 0xC0, 0xFF, 0xFF, 0xFF, 0xFF, // MOV RAX, -1
        0x48, 0x83, 0xC0, 0x01, // ADD RAX, 1 (sets ZF)
        // Save flags
        0x9C, // PUSHFQ
        0x5B, // POP RBX
        // Read GS base
        0xF3, 0x48, 0x0F, 0xAE, 0xC8, // RDGSBASE RAX
        // Check flags
        0x9C, // PUSHFQ
        0x59, // POP RCX
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rbx & 0xCD5,
        regs.rcx & 0xCD5,
        "RDGSBASE should preserve flags"
    );
    assert_eq!(regs.rax, 0x9000, "RDGSBASE should read correct value");
}

// ============================================================================
// Combined FS and GS Tests
// ============================================================================

#[test]
fn test_fs_gs_independent() {
    // Verify FS and GS bases are independent
    let code = [
        // Set FS base
        0x48, 0xC7, 0xC0, 0x00, 0x10, 0x00, 0x00, // MOV RAX, 0x1000
        0xF3, 0x48, 0x0F, 0xAE, 0xD0, // WRFSBASE RAX
        // Set GS base
        0x48, 0xC7, 0xC0, 0x00, 0x20, 0x00, 0x00, // MOV RAX, 0x2000
        0xF3, 0x48, 0x0F, 0xAE, 0xD8, // WRGSBASE RAX
        // Read FS base
        0xF3, 0x48, 0x0F, 0xAE, 0xC3, // RDFSBASE RBX
        // Read GS base
        0xF3, 0x48, 0x0F, 0xAE, 0xC9, // RDGSBASE RCX
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx, 0x1000, "FS base should be 0x1000");
    assert_eq!(regs.rcx, 0x2000, "GS base should be 0x2000");
    assert_ne!(regs.rbx, regs.rcx, "FS and GS bases should be independent");
}

#[test]
fn test_write_read_cycle_fs() {
    // Test write-read cycle for FS base with multiple values
    let test_values = [
        0x0000000000001000u64,
        0x0000000000002000u64,
        0x00007FFFFFFF0000u64,
    ];

    for &value in &test_values {
        let code = [
            0x48,
            0xB8,
            (value & 0xFF) as u8,
            ((value >> 8) & 0xFF) as u8,
            ((value >> 16) & 0xFF) as u8,
            ((value >> 24) & 0xFF) as u8,
            ((value >> 32) & 0xFF) as u8,
            ((value >> 40) & 0xFF) as u8,
            ((value >> 48) & 0xFF) as u8,
            ((value >> 56) & 0xFF) as u8, // MOV RAX, value
            0xF3,
            0x48,
            0x0F,
            0xAE,
            0xD0, // WRFSBASE RAX
            0x48,
            0x31,
            0xDB, // XOR RBX, RBX
            0xF3,
            0x48,
            0x0F,
            0xAE,
            0xC3, // RDFSBASE RBX
            0xF4, // HLT
        ];
        let (mut vcpu, _) = setup_vm(&code, None);

        let regs = run_until_hlt(&mut vcpu).unwrap();

        assert_eq!(
            regs.rbx, value,
            "RDFSBASE should read back 0x{:016X}",
            value
        );
    }
}

#[test]
fn test_write_read_cycle_gs() {
    // Test write-read cycle for GS base
    let test_values = [
        0x0000000000003000u64,
        0x0000000000004000u64,
        0x00007FFFFFFFE000u64,
    ];

    for &value in &test_values {
        let code = [
            0x48,
            0xB8,
            (value & 0xFF) as u8,
            ((value >> 8) & 0xFF) as u8,
            ((value >> 16) & 0xFF) as u8,
            ((value >> 24) & 0xFF) as u8,
            ((value >> 32) & 0xFF) as u8,
            ((value >> 40) & 0xFF) as u8,
            ((value >> 48) & 0xFF) as u8,
            ((value >> 56) & 0xFF) as u8, // MOV RAX, value
            0xF3,
            0x48,
            0x0F,
            0xAE,
            0xD8, // WRGSBASE RAX
            0x48,
            0x31,
            0xDB, // XOR RBX, RBX
            0xF3,
            0x48,
            0x0F,
            0xAE,
            0xCB, // RDGSBASE RBX
            0xF4, // HLT
        ];
        let (mut vcpu, _) = setup_vm(&code, None);

        let regs = run_until_hlt(&mut vcpu).unwrap();

        assert_eq!(
            regs.rbx, value,
            "RDGSBASE should read back 0x{:016X}",
            value
        );
    }
}

#[test]
fn test_alternate_fs_gs_writes() {
    // Alternate writes to FS and GS bases
    let code = [
        0x48, 0xC7, 0xC0, 0x11, 0x11, 0x00, 0x00, // MOV RAX, 0x1111
        0xF3, 0x48, 0x0F, 0xAE, 0xD0, // WRFSBASE RAX
        0x48, 0xC7, 0xC0, 0x22, 0x22, 0x00, 0x00, // MOV RAX, 0x2222
        0xF3, 0x48, 0x0F, 0xAE, 0xD8, // WRGSBASE RAX
        0x48, 0xC7, 0xC0, 0x33, 0x33, 0x00, 0x00, // MOV RAX, 0x3333
        0xF3, 0x48, 0x0F, 0xAE, 0xD0, // WRFSBASE RAX
        0x48, 0xC7, 0xC0, 0x44, 0x44, 0x00, 0x00, // MOV RAX, 0x4444
        0xF3, 0x48, 0x0F, 0xAE, 0xD8, // WRGSBASE RAX
        // Read both
        0xF3, 0x48, 0x0F, 0xAE, 0xC3, // RDFSBASE RBX
        0xF3, 0x48, 0x0F, 0xAE, 0xC9, // RDGSBASE RCX
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx, 0x3333, "FS base should be last written value");
    assert_eq!(regs.rcx, 0x4444, "GS base should be last written value");
}

#[test]
fn test_32bit_write_clears_upper_fs() {
    // Verify 32-bit WRFSBASE clears upper 32 bits
    let code = [
        // First set a 64-bit value
        0x48, 0xB8, 0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF,
        0xFF, // MOV RAX, 0xFFFFFFFF00000000
        0xF3, 0x48, 0x0F, 0xAE, 0xD0, // WRFSBASE RAX (64-bit)
        // Now write 32-bit value
        0x48, 0xC7, 0xC0, 0x00, 0x50, 0x00, 0x00, // MOV RAX, 0x5000
        0xF3, 0x0F, 0xAE, 0xD0, // WRFSBASE EAX (32-bit)
        // Read back
        0xF3, 0x48, 0x0F, 0xAE, 0xC3, // RDFSBASE RBX
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rbx, 0x5000,
        "FS base should be 0x5000 with upper bits cleared"
    );
    assert_eq!(
        regs.rbx >> 32,
        0,
        "Upper 32 bits should be cleared by 32-bit write"
    );
}

#[test]
fn test_32bit_write_clears_upper_gs() {
    // Verify 32-bit WRGSBASE clears upper 32 bits
    let code = [
        // First set a 64-bit value
        0x48, 0xB8, 0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF,
        0xFF, // MOV RAX, 0xFFFFFFFF00000000
        0xF3, 0x48, 0x0F, 0xAE, 0xD8, // WRGSBASE RAX (64-bit)
        // Now write 32-bit value
        0x48, 0xC7, 0xC0, 0x00, 0x60, 0x00, 0x00, // MOV RAX, 0x6000
        0xF3, 0x0F, 0xAE, 0xD8, // WRGSBASE EAX (32-bit)
        // Read back
        0xF3, 0x48, 0x0F, 0xAE, 0xCB, // RDGSBASE RBX
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rbx, 0x6000,
        "GS base should be 0x6000 with upper bits cleared"
    );
    assert_eq!(
        regs.rbx >> 32,
        0,
        "Upper 32 bits should be cleared by 32-bit write"
    );
}
