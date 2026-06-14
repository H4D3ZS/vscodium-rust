use crate::common::*;
use rax::cpu::{Registers, VCpu};

// IMUL — Signed Multiply
//
// IMUL has THREE distinct forms:
//
// 1. ONE-OPERAND FORM (similar to MUL):
//    - F6 /5       IMUL r/m8      AX := AL * r/m8
//    - F7 /5       IMUL r/m16     DX:AX := AX * r/m16
//    - F7 /5       IMUL r/m32     EDX:EAX := EAX * r/m32
//    - REX.W+F7 /5 IMUL r/m64     RDX:RAX := RAX * r/m64
//
// 2. TWO-OPERAND FORM (dest *= src, truncated):
//    - 0F AF /r         IMUL r16, r/m16
//    - 0F AF /r         IMUL r32, r/m32
//    - REX.W+0F AF /r   IMUL r64, r/m64
//
// 3. THREE-OPERAND FORM (dest = src1 * imm, truncated):
//    - 6B /r ib         IMUL r16/32/64, r/m16/32/64, imm8
//    - 69 /r iw/id      IMUL r16/32/64, r/m16/32/64, imm16/32
//
// Flags: CF and OF are set when significant bits are lost due to truncation
//        or when result doesn't fit in sign-extended lower half.
//        SF, ZF, AF, PF are undefined.
//
// CRITICAL: IMUL handles SIGNED integers, unlike MUL which is unsigned.

// ============================================================================
// ONE-OPERAND FORM: 8-bit IMUL (opcode F6 /5) - Result in AX (AH:AL)
// ============================================================================

#[test]
fn test_imul_al_positive_basic() {
    let code = [
        0xf6, 0xeb, // IMUL BL (F6 /5, ModRM=11_101_011)
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x05; // AL = 5
    regs.rbx = 0x03; // BL = 3
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFF, 0x000F, "5 * 3 = 15");
    assert!(!cf_set(regs.rflags), "CF should be clear (fits in AL)");
    assert!(!of_set(regs.rflags), "OF should be clear");
}

#[test]
fn test_imul_al_negative_by_positive() {
    // -5 (0xFB) * 3 = -15 (0xFFF1 in 16-bit two's complement)
    let code = [0xf6, 0xeb, 0xf4]; // IMUL BL
    let mut regs = Registers::default();
    regs.rax = 0xFB; // AL = -5 (as i8)
    regs.rbx = 0x03; // BL = 3
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFF, 0xFFF1, "-5 * 3 = -15 (0xFFF1)");
    assert!(
        !cf_set(regs.rflags),
        "CF should be clear (fits in sign-extended AL)"
    );
}

#[test]
fn test_imul_al_negative_by_negative() {
    // -5 (0xFB) * -3 (0xFD) = 15 (0x000F)
    let code = [0xf6, 0xeb, 0xf4]; // IMUL BL
    let mut regs = Registers::default();
    regs.rax = 0xFB; // AL = -5
    regs.rbx = 0xFD; // BL = -3
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFF, 0x000F, "-5 * -3 = 15");
    assert!(!cf_set(regs.rflags), "CF should be clear");
}

#[test]
fn test_imul_al_overflow() {
    // 100 * 2 = 200 (0x00C8), doesn't fit in sign-extended i8 (-128 to 127)
    let code = [0xf6, 0xeb, 0xf4]; // IMUL BL
    let mut regs = Registers::default();
    regs.rax = 100; // AL = 100
    regs.rbx = 2; // BL = 2
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFF, 0x00C8, "100 * 2 = 200");
    assert!(
        cf_set(regs.rflags),
        "CF should be set (doesn't fit in i8 range)"
    );
    assert!(of_set(regs.rflags), "OF should be set");
}

#[test]
fn test_imul_al_max_positive() {
    // 127 * 1 = 127 (fits in i8)
    let code = [0xf6, 0xeb, 0xf4]; // IMUL BL
    let mut regs = Registers::default();
    regs.rax = 127; // AL = 127 (max i8)
    regs.rbx = 1; // BL = 1
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFF, 127, "127 * 1 = 127");
    assert!(!cf_set(regs.rflags), "CF should be clear (fits)");
}

#[test]
fn test_imul_al_min_negative() {
    // -128 * 1 = -128 (0xFF80, fits in sign-extended i8)
    let code = [0xf6, 0xeb, 0xf4]; // IMUL BL
    let mut regs = Registers::default();
    regs.rax = 0x80; // AL = -128 (min i8)
    regs.rbx = 1; // BL = 1
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFF, 0xFF80, "-128 * 1 = -128");
    assert!(!cf_set(regs.rflags), "CF should be clear");
}

