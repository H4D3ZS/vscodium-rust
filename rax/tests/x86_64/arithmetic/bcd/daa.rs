use crate::common::*;
use rax::cpu::{Registers, VCpu};

// DAA — Decimal Adjust AL After Addition
//
// Opcode: 27
// Instruction: DAA
// Op/En: ZO (no operands)
// 64-bit Mode: Invalid
// Compat/Leg Mode: Valid
//
// Description:
// Adjusts the sum of two packed BCD values to create a packed BCD result.
// The AL register is the implied source and destination operand. DAA is only
// useful when it follows an ADD instruction that adds (binary addition) two
// 2-digit, packed BCD values and stores a byte result in the AL register.
//
// Operation:
// old_AL := AL;
// old_CF := CF;
// CF := 0;
// IF (((AL AND 0FH) > 9) or AF = 1) THEN
//     AL := AL + 6;
//     CF := old_CF or (Carry from AL := AL + 6);
//     AF := 1;
// ELSE
//     AF := 0;
// FI;
// IF ((old_AL > 99H) or (old_CF = 1)) THEN
//     AL := AL + 60H;
//     CF := 1;
// ELSE
//     CF := 0;
// FI;
//
// Flags Affected:
// CF and AF are set if adjustment results in decimal carry in either digit.
// SF, ZF, and PF are set according to result. OF is undefined.

// ============================================================================
// DAA - Basic Cases (No Adjustment)
// ============================================================================

#[test]
fn test_daa_no_adjustment() {
    // AL = 0x25 (valid packed BCD), no adjustment needed
    let code = [
        0x27, // DAA
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x25;
    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x25, "AL should remain 0x25");
    assert!(!cf_set(regs.rflags), "CF should be clear");
    assert!(!af_set(regs.rflags), "AF should be clear");
}

#[test]
fn test_daa_zero() {
    // AL = 0x00
    let code = [0x27, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 0x00;
    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x00, "AL should remain 0x00");
    assert!(!cf_set(regs.rflags), "CF should be clear");
    assert!(!af_set(regs.rflags), "AF should be clear");
    assert!(zf_set(regs.rflags), "ZF should be set");
}

#[test]
fn test_daa_valid_bcd_values() {
    // Test various valid packed BCD values (no adjustment needed)
    let valid_bcd = [0x00, 0x09, 0x10, 0x19, 0x25, 0x33, 0x44, 0x58, 0x67, 0x99];

    for val in valid_bcd.iter() {
        let code = [0x27, 0xf4];
        let mut regs = Registers::default();
        regs.rax = *val as u64;
        let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        assert_eq!(
            regs.rax & 0xFF,
            *val as u64,
            "AL should remain {:#04x}",
            val
        );
        assert!(!cf_set(regs.rflags), "CF should be clear for {:#04x}", val);
        assert!(!af_set(regs.rflags), "AF should be clear for {:#04x}", val);
    }
}

// ============================================================================
// DAA - Lower Nibble Adjustment
// ============================================================================

#[test]
fn test_daa_lower_nibble_0a() {
    // AL = 0x0A (lower nibble > 9), needs adjustment
    let code = [0x27, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 0x0A;
    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x10, "AL should be 0x10 (0x0A + 0x06)");
    assert!(!cf_set(regs.rflags), "CF should be clear");
    assert!(af_set(regs.rflags), "AF should be set");
}

#[test]
fn test_daa_lower_nibble_0f() {
    // AL = 0x0F (lower nibble = 15), needs adjustment
    let code = [0x27, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 0x0F;
    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x15, "AL should be 0x15 (0x0F + 0x06)");
    assert!(!cf_set(regs.rflags), "CF should be clear");
    assert!(af_set(regs.rflags), "AF should be set");
}

#[test]
fn test_daa_lower_nibble_1c() {
    // AL = 0x1C (lower nibble = 12), needs adjustment
    let code = [0x27, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 0x1C;
    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x22, "AL should be 0x22 (0x1C + 0x06)");
    assert!(!cf_set(regs.rflags), "CF should be clear");
    assert!(af_set(regs.rflags), "AF should be set");
}

// ============================================================================
// DAA - Upper Nibble Adjustment
// ============================================================================

#[test]
fn test_daa_upper_nibble_a0() {
    // AL = 0xA0 (upper nibble > 9), needs adjustment
    let code = [0x27, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 0xA0;
    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFF,
        0x00,
        "AL should be 0x00 (0xA0 + 0x60 = 0x100, wrapped)"
    );
    assert!(cf_set(regs.rflags), "CF should be set");
}

#[test]
fn test_daa_upper_nibble_f0() {
    // AL = 0xF0 (upper nibble = 15), needs adjustment
    let code = [0x27, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 0xF0;
    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFF,
        0x50,
        "AL should be 0x50 (0xF0 + 0x60 = 0x150, wrapped)"
    );
    assert!(cf_set(regs.rflags), "CF should be set");
}

// ============================================================================
// DAA - Both Nibbles Require Adjustment
// ============================================================================

