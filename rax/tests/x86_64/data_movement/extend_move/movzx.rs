use crate::common::*;
use rax::cpu::Registers;

// MOVZX - Move with Zero-Extend
// Moves a byte or word from the source operand to a larger register,
// zero-extending the upper bits of the destination.
// The source can be a register or memory location.
// The destination must be a register.
//
// Opcodes:
// 0F B6 /r    MOVZX r16, r/m8     - Move byte to word with zero-extension
// 0F B6 /r    MOVZX r32, r/m8     - Move byte to doubleword with zero-extension
// REX.W 0F B6 /r MOVZX r64, r/m8  - Move byte to quadword with zero-extension
// 0F B7 /r    MOVZX r32, r/m16    - Move word to doubleword with zero-extension
// REX.W 0F B7 /r MOVZX r64, r/m16 - Move word to quadword with zero-extension

#[test]
fn test_movzx_ax_bl() {
    // MOVZX AX, BL - byte to word
    let code = [
        0x66, 0x0f, 0xb6, 0xc3, // MOVZX AX, BL
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFF, 0x00FF, "AX should be zero-extended");
}

#[test]
fn test_movzx_eax_bl() {
    // MOVZX EAX, BL - byte to doubleword
    let code = [
        0x0f, 0xb6, 0xc3, // MOVZX EAX, BL
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x000000FF,
        "EAX should be zero-extended"
    );
}

#[test]
fn test_movzx_rax_bl() {
    // MOVZX RAX, BL - byte to quadword
    let code = [
        0x48, 0x0f, 0xb6, 0xc3, // MOVZX RAX, BL
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x00000000000000FF, "RAX should be zero-extended");
}

#[test]
fn test_movzx_eax_bx() {
    // MOVZX EAX, BX - word to doubleword
    let code = [
        0x0f, 0xb7, 0xc3, // MOVZX EAX, BX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xFFFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x0000FFFF,
        "EAX should be zero-extended"
    );
}

