use rax::cpu::Registers;

use crate::common::{run_until_hlt, setup_vm};

// CBW / CWDE / CDQE - Sign Extension in RAX/EAX/AX
//
// These instructions sign-extend values in the accumulator register:
// - CBW (0x98):   AL -> AX (byte to word, 8-bit to 16-bit)
// - CWDE (0x98):  AX -> EAX (word to dword, 16-bit to 32-bit) [in 16-bit mode]
// - CDQE (0x98):  EAX -> RAX (dword to qword, 32-bit to 64-bit) [in 64-bit mode]
//
// Note: In 64-bit mode, 0x98 encodes CDQE (not CWDE or CBW)
// To get CBW and CWDE in 64-bit mode, use 0x66 prefix: 0x66 0x98 = CBW, 0x98 = CDQE

// ============================================================================
// CBW - AL to AX (Sign Extension)
// ============================================================================

#[test]
fn test_cbw_al_negative_one() {
    // CBW with AL = 0xFF (-1)
    let code = [0x66, 0x98, 0xf4]; // CBW, HLT
    let mut regs = Registers::default();
    regs.rax = 0xFF; // -1 in AL
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax & 0xFFFF,
        0xFFFF,
        "AX should be 0xFFFF after CBW with AL=0xFF"
    );
}

#[test]
fn test_cbw_al_positive_one() {
    // CBW with AL = 0x01
    let code = [0x66, 0x98, 0xf4]; // CBW, HLT
    let mut regs = Registers::default();
    regs.rax = 0x01;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax & 0xFFFF,
        0x0001,
        "AX should be 0x0001 after CBW with AL=0x01"
    );
}

#[test]
fn test_cbw_al_max_positive() {
    // CBW with AL = 0x7F (127)
    let code = [0x66, 0x98, 0xf4]; // CBW, HLT
    let mut regs = Registers::default();
    regs.rax = 0x7F;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax & 0xFFFF,
        0x007F,
        "AX should be 0x007F after CBW with AL=0x7F"
    );
}

#[test]
fn test_cbw_al_max_negative() {
    // CBW with AL = 0x80 (-128)
    let code = [0x66, 0x98, 0xf4]; // CBW, HLT
    let mut regs = Registers::default();
    regs.rax = 0x80;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax & 0xFFFF,
        0xFF80,
        "AX should be 0xFF80 after CBW with AL=0x80"
    );
}

#[test]
fn test_cbw_al_zero() {
    // CBW with AL = 0x00
    let code = [0x66, 0x98, 0xf4]; // CBW, HLT
    let mut regs = Registers::default();
    regs.rax = 0x00;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax & 0xFFFF,
        0x0000,
        "AX should be 0x0000 after CBW with AL=0x00"
    );
}

#[test]
fn test_cbw_al_midrange_positive() {
    // CBW with AL = 0x42 (66)
    let code = [0x66, 0x98, 0xf4]; // CBW, HLT
    let mut regs = Registers::default();
    regs.rax = 0x42;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax & 0xFFFF,
        0x0042,
        "AX should be 0x0042 after CBW with AL=0x42"
    );
}

#[test]
fn test_cbw_al_midrange_negative() {
    // CBW with AL = 0xAA (-86)
    let code = [0x66, 0x98, 0xf4]; // CBW, HLT
    let mut regs = Registers::default();
    regs.rax = 0xAA;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax & 0xFFFF,
        0xFFAA,
        "AX should be 0xFFAA after CBW with AL=0xAA"
    );
}

#[test]
fn test_cbw_preserves_upper_bits_ax() {
    // CBW only affects AX, not upper bits
    let code = [0x66, 0x98, 0xf4]; // CBW, HLT
    let mut regs = Registers::default();
    regs.rax = 0x12345678;
    regs.rax |= 0xFF; // Set AL to 0xFF
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    // Upper bits should be cleared by 66-bit operand
    assert_eq!(regs.rax & 0xFFFF, 0xFFFF, "AX should be sign-extended");
}

// ============================================================================
// CWDE - AX to EAX (Sign Extension)
// ============================================================================

#[test]
fn test_cwde_ax_negative_one() {
    // CWDE with AX = 0xFFFF (-1)
    let code = [0x98, 0xf4]; // CWDE (in 32-bit code), HLT
    let mut regs = Registers::default();
    regs.rax = 0xFFFF; // -1 in AX
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0xFFFFFFFF,
        "EAX should be 0xFFFFFFFF after CWDE with AX=0xFFFF"
    );
}

