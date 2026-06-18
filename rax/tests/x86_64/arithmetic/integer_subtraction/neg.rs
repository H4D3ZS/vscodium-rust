use crate::common::*;
use rax::cpu::Registers;

// NEG — Two's Complement Negation
//
// Opcodes:
// - F6 /3       NEG r/m8      Two's complement negate r/m8
// - REX + F6 /3 NEG r/m8*     Two's complement negate r/m8 (with REX for extended regs)
// - F7 /3       NEG r/m16     Two's complement negate r/m16
// - F7 /3       NEG r/m32     Two's complement negate r/m32
// - REX.W+F7 /3 NEG r/m64     Two's complement negate r/m64
//
// Operation: IF DEST = 0 THEN CF := 0; ELSE CF := 1; FI; DEST := -(DEST)
//            (Equivalent to: DEST := 0 - DEST)
//
// Flags: CF is set to 0 if source is 0, otherwise 1.
//        OF, SF, ZF, AF, PF are set according to result.
//
// CRITICAL: NEG of the most negative value (e.g., 0x80 for i8) causes signed overflow
// because the positive equivalent cannot be represented.

// ============================================================================
// 8-bit NEG (opcode F6 /3)
// ============================================================================

#[test]
fn test_neg_al_positive() {
    let code = [
        0xf6, 0xd8, // NEG AL (F6 /3, ModRM=11_011_000)
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x42; // 66
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFF,
        0xBE,
        "NEG 0x42 (66) = 0xBE (-66 in two's complement)"
    );
    assert!(
        cf_set(regs.rflags),
        "CF should be set (operand was non-zero)"
    );
    assert!(sf_set(regs.rflags), "SF should be set (negative result)");
    assert!(!zf_set(regs.rflags), "ZF should be clear");
}

#[test]
fn test_neg_al_one() {
    let code = [0xf6, 0xd8, 0xf4]; // NEG AL
    let mut regs = Registers::default();
    regs.rax = 0x01;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFF,
        0xFF,
        "NEG 1 = 0xFF (-1 in two's complement)"
    );
    assert!(cf_set(regs.rflags), "CF should be set");
    assert!(sf_set(regs.rflags), "SF should be set");
}

#[test]
fn test_neg_al_negative() {
    // NEG of -1 (0xFF) should give 1
    let code = [0xf6, 0xd8, 0xf4]; // NEG AL
    let mut regs = Registers::default();
    regs.rax = 0xFF; // -1
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x01, "NEG 0xFF (-1) = 1");
    assert!(cf_set(regs.rflags), "CF should be set");
    assert!(!sf_set(regs.rflags), "SF should be clear (positive result)");
}

#[test]
fn test_neg_al_zero() {
    // CRITICAL: NEG 0 = 0, and CF should be CLEAR (special case)
    let code = [0xf6, 0xd8, 0xf4]; // NEG AL
    let mut regs = Registers::default();
    regs.rax = 0x00;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0, "NEG 0 = 0");
    assert!(
        !cf_set(regs.rflags),
        "CF should be CLEAR (operand was zero)"
    );
    assert!(zf_set(regs.rflags), "ZF should be set");
    assert!(!sf_set(regs.rflags), "SF should be clear");
}

#[test]
fn test_neg_al_signed_overflow() {
    // NEG of 0x80 (-128, the most negative i8) = 0x80 (cannot represent +128)
    // This sets the overflow flag
    let code = [0xf6, 0xd8, 0xf4]; // NEG AL
    let mut regs = Registers::default();
    regs.rax = 0x80; // -128
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x80, "NEG 0x80 (-128) = 0x80 (overflow)");
    assert!(cf_set(regs.rflags), "CF should be set (non-zero operand)");
    assert!(of_set(regs.rflags), "OF should be set (signed overflow)");
    assert!(sf_set(regs.rflags), "SF should be set");
}

