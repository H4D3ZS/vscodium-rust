use crate::common::*;
use crate::common::{run_until_hlt, setup_vm};
use rax::backend::emulator::x86_64::flags;
use rax::cpu::Registers;

// OR — Logical Inclusive OR
//
// Opcodes:
// - 0C ib           OR AL, imm8
// - 0D iw/id        OR AX/EAX/RAX, imm16/32
// - 80 /1 ib        OR r/m8, imm8
// - 81 /1 iw/id     OR r/m16/32/64, imm16/32
// - 83 /1 ib        OR r/m16/32/64, imm8 (sign-extended)
// - 08 /r           OR r/m8, r8
// - 09 /r           OR r/m16/32/64, r16/32/64
// - 0A /r           OR r8, r/m8
// - 0B /r           OR r16/32/64, r/m16/32/64
//
// Operation: DEST := DEST OR SRC
//
// Flags: OF and CF are CLEARED.
//        SF, ZF, PF are set according to result.
//        AF is undefined (not tested).

// ============================================================================
// OR AL, imm8
// ============================================================================

#[test]
fn test_or_al_imm8_basic() {
    let code = [0x0C, 0x0F, 0xf4]; // OR AL, 0x0F; HLT
    let mut regs = Registers::default();
    regs.rax = 0xA0; // AL = 0xA0 (10100000)
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // 0xA0 | 0x0F = 10100000 | 00001111 = 10101111 = 0xAF
    assert_eq!(regs.rax & 0xFF, 0xAF, "AL: 0xA0 OR 0x0F = 0xAF");
    assert!(!zf_set(regs.rflags), "ZF should be clear");
    assert!(!cf_set(regs.rflags), "CF should be clear");
    assert!(!of_set(regs.rflags), "OF should be clear");
}

#[test]
fn test_or_al_imm8_zero_identity() {
    let code = [0x0C, 0x00, 0xf4]; // OR AL, 0
    let mut regs = Registers::default();
    regs.rax = 0x42;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x42, "AL: 0x42 OR 0 = 0x42 (identity)");
}

#[test]
fn test_or_al_imm8_all_ones() {
    let code = [0x0C, 0xFF, 0xf4]; // OR AL, 0xFF
    let mut regs = Registers::default();
    regs.rax = 0x00;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0xFF, "AL: 0x00 OR 0xFF = 0xFF");
}

#[test]
fn test_or_al_imm8_zero_result() {
    let code = [0x0C, 0x00, 0xf4]; // OR AL, 0
    let mut regs = Registers::default();
    regs.rax = 0x00;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0, "AL: 0 OR 0 = 0");
    assert!(zf_set(regs.rflags), "ZF should be set");
}

#[test]
fn test_or_al_imm8_sign_flag() {
    let code = [0x0C, 0x80, 0xf4]; // OR AL, 0x80
    let mut regs = Registers::default();
    regs.rax = 0x00;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x80, "AL: 0x00 OR 0x80 = 0x80");
    assert!(sf_set(regs.rflags), "SF should be set (high bit = 1)");
}

#[test]
fn test_or_al_imm8_parity_even() {
    let code = [0x0C, 0x03, 0xf4]; // OR AL, 0x03
    let mut regs = Registers::default();
    regs.rax = 0x00;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Result = 0x03 (00000011), two 1-bits = even parity
    assert_eq!(regs.rax & 0xFF, 0x03);
    assert!(pf_set(regs.rflags), "PF should be set (even parity)");
}

#[test]
fn test_or_al_imm8_parity_odd() {
    let code = [0x0C, 0x07, 0xf4]; // OR AL, 0x07
    let mut regs = Registers::default();
    regs.rax = 0x00;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Result = 0x07 (00000111), three 1-bits = odd parity
    assert_eq!(regs.rax & 0xFF, 0x07);
    assert!(!pf_set(regs.rflags), "PF should be clear (odd parity)");
}

// ============================================================================
// OR AX, imm16
// ============================================================================

