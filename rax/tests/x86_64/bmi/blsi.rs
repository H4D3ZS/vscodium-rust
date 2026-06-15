use crate::common::*;
use rax::cpu::Registers;

// BLSI - Extract Lowest Set Isolated Bit (BMI1)
// Extracts the lowest set bit from the source operand and sets that bit in the destination.
// All other bits in the destination are cleared.
// This is equivalent to: dest = src & -src
// Sets ZF if result is zero, sets CF if source is non-zero, clears OF.
//
// Opcodes:
// VEX.NDD.LZ.0F38.W0 F3 /3   BLSI r32, r/m32   - Extract lowest set bit (32-bit)
// VEX.NDD.LZ.0F38.W1 F3 /3   BLSI r64, r/m64   - Extract lowest set bit (64-bit)

#[test]
fn test_blsi_eax_ebx_bit_0() {
    // BLSI EAX, EBX - extract lowest bit (bit 0)
    let code = [
        0xc4, 0xe2, 0x78, 0xf3, 0xdb, // BLSI EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0b0000_0001; // bit 0 set
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0b0000_0001,
        "EAX should contain isolated bit 0"
    );
    assert!(
        !zf_set(regs.rflags),
        "ZF should be clear (source is non-zero)"
    );
    assert!(cf_set(regs.rflags), "CF should be set (source is non-zero)");
}

#[test]
fn test_blsi_eax_ebx_bit_31() {
    // BLSI EAX, EBX - extract lowest bit (only bit 31 set)
    let code = [
        0xc4, 0xe2, 0x78, 0xf3, 0xdb, // BLSI EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x80000000; // only bit 31 set
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x80000000,
        "EAX should contain isolated bit 31"
    );
    assert!(!zf_set(regs.rflags), "ZF should be clear");
}

#[test]
fn test_blsi_rax_rbx_bit_0() {
    // BLSI RAX, RBX - 64-bit version
    let code = [
        0xc4, 0xe2, 0xf8, 0xf3, 0xdb, // BLSI RAX, RBX (W1)
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0b0000_0001;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0b0000_0001, "RAX should contain isolated bit 0");
    assert!(!zf_set(regs.rflags), "ZF should be clear");
}

#[test]
fn test_blsi_rax_rbx_bit_63() {
    // BLSI RAX, RBX - extract bit 63
    let code = [
        0xc4, 0xe2, 0xf8, 0xf3, 0xdb, // BLSI RAX, RBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x8000_0000_0000_0000; // only bit 63 set
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax, 0x8000_0000_0000_0000,
        "RAX should contain isolated bit 63"
    );
    assert!(!zf_set(regs.rflags), "ZF should be clear");
}

#[test]
fn test_blsi_zero_source() {
    // BLSI with zero source
    let code = [
        0xc4, 0xe2, 0x78, 0xf3, 0xdb, // BLSI EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0; // zero source
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 0, "EAX should be zero");
    assert!(zf_set(regs.rflags), "ZF should be set (source is zero)");
    assert!(!cf_set(regs.rflags), "CF should be clear (source is zero)");
}

#[test]
fn test_blsi_multiple_bits_isolates_lowest() {
    // BLSI should isolate only the lowest set bit
    let code = [
        0xc4, 0xe2, 0x78, 0xf3, 0xdb, // BLSI EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0b1010_1000; // bits 3, 5, 7 set
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0b0000_1000,
        "EAX should contain only bit 3 (lowest)"
    );
    assert!(!zf_set(regs.rflags), "ZF should be clear");
}

