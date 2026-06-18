// Module path for tests run via x86_64.rs
use crate::common::*;
use rax::cpu::Registers;

// CDQE/CQO - Sign Extension Instructions (Comprehensive Extended Tests)
// These instructions sign-extend values in RAX/RDX:RAX for division/multiplication
//
// CDQE - Convert Doubleword to Quadword Extended
//        Sign-extends EAX into RAX (32-bit to 64-bit)
//
// CQO  - Convert Quadword to Octaword
//        Sign-extends RAX into RDX:RAX (64-bit to 128-bit)
//
// Related instructions (16/32-bit):
// CBW  - Convert Byte to Word (AL -> AX)
// CWDE - Convert Word to Doubleword Extended (AX -> EAX)
// CWD  - Convert Word to Doubleword (AX -> DX:AX)
// CDQ  - Convert Doubleword to Quadword (EAX -> EDX:EAX)
//
// Opcodes:
// 98           CBW/CWDE/CDQE  - Sign extend AL->AX, AX->EAX, or EAX->RAX
// 99           CWD/CDQ/CQO    - Sign extend AX->DX:AX, EAX->EDX:EAX, or RAX->RDX:RAX

#[test]
fn test_cdqe_positive_value() {
    // CDQE with positive value (sign bit = 0)
    let code = [
        0x48, 0x98, // CDQE
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x000000007FFFFFFF; // Largest positive 32-bit value
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x000000007FFFFFFF); // Should be zero-extended
}

#[test]
fn test_cdqe_negative_value() {
    // CDQE with negative value (sign bit = 1)
    let code = [
        0x48, 0x98, // CDQE
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x0000000080000000; // Smallest negative 32-bit value
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0xFFFFFFFF80000000); // Sign-extended to 64-bit
}

#[test]
fn test_cdqe_zero() {
    // CDQE with zero
    let code = [
        0x48, 0x98, // CDQE
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x0000000000000000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x0000000000000000);
}

#[test]
fn test_cdqe_minus_one() {
    // CDQE with -1
    let code = [
        0x48, 0x98, // CDQE
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x00000000FFFFFFFF; // -1 in 32-bit
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0xFFFFFFFFFFFFFFFF); // -1 in 64-bit
}

#[test]
fn test_cdqe_one() {
    // CDQE with 1
    let code = [
        0x48, 0x98, // CDQE
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x0000000000000001;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x0000000000000001);
}

#[test]
fn test_cqo_positive_value() {
    // CQO with positive value
    let code = [
        0x48, 0x99, // CQO
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x7FFFFFFFFFFFFFFF; // Largest positive 64-bit value
    regs.rdx = 0xFFFFFFFFFFFFFFFF; // Will be cleared
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rdx, 0x0000000000000000); // Sign extend with zeros
    assert_eq!(regs.rax, 0x7FFFFFFFFFFFFFFF); // RAX unchanged
}

#[test]
fn test_cqo_negative_value() {
    // CQO with negative value
    let code = [
        0x48, 0x99, // CQO
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x8000000000000000; // Smallest negative 64-bit value
    regs.rdx = 0x0000000000000000; // Will be set
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rdx, 0xFFFFFFFFFFFFFFFF); // Sign extend with ones
    assert_eq!(regs.rax, 0x8000000000000000); // RAX unchanged
}

#[test]
fn test_cqo_zero() {
    // CQO with zero
    let code = [
        0x48, 0x99, // CQO
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x0000000000000000;
    regs.rdx = 0xFFFFFFFFFFFFFFFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rdx, 0x0000000000000000);
    assert_eq!(regs.rax, 0x0000000000000000);
}

#[test]
fn test_cqo_minus_one() {
    // CQO with -1
    let code = [
        0x48, 0x99, // CQO
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0xFFFFFFFFFFFFFFFF; // -1
    regs.rdx = 0x0000000000000000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rdx, 0xFFFFFFFFFFFFFFFF);
    assert_eq!(regs.rax, 0xFFFFFFFFFFFFFFFF);
}

