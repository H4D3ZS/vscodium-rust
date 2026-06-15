use rax::cpu::Registers;

use crate::common::{run_until_hlt, setup_vm};

// MOVSX - Move with Sign-Extension
// Copies a byte or word to a larger register with sign extension

// Basic byte to word/dword/qword sign extension
#[test]
fn test_movsx_ax_bl() {
    let code = [0x66, 0x0f, 0xbe, 0xc3, 0xf4]; // MOVSX AX, BL
    let mut regs = Registers::default();
    regs.rbx = 0xFF; // -1 in signed byte
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax & 0xFFFF,
        0xFFFF,
        "AX should be sign-extended to 0xFFFF"
    );
}

#[test]
fn test_movsx_eax_bl() {
    let code = [0x0f, 0xbe, 0xc3, 0xf4]; // MOVSX EAX, BL
    let mut regs = Registers::default();
    regs.rbx = 0xFF; // -1 in signed byte
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0xFFFFFFFF,
        "EAX should be sign-extended to 0xFFFFFFFF"
    );
}

#[test]
fn test_movsx_rax_bl() {
    let code = [0x48, 0x0f, 0xbe, 0xc3, 0xf4]; // MOVSX RAX, BL
    let mut regs = Registers::default();
    regs.rbx = 0xFF; // -1 in signed byte
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax, 0xFFFFFFFFFFFFFFFF,
        "RAX should be sign-extended to all 1s"
    );
}

// Test positive byte (sign bit clear)
#[test]
fn test_movsx_eax_bl_positive() {
    let code = [0x0f, 0xbe, 0xc3, 0xf4]; // MOVSX EAX, BL
    let mut regs = Registers::default();
    regs.rbx = 0x7F; // 127 - positive
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x0000007F,
        "EAX should be sign-extended to 0x7F"
    );
}

// Test negative byte (sign bit set)
#[test]
fn test_movsx_eax_bl_negative() {
    let code = [0x0f, 0xbe, 0xc3, 0xf4]; // MOVSX EAX, BL
    let mut regs = Registers::default();
    regs.rbx = 0x80; // -128 in signed byte
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0xFFFFFF80,
        "EAX should be sign-extended to 0xFFFFFF80"
    );
}

// Word to dword/qword sign extension
#[test]
fn test_movsx_eax_bx() {
    let code = [0x0f, 0xbf, 0xc3, 0xf4]; // MOVSX EAX, BX
    let mut regs = Registers::default();
    regs.rbx = 0xFFFF; // -1 in signed word
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0xFFFFFFFF,
        "EAX should be sign-extended to 0xFFFFFFFF"
    );
}

#[test]
fn test_movsx_rax_bx() {
    let code = [0x48, 0x0f, 0xbf, 0xc3, 0xf4]; // MOVSX RAX, BX
    let mut regs = Registers::default();
    regs.rbx = 0xFFFF; // -1 in signed word
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax, 0xFFFFFFFFFFFFFFFF,
        "RAX should be sign-extended to all 1s"
    );
}

#[test]
fn test_movsx_eax_bx_positive() {
    let code = [0x0f, 0xbf, 0xc3, 0xf4]; // MOVSX EAX, BX
    let mut regs = Registers::default();
    regs.rbx = 0x7FFF; // 32767 - positive
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x00007FFF,
        "EAX should be sign-extended to 0x7FFF"
    );
}

#[test]
fn test_movsx_eax_bx_negative() {
    let code = [0x0f, 0xbf, 0xc3, 0xf4]; // MOVSX EAX, BX
    let mut regs = Registers::default();
    regs.rbx = 0x8000; // -32768 in signed word
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0xFFFF8000,
        "EAX should be sign-extended to 0xFFFF8000"
    );
}

// MOVSXD - Move with sign extension (dword to qword)
#[test]
fn test_movsxd_rax_ebx() {
    let code = [0x48, 0x63, 0xc3, 0xf4]; // MOVSXD RAX, EBX
    let mut regs = Registers::default();
    regs.rbx = 0xFFFFFFFF; // -1 in signed dword
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax, 0xFFFFFFFFFFFFFFFF,
        "RAX should be sign-extended to all 1s"
    );
}

#[test]
fn test_movsxd_rax_ebx_positive() {
    let code = [0x48, 0x63, 0xc3, 0xf4]; // MOVSXD RAX, EBX
    let mut regs = Registers::default();
    regs.rbx = 0x7FFFFFFF; // max positive signed dword
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax, 0x000000007FFFFFFF,
        "RAX should be sign-extended to 0x7FFFFFFF"
    );
}