#[test]
fn test_imul_al_overflow_negative() {
    // -128 * 2 = -256 (0xFF00), doesn't fit in i8
    let code = [0xf6, 0xeb, 0xf4]; // IMUL BL
    let mut regs = Registers::default();
    regs.rax = 0x80; // AL = -128
    regs.rbx = 2; // BL = 2
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFF, 0xFF00, "-128 * 2 = -256");
    assert!(cf_set(regs.rflags), "CF should be set (overflow)");
}

#[test]
fn test_imul_al_zero() {
    let code = [0xf6, 0xeb, 0xf4]; // IMUL BL
    let mut regs = Registers::default();
    regs.rax = 0; // AL = 0
    regs.rbx = 100;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFF, 0, "0 * 100 = 0");
    assert!(!cf_set(regs.rflags), "CF should be clear");
}

// ============================================================================
// ONE-OPERAND FORM: 16-bit IMUL (opcode F7 /5 with 0x66 prefix) - Result in DX:AX
// ============================================================================

#[test]
fn test_imul_ax_positive_basic() {
    let code = [
        0x66, 0xf7, 0xeb, // IMUL BX (66 F7 /5)
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 100; // AX = 100
    regs.rbx = 50; // BX = 50
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFF, 5000, "AX: 100 * 50 = 5000");
    assert_eq!(regs.rdx & 0xFFFF, 0, "DX = 0");
    assert!(!cf_set(regs.rflags), "CF should be clear");
}

#[test]
fn test_imul_ax_negative_by_positive() {
    // -100 (0xFF9C) * 50 = -5000 (0xFFFFEC78 in 32-bit)
    let code = [0x66, 0xf7, 0xeb, 0xf4]; // IMUL BX
    let mut regs = Registers::default();
    regs.rax = 0xFF9C; // AX = -100 (as i16)
    regs.rbx = 50;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFF, 0xEC78, "AX (low word)");
    assert_eq!(regs.rdx & 0xFFFF, 0xFFFF, "DX (high word, sign extension)");
    assert!(
        !cf_set(regs.rflags),
        "CF should be clear (fits in sign-extended)"
    );
}

#[test]
fn test_imul_ax_overflow() {
    // 1000 * 100 = 100000 (0x0186A0), doesn't fit in i16
    let code = [0x66, 0xf7, 0xeb, 0xf4]; // IMUL BX
    let mut regs = Registers::default();
    regs.rax = 1000;
    regs.rbx = 100;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFF, 0x86A0, "AX (low)");
    assert_eq!(regs.rdx & 0xFFFF, 0x0001, "DX (high)");
    assert!(cf_set(regs.rflags), "CF should be set (overflow)");
}

#[test]
fn test_imul_ax_max_positive() {
    // 32767 * 1 = 32767 (max i16)
    let code = [0x66, 0xf7, 0xeb, 0xf4]; // IMUL BX
    let mut regs = Registers::default();
    regs.rax = 0x7FFF; // max i16
    regs.rbx = 1;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFF, 0x7FFF, "32767 * 1 = 32767");
    assert_eq!(regs.rdx & 0xFFFF, 0, "DX = 0");
    assert!(!cf_set(regs.rflags), "CF should be clear");
}

// ============================================================================
// ONE-OPERAND FORM: 32-bit IMUL (opcode F7 /5) - Result in EDX:EAX
// ============================================================================

#[test]
fn test_imul_eax_positive_basic() {
    let code = [
        0xf7, 0xeb, // IMUL EBX (F7 /5)
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 1000;
    regs.rbx = 2000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 2000000, "EAX: 1000 * 2000 = 2000000");
    assert_eq!(regs.rdx, 0, "EDX = 0");
    assert!(!cf_set(regs.rflags), "CF should be clear");
}

#[test]
fn test_imul_eax_negative_by_positive() {
    // -1000 * 2000 = -2000000
    let code = [0xf7, 0xeb, 0xf4]; // IMUL EBX
    let mut regs = Registers::default();
    regs.rax = (-1000i32) as u64;
    regs.rbx = 2000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax as i32, -2000000, "EAX: -1000 * 2000 = -2000000");
    assert_eq!(regs.rdx, 0xFFFFFFFF, "EDX (sign extension)");
    assert!(!cf_set(regs.rflags), "CF should be clear");
}