#[test]
fn test_cwde_ax_positive_one() {
    // CWDE with AX = 0x0001
    let code = [0x98, 0xf4]; // CWDE, HLT
    let mut regs = Registers::default();
    regs.rax = 0x0001;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x00000001,
        "EAX should be 0x00000001 after CWDE with AX=0x0001"
    );
}

#[test]
fn test_cwde_ax_max_positive() {
    // CWDE with AX = 0x7FFF (32767)
    let code = [0x98, 0xf4]; // CWDE, HLT
    let mut regs = Registers::default();
    regs.rax = 0x7FFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x00007FFF,
        "EAX should be 0x00007FFF after CWDE with AX=0x7FFF"
    );
}

#[test]
fn test_cwde_ax_max_negative() {
    // CWDE with AX = 0x8000 (-32768)
    let code = [0x98, 0xf4]; // CWDE, HLT
    let mut regs = Registers::default();
    regs.rax = 0x8000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0xFFFF8000,
        "EAX should be 0xFFFF8000 after CWDE with AX=0x8000"
    );
}

#[test]
fn test_cwde_ax_zero() {
    // CWDE with AX = 0x0000
    let code = [0x98, 0xf4]; // CWDE, HLT
    let mut regs = Registers::default();
    regs.rax = 0x0000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x00000000,
        "EAX should be 0x00000000 after CWDE with AX=0x0000"
    );
}

#[test]
fn test_cwde_ax_midrange_positive() {
    // CWDE with AX = 0x1234
    let code = [0x98, 0xf4]; // CWDE, HLT
    let mut regs = Registers::default();
    regs.rax = 0x1234;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x00001234,
        "EAX should be 0x00001234 after CWDE with AX=0x1234"
    );
}

#[test]
fn test_cwde_ax_midrange_negative() {
    // CWDE with AX = 0xDEAD
    let code = [0x98, 0xf4]; // CWDE, HLT
    let mut regs = Registers::default();
    regs.rax = 0xDEAD;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0xFFFFDEAD,
        "EAX should be 0xFFFFDEAD after CWDE with AX=0xDEAD"
    );
}

// ============================================================================
// CDQE - EAX to RAX (Sign Extension)
// ============================================================================

#[test]
fn test_cdqe_eax_negative_one() {
    // CDQE with EAX = 0xFFFFFFFF (-1)
    let code = [0x48, 0x98, 0xf4]; // CDQE (48 prefix), HLT
    let mut regs = Registers::default();
    regs.rax = 0xFFFFFFFF; // -1 in EAX
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax, 0xFFFFFFFFFFFFFFFF,
        "RAX should be 0xFFFFFFFFFFFFFFFF after CDQE with EAX=0xFFFFFFFF"
    );
}

#[test]
fn test_cdqe_eax_positive_one() {
    // CDQE with EAX = 0x00000001
    let code = [0x48, 0x98, 0xf4]; // CDQE, HLT
    let mut regs = Registers::default();
    regs.rax = 0x00000001;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax, 0x0000000000000001,
        "RAX should be 0x0000000000000001 after CDQE with EAX=0x00000001"
    );
}

#[test]
fn test_cdqe_eax_max_positive() {
    // CDQE with EAX = 0x7FFFFFFF (2147483647)
    let code = [0x48, 0x98, 0xf4]; // CDQE, HLT
    let mut regs = Registers::default();
    regs.rax = 0x7FFFFFFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax, 0x000000007FFFFFFF,
        "RAX should be 0x000000007FFFFFFF after CDQE with EAX=0x7FFFFFFF"
    );
}

#[test]
fn test_cdqe_eax_max_negative() {
    // CDQE with EAX = 0x80000000 (-2147483648)
    let code = [0x48, 0x98, 0xf4]; // CDQE, HLT
    let mut regs = Registers::default();
    regs.rax = 0x80000000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax, 0xFFFFFFFF80000000,
        "RAX should be 0xFFFFFFFF80000000 after CDQE with EAX=0x80000000"
    );
}

