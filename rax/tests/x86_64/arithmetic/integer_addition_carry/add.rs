//! Tests for the ADD instruction.
//!
//! ADD - Add
//!
//! Adds the destination operand (first operand) and the source operand (second operand)
//! and stores the result in the destination operand.
//!
//! Flags affected: OF, SF, ZF, AF, CF, PF
//!
//! Reference: docs/add.txt

use crate::common::*;
use rax::cpu::Registers;

// ============================================================================
// ADD AL, imm8 (opcode 04 ib)
// ============================================================================

#[test]
fn test_add_al_imm8_basic() {
    // ADD AL, 5
    // 04 05 = ADD AL, 5
    // f4 = HLT
    let code = [0x04, 0x05, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 10;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFF, 15, "ADD AL, 5: 10 + 5 = 15");
    assert!(!zf_set(regs.rflags), "ZF should be clear (result != 0)");
    assert!(!sf_set(regs.rflags), "SF should be clear (result positive)");
    assert!(!cf_set(regs.rflags), "CF should be clear (no carry)");
}

#[test]
fn test_add_al_imm8_zero_result() {
    // ADD AL, 0x100 - 0x55 = 0xAB to get 0
    let code = [0x04, 0xAB, 0xf4]; // ADD AL, 0xAB
    let mut regs = Registers::default();
    regs.rax = 0x55;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax & 0xFF,
        0,
        "ADD AL, 0xAB: 0x55 + 0xAB = 0x100 (wraps to 0)"
    );
    assert!(zf_set(regs.rflags), "ZF should be set (result = 0)");
    assert!(cf_set(regs.rflags), "CF should be set (carry out)");
}

#[test]
fn test_add_al_imm8_carry() {
    // ADD AL, 1 when AL = 0xFF -> 0x00 with carry
    let code = [0x04, 0x01, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 0xFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax & 0xFF,
        0,
        "ADD AL, 1: 0xFF + 1 = 0x00 (with carry)"
    );
    assert!(zf_set(regs.rflags), "ZF should be set");
    assert!(cf_set(regs.rflags), "CF should be set (carry out)");
}

#[test]
fn test_add_al_imm8_signed_overflow() {
    // Signed overflow: 0x7F + 1 = 0x80 (127 + 1 = -128 in signed)
    let code = [0x04, 0x01, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 0x7F;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFF, 0x80, "ADD AL, 1: 0x7F + 1 = 0x80");
    assert!(of_set(regs.rflags), "OF should be set (signed overflow)");
    assert!(sf_set(regs.rflags), "SF should be set (result negative)");
    assert!(
        !cf_set(regs.rflags),
        "CF should be clear (no unsigned carry)"
    );
}

#[test]
fn test_add_al_imm8_negative_overflow() {
    // Adding two negative numbers with overflow: 0x80 + 0x80 = 0x00
    let code = [0x04, 0x80, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 0x80;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFF, 0, "ADD AL, 0x80: 0x80 + 0x80 = 0x00");
    assert!(
        of_set(regs.rflags),
        "OF should be set (signed overflow: -128 + -128)"
    );
    assert!(cf_set(regs.rflags), "CF should be set (unsigned carry)");
    assert!(zf_set(regs.rflags), "ZF should be set (result = 0)");
}

#[test]
fn test_add_al_imm8_parity() {
    // Test parity flag - even number of 1s in low byte
    // 0x01 + 0x02 = 0x03 (binary 00000011) - 2 bits set (even) -> PF set
    let code = [0x04, 0x02, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 0x01;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFF, 0x03);
    assert!(pf_set(regs.rflags), "PF should be set (even parity)");
}

#[test]
fn test_add_al_imm8_no_parity() {
    // 0x01 + 0x00 = 0x01 (binary 00000001) - 1 bit set (odd) -> PF clear
    let code = [0x04, 0x00, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 0x01;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFF, 0x01);
    assert!(!pf_set(regs.rflags), "PF should be clear (odd parity)");
}

