use crate::common::*;
use rax::cpu::Registers;

// AAA — ASCII Adjust After Addition
// AAS — ASCII Adjust After Subtraction
//
// Opcodes:
// - 37       AAA       ASCII adjust AL after addition
// - 3F       AAS       ASCII adjust AL after subtraction
//
// AAA Operation:
//   IF ((AL AND 0FH) > 9) OR (AF = 1) THEN
//     AX := AX + 106H;
//     AF := 1;
//     CF := 1;
//   ELSE
//     AF := 0;
//     CF := 0;
//   FI;
//   AL := AL AND 0FH;
//
// AAS Operation:
//   IF ((AL AND 0FH) > 9) OR (AF = 1) THEN
//     AX := AX - 6;
//     AH := AH - 1;
//     AF := 1;
//     CF := 1;
//   ELSE
//     AF := 0;
//     CF := 0;
//   FI;
//   AL := AL AND 0FH;
//
// Flags: AF and CF are modified. OF, SF, ZF, PF are undefined.

// ============================================================================
// AAA (ASCII Adjust After Addition) Tests
// ============================================================================

#[test]
fn test_aaa_no_adjustment_needed() {
    // AL = 5 (0x05), low nibble <= 9 and AF = 0
    let code = [
        0x37, // AAA
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x0005;
    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x05, "AL should remain 5");
    assert_eq!((regs.rax >> 8) & 0xFF, 0x00, "AH should remain 0");
    assert!(!cf_set(regs.rflags), "CF should be clear");
    assert!(!af_set(regs.rflags), "AF should be clear");
}

#[test]
fn test_aaa_adjustment_needed_low_nibble() {
    // AL = 0x0A (low nibble > 9)
    let code = [0x37, 0xf4]; // AAA, HLT
    let mut regs = Registers::default();
    regs.rax = 0x000A;
    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // AX = 0x000A + 0x0106 = 0x0110, then AL masked to 0x0F -> 0x00
    assert_eq!(regs.rax & 0xFF, 0x00, "AL should be 0 after masking");
    assert_eq!(
        (regs.rax >> 8) & 0xFF,
        0x01,
        "AH should be incremented to 1"
    );
    assert!(cf_set(regs.rflags), "CF should be set");
    assert!(af_set(regs.rflags), "AF should be set");
}

#[test]
fn test_aaa_adjustment_needed_af_set() {
    // AL = 5, but AF is set (previous operation had auxiliary carry)
    let code = [0x37, 0xf4]; // AAA, HLT
    let mut regs = Registers::default();
    regs.rax = 0x0005;
    regs.rflags = 0x10; // Set AF flag
    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // AX = 0x0005 + 0x0106 = 0x010B, then AL masked to 0x0F -> 0x0B
    assert_eq!(regs.rax & 0xFF, 0x0B, "AL should be 0x0B after masking");
    assert_eq!((regs.rax >> 8) & 0xFF, 0x01, "AH should be incremented");
    assert!(cf_set(regs.rflags), "CF should be set");
    assert!(af_set(regs.rflags), "AF should be set");
}

#[test]
fn test_aaa_all_digits_0_through_9() {
    for digit in 0..=9 {
        let code = [0x37, 0xf4]; // AAA, HLT
        let mut regs = Registers::default();
        regs.rax = digit;
        let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        assert_eq!(regs.rax & 0xFF, digit, "AL should remain {}", digit);
        assert!(
            !cf_set(regs.rflags),
            "CF should be clear for digit {}",
            digit
        );
        assert!(
            !af_set(regs.rflags),
            "AF should be clear for digit {}",
            digit
        );
    }
}

