// Module path for tests run via x86_64.rs
use crate::common::*;
use rax::cpu::Registers;

// MOVZX - Move with Zero Extension (Comprehensive Extended Tests)
// Zero-extends a smaller operand to a larger destination.
// Critical for unsigned integer operations and type conversions.
//
// Opcodes:
// 0F B6 /r     MOVZX r16/32/64, r/m8   - Move byte to word/dword/qword with zero extension
// 0F B7 /r     MOVZX r32/64, r/m16     - Move word to dword/qword with zero extension
//
// Note: There is no MOVZX for dword to qword. 32-bit operations automatically zero-extend
// to 64 bits in x86-64 mode.

#[test]
fn test_movzx_byte_to_word() {
    // MOVZX AX, BL - Zero extend byte to word
    let code = [
        0x66, 0x0f, 0xb6, 0xc3, // MOVZX AX, BL
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xFF; // Would be -1 if signed
    regs.rax = 0xFFFFFFFFFFFFFFFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFFFF, 0x00FF); // Zero extended to 255 in word
}

#[test]
fn test_movzx_byte_to_dword() {
    // MOVZX EAX, BL - Zero extend byte to dword
    let code = [
        0x0f, 0xb6, 0xc3, // MOVZX EAX, BL
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x00000000000000FF); // Zero extended to 64-bit
}

#[test]
fn test_movzx_byte_to_qword() {
    // MOVZX RAX, BL - Zero extend byte to qword
    let code = [
        0x48, 0x0f, 0xb6, 0xc3, // MOVZX RAX, BL
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x00000000000000FF); // Zero extended to qword
}

#[test]
fn test_movzx_word_to_dword() {
    // MOVZX EAX, BX - Zero extend word to dword
    let code = [
        0x0f, 0xb7, 0xc3, // MOVZX EAX, BX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xFFFF; // Would be -1 if signed
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x000000000000FFFF); // Zero extended
}

