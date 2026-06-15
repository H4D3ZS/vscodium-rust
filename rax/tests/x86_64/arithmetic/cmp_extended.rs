use crate::common::*;
use rax::cpu::Registers;

// CMP — Compare Two Operands
//
// Opcodes:
// - 3C ib           CMP AL, imm8
// - 3D iw           CMP AX, imm16
// - 3D id           CMP EAX, imm32
// - REX.W + 3D id   CMP RAX, imm32 (sign-extended)
// - 80 /7 ib        CMP r/m8, imm8
// - 81 /7 iw        CMP r/m16, imm16
// - 81 /7 id        CMP r/m32, imm32
// - REX.W + 81 /7 id CMP r/m64, imm32 (sign-extended)
// - 83 /7 ib        CMP r/m16, imm8 (sign-extended)
// - 83 /7 ib        CMP r/m32, imm8 (sign-extended)
// - REX.W + 83 /7 ib CMP r/m64, imm8 (sign-extended)
// - 38 /r           CMP r/m8, r8
// - 39 /r           CMP r/m16, r16
// - 39 /r           CMP r/m32, r32
// - REX.W + 39 /r   CMP r/m64, r64
// - 3A /r           CMP r8, r/m8
// - 3B /r           CMP r16, r/m16
// - 3B /r           CMP r32, r/m32
// - REX.W + 3B /r   CMP r64, r/m64
//
// Operation: temp := DEST - SRC (result discarded, only flags set)
// Flags: CF, OF, SF, ZF, AF, PF are set according to result
// Note: CMP does NOT modify the destination operand

// ============================================================================
// 8-bit CMP Tests
// ============================================================================

#[test]
fn test_cmp_al_imm8_equal() {
    let code = [0x3C, 0x42, 0xf4]; // CMP AL, 0x42; HLT
    let mut regs = Registers::default();
    regs.rax = 0x42;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x42, "AL should not be modified");
    assert!(zf_set(regs.rflags), "ZF should be set (equal)");
    assert!(!cf_set(regs.rflags), "CF should be clear");
}

#[test]
fn test_cmp_al_imm8_greater() {
    let code = [0x3C, 0x20, 0xf4]; // CMP AL, 0x20
    let mut regs = Registers::default();
    regs.rax = 0x42;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x42, "AL should not be modified");
    assert!(!zf_set(regs.rflags), "ZF should be clear (not equal)");
    assert!(!cf_set(regs.rflags), "CF should be clear (AL > imm8)");
}

#[test]
fn test_cmp_al_imm8_less() {
    let code = [0x3C, 0x80, 0xf4]; // CMP AL, 0x80
    let mut regs = Registers::default();
    regs.rax = 0x42;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x42, "AL should not be modified");
    assert!(!zf_set(regs.rflags), "ZF should be clear (not equal)");
    assert!(cf_set(regs.rflags), "CF should be set (AL < imm8 unsigned)");
}

#[test]
fn test_cmp_r8_r8_equal() {
    let code = [0x38, 0xd8, 0xf4]; // CMP AL, BL
    let mut regs = Registers::default();
    regs.rax = 0x42;
    regs.rbx = 0x42;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x42, "AL should not be modified");
    assert_eq!(regs.rbx & 0xFF, 0x42, "BL should not be modified");
    assert!(zf_set(regs.rflags), "ZF should be set");
}

#[test]
fn test_cmp_r8_r8_less() {
    let code = [0x38, 0xd8, 0xf4]; // CMP AL, BL
    let mut regs = Registers::default();
    regs.rax = 0x10;
    regs.rbx = 0x20;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags), "CF should be set (AL < BL)");
    assert!(sf_set(regs.rflags), "SF should be set");
}

#[test]
fn test_cmp_all_8bit_registers() {
    for reg_num in 8..=15 {
        let modrm = 0xC0 | ((reg_num - 8) << 3) | (reg_num - 8);
        let code = [0x45, 0x38, modrm, 0xf4]; // CMP R*B, R*B
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

        assert!(
            zf_set(regs.rflags),
            "ZF should be set for R{} - R{}",
            reg_num,
            reg_num
        );
    }
}

