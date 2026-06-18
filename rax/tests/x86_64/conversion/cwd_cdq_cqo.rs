// Module path for tests run via x86_64.rs
use crate::common::*;
use rax::cpu::Registers;

// CWD/CDQ/CQO - Convert Word to Doubleword/Convert Doubleword to Quadword
// Doubles the size of the operand in register AX, EAX, or RAX by means of sign extension
// and stores the result in registers DX:AX, EDX:EAX, or RDX:RAX, respectively.
//
// CWD (99):           DX:AX := sign-extend of AX
// CDQ (99):           EDX:EAX := sign-extend of EAX
// CQO (REX.W + 99):   RDX:RAX:= sign-extend of RAX
//
// The instruction copies the sign bit of the source register into every bit position
// in the destination register.

// ============================================================================
// CWD Tests - Convert Word to Doubleword (AX -> DX:AX)
// ============================================================================

#[test]
fn test_cwd_positive_value() {
    // CWD with positive value in AX
    // AX = 0x1234, sign bit = 0, so DX should be 0x0000
    let code = [
        0x66, 0x99, // CWD (operand size prefix for 16-bit)
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x1234; // Positive value (bit 15 = 0)
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFF, 0x1234, "AX should remain 0x1234");
    assert_eq!(
        regs.rdx & 0xFFFF,
        0x0000,
        "DX should be 0x0000 (sign-extended from positive)"
    );
}

#[test]
fn test_cwd_negative_value() {
    // CWD with negative value in AX
    // AX = 0x8000, sign bit = 1, so DX should be 0xFFFF
    let code = [
        0x66, 0x99, // CWD
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x8000; // Negative value (bit 15 = 1)
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFF, 0x8000, "AX should remain 0x8000");
    assert_eq!(
        regs.rdx & 0xFFFF,
        0xFFFF,
        "DX should be 0xFFFF (sign-extended from negative)"
    );
}

#[test]
fn test_cwd_zero() {
    // CWD with zero in AX
    let code = [
        0x66, 0x99, // CWD
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x0000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFF, 0x0000, "AX should be 0");
    assert_eq!(
        regs.rdx & 0xFFFF,
        0x0000,
        "DX should be 0 (sign-extended from 0)"
    );
}

#[test]
fn test_cwd_max_positive() {
    // CWD with maximum positive 16-bit value
    // 0x7FFF = 32767, sign bit = 0
    let code = [
        0x66, 0x99, // CWD
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x7FFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFF, 0x7FFF, "AX should be 0x7FFF");
    assert_eq!(regs.rdx & 0xFFFF, 0x0000, "DX should be 0x0000");
}

#[test]
fn test_cwd_min_negative() {
    // CWD with minimum negative 16-bit value
    // 0x8000 = -32768, sign bit = 1
    let code = [
        0x66, 0x99, // CWD
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x8000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFF, 0x8000, "AX should be 0x8000");
    assert_eq!(regs.rdx & 0xFFFF, 0xFFFF, "DX should be 0xFFFF");
}

#[test]
fn test_cwd_one() {
    // CWD with 1 in AX
    let code = [
        0x66, 0x99, // CWD
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x0001;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFF, 0x0001, "AX should be 1");
    assert_eq!(regs.rdx & 0xFFFF, 0x0000, "DX should be 0");
}

#[test]
fn test_cwd_minus_one() {
    // CWD with -1 (0xFFFF) in AX
    let code = [
        0x66, 0x99, // CWD
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0xFFFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFF, 0xFFFF, "AX should be 0xFFFF");
    assert_eq!(regs.rdx & 0xFFFF, 0xFFFF, "DX should be 0xFFFF");
}

#[test]
fn test_cwd_sign_boundary() {
    // CWD with value just below sign bit
    // 0x7FFE, sign bit = 0
    let code = [
        0x66, 0x99, // CWD
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x7FFE;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFF, 0x7FFE, "AX should be 0x7FFE");
    assert_eq!(regs.rdx & 0xFFFF, 0x0000, "DX should be 0");
}

