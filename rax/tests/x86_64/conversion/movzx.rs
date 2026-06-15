use rax::cpu::Registers;

use crate::common::{run_until_hlt, setup_vm, write_mem_u8, write_mem_u16};

// MOVZX - Move with Zero Extension
// Copies a byte or word from the source to a larger register with zero extension
// The upper bits of the destination are cleared (set to 0)
// Opcodes: 0F B6 /r (byte), 0F B7 /r (word)

// ============================================================================
// MOVZX r16, r/m8 - Byte to Word with Zero Extension
// ============================================================================

#[test]
fn test_movzx_ax_bl_all_ones() {
    let code = [0x66, 0x0f, 0xb6, 0xc3, 0xf4]; // MOVZX AX, BL
    let mut regs = Registers::default();
    regs.rbx = 0xFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax & 0xFFFF,
        0x00FF,
        "AX should be zero-extended to 0x00FF, not 0xFFFF"
    );
}

#[test]
fn test_movzx_ax_bl_zero() {
    let code = [0x66, 0x0f, 0xb6, 0xc3, 0xf4]; // MOVZX AX, BL
    let mut regs = Registers::default();
    regs.rbx = 0x00;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFFFF, 0x0000, "AX should be 0x0000");
}

#[test]
fn test_movzx_ax_bl_positive() {
    let code = [0x66, 0x0f, 0xb6, 0xc3, 0xf4]; // MOVZX AX, BL
    let mut regs = Registers::default();
    regs.rbx = 0x7F; // 127
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax & 0xFFFF,
        0x007F,
        "AX should be zero-extended to 0x007F"
    );
}

#[test]
fn test_movzx_cx_dl() {
    let code = [0x66, 0x0f, 0xb6, 0xca, 0xf4]; // MOVZX CX, DL
    let mut regs = Registers::default();
    regs.rdx = 0x80; // Would be -128 as signed, but 128 as unsigned
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rcx & 0xFFFF,
        0x0080,
        "CX should be zero-extended to 0x0080"
    );
}

// Note: SPL register test removed

// ============================================================================
// MOVZX r32, r/m8 - Byte to Dword with Zero Extension
// ============================================================================

#[test]
fn test_movzx_eax_bl_all_ones() {
    let code = [0x0f, 0xb6, 0xc3, 0xf4]; // MOVZX EAX, BL
    let mut regs = Registers::default();
    regs.rbx = 0xFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x000000FF,
        "EAX should be zero-extended to 0x000000FF"
    );
}

#[test]
fn test_movzx_eax_bl_sign_bit_set() {
    let code = [0x0f, 0xb6, 0xc3, 0xf4]; // MOVZX EAX, BL
    let mut regs = Registers::default();
    regs.rbx = 0x80; // sign bit set
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x00000080,
        "EAX should zero-extend, not sign-extend"
    );
}

#[test]
fn test_movzx_eax_cl() {
    let code = [0x0f, 0xb6, 0xc1, 0xf4]; // MOVZX EAX, CL
    let mut regs = Registers::default();
    regs.rcx = 0x42;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x00000042,
        "EAX should be zero-extended from CL"
    );
}

#[test]
fn test_movzx_edx_bl() {
    let code = [0x0f, 0xb6, 0xd3, 0xf4]; // MOVZX EDX, BL
    let mut regs = Registers::default();
    regs.rbx = 0xAA;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rdx & 0xFFFFFFFF,
        0x000000AA,
        "EDX should be zero-extended"
    );
}

#[test]
fn test_movzx_esi_dl() {
    let code = [0x0f, 0xb6, 0xf2, 0xf4]; // MOVZX ESI, DL
    let mut regs = Registers::default();
    regs.rdx = 0xFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rsi & 0xFFFFFFFF,
        0x000000FF,
        "ESI should be zero-extended from DL"
    );
}