#[test]
fn test_imul_eax_overflow() {
    // 100000 * 100000 = 10000000000, doesn't fit in i32
    let code = [0xf7, 0xeb, 0xf4]; // IMUL EBX
    let mut regs = Registers::default();
    regs.rax = 100000;
    regs.rbx = 100000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x540BE400, "EAX (low)");
    assert_eq!(regs.rdx, 0x00000002, "EDX (high)");
    assert!(cf_set(regs.rflags), "CF should be set (overflow)");
}

#[test]
fn test_imul_eax_max_positive() {
    // 2147483647 * 1 = 2147483647 (max i32)
    let code = [0xf7, 0xeb, 0xf4]; // IMUL EBX
    let mut regs = Registers::default();
    regs.rax = 0x7FFFFFFF;
    regs.rbx = 1;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x7FFFFFFF, "max i32 * 1");
    assert_eq!(regs.rdx, 0, "EDX = 0");
    assert!(!cf_set(regs.rflags), "CF should be clear");
}

// ============================================================================
// ONE-OPERAND FORM: 64-bit IMUL (opcode REX.W + F7 /5) - Result in RDX:RAX
// ============================================================================

#[test]
fn test_imul_rax_positive_basic() {
    let code = [
        0x48, 0xf7, 0xeb, // IMUL RBX (REX.W F7 /5)
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 1000000;
    regs.rbx = 2000000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 2000000000000, "RAX: 1M * 2M");
    assert_eq!(regs.rdx, 0, "RDX = 0");
    assert!(!cf_set(regs.rflags), "CF should be clear");
}

#[test]
fn test_imul_rax_negative_by_positive() {
    // -1000000 * 2000000 = -2000000000000
    let code = [0x48, 0xf7, 0xeb, 0xf4]; // IMUL RBX
    let mut regs = Registers::default();
    regs.rax = (-1000000i64) as u64;
    regs.rbx = 2000000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax as i64, -2000000000000, "RAX: product");
    assert_eq!(regs.rdx, 0xFFFFFFFFFFFFFFFF, "RDX (sign extension)");
    assert!(!cf_set(regs.rflags), "CF should be clear");
}

#[test]
fn test_imul_rax_overflow() {
    // Large multiplication that overflows to RDX
    let code = [0x48, 0xf7, 0xeb, 0xf4]; // IMUL RBX
    let mut regs = Registers::default();
    regs.rax = 0x0001000000000000;
    regs.rbx = 0x0001000000000000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x0000000000000000, "RAX = 0");
    assert_eq!(regs.rdx, 0x0000000100000000, "RDX (high bits)");
    assert!(cf_set(regs.rflags), "CF should be set");
}

#[test]
fn test_imul_rax_max_positive() {
    // 0x7FFFFFFFFFFFFFFF * 1 (max i64)
    let code = [0x48, 0xf7, 0xeb, 0xf4]; // IMUL RBX
    let mut regs = Registers::default();
    regs.rax = 0x7FFFFFFFFFFFFFFF;
    regs.rbx = 1;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x7FFFFFFFFFFFFFFF, "max i64 * 1");
    assert_eq!(regs.rdx, 0, "RDX = 0");
    assert!(!cf_set(regs.rflags), "CF should be clear");
}

// ============================================================================
// TWO-OPERAND FORM: IMUL r, r/m (opcode 0F AF /r) - Truncated result
// ============================================================================

#[test]
fn test_imul_two_op_16bit_basic() {
    let code = [
        0x66, 0x0f, 0xaf, 0xd8, // IMUL BX, AX (66 0F AF /r)
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 100; // AX = 100
    regs.rbx = 50; // BX = 50 (will be overwritten with result)
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx & 0xFFFF, 5000, "BX = BX * AX = 50 * 100 = 5000");
    assert!(!cf_set(regs.rflags), "CF should be clear (no truncation)");
}

#[test]
fn test_imul_two_op_16bit_negative() {
    // -50 * 100 = -5000
    let code = [0x66, 0x0f, 0xaf, 0xd8, 0xf4]; // IMUL BX, AX
    let mut regs = Registers::default();
    regs.rax = 100;
    regs.rbx = (-50i16) as u64;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!((regs.rbx & 0xFFFF) as i16, -5000, "BX = -50 * 100 = -5000");
    assert!(!cf_set(regs.rflags), "CF should be clear");
}