#[test]
fn test_aaa_values_0a_through_0f() {
    for val in 0x0A..=0x0F {
        let code = [0x37, 0xf4]; // AAA, HLT
        let mut regs = Registers::default();
        regs.rax = val;
        let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        let expected_al = val.wrapping_add(6) & 0x0F;
        assert_eq!(
            regs.rax & 0xFF,
            expected_al,
            "AL should be masked for value 0x{:02X}",
            val
        );
        assert_eq!(
            (regs.rax >> 8) & 0xFF,
            0x01,
            "AH should be 1 for value 0x{:02X}",
            val
        );
        assert!(
            cf_set(regs.rflags),
            "CF should be set for value 0x{:02X}",
            val
        );
        assert!(
            af_set(regs.rflags),
            "AF should be set for value 0x{:02X}",
            val
        );
    }
}

#[test]
fn test_aaa_with_initial_ah() {
    // AH = 5, AL = 0x0A
    let code = [0x37, 0xf4]; // AAA, HLT
    let mut regs = Registers::default();
    regs.rax = 0x050A; // AH=5, AL=0x0A
    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // AX = 0x050A + 0x0106 = 0x0610, then AL masked to 0x0F -> 0x00
    assert_eq!(regs.rax & 0xFF, 0x00, "AL should be 0");
    assert_eq!((regs.rax >> 8) & 0xFF, 0x06, "AH should be 6");
    assert!(cf_set(regs.rflags), "CF should be set");
}

#[test]
fn test_aaa_bcd_addition_example() {
    // Simulating BCD addition: 8 + 7 = 15 (decimal)
    // In BCD: 0x08 + 0x07 = 0x0F, AAA adjusts to 0x0105 (1 and 5 in BCD)
    let code = [0x37, 0xf4]; // AAA, HLT
    let mut regs = Registers::default();
    regs.rax = 0x000F; // Result of 8 + 7
    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x05, "AL should be 5 (ones digit)");
    assert_eq!((regs.rax >> 8) & 0xFF, 0x01, "AH should be 1 (tens digit)");
}

#[test]
fn test_aaa_preserves_high_bits() {
    let code = [0x37, 0xf4]; // AAA, HLT
    let mut regs = Registers::default();
    regs.rax = 0xDEADBEEF_12340A0F;
    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Only AX (lower 16 bits) should be modified
    assert_eq!(
        regs.rax >> 16,
        0xDEADBEEF_1234,
        "High bits should be preserved"
    );
}

#[test]
fn test_aaa_max_al_value() {
    // AL = 0xFF (both nibbles high)
    let code = [0x37, 0xf4]; // AAA, HLT
    let mut regs = Registers::default();
    regs.rax = 0x00FF;
    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // 0x00FF + 0x0106 = 0x0205, AL masked to 0x0F -> 0x05
    assert_eq!(regs.rax & 0xFF, 0x05, "AL should be 5 after masking");
    assert_eq!((regs.rax >> 8) & 0xFF, 0x02, "AH should be 2");
}

#[test]
fn test_aaa_ah_overflow() {
    // AH = 0xFF, AL = 0x0A (causes AH to wrap)
    let code = [0x37, 0xf4]; // AAA, HLT
    let mut regs = Registers::default();
    regs.rax = 0xFF0A;
    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // 0xFF0A + 0x0106 = 0x10010, AX wraps to 0x0010, then AL masked to 0x00
    assert_eq!(regs.rax & 0xFFFF, 0x0000, "AX should wrap and mask to 0");
}

// ============================================================================
// AAS (ASCII Adjust After Subtraction) Tests
// ============================================================================

#[test]
fn test_aas_no_adjustment_needed() {
    // AL = 5 (0x05), low nibble <= 9 and AF = 0
    let code = [
        0x3F, // AAS
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x0005;
    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x05, "AL should remain 5");
    assert_eq!((regs.rax >> 8) & 0xFF, 0x00, "AH should remain 0");
    assert!(!cf_set(regs.rflags), "CF should be clear");
    assert!(!af_set(regs.rflags), "AF should be clear");
}

