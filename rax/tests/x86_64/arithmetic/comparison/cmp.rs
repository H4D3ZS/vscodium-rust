//! Tests for the CMP instruction.
//!
//! CMP - Compare Two Operands
//!
//! Compares the first operand with the second operand by performing a subtraction
//! (first - second) and setting flags accordingly, without storing the result.
//!
//! Flags affected: OF, SF, ZF, AF, CF, PF
//!
//! Reference: docs/cmp.txt

use crate::common::*;
use rax::cpu::Registers;

// ============================================================================
// CMP AL, imm8 (opcode 3C ib)
// ============================================================================

#[test]
fn test_cmp_al_imm8_equal() {
    // CMP AL, 10 when AL = 10
    // 3C 0a = CMP AL, 10
    // f4 = HLT
    let code = [0x3c, 0x0a, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 10;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 10, "CMP should not modify operands");
    assert!(zf_set(regs.rflags), "ZF should be set (equal)");
    assert!(!cf_set(regs.rflags), "CF should be clear (no borrow)");
}

#[test]
fn test_cmp_al_imm8_greater() {
    // CMP AL, 5 when AL = 10 (10 > 5, so 10 - 5 = 5 > 0)
    let code = [0x3c, 0x05, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 10;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert!(!zf_set(regs.rflags), "ZF should be clear (not equal)");
    assert!(
        !cf_set(regs.rflags),
        "CF should be clear (no borrow, first >= second)"
    );
    assert!(!sf_set(regs.rflags), "SF should be clear (positive result)");
}

#[test]
fn test_cmp_al_imm8_less() {
    // CMP AL, 15 when AL = 10 (10 < 15, so 10 - 15 = -5 < 0)
    let code = [0x3c, 0x0f, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 10;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert!(!zf_set(regs.rflags), "ZF should be clear (not equal)");
    assert!(
        cf_set(regs.rflags),
        "CF should be set (borrow, first < second)"
    );
    assert!(sf_set(regs.rflags), "SF should be set (negative result)");
}

#[test]
fn test_cmp_al_imm8_signed_overflow() {
    // CMP 0x80, 1 -> 0x80 - 1 = 0x7F (signed overflow)
    let code = [0x3c, 0x01, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 0x80;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert!(of_set(regs.rflags), "OF should be set (signed overflow)");
}

#[test]
fn test_cmp_al_imm8_preserves_register() {
    // Verify CMP doesn't modify AL
    let code = [0x3c, 0x42, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 0xDEADBEEF_12345678;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0xDEADBEEF_12345678, "CMP should not modify RAX");
}

// ============================================================================
// CMP AX/EAX/RAX, imm16/32 (opcode 3D)
// ============================================================================

#[test]
fn test_cmp_ax_imm16_equal() {
    // CMP AX, 0x1234 when AX = 0x1234
    // 66 3D 34 12 = CMP AX, 0x1234
    let code = [0x66, 0x3d, 0x34, 0x12, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 0x1234;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert!(zf_set(regs.rflags), "ZF should be set (equal)");
}

#[test]
fn test_cmp_eax_imm32_greater() {
    // CMP EAX, 0x12345678 when EAX = 0x23456789
    // 3D 78 56 34 12 = CMP EAX, 0x12345678
    let code = [0x3d, 0x78, 0x56, 0x34, 0x12, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 0x23456789;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert!(!zf_set(regs.rflags), "ZF should be clear (not equal)");
    assert!(!cf_set(regs.rflags), "CF should be clear (first > second)");
}

#[test]
fn test_cmp_rax_imm32_sign_extended() {
    // REX.W CMP RAX, -1 (sign-extended to 64-bit)
    // 48 3D ff ff ff ff = CMP RAX, -1
    let code = [0x48, 0x3d, 0xff, 0xff, 0xff, 0xff, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 0xFFFFFFFFFFFFFFFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert!(zf_set(regs.rflags), "ZF should be set (equal to -1)");
}

// ============================================================================
// CMP r/m8, imm8 (opcode 80 /7)
// ============================================================================

#[test]
fn test_cmp_rm8_imm8_register() {
    // CMP CL, 10
    // 80 f9 0a = CMP CL, 10
    let code = [0x80, 0xf9, 0x0a, 0xf4];
    let mut regs = Registers::default();
    regs.rcx = 10;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rcx, 10, "CMP should not modify CL");
    assert!(zf_set(regs.rflags), "ZF should be set");
}

#[test]
fn test_cmp_rm8_imm8_memory() {
    // CMP BYTE PTR [RBX], 25
    // 80 3b 19 = CMP BYTE PTR [RBX], 25
    let code = [0x80, 0x3b, 0x19, 0xf4];
    let mut regs = Registers::default();
    regs.rbx = DATA_ADDR;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    write_mem_u8(&mem, 25);

    let _ = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_mem_u8(&mem), 25, "CMP should not modify memory");
}

// ============================================================================
// CMP r/m16/32/64, imm32 (opcode 81 /7)
// ============================================================================

#[test]
fn test_cmp_rm32_imm32_register() {
    // CMP ECX, 0x12345678
    // 81 f9 78 56 34 12 = CMP ECX, 0x12345678
    let code = [0x81, 0xf9, 0x78, 0x56, 0x34, 0x12, 0xf4];
    let mut regs = Registers::default();
    regs.rcx = 0x12345678;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert!(zf_set(regs.rflags), "ZF should be set (equal)");
}

#[test]
fn test_cmp_rm64_imm32_register() {
    // REX.W CMP RCX, -1 (sign-extended)
    // 48 81 f9 ff ff ff ff = CMP RCX, -1
    let code = [0x48, 0x81, 0xf9, 0xff, 0xff, 0xff, 0xff, 0xf4];
    let mut regs = Registers::default();
    regs.rcx = 0xFFFFFFFFFFFFFFFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert!(zf_set(regs.rflags), "ZF should be set");
}

// ============================================================================
// CMP r/m16/32/64, imm8 sign-extended (opcode 83 /7)
// ============================================================================

#[test]
fn test_cmp_rm32_imm8_positive() {
    // CMP ECX, 10
    // 83 f9 0a = CMP ECX, 10
    let code = [0x83, 0xf9, 0x0a, 0xf4];
    let mut regs = Registers::default();
    regs.rcx = 100;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert!(!cf_set(regs.rflags), "CF should be clear (100 > 10)");
}

#[test]
fn test_cmp_rm32_imm8_negative() {
    // CMP ECX, -10 (0xF6 sign-extended to 0xFFFFFFF6)
    // 83 f9 f6 = CMP ECX, -10
    let code = [0x83, 0xf9, 0xf6, 0xf4];
    let mut regs = Registers::default();
    regs.rcx = 100;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let regs = run_until_hlt(&mut vcpu).unwrap();
    // 100 compared to -10 (as unsigned: 0xFFFFFFF6)
    // 100 - 0xFFFFFFF6 produces borrow
    assert!(
        cf_set(regs.rflags),
        "CF should be set (100 < 0xFFFFFFF6 unsigned)"
    );
}

// ============================================================================
// CMP r/m8, r8 (opcode 38 /r)
// ============================================================================

#[test]
fn test_cmp_rm8_r8_register_equal() {
    // CMP AL, CL when both are 10
    // 38 c8 = CMP AL, CL
    let code = [0x38, 0xc8, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 10;
    regs.rcx = 10;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 10, "AL should not change");
    assert_eq!(regs.rcx, 10, "CL should not change");
    assert!(zf_set(regs.rflags), "ZF should be set (equal)");
}

#[test]
fn test_cmp_rm8_r8_register_greater() {
    // CMP AL, CL when AL = 15, CL = 10
    let code = [0x38, 0xc8, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 15;
    regs.rcx = 10;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert!(!cf_set(regs.rflags), "CF should be clear (AL > CL)");
    assert!(!sf_set(regs.rflags), "SF should be clear");
}

#[test]
fn test_cmp_rm8_r8_memory() {
    // CMP [RBX], CL
    // 38 0b = CMP [RBX], CL
    let code = [0x38, 0x0b, 0xf4];
    let mut regs = Registers::default();
    regs.rbx = DATA_ADDR;
    regs.rcx = 30;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    write_mem_u8(&mem, 30);

    let _ = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_mem_u8(&mem), 30, "Memory should not change");
}

// ============================================================================
// CMP r/m16/32/64, r16/32/64 (opcode 39 /r)
// ============================================================================

#[test]
fn test_cmp_rm32_r32_register() {
    // CMP EAX, ECX
    // 39 c8 = CMP EAX, ECX
    let code = [0x39, 0xc8, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 0x12345678;
    regs.rcx = 0x12345678;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert!(zf_set(regs.rflags), "ZF should be set (equal)");
}

#[test]
fn test_cmp_rm64_r64_register() {
    // REX.W CMP RAX, RCX
    // 48 39 c8 = CMP RAX, RCX
    let code = [0x48, 0x39, 0xc8, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 0xFFFFFFFF_00000000;
    regs.rcx = 0x00000000_FFFFFFFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert!(
        !cf_set(regs.rflags),
        "CF should be clear (RAX > RCX unsigned)"
    );
}

// ============================================================================
// CMP r8, r/m8 (opcode 3A /r)
// ============================================================================

#[test]
fn test_cmp_r8_rm8_register() {
    // CMP CL, AL
    // 3A c8 = CMP CL, AL
    let code = [0x3a, 0xc8, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 10;
    regs.rcx = 10;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert!(zf_set(regs.rflags), "ZF should be set (equal)");
}

#[test]
fn test_cmp_r8_rm8_memory() {
    // CMP CL, [RBX]
    // 3A 0b = CMP CL, [RBX]
    let code = [0x3a, 0x0b, 0xf4];
    let mut regs = Registers::default();
    regs.rbx = DATA_ADDR;
    regs.rcx = 50;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    write_mem_u8(&mem, 30);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert!(!cf_set(regs.rflags), "CF should be clear (50 > 30)");
}

// ============================================================================
// CMP r16/32/64, r/m16/32/64 (opcode 3B /r)
// ============================================================================

#[test]
fn test_cmp_r32_rm32_register() {
    // CMP ECX, EAX
    // 3B c8 = CMP ECX, EAX
    let code = [0x3b, 0xc8, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 0x11111111;
    regs.rcx = 0x22222222;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert!(!cf_set(regs.rflags), "CF should be clear (ECX > EAX)");
}

#[test]
fn test_cmp_r64_rm64_register() {
    // REX.W CMP RCX, RAX
    // 48 3B c8 = CMP RCX, RAX
    let code = [0x48, 0x3b, 0xc8, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 0xFFFFFFFFFFFFFFFF;
    regs.rcx = 0x1111111111111111;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert!(cf_set(regs.rflags), "CF should be set (RCX < RAX)");
}

// ============================================================================
// Condition Code Tests
// ============================================================================

#[test]
fn test_cmp_for_je_condition() {
    // Testing ZF for JE (jump if equal)
    // CMP EAX, 42 when EAX = 42
    let code = [0x3d, 0x2a, 0x00, 0x00, 0x00, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 42;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert!(zf_set(regs.rflags), "ZF=1 for JE condition");
}

#[test]
fn test_cmp_for_jne_condition() {
    // Testing ZF for JNE (jump if not equal)
    // CMP EAX, 42 when EAX = 50
    let code = [0x3d, 0x2a, 0x00, 0x00, 0x00, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 50;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert!(!zf_set(regs.rflags), "ZF=0 for JNE condition");
}

#[test]
fn test_cmp_for_jb_condition() {
    // Testing CF for JB (jump if below, unsigned <)
    // CMP EAX, 100 when EAX = 50
    let code = [0x3d, 0x64, 0x00, 0x00, 0x00, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 50;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert!(cf_set(regs.rflags), "CF=1 for JB condition (50 < 100)");
}

#[test]
fn test_cmp_for_jae_condition() {
    // Testing CF for JAE (jump if above or equal, unsigned >=)
    // CMP EAX, 50 when EAX = 100
    let code = [0x3d, 0x32, 0x00, 0x00, 0x00, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 100;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert!(!cf_set(regs.rflags), "CF=0 for JAE condition (100 >= 50)");
}

#[test]
fn test_cmp_for_jl_condition() {
    // Testing SF^OF for JL (jump if less, signed <)
    // CMP EAX, 0 when EAX = -10 (0xFFFFFFF6)
    let code = [0x3d, 0x00, 0x00, 0x00, 0x00, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 0xFFFFFFF6; // -10 in two's complement
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let regs = run_until_hlt(&mut vcpu).unwrap();
    // -10 - 0 = -10 (negative), so SF=1, OF=0 -> SF^OF = 1
    assert!(sf_set(regs.rflags), "SF=1");
    assert!(!of_set(regs.rflags), "OF=0");
    // For JL: (SF^OF) == 1
}

#[test]
fn test_cmp_for_jg_condition() {
    // Testing ZF=0 AND SF=OF for JG (jump if greater, signed >)
    // CMP EAX, 0 when EAX = 10
    let code = [0x3d, 0x00, 0x00, 0x00, 0x00, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 10;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert!(!zf_set(regs.rflags), "ZF=0 (not equal)");
    assert!(!sf_set(regs.rflags), "SF=0 (positive result)");
    assert!(!of_set(regs.rflags), "OF=0 (no overflow)");
    // For JG: ZF=0 AND SF=OF (0=0)
}

// ============================================================================
// Edge Cases
// ============================================================================

#[test]
fn test_cmp_zero_with_zero() {
    // CMP 0, 0 should set ZF and clear CF, SF, OF
    let code = [0x3d, 0x00, 0x00, 0x00, 0x00, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert!(zf_set(regs.rflags), "ZF should be set");
    assert!(!cf_set(regs.rflags), "CF should be clear");
    assert!(!sf_set(regs.rflags), "SF should be clear");
    assert!(!of_set(regs.rflags), "OF should be clear");
}

#[test]
fn test_cmp_max_with_max() {
    // CMP 0xFFFFFFFF, 0xFFFFFFFF
    let code = [0x3d, 0xff, 0xff, 0xff, 0xff, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 0xFFFFFFFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert!(zf_set(regs.rflags), "ZF should be set (equal)");
}

#[test]
fn test_cmp_parity_flag() {
    // CMP should set parity flag based on result
    // 0x0F - 0x03 = 0x0C (binary 00001100, 2 bits = even parity)
    let code = [0x3c, 0x03, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 0x0F;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert!(pf_set(regs.rflags), "PF should be set (even parity)");
}

#[test]
fn test_cmp_auxiliary_flag() {
    // CMP should set AF when borrow from bit 4
    // 0x10 - 0x01 = 0x0F (borrow from bit 4)
    let code = [0x3c, 0x01, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 0x10;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert!(af_set(regs.rflags), "AF should be set (borrow from bit 4)");
}

// ============================================================================
// Extended Registers
// ============================================================================

#[test]
fn test_cmp_r8_extended() {
    // REX.B CMP R8D, 100
    // 41 83 f8 64 = CMP R8D, 100
    let code = [0x41, 0x83, 0xf8, 0x64, 0xf4];
    let mut regs = Registers::default();
    regs.r8 = 150;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.r8, 150, "R8 should not change");
    assert!(!cf_set(regs.rflags), "CF should be clear (150 > 100)");
}

#[test]
fn test_cmp_r15_extended() {
    // REX.WB CMP R15, RAX
    // 49 39 c7 = CMP R15, RAX
    let code = [0x49, 0x39, 0xc7, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 0x1000;
    regs.r15 = 0x1000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.r15, 0x1000, "R15 should not change");
    assert!(zf_set(regs.rflags), "ZF should be set (equal)");
}
