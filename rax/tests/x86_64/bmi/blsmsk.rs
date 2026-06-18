use crate::common::*;
use rax::cpu::Registers;

// BLSMSK - Get Mask Up to Lowest Set Bit (BMI1)
// Sets all the lower bits of the destination operand to 1 up to and including the lowest set bit
// in the source operand. All other bits are cleared.
// This is equivalent to: dest = src ^ (src - 1)
// Sets CF if source is zero, clears ZF and OF (SF reflects the result).
//
// Opcodes:
// VEX.NDD.LZ.0F38.W0 F3 /2   BLSMSK r32, r/m32   - Create mask from lowest set bit (32-bit)
// VEX.NDD.LZ.0F38.W1 F3 /2   BLSMSK r64, r/m64   - Create mask from lowest set bit (64-bit)

#[test]
fn test_blsmsk_eax_ebx_bit_0() {
    // BLSMSK EAX, EBX - mask up to bit 0
    let code = [
        0xc4, 0xe2, 0x78, 0xf3, 0xd3, // BLSMSK EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0b0000_0001; // bit 0 set
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0b0000_0001,
        "EAX should contain mask up to bit 0"
    );
    assert!(
        !zf_set(regs.rflags),
        "ZF should be clear (source is non-zero)"
    );
    assert!(
        !cf_set(regs.rflags),
        "CF should be clear (source is non-zero)"
    );
}

#[test]
fn test_blsmsk_eax_ebx_bit_3() {
    // BLSMSK EAX, EBX - mask up to bit 3
    let code = [
        0xc4, 0xe2, 0x78, 0xf3, 0xd3, // BLSMSK EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0b0000_1000; // bit 3 set
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0b0000_1111,
        "EAX should contain mask up to bit 3 (bits 0-3)"
    );
    assert!(!zf_set(regs.rflags), "ZF should be clear");
    assert!(!cf_set(regs.rflags), "CF should be clear");
}

#[test]
fn test_blsmsk_eax_ebx_bit_7() {
    // BLSMSK EAX, EBX - mask up to bit 7
    let code = [
        0xc4, 0xe2, 0x78, 0xf3, 0xd3, // BLSMSK EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x00000080; // bit 7 set
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x000000FF,
        "EAX should contain bits 0-7 mask"
    );
}

#[test]
fn test_blsmsk_rax_rbx_bit_0() {
    // BLSMSK RAX, RBX - 64-bit version with bit 0
    let code = [
        0xc4, 0xe2, 0xf8, 0xf3, 0xd3, // BLSMSK RAX, RBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x0000_0000_0000_0001;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax, 0x0000_0000_0000_0001,
        "RAX should contain mask up to bit 0"
    );
    assert!(!zf_set(regs.rflags), "ZF should be clear");
    assert!(!cf_set(regs.rflags), "CF should be clear");
}

#[test]
fn test_blsmsk_zero_source() {
    // BLSMSK with zero source
    let code = [
        0xc4, 0xe2, 0x78, 0xf3, 0xd3, // BLSMSK EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0; // zero source
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0xFFFFFFFF,
        "EAX should contain all 1s for zero source"
    );
    assert!(
        !zf_set(regs.rflags),
        "ZF should be clear (BLSMSK clears ZF)"
    );
    assert!(cf_set(regs.rflags), "CF should be set (source is zero)");
}

#[test]
fn test_blsmsk_multiple_bits() {
    // BLSMSK with multiple bits set
    let code = [
        0xc4, 0xe2, 0x78, 0xf3, 0xd3, // BLSMSK EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0b1010_1000; // bits 3, 5, 7 set - lowest is bit 3
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0b0000_1111,
        "EAX should contain bits 0-3 mask"
    );
    assert!(!zf_set(regs.rflags), "ZF should be clear");
}

#[test]
fn test_blsmsk_all_bits_set() {
    // BLSMSK with all bits set
    let code = [
        0xc4, 0xe2, 0x78, 0xf3, 0xd3, // BLSMSK EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xFFFFFFFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x00000001,
        "EAX should contain bit 0 mask"
    );
    assert!(!zf_set(regs.rflags), "ZF should be clear");
}

#[test]
fn test_blsmsk_alternating_pattern() {
    // BLSMSK with alternating pattern 1010...1010
    let code = [
        0xc4, 0xe2, 0x78, 0xf3, 0xd3, // BLSMSK EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xAAAAAAAA; // 1010...1010 (bit 1 is lowest)
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0b00000011,
        "EAX should contain bits 0-1 mask"
    );
}

