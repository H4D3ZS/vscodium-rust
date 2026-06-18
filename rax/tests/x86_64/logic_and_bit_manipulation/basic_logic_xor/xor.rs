use crate::common::*;
use rax::backend::emulator::x86_64::flags;
use rax::cpu::Registers;

// XOR — Logical Exclusive OR
//
// Opcodes: Similar structure to AND/OR but opcode /6
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
// Common idioms: XOR reg, reg (zero register), XOR reg, -1 (invert all bits).

// ============================================================================
// XOR with immediate
// ============================================================================

#[test]
fn test_xor_al_imm8_basic() {
    let code = [
        0x34, 0x0F, // XOR AL, 0x0F
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0xAA; // 10101010
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // 0xAA ^ 0x0F = 10101010 ^ 00001111 = 10100101 = 0xA5
    assert_eq!(regs.rax & 0xFF, 0xA5, "AL: 0xAA XOR 0x0F = 0xA5");
    assert!(!zf_set(regs.rflags), "ZF should be clear");
    assert!(!cf_set(regs.rflags), "CF should be clear");
    assert!(!of_set(regs.rflags), "OF should be clear");
    assert!(sf_set(regs.rflags), "SF should be set (high bit = 1)");
}

#[test]
fn test_xor_al_imm8_identity() {
    // XOR with 0 is identity operation
    let code = [0x34, 0x00, 0xf4]; // XOR AL, 0
    let mut regs = Registers::default();
    regs.rax = 0x42;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x42, "AL: 0x42 XOR 0 = 0x42 (identity)");
}

#[test]
fn test_xor_al_imm8_invert() {
    // XOR with 0xFF inverts all bits
    let code = [0x34, 0xFF, 0xf4]; // XOR AL, 0xFF
    let mut regs = Registers::default();
    regs.rax = 0xAA; // 10101010
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x55, "AL: 0xAA XOR 0xFF = 0x55 (inverted)");
}

#[test]
fn test_xor_al_imm8_same_value() {
    // XOR with same value = 0
    let code = [0x34, 0x42, 0xf4]; // XOR AL, 0x42
    let mut regs = Registers::default();
    regs.rax = 0x42;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0, "AL: 0x42 XOR 0x42 = 0");
    assert!(zf_set(regs.rflags), "ZF should be set (zero result)");
}

#[test]
fn test_xor_eax_imm32_basic() {
    let code = [
        0x35, 0xFF, 0x00, 0xFF, 0x00, // XOR EAX, 0x00FF00FF
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0xFFFFFFFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0xFF00FF00, "EAX: toggle specific bytes");
}