#[test]
fn test_add_al_imm8_preserves_high_bytes() {
    // Verify that adding to AL doesn't affect AH or higher bytes
    let code = [0x04, 0x05, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 0xDEADBEEF_12345678;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFF, 0x78 + 0x05);
    assert_eq!(
        regs.rax & !0xFF,
        0xDEADBEEF_12345600,
        "High bytes should be preserved"
    );
}

// ============================================================================
// ADD AX/EAX/RAX, imm16/32 (opcode 05)
// ============================================================================

#[test]
fn test_add_ax_imm16() {
    // ADD AX, 0x1234 (16-bit mode with 66 prefix)
    // 66 05 34 12 = ADD AX, 0x1234
    let code = [0x66, 0x05, 0x34, 0x12, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 0x1000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax & 0xFFFF,
        0x2234,
        "ADD AX, 0x1234: 0x1000 + 0x1234 = 0x2234"
    );
}

#[test]
fn test_add_eax_imm32() {
    // ADD EAX, 0x12345678
    // 05 78 56 34 12 = ADD EAX, 0x12345678
    let code = [0x05, 0x78, 0x56, 0x34, 0x12, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 0xFFFFFFFF_00000001;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let regs = run_until_hlt(&mut vcpu).unwrap();
    // 32-bit operation clears high 32 bits
    assert_eq!(regs.rax, 0x12345679, "ADD EAX clears high 32 bits of RAX");
}

#[test]
fn test_add_rax_imm32_sign_extended() {
    // REX.W ADD RAX, imm32 (sign-extended)
    // 48 05 ff ff ff ff = ADD RAX, -1 (sign-extended to 64 bits)
    let code = [0x48, 0x05, 0xff, 0xff, 0xff, 0xff, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 100;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let regs = run_until_hlt(&mut vcpu).unwrap();
    // 100 + (-1) = 99
    assert_eq!(regs.rax, 99, "ADD RAX, -1 (sign-extended): 100 + (-1) = 99");
}

#[test]
fn test_add_rax_imm32_large() {
    // REX.W ADD RAX, 0x7FFFFFFF
    // 48 05 ff ff ff 7f = ADD RAX, 0x7FFFFFFF
    let code = [0x48, 0x05, 0xff, 0xff, 0xff, 0x7f, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 1;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax, 0x80000000,
        "ADD RAX, 0x7FFFFFFF: 1 + 0x7FFFFFFF = 0x80000000"
    );
}

// ============================================================================
// ADD r/m8, imm8 (opcode 80 /0)
// ============================================================================

#[test]
fn test_add_rm8_imm8_register() {
    // ADD CL, 10
    // 80 c1 0a = ADD CL, 10
    let code = [0x80, 0xc1, 0x0a, 0xf4];
    let mut regs = Registers::default();
    regs.rcx = 5;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rcx & 0xFF, 15, "ADD CL, 10: 5 + 10 = 15");
}

#[test]
fn test_add_rm8_imm8_memory() {
    // ADD BYTE PTR [RBX], 10
    // 80 03 0a = ADD BYTE PTR [RBX], 10
    let code = [0x80, 0x03, 0x0a, 0xf4];
    let mut regs = Registers::default();
    regs.rbx = DATA_ADDR;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    write_mem_u8(&mem, 25);

    let _ = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_mem_u8(&mem), 35, "ADD [RBX], 10: 25 + 10 = 35");
}

// ============================================================================
// ADD r/m16/32/64, imm32 (opcode 81 /0)
// ============================================================================

#[test]
fn test_add_rm32_imm32_register() {
    // ADD ECX, 0x12345678
    // 81 c1 78 56 34 12 = ADD ECX, 0x12345678
    let code = [0x81, 0xc1, 0x78, 0x56, 0x34, 0x12, 0xf4];
    let mut regs = Registers::default();
    regs.rcx = 0x1;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rcx, 0x12345679,
        "ADD ECX, 0x12345678: 1 + 0x12345678 = 0x12345679"
    );
}