#[test]
fn test_neg_al_max_positive() {
    // NEG of 0x7F (127, max positive i8) = 0x81 (-127)
    let code = [0xf6, 0xd8, 0xf4]; // NEG AL
    let mut regs = Registers::default();
    regs.rax = 0x7F;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x81, "NEG 0x7F (127) = 0x81 (-127)");
    assert!(cf_set(regs.rflags), "CF should be set");
    assert!(sf_set(regs.rflags), "SF should be set");
    assert!(!of_set(regs.rflags), "OF should be clear (no overflow)");
}

#[test]
fn test_neg_bl_register() {
    let code = [
        0xf6, 0xdb, // NEG BL (F6 /3, ModRM=11_011_011)
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x05;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx & 0xFF, 0xFB, "NEG 5 = 0xFB (-5)");
    assert!(cf_set(regs.rflags), "CF should be set");
}

#[test]
fn test_neg_cl_register() {
    let code = [0xf6, 0xd9, 0xf4]; // NEG CL
    let mut regs = Registers::default();
    regs.rcx = 0x0A;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx & 0xFF, 0xF6, "NEG 10 = 0xF6 (-10)");
}

#[test]
fn test_neg_preserves_high_bytes_8bit() {
    let code = [0xf6, 0xd8, 0xf4]; // NEG AL
    let mut regs = Registers::default();
    regs.rax = 0xDEADBEEF_12345678;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x88, "AL: NEG 0x78 = 0x88");
    assert_eq!(
        regs.rax & !0xFF,
        0xDEADBEEF_12345600,
        "High bytes preserved"
    );
}

// ============================================================================
// 16-bit NEG (opcode F7 /3 with 0x66 prefix)
// ============================================================================

#[test]
fn test_neg_ax_positive() {
    let code = [
        0x66, 0xf7, 0xd8, // NEG AX (66 F7 /3)
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x1234;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFF, 0xEDCC, "NEG 0x1234 = 0xEDCC");
    assert!(cf_set(regs.rflags), "CF should be set");
    assert!(sf_set(regs.rflags), "SF should be set");
}

#[test]
fn test_neg_ax_zero() {
    let code = [0x66, 0xf7, 0xd8, 0xf4]; // NEG AX
    let mut regs = Registers::default();
    regs.rax = 0x0000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFF, 0, "NEG 0 = 0");
    assert!(!cf_set(regs.rflags), "CF should be clear (zero operand)");
    assert!(zf_set(regs.rflags), "ZF should be set");
}

#[test]
fn test_neg_ax_signed_overflow() {
    // NEG 0x8000 (-32768) = 0x8000 (overflow)
    let code = [0x66, 0xf7, 0xd8, 0xf4]; // NEG AX
    let mut regs = Registers::default();
    regs.rax = 0x8000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFF, 0x8000, "NEG 0x8000 = 0x8000 (overflow)");
    assert!(of_set(regs.rflags), "OF should be set");
    assert!(cf_set(regs.rflags), "CF should be set");
}

#[test]
fn test_neg_ax_negative() {
    // NEG -100 (0xFF9C) = 100 (0x0064)
    let code = [0x66, 0xf7, 0xd8, 0xf4]; // NEG AX
    let mut regs = Registers::default();
    regs.rax = 0xFF9C; // -100
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFF, 0x0064, "NEG -100 = 100");
    assert!(!sf_set(regs.rflags), "SF should be clear");
}

#[test]
fn test_neg_bx_register() {
    let code = [0x66, 0xf7, 0xdb, 0xf4]; // NEG BX
    let mut regs = Registers::default();
    regs.rbx = 0x0001;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx & 0xFFFF, 0xFFFF, "NEG 1 = 0xFFFF (-1)");
}

#[test]
fn test_neg_preserves_high_bytes_16bit() {
    let code = [0x66, 0xf7, 0xd8, 0xf4]; // NEG AX
    let mut regs = Registers::default();
    regs.rax = 0xDEADBEEF_12345678;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFF, 0xA988, "AX: NEG 0x5678");
    assert_eq!(
        regs.rax & !0xFFFF,
        0xDEADBEEF_12340000,
        "Upper bits preserved"
    );
}

// ============================================================================
// 32-bit NEG (opcode F7 /3, no prefix in 64-bit mode)
// ============================================================================

