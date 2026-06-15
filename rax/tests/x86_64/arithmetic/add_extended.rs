use crate::common::*;
use rax::cpu::Registers;

// ADD — Integer Addition
//
// Opcodes:
// - 04 ib           ADD AL, imm8
// - 05 iw           ADD AX, imm16
// - 05 id           ADD EAX, imm32
// - REX.W + 05 id   ADD RAX, imm32 (sign-extended)
// - 80 /0 ib        ADD r/m8, imm8
// - 81 /0 iw        ADD r/m16, imm16
// - 81 /0 id        ADD r/m32, imm32
// - REX.W + 81 /0 id ADD r/m64, imm32 (sign-extended)
// - 83 /0 ib        ADD r/m16, imm8 (sign-extended)
// - 83 /0 ib        ADD r/m32, imm8 (sign-extended)
// - REX.W + 83 /0 ib ADD r/m64, imm8 (sign-extended)
// - 00 /r           ADD r/m8, r8
// - 01 /r           ADD r/m16, r16
// - 01 /r           ADD r/m32, r32
// - REX.W + 01 /r   ADD r/m64, r64
// - 02 /r           ADD r8, r/m8
// - 03 /r           ADD r16, r/m16
// - 03 /r           ADD r32, r/m32
// - REX.W + 03 /r   ADD r64, r/m64
//
// Operation: DEST := DEST + SRC
// Flags: CF, OF, SF, ZF, AF, PF are set according to result

// ============================================================================
// 8-bit ADD Tests
// ============================================================================

#[test]
fn test_add_al_imm8() {
    let code = [0x04, 0x05, 0xf4]; // ADD AL, 5; HLT
    let mut regs = Registers::default();
    regs.rax = 0x0A;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x0F, "AL should be 15 (10 + 5)");
    assert!(!cf_set(regs.rflags), "CF should be clear");
    assert!(!of_set(regs.rflags), "OF should be clear");
}

#[test]
fn test_add_al_overflow() {
    let code = [0x04, 0xFF, 0xf4]; // ADD AL, 0xFF
    let mut regs = Registers::default();
    regs.rax = 0x02;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x01, "AL should wrap to 1");
    assert!(cf_set(regs.rflags), "CF should be set (unsigned overflow)");
}

#[test]
fn test_add_r8_r8() {
    let code = [0x00, 0xd8, 0xf4]; // ADD AL, BL; HLT
    let mut regs = Registers::default();
    regs.rax = 0x20;
    regs.rbx = 0x15;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x35, "AL should be 0x35");
    assert!(!cf_set(regs.rflags), "CF should be clear");
}

#[test]
fn test_add_r8_r8_signed_overflow() {
    let code = [0x00, 0xc8, 0xf4]; // ADD AL, CL
    let mut regs = Registers::default();
    regs.rax = 0x7F; // Max positive i8
    regs.rcx = 0x01;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x80, "AL should be 0x80");
    assert!(of_set(regs.rflags), "OF should be set (signed overflow)");
    assert!(sf_set(regs.rflags), "SF should be set");
}

#[test]
fn test_add_all_8bit_gp_registers() {
    // Test ADD with AL, BL, CL, DL, SIL, DIL
    let test_cases = vec![
        (0xd8, 0x10, 0x05), // ADD AL, BL
        (0xc8, 0x10, 0x06), // ADD AL, CL
        (0xd0, 0x10, 0x07), // ADD AL, DL
    ];

    for (modrm, base, addend) in test_cases {
        let code = [0x00, modrm, 0xf4];
        let mut regs = Registers::default();
        regs.rax = base;
        regs.rbx = addend;
        regs.rcx = addend + 1;
        regs.rdx = addend + 2;
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        assert!((regs.rax & 0xFF) > base, "ADD should increase AL");
    }
}

#[test]
fn test_add_extended_r8_registers() {
    // ADD R8B, R9B
    let code = [0x45, 0x00, 0xc8, 0xf4]; // ADD R8B, R9B
    let mut regs = Registers::default();
    regs.r8 = 0x40;
    regs.r9 = 0x30;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.r8 & 0xFF, 0x70, "R8B should be 0x70");
}