#[test]
fn test_xor_rax_imm32_basic() {
    let code = [
        0x48, 0x35, 0xFF, 0xFF, 0xFF, 0xFF, // XOR RAX, 0xFFFFFFFF (sign-extended)
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x0000000012345678;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // 0xFFFFFFFF sign-extended to 64-bit is 0xFFFFFFFFFFFFFFFF
    assert_eq!(regs.rax, 0xFFFFFFFFEDCBA987, "RAX: invert all bits");
}

// ============================================================================
// XOR r/m with immediate
// ============================================================================

#[test]
fn test_xor_rm8_imm8_basic() {
    let code = [
        0x80, 0xf3, 0xFF, // XOR BL, 0xFF
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xAA;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx & 0xFF, 0x55, "BL: 0xAA XOR 0xFF = 0x55");
}

#[test]
fn test_xor_rm32_imm32_basic() {
    let code = [
        0x81, 0xf3, 0xFF, 0xFF, 0x00, 0x00, // XOR EBX, 0x0000FFFF
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x12345678;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx, 0x1234A987, "EBX: toggle low 16 bits");
}

#[test]
fn test_xor_rm64_imm8_sign_extended() {
    let code = [
        0x48, 0x83, 0xf3, 0xFF, // XOR RBX, 0xFF (sign-extended to -1)
        0xf4,
    ];
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
fn test_xor_rm8_r8_basic() {
    let code = [
        0x30, 0xd8, // XOR AL, BL
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0xAA; // 10101010
    regs.rbx = 0x55; // 01010101
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFF,
        0xFF,
        "AL: 0xAA XOR 0x55 = 0xFF (all different)"
    );
}

#[test]
fn test_xor_rm8_r8_same_value() {
    let code = [
        0x30, 0xd8, // XOR AL, BL
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x42;
    regs.rbx = 0x42; // Same value
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0, "AL: 0x42 XOR 0x42 = 0");
    assert!(zf_set(regs.rflags), "ZF set");
}

#[test]
fn test_xor_rm32_r32_basic() {
    let code = [
        0x31, 0xd8, // XOR EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0xFF00FF00;
    regs.rbx = 0x00FF00FF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0xFFFFFFFF, "EAX: toggle pattern");
}

#[test]
fn test_xor_rm64_r64_basic() {
    let code = [
        0x48, 0x31, 0xd8, // XOR RAX, RBX
        0xf4,
    ];
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
fn test_xor_r8_rm8_basic() {
    let code = [
        0x32, 0xc3, // XOR AL, BL
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x0F;
    regs.rbx = 0xF0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0xFF, "AL: 0x0F XOR 0xF0 = 0xFF");
}

// ============================================================================
// XOR reg, reg (common idiom to zero register)
// ============================================================================

#[test]
fn test_xor_eax_eax_zero() {
    // XOR EAX, EAX is common idiom to zero EAX
    let code = [
        0x31, 0xc0, // XOR EAX, EAX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x12345678;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0, "EAX: XOR EAX, EAX = 0 (common zero idiom)");
    assert!(zf_set(regs.rflags), "ZF set");
    assert!(!sf_set(regs.rflags), "SF clear");
    assert!(!cf_set(regs.rflags), "CF clear");
    assert!(!of_set(regs.rflags), "OF clear");
}

#[test]
fn test_xor_rax_rax_zero() {
    let code = [
        0x48, 0x31, 0xc0, // XOR RAX, RAX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0xFFFFFFFFFFFFFFFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0, "RAX: XOR RAX, RAX = 0");
    assert!(zf_set(regs.rflags), "ZF set");
}

#[test]
fn test_xor_r8b_r8b_zero() {
    let code = [
        0x45, 0x30, 0xc0, // XOR R8B, R8B
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.r8 = 0xFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.r8 & 0xFF, 0, "R8B: XOR R8B, R8B = 0");
}

// ============================================================================
// Bit toggling use cases
// ============================================================================

#[test]
fn test_xor_toggle_specific_bit() {
    // Toggle bit 4 using XOR with 0x10
    let code = [0x34, 0x10, 0xf4]; // XOR AL, 0x10
    let mut regs = Registers::default();
    regs.rax = 0x0F; // bit 4 is clear
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x1F, "Toggle bit 4 on: 0x0F ^ 0x10 = 0x1F");

    // Toggle again to clear it
    let code = [0x34, 0x10, 0xf4]; // XOR AL, 0x10
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
fn test_xor_toggle_multiple_bits() {
    let code = [0x34, 0x55, 0xf4]; // XOR AL, 0x55 (01010101)
    let mut regs = Registers::default();
    regs.rax = 0xFF; // 11111111
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // 11111111 ^ 01010101 = 10101010 = 0xAA
    assert_eq!(regs.rax & 0xFF, 0xAA, "Toggle alternating bits");
}

#[test]
fn test_xor_swap_nibbles() {
    // Swap high and low nibbles using series of XORs
    // Actually, this just toggles specific bits, but demonstrates XOR usage
    let code = [
        0x34, 0xFF, // XOR AL, 0xFF (invert all)
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x3C; // 00111100
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0xC3, "Inverted: 11000011");
}

// ============================================================================
// OF and CF always cleared
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
// Parity flag tests
// ============================================================================

#[test]
fn test_xor_parity_even() {
    let code = [0x34, 0x02, 0xf4]; // XOR AL, 0x02
    let mut regs = Registers::default();
    regs.rax = 0x01; // 00000001 ^ 00000010 = 00000011
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Result = 0x03 (two 1-bits = even parity)
    assert_eq!(regs.rax & 0xFF, 0x03);
    assert!(pf_set(regs.rflags), "PF set (even parity)");
}

#[test]
fn test_xor_parity_odd() {
    let code = [0x34, 0x04, 0xf4]; // XOR AL, 0x04
    let mut regs = Registers::default();
    regs.rax = 0x03; // 00000011 ^ 00000100 = 00000111
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Result = 0x07 (three 1-bits = odd parity)
    assert_eq!(regs.rax & 0xFF, 0x07);
    assert!(!pf_set(regs.rflags), "PF clear (odd parity)");
}

// ============================================================================
// Different registers
// ============================================================================

#[test]
fn test_xor_different_registers() {
    // XOR CL, DL
    let code = [0x30, 0xd1, 0xf4];
    let mut regs = Registers::default();
    regs.rcx = 0x0F;
    regs.rdx = 0xF0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx & 0xFF, 0xFF, "CL: 0x0F XOR 0xF0 = 0xFF");
}

// ============================================================================
// Extended registers (R8-R15)
// ============================================================================

#[test]
fn test_xor_r8b_basic() {
    let code = [
        0x41, 0x80, 0xf0, 0xFF, // XOR R8B, 0xFF
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.r8 = 0xAA;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.r8 & 0xFF, 0x55, "R8B: 0xAA XOR 0xFF = 0x55");
}

#[test]
fn test_xor_r10d_basic() {
    let code = [
        0x41, 0x81, 0xf2, 0xFF, 0xFF, 0x00, 0x00, // XOR R10D, 0x0000FFFF
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.r10 = 0x12345678;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.r10, 0x1234A987, "R10D: toggle low 16 bits");
}

#[test]
fn test_xor_r11_r12_zero() {
    let code = [
        0x4d, 0x31, 0xe3, // XOR R11, R12
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.r11 = 0x123456789ABCDEF0;
    regs.r12 = 0x123456789ABCDEF0; // Same value
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.r11, 0, "R11: XOR with same value = 0");
    assert!(zf_set(regs.rflags), "ZF set");
}

// ============================================================================
// Memory operands
// ============================================================================

#[test]
fn test_xor_byte_ptr_mem() {
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
fn test_xor_dword_ptr_mem() {
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

// ============================================================================
// Practical use cases
// ============================================================================

#[test]
fn test_xor_encryption_basic() {
    // Simple XOR "encryption" (XOR with key, XOR again to decrypt)
    let key = 0x5A;

    // Encrypt
    let code = [0x34, key, 0xf4]; // XOR AL, key
    let mut regs = Registers::default();
    let plaintext = 0x42;
    regs.rax = plaintext;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    let ciphertext = regs.rax & 0xFF;

    // Decrypt
    let code = [0x34, key, 0xf4]; // XOR AL, key (again)
    let mut regs = Registers::default();
    regs.rax = ciphertext;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, plaintext, "XOR encryption/decryption");
}

#[test]
fn test_xor_commutative() {
    // XOR is commutative: a XOR b = b XOR a
    let a: u8 = 0x12;
    let b: u8 = 0x34;

    // a XOR b
    let code = [0x34, b, 0xf4];
    let mut regs = Registers::default();
    regs.rax = u64::from(a);
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs1 = run_until_hlt(&mut vcpu).unwrap();

    // b XOR a
    let code = [0x34, a, 0xf4];
    let mut regs = Registers::default();
    regs.rax = u64::from(b);
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs2 = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs1.rax & 0xFF, regs2.rax & 0xFF, "XOR is commutative");
}

#[test]
fn test_xor_associative() {
    // XOR is associative: (a XOR b) XOR c = a XOR (b XOR c)
    let a: u8 = 0x12;
    let b: u8 = 0x34;
    let c: u8 = 0x56;

    // (a XOR b) XOR c
    let code = [0x34, b, 0x34, c, 0xf4];
    let mut regs = Registers::default();
    regs.rax = u64::from(a);
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs1 = run_until_hlt(&mut vcpu).unwrap();

    // a XOR (b XOR c)
    let code = [0x34, (b ^ c), 0xf4];
    let mut regs = Registers::default();
    regs.rax = u64::from(a);
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs2 = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs1.rax & 0xFF, regs2.rax & 0xFF, "XOR is associative");
}