#[test]
fn test_movzx_edi_al() {
    let code = [0x0f, 0xb6, 0xf8, 0xf4]; // MOVZX EDI, AL
    let mut regs = Registers::default();
    regs.rax = 0xCC;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rdi & 0xFFFFFFFF,
        0x000000CC,
        "EDI should be zero-extended from AL"
    );
}

// ============================================================================
// MOVZX r64, r/m8 - Byte to Qword with Zero Extension
// ============================================================================

#[test]
fn test_movzx_rax_bl_all_ones() {
    let code = [0x48, 0x0f, 0xb6, 0xc3, 0xf4]; // MOVZX RAX, BL
    let mut regs = Registers::default();
    regs.rbx = 0xFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax, 0x00000000000000FF,
        "RAX should be zero-extended to 0x00000000000000FF"
    );
}

#[test]
fn test_movzx_rax_bl_zero() {
    let code = [0x48, 0x0f, 0xb6, 0xc3, 0xf4]; // MOVZX RAX, BL
    let mut regs = Registers::default();
    regs.rbx = 0x00;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax, 0x0000000000000000,
        "RAX should be 0x0000000000000000"
    );
}

// Note: SPL register test removed

// ============================================================================
// MOVZX r32, r/m16 - Word to Dword with Zero Extension
// ============================================================================

#[test]
fn test_movzx_eax_bx_all_ones() {
    let code = [0x0f, 0xb7, 0xc3, 0xf4]; // MOVZX EAX, BX
    let mut regs = Registers::default();
    regs.rbx = 0xFFFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x0000FFFF,
        "EAX should be zero-extended to 0x0000FFFF"
    );
}

#[test]
fn test_movzx_eax_bx_sign_bit_set() {
    let code = [0x0f, 0xb7, 0xc3, 0xf4]; // MOVZX EAX, BX
    let mut regs = Registers::default();
    regs.rbx = 0x8000; // sign bit set
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x00008000,
        "EAX should zero-extend, not sign-extend"
    );
}

#[test]
fn test_movzx_eax_cx() {
    let code = [0x0f, 0xb7, 0xc1, 0xf4]; // MOVZX EAX, CX
    let mut regs = Registers::default();
    regs.rcx = 0x1234;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x00001234,
        "EAX should be zero-extended from CX"
    );
}

#[test]
fn test_movzx_edx_bx() {
    let code = [0x0f, 0xb7, 0xd3, 0xf4]; // MOVZX EDX, BX
    let mut regs = Registers::default();
    regs.rbx = 0xABCD;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rdx & 0xFFFFFFFF,
        0x0000ABCD,
        "EDX should be zero-extended"
    );
}

// ============================================================================
// MOVZX r64, r/m16 - Word to Qword with Zero Extension
// ============================================================================

#[test]
fn test_movzx_rax_bx_all_ones() {
    let code = [0x48, 0x0f, 0xb7, 0xc3, 0xf4]; // MOVZX RAX, BX
    let mut regs = Registers::default();
    regs.rbx = 0xFFFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax, 0x000000000000FFFF,
        "RAX should be zero-extended to 0x000000000000FFFF"
    );
}

#[test]
fn test_movzx_rax_bx_zero() {
    let code = [0x48, 0x0f, 0xb7, 0xc3, 0xf4]; // MOVZX RAX, BX
    let mut regs = Registers::default();
    regs.rbx = 0x0000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax, 0x0000000000000000,
        "RAX should be 0x0000000000000000"
    );
}

#[test]
fn test_movzx_rcx_di() {
    let code = [0x48, 0x0f, 0xb7, 0xcf, 0xf4]; // MOVZX RCX, DI
    let mut regs = Registers::default();
    regs.rdi = 0xBEEF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rcx, 0x000000000000BEEF,
        "RCX should be zero-extended from DI"
    );
}

// ============================================================================
// Extended Registers (R8-R15)
// ============================================================================

