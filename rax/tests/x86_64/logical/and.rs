use crate::common::*;
use crate::common::{run_until_hlt, setup_vm};
use rax::backend::emulator::x86_64::flags;
use rax::cpu::Registers;

// AND — Logical AND
//
// Opcodes:
// - 24 ib           AND AL, imm8
// - 25 iw/id        AND AX/EAX/RAX, imm16/32
// - 80 /4 ib        AND r/m8, imm8
// - 81 /4 iw/id     AND r/m16/32/64, imm16/32
// - 83 /4 ib        AND r/m16/32/64, imm8 (sign-extended)
// - 20 /r           AND r/m8, r8
// - 21 /r           AND r/m16/32/64, r16/32/64
// - 22 /r           AND r8, r/m8
// - 23 /r           AND r16/32/64, r/m16/32/64
//
// Operation: DEST := DEST AND SRC
//
// Flags: OF and CF are CLEARED.
//        SF, ZF, PF are set according to result.
//        AF is undefined (not tested).

// ============================================================================
// AND AL, imm8 - Test accumulator with immediate
// ============================================================================

#[test]
fn test_and_al_imm8_basic() {
    let code = [0x24, 0x0F, 0xf4]; // AND AL, 0x0F; HLT
    let mut regs = Registers::default();
    regs.rax = 0xAB; // AL = 0xAB (10101011)
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x0B, "AL: 0xAB AND 0x0F = 0x0B");
    assert!(!zf_set(regs.rflags), "ZF should be clear");
    assert!(!cf_set(regs.rflags), "CF should be clear");
    assert!(!of_set(regs.rflags), "OF should be clear");
}

#[test]
fn test_and_al_imm8_zero_result() {
    let code = [0x24, 0x00, 0xf4]; // AND AL, 0
    let mut regs = Registers::default();
    regs.rax = 0xFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0, "AL: 0xFF AND 0 = 0");
    assert!(zf_set(regs.rflags), "ZF should be set");
}

#[test]
fn test_and_al_imm8_all_ones() {
    let code = [0x24, 0xFF, 0xf4]; // AND AL, 0xFF
    let mut regs = Registers::default();
    regs.rax = 0x42;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x42, "AL: 0x42 AND 0xFF = 0x42");
}

#[test]
fn test_and_al_imm8_sign_flag() {
    let code = [0x24, 0x80, 0xf4]; // AND AL, 0x80
    let mut regs = Registers::default();
    regs.rax = 0xFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x80, "AL: 0xFF AND 0x80 = 0x80");
    assert!(sf_set(regs.rflags), "SF should be set (high bit = 1)");
}

#[test]
fn test_and_al_imm8_parity_even() {
    let code = [0x24, 0x03, 0xf4]; // AND AL, 0x03
    let mut regs = Registers::default();
    regs.rax = 0xFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x03);
    assert!(pf_set(regs.rflags), "PF should be set (even parity)");
}

#[test]
fn test_and_al_imm8_parity_odd() {
    let code = [0x24, 0x07, 0xf4]; // AND AL, 0x07
    let mut regs = Registers::default();
    regs.rax = 0xFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x07);
    assert!(!pf_set(regs.rflags), "PF should be clear (odd parity)");
}

// ============================================================================
// AND AX, imm16 - 16-bit accumulator
// ============================================================================

#[test]
fn test_and_ax_imm16_basic() {
    let code = [0x66, 0x25, 0x0F, 0x00, 0xf4]; // AND AX, 0x000F
    let mut regs = Registers::default();
    regs.rax = 0x1234;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFF, 0x0004, "AX: 0x1234 AND 0x000F = 0x0004");
}

#[test]
fn test_and_ax_imm16_high_byte() {
    let code = [0x66, 0x25, 0x00, 0xFF, 0xf4]; // AND AX, 0xFF00
    let mut regs = Registers::default();
    regs.rax = 0xABCD;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFF, 0xAB00, "AX: keep only high byte");
}

