use crate::common::*;
use rax::cpu::Registers;

// LZCNT - Count Number of Leading Zero Bits (LZCNT/ABM)
// Counts the number of leading zero bits in the source operand.
// If the source is zero, the result is the operand size in bits.
// Sets CF if source is zero, clears CF otherwise.
// Sets ZF if result is zero, clears ZF otherwise.
//
// Opcodes:
// F3 0F BD /r           LZCNT r16, r/m16   - Count leading zeros (16-bit)
// F3 0F BD /r           LZCNT r32, r/m32   - Count leading zeros (32-bit)
// F3 REX.W 0F BD /r     LZCNT r64, r/m64   - Count leading zeros (64-bit)

#[test]
fn test_lzcnt_bit_31() {
    // LZCNT EAX, EBX - bit 31 set (0 leading zeros)
    let code = [
        0xf3, 0x0f, 0xbd, 0xc3, // LZCNT EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x80000000; // bit 31 set
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 0, "0 leading zeros");
    assert!(
        !cf_set(regs.rflags),
        "CF should be clear (source is non-zero)"
    );
    assert!(zf_set(regs.rflags), "ZF should be set (result is zero)");
}

#[test]
fn test_lzcnt_bit_30() {
    // LZCNT EAX, EBX - bit 30 set (1 leading zero)
    let code = [
        0xf3, 0x0f, 0xbd, 0xc3, // LZCNT EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x40000000; // bit 30 set
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 1, "1 leading zero");
    assert!(!cf_set(regs.rflags), "CF should be clear");
}

#[test]
fn test_lzcnt_bit_0() {
    // LZCNT EAX, EBX - only bit 0 set
    let code = [
        0xf3, 0x0f, 0xbd, 0xc3, // LZCNT EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x00000001; // only bit 0 set
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 31, "31 leading zeros");
    assert!(!cf_set(regs.rflags), "CF should be clear");
}

#[test]
fn test_lzcnt_zero_source_32bit() {
    // LZCNT with zero source (32-bit)
    let code = [
        0xf3, 0x0f, 0xbd, 0xc3, // LZCNT EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0; // zero source
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        32,
        "32 leading zeros for zero source"
    );
    assert!(cf_set(regs.rflags), "CF should be set (source is zero)");
    assert!(
        !zf_set(regs.rflags),
        "ZF should be clear (result is non-zero)"
    );
}

#[test]
fn test_lzcnt_64bit_basic() {
    // LZCNT RAX, RBX - 64-bit version
    let code = [
        0xf3, 0x48, 0x0f, 0xbd, 0xc3, // LZCNT RAX, RBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x0010_0000_0000_0000; // bit 52 set
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 11, "11 leading zeros");
    assert!(!cf_set(regs.rflags), "CF should be clear");
}

#[test]
fn test_lzcnt_64bit_bit_63() {
    // LZCNT RAX, RBX - bit 63 set (0 leading zeros)
    let code = [
        0xf3, 0x48, 0x0f, 0xbd, 0xc3, // LZCNT RAX, RBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x8000_0000_0000_0000; // bit 63 set
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0, "0 leading zeros");
    assert!(!cf_set(regs.rflags), "CF should be clear");
}

#[test]
fn test_lzcnt_zero_source_64bit() {
    // LZCNT with zero source (64-bit)
    let code = [
        0xf3, 0x48, 0x0f, 0xbd, 0xc3, // LZCNT RAX, RBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0; // zero source
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 64, "64 leading zeros for zero source");
    assert!(cf_set(regs.rflags), "CF should be set");
    assert!(
        !zf_set(regs.rflags),
        "ZF should be clear (result is non-zero)"
    );
}