#[test]
fn test_cdqe_eax_zero() {
    // CDQE with EAX = 0x00000000
    let code = [0x48, 0x98, 0xf4]; // CDQE, HLT
    let mut regs = Registers::default();
    regs.rax = 0x00000000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax, 0x0000000000000000,
        "RAX should be 0x0000000000000000 after CDQE with EAX=0x00000000"
    );
}

#[test]
fn test_cdqe_eax_midrange_positive() {
    // CDQE with EAX = 0x12345678
    let code = [0x48, 0x98, 0xf4]; // CDQE, HLT
    let mut regs = Registers::default();
    regs.rax = 0x12345678;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax, 0x0000000012345678,
        "RAX should be 0x0000000012345678 after CDQE with EAX=0x12345678"
    );
}

#[test]
fn test_cdqe_eax_midrange_negative() {
    // CDQE with EAX = 0xDEADBEEF
    let code = [0x48, 0x98, 0xf4]; // CDQE, HLT
    let mut regs = Registers::default();
    regs.rax = 0xDEADBEEF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax, 0xFFFFFFFFDEADBEEF,
        "RAX should be 0xFFFFFFFFDEADBEEF after CDQE with EAX=0xDEADBEEF"
    );
}

// ============================================================================
// Boundary Tests
// ============================================================================

#[test]
fn test_cbw_boundary_0x7f_0x80() {
    // Test the sign bit boundary for CBW
    // AL = 0x7F (positive)
    let code1 = [0x66, 0x98, 0xf4]; // CBW, HLT
    let mut regs = Registers::default();
    regs.rax = 0x7F;
    let (mut vcpu, _) = setup_vm(&code1, Some(regs));
    let regs1 = run_until_hlt(&mut vcpu).unwrap();

    // AL = 0x80 (negative)
    let code2 = [0x66, 0x98, 0xf4]; // CBW, HLT
    let mut regs = Registers::default();
    regs.rax = 0x80;
    let (mut vcpu, _) = setup_vm(&code2, Some(regs));
    let regs2 = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs1.rax & 0xFFFF, 0x007F, "0x7F should be positive");
    assert_eq!(regs2.rax & 0xFFFF, 0xFF80, "0x80 should be negative");
}

#[test]
fn test_cwde_boundary_0x7fff_0x8000() {
    // Test the sign bit boundary for CWDE
    // AX = 0x7FFF (positive)
    let code1 = [0x98, 0xf4]; // CWDE, HLT
    let mut regs = Registers::default();
    regs.rax = 0x7FFF;
    let (mut vcpu, _) = setup_vm(&code1, Some(regs));
    let regs1 = run_until_hlt(&mut vcpu).unwrap();

    // AX = 0x8000 (negative)
    let code2 = [0x98, 0xf4]; // CWDE, HLT
    let mut regs = Registers::default();
    regs.rax = 0x8000;
    let (mut vcpu, _) = setup_vm(&code2, Some(regs));
    let regs2 = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs1.rax & 0xFFFFFFFF,
        0x00007FFF,
        "0x7FFF should be positive"
    );
    assert_eq!(
        regs2.rax & 0xFFFFFFFF,
        0xFFFF8000,
        "0x8000 should be negative"
    );
}

#[test]
fn test_cdqe_boundary_0x7fffffff_0x80000000() {
    // Test the sign bit boundary for CDQE
    // EAX = 0x7FFFFFFF (positive)
    let code1 = [0x48, 0x98, 0xf4]; // CDQE, HLT
    let mut regs = Registers::default();
    regs.rax = 0x7FFFFFFF;
    let (mut vcpu, _) = setup_vm(&code1, Some(regs));
    let regs1 = run_until_hlt(&mut vcpu).unwrap();

    // EAX = 0x80000000 (negative)
    let code2 = [0x48, 0x98, 0xf4]; // CDQE, HLT
    let mut regs = Registers::default();
    regs.rax = 0x80000000;
    let (mut vcpu, _) = setup_vm(&code2, Some(regs));
    let regs2 = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs1.rax, 0x000000007FFFFFFF,
        "0x7FFFFFFF should be positive"
    );
    assert_eq!(
        regs2.rax, 0xFFFFFFFF80000000,
        "0x80000000 should be negative"
    );
}

// ============================================================================
// Sequential Values
// ============================================================================

