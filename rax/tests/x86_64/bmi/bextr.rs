use crate::common::*;
use rax::cpu::Registers;

// BEXTR - Bit Field Extract (BMI1)
// Extracts contiguous bits from the first source operand using an index and length specified
// in the second source operand. Bits[7:0] of the second source specify the starting bit position.
// Bits[15:8] specify the length in bits to extract.
// The extracted bits are written to the destination register with zero extension.
// ZF is set if the extracted field is all zeros, CF is cleared, OF/SF/AF/PF are undefined.
//
// Opcodes:
// VEX.NDS.LZ.0F38.W0 F7 /r   BEXTR r32, r/m32, r32   - Extract bits from r/m32 using r32 (32-bit)
// VEX.NDS.LZ.0F38.W1 F7 /r   BEXTR r64, r/m64, r64   - Extract bits from r/m64 using r64 (64-bit)

#[test]
fn test_bextr_eax_ebx_ecx_basic() {
    // BEXTR EAX, EBX, ECX - extract 8 bits starting at bit 4
    let code = [
        0xc4, 0xe2, 0x70, 0xf7, 0xc3, // BEXTR EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x12345678;
    regs.rcx = (8 << 8) | 4; // length=8, start=4
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Extract 8 bits starting at bit 4: bits 4-11 of 0x12345678
    // 0x12345678 >> 4 = 0x01234567, mask 8 bits = 0x67
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x67,
        "EAX should contain extracted bits"
    );
    assert!(
        !zf_set(regs.rflags),
        "ZF should be clear (result is non-zero)"
    );
    assert!(!cf_set(regs.rflags), "CF should be clear");
}

#[test]
fn test_bextr_eax_ebx_ecx_zero_result() {
    // BEXTR that extracts all zeros
    let code = [
        0xc4, 0xe2, 0x70, 0xf7, 0xc3, // BEXTR EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x00000000;
    regs.rcx = (8 << 8) | 4; // length=8, start=4
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 0, "EAX should be zero");
    assert!(zf_set(regs.rflags), "ZF should be set (result is zero)");
}

#[test]
fn test_bextr_single_bit_extract() {
    // BEXTR to extract a single bit
    let code = [
        0xc4, 0xe2, 0x70, 0xf7, 0xc3, // BEXTR EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xFFFFFFFF; // All bits set
    regs.rcx = (1 << 8) | 5; // length=1, start=5 (extract bit 5)
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x00000001,
        "EAX should contain extracted bit"
    );
}

#[test]
fn test_bextr_full_range_extract() {
    // BEXTR to extract from bit 0 with length 32
    let code = [
        0xc4, 0xe2, 0x70, 0xf7, 0xc3, // BEXTR EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xDEADBEEF;
    regs.rcx = (32 << 8) | 0; // length=32, start=0 (extract all)
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0xDEADBEEF,
        "EAX should contain all 32 bits"
    );
}

#[test]
fn test_bextr_partial_range() {
    // BEXTR with various start and length combinations
    let code = [
        0xc4, 0xe2, 0x70, 0xf7, 0xc3, // BEXTR EAX, EBX, ECX
        0xf4,
    ];

    let test_cases = [
        (0x0000_00FFu32, (8 << 8) | 0, 0xFFu32),     // bits 0-7
        (0x0000_FF00u32, (8 << 8) | 8, 0xFFu32),     // bits 8-15
        (0x00FF_0000u32, (8 << 8) | 16, 0xFFu32),    // bits 16-23
        (0xFF00_0000u32, (8 << 8) | 24, 0xFFu32),    // bits 24-31
        (0xFFFF_0000u32, (16 << 8) | 16, 0xFFFFu32), // bits 16-31
        (0x0000_FFFFu32, (16 << 8) | 0, 0xFFFFu32),  // bits 0-15
    ];

    for (value, params, expected) in &test_cases {
        let mut regs = Registers::default();
        regs.rbx = *value as u64;
        regs.rcx = *params as u64;
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        assert_eq!(
            regs.rax & 0xFFFFFFFF,
            *expected as u64,
            "BEXTR({:08x}, {:04x}) extraction failed",
            value,
            params
        );
    }
}