#[test]
fn test_cqo_one() {
    // CQO with 1
    let code = [
        0x48, 0x99, // CQO
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x0000000000000001;
    regs.rdx = 0xFFFFFFFFFFFFFFFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rdx, 0x0000000000000000);
    assert_eq!(regs.rax, 0x0000000000000001);
}

#[test]
fn test_cbw_positive() {
    // CBW - Convert Byte to Word (AL -> AX)
    let code = [
        0x66, 0x98, // CBW
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x000000000000007F; // AL = 127 (max positive byte)
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFF, 0x007F); // AX = 0x007F
}

#[test]
fn test_cbw_negative() {
    // CBW with negative value
    let code = [
        0x66, 0x98, // CBW
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x0000000000000080; // AL = -128 (min negative byte)
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFF, 0xFF80); // AX = 0xFF80 (sign-extended)
}

#[test]
fn test_cwde_positive() {
    // CWDE - Convert Word to Doubleword Extended (AX -> EAX)
    let code = [
        0x98, // CWDE
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x0000000000007FFF; // AX = 32767 (max positive word)
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x0000000000007FFF); // EAX zero-extended to RAX
}

#[test]
fn test_cwde_negative() {
    // CWDE with negative value
    let code = [
        0x98, // CWDE
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x0000000000008000; // AX = -32768 (min negative word)
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x00000000FFFF8000); // EAX sign-extended, then zero-extended to RAX
}

#[test]
fn test_cwd_positive() {
    // CWD - Convert Word to Doubleword (AX -> DX:AX)
    let code = [
        0x66, 0x99, // CWD
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x0000000000007FFF; // AX = 32767
    regs.rdx = 0xFFFFFFFFFFFFFFFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rdx & 0xFFFF, 0x0000); // DX = 0
}

#[test]
fn test_cwd_negative() {
    // CWD with negative value
    let code = [
        0x66, 0x99, // CWD
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x0000000000008000; // AX = -32768
    regs.rdx = 0x0000000000000000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rdx & 0xFFFF, 0xFFFF); // DX = 0xFFFF
}

#[test]
fn test_cdq_positive() {
    // CDQ - Convert Doubleword to Quadword (EAX -> EDX:EAX)
    let code = [
        0x99, // CDQ
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x000000007FFFFFFF; // EAX = 2147483647
    regs.rdx = 0xFFFFFFFFFFFFFFFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rdx, 0x0000000000000000); // EDX = 0 (zero-extended to RDX)
}

#[test]
fn test_cdq_negative() {
    // CDQ with negative value
    let code = [
        0x99, // CDQ
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x0000000080000000; // EAX = -2147483648
    regs.rdx = 0x0000000000000000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rdx, 0x00000000FFFFFFFF); // EDX = 0xFFFFFFFF (zero-extended to RDX)
}

#[test]
fn test_cdqe_various_values() {
    // Test CDQE with various 32-bit values
    let test_cases = vec![
        (0x00000000u32, 0x0000000000000000u64),
        (0x00000001u32, 0x0000000000000001u64),
        (0x7FFFFFFFu32, 0x000000007FFFFFFFu64),
        (0x80000000u32, 0xFFFFFFFF80000000u64),
        (0xFFFFFFFFu32, 0xFFFFFFFFFFFFFFFFu64),
        (0x12345678u32, 0x0000000012345678u64),
        (0xDEADBEEFu32, 0xFFFFFFFFDEADBEEFu64),
    ];

    for (input, expected) in test_cases {
        let code = [
            0x48, 0x98, // CDQE
            0xf4,
        ];
        let mut regs = Registers::default();
        regs.rax = input as u64;
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        assert_eq!(
            regs.rax, expected,
            "CDQE({:#x}) should be {:#x}",
            input, expected
        );
    }
}