#[test]
fn test_and_ax_imm16_zero() {
    let code = [0x66, 0x25, 0x00, 0x00, 0xf4]; // AND AX, 0
    let mut regs = Registers::default();
    regs.rax = 0xFFFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFF, 0, "AX: AND with 0 gives 0");
    assert!(zf_set(regs.rflags), "ZF should be set");
}

// ============================================================================
// AND EAX, imm32 - 32-bit accumulator
// ============================================================================

#[test]
fn test_and_eax_imm32_basic() {
    let code = [0x25, 0xFF, 0x00, 0x00, 0x00, 0xf4]; // AND EAX, 0x000000FF
    let mut regs = Registers::default();
    regs.rax = 0x12345678;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x00000078, "EAX: mask to low byte");
}

#[test]
fn test_and_eax_imm32_pattern() {
    let code = [0x25, 0x00, 0xFF, 0x00, 0x00, 0xf4]; // AND EAX, 0x0000FF00
    let mut regs = Registers::default();
    regs.rax = 0x12345678;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x00005600, "EAX: mask middle byte");
}

#[test]
fn test_and_eax_imm32_high_bit() {
    let code = [0x25, 0x00, 0x00, 0x00, 0x80, 0xf4]; // AND EAX, 0x80000000
    let mut regs = Registers::default();
    regs.rax = 0xFFFFFFFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x80000000, "EAX: keep only high bit");
    assert!(sf_set(regs.rflags), "SF should be set");
}

// ============================================================================
// AND RAX, imm32 - 64-bit accumulator (imm32 sign-extended)
// ============================================================================

#[test]
fn test_and_rax_imm32_basic() {
    let code = [0x48, 0x25, 0xFF, 0xFF, 0x00, 0x00, 0xf4]; // AND RAX, 0x0000FFFF
    let mut regs = Registers::default();
    regs.rax = 0x123456789ABCDEF0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x000000000000DEF0, "RAX: mask to low word");
}

#[test]
fn test_and_rax_imm32_negative() {
    let code = [0x48, 0x25, 0xFF, 0xFF, 0xFF, 0xFF, 0xf4]; // AND RAX, 0xFFFFFFFF
    let mut regs = Registers::default();
    regs.rax = 0x123456789ABCDEF0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Sign-extended to 0xFFFFFFFFFFFFFFFF
    assert_eq!(regs.rax, 0x123456789ABCDEF0, "RAX: no change with all ones");
}

// ============================================================================
// AND r/m8, imm8
// ============================================================================

#[test]
fn test_and_rm8_imm8_bl() {
    let code = [0x80, 0xe3, 0x0F, 0xf4]; // AND BL, 0x0F
    let mut regs = Registers::default();
    regs.rbx = 0xFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx & 0xFF, 0x0F, "BL: 0xFF AND 0x0F = 0x0F");
}

#[test]
fn test_and_rm8_imm8_cl() {
    let code = [0x80, 0xe1, 0xAA, 0xf4]; // AND CL, 0xAA
    let mut regs = Registers::default();
    regs.rcx = 0xFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx & 0xFF, 0xAA, "CL: 0xFF AND 0xAA = 0xAA");
}

#[test]
fn test_and_rm8_imm8_dh() {
    let code = [0x80, 0xe6, 0x55, 0xf4]; // AND DH, 0x55
    let mut regs = Registers::default();
    regs.rdx = 0xFF00;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!((regs.rdx >> 8) & 0xFF, 0x55, "DH: 0xFF AND 0x55 = 0x55");
}

// ============================================================================
// AND r/m16, imm16
// ============================================================================

#[test]
fn test_and_rm16_imm16_bx() {
    let code = [0x66, 0x81, 0xe3, 0xF0, 0x0F, 0xf4]; // AND BX, 0x0FF0
    let mut regs = Registers::default();
    regs.rbx = 0xFFFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx & 0xFFFF, 0x0FF0, "BX: 0xFFFF AND 0x0FF0 = 0x0FF0");
}

#[test]
fn test_and_rm16_imm16_si() {
    let code = [0x66, 0x81, 0xe6, 0x00, 0xFF, 0xf4]; // AND SI, 0xFF00
    let mut regs = Registers::default();
    regs.rsi = 0x1234;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rsi & 0xFFFF, 0x1200, "SI: keep only high byte");
}