#[test]
fn test_add_r8_from_r9_to_r15() {
    // Test all extended registers R8-R15
    for reg_num in 8..=15 {
        let modrm = 0xC0 | ((reg_num - 8) << 3) | (reg_num - 8);
        let code = [0x45, 0x00, modrm, 0xf4]; // ADD R*B, R*B (double itself)
        let mut regs = Registers::default();

        // Set value in the specific register
        match reg_num {
            8 => regs.r8 = 0x10,
            9 => regs.r9 = 0x10,
            10 => regs.r10 = 0x10,
            11 => regs.r11 = 0x10,
            12 => regs.r12 = 0x10,
            13 => regs.r13 = 0x10,
            14 => regs.r14 = 0x10,
            15 => regs.r15 = 0x10,
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

        assert_eq!(result & 0xFF, 0x20, "R{} should be 0x20", reg_num);
    }
}

// ============================================================================
// 16-bit ADD Tests
// ============================================================================

#[test]
fn test_add_ax_imm16() {
    let code = [0x66, 0x05, 0x34, 0x12, 0xf4]; // ADD AX, 0x1234
    let mut regs = Registers::default();
    regs.rax = 0x5678;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFF, 0x68AC, "AX should be 0x68AC");
}

#[test]
fn test_add_ax_overflow() {
    let code = [0x66, 0x05, 0xFF, 0xFF, 0xf4]; // ADD AX, 0xFFFF
    let mut regs = Registers::default();
    regs.rax = 0x0002;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFF, 0x0001, "AX should wrap to 1");
    assert!(cf_set(regs.rflags), "CF should be set");
}

#[test]
fn test_add_r16_r16() {
    let code = [0x66, 0x01, 0xd8, 0xf4]; // ADD AX, BX
    let mut regs = Registers::default();
    regs.rax = 0x1000;
    regs.rbx = 0x2000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFF, 0x3000, "AX should be 0x3000");
}

#[test]
fn test_add_r16_imm8_sign_extended() {
    let code = [0x66, 0x83, 0xc0, 0xFF, 0xf4]; // ADD AX, -1 (sign-extended)
    let mut regs = Registers::default();
    regs.rax = 0x1000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFF, 0x0FFF, "AX should be 0x0FFF");
}

#[test]
fn test_add_extended_r16_registers() {
    let code = [0x66, 0x45, 0x01, 0xda, 0xf4]; // ADD R10W, R11W
    let mut regs = Registers::default();
    regs.r10 = 0x4000;
    regs.r11 = 0x3000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.r10 & 0xFFFF, 0x7000, "R10W should be 0x7000");
}

#[test]
fn test_add_all_16bit_gp_registers() {
    // Test BX, CX, DX, SI, DI, SP, BP
    let regs_to_test = vec![
        (0xD8, "BX"),
        (0xC8, "CX"),
        (0xD0, "DX"),
        (0xF0, "SI"),
        (0xF8, "DI"),
        (0xE0, "SP"),
        (0xE8, "BP"),
    ];

    for (modrm, _name) in regs_to_test {
        let code = [0x66, 0x01, modrm, 0xf4]; // ADD AX, r16
        let mut regs = Registers::default();
        regs.rax = 0x1000;
        regs.rbx = 0x0100;
        regs.rcx = 0x0200;
        regs.rdx = 0x0300;
        regs.rsi = 0x0400;
        regs.rdi = 0x0500;
        regs.rsp = STACK_ADDR + 0x0600;
        regs.rbp = 0x0700;
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        assert!((regs.rax & 0xFFFF) > 0x1000, "AX should increase");
    }
}

// ============================================================================
// 32-bit ADD Tests
// ============================================================================

#[test]
fn test_add_eax_imm32() {
    let code = [0x05, 0x78, 0x56, 0x34, 0x12, 0xf4]; // ADD EAX, 0x12345678
    let mut regs = Registers::default();
    regs.rax = 0x11111111;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x23456789, "EAX should be 0x23456789");
}