#[test]
fn test_cmp_extended_r8() {
    let code = [0x45, 0x38, 0xc8, 0xf4]; // CMP R8B, R9B
    let mut regs = Registers::default();
    regs.r8 = 0x50;
    regs.r9 = 0x30;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.r8 & 0xFF, 0x50, "R8B should not be modified");
    assert_eq!(regs.r9 & 0xFF, 0x30, "R9B should not be modified");
    assert!(!cf_set(regs.rflags), "CF should be clear (R8B > R9B)");
}

// ============================================================================
// 16-bit CMP Tests
// ============================================================================

#[test]
fn test_cmp_ax_imm16_equal() {
    let code = [0x66, 0x3D, 0x34, 0x12, 0xf4]; // CMP AX, 0x1234
    let mut regs = Registers::default();
    regs.rax = 0x1234;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFF, 0x1234, "AX should not be modified");
    assert!(zf_set(regs.rflags), "ZF should be set");
}

#[test]
fn test_cmp_ax_imm16_greater() {
    let code = [0x66, 0x3D, 0x00, 0x10, 0xf4]; // CMP AX, 0x1000
    let mut regs = Registers::default();
    regs.rax = 0x2000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFF, 0x2000, "AX should not be modified");
    assert!(!cf_set(regs.rflags), "CF should be clear (AX > imm16)");
}

#[test]
fn test_cmp_ax_imm16_less() {
    let code = [0x66, 0x3D, 0x00, 0x80, 0xf4]; // CMP AX, 0x8000
    let mut regs = Registers::default();
    regs.rax = 0x4000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFF, 0x4000, "AX should not be modified");
    assert!(cf_set(regs.rflags), "CF should be set (AX < imm16)");
}

#[test]
fn test_cmp_r16_r16() {
    let code = [0x66, 0x39, 0xd8, 0xf4]; // CMP AX, BX
    let mut regs = Registers::default();
    regs.rax = 0x3000;
    regs.rbx = 0x3000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFF, 0x3000, "AX should not be modified");
    assert_eq!(regs.rbx & 0xFFFF, 0x3000, "BX should not be modified");
    assert!(zf_set(regs.rflags), "ZF should be set");
}

#[test]
fn test_cmp_r16_imm8_sign_extended() {
    let code = [0x66, 0x83, 0xf8, 0xFF, 0xf4]; // CMP AX, -1
    let mut regs = Registers::default();
    regs.rax = 0xFFFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFF, 0xFFFF, "AX should not be modified");
    assert!(zf_set(regs.rflags), "ZF should be set");
}

#[test]
fn test_cmp_extended_r16() {
    let code = [0x66, 0x45, 0x39, 0xda, 0xf4]; // CMP R10W, R11W
    let mut regs = Registers::default();
    regs.r10 = 0x8000;
    regs.r11 = 0x4000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.r10 & 0xFFFF, 0x8000, "R10W should not be modified");
    assert!(!cf_set(regs.rflags), "CF should be clear (R10W > R11W)");
}

// ============================================================================
// 32-bit CMP Tests
// ============================================================================

#[test]
fn test_cmp_eax_imm32_equal() {
    let code = [0x3D, 0x78, 0x56, 0x34, 0x12, 0xf4]; // CMP EAX, 0x12345678
    let mut regs = Registers::default();
    regs.rax = 0x12345678;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x12345678, "EAX should not be modified");
    assert!(zf_set(regs.rflags), "ZF should be set");
}

#[test]
fn test_cmp_eax_imm32_greater() {
    let code = [0x3D, 0x00, 0x00, 0x00, 0x10, 0xf4]; // CMP EAX, 0x10000000
    let mut regs = Registers::default();
    regs.rax = 0x20000000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x20000000, "EAX should not be modified");
    assert!(!cf_set(regs.rflags), "CF should be clear (EAX > imm32)");
}

#[test]
fn test_cmp_eax_imm32_less() {
    let code = [0x3D, 0x00, 0x00, 0x00, 0x80, 0xf4]; // CMP EAX, 0x80000000
    let mut regs = Registers::default();
    regs.rax = 0x40000000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x40000000, "EAX should not be modified");
    assert!(cf_set(regs.rflags), "CF should be set (EAX < imm32)");
}