// ============================================================================
// AND r/m32, imm32
// ============================================================================

#[test]
fn test_and_rm32_imm32_ebx() {
    let code = [0x81, 0xe3, 0x00, 0xFF, 0x00, 0x00, 0xf4]; // AND EBX, 0x0000FF00
    let mut regs = Registers::default();
    regs.rbx = 0x12345678;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx, 0x00005600, "EBX: mask middle byte");
}

#[test]
fn test_and_rm32_imm32_esi() {
    let code = [0x81, 0xe6, 0xFF, 0x00, 0x00, 0x00, 0xf4]; // AND ESI, 0x000000FF
    let mut regs = Registers::default();
    regs.rsi = 0xABCDEF01;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rsi, 0x00000001, "ESI: mask to low byte");
}

// ============================================================================
// AND r/m64, imm32 (sign-extended)
// ============================================================================

#[test]
fn test_and_rm64_imm32_rbx() {
    let code = [0x48, 0x81, 0xe3, 0xFF, 0xFF, 0xFF, 0x00, 0xf4]; // AND RBX, 0x00FFFFFF
    let mut regs = Registers::default();
    regs.rbx = 0x123456789ABCDEF0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx, 0x0000000000BCDEF0, "RBX: mask low 3 bytes");
}

#[test]
fn test_and_rm64_imm32_r8() {
    let code = [0x49, 0x81, 0xe0, 0xFF, 0x00, 0x00, 0x00, 0xf4]; // AND R8, 0x000000FF
    let mut regs = Registers::default();
    regs.r8 = 0xFFFFFFFFFFFFFFFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.r8, 0x00000000000000FF, "R8: mask to low byte");
}

// ============================================================================
// AND r/m, imm8 (sign-extended)
// ============================================================================

#[test]
fn test_and_rm16_imm8_sign_ext() {
    let code = [0x66, 0x83, 0xe3, 0xFF, 0xf4]; // AND BX, 0xFF (sign-extended to 0xFFFF)
    let mut regs = Registers::default();
    regs.rbx = 0x1234;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx & 0xFFFF, 0x1234, "BX: AND with 0xFFFF (no change)");
}

#[test]
fn test_and_rm32_imm8_sign_ext() {
    let code = [0x83, 0xe3, 0x0F, 0xf4]; // AND EBX, 0x0F
    let mut regs = Registers::default();
    regs.rbx = 0xFFFFFFFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx, 0x0000000F, "EBX: AND with sign-extended imm8");
}

#[test]
fn test_and_rm64_imm8_sign_ext() {
    let code = [0x48, 0x83, 0xe0, 0xF0, 0xf4]; // AND RAX, 0xFFFFFFFFFFFFFFF0
    let mut regs = Registers::default();
    regs.rax = 0x123456789ABCDEF7;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax, 0x123456789ABCDEF0,
        "RAX: align to 16-byte boundary"
    );
}

// ============================================================================
// AND r/m, r (destination is r/m)
// ============================================================================

#[test]
fn test_and_rm8_r8_al_bl() {
    let code = [0x20, 0xd8, 0xf4]; // AND AL, BL
    let mut regs = Registers::default();
    regs.rax = 0xFF;
    regs.rbx = 0x0F;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x0F, "AL: 0xFF AND 0x0F = 0x0F");
}

#[test]
fn test_and_rm16_r16_ax_bx() {
    let code = [0x66, 0x21, 0xd8, 0xf4]; // AND AX, BX
    let mut regs = Registers::default();
    regs.rax = 0xFFFF;
    regs.rbx = 0x00FF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFF, 0x00FF, "AX: 0xFFFF AND 0x00FF = 0x00FF");
}

#[test]
fn test_and_rm32_r32_eax_ebx() {
    let code = [0x21, 0xd8, 0xf4]; // AND EAX, EBX
    let mut regs = Registers::default();
    regs.rax = 0x12345678;
    regs.rbx = 0xF0F0F0F0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x10305070, "EAX: bitwise AND with EBX");
}