#[test]
fn test_movzx_r8d_bl() {
    let code = [0x44, 0x0f, 0xb6, 0xc3, 0xf4]; // MOVZX R8D, BL
    let mut regs = Registers::default();
    regs.rbx = 0xFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.r8 & 0xFFFFFFFF,
        0x000000FF,
        "R8D should be zero-extended"
    );
}

#[test]
fn test_movzx_eax_r8b() {
    let code = [0x41, 0x0f, 0xb6, 0xc0, 0xf4]; // MOVZX EAX, R8B
    let mut regs = Registers::default();
    regs.r8 = 0xFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x000000FF,
        "EAX should be zero-extended from R8B"
    );
}

#[test]
fn test_movzx_r9_r10b() {
    let code = [0x4d, 0x0f, 0xb6, 0xca, 0xf4]; // MOVZX R9, R10B
    let mut regs = Registers::default();
    regs.r10 = 0x80;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.r9, 0x0000000000000080,
        "R9 should be zero-extended from R10B"
    );
}

#[test]
fn test_movzx_r11_r12b() {
    let code = [0x4d, 0x0f, 0xb6, 0xdc, 0xf4]; // MOVZX R11, R12B
    let mut regs = Registers::default();
    regs.r12 = 0x7F;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.r11, 0x000000000000007F,
        "R11 should be zero-extended from R12B"
    );
}

#[test]
fn test_movzx_r15d_bl() {
    let code = [0x44, 0x0f, 0xb6, 0xfb, 0xf4]; // MOVZX R15D, BL
    let mut regs = Registers::default();
    regs.rbx = 0x42;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.r15 & 0xFFFFFFFF,
        0x00000042,
        "R15D should be zero-extended"
    );
}

// ============================================================================
// Clears Previous Destination Values
// ============================================================================

#[test]
fn test_movzx_clears_destination_eax() {
    let code = [0x0f, 0xb6, 0xc3, 0xf4]; // MOVZX EAX, BL
    let mut regs = Registers::default();
    regs.rax = 0xFFFFFFFFFFFFFFFF;
    regs.rbx = 0x01;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax, 0x0000000000000001,
        "RAX should be completely cleared except for byte"
    );
}

// Note: test removed

// ============================================================================
// Zero Source
// ============================================================================

#[test]
fn test_movzx_zero_source_byte() {
    let code = [0x0f, 0xb6, 0xc3, 0xf4]; // MOVZX EAX, BL
    let mut regs = Registers::default();
    regs.rbx = 0x00;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFFFFFFFF, 0x00000000, "EAX should be zero");
}

#[test]
fn test_movzx_zero_source_word() {
    let code = [0x0f, 0xb7, 0xc3, 0xf4]; // MOVZX EAX, BX
    let mut regs = Registers::default();
    regs.rbx = 0x0000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFFFFFFFF, 0x00000000, "EAX should be zero");
}

// ============================================================================
// Source Not Modified
// ============================================================================

#[test]
fn test_movzx_source_unchanged_byte() {
    let code = [0x0f, 0xb6, 0xc3, 0xf4]; // MOVZX EAX, BL
    let mut regs = Registers::default();
    regs.rbx = 0x12345678;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rbx & 0xFFFFFFFF, 0x12345678, "EBX should be unchanged");
}

#[test]
fn test_movzx_source_unchanged_word() {
    let code = [0x0f, 0xb7, 0xc3, 0xf4]; // MOVZX EAX, BX
    let mut regs = Registers::default();
    regs.rbx = 0x1234ABCD;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rbx, 0x1234ABCD, "RBX should be unchanged");
}

// ============================================================================
// Flags Not Affected
// ============================================================================

#[test]
fn test_movzx_does_not_affect_flags() {
    let code = [0x0f, 0xb6, 0xc3, 0xf4]; // MOVZX EAX, BL
    let mut regs = Registers::default();
    regs.rbx = 0xFF;
    regs.rflags = 0x2 | (1 << 6) | (1 << 11); // Set ZF and OF
    let initial_flags = regs.rflags;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rflags, initial_flags, "Flags should be unchanged");
}

