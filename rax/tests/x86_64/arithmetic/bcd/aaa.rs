use crate::common::*;
use rax::cpu::{Registers, VCpu};

// AAA — ASCII Adjust After Addition
//
// Opcode: 37
// Instruction: AAA
// Op/En: ZO (no operands)
// 64-bit Mode: Invalid
// Compat/Leg Mode: Valid
//
// Description:
// Adjusts the sum of two unpacked BCD values to create an unpacked BCD result.
// The AL register is the implied source and destination operand.
// AAA is only useful when it follows an ADD instruction that adds (binary addition)
// two unpacked BCD values and stores a byte result in the AL register.
//
// Operation:
// IF ((AL AND 0FH) > 9) or (AF = 1)
// THEN
//     AX := AX + 106H;
//     AF := 1;
//     CF := 1;
// ELSE
//     AF := 0;
//     CF := 0;
// FI;
// AL := AL AND 0FH;
//
// Flags Affected:
// AF and CF are set to 1 if adjustment results in decimal carry; otherwise 0.
// OF, SF, ZF, and PF are undefined.

// ============================================================================
// AAA - Basic Adjustment Cases (No Carry)
// ============================================================================

#[test]
fn test_aaa_no_adjustment_needed() {
    // AL = 0x05 (valid BCD), no adjustment needed
    let code = [
        0x37, // AAA
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
fn test_aaa_zero() {
    // AL = 0x00, no adjustment needed
    let code = [0x37, 0xf4]; // AAA, HLT
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
fn test_aaa_max_valid_bcd() {
    // AL = 0x09 (max valid BCD digit), no adjustment needed
    let code = [0x37, 0xf4];
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
// AAA - Adjustment Required (Lower Nibble > 9)
// ============================================================================

#[test]
fn test_aaa_lower_nibble_0a() {
    // AL = 0x0A, requires adjustment
    let code = [0x37, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 0x000A;
    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFF,
        0x00,
        "AL should be 0x00 (0x0A + 0x06 = 0x10, masked to 0x00)"
    );
    assert_eq!(
        (regs.rax >> 8) & 0xFF,
        0x01,
        "AH should be incremented to 0x01"
    );
    assert!(cf_set(regs.rflags), "CF should be set");
    assert!(af_set(regs.rflags), "AF should be set");
}

#[test]
fn test_aaa_lower_nibble_0b() {
    // AL = 0x0B
    let code = [0x37, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 0x000B;
    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x01, "AL should be 0x01");
    assert_eq!(
        (regs.rax >> 8) & 0xFF,
        0x01,
        "AH should be incremented to 0x01"
    );
    assert!(cf_set(regs.rflags), "CF should be set");
    assert!(af_set(regs.rflags), "AF should be set");
}

#[test]
fn test_aaa_lower_nibble_0f() {
    // AL = 0x0F (max value for lower nibble)
    let code = [0x37, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 0x000F;
    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x05, "AL should be 0x05");
    assert_eq!(
        (regs.rax >> 8) & 0xFF,
        0x01,
        "AH should be incremented to 0x01"
    );
    assert!(cf_set(regs.rflags), "CF should be set");
    assert!(af_set(regs.rflags), "AF should be set");
}

// ============================================================================
// AAA - With Upper Nibble Set
// ============================================================================

#[test]
fn test_aaa_with_upper_nibble_1x() {
    // AL = 0x1A, upper nibble should be cleared
    let code = [0x37, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 0x001A;
    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFF,
        0x00,
        "AL should be 0x00 (upper nibble cleared)"
    );
    assert_eq!(
        (regs.rax >> 8) & 0xFF,
        0x01,
        "AH should be incremented to 0x01"
    );
    assert!(cf_set(regs.rflags), "CF should be set");
    assert!(af_set(regs.rflags), "AF should be set");
}

#[test]
fn test_aaa_with_upper_nibble_2x() {
    // AL = 0x2B
    let code = [0x37, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 0x002B;
    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x01, "AL should be 0x01");
    assert_eq!(
        (regs.rax >> 8) & 0xFF,
        0x01,
        "AH should be incremented to 0x01"
    );
    assert!(cf_set(regs.rflags), "CF should be set");
    assert!(af_set(regs.rflags), "AF should be set");
}

#[test]
fn test_aaa_with_upper_nibble_fx() {
    // AL = 0xFC
    let code = [0x37, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 0x00FC;
    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x02, "AL should be 0x02");
    assert_eq!(
        (regs.rax >> 8) & 0xFF,
        0x02,
        "AH should be incremented to 0x02"
    );
    assert!(cf_set(regs.rflags), "CF should be set");
    assert!(af_set(regs.rflags), "AF should be set");
}

// ============================================================================
// AAA - With Non-Zero AH
// ============================================================================

#[test]
fn test_aaa_ah_nonzero_no_adjust() {
    // AH = 0x05, AL = 0x03, no adjustment needed
    let code = [0x37, 0xf4];
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
fn test_aaa_ah_nonzero_adjust() {
    // AH = 0x07, AL = 0x0D, adjustment needed
    let code = [0x37, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 0x070D;
    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x03, "AL should be 0x03");
    assert_eq!(
        (regs.rax >> 8) & 0xFF,
        0x08,
        "AH should be incremented to 0x08"
    );
    assert!(cf_set(regs.rflags), "CF should be set");
    assert!(af_set(regs.rflags), "AF should be set");
}

#[test]
fn test_aaa_ah_ff() {
    // AH = 0xFF, AL = 0x0E, test AH overflow
    let code = [0x37, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 0xFF0E;
    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x04, "AL should be 0x04");
    assert_eq!((regs.rax >> 8) & 0xFF, 0x00, "AH should wrap to 0x00");
    assert!(cf_set(regs.rflags), "CF should be set");
    assert!(af_set(regs.rflags), "AF should be set");
}

// ============================================================================
// AAA - With AF Flag Set
// ============================================================================

#[test]
fn test_aaa_af_set_valid_bcd() {
    // AL = 0x05 (valid BCD), but AF is set - should still adjust
    let code = [0x37, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 0x0005;
    regs.rflags = 0x10; // Set AF
    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFF,
        0x0B,
        "AL should be 0x0B (0x05 + 0x06 = 0x0B)"
    );
    assert_eq!(
        (regs.rax >> 8) & 0xFF,
        0x01,
        "AH should be incremented to 0x01"
    );
    assert!(cf_set(regs.rflags), "CF should be set");
    assert!(af_set(regs.rflags), "AF should be set");
}

#[test]
fn test_aaa_af_set_zero() {
    // AL = 0x00, AF is set - should adjust
    let code = [0x37, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 0x0000;
    regs.rflags = 0x10; // Set AF
    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x06, "AL should be 0x06");
    assert_eq!(
        (regs.rax >> 8) & 0xFF,
        0x01,
        "AH should be incremented to 0x01"
    );
    assert!(cf_set(regs.rflags), "CF should be set");
    assert!(af_set(regs.rflags), "AF should be set");
}

#[test]
fn test_aaa_af_set_nine() {
    // AL = 0x09, AF is set - should adjust
    let code = [0x37, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 0x0009;
    regs.rflags = 0x10; // Set AF
    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFF,
        0x0F,
        "AL should be 0x0F (0x09 + 0x06 = 0x0F)"
    );
    assert_eq!(
        (regs.rax >> 8) & 0xFF,
        0x01,
        "AH should be incremented to 0x01"
    );
    assert!(cf_set(regs.rflags), "CF should be set");
    assert!(af_set(regs.rflags), "AF should be set");
}

// ============================================================================
// AAA - Realistic BCD Addition Examples
// ============================================================================

#[test]
fn test_aaa_after_add_3_plus_4() {
    // Simulate: 3 + 4 = 7 (no adjustment needed)
    let code = [
        0xb0, 0x03, // MOV AL, 3
        0x04, 0x04, // ADD AL, 4
        0x37, // AAA
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm_compat(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x07, "Result should be 7");
    assert_eq!((regs.rax >> 8) & 0xFF, 0x00, "AH should be 0");
    assert!(!cf_set(regs.rflags), "CF should be clear");
    assert!(!af_set(regs.rflags), "AF should be clear");
}

#[test]
fn test_aaa_after_add_5_plus_6() {
    // Simulate: 5 + 6 = 11 (0x0B) -> should adjust to 1 with carry
    let code = [
        0xb0, 0x05, // MOV AL, 5
        0x04, 0x06, // ADD AL, 6
        0x37, // AAA
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm_compat(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x01, "Result should be 1 (ones digit)");
    assert_eq!((regs.rax >> 8) & 0xFF, 0x01, "AH should be 1 (tens digit)");
    assert!(cf_set(regs.rflags), "CF should be set");
    assert!(af_set(regs.rflags), "AF should be set");
}

#[test]
fn test_aaa_after_add_9_plus_9() {
    // Simulate: 9 + 9 = 18 (0x12) -> should adjust to 8 with carry
    let code = [
        0xb0, 0x09, // MOV AL, 9
        0x04, 0x09, // ADD AL, 9
        0x37, // AAA
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm_compat(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x08, "Result should be 8 (ones digit)");
    assert_eq!((regs.rax >> 8) & 0xFF, 0x01, "AH should be 1 (tens digit)");
    assert!(cf_set(regs.rflags), "CF should be set");
    assert!(af_set(regs.rflags), "AF should be set");
}

#[test]
fn test_aaa_after_add_7_plus_8() {
    // Simulate: 7 + 8 = 15 (0x0F) -> should adjust to 5 with carry
    let code = [
        0xb0, 0x07, // MOV AL, 7
        0x04, 0x08, // ADD AL, 8
        0x37, // AAA
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm_compat(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x05, "Result should be 5 (ones digit)");
    assert_eq!((regs.rax >> 8) & 0xFF, 0x01, "AH should be 1 (tens digit)");
    assert!(cf_set(regs.rflags), "CF should be set");
    assert!(af_set(regs.rflags), "AF should be set");
}

// ============================================================================
// AAA - Multi-Digit BCD Addition
// ============================================================================

#[test]
fn test_aaa_multidigit_34_plus_45() {
    // Simulate: 34 + 45 = 79
    // First digit: 4 + 5 = 9 (no carry)
    let code = [
        0xb0, 0x04, // MOV AL, 4
        0x04, 0x05, // ADD AL, 5
        0x37, // AAA
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm_compat(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x09, "Ones digit should be 9");
    assert_eq!((regs.rax >> 8) & 0xFF, 0x00, "No carry to tens");
}

#[test]
fn test_aaa_multidigit_38_plus_47() {
    // Simulate: 38 + 47 = 85
    // First digit: 8 + 7 = 15 (0x0F) -> adjust to 5 with carry
    let code = [
        0xb0, 0x08, // MOV AL, 8
        0x04, 0x07, // ADD AL, 7
        0x37, // AAA
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm_compat(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x05, "Ones digit should be 5");
    assert_eq!((regs.rax >> 8) & 0xFF, 0x01, "Carry to tens");
}

// ============================================================================
// AAA - Edge Cases and Boundary Conditions
// ============================================================================

#[test]
fn test_aaa_all_lower_nibbles() {
    // Test all lower nibble values from 0x00 to 0x0F
    for val in 0x00..=0x0F {
        let code = [0x37, 0xf4];
        let mut regs = Registers::default();
        regs.rax = val;
        let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        if val <= 9 {
            assert_eq!(regs.rax & 0xFF, val, "AL should remain {:#04x}", val);
            assert!(!cf_set(regs.rflags), "CF should be clear for {:#04x}", val);
        } else {
            let expected = (val + 6) & 0x0F;
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
fn test_aaa_upper_bits_cleared() {
    // Verify that upper 4 bits of AL are always cleared
    let code = [0x37, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 0xAB;
    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xF0,
        0x00,
        "Upper 4 bits of AL should be cleared"
    );
}

#[test]
fn test_aaa_preserves_high_rax() {
    // Verify that bits above AX are preserved
    let code = [0x37, 0xf4];
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
fn test_aaa_sequential_operations() {
    // Test multiple AAA operations in sequence
    let code = [
        0xb0, 0x08, // MOV AL, 8
        0x04, 0x05, // ADD AL, 5 (result: 13 = 0x0D)
        0x37, // AAA (result: AL=3, AH=1)
        0x04, 0x07, // ADD AL, 7 (result: 10 = 0x0A)
        0x37, // AAA (result: AL=0, AH=2)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm_compat(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x00, "Final AL should be 0");
    assert_eq!((regs.rax >> 8) & 0xFF, 0x02, "Final AH should be 2");
}

#[test]
fn test_aaa_with_initial_carry() {
    // Simulate addition with initial carry from previous operation
    let code = [
        0xb8, 0x08, 0x00, // MOV AX, 0x0008 (AH=0, AL=8)
        0x04, 0x09, // ADD AL, 9 (result: 17 = 0x11)
        0x37, // AAA (result: AL=7, AH=1)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm_compat(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x07, "AL should be 7");
    assert_eq!((regs.rax >> 8) & 0xFF, 0x01, "AH should be 1");
}

#[test]
fn test_aaa_max_value_ff() {
    // AL = 0xFF (all bits set)
    let code = [0x37, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 0x00FF;
    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x05, "AL should be 0x05");
    assert_eq!((regs.rax >> 8) & 0xFF, 0x02, "AH should be 0x02");
    assert!(cf_set(regs.rflags), "CF should be set");
    assert!(af_set(regs.rflags), "AF should be set");
}