#[test]
fn test_imul_two_op_16bit_overflow() {
    // 1000 * 100 = 100000, truncated to 16 bits = 0x86A0 (34464)
    let code = [0x66, 0x0f, 0xaf, 0xd8, 0xf4]; // IMUL BX, AX
    let mut regs = Registers::default();
    regs.rax = 100;
    regs.rbx = 1000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx & 0xFFFF, 0x86A0, "BX = truncated result");
    assert!(
        cf_set(regs.rflags),
        "CF should be set (truncation occurred)"
    );
    assert!(of_set(regs.rflags), "OF should be set");
}

#[test]
fn test_imul_two_op_32bit_basic() {
    let code = [
        0x0f, 0xaf, 0xd8, // IMUL EBX, EAX (0F AF /r)
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 1000;
    regs.rbx = 2000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx, 2000000, "EBX = 2000 * 1000 = 2000000");
    assert!(!cf_set(regs.rflags), "CF should be clear");
}

#[test]
fn test_imul_two_op_32bit_negative() {
    // -1000 * 2000 = -2000000
    let code = [0x0f, 0xaf, 0xd8, 0xf4]; // IMUL EBX, EAX
    let mut regs = Registers::default();
    regs.rax = 2000;
    regs.rbx = (-1000i32) as u64;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx as i32, -2000000, "EBX = -1000 * 2000 = -2000000");
    assert!(!cf_set(regs.rflags), "CF should be clear");
}

#[test]
fn test_imul_two_op_32bit_overflow() {
    // 100000 * 100000 = 10000000000, truncated to 32 bits
    let code = [0x0f, 0xaf, 0xd8, 0xf4]; // IMUL EBX, EAX
    let mut regs = Registers::default();
    regs.rax = 100000;
    regs.rbx = 100000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx, 0x540BE400, "EBX = truncated to 32 bits");
    assert!(cf_set(regs.rflags), "CF should be set (truncation)");
}

#[test]
fn test_imul_two_op_64bit_basic() {
    let code = [
        0x48, 0x0f, 0xaf, 0xd8, // IMUL RBX, RAX (REX.W 0F AF /r)
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 1000000;
    regs.rbx = 2000000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx, 2000000000000, "RBX = 2M * 1M");
    assert!(!cf_set(regs.rflags), "CF should be clear");
}

#[test]
fn test_imul_two_op_64bit_negative() {
    // -1000000 * 2000000 = -2000000000000
    let code = [0x48, 0x0f, 0xaf, 0xd8, 0xf4]; // IMUL RBX, RAX
    let mut regs = Registers::default();
    regs.rax = 2000000;
    regs.rbx = (-1000000i64) as u64;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx as i64, -2000000000000, "RBX = product");
    assert!(!cf_set(regs.rflags), "CF should be clear");
}

#[test]
fn test_imul_two_op_other_registers() {
    // IMUL ECX, EDX
    let code = [0x0f, 0xaf, 0xca, 0xf4]; // ModRM=11_001_010 (ECX, EDX)
    let mut regs = Registers::default();
    regs.rcx = 123;
    regs.rdx = 456;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 56088, "ECX = 123 * 456");
}

// ============================================================================
// THREE-OPERAND FORM: IMUL r, r/m, imm (opcodes 6B, 69) - Truncated result
// ============================================================================

#[test]
fn test_imul_three_op_imm8_basic() {
    let code = [
        0x6b, 0xd8, 0x0A, // IMUL BX, AX, 10 (6B /r ib)
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 50; // AX = 50
    regs.rbx = 0; // BX will be set to AX * 10
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx & 0xFFFF, 500, "BX = AX * 10 = 50 * 10 = 500");
    assert!(!cf_set(regs.rflags), "CF should be clear");
}

#[test]
fn test_imul_three_op_imm8_negative() {
    // Sign-extended immediate: -5 (0xFB)
    let code = [0x6b, 0xd8, 0xFB, 0xf4]; // IMUL BX, AX, -5
    let mut regs = Registers::default();
    regs.rax = 100;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!((regs.rbx & 0xFFFF) as i16, -500, "BX = 100 * -5 = -500");
    assert!(!cf_set(regs.rflags), "CF should be clear");
}

#[test]
fn test_imul_three_op_imm8_32bit() {
    let code = [
        0x6b, 0xd8, 0x14, // IMUL EBX, EAX, 20
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 1000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx, 20000, "EBX = 1000 * 20 = 20000");
    assert!(!cf_set(regs.rflags), "CF should be clear");
}