#[test]
fn test_neg_eax_positive() {
    let code = [
        0xf7, 0xd8, // NEG EAX (F7 /3)
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x12345678;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0xEDCBA988, "NEG 0x12345678 = 0xEDCBA988");
    assert!(cf_set(regs.rflags), "CF should be set");
    assert!(sf_set(regs.rflags), "SF should be set");
}

#[test]
fn test_neg_eax_zero() {
    let code = [0xf7, 0xd8, 0xf4]; // NEG EAX
    let mut regs = Registers::default();
    regs.rax = 0x00000000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0, "NEG 0 = 0");
    assert!(!cf_set(regs.rflags), "CF should be clear");
    assert!(zf_set(regs.rflags), "ZF should be set");
}

#[test]
fn test_neg_eax_signed_overflow() {
    // NEG 0x80000000 (-2147483648) = 0x80000000 (overflow)
    let code = [0xf7, 0xd8, 0xf4]; // NEG EAX
    let mut regs = Registers::default();
    regs.rax = 0x80000000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax, 0x80000000,
        "NEG 0x80000000 = 0x80000000 (overflow)"
    );
    assert!(of_set(regs.rflags), "OF should be set");
    assert!(cf_set(regs.rflags), "CF should be set");
}

#[test]
fn test_neg_eax_one() {
    let code = [0xf7, 0xd8, 0xf4]; // NEG EAX
    let mut regs = Registers::default();
    regs.rax = 0x00000001;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0xFFFFFFFF, "NEG 1 = 0xFFFFFFFF (-1)");
    assert!(cf_set(regs.rflags), "CF should be set");
}

#[test]
fn test_neg_eax_negative() {
    // NEG -1 (0xFFFFFFFF) = 1
    let code = [0xf7, 0xd8, 0xf4]; // NEG EAX
    let mut regs = Registers::default();
    regs.rax = 0xFFFFFFFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x00000001, "NEG -1 = 1");
    assert!(!sf_set(regs.rflags), "SF should be clear");
}

#[test]
fn test_neg_ebx_register() {
    let code = [0xf7, 0xdb, 0xf4]; // NEG EBX
    let mut regs = Registers::default();
    regs.rbx = 0x000000FF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx, 0xFFFFFF01, "NEG 255 = 0xFFFFFF01");
}

#[test]
fn test_neg_ecx_register() {
    let code = [0xf7, 0xd9, 0xf4]; // NEG ECX
    let mut regs = Registers::default();
    regs.rcx = 0x7FFFFFFF; // Max positive i32
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 0x80000001, "NEG 0x7FFFFFFF = 0x80000001");
}

// ============================================================================
// 64-bit NEG (opcode REX.W + F7 /3)
// ============================================================================

#[test]
fn test_neg_rax_positive() {
    let code = [
        0x48, 0xf7, 0xd8, // NEG RAX (REX.W F7 /3)
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x1234567890ABCDEF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0xEDCBA9876F543211, "NEG 0x1234567890ABCDEF");
    assert!(cf_set(regs.rflags), "CF should be set");
    assert!(sf_set(regs.rflags), "SF should be set");
}

#[test]
fn test_neg_rax_zero() {
    let code = [0x48, 0xf7, 0xd8, 0xf4]; // NEG RAX
    let mut regs = Registers::default();
    regs.rax = 0x0000000000000000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0, "NEG 0 = 0");
    assert!(!cf_set(regs.rflags), "CF should be clear");
    assert!(zf_set(regs.rflags), "ZF should be set");
}

#[test]
fn test_neg_rax_signed_overflow() {
    // NEG 0x8000000000000000 (most negative i64) = overflow
    let code = [0x48, 0xf7, 0xd8, 0xf4]; // NEG RAX
    let mut regs = Registers::default();
    regs.rax = 0x8000000000000000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x8000000000000000, "NEG 0x8000...000 = overflow");
    assert!(of_set(regs.rflags), "OF should be set");
    assert!(cf_set(regs.rflags), "CF should be set");
}

