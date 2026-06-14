use crate::common::*;
use rax::cpu::Registers;

// AAM — ASCII Adjust AX After Multiply
// AAD — ASCII Adjust AX Before Division
//
// Opcodes:
// - D4 0A       AAM       ASCII adjust AX after multiply (base 10)
// - D4 ib       AAM imm8  ASCII adjust AX after multiply (custom base)
// - D5 0A       AAD       ASCII adjust AX before division (base 10)
// - D5 ib       AAD imm8  ASCII adjust AX before division (custom base)
//
// AAM Operation (base in imm8, usually 0x0A for decimal):
//   tempAL := AL;
//   AH := tempAL / imm8;  // Quotient
//   AL := tempAL MOD imm8; // Remainder
//
// AAD Operation (base in imm8, usually 0x0A for decimal):
//   tempAL := AL;
//   tempAH := AH;
//   AL := (tempAL + (tempAH * imm8)) AND 0FFH;
//   AH := 0;
//
// Flags: SF, ZF, PF are set according to result. CF, OF, AF are undefined.

// ============================================================================
// AAM (ASCII Adjust After Multiply) Tests
// ============================================================================

#[test]
fn test_aam_basic_decimal() {
    // AAM with base 10 (standard): AL = 35
    let code = [
        0xD4, 0x0A, // AAM (base 10)
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x0023; // AL = 35 decimal
    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // AH = 35 / 10 = 3, AL = 35 % 10 = 5
    assert_eq!((regs.rax >> 8) & 0xFF, 0x03, "AH should be 3 (quotient)");
    assert_eq!(regs.rax & 0xFF, 0x05, "AL should be 5 (remainder)");
    assert!(!zf_set(regs.rflags), "ZF should be clear");
    assert!(!sf_set(regs.rflags), "SF should be clear");
}

#[test]
fn test_aam_zero() {
    let code = [0xD4, 0x0A, 0xf4]; // AAM, HLT
    let mut regs = Registers::default();
    regs.rax = 0x0000;
    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // AH = 0 / 10 = 0, AL = 0 % 10 = 0
    assert_eq!(regs.rax & 0xFFFF, 0x0000, "AX should be 0");
    assert!(zf_set(regs.rflags), "ZF should be set for zero result");
}

#[test]
fn test_aam_product_of_single_digits() {
    // Simulate 7 * 8 = 56
    let code = [0xD4, 0x0A, 0xf4]; // AAM, HLT
    let mut regs = Registers::default();
    regs.rax = 0x0038; // AL = 56 (7 * 8)
    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // AH = 56 / 10 = 5, AL = 56 % 10 = 6
    assert_eq!((regs.rax >> 8) & 0xFF, 0x05, "AH should be 5");
    assert_eq!(regs.rax & 0xFF, 0x06, "AL should be 6");
}

#[test]
fn test_aam_max_al_value() {
    // AL = 255 (max value)
    let code = [0xD4, 0x0A, 0xf4]; // AAM, HLT
    let mut regs = Registers::default();
    regs.rax = 0x00FF;
    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // AH = 255 / 10 = 25, AL = 255 % 10 = 5
    assert_eq!((regs.rax >> 8) & 0xFF, 25, "AH should be 25");
    assert_eq!(regs.rax & 0xFF, 5, "AL should be 5");
}

#[test]
fn test_aam_all_single_digit_products() {
    // Test all products from 0-9 * 0-9
    for i in 0..=9 {
        for j in 0..=9 {
            let product = i * j;
            let code = [0xD4, 0x0A, 0xf4]; // AAM, HLT
            let mut regs = Registers::default();
            regs.rax = product as u64;
            let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
            let regs = run_until_hlt(&mut vcpu).unwrap();

            let expected_ah = product / 10;
            let expected_al = product % 10;
            assert_eq!(
                (regs.rax >> 8) & 0xFF,
                expected_ah as u64,
                "AH wrong for {} * {} = {}",
                i,
                j,
                product
            );
            assert_eq!(
                regs.rax & 0xFF,
                expected_al as u64,
                "AL wrong for {} * {} = {}",
                i,
                j,
                product
            );
        }
    }
}

#[test]
fn test_aam_base_16() {
    // AAM with base 16 (hexadecimal)
    let code = [0xD4, 0x10, 0xf4]; // AAM base 16
    let mut regs = Registers::default();
    regs.rax = 0x00AB; // 171 decimal
    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // AH = 171 / 16 = 10 (0x0A), AL = 171 % 16 = 11 (0x0B)
    assert_eq!((regs.rax >> 8) & 0xFF, 0x0A, "AH should be 0x0A");
    assert_eq!(regs.rax & 0xFF, 0x0B, "AL should be 0x0B");
}

#[test]
fn test_aam_base_2() {
    // AAM with base 2 (binary)
    let code = [0xD4, 0x02, 0xf4]; // AAM base 2
    let mut regs = Registers::default();
    regs.rax = 0x0005; // 5 decimal = 101 binary
    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // AH = 5 / 2 = 2, AL = 5 % 2 = 1
    assert_eq!((regs.rax >> 8) & 0xFF, 0x02, "AH should be 2");
    assert_eq!(regs.rax & 0xFF, 0x01, "AL should be 1");
}

#[test]
fn test_aam_base_3() {
    // AAM with base 3
    let code = [0xD4, 0x03, 0xf4]; // AAM base 3
    let mut regs = Registers::default();
    regs.rax = 0x000B; // 11 decimal
    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // AH = 11 / 3 = 3, AL = 11 % 3 = 2
    assert_eq!((regs.rax >> 8) & 0xFF, 0x03, "AH should be 3");
    assert_eq!(regs.rax & 0xFF, 0x02, "AL should be 2");
}

#[test]
fn test_aam_base_7() {
    // AAM with base 7
    let code = [0xD4, 0x07, 0xf4]; // AAM base 7
    let mut regs = Registers::default();
    regs.rax = 0x0032; // 50 decimal
    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // AH = 50 / 7 = 7, AL = 50 % 7 = 1
    assert_eq!((regs.rax >> 8) & 0xFF, 0x07, "AH should be 7");
    assert_eq!(regs.rax & 0xFF, 0x01, "AL should be 1");
}

#[test]
fn test_aam_preserves_high_bits() {
    let code = [0xD4, 0x0A, 0xf4]; // AAM, HLT
    let mut regs = Registers::default();
    regs.rax = 0xDEADBEEF_12345678;
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
fn test_aam_sign_flag() {
    // When AL result has bit 7 set, SF should be set
    let code = [0xD4, 0x0A, 0xf4]; // AAM, HLT
    let mut regs = Registers::default();
    regs.rax = 0x0088; // 136 decimal -> AH=13, AL=6
    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 6, "AL should be 6");
    assert!(!sf_set(regs.rflags), "SF should be clear when AL < 128");
}

#[test]
fn test_aam_parity_flag() {
    // Test parity flag based on AL value
    let code = [0xD4, 0x0A, 0xf4]; // AAM, HLT
    let mut regs = Registers::default();
    regs.rax = 0x000F; // 15 -> AH=1, AL=5 (0b00000101, even parity)
    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 5, "AL should be 5");
    assert!(pf_set(regs.rflags), "PF should be set for even parity");
}