#[test]
fn test_lzcnt_all_bits_set_32bit() {
    // LZCNT with all bits set
    let code = [
        0xf3, 0x0f, 0xbd, 0xc3, // LZCNT EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xFFFFFFFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 0, "0 leading zeros");
    assert!(!cf_set(regs.rflags), "CF should be clear");
}

#[test]
fn test_lzcnt_all_bits_set_64bit() {
    // LZCNT with all bits set (64-bit)
    let code = [
        0xf3, 0x48, 0x0f, 0xbd, 0xc3, // LZCNT RAX, RBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xFFFFFFFFFFFFFFFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0, "0 leading zeros");
    assert!(!cf_set(regs.rflags), "CF should be clear");
}

#[test]
fn test_lzcnt_single_bit_positions_32bit() {
    // Test each bit position for 32-bit
    for bit_pos in 0..32 {
        let code = [
            0xf3, 0x0f, 0xbd, 0xc3, // LZCNT EAX, EBX
            0xf4,
        ];
        let mut regs = Registers::default();
        regs.rbx = 1u64 << bit_pos;
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        let expected = 31 - bit_pos;
        assert_eq!(
            regs.rax & 0xFFFFFFFF,
            expected as u64,
            "LZCNT for bit {}",
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
fn test_lzcnt_single_bit_positions_64bit() {
    // Test each bit position for 64-bit
    for bit_pos in 0..64 {
        let code = [
            0xf3, 0x48, 0x0f, 0xbd, 0xc3, // LZCNT RAX, RBX
            0xf4,
        ];
        let mut regs = Registers::default();
        regs.rbx = 1u64 << bit_pos;
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        let expected = 63 - bit_pos;
        assert_eq!(regs.rax, expected as u64, "LZCNT for bit {}", bit_pos);
        assert!(
            !cf_set(regs.rflags),
            "CF should be clear for bit {}",
            bit_pos
        );
    }
}

#[test]
fn test_lzcnt_alternating_pattern() {
    // LZCNT with alternating pattern 1010...1010
    let code = [
        0xf3, 0x0f, 0xbd, 0xc3, // LZCNT EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xAAAAAAAA; // 1010...1010 (bit 31 is set)
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 0, "0 leading zeros (bit 31 set)");
}

#[test]
fn test_lzcnt_alternating_pattern_inverted() {
    // LZCNT with alternating pattern 0101...0101
    let code = [
        0xf3, 0x0f, 0xbd, 0xc3, // LZCNT EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x55555555; // 0101...0101 (bit 30 is highest set)
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 1, "1 leading zero");
}

#[test]
fn test_lzcnt_extended_registers_r8_r9() {
    // LZCNT R8D, R9D
    let code = [
        0xf3, 0x45, 0x0f, 0xbd, 0xc1, // LZCNT R8D, R9D
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.r9 = 0x00100000; // bit 20 set
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.r8 & 0xFFFFFFFF, 11, "11 leading zeros");
}

#[test]
fn test_lzcnt_r15_r14() {
    // LZCNT R15, R14
    let code = [
        0xf3, 0x4d, 0x0f, 0xbd, 0xfe, // LZCNT R15, R14
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.r14 = 0x0000_0001_0000_0000; // bit 32 set
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.r15, 31, "31 leading zeros");
}

#[test]
fn test_lzcnt_mem32() {
    // LZCNT EAX, [mem]
    let code = [
        0xf3, 0x0f, 0xbd, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // LZCNT EAX, [DATA_ADDR]
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u32(&mem, 0x00001000); // bit 12 set
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 19, "19 leading zeros");
}

#[test]
fn test_lzcnt_mem64() {
    // LZCNT RAX, [mem]
    let code = [
        0xf3, 0x48, 0x0f, 0xbd, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // LZCNT RAX, [DATA_ADDR]
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u64(&mem, 0x0000_0100_0000_0000); // bit 40 set
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 23, "23 leading zeros");
}

#[test]
fn test_lzcnt_preserves_source() {
    // LZCNT should not modify source operand
    let code = [
        0xf3, 0x0f, 0xbd, 0xc3, // LZCNT EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x12345678;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx & 0xFFFFFFFF, 0x12345678, "EBX should be unchanged");
}

#[test]
fn test_lzcnt_power_of_two() {
    // LZCNT of power of two
    for i in 0..32 {
        let code = [
            0xf3, 0x0f, 0xbd, 0xc3, // LZCNT EAX, EBX
            0xf4,
        ];
        let mut regs = Registers::default();
        regs.rbx = 1u64 << i;
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        let expected = 31 - i;
        assert_eq!(
            regs.rax & 0xFFFFFFFF,
            expected as u64,
            "LZCNT(2^{}) = {}",
            i,
            expected
        );
    }
}

#[test]
fn test_lzcnt_consecutive_bits_from_bit_31() {
    // LZCNT with consecutive bits starting from bit 31
    let code = [
        0xf3, 0x0f, 0xbd, 0xc3, // LZCNT EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xFFFF0000; // bits 16-31 set
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 0, "0 leading zeros");
}

#[test]
fn test_lzcnt_consecutive_bits_lower() {
    // LZCNT with consecutive bits in lower positions
    let code = [
        0xf3, 0x0f, 0xbd, 0xc3, // LZCNT EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x0000FFFF; // bits 0-15 set
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 16, "16 leading zeros");
}

#[test]
fn test_lzcnt_byte_boundaries() {
    // Test counting at byte boundaries
    let test_cases = [
        (0x01000000u32, 7u32),  // bit 24
        (0x00010000u32, 15u32), // bit 16
        (0x00000100u32, 23u32), // bit 8
    ];

    for (input, expected) in &test_cases {
        let code = [
            0xf3, 0x0f, 0xbd, 0xc3, // LZCNT EAX, EBX
            0xf4,
        ];
        let mut regs = Registers::default();
        regs.rbx = *input as u64;
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        assert_eq!(
            regs.rax & 0xFFFFFFFF,
            *expected as u64,
            "LZCNT({:08x}) = {}",
            input,
            expected
        );
    }
}

#[test]
fn test_lzcnt_mixed_pattern() {
    // LZCNT with mixed bit pattern
    let code = [
        0xf3, 0x0f, 0xbd, 0xc3, // LZCNT EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x12345678;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // 0x12345678 = 0001 0010 0011 0100 0101 0110 0111 1000
    // Highest set bit is bit 28
    assert_eq!(regs.rax & 0xFFFFFFFF, 3, "3 leading zeros for 0x12345678");
}

#[test]
fn test_lzcnt_high_bits_64() {
    // LZCNT with high bits in 64-bit operand
    let code = [
        0xf3, 0x48, 0x0f, 0xbd, 0xc3, // LZCNT RAX, RBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x0800_0000_0000_0000; // bit 59 set
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 4, "4 leading zeros");
}

#[test]
fn test_lzcnt_multiple_set_bits() {
    // LZCNT with multiple bits set (counts to first set bit from MSB)
    let code = [
        0xf3, 0x0f, 0xbd, 0xc3, // LZCNT EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x00000FFF; // bits 0-11 set
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        20,
        "20 leading zeros (highest bit is 11)"
    );
}

#[test]
fn test_lzcnt_sign_bit_32() {
    // LZCNT with sign bit set (32-bit)
    let code = [
        0xf3, 0x0f, 0xbd, 0xc3, // LZCNT EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x80000000; // sign bit set
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 0, "0 leading zeros");
}

#[test]
fn test_lzcnt_sign_bit_64() {
    // LZCNT with sign bit set (64-bit)
    let code = [
        0xf3, 0x48, 0x0f, 0xbd, 0xc3, // LZCNT RAX, RBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x8000_0000_0000_0000; // sign bit set
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0, "0 leading zeros");
}

#[test]
fn test_lzcnt_bitwidth_calculation() {
    // LZCNT can calculate bit width: bitwidth = operand_size - LZCNT(x)
    let test_values = [0x1u32, 0x3, 0x7, 0xF, 0x1F, 0x3F, 0x7F, 0xFF];

    for (idx, value) in test_values.iter().enumerate() {
        let code = [
            0xf3, 0x0f, 0xbd, 0xc3, // LZCNT EAX, EBX
            0xf4,
        ];
        let mut regs = Registers::default();
        regs.rbx = *value as u64;
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        let bitwidth = 32 - (regs.rax & 0xFFFFFFFF);
        assert_eq!(bitwidth, (idx + 1) as u64, "Bitwidth for {:02x}", value);
    }
}

#[test]
fn test_lzcnt_16bit() {
    // LZCNT AX, BX - 16-bit version
    let code = [
        0x66, 0xf3, 0x0f, 0xbd, 0xc3, // LZCNT AX, BX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x1000; // bit 12 set
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFF, 3, "3 leading zeros (16-bit)");
}

#[test]
fn test_lzcnt_16bit_zero() {
    // LZCNT AX, BX - 16-bit zero
    let code = [
        0x66, 0xf3, 0x0f, 0xbd, 0xc3, // LZCNT AX, BX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0; // zero
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFF, 16, "16 leading zeros for zero (16-bit)");
    assert!(cf_set(regs.rflags), "CF should be set");
    assert!(
        !zf_set(regs.rflags),
        "ZF should be clear (result is non-zero)"
    );
}

#[test]
fn test_lzcnt_flags_non_zero() {
    // Test flag behavior for non-zero source
    let code = [
        0xf3, 0x0f, 0xbd, 0xc3, // LZCNT EAX, EBX
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
fn test_lzcnt_normalize_float() {
    // LZCNT used in floating-point normalization
    let code = [
        0xf3, 0x0f, 0xbd, 0xc3, // LZCNT EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x00000800; // bit 11 set
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let shift_amount = regs.rax & 0xFFFFFFFF;
    assert_eq!(shift_amount, 20, "Shift amount for normalization");
}

#[test]
fn test_lzcnt_max_value_32bit() {
    // LZCNT with maximum value 32-bit
    let code = [
        0xf3, 0x0f, 0xbd, 0xc3, // LZCNT EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xFFFFFFFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 0, "0 leading zeros");
}

#[test]
fn test_lzcnt_max_value_64bit() {
    // LZCNT with maximum value 64-bit
    let code = [
        0xf3, 0x48, 0x0f, 0xbd, 0xc3, // LZCNT RAX, RBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xFFFFFFFFFFFFFFFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0, "0 leading zeros");
}

#[test]
fn test_lzcnt_sparse_bits() {
    // LZCNT with sparse bit pattern
    let code = [
        0xf3, 0x48, 0x0f, 0xbd, 0xc3, // LZCNT RAX, RBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x0000_0000_0000_0101; // bits 0, 2, 8
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 55, "55 leading zeros (highest bit is 8)");
}

#[test]
fn test_lzcnt_complement_tzcnt() {
    // For power of 2, LZCNT + TZCNT + 1 = operand_size
    for i in 0..32 {
        let code = [
            0xf3, 0x0f, 0xbd, 0xc3, // LZCNT EAX, EBX
            0xf4,
        ];
        let mut regs = Registers::default();
        regs.rbx = 1u64 << i;
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        let lzcnt = regs.rax & 0xFFFFFFFF;
        let tzcnt = i as u64;
        assert_eq!(lzcnt + tzcnt + 1, 32, "LZCNT + TZCNT + 1 = 32 for 2^{}", i);
    }
}

#[test]
fn test_lzcnt_all_registers_32bit() {
    // Test various register combinations
    let code1 = [
        0xf3, 0x0f, 0xbd, 0xd1, // LZCNT EDX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rcx = 0x00001000;
    let (mut vcpu, _) = setup_vm(&code1, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rdx & 0xFFFFFFFF, 19);

    let code2 = [
        0xf3, 0x0f, 0xbd, 0xfb, // LZCNT EDI, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x00000200;
    let (mut vcpu, _) = setup_vm(&code2, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rdi & 0xFFFFFFFF, 22);
}

#[test]
fn test_lzcnt_log2_floor() {
    // LZCNT can calculate floor(log2(x)) = 31 - LZCNT(x) for 32-bit
    let test_cases = [
        (1u32, 0u32),
        (2u32, 1u32),
        (4u32, 2u32),
        (8u32, 3u32),
        (16u32, 4u32),
        (255u32, 7u32),
        (256u32, 8u32),
        (1024u32, 10u32),
    ];

    for (value, expected_log2) in &test_cases {
        let code = [
            0xf3, 0x0f, 0xbd, 0xc3, // LZCNT EAX, EBX
            0xf4,
        ];
        let mut regs = Registers::default();
        regs.rbx = *value as u64;
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        let log2_floor = 31 - (regs.rax & 0xFFFFFFFF);
        assert_eq!(
            log2_floor, *expected_log2 as u64,
            "floor(log2({})) = {}",
            value, expected_log2
        );
    }
}

#[test]
fn test_lzcnt_nibble_positions() {
    // Test leading zeros for nibble-aligned values
    let code = [
        0xf3, 0x0f, 0xbd, 0xc3, // LZCNT EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x00000F00; // nibble at bits 8-11
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 20, "20 leading zeros");
}

#[test]
fn test_lzcnt_64bit_word_boundaries() {
    // Test at 64-bit word boundaries
    let test_cases = [
        (0x0000_0001_0000_0000u64, 31u64),
        (0x0000_0100_0000_0000u64, 23u64),
        (0x0001_0000_0000_0000u64, 15u64),
        (0x0100_0000_0000_0000u64, 7u64),
    ];

    for (value, expected) in &test_cases {
        let code = [
            0xf3, 0x48, 0x0f, 0xbd, 0xc3, // LZCNT RAX, RBX
            0xf4,
        ];
        let mut regs = Registers::default();
        regs.rbx = *value;
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        assert_eq!(regs.rax, *expected, "LZCNT({:016x}) = {}", value, expected);
    }
}

#[test]
fn test_lzcnt_binary_search() {
    // LZCNT for binary search on bit positions
    let code = [
        0xf3, 0x0f, 0xbd, 0xc3, // LZCNT EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x00080000; // bit 19
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let bit_position = 31 - (regs.rax & 0xFFFFFFFF);
    assert_eq!(bit_position, 19);
}

#[test]
fn test_lzcnt_priority_encoding() {
    // LZCNT for priority encoding (highest priority first)
    let code = [
        0xf3, 0x0f, 0xbd, 0xc3, // LZCNT EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x00101010; // multiple priorities
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let highest_priority = 31 - (regs.rax & 0xFFFFFFFF);
    assert_eq!(highest_priority, 20, "Highest priority at bit 20");
}

#[test]
fn test_lzcnt_compression_helper() {
    // LZCNT for data compression algorithms
    let code = [
        0xf3, 0x48, 0x0f, 0xbd, 0xc3, // LZCNT RAX, RBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x0000_00FF_0000_0000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 24, "Leading zeros for compression");
}

#[test]
fn test_lzcnt_integer_log2_ceiling() {
    // Calculate ceiling(log2(x)) = 32 - LZCNT(x-1) for x > 0
    let test_cases = [
        (1u32, 0u32),
        (2u32, 1u32),
        (3u32, 2u32),
        (5u32, 3u32),
        (9u32, 4u32),
        (17u32, 5u32),
    ];

    for (value, expected_log2_ceil) in &test_cases {
        let code = [
            0xf3, 0x0f, 0xbd, 0xc3, // LZCNT EAX, EBX
            0xf4,
        ];
        let mut regs = Registers::default();
        regs.rbx = (value - 1) as u64;
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        let log2_ceil = 32 - (regs.rax & 0xFFFFFFFF);
        assert_eq!(
            log2_ceil, *expected_log2_ceil as u64,
            "ceil(log2({})) = {}",
            value, expected_log2_ceil
        );
    }
}

#[test]
fn test_lzcnt_find_msb() {
    // Find most significant bit position
    let code = [
        0xf3, 0x0f, 0xbd, 0xc3, // LZCNT EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x12345678;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let msb_pos = 31 - (regs.rax & 0xFFFFFFFF);
    assert_eq!(msb_pos, 28, "MSB position of 0x12345678");
}

#[test]
fn test_lzcnt_network_prefix() {
    // Calculate network prefix length (CIDR)
    let code = [
        0xf3, 0x0f, 0xbd, 0xc3, // LZCNT EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xFF000000; // 255.0.0.0 (/8 network)
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 0, "0 leading zeros for /8 mask");
}

#[test]
fn test_lzcnt_bit_significance() {
    // Determine number of significant bits
    let code = [
        0xf3, 0x48, 0x0f, 0xbd, 0xc3, // LZCNT RAX, RBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x0000_0000_0FFF_FFFF; // 28 significant bits
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let significant_bits = 64 - regs.rax;
    assert_eq!(significant_bits, 28, "28 significant bits");
}

#[test]
fn test_lzcnt_hash_table_sizing() {
    // Calculate hash table size (next power of 2)
    let code = [
        0xf3, 0x0f, 0xbd, 0xc3, // LZCNT EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 100; // need hash table for 100 entries
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let bits_needed = 32 - (regs.rax & 0xFFFFFFFF);
    let next_power_of_2 = 1u64 << bits_needed;
    assert!(next_power_of_2 >= 100, "Next power of 2 >= 100");
}

#[test]
fn test_lzcnt_low_byte() {
    // LZCNT on value with only low byte set
    let code = [
        0xf3, 0x0f, 0xbd, 0xc3, // LZCNT EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x000000FF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 24, "24 leading zeros");
}

#[test]
fn test_lzcnt_high_byte_32() {
    // LZCNT on value with only high byte set (32-bit)
    let code = [
        0xf3, 0x0f, 0xbd, 0xc3, // LZCNT EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xFF000000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 0, "0 leading zeros");
}