#[test]
fn test_add_rm64_imm32_register() {
    // REX.W ADD RCX, 0xFFFFFFFF (-1 sign-extended)
    // 48 81 c1 ff ff ff ff = ADD RCX, -1
    let code = [0x48, 0x81, 0xc1, 0xff, 0xff, 0xff, 0xff, 0xf4];
    let mut regs = Registers::default();
    regs.rcx = 0x100000000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rcx, 0xFFFFFFFF,
        "ADD RCX, -1: 0x100000000 + (-1) = 0xFFFFFFFF"
    );
}

#[test]
fn test_add_rm32_imm32_memory() {
    // ADD DWORD PTR [RBX], 0x1000
    // 81 03 00 10 00 00 = ADD DWORD PTR [RBX], 0x1000
    let code = [0x81, 0x03, 0x00, 0x10, 0x00, 0x00, 0xf4];
    let mut regs = Registers::default();
    regs.rbx = DATA_ADDR;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    write_mem_u32(&mem, 0x12345678);

    let _ = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_mem_u32(&mem), 0x12346678, "ADD [RBX], 0x1000");
}

// ============================================================================
// ADD r/m16/32/64, imm8 sign-extended (opcode 83 /0)
// ============================================================================

#[test]
fn test_add_rm32_imm8_positive() {
    // ADD ECX, 10 (using sign-extended imm8)
    // 83 c1 0a = ADD ECX, 10
    let code = [0x83, 0xc1, 0x0a, 0xf4];
    let mut regs = Registers::default();
    regs.rcx = 100;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rcx, 110, "ADD ECX, 10: 100 + 10 = 110");
}

#[test]
fn test_add_rm32_imm8_negative() {
    // ADD ECX, -10 (0xF6 sign-extended)
    // 83 c1 f6 = ADD ECX, -10
    let code = [0x83, 0xc1, 0xf6, 0xf4];
    let mut regs = Registers::default();
    regs.rcx = 100;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rcx, 90, "ADD ECX, -10: 100 + (-10) = 90");
}

#[test]
fn test_add_rm64_imm8_sign_extended() {
    // REX.W ADD RCX, -1 (0xFF sign-extended to 64-bit)
    // 48 83 c1 ff = ADD RCX, -1
    let code = [0x48, 0x83, 0xc1, 0xff, 0xf4];
    let mut regs = Registers::default();
    regs.rcx = 0x100000000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rcx, 0xFFFFFFFF,
        "ADD RCX, -1: 0x100000000 - 1 = 0xFFFFFFFF"
    );
}

#[test]
fn test_add_rm16_imm8_sign_extended() {
    // ADD CX, -1 (with 66 prefix)
    // 66 83 c1 ff = ADD CX, -1
    let code = [0x66, 0x83, 0xc1, 0xff, 0xf4];
    let mut regs = Registers::default();
    regs.rcx = 0x1000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rcx & 0xFFFF, 0x0FFF, "ADD CX, -1: 0x1000 - 1 = 0x0FFF");
}

// ============================================================================
// ADD r/m8, r8 (opcode 00 /r)
// ============================================================================

#[test]
fn test_add_rm8_r8_register() {
    // ADD AL, CL
    // 00 c8 = ADD AL, CL
    let code = [0x00, 0xc8, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 10;
    regs.rcx = 5;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFF, 15, "ADD AL, CL: 10 + 5 = 15");
}

#[test]
fn test_add_rm8_r8_memory() {
    // ADD [RBX], CL
    // 00 0b = ADD [RBX], CL
    let code = [0x00, 0x0b, 0xf4];
    let mut regs = Registers::default();
    regs.rbx = DATA_ADDR;
    regs.rcx = 20;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    write_mem_u8(&mem, 30);

    let _ = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_mem_u8(&mem), 50, "ADD [RBX], CL: 30 + 20 = 50");
}