#[test]
fn test_movzx_word_to_qword() {
    // MOVZX RAX, BX - Zero extend word to qword
    let code = [
        0x48, 0x0f, 0xb7, 0xc3, // MOVZX RAX, BX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xFFFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x000000000000FFFF); // Zero extended to qword
}

#[test]
fn test_movzx_zero_byte() {
    // MOVZX RAX, BL - Zero value
    let code = [
        0x48, 0x0f, 0xb6, 0xc3, // MOVZX RAX, BL
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x00;
    regs.rax = 0xFFFFFFFFFFFFFFFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x0000000000000000);
}

#[test]
fn test_movzx_one_byte() {
    // MOVZX RAX, BL - Value 1
    let code = [
        0x48, 0x0f, 0xb6, 0xc3, // MOVZX RAX, BL
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x01;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x0000000000000001);
}

#[test]
fn test_movzx_max_byte() {
    // MOVZX RAX, BL - Maximum byte value (0xFF)
    let code = [
        0x48, 0x0f, 0xb6, 0xc3, // MOVZX RAX, BL
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x00000000000000FF); // Not sign extended
}

#[test]
fn test_movzx_high_bit_set_byte() {
    // MOVZX with high bit set (would be negative if signed)
    let code = [
        0x48, 0x0f, 0xb6, 0xc3, // MOVZX RAX, BL
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x80; // Would be -128 in signed
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x0000000000000080); // Zero extended, not sign extended
}

#[test]
fn test_movzx_high_bit_set_word() {
    // MOVZX with high bit set in word
    let code = [
        0x48, 0x0f, 0xb7, 0xc3, // MOVZX RAX, BX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x8000; // Would be -32768 in signed
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x0000000000008000); // Zero extended
}

#[test]
fn test_movzx_from_memory_byte() {
    // MOVZX RAX, BYTE PTR [RBX]
    let code = [
        0x48, 0x0f, 0xb6, 0x03, // MOVZX RAX, BYTE PTR [RBX]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = DATA_ADDR;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    write_mem_u8(&mem, 0xFF);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x00000000000000FF);
}

#[test]
fn test_movzx_from_memory_word() {
    // MOVZX RAX, WORD PTR [RBX]
    let code = [
        0x48, 0x0f, 0xb7, 0x03, // MOVZX RAX, WORD PTR [RBX]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = DATA_ADDR;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    write_mem_u16(&mem, 0xFFFF);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x000000000000FFFF);
}

#[test]
fn test_movzx_with_displacement() {
    // MOVZX RAX, BYTE PTR [RBX+16]
    let code = [
        0x48, 0x0f, 0xb6, 0x43, 0x10, // MOVZX RAX, BYTE PTR [RBX+16]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = DATA_ADDR;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    write_mem_at_u8(&mem, DATA_ADDR + 16, 0x80);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x0000000000000080);
}

#[test]
fn test_movzx_extended_registers() {
    // MOVZX R8, R9B - Extended registers
    let code = [
        0x4d, 0x0f, 0xb6, 0xc1, // MOVZX R8, R9B
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.r9 = 0xFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.r8, 0x00000000000000FF);
}

#[test]
fn test_movzx_r15_from_byte() {
    // MOVZX R15, AL
    let code = [
        0x4c, 0x0f, 0xb6, 0xf8, // MOVZX R15, AL
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x80;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.r15, 0x0000000000000080);
}

#[test]
fn test_movzx_boundary_values_byte() {
    // Test boundary values for byte zero extension
    let test_cases = vec![
        (0x00, 0x0000000000000000u64),
        (0x01, 0x0000000000000001u64),
        (0x7F, 0x000000000000007Fu64),
        (0x80, 0x0000000000000080u64), // High bit set
        (0xFF, 0x00000000000000FFu64), // Max value
    ];

    for (input, expected) in test_cases {
        let code = [
            0x48, 0x0f, 0xb6, 0xc3, // MOVZX RAX, BL
            0xf4,
        ];
        let mut regs = Registers::default();
        regs.rbx = input;
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();
        assert_eq!(
            regs.rax, expected,
            "MOVZX byte {:#x} should be {:#x}",
            input, expected
        );
    }
}

#[test]
fn test_movzx_boundary_values_word() {
    // Test boundary values for word zero extension
    let test_cases = vec![
        (0x0000, 0x0000000000000000u64),
        (0x0001, 0x0000000000000001u64),
        (0x7FFF, 0x0000000000007FFFu64),
        (0x8000, 0x0000000000008000u64), // High bit set
        (0xFFFF, 0x000000000000FFFFu64), // Max value
    ];

    for (input, expected) in test_cases {
        let code = [
            0x48, 0x0f, 0xb7, 0xc3, // MOVZX RAX, BX
            0xf4,
        ];
        let mut regs = Registers::default();
        regs.rbx = input;
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();
        assert_eq!(
            regs.rax, expected,
            "MOVZX word {:#x} should be {:#x}",
            input, expected
        );
    }
}

#[test]
fn test_movzx_chain_operations() {
    // Chain of MOVZX operations
    let code = [
        0x48, 0x0f, 0xb6, 0xc3, // MOVZX RAX, BL
        0x48, 0x0f, 0xb6, 0xd0, // MOVZX RDX, AL
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x00000000000000FF);
    assert_eq!(regs.rdx, 0x00000000000000FF);
}

#[test]
fn test_movzx_preserves_flags() {
    // MOVZX should not affect flags
    let code = [
        0x48, 0x0f, 0xb6, 0xc3, // MOVZX RAX, BL
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xFF;
    regs.rflags = 0x246; // Some flags set
    let initial_flags = regs.rflags;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rflags, initial_flags);
}

#[test]
fn test_movzx_all_extended_regs_as_dest() {
    // Test MOVZX to all extended registers
    let code = [
        0x4c, 0x0f, 0xb6, 0xc3, // MOVZX R8, BL
        0x4c, 0x0f, 0xb6, 0xcb, // MOVZX R9, BL
        0x4c, 0x0f, 0xb6, 0xd3, // MOVZX R10, BL
        0x4c, 0x0f, 0xb6, 0xdb, // MOVZX R11, BL
        0x4c, 0x0f, 0xb6, 0xe3, // MOVZX R12, BL
        0x4c, 0x0f, 0xb6, 0xeb, // MOVZX R13, BL
        0x4c, 0x0f, 0xb6, 0xf3, // MOVZX R14, BL
        0x4c, 0x0f, 0xb6, 0xfb, // MOVZX R15, BL
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.r8, 0x00000000000000FF);
    assert_eq!(regs.r9, 0x00000000000000FF);
    assert_eq!(regs.r10, 0x00000000000000FF);
    assert_eq!(regs.r11, 0x00000000000000FF);
    assert_eq!(regs.r12, 0x00000000000000FF);
    assert_eq!(regs.r13, 0x00000000000000FF);
    assert_eq!(regs.r14, 0x00000000000000FF);
    assert_eq!(regs.r15, 0x00000000000000FF);
}

#[test]
fn test_movzx_typical_uchar_to_int() {
    // Typical use case: unsigned char to int conversion
    let code = [
        0x0f, 0xb6, 0xc3, // MOVZX EAX, BL
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 'A' as u64; // 0x41
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x0000000000000041);
}

#[test]
fn test_movzx_typical_ushort_to_int() {
    // Typical use case: unsigned short to int conversion
    let code = [
        0x0f, 0xb7, 0xc3, // MOVZX EAX, BX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x8000; // 32768 (unsigned)
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x0000000000008000);
}

#[test]
fn test_movzx_sib_addressing() {
    // MOVZX with SIB byte addressing
    let code = [
        0x48, 0x0f, 0xb6, 0x04, 0x0b, // MOVZX RAX, BYTE PTR [RBX+RCX]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = DATA_ADDR;
    regs.rcx = 8;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    write_mem_at_u8(&mem, DATA_ADDR + 8, 0xFF);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x00000000000000FF);
}

#[test]
fn test_movzx_rip_relative() {
    // MOVZX with RIP-relative addressing
    // RIP after the instruction points to offset 8 (HLT), so displacement of 1 reads offset 9 (data)
    let code = [
        0x48, 0x0f, 0xb6, 0x05, 0x01, 0x00, 0x00, 0x00, // MOVZX RAX, BYTE PTR [RIP+1]
        0xf4, 0x80, // Data: 128
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x0000000000000080);
}

#[test]
fn test_movzx_multiple_sources() {
    // Multiple MOVZX from different sources
    let code = [
        0x48, 0x0f, 0xb6, 0xc3, // MOVZX RAX, BL
        0x48, 0x0f, 0xb6, 0xcf, // MOVZX RCX, DIL
        0x48, 0x0f, 0xb6, 0xd6, // MOVZX RDX, SIL
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x80;
    regs.rdi = 0x7F;
    regs.rsi = 0xFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x0000000000000080);
    assert_eq!(regs.rcx, 0x000000000000007F);
    assert_eq!(regs.rdx, 0x00000000000000FF);
}

#[test]
fn test_movzx_overwrite_previous_value() {
    // MOVZX overwrites previous register value
    let code = [
        0x48, 0x0f, 0xb6, 0xc3, // MOVZX RAX, BL
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0xFFFFFFFFFFFFFFFF; // Will be completely replaced
    regs.rbx = 0x42;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x0000000000000042);
}

#[test]
fn test_movzx_word_various_patterns() {
    // Various bit patterns for word zero extension
    let test_cases = vec![
        (0x0001, 0x0000000000000001u64),
        (0x00FF, 0x00000000000000FFu64),
        (0x0100, 0x0000000000000100u64),
        (0x7FFE, 0x0000000000007FFEu64),
        (0x8001, 0x0000000000008001u64),
        (0xFF00, 0x000000000000FF00u64),
        (0xFFFE, 0x000000000000FFFEu64),
    ];

    for (input, expected) in test_cases {
        let code = [
            0x48, 0x0f, 0xb7, 0xc3, // MOVZX RAX, BX
            0xf4,
        ];
        let mut regs = Registers::default();
        regs.rbx = input;
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();
        assert_eq!(
            regs.rax, expected,
            "MOVZX word {:#x} should be {:#x}",
            input, expected
        );
    }
}

#[test]
fn test_movzx_from_high_byte() {
    // MOVZX from high byte register (AH, BH, CH, DH)
    let code = [
        0x0f, 0xb6, 0xc7, // MOVZX EAX, BH
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xFF00; // BH = 0xFF
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x00000000000000FF);
}

#[test]
fn test_movzx_memory_indirect_r8() {
    // MOVZX with R8 as base register
    let code = [
        0x4d, 0x0f, 0xb6, 0x00, // MOVZX R8, BYTE PTR [R8]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.r8 = DATA_ADDR;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    write_mem_u8(&mem, 0x80);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.r8, 0x0000000000000080);
}

#[test]
fn test_movzx_with_scale() {
    // MOVZX with scaled index
    let code = [
        0x48, 0x0f, 0xb6, 0x04, 0x8b, // MOVZX RAX, BYTE PTR [RBX+RCX*4]
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
fn test_movzx_sequential_bytes() {
    // Zero extend sequential byte values
    let code = [
        0x48, 0x0f, 0xb6, 0x03, // MOVZX RAX, BYTE PTR [RBX]
        0x48, 0x0f, 0xb6, 0x4b, 0x01, // MOVZX RCX, BYTE PTR [RBX+1]
        0x48, 0x0f, 0xb6, 0x53, 0x02, // MOVZX RDX, BYTE PTR [RBX+2]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = DATA_ADDR;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    write_mem_at_u8(&mem, DATA_ADDR, 0x80);
    write_mem_at_u8(&mem, DATA_ADDR + 1, 0x00);
    write_mem_at_u8(&mem, DATA_ADDR + 2, 0xFF);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x0000000000000080);
    assert_eq!(regs.rcx, 0x0000000000000000);
    assert_eq!(regs.rdx, 0x00000000000000FF);
}

#[test]
fn test_movzx_array_access() {
    // Typical pattern: array indexing with zero extension
    let code = [
        0x48, 0x0f, 0xb6, 0x04, 0x0b, // MOVZX RAX, BYTE PTR [RBX+RCX]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = DATA_ADDR;
    regs.rcx = 5; // Index 5
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    write_mem_at_u8(&mem, DATA_ADDR + 5, 0xFF);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x00000000000000FF);
}

#[test]
fn test_movzx_does_not_affect_source() {
    // MOVZX should not modify source register
    let code = [
        0x48, 0x0f, 0xb6, 0xc3, // MOVZX RAX, BL
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x123456789ABCDE80;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rbx, 0x123456789ABCDE80); // Unchanged
    assert_eq!(regs.rax, 0x0000000000000080); // Zero extended from BL
}

#[test]
fn test_movzx_vs_movsx_difference() {
    // Demonstrate difference between MOVZX and MOVSX for same input
    // MOVZX version
    let code = [
        0x48, 0x0f, 0xb6, 0xc3, // MOVZX RAX, BL
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x80; // High bit set
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x0000000000000080); // Zero extended, treated as 128
}

#[test]
fn test_movzx_ascii_range() {
    // MOVZX with ASCII character range
    let test_cases = vec![
        ('A' as u8, 0x0000000000000041u64),
        ('Z' as u8, 0x000000000000005Au64),
        ('a' as u8, 0x0000000000000061u64),
        ('z' as u8, 0x000000000000007Au64),
        ('0' as u8, 0x0000000000000030u64),
        ('9' as u8, 0x0000000000000039u64),
    ];

    for (input, expected) in test_cases {
        let code = [
            0x48, 0x0f, 0xb6, 0xc3, // MOVZX RAX, BL
            0xf4,
        ];
        let mut regs = Registers::default();
        regs.rbx = input as u64;
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();
        assert_eq!(
            regs.rax, expected,
            "MOVZX ASCII '{:#x}' should be {:#x}",
            input, expected
        );
    }
}

#[test]
fn test_movzx_power_of_two_values() {
    // Test with powers of 2
    let test_cases = vec![
        (0x01, 0x0000000000000001u64),
        (0x02, 0x0000000000000002u64),
        (0x04, 0x0000000000000004u64),
        (0x08, 0x0000000000000008u64),
        (0x10, 0x0000000000000010u64),
        (0x20, 0x0000000000000020u64),
        (0x40, 0x0000000000000040u64),
        (0x80, 0x0000000000000080u64),
    ];

    for (input, expected) in test_cases {
        let code = [
            0x48, 0x0f, 0xb6, 0xc3, // MOVZX RAX, BL
            0xf4,
        ];
        let mut regs = Registers::default();
        regs.rbx = input;
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();
        assert_eq!(
            regs.rax, expected,
            "MOVZX byte {:#x} should be {:#x}",
            input, expected
        );
    }
}

#[test]
fn test_movzx_word_from_memory_displacement() {
    // MOVZX with word from memory with displacement
    let code = [
        0x48, 0x0f, 0xb7, 0x43, 0x20, // MOVZX RAX, WORD PTR [RBX+32]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = DATA_ADDR;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    write_mem_at_u16(&mem, DATA_ADDR + 32, 0x8000);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x0000000000008000);
}

#[test]
fn test_movzx_32bit_dest_zero_extends_to_64() {
    // 32-bit destination automatically zero-extends to 64-bit in x86-64
    let code = [
        0x0f, 0xb6, 0xc3, // MOVZX EAX, BL (32-bit dest)
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0xFFFFFFFFFFFFFFFF; // All bits set
    regs.rbx = 0x42;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x0000000000000042); // Upper 32 bits cleared by 32-bit operation
}

#[test]
fn test_movzx_alternating_bit_pattern() {
    // Test with alternating bit patterns
    let test_cases = vec![
        (0xAA, 0x00000000000000AAu64), // 10101010
        (0x55, 0x0000000000000055u64), // 01010101
        (0xCC, 0x00000000000000CCu64), // 11001100
        (0x33, 0x0000000000000033u64), // 00110011
    ];

    for (input, expected) in test_cases {
        let code = [
            0x48, 0x0f, 0xb6, 0xc3, // MOVZX RAX, BL
            0xf4,
        ];
        let mut regs = Registers::default();
        regs.rbx = input;
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();
        assert_eq!(
            regs.rax, expected,
            "MOVZX byte {:#x} should be {:#x}",
            input, expected
        );
    }
}

#[test]
fn test_movzx_negative_displacement() {
    // MOVZX with negative displacement
    let code = [
        0x48, 0x0f, 0xb6, 0x43, 0xf0, // MOVZX RAX, BYTE PTR [RBX-16]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = DATA_ADDR + 32;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    write_mem_at_u8(&mem, DATA_ADDR + 16, 0xFF);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x00000000000000FF);
}

#[test]
fn test_movzx_large_displacement() {
    // MOVZX with large displacement
    let code = [
        0x48, 0x0f, 0xb6, 0x83, 0x00, 0x10, 0x00, 0x00, // MOVZX RAX, BYTE PTR [RBX+0x1000]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = DATA_ADDR;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    write_mem_at_u8(&mem, DATA_ADDR + 0x1000, 0xAB);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x00000000000000AB);
}
