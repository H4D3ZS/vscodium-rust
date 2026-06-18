use crate::common::*;
use rax::cpu::Registers;

// BLSR - Reset Lowest Set Bit (BMI1)
// Resets the lowest set bit in the source operand and writes the result to the destination.
// All other bits are unchanged.
// This is equivalent to: dest = src & (src - 1)
// Sets ZF if result is zero, sets CF if source was zero, clears SF and OF.
//
// Opcodes:
// VEX.NDD.LZ.0F38.W0 F3 /1   BLSR r32, r/m32   - Reset lowest set bit (32-bit)
// VEX.NDD.LZ.0F38.W1 F3 /1   BLSR r64, r/m64   - Reset lowest set bit (64-bit)

#[test]
fn test_blsr_eax_ebx_bit_0() {
    // BLSR EAX, EBX - reset bit 0 (only bit set)
    let code = [
        0xc4, 0xe2, 0x78, 0xf3, 0xcb, // BLSR EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0b0000_0001; // bit 0 set
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0b0000_0000,
        "EAX should be zero (bit 0 reset)"
    );
    assert!(zf_set(regs.rflags), "ZF should be set (result is zero)");
    assert!(
        !cf_set(regs.rflags),
        "CF should be clear (source was non-zero)"
    );
}

#[test]
fn test_blsr_eax_ebx_bit_3() {
    // BLSR EAX, EBX - reset bit 3
    let code = [
        0xc4, 0xe2, 0x78, 0xf3, 0xcb, // BLSR EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0b0000_1000; // only bit 3 set
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 0b0000_0000, "EAX should be zero");
    assert!(zf_set(regs.rflags), "ZF should be set");
}

#[test]
fn test_blsr_eax_ebx_multiple_bits() {
    // BLSR EAX, EBX - reset lowest of multiple bits
    let code = [
        0xc4, 0xe2, 0x78, 0xf3, 0xcb, // BLSR EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0b1010_1000; // bits 3, 5, 7 set
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0b1010_0000,
        "EAX should have bit 3 reset"
    );
    assert!(
        !zf_set(regs.rflags),
        "ZF should be clear (result is non-zero)"
    );
}

