// Module path for tests run via x86_64.rs
use crate::common::*;
use rax::cpu::Registers;

// MOVSX - Move with Sign Extension (Comprehensive Extended Tests)
// Sign-extends a smaller operand to a larger destination.
// Critical for signed integer operations and type conversions.
//
// Opcodes:
// 0F BE /r     MOVSX r16/32/64, r/m8   - Move byte to word/dword/qword with sign extension
// 0F BF /r     MOVSX r32/64, r/m16     - Move word to dword/qword with sign extension
// REX.W + 63 /r MOVSXD r64, r/m32      - Move dword to qword with sign extension (also known as MOVSXD/MOVSX)

#[test]
fn test_movsx_byte_to_word() {
    // MOVSX AX, BL - Sign extend byte to word
    let code = [
        0x66, 0x0f, 0xbe, 0xc3, // MOVSX AX, BL
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x80; // -128 in signed byte
    regs.rax = 0xFFFFFFFFFFFFFFFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFFFF, 0xFF80); // Sign extended to -128 in word
}

#[test]
fn test_movsx_byte_to_dword() {
    // MOVSX EAX, BL - Sign extend byte to dword
    let code = [
        0x0f, 0xbe, 0xc3, // MOVSX EAX, BL
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x80; // -128 in signed byte
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x00000000FFFFFF80); // Sign extended and zero extended to 64-bit
}

#[test]
fn test_movsx_byte_to_qword() {
    // MOVSX RAX, BL - Sign extend byte to qword
    let code = [
        0x48, 0x0f, 0xbe, 0xc3, // MOVSX RAX, BL
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x80; // -128 in signed byte
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0xFFFFFFFFFFFFFF80); // Sign extended to -128 in qword
}

#[test]
fn test_movsx_positive_byte_to_qword() {
    // MOVSX RAX, BL - Positive byte (no sign extension needed)
    let code = [
        0x48, 0x0f, 0xbe, 0xc3, // MOVSX RAX, BL
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x7F; // +127 in signed byte
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x000000000000007F); // Sign bit is 0, so extended with zeros
}

#[test]
fn test_movsx_word_to_dword() {
    // MOVSX EAX, BX - Sign extend word to dword
    let code = [
        0x0f, 0xbf, 0xc3, // MOVSX EAX, BX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x8000; // -32768 in signed word
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x00000000FFFF8000); // Sign extended and zero extended to 64-bit
}

#[test]
fn test_movsx_word_to_qword() {
    // MOVSX RAX, BX - Sign extend word to qword
    let code = [
        0x48, 0x0f, 0xbf, 0xc3, // MOVSX RAX, BX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x8000; // -32768 in signed word
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0xFFFFFFFFFFFF8000); // Sign extended to -32768 in qword
}

#[test]
fn test_movsx_positive_word_to_qword() {
    // MOVSX RAX, BX - Positive word
    let code = [
        0x48, 0x0f, 0xbf, 0xc3, // MOVSX RAX, BX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x7FFF; // +32767 in signed word
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x0000000000007FFF);
}

#[test]
fn test_movsxd_dword_to_qword() {
    // MOVSXD RAX, EBX - Sign extend dword to qword
    let code = [
        0x48, 0x63, 0xc3, // MOVSXD RAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x80000000; // -2147483648 in signed dword
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0xFFFFFFFF80000000); // Sign extended to qword
}

#[test]
fn test_movsxd_positive_dword_to_qword() {
    // MOVSXD RAX, EBX - Positive dword
    let code = [
        0x48, 0x63, 0xc3, // MOVSXD RAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x7FFFFFFF; // +2147483647 in signed dword
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x000000007FFFFFFF);
}

#[test]
fn test_movsx_zero_byte() {
    // MOVSX RAX, BL - Zero value
    let code = [
        0x48, 0x0f, 0xbe, 0xc3, // MOVSX RAX, BL
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x00;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x0000000000000000);
}

#[test]
fn test_movsx_one_byte() {
    // MOVSX RAX, BL - Value 1
    let code = [
        0x48, 0x0f, 0xbe, 0xc3, // MOVSX RAX, BL
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x01;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x0000000000000001);
}

#[test]
fn test_movsx_minus_one_byte() {
    // MOVSX RAX, BL - Value -1 (0xFF)
    let code = [
        0x48, 0x0f, 0xbe, 0xc3, // MOVSX RAX, BL
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0xFFFFFFFFFFFFFFFF);
}

#[test]
fn test_movsx_from_memory_byte() {
    // MOVSX RAX, BYTE PTR [RBX]
    let code = [
        0x48, 0x0f, 0xbe, 0x03, // MOVSX RAX, BYTE PTR [RBX]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = DATA_ADDR;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    write_mem_u8(&mem, 0x80); // -128
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0xFFFFFFFFFFFFFF80);
}