#[test]
fn test_add_rm8_r8_same_register() {
    // ADD AL, AL (doubles the value)
    // 00 c0 = ADD AL, AL
    let code = [0x00, 0xc0, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 50;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFF, 100, "ADD AL, AL: 50 + 50 = 100");
}

// ============================================================================
// ADD r/m16/32/64, r16/32/64 (opcode 01 /r)
// ============================================================================

#[test]
fn test_add_rm32_r32_register() {
    // ADD EAX, ECX
    // 01 c8 = ADD EAX, ECX
    let code = [0x01, 0xc8, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 0x12345678;
    regs.rcx = 0x11111111;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x23456789, "ADD EAX, ECX");
}

#[test]
fn test_add_rm64_r64_register() {
    // REX.W ADD RAX, RCX
    // 48 01 c8 = ADD RAX, RCX
    let code = [0x48, 0x01, 0xc8, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 0xFFFFFFFF_00000000;
    regs.rcx = 0x00000001_00000000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x00000000_00000000, "ADD RAX, RCX with overflow");
    assert!(cf_set(regs.rflags), "CF should be set on overflow");
}

#[test]
fn test_add_rm32_r32_memory() {
    // ADD [RBX], ECX
    // 01 0b = ADD [RBX], ECX
    let code = [0x01, 0x0b, 0xf4];
    let mut regs = Registers::default();
    regs.rbx = DATA_ADDR;
    regs.rcx = 0x1000;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    write_mem_u32(&mem, 0x12345678);

    let _ = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_mem_u32(&mem), 0x12346678, "ADD [RBX], ECX");
}

#[test]
fn test_add_rm64_r64_memory() {
    // REX.W ADD [RBX], RCX
    // 48 01 0b = ADD [RBX], RCX
    let code = [0x48, 0x01, 0x0b, 0xf4];
    let mut regs = Registers::default();
    regs.rbx = DATA_ADDR;
    regs.rcx = 0x100000000;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    write_mem_u64(&mem, 0xFFFFFFFF);

    let _ = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_mem_u64(&mem), 0x1FFFFFFFF, "ADD [RBX], RCX (64-bit)");
}

// ============================================================================
// ADD r8, r/m8 (opcode 02 /r)
// ============================================================================

#[test]
fn test_add_r8_rm8_register() {
    // ADD CL, AL
    // 02 c8 = ADD CL, AL
    let code = [0x02, 0xc8, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 10;
    regs.rcx = 5;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rcx & 0xFF, 15, "ADD CL, AL: 5 + 10 = 15");
}

#[test]
fn test_add_r8_rm8_memory() {
    // ADD CL, [RBX]
    // 02 0b = ADD CL, [RBX]
    let code = [0x02, 0x0b, 0xf4];
    let mut regs = Registers::default();
    regs.rbx = DATA_ADDR;
    regs.rcx = 20;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    write_mem_u8(&mem, 30);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rcx & 0xFF, 50, "ADD CL, [RBX]: 20 + 30 = 50");
}

// ============================================================================
// ADD r16/32/64, r/m16/32/64 (opcode 03 /r)
// ============================================================================

#[test]
fn test_add_r32_rm32_register() {
    // ADD ECX, EAX
    // 03 c8 = ADD ECX, EAX
    let code = [0x03, 0xc8, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 0x11111111;
    regs.rcx = 0x22222222;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rcx, 0x33333333, "ADD ECX, EAX");
}

#[test]
fn test_add_r64_rm64_register() {
    // REX.W ADD RCX, RAX
    // 48 03 c8 = ADD RCX, RAX
    let code = [0x48, 0x03, 0xc8, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 0x1111111111111111;
    regs.rcx = 0x2222222222222222;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rcx, 0x3333333333333333, "ADD RCX, RAX (64-bit)");
}

