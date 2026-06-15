use crate::common::*;
use rax::cpu::Registers;

// TZCNT - Count Number of Trailing Zero Bits (BMI1)
// Counts the number of trailing zero bits in the source operand.
// If the source is zero, the result is the operand size in bits.
// Sets CF if source is zero, clears CF otherwise.
// Sets ZF if result is zero, clears ZF otherwise.
//
// Opcodes:
// F3 0F BC /r           TZCNT r16, r/m16   - Count trailing zeros (16-bit)
// F3 0F BC /r           TZCNT r32, r/m32   - Count trailing zeros (32-bit)
// F3 REX.W 0F BC /r     TZCNT r64, r/m64   - Count trailing zeros (64-bit)

#[test]
fn test_tzcnt_bit_0() {
    // TZCNT EAX, EBX - bit 0 set (0 trailing zeros)
    let code = [
        0xf3, 0x0f, 0xbc, 0xc3, // TZCNT EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0b0001; // bit 0 set
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 0, "0 trailing zeros");
    assert!(
        !cf_set(regs.rflags),
        "CF should be clear (source is non-zero)"
    );
    assert!(zf_set(regs.rflags), "ZF should be set (result is zero)");
}

#[test]
fn test_tzcnt_bit_1() {
    // TZCNT EAX, EBX - bit 1 set (1 trailing zero)
    let code = [
        0xf3, 0x0f, 0xbc, 0xc3, // TZCNT EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0b0010; // bit 1 set
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 1, "1 trailing zero");
    assert!(!cf_set(regs.rflags), "CF should be clear");
}

#[test]
fn test_tzcnt_bit_31() {
    // TZCNT EAX, EBX - only bit 31 set
    let code = [
        0xf3, 0x0f, 0xbc, 0xc3, // TZCNT EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x80000000; // only bit 31 set
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 31, "31 trailing zeros");
    assert!(!cf_set(regs.rflags), "CF should be clear");
}

#[test]
fn test_tzcnt_zero_source_32bit() {
    // TZCNT with zero source (32-bit)
    let code = [
        0xf3, 0x0f, 0xbc, 0xc3, // TZCNT EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0; // zero source
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        32,
        "32 trailing zeros for zero source"
    );
    assert!(cf_set(regs.rflags), "CF should be set (source is zero)");
    assert!(
        !zf_set(regs.rflags),
        "ZF should be clear (result is non-zero)"
    );
}

#[test]
fn test_tzcnt_64bit_basic() {
    // TZCNT RAX, RBX - 64-bit version
    let code = [
        0xf3, 0x48, 0x0f, 0xbc, 0xc3, // TZCNT RAX, RBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0b1000; // bit 3 set
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 3, "3 trailing zeros");
    assert!(!cf_set(regs.rflags), "CF should be clear");
}

#[test]
fn test_tzcnt_64bit_bit_63() {
    // TZCNT RAX, RBX - only bit 63 set
    let code = [
        0xf3, 0x48, 0x0f, 0xbc, 0xc3, // TZCNT RAX, RBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x8000_0000_0000_0000; // only bit 63 set
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 63, "63 trailing zeros");
    assert!(!cf_set(regs.rflags), "CF should be clear");
}

#[test]
fn test_tzcnt_zero_source_64bit() {
    // TZCNT with zero source (64-bit)
    let code = [
        0xf3, 0x48, 0x0f, 0xbc, 0xc3, // TZCNT RAX, RBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0; // zero source
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 64, "64 trailing zeros for zero source");
    assert!(cf_set(regs.rflags), "CF should be set");
    assert!(
        !zf_set(regs.rflags),
        "ZF should be clear (result is non-zero)"
    );
}