#[test]
fn test_blsmsk_alternating_pattern_inverted() {
    // BLSMSK with alternating pattern 0101...0101
    let code = [
        0xc4, 0xe2, 0x78, 0xf3, 0xd3, // BLSMSK EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x55555555; // 0101...0101 (bit 0 is lowest)
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x00000001,
        "EAX should contain bit 0 mask"
    );
}

#[test]
fn test_blsmsk_single_bit_positions_32bit() {
    // Test each individual bit position for 32-bit
    for bit_pos in 0..32 {
        let code = [
            0xc4, 0xe2, 0x78, 0xf3, 0xd3, // BLSMSK EAX, EBX
            0xf4,
        ];
        let mut regs = Registers::default();
        regs.rbx = 1u64 << bit_pos;
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        let expected = (1u64 << (bit_pos + 1)) - 1;
        assert_eq!(
            regs.rax & 0xFFFFFFFF,
            expected,
            "EAX should contain mask for bit {}",
            bit_pos
        );
    }
}

#[test]
fn test_blsmsk_single_bit_positions_64bit() {
    // Test each individual bit position for 64-bit
    for bit_pos in 0..64 {
        let code = [
            0xc4, 0xe2, 0xf8, 0xf3, 0xd3, // BLSMSK RAX, RBX
            0xf4,
        ];
        let mut regs = Registers::default();
        regs.rbx = 1u64 << bit_pos;
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        let expected = if bit_pos == 63 {
            u64::MAX
        } else {
            (1u64 << (bit_pos + 1)) - 1
        };
        assert_eq!(
            regs.rax, expected,
            "RAX should contain mask for bit {}",
            bit_pos
        );
    }
}

#[test]
fn test_blsmsk_with_extended_registers() {
    // BLSMSK R8D, R9D
    let code = [
        0xc4, 0x42, 0x38, 0xf3, 0xd1, // BLSMSK R8D, R9D (vvvv=0111 inv=8=R8, r/m=001+B=R9)
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.r9 = 0b0001_1000; // bits 3 and 4 set
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.r8 & 0xFFFFFFFF, 0b0000_1111, "R8D should contain mask");
}

#[test]
fn test_blsmsk_mem32() {
    // BLSMSK EAX, [mem]
    let code = [
        0xc4, 0xe2, 0x78, 0xf3, 0x14, 0x25, 0x00, 0x20, 0x00, 0x00, // BLSMSK EAX, [DATA_ADDR]
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u32(&mem, 0x00001000); // bit 12 set
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x00001FFF,
        "EAX should contain bits 0-12 mask"
    );
}

#[test]
fn test_blsmsk_mem64() {
    // BLSMSK RAX, [mem]
    let code = [
        0xc4, 0xe2, 0xf8, 0xf3, 0x14, 0x25, 0x00, 0x20, 0x00, 0x00, // BLSMSK RAX, [DATA_ADDR]
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u64(&mem, 0x100_0000_0000); // bit 40 set
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax, 0x1FF_FFFF_FFFF,
        "RAX should contain bits 0-40 mask"
    );
}

#[test]
fn test_blsmsk_trailing_zeros() {
    // BLSMSK with trailing zeros
    let code = [
        0xc4, 0xe2, 0x78, 0xf3, 0xd3, // BLSMSK EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xFFFFF000; // 12 trailing zeros, bit 12 is lowest set
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x00001FFF,
        "EAX should contain bits 0-12 mask"
    );
}

#[test]
fn test_blsmsk_sparse_pattern() {
    // BLSMSK with sparse bit pattern
    let code = [
        0xc4, 0xe2, 0x78, 0xf3, 0xd3, // BLSMSK EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x80001000; // bits 12 and 31 set
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x00001FFF,
        "EAX should contain bits 0-12 mask"
    );
}

#[test]
fn test_blsmsk_preserves_source() {
    // BLSMSK should not modify source operand
    let code = [
        0xc4, 0xe2, 0x78, 0xf3, 0xd3, // BLSMSK EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x12345678;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx & 0xFFFFFFFF, 0x12345678, "EBX should be unchanged");
}

#[test]
fn test_blsmsk_vs_xor_sub() {
    // BLSMSK is equivalent to src ^ (src - 1)
    let code = [
        0xc4, 0xe2, 0x78, 0xf3, 0xd3, // BLSMSK EAX, EBX
        0xf4,
    ];
    let value = 0x12345678u32;
    let mut regs = Registers::default();
    regs.rbx = value as u64;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let expected = value ^ value.wrapping_sub(1);
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        expected as u64,
        "BLSMSK should equal src ^ (src - 1)"
    );
}

