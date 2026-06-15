use crate::common::*;
use crate::common::{run_until_hlt, setup_vm};
use rax::cpu::Registers;

// NOT — One's Complement Negation
//
// Opcodes:
// - F6 /2        NOT r/m8
// - F7 /2        NOT r/m16/32/64
//
// Operation: DEST := NOT DEST (bitwise inversion)
//
// Flags: No flags are affected.
//
// CRITICAL: NOT does NOT affect any flags (unlike other logical operations).

// ============================================================================
// NOT r/m8
// ============================================================================

#[test]
fn test_not_al_basic() {
    let code = [0xf6, 0xd0, 0xf4]; // NOT AL
    let mut regs = Registers::default();
    regs.rax = 0xAA; // 10101010
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x55, "AL: NOT 0xAA = 0x55");
}

#[test]
fn test_not_bl_all_zeros() {
    let code = [0xf6, 0xd3, 0xf4]; // NOT BL
    let mut regs = Registers::default();
    regs.rbx = 0x00;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx & 0xFF, 0xFF, "BL: NOT 0x00 = 0xFF");
}

#[test]
fn test_not_cl_all_ones() {
    let code = [0xf6, 0xd1, 0xf4]; // NOT CL
    let mut regs = Registers::default();
    regs.rcx = 0xFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx & 0xFF, 0x00, "CL: NOT 0xFF = 0x00");
}

#[test]
fn test_not_dl() {
    let code = [0xf6, 0xd2, 0xf4]; // NOT DL
    let mut regs = Registers::default();
    regs.rdx = 0xF0; // 11110000
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rdx & 0xFF, 0x0F, "DL: NOT 0xF0 = 0x0F");
}

#[test]
fn test_not_dh() {
    let code = [0xf6, 0xd6, 0xf4]; // NOT DH
    let mut regs = Registers::default();
    regs.rdx = 0x5500; // DH = 0x55
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!((regs.rdx >> 8) & 0xFF, 0xAA, "DH: NOT 0x55 = 0xAA");
}

// ============================================================================
// NOT r/m16
// ============================================================================

#[test]
fn test_not_ax_basic() {
    let code = [0x66, 0xf7, 0xd0, 0xf4]; // NOT AX
    let mut regs = Registers::default();
    regs.rax = 0xAAAA;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFF, 0x5555, "AX: NOT 0xAAAA = 0x5555");
}

#[test]
fn test_not_bx_all_zeros() {
    let code = [0x66, 0xf7, 0xd3, 0xf4]; // NOT BX
    let mut regs = Registers::default();
    regs.rbx = 0x0000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx & 0xFFFF, 0xFFFF, "BX: NOT 0x0000 = 0xFFFF");
}

#[test]
fn test_not_cx_pattern() {
    let code = [0x66, 0xf7, 0xd1, 0xf4]; // NOT CX
    let mut regs = Registers::default();
    regs.rcx = 0xFF00;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx & 0xFFFF, 0x00FF, "CX: NOT 0xFF00 = 0x00FF");
}

#[test]
fn test_not_si() {
    let code = [0x66, 0xf7, 0xd6, 0xf4]; // NOT SI
    let mut regs = Registers::default();
    regs.rsi = 0x1234;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rsi & 0xFFFF, 0xEDCB, "SI: NOT 0x1234 = 0xEDCB");
}

// ============================================================================
// NOT r/m32
// ============================================================================

#[test]
fn test_not_eax_basic() {
    let code = [0xf7, 0xd0, 0xf4]; // NOT EAX
    let mut regs = Registers::default();
    regs.rax = 0xAAAAAAAA;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x55555555, "EAX: NOT 0xAAAAAAAA = 0x55555555");
}

#[test]
fn test_not_ebx_all_zeros() {
    let code = [0xf7, 0xd3, 0xf4]; // NOT EBX
    let mut regs = Registers::default();
    regs.rbx = 0x00000000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx, 0xFFFFFFFF, "EBX: NOT 0x00000000 = 0xFFFFFFFF");
}

#[test]
fn test_not_ecx_pattern() {
    let code = [0xf7, 0xd1, 0xf4]; // NOT ECX
    let mut regs = Registers::default();
    regs.rcx = 0xFFFF0000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 0x0000FFFF, "ECX: NOT 0xFFFF0000 = 0x0000FFFF");
}