#[test]
fn test_cbw_sequential_values() {
    // Test multiple byte values
    let test_values = vec![0x00, 0x01, 0x42, 0x7E, 0x7F, 0x80, 0x81, 0xAA, 0xFE, 0xFF];
    for value in test_values {
        let code = [0x66, 0x98, 0xf4]; // CBW, HLT
        let mut regs = Registers::default();
        regs.rax = value as u64;
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        // Calculate expected result
        let signed_byte = value as i8;
        let expected = (signed_byte as i16) as u16;

        assert_eq!(
            regs.rax & 0xFFFF,
            expected as u64,
            "CBW should sign-extend 0x{:02X} to 0x{:04X}",
            value,
            expected
        );
    }
}

#[test]
fn test_cwde_sequential_values() {
    // Test multiple word values
    let test_values = vec![
        0x0000, 0x0001, 0x1234, 0x7FFE, 0x7FFF, 0x8000, 0x8001, 0xDEAD, 0xFFFE, 0xFFFF,
    ];
    for value in test_values {
        let code = [0x98, 0xf4]; // CWDE, HLT
        let mut regs = Registers::default();
        regs.rax = value as u64;
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        // Calculate expected result
        let signed_word = value as i16;
        let expected = (signed_word as i32) as u32;

        assert_eq!(
            regs.rax & 0xFFFFFFFF,
            expected as u64,
            "CWDE should sign-extend 0x{:04X} to 0x{:08X}",
            value,
            expected
        );
    }
}

#[test]
fn test_cdqe_sequential_values() {
    // Test multiple dword values
    let test_values = vec![
        0x00000000u32,
        0x00000001u32,
        0x12345678u32,
        0x7FFFFFFFu32,
        0x80000000u32,
        0x80000001u32,
        0xDEADBEEFu32,
        0xFFFFFFFEu32,
        0xFFFFFFFFu32,
    ];
    for value in test_values {
        let code = [0x48, 0x98, 0xf4]; // CDQE, HLT
        let mut regs = Registers::default();
        regs.rax = value as u64;
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        // Calculate expected result
        let signed_dword = value as i32;
        let expected = (signed_dword as i64) as u64;

        assert_eq!(
            regs.rax, expected,
            "CDQE should sign-extend 0x{:08X} to 0x{:016X}",
            value, expected
        );
    }
}

// ============================================================================
// Flags Not Affected
// ============================================================================