#[test]
fn test_cmp_r32_r32() {
    let code = [0x39, 0xd8, 0xf4]; // CMP EAX, EBX
    let mut regs = Registers::default();
    regs.rax = 0x30000000;
    regs.rbx = 0x30000000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x30000000, "EAX should not be modified");
    assert_eq!(regs.rbx, 0x30000000, "EBX should not be modified");
    assert!(zf_set(regs.rflags), "ZF should be set");
}

#[test]
fn test_cmp_r32_imm8_sign_extended() {
    let code = [0x83, 0xf8, 0x7F, 0xf4]; // CMP EAX, 127
    let mut regs = Registers::default();
    regs.rax = 0x0000007F;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x0000007F, "EAX should not be modified");
    assert!(zf_set(regs.rflags), "ZF should be set");
}

#[test]
fn test_cmp_extended_r32() {
    let code = [0x45, 0x39, 0xec, 0xf4]; // CMP R12D, R13D
    let mut regs = Registers::default();
    regs.r12 = 0x70000000;
    regs.r13 = 0x30000000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.r12, 0x70000000, "R12D should not be modified");
    assert!(!cf_set(regs.rflags), "CF should be clear (R12D > R13D)");
}

// ============================================================================
// 64-bit CMP Tests
// ============================================================================

#[test]
fn test_cmp_rax_imm32_equal() {
    let code = [0x48, 0x3D, 0x78, 0x56, 0x34, 0x12, 0xf4]; // CMP RAX, 0x12345678
    let mut regs = Registers::default();
    regs.rax = 0x0000000012345678;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x0000000012345678, "RAX should not be modified");
    assert!(zf_set(regs.rflags), "ZF should be set");
}

#[test]
fn test_cmp_rax_imm32_greater() {
    let code = [0x48, 0x3D, 0x00, 0x00, 0x00, 0x10, 0xf4]; // CMP RAX, 0x10000000
    let mut regs = Registers::default();
    regs.rax = 0x0000000020000000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x0000000020000000, "RAX should not be modified");
    assert!(!cf_set(regs.rflags), "CF should be clear (RAX > imm32)");
}

#[test]
fn test_cmp_rax_imm32_less() {
    let code = [0x48, 0x3D, 0xFF, 0xFF, 0xFF, 0x7F, 0xf4]; // CMP RAX, 0x7FFFFFFF
    let mut regs = Registers::default();
    regs.rax = 0x0000000010000000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x0000000010000000, "RAX should not be modified");
    assert!(cf_set(regs.rflags), "CF should be set (RAX < imm32)");
}

#[test]
fn test_cmp_r64_r64_equal() {
    let code = [0x48, 0x39, 0xd8, 0xf4]; // CMP RAX, RBX
    let mut regs = Registers::default();
    regs.rax = 0x1234567890ABCDEF;
    regs.rbx = 0x1234567890ABCDEF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x1234567890ABCDEF, "RAX should not be modified");
    assert_eq!(regs.rbx, 0x1234567890ABCDEF, "RBX should not be modified");
    assert!(zf_set(regs.rflags), "ZF should be set");
}

#[test]
fn test_cmp_r64_r64_less() {
    let code = [0x48, 0x39, 0xd8, 0xf4]; // CMP RAX, RBX
    let mut regs = Registers::default();
    regs.rax = 0x1000000000000000;
    regs.rbx = 0x2000000000000000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags), "CF should be set (RAX < RBX)");
}

#[test]
fn test_cmp_r64_r64_greater() {
    let code = [0x48, 0x39, 0xd8, 0xf4]; // CMP RAX, RBX
    let mut regs = Registers::default();
    regs.rax = 0x3000000000000000;
    regs.rbx = 0x1000000000000000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(!cf_set(regs.rflags), "CF should be clear (RAX > RBX)");
    assert!(!zf_set(regs.rflags), "ZF should be clear");
}

#[test]
fn test_cmp_r64_imm8_sign_extended() {
    let code = [0x48, 0x83, 0xf8, 0xFF, 0xf4]; // CMP RAX, -1
    let mut regs = Registers::default();
    regs.rax = 0xFFFFFFFFFFFFFFFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0xFFFFFFFFFFFFFFFF, "RAX should not be modified");
    assert!(zf_set(regs.rflags), "ZF should be set");
}