#[test]
fn test_or_ax_imm16_basic() {
    let code = [0x66, 0x0D, 0x0F, 0x00, 0xf4]; // OR AX, 0x000F
    let mut regs = Registers::default();
    regs.rax = 0x1230;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFF, 0x123F, "AX: 0x1230 OR 0x000F = 0x123F");
}

#[test]
fn test_or_ax_imm16_high_byte() {
    let code = [0x66, 0x0D, 0x00, 0xFF, 0xf4]; // OR AX, 0xFF00
    let mut regs = Registers::default();
    regs.rax = 0x00CD;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFF, 0xFFCD, "AX: set high byte");
}

#[test]
fn test_or_ax_imm16_zero() {
    let code = [0x66, 0x0D, 0x00, 0x00, 0xf4]; // OR AX, 0
    let mut regs = Registers::default();
    regs.rax = 0x1234;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFF, 0x1234, "AX: OR with 0 is identity");
}

// ============================================================================
// OR EAX, imm32
// ============================================================================

#[test]
fn test_or_eax_imm32_basic() {
    let code = [0x0D, 0xFF, 0x00, 0x00, 0x00, 0xf4]; // OR EAX, 0x000000FF
    let mut regs = Registers::default();
    regs.rax = 0x12345600;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x123456FF, "EAX: set low byte");
}

#[test]
fn test_or_eax_imm32_pattern() {
    let code = [0x0D, 0x00, 0xFF, 0x00, 0x00, 0xf4]; // OR EAX, 0x0000FF00
    let mut regs = Registers::default();
    regs.rax = 0x12340078;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x1234FF78, "EAX: set middle byte");
}

#[test]
fn test_or_eax_imm32_high_bit() {
    let code = [0x0D, 0x00, 0x00, 0x00, 0x80, 0xf4]; // OR EAX, 0x80000000
    let mut regs = Registers::default();
    regs.rax = 0x00000000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x80000000, "EAX: set only high bit");
    assert!(sf_set(regs.rflags), "SF should be set");
}

// ============================================================================
// OR RAX, imm32 (sign-extended)
// ============================================================================

#[test]
fn test_or_rax_imm32_basic() {
    let code = [0x48, 0x0D, 0xFF, 0xFF, 0x00, 0x00, 0xf4]; // OR RAX, 0x0000FFFF
    let mut regs = Registers::default();
    regs.rax = 0x1234567800000000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x123456780000FFFF, "RAX: set low word");
}

#[test]
fn test_or_rax_imm32_negative() {
    let code = [0x48, 0x0D, 0xFF, 0xFF, 0xFF, 0xFF, 0xf4]; // OR RAX, 0xFFFFFFFF
    let mut regs = Registers::default();
    regs.rax = 0x0000000000000000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Sign-extended to 0xFFFFFFFFFFFFFFFF
    assert_eq!(regs.rax, 0xFFFFFFFFFFFFFFFF, "RAX: all bits set");
}

// ============================================================================
// OR r/m8, imm8
// ============================================================================

#[test]
fn test_or_rm8_imm8_bl() {
    let code = [0x80, 0xcb, 0x0F, 0xf4]; // OR BL, 0x0F
    let mut regs = Registers::default();
    regs.rbx = 0xA0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx & 0xFF, 0xAF, "BL: 0xA0 OR 0x0F = 0xAF");
}

#[test]
fn test_or_rm8_imm8_cl() {
    let code = [0x80, 0xc9, 0xAA, 0xf4]; // OR CL, 0xAA
    let mut regs = Registers::default();
    regs.rcx = 0x55;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx & 0xFF, 0xFF, "CL: 0x55 OR 0xAA = 0xFF");
}

#[test]
fn test_or_rm8_imm8_dh() {
    let code = [0x80, 0xce, 0x55, 0xf4]; // OR DH, 0x55
    let mut regs = Registers::default();
    regs.rdx = 0xAA00;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!((regs.rdx >> 8) & 0xFF, 0xFF, "DH: 0xAA OR 0x55 = 0xFF");
}

// ============================================================================
// OR r/m16, imm16
// ============================================================================