#[test]
fn test_aam_ignores_initial_ah() {
    // AAM only uses AL, ignores initial AH value
    let code = [0xD4, 0x0A, 0xf4]; // AAM, HLT
    let mut regs = Registers::default();
    regs.rax = 0xFF23; // AH=0xFF, AL=35
    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Result based only on AL=35
    assert_eq!((regs.rax >> 8) & 0xFF, 0x03, "AH should be 3");
    assert_eq!(regs.rax & 0xFF, 0x05, "AL should be 5");
}

// ============================================================================
// AAD (ASCII Adjust Before Division) Tests
// ============================================================================

#[test]
fn test_aad_basic_decimal() {
    // AAD with base 10: AH=3, AL=5 (representing 35 in unpacked BCD)
    let code = [
        0xD5, 0x0A, // AAD (base 10)
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x0305; // AH=3, AL=5
    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // AL = (5 + (3 * 10)) & 0xFF = 35, AH = 0
    assert_eq!(regs.rax & 0xFF, 35, "AL should be 35");
    assert_eq!((regs.rax >> 8) & 0xFF, 0x00, "AH should be 0");
    assert!(!zf_set(regs.rflags), "ZF should be clear");
    assert!(!sf_set(regs.rflags), "SF should be clear");
}

#[test]
fn test_aad_zero() {
    let code = [0xD5, 0x0A, 0xf4]; // AAD, HLT
    let mut regs = Registers::default();
    regs.rax = 0x0000;
    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // AL = (0 + (0 * 10)) & 0xFF = 0, AH = 0
    assert_eq!(regs.rax & 0xFFFF, 0x0000, "AX should be 0");
    assert!(zf_set(regs.rflags), "ZF should be set for zero result");
}

#[test]
fn test_aad_max_unpacked_bcd() {
    // AH=9, AL=9 (representing 99)
    let code = [0xD5, 0x0A, 0xf4]; // AAD, HLT
    let mut regs = Registers::default();
    regs.rax = 0x0909;
    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // AL = (9 + (9 * 10)) & 0xFF = 99, AH = 0
    assert_eq!(regs.rax & 0xFF, 99, "AL should be 99");
    assert_eq!((regs.rax >> 8) & 0xFF, 0x00, "AH should be 0");
}

#[test]
fn test_aad_overflow_wrapping() {
    // Test overflow: AH=30, AL=0 -> 30 * 10 = 300, masked to 8 bits = 44
    let code = [0xD5, 0x0A, 0xf4]; // AAD, HLT
    let mut regs = Registers::default();
    regs.rax = 0x1E00; // AH=30, AL=0
    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // AL = (0 + (30 * 10)) & 0xFF = 300 & 0xFF = 44
    assert_eq!(regs.rax & 0xFF, 44, "AL should be 44 (300 & 0xFF)");
    assert_eq!((regs.rax >> 8) & 0xFF, 0x00, "AH should be 0");
}

#[test]
fn test_aad_all_two_digit_values() {
    // Test all values from 00-99
    for tens in 0..=9 {
        for ones in 0..=9 {
            let code = [0xD5, 0x0A, 0xf4]; // AAD, HLT
            let mut regs = Registers::default();
            regs.rax = ((tens << 8) | ones) as u64;
            let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
            let regs = run_until_hlt(&mut vcpu).unwrap();

            let expected = tens * 10 + ones;
            assert_eq!(
                regs.rax & 0xFF,
                expected as u64,
                "Wrong result for AH={}, AL={}",
                tens,
                ones
            );
            assert_eq!(
                (regs.rax >> 8) & 0xFF,
                0,
                "AH should be 0 for AH={}, AL={}",
                tens,
                ones
            );
        }
    }
}

#[test]
fn test_aad_base_16() {
    // AAD with base 16 (hexadecimal)
    let code = [0xD5, 0x10, 0xf4]; // AAD base 16
    let mut regs = Registers::default();
    regs.rax = 0x0A0B; // AH=0x0A, AL=0x0B
    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // AL = (11 + (10 * 16)) & 0xFF = 171 & 0xFF = 171
    assert_eq!(regs.rax & 0xFF, 171, "AL should be 171");
    assert_eq!((regs.rax >> 8) & 0xFF, 0x00, "AH should be 0");
}

#[test]
fn test_aad_base_2() {
    // AAD with base 2 (binary)
    let code = [0xD5, 0x02, 0xf4]; // AAD base 2
    let mut regs = Registers::default();
    regs.rax = 0x0201; // AH=2, AL=1 (represents binary 101)
    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // AL = (1 + (2 * 2)) & 0xFF = 5
    assert_eq!(regs.rax & 0xFF, 5, "AL should be 5");
    assert_eq!((regs.rax >> 8) & 0xFF, 0x00, "AH should be 0");
}

#[test]
fn test_aad_base_7() {
    // AAD with base 7
    let code = [0xD5, 0x07, 0xf4]; // AAD base 7
    let mut regs = Registers::default();
    regs.rax = 0x0701; // AH=7, AL=1 (represents 7*7 + 1 = 50)
    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // AL = (1 + (7 * 7)) & 0xFF = 50
    assert_eq!(regs.rax & 0xFF, 50, "AL should be 50");
    assert_eq!((regs.rax >> 8) & 0xFF, 0x00, "AH should be 0");
}

#[test]
fn test_aad_preserves_high_bits() {
    let code = [0xD5, 0x0A, 0xf4]; // AAD, HLT
    let mut regs = Registers::default();
    regs.rax = 0xDEADBEEF_12340305;
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
fn test_aad_sign_flag() {
    // When result >= 128, SF should be set
    let code = [0xD5, 0x0A, 0xf4]; // AAD, HLT
    let mut regs = Registers::default();
    regs.rax = 0x0D00; // AH=13, AL=0 -> 130 (bit 7 set)
    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 130, "AL should be 130");
    assert!(sf_set(regs.rflags), "SF should be set when AL >= 128");
}

#[test]
fn test_aad_parity_flag() {
    // Test parity flag
    let code = [0xD5, 0x0A, 0xf4]; // AAD, HLT
    let mut regs = Registers::default();
    regs.rax = 0x0105; // AH=1, AL=5 -> 15 (0b00001111, even parity)
    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 15, "AL should be 15");
    assert!(pf_set(regs.rflags), "PF should be set for even parity");
}