#[test]
fn test_blsmsk_power_of_two() {
    // BLSMSK of power of two returns all lower bits
    for i in 0..32 {
        let code = [
            0xc4, 0xe2, 0x78, 0xf3, 0xd3, // BLSMSK EAX, EBX
            0xf4,
        ];
        let mut regs = Registers::default();
        regs.rbx = 1u64 << i;
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        let expected = (1u64 << i) | ((1u64 << i) - 1);
        assert_eq!(regs.rax & 0xFFFFFFFF, expected, "BLSMSK(2^{}) incorrect", i);
    }
}

#[test]
fn test_blsmsk_consecutive_bits() {
    // BLSMSK with consecutive bits set
    let code = [
        0xc4, 0xe2, 0x78, 0xf3, 0xd3, // BLSMSK EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x00FF0000; // bits 16-23 set
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x0001FFFF,
        "EAX should contain bits 0-16 mask"
    );
}

#[test]
fn test_blsmsk_sign_bit() {
    // BLSMSK with sign bit set
    let code = [
        0xc4, 0xe2, 0x78, 0xf3, 0xd3, // BLSMSK EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x80000000; // sign bit set (bit 31)
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0xFFFFFFFF,
        "EAX should contain all 1s"
    );
}

#[test]
fn test_blsmsk_high_bits_64() {
    // BLSMSK with high bits in 64-bit operand
    let code = [
        0xc4, 0xe2, 0xf8, 0xf3, 0xd3, // BLSMSK RAX, RBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x0800_0000_0000_0000; // bit 59 set
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax, 0x0FFF_FFFF_FFFF_FFFF,
        "RAX should contain bits 0-59 mask"
    );
}

#[test]
fn test_blsmsk_mixed_high_low_64() {
    // BLSMSK with both high and low bits
    let code = [
        0xc4, 0xe2, 0xf8, 0xf3, 0xd3, // BLSMSK RAX, RBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x8000_0000_0000_0100; // bits 8 and 63 set
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x00000001FF, "RAX should contain bits 0-8 mask");
}

#[test]
fn test_blsmsk_flags_cf() {
    // CF should be clear for non-zero, set for zero
    let code = [
        0xc4, 0xe2, 0x78, 0xf3, 0xd3, // BLSMSK EAX, EBX
        0xf4,
    ];

    // Test non-zero
    let mut regs = Registers::default();
    regs.rbx = 0x12345678;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert!(
        !cf_set(regs.rflags),
        "CF should be clear for non-zero source"
    );

    // Test zero
    let mut regs = Registers::default();
    regs.rbx = 0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert!(cf_set(regs.rflags), "CF should be set for zero source");
}

#[test]
fn test_blsmsk_practical_mask_creation() {
    // Practical use case: create mask for lower bits
    let code = [
        0xc4, 0xe2, 0x78, 0xf3, 0xd3, // BLSMSK EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x00000100; // bit 8 set
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Should create mask for bits 0-8
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x000001FF,
        "Should create mask for lower bits"
    );
}

#[test]
fn test_blsmsk_byte_boundaries_32bit() {
    // Test with byte boundaries
    let test_cases = [
        (0x00000001u32, 0x00000001u32), // bit 0
        (0x00000100u32, 0x000001FFu32), // bit 8
        (0x00010000u32, 0x0001FFFFu32), // bit 16
        (0x01000000u32, 0x01FFFFFFu32), // bit 24
        (0x80000000u32, 0xFFFFFFFFu32), // bit 31
    ];

    for (input, expected) in &test_cases {
        let code = [
            0xc4, 0xe2, 0x78, 0xf3, 0xd3, // BLSMSK EAX, EBX
            0xf4,
        ];
        let mut regs = Registers::default();
        regs.rbx = *input as u64;
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        assert_eq!(
            regs.rax & 0xFFFFFFFF,
            *expected as u64,
            "BLSMSK({:08x}) should be {:08x}",
            input,
            expected
        );
    }
}

#[test]
fn test_blsmsk_64bit_comprehensive() {
    // Comprehensive 64-bit test
    let code = [
        0xc4, 0xe2, 0xf8, 0xf3, 0xd3, // BLSMSK RAX, RBX
        0xf4,
    ];
    let test_cases = [
        (0x0000_0000_0000_0001u64, 0x0000_0000_0000_0001u64), // bit 0
        (0x0000_0000_0000_0100u64, 0x0000_0000_0000_01FFu64), // bit 8
        (0x0000_0001_0000_0000u64, 0x0000_0001_FFFFFFFFu64),  // bit 32
        (0x8000_0000_0000_0000u64, 0xFFFFFFFFFFFFFFFFu64),    // bit 63
    ];

    for (input, expected) in &test_cases {
        let mut regs = Registers::default();
        regs.rbx = *input;
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        assert_eq!(
            regs.rax, *expected,
            "BLSMSK({:016x}) should be {:016x}",
            input, expected
        );
    }
}
