use crate::common::*;
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
//
// CRITICAL: AND is a bitwise operation. Each bit is 1 if BOTH corresponding
// bits are 1, otherwise 0. Commonly used for bit masking.

// ============================================================================
// AND with immediate: AL, AX, EAX, RAX
// ============================================================================

#[test]
fn test_and_al_imm8_basic() {
    let code = [
        0x24, 0x0F, // AND AL, 0x0F
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0xAB; // AL = 0xAB (10101011)
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // 0xAB & 0x0F = 0b10101011 & 0b00001111 = 0b00001011 = 0x0B
    assert_eq!(regs.rax & 0xFF, 0x0B, "AL: 0xAB AND 0x0F = 0x0B");
    assert!(!zf_set(regs.rflags), "ZF should be clear");
    assert!(!cf_set(regs.rflags), "CF should be clear");
    assert!(!of_set(regs.rflags), "OF should be clear");
}

#[test]
fn test_and_al_imm8_zero_result() {
    // AND with 0 always gives 0
    let code = [0x24, 0x00, 0xf4]; // AND AL, 0
    let mut regs = Registers::default();
    regs.rax = 0xFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0, "AL: 0xFF AND 0 = 0");
    assert!(zf_set(regs.rflags), "ZF should be set (zero result)");
    assert!(!cf_set(regs.rflags), "CF should be clear");
    assert!(!of_set(regs.rflags), "OF should be clear");
}

#[test]
fn test_and_al_imm8_all_bits() {
    // AND with 0xFF preserves value
    let code = [0x24, 0xFF, 0xf4]; // AND AL, 0xFF
    let mut regs = Registers::default();
    regs.rax = 0x42;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x42, "AL: 0x42 AND 0xFF = 0x42");
    assert!(!zf_set(regs.rflags), "ZF should be clear");
}

#[test]
fn test_and_al_imm8_sign_flag() {
    // Result with high bit set (negative in signed interpretation)
    let code = [0x24, 0x80, 0xf4]; // AND AL, 0x80
    let mut regs = Registers::default();
    regs.rax = 0xFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x80, "AL: 0xFF AND 0x80 = 0x80");
    assert!(sf_set(regs.rflags), "SF should be set (high bit = 1)");
}

#[test]
fn test_and_ax_imm16_basic() {
    let code = [
        0x66, 0x25, 0x0F, 0x00, // AND AX, 0x000F
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x1234;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFF, 0x0004, "AX: 0x1234 AND 0x000F = 0x0004");
}

#[test]
fn test_and_eax_imm32_basic() {
    let code = [
        0x25, 0xFF, 0x00, 0x00, 0x00, // AND EAX, 0x000000FF
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x12345678;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x00000078, "EAX: mask to low byte");
    assert!(!zf_set(regs.rflags), "ZF should be clear");
}

#[test]
fn test_and_rax_imm32_basic() {
    let code = [
        0x48, 0x25, 0xFF, 0xFF, 0x00, 0x00, // AND RAX, 0x0000FFFF (sign-extended)
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x123456789ABCDEF0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x000000000000DEF0, "RAX: mask to low word");
}

// ============================================================================
// AND r/m with immediate (opcodes 80/4, 81/4, 83/4)
// ============================================================================

#[test]
fn test_and_rm8_imm8_basic() {
    let code = [
        0x80, 0xe3, 0x0F, // AND BL, 0x0F (ModRM=11_100_011)
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx & 0xFF, 0x0F, "BL: 0xFF AND 0x0F = 0x0F");
}

#[test]
fn test_and_rm16_imm16_basic() {
    let code = [
        0x66, 0x81, 0xe3, 0xF0, 0x0F, // AND BX, 0x0FF0
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xFFFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx & 0xFFFF, 0x0FF0, "BX: 0xFFFF AND 0x0FF0 = 0x0FF0");
}

#[test]
fn test_and_rm32_imm32_basic() {
    let code = [
        0x81, 0xe3, 0x00, 0xFF, 0x00, 0x00, // AND EBX, 0x0000FF00
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x12345678;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx, 0x00005600, "EBX: mask middle byte");
}

#[test]
fn test_and_rm64_imm32_basic() {
    let code = [
        0x48, 0x81, 0xe3, 0xFF, 0xFF, 0xFF, 0x00, // AND RBX, 0x00FFFFFF (sign-extended)
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x123456789ABCDEF0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx, 0x0000000000BCDEF0, "RBX: mask low 3 bytes");
}

#[test]
fn test_and_rm16_imm8_sign_extended() {
    // imm8 is sign-extended to 16 bits
    // 0xFF sign-extended to 16 bits = 0xFFFF
    let code = [
        0x66, 0x83, 0xe3, 0xFF, // AND BX, 0xFF (sign-extended to 0xFFFF)
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x1234;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx & 0xFFFF, 0x1234, "BX: AND with 0xFFFF (no change)");
}