#[test]
fn test_or_rm16_imm16_bx() {
    let code = [0x66, 0x81, 0xcb, 0x0F, 0x00, 0xf4]; // OR BX, 0x000F
    let mut regs = Registers::default();
    regs.rbx = 0xFF00;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx & 0xFFFF, 0xFF0F, "BX: 0xFF00 OR 0x000F = 0xFF0F");
}

#[test]
fn test_or_rm16_imm16_si() {
    let code = [0x66, 0x81, 0xce, 0x00, 0xFF, 0xf4]; // OR SI, 0xFF00
    let mut regs = Registers::default();
    regs.rsi = 0x0034;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rsi & 0xFFFF, 0xFF34, "SI: set high byte");
}

// ============================================================================
// OR r/m32, imm32
// ============================================================================

#[test]
fn test_or_rm32_imm32_ebx() {
    let code = [0x81, 0xcb, 0x00, 0xFF, 0x00, 0x00, 0xf4]; // OR EBX, 0x0000FF00
    let mut regs = Registers::default();
    regs.rbx = 0x12340078;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx, 0x1234FF78, "EBX: set middle byte");
}

#[test]
fn test_or_rm32_imm32_esi() {
    let code = [0x81, 0xce, 0xFF, 0x00, 0x00, 0x00, 0xf4]; // OR ESI, 0x000000FF
    let mut regs = Registers::default();
    regs.rsi = 0xABCDEF00;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rsi, 0xABCDEFFF, "ESI: set low byte");
}

// ============================================================================
// OR r/m64, imm32 (sign-extended)
// ============================================================================

#[test]
fn test_or_rm64_imm32_rbx() {
    let code = [0x48, 0x81, 0xcb, 0xFF, 0xFF, 0xFF, 0x00, 0xf4]; // OR RBX, 0x00FFFFFF
    let mut regs = Registers::default();
    regs.rbx = 0x1234567800000000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx, 0x1234567800FFFFFF, "RBX: set low 3 bytes");
}

#[test]
fn test_or_rm64_imm32_r8() {
    let code = [0x49, 0x81, 0xc8, 0xFF, 0x00, 0x00, 0x00, 0xf4]; // OR R8, 0x000000FF
    let mut regs = Registers::default();
    regs.r8 = 0x0000000000000000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.r8, 0x00000000000000FF, "R8: set low byte");
}

// ============================================================================
// OR r/m, imm8 (sign-extended)
// ============================================================================

#[test]
fn test_or_rm16_imm8_sign_ext() {
    let code = [0x66, 0x83, 0xcb, 0xFF, 0xf4]; // OR BX, 0xFF (sign-extended to 0xFFFF)
    let mut regs = Registers::default();
    regs.rbx = 0x0000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx & 0xFFFF, 0xFFFF, "BX: OR with 0xFFFF");
}

#[test]
fn test_or_rm32_imm8_sign_ext() {
    let code = [0x83, 0xcb, 0x0F, 0xf4]; // OR EBX, 0x0F
    let mut regs = Registers::default();
    regs.rbx = 0x00000000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx, 0x0000000F, "EBX: OR with sign-extended imm8");
}

#[test]
fn test_or_rm64_imm8_sign_ext() {
    let code = [0x48, 0x83, 0xc8, 0x0F, 0xf4]; // OR RAX, 0x0F
    let mut regs = Registers::default();
    regs.rax = 0x1234567800000000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x123456780000000F, "RAX: set low nibble");
}

// ============================================================================
// OR r/m, r (destination is r/m)
// ============================================================================

#[test]
fn test_or_rm8_r8_al_bl() {
    let code = [0x08, 0xd8, 0xf4]; // OR AL, BL
    let mut regs = Registers::default();
    regs.rax = 0xA0;
    regs.rbx = 0x0F;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0xAF, "AL: 0xA0 OR 0x0F = 0xAF");
}

#[test]
fn test_or_rm16_r16_ax_bx() {
    let code = [0x66, 0x09, 0xd8, 0xf4]; // OR AX, BX
    let mut regs = Registers::default();
    regs.rax = 0xFF00;
    regs.rbx = 0x00FF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFF, 0xFFFF, "AX: 0xFF00 OR 0x00FF = 0xFFFF");
}