#[test]
fn test_add_r32_rm32_memory() {
    // ADD ECX, [RBX]
    // 03 0b = ADD ECX, [RBX]
    let code = [0x03, 0x0b, 0xf4];
    let mut regs = Registers::default();
    regs.rbx = DATA_ADDR;
    regs.rcx = 0x1000;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    write_mem_u32(&mem, 0x12345678);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rcx, 0x12346678, "ADD ECX, [RBX]");
}

#[test]
fn test_add_r64_rm64_memory() {
    // REX.W ADD RCX, [RBX]
    // 48 03 0b = ADD RCX, [RBX]
    let code = [0x48, 0x03, 0x0b, 0xf4];
    let mut regs = Registers::default();
    regs.rbx = DATA_ADDR;
    regs.rcx = 0x100000000;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    write_mem_u64(&mem, 0xFFFFFFFF);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rcx, 0x1FFFFFFFF, "ADD RCX, [RBX] (64-bit)");
}

// ============================================================================
// Extended Register Tests (R8-R15)
// ============================================================================

#[test]
fn test_add_r8_extended() {
    // REX.B ADD R8D, 100
    // 41 83 c0 64 = ADD R8D, 100
    let code = [0x41, 0x83, 0xc0, 0x64, 0xf4];
    let mut regs = Registers::default();
    regs.r8 = 50;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.r8, 150, "ADD R8D, 100: 50 + 100 = 150");
}

#[test]
fn test_add_r15_extended() {
    // REX.WB ADD R15, RAX
    // 49 01 c7 = ADD R15, RAX
    let code = [0x49, 0x01, 0xc7, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 0x1000;
    regs.r15 = 0x2000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.r15, 0x3000, "ADD R15, RAX: 0x2000 + 0x1000 = 0x3000");
}

// ============================================================================
// Flag Edge Cases
// ============================================================================

#[test]
fn test_add_flags_32bit_overflow() {
    // 32-bit signed overflow: 0x7FFFFFFF + 1 = 0x80000000
    let code = [0x83, 0xc0, 0x01, 0xf4]; // ADD EAX, 1
    let mut regs = Registers::default();
    regs.rax = 0x7FFFFFFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x80000000);
    assert!(
        of_set(regs.rflags),
        "OF should be set (32-bit signed overflow)"
    );
    assert!(
        sf_set(regs.rflags),
        "SF should be set (result negative in 32-bit)"
    );
    assert!(
        !cf_set(regs.rflags),
        "CF should be clear (no unsigned overflow)"
    );
}

#[test]
fn test_add_flags_64bit_overflow() {
    // 64-bit signed overflow: 0x7FFFFFFFFFFFFFFF + 1 = 0x8000000000000000
    let code = [0x48, 0x83, 0xc0, 0x01, 0xf4]; // ADD RAX, 1
    let mut regs = Registers::default();
    regs.rax = 0x7FFFFFFFFFFFFFFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x8000000000000000);
    assert!(
        of_set(regs.rflags),
        "OF should be set (64-bit signed overflow)"
    );
    assert!(sf_set(regs.rflags), "SF should be set (result negative)");
}

#[test]
fn test_add_flags_64bit_carry() {
    // 64-bit unsigned overflow: 0xFFFFFFFFFFFFFFFF + 1 = 0
    let code = [0x48, 0x83, 0xc0, 0x01, 0xf4]; // ADD RAX, 1
    let mut regs = Registers::default();
    regs.rax = 0xFFFFFFFFFFFFFFFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0);
    assert!(cf_set(regs.rflags), "CF should be set (unsigned overflow)");
    assert!(zf_set(regs.rflags), "ZF should be set (result = 0)");
}

#[test]
fn test_add_auxiliary_carry() {
    // AF is set when there's a carry from bit 3 to bit 4
    // 0x0F + 0x01 = 0x10 (carry from bit 3)
    let code = [0x04, 0x01, 0xf4]; // ADD AL, 1
    let mut regs = Registers::default();
    regs.rax = 0x0F;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFF, 0x10);
    assert!(af_set(regs.rflags), "AF should be set (carry from bit 3)");
}