#[test]
fn test_and_rm32_imm8_sign_extended() {
    // 0x0F sign-extended to 32 bits = 0x0000000F
    let code = [
        0x83, 0xe3, 0x0F, // AND EBX, 0x0F
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xFFFFFFFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx, 0x0000000F, "EBX: AND with sign-extended imm8");
}

// ============================================================================
// AND r/m, r (opcodes 20/r, 21/r)
// ============================================================================

#[test]
fn test_and_rm8_r8_basic() {
    let code = [
        0x20, 0xd8, // AND AL, BL (ModRM=11_011_000)
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0xFF; // AL = 0xFF
    regs.rbx = 0x0F; // BL = 0x0F
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x0F, "AL: 0xFF AND 0x0F = 0x0F");
}

#[test]
fn test_and_rm16_r16_basic() {
    let code = [
        0x66, 0x21, 0xd8, // AND AX, BX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0xFFFF;
    regs.rbx = 0x00FF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFF, 0x00FF, "AX: 0xFFFF AND 0x00FF = 0x00FF");
}

#[test]
fn test_and_rm32_r32_basic() {
    let code = [
        0x21, 0xd8, // AND EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x12345678;
    regs.rbx = 0xF0F0F0F0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x10305070, "EAX: bitwise AND with EBX");
}

#[test]
fn test_and_rm64_r64_basic() {
    let code = [
        0x48, 0x21, 0xd8, // AND RAX, RBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0xFFFFFFFFFFFFFFFF;
    regs.rbx = 0x00000000FFFFFFFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x00000000FFFFFFFF, "RAX: mask to low 32 bits");
}

// ============================================================================
// AND r, r/m (opcodes 22/r, 23/r)
// ============================================================================

#[test]
fn test_and_r8_rm8_basic() {
    let code = [
        0x22, 0xc3, // AND AL, BL (ModRM=11_000_011)
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0xAA; // AL = 0xAA (10101010)
    regs.rbx = 0x55; // BL = 0x55 (01010101)
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0, "AL: 0xAA AND 0x55 = 0 (no common bits)");
    assert!(zf_set(regs.rflags), "ZF should be set (zero result)");
}

#[test]
fn test_and_r16_rm16_basic() {
    let code = [
        0x66, 0x23, 0xc3, // AND AX, BX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x1234;
    regs.rbx = 0x0F0F;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFF, 0x0204, "AX: 0x1234 AND 0x0F0F");
}

#[test]
fn test_and_r32_rm32_basic() {
    let code = [
        0x23, 0xc3, // AND EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0xAAAAAAAA;
    regs.rbx = 0x55555555;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0, "EAX: alternating bits AND = 0");
    assert!(zf_set(regs.rflags), "ZF should be set");
}

#[test]
fn test_and_r64_rm64_basic() {
    let code = [
        0x48, 0x23, 0xc3, // AND RAX, RBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0xFF00FF00FF00FF00;
    regs.rbx = 0x00FF00FF00FF00FF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0, "RAX: alternating bytes AND = 0");
    assert!(zf_set(regs.rflags), "ZF should be set");
}

// ============================================================================
// Bit masking use cases
// ============================================================================

#[test]
fn test_and_mask_low_nibble() {
    // Extract low 4 bits
    let code = [0x24, 0x0F, 0xf4]; // AND AL, 0x0F
    let mut regs = Registers::default();
    regs.rax = 0xB7; // 10110111
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFF,
        0x07,
        "Extract low nibble: 0xB7 & 0x0F = 0x07"
    );
}

#[test]
fn test_and_mask_high_byte() {
    // Mask to keep only high byte of 32-bit value
    let code = [
        0x25, 0x00, 0x00, 0x00, 0xFF, // AND EAX, 0xFF000000
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x12345678;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x12000000, "EAX: keep only high byte");
}

#[test]
fn test_and_clear_specific_bits() {
    // Clear bit 4 and bit 6 using AND with inverted mask
    // Original: 0xFF (11111111)
    // Mask: ~(0x10 | 0x40) = ~0x50 = 0xAF (10101111)
    let code = [0x24, 0xAF, 0xf4]; // AND AL, 0xAF
    let mut regs = Registers::default();
    regs.rax = 0xFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0xAF, "Clear bits 4 and 6");
}

// ============================================================================
// Parity flag tests
// ============================================================================

#[test]
fn test_and_parity_even() {
    // Result with even number of 1-bits
    let code = [0x24, 0x03, 0xf4]; // AND AL, 0x03
    let mut regs = Registers::default();
    regs.rax = 0xFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Result = 0x03 (00000011), two 1-bits = even parity
    assert_eq!(regs.rax & 0xFF, 0x03);
    assert!(pf_set(regs.rflags), "PF should be set (even parity)");
}

#[test]
fn test_and_parity_odd() {
    // Result with odd number of 1-bits
    let code = [0x24, 0x07, 0xf4]; // AND AL, 0x07
    let mut regs = Registers::default();
    regs.rax = 0xFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Result = 0x07 (00000111), three 1-bits = odd parity
    assert_eq!(regs.rax & 0xFF, 0x07);
    assert!(!pf_set(regs.rflags), "PF should be clear (odd parity)");
}