#[test]
fn test_or_rm32_r32_eax_ebx() {
    let code = [0x09, 0xd8, 0xf4]; // OR EAX, EBX
    let mut regs = Registers::default();
    regs.rax = 0x12340000;
    regs.rbx = 0x00005678;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x12345678, "EAX: combine with EBX");
}

#[test]
fn test_or_rm64_r64_rax_rbx() {
    let code = [0x48, 0x09, 0xd8, 0xf4]; // OR RAX, RBX
    let mut regs = Registers::default();
    regs.rax = 0xFFFFFFFF00000000;
    regs.rbx = 0x00000000FFFFFFFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0xFFFFFFFFFFFFFFFF, "RAX: all bits set");
}

// ============================================================================
// OR r, r/m (destination is register)
// ============================================================================

#[test]
fn test_or_r8_rm8_al_bl() {
    let code = [0x0A, 0xc3, 0xf4]; // OR AL, BL
    let mut regs = Registers::default();
    regs.rax = 0xAA;
    regs.rbx = 0x55;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0xFF, "AL: 0xAA OR 0x55 = 0xFF");
}

#[test]
fn test_or_r16_rm16_ax_bx() {
    let code = [0x66, 0x0B, 0xc3, 0xf4]; // OR AX, BX
    let mut regs = Registers::default();
    regs.rax = 0x1234;
    regs.rbx = 0x0F0F;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFF, 0x1F3F, "AX: 0x1234 OR 0x0F0F = 0x1F3F");
}

#[test]
fn test_or_r32_rm32_eax_ebx() {
    let code = [0x0B, 0xc3, 0xf4]; // OR EAX, EBX
    let mut regs = Registers::default();
    regs.rax = 0xAAAAAAAA;
    regs.rbx = 0x55555555;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0xFFFFFFFF, "EAX: alternating bits OR = all ones");
}

#[test]
fn test_or_r64_rm64_rax_rbx() {
    let code = [0x48, 0x0B, 0xc3, 0xf4]; // OR RAX, RBX
    let mut regs = Registers::default();
    regs.rax = 0xFF00FF00FF00FF00;
    regs.rbx = 0x00FF00FF00FF00FF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax, 0xFFFFFFFFFFFFFFFF,
        "RAX: alternating bytes OR = all ones"
    );
}

// ============================================================================
// OR with various register combinations
// ============================================================================

#[test]
fn test_or_cl_dl() {
    let code = [0x08, 0xd1, 0xf4]; // OR CL, DL
    let mut regs = Registers::default();
    regs.rcx = 0xF0;
    regs.rdx = 0x0F;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx & 0xFF, 0xFF, "CL: 0xF0 OR 0x0F = 0xFF");
}

#[test]
fn test_or_ecx_edx() {
    let code = [0x09, 0xd1, 0xf4]; // OR ECX, EDX
    let mut regs = Registers::default();
    regs.rcx = 0xF0F0F0F0;
    regs.rdx = 0x0F0F0F0F;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rcx, 0xFFFFFFFF,
        "ECX: complementary patterns OR = all ones"
    );
}

#[test]
fn test_or_rsi_rdi() {
    let code = [0x48, 0x09, 0xfe, 0xf4]; // OR RSI, RDI
    let mut regs = Registers::default();
    regs.rsi = 0xAAAAAAAAAAAAAAAA;
    regs.rdi = 0x5555555555555555;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rsi, 0xFFFFFFFFFFFFFFFF, "RSI: OR all bits set");
}

// ============================================================================
// Extended registers (R8-R15)
// ============================================================================

#[test]
fn test_or_r8b_imm8() {
    let code = [0x41, 0x80, 0xc8, 0x0F, 0xf4]; // OR R8B, 0x0F
    let mut regs = Registers::default();
    regs.r8 = 0xA0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.r8 & 0xFF, 0xAF, "R8B: 0xA0 OR 0x0F = 0xAF");
}