#[test]
fn test_movsxd_rax_ebx_negative() {
    let code = [0x48, 0x63, 0xc3, 0xf4]; // MOVSXD RAX, EBX
    let mut regs = Registers::default();
    regs.rbx = 0x80000000; // min negative signed dword
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax, 0xFFFFFFFF80000000,
        "RAX should be sign-extended to 0xFFFFFFFF80000000"
    );
}

// Test with different source registers
#[test]
fn test_movsx_eax_cl() {
    let code = [0x0f, 0xbe, 0xc1, 0xf4]; // MOVSX EAX, CL
    let mut regs = Registers::default();
    regs.rcx = 0x80;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0xFFFFFF80,
        "EAX should be sign-extended from CL"
    );
}

#[test]
fn test_movsx_eax_dl() {
    let code = [0x0f, 0xbe, 0xc2, 0xf4]; // MOVSX EAX, DL
    let mut regs = Registers::default();
    regs.rdx = 0x7F;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x0000007F,
        "EAX should be sign-extended from DL"
    );
}

// Test with extended registers (R8-R15)
#[test]
fn test_movsx_r8d_bl() {
    let code = [0x44, 0x0f, 0xbe, 0xc3, 0xf4]; // MOVSX R8D, BL
    let mut regs = Registers::default();
    regs.rbx = 0x80;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.r8 & 0xFFFFFFFF,
        0xFFFFFF80,
        "R8D should be sign-extended"
    );
}

#[test]
fn test_movsx_eax_r8b() {
    let code = [0x41, 0x0f, 0xbe, 0xc0, 0xf4]; // MOVSX EAX, R8B
    let mut regs = Registers::default();
    regs.r8 = 0xFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0xFFFFFFFF,
        "EAX should be sign-extended from R8B"
    );
}

#[test]
fn test_movsx_r9_r10b() {
    let code = [0x4d, 0x0f, 0xbe, 0xca, 0xf4]; // MOVSX R9, R10B
    let mut regs = Registers::default();
    regs.r10 = 0x80;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.r9, 0xFFFFFFFFFFFFFF80,
        "R9 should be sign-extended from R10B"
    );
}

// Test zero value
#[test]
fn test_movsx_eax_bl_zero() {
    let code = [0x0f, 0xbe, 0xc3, 0xf4]; // MOVSX EAX, BL
    let mut regs = Registers::default();
    regs.rbx = 0x00;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFFFFFFFF, 0x00000000, "EAX should be 0");
}

// Test boundary values
#[test]
fn test_movsx_byte_boundary_0x7f() {
    let code = [0x0f, 0xbe, 0xc3, 0xf4]; // MOVSX EAX, BL
    let mut regs = Registers::default();
    regs.rbx = 0x7F; // max positive signed byte
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFFFFFFFF, 0x0000007F, "EAX should be 0x7F");
}

#[test]
fn test_movsx_word_boundary_0x7fff() {
    let code = [0x0f, 0xbf, 0xc3, 0xf4]; // MOVSX EAX, BX
    let mut regs = Registers::default();
    regs.rbx = 0x7FFF; // max positive signed word
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFFFFFFFF, 0x00007FFF, "EAX should be 0x7FFF");
}

// Test that it differs from MOVZX
#[test]
fn test_movsx_vs_movzx_behavior() {
    // MOVSX with negative byte
    let code = [0x0f, 0xbe, 0xc3, 0xf4]; // MOVSX EAX, BL
    let mut regs = Registers::default();
    regs.rbx = 0x80; // -128 signed, 128 unsigned
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0xFFFFFF80,
        "MOVSX should sign-extend to 0xFFFFFF80"
    );
    // MOVZX would produce 0x00000080 instead
}

// Test sequential values to verify sign extension behavior
#[test]
fn test_movsx_byte_values_127_to_128() {
    // Value 127 (0x7F) - positive
    let code = [0x0f, 0xbe, 0xc3, 0xf4]; // MOVSX EAX, BL
    let mut regs = Registers::default();
    regs.rbx = 0x7F;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x0000007F,
        "127 should sign-extend to 0x0000007F"
    );

    // Value 128 (0x80) - negative
    let code = [0x0f, 0xbe, 0xc3, 0xf4]; // MOVSX EAX, BL
    let mut regs = Registers::default();
    regs.rbx = 0x80;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0xFFFFFF80,
        "128 should sign-extend to 0xFFFFFF80 (-128)"
    );
}

