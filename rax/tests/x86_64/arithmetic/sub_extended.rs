use crate::common::*;
use rax::cpu::Registers;

// SUB — Integer Subtraction
//
// Opcodes:
// - 2C ib           SUB AL, imm8
// - 2D iw           SUB AX, imm16
// - 2D id           SUB EAX, imm32
// - REX.W + 2D id   SUB RAX, imm32 (sign-extended)
// - 80 /5 ib        SUB r/m8, imm8
// - 81 /5 iw        SUB r/m16, imm16
// - 81 /5 id        SUB r/m32, imm32
// - REX.W + 81 /5 id SUB r/m64, imm32 (sign-extended)
// - 83 /5 ib        SUB r/m16, imm8 (sign-extended)
// - 83 /5 ib        SUB r/m32, imm8 (sign-extended)
// - REX.W + 83 /5 ib SUB r/m64, imm8 (sign-extended)
// - 28 /r           SUB r/m8, r8
// - 29 /r           SUB r/m16, r16
// - 29 /r           SUB r/m32, r32
// - REX.W + 29 /r   SUB r/m64, r64
// - 2A /r           SUB r8, r/m8
// - 2B /r           SUB r16, r/m16
// - 2B /r           SUB r32, r/m32
// - REX.W + 2B /r   SUB r64, r/m64
//
// Operation: DEST := DEST - SRC
// Flags: CF, OF, SF, ZF, AF, PF are set according to result

// ============================================================================
// 8-bit SUB Tests
// ============================================================================

#[test]
fn test_sub_al_imm8() {
    let code = [0x2C, 0x05, 0xf4]; // SUB AL, 5; HLT
    let mut regs = Registers::default();
    regs.rax = 0x0F;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x0A, "AL should be 10 (15 - 5)");
    assert!(!cf_set(regs.rflags), "CF should be clear");
}

#[test]
fn test_sub_al_underflow() {
    let code = [0x2C, 0x10, 0xf4]; // SUB AL, 0x10
    let mut regs = Registers::default();
    regs.rax = 0x05;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0xF5, "AL should wrap to 0xF5");
    assert!(cf_set(regs.rflags), "CF should be set (borrow)");
    assert!(sf_set(regs.rflags), "SF should be set");
}

#[test]
fn test_sub_r8_r8() {
    let code = [0x28, 0xd8, 0xf4]; // SUB AL, BL
    let mut regs = Registers::default();
    regs.rax = 0x30;
    regs.rbx = 0x10;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x20, "AL should be 0x20");
}

#[test]
fn test_sub_r8_r8_signed_overflow() {
    let code = [0x28, 0xc8, 0xf4]; // SUB AL, CL
    let mut regs = Registers::default();
    regs.rax = 0x80; // Min negative i8
    regs.rcx = 0x01;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x7F, "AL should be 0x7F");
    assert!(of_set(regs.rflags), "OF should be set (signed overflow)");
}

#[test]
fn test_sub_extended_r8() {
    let code = [0x45, 0x28, 0xc8, 0xf4]; // SUB R8B, R9B
    let mut regs = Registers::default();
    regs.r8 = 0x50;
    regs.r9 = 0x20;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.r8 & 0xFF, 0x30, "R8B should be 0x30");
}

#[test]
fn test_sub_all_8bit_registers() {
    for reg_num in 8..=15 {
        let modrm = 0xC0 | ((reg_num - 8) << 3) | (reg_num - 8);
        let code = [0x45, 0x28, modrm, 0xf4]; // SUB R*B, R*B (result: 0)
        let mut regs = Registers::default();

        match reg_num {
            8 => regs.r8 = 0x42,
            9 => regs.r9 = 0x42,
            10 => regs.r10 = 0x42,
            11 => regs.r11 = 0x42,
            12 => regs.r12 = 0x42,
            13 => regs.r13 = 0x42,
            14 => regs.r14 = 0x42,
            15 => regs.r15 = 0x42,
            _ => unreachable!(),
        }

        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        let result = match reg_num {
            8 => regs.r8,
            9 => regs.r9,
            10 => regs.r10,
            11 => regs.r11,
            12 => regs.r12,
            13 => regs.r13,
            14 => regs.r14,
            15 => regs.r15,
            _ => unreachable!(),
        };

        assert_eq!(
            result & 0xFF,
            0x00,
            "R{} - R{} should be 0",
            reg_num,
            reg_num
        );
    }
}

// ============================================================================
// 16-bit SUB Tests
// ============================================================================

#[test]
fn test_sub_ax_imm16() {
    let code = [0x66, 0x2D, 0x34, 0x12, 0xf4]; // SUB AX, 0x1234
    let mut regs = Registers::default();
    regs.rax = 0x5678;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFF, 0x4444, "AX should be 0x4444");
}

#[test]
fn test_sub_ax_underflow() {
    let code = [0x66, 0x2D, 0x00, 0x10, 0xf4]; // SUB AX, 0x1000
    let mut regs = Registers::default();
    regs.rax = 0x0500;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFF, 0xF500, "AX should wrap to 0xF500");
    assert!(cf_set(regs.rflags), "CF should be set");
}

