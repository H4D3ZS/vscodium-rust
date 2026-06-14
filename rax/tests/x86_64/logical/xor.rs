use crate::common::*;
use crate::common::{run_until_hlt, setup_vm};
use rax::backend::emulator::x86_64::flags;
use rax::cpu::Registers;

// XOR — Logical Exclusive OR
//
// Opcodes:
// - 34 ib           XOR AL, imm8
// - 35 iw/id        XOR AX/EAX/RAX, imm16/32
// - 80 /6 ib        XOR r/m8, imm8
// - 81 /6 iw/id     XOR r/m16/32/64, imm16/32
// - 83 /6 ib        XOR r/m16/32/64, imm8 (sign-extended)
// - 30 /r           XOR r/m8, r8
// - 31 /r           XOR r/m16/32/64, r16/32/64
// - 32 /r           XOR r8, r/m8
// - 33 /r           XOR r16/32/64, r/m16/32/64
//
// Operation: DEST := DEST XOR SRC
//
// Flags: OF and CF are CLEARED.
//        SF, ZF, PF are set according to result.
//        AF is undefined.
//
// CRITICAL: XOR is exclusive OR. Each bit is 1 if bits DIFFER, 0 if same.
// Common idiom: XOR reg, reg (zero register)

// ============================================================================
// XOR AL, imm8
// ============================================================================

#[test]
fn test_xor_al_imm8_basic() {
    let code = [0x34, 0x0F, 0xf4]; // XOR AL, 0x0F
    let mut regs = Registers::default();
    regs.rax = 0xAA; // 10101010
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // 0xAA ^ 0x0F = 10101010 ^ 00001111 = 10100101 = 0xA5
    assert_eq!(regs.rax & 0xFF, 0xA5, "AL: 0xAA XOR 0x0F = 0xA5");
    assert!(!zf_set(regs.rflags), "ZF should be clear");
    assert!(!cf_set(regs.rflags), "CF should be clear");
    assert!(!of_set(regs.rflags), "OF should be clear");
}

#[test]
fn test_xor_al_imm8_identity() {
    let code = [0x34, 0x00, 0xf4]; // XOR AL, 0
    let mut regs = Registers::default();
    regs.rax = 0x42;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x42, "AL: 0x42 XOR 0 = 0x42 (identity)");
}

#[test]
fn test_xor_al_imm8_invert() {
    let code = [0x34, 0xFF, 0xf4]; // XOR AL, 0xFF
    let mut regs = Registers::default();
    regs.rax = 0xAA;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x55, "AL: 0xAA XOR 0xFF = 0x55 (inverted)");
}

#[test]
fn test_xor_al_imm8_same_value() {
    let code = [0x34, 0x42, 0xf4]; // XOR AL, 0x42
    let mut regs = Registers::default();
    regs.rax = 0x42;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0, "AL: 0x42 XOR 0x42 = 0");
    assert!(zf_set(regs.rflags), "ZF should be set");
}

#[test]
fn test_xor_al_imm8_parity_even() {
    let code = [0x34, 0x02, 0xf4]; // XOR AL, 0x02
    let mut regs = Registers::default();
    regs.rax = 0x01;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Result = 0x03 (two 1-bits = even parity)
    assert_eq!(regs.rax & 0xFF, 0x03);
    assert!(pf_set(regs.rflags), "PF set (even parity)");
}

#[test]
fn test_xor_al_imm8_parity_odd() {
    let code = [0x34, 0x04, 0xf4]; // XOR AL, 0x04
    let mut regs = Registers::default();
    regs.rax = 0x03;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Result = 0x07 (three 1-bits = odd parity)
    assert_eq!(regs.rax & 0xFF, 0x07);
    assert!(!pf_set(regs.rflags), "PF clear (odd parity)");
}

// ============================================================================
// XOR AX/EAX/RAX, imm
// ============================================================================

#[test]
fn test_xor_ax_imm16_basic() {
    let code = [0x66, 0x35, 0x0F, 0x00, 0xf4]; // XOR AX, 0x000F
    let mut regs = Registers::default();
    regs.rax = 0x1234;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFF, 0x123B, "AX: 0x1234 XOR 0x000F = 0x123B");
}

#[test]
fn test_xor_eax_imm32_basic() {
    let code = [0x35, 0xFF, 0x00, 0xFF, 0x00, 0xf4]; // XOR EAX, 0x00FF00FF
    let mut regs = Registers::default();
    regs.rax = 0xFFFFFFFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0xFF00FF00, "EAX: toggle specific bytes");
}

