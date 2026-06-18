use crate::common::*;
use rax::cpu::{Registers, VCpu};

// DAS — Decimal Adjust AL After Subtraction
//
// Opcode: 2F
// Instruction: DAS
// Op/En: ZO (no operands)
// 64-bit Mode: Invalid
// Compat/Leg Mode: Valid
//
// Description:
// Adjusts the result of the subtraction of two packed BCD values to create a
// packed BCD result. The AL register is the implied source and destination.
// DAS is only useful when it follows a SUB instruction that subtracts (binary
// subtraction) one 2-digit, packed BCD value from another and stores a byte
// result in the AL register.
//
// Operation:
// old_AL := AL;
// old_CF := CF;
// CF := 0;
// IF (((AL AND 0FH) > 9) or AF = 1) THEN
//     AL := AL - 6;
//     CF := old_CF or (Borrow from AL := AL - 6);
//     AF := 1;
// ELSE
//     AF := 0;
// FI;
// IF ((old_AL > 99H) or (old_CF = 1)) THEN
//     AL := AL - 60H;
//     CF := 1;
// FI;
//
// Flags Affected:
// CF and AF are set if adjustment results in decimal borrow in either digit.
// SF, ZF, and PF are set according to result. OF is undefined.

// ============================================================================
// DAS - Basic Cases (No Adjustment)
// ============================================================================