#[test]
fn test_bextr_rax_rbx_rcx_64bit() {
    // BEXTR RAX, RBX, RCX - 64-bit version
    let code = [
        0xc4, 0xe2, 0xf0, 0xf7, 0xc3, // BEXTR RAX, RBX, RCX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x0123_4567_89AB_CDEF;
    regs.rcx = (32 << 8) | 32; // length=32, start=32
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Extract bits 32-63 of 0x0123_4567_89AB_CDEF = 0x0123_4567
    assert_eq!(
        regs.rax, 0x0000_0000_0123_4567,
        "RAX should contain high 32 bits"
    );
}

#[test]
fn test_bextr_zero_length() {
    // BEXTR with zero length should extract nothing
    let code = [
        0xc4, 0xe2, 0x70, 0xf7, 0xc3, // BEXTR EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xFFFFFFFF;
    regs.rcx = (0 << 8) | 5; // length=0, start=5
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0,
        "EAX should be zero for zero length"
    );
    assert!(zf_set(regs.rflags), "ZF should be set for zero result");
}

#[test]
fn test_bextr_beyond_boundary() {
    // BEXTR that tries to extract beyond word boundary
    let code = [
        0xc4, 0xe2, 0x70, 0xf7, 0xc3, // BEXTR EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xFFFFFFFF;
    regs.rcx = (16 << 8) | 28; // length=16, start=28 (goes beyond 32-bit)
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Should extract bits 28-31 (4 bits) and zero-extend
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x0000000F,
        "EAX should zero-extend partial extraction"
    );
}

#[test]
fn test_bextr_with_extended_registers() {
    // BEXTR R8D, R9D, R10D
    // VEX: R~=0, X~=1, B~=0, vvvv=0101 (R10 inverted)
    let code = [
        0xc4, 0x42, 0x28, 0xf7,
        0xc1, // BEXTR R8D, R9D, R10D (vvvv=0101=R10, R~=0 for R8, B~=0 for R9)
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.r9 = 0x12345678;
    regs.r10 = (8 << 8) | 4;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.r8 & 0xFFFFFFFF,
        0x67,
        "R8D should contain extracted bits"
    );
}

#[test]
fn test_bextr_preserves_operands() {
    // BEXTR should not modify source operands
    let code = [
        0xc4, 0xe2, 0x70, 0xf7, 0xc3, // BEXTR EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x12345678;
    regs.rcx = 0x0804;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx & 0xFFFFFFFF, 0x12345678, "EBX should be unchanged");
    assert_eq!(regs.rcx & 0xFFFFFFFF, 0x0804, "ECX should be unchanged");
}

#[test]
fn test_bextr_mem32() {
    // BEXTR EAX, [mem], ECX
    // ModRM 0x04: mod=00, reg=0 (EAX), r/m=4 (SIB follows)
    let code = [
        0xc4, 0xe2, 0x70, 0xf7, 0x04, 0x25, 0x00, 0x20, 0x00,
        0x00, // BEXTR EAX, [DATA_ADDR], ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rcx = (8 << 8) | 8; // length=8, start=8
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    write_mem_u32(&mem, 0xDEADBEEF);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Extract 8 bits starting at bit 8 from 0xDEADBEEF
    // 0xDEADBEEF >> 8 = 0x00DEADBE, mask 8 bits = 0xBE
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0xBE,
        "EAX should contain extracted bits from memory"
    );
}

#[test]
fn test_bextr_mem64() {
    // BEXTR RAX, [mem], RCX
    // ModRM 0x04: mod=00, reg=0 (RAX), r/m=4 (SIB follows)
    let code = [
        0xc4, 0xe2, 0xf0, 0xf7, 0x04, 0x25, 0x00, 0x20, 0x00,
        0x00, // BEXTR RAX, [DATA_ADDR], RCX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rcx = (16 << 8) | 16; // length=16, start=16
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    write_mem_u64(&mem, 0x0123456789ABCDEF);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Extract 16 bits starting at bit 16 from 0x0123456789ABCDEF
    // 0x0123456789ABCDEF >> 16 = 0x01234567_89AB, mask 16 bits = 0x89AB
    assert_eq!(
        regs.rax, 0x89AB,
        "RAX should contain extracted bits from memory"
    );
}

#[test]
fn test_bextr_byte_boundaries() {
    // Test extraction at byte boundaries
    let code = [
        0xc4, 0xe2, 0x70, 0xf7, 0xc3, // BEXTR EAX, EBX, ECX
        0xf4,
    ];

    let test_cases = [
        (0xFF00FF00u32, (8 << 8) | 0, 0x00),  // Extract byte 0
        (0xFF00FF00u32, (8 << 8) | 8, 0xFF),  // Extract byte 1
        (0xFF00FF00u32, (8 << 8) | 16, 0x00), // Extract byte 2
        (0xFF00FF00u32, (8 << 8) | 24, 0xFF), // Extract byte 3
    ];

    for (value, params, expected) in &test_cases {
        let mut regs = Registers::default();
        regs.rbx = *value as u64;
        regs.rcx = *params as u64;
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        assert_eq!(
            regs.rax & 0xFFFFFFFF,
            *expected as u64,
            "BEXTR byte extraction failed"
        );
    }
}

#[test]
fn test_bextr_nibble_extraction() {
    // Test extraction of 4-bit nibbles
    let code = [
        0xc4, 0xe2, 0x70, 0xf7, 0xc3, // BEXTR EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x0F0E0D0C;

    for nibble_idx in 0..8 {
        regs.rcx = (4 << 8) | (nibble_idx * 4); // Extract 4 bits
        let (mut vcpu, _) = setup_vm(&code, Some(regs.clone()));
        let result = run_until_hlt(&mut vcpu).unwrap();

        let expected = (0x0F0E0D0Cu32 >> (nibble_idx * 4)) & 0xF;
        assert_eq!(
            result.rax & 0xFFFFFFFF,
            expected as u64,
            "Nibble {} extraction failed",
            nibble_idx
        );
    }
}

#[test]
fn test_bextr_bit_shifting() {
    // Practical use: extract fields from bit-packed data
    let code = [
        0xc4, 0xe2, 0x70, 0xf7, 0xc3, // BEXTR EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    // Bit-packed: [bits 0-3: version], [bits 4-11: flags], [bits 12-27: id], [bits 28-31: type]
    regs.rbx = 0xABCD1234;
    // 0xABCD1234 >> 0 & 0xF = 0x4 (version)
    // 0xABCD1234 >> 4 & 0xFF = 0x23 (flags)
    // 0xABCD1234 >> 12 & 0xFFFF = 0xBCD1 (id)
    // 0xABCD1234 >> 28 & 0xF = 0xA (type)

    // Extract version (bits 0-3)
    regs.rcx = (4 << 8) | 0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs.clone()));
    let result = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(result.rax & 0xFFFFFFFF, 0x4, "Version extraction failed");

    // Extract flags (bits 4-11)
    regs.rcx = (8 << 8) | 4;
    let (mut vcpu, _) = setup_vm(&code, Some(regs.clone()));
    let result = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(result.rax & 0xFFFFFFFF, 0x23, "Flags extraction failed");

    // Extract id (bits 12-27)
    regs.rcx = (16 << 8) | 12;
    let (mut vcpu, _) = setup_vm(&code, Some(regs.clone()));
    let result = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(result.rax & 0xFFFFFFFF, 0xBCD1, "ID extraction failed");

    // Extract type (bits 28-31)
    regs.rcx = (4 << 8) | 28;
    let (mut vcpu, _) = setup_vm(&code, Some(regs.clone()));
    let result = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(result.rax & 0xFFFFFFFF, 0xA, "Type extraction failed");
}

#[test]
fn test_bextr_large_length_64bit() {
    // Test large length values in 64-bit mode
    let code = [
        0xc4, 0xe2, 0xf0, 0xf7, 0xc3, // BEXTR RAX, RBX, RCX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xFFFFFFFFFFFFFFFF;
    regs.rcx = (64 << 8) | 0; // Extract all 64 bits
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax, 0xFFFFFFFFFFFFFFFF,
        "RAX should contain all 64 bits"
    );
}

#[test]
fn test_bextr_high_bit_extraction_64bit() {
    // Test extraction from high bits in 64-bit mode
    let code = [
        0xc4, 0xe2, 0xf0, 0xf7, 0xc3, // BEXTR RAX, RBX, RCX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xFF00_0000_0000_00FF;
    regs.rcx = (8 << 8) | 56; // Extract 8 bits starting at bit 56
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0xFF, "RAX should contain extracted high bits");
}

#[test]
fn test_bextr_sequential_fields() {
    // Test extracting multiple sequential fields
    let code = [
        0xc4, 0xe2, 0x70, 0xf7, 0xc3, // BEXTR EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xDEADBEEF; // 4 bytes: 0xDE, 0xAD, 0xBE, 0xEF

    let fields = [
        (0xEF, (8 << 8) | 0),  // Byte 0
        (0xBE, (8 << 8) | 8),  // Byte 1
        (0xAD, (8 << 8) | 16), // Byte 2
        (0xDE, (8 << 8) | 24), // Byte 3
    ];

    for (expected, params) in &fields {
        regs.rcx = *params as u64;
        let (mut vcpu, _) = setup_vm(&code, Some(regs.clone()));
        let result = run_until_hlt(&mut vcpu).unwrap();
        assert_eq!(
            result.rax & 0xFFFFFFFF,
            *expected as u64,
            "Field extraction failed"
        );
    }
}

#[test]
fn test_bextr_flags_behavior() {
    // Test flag behavior of BEXTR
    let code = [
        0xc4, 0xe2, 0x70, 0xf7, 0xc3, // BEXTR EAX, EBX, ECX
        0xf4,
    ];

    // Zero result should set ZF
    let mut regs = Registers::default();
    regs.rbx = 0x00FF00FF;
    regs.rcx = (4 << 8) | 28; // Extract bits 28-31 (all zero)
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert!(zf_set(regs.rflags), "ZF should be set for zero result");

    // Non-zero result should clear ZF
    let mut regs = Registers::default();
    regs.rbx = 0xFFFFFFFF;
    regs.rcx = (4 << 8) | 28; // Extract bits 28-31 (all one)
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert!(
        !zf_set(regs.rflags),
        "ZF should be clear for non-zero result"
    );

    // CF should always be clear
    assert!(!cf_set(regs.rflags), "CF should always be clear");
}

#[test]
fn test_bextr_comprehensive_32bit() {
    // Comprehensive 32-bit test with various parameters
    let code = [
        0xc4, 0xe2, 0x70, 0xf7, 0xc3, // BEXTR EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x12345678;

    let test_cases = [
        ((4 << 8) | 0, 0x8),      // Extract bits 0-3
        ((4 << 8) | 4, 0x7),      // Extract bits 4-7
        ((8 << 8) | 0, 0x78),     // Extract bits 0-7
        ((8 << 8) | 8, 0x56),     // Extract bits 8-15
        ((16 << 8) | 0, 0x5678),  // Extract bits 0-15
        ((16 << 8) | 16, 0x1234), // Extract bits 16-31
    ];

    for (params, expected) in &test_cases {
        regs.rcx = *params as u64;
        let (mut vcpu, _) = setup_vm(&code, Some(regs.clone()));
        let result = run_until_hlt(&mut vcpu).unwrap();
        assert_eq!(
            result.rax & 0xFFFFFFFF,
            *expected as u64,
            "Extraction with params {:04x} failed",
            params
        );
    }
}

#[test]
fn test_bextr_comprehensive_64bit() {
    // Comprehensive 64-bit test
    let code = [
        0xc4, 0xe2, 0xf0, 0xf7, 0xc3, // BEXTR RAX, RBX, RCX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x0123456789ABCDEF;
    // In little-endian byte order: EF, CD, AB, 89, 67, 45, 23, 01
    // 0x0123456789ABCDEF >> 32 = 0x01234567
    // (0x01234567 >> 0) & 0xFF = 0x67

    let test_cases = [
        ((8 << 8) | 0, 0xEF),         // Low byte
        ((8 << 8) | 32, 0x67),        // Byte at 32-bit boundary: (val >> 32) & 0xFF = 0x67
        ((16 << 8) | 32, 0x4567),     // 16 bits starting at 32-bit boundary
        ((32 << 8) | 32, 0x01234567), // High 32 bits
        ((8 << 8) | 56, 0x01),        // High byte
    ];

    for (params, expected) in &test_cases {
        regs.rcx = *params as u64;
        let (mut vcpu, _) = setup_vm(&code, Some(regs.clone()));
        let result = run_until_hlt(&mut vcpu).unwrap();
        assert_eq!(
            result.rax, *expected,
            "64-bit extraction with params {:04x} failed",
            params
        );
    }
}

#[test]
fn test_bextr_variable_length_extraction() {
    // Test with various length values
    let code = [
        0xc4, 0xe2, 0x70, 0xf7, 0xc3, // BEXTR EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xFEDCBA98;

    for len in 0..=32 {
        regs.rcx = (len << 8) | 0;
        let (mut vcpu, _) = setup_vm(&code, Some(regs.clone()));
        let result = run_until_hlt(&mut vcpu).unwrap();

        let expected = if len == 0 {
            0
        } else if len >= 32 {
            0xFEDCBA98
        } else {
            0xFEDCBA98 & ((1u32 << len) - 1)
        };

        assert_eq!(
            result.rax & 0xFFFFFFFF,
            expected as u64,
            "Variable length extraction failed"
        );
    }
}

#[test]
fn test_bextr_variable_start_position() {
    // Test with various start positions
    let code = [
        0xc4, 0xe2, 0x70, 0xf7, 0xc3, // BEXTR EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x12345678;

    for start in 0..32 {
        regs.rcx = (8 << 8) | start;
        let (mut vcpu, _) = setup_vm(&code, Some(regs.clone()));
        let result = run_until_hlt(&mut vcpu).unwrap();

        let expected = (0x12345678u32 >> start) & 0xFF;
        assert_eq!(
            result.rax & 0xFFFFFFFF,
            expected as u64,
            "Variable start extraction failed"
        );
    }
}

#[test]
fn test_bextr_cross_word_extraction() {
    // Test extraction across word boundaries (16-bit boundaries)
    let code = [
        0xc4, 0xe2, 0x70, 0xf7, 0xc3, // BEXTR EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xDEADBEEFu64;
    // 0xDEADBEEF >> 12 & 0xFF = 0xDB
    // 0xDEADBEEF >> 14 & 0xFF = 0xB6
    // 0xDEADBEEF >> 10 & 0xFFF = 0xB6F

    let test_cases = [
        ((8 << 8) | 12, 0xDB),   // 8 bits at pos 12: (0xDEADBEEF >> 12) & 0xFF
        ((8 << 8) | 14, 0xB6),   // 8 bits at pos 14: (0xDEADBEEF >> 14) & 0xFF
        ((12 << 8) | 10, 0xB6F), // 12 bits at pos 10: (0xDEADBEEF >> 10) & 0xFFF
    ];

    for (params, expected) in &test_cases {
        regs.rcx = *params as u64;
        let (mut vcpu, _) = setup_vm(&code, Some(regs.clone()));
        let result = run_until_hlt(&mut vcpu).unwrap();
        assert_eq!(
            result.rax & 0xFFFFFFFF,
            *expected as u64,
            "Cross-word extraction"
        );
    }
}

#[test]
fn test_bextr_extract_sign_extension() {
    // Extract sign bits
    let code = [
        0xc4, 0xe2, 0x70, 0xf7, 0xc3, // BEXTR EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x80000000;
    regs.rcx = (1 << 8) | 31; // Extract bit 31
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let result = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(result.rax & 0xFFFFFFFF, 1, "Sign bit extraction");
}

#[test]
fn test_bextr_multi_byte_sequential() {
    // Extract multiple bytes sequentially
    let code = [
        0xc4, 0xe2, 0x70, 0xf7, 0xc3, // BEXTR EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x12345678;

    let test_cases = [
        ((8 << 8) | 0, 0x78u32),  // Extract byte 0
        ((8 << 8) | 8, 0x56u32),  // Extract byte 1
        ((8 << 8) | 16, 0x34u32), // Extract byte 2
        ((8 << 8) | 24, 0x12u32), // Extract byte 3
    ];

    for (params, expected) in &test_cases {
        regs.rcx = *params as u64;
        let (mut vcpu, _) = setup_vm(&code, Some(regs.clone()));
        let result = run_until_hlt(&mut vcpu).unwrap();
        assert_eq!(
            result.rax & 0xFFFFFFFF,
            *expected as u64,
            "Multi-byte sequential extraction"
        );
    }
}

#[test]
fn test_bextr_with_r8_r10() {
    // Test using R8, R10 as source, R9 as control
    // VEX: R~=0, X~=1, B~=0, vvvv=0110 (R9 inverted from 9)
    let code = [
        0xc4, 0x42, 0x30, 0xf7, 0xc2, // BEXTR R8D, R10D, R9D (vvvv=0110=R9, rm=2+B=R10)
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.r10 = 0xABCD1234;
    regs.r9 = (8 << 8) | 4;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let result = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(result.r8 & 0xFFFFFFFF, 0x23, "R8/R10 extraction");
}

#[test]
fn test_bextr_all_ones_pattern() {
    // BEXTR with all ones gives all 1s in extract region
    let code = [
        0xc4, 0xe2, 0x70, 0xf7, 0xc3, // BEXTR EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xFFFFFFFF;
    regs.rcx = (16 << 8) | 8;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let result = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        result.rax & 0xFFFFFFFF,
        0xFFFF,
        "All ones pattern extraction"
    );
}

#[test]
fn test_bextr_all_zeros_pattern() {
    // BEXTR with all zeros gives zero
    let code = [
        0xc4, 0xe2, 0x70, 0xf7, 0xc3, // BEXTR EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x00000000;
    regs.rcx = (16 << 8) | 8;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let result = run_until_hlt(&mut vcpu).unwrap();

    assert!(
        zf_set(result.rflags),
        "All zeros pattern extraction should set ZF"
    );
    assert_eq!(
        result.rax & 0xFFFFFFFF,
        0,
        "All zeros pattern extraction result"
    );
}

#[test]
fn test_bextr_practical_struct_fields() {
    // Extract fields from a packed structure
    let code = [
        0xc4, 0xe2, 0x70, 0xf7, 0xc3, // BEXTR EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    // Simulate a packed structure: [bits 0-3: type], [bits 4-7: flags], [bits 8-15: id], [bits 16-31: data]
    regs.rbx = 0xABCD1234;

    // Extract type (bits 0-3)
    regs.rcx = (4 << 8) | 0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs.clone()));
    let result = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(result.rax & 0xFFFFFFFF, 0x4, "Extract type field");

    // Extract flags (bits 4-7)
    regs.rcx = (4 << 8) | 4;
    let (mut vcpu, _) = setup_vm(&code, Some(regs.clone()));
    let result = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(result.rax & 0xFFFFFFFF, 0x3, "Extract flags field");

    // Extract id (bits 8-15)
    regs.rcx = (8 << 8) | 8;
    let (mut vcpu, _) = setup_vm(&code, Some(regs.clone()));
    let result = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(result.rax & 0xFFFFFFFF, 0x12, "Extract id field");

    // Extract data (bits 16-31)
    regs.rcx = (16 << 8) | 16;
    let (mut vcpu, _) = setup_vm(&code, Some(regs.clone()));
    let result = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(result.rax & 0xFFFFFFFF, 0xABCD, "Extract data field");
}

#[test]
fn test_bextr_alternating_patterns() {
    // Test with alternating bit patterns
    let code = [
        0xc4, 0xe2, 0x70, 0xf7, 0xc3, // BEXTR EAX, EBX, ECX
        0xf4,
    ];
    let test_cases = [
        (0xAAAAAAAAu32, (4 << 8) | 0, 0xA),  // Extract 0xA
        (0x55555555u32, (4 << 8) | 0, 0x5),  // Extract 0x5
        (0xAAAAAAAAu32, (8 << 8) | 0, 0xAA), // Extract 0xAA
    ];

    for (value, params, expected) in &test_cases {
        let mut regs = Registers::default();
        regs.rbx = *value as u64;
        regs.rcx = *params as u64;
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let result = run_until_hlt(&mut vcpu).unwrap();
        assert_eq!(
            result.rax & 0xFFFFFFFF,
            *expected as u64,
            "Alternating pattern extraction"
        );
    }
}

#[test]
fn test_bextr_offset_length_combinations() {
    // Test various combinations of offset and length
    let code = [
        0xc4, 0xe2, 0x70, 0xf7, 0xc3, // BEXTR EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xCDEF0123u64;
    // 0xCDEF0123 = 0b11001101111011110000000100100011
    // bit 0 = 1, bit 31 = 1
    // bits 0-1 = 0b11 = 3, bits 30-31 = 0b11 = 3
    // bits 0-2 = 0b011 = 3, bits 29-31 = 0b110 = 6

    let test_cases = [
        ((1 << 8) | 0, 0x01),  // 1 bit at 0: bit 0 = 1
        ((1 << 8) | 31, 0x01), // 1 bit at 31: bit 31 = 1
        ((2 << 8) | 0, 0x03),  // 2 bits at 0: bits 0-1 = 0b11
        ((2 << 8) | 30, 0x03), // 2 bits at 30: bits 30-31 = 0b11
        ((3 << 8) | 0, 0x03),  // 3 bits at 0: bits 0-2 = 0b011
        ((3 << 8) | 29, 0x06), // 3 bits at 29: bits 29-31 = 0b110
    ];

    for (params, expected) in &test_cases {
        regs.rcx = *params as u64;
        let (mut vcpu, _) = setup_vm(&code, Some(regs.clone()));
        let result = run_until_hlt(&mut vcpu).unwrap();
        assert_eq!(
            result.rax & 0xFFFFFFFF,
            *expected as u64,
            "Offset/length combination"
        );
    }
}