// Test with garbage in upper bits of source register
#[test]
fn test_movsx_ignores_upper_bits() {
    let code = [0x0f, 0xbe, 0xc3, 0xf4]; // MOVSX EAX, BL
    let mut regs = Registers::default();
    regs.rbx = 0xDEADBEEF000000FF; // garbage in upper bits, 0xFF in BL
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0xFFFFFFFF,
        "Should only use BL, ignoring upper bits"
    );
}

// Test different register combinations for word extension
#[test]
fn test_movsx_edx_cx() {
    let code = [0x0f, 0xbf, 0xd1, 0xf4]; // MOVSX EDX, CX
    let mut regs = Registers::default();
    regs.rcx = 0x8000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rdx & 0xFFFFFFFF,
        0xFFFF8000,
        "EDX should be sign-extended from CX"
    );
}

#[test]
fn test_movsx_esi_di() {
    let code = [0x0f, 0xbf, 0xf7, 0xf4]; // MOVSX ESI, DI
    let mut regs = Registers::default();
    regs.rdi = 0x7FFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rsi & 0xFFFFFFFF,
        0x00007FFF,
        "ESI should be sign-extended from DI"
    );
}

// Test MOVSXD with different values
#[test]
fn test_movsxd_small_positive() {
    let code = [0x48, 0x63, 0xc3, 0xf4]; // MOVSXD RAX, EBX
    let mut regs = Registers::default();
    regs.rbx = 0x00000001;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x0000000000000001, "RAX should be 1");
}

#[test]
fn test_movsxd_small_negative() {
    let code = [0x48, 0x63, 0xc3, 0xf4]; // MOVSXD RAX, EBX
    let mut regs = Registers::default();
    regs.rbx = 0xFFFFFFFF; // -1 as 32-bit
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0xFFFFFFFFFFFFFFFF, "RAX should be -1 as 64-bit");
}

// Test register-to-register with all byte registers
#[test]
fn test_movsx_rax_spl() {
    let code = [0x48, 0x0f, 0xbe, 0xc4, 0xf4]; // MOVSX RAX, SPL
    let mut regs = Registers::default();
    regs.rsp = 0xFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax, 0xFFFFFFFFFFFFFFFF,
        "RAX should be sign-extended from SPL"
    );
}

#[test]
fn test_movsx_rbx_bpl() {
    let code = [0x48, 0x0f, 0xbe, 0xdd, 0xf4]; // MOVSX RBX, BPL
    let mut regs = Registers::default();
    regs.rbp = 0x7F;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rbx, 0x000000000000007F,
        "RBX should be sign-extended from BPL"
    );
}

// Test that 32-bit operations zero upper 32 bits
#[test]
fn test_movsx_eax_bl_zeros_upper_32() {
    let code = [0x0f, 0xbe, 0xc3, 0xf4]; // MOVSX EAX, BL
    let mut regs = Registers::default();
    regs.rax = 0xDEADBEEFDEADBEEF; // Set RAX to all garbage
    regs.rbx = 0x7F; // Positive byte
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax, 0x000000000000007F,
        "Upper 32 bits of RAX should be zeroed"
    );
}

#[test]
fn test_movsx_eax_bl_zeros_upper_32_negative() {
    let code = [0x0f, 0xbe, 0xc3, 0xf4]; // MOVSX EAX, BL
    let mut regs = Registers::default();
    regs.rax = 0xDEADBEEFDEADBEEF; // Set RAX to all garbage
    regs.rbx = 0x80; // Negative byte
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax, 0x00000000FFFFFF80,
        "Upper 32 bits should be zeroed, lower 32 sign-extended"
    );
}

// Test flags are not affected
#[test]
fn test_movsx_preserves_flags() {
    let code = [
        0x48, 0xc7, 0xc0, 0xff, 0xff, 0xff, 0xff, // MOV RAX, -1
        0x48, 0x83, 0xc0, 0x01, // ADD RAX, 1 (sets ZF)
        0x0f, 0xbe, 0xc3, // MOVSX EAX, BL
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x80;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert!(
        regs.rflags & 0x40 != 0,
        "ZF should still be set after MOVSX"
    );
}

// Test practical use case: loading signed char into int
#[test]
fn test_movsx_practical_char_to_int() {
    let code = [0x0f, 0xbe, 0xc3, 0xf4]; // MOVSX EAX, BL
    let mut regs = Registers::default();
    regs.rbx = 0xF0; // -16 as signed char
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0xFFFFFFF0,
        "EAX should represent -16 as 32-bit signed"
    );
}