#[test]
fn test_cmp_all_64bit_registers() {
    for i in 0..16 {
        let (rex, modrm) = if i < 8 {
            (0x48, 0xC0 | (i as u8))
        } else {
            (0x4C, 0xC0 | ((i - 8) as u8))
        };

        let code = [rex, 0x39, modrm, 0xf4]; // CMP RAX, reg
        let mut regs = Registers::default();
        let test_val = 0x1234567890ABCDEF;
        regs.rax = test_val;
        regs.rcx = test_val;
        regs.rdx = test_val;
        regs.rbx = test_val;
        regs.rsp = test_val;
        regs.rbp = test_val;
        regs.rsi = test_val;
        regs.rdi = test_val;
        regs.r8 = test_val;
        regs.r9 = test_val;
        regs.r10 = test_val;
        regs.r11 = test_val;
        regs.r12 = test_val;
        regs.r13 = test_val;
        regs.r14 = test_val;
        regs.r15 = test_val;

        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        assert_eq!(regs.rax, test_val, "RAX should not be modified");
        assert!(zf_set(regs.rflags), "ZF should be set for equal values");
    }
}

// ============================================================================
// Memory Operand Tests
// ============================================================================

#[test]
fn test_cmp_byte_ptr_imm8() {
    let code = [
        0x80, 0x3D, 0xF9, 0x0F, 0x00, 0x00, 0x42, // CMP BYTE PTR [rip+0x0FF9], 0x42
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u8(&mem, 0x42);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    let result = read_mem_u8(&mem);

    assert_eq!(result, 0x42, "Memory should not be modified");
    assert!(zf_set(regs.rflags), "ZF should be set");
}

#[test]
fn test_cmp_qword_ptr_r64() {
    let code = [
        0x48, 0x39, 0x1d, 0xF9, 0x0F, 0x00, 0x00, // CMP QWORD PTR [rip+0x0FF6], RBX
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    let test_val = 0x1234567890ABCDEF;
    write_mem_u64(&mem, test_val);
    let mut regs = vcpu.get_regs().unwrap();
    regs.rbx = test_val;
    vcpu.set_regs(&regs).unwrap();

    let regs = run_until_hlt(&mut vcpu).unwrap();
    let result = read_mem_u64(&mem);

    assert_eq!(result, test_val, "Memory should not be modified");
    assert_eq!(regs.rbx, test_val, "RBX should not be modified");
    assert!(zf_set(regs.rflags), "ZF should be set");
}

#[test]
fn test_cmp_r64_from_memory() {
    let code = [
        0x48, 0x3B, 0x05, 0xF9, 0x0F, 0x00, 0x00, // CMP RAX, QWORD PTR [rip+0x0FF6]
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u64(&mem, 0x1000000000000000);
    let mut regs = vcpu.get_regs().unwrap();
    regs.rax = 0x1000000000000000;
    vcpu.set_regs(&regs).unwrap();

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x1000000000000000, "RAX should not be modified");
    assert!(zf_set(regs.rflags), "ZF should be set");
}

// ============================================================================
// Flag Tests
// ============================================================================

#[test]
fn test_cmp_zero_flag() {
    let code = [0x3C, 0x42, 0xf4]; // CMP AL, 0x42
    let mut regs = Registers::default();
    regs.rax = 0x42;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(zf_set(regs.rflags), "ZF should be set");
    assert!(!cf_set(regs.rflags), "CF should be clear");
}

#[test]
fn test_cmp_carry_flag_set() {
    let code = [0x3C, 0x50, 0xf4]; // CMP AL, 0x50
    let mut regs = Registers::default();
    regs.rax = 0x40;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags), "CF should be set (AL < imm8)");
}

#[test]
fn test_cmp_carry_flag_clear() {
    let code = [0x3C, 0x30, 0xf4]; // CMP AL, 0x30
    let mut regs = Registers::default();
    regs.rax = 0x40;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(!cf_set(regs.rflags), "CF should be clear (AL > imm8)");
}