#[test]
fn test_add_eax_overflow() {
    let code = [0x05, 0xFF, 0xFF, 0xFF, 0xFF, 0xf4]; // ADD EAX, 0xFFFFFFFF
    let mut regs = Registers::default();
    regs.rax = 0x00000002;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x00000001, "EAX should wrap to 1");
    assert!(cf_set(regs.rflags), "CF should be set");
}

#[test]
fn test_add_r32_r32() {
    let code = [0x01, 0xd8, 0xf4]; // ADD EAX, EBX
    let mut regs = Registers::default();
    regs.rax = 0x10000000;
    regs.rbx = 0x20000000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x30000000, "EAX should be 0x30000000");
}

#[test]
fn test_add_r32_imm8_sign_extended() {
    let code = [0x83, 0xc0, 0x7F, 0xf4]; // ADD EAX, 127
    let mut regs = Registers::default();
    regs.rax = 0x10000000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x1000007F, "EAX should be 0x1000007F");
}

#[test]
fn test_add_extended_r32_registers() {
    let code = [0x45, 0x01, 0xec, 0xf4]; // ADD R12D, R13D
    let mut regs = Registers::default();
    regs.r12 = 0x40000000;
    regs.r13 = 0x30000000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.r12, 0x70000000, "R12D should be 0x70000000");
}

#[test]
fn test_add_all_32bit_gp_registers() {
    // Test all 32-bit GP registers
    let regs_to_test = vec!["EAX", "EBX", "ECX", "EDX", "ESI", "EDI", "ESP", "EBP"];

    for i in 0..regs_to_test.len() {
        let modrm = 0xC0 | (i as u8);
        let code = [0x01, modrm, 0xf4]; // ADD EAX, reg
        let mut regs = Registers::default();
        regs.rax = 0x10000000;
        regs.rbx = 0x01000000;
        regs.rcx = 0x02000000;
        regs.rdx = 0x03000000;
        regs.rsi = 0x04000000;
        regs.rdi = 0x05000000;
        regs.rsp = STACK_ADDR + 0x06000000;
        regs.rbp = 0x07000000;
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        assert!(regs.rax >= 0x10000000, "EAX should increase");
    }
}

// ============================================================================
// 64-bit ADD Tests
// ============================================================================

#[test]
fn test_add_rax_imm32() {
    let code = [0x48, 0x05, 0x78, 0x56, 0x34, 0x12, 0xf4]; // ADD RAX, 0x12345678
    let mut regs = Registers::default();
    regs.rax = 0x1111111111111111;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x1111111123456789, "RAX should be correct");
}

#[test]
fn test_add_rax_overflow() {
    let code = [0x48, 0x05, 0xFF, 0xFF, 0xFF, 0x7F, 0xf4]; // ADD RAX, 0x7FFFFFFF
    let mut regs = Registers::default();
    regs.rax = 0xFFFFFFFF80000002;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x0000000000000001, "RAX should wrap");
    assert!(cf_set(regs.rflags), "CF should be set");
}

#[test]
fn test_add_r64_r64() {
    let code = [0x48, 0x01, 0xd8, 0xf4]; // ADD RAX, RBX
    let mut regs = Registers::default();
    regs.rax = 0x1000000000000000;
    regs.rbx = 0x2000000000000000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x3000000000000000, "RAX should be correct sum");
}

#[test]
fn test_add_r64_imm8_sign_extended() {
    let code = [0x48, 0x83, 0xc0, 0xFF, 0xf4]; // ADD RAX, -1
    let mut regs = Registers::default();
    regs.rax = 0x1000000000000000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x0FFFFFFFFFFFFFFF, "RAX should be decremented");
}

#[test]
fn test_add_all_64bit_gp_registers() {
    // Test RAX through R15
    let all_regs = [
        "RAX", "RCX", "RDX", "RBX", "RSP", "RBP", "RSI", "RDI", "R8", "R9", "R10", "R11", "R12",
        "R13", "R14", "R15",
    ];

    for i in 0..all_regs.len() {
        let (rex, modrm) = if i < 8 {
            (0x48, 0xC0 | (i as u8))
        } else {
            (0x4C, 0xC0 | ((i - 8) as u8))
        };

        let code = [rex, 0x01, modrm, 0xf4]; // ADD RAX, reg
        let mut regs = Registers::default();
        regs.rax = 0x1000000000000000;
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

        assert!(
            regs.rax >= 0x1000000000000000,
            "RAX should increase for {}",
            all_regs[i]
        );
    }
}