#[test]
fn test_movsx_from_memory_word() {
    // MOVSX RAX, WORD PTR [RBX]
    let code = [
        0x48, 0x0f, 0xbf, 0x03, // MOVSX RAX, WORD PTR [RBX]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = DATA_ADDR;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    write_mem_u16(&mem, 0x8000); // -32768
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0xFFFFFFFFFFFF8000);
}

#[test]
fn test_movsxd_from_memory_dword() {
    // MOVSXD RAX, DWORD PTR [RBX]
    let code = [
        0x48, 0x63, 0x03, // MOVSXD RAX, DWORD PTR [RBX]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = DATA_ADDR;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    write_mem_u32(&mem, 0x80000000); // -2147483648
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0xFFFFFFFF80000000);
}

#[test]
fn test_movsx_with_displacement() {
    // MOVSX RAX, BYTE PTR [RBX+16]
    let code = [
        0x48, 0x0f, 0xbe, 0x43, 0x10, // MOVSX RAX, BYTE PTR [RBX+16]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = DATA_ADDR;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    write_mem_at_u8(&mem, DATA_ADDR + 16, 0x80);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0xFFFFFFFFFFFFFF80);
}

#[test]
fn test_movsx_extended_registers() {
    // MOVSX R8, R9B - Extended registers
    let code = [
        0x4d, 0x0f, 0xbe, 0xc1, // MOVSX R8, R9B
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.r9 = 0xFF; // -1 in signed byte
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.r8, 0xFFFFFFFFFFFFFFFF);
}

#[test]
fn test_movsx_r15_from_byte() {
    // MOVSX R15, AL
    let code = [
        0x4c, 0x0f, 0xbe, 0xf8, // MOVSX R15, AL
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x80;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.r15, 0xFFFFFFFFFFFFFF80);
}

#[test]
fn test_movsx_boundary_values_byte() {
    // Test boundary values for byte sign extension
    let test_cases = vec![
        (0x00, 0x0000000000000000u64),
        (0x01, 0x0000000000000001u64),
        (0x7F, 0x000000000000007Fu64), // Max positive
        (0x80, 0xFFFFFFFFFFFFFF80u64), // Min negative
        (0xFF, 0xFFFFFFFFFFFFFFFFu64), // -1
    ];

    for (input, expected) in test_cases {
        let code = [
            0x48, 0x0f, 0xbe, 0xc3, // MOVSX RAX, BL
            0xf4,
        ];
        let mut regs = Registers::default();
        regs.rbx = input;
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();
        assert_eq!(
            regs.rax, expected,
            "MOVSX byte {:#x} should be {:#x}",
            input, expected
        );
    }
}

#[test]
fn test_movsx_boundary_values_word() {
    // Test boundary values for word sign extension
    let test_cases = vec![
        (0x0000, 0x0000000000000000u64),
        (0x0001, 0x0000000000000001u64),
        (0x7FFF, 0x0000000000007FFFu64), // Max positive
        (0x8000, 0xFFFFFFFFFFFF8000u64), // Min negative
        (0xFFFF, 0xFFFFFFFFFFFFFFFFu64), // -1
    ];

    for (input, expected) in test_cases {
        let code = [
            0x48, 0x0f, 0xbf, 0xc3, // MOVSX RAX, BX
            0xf4,
        ];
        let mut regs = Registers::default();
        regs.rbx = input;
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();
        assert_eq!(
            regs.rax, expected,
            "MOVSX word {:#x} should be {:#x}",
            input, expected
        );
    }
}

#[test]
fn test_movsxd_boundary_values_dword() {
    // Test boundary values for dword sign extension
    let test_cases = vec![
        (0x00000000, 0x0000000000000000u64),
        (0x00000001, 0x0000000000000001u64),
        (0x7FFFFFFFu32 as i32, 0x000000007FFFFFFFu64), // Max positive
        (0x80000000u32 as i32, 0xFFFFFFFF80000000u64), // Min negative
        (0xFFFFFFFFu32 as i32, 0xFFFFFFFFFFFFFFFFu64), // -1
    ];

    for (input, expected) in test_cases {
        let code = [
            0x48, 0x63, 0xc3, // MOVSXD RAX, EBX
            0xf4,
        ];
        let mut regs = Registers::default();
        regs.rbx = input as u64;
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();
        assert_eq!(
            regs.rax, expected,
            "MOVSXD dword {:#x} should be {:#x}",
            input, expected
        );
    }
}

#[test]
fn test_movsx_chain_operations() {
    // Chain of MOVSX operations
    let code = [
        0x48, 0x0f, 0xbe, 0xc3, // MOVSX RAX, BL
        0x48, 0x0f, 0xbe, 0xd0, // MOVSX RDX, AL
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x80;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0xFFFFFFFFFFFFFF80);
    assert_eq!(regs.rdx, 0xFFFFFFFFFFFFFF80);
}

#[test]
fn test_movsx_preserves_flags() {
    // MOVSX should not affect flags
    let code = [
        0x48, 0x0f, 0xbe, 0xc3, // MOVSX RAX, BL
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x80;
    regs.rflags = 0x246; // Some flags set
    let initial_flags = regs.rflags;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rflags, initial_flags);
}

#[test]
fn test_movsx_all_extended_regs_as_dest() {
    // Test MOVSX to all extended registers
    let code = [
        0x4c, 0x0f, 0xbe, 0xc3, // MOVSX R8, BL
        0x4c, 0x0f, 0xbe, 0xcb, // MOVSX R9, BL
        0x4c, 0x0f, 0xbe, 0xd3, // MOVSX R10, BL
        0x4c, 0x0f, 0xbe, 0xdb, // MOVSX R11, BL
        0x4c, 0x0f, 0xbe, 0xe3, // MOVSX R12, BL
        0x4c, 0x0f, 0xbe, 0xeb, // MOVSX R13, BL
        0x4c, 0x0f, 0xbe, 0xf3, // MOVSX R14, BL
        0x4c, 0x0f, 0xbe, 0xfb, // MOVSX R15, BL
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xFF; // -1
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.r8, 0xFFFFFFFFFFFFFFFF);
    assert_eq!(regs.r9, 0xFFFFFFFFFFFFFFFF);
    assert_eq!(regs.r10, 0xFFFFFFFFFFFFFFFF);
    assert_eq!(regs.r11, 0xFFFFFFFFFFFFFFFF);
    assert_eq!(regs.r12, 0xFFFFFFFFFFFFFFFF);
    assert_eq!(regs.r13, 0xFFFFFFFFFFFFFFFF);
    assert_eq!(regs.r14, 0xFFFFFFFFFFFFFFFF);
    assert_eq!(regs.r15, 0xFFFFFFFFFFFFFFFF);
}

#[test]
fn test_movsx_typical_char_to_int() {
    // Typical use case: char to int conversion
    let code = [
        0x0f, 0xbe, 0xc3, // MOVSX EAX, BL
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 'A' as u64; // 0x41
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x0000000000000041);
}

#[test]
fn test_movsx_typical_short_to_int() {
    // Typical use case: short to int conversion
    let code = [
        0x0f, 0xbf, 0xc3, // MOVSX EAX, BX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x8000; // -32768
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFFFFFFFF, 0xFFFF8000);
}

#[test]
fn test_movsxd_typical_int_to_long() {
    // Typical use case: int to long conversion
    let code = [
        0x48, 0x63, 0xc3, // MOVSXD RAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x80000000; // -2147483648
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0xFFFFFFFF80000000u64 as u64);
}

#[test]
fn test_movsx_sib_addressing() {
    // MOVSX with SIB byte addressing
    let code = [
        0x48, 0x0f, 0xbe, 0x04, 0x0b, // MOVSX RAX, BYTE PTR [RBX+RCX]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = DATA_ADDR;
    regs.rcx = 8;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    write_mem_at_u8(&mem, DATA_ADDR + 8, 0xFF);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0xFFFFFFFFFFFFFFFF);
}

#[test]
fn test_movsx_rip_relative() {
    // MOVSX with RIP-relative addressing
    // RIP after the instruction points to offset 8 (HLT), so displacement of 1 reads offset 9 (data)
    let code = [
        0x48, 0x0f, 0xbe, 0x05, 0x01, 0x00, 0x00, 0x00, // MOVSX RAX, BYTE PTR [RIP+1]
        0xf4, 0x80, // Data: -128
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0xFFFFFFFFFFFFFF80);
}

#[test]
fn test_movsx_multiple_sources() {
    // Multiple MOVSX from different sources
    let code = [
        0x48, 0x0f, 0xbe, 0xc3, // MOVSX RAX, BL
        0x48, 0x0f, 0xbe, 0xcf, // MOVSX RCX, DIL
        0x48, 0x0f, 0xbe, 0xd6, // MOVSX RDX, SIL
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x80;
    regs.rdi = 0x7F;
    regs.rsi = 0xFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0xFFFFFFFFFFFFFF80);
    assert_eq!(regs.rcx, 0x000000000000007F);
    assert_eq!(regs.rdx, 0xFFFFFFFFFFFFFFFF);
}

#[test]
fn test_movsx_overwrite_previous_value() {
    // MOVSX overwrites previous register value
    let code = [
        0x48, 0x0f, 0xbe, 0xc3, // MOVSX RAX, BL
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x123456789ABCDEF0; // Will be completely replaced
    regs.rbx = 0x42;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x0000000000000042);
}

#[test]
fn test_movsx_word_various_patterns() {
    // Various bit patterns for word sign extension
    let test_cases = vec![
        (0x0001, 0x0000000000000001u64),
        (0x00FF, 0x00000000000000FFu64),
        (0x0100, 0x0000000000000100u64),
        (0x7FFE, 0x0000000000007FFEu64),
        (0x8001, 0xFFFFFFFFFFFF8001u64),
        (0xFF00, 0xFFFFFFFFFFFFFF00u64),
        (0xFFFE, 0xFFFFFFFFFFFFFFFEu64),
    ];

    for (input, expected) in test_cases {
        let code = [
            0x48, 0x0f, 0xbf, 0xc3, // MOVSX RAX, BX
            0xf4,
        ];
        let mut regs = Registers::default();
        regs.rbx = input;
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();
        assert_eq!(
            regs.rax, expected,
            "MOVSX word {:#x} should be {:#x}",
            input, expected
        );
    }
}

#[test]
fn test_movsxd_various_patterns() {
    // Various bit patterns for dword sign extension
    let test_cases = vec![
        (0x00000001u32, 0x0000000000000001u64),
        (0x000000FFu32, 0x00000000000000FFu64),
        (0x00000100u32, 0x0000000000000100u64),
        (0x7FFFFFFEu32, 0x000000007FFFFFFEu64),
        (0x80000001u32, 0xFFFFFFFF80000001u64),
        (0xFF000000u32, 0xFFFFFFFFFF000000u64),
        (0xFFFFFFFEu32, 0xFFFFFFFFFFFFFFFEu64),
    ];

    for (input, expected) in test_cases {
        let code = [
            0x48, 0x63, 0xc3, // MOVSXD RAX, EBX
            0xf4,
        ];
        let mut regs = Registers::default();
        regs.rbx = input as u64;
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();
        assert_eq!(
            regs.rax, expected,
            "MOVSXD dword {:#x} should be {:#x}",
            input, expected
        );
    }
}

#[test]
fn test_movsx_from_high_byte() {
    // MOVSX from high byte register (AH, BH, CH, DH)
    let code = [
        0x0f, 0xbe, 0xc7, // MOVSX EAX, BH
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xFF00; // BH = 0xFF
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x00000000FFFFFFFF);
}

#[test]
fn test_movsx_memory_indirect_r8() {
    // MOVSX with R8 as base register
    let code = [
        0x4d, 0x0f, 0xbe, 0x00, // MOVSX R8, BYTE PTR [R8]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.r8 = DATA_ADDR;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    write_mem_u8(&mem, 0x80);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.r8, 0xFFFFFFFFFFFFFF80);
}

#[test]
fn test_movsx_with_scale() {
    // MOVSX with scaled index
    let code = [
        0x48, 0x0f, 0xbe, 0x04, 0x8b, // MOVSX RAX, BYTE PTR [RBX+RCX*4]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = DATA_ADDR;
    regs.rcx = 4;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    write_mem_at_u8(&mem, DATA_ADDR + 16, 0x7F);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x000000000000007F);
}

#[test]
fn test_movsx_sequential_bytes() {
    // Sign extend sequential byte values
    let code = [
        0x48, 0x0f, 0xbe, 0x03, // MOVSX RAX, BYTE PTR [RBX]
        0x48, 0x0f, 0xbe, 0x4b, 0x01, // MOVSX RCX, BYTE PTR [RBX+1]
        0x48, 0x0f, 0xbe, 0x53, 0x02, // MOVSX RDX, BYTE PTR [RBX+2]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = DATA_ADDR;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    write_mem_at_u8(&mem, DATA_ADDR, 0x80);
    write_mem_at_u8(&mem, DATA_ADDR + 1, 0x00);
    write_mem_at_u8(&mem, DATA_ADDR + 2, 0x7F);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0xFFFFFFFFFFFFFF80);
    assert_eq!(regs.rcx, 0x0000000000000000);
    assert_eq!(regs.rdx, 0x000000000000007F);
}

#[test]
fn test_movsxd_array_access() {
    // Typical pattern: array indexing with sign extension
    let code = [
        0x48, 0x63, 0x04, 0x8b, // MOVSXD RAX, DWORD PTR [RBX+RCX*4]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = DATA_ADDR;
    regs.rcx = 2; // Index 2
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    write_mem_at_u32(&mem, DATA_ADDR + 8, 0xFFFFFFFF); // -1 at index 2
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0xFFFFFFFFFFFFFFFF);
}

#[test]
fn test_movsx_does_not_affect_source() {
    // MOVSX should not modify source register
    let code = [
        0x48, 0x0f, 0xbe, 0xc3, // MOVSX RAX, BL
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x123456789ABCDE80;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rbx, 0x123456789ABCDE80); // Unchanged
    assert_eq!(regs.rax, 0xFFFFFFFFFFFFFF80); // Sign extended from BL
}