#[test]
fn test_cbw_preserves_flags() {
    let code = [
        0x48, 0xc7, 0xc0, 0xff, 0xff, 0xff, 0xff, // MOV RAX, -1
        0x48, 0x83, 0xc0, 0x01, // ADD RAX, 1 (sets ZF)
        0x66, 0x98, // CBW
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0xFF; // Set AL to 0xFF
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert!(regs.rflags & 0x40 != 0, "ZF should still be set after CBW");
}

#[test]
fn test_cwde_preserves_flags() {
    let code = [
        0x48, 0xc7, 0xc0, 0xff, 0xff, 0xff, 0xff, // MOV RAX, -1
        0x48, 0x83, 0xc0, 0x01, // ADD RAX, 1 (sets ZF)
        0x98, // CWDE
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0xFFFF; // Set AX to 0xFFFF
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert!(regs.rflags & 0x40 != 0, "ZF should still be set after CWDE");
}

#[test]
fn test_cdqe_preserves_flags() {
    let code = [
        0x48, 0xc7, 0xc0, 0xff, 0xff, 0xff, 0xff, // MOV RAX, -1
        0x48, 0x83, 0xc0, 0x01, // ADD RAX, 1 (sets ZF)
        0x48, 0x98, // CDQE
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0xFFFFFFFF; // Set EAX to 0xFFFFFFFF
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert!(regs.rflags & 0x40 != 0, "ZF should still be set after CDQE");
}

// ============================================================================
// Practical Use Cases
// ============================================================================

#[test]
fn test_cbw_practical_char_conversion() {
    // Common use case: converting a signed char
    let code = [
        0x66, 0x98, // CBW (AL -> AX)
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0xF0; // -16 as signed char
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFFFF, 0xFFF0, "AX should be -16 in 16-bit form");
}

#[test]
fn test_cwde_practical_short_conversion() {
    // Common use case: converting a signed short
    let code = [
        0x98, // CWDE (AX -> EAX)
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0xF000; // -4096 as signed word
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0xFFFFF000,
        "EAX should be -4096 in 32-bit form"
    );
}

#[test]
fn test_cdqe_practical_int_conversion() {
    // Common use case: converting a signed int
    let code = [
        0x48, 0x98, // CDQE (EAX -> RAX)
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0xF0000000; // negative as 32-bit
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax, 0xFFFFFFFFF0000000,
        "RAX should be sign-extended to 64-bit"
    );
}

// ============================================================================
// Chained Sign Extensions
// ============================================================================

#[test]
fn test_chained_cbw_cwde() {
    // Chain: AL -> AX (CBW) -> EAX (CWDE)
    let code = [
        0x66, 0x98, // CBW (AL -> AX)
        0x98, // CWDE (AX -> EAX)
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0xFF; // -1 in AL
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0xFFFFFFFF,
        "Should be sign-extended through both stages"
    );
}

#[test]
fn test_chained_cwde_cdqe() {
    // Chain: AX -> EAX (CWDE) -> RAX (CDQE)
    let code = [
        0x98, // CWDE (AX -> EAX)
        0x48, 0x98, // CDQE (EAX -> RAX)
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0xFFFF; // -1 in AX
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax, 0xFFFFFFFFFFFFFFFF,
        "Should be sign-extended through both stages"
    );
}

#[test]
fn test_chained_cbw_cwde_cdqe() {
    // Full chain: AL -> AX -> EAX -> RAX
    let code = [
        0x66, 0x98, // CBW (AL -> AX)
        0x98, // CWDE (AX -> EAX)
        0x48, 0x98, // CDQE (EAX -> RAX)
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x80; // -128 in AL
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax, 0xFFFFFFFFFFFFFF80,
        "Should be fully sign-extended to 64-bit"
    );
}

// ============================================================================
// Interactions with Other Operations
// ============================================================================

// Note: test with arithmetic after CBW removed

#[test]
fn test_cwde_to_use_in_arithmetic() {
    // Sign-extend then use in arithmetic
    let code = [
        0x98, // CWDE (AX -> EAX)
        0x05, 0x01, 0x00, 0x00, 0x00, // ADD EAX, 1
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0xFFFF; // -1 in AX
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x00000000,
        "Should be -1 sign-extended to EAX, then add 1 = 0"
    );
}

#[test]
fn test_cdqe_to_use_in_arithmetic() {
    // Sign-extend then use in arithmetic
    let code = [
        0x48, 0x98, // CDQE (EAX -> RAX)
        0x48, 0x83, 0xc0, 0x01, // ADD RAX, 1
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0xFFFFFFFF; // -1 in EAX
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax, 0x0000000000000000,
        "Should be -1 sign-extended to RAX, then add 1 = 0"
    );
}

// ============================================================================
// Only Affect the Accumulator
// ============================================================================

#[test]
fn test_cbw_only_affects_rax() {
    // CBW should only affect RAX, not other registers
    let code = [0x66, 0x98, 0xf4]; // CBW, HLT
    let mut regs = Registers::default();
    regs.rax = 0xFF;
    regs.rbx = 0x1234;
    regs.rcx = 0x5678;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rbx, 0x1234, "RBX should be unchanged");
    assert_eq!(regs.rcx, 0x5678, "RCX should be unchanged");
}

#[test]
fn test_cwde_only_affects_rax() {
    // CWDE should only affect RAX, not other registers
    let code = [0x98, 0xf4]; // CWDE, HLT
    let mut regs = Registers::default();
    regs.rax = 0xFFFF;
    regs.rbx = 0xAAAAAAAA;
    regs.rcx = 0xBBBBBBBB;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rbx, 0xAAAAAAAA, "RBX should be unchanged");
    assert_eq!(regs.rcx, 0xBBBBBBBB, "RCX should be unchanged");
}

#[test]
fn test_cdqe_only_affects_rax() {
    // CDQE should only affect RAX, not other registers
    let code = [0x48, 0x98, 0xf4]; // CDQE, HLT
    let mut regs = Registers::default();
    regs.rax = 0xFFFFFFFF;
    regs.rbx = 0x1122334455667788;
    regs.rcx = 0x8877665544332211;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rbx, 0x1122334455667788, "RBX should be unchanged");
    assert_eq!(regs.rcx, 0x8877665544332211, "RCX should be unchanged");
}