// ============================================================================
// Memory Operand Tests
// ============================================================================

#[test]
fn test_add_byte_ptr_imm8() {
    let code = [
        0x80, 0x05, 0xF9, 0x0F, 0x00, 0x00, 0x10, // ADD BYTE PTR [rip+0x0FF9], 0x10
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u8(&mem, 0x20);

    let _ = run_until_hlt(&mut vcpu).unwrap();
    let result = read_mem_u8(&mem);

    assert_eq!(result, 0x30, "Memory should be 0x30");
}

#[test]
fn test_add_word_ptr_r16() {
    let code = [
        0x66, 0x01, 0x1d, 0xF9, 0x0F, 0x00, 0x00, // ADD WORD PTR [rip+0x0FF6], BX
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u16(&mem, 0x1000);
    let mut regs = vcpu.get_regs().unwrap();
    regs.rbx = 0x2000;
    vcpu.set_regs(&regs).unwrap();

    let _ = run_until_hlt(&mut vcpu).unwrap();
    let result = read_mem_u16(&mem);

    assert_eq!(result, 0x3000, "Memory should be 0x3000");
}

#[test]
fn test_add_dword_ptr_r32() {
    let code = [
        0x01, 0x1d, 0xFA, 0x0F, 0x00, 0x00, // ADD DWORD PTR [rip+0x0FF7], EBX
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u32(&mem, 0x10000000);
    let mut regs = vcpu.get_regs().unwrap();
    regs.rbx = 0x20000000;
    vcpu.set_regs(&regs).unwrap();

    let _ = run_until_hlt(&mut vcpu).unwrap();
    let result = read_mem_u32(&mem);

    assert_eq!(result, 0x30000000, "Memory should be 0x30000000");
}

#[test]
fn test_add_qword_ptr_r64() {
    let code = [
        0x48, 0x01, 0x1d, 0xF9, 0x0F, 0x00, 0x00, // ADD QWORD PTR [rip+0x0FF6], RBX
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u64(&mem, 0x1000000000000000);
    let mut regs = vcpu.get_regs().unwrap();
    regs.rbx = 0x2000000000000000;
    vcpu.set_regs(&regs).unwrap();

    let _ = run_until_hlt(&mut vcpu).unwrap();
    let result = read_mem_u64(&mem);

    assert_eq!(result, 0x3000000000000000, "Memory should be correct sum");
}

#[test]
fn test_add_r64_from_memory() {
    let code = [
        0x48, 0x03, 0x05, 0xF9, 0x0F, 0x00, 0x00, // ADD RAX, QWORD PTR [rip+0x0FF6]
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u64(&mem, 0x2000000000000000);
    let mut regs = vcpu.get_regs().unwrap();
    regs.rax = 0x1000000000000000;
    vcpu.set_regs(&regs).unwrap();

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x3000000000000000, "RAX should be correct sum");
}

// ============================================================================
// Flag Tests
// ============================================================================

#[test]
fn test_add_zero_flag() {
    let code = [0x04, 0x00, 0xf4]; // ADD AL, 0
    let mut regs = Registers::default();
    regs.rax = 0x00;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x00, "AL should be 0");
    assert!(zf_set(regs.rflags), "ZF should be set");
}

#[test]
fn test_add_sign_flag() {
    let code = [0x04, 0x7F, 0xf4]; // ADD AL, 0x7F
    let mut regs = Registers::default();
    regs.rax = 0x01;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x80, "AL should be 0x80");
    assert!(sf_set(regs.rflags), "SF should be set");
}

#[test]
fn test_add_parity_flag() {
    let code = [0x04, 0x02, 0xf4]; // ADD AL, 2
    let mut regs = Registers::default();
    regs.rax = 0x01;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x03, "AL should be 3");
    assert!(pf_set(regs.rflags), "PF should be set (even parity)");
}

#[test]
fn test_add_auxiliary_carry() {
    let code = [0x04, 0x0A, 0xf4]; // ADD AL, 0x0A
    let mut regs = Registers::default();
    regs.rax = 0x08;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x12, "AL should be 0x12");
    assert!(af_set(regs.rflags), "AF should be set");
}