#[test]
fn test_cwd_sign_boundary_negative() {
    // CWD with value just above sign bit
    // 0x8001, sign bit = 1
    let code = [
        0x66, 0x99, // CWD
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x8001;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFF, 0x8001, "AX should be 0x8001");
    assert_eq!(regs.rdx & 0xFFFF, 0xFFFF, "DX should be 0xFFFF");
}

#[test]
fn test_cwd_preserves_upper_bits_rax() {
    // CWD should not affect upper bits of RAX beyond AX
    let code = [
        0x66, 0x99, // CWD
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x12345678_9ABC_1234; // AX portion = 0x1234
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // AX should stay 0x1234, upper bits should be preserved
    assert_eq!(regs.rax & 0xFFFF, 0x1234, "AX unchanged");
    assert_eq!(
        regs.rax >> 16,
        0x12345678_9ABC,
        "Upper bits of RAX preserved"
    );
}

#[test]
fn test_cwd_preserves_upper_bits_rdx() {
    // CWD should only affect DX portion of RDX
    let code = [
        0x66, 0x99, // CWD
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x1234;
    regs.rdx = 0xDEADBEEF_CAFE_0000; // Upper bits set
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rdx & 0xFFFF, 0x0000, "DX should be 0");
    assert_eq!(
        regs.rdx >> 16,
        0xDEADBEEF_CAFE,
        "Upper bits of RDX preserved"
    );
}

#[test]
fn test_cwd_does_not_affect_flags() {
    // CWD should not modify any flags
    let code = [
        0x66, 0x99, // CWD
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x8000;
    regs.rflags = 0x2 | 0x1 | (1 << 6) | (1 << 7); // CF, ZF, SF set
    let initial_flags = regs.rflags;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rflags, initial_flags, "Flags should not be modified");
}

// ============================================================================
// CDQ Tests - Convert Doubleword to Quadword (EAX -> EDX:EAX)
// ============================================================================

#[test]
fn test_cdq_positive_value() {
    // CDQ with positive value in EAX
    // EAX = 0x12345678, sign bit (bit 31) = 0, so EDX should be 0x00000000
    let code = [
        0x99, // CDQ
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x12345678; // Positive value
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x12345678,
        "EAX should remain 0x12345678"
    );
    assert_eq!(
        regs.rdx & 0xFFFFFFFF,
        0x00000000,
        "EDX should be 0x00000000"
    );
}

#[test]
fn test_cdq_negative_value() {
    // CDQ with negative value in EAX
    // EAX = 0x80000000, sign bit = 1, so EDX should be 0xFFFFFFFF
    let code = [
        0x99, // CDQ
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x80000000; // Negative value
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x80000000,
        "EAX should remain 0x80000000"
    );
    assert_eq!(
        regs.rdx & 0xFFFFFFFF,
        0xFFFFFFFF,
        "EDX should be 0xFFFFFFFF"
    );
}

#[test]
fn test_cdq_zero() {
    // CDQ with zero in EAX
    let code = [
        0x99, // CDQ
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x00000000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 0x00000000, "EAX should be 0");
    assert_eq!(regs.rdx & 0xFFFFFFFF, 0x00000000, "EDX should be 0");
}

#[test]
fn test_cdq_max_positive() {
    // CDQ with maximum positive 32-bit value
    // 0x7FFFFFFF, sign bit = 0
    let code = [
        0x99, // CDQ
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x7FFFFFFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x7FFFFFFF,
        "EAX should be 0x7FFFFFFF"
    );
    assert_eq!(regs.rdx & 0xFFFFFFFF, 0x00000000, "EDX should be 0");
}

#[test]
fn test_cdq_min_negative() {
    // CDQ with minimum negative 32-bit value
    // 0x80000000, sign bit = 1
    let code = [
        0x99, // CDQ
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x80000000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x80000000,
        "EAX should be 0x80000000"
    );
    assert_eq!(
        regs.rdx & 0xFFFFFFFF,
        0xFFFFFFFF,
        "EDX should be 0xFFFFFFFF"
    );
}

#[test]
fn test_cdq_one() {
    // CDQ with 1 in EAX
    let code = [
        0x99, // CDQ
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x00000001;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 0x00000001, "EAX should be 1");
    assert_eq!(regs.rdx & 0xFFFFFFFF, 0x00000000, "EDX should be 0");
}

#[test]
fn test_cdq_minus_one() {
    // CDQ with -1 (0xFFFFFFFF) in EAX
    let code = [
        0x99, // CDQ
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0xFFFFFFFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0xFFFFFFFF,
        "EAX should be 0xFFFFFFFF"
    );
    assert_eq!(
        regs.rdx & 0xFFFFFFFF,
        0xFFFFFFFF,
        "EDX should be 0xFFFFFFFF"
    );
}

#[test]
fn test_cdq_sign_boundary_positive() {
    // CDQ with value just below sign bit
    // 0x7FFFFFFE, sign bit = 0
    let code = [
        0x99, // CDQ
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x7FFFFFFE;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x7FFFFFFE,
        "EAX should be 0x7FFFFFFE"
    );
    assert_eq!(regs.rdx & 0xFFFFFFFF, 0x00000000, "EDX should be 0");
}

#[test]
fn test_cdq_sign_boundary_negative() {
    // CDQ with value just above sign bit
    // 0x80000001, sign bit = 1
    let code = [
        0x99, // CDQ
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x80000001;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x80000001,
        "EAX should be 0x80000001"
    );
    assert_eq!(
        regs.rdx & 0xFFFFFFFF,
        0xFFFFFFFF,
        "EDX should be 0xFFFFFFFF"
    );
}

#[test]
fn test_cdq_clears_upper_bits_rax() {
    // CDQ sign-extends EAX into EDX:EAX, but does NOT modify RAX
    // CDQ only writes to EDX, RAX remains unchanged
    let code = [
        0x99, // CDQ
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0xDEADBEEF_12345678; // EAX = 0x12345678 (positive, MSB=0)
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // RAX is NOT modified by CDQ - only EDX/RDX is written
    assert_eq!(regs.rax, 0xDEADBEEF_12345678, "RAX should be unchanged");
    // EDX = 0 because EAX[31] = 0, and writing to EDX clears upper 32 bits of RDX
    assert_eq!(
        regs.rdx, 0x00000000,
        "RDX should be 0 (sign extension of positive EAX)"
    );
}

#[test]
fn test_cdq_clears_upper_bits_rdx() {
    // CDQ should clear upper 32 bits of RDX
    let code = [
        0x99, // CDQ
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x80000000;
    regs.rdx = 0xCAFEBABE_00000000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rdx, 0xFFFFFFFF,
        "RDX should be 0xFFFFFFFF (upper bits cleared)"
    );
}

#[test]
fn test_cdq_does_not_affect_flags() {
    // CDQ should not modify any flags
    let code = [
        0x99, // CDQ
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x80000000;
    regs.rflags = 0x2 | 0x1 | (1 << 6) | (1 << 7); // CF, ZF, SF set
    let initial_flags = regs.rflags;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rflags, initial_flags, "Flags should not be modified");
}

// ============================================================================
// CQO Tests - Convert Quadword to Octaword (RAX -> RDX:RAX)
// ============================================================================

#[test]
fn test_cqo_positive_value() {
    // CQO with positive value in RAX
    // RAX = 0x123456789ABCDEF0, sign bit (bit 63) = 0, so RDX should be 0x0000000000000000
    let code = [
        0x48, 0x99, // CQO (REX.W + 99)
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x123456789ABCDEF0; // Positive value
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x123456789ABCDEF0, "RAX should remain unchanged");
    assert_eq!(regs.rdx, 0x0000000000000000, "RDX should be 0");
}

#[test]
fn test_cqo_negative_value() {
    // CQO with negative value in RAX
    // RAX = 0x8000000000000000, sign bit = 1, so RDX should be 0xFFFFFFFFFFFFFFFF
    let code = [
        0x48, 0x99, // CQO
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x8000000000000000; // Negative value
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x8000000000000000, "RAX should remain unchanged");
    assert_eq!(
        regs.rdx, 0xFFFFFFFFFFFFFFFF,
        "RDX should be 0xFFFFFFFFFFFFFFFF"
    );
}

#[test]
fn test_cqo_zero() {
    // CQO with zero in RAX
    let code = [
        0x48, 0x99, // CQO
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x0000000000000000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x0000000000000000, "RAX should be 0");
    assert_eq!(regs.rdx, 0x0000000000000000, "RDX should be 0");
}

#[test]
fn test_cqo_max_positive() {
    // CQO with maximum positive 64-bit value
    // 0x7FFFFFFFFFFFFFFF, sign bit = 0
    let code = [
        0x48, 0x99, // CQO
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x7FFFFFFFFFFFFFFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax, 0x7FFFFFFFFFFFFFFF,
        "RAX should be 0x7FFFFFFFFFFFFFFF"
    );
    assert_eq!(regs.rdx, 0x0000000000000000, "RDX should be 0");
}

#[test]
fn test_cqo_min_negative() {
    // CQO with minimum negative 64-bit value
    // 0x8000000000000000, sign bit = 1
    let code = [
        0x48, 0x99, // CQO
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x8000000000000000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax, 0x8000000000000000,
        "RAX should be 0x8000000000000000"
    );
    assert_eq!(
        regs.rdx, 0xFFFFFFFFFFFFFFFF,
        "RDX should be 0xFFFFFFFFFFFFFFFF"
    );
}

#[test]
fn test_cqo_one() {
    // CQO with 1 in RAX
    let code = [
        0x48, 0x99, // CQO
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x0000000000000001;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x0000000000000001, "RAX should be 1");
    assert_eq!(regs.rdx, 0x0000000000000000, "RDX should be 0");
}

#[test]
fn test_cqo_minus_one() {
    // CQO with -1 (0xFFFFFFFFFFFFFFFF) in RAX
    let code = [
        0x48, 0x99, // CQO
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0xFFFFFFFFFFFFFFFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax, 0xFFFFFFFFFFFFFFFF,
        "RAX should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(
        regs.rdx, 0xFFFFFFFFFFFFFFFF,
        "RDX should be 0xFFFFFFFFFFFFFFFF"
    );
}

#[test]
fn test_cqo_sign_boundary_positive() {
    // CQO with value just below sign bit
    // 0x7FFFFFFFFFFFFFFE, sign bit = 0
    let code = [
        0x48, 0x99, // CQO
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x7FFFFFFFFFFFFFFE;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax, 0x7FFFFFFFFFFFFFFE,
        "RAX should be 0x7FFFFFFFFFFFFFFE"
    );
    assert_eq!(regs.rdx, 0x0000000000000000, "RDX should be 0");
}

#[test]
fn test_cqo_sign_boundary_negative() {
    // CQO with value just above sign bit
    // 0x8000000000000001, sign bit = 1
    let code = [
        0x48, 0x99, // CQO
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x8000000000000001;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax, 0x8000000000000001,
        "RAX should be 0x8000000000000001"
    );
    assert_eq!(
        regs.rdx, 0xFFFFFFFFFFFFFFFF,
        "RDX should be 0xFFFFFFFFFFFFFFFF"
    );
}

#[test]
fn test_cqo_overwrites_rdx() {
    // CQO should completely overwrite RDX with sign-extended value
    let code = [
        0x48, 0x99, // CQO
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x1234567890ABCDEF;
    regs.rdx = 0xDEADBEEFCAFEBABE; // Will be overwritten
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x1234567890ABCDEF, "RAX unchanged");
    assert_eq!(
        regs.rdx, 0x0000000000000000,
        "RDX completely overwritten to 0"
    );
}

#[test]
fn test_cqo_does_not_affect_flags() {
    // CQO should not modify any flags
    let code = [
        0x48, 0x99, // CQO
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x8000000000000000;
    regs.rflags = 0x2 | 0x1 | (1 << 6) | (1 << 7); // CF, ZF, SF set
    let initial_flags = regs.rflags;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rflags, initial_flags, "Flags should not be modified");
}

// ============================================================================
// Additional Edge Case Tests
// ============================================================================

#[test]
fn test_cqo_multiple_calls() {
    // Multiple CQO instructions in sequence
    let code = [
        0x48, 0x99, // CQO
        0x48, 0x99, // CQO again
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0xFFFFFFFFFFFFFFFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax, 0xFFFFFFFFFFFFFFFF,
        "RAX unchanged after multiple CQO"
    );
    assert_eq!(regs.rdx, 0xFFFFFFFFFFFFFFFF, "RDX still sign-extended");
}

#[test]
fn test_cdq_multiple_calls() {
    // Multiple CDQ instructions in sequence
    let code = [
        0x99, // CDQ
        0x99, // CDQ again
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0xFFFFFFFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0xFFFFFFFF,
        "EAX unchanged after multiple CDQ"
    );
    assert_eq!(regs.rdx & 0xFFFFFFFF, 0xFFFFFFFF, "EDX still sign-extended");
}

#[test]
fn test_cwd_after_arithmetic() {
    // CWD used after arithmetic operation (common in division setup)
    let code = [
        0x66, 0xb8, 0x00, 0x80, // MOV AX, 0x8000
        0x66, 0x99, // CWD
        0xf4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFF, 0x8000, "AX should be 0x8000");
    assert_eq!(regs.rdx & 0xFFFF, 0xFFFF, "DX should be 0xFFFF");
}

#[test]
fn test_cdq_after_arithmetic() {
    // CDQ used after arithmetic operation
    let code = [
        0xb8, 0x00, 0x00, 0x00, 0x80, // MOV EAX, 0x80000000
        0x99, // CDQ
        0xf4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x80000000,
        "EAX should be 0x80000000"
    );
    assert_eq!(
        regs.rdx & 0xFFFFFFFF,
        0xFFFFFFFF,
        "EDX should be 0xFFFFFFFF"
    );
}

#[test]
fn test_cqo_after_arithmetic() {
    // CQO used after arithmetic operation
    let code = [
        0x48, 0xb8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x80, // MOV RAX, 0x8000000000000000
        0x48, 0x99, // CQO
        0xf4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax, 0x8000000000000000,
        "RAX should be 0x8000000000000000"
    );
    assert_eq!(
        regs.rdx, 0xFFFFFFFFFFFFFFFF,
        "RDX should be 0xFFFFFFFFFFFFFFFF"
    );
}

#[test]
fn test_cqo_preserves_other_registers() {
    // CQO should not affect other registers
    let code = [
        0x48, 0x99, // CQO
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x1234567890ABCDEF;
    regs.rbx = 0x1111111111111111;
    regs.rcx = 0x2222222222222222;
    regs.rsi = 0x3333333333333333;
    regs.rdi = 0x4444444444444444;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx, 0x1111111111111111, "RBX unchanged");
    assert_eq!(regs.rcx, 0x2222222222222222, "RCX unchanged");
    assert_eq!(regs.rsi, 0x3333333333333333, "RSI unchanged");
    assert_eq!(regs.rdi, 0x4444444444444444, "RDI unchanged");
}

#[test]
fn test_cdq_preserves_other_registers() {
    // CDQ should not affect other registers
    let code = [
        0x99, // CDQ
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x12345678;
    regs.rbx = 0x11111111;
    regs.rcx = 0x22222222;
    regs.rsi = 0x33333333;
    regs.rdi = 0x44444444;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx & 0xFFFFFFFF, 0x11111111, "EBX unchanged");
    assert_eq!(regs.rcx & 0xFFFFFFFF, 0x22222222, "ECX unchanged");
    assert_eq!(regs.rsi & 0xFFFFFFFF, 0x33333333, "ESI unchanged");
    assert_eq!(regs.rdi & 0xFFFFFFFF, 0x44444444, "EDI unchanged");
}

#[test]
fn test_cwd_preserves_other_registers() {
    // CWD should not affect other registers
    let code = [
        0x66, 0x99, // CWD
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x1234;
    regs.rbx = 0x1111;
    regs.rcx = 0x2222;
    regs.rsi = 0x3333;
    regs.rdi = 0x4444;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx & 0xFFFF, 0x1111, "BX unchanged");
    assert_eq!(regs.rcx & 0xFFFF, 0x2222, "CX unchanged");
    assert_eq!(regs.rsi & 0xFFFF, 0x3333, "SI unchanged");
    assert_eq!(regs.rdi & 0xFFFF, 0x4444, "DI unchanged");
}
