use crate::common::*;
use rax::cpu::{Registers, VCpu};

// AAS — ASCII Adjust AL After Subtraction
//
// Opcode: 3F
// Instruction: AAS
// Op/En: ZO (no operands)
// 64-bit Mode: Invalid
// Compat/Leg Mode: Valid
//
// Description:
// Adjusts the result of the subtraction of two unpacked BCD values to create
// an unpacked BCD result. The AL register is the implied source and destination.
// AAS is only useful when it follows a SUB instruction that subtracts (binary
// subtraction) one unpacked BCD value from another and stores a byte result in AL.
//
// Operation:
// IF ((AL AND 0FH) > 9) or (AF = 1)
// THEN
//     AX := AX - 6;
//     AH := AH - 1;
//     AF := 1;
//     CF := 1;
//     AL := AL AND 0FH;
// ELSE
//     CF := 0;
//     AF := 0;
//     AL := AL AND 0FH;
// FI;
//
// Flags Affected:
// AF and CF are set to 1 if there is a decimal borrow; otherwise cleared to 0.
// OF, SF, ZF, and PF are undefined.

// ============================================================================
// AAS - Basic Cases (No Adjustment)
// ============================================================================

#[test]
fn test_aas_no_adjustment_needed() {
    // AL = 0x05 (valid BCD), no adjustment needed
    let code = [
        0x3f, // AAS
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x05;
    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x05, "AL should remain 0x05");
    assert_eq!((regs.rax >> 8) & 0xFF, 0x00, "AH should remain 0x00");
    assert!(!cf_set(regs.rflags), "CF should be clear");
    assert!(!af_set(regs.rflags), "AF should be clear");
}

#[test]
fn test_aas_zero() {
    // AL = 0x00, no adjustment needed
    let code = [0x3f, 0xf4]; // AAS, HLT
    let mut regs = Registers::default();
    regs.rax = 0x00;
    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x00, "AL should remain 0x00");
    assert_eq!((regs.rax >> 8) & 0xFF, 0x00, "AH should remain 0x00");
    assert!(!cf_set(regs.rflags), "CF should be clear");
    assert!(!af_set(regs.rflags), "AF should be clear");
}

#[test]
fn test_aas_max_valid_bcd() {
    // AL = 0x09 (max valid BCD digit), no adjustment needed
    let code = [0x3f, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 0x09;
    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x09, "AL should remain 0x09");
    assert_eq!((regs.rax >> 8) & 0xFF, 0x00, "AH should remain 0x00");
    assert!(!cf_set(regs.rflags), "CF should be clear");
    assert!(!af_set(regs.rflags), "AF should be clear");
}

// ============================================================================
// AAS - Adjustment Required (Lower Nibble > 9)
// ============================================================================

#[test]
fn test_aas_lower_nibble_0a() {
    // AL = 0x0A, requires adjustment
    let code = [0x3f, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 0x010A;
    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFF,
        0x04,
        "AL should be 0x04 (0x0A - 0x06 = 0x04)"
    );
    assert_eq!(
        (regs.rax >> 8) & 0xFF,
        0x00,
        "AH should be decremented to 0x00"
    );
    assert!(cf_set(regs.rflags), "CF should be set");
    assert!(af_set(regs.rflags), "AF should be set");
}

#[test]
fn test_aas_lower_nibble_0b() {
    // AL = 0x0B
    let code = [0x3f, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 0x010B;
    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x05, "AL should be 0x05");
    assert_eq!(
        (regs.rax >> 8) & 0xFF,
        0x00,
        "AH should be decremented to 0x00"
    );
    assert!(cf_set(regs.rflags), "CF should be set");
    assert!(af_set(regs.rflags), "AF should be set");
}

#[test]
fn test_aas_lower_nibble_0f() {
    // AL = 0x0F (max value for lower nibble)
    let code = [0x3f, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 0x010F;
    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x09, "AL should be 0x09");
    assert_eq!(
        (regs.rax >> 8) & 0xFF,
        0x00,
        "AH should be decremented to 0x00"
    );
    assert!(cf_set(regs.rflags), "CF should be set");
    assert!(af_set(regs.rflags), "AF should be set");
}