#[test]
fn test_and_rm64_r64_rax_rbx() {
    let code = [0x48, 0x21, 0xd8, 0xf4]; // AND RAX, RBX
    let mut regs = Registers::default();
    regs.rax = 0xFFFFFFFFFFFFFFFF;
    regs.rbx = 0x00000000FFFFFFFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x00000000FFFFFFFF, "RAX: mask to low 32 bits");
}

// ============================================================================
// AND r, r/m (destination is register)
// ============================================================================

#[test]
fn test_and_r8_rm8_al_bl() {
    let code = [0x22, 0xc3, 0xf4]; // AND AL, BL
    let mut regs = Registers::default();
    regs.rax = 0xAA;
    regs.rbx = 0x55;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0, "AL: 0xAA AND 0x55 = 0");
    assert!(zf_set(regs.rflags), "ZF should be set");
}

#[test]
fn test_and_r16_rm16_ax_bx() {
    let code = [0x66, 0x23, 0xc3, 0xf4]; // AND AX, BX
    let mut regs = Registers::default();
    regs.rax = 0x1234;
    regs.rbx = 0x0F0F;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFF, 0x0204, "AX: 0x1234 AND 0x0F0F = 0x0204");
}

#[test]
fn test_and_r32_rm32_eax_ebx() {
    let code = [0x23, 0xc3, 0xf4]; // AND EAX, EBX
    let mut regs = Registers::default();
    regs.rax = 0xAAAAAAAA;
    regs.rbx = 0x55555555;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0, "EAX: alternating bits AND = 0");
    assert!(zf_set(regs.rflags), "ZF should be set");
}

#[test]
fn test_and_r64_rm64_rax_rbx() {
    let code = [0x48, 0x23, 0xc3, 0xf4]; // AND RAX, RBX
    let mut regs = Registers::default();
    regs.rax = 0xFF00FF00FF00FF00;
    regs.rbx = 0x00FF00FF00FF00FF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0, "RAX: alternating bytes AND = 0");
    assert!(zf_set(regs.rflags), "ZF should be set");
}

// ============================================================================
// AND with various register combinations
// ============================================================================

#[test]
fn test_and_cl_dl() {
    let code = [0x20, 0xd1, 0xf4]; // AND CL, DL
    let mut regs = Registers::default();
    regs.rcx = 0xFF;
    regs.rdx = 0x3C;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx & 0xFF, 0x3C, "CL: 0xFF AND 0x3C = 0x3C");
}

#[test]
fn test_and_ecx_edx() {
    let code = [0x21, 0xd1, 0xf4]; // AND ECX, EDX
    let mut regs = Registers::default();
    regs.rcx = 0xF0F0F0F0;
    regs.rdx = 0x0F0F0F0F;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 0, "ECX: complementary patterns AND = 0");
}

#[test]
fn test_and_rsi_rdi() {
    let code = [0x48, 0x21, 0xfe, 0xf4]; // AND RSI, RDI
    let mut regs = Registers::default();
    regs.rsi = 0xAAAAAAAAAAAAAAAA;
    regs.rdi = 0xAAAAAAAAAAAAAAAA;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rsi, 0xAAAAAAAAAAAAAAAA, "RSI: AND with same value");
}

// ============================================================================
// Extended registers (R8-R15)
// ============================================================================

#[test]
fn test_and_r8b_imm8() {
    let code = [0x41, 0x80, 0xe0, 0x0F, 0xf4]; // AND R8B, 0x0F
    let mut regs = Registers::default();
    regs.r8 = 0xFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.r8 & 0xFF, 0x0F, "R8B: 0xFF AND 0x0F = 0x0F");
}

#[test]
fn test_and_r9w_imm16() {
    let code = [0x66, 0x41, 0x81, 0xe1, 0xF0, 0x0F, 0xf4]; // AND R9W, 0x0FF0
    let mut regs = Registers::default();
    regs.r9 = 0xFFFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.r9 & 0xFFFF, 0x0FF0, "R9W: mask");
}

#[test]
fn test_and_r10d_imm32() {
    let code = [0x41, 0x81, 0xe2, 0xFF, 0x00, 0x00, 0x00, 0xf4]; // AND R10D, 0x000000FF
    let mut regs = Registers::default();
    regs.r10 = 0x12345678;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.r10, 0x00000078, "R10D: mask to low byte");
}