#[test]
fn test_not_esi() {
    let code = [0xf7, 0xd6, 0xf4]; // NOT ESI
    let mut regs = Registers::default();
    regs.rsi = 0x12345678;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rsi, 0xEDCBA987, "ESI: NOT 0x12345678 = 0xEDCBA987");
}

// ============================================================================
// NOT r/m64
// ============================================================================

#[test]
fn test_not_rax_basic() {
    let code = [0x48, 0xf7, 0xd0, 0xf4]; // NOT RAX
    let mut regs = Registers::default();
    regs.rax = 0xAAAAAAAAAAAAAAAA;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x5555555555555555, "RAX: invert all bits");
}

#[test]
fn test_not_rbx_all_zeros() {
    let code = [0x48, 0xf7, 0xd3, 0xf4]; // NOT RBX
    let mut regs = Registers::default();
    regs.rbx = 0x0000000000000000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx, 0xFFFFFFFFFFFFFFFF, "RBX: NOT 0 = all ones");
}

#[test]
fn test_not_rcx_pattern() {
    let code = [0x48, 0xf7, 0xd1, 0xf4]; // NOT RCX
    let mut regs = Registers::default();
    regs.rcx = 0xFFFFFFFF00000000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 0x00000000FFFFFFFF, "RCX: invert pattern");
}

#[test]
fn test_not_rsi() {
    let code = [0x48, 0xf7, 0xd6, 0xf4]; // NOT RSI
    let mut regs = Registers::default();
    regs.rsi = 0x123456789ABCDEF0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rsi, 0xEDCBA9876543210F, "RSI: invert all bits");
}

// ============================================================================
// Extended registers (R8-R15)
// ============================================================================

#[test]
fn test_not_r8b() {
    let code = [0x41, 0xf6, 0xd0, 0xf4]; // NOT R8B
    let mut regs = Registers::default();
    regs.r8 = 0xAA;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.r8 & 0xFF, 0x55, "R8B: NOT 0xAA = 0x55");
}

#[test]
fn test_not_r9w() {
    let code = [0x66, 0x41, 0xf7, 0xd1, 0xf4]; // NOT R9W
    let mut regs = Registers::default();
    regs.r9 = 0xAAAA;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.r9 & 0xFFFF, 0x5555, "R9W: NOT 0xAAAA = 0x5555");
}

#[test]
fn test_not_r10d() {
    let code = [0x41, 0xf7, 0xd2, 0xf4]; // NOT R10D
    let mut regs = Registers::default();
    regs.r10 = 0x12345678;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.r10, 0xEDCBA987, "R10D: invert");
}

#[test]
fn test_not_r11() {
    let code = [0x49, 0xf7, 0xd3, 0xf4]; // NOT R11
    let mut regs = Registers::default();
    regs.r11 = 0x123456789ABCDEF0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.r11, 0xEDCBA9876543210F, "R11: invert all bits");
}

#[test]
fn test_not_r15() {
    let code = [0x49, 0xf7, 0xd7, 0xf4]; // NOT R15
    let mut regs = Registers::default();
    regs.r15 = 0xFFFFFFFF00000000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.r15, 0x00000000FFFFFFFF, "R15: invert");
}

// ============================================================================
// Memory operands
// ============================================================================