#[test]
fn test_xor_rax_imm32_basic() {
    let code = [0x48, 0x35, 0xFF, 0xFF, 0xFF, 0xFF, 0xf4]; // XOR RAX, 0xFFFFFFFF
    let mut regs = Registers::default();
    regs.rax = 0x0000000012345678;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Sign-extended to 0xFFFFFFFFFFFFFFFF
    assert_eq!(regs.rax, 0xFFFFFFFFEDCBA987, "RAX: invert all bits");
}

// ============================================================================
// XOR r/m, imm
// ============================================================================

#[test]
fn test_xor_rm8_imm8_bl() {
    let code = [0x80, 0xf3, 0xFF, 0xf4]; // XOR BL, 0xFF
    let mut regs = Registers::default();
    regs.rbx = 0xAA;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx & 0xFF, 0x55, "BL: 0xAA XOR 0xFF = 0x55");
}

#[test]
fn test_xor_rm16_imm16_bx() {
    let code = [0x66, 0x81, 0xf3, 0xFF, 0xFF, 0xf4]; // XOR BX, 0xFFFF
    let mut regs = Registers::default();
    regs.rbx = 0x1234;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx & 0xFFFF, 0xEDCB, "BX: invert all bits");
}

#[test]
fn test_xor_rm32_imm32_ebx() {
    let code = [0x81, 0xf3, 0xFF, 0xFF, 0x00, 0x00, 0xf4]; // XOR EBX, 0x0000FFFF
    let mut regs = Registers::default();
    regs.rbx = 0x12345678;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx, 0x1234A987, "EBX: toggle low 16 bits");
}

#[test]
fn test_xor_rm64_imm32_rbx() {
    let code = [0x48, 0x81, 0xf3, 0xFF, 0xFF, 0xFF, 0xFF, 0xf4]; // XOR RBX, 0xFFFFFFFF
    let mut regs = Registers::default();
    regs.rbx = 0x123456789ABCDEF0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Sign-extended to 0xFFFFFFFFFFFFFFFF
    assert_eq!(regs.rbx, 0xEDCBA9876543210F, "RBX: invert all bits");
}

#[test]
fn test_xor_rm64_imm8_sign_ext() {
    let code = [0x48, 0x83, 0xf3, 0xFF, 0xf4]; // XOR RBX, 0xFF (sign-extended)
    let mut regs = Registers::default();
    regs.rbx = 0x123456789ABCDEF0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx, 0xEDCBA9876543210F, "RBX: invert all bits");
}

// ============================================================================
// XOR r/m, r
// ============================================================================

#[test]
fn test_xor_rm8_r8_al_bl() {
    let code = [0x30, 0xd8, 0xf4]; // XOR AL, BL
    let mut regs = Registers::default();
    regs.rax = 0xAA;
    regs.rbx = 0x55;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0xFF, "AL: 0xAA XOR 0x55 = 0xFF");
}

#[test]
fn test_xor_rm8_r8_same_value() {
    let code = [0x30, 0xd8, 0xf4]; // XOR AL, BL
    let mut regs = Registers::default();
    regs.rax = 0x42;
    regs.rbx = 0x42;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0, "AL: 0x42 XOR 0x42 = 0");
    assert!(zf_set(regs.rflags), "ZF set");
}

#[test]
fn test_xor_rm32_r32_eax_ebx() {
    let code = [0x31, 0xd8, 0xf4]; // XOR EAX, EBX
    let mut regs = Registers::default();
    regs.rax = 0xFF00FF00;
    regs.rbx = 0x00FF00FF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0xFFFFFFFF, "EAX: toggle pattern");
}

#[test]
fn test_xor_rm64_r64_rax_rbx() {
    let code = [0x48, 0x31, 0xd8, 0xf4]; // XOR RAX, RBX
    let mut regs = Registers::default();
    regs.rax = 0xFFFFFFFF00000000;
    regs.rbx = 0x00000000FFFFFFFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0xFFFFFFFFFFFFFFFF, "RAX: all bits different");
}

// ============================================================================
// XOR r, r/m
// ============================================================================

#[test]
fn test_xor_r8_rm8_al_bl() {
    let code = [0x32, 0xc3, 0xf4]; // XOR AL, BL
    let mut regs = Registers::default();
    regs.rax = 0x0F;
    regs.rbx = 0xF0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0xFF, "AL: 0x0F XOR 0xF0 = 0xFF");
}