#[test]
fn test_add_overflow_positive_to_negative() {
    let code = [0x04, 0x01, 0xf4]; // ADD AL, 1
    let mut regs = Registers::default();
    regs.rax = 0x7F; // Max positive i8
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x80, "AL should be 0x80");
    assert!(of_set(regs.rflags), "OF should be set");
}

#[test]
fn test_add_overflow_negative_to_negative() {
    let code = [0x04, 0xFF, 0xf4]; // ADD AL, -1
    let mut regs = Registers::default();
    regs.rax = 0x80; // Min negative i8
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x7F, "AL should be 0x7F");
    assert!(of_set(regs.rflags), "OF should be set");
}

// ============================================================================
// Edge Cases and Special Tests
// ============================================================================

#[test]
fn test_add_zero_to_zero() {
    let code = [0x04, 0x00, 0xf4]; // ADD AL, 0
    let mut regs = Registers::default();
    regs.rax = 0x00;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x00, "AL should be 0");
    assert!(zf_set(regs.rflags), "ZF should be set");
    assert!(!cf_set(regs.rflags), "CF should be clear");
    assert!(!of_set(regs.rflags), "OF should be clear");
}

#[test]
fn test_add_self() {
    let code = [0x00, 0xc0, 0xf4]; // ADD AL, AL
    let mut regs = Registers::default();
    regs.rax = 0x42;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x84, "AL should be doubled");
}

#[test]
fn test_add_preserves_high_bits_8bit() {
    let code = [0x04, 0x05, 0xf4]; // ADD AL, 5
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
fn test_add_preserves_high_bits_16bit() {
    let code = [0x66, 0x05, 0x00, 0x10, 0xf4]; // ADD AX, 0x1000
    let mut regs = Registers::default();
    regs.rax = 0xDEADBEEF12345678;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax >> 16,
        0xDEADBEEF1234,
        "High bits should be preserved"
    );
}

#[test]
fn test_add_zeros_high_bits_32bit() {
    let code = [0x05, 0x00, 0x00, 0x00, 0x10, 0xf4]; // ADD EAX, 0x10000000
    let mut regs = Registers::default();
    regs.rax = 0xDEADBEEF12345678;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax >> 32,
        0,
        "High 32 bits should be zeroed for 32-bit op"
    );
}

#[test]
fn test_add_max_values() {
    let code = [0x48, 0x01, 0xd8, 0xf4]; // ADD RAX, RBX
    let mut regs = Registers::default();
    regs.rax = 0xFFFFFFFFFFFFFFFF;
    regs.rbx = 0x0000000000000001;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x0000000000000000, "RAX should wrap to 0");
    assert!(cf_set(regs.rflags), "CF should be set");
}

#[test]
fn test_add_commutative() {
    // Test that ADD is commutative: a + b == b + a
    let code1 = [0x00, 0xd8, 0xf4]; // ADD AL, BL
    let code2 = [0x00, 0xc3, 0xf4]; // ADD BL, AL

    let mut regs1 = Registers::default();
    regs1.rax = 0x42;
    regs1.rbx = 0x17;
    let (mut vcpu1, _) = setup_vm(&code1, Some(regs1));
    let result1 = run_until_hlt(&mut vcpu1).unwrap();

    let mut regs2 = Registers::default();
    regs2.rax = 0x42;
    regs2.rbx = 0x17;
    let (mut vcpu2, _) = setup_vm(&code2, Some(regs2));
    let result2 = run_until_hlt(&mut vcpu2).unwrap();

    assert_eq!(
        result1.rax & 0xFF,
        result2.rbx & 0xFF,
        "Results should match"
    );
}