#[test]
fn test_sub_r16_r16() {
    let code = [0x66, 0x29, 0xd8, 0xf4]; // SUB AX, BX
    let mut regs = Registers::default();
    regs.rax = 0x3000;
    regs.rbx = 0x1000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFF, 0x2000, "AX should be 0x2000");
}

#[test]
fn test_sub_r16_imm8_sign_extended() {
    let code = [0x66, 0x83, 0xe8, 0xFF, 0xf4]; // SUB AX, -1
    let mut regs = Registers::default();
    regs.rax = 0x1000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFF, 0x1001, "AX should be 0x1001");
}

#[test]
fn test_sub_extended_r16() {
    let code = [0x66, 0x45, 0x29, 0xda, 0xf4]; // SUB R10W, R11W
    let mut regs = Registers::default();
    regs.r10 = 0x8000;
    regs.r11 = 0x4000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.r10 & 0xFFFF, 0x4000, "R10W should be 0x4000");
}

// ============================================================================
// 32-bit SUB Tests
// ============================================================================

#[test]
fn test_sub_eax_imm32() {
    let code = [0x2D, 0x78, 0x56, 0x34, 0x12, 0xf4]; // SUB EAX, 0x12345678
    let mut regs = Registers::default();
    regs.rax = 0x23456789;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x11111111, "EAX should be 0x11111111");
}

#[test]
fn test_sub_eax_underflow() {
    let code = [0x2D, 0x02, 0x00, 0x00, 0x00, 0xf4]; // SUB EAX, 2
    let mut regs = Registers::default();
    regs.rax = 0x00000001;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0xFFFFFFFF, "EAX should wrap to 0xFFFFFFFF");
    assert!(cf_set(regs.rflags), "CF should be set");
}

#[test]
fn test_sub_r32_r32() {
    let code = [0x29, 0xd8, 0xf4]; // SUB EAX, EBX
    let mut regs = Registers::default();
    regs.rax = 0x30000000;
    regs.rbx = 0x10000000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x20000000, "EAX should be 0x20000000");
}

#[test]
fn test_sub_r32_imm8_sign_extended() {
    let code = [0x83, 0xe8, 0x7F, 0xf4]; // SUB EAX, 127
    let mut regs = Registers::default();
    regs.rax = 0x10000000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x0FFFFF81, "EAX should be 0x0FFFFF81");
}

#[test]
fn test_sub_extended_r32() {
    let code = [0x45, 0x29, 0xec, 0xf4]; // SUB R12D, R13D
    let mut regs = Registers::default();
    regs.r12 = 0x70000000;
    regs.r13 = 0x30000000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.r12, 0x40000000, "R12D should be 0x40000000");
}

// ============================================================================
// 64-bit SUB Tests
// ============================================================================

#[test]
fn test_sub_rax_imm32() {
    let code = [0x48, 0x2D, 0x78, 0x56, 0x34, 0x12, 0xf4]; // SUB RAX, 0x12345678
    let mut regs = Registers::default();
    regs.rax = 0x1111111123456789;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x1111111111111111, "RAX should be correct");
}

#[test]
fn test_sub_rax_underflow() {
    let code = [0x48, 0x2D, 0x02, 0x00, 0x00, 0x00, 0xf4]; // SUB RAX, 2
    let mut regs = Registers::default();
    regs.rax = 0x0000000000000001;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0xFFFFFFFFFFFFFFFF, "RAX should wrap to max u64");
    assert!(cf_set(regs.rflags), "CF should be set");
}

#[test]
fn test_sub_r64_r64() {
    let code = [0x48, 0x29, 0xd8, 0xf4]; // SUB RAX, RBX
    let mut regs = Registers::default();
    regs.rax = 0x3000000000000000;
    regs.rbx = 0x1000000000000000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x2000000000000000, "RAX should be correct");
}

#[test]
fn test_sub_r64_imm8_sign_extended() {
    let code = [0x48, 0x83, 0xe8, 0xFF, 0xf4]; // SUB RAX, -1
    let mut regs = Registers::default();
    regs.rax = 0x1000000000000000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x1000000000000001, "RAX should be incremented");
}

#[test]
fn test_sub_all_64bit_registers() {
    for i in 0..16 {
        let (rex, modrm) = if i < 8 {
            (0x48, 0xC0 | (i as u8))
        } else {
            (0x4C, 0xC0 | ((i - 8) as u8))
        };

        let code = [rex, 0x29, modrm, 0xf4]; // SUB RAX, reg
        let mut regs = Registers::default();
        regs.rax = 0x2000000000000000;
        regs.rcx = 0x0100000000000000;
        regs.rdx = 0x0200000000000000;
        regs.rbx = 0x0300000000000000;
        regs.rsp = STACK_ADDR;
        regs.rbp = 0x0500000000000000;
        regs.rsi = 0x0600000000000000;
        regs.rdi = 0x0700000000000000;
        regs.r8 = 0x0800000000000000;
        regs.r9 = 0x0900000000000000;
        regs.r10 = 0x0A00000000000000;
        regs.r11 = 0x0B00000000000000;
        regs.r12 = 0x0C00000000000000;
        regs.r13 = 0x0D00000000000000;
        regs.r14 = 0x0E00000000000000;
        regs.r15 = 0x0F00000000000000;

        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        assert!(regs.rax <= 0x2000000000000000, "RAX should decrease");
    }
}