// ============================================================================
// OF and CF always cleared
// ============================================================================

#[test]
fn test_and_clears_of_cf() {
    let code = [0x24, 0xFF, 0xf4]; // AND AL, 0xFF
    let mut regs = Registers::default();
    regs.rax = 0xFF;
    regs.rflags = 0x2 | flags::bits::OF | flags::bits::CF; // Set OF and CF
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(!of_set(regs.rflags), "OF should be cleared by AND");
    assert!(!cf_set(regs.rflags), "CF should be cleared by AND");
}

// ============================================================================
// Different registers
// ============================================================================

#[test]
fn test_and_different_registers() {
    // AND CL, DL
    let code = [0x20, 0xd1, 0xf4]; // ModRM=11_010_001
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

// ============================================================================
// Extended registers (R8-R15)
// ============================================================================

#[test]
fn test_and_r8b_basic() {
    let code = [
        0x41, 0x80, 0xe0, 0x0F, // AND R8B, 0x0F (REX.B 80 /4 ib)
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.r8 = 0xFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.r8 & 0xFF, 0x0F, "R8B: 0xFF AND 0x0F = 0x0F");
}

#[test]
fn test_and_r9w_basic() {
    let code = [
        0x66, 0x41, 0x81, 0xe1, 0xF0, 0x0F, // AND R9W, 0x0FF0
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.r9 = 0xFFFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.r9 & 0xFFFF, 0x0FF0, "R9W: mask");
}

#[test]
fn test_and_r10d_basic() {
    let code = [
        0x41, 0x81, 0xe2, 0xFF, 0x00, 0x00, 0x00, // AND R10D, 0x000000FF
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.r10 = 0x12345678;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.r10, 0x00000078, "R10D: mask to low byte");
}

#[test]
fn test_and_r11_basic() {
    let code = [
        0x49, 0x81, 0xe3, 0xFF, 0xFF, 0x00, 0x00, // AND R11, 0x0000FFFF
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.r11 = 0x123456789ABCDEF0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.r11, 0x000000000000DEF0, "R11: mask to low word");
}

#[test]
fn test_and_r15_r14() {
    let code = [
        0x4d, 0x21, 0xf7, // AND R15, R14 (REX.WRB 21 /r)
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.r15 = 0xFFFFFFFF00000000;
    regs.r14 = 0x00000000FFFFFFFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.r15, 0, "R15: no overlapping bits");
    assert!(zf_set(regs.rflags), "ZF should be set");
}

// ============================================================================
// Memory operands
// ============================================================================

#[test]
fn test_and_byte_ptr_mem() {
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
fn test_and_dword_ptr_mem() {
    let code = [
        0x81, 0x25, 0xf6, 0x0f, 0x00, 0x00, 0xFF, 0x00, 0x00,
        0x00, // AND DWORD PTR [rip+0x0FF6], 0x000000FF
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u32(&mem, 0x12345678);

    let _ = run_until_hlt(&mut vcpu).unwrap();
    let result = read_mem_u32(&mem);

    assert_eq!(result, 0x00000078, "Memory: mask to low byte");
}

#[test]
fn test_and_qword_ptr_mem() {
    let code = [
        0x48, 0x81, 0x25, 0xf5, 0x0f, 0x00, 0x00, 0xFF, 0xFF, 0x00,
        0x00, // AND QWORD PTR [rip+0x0FF5], 0x0000FFFF
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u64(&mem, 0x123456789ABCDEF0);

    let _ = run_until_hlt(&mut vcpu).unwrap();
    let result = read_mem_u64(&mem);

    assert_eq!(result, 0x000000000000DEF0, "Memory: mask to low word");
}

// ============================================================================
// Practical use cases
// ============================================================================

#[test]
fn test_and_check_bit_set() {
    // Check if bit 4 is set by ANDing with 0x10
    let code = [0x24, 0x10, 0xf4]; // AND AL, 0x10
    let mut regs = Registers::default();
    regs.rax = 0x1F; // bit 4 is set
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x10, "Bit 4 is set");
    assert!(!zf_set(regs.rflags), "ZF clear means bit was set");
}

#[test]
fn test_and_check_bit_clear() {
    // Check if bit 4 is set
    let code = [0x24, 0x10, 0xf4]; // AND AL, 0x10
    let mut regs = Registers::default();
    regs.rax = 0x0F; // bit 4 is clear
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0, "Bit 4 is clear");
    assert!(zf_set(regs.rflags), "ZF set means bit was clear");
}

#[test]
fn test_and_align_to_boundary() {
    // Align address to 16-byte boundary: addr & ~0x0F
    let code = [
        0x48, 0x83, 0xe0, 0xF0, // AND RAX, 0xFFFFFFFFFFFFFFF0 (sign-extended)
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x123456789ABCDEF7; // Unaligned address
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x123456789ABCDEF0, "Aligned to 16-byte boundary");
    assert_eq!(regs.rax & 0x0F, 0, "Low 4 bits cleared");
}
