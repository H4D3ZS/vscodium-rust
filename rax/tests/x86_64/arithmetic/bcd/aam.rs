use crate::common::*;
use rax::cpu::{Registers, VCpu};

// AAM — ASCII Adjust AX After Multiply
//
// Opcode: D4 0A
// Instruction: AAM
// Op/En: ZO (no operands)
// 64-bit Mode: Invalid
// Compat/Leg Mode: Valid
//
// Also: D4 ib - AAM imm8 (adjust to number base imm8)
//
// Description:
// Adjusts the result of the multiplication of two unpacked BCD values to create
// a pair of unpacked (base 10) BCD values. The AX register is the implied source
// and destination operand. AAM is only useful when it follows a MUL instruction
// that multiplies (binary multiplication) two unpacked BCD values and stores a
// word result in the AX register.
//
// Operation:
// tempAL := AL;
// AH := tempAL / imm8;  (* imm8 is set to 0AH for the AAM mnemonic *)
// AL := tempAL MOD imm8;
//
// Flags Affected:
// SF, ZF, and PF are set according to the resulting binary value in the AL register.
// OF, AF, and CF are undefined.

// ============================================================================
// AAM - Basic Multiplication Results
// ============================================================================

#[test]
fn test_aam_zero() {
    // AL = 0, result: AH = 0, AL = 0
    let code = [
        0xd4, 0x0a, // AAM
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x00;
    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x00, "AL should be 0");
    assert_eq!((regs.rax >> 8) & 0xFF, 0x00, "AH should be 0");
    assert!(zf_set(regs.rflags), "ZF should be set");
    assert!(!sf_set(regs.rflags), "SF should be clear");
}

#[test]
fn test_aam_one() {
    // AL = 1, result: AH = 0, AL = 1
    let code = [0xd4, 0x0a, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 0x01;
    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x01, "AL should be 1");
    assert_eq!((regs.rax >> 8) & 0xFF, 0x00, "AH should be 0");
    assert!(!zf_set(regs.rflags), "ZF should be clear");
    assert!(!sf_set(regs.rflags), "SF should be clear");
}

#[test]
fn test_aam_nine() {
    // AL = 9, result: AH = 0, AL = 9
    let code = [0xd4, 0x0a, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 0x09;
    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x09, "AL should be 9");
    assert_eq!((regs.rax >> 8) & 0xFF, 0x00, "AH should be 0");
    assert!(!zf_set(regs.rflags), "ZF should be clear");
    assert!(!sf_set(regs.rflags), "SF should be clear");
}

#[test]
fn test_aam_ten() {
    // AL = 10 (0x0A), result: AH = 1, AL = 0
    let code = [0xd4, 0x0a, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 0x0A;
    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x00, "AL should be 0 (10 MOD 10)");
    assert_eq!((regs.rax >> 8) & 0xFF, 0x01, "AH should be 1 (10 / 10)");
    assert!(zf_set(regs.rflags), "ZF should be set (AL = 0)");
}

// ============================================================================
// AAM - Two Digit Results
// ============================================================================

#[test]
fn test_aam_12() {
    // AL = 12 (0x0C), result: AH = 1, AL = 2
    let code = [0xd4, 0x0a, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 0x0C;
    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x02, "AL should be 2");
    assert_eq!((regs.rax >> 8) & 0xFF, 0x01, "AH should be 1");
}

#[test]
fn test_aam_25() {
    // AL = 25 (0x19), result: AH = 2, AL = 5
    let code = [0xd4, 0x0a, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 0x19;
    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x05, "AL should be 5");
    assert_eq!((regs.rax >> 8) & 0xFF, 0x02, "AH should be 2");
}

#[test]
fn test_aam_45() {
    // AL = 45 (0x2D), result: AH = 4, AL = 5
    let code = [0xd4, 0x0a, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 0x2D;
    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x05, "AL should be 5");
    assert_eq!((regs.rax >> 8) & 0xFF, 0x04, "AH should be 4");
}

#[test]
fn test_aam_81() {
    // AL = 81 (0x51 = 9 * 9), result: AH = 8, AL = 1
    let code = [0xd4, 0x0a, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 0x51;
    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x01, "AL should be 1");
    assert_eq!((regs.rax >> 8) & 0xFF, 0x08, "AH should be 8");
}

#[test]
fn test_aam_99() {
    // AL = 99 (0x63), result: AH = 9, AL = 9
    let code = [0xd4, 0x0a, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 0x63;
    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x09, "AL should be 9");
    assert_eq!((regs.rax >> 8) & 0xFF, 0x09, "AH should be 9");
}

// ============================================================================
// AAM - Realistic Multiplication Examples
// ============================================================================