#[test]
fn test_not_byte_ptr() {
    let code = [
        0xf6, 0x15, 0xfa, 0x0f, 0x00, 0x00, // NOT BYTE PTR [rip+0x0FFA]
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u8(&mem, 0xAA);

    let _ = run_until_hlt(&mut vcpu).unwrap();
    let result = read_mem_u8(&mem);

    assert_eq!(result, 0x55, "Memory: NOT 0xAA = 0x55");
}

#[test]
fn test_not_word_ptr() {
    let code = [
        0x66, 0xf7, 0x15, 0xf9, 0x0f, 0x00, 0x00, // NOT WORD PTR [rip+0x0FF9]
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u16(&mem, 0xAAAA);

    let _ = run_until_hlt(&mut vcpu).unwrap();
    let result = read_mem_u16(&mem);

    assert_eq!(result, 0x5555, "Memory: NOT word");
}

#[test]
fn test_not_dword_ptr() {
    let code = [
        0xf7, 0x15, 0xfa, 0x0f, 0x00, 0x00, // NOT DWORD PTR [rip+0x0FFA]
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u32(&mem, 0x12345678);

    let _ = run_until_hlt(&mut vcpu).unwrap();
    let result = read_mem_u32(&mem);

    assert_eq!(result, 0xEDCBA987, "Memory: NOT dword");
}

#[test]
fn test_not_qword_ptr() {
    let code = [
        0x48, 0xf7, 0x15, 0xf9, 0x0f, 0x00, 0x00, // NOT QWORD PTR [rip+0x0FF9]
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u64(&mem, 0x123456789ABCDEF0);

    let _ = run_until_hlt(&mut vcpu).unwrap();
    let result = read_mem_u64(&mem);

    assert_eq!(result, 0xEDCBA9876543210F, "Memory: NOT qword");
}

// ============================================================================
// Double NOT (should return original value)
// ============================================================================

#[test]
fn test_not_not_al() {
    let code = [
        0xf6, 0xd0, // NOT AL
        0xf6, 0xd0, // NOT AL
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x42;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x42, "Double NOT returns original");
}

#[test]
fn test_not_not_eax() {
    let code = [
        0xf7, 0xd0, // NOT EAX
        0xf7, 0xd0, // NOT EAX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x12345678;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x12345678, "Double NOT EAX returns original");
}

#[test]
fn test_not_not_rax() {
    let code = [
        0x48, 0xf7, 0xd0, // NOT RAX
        0x48, 0xf7, 0xd0, // NOT RAX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x123456789ABCDEF0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax, 0x123456789ABCDEF0,
        "Double NOT RAX returns original"
    );
}

// ============================================================================
// Verify flags are NOT affected
// ============================================================================

#[test]
fn test_not_preserves_flags() {
    let code = [0xf6, 0xd0, 0xf4]; // NOT AL
    let mut regs = Registers::default();
    regs.rax = 0x00;
    regs.rflags = 0x2 | 0x1 | 0x40 | 0x80 | 0x800; // Set CF, PF, ZF, SF, OF
    let initial_flags = regs.rflags;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rflags & 0x8D5,
        initial_flags & 0x8D5,
        "NOT preserves all flags"
    );
}

// ============================================================================
// Strengthened NOT tests (appended): exact one's-complement results across
// operand sizes (with the 32-bit zero-extension), memory operand, and the
// guarantee that NOT never touches flags.
// ============================================================================

#[test]
fn test_strict_not_r64_exact() {
    // NOT RAX: bitwise complement of a known value.
    let code = [0x48, 0xf7, 0xd0, 0xf4]; // NOT RAX
    let mut regs = Registers::default();
    regs.rax = 0x0123_4567_89AB_CDEF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0xFEDC_BA98_7654_3210, "NOT RAX one's complement");
}

#[test]
fn test_strict_not_r32_zero_extends() {
    // NOT EAX: complement low 32, clear upper 32.
    let code = [0xf7, 0xd0, 0xf4]; // NOT EAX
    let mut regs = Registers::default();
    regs.rax = 0xFFFF_FFFF_0000_FFFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax, 0x0000_0000_FFFF_0000,
        "NOT EAX complements low32 and zero-extends"
    );
}

#[test]
fn test_strict_not_r16_preserves_upper() {
    // NOT AX (0x66): complement low 16, preserve upper 48.
    let code = [0x66, 0xf7, 0xd0, 0xf4]; // NOT AX
    let mut regs = Registers::default();
    regs.rax = 0x1234_5678_9ABC_00FF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax, 0x1234_5678_9ABC_FF00,
        "NOT AX complements only low 16"
    );
}

#[test]
fn test_strict_not_mem64() {
    // NOT qword [RBX]: complement an in-memory value.
    let code = [0x48, 0xf7, 0x13, 0xf4]; // NOT [RBX]
    let mut regs = Registers::default();
    regs.rbx = DATA_ADDR;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    write_mem_at_u64(&mem, DATA_ADDR, 0x0000_FFFF_0000_FFFF);
    let _ = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        read_mem_at_u64(&mem, DATA_ADDR),
        0xFFFF_0000_FFFF_0000,
        "NOT memory operand"
    );
}
