//! Tests for the FLDCW, FSTCW, and FNSTCW instructions.
//!
//! FLDCW - Load x87 FPU Control Word
//! FSTCW - Store x87 FPU Control Word (with exception check)
//! FNSTCW - Store x87 FPU Control Word (without exception check)
//!
//! FLDCW loads the 16-bit source operand into the FPU control word.
//! The control word controls precision, rounding mode, and exception masks.
//!
//! FSTCW stores the current FPU control word to memory after checking for pending exceptions.
//! FNSTCW stores the control word without checking for exceptions.
//!
//! Opcodes:
//! - FLDCW: D9 /5
//! - FSTCW: 9B D9 /7
//! - FNSTCW: D9 /7
//!
//! Control Word Format (16 bits):
//! - Bits 0-5: Exception masks (IM, DM, ZM, OM, UM, PM)
//! - Bits 8-9: Precision control (00=single, 10=double, 11=extended)
//! - Bits 10-11: Rounding control (00=nearest, 01=down, 10=up, 11=toward zero)
//! - Bit 12: Infinity control (deprecated, should be 0)
//!
//! Flags affected:
//! - C0, C1, C2, C3: Undefined
//!
//! References: /Users/int/dev/rax/docs/fldcw.txt, /Users/int/dev/rax/docs/fstcw:fnstcw.txt

use crate::common::*;
use rax::cpu::Registers;
use vm_memory::{Bytes, GuestAddress};

// Helper function to write u16 to memory
fn write_u16(mem: &vm_memory::GuestMemoryMmap, addr: u64, val: u16) {
    mem.write_slice(&val.to_le_bytes(), GuestAddress(addr))
        .unwrap();
}

// Helper function to read u16 from memory
fn read_u16(mem: &vm_memory::GuestMemoryMmap, addr: u64) -> u16 {
    let mut buf = [0u8; 2];
    mem.read_slice(&mut buf, GuestAddress(addr)).unwrap();
    u16::from_le_bytes(buf)
}

// Helper function to write f64 to memory
fn write_f64(mem: &vm_memory::GuestMemoryMmap, addr: u64, val: f64) {
    mem.write_slice(&val.to_le_bytes(), GuestAddress(addr))
        .unwrap();
}

// Helper function to read f64 from memory
fn read_f64(mem: &vm_memory::GuestMemoryMmap, addr: u64) -> f64 {
    let mut buf = [0u8; 8];
    mem.read_slice(&mut buf, GuestAddress(addr)).unwrap();
    f64::from_le_bytes(buf)
}

// Control word bit definitions
const CW_MASK_INVALID: u16 = 0x0001;
const CW_MASK_DENORMAL: u16 = 0x0002;
const CW_MASK_ZERODIVIDE: u16 = 0x0004;
const CW_MASK_OVERFLOW: u16 = 0x0008;
const CW_MASK_UNDERFLOW: u16 = 0x0010;
const CW_MASK_PRECISION: u16 = 0x0020;
const CW_PRECISION_MASK: u16 = 0x0300;
const CW_ROUNDING_MASK: u16 = 0x0C00;
const CW_ROUNDING_NEAREST: u16 = 0x0000;
const CW_ROUNDING_DOWN: u16 = 0x0400;
const CW_ROUNDING_UP: u16 = 0x0800;
const CW_ROUNDING_TRUNC: u16 = 0x0C00;

// ============================================================================
// FNSTCW - Store Control Word
// ============================================================================