#[test]
fn test_aam_after_mul_3_times_4() {
    // Simulate: 3 * 4 = 12
    let code = [
        0xb0, 0x03, // MOV AL, 3
        0xb3, 0x04, // MOV BL, 4
        0xf6, 0xe3, // MUL BL
        0xd4, 0x0a, // AAM
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm_compat(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x02, "Ones digit should be 2");
    assert_eq!((regs.rax >> 8) & 0xFF, 0x01, "Tens digit should be 1");
}

#[test]
fn test_aam_after_mul_5_times_6() {
    // Simulate: 5 * 6 = 30
    let code = [
        0xb0, 0x05, // MOV AL, 5
        0xb3, 0x06, // MOV BL, 6
        0xf6, 0xe3, // MUL BL
        0xd4, 0x0a, // AAM
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm_compat(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x00, "Ones digit should be 0");
    assert_eq!((regs.rax >> 8) & 0xFF, 0x03, "Tens digit should be 3");
}

#[test]
fn test_aam_after_mul_7_times_8() {
    // Simulate: 7 * 8 = 56
    let code = [
        0xb0, 0x07, // MOV AL, 7
        0xb3, 0x08, // MOV BL, 8
        0xf6, 0xe3, // MUL BL
        0xd4, 0x0a, // AAM
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm_compat(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x06, "Ones digit should be 6");
    assert_eq!((regs.rax >> 8) & 0xFF, 0x05, "Tens digit should be 5");
}

#[test]
fn test_aam_after_mul_9_times_9() {
    // Simulate: 9 * 9 = 81
    let code = [
        0xb0, 0x09, // MOV AL, 9
        0xb3, 0x09, // MOV BL, 9
        0xf6, 0xe3, // MUL BL
        0xd4, 0x0a, // AAM
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm_compat(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x01, "Ones digit should be 1");
    assert_eq!((regs.rax >> 8) & 0xFF, 0x08, "Tens digit should be 8");
}

#[test]
fn test_aam_after_mul_8_times_7() {
    // Simulate: 8 * 7 = 56
    let code = [
        0xb0, 0x08, // MOV AL, 8
        0xb3, 0x07, // MOV BL, 7
        0xf6, 0xe3, // MUL BL
        0xd4, 0x0a, // AAM
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm_compat(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x06, "Ones digit should be 6");
    assert_eq!((regs.rax >> 8) & 0xFF, 0x05, "Tens digit should be 5");
}

// ============================================================================
// AAM - Large Values
// ============================================================================

#[test]
fn test_aam_100() {
    // AL = 100 (0x64), result: AH = 10, AL = 0
    let code = [0xd4, 0x0a, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 0x64;
    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x00, "AL should be 0");
    assert_eq!((regs.rax >> 8) & 0xFF, 0x0A, "AH should be 10");
}

#[test]
fn test_aam_127() {
    // AL = 127 (0x7F), result: AH = 12, AL = 7
    let code = [0xd4, 0x0a, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 0x7F;
    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x07, "AL should be 7");
    assert_eq!((regs.rax >> 8) & 0xFF, 0x0C, "AH should be 12");
}

#[test]
fn test_aam_200() {
    // AL = 200 (0xC8), result: AH = 20, AL = 0
    let code = [0xd4, 0x0a, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 0xC8;
    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x00, "AL should be 0");
    assert_eq!((regs.rax >> 8) & 0xFF, 0x14, "AH should be 20");
}

#[test]
fn test_aam_255() {
    // AL = 255 (0xFF), result: AH = 25, AL = 5
    let code = [0xd4, 0x0a, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 0xFF;
    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x05, "AL should be 5");
    assert_eq!((regs.rax >> 8) & 0xFF, 0x19, "AH should be 25");
}

// ============================================================================
// AAM - Custom Bases (imm8)
// ============================================================================

#[test]
fn test_aam_base_2() {
    // AL = 5, base 2: AH = 2, AL = 1 (5 = 2*2 + 1)
    let code = [
        0xd4, 0x02, // AAM 2
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x05;
    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x01, "AL should be 1");
    assert_eq!((regs.rax >> 8) & 0xFF, 0x02, "AH should be 2");
}

#[test]
fn test_aam_base_8_octal() {
    // AL = 25 (0x19), base 8: AH = 3, AL = 1 (25 = 3*8 + 1)
    let code = [
        0xd4, 0x08, // AAM 8 (octal)
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x19;
    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x01, "AL should be 1");
    assert_eq!((regs.rax >> 8) & 0xFF, 0x03, "AH should be 3");
}

#[test]
fn test_aam_base_12() {
    // AL = 37 (0x25), base 12: AH = 3, AL = 1 (37 = 3*12 + 1)
    let code = [
        0xd4, 0x0c, // AAM 12
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x25;
    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x01, "AL should be 1");
    assert_eq!((regs.rax >> 8) & 0xFF, 0x03, "AH should be 3");
}

#[test]
fn test_aam_base_16_hex() {
    // AL = 0xAB (171), base 16: AH = 10, AL = 11
    let code = [
        0xd4, 0x10, // AAM 16 (hex)
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0xAB;
    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x0B, "AL should be 11");
    assert_eq!((regs.rax >> 8) & 0xFF, 0x0A, "AH should be 10");
}

#[test]
fn test_aam_base_7() {
    // AL = 50 (0x32), base 7: AH = 7, AL = 1 (50 = 7*7 + 1)
    let code = [
        0xd4, 0x07, // AAM 7
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x32;
    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x01, "AL should be 1");
    assert_eq!((regs.rax >> 8) & 0xFF, 0x07, "AH should be 7");
}

// ============================================================================
// AAM - Flag Testing
// ============================================================================

#[test]
fn test_aam_flags_zero_result() {
    // AL = 10, result AL = 0 (should set ZF)
    let code = [0xd4, 0x0a, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 0x0A;
    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x00, "AL should be 0");
    assert!(zf_set(regs.rflags), "ZF should be set");
    assert!(!sf_set(regs.rflags), "SF should be clear");
}

#[test]
fn test_aam_flags_nonzero_result() {
    // AL = 25, result AL = 5 (should clear ZF)
    let code = [0xd4, 0x0a, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 0x19;
    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x05, "AL should be 5");
    assert!(!zf_set(regs.rflags), "ZF should be clear");
}

#[test]
fn test_aam_parity_flag_even() {
    // AL = 13, result AL = 3 (even parity)
    let code = [0xd4, 0x0a, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 0x0D;
    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x03, "AL should be 3");
    assert!(pf_set(regs.rflags), "PF should be set for even parity");
}

#[test]
fn test_aam_parity_flag_odd() {
    // AL = 11, result AL = 1 (odd parity)
    let code = [0xd4, 0x0a, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 0x0B;
    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x01, "AL should be 1");
    assert!(!pf_set(regs.rflags), "PF should be clear for odd parity");
}

// ============================================================================
// AAM - Edge Cases
// ============================================================================

#[test]
fn test_aam_preserves_high_rax() {
    // Verify that bits above AX are preserved
    let code = [0xd4, 0x0a, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 0x1234_5678_DEAD_BE19;
    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax >> 16,
        0x1234_5678_DEAD,
        "High bits of RAX should be preserved"
    );
    assert_eq!(regs.rax & 0xFF, 0x05, "AL should be 5");
    assert_eq!((regs.rax >> 8) & 0xFF, 0x02, "AH should be 2");
}

#[test]
fn test_aam_all_single_digits() {
    // Test all single digit BCD values (0-9)
    for val in 0..=9 {
        let code = [0xd4, 0x0a, 0xf4];
        let mut regs = Registers::default();
        regs.rax = val;
        let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        assert_eq!(
            regs.rax & 0xFF,
            val,
            "AL should remain {} for single digit",
            val
        );
        assert_eq!(
            (regs.rax >> 8) & 0xFF,
            0x00,
            "AH should be 0 for single digit"
        );
    }
}

#[test]
fn test_aam_multiples_of_10() {
    // Test multiples of 10: should result in AL = 0
    for mult in 1..=25 {
        let val = mult * 10;
        let code = [0xd4, 0x0a, 0xf4];
        let mut regs = Registers::default();
        regs.rax = val;
        let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        assert_eq!(regs.rax & 0xFF, 0x00, "AL should be 0 for {}", val);
        assert_eq!(
            (regs.rax >> 8) & 0xFF,
            mult as u64,
            "AH should be {} for {}",
            mult,
            val
        );
    }
}

#[test]
fn test_aam_sequential_values() {
    // Test a range of sequential values
    for val in 0..=99 {
        let code = [0xd4, 0x0a, 0xf4];
        let mut regs = Registers::default();
        regs.rax = val;
        let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        let expected_ah = val / 10;
        let expected_al = val % 10;
        assert_eq!(
            regs.rax & 0xFF,
            expected_al,
            "AL should be {} for input {}",
            expected_al,
            val
        );
        assert_eq!(
            (regs.rax >> 8) & 0xFF,
            expected_ah,
            "AH should be {} for input {}",
            expected_ah,
            val
        );
    }
}

#[test]
fn test_aam_ignores_initial_ah() {
    // AAM should only use AL, not AH
    let code = [0xd4, 0x0a, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 0xFF19; // AH = 0xFF, AL = 25
    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x05, "AL should be 5 (25 MOD 10)");
    assert_eq!((regs.rax >> 8) & 0xFF, 0x02, "AH should be 2 (25 / 10)");
}

#[test]
fn test_aam_with_different_bases_comprehensive() {
    // Test various bases with value 100
    let test_cases = [
        (2, 0, 100 / 2),   // base 2
        (3, 1, 100 / 3),   // base 3
        (5, 0, 100 / 5),   // base 5
        (10, 0, 10),       // base 10
        (11, 1, 100 / 11), // base 11
        (20, 0, 5),        // base 20
    ];

    for (base, expected_al, expected_ah) in test_cases.iter() {
        let code = [0xd4, *base, 0xf4];
        let mut regs = Registers::default();
        regs.rax = 100;
        let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        assert_eq!(
            regs.rax & 0xFF,
            *expected_al as u64,
            "AL incorrect for base {}",
            base
        );
        assert_eq!(
            (regs.rax >> 8) & 0xFF,
            *expected_ah as u64,
            "AH incorrect for base {}",
            base
        );
    }
}