#[test]
fn test_blsr_rax_rbx_bit_0() {
    // BLSR RAX, RBX - 64-bit version with bit 0
    let code = [
        0xc4, 0xe2, 0xf8, 0xf3, 0xcb, // BLSR RAX, RBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x0000_0000_0000_0001;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0, "RAX should be zero");
    assert!(zf_set(regs.rflags), "ZF should be set");
}

#[test]
fn test_blsr_zero_source() {
    // BLSR with zero source
    let code = [
        0xc4, 0xe2, 0x78, 0xf3, 0xcb, // BLSR EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0; // zero source
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 0, "EAX should be zero");
    assert!(zf_set(regs.rflags), "ZF should be set");
    assert!(cf_set(regs.rflags), "CF should be set (source was zero)");
}

#[test]
fn test_blsr_all_bits_set() {
    // BLSR with all bits set
    let code = [
        0xc4, 0xe2, 0x78, 0xf3, 0xcb, // BLSR EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xFFFFFFFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0xFFFFFFFE,
        "EAX should have bit 0 reset"
    );
    assert!(!zf_set(regs.rflags), "ZF should be clear");
}

#[test]
fn test_blsr_alternating_pattern() {
    // BLSR with alternating pattern 1010...1010
    let code = [
        0xc4, 0xe2, 0x78, 0xf3, 0xcb, // BLSR EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xAAAAAAAA; // 1010...1010 (bit 1 is lowest)
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0xAAAAAAAA & !0x2,
        "EAX should have bit 1 reset"
    );
}

#[test]
fn test_blsr_alternating_pattern_inverted() {
    // BLSR with alternating pattern 0101...0101
    let code = [
        0xc4, 0xe2, 0x78, 0xf3, 0xcb, // BLSR EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x55555555; // 0101...0101 (bit 0 is lowest)
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x55555554,
        "EAX should have bit 0 reset"
    );
}

#[test]
fn test_blsr_single_bit_positions_32bit() {
    // Test each individual bit position for 32-bit
    for bit_pos in 0..32 {
        let code = [
            0xc4, 0xe2, 0x78, 0xf3, 0xcb, // BLSR EAX, EBX
            0xf4,
        ];
        let mut regs = Registers::default();
        regs.rbx = 1u64 << bit_pos;
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        assert_eq!(
            regs.rax & 0xFFFFFFFF,
            0,
            "EAX should be zero for single bit at {}",
            bit_pos
        );
        assert!(
            zf_set(regs.rflags),
            "ZF should be set for single bit at {}",
            bit_pos
        );
    }
}

#[test]
fn test_blsr_single_bit_positions_64bit() {
    // Test each individual bit position for 64-bit
    for bit_pos in 0..64 {
        let code = [
            0xc4, 0xe2, 0xf8, 0xf3, 0xcb, // BLSR RAX, RBX
            0xf4,
        ];
        let mut regs = Registers::default();
        regs.rbx = 1u64 << bit_pos;
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        assert_eq!(
            regs.rax, 0,
            "RAX should be zero for single bit at {}",
            bit_pos
        );
        assert!(
            zf_set(regs.rflags),
            "ZF should be set for single bit at {}",
            bit_pos
        );
    }
}

#[test]
fn test_blsr_with_extended_registers() {
    // BLSR R8D, R9D
    let code = [
        0xc4, 0x42, 0x38, 0xf3, 0xc9, // BLSR R8D, R9D (vvvv=0111 inv=8=R8, r/m=001+B=R9)
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.r9 = 0b0001_1000; // bits 3 and 4 set
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.r8 & 0xFFFFFFFF,
        0b0001_0000,
        "R8D should have bit 3 reset"
    );
}

#[test]
fn test_blsr_mem32() {
    // BLSR EAX, [mem]
    let code = [
        0xc4, 0xe2, 0x78, 0xf3, 0x0c, 0x25, 0x00, 0x20, 0x00, 0x00, // BLSR EAX, [DATA_ADDR]
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u32(&mem, 0xFFFFF000); // bits 12-31 set
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0xFFFFE000,
        "EAX should have bit 12 reset"
    );
}

#[test]
fn test_blsr_mem64() {
    // BLSR RAX, [mem]
    let code = [
        0xc4, 0xe2, 0xf8, 0xf3, 0x0c, 0x25, 0x00, 0x20, 0x00, 0x00, // BLSR RAX, [DATA_ADDR]
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u64(&mem, 0x100_0000_0000); // bit 40 set
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0, "RAX should be zero");
}

#[test]
fn test_blsr_trailing_zeros() {
    // BLSR with trailing zeros
    let code = [
        0xc4, 0xe2, 0x78, 0xf3, 0xcb, // BLSR EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xFFFFF000; // 12 trailing zeros, bit 12 is lowest set
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0xFFFFE000,
        "EAX should have bit 12 reset"
    );
}

#[test]
fn test_blsr_sparse_pattern() {
    // BLSR with sparse bit pattern
    let code = [
        0xc4, 0xe2, 0x78, 0xf3, 0xcb, // BLSR EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x80001000; // bits 12 and 31 set
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x80000000,
        "EAX should have bit 12 reset"
    );
}

#[test]
fn test_blsr_preserves_source() {
    // BLSR should not modify source operand
    let code = [
        0xc4, 0xe2, 0x78, 0xf3, 0xcb, // BLSR EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x12345678;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx & 0xFFFFFFFF, 0x12345678, "EBX should be unchanged");
}

#[test]
fn test_blsr_vs_and_sub() {
    // BLSR is equivalent to src & (src - 1)
    let code = [
        0xc4, 0xe2, 0x78, 0xf3, 0xcb, // BLSR EAX, EBX
        0xf4,
    ];
    let value = 0x12345678u32;
    let mut regs = Registers::default();
    regs.rbx = value as u64;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let expected = value & value.wrapping_sub(1);
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        expected as u64,
        "BLSR should equal src & (src - 1)"
    );
}

#[test]
fn test_blsr_power_of_two() {
    // BLSR of power of two returns zero
    for i in 0..32 {
        let code = [
            0xc4, 0xe2, 0x78, 0xf3, 0xcb, // BLSR EAX, EBX
            0xf4,
        ];
        let mut regs = Registers::default();
        regs.rbx = 1u64 << i;
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        assert_eq!(regs.rax & 0xFFFFFFFF, 0, "BLSR(2^{}) should be zero", i);
    }
}

#[test]
fn test_blsr_consecutive_bits() {
    // BLSR with consecutive bits set
    let code = [
        0xc4, 0xe2, 0x78, 0xf3, 0xcb, // BLSR EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x00FF0000; // bits 16-23 set
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x00FE0000,
        "EAX should have bit 16 reset"
    );
}

#[test]
fn test_blsr_sign_bit() {
    // BLSR with sign bit set
    let code = [
        0xc4, 0xe2, 0x78, 0xf3, 0xcb, // BLSR EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x80000000; // sign bit set (bit 31)
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 0, "EAX should be zero");
    assert!(zf_set(regs.rflags), "ZF should be set");
}

#[test]
fn test_blsr_iterative_removal() {
    // Use BLSR to iterate through set bits
    let code = [
        0xc4, 0xe2, 0x78, 0xf3, 0xcb, // BLSR EAX, EBX
        0xf4,
    ];

    let mut value = 0b1010_1010u64;
    let expected_bits = [0b1010_1000, 0b1010_0000, 0b1000_0000, 0b0000_0000];

    for (idx, &expected) in expected_bits.iter().enumerate() {
        let mut regs = Registers::default();
        regs.rbx = value;
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        assert_eq!(
            regs.rax & 0xFFFFFFFF,
            expected,
            "Iteration {}: should have reset lowest bit",
            idx
        );

        // Update value for next iteration
        value = regs.rax & 0xFFFFFFFF;
    }
}

#[test]
fn test_blsr_high_bits_64() {
    // BLSR with high bits in 64-bit operand
    let code = [
        0xc4, 0xe2, 0xf8, 0xf3, 0xcb, // BLSR RAX, RBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x0800_0000_0000_0000; // bit 59 set
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0, "RAX should be zero");
    assert!(zf_set(regs.rflags), "ZF should be set");
}

#[test]
fn test_blsr_mixed_high_low_64() {
    // BLSR with both high and low bits
    let code = [
        0xc4, 0xe2, 0xf8, 0xf3, 0xcb, // BLSR RAX, RBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x8000_0000_0000_0100; // bits 8 and 63 set
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax, 0x8000_0000_0000_0000,
        "RAX should have bit 8 reset"
    );
}

#[test]
fn test_blsr_flags_zf() {
    // Test ZF flag behavior
    let code = [
        0xc4, 0xe2, 0x78, 0xf3, 0xcb, // BLSR EAX, EBX
        0xf4,
    ];

    // Single bit - result should be zero
    let mut regs = Registers::default();
    regs.rbx = 0x00000001;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert!(zf_set(regs.rflags), "ZF should be set for single bit");

    // Multiple bits - result should be non-zero
    let mut regs = Registers::default();
    regs.rbx = 0x00000003; // bits 0 and 1
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert!(!zf_set(regs.rflags), "ZF should be clear for multiple bits");
}

#[test]
fn test_blsr_practical_popcount() {
    // Practical use: count set bits by repeatedly applying BLSR
    let code = [
        0xc4, 0xe2, 0x78, 0xf3, 0xcb, // BLSR EAX, EBX
        0xf4,
    ];

    let mut value = 0b1010_1010u64;
    let mut count = 0;

    while value != 0 {
        let mut regs = Registers::default();
        regs.rbx = value;
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();
        value = regs.rax & 0xFFFFFFFF;
        count += 1;
    }

    assert_eq!(count, 4, "Should count 4 set bits");
}

#[test]
fn test_blsr_byte_boundaries_32bit() {
    // Test at byte boundaries
    let test_cases = [
        (0x00000100u32, 0x00000000u32), // bit 8 alone
        (0x00010000u32, 0x00000000u32), // bit 16 alone
        (0x01000000u32, 0x00000000u32), // bit 24 alone
        (0x80000000u32, 0x00000000u32), // bit 31 alone
        (0x00010100u32, 0x00010000u32), // bits 8,16
    ];

    for (input, expected) in &test_cases {
        let code = [
            0xc4, 0xe2, 0x78, 0xf3, 0xcb, // BLSR EAX, EBX
            0xf4,
        ];
        let mut regs = Registers::default();
        regs.rbx = *input as u64;
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        assert_eq!(
            regs.rax & 0xFFFFFFFF,
            *expected as u64,
            "BLSR({:08x}) should be {:08x}",
            input,
            expected
        );
    }
}

#[test]
fn test_blsr_64bit_comprehensive() {
    // Comprehensive 64-bit test
    let code = [
        0xc4, 0xe2, 0xf8, 0xf3, 0xcb, // BLSR RAX, RBX
        0xf4,
    ];
    let test_cases = [
        (0x0000_0000_0000_0001u64, 0x0000_0000_0000_0000u64), // bit 0
        (0x0000_0000_0000_0002u64, 0x0000_0000_0000_0000u64), // bit 1
        (0x0000_0001_0000_0000u64, 0x0000_0000_0000_0000u64), // bit 32
        (0x0000_0000_0000_0003u64, 0x0000_0000_0000_0002u64), // bits 0,1
        (0xFFFFFFFFFFFFFFFFu64, 0xFFFFFFFFFFFFFFFEu64),       // all bits
    ];

    for (input, expected) in &test_cases {
        let mut regs = Registers::default();
        regs.rbx = *input;
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        assert_eq!(
            regs.rax, *expected,
            "BLSR({:016x}) should be {:016x}",
            input, expected
        );
    }
}