// Test practical use case: loading signed short into long
#[test]
fn test_movsx_practical_short_to_long() {
    let code = [0x48, 0x0f, 0xbf, 0xc3, 0xf4]; // MOVSX RAX, BX
    let mut regs = Registers::default();
    regs.rbx = 0x8001; // -32767 as signed short
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax, 0xFFFFFFFFFFFF8001,
        "RAX should represent -32767 as 64-bit signed"
    );
}

// ============================================================================
// Strengthened MOVSX/MOVSXD tests (appended): exact extended values for each
// source/dest size combination plus the full-RAX result, and flag-neutrality.
// ============================================================================

#[test]
fn test_strict_movsx_r32_r8_negative_full_rax() {
    // MOVSX EAX, BL: BL=0x80 -> EAX=0xFFFFFF80, RAX upper 32 zeroed.
    let code = [0x0f, 0xbe, 0xc3, 0xf4]; // MOVSX EAX, BL
    let mut regs = Registers::default();
    regs.rax = 0xFFFF_FFFF_FFFF_FFFF;
    regs.rbx = 0x80;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax, 0x0000_0000_FFFF_FF80,
        "EAX sign-extend of 0x80, upper RAX cleared"
    );
}

#[test]
fn test_strict_movsx_r64_r8_negative() {
    // MOVSX RAX, BL: BL=0xFF (-1) -> RAX=all ones.
    let code = [0x48, 0x0f, 0xbe, 0xc3, 0xf4]; // MOVSX RAX, BL
    let mut regs = Registers::default();
    regs.rbx = 0xFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0xFFFF_FFFF_FFFF_FFFF, "MOVSX RAX,BL of -1");
}

#[test]
fn test_strict_movsx_r64_r16_positive() {
    // MOVSX RAX, BX: BX=0x7FFF (max positive) -> 0x0000_0000_0000_7FFF.
    let code = [0x48, 0x0f, 0xbf, 0xc3, 0xf4]; // MOVSX RAX, BX
    let mut regs = Registers::default();
    regs.rax = 0xFFFF_FFFF_FFFF_FFFF;
    regs.rbx = 0x7FFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax, 0x0000_0000_0000_7FFF,
        "positive 16-bit sign-extend"
    );
}

#[test]
fn test_strict_movsx_r16_r8_preserves_upper() {
    // MOVSX AX, BL (operand-size 0x66): only AX is written; upper 48 bits preserved.
    let code = [0x66, 0x0f, 0xbe, 0xc3, 0xf4]; // MOVSX AX, BL
    let mut regs = Registers::default();
    regs.rax = 0x1234_5678_9ABC_DEF0;
    regs.rbx = 0x80; // -128 -> 0xFF80
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax, 0x1234_5678_9ABC_FF80,
        "AX sign-extended, upper 48 preserved"
    );
}

#[test]
fn test_strict_movsxd_from_mem() {
    // MOVSXD RAX, dword [RBX]: load 0xFFFFFFFE (-2) from memory, sign-extend.
    let code = [0x48, 0x63, 0x03, 0xf4]; // MOVSXD RAX, [RBX]
    let mut regs = Registers::default();
    regs.rbx = crate::common::DATA_ADDR;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    crate::common::write_mem_at_u32(&mem, crate::common::DATA_ADDR, 0xFFFF_FFFE);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax, 0xFFFF_FFFF_FFFF_FFFE,
        "MOVSXD from memory sign-extends"
    );
}

#[test]
fn test_strict_movsx_does_not_touch_flags() {
    let code = [0x48, 0x0f, 0xbe, 0xc3, 0xf4]; // MOVSX RAX, BL
    let mut regs = Registers::default();
    regs.rbx = 0xFF;
    regs.rflags = 0x2 | 0x1 | 0x40 | 0x800;
    let before = regs.rflags;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0xFFFF_FFFF_FFFF_FFFF);
    assert_eq!(
        regs.rflags & 0x8D5,
        before & 0x8D5,
        "MOVSX must not alter status flags"
    );
}