#[test]
fn test_tzcnt_all_bits_set_32bit() {
    // TZCNT with all bits set
    let code = [
        0xf3, 0x0f, 0xbc, 0xc3, // TZCNT EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xFFFFFFFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 0, "0 trailing zeros");
    assert!(!cf_set(regs.rflags), "CF should be clear");
}

#[test]
fn test_tzcnt_all_bits_set_64bit() {
    // TZCNT with all bits set (64-bit)
    let code = [
        0xf3, 0x48, 0x0f, 0xbc, 0xc3, // TZCNT RAX, RBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xFFFFFFFFFFFFFFFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0, "0 trailing zeros");
    assert!(!cf_set(regs.rflags), "CF should be clear");
}

#[test]
fn test_tzcnt_single_bit_positions_32bit() {
    // Test each bit position for 32-bit
    for bit_pos in 0..32 {
        let code = [
            0xf3, 0x0f, 0xbc, 0xc3, // TZCNT EAX, EBX
            0xf4,
        ];
        let mut regs = Registers::default();
        regs.rbx = 1u64 << bit_pos;
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        assert_eq!(
            regs.rax & 0xFFFFFFFF,
            bit_pos as u64,
            "TZCNT for bit {}",
            bit_pos
        );
        assert!(
            !cf_set(regs.rflags),
            "CF should be clear for bit {}",
            bit_pos
        );
    }
}

#[test]
fn test_tzcnt_single_bit_positions_64bit() {
    // Test each bit position for 64-bit
    for bit_pos in 0..64 {
        let code = [
            0xf3, 0x48, 0x0f, 0xbc, 0xc3, // TZCNT RAX, RBX
            0xf4,
        ];
        let mut regs = Registers::default();
        regs.rbx = 1u64 << bit_pos;
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        assert_eq!(regs.rax, bit_pos as u64, "TZCNT for bit {}", bit_pos);
        assert!(
            !cf_set(regs.rflags),
            "CF should be clear for bit {}",
            bit_pos
        );
    }
}

#[test]
fn test_tzcnt_alternating_pattern() {
    // TZCNT with alternating pattern 1010...1010
    let code = [
        0xf3, 0x0f, 0xbc, 0xc3, // TZCNT EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xAAAAAAAA; // 1010...1010 (bit 0 is clear, bit 1 is lowest set)
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 1, "1 trailing zero");
}

#[test]
fn test_tzcnt_alternating_pattern_inverted() {
    // TZCNT with alternating pattern 0101...0101
    let code = [
        0xf3, 0x0f, 0xbc, 0xc3, // TZCNT EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x55555555; // 0101...0101 (bit 0 is set)
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 0, "0 trailing zeros");
}

#[test]
fn test_tzcnt_extended_registers_r8_r9() {
    // TZCNT R8D, R9D
    let code = [
        0xf3, 0x45, 0x0f, 0xbc, 0xc1, // TZCNT R8D, R9D
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.r9 = 0x100; // bit 8 set
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.r8 & 0xFFFFFFFF, 8, "8 trailing zeros");
}

#[test]
fn test_tzcnt_r15_r14() {
    // TZCNT R15, R14
    let code = [
        0xf3, 0x4d, 0x0f, 0xbc, 0xfe, // TZCNT R15, R14
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.r14 = 0x1_0000_0000; // bit 32 set
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.r15, 32, "32 trailing zeros");
}

#[test]
fn test_tzcnt_mem32() {
    // TZCNT EAX, [mem]
    let code = [
        0xf3, 0x0f, 0xbc, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // TZCNT EAX, [DATA_ADDR]
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u32(&mem, 0xFFFFF000); // bits 12-31 set
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 12, "12 trailing zeros");
}

#[test]
fn test_tzcnt_mem64() {
    // TZCNT RAX, [mem]
    let code = [
        0xf3, 0x48, 0x0f, 0xbc, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // TZCNT RAX, [DATA_ADDR]
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u64(&mem, 0x100_0000_0000); // bit 40 set
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 40, "40 trailing zeros");
}

#[test]
fn test_tzcnt_preserves_source() {
    // TZCNT should not modify source operand
    let code = [
        0xf3, 0x0f, 0xbc, 0xc3, // TZCNT EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x12345678;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx & 0xFFFFFFFF, 0x12345678, "EBX should be unchanged");
}

#[test]
fn test_tzcnt_power_of_two() {
    // TZCNT of power of two
    for i in 0..32 {
        let code = [
            0xf3, 0x0f, 0xbc, 0xc3, // TZCNT EAX, EBX
            0xf4,
        ];
        let mut regs = Registers::default();
        regs.rbx = 1u64 << i;
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        assert_eq!(regs.rax & 0xFFFFFFFF, i as u64, "TZCNT(2^{}) = {}", i, i);
    }
}

#[test]
fn test_tzcnt_consecutive_bits_from_bit_0() {
    // TZCNT with consecutive bits starting from bit 0
    let code = [
        0xf3, 0x0f, 0xbc, 0xc3, // TZCNT EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x0000FFFF; // bits 0-15 set
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 0, "0 trailing zeros");
}

#[test]
fn test_tzcnt_consecutive_bits_higher() {
    // TZCNT with consecutive bits starting higher
    let code = [
        0xf3, 0x0f, 0xbc, 0xc3, // TZCNT EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x00FF0000; // bits 16-23 set
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 16, "16 trailing zeros");
}

#[test]
fn test_tzcnt_byte_boundaries() {
    // Test counting at byte boundaries
    let test_cases = [
        (0x00000100u32, 8u32),  // bit 8
        (0x00010000u32, 16u32), // bit 16
        (0x01000000u32, 24u32), // bit 24
    ];

    for (input, expected) in &test_cases {
        let code = [
            0xf3, 0x0f, 0xbc, 0xc3, // TZCNT EAX, EBX
            0xf4,
        ];
        let mut regs = Registers::default();
        regs.rbx = *input as u64;
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        assert_eq!(
            regs.rax & 0xFFFFFFFF,
            *expected as u64,
            "TZCNT({:08x}) = {}",
            input,
            expected
        );
    }
}

#[test]
fn test_tzcnt_mixed_pattern() {
    // TZCNT with mixed bit pattern
    let code = [
        0xf3, 0x0f, 0xbc, 0xc3, // TZCNT EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x12345678;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // 0x12345678 = 0001 0010 0011 0100 0101 0110 0111 1000
    // Lowest set bit is bit 3
    assert_eq!(regs.rax & 0xFFFFFFFF, 3, "3 trailing zeros for 0x12345678");
}

#[test]
fn test_tzcnt_odd_numbers() {
    // Odd numbers always have 0 trailing zeros
    let odd_values = [1u32, 3, 7, 15, 31, 63, 127, 255, 511, 1023];

    for value in &odd_values {
        let code = [
            0xf3, 0x0f, 0xbc, 0xc3, // TZCNT EAX, EBX
            0xf4,
        ];
        let mut regs = Registers::default();
        regs.rbx = *value as u64;
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        assert_eq!(
            regs.rax & 0xFFFFFFFF,
            0,
            "TZCNT({}) = 0 (odd number)",
            value
        );
    }
}

#[test]
fn test_tzcnt_even_numbers() {
    // Even numbers have at least 1 trailing zero
    let test_cases = [
        (2u32, 1u32),
        (4u32, 2u32),
        (8u32, 3u32),
        (16u32, 4u32),
        (32u32, 5u32),
        (64u32, 6u32),
        (128u32, 7u32),
        (256u32, 8u32),
    ];

    for (value, expected) in &test_cases {
        let code = [
            0xf3, 0x0f, 0xbc, 0xc3, // TZCNT EAX, EBX
            0xf4,
        ];
        let mut regs = Registers::default();
        regs.rbx = *value as u64;
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        assert_eq!(
            regs.rax & 0xFFFFFFFF,
            *expected as u64,
            "TZCNT({}) = {}",
            value,
            expected
        );
    }
}

#[test]
fn test_tzcnt_high_bits_64() {
    // TZCNT with high bits in 64-bit operand
    let code = [
        0xf3, 0x48, 0x0f, 0xbc, 0xc3, // TZCNT RAX, RBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x0800_0000_0000_0000; // bit 59 set
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 59, "59 trailing zeros");
}

#[test]
fn test_tzcnt_multiple_set_bits() {
    // TZCNT with multiple bits set (counts to first set bit)
    let code = [
        0xf3, 0x0f, 0xbc, 0xc3, // TZCNT EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xFFFFF000; // bits 12-31 set
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        12,
        "12 trailing zeros (first set bit)"
    );
}

#[test]
fn test_tzcnt_sign_bit_32() {
    // TZCNT with sign bit set (32-bit)
    let code = [
        0xf3, 0x0f, 0xbc, 0xc3, // TZCNT EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x80000000; // sign bit set
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 31, "31 trailing zeros");
}

#[test]
fn test_tzcnt_sign_bit_64() {
    // TZCNT with sign bit set (64-bit)
    let code = [
        0xf3, 0x48, 0x0f, 0xbc, 0xc3, // TZCNT RAX, RBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x8000_0000_0000_0000; // sign bit set
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 63, "63 trailing zeros");
}

#[test]
fn test_tzcnt_combined_with_blsi() {
    // TZCNT gives position of bit isolated by BLSI
    let code = [
        0xf3, 0x0f, 0xbc, 0xc3, // TZCNT EAX, EBX
        0xf4,
    ];
    let test_values = [0x1000u32, 0x2000, 0x4000, 0x8000];

    for value in &test_values {
        let mut regs = Registers::default();
        regs.rbx = *value as u64;
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        let expected = value.trailing_zeros();
        assert_eq!(
            regs.rax & 0xFFFFFFFF,
            expected as u64,
            "TZCNT({:04x})",
            value
        );
    }
}

#[test]
fn test_tzcnt_bitboard_applications() {
    // Chess bitboard - find least significant piece
    let code = [
        0xf3, 0x48, 0x0f, 0xbc, 0xc3, // TZCNT RAX, RBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x0000_0000_0000_0042; // pieces at positions 1 and 6
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 1, "First piece at position 1");
}

#[test]
fn test_tzcnt_alignment_check() {
    // Use TZCNT to check alignment
    let test_cases = [
        (0x1000u32, 12u32),  // 4KB aligned
        (0x2000u32, 13u32),  // 8KB aligned
        (0x10000u32, 16u32), // 64KB aligned
    ];

    for (value, expected) in &test_cases {
        let code = [
            0xf3, 0x0f, 0xbc, 0xc3, // TZCNT EAX, EBX
            0xf4,
        ];
        let mut regs = Registers::default();
        regs.rbx = *value as u64;
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        assert_eq!(
            regs.rax & 0xFFFFFFFF,
            *expected as u64,
            "Alignment of {:04x}",
            value
        );
    }
}

#[test]
fn test_tzcnt_nibble_positions() {
    // Test trailing zeros for nibble-aligned values
    let code = [
        0xf3, 0x0f, 0xbc, 0xc3, // TZCNT EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x00F00000; // nibble at bits 20-23
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 20, "20 trailing zeros");
}

#[test]
fn test_tzcnt_16bit() {
    // TZCNT AX, BX - 16-bit version
    let code = [
        0x66, 0xf3, 0x0f, 0xbc, 0xc3, // TZCNT AX, BX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x1000; // bit 12 set
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFF, 12, "12 trailing zeros (16-bit)");
}

#[test]
fn test_tzcnt_16bit_zero() {
    // TZCNT AX, BX - 16-bit zero
    let code = [
        0x66, 0xf3, 0x0f, 0xbc, 0xc3, // TZCNT AX, BX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0; // zero
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFF, 16, "16 trailing zeros for zero (16-bit)");
    assert!(cf_set(regs.rflags), "CF should be set");
    assert!(
        !zf_set(regs.rflags),
        "ZF should be clear (result is non-zero)"
    );
}

#[test]
fn test_tzcnt_flags_non_zero() {
    // Test flag behavior for non-zero source
    let code = [
        0xf3, 0x0f, 0xbc, 0xc3, // TZCNT EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x12345678;
    regs.rflags = 0x2 | (1 << 0) | (1 << 6); // Set CF and ZF initially
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(!cf_set(regs.rflags), "CF should be clear for non-zero");
    assert!(!zf_set(regs.rflags), "ZF should be clear for non-zero");
}

#[test]
fn test_tzcnt_iterative_bit_scan() {
    // Use TZCNT to iterate through set bits
    let value = 0b10101010u32;
    let expected_positions = [1, 3, 5, 7];

    for &expected_pos in &expected_positions {
        let code = [
            0xf3, 0x0f, 0xbc, 0xc3, // TZCNT EAX, EBX
            0xf4,
        ];
        let mut regs = Registers::default();
        regs.rbx = (value
            >> expected_positions
                .iter()
                .position(|&p| p == expected_pos)
                .unwrap()) as u64;
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        // Each iteration should find the next set bit
        let shift_amount = expected_positions
            .iter()
            .position(|&p| p == expected_pos)
            .unwrap();
        let shifted_value = value >> shift_amount;
        assert_eq!(
            regs.rax & 0xFFFFFFFF,
            (shifted_value.trailing_zeros()) as u64
        );
    }
}

#[test]
fn test_tzcnt_max_value_32bit() {
    // TZCNT with maximum value 32-bit
    let code = [
        0xf3, 0x0f, 0xbc, 0xc3, // TZCNT EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xFFFFFFFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 0, "0 trailing zeros");
}

#[test]
fn test_tzcnt_max_value_64bit() {
    // TZCNT with maximum value 64-bit
    let code = [
        0xf3, 0x48, 0x0f, 0xbc, 0xc3, // TZCNT RAX, RBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xFFFFFFFFFFFFFFFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0, "0 trailing zeros");
}

#[test]
fn test_tzcnt_sparse_bits() {
    // TZCNT with sparse bit pattern
    let code = [
        0xf3, 0x48, 0x0f, 0xbc, 0xc3, // TZCNT RAX, RBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x8000_0000_0000_0100; // bits 8 and 63
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 8, "8 trailing zeros (first set bit)");
}

#[test]
fn test_tzcnt_complement_lzcnt() {
    // TZCNT and LZCNT are complementary operations
    // For power of 2, TZCNT(x) + LZCNT(x) + 1 = bit_width
    for i in 0..32 {
        let code = [
            0xf3, 0x0f, 0xbc, 0xc3, // TZCNT EAX, EBX
            0xf4,
        ];
        let mut regs = Registers::default();
        regs.rbx = 1u64 << i;
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        assert_eq!(regs.rax & 0xFFFFFFFF, i as u64);
    }
}

#[test]
fn test_tzcnt_all_registers_32bit() {
    // Test various register combinations
    let code1 = [
        0xf3, 0x0f, 0xbc, 0xd1, // TZCNT EDX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rcx = 0x1000;
    let (mut vcpu, _) = setup_vm(&code1, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rdx & 0xFFFFFFFF, 12);

    let code2 = [
        0xf3, 0x0f, 0xbc, 0xfb, // TZCNT EDI, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x200;
    let (mut vcpu, _) = setup_vm(&code2, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rdi & 0xFFFFFFFF, 9);
}

#[test]
fn test_tzcnt_binary_search_helper() {
    // TZCNT can help with binary search on bits
    let code = [
        0xf3, 0x0f, 0xbc, 0xc3, // TZCNT EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x00080000; // bit 19
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 19);
}

#[test]
fn test_tzcnt_modulo_power_of_two() {
    // TZCNT can help determine divisibility by powers of 2
    let test_cases = [
        (8u32, 3u32),     // divisible by 2^3
        (16u32, 4u32),    // divisible by 2^4
        (32u32, 5u32),    // divisible by 2^5
        (1024u32, 10u32), // divisible by 2^10
    ];

    for (value, expected) in &test_cases {
        let code = [
            0xf3, 0x0f, 0xbc, 0xc3, // TZCNT EAX, EBX
            0xf4,
        ];
        let mut regs = Registers::default();
        regs.rbx = *value as u64;
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        assert_eq!(
            regs.rax & 0xFFFFFFFF,
            *expected as u64,
            "{} is divisible by 2^{}",
            value,
            expected
        );
    }
}

#[test]
fn test_tzcnt_gray_code_helper() {
    // TZCNT used in Gray code algorithms
    let code = [
        0xf3, 0x0f, 0xbc, 0xc3, // TZCNT EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x01010100; // pattern
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 8);
}

#[test]
fn test_tzcnt_64bit_word_boundaries() {
    // Test at 64-bit word boundaries
    let test_cases = [
        (0x0000_0001_0000_0000u64, 32u64),
        (0x0000_0100_0000_0000u64, 40u64),
        (0x0001_0000_0000_0000u64, 48u64),
        (0x0100_0000_0000_0000u64, 56u64),
    ];

    for (value, expected) in &test_cases {
        let code = [
            0xf3, 0x48, 0x0f, 0xbc, 0xc3, // TZCNT RAX, RBX
            0xf4,
        ];
        let mut regs = Registers::default();
        regs.rbx = *value;
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        assert_eq!(regs.rax, *expected, "TZCNT({:016x}) = {}", value, expected);
    }
}