#[test]
fn test_xor_r16_rm16_ax_bx() {
    let code = [0x66, 0x33, 0xc3, 0xf4]; // XOR AX, BX
    let mut regs = Registers::default();
    regs.rax = 0xAAAA;
    regs.rbx = 0x5555;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFF, 0xFFFF, "AX: alternating bits XOR");
}

#[test]
fn test_xor_r32_rm32_eax_ebx() {
    let code = [0x33, 0xc3, 0xf4]; // XOR EAX, EBX
    let mut regs = Registers::default();
    regs.rax = 0xAAAAAAAA;
    regs.rbx = 0x55555555;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0xFFFFFFFF, "EAX: all bits set");
}

// ============================================================================
// XOR reg, reg (zeroing idiom)
// ============================================================================

#[test]
fn test_xor_eax_eax_zero() {
    let code = [0x31, 0xc0, 0xf4]; // XOR EAX, EAX
    let mut regs = Registers::default();
    regs.rax = 0x12345678;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0, "EAX: XOR EAX, EAX = 0 (zeroing idiom)");
    assert!(zf_set(regs.rflags), "ZF set");
    assert!(!sf_set(regs.rflags), "SF clear");
    assert!(!cf_set(regs.rflags), "CF clear");
    assert!(!of_set(regs.rflags), "OF clear");
}

#[test]
fn test_xor_rax_rax_zero() {
    let code = [0x48, 0x31, 0xc0, 0xf4]; // XOR RAX, RAX
    let mut regs = Registers::default();
    regs.rax = 0xFFFFFFFFFFFFFFFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0, "RAX: XOR RAX, RAX = 0");
    assert!(zf_set(regs.rflags), "ZF set");
}

#[test]
fn test_xor_r8b_r8b_zero() {
    let code = [0x45, 0x30, 0xc0, 0xf4]; // XOR R8B, R8B
    let mut regs = Registers::default();
    regs.r8 = 0xFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.r8 & 0xFF, 0, "R8B: XOR R8B, R8B = 0");
}

// ============================================================================
// Different register combinations
// ============================================================================

#[test]
fn test_xor_cl_dl() {
    let code = [0x30, 0xd1, 0xf4]; // XOR CL, DL
    let mut regs = Registers::default();
    regs.rcx = 0x0F;
    regs.rdx = 0xF0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx & 0xFF, 0xFF, "CL: 0x0F XOR 0xF0 = 0xFF");
}

#[test]
fn test_xor_ecx_edx() {
    let code = [0x31, 0xd1, 0xf4]; // XOR ECX, EDX
    let mut regs = Registers::default();
    regs.rcx = 0xAAAAAAAA;
    regs.rdx = 0x55555555;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 0xFFFFFFFF, "ECX: XOR alternating patterns");
}

#[test]
fn test_xor_rsi_rdi() {
    let code = [0x48, 0x31, 0xfe, 0xf4]; // XOR RSI, RDI
    let mut regs = Registers::default();
    regs.rsi = 0xFF00FF00FF00FF00;
    regs.rdi = 0x00FF00FF00FF00FF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rsi, 0xFFFFFFFFFFFFFFFF, "RSI: all bits set");
}

// ============================================================================
// Extended registers (R8-R15)
// ============================================================================

#[test]
fn test_xor_r8b_imm8() {
    let code = [0x41, 0x80, 0xf0, 0xFF, 0xf4]; // XOR R8B, 0xFF
    let mut regs = Registers::default();
    regs.r8 = 0xAA;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.r8 & 0xFF, 0x55, "R8B: 0xAA XOR 0xFF = 0x55");
}

#[test]
fn test_xor_r9w_imm16() {
    let code = [0x66, 0x41, 0x81, 0xf1, 0xFF, 0xFF, 0xf4]; // XOR R9W, 0xFFFF
    let mut regs = Registers::default();
    regs.r9 = 0x1234;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.r9 & 0xFFFF, 0xEDCB, "R9W: invert all bits");
}

#[test]
fn test_xor_r10d_imm32() {
    let code = [0x41, 0x81, 0xf2, 0xFF, 0xFF, 0x00, 0x00, 0xf4]; // XOR R10D, 0x0000FFFF
    let mut regs = Registers::default();
    regs.r10 = 0x12345678;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.r10, 0x1234A987, "R10D: toggle low 16 bits");
}