#[test]
fn test_neg_rax_one() {
    let code = [0x48, 0xf7, 0xd8, 0xf4]; // NEG RAX
    let mut regs = Registers::default();
    regs.rax = 0x0000000000000001;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0xFFFFFFFFFFFFFFFF, "NEG 1 = -1");
    assert!(cf_set(regs.rflags), "CF should be set");
}

#[test]
fn test_neg_rax_negative() {
    // NEG -1 = 1
    let code = [0x48, 0xf7, 0xd8, 0xf4]; // NEG RAX
    let mut regs = Registers::default();
    regs.rax = 0xFFFFFFFFFFFFFFFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x0000000000000001, "NEG -1 = 1");
    assert!(!sf_set(regs.rflags), "SF should be clear");
}

#[test]
fn test_neg_rbx_register() {
    let code = [0x48, 0xf7, 0xdb, 0xf4]; // NEG RBX
    let mut regs = Registers::default();
    regs.rbx = 0x0000000000000100;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx, 0xFFFFFFFFFFFFFf00, "NEG 256");
}

#[test]
fn test_neg_rcx_register() {
    let code = [0x48, 0xf7, 0xd9, 0xf4]; // NEG RCX
    let mut regs = Registers::default();
    regs.rcx = 0x7FFFFFFFFFFFFFFF; // Max positive i64
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 0x8000000000000001, "NEG max_i64");
}

#[test]
fn test_neg_rdx_register() {
    let code = [0x48, 0xf7, 0xda, 0xf4]; // NEG RDX
    let mut regs = Registers::default();
    regs.rdx = 0xFFFFFFFFFFFFFFFF; // -1
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rdx, 1, "NEG -1 = 1");
}

// ============================================================================
// Extended registers (R8-R15) with REX prefix
// ============================================================================

#[test]
fn test_neg_r8b_extended_register() {
    let code = [
        0x41, 0xf6, 0xd8, // NEG R8B (REX.B F6 /3)
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.r8 = 0x42;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.r8 & 0xFF, 0xBE, "NEG R8B: 0x42 -> 0xBE");
}

#[test]
fn test_neg_r9w_extended_register() {
    let code = [
        0x66, 0x41, 0xf7, 0xd9, // NEG R9W
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.r9 = 0x1234;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.r9 & 0xFFFF, 0xEDCC, "NEG R9W works");
}

#[test]
fn test_neg_r10d_extended_register() {
    let code = [
        0x41, 0xf7, 0xda, // NEG R10D
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.r10 = 0x12345678;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.r10, 0xEDCBA988, "NEG R10D works");
}

#[test]
fn test_neg_r11_extended_register() {
    let code = [
        0x49, 0xf7, 0xdb, // NEG R11 (REX.WB F7 /3)
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.r11 = 0x1234567890ABCDEF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.r11, 0xEDCBA9876F543211, "NEG R11 works");
}

#[test]
fn test_neg_r15_zero() {
    let code = [0x49, 0xf7, 0xdf, 0xf4]; // NEG R15
    let mut regs = Registers::default();
    regs.r15 = 0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.r15, 0, "NEG 0 = 0");
    assert!(!cf_set(regs.rflags), "CF should be clear (zero operand)");
}

// ============================================================================
// Memory operands
// ============================================================================