#[test]
fn test_or_r9w_imm16() {
    let code = [0x66, 0x41, 0x81, 0xc9, 0x0F, 0x00, 0xf4]; // OR R9W, 0x000F
    let mut regs = Registers::default();
    regs.r9 = 0xFF00;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.r9 & 0xFFFF, 0xFF0F, "R9W: set low nibble");
}

#[test]
fn test_or_r10d_imm32() {
    let code = [0x41, 0x81, 0xca, 0xFF, 0x00, 0x00, 0x00, 0xf4]; // OR R10D, 0x000000FF
    let mut regs = Registers::default();
    regs.r10 = 0x12345600;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.r10, 0x123456FF, "R10D: set low byte");
}

#[test]
fn test_or_r11_imm32() {
    let code = [0x49, 0x81, 0xcb, 0xFF, 0xFF, 0x00, 0x00, 0xf4]; // OR R11, 0x0000FFFF
    let mut regs = Registers::default();
    regs.r11 = 0x1234567800000000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.r11, 0x123456780000FFFF, "R11: set low word");
}

#[test]
fn test_or_r12d_r13d() {
    let code = [0x45, 0x09, 0xec, 0xf4]; // OR R12D, R13D
    let mut regs = Registers::default();
    regs.r12 = 0xFFFF0000;
    regs.r13 = 0x0000FFFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.r12, 0xFFFFFFFF, "R12D: OR with R13D");
}

#[test]
fn test_or_r14_r15() {
    let code = [0x4d, 0x09, 0xfe, 0xf4]; // OR R14, R15
    let mut regs = Registers::default();
    regs.r14 = 0xFFFFFFFF00000000;
    regs.r15 = 0x00000000FFFFFFFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.r14, 0xFFFFFFFFFFFFFFFF, "R14: all bits set");
}

// ============================================================================
// Memory operands
// ============================================================================

#[test]
fn test_or_byte_ptr_imm8() {
    let code = [
        0x80, 0x0d, 0xf9, 0x0f, 0x00, 0x00, 0x0F, // OR BYTE PTR [rip+0x0FF9], 0x0F
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u8(&mem, 0xA0);

    let _ = run_until_hlt(&mut vcpu).unwrap();
    let result = read_mem_u8(&mem);

    assert_eq!(result, 0xAF, "Memory: 0xA0 OR 0x0F = 0xAF");
}

#[test]
fn test_or_word_ptr_imm16() {
    let code = [
        0x66, 0x81, 0x0d, 0xf7, 0x0f, 0x00, 0x00, 0x0F,
        0x00, // OR WORD PTR [rip+0x0FF7], 0x000F
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u16(&mem, 0xFF00);

    let _ = run_until_hlt(&mut vcpu).unwrap();
    let result = read_mem_u16(&mem);

    assert_eq!(result, 0xFF0F, "Memory: word OR");
}

#[test]
fn test_or_dword_ptr_imm32() {
    let code = [
        0x81, 0x0d, 0xf6, 0x0f, 0x00, 0x00, 0xFF, 0x00, 0x00,
        0x00, // OR DWORD PTR [rip+0x0FF6], 0x000000FF
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u32(&mem, 0x12345600);

    let _ = run_until_hlt(&mut vcpu).unwrap();
    let result = read_mem_u32(&mem);

    assert_eq!(result, 0x123456FF, "Memory: dword set low byte");
}

#[test]
fn test_or_qword_ptr_imm32() {
    let code = [
        0x48, 0x81, 0x0d, 0xf5, 0x0f, 0x00, 0x00, 0xFF, 0xFF, 0x00,
        0x00, // OR QWORD PTR [rip+0x0FF5], 0x0000FFFF
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u64(&mem, 0x1234567800000000);

    let _ = run_until_hlt(&mut vcpu).unwrap();
    let result = read_mem_u64(&mem);

    assert_eq!(result, 0x123456780000FFFF, "Memory: qword set low word");
}

// ============================================================================
// Flag behavior tests
// ============================================================================

#[test]
fn test_or_clears_of_cf() {
    let code = [0x0C, 0xFF, 0xf4]; // OR AL, 0xFF
    let mut regs = Registers::default();
    regs.rax = 0x00;
    regs.rflags = 0x2 | flags::bits::OF | flags::bits::CF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(!of_set(regs.rflags), "OF cleared by OR");
    assert!(!cf_set(regs.rflags), "CF cleared by OR");
}

// ============================================================================
// Practical use cases
// ============================================================================

#[test]
fn test_or_set_specific_bits() {
    let code = [0x0C, 0x10, 0xf4]; // OR AL, 0x10
    let mut regs = Registers::default();
    regs.rax = 0x0F;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x1F, "Set bit 4");
}

#[test]
fn test_or_combine_flags() {
    let code = [0x0C, 0x04, 0xf4]; // OR AL, 0x04
    let mut regs = Registers::default();
    regs.rax = 0x01;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x05, "Combine flags: 0x01 | 0x04 = 0x05");
}

#[test]
fn test_or_idempotent() {
    // OR is idempotent: a OR a = a
    let code = [0x08, 0xc0, 0xf4]; // OR AL, AL
    let mut regs = Registers::default();
    regs.rax = 0x42;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x42, "OR is idempotent");
}

// ============================================================================
// Strengthened OR tests (appended): exact result and the full flag contract —
// OF and CF always cleared; SF/ZF/PF set per the result.
// ============================================================================

#[test]
fn test_strict_or_r64_result_and_flags() {
    // OR RAX, RBX: 0x00FF_0000_0000_0000 | 0x0000_0000_0000_00F0 = 0x00FF_0000_0000_00F0.
    // Result nonzero, bit 63 clear -> SF=0, ZF=0. Low byte 0xF0 has 4 set bits -> PF=1.
    let code = [0x48, 0x09, 0xd8, 0xf4]; // OR RAX, RBX
    let mut regs = Registers::default();
    regs.rax = 0x00FF_0000_0000_0000;
    regs.rbx = 0x0000_0000_0000_00F0;
    regs.rflags = 0x2 | 0x1 | 0x800; // seed CF and OF to confirm they get cleared
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x00FF_0000_0000_00F0, "OR result");
    assert!(!cf_set(regs.rflags), "CF cleared by OR");
    assert!(!of_set(regs.rflags), "OF cleared by OR");
    assert!(!zf_set(regs.rflags), "ZF clear (nonzero)");
    assert!(!sf_set(regs.rflags), "SF clear (bit 63 = 0)");
    assert!(
        pf_set(regs.rflags),
        "PF set (low byte 0xF0 has even parity)"
    );
}