#[test]
fn test_and_r11_imm32() {
    let code = [0x49, 0x81, 0xe3, 0xFF, 0xFF, 0x00, 0x00, 0xf4]; // AND R11, 0x0000FFFF
    let mut regs = Registers::default();
    regs.r11 = 0x123456789ABCDEF0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.r11, 0x000000000000DEF0, "R11: mask to low word");
}

#[test]
fn test_and_r12d_r13d() {
    let code = [0x45, 0x21, 0xec, 0xf4]; // AND R12D, R13D
    let mut regs = Registers::default();
    regs.r12 = 0xFFFFFFFF;
    regs.r13 = 0x12345678;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.r12, 0x12345678, "R12D: AND with R13D");
}

#[test]
fn test_and_r14_r15() {
    let code = [0x4d, 0x21, 0xfe, 0xf4]; // AND R14, R15
    let mut regs = Registers::default();
    regs.r14 = 0xFFFFFFFF00000000;
    regs.r15 = 0x00000000FFFFFFFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.r14, 0, "R14: no overlapping bits");
    assert!(zf_set(regs.rflags), "ZF should be set");
}

// ============================================================================
// Memory operands
// ============================================================================

#[test]
fn test_and_byte_ptr_imm8() {
    let code = [
        0x80, 0x25, 0xf9, 0x0f, 0x00, 0x00, 0x0F, // AND BYTE PTR [rip+0x0FF9], 0x0F
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u8(&mem, 0xFF);

    let _ = run_until_hlt(&mut vcpu).unwrap();
    let result = read_mem_u8(&mem);

    assert_eq!(result, 0x0F, "Memory: 0xFF AND 0x0F = 0x0F");
}

#[test]
fn test_and_word_ptr_imm16() {
    let code = [
        0x66, 0x81, 0x25, 0xf7, 0x0f, 0x00, 0x00, 0xF0,
        0x0F, // AND WORD PTR [rip+0x0FF7], 0x0FF0
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u16(&mem, 0xFFFF);

    let _ = run_until_hlt(&mut vcpu).unwrap();
    let result = read_mem_u16(&mem);

    assert_eq!(result, 0x0FF0, "Memory: word AND");
}

#[test]
fn test_and_dword_ptr_imm32() {
    let code = [
        0x81, 0x25, 0xf6, 0x0f, 0x00, 0x00, 0xFF, 0x00, 0x00,
        0x00, // AND DWORD PTR [rip+0x0FF6], 0x000000FF
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u32(&mem, 0x12345678);

    let _ = run_until_hlt(&mut vcpu).unwrap();
    let result = read_mem_u32(&mem);

    assert_eq!(result, 0x00000078, "Memory: dword mask to low byte");
}

#[test]
fn test_and_qword_ptr_imm32() {
    let code = [
        0x48, 0x81, 0x25, 0xf5, 0x0f, 0x00, 0x00, 0xFF, 0xFF, 0x00,
        0x00, // AND QWORD PTR [rip+0x0FF5], 0x0000FFFF
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u64(&mem, 0x123456789ABCDEF0);

    let _ = run_until_hlt(&mut vcpu).unwrap();
    let result = read_mem_u64(&mem);

    assert_eq!(result, 0x000000000000DEF0, "Memory: qword mask to low word");
}

// ============================================================================
// Flag behavior tests
// ============================================================================

#[test]
fn test_and_clears_of_cf() {
    let code = [0x24, 0xFF, 0xf4]; // AND AL, 0xFF
    let mut regs = Registers::default();
    regs.rax = 0xFF;
    regs.rflags = 0x2 | flags::bits::OF | flags::bits::CF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(!of_set(regs.rflags), "OF cleared by AND");
    assert!(!cf_set(regs.rflags), "CF cleared by AND");
}

// ============================================================================
// Practical use cases
// ============================================================================

#[test]
fn test_and_mask_low_nibble() {
    let code = [0x24, 0x0F, 0xf4]; // AND AL, 0x0F
    let mut regs = Registers::default();
    regs.rax = 0xB7;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x07, "Extract low nibble");
}

#[test]
fn test_and_check_bit_set() {
    let code = [0x24, 0x10, 0xf4]; // AND AL, 0x10
    let mut regs = Registers::default();
    regs.rax = 0x1F;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x10, "Bit 4 is set");
    assert!(!zf_set(regs.rflags), "ZF clear means bit was set");
}

#[test]
fn test_and_check_bit_clear() {
    let code = [0x24, 0x10, 0xf4]; // AND AL, 0x10
    let mut regs = Registers::default();
    regs.rax = 0x0F;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0, "Bit 4 is clear");
    assert!(zf_set(regs.rflags), "ZF set means bit was clear");
}

// ============================================================================
// Strengthened AND tests (appended): exact result with the full flag contract
// (OF and CF always cleared; SF/ZF/PF per the result) across operand sizes.
// ============================================================================

#[test]
fn test_strict_and_r64_full_flags() {
    // AND RAX, RBX: 0xF0F0_F0F0_F0F0_F0FF & 0x8000_0000_0000_00F0 = 0x8000_0000_0000_00F0.
    // Bit 63 set -> SF=1; nonzero -> ZF=0; low byte 0xF0 -> PF=1; CF=0, OF=0.
    let code = [0x48, 0x21, 0xd8, 0xf4]; // AND RAX, RBX
    let mut regs = Registers::default();
    regs.rax = 0xF0F0_F0F0_F0F0_F0FF;
    regs.rbx = 0x8000_0000_0000_00F0;
    regs.rflags = 0x2 | 0x1 | 0x800; // seed CF and OF to confirm clearing
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x8000_0000_0000_00F0, "AND result");
    assert!(sf_set(regs.rflags), "SF set (bit 63)");
    assert!(!zf_set(regs.rflags), "ZF clear (nonzero)");
    assert!(pf_set(regs.rflags), "PF set (0xF0 even parity)");
    assert!(!cf_set(regs.rflags), "CF cleared by AND");
    assert!(!of_set(regs.rflags), "OF cleared by AND");
}

