//! Tests for the ARPL instruction.
//!
//! ARPL - Adjust RPL Field of Segment Selector
//!
//! Compares the RPL fields of two segment selectors. The first operand (the
//! destination operand) contains one segment selector and the second operand
//! (source operand) contains the other. If the RPL field of the destination
//! operand is less than the RPL field of the source operand, the ZF flag is
//! set and the RPL field of the destination operand is increased to match
//! that of the source operand.
//!
//! **IMPORTANT**: In 64-bit mode, the ARPL instruction is not encodable. The
//! opcode 0x63 is instead used for MOVSXD (sign-extend doubleword to quadword).
//!
//! Opcode: 63 /r
//! Flags affected: ZF
//!
//! Reference: docs/arpl.txt

use crate::common::*;
use rax::cpu::Registers;

// ============================================================================
// ARPL in 64-bit Mode (Actually MOVSXD)
// ============================================================================

#[test]
fn test_arpl_opcode_is_movsxd_in_64bit() {
    // In 64-bit mode, opcode 0x63 is MOVSXD, not ARPL
    // MOVSXD RAX, ECX - sign-extend ECX to RAX
    // 0x48 0x63 0xC1 = MOVSXD RAX, ECX
    let code = [
        0x48, 0xC7, 0xC1, 0x00, 0x00, 0x00, 0x80, // MOV RCX, 0x80000000
        0x48, 0x63, 0xC1, // MOVSXD RAX, ECX (opcode 0x63)
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Should sign-extend 0x80000000 to 0xFFFFFFFF80000000
    assert_eq!(regs.rax, 0xFFFFFFFF80000000u64 as i64 as u64);
}

#[test]
fn test_arpl_opcode_movsxd_positive() {
    // Sign-extend positive value
    // MOVSXD RBX, EAX
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x00, 0x00, 0x7F, // MOV RAX, 0x7F000000
        0x48, 0x63, 0xD8, // MOVSXD RBX, EAX
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Positive value, high bit clear - zero extend
    assert_eq!(regs.rbx, 0x7F000000);
}

#[test]
fn test_arpl_opcode_movsxd_negative() {
    // Sign-extend negative value
    // MOVSXD RDX, EDI
    let code = [
        0x48, 0xC7, 0xC7, 0xFF, 0xFF, 0xFF, 0xFF, // MOV RDI, 0xFFFFFFFF
        0x48, 0x63, 0xD7, // MOVSXD RDX, EDI
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Should sign-extend to all 1s
    assert_eq!(regs.rdx, 0xFFFFFFFFFFFFFFFF);
}

#[test]
fn test_arpl_opcode_movsxd_from_memory() {
    // MOVSXD from memory location
    let code = [
        0x48, 0xC7, 0x04, 0x24, 0x34, 0x12, 0x00, 0x80, // MOV [RSP], 0x80001234
        0x48, 0x63, 0x04, 0x24, // MOVSXD RAX, [RSP]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Sign-extend 0x80001234 (negative)
    assert_eq!(regs.rax, 0xFFFFFFFF80001234);
}

#[test]
fn test_arpl_opcode_movsxd_zero() {
    // Sign-extend zero
    let code = [
        0x48, 0x31, 0xC0, // XOR RAX, RAX
        0x48, 0x63, 0xC0, // MOVSXD RAX, EAX
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0);
}

// ============================================================================
// ARPL Semantics Tests (Theoretical - for documentation)
// ============================================================================
// These tests document what ARPL would do in 32-bit/16-bit modes, though
// the emulator running in 64-bit mode will execute MOVSXD instead.

#[test]
fn test_arpl_concept_rpl_lower() {
    // In 32-bit mode, ARPL would work like this:
    // If dest RPL (bits 1:0) < src RPL, then:
    //   - Set ZF = 1
    //   - Copy src RPL to dest RPL
    //
    // Example: dest=0x0008 (RPL=0), src=0x0003 (RPL=3)
    // Result:  dest=0x000B (RPL=3), ZF=1
    //
    // But in 64-bit mode this opcode does MOVSXD, so we test the concept
    // via documentation only.

    // This test demonstrates understanding of ARPL semantics
    let dest_selector: u16 = 0x0008; // Selector with RPL=0
    let src_selector: u16 = 0x0003; // Selector with RPL=3

    let dest_rpl = dest_selector & 0x3;
    let src_rpl = src_selector & 0x3;

    assert_eq!(dest_rpl, 0, "Destination RPL should be 0");
    assert_eq!(src_rpl, 3, "Source RPL should be 3");

    // ARPL would set ZF and adjust dest to have RPL=3
    let would_set_zf = dest_rpl < src_rpl;
    let adjusted_dest = (dest_selector & 0xFFFC) | src_rpl;

    assert!(would_set_zf, "ARPL would set ZF when dest RPL < src RPL");
    assert_eq!(adjusted_dest, 0x000B, "ARPL would adjust dest to 0x000B");
}

#[test]
fn test_arpl_concept_rpl_equal() {
    // When RPLs are equal, ARPL does nothing and clears ZF
    let dest_selector: u16 = 0x0023; // RPL=3
    let src_selector: u16 = 0x0013; // RPL=3

    let dest_rpl = dest_selector & 0x3;
    let src_rpl = src_selector & 0x3;

    assert_eq!(dest_rpl, 3);
    assert_eq!(src_rpl, 3);

    let would_set_zf = dest_rpl < src_rpl;
    assert!(!would_set_zf, "ARPL would clear ZF when RPLs are equal");
}

#[test]
fn test_arpl_concept_rpl_higher() {
    // When dest RPL > src RPL, ARPL does nothing and clears ZF
    let dest_selector: u16 = 0x0023; // RPL=3
    let src_selector: u16 = 0x0010; // RPL=0

    let dest_rpl = dest_selector & 0x3;
    let src_rpl = src_selector & 0x3;

    assert_eq!(dest_rpl, 3);
    assert_eq!(src_rpl, 0);

    let would_set_zf = dest_rpl < src_rpl;
    assert!(
        !would_set_zf,
        "ARPL would clear ZF when dest RPL >= src RPL"
    );
}

// ============================================================================
// MOVSXD Comprehensive Tests (Actual 64-bit Behavior)
// ============================================================================

#[test]
fn test_movsxd_all_registers() {
    // Test MOVSXD with various register combinations
    let code = [
        // Setup source values
        0x48, 0xC7, 0xC0, 0x00, 0x00, 0x00, 0x80, // MOV RAX, 0x80000000
        0x48, 0xC7, 0xC3, 0xFF, 0xFF, 0xFF, 0x7F, // MOV RBX, 0x7FFFFFFF
        0x48, 0xC7, 0xC1, 0x01, 0x00, 0x00, 0x00, // MOV RCX, 0x00000001
        // Test MOVSXD operations
        0x48, 0x63, 0xD0, // MOVSXD RDX, EAX (negative)
        0x48, 0x63, 0xF3, // MOVSXD RSI, EBX (positive max)
        0x4C, 0x63, 0xC1, // MOVSXD R8, ECX (small positive)
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rdx, 0xFFFFFFFF80000000); // Sign-extended negative
    assert_eq!(regs.rsi, 0x000000007FFFFFFF); // Zero-extended positive
    assert_eq!(regs.r8, 0x0000000000000001); // Zero-extended small
}

#[test]
fn test_movsxd_boundary_values() {
    // Test boundary values for sign extension
    let code = [
        // Test 0x7FFFFFFF (largest positive 32-bit)
        0x48, 0xC7, 0xC0, 0xFF, 0xFF, 0xFF, 0x7F, // MOV RAX, 0x7FFFFFFF
        0x48, 0x63, 0xC8, // MOVSXD RCX, EAX
        // Test 0x80000000 (smallest negative 32-bit)
        0x48, 0xC7, 0xC0, 0x00, 0x00, 0x00, 0x80, // MOV RAX, 0x80000000
        0x48, 0x63, 0xD0, // MOVSXD RDX, EAX
        // Test 0x00000000 (zero)
        0x48, 0x31, 0xC0, // XOR RAX, RAX
        0x48, 0x63, 0xD8, // MOVSXD RBX, EAX
        // Test 0xFFFFFFFF (all bits set in 32-bit, -1)
        0x48, 0xC7, 0xC0, 0xFF, 0xFF, 0xFF, 0xFF, // MOV RAX, 0xFFFFFFFF
        0x48, 0x63, 0xF8, // MOVSXD RDI, EAX
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rcx, 0x000000007FFFFFFF,
        "Max positive should zero-extend"
    );
    assert_eq!(
        regs.rdx, 0xFFFFFFFF80000000,
        "Min negative should sign-extend"
    );
    assert_eq!(regs.rbx, 0x0000000000000000, "Zero should stay zero");
    assert_eq!(
        regs.rdi, 0xFFFFFFFFFFFFFFFF,
        "-1 should sign-extend to all 1s"
    );
}

#[test]
fn test_movsxd_preserves_flags() {
    // MOVSXD should not modify any flags
    let code = [
        // Set all flags
        0x48, 0xC7, 0xC0, 0xFF, 0xFF, 0xFF, 0xFF, // MOV RAX, -1
        0x48, 0x83, 0xC0, 0x01, // ADD RAX, 1 (sets ZF, clears CF, OF)
        // Save RFLAGS
        0x9C, // PUSHFQ
        0x5E, // POP RSI (save flags)
        // Do MOVSXD
        0x48, 0xC7, 0xC0, 0x00, 0x00, 0x00, 0x80, // MOV RAX, 0x80000000
        0x48, 0x63, 0xD8, // MOVSXD RBX, EAX
        // Check RFLAGS unchanged
        0x9C, // PUSHFQ
        0x5F, // POP RDI (current flags)
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Flags should be unchanged by MOVSXD
    assert_eq!(
        regs.rsi & 0xCD5,
        regs.rdi & 0xCD5,
        "MOVSXD should preserve flags"
    );
}

#[test]
fn test_movsxd_memory_to_register() {
    // Test MOVSXD from various memory locations
    let code = [
        // Write test values to stack
        0x48, 0xC7, 0x44, 0x24, 0x00, 0x00, 0x00, 0x00, 0x80, // MOV [RSP+0], 0x80000000
        0x48, 0xC7, 0x44, 0x24, 0x08, 0xFF, 0xFF, 0xFF, 0x7F, // MOV [RSP+8], 0x7FFFFFFF
        0x48, 0xC7, 0x44, 0x24, 0x10, 0x34, 0x12, 0x00, 0x00, // MOV [RSP+16], 0x00001234
        // Load with sign extension
        0x48, 0x63, 0x04, 0x24, // MOVSXD RAX, [RSP+0]
        0x48, 0x63, 0x5C, 0x24, 0x08, // MOVSXD RBX, [RSP+8]
        0x48, 0x63, 0x4C, 0x24, 0x10, // MOVSXD RCX, [RSP+16]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0xFFFFFFFF80000000);
    assert_eq!(regs.rbx, 0x000000007FFFFFFF);
    assert_eq!(regs.rcx, 0x0000000000001234);
}

#[test]
fn test_movsxd_chained_operations() {
    // Chain multiple MOVSXD operations
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x00, 0x00, 0x80, // MOV RAX, 0x80000000
        0x48, 0x63, 0xD8, // MOVSXD RBX, EAX
        0x48, 0x63, 0xCB, // MOVSXD RCX, EBX
        0x48, 0x63, 0xD1, // MOVSXD RDX, ECX
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    // All should be sign-extended version of 0x80000000
    assert_eq!(regs.rbx, 0xFFFFFFFF80000000);
    assert_eq!(regs.rcx, 0xFFFFFFFF80000000);
    assert_eq!(regs.rdx, 0xFFFFFFFF80000000);
}

#[test]
fn test_movsxd_with_displacement() {
    // Test MOVSXD with various addressing modes
    let code = [
        // Setup base pointer
        0x48, 0xC7, 0xC3, 0x00, 0x10, 0x00, 0x00, // MOV RBX, 0x1000
        // Write value at RBX
        0x48, 0xC7, 0x03, 0xEF, 0xBE, 0xAD, 0xDE, // MOV [RBX], 0xDEADBEEF
        // MOVSXD with base addressing
        0x48, 0x63, 0x03, // MOVSXD RAX, [RBX]
        // MOVSXD with displacement
        0x48, 0x63, 0x4B, 0x00, // MOVSXD RCX, [RBX+0]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    // 0xDEADBEEF has bit 31 set, so sign-extend
    assert_eq!(regs.rax, 0xFFFFFFFFDEADBEEF);
    assert_eq!(regs.rcx, 0xFFFFFFFFDEADBEEF);
}

#[test]
fn test_movsxd_zero_extends_positive() {
    // Verify that positive values (bit 31 = 0) are zero-extended
    let test_values = [
        0x00000000u32,
        0x00000001u32,
        0x0000FFFFu32,
        0x12345678u32,
        0x7FFFFFFFu32,
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
            0x63,
            0xD8, // MOVSXD RBX, EAX
            0xF4, // HLT
        ];
        let (mut vcpu, _) = setup_vm(&code, None);

        let regs = run_until_hlt(&mut vcpu).unwrap();

        assert_eq!(
            regs.rbx, value as u64,
            "MOVSXD should zero-extend positive value 0x{:08X}",
            value
        );
    }
}

#[test]
fn test_movsxd_sign_extends_negative() {
    // Verify that negative values (bit 31 = 1) are sign-extended
    let test_values = [
        0x80000000u32,
        0x80000001u32,
        0xFFFF0000u32,
        0xDEADBEEFu32,
        0xFFFFFFFFu32,
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
            0x63,
            0xD8, // MOVSXD RBX, EAX
            0xF4, // HLT
        ];
        let (mut vcpu, _) = setup_vm(&code, None);

        let regs = run_until_hlt(&mut vcpu).unwrap();

        let expected = value as i32 as i64 as u64;
        assert_eq!(
            regs.rbx, expected,
            "MOVSXD should sign-extend negative value 0x{:08X} to 0x{:016X}",
            value, expected
        );
    }
}