#[test]
fn test_strict_or_zero_sets_zf_clears_sf() {
    // OR AL, 0 with AL=0 -> result 0 -> ZF=1, SF=0, PF=1 (0 has even parity).
    let code = [0x0c, 0x00, 0xf4]; // OR AL, 0
    let mut regs = Registers::default();
    regs.rax = 0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFF, 0);
    assert!(zf_set(regs.rflags), "ZF set");
    assert!(!sf_set(regs.rflags), "SF clear");
    assert!(pf_set(regs.rflags), "PF set for 0");
    assert!(
        !cf_set(regs.rflags) && !of_set(regs.rflags),
        "CF/OF cleared"
    );
}

#[test]
fn test_strict_or_sets_sf() {
    // OR AL, 0x80 with AL=0 -> 0x80 -> SF=1, ZF=0, PF=0 (one bit set, odd parity).
    let code = [0x0c, 0x80, 0xf4]; // OR AL, 0x80
    let mut regs = Registers::default();
    regs.rax = 0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFF, 0x80);
    assert!(sf_set(regs.rflags), "SF set");
    assert!(!zf_set(regs.rflags), "ZF clear");
    assert!(!pf_set(regs.rflags), "PF clear (0x80 odd parity)");
}

#[test]
fn test_strict_or_r32_zero_extends_upper() {
    // OR EAX, EBX clears upper 32 bits of RAX.
    let code = [0x09, 0xd8, 0xf4]; // OR EAX, EBX
    let mut regs = Registers::default();
    regs.rax = 0xFFFF_FFFF_0000_000F;
    regs.rbx = 0x0000_0000_0000_00F0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x0000_0000_0000_00FF, "32-bit OR zero-extends");
}