// ============================================================================
// AAS - With Upper Nibble Set
// ============================================================================

#[test]
fn test_aas_with_upper_nibble_1x() {
    // AL = 0x1A, upper nibble should be cleared
    let code = [0x3f, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 0x011A;
    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFF,
        0x04,
        "AL should be 0x04 (upper nibble cleared)"
    );
    assert_eq!(
        (regs.rax >> 8) & 0xFF,
        0x00,
        "AH should be decremented to 0x00"
    );
    assert!(cf_set(regs.rflags), "CF should be set");
    assert!(af_set(regs.rflags), "AF should be set");
}

#[test]
fn test_aas_with_upper_nibble_2x() {
    // AL = 0x2B
    let code = [0x3f, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 0x012B;
    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x05, "AL should be 0x05");
    assert_eq!(
        (regs.rax >> 8) & 0xFF,
        0x00,
        "AH should be decremented to 0x00"
    );
    assert!(cf_set(regs.rflags), "CF should be set");
    assert!(af_set(regs.rflags), "AF should be set");
}

#[test]
fn test_aas_with_upper_nibble_fx() {
    // AL = 0xFC
    let code = [0x3f, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 0x01FC;
    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x06, "AL should be 0x06");
    assert_eq!(
        (regs.rax >> 8) & 0xFF,
        0x00,
        "AH should be decremented to 0x00"
    );
    assert!(cf_set(regs.rflags), "CF should be set");
    assert!(af_set(regs.rflags), "AF should be set");
}

// ============================================================================
// AAS - With Non-Zero AH
// ============================================================================

#[test]
fn test_aas_ah_nonzero_no_adjust() {
    // AH = 0x05, AL = 0x03, no adjustment needed
    let code = [0x3f, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 0x0503;
    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x03, "AL should remain 0x03");
    assert_eq!((regs.rax >> 8) & 0xFF, 0x05, "AH should remain 0x05");
    assert!(!cf_set(regs.rflags), "CF should be clear");
    assert!(!af_set(regs.rflags), "AF should be clear");
}

#[test]
fn test_aas_ah_nonzero_adjust() {
    // AH = 0x07, AL = 0x0D, adjustment needed
    let code = [0x3f, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 0x070D;
    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x07, "AL should be 0x07");
    assert_eq!(
        (regs.rax >> 8) & 0xFF,
        0x06,
        "AH should be decremented to 0x06"
    );
    assert!(cf_set(regs.rflags), "CF should be set");
    assert!(af_set(regs.rflags), "AF should be set");
}

#[test]
fn test_aas_ah_underflow() {
    // AH = 0x00, AL = 0x0E, test AH underflow
    let code = [0x3f, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 0x000E;
    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x08, "AL should be 0x08");
    assert_eq!((regs.rax >> 8) & 0xFF, 0xFF, "AH should wrap to 0xFF");
    assert!(cf_set(regs.rflags), "CF should be set");
    assert!(af_set(regs.rflags), "AF should be set");
}

// ============================================================================
// AAS - With AF Flag Set
// ============================================================================

#[test]
fn test_aas_af_set_valid_bcd() {
    // AL = 0x05 (valid BCD), but AF is set - should still adjust
    let code = [0x3f, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 0x0105;
    regs.rflags = 0x10; // Set AF
    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFF,
        0x0F,
        "AL should be 0x0F (0x05 - 0x06 = 0xFF, masked = 0x0F)"
    );
    assert_eq!(
        (regs.rax >> 8) & 0xFF,
        0xFF,
        "AH should be decremented with borrow"
    );
    assert!(cf_set(regs.rflags), "CF should be set");
    assert!(af_set(regs.rflags), "AF should be set");
}