#[test]
fn test_blsi_all_bits_set() {
    // BLSI with all bits set should isolate bit 0
    let code = [
        0xc4, 0xe2, 0x78, 0xf3, 0xdb, // BLSI EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xFFFFFFFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 1, "EAX should contain bit 0");
    assert!(!zf_set(regs.rflags), "ZF should be clear");
}

#[test]
fn test_blsi_alternating_pattern() {
    // BLSI with alternating pattern 1010...1010
    let code = [
        0xc4, 0xe2, 0x78, 0xf3, 0xdb, // BLSI EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xAAAAAAAA; // 1010...1010 (bit 1 is lowest set bit)
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 0b10, "EAX should contain bit 1");
    assert!(!zf_set(regs.rflags), "ZF should be clear");
}

#[test]
fn test_blsi_alternating_pattern_inverted() {
    // BLSI with alternating pattern 0101...0101
    let code = [
        0xc4, 0xe2, 0x78, 0xf3, 0xdb, // BLSI EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x55555555; // 0101...0101 (bit 0 is lowest set bit)
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 1, "EAX should contain bit 0");
    assert!(!zf_set(regs.rflags), "ZF should be clear");
}

#[test]
fn test_blsi_single_bit_positions_32bit() {
    // Test each individual bit position for 32-bit
    for bit_pos in 0..32 {
        let code = [
            0xc4, 0xe2, 0x78, 0xf3, 0xdb, // BLSI EAX, EBX
            0xf4,
        ];
        let mut regs = Registers::default();
        regs.rbx = 1u64 << bit_pos;
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        assert_eq!(
            regs.rax & 0xFFFFFFFF,
            1u64 << bit_pos,
            "EAX should contain isolated bit {}",
            bit_pos
        );
        assert!(
            !zf_set(regs.rflags),
            "ZF should be clear for bit {}",
            bit_pos
        );
    }
}

#[test]
fn test_blsi_single_bit_positions_64bit() {
    // Test each individual bit position for 64-bit
    for bit_pos in 0..64 {
        let code = [
            0xc4, 0xe2, 0xf8, 0xf3, 0xdb, // BLSI RAX, RBX
            0xf4,
        ];
        let mut regs = Registers::default();
        regs.rbx = 1u64 << bit_pos;
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        assert_eq!(
            regs.rax,
            1u64 << bit_pos,
            "RAX should contain isolated bit {}",
            bit_pos
        );
        assert!(
            !zf_set(regs.rflags),
            "ZF should be clear for bit {}",
            bit_pos
        );
    }
}

#[test]
fn test_blsi_with_extended_registers() {
    // BLSI R8D, R9D
    let code = [
        0xc4, 0x42, 0x38, 0xf3, 0xd9, // BLSI R8D, R9D (vvvv=0111 inv=8=R8, B=0 for R9)
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.r9 = 0b0001_1000; // bits 3 and 4 set
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.r8 & 0xFFFFFFFF,
        0b0000_1000,
        "R8D should contain bit 3"
    );
    assert!(!zf_set(regs.rflags), "ZF should be clear");
}

#[test]
fn test_blsi_r15() {
    // BLSI R15, R15
    let code = [
        0xc4, 0x42, 0x80, 0xf3, 0xdf, // BLSI R15, R15
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.r15 = 0x1_0000_0000; // bit 32 set
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.r15, 0x1_0000_0000,
        "R15 should contain isolated bit 32"
    );
    assert!(!zf_set(regs.rflags), "ZF should be clear");
}

#[test]
fn test_blsi_mem32() {
    // BLSI EAX, [mem]
    let code = [
        0xc4, 0xe2, 0x78, 0xf3, 0x1c, 0x25, 0x00, 0x20, 0x00, 0x00, // BLSI EAX, [DATA_ADDR]
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u32(&mem, 0xFFFFF000); // bits 12-31 set
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x1000,
        "EAX should contain isolated bit 12"
    );
}

#[test]
fn test_blsi_mem64() {
    // BLSI RAX, [mem]
    let code = [
        0xc4, 0xe2, 0xf8, 0xf3, 0x1c, 0x25, 0x00, 0x20, 0x00, 0x00, // BLSI RAX, [DATA_ADDR]
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u64(&mem, 0x100_0000_0000); // bit 40 set
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax, 0x100_0000_0000,
        "RAX should contain isolated bit 40"
    );
}

#[test]
fn test_blsi_trailing_zeros() {
    // BLSI isolates bit at position of trailing zeros count
    let code = [
        0xc4, 0xe2, 0x78, 0xf3, 0xdb, // BLSI EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xFFFFF000; // 12 trailing zeros
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 0x1000, "EAX should contain bit 12");
}

#[test]
fn test_blsi_sparse_pattern() {
    // BLSI with sparse bit pattern
    let code = [
        0xc4, 0xe2, 0x78, 0xf3, 0xdb, // BLSI EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x80001000; // bits 12 and 31 set
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x1000,
        "EAX should contain bit 12 (lowest)"
    );
}

#[test]
fn test_blsi_preserves_source() {
    // BLSI should not modify source operand
    let code = [
        0xc4, 0xe2, 0x78, 0xf3, 0xdb, // BLSI EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x12345678;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx & 0xFFFFFFFF, 0x12345678, "EBX should be unchanged");
}

#[test]
fn test_blsi_vs_and_neg() {
    // BLSI is equivalent to src & -src
    let code = [
        0xc4, 0xe2, 0x78, 0xf3, 0xdb, // BLSI EAX, EBX
        0xf4,
    ];
    let value = 0x12345678u32;
    let mut regs = Registers::default();
    regs.rbx = value as u64;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let expected = value & value.wrapping_neg();
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        expected as u64,
        "BLSI should equal src & -src"
    );
}

#[test]
fn test_blsi_power_of_two() {
    // BLSI of power of two returns itself
    for i in 0..32 {
        let code = [
            0xc4, 0xe2, 0x78, 0xf3, 0xdb, // BLSI EAX, EBX
            0xf4,
        ];
        let mut regs = Registers::default();
        regs.rbx = 1u64 << i;
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        assert_eq!(
            regs.rax & 0xFFFFFFFF,
            1u64 << i,
            "BLSI(2^{}) should equal 2^{}",
            i,
            i
        );
    }
}

#[test]
fn test_blsi_consecutive_bits() {
    // BLSI with consecutive bits set
    let code = [
        0xc4, 0xe2, 0x78, 0xf3, 0xdb, // BLSI EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x00FF0000; // bits 16-23 set
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x00010000,
        "EAX should contain bit 16 (lowest of consecutive)"
    );
}

#[test]
fn test_blsi_sign_bit() {
    // BLSI with sign bit set
    let code = [
        0xc4, 0xe2, 0x78, 0xf3, 0xdb, // BLSI EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x80000000; // sign bit set (bit 31)
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x80000000,
        "EAX should contain bit 31"
    );
}

#[test]
fn test_blsi_iterative_isolation() {
    // Use BLSI to iterate through set bits
    let code = [
        0xc4, 0xe2, 0x78, 0xf3, 0xdb, // BLSI EAX, EBX
        0xf4,
    ];

    let mut value = 0b1010_1010u64;
    let expected_bits = vec![1, 3, 5, 7];

    for &expected_bit in &expected_bits {
        let mut regs = Registers::default();
        regs.rbx = value;
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        assert_eq!(
            regs.rax & 0xFFFFFFFF,
            1u64 << expected_bit,
            "Should isolate bit {}",
            expected_bit
        );

        // Remove lowest bit for next iteration
        value = value & (value - 1);
    }
}

#[test]
fn test_blsi_high_bits_64() {
    // BLSI with high bits in 64-bit operand
    let code = [
        0xc4, 0xe2, 0xf8, 0xf3, 0xdb, // BLSI RAX, RBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x0800_0000_0000_0000; // bit 59 set
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax, 0x0800_0000_0000_0000,
        "RAX should contain isolated bit 59"
    );
}

#[test]
fn test_blsi_mixed_high_low() {
    // BLSI with both high and low bits, should isolate lowest
    let code = [
        0xc4, 0xe2, 0xf8, 0xf3, 0xdb, // BLSI RAX, RBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x8000_0000_0000_0100; // bits 8 and 63 set
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x100, "RAX should contain bit 8 (lowest)");
}

#[test]
fn test_blsi_max_value_32bit() {
    // BLSI with maximum value 32-bit
    let code = [
        0xc4, 0xe2, 0x78, 0xf3, 0xdb, // BLSI EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xFFFFFFFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 1, "EAX should contain bit 0");
}

#[test]
fn test_blsi_max_value_64bit() {
    // BLSI with maximum value 64-bit
    let code = [
        0xc4, 0xe2, 0xf8, 0xf3, 0xdb, // BLSI RAX, RBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xFFFFFFFFFFFFFFFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 1, "RAX should contain bit 0");
}

#[test]
fn test_blsi_flags_behavior() {
    // Test that OF is cleared and SF is set according to result
    // Note: Intel docs say "ZF and SF are updated based on the result. OF is cleared."
    let code = [
        0xc4, 0xe2, 0x78, 0xf3, 0xdb, // BLSI EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x80000000; // Only bit 31 set, result will also be 0x80000000 (sign bit set)
    regs.rflags = 0x2 | (1 << 11); // Set OF to verify it gets cleared
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(
        sf_set(regs.rflags),
        "SF should be set (result has sign bit)"
    );
    assert!(!of_set(regs.rflags), "OF should be clear");
}

#[test]
fn test_blsi_practical_bit_extraction() {
    // Practical use case: extract lowest set bit for bit iteration
    let code = [
        0xc4, 0xe2, 0x78, 0xf3, 0xdb, // BLSI EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x12345678;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // 0x12345678 = 0001 0010 0011 0100 0101 0110 0111 1000
    // Lowest set bit is bit 3 (0x8)
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x8,
        "Should extract lowest set bit (0x8)"
    );
}

#[test]
fn test_blsi_byte_boundaries() {
    // Test extraction at byte boundaries
    let test_cases = [
        (0x00000100u32, 0x100u32),      // bit 8
        (0x00010000u32, 0x10000u32),    // bit 16
        (0x01000000u32, 0x01000000u32), // bit 24
        (0x80000000u32, 0x80000000u32), // bit 31
    ];

    for (input, expected) in &test_cases {
        let code = [
            0xc4, 0xe2, 0x78, 0xf3, 0xdb, // BLSI EAX, EBX
            0xf4,
        ];
        let mut regs = Registers::default();
        regs.rbx = *input as u64;
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        assert_eq!(
            regs.rax & 0xFFFFFFFF,
            *expected as u64,
            "BLSI({:08x}) should be {:08x}",
            input,
            expected
        );
    }
}

#[test]
fn test_blsi_64bit_comprehensive() {
    // Comprehensive 64-bit test
    let code = [
        0xc4, 0xe2, 0xf8, 0xf3, 0xdb, // BLSI RAX, RBX
        0xf4,
    ];
    let test_cases = [
        (0x0000_0000_0000_0001u64, 0x0000_0000_0000_0001u64), // bit 0
        (0x0000_0001_0000_0000u64, 0x0000_0001_0000_0000u64), // bit 32
        (0x8000_0000_0000_0000u64, 0x8000_0000_0000_0000u64), // bit 63
        (0xAAAA_AAAA_AAAA_AAAAu64, 0x0000_0000_0000_0002u64), // bit 1
    ];

    for (input, expected) in &test_cases {
        let mut regs = Registers::default();
        regs.rbx = *input;
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        assert_eq!(
            regs.rax, *expected,
            "BLSI({:016x}) should be {:016x}",
            input, expected
        );
    }
}