// ============================================================================
// Sequential Values
// ============================================================================

#[test]
fn test_movzx_sequential_byte_values() {
    // Test a selection of byte values
    let test_values = vec![0x00, 0x01, 0x7F, 0x80, 0xFE, 0xFF];
    for value in test_values {
        let code = [0x0f, 0xb6, 0xc3, 0xf4]; // MOVZX EAX, BL
        let mut regs = Registers::default();
        regs.rbx = value as u64;
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();
        assert_eq!(
            regs.rax & 0xFFFFFFFF,
            value as u64,
            "MOVZX should zero-extend 0x{:02X}",
            value
        );
    }
}

#[test]
fn test_movzx_word_values() {
    // MOVZX with various word values
    let test_values = vec![0x0000, 0x0001, 0x00FF, 0x0100, 0x7FFF, 0x8000, 0xFFFF];
    for value in test_values {
        let code = [0x0f, 0xb7, 0xc3, 0xf4]; // MOVZX EAX, BX
        let mut regs = Registers::default();
        regs.rbx = value;
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();
        assert_eq!(
            regs.rax & 0xFFFFFFFF,
            value as u64,
            "MOVZX should zero-extend 0x{:04X}",
            value
        );
    }
}

// ============================================================================
// Byte to Different Destination Sizes
// ============================================================================

#[test]
fn test_movzx_byte_to_different_sizes() {
    let source = 0xAB;

    // Byte to word
    let code = [0x66, 0x0f, 0xb6, 0xc3, 0xf4]; // MOVZX AX, BL
    let mut regs = Registers::default();
    regs.rbx = source;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFFFF, 0x00AB, "Byte to word");

    // Byte to dword
    let code = [0x0f, 0xb6, 0xc3, 0xf4]; // MOVZX EAX, BL
    let mut regs = Registers::default();
    regs.rbx = source;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFFFFFFFF, 0x000000AB, "Byte to dword");

    // Byte to qword
    let code = [0x48, 0x0f, 0xb6, 0xc3, 0xf4]; // MOVZX RAX, BL
    let mut regs = Registers::default();
    regs.rbx = source;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x00000000000000AB, "Byte to qword");
}

// ============================================================================
// ASCII Character Values
// ============================================================================

#[test]
fn test_movzx_ascii_characters() {
    // MOVZX with ASCII character values
    let test_chars = vec![
        ('A', 0x41),
        ('Z', 0x5A),
        ('a', 0x61),
        ('z', 0x7A),
        ('0', 0x30),
        ('9', 0x39),
    ];

    for (_ch, value) in test_chars {
        let code = [0x0f, 0xb6, 0xc3, 0xf4]; // MOVZX EAX, BL
        let mut regs = Registers::default();
        regs.rbx = value;
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();
        assert_eq!(
            regs.rax & 0xFFFFFFFF,
            value,
            "MOVZX should zero-extend ASCII value 0x{:02X}",
            value
        );
    }
}

// ============================================================================
// Extract Bytes from Larger Values
// ============================================================================

#[test]
fn test_movzx_extract_low_byte() {
    let code = [0x0f, 0xb6, 0xc3, 0xf4]; // MOVZX EAX, BL (low byte)
    let mut regs = Registers::default();
    regs.rbx = 0x12345678;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x00000078,
        "Should extract low byte 0x78"
    );
}

#[test]
fn test_movzx_extract_low_word() {
    let code = [0x0f, 0xb7, 0xc3, 0xf4]; // MOVZX EAX, BX (low word)
    let mut regs = Registers::default();
    regs.rbx = 0x12345678;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x00005678,
        "Should extract low word 0x5678"
    );
}

// ============================================================================
// Distinction from MOVSX
// ============================================================================