#[test]
fn test_neg_byte_ptr_mem() {
    let code = [
        0xf6, 0x1d, 0xfa, 0x0f, 0x00, 0x00, // NEG BYTE PTR [rip+0x0FFA]
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u8(&mem, 0x42);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    let result = read_mem_u8(&mem);

    assert_eq!(result, 0xBE, "NEG byte [mem]: 0x42 -> 0xBE");
    assert!(cf_set(regs.rflags), "CF should be set");
}

#[test]
fn test_neg_word_ptr_mem() {
    let code = [
        0x66, 0xf7, 0x1d, 0xf9, 0x0f, 0x00, 0x00, // NEG WORD PTR [rip+0x0FF9]
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u16(&mem, 0x1234);

    let _ = run_until_hlt(&mut vcpu).unwrap();
    let result = read_mem_u16(&mem);

    assert_eq!(result, 0xEDCC, "NEG word [mem]: 0x1234 -> 0xEDCC");
}

#[test]
fn test_neg_dword_ptr_mem() {
    let code = [
        0xf7, 0x1d, 0xfa, 0x0f, 0x00, 0x00, // NEG DWORD PTR [rip+0x0FFA]
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u32(&mem, 0x12345678);

    let _ = run_until_hlt(&mut vcpu).unwrap();
    let result = read_mem_u32(&mem);

    assert_eq!(result, 0xEDCBA988, "NEG dword [mem] works");
}

#[test]
fn test_neg_qword_ptr_mem() {
    let code = [
        0x48, 0xf7, 0x1d, 0xf9, 0x0f, 0x00, 0x00, // NEG QWORD PTR [rip+0x0FF9]
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u64(&mem, 0x1234567890ABCDEF);

    let _ = run_until_hlt(&mut vcpu).unwrap();
    let result = read_mem_u64(&mem);

    assert_eq!(result, 0xEDCBA9876F543211, "NEG qword [mem] works");
}

#[test]
fn test_neg_mem_zero() {
    let code = [
        0xf6, 0x1d, 0xfa, 0x0f, 0x00, 0x00, // NEG BYTE PTR [rip+0x0FFA]
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u8(&mem, 0x00);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    let result = read_mem_u8(&mem);

    assert_eq!(result, 0, "NEG 0 = 0");
    assert!(!cf_set(regs.rflags), "CF should be clear (zero operand)");
    assert!(zf_set(regs.rflags), "ZF should be set");
}

// ============================================================================
// Parity flag tests
// ============================================================================

#[test]
fn test_neg_parity_flag_even() {
    // NEG 3 = 0xFD (0b11111101, seven 1-bits = odd parity, PF=0)
    let code = [0xf6, 0xd8, 0xf4]; // NEG AL
    let mut regs = Registers::default();
    regs.rax = 0x03;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0xFD);
    assert!(!pf_set(regs.rflags), "PF should be clear (odd parity)");
}

#[test]
fn test_neg_parity_flag_odd() {
    // NEG 1 = 0xFF (0b11111111, eight 1-bits = even parity, PF=1)
    let code = [0xf6, 0xd8, 0xf4]; // NEG AL
    let mut regs = Registers::default();
    regs.rax = 0x01;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0xFF);
    assert!(pf_set(regs.rflags), "PF should be set (even parity)");
}

// ============================================================================
// Auxiliary carry flag tests
// ============================================================================

#[test]
fn test_neg_auxiliary_flag() {
    // NEG changes AF based on borrow from bit 3
    // NEG 0x0F = 0xF1: 0 - 0x0F requires borrow, AF should be set
    let code = [0xf6, 0xd8, 0xf4]; // NEG AL
    let mut regs = Registers::default();
    regs.rax = 0x0F;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0xF1);
    assert!(af_set(regs.rflags), "AF should be set");
}

// ============================================================================
// Edge cases and special scenarios
// ============================================================================

#[test]
fn test_neg_double_negation() {
    // NEG(NEG(x)) should equal x (except for overflow cases)
    let code = [
        0xf6, 0xd8, // NEG AL (first time)
        0xf6, 0xd8, // NEG AL (second time)
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x42;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x42, "NEG(NEG(0x42)) = 0x42");
}

#[test]
fn test_neg_all_operand_sizes_non_zero() {
    // Verify CF is set for all non-zero operands across all sizes

    // 8-bit
    let code = [0xf6, 0xd8, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 1;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert!(cf_set(regs.rflags), "8-bit: CF should be set for non-zero");

    // 16-bit
    let code = [0x66, 0xf7, 0xd8, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 1;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert!(cf_set(regs.rflags), "16-bit: CF should be set for non-zero");

    // 32-bit
    let code = [0xf7, 0xd8, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 1;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert!(cf_set(regs.rflags), "32-bit: CF should be set for non-zero");

    // 64-bit
    let code = [0x48, 0xf7, 0xd8, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 1;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert!(cf_set(regs.rflags), "64-bit: CF should be set for non-zero");
}