#[test]
fn test_aas_adjustment_needed_low_nibble() {
    // AL = 0x0F (low nibble > 9)
    let code = [0x3F, 0xf4]; // AAS, HLT
    let mut regs = Registers::default();
    regs.rax = 0x000F;
    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // AL = AL - 6 = 0x0F - 6 = 0x09, then masked to 0x0F -> 0x09
    // AH = AH - 1 = 0x00 - 1 = 0xFF
    assert_eq!(regs.rax & 0xFF, 0x09, "AL should be 9");
    assert_eq!(
        (regs.rax >> 8) & 0xFF,
        0xFF,
        "AH should be 0xFF (decremented)"
    );
    assert!(cf_set(regs.rflags), "CF should be set");
    assert!(af_set(regs.rflags), "AF should be set");
}

#[test]
fn test_aas_adjustment_needed_af_set() {
    // AL = 5, but AF is set
    let code = [0x3F, 0xf4]; // AAS, HLT
    let mut regs = Registers::default();
    regs.rax = 0x0005;
    regs.rflags = 0x10; // Set AF flag
    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // AL = AL - 6 = 5 - 6 = -1 = 0xFF, masked to 0x0F -> 0x0F
    // AH borrows from AL subtraction, then decrements by 1
    assert_eq!(regs.rax & 0xFF, 0x0F, "AL should be 0x0F");
    assert_eq!(
        (regs.rax >> 8) & 0xFF,
        0xFE,
        "AH should be decremented with borrow"
    );
    assert!(cf_set(regs.rflags), "CF should be set");
    assert!(af_set(regs.rflags), "AF should be set");
}

#[test]
fn test_aas_all_digits_0_through_9() {
    for digit in 0..=9 {
        let code = [0x3F, 0xf4]; // AAS, HLT
        let mut regs = Registers::default();
        regs.rax = digit;
        let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        assert_eq!(regs.rax & 0xFF, digit, "AL should remain {}", digit);
        assert!(
            !cf_set(regs.rflags),
            "CF should be clear for digit {}",
            digit
        );
        assert!(
            !af_set(regs.rflags),
            "AF should be clear for digit {}",
            digit
        );
    }
}

#[test]
fn test_aas_values_0a_through_0f() {
    for val in 0x0A..=0x0F {
        let code = [0x3F, 0xf4]; // AAS, HLT
        let mut regs = Registers::default();
        regs.rax = val;
        let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        let expected_al = ((val as i8 - 6) as u8) & 0x0F;
        assert_eq!(
            regs.rax & 0xFF,
            expected_al as u64,
            "AL should be adjusted for value 0x{:02X}",
            val
        );
        assert_eq!(
            (regs.rax >> 8) & 0xFF,
            0xFF,
            "AH should be 0xFF for value 0x{:02X}",
            val
        );
        assert!(
            cf_set(regs.rflags),
            "CF should be set for value 0x{:02X}",
            val
        );
        assert!(
            af_set(regs.rflags),
            "AF should be set for value 0x{:02X}",
            val
        );
    }
}

#[test]
fn test_aas_with_initial_ah() {
    // AH = 5, AL = 0x0A
    let code = [0x3F, 0xf4]; // AAS, HLT
    let mut regs = Registers::default();
    regs.rax = 0x050A;
    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // AL = 0x0A - 6 = 4, masked to 0x0F -> 0x04
    // AH = 5 - 1 = 4
    assert_eq!(regs.rax & 0xFF, 0x04, "AL should be 4");
    assert_eq!((regs.rax >> 8) & 0xFF, 0x04, "AH should be 4");
    assert!(cf_set(regs.rflags), "CF should be set");
}

#[test]
fn test_aas_bcd_subtraction_example() {
    // Simulating BCD subtraction with borrow
    let code = [0x3F, 0xf4]; // AAS, HLT
    let mut regs = Registers::default();
    regs.rax = 0x020C; // Simulating 12 - 6 with intermediate result
    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // AL = 0x0C - 6 = 6, masked to 0x0F -> 0x06
    // AH = 2 - 1 = 1
    assert_eq!(regs.rax & 0xFF, 0x06, "AL should be 6");
    assert_eq!((regs.rax >> 8) & 0xFF, 0x01, "AH should be 1");
}