#[test]
fn test_strict_and_zero_result_full_flags() {
    // AND RAX, RBX with disjoint masks -> 0: ZF=1, SF=0, PF=1, CF=0, OF=0.
    let code = [0x48, 0x21, 0xd8, 0xf4]; // AND RAX, RBX
    let mut regs = Registers::default();
    regs.rax = 0xFFFF_0000_FFFF_0000;
    regs.rbx = 0x0000_FFFF_0000_FFFF;
    regs.rflags = 0x2 | 0x1 | 0x800;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0);
    assert!(zf_set(regs.rflags), "ZF set");
    assert!(!sf_set(regs.rflags), "SF clear");
    assert!(pf_set(regs.rflags), "PF set (0)");
    assert!(
        !cf_set(regs.rflags) && !of_set(regs.rflags),
        "CF/OF cleared"
    );
}

#[test]
fn test_strict_and_r32_zero_extends() {
    // AND EAX, EBX clears the upper 32 bits of RAX.
    let code = [0x21, 0xd8, 0xf4]; // AND EAX, EBX
    let mut regs = Registers::default();
    regs.rax = 0xFFFF_FFFF_00FF_00FF;
    regs.rbx = 0x0000_0000_0F0F_0F0F;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax, 0x0000_0000_000F_000F,
        "32-bit AND zero-extends upper RAX"
    );
}

#[test]
fn test_strict_and_mem_operand() {
    // AND qword [RBX], RAX: mask a memory operand in place.
    let code = [0x48, 0x21, 0x03, 0xf4]; // AND [RBX], RAX
    let mut regs = Registers::default();
    regs.rax = 0x00FF_00FF_00FF_00FF;
    regs.rbx = DATA_ADDR;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    write_mem_at_u64(&mem, DATA_ADDR, 0xFFFF_FFFF_FFFF_FFFF);
    let _ = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        read_mem_at_u64(&mem, DATA_ADDR),
        0x00FF_00FF_00FF_00FF,
        "AND applied to memory"
    );
}