#[test]
fn test_add_no_auxiliary_carry() {
    // 0x0E + 0x01 = 0x0F (no carry from bit 3)
    let code = [0x04, 0x01, 0xf4]; // ADD AL, 1
    let mut regs = Registers::default();
    regs.rax = 0x0E;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFF, 0x0F);
    assert!(
        !af_set(regs.rflags),
        "AF should be clear (no carry from bit 3)"
    );
}

// ============================================================================
// Complex Addressing Mode Tests
// ============================================================================

#[test]
fn test_add_with_displacement() {
    // ADD EAX, [RBX + 8]
    // 03 43 08 = ADD EAX, [RBX + 8]
    let code = [0x03, 0x43, 0x08, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 100;
    regs.rbx = DATA_ADDR;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    // Write value at DATA_ADDR + 8
    write_mem_at_u32(&mem, DATA_ADDR + 8, 50);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 150, "ADD EAX, [RBX + 8]: 100 + 50 = 150");
}

#[test]
fn test_add_with_sib() {
    // ADD EAX, [RBX + RCX*4]
    // 03 04 8b = ADD EAX, [RBX + RCX*4]
    let code = [0x03, 0x04, 0x8b, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 100;
    regs.rbx = DATA_ADDR;
    regs.rcx = 2; // index = 2, scale = 4, so offset = 8
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    // Write value at DATA_ADDR + 8
    write_mem_at_u32(&mem, DATA_ADDR + 8, 25);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 125, "ADD EAX, [RBX + RCX*4]: 100 + 25 = 125");
}

// ============================================================================
// 16-bit Operand Tests
// ============================================================================

#[test]
fn test_add_rm16_r16() {
    // ADD AX, CX (with 66 prefix)
    // 66 01 c8 = ADD AX, CX
    let code = [0x66, 0x01, 0xc8, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 0xDEAD_1000;
    regs.rcx = 0xBEEF_0234;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let regs = run_until_hlt(&mut vcpu).unwrap();
    // Only low 16 bits affected, high bits of RAX preserved
    assert_eq!(
        regs.rax & 0xFFFF,
        0x1234,
        "ADD AX, CX: 0x1000 + 0x0234 = 0x1234"
    );
    assert_eq!(
        regs.rax & 0xFFFF0000,
        0xDEAD0000,
        "High word of EAX should be preserved"
    );
}

#[test]
fn test_add_16bit_overflow() {
    // ADD AX, 1 when AX = 0xFFFF
    let code = [0x66, 0x83, 0xc0, 0x01, 0xf4]; // ADD AX, 1
    let mut regs = Registers::default();
    regs.rax = 0xFFFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFFFF, 0, "ADD AX, 1: 0xFFFF + 1 = 0");
    assert!(cf_set(regs.rflags), "CF should be set (16-bit overflow)");
    assert!(zf_set(regs.rflags), "ZF should be set");
}

// ============================================================================
// Chained ADD Tests
// ============================================================================

#[test]
fn test_add_chain_multi_register() {
    // Multiple ADDs in sequence
    // ADD EAX, EBX
    // ADD EAX, ECX
    // ADD EAX, EDX
    let code = [
        0x01, 0xd8, // ADD EAX, EBX
        0x01, 0xc8, // ADD EAX, ECX
        0x01, 0xd0, // ADD EAX, EDX
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 10;
    regs.rbx = 20;
    regs.rcx = 30;
    regs.rdx = 40;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 100, "10 + 20 + 30 + 40 = 100");
}

#[test]
fn test_add_self_increment() {
    // ADD RAX, RAX three times (multiplies by 8)
    let code = [
        0x48, 0x01, 0xc0, // ADD RAX, RAX (x2)
        0x48, 0x01, 0xc0, // ADD RAX, RAX (x4)
        0x48, 0x01, 0xc0, // ADD RAX, RAX (x8)
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 5;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 40, "5 * 2 * 2 * 2 = 40");
}