#[test]
fn test_fnstcw_basic() {
    // Store the current control word
    let code = [
        0xD9, 0x3C, 0x25, 0x00, 0x30, 0x00, 0x00, // FNSTCW [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    run_until_hlt(&mut vcpu).unwrap();

    let cw = read_u16(&mem, 0x3000);
    // Default control word typically has all exception masks set (0x37F)
    assert!(cw != 0, "Control word should not be zero");
}

#[test]
fn test_fnstcw_twice() {
    // Store control word twice, should be identical
    let code = [
        0xD9, 0x3C, 0x25, 0x00, 0x30, 0x00, 0x00, // FNSTCW [0x3000]
        0xD9, 0x3C, 0x25, 0x02, 0x30, 0x00, 0x00, // FNSTCW [0x3002]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    run_until_hlt(&mut vcpu).unwrap();

    let cw1 = read_u16(&mem, 0x3000);
    let cw2 = read_u16(&mem, 0x3002);
    assert_eq!(cw1, cw2, "Control word should be consistent");
}

// ============================================================================
// FLDCW - Load Control Word
// ============================================================================

#[test]
fn test_fldcw_basic() {
    // Load a control word and verify it's stored
    let code = [
        0xD9, 0x2C, 0x25, 0x00, 0x20, 0x00, 0x00, // FLDCW [0x2000]
        0xD9, 0x3C, 0x25, 0x00, 0x30, 0x00, 0x00, // FNSTCW [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    let test_cw: u16 = 0x037F; // Default control word
    write_u16(&mem, 0x2000, test_cw);

    run_until_hlt(&mut vcpu).unwrap();

    let cw = read_u16(&mem, 0x3000);
    assert_eq!(cw, test_cw, "Loaded control word should match");
}

#[test]
fn test_fldcw_different_values() {
    // Load different control word values
    let test_values = vec![0x027F, 0x037F, 0x047F, 0x0C7F];

    for test_cw in test_values {
        let code = [
            0xD9, 0x2C, 0x25, 0x00, 0x20, 0x00, 0x00, // FLDCW [0x2000]
            0xD9, 0x3C, 0x25, 0x00, 0x30, 0x00, 0x00, // FNSTCW [0x3000]
            0xF4, // HLT
        ];

        let (mut vcpu, mem) = setup_vm(&code, None);
        write_u16(&mem, 0x2000, test_cw);

        run_until_hlt(&mut vcpu).unwrap();

        let cw = read_u16(&mem, 0x3000);
        assert_eq!(cw, test_cw, "Control word 0x{:04X} should match", test_cw);
    }
}

// ============================================================================
// Rounding Mode Control
// ============================================================================

#[test]
fn test_rounding_mode_nearest() {
    // Set rounding mode to nearest (default)
    let code = [
        0xD9, 0x2C, 0x25, 0x00, 0x20, 0x00, 0x00, // FLDCW [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008]
        0xD9, 0xFC, // FRNDINT
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_u16(&mem, 0x2000, 0x037F | CW_ROUNDING_NEAREST); // Round to nearest
    write_f64(&mem, 0x2008, 2.5);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert_eq!(result, 2.0, "2.5 should round to nearest even (2.0)");
}

#[test]
fn test_rounding_mode_down() {
    // Set rounding mode to down (toward -infinity)
    let code = [
        0xD9, 0x2C, 0x25, 0x00, 0x20, 0x00, 0x00, // FLDCW [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008]
        0xD9, 0xFC, // FRNDINT
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_u16(&mem, 0x2000, 0x037F | CW_ROUNDING_DOWN); // Round down
    write_f64(&mem, 0x2008, 2.7);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert_eq!(result, 2.0, "2.7 should round down to 2.0");
}

#[test]
fn test_rounding_mode_up() {
    // Set rounding mode to up (toward +infinity)
    let code = [
        0xD9, 0x2C, 0x25, 0x00, 0x20, 0x00, 0x00, // FLDCW [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008]
        0xD9, 0xFC, // FRNDINT
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_u16(&mem, 0x2000, 0x037F | CW_ROUNDING_UP); // Round up
    write_f64(&mem, 0x2008, 2.3);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert_eq!(result, 3.0, "2.3 should round up to 3.0");
}

#[test]
fn test_rounding_mode_truncate() {
    // Set rounding mode to truncate (toward zero)
    let code = [
        0xD9, 0x2C, 0x25, 0x00, 0x20, 0x00, 0x00, // FLDCW [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008]
        0xD9, 0xFC, // FRNDINT
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_u16(&mem, 0x2000, 0x037F | CW_ROUNDING_TRUNC); // Round toward zero
    write_f64(&mem, 0x2008, 2.9);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert_eq!(result, 2.0, "2.9 should truncate to 2.0");
}

#[test]
fn test_rounding_mode_truncate_negative() {
    // Truncate negative value (toward zero)
    let code = [
        0xD9, 0x2C, 0x25, 0x00, 0x20, 0x00, 0x00, // FLDCW [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008]
        0xD9, 0xFC, // FRNDINT
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_u16(&mem, 0x2000, 0x037F | CW_ROUNDING_TRUNC);
    write_f64(&mem, 0x2008, -2.9);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert_eq!(result, -2.0, "-2.9 should truncate to -2.0");
}

// ============================================================================
// Exception Mask Tests
// ============================================================================

#[test]
fn test_exception_mask_all_set() {
    // All exceptions masked
    let code = [
        0xD9, 0x2C, 0x25, 0x00, 0x20, 0x00, 0x00, // FLDCW [0x2000]
        0xD9, 0x3C, 0x25, 0x00, 0x30, 0x00, 0x00, // FNSTCW [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    let cw = 0x037F; // All exception masks set
    write_u16(&mem, 0x2000, cw);

    run_until_hlt(&mut vcpu).unwrap();

    let result_cw = read_u16(&mem, 0x3000);
    assert_eq!(result_cw & 0x3F, 0x3F, "All exception masks should be set");
}

#[test]
fn test_exception_mask_invalid() {
    // Test invalid operation mask
    let code = [
        0xD9, 0x2C, 0x25, 0x00, 0x20, 0x00, 0x00, // FLDCW [0x2000]
        0xD9, 0x3C, 0x25, 0x00, 0x30, 0x00, 0x00, // FNSTCW [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    let cw = 0x037E; // All except invalid operation masked
    write_u16(&mem, 0x2000, cw);

    run_until_hlt(&mut vcpu).unwrap();

    let result_cw = read_u16(&mem, 0x3000);
    assert_eq!(
        result_cw & CW_MASK_INVALID,
        0,
        "Invalid mask should be clear"
    );
}

// ============================================================================
// FLDCW/FNSTCW Round Trip
// ============================================================================

#[test]
fn test_fldcw_fnstcw_roundtrip() {
    // Load and store should preserve value
    let code = [
        0xD9, 0x2C, 0x25, 0x00, 0x20, 0x00, 0x00, // FLDCW [0x2000]
        0xD9, 0x3C, 0x25, 0x00, 0x30, 0x00, 0x00, // FNSTCW [0x3000]
        0xF4, // HLT
    ];

    let test_values = vec![0x027F, 0x037F, 0x047F, 0x067F, 0x0B7F, 0x0C7F, 0x0F7F];

    for test_cw in test_values {
        let (mut vcpu, mem) = setup_vm(&code, None);
        write_u16(&mem, 0x2000, test_cw);

        run_until_hlt(&mut vcpu).unwrap();

        let result = read_u16(&mem, 0x3000);
        assert_eq!(result, test_cw, "Round trip failed for 0x{:04X}", test_cw);
    }
}

#[test]
fn test_multiple_fldcw() {
    // Multiple FLDCW operations
    let code = [
        0xD9, 0x2C, 0x25, 0x00, 0x20, 0x00, 0x00, // FLDCW [0x2000]
        0xD9, 0x2C, 0x25, 0x02, 0x20, 0x00, 0x00, // FLDCW [0x2002]
        0xD9, 0x3C, 0x25, 0x00, 0x30, 0x00, 0x00, // FNSTCW [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_u16(&mem, 0x2000, 0x027F);
    write_u16(&mem, 0x2002, 0x0C7F);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_u16(&mem, 0x3000);
    assert_eq!(result, 0x0C7F, "Last FLDCW should take effect");
}

// ============================================================================
// Rounding Combinations
// ============================================================================

#[test]
fn test_rounding_modes_with_different_values() {
    let code = [
        0xD9, 0x2C, 0x25, 0x00, 0x20, 0x00, 0x00, // FLDCW [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008]
        0xD9, 0xFC, // FRNDINT
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    // Test different rounding modes with 1.5 (halfway case)
    let test_cases = vec![
        (CW_ROUNDING_NEAREST, 1.5, 2.0), // Round to even
        (CW_ROUNDING_DOWN, 1.5, 1.0),
        (CW_ROUNDING_UP, 1.5, 2.0),
        (CW_ROUNDING_TRUNC, 1.5, 1.0),
    ];

    for (mode, input, expected) in test_cases {
        let (mut vcpu, mem) = setup_vm(&code, None);
        write_u16(&mem, 0x2000, 0x037F | mode);
        write_f64(&mem, 0x2008, input);

        run_until_hlt(&mut vcpu).unwrap();

        let result = read_f64(&mem, 0x3000);
        assert_eq!(
            result, expected,
            "Rounding mode 0x{:04X} with input {}",
            mode, input
        );
    }
}

#[test]
fn test_rounding_negative_values() {
    let code = [
        0xD9, 0x2C, 0x25, 0x00, 0x20, 0x00, 0x00, // FLDCW [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008]
        0xD9, 0xFC, // FRNDINT
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let test_cases = vec![
        (CW_ROUNDING_DOWN, -1.3, -2.0),  // Down means toward -infinity
        (CW_ROUNDING_UP, -1.3, -1.0),    // Up means toward +infinity
        (CW_ROUNDING_TRUNC, -1.9, -1.0), // Truncate toward zero
    ];

    for (mode, input, expected) in test_cases {
        let (mut vcpu, mem) = setup_vm(&code, None);
        write_u16(&mem, 0x2000, 0x037F | mode);
        write_f64(&mem, 0x2008, input);

        run_until_hlt(&mut vcpu).unwrap();

        let result = read_f64(&mem, 0x3000);
        assert_eq!(
            result, expected,
            "Rounding mode 0x{:04X} with negative input {}",
            mode, input
        );
    }
}

// ============================================================================
// Control Word Persistence
// ============================================================================

#[test]
fn test_control_word_persists_across_operations() {
    let code = [
        0xD9, 0x2C, 0x25, 0x00, 0x20, 0x00, 0x00, // FLDCW [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008]
        0xDD, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, // FLD qword [0x2010]
        0xDE, 0xC1, // FADDP
        0xD9, 0xFC, // FRNDINT
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xD9, 0x3C, 0x25, 0x02, 0x30, 0x00, 0x00, // FNSTCW [0x3002]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    let test_cw = 0x0C7F; // Truncate mode
    write_u16(&mem, 0x2000, test_cw);
    write_f64(&mem, 0x2008, 2.3);
    write_f64(&mem, 0x2010, 1.7);

    run_until_hlt(&mut vcpu).unwrap();

    let result_cw = read_u16(&mem, 0x3002);
    assert_eq!(result_cw, test_cw, "Control word should persist");
}

// ============================================================================
// Edge Cases
// ============================================================================

#[test]
fn test_control_word_all_zeros() {
    let code = [
        0xD9, 0x2C, 0x25, 0x00, 0x20, 0x00, 0x00, // FLDCW [0x2000]
        0xD9, 0x3C, 0x25, 0x00, 0x30, 0x00, 0x00, // FNSTCW [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_u16(&mem, 0x2000, 0x0000);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_u16(&mem, 0x3000);
    assert_eq!(result, 0x0000, "Control word of all zeros should be valid");
}

#[test]
fn test_control_word_all_ones() {
    let code = [
        0xD9, 0x2C, 0x25, 0x00, 0x20, 0x00, 0x00, // FLDCW [0x2000]
        0xD9, 0x3C, 0x25, 0x00, 0x30, 0x00, 0x00, // FNSTCW [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_u16(&mem, 0x2000, 0xFFFF);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_u16(&mem, 0x3000);
    // Some bits are reserved and may be masked
    assert!(result != 0, "Control word loaded");
}

// ============================================================================
// FSTCW vs FNSTCW
// ============================================================================

#[test]
fn test_fstcw_fnstcw_equivalence() {
    // In normal operation, FSTCW and FNSTCW should produce same result
    let code1 = [
        0x9B, 0xD9, 0x3C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTCW [0x3000]
        0xF4, // HLT
    ];

    let code2 = [
        0xD9, 0x3C, 0x25, 0x00, 0x30, 0x00, 0x00, // FNSTCW [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu1, mem1) = setup_vm(&code1, None);
    run_until_hlt(&mut vcpu1).unwrap();
    let cw1 = read_u16(&mem1, 0x3000);

    let (mut vcpu2, mem2) = setup_vm(&code2, None);
    run_until_hlt(&mut vcpu2).unwrap();
    let cw2 = read_u16(&mem2, 0x3000);

    assert_eq!(cw1, cw2, "FSTCW and FNSTCW should give same result");
}

// ============================================================================
// Complex Scenarios
// ============================================================================

#[test]
fn test_changing_rounding_modes_dynamically() {
    // Change rounding mode between operations
    let code = [
        // Round 2.5 with nearest (should be 2.0)
        0xD9, 0x2C, 0x25, 0x00, 0x20, 0x00, 0x00, // FLDCW [0x2000] - nearest
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008]
        0xD9, 0xFC, // FRNDINT
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        // Round 2.5 with up (should be 3.0)
        0xD9, 0x2C, 0x25, 0x02, 0x20, 0x00, 0x00, // FLDCW [0x2002] - up
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008]
        0xD9, 0xFC, // FRNDINT
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, // FSTP qword [0x3008]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_u16(&mem, 0x2000, 0x037F | CW_ROUNDING_NEAREST);
    write_u16(&mem, 0x2002, 0x037F | CW_ROUNDING_UP);
    write_f64(&mem, 0x2008, 2.5);

    run_until_hlt(&mut vcpu).unwrap();

    let result1 = read_f64(&mem, 0x3000);
    let result2 = read_f64(&mem, 0x3008);
    assert_eq!(result1, 2.0, "First round should use nearest mode");
    assert_eq!(result2, 3.0, "Second round should use up mode");
}

#[test]
fn test_control_word_with_arithmetic() {
    // Verify control word affects arithmetic operations
    let code = [
        0xD9, 0x2C, 0x25, 0x00, 0x20, 0x00, 0x00, // FLDCW [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008]
        0xDD, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, // FLD qword [0x2010]
        0xDE, 0xC1, // FADDP
        0xD9, 0xFC, // FRNDINT
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_u16(&mem, 0x2000, 0x037F | CW_ROUNDING_DOWN);
    write_f64(&mem, 0x2008, 1.6);
    write_f64(&mem, 0x2010, 1.6);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert_eq!(result, 3.0, "1.6 + 1.6 = 3.2, rounded down = 3.0");
}