#[test]
fn test_xor_r11_r12() {
    let code = [0x4d, 0x31, 0xe3, 0xf4]; // XOR R11, R12
    let mut regs = Registers::default();
    regs.r11 = 0x123456789ABCDEF0;
    regs.r12 = 0x123456789ABCDEF0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.r11, 0, "R11: XOR with same value = 0");
    assert!(zf_set(regs.rflags), "ZF set");
}

#[test]
fn test_xor_r14_r15() {
    let code = [0x4d, 0x31, 0xfe, 0xf4]; // XOR R14, R15
    let mut regs = Registers::default();
    regs.r14 = 0xAAAAAAAAAAAAAAAA;
    regs.r15 = 0x5555555555555555;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.r14, 0xFFFFFFFFFFFFFFFF, "R14: all bits set");
}

// ============================================================================
// Memory operands
// ============================================================================

#[test]
fn test_xor_byte_ptr_imm8() {
    let code = [
        0x80, 0x35, 0xf9, 0x0f, 0x00, 0x00, 0xFF, // XOR BYTE PTR [rip+0x0FF9], 0xFF
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u8(&mem, 0xAA);

    let _ = run_until_hlt(&mut vcpu).unwrap();
    let result = read_mem_u8(&mem);

    assert_eq!(result, 0x55, "Memory: 0xAA XOR 0xFF = 0x55");
}

#[test]
fn test_xor_word_ptr_imm16() {
    let code = [
        0x66, 0x81, 0x35, 0xf7, 0x0f, 0x00, 0x00, 0xFF,
        0xFF, // XOR WORD PTR [rip+0x0FF7], 0xFFFF
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u16(&mem, 0x1234);

    let _ = run_until_hlt(&mut vcpu).unwrap();
    let result = read_mem_u16(&mem);

    assert_eq!(result, 0xEDCB, "Memory: word XOR");
}

#[test]
fn test_xor_dword_ptr_imm32() {
    let code = [
        0x81, 0x35, 0xf6, 0x0f, 0x00, 0x00, 0xFF, 0xFF, 0x00,
        0x00, // XOR DWORD PTR [rip+0x0FF6], 0x0000FFFF
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u32(&mem, 0x12345678);

    let _ = run_until_hlt(&mut vcpu).unwrap();
    let result = read_mem_u32(&mem);

    assert_eq!(result, 0x1234A987, "Memory: toggle low 16 bits");
}

#[test]
fn test_xor_qword_ptr_imm32() {
    let code = [
        0x48, 0x81, 0x35, 0xf5, 0x0f, 0x00, 0x00, 0xFF, 0xFF, 0xFF,
        0xFF, // XOR QWORD PTR [rip+0x0FF5], 0xFFFFFFFF
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u64(&mem, 0x123456789ABCDEF0);

    let _ = run_until_hlt(&mut vcpu).unwrap();
    let result = read_mem_u64(&mem);

    // Sign-extended to 0xFFFFFFFFFFFFFFFF
    assert_eq!(result, 0xEDCBA9876543210F, "Memory: invert all bits");
}

// ============================================================================
// Flag behavior tests
// ============================================================================

#[test]
fn test_xor_clears_of_cf() {
    let code = [0x34, 0xFF, 0xf4]; // XOR AL, 0xFF
    let mut regs = Registers::default();
    regs.rax = 0x00;
    regs.rflags = 0x2 | flags::bits::OF | flags::bits::CF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(!of_set(regs.rflags), "OF cleared by XOR");
    assert!(!cf_set(regs.rflags), "CF cleared by XOR");
}

// ============================================================================
// Practical use cases
// ============================================================================

#[test]
fn test_xor_toggle_bit() {
    let code = [0x34, 0x10, 0xf4]; // XOR AL, 0x10
    let mut regs = Registers::default();
    regs.rax = 0x0F;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x1F, "Toggle bit 4 on: 0x0F ^ 0x10 = 0x1F");

    // Toggle again
    let code = [0x34, 0x10, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 0x1F;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFF,
        0x0F,
        "Toggle bit 4 off: 0x1F ^ 0x10 = 0x0F"
    );
}

#[test]
fn test_xor_encryption_basic() {
    // Simple XOR encryption
    let key = 0x5A;

    // Encrypt
    let code = [0x34, key, 0xf4];
    let mut regs = Registers::default();
    let plaintext = 0x42;
    regs.rax = plaintext;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    let ciphertext = regs.rax & 0xFF;

    // Decrypt
    let code = [0x34, key, 0xf4];
    let mut regs = Registers::default();
    regs.rax = ciphertext;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, plaintext, "XOR encryption/decryption");
}

#[test]
fn test_xor_commutative() {
    // a XOR b = b XOR a
    let a: u8 = 0x12;
    let b: u8 = 0x34;

    let code = [0x34, b, 0xf4];
    let mut regs = Registers::default();
    regs.rax = u64::from(a);
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs1 = run_until_hlt(&mut vcpu).unwrap();

    let code = [0x34, a, 0xf4];
    let mut regs = Registers::default();
    regs.rax = u64::from(b);
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs2 = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs1.rax & 0xFF, regs2.rax & 0xFF, "XOR is commutative");
}

#[test]
fn test_xor_associative() {
    // (a XOR b) XOR c = a XOR (b XOR c)
    let a: u8 = 0x12;
    let b: u8 = 0x34;
    let c: u8 = 0x56;

    let code = [0x34, b, 0x34, c, 0xf4];
    let mut regs = Registers::default();
    regs.rax = u64::from(a);
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs1 = run_until_hlt(&mut vcpu).unwrap();

    let code = [0x34, (b ^ c), 0xf4];
    let mut regs = Registers::default();
    regs.rax = u64::from(a);
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs2 = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs1.rax & 0xFF, regs2.rax & 0xFF, "XOR is associative");
}