#[test]
fn test_aas_af_set_zero() {
    // AL = 0x00, AF is set - should adjust
    let code = [0x3f, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 0x0100;
    regs.rflags = 0x10; // Set AF
    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x0A, "AL should be 0x0A");
    assert_eq!(
        (regs.rax >> 8) & 0xFF,
        0xFF,
        "AH should be decremented with borrow"
    );
    assert!(cf_set(regs.rflags), "CF should be set");
    assert!(af_set(regs.rflags), "AF should be set");
}

#[test]
fn test_aas_af_set_nine() {
    // AL = 0x09, AF is set - should adjust
    let code = [0x3f, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 0x0109;
    regs.rflags = 0x10; // Set AF
    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFF,
        0x03,
        "AL should be 0x03 (0x09 - 0x06 = 0x03)"
    );
    assert_eq!(
        (regs.rax >> 8) & 0xFF,
        0x00,
        "AH should be decremented to 0x00"
    );
    assert!(cf_set(regs.rflags), "CF should be set");
    assert!(af_set(regs.rflags), "AF should be set");
}

// ============================================================================
// AAS - Realistic BCD Subtraction Examples
// ============================================================================

#[test]
fn test_aas_after_sub_8_minus_3() {
    // Simulate: 8 - 3 = 5 (no adjustment needed)
    let code = [
        0xb0, 0x08, // MOV AL, 8
        0x2c, 0x03, // SUB AL, 3
        0x3f, // AAS
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm_compat(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x05, "Result should be 5");
    assert_eq!((regs.rax >> 8) & 0xFF, 0x00, "AH should be 0");
    assert!(!cf_set(regs.rflags), "CF should be clear");
    assert!(!af_set(regs.rflags), "AF should be clear");
}

#[test]
fn test_aas_after_sub_3_minus_5() {
    // Simulate: 3 - 5 = -2 (0xFE) -> should adjust with borrow
    let code = [
        0xb0, 0x03, // MOV AL, 3
        0x2c, 0x05, // SUB AL, 5 (result: 0xFE, AF set)
        0x3f, // AAS
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm_compat(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFF,
        0x08,
        "AL should be 0x08 (0xFE - 0x06 = 0xF8, masked = 0x08)"
    );
    assert_eq!((regs.rax >> 8) & 0xFF, 0xFF, "AH should be 0xFF (borrow)");
    assert!(cf_set(regs.rflags), "CF should be set");
    assert!(af_set(regs.rflags), "AF should be set");
}

#[test]
fn test_aas_after_sub_9_minus_9() {
    // Simulate: 9 - 9 = 0 (no adjustment needed)
    let code = [
        0xb0, 0x09, // MOV AL, 9
        0x2c, 0x09, // SUB AL, 9
        0x3f, // AAS
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm_compat(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x00, "Result should be 0");
    assert_eq!((regs.rax >> 8) & 0xFF, 0x00, "AH should be 0");
    assert!(!cf_set(regs.rflags), "CF should be clear");
    assert!(!af_set(regs.rflags), "AF should be clear");
}

#[test]
fn test_aas_after_sub_2_minus_7() {
    // Simulate: 2 - 7 = -5 (0xFB) -> should adjust with borrow
    let code = [
        0xb0, 0x02, // MOV AL, 2
        0x2c, 0x07, // SUB AL, 7 (result: 0xFB, AF set)
        0x3f, // AAS
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm_compat(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFF,
        0x05,
        "AL should be 0x05 (0xFB - 0x06 = 0xF5, masked = 0x05)"
    );
    assert_eq!((regs.rax >> 8) & 0xFF, 0xFF, "AH should be 0xFF (borrow)");
    assert!(cf_set(regs.rflags), "CF should be set");
    assert!(af_set(regs.rflags), "AF should be set");
}

// ============================================================================
// AAS - Multi-Digit BCD Subtraction
// ============================================================================

#[test]
fn test_aas_multidigit_78_minus_34() {
    // Simulate: 78 - 34 = 44
    // First digit: 8 - 4 = 4 (no borrow)
    let code = [
        0xb0, 0x08, // MOV AL, 8
        0x2c, 0x04, // SUB AL, 4
        0x3f, // AAS
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm_compat(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x04, "Ones digit should be 4");
    assert_eq!((regs.rax >> 8) & 0xFF, 0x00, "No borrow to tens");
}

#[test]
fn test_aas_multidigit_52_minus_37() {
    // Simulate: 52 - 37 = 15
    // First digit: 2 - 7 (needs borrow)
    let code = [
        0xb0, 0x02, // MOV AL, 2
        0x2c, 0x07, // SUB AL, 7 (result: 0xFB, AF set)
        0x3f, // AAS
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm_compat(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x05, "Ones digit should be 5");
    assert_eq!((regs.rax >> 8) & 0xFF, 0xFF, "Borrow from tens");
}

// ============================================================================
// AAS - Edge Cases and Boundary Conditions
// ============================================================================

#[test]
fn test_aas_all_lower_nibbles() {
    // Test all lower nibble values from 0x00 to 0x0F
    for val in 0x00..=0x0F {
        let code = [0x3f, 0xf4];
        let mut regs = Registers::default();
        regs.rax = 0x0100 | val;
        let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        if val <= 9 {
            assert_eq!(regs.rax & 0xFF, val, "AL should remain {:#04x}", val);
            assert!(!cf_set(regs.rflags), "CF should be clear for {:#04x}", val);
        } else {
            let expected = (val.wrapping_sub(6)) & 0x0F;
            assert_eq!(
                regs.rax & 0xFF,
                expected,
                "AL should be {:#04x} for input {:#04x}",
                expected,
                val
            );
            assert!(cf_set(regs.rflags), "CF should be set for {:#04x}", val);
        }
    }
}

#[test]
fn test_aas_upper_bits_cleared() {
    // Verify that upper 4 bits of AL are always cleared
    let code = [0x3f, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 0x01AB;
    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xF0,
        0x00,
        "Upper 4 bits of AL should be cleared"
    );
}

#[test]
fn test_aas_preserves_high_rax() {
    // Verify that bits above AX are preserved
    let code = [0x3f, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 0x1234_5678_DEAD_BE0F;
    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax >> 16,
        0x1234_5678_DEAD,
        "High bits of RAX should be preserved"
    );
}

#[test]
fn test_aas_sequential_operations() {
    // Test multiple AAS operations in sequence
    let code = [
        0xb8, 0x03, 0x00, // MOV AX, 0x0003 (AH=0, AL=3)
        0x2c, 0x05, // SUB AL, 5 (result: 0xFE, borrow)
        0x3f, // AAS (result: AL=8, AH=FF)
        0x80, 0xec, 0x09, // SUB AH, 9 (manually adjust tens)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm_compat(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x08, "Final AL should be 8");
}

#[test]
fn test_aas_max_value_ff() {
    // AL = 0xFF (all bits set)
    let code = [0x3f, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 0x01FF;
    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x09, "AL should be 0x09");
    assert_eq!((regs.rax >> 8) & 0xFF, 0x00, "AH should be 0x00");
    assert!(cf_set(regs.rflags), "CF should be set");
    assert!(af_set(regs.rflags), "AF should be set");
}

#[test]
fn test_aas_double_borrow() {
    // Test with AL requiring adjustment from zero AH
    let code = [
        0xb8, 0x00, 0x00, // MOV AX, 0x0000
        0x2c, 0x05, // SUB AL, 5 (result: 0xFB)
        0x3f, // AAS
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm_compat(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x05, "AL should be 5");
    assert_eq!((regs.rax >> 8) & 0xFF, 0xFF, "AH should wrap to 0xFF");
}

#[test]
fn test_aas_with_high_ah() {
    // AH = 0x99, AL = 0x0C (test with high AH value)
    let code = [0x3f, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 0x990C;
    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x06, "AL should be 0x06");
    assert_eq!(
        (regs.rax >> 8) & 0xFF,
        0x98,
        "AH should be decremented to 0x98"
    );
    assert!(cf_set(regs.rflags), "CF should be set");
    assert!(af_set(regs.rflags), "AF should be set");
}