#[test]
fn test_movzx_rax_bx() {
    // MOVZX RAX, BX - word to quadword
    let code = [
        0x48, 0x0f, 0xb7, 0xc3, // MOVZX RAX, BX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xFFFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x000000000000FFFF, "RAX should be zero-extended");
}

#[test]
fn test_movzx_byte_sign_bit_set() {
    // MOVZX with source sign bit set (should still zero-extend)
    let code = [
        0x0f, 0xb6, 0xc3, // MOVZX EAX, BL
        0xf4,
    ];
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
fn test_movzx_word_sign_bit_set() {
    // MOVZX with word source sign bit set
    let code = [
        0x0f, 0xb7, 0xc3, // MOVZX EAX, BX
        0xf4,
    ];
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
fn test_movzx_from_memory_byte() {
    // MOVZX EAX, byte [mem]
    let code = [
        0x0f, 0xb6, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // MOVZX EAX, byte [DATA_ADDR]
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u8(&mem, 0xAB);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x000000AB,
        "EAX should contain zero-extended byte"
    );
}

#[test]
fn test_movzx_from_memory_word() {
    // MOVZX EAX, word [mem]
    let code = [
        0x0f, 0xb7, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // MOVZX EAX, word [DATA_ADDR]
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u16(&mem, 0xABCD);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x0000ABCD,
        "EAX should contain zero-extended word"
    );
}

#[test]
fn test_movzx_with_extended_registers() {
    // MOVZX R8D, R9B
    let code = [
        0x45, 0x0f, 0xb6, 0xc1, // MOVZX R8D, R9B
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.r9 = 0xFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.r8 & 0xFFFFFFFF,
        0x000000FF,
        "R8D should be zero-extended"
    );
}

#[test]
fn test_movzx_r15_byte() {
    // MOVZX R15, byte
    let code = [
        0x4c, 0x0f, 0xb6, 0xfb, // MOVZX R15, BL
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x42;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.r15, 0x0000000000000042, "R15 should be zero-extended");
}

#[test]
fn test_movzx_clears_destination() {
    // MOVZX clears previous destination value
    let code = [
        0x0f, 0xb6, 0xc3, // MOVZX EAX, BL
        0xf4,
    ];
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

#[test]
fn test_movzx_zero_source() {
    // MOVZX with zero source
    let code = [
        0x0f, 0xb6, 0xc3, // MOVZX EAX, BL
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x00;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 0x00000000, "EAX should be zero");
}

#[test]
fn test_movzx_preserves_source() {
    // MOVZX should not modify source
    let code = [
        0x0f, 0xb6, 0xc3, // MOVZX EAX, BL
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x12345678;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx & 0xFFFFFFFF, 0x12345678, "EBX should be unchanged");
}

#[test]
fn test_movzx_sequential_values() {
    // MOVZX with sequential byte values
    for i in 0..=255u8 {
        let code = [
            0x0f, 0xb6, 0xc3, // MOVZX EAX, BL
            0xf4,
        ];
        let mut regs = Registers::default();
        regs.rbx = i as u64;
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        assert_eq!(
            regs.rax & 0xFFFFFFFF,
            i as u64,
            "MOVZX should zero-extend {}",
            i
        );
    }
}

#[test]
fn test_movzx_word_values() {
    // MOVZX with various word values
    let test_values = vec![0x0000, 0x0001, 0x00FF, 0x0100, 0x7FFF, 0x8000, 0xFFFF];

    for value in test_values {
        let code = [
            0x0f, 0xb7, 0xc3, // MOVZX EAX, BX
            0xf4,
        ];
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

#[test]
fn test_movzx_does_not_affect_flags() {
    // MOVZX should not modify flags
    let code = [
        0x0f, 0xb6, 0xc3, // MOVZX EAX, BL
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xFF;
    regs.rflags = 0x2 | (1 << 6) | (1 << 11); // Set ZF and OF
    let initial_flags = regs.rflags;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rflags, initial_flags, "Flags should be unchanged");
}

#[test]
fn test_movzx_byte_to_different_sizes() {
    // MOVZX byte to different destination sizes
    let source = 0xAB;

    // Byte to word
    let code = [
        0x66, 0x0f, 0xb6, 0xc3, // MOVZX AX, BL
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = source;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFFFF, 0x00AB, "Byte to word");

    // Byte to dword
    let code = [
        0x0f, 0xb6, 0xc3, // MOVZX EAX, BL
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = source;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFFFFFFFF, 0x000000AB, "Byte to dword");

    // Byte to qword
    let code = [
        0x48, 0x0f, 0xb6, 0xc3, // MOVZX RAX, BL
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = source;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x00000000000000AB, "Byte to qword");
}

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

    for (ch, value) in test_chars {
        let code = [
            0x0f, 0xb6, 0xc3, // MOVZX EAX, BL
            0xf4,
        ];
        let mut regs = Registers::default();
        regs.rbx = value;
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        assert_eq!(
            regs.rax & 0xFFFFFFFF,
            value,
            "MOVZX should zero-extend ASCII '{}'",
            ch
        );
    }
}

#[test]
fn test_movzx_high_low_bytes() {
    // MOVZX can extract individual bytes
    let code = [
        0x0f, 0xb6, 0xc3, // MOVZX EAX, BL (low byte)
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x12345678;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 0x00000078, "Should extract low byte");
}

#[test]
fn test_movzx_memory_byte_negative() {
    // MOVZX from memory with "negative" byte (MSB set)
    let code = [
        0x0f, 0xb6, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // MOVZX EAX, byte [DATA_ADDR]
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u8(&mem, 0xFF);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x000000FF,
        "Should zero-extend, not sign-extend"
    );
}

#[test]
fn test_movzx_memory_word_negative() {
    // MOVZX from memory with "negative" word
    let code = [
        0x0f, 0xb7, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // MOVZX EAX, word [DATA_ADDR]
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u16(&mem, 0xFFFF);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x0000FFFF,
        "Should zero-extend, not sign-extend"
    );
}

// ============================================================================
// Strengthened MOVZX tests (appended): exact full-RAX zero-extended results
// for each source/dest combination, upper-bit clearing, and flag-neutrality.
// ============================================================================

#[test]
fn test_strict_movzx_r32_r8_clears_full_rax() {
    // MOVZX EAX, BL: BL=0xFF -> EAX=0x000000FF, RAX upper 32 cleared.
    let code = [0x0f, 0xb6, 0xc3, 0xf4]; // MOVZX EAX, BL
    let mut regs = Registers::default();
    regs.rax = 0xFFFF_FFFF_FFFF_FFFF;
    regs.rbx = 0xFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax, 0x0000_0000_0000_00FF,
        "MOVZX EAX,BL zero-extends and clears upper RAX"
    );
}

#[test]
fn test_strict_movzx_r64_r8() {
    // MOVZX RAX, BL: REX.W with 0F B6.
    let code = [0x48, 0x0f, 0xb6, 0xc3, 0xf4]; // MOVZX RAX, BL
    let mut regs = Registers::default();
    regs.rax = 0xFFFF_FFFF_FFFF_FFFF;
    regs.rbx = 0xAB;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x0000_0000_0000_00AB, "MOVZX RAX,BL");
}

#[test]
fn test_strict_movzx_r64_r16() {
    // MOVZX RAX, BX: 0F B7 with REX.W.
    let code = [0x48, 0x0f, 0xb7, 0xc3, 0xf4]; // MOVZX RAX, BX
    let mut regs = Registers::default();
    regs.rax = 0xFFFF_FFFF_FFFF_FFFF;
    regs.rbx = 0x8000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax, 0x0000_0000_0000_8000,
        "MOVZX RAX,BX zero-extends 0x8000"
    );
}

#[test]
fn test_strict_movzx_r16_r8_preserves_upper() {
    // MOVZX AX, BL (operand-size 0x66): only AX written, upper 48 preserved.
    let code = [0x66, 0x0f, 0xb6, 0xc3, 0xf4]; // MOVZX AX, BL
    let mut regs = Registers::default();
    regs.rax = 0x1234_5678_9ABC_DEF0;
    regs.rbx = 0xFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax, 0x1234_5678_9ABC_00FF,
        "AX zero-extended, upper 48 preserved"
    );
}

#[test]
fn test_strict_movzx_from_mem_byte() {
    // MOVZX RAX, byte [RBX]: load 0xC3 from memory and zero-extend.
    let code = [0x48, 0x0f, 0xb6, 0x03, 0xf4]; // MOVZX RAX, byte [RBX]
    let mut regs = Registers::default();
    regs.rax = 0xFFFF_FFFF_FFFF_FFFF;
    regs.rbx = DATA_ADDR;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    write_mem_at_u8(&mem, DATA_ADDR, 0xC3);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax, 0x0000_0000_0000_00C3,
        "MOVZX from memory byte zero-extends"
    );
}

#[test]
fn test_strict_movzx_does_not_touch_flags() {
    let code = [0x0f, 0xb6, 0xc3, 0xf4]; // MOVZX EAX, BL
    let mut regs = Registers::default();
    regs.rbx = 0x12;
    regs.rflags = 0x2 | 0x1 | 0x40 | 0x80 | 0x800;
    let before = regs.rflags;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x12);
    assert_eq!(
        regs.rflags & 0x8D5,
        before & 0x8D5,
        "MOVZX must not alter status flags"
    );
}