// ============================================================================
// AAM/AAD Combined Tests (Round-trip)
// ============================================================================

#[test]
fn test_aam_aad_roundtrip() {
    // AAM followed by AAD should be identity for values 0-99
    let code = [
        0xD4, 0x0A, // AAM
        0xD5, 0x0A, // AAD
        0xf4, // HLT
    ];
    for val in 0..100 {
        let mut regs = Registers::default();
        regs.rax = val;
        let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        assert_eq!(regs.rax & 0xFF, val, "Roundtrip failed for {}", val);
        assert_eq!((regs.rax >> 8) & 0xFF, 0, "AH should be 0 after roundtrip");
    }
}

#[test]
fn test_aad_aam_sequence() {
    // AAD converts unpacked BCD to binary, AAM converts back
    let code = [
        0xD5, 0x0A, // AAD (unpacked -> binary)
        0xD4, 0x0A, // AAM (binary -> unpacked)
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x0807; // 87 in unpacked BCD
    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // After AAD: AL=87, AH=0
    // After AAM: AH=8, AL=7 (back to unpacked)
    assert_eq!((regs.rax >> 8) & 0xFF, 0x08, "AH should be 8");
    assert_eq!(regs.rax & 0xFF, 0x07, "AL should be 7");
}

#[test]
fn test_multiply_with_aam() {
    // Simulate 6 * 7 = 42, then AAM to get unpacked BCD
    let code = [
        0xD4, 0x0A, // AAM
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 42; // Product of 6 * 7
    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!((regs.rax >> 8) & 0xFF, 4, "AH should be 4 (tens)");
    assert_eq!(regs.rax & 0xFF, 2, "AL should be 2 (ones)");
}

#[test]
fn test_division_with_aad() {
    // Prepare 87 in unpacked BCD, AAD to prepare for division
    let code = [
        0xD5, 0x0A, // AAD
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x0807; // 87 in unpacked BCD
    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 87, "AL should be 87 (ready for division)");
    assert_eq!((regs.rax >> 8) & 0xFF, 0, "AH should be 0");
}

// ============================================================================
// Edge Cases and Special Values
// ============================================================================

#[test]
fn test_aam_base_1() {
    // Base 1 would cause division by 1
    let code = [0xD4, 0x01, 0xf4]; // AAM base 1
    let mut regs = Registers::default();
    regs.rax = 0x000A; // 10
    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // AH = 10 / 1 = 10, AL = 10 % 1 = 0
    assert_eq!((regs.rax >> 8) & 0xFF, 10, "AH should be 10");
    assert_eq!(regs.rax & 0xFF, 0, "AL should be 0");
}

#[test]
fn test_aad_base_1() {
    // Base 1
    let code = [0xD5, 0x01, 0xf4]; // AAD base 1
    let mut regs = Registers::default();
    regs.rax = 0x0A05; // AH=10, AL=5
    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // AL = (5 + (10 * 1)) & 0xFF = 15
    assert_eq!(regs.rax & 0xFF, 15, "AL should be 15");
}

#[test]
fn test_aam_base_255() {
    // Maximum base value
    let code = [0xD4, 0xFF, 0xf4]; // AAM base 255
    let mut regs = Registers::default();
    regs.rax = 0x00FE; // 254
    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // AH = 254 / 255 = 0, AL = 254 % 255 = 254
    assert_eq!((regs.rax >> 8) & 0xFF, 0, "AH should be 0");
    assert_eq!(regs.rax & 0xFF, 254, "AL should be 254");
}

#[test]
fn test_aad_base_255() {
    // Maximum base value
    let code = [0xD5, 0xFF, 0xf4]; // AAD base 255
    let mut regs = Registers::default();
    regs.rax = 0x0101; // AH=1, AL=1
    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // AL = (1 + (1 * 255)) & 0xFF = 256 & 0xFF = 0
    assert_eq!(regs.rax & 0xFF, 0, "AL should be 0 (wrapped)");
}

#[test]
fn test_aam_all_bases() {
    // Test AAM with various bases
    for base in 2..=20 {
        let code = [0xD4, base, 0xf4]; // AAM with custom base
        let mut regs = Registers::default();
        let test_val = 100u8;
        regs.rax = test_val as u64;
        let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        let expected_ah = test_val / base;
        let expected_al = test_val % base;
        assert_eq!(
            (regs.rax >> 8) & 0xFF,
            expected_ah as u64,
            "AH wrong for base {}",
            base
        );
        assert_eq!(
            regs.rax & 0xFF,
            expected_al as u64,
            "AL wrong for base {}",
            base
        );
    }
}

#[test]
fn test_aad_large_ah_value() {
    // Test with large AH value causing overflow
    let code = [0xD5, 0x0A, 0xf4]; // AAD base 10
    let mut regs = Registers::default();
    regs.rax = 0xFF09; // AH=255, AL=9
    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // AL = (9 + (255 * 10)) & 0xFF = 2559 & 0xFF = 255
    assert_eq!(regs.rax & 0xFF, 255, "AL should be 255 (wrapped)");
}

#[test]
fn test_aam_zero_flag_combinations() {
    // Test zero flag with different values
    let test_cases = vec![
        (0u8, true),   // Should set ZF
        (10u8, true),  // 10 % 10 = 0, should set ZF
        (11u8, false), // 11 % 10 = 1, should clear ZF
    ];

    for (val, expect_zf) in test_cases {
        let code = [0xD4, 0x0A, 0xf4]; // AAM
        let mut regs = Registers::default();
        regs.rax = val as u64;
        let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        assert_eq!(
            zf_set(regs.rflags),
            expect_zf,
            "ZF incorrect for value {}",
            val
        );
    }
}

#[test]
fn test_aad_zero_flag_combinations() {
    // Test zero flag with different values
    let test_cases = vec![
        (0x0000u16, true),  // Should set ZF
        (0x0100u16, false), // Should clear ZF (AH=1, AL=0 -> 10)
        (0x0A00u16, false), // Should clear ZF
    ];

    for (val, expect_zf) in test_cases {
        let code = [0xD5, 0x0A, 0xf4]; // AAD
        let mut regs = Registers::default();
        regs.rax = val as u64;
        let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        assert_eq!(
            zf_set(regs.rflags),
            expect_zf,
            "ZF incorrect for value 0x{:04X}",
            val
        );
    }
}