#[test]
fn test_cmp_sign_flag() {
    let code = [0x3C, 0x90, 0xf4]; // CMP AL, 0x90
    let mut regs = Registers::default();
    regs.rax = 0x40;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(sf_set(regs.rflags), "SF should be set (negative result)");
}

#[test]
fn test_cmp_overflow_flag() {
    let code = [0x3C, 0x01, 0xf4]; // CMP AL, 1
    let mut regs = Registers::default();
    regs.rax = 0x80; // Min negative i8
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(of_set(regs.rflags), "OF should be set");
}

#[test]
fn test_cmp_parity_flag() {
    let code = [0x3C, 0x02, 0xf4]; // CMP AL, 2
    let mut regs = Registers::default();
    regs.rax = 0x05;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Result would be 3 (even parity)
    assert!(pf_set(regs.rflags), "PF should be set");
}

#[test]
fn test_cmp_auxiliary_flag() {
    let code = [0x3C, 0x05, 0xf4]; // CMP AL, 5
    let mut regs = Registers::default();
    regs.rax = 0x12;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(af_set(regs.rflags), "AF should be set");
}

// ============================================================================
// Edge Cases and Special Tests
// ============================================================================

#[test]
fn test_cmp_zero_to_zero() {
    let code = [0x3C, 0x00, 0xf4]; // CMP AL, 0
    let mut regs = Registers::default();
    regs.rax = 0x00;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x00, "AL should not be modified");
    assert!(zf_set(regs.rflags), "ZF should be set");
    assert!(!cf_set(regs.rflags), "CF should be clear");
}

#[test]
fn test_cmp_self() {
    let code = [0x38, 0xc0, 0xf4]; // CMP AL, AL
    let mut regs = Registers::default();
    regs.rax = 0x42;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x42, "AL should not be modified");
    assert!(zf_set(regs.rflags), "ZF should be set (equal)");
    assert!(!cf_set(regs.rflags), "CF should be clear");
}

#[test]
fn test_cmp_max_unsigned_values() {
    let code = [0x48, 0x39, 0xd8, 0xf4]; // CMP RAX, RBX
    let mut regs = Registers::default();
    regs.rax = 0xFFFFFFFFFFFFFFFF;
    regs.rbx = 0xFFFFFFFFFFFFFFFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0xFFFFFFFFFFFFFFFF, "RAX should not be modified");
    assert!(zf_set(regs.rflags), "ZF should be set");
}

#[test]
fn test_cmp_signed_negative_comparison() {
    let code = [0x3C, 0xFF, 0xf4]; // CMP AL, -1
    let mut regs = Registers::default();
    regs.rax = 0xFE; // -2 in i8
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0xFE, "AL should not be modified");
    assert!(
        cf_set(regs.rflags),
        "CF should be set (0xFE < 0xFF unsigned)"
    );
}

#[test]
fn test_cmp_preserves_all_bits() {
    let code = [0x3C, 0x78, 0xf4]; // CMP AL, 0x78
    let mut regs = Registers::default();
    regs.rax = 0xDEADBEEF12345678;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax, 0xDEADBEEF12345678,
        "RAX should be completely unchanged"
    );
    assert!(zf_set(regs.rflags), "ZF should be set");
}

#[test]
fn test_cmp_anti_commutativity() {
    // CMP A, B should have inverted CF compared to CMP B, A
    let code1 = [0x38, 0xd8, 0xf4]; // CMP AL, BL
    let code2 = [0x38, 0xc3, 0xf4]; // CMP BL, AL

    let mut regs1 = Registers::default();
    regs1.rax = 0x10;
    regs1.rbx = 0x20;
    let (mut vcpu1, _) = setup_vm(&code1, Some(regs1));
    let result1 = run_until_hlt(&mut vcpu1).unwrap();

    let mut regs2 = Registers::default();
    regs2.rax = 0x10;
    regs2.rbx = 0x20;
    let (mut vcpu2, _) = setup_vm(&code2, Some(regs2));
    let result2 = run_until_hlt(&mut vcpu2).unwrap();

    // AL < BL should set CF, but BL > AL should clear CF
    assert!(cf_set(result1.rflags), "CF should be set for AL < BL");
    assert!(!cf_set(result2.rflags), "CF should be clear for BL > AL");
}