#[test]
fn test_aas_preserves_high_bits() {
    let code = [0x3F, 0xf4]; // AAS, HLT
    let mut regs = Registers::default();
    regs.rax = 0xDEADBEEF_12340F0F;
    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Only AX (lower 16 bits) should be modified
    assert_eq!(
        regs.rax >> 16,
        0xDEADBEEF_1234,
        "High bits should be preserved"
    );
}

#[test]
fn test_aas_zero_ah() {
    // AH = 0, AL = 0x0E (requires decrement of AH)
    let code = [0x3F, 0xf4]; // AAS, HLT
    let mut regs = Registers::default();
    regs.rax = 0x000E;
    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // AL = 0x0E - 6 = 8, masked to 0x0F -> 0x08
    // AH = 0 - 1 = 0xFF (wraps)
    assert_eq!(regs.rax & 0xFF, 0x08, "AL should be 8");
    assert_eq!((regs.rax >> 8) & 0xFF, 0xFF, "AH should wrap to 0xFF");
}

#[test]
fn test_aas_max_ah() {
    // AH = 0xFF, AL = 0x0B
    let code = [0x3F, 0xf4]; // AAS, HLT
    let mut regs = Registers::default();
    regs.rax = 0xFF0B;
    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // AL = 0x0B - 6 = 5, masked to 0x0F -> 0x05
    // AH = 0xFF - 1 = 0xFE
    assert_eq!(regs.rax & 0xFF, 0x05, "AL should be 5");
    assert_eq!((regs.rax >> 8) & 0xFF, 0xFE, "AH should be 0xFE");
}

// ============================================================================
// Sequential AAA/AAS Tests
// ============================================================================

#[test]
fn test_aaa_then_aas() {
    // Test AAA followed by AAS
    let code = [
        0x37, // AAA
        0x3F, // AAS
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x000E;
    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // After AAA: 0x000E + 0x0106 = 0x0114, masked -> AH=1, AL=4 (AF set)
    // After AAS: adjustment due to AF set -> AX becomes 0xFF0E
    assert_eq!(regs.rax & 0xFFFF, 0xFF0E, "Result should be 0xFF0E");
}

#[test]
fn test_multiple_aaa_operations() {
    // Chain multiple AAA operations
    let code = [
        0x37, // AAA
        0x37, // AAA
        0x37, // AAA
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x000F;
    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Each AAA processes the current state
    // First AAA: 0x000F + 0x0106 = 0x0115, masked -> 0x0105
    // Second AAA: 0x0105 + 0x0106 = 0x020B (AL=5 adjusted), masked -> 0x020B
    // Third AAA: 0x020B + 0x0106 = 0x0311, masked -> 0x0301
    assert_eq!((regs.rax >> 8) & 0xFF, 0x03, "AH should be 3");
}

// ============================================================================
// Edge Cases and Corner Cases
// ============================================================================

#[test]
fn test_aaa_with_all_flags_set() {
    let code = [0x37, 0xf4]; // AAA, HLT
    let mut regs = Registers::default();
    regs.rax = 0x0003;
    regs.rflags = 0xFFF; // Set all flags
    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // AF was set, so adjustment occurs: 0x0003 + 0x0106 = 0x0109, masked -> 0x0109
    assert_eq!(regs.rax & 0xFF, 0x09, "AL should be 9");
    assert_eq!((regs.rax >> 8) & 0xFF, 0x01, "AH should be 1");
}

#[test]
fn test_aas_with_all_flags_set() {
    let code = [0x3F, 0xf4]; // AAS, HLT
    let mut regs = Registers::default();
    regs.rax = 0x0508;
    regs.rflags = 0xFFF; // Set all flags
    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // AF was set, so adjustment occurs: AL = 8 - 6 = 2, AH = 5 - 1 = 4
    assert_eq!(regs.rax & 0xFF, 0x02, "AL should be 2");
    assert_eq!((regs.rax >> 8) & 0xFF, 0x04, "AH should be 4");
}

#[test]
fn test_aaa_zero_value() {
    let code = [0x37, 0xf4]; // AAA, HLT
    let mut regs = Registers::default();
    regs.rax = 0x0000;
    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFF, 0x0000, "AX should remain 0");
    assert!(!cf_set(regs.rflags), "CF should be clear");
    assert!(!af_set(regs.rflags), "AF should be clear");
}

#[test]
fn test_aas_zero_value() {
    let code = [0x3F, 0xf4]; // AAS, HLT
    let mut regs = Registers::default();
    regs.rax = 0x0000;
    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFF, 0x0000, "AX should remain 0");
    assert!(!cf_set(regs.rflags), "CF should be clear");
    assert!(!af_set(regs.rflags), "AF should be clear");
}