#[test]
fn test_imul_three_op_imm8_64bit() {
    let code = [
        0x48, 0x6b, 0xd8, 0x0A, // IMUL RBX, RAX, 10
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 123456789;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx, 1234567890, "RBX = 123456789 * 10");
    assert!(!cf_set(regs.rflags), "CF should be clear");
}

#[test]
fn test_imul_three_op_imm16_basic() {
    let code = [
        0x66, 0x69, 0xd8, 0xE8, 0x03, // IMUL BX, AX, 1000 (69 /r iw)
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 10; // AX = 10
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx & 0xFFFF, 10000, "BX = 10 * 1000 = 10000");
    assert!(!cf_set(regs.rflags), "CF should be clear");
}

#[test]
fn test_imul_three_op_imm32_basic() {
    let code = [
        0x69, 0xd8, 0x10, 0x27, 0x00, 0x00, // IMUL EBX, EAX, 10000 (69 /r id)
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 100;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx, 1000000, "EBX = 100 * 10000 = 1000000");
    assert!(!cf_set(regs.rflags), "CF should be clear");
}

#[test]
fn test_imul_three_op_imm32_negative() {
    // -1000 as 32-bit immediate
    let code = [
        0x69, 0xd8, 0x18, 0xFC, 0xFF, 0xFF, // IMUL EBX, EAX, -1000
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 2000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx as i32, -2000000, "EBX = 2000 * -1000 = -2000000");
    assert!(!cf_set(regs.rflags), "CF should be clear");
}

#[test]
fn test_imul_three_op_overflow() {
    // 10000 * 10000 = 100000000, overflows 16-bit
    let code = [
        0x66, 0x69, 0xd8, 0x10, 0x27, // IMUL BX, AX, 10000
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 10000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Result truncated to 16 bits
    assert!(cf_set(regs.rflags), "CF should be set (overflow)");
    assert!(of_set(regs.rflags), "OF should be set");
}

#[test]
fn test_imul_three_op_different_regs() {
    // IMUL ECX, EDX, 5
    let code = [0x6b, 0xca, 0x05, 0xf4];
    let mut regs = Registers::default();
    regs.rdx = 100;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 500, "ECX = EDX * 5 = 100 * 5");
}

// ============================================================================
// Extended registers (R8-R15)
// ============================================================================

#[test]
fn test_imul_r8_one_operand() {
    let code = [
        0x41, 0xf6, 0xe8, // IMUL R8B (REX.B F6 /5)
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 20; // AL = 20
    regs.r8 = 5; // R8B = 5
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFF, 100, "AX = 20 * 5 = 100");
}

#[test]
fn test_imul_r10_two_operand() {
    let code = [
        0x49, 0x0f, 0xaf, 0xd2, // IMUL RDX, R10 (REX.WB 0F AF /r)
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rdx = 123;
    regs.r10 = 456;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rdx, 56088, "RDX = RDX * R10 = 123 * 456");
}

#[test]
fn test_imul_r11_three_operand() {
    let code = [
        0x4d, 0x6b, 0xdb, 0x0A, // IMUL R11, R11, 10 (REX.WRB 6B /r ib)
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.r11 = 999;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.r11, 9990, "R11 = R11 * 10 = 999 * 10");
}

// ============================================================================
// Memory operands
// ============================================================================

#[test]
fn test_imul_byte_ptr_mem() {
    let code = [
        0xf6, 0x2d, 0xfa, 0x0f, 0x00, 0x00, // IMUL BYTE PTR [rip+0x0FFA]
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u8(&mem, 10);

    // Get current regs to preserve RIP, then modify
    let mut regs = vcpu.get_regs().unwrap();
    regs.rax = 20; // AL = 20
    vcpu.set_regs(&regs).unwrap();

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFF, 200, "AX = 20 * 10 = 200");
}

#[test]
fn test_imul_two_op_mem() {
    let code = [
        0x0f, 0xaf, 0x1d, 0xf9, 0x0f, 0x00, 0x00, // IMUL EBX, [rip+0x0FF9]
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u32(&mem, 123);

    let mut regs = Registers::default();
    regs.rbx = 456;
    vcpu.set_regs(&regs).unwrap();

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx, 56088, "EBX = 456 * 123");
}

#[test]
fn test_imul_three_op_mem() {
    let code = [
        0x6b, 0x1d, 0xf9, 0x0f, 0x00, 0x00, 0x05, // IMUL EBX, [rip+0x0FF9], 5
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u32(&mem, 100);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx, 500, "EBX = mem * 5 = 100 * 5");
}