// ============================================================================
// Memory Operand Tests
// ============================================================================

#[test]
fn test_sub_byte_ptr_imm8() {
    let code = [
        0x80, 0x2D, 0xF9, 0x0F, 0x00, 0x00, 0x10, // SUB BYTE PTR [rip+0x0FF9], 0x10
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u8(&mem, 0x30);

    let _ = run_until_hlt(&mut vcpu).unwrap();
    let result = read_mem_u8(&mem);

    assert_eq!(result, 0x20, "Memory should be 0x20");
}

#[test]
fn test_sub_qword_ptr_r64() {
    let code = [
        0x48, 0x29, 0x1d, 0xF9, 0x0F, 0x00, 0x00, // SUB QWORD PTR [rip+0x0FF6], RBX
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u64(&mem, 0x3000000000000000);
    let mut regs = vcpu.get_regs().unwrap();
    regs.rbx = 0x1000000000000000;
    vcpu.set_regs(&regs).unwrap();

    let _ = run_until_hlt(&mut vcpu).unwrap();
    let result = read_mem_u64(&mem);

    assert_eq!(result, 0x2000000000000000, "Memory should be correct");
}

#[test]
fn test_sub_r64_from_memory() {
    let code = [
        0x48, 0x2B, 0x05, 0xF9, 0x0F, 0x00, 0x00, // SUB RAX, QWORD PTR [rip+0x0FF6]
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u64(&mem, 0x1000000000000000);
    let mut regs = vcpu.get_regs().unwrap();
    regs.rax = 0x3000000000000000;
    vcpu.set_regs(&regs).unwrap();

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x2000000000000000, "RAX should be correct");
}

// ============================================================================
// Flag Tests
// ============================================================================

#[test]
fn test_sub_zero_flag() {
    let code = [0x2C, 0x05, 0xf4]; // SUB AL, 5
    let mut regs = Registers::default();
    regs.rax = 0x05;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x00, "AL should be 0");
    assert!(zf_set(regs.rflags), "ZF should be set");
}

#[test]
fn test_sub_sign_flag() {
    let code = [0x2C, 0x01, 0xf4]; // SUB AL, 1
    let mut regs = Registers::default();
    regs.rax = 0x00;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0xFF, "AL should be 0xFF");
    assert!(sf_set(regs.rflags), "SF should be set");
}

#[test]
fn test_sub_parity_flag() {
    let code = [0x2C, 0x02, 0xf4]; // SUB AL, 2
    let mut regs = Registers::default();
    regs.rax = 0x05;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x03, "AL should be 3");
    assert!(pf_set(regs.rflags), "PF should be set (even parity)");
}

#[test]
fn test_sub_auxiliary_flag() {
    let code = [0x2C, 0x05, 0xf4]; // SUB AL, 5
    let mut regs = Registers::default();
    regs.rax = 0x12;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x0D, "AL should be 0x0D");
    assert!(af_set(regs.rflags), "AF should be set");
}

#[test]
fn test_sub_overflow_flag() {
    let code = [0x2C, 0x01, 0xf4]; // SUB AL, 1
    let mut regs = Registers::default();
    regs.rax = 0x80; // Min negative i8
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x7F, "AL should be 0x7F");
    assert!(of_set(regs.rflags), "OF should be set");
}

// ============================================================================
// Edge Cases
// ============================================================================

#[test]
fn test_sub_self() {
    let code = [0x28, 0xc0, 0xf4]; // SUB AL, AL
    let mut regs = Registers::default();
    regs.rax = 0x42;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x00, "AL should be 0");
    assert!(zf_set(regs.rflags), "ZF should be set");
    assert!(!cf_set(regs.rflags), "CF should be clear");
}

#[test]
fn test_sub_preserves_high_bits() {
    let code = [0x2C, 0x05, 0xf4]; // SUB AL, 5
    let mut regs = Registers::default();
    regs.rax = 0xDEADBEEF12345678;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax >> 8,
        0xDEADBEEF123456,
        "High bits should be preserved"
    );
}

#[test]
fn test_sub_zero_from_zero() {
    let code = [0x2C, 0x00, 0xf4]; // SUB AL, 0
    let mut regs = Registers::default();
    regs.rax = 0x00;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x00, "AL should be 0");
    assert!(zf_set(regs.rflags), "ZF should be set");
    assert!(!cf_set(regs.rflags), "CF should be clear");
}