#[test]
fn test_aaa_boundary_9_to_10() {
    // Test transition from 9 (no adjust) to 10 (adjust needed)
    let code = [0x37, 0xf4]; // AAA, HLT

    // Test 9: no adjustment
    let mut regs = Registers::default();
    regs.rax = 0x0009;
    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFF, 0x09, "AL=9 needs no adjustment");
    assert!(!af_set(regs.rflags), "AF should be clear for 9");

    // Test 10 (0x0A): adjustment needed
    let mut regs = Registers::default();
    regs.rax = 0x000A;
    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFFFF, 0x0100, "AL=0x0A should adjust to 0x0100");
    assert!(af_set(regs.rflags), "AF should be set for 0x0A");
}

#[test]
fn test_aas_boundary_9_to_10() {
    // Test transition from 9 (no adjust) to 10 (adjust needed)
    let code = [0x3F, 0xf4]; // AAS, HLT

    // Test 9: no adjustment
    let mut regs = Registers::default();
    regs.rax = 0x0509;
    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFFFF, 0x0509, "AL=9 needs no adjustment");
    assert!(!af_set(regs.rflags), "AF should be clear for 9");

    // Test 10 (0x0A): adjustment needed
    let mut regs = Registers::default();
    regs.rax = 0x050A;
    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFFFF, 0x0404, "AL=0x0A should adjust");
    assert!(af_set(regs.rflags), "AF should be set for 0x0A");
}

#[test]
fn test_aaa_unpacked_bcd_chain() {
    // Test a chain representing unpacked BCD addition: 9 + 8 = 17
    let code = [
        0x37, // AAA on first result
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x0011; // 9 + 8 = 17 (0x11 in hex, adjust needed)
    regs.rflags = 0x10; // Set AF to reflect carry from prior addition
    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // 0x0011 + 0x0106 = 0x0117, masked -> 0x0107
    assert_eq!(regs.rax & 0xFF, 0x07, "AL should be 7 (ones digit)");
    assert_eq!((regs.rax >> 8) & 0xFF, 0x01, "AH should be 1 (tens digit)");
}

#[test]
fn test_aas_unpacked_bcd_chain() {
    // Test unpacked BCD subtraction with borrow
    let code = [
        0x3F, // AAS
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x030C; // Intermediate result needing adjustment
    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // AL = 0x0C - 6 = 6, AH = 3 - 1 = 2
    assert_eq!(regs.rax & 0xFF, 0x06, "AL should be 6");
    assert_eq!((regs.rax >> 8) & 0xFF, 0x02, "AH should be 2");
}

#[test]
fn test_aaa_masking_high_nibble() {
    // Verify that high nibble is always masked away
    let code = [0x37, 0xf4]; // AAA, HLT
    let mut regs = Registers::default();
    regs.rax = 0x00F5; // High nibble = F, low nibble = 5
    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // No adjustment (low nibble 5 <= 9), but high nibble masked
    assert_eq!(regs.rax & 0xFF, 0x05, "AL should be 5 (high nibble masked)");
}

#[test]
fn test_aas_masking_high_nibble() {
    // Verify that high nibble is always masked away
    let code = [0x3F, 0xf4]; // AAS, HLT
    let mut regs = Registers::default();
    regs.rax = 0x05F8; // High nibble = F, low nibble = 8
    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // No adjustment (low nibble 8 <= 9), but high nibble masked
    assert_eq!(regs.rax & 0xFF, 0x08, "AL should be 8 (high nibble masked)");
}