#[test]
fn test_cqo_various_values() {
    // Test CQO with various 64-bit values
    let test_cases = vec![
        (0x0000000000000000u64, 0x0000000000000000u64),
        (0x0000000000000001u64, 0x0000000000000000u64),
        (0x7FFFFFFFFFFFFFFFu64, 0x0000000000000000u64),
        (0x8000000000000000u64, 0xFFFFFFFFFFFFFFFFu64),
        (0xFFFFFFFFFFFFFFFFu64, 0xFFFFFFFFFFFFFFFFu64),
        (0x123456789ABCDEFu64, 0x0000000000000000u64),
        (0xFEDCBA9876543210u64, 0xFFFFFFFFFFFFFFFFu64),
    ];

    for (input, expected_rdx) in test_cases {
        let code = [
            0x48, 0x99, // CQO
            0xf4,
        ];
        let mut regs = Registers::default();
        regs.rax = input;
        regs.rdx = 0;
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        assert_eq!(
            regs.rdx, expected_rdx,
            "CQO({:#x}) RDX should be {:#x}",
            input, expected_rdx
        );
        assert_eq!(regs.rax, input, "CQO should not modify RAX");
    }
}

#[test]
fn test_cdqe_before_division() {
    // Common pattern: CDQE before signed division
    let code = [
        0x48, 0x98, // CDQE (sign-extend EAX to RAX)
        0x48, 0x99, // CQO (sign-extend RAX to RDX:RAX)
        // Would normally follow with IDIV here
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x00000000FFFFFF9C; // -100 in 32-bit
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0xFFFFFFFFFFFFFF9C); // Sign-extended to 64-bit
    assert_eq!(regs.rdx, 0xFFFFFFFFFFFFFFFF); // Sign-extended to RDX
}

#[test]
fn test_cdqe_doesnt_affect_flags() {
    // CDQE should not modify flags
    let code = [
        0x48, 0x98, // CDQE
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x0000000080000000;
    regs.rflags = 0x246; // Some flags set
    let initial_flags = regs.rflags;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rflags, initial_flags);
}

#[test]
fn test_cqo_doesnt_affect_flags() {
    // CQO should not modify flags
    let code = [
        0x48, 0x99, // CQO
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x8000000000000000;
    regs.rflags = 0x246; // Some flags set
    let initial_flags = regs.rflags;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rflags, initial_flags);
}

#[test]
fn test_cdqe_preserves_other_registers() {
    // CDQE should only affect RAX
    let code = [
        0x48, 0x98, // CDQE
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x0000000080000000;
    regs.rbx = 0x1111111111111111;
    regs.rcx = 0x2222222222222222;
    regs.rdx = 0x3333333333333333;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx, 0x1111111111111111);
    assert_eq!(regs.rcx, 0x2222222222222222);
    assert_eq!(regs.rdx, 0x3333333333333333);
}

#[test]
fn test_cqo_preserves_other_registers() {
    // CQO should only affect RDX (and leave RAX unchanged)
    let code = [
        0x48, 0x99, // CQO
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x8000000000000000;
    regs.rbx = 0x1111111111111111;
    regs.rcx = 0x2222222222222222;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx, 0x1111111111111111);
    assert_eq!(regs.rcx, 0x2222222222222222);
}

#[test]
fn test_cdqe_cqo_sequence() {
    // CDQE followed by CQO
    let code = [
        0x48, 0x98, // CDQE
        0x48, 0x99, // CQO
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x00000000DEADBEEF; // Negative in 32-bit
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0xFFFFFFFFDEADBEEF); // Sign-extended by CDQE
    assert_eq!(regs.rdx, 0xFFFFFFFFFFFFFFFF); // Sign-extended by CQO
}

#[test]
fn test_cbw_cwde_cdqe_chain() {
    // Full chain: byte -> word -> dword -> qword
    let code = [
        0x66, 0x98, // CBW (AL -> AX)
        0x98, // CWDE (AX -> EAX)
        0x48, 0x98, // CDQE (EAX -> RAX)
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x0000000000000080; // AL = -128
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0xFFFFFFFFFFFFFF80); // Fully sign-extended to 64-bit
}

#[test]
fn test_cqo_with_max_positive() {
    // CQO with maximum positive value
    let code = [
        0x48, 0x99, // CQO
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x7FFFFFFFFFFFFFFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rdx, 0x0000000000000000);
}

#[test]
fn test_cqo_with_min_negative() {
    // CQO with minimum negative value
    let code = [
        0x48, 0x99, // CQO
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x8000000000000000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rdx, 0xFFFFFFFFFFFFFFFF);
}