#[test]
fn test_das_no_adjustment() {
    // AL = 0x25 (valid packed BCD), no adjustment needed
    let code = [
        0x2f, // DAS
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
fn test_das_zero() {
    // AL = 0x00
    let code = [0x2f, 0xf4];
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
fn test_das_valid_bcd_values() {
    // Test various valid packed BCD values (no adjustment needed)
    let valid_bcd = [0x00, 0x09, 0x10, 0x19, 0x25, 0x33, 0x44, 0x58, 0x67, 0x99];

    for val in valid_bcd.iter() {
        let code = [0x2f, 0xf4];
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
// DAS - Lower Nibble Adjustment
// ============================================================================

#[test]
fn test_das_lower_nibble_0a() {
    // AL = 0x0A (lower nibble > 9), needs adjustment
    let code = [0x2f, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 0x0A;
    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x04, "AL should be 0x04 (0x0A - 0x06)");
    assert!(!cf_set(regs.rflags), "CF should be clear");
    assert!(af_set(regs.rflags), "AF should be set");
}

#[test]
fn test_das_lower_nibble_0f() {
    // AL = 0x0F (lower nibble = 15), needs adjustment
    let code = [0x2f, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 0x0F;
    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x09, "AL should be 0x09 (0x0F - 0x06)");
    assert!(!cf_set(regs.rflags), "CF should be clear");
    assert!(af_set(regs.rflags), "AF should be set");
}

#[test]
fn test_das_lower_nibble_1c() {
    // AL = 0x1C (lower nibble = 12), needs adjustment
    let code = [0x2f, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 0x1C;
    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x16, "AL should be 0x16 (0x1C - 0x06)");
    assert!(!cf_set(regs.rflags), "CF should be clear");
    assert!(af_set(regs.rflags), "AF should be set");
}

// ============================================================================
// DAS - Upper Nibble Adjustment
// ============================================================================

#[test]
fn test_das_upper_nibble_a0() {
    // AL = 0xA0 (upper nibble > 9), needs adjustment
    let code = [0x2f, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 0xA0;
    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x40, "AL should be 0x40 (0xA0 - 0x60)");
    assert!(cf_set(regs.rflags), "CF should be set");
}

#[test]
fn test_das_upper_nibble_f0() {
    // AL = 0xF0 (upper nibble = 15), needs adjustment
    let code = [0x2f, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 0xF0;
    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x90, "AL should be 0x90 (0xF0 - 0x60)");
    assert!(cf_set(regs.rflags), "CF should be set");
}

// ============================================================================
// DAS - Both Nibbles Require Adjustment
// ============================================================================

#[test]
fn test_das_both_nibbles_ae() {
    // AL = 0xAE (both nibbles > 9), needs both adjustments
    let code = [0x2f, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 0xAE;
    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // 0xAE - 0x06 = 0xA8, then 0xA8 - 0x60 = 0x48
    assert_eq!(regs.rax & 0xFF, 0x48, "AL should be 0x48");
    assert!(cf_set(regs.rflags), "CF should be set");
    assert!(af_set(regs.rflags), "AF should be set");
}

#[test]
fn test_das_both_nibbles_ff() {
    // AL = 0xFF (maximum value)
    let code = [0x2f, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 0xFF;
    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // 0xFF - 0x06 = 0xF9, then 0xF9 - 0x60 = 0x99
    assert_eq!(regs.rax & 0xFF, 0x99, "AL should be 0x99");
    assert!(cf_set(regs.rflags), "CF should be set");
    assert!(af_set(regs.rflags), "AF should be set");
}

// ============================================================================
// DAS - Realistic Packed BCD Subtraction Examples
// ============================================================================

#[test]
fn test_das_after_sub_79_minus_35() {
    // 79 - 35 = 44 (no borrow)
    let code = [
        0xb0, 0x79, // MOV AL, 0x79
        0x2c, 0x35, // SUB AL, 0x35
        0x2f, // DAS
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm_compat(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x44, "Result should be 0x44 (BCD 44)");
    assert!(!cf_set(regs.rflags), "CF should be clear");
}

#[test]
fn test_das_after_sub_35_minus_47() {
    // Example from Intel documentation: 35 - 47 (requires borrow)
    let code = [
        0xb0, 0x35, // MOV AL, 0x35
        0x2c, 0x47, // SUB AL, 0x47 (result: 0xEE with borrow)
        0x2f, // DAS (should produce 0x88 with CF=1)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm_compat(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x88, "Result should be 0x88");
    assert!(cf_set(regs.rflags), "CF should be set (borrow occurred)");
}

#[test]
fn test_das_after_sub_99_minus_01() {
    // 99 - 01 = 98
    let code = [
        0xb0, 0x99, // MOV AL, 0x99
        0x2c, 0x01, // SUB AL, 0x01 (result: 0x98)
        0x2f, // DAS
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm_compat(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x98, "Result should be 0x98 (BCD 98)");
    assert!(!cf_set(regs.rflags), "CF should be clear");
}

#[test]
fn test_das_after_sub_50_minus_25() {
    // 50 - 25 = 25
    let code = [
        0xb0, 0x50, // MOV AL, 0x50
        0x2c, 0x25, // SUB AL, 0x25 (result: 0x2B)
        0x2f, // DAS
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm_compat(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x25, "Result should be 0x25 (BCD 25)");
    assert!(!cf_set(regs.rflags), "CF should be clear");
}

#[test]
fn test_das_after_sub_88_minus_99() {
    // 88 - 99 (requires borrow)
    let code = [
        0xb0, 0x88, // MOV AL, 0x88
        0x2c, 0x99, // SUB AL, 0x99 (result: 0xEF with borrow)
        0x2f, // DAS
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm_compat(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x89, "Result should be 0x89");
    assert!(cf_set(regs.rflags), "CF should be set");
}

// ============================================================================
// DAS - With AF Flag Set
// ============================================================================

#[test]
fn test_das_af_set_valid_lower_nibble() {
    // AL = 0x25 (valid), but AF is set - should still adjust
    let code = [0x2f, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 0x25;
    regs.rflags = 0x10; // Set AF
    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x1F, "AL should be 0x1F (0x25 - 0x06)");
    assert!(!cf_set(regs.rflags), "CF should be clear");
    assert!(af_set(regs.rflags), "AF should be set");
}

#[test]
fn test_das_af_set_causes_underflow() {
    // AL = 0x05, AF set - causes underflow in lower nibble
    let code = [0x2f, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 0x05;
    regs.rflags = 0x10; // Set AF
    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // 0x05 - 0x06 = 0xFF (underflow)
    assert_eq!(regs.rax & 0xFF, 0xFF, "AL should be 0xFF");
    assert!(cf_set(regs.rflags), "CF should be set");
    assert!(af_set(regs.rflags), "AF should be set");
}

// ============================================================================
// DAS - With CF Flag Set
// ============================================================================

#[test]
fn test_das_cf_set_causes_upper_adjust() {
    // AL = 0x25, CF is set - should adjust upper nibble
    let code = [0x2f, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 0x25;
    regs.rflags = 0x01; // Set CF
    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFF,
        0xC5,
        "AL should be 0xC5 (0x25 - 0x60, wrapped)"
    );
    assert!(cf_set(regs.rflags), "CF should remain set");
}

#[test]
fn test_das_cf_set_with_underflow() {
    // AL = 0x50, CF is set
    let code = [0x2f, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 0x50;
    regs.rflags = 0x01; // Set CF
    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // 0x50 - 0x60 = 0xF0 (wrapped)
    assert_eq!(regs.rax & 0xFF, 0xF0, "AL should be 0xF0");
    assert!(cf_set(regs.rflags), "CF should be set");
}

// ============================================================================
// DAS - Multi-Digit Subtraction Simulation
// ============================================================================

#[test]
fn test_das_multidigit_85_minus_32() {
    // Ones: 5 - 2 = 3 (no borrow)
    let code = [
        0xb0, 0x05, // MOV AL, 5
        0x2c, 0x02, // SUB AL, 2
        0x2f, // DAS
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm_compat(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x03, "Ones digit should be 3");
    assert!(!cf_set(regs.rflags), "No borrow");
}

#[test]
fn test_das_multidigit_52_minus_37() {
    // Ones: 2 - 7 (requires borrow)
    let code = [
        0xb0, 0x02, // MOV AL, 2
        0x2c, 0x07, // SUB AL, 7 (result: 0xFB with borrow)
        0x2f, // DAS
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm_compat(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // 0xFB - 0x06 = 0xF5, then 0xF5 - 0x60 = 0x95
    assert_eq!(regs.rax & 0xFF, 0x95, "Result should be 0x95");
    assert!(cf_set(regs.rflags), "Borrow occurred");
}

// ============================================================================
// DAS - Edge Cases
// ============================================================================

#[test]
fn test_das_all_lower_nibbles() {
    // Test all lower nibble values
    for lower in 0..=0xF {
        let code = [0x2f, 0xf4];
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
            let expected = lower - 6;
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
fn test_das_preserves_high_rax() {
    // Verify that bits above AL are preserved
    let code = [0x2f, 0xf4];
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
fn test_das_flag_combinations() {
    // Test with both AF and CF set
    let code = [0x2f, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 0x88;
    regs.rflags = 0x11; // Set both AF and CF
    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // 0x88 - 0x06 = 0x82, then 0x82 - 0x60 = 0x22
    assert_eq!(regs.rax & 0xFF, 0x22, "AL should be 0x22");
    assert!(cf_set(regs.rflags), "CF should remain set");
    assert!(af_set(regs.rflags), "AF should be set");
}

#[test]
fn test_das_boundary_09() {
    // AL = 0x09 (boundary of lower nibble)
    let code = [0x2f, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 0x09;
    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x09, "AL should remain 0x09");
    assert!(!af_set(regs.rflags), "AF should be clear");
}

#[test]
fn test_das_boundary_99() {
    // AL = 0x99 (max valid BCD)
    let code = [0x2f, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 0x99;
    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x99, "AL should remain 0x99");
    assert!(!cf_set(regs.rflags), "CF should be clear");
}

#[test]
fn test_das_sequential_subtractions() {
    // Simulate subtracting multiple BCD numbers
    let code = [
        0xb0, 0x99, // MOV AL, 0x99
        0x2c, 0x15, // SUB AL, 0x15
        0x2f, // DAS (result: 0x84)
        0x2c, 0x27, // SUB AL, 0x27
        0x2f, // DAS (result: 0x57)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm_compat(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFF,
        0x57,
        "Final result should be 0x57 (BCD 57)"
    );
}

#[test]
fn test_das_with_borrow_propagation() {
    // Simulate multi-byte BCD subtraction with borrow
    let code = [
        0xb0, 0x00, // MOV AL, 0x00
        0x2c, 0x01, // SUB AL, 0x01 (result: 0xFF with borrow)
        0x2f, // DAS
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm_compat(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // 0xFF - 0x06 = 0xF9, then 0xF9 - 0x60 = 0x99
    assert_eq!(regs.rax & 0xFF, 0x99, "Result should be 0x99");
    assert!(
        cf_set(regs.rflags),
        "CF should be set (borrow to next byte)"
    );
}

#[test]
fn test_das_comprehensive_packed_bcd() {
    // Test various valid packed BCD subtractions
    let test_cases = [
        (0x79, 0x35, 0x44), // 79 - 35 = 44
        (0x68, 0x23, 0x45), // 68 - 23 = 45
        (0x99, 0x49, 0x50), // 99 - 49 = 50
        (0x77, 0x44, 0x33), // 77 - 44 = 33
    ];

    for (a, b, expected) in test_cases.iter() {
        let code = [
            0xb0, *a, // MOV AL, a
            0x2c, *b,   // SUB AL, b
            0x2f, // DAS
            0xf4, // HLT
        ];
        let (mut vcpu, _) = setup_vm_compat(&code, None);
        let regs = run_until_hlt(&mut vcpu).unwrap();

        assert_eq!(
            regs.rax & 0xFF,
            *expected as u64,
            "Result of {:#04x} - {:#04x} should be {:#04x}",
            a,
            b,
            expected
        );
    }
}

#[test]
fn test_das_underflow_cases() {
    // Test subtractions that result in negative (borrow required)
    let test_cases = [
        (0x10, 0x20), // 10 - 20 (needs borrow)
        (0x25, 0x50), // 25 - 50 (needs borrow)
        (0x00, 0x99), // 00 - 99 (needs borrow)
    ];

    for (a, b) in test_cases.iter() {
        let code = [
            0xb0, *a, // MOV AL, a
            0x2c, *b,   // SUB AL, b
            0x2f, // DAS
            0xf4, // HLT
        ];
        let (mut vcpu, _) = setup_vm_compat(&code, None);
        let regs = run_until_hlt(&mut vcpu).unwrap();

        assert!(
            cf_set(regs.rflags),
            "CF should be set for {:#04x} - {:#04x}",
            a,
            b
        );
    }
}

#[test]
fn test_das_exact_zero_result() {
    // 45 - 45 = 0
    let code = [
        0xb0, 0x45, // MOV AL, 0x45
        0x2c, 0x45, // SUB AL, 0x45
        0x2f, // DAS
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm_compat(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x00, "Result should be 0x00");
    assert!(zf_set(regs.rflags), "ZF should be set");
    assert!(!cf_set(regs.rflags), "CF should be clear");
}

#[test]
fn test_das_with_af_from_subtraction() {
    // Test DAS after subtraction that sets AF
    let code = [
        0xb0, 0x23, // MOV AL, 0x23
        0x2c, 0x05, // SUB AL, 0x05 (result: 0x1E, AF set)
        0x2f, // DAS
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm_compat(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // 0x1E - 0x06 = 0x18 (due to AF being set)
    assert_eq!(regs.rax & 0xFF, 0x18, "Result should be 0x18");
}