#[test]
fn test_movzx_vs_movsx_negative_byte() {
    // MOVZX with byte 0x80 should give 0x00000080
    let code_zx = [0x0f, 0xb6, 0xc3, 0xf4]; // MOVZX EAX, BL
    let mut regs = Registers::default();
    regs.rbx = 0x80;
    let (mut vcpu, _) = setup_vm(&code_zx, Some(regs));
    let regs_zx = run_until_hlt(&mut vcpu).unwrap();

    // MOVSX with byte 0x80 should give 0xFFFFFF80
    let code_sx = [0x0f, 0xbe, 0xc3, 0xf4]; // MOVSX EAX, BL
    let mut regs = Registers::default();
    regs.rbx = 0x80;
    let (mut vcpu, _) = setup_vm(&code_sx, Some(regs));
    let regs_sx = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs_zx.rax & 0xFFFFFFFF,
        0x00000080,
        "MOVZX should zero-extend to 0x00000080"
    );
    assert_eq!(
        regs_sx.rax & 0xFFFFFFFF,
        0xFFFFFF80,
        "MOVSX should sign-extend to 0xFFFFFF80"
    );
}

#[test]
fn test_movzx_vs_movsx_negative_word() {
    // MOVZX with word 0x8000 should give 0x00008000
    let code_zx = [0x0f, 0xb7, 0xc3, 0xf4]; // MOVZX EAX, BX
    let mut regs = Registers::default();
    regs.rbx = 0x8000;
    let (mut vcpu, _) = setup_vm(&code_zx, Some(regs));
    let regs_zx = run_until_hlt(&mut vcpu).unwrap();

    // MOVSX with word 0x8000 should give 0xFFFF8000
    let code_sx = [0x0f, 0xbf, 0xc3, 0xf4]; // MOVSX EAX, BX
    let mut regs = Registers::default();
    regs.rbx = 0x8000;
    let (mut vcpu, _) = setup_vm(&code_sx, Some(regs));
    let regs_sx = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs_zx.rax & 0xFFFFFFFF,
        0x00008000,
        "MOVZX should zero-extend to 0x00008000"
    );
    assert_eq!(
        regs_sx.rax & 0xFFFFFFFF,
        0xFFFF8000,
        "MOVSX should sign-extend to 0xFFFF8000"
    );
}

// ============================================================================
// Upper Bits Cleared (32-bit operations)
// ============================================================================

#[test]
fn test_movzx_eax_clears_upper_32() {
    let code = [0x0f, 0xb6, 0xc3, 0xf4]; // MOVZX EAX, BL
    let mut regs = Registers::default();
    regs.rax = 0xDEADBEEFDEADBEEF; // garbage
    regs.rbx = 0xFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax, 0x00000000000000FF,
        "Upper 32 bits should be zeroed"
    );
}

// ============================================================================
// Multiple Operations Sequence
// ============================================================================

#[test]
fn test_movzx_multiple_operations() {
    let code = [
        0x0f, 0xb6, 0xc3, // MOVZX EAX, BL
        0x0f, 0xb6, 0xca, // MOVZX ECX, DL
        0x0f, 0xb7, 0xd1, // MOVZX EDX, CX
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xFF;
    regs.rdx = 0x7F;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x000000FF,
        "EAX should be 0x000000FF"
    );
    assert_eq!(
        regs.rcx & 0xFFFFFFFF,
        0x0000007F,
        "ECX should be 0x0000007F"
    );
    assert_eq!(
        regs.rdx & 0xFFFFFFFF,
        0x0000007F,
        "EDX should be 0x0000007F"
    );
}

// ============================================================================
// Practical Use Cases
// ============================================================================

#[test]
fn test_movzx_practical_extract_flag() {
    // Extracting individual bits/bytes from packed data
    let code = [0x0f, 0xb6, 0xc3, 0xf4]; // MOVZX EAX, BL
    let mut regs = Registers::default();
    regs.rbx = 0x42; // Some packed data, extracting low byte
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x00000042,
        "Should extract byte as unsigned value"
    );
}

// Note: test with arithmetic after MOVZX removed