// ============================================================================
// Strengthened XOR tests (appended): exact result and full flag contract.
// ============================================================================

#[test]
fn test_strict_xor_self_zeroes_and_flags() {
    // XOR RAX, RAX -> 0; ZF=1, SF=0, PF=1, CF=0, OF=0.
    let code = [0x48, 0x31, 0xc0, 0xf4]; // XOR RAX, RAX
    let mut regs = Registers::default();
    regs.rax = 0xDEAD_BEEF_DEAD_BEEF;
    regs.rflags = 0x2 | 0x1 | 0x800; // seed CF/OF to be cleared
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0, "XOR self gives 0");
    assert!(zf_set(regs.rflags), "ZF set");
    assert!(!sf_set(regs.rflags), "SF clear");
    assert!(pf_set(regs.rflags), "PF set (0 even parity)");
    assert!(
        !cf_set(regs.rflags) && !of_set(regs.rflags),
        "CF/OF cleared"
    );
}

#[test]
fn test_strict_xor_r64_result_and_flags() {
    // XOR RAX, RBX: 0xF0F0..F0F0 ^ 0x0F0F..0F0F = all ones.
    // Result bit 63 set -> SF=1; nonzero -> ZF=0; low byte 0xFF has 8 bits -> PF=1.
    let code = [0x48, 0x31, 0xd8, 0xf4]; // XOR RAX, RBX
    let mut regs = Registers::default();
    regs.rax = 0xF0F0_F0F0_F0F0_F0F0;
    regs.rbx = 0x0F0F_0F0F_0F0F_0F0F;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0xFFFF_FFFF_FFFF_FFFF);
    assert!(sf_set(regs.rflags), "SF set");
    assert!(!zf_set(regs.rflags), "ZF clear");
    assert!(pf_set(regs.rflags), "PF set (0xFF even parity)");
    assert!(
        !cf_set(regs.rflags) && !of_set(regs.rflags),
        "CF/OF cleared"
    );
}

#[test]
fn test_strict_xor_toggle_low_byte_pf_odd() {
    // XOR AL, 0x01 with AL=0x00 -> 0x01: one bit set -> PF=0 (odd), SF=0, ZF=0.
    let code = [0x34, 0x01, 0xf4]; // XOR AL, 0x01
    let mut regs = Registers::default();
    regs.rax = 0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFF, 0x01);
    assert!(!pf_set(regs.rflags), "PF clear (odd parity)");
    assert!(!zf_set(regs.rflags) && !sf_set(regs.rflags));
}

#[test]
fn test_strict_xor_r32_zero_extends() {
    let code = [0x31, 0xd8, 0xf4]; // XOR EAX, EBX
    let mut regs = Registers::default();
    regs.rax = 0xFFFF_FFFF_AAAA_AAAA;
    regs.rbx = 0x0000_0000_5555_5555;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax, 0x0000_0000_FFFF_FFFF,
        "32-bit XOR zero-extends upper"
    );
}