#[test]
fn test_daa_both_nibbles_ae() {
    // AL = 0xAE (both nibbles > 9), needs both adjustments
    let code = [0x27, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 0xAE;
    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // 0xAE + 0x06 = 0xB4, then 0xB4 + 0x60 = 0x14 (wrapped)
    assert_eq!(regs.rax & 0xFF, 0x14, "AL should be 0x14");
    assert!(cf_set(regs.rflags), "CF should be set");
    assert!(af_set(regs.rflags), "AF should be set");
}

#[test]
fn test_daa_both_nibbles_ff() {
    // AL = 0xFF (maximum value)
    let code = [0x27, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 0xFF;
    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // 0xFF + 0x06 = 0x105 (wrapped to 0x05), then 0x05 + 0x60 = 0x65
    assert_eq!(regs.rax & 0xFF, 0x65, "AL should be 0x65");
    assert!(cf_set(regs.rflags), "CF should be set");
    assert!(af_set(regs.rflags), "AF should be set");
}

// ============================================================================
// DAA - Realistic Packed BCD Addition Examples
// ============================================================================

#[test]
fn test_daa_after_add_25_plus_34() {
    // 25 + 34 = 59 (no carry)
    let code = [
        0xb0, 0x25, // MOV AL, 0x25
        0x04, 0x34, // ADD AL, 0x34
        0x27, // DAA
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm_compat(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x59, "Result should be 0x59 (BCD 59)");
    assert!(!cf_set(regs.rflags), "CF should be clear");
}

#[test]
fn test_daa_after_add_79_plus_35() {
    // Example from Intel documentation: 79 + 35 = 114
    let code = [
        0xb0, 0x79, // MOV AL, 0x79
        0x04, 0x35, // ADD AL, 0x35 (result: 0xAE)
        0x27, // DAA (should produce 0x14 with CF=1)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm_compat(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFF,
        0x14,
        "Result should be 0x14 (ones place of 114)"
    );
    assert!(
        cf_set(regs.rflags),
        "CF should be set (carry to next digit)"
    );
}

#[test]
fn test_daa_after_add_58_plus_46() {
    // 58 + 46 = 104
    let code = [
        0xb0, 0x58, // MOV AL, 0x58
        0x04, 0x46, // ADD AL, 0x46 (result: 0x9E)
        0x27, // DAA
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm_compat(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFF,
        0x04,
        "Result should be 0x04 (ones place of 104)"
    );
    assert!(cf_set(regs.rflags), "CF should be set");
}

#[test]
fn test_daa_after_add_99_plus_99() {
    // 99 + 99 = 198
    let code = [
        0xb0, 0x99, // MOV AL, 0x99
        0x04, 0x99, // ADD AL, 0x99 (result: 0x132, wrapped to 0x32, CF=1)
        0x27, // DAA
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm_compat(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFF,
        0x98,
        "Result should be 0x98 (ones place of 198)"
    );
    assert!(cf_set(regs.rflags), "CF should be set");
}

#[test]
fn test_daa_after_add_15_plus_27() {
    // 15 + 27 = 42
    let code = [
        0xb0, 0x15, // MOV AL, 0x15
        0x04, 0x27, // ADD AL, 0x27 (result: 0x3C)
        0x27, // DAA
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm_compat(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x42, "Result should be 0x42 (BCD 42)");
    assert!(!cf_set(regs.rflags), "CF should be clear");
}

// ============================================================================
// DAA - With AF Flag Set
// ============================================================================

#[test]
fn test_daa_af_set_valid_lower_nibble() {
    // AL = 0x25 (valid), but AF is set - should still adjust
    let code = [0x27, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 0x25;
    regs.rflags = 0x10; // Set AF
    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x2B, "AL should be 0x2B (0x25 + 0x06)");
    assert!(!cf_set(regs.rflags), "CF should be clear");
    assert!(af_set(regs.rflags), "AF should be set");
}

#[test]
fn test_daa_af_set_causes_upper_adjust() {
    // AL = 0x99, AF set - causes cascade to upper nibble
    let code = [0x27, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 0x99;
    regs.rflags = 0x10; // Set AF
    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // 0x99 + 0x06 = 0x9F; upper adjust only applies when old AL > 0x99 or CF=1
    assert_eq!(regs.rax & 0xFF, 0x9F, "AL should be 0x9F");
    assert!(!cf_set(regs.rflags), "CF should be clear");
    assert!(af_set(regs.rflags), "AF should be set");
}

// ============================================================================
// DAA - With CF Flag Set
// ============================================================================

#[test]
fn test_daa_cf_set_causes_upper_adjust() {
    // AL = 0x25, CF is set - should adjust upper nibble
    let code = [0x27, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 0x25;
    regs.rflags = 0x01; // Set CF
    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x85, "AL should be 0x85 (0x25 + 0x60)");
    assert!(cf_set(regs.rflags), "CF should remain set");
}

#[test]
fn test_daa_cf_set_with_overflow() {
    // AL = 0xA5, CF is set
    let code = [0x27, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 0xA5;
    regs.rflags = 0x01; // Set CF
    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // 0xA5 + 0x60 = 0x105, wrapped to 0x05
    assert_eq!(regs.rax & 0xFF, 0x05, "AL should be 0x05");
    assert!(cf_set(regs.rflags), "CF should be set");
}

// ============================================================================
// DAA - Multi-Digit Addition Simulation
// ============================================================================

#[test]
fn test_daa_multidigit_12_plus_34() {
    // Ones: 2 + 4 = 6 (no carry)
    let code = [
        0xb0, 0x02, // MOV AL, 2
        0x04, 0x04, // ADD AL, 4
        0x27, // DAA
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm_compat(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x06, "Ones digit should be 6");
    assert!(!cf_set(regs.rflags), "No carry");
}

#[test]
fn test_daa_multidigit_28_plus_37() {
    // Ones: 8 + 7 = 15 -> 5 with carry
    let code = [
        0xb0, 0x08, // MOV AL, 8
        0x04, 0x07, // ADD AL, 7 (result: 0x0F)
        0x27, // DAA
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm_compat(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x15, "Result should be 0x15 (BCD 15)");
    assert!(!cf_set(regs.rflags), "CF should be clear (< 100)");
}

// ============================================================================
// DAA - Edge Cases
// ============================================================================

#[test]
fn test_daa_all_lower_nibbles() {
    // Test all lower nibble values
    for lower in 0..=0xF {
        let code = [0x27, 0xf4];
        let mut regs = Registers::default();
        regs.rax = lower;
        let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        if lower <= 9 {
            assert_eq!(regs.rax & 0xFF, lower, "AL should remain {:#04x}", lower);
            assert!(
                !af_set(regs.rflags),
                "AF should be clear for {:#04x}",
                lower
            );
        } else {
            let expected = lower + 6;
            assert_eq!(
                regs.rax & 0xFF,
                expected,
                "AL should be {:#04x} for input {:#04x}",
                expected,
                lower
            );
            assert!(af_set(regs.rflags), "AF should be set for {:#04x}", lower);
        }
    }
}

#[test]
fn test_daa_preserves_high_rax() {
    // Verify that bits above AL are preserved
    let code = [0x27, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 0x1234_5678_DEAD_BE0A;
    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax >> 8,
        0x1234_5678_DEAD_BE,
        "High bits of RAX should be preserved"
    );
}

#[test]
fn test_daa_flag_combinations() {
    // Test with both AF and CF set
    let code = [0x27, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 0x88;
    regs.rflags = 0x11; // Set both AF and CF
    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // 0x88 + 0x06 = 0x8E, then 0x8E + 0x60 = 0xEE
    assert_eq!(regs.rax & 0xFF, 0xEE, "AL should be 0xEE");
    assert!(cf_set(regs.rflags), "CF should remain set");
    assert!(af_set(regs.rflags), "AF should be set");
}

#[test]
fn test_daa_boundary_09() {
    // AL = 0x09 (boundary of lower nibble)
    let code = [0x27, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 0x09;
    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x09, "AL should remain 0x09");
    assert!(!af_set(regs.rflags), "AF should be clear");
}

#[test]
fn test_daa_boundary_90() {
    // AL = 0x90 (boundary of upper nibble)
    let code = [0x27, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 0x90;
    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x90, "AL should remain 0x90");
    assert!(!cf_set(regs.rflags), "CF should be clear");
}

#[test]
fn test_daa_sequential_additions() {
    // Simulate adding multiple BCD numbers
    let code = [
        0xb0, 0x15, // MOV AL, 0x15
        0x04, 0x27, // ADD AL, 0x27
        0x27, // DAA (result: 0x42)
        0x04, 0x38, // ADD AL, 0x38
        0x27, // DAA (result: 0x80)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm_compat(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFF,
        0x80,
        "Final result should be 0x80 (BCD 80)"
    );
}

#[test]
fn test_daa_with_previous_carry() {
    // Simulate multi-byte BCD addition with carry
    let code = [
        0xb0, 0x99, // MOV AL, 0x99
        0x04, 0x01, // ADD AL, 0x01 (result: 0x9A)
        0x27, // DAA (should produce 0x00 with CF=1)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm_compat(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x00, "Result should be 0x00");
    assert!(cf_set(regs.rflags), "CF should be set (carry to next byte)");
}

#[test]
fn test_daa_comprehensive_packed_bcd() {
    // Test various valid packed BCD additions
    let test_cases = [
        (0x12, 0x34, 0x46), // 12 + 34 = 46
        (0x45, 0x23, 0x68), // 45 + 23 = 68
        (0x50, 0x49, 0x99), // 50 + 49 = 99
        (0x33, 0x44, 0x77), // 33 + 44 = 77
    ];

    for (a, b, expected) in test_cases.iter() {
        let code = [
            0xb0, *a, // MOV AL, a
            0x04, *b,   // ADD AL, b
            0x27, // DAA
            0xf4, // HLT
        ];
        let (mut vcpu, _) = setup_vm_compat(&code, None);
        let regs = run_until_hlt(&mut vcpu).unwrap();

        assert_eq!(
            regs.rax & 0xFF,
            *expected as u64,
            "Result of {:#04x} + {:#04x} should be {:#04x}",
            a,
            b,
            expected
        );
    }
}
